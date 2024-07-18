#!/bin/bash

set -e

# Prefer possible `cargo build` binaries over PATH binaries
script_dir="$(readlink -f "$(dirname "$0")")"
if [[ "$script_dir" =~ /scripts$ ]]; then
  cd "$script_dir/.."
else
  cd "$script_dir"
fi


profile=release
if [[ -n $NDEBUG ]]; then
  profile=release
fi

for program in ./target/"$profile"/svm-station-{faucet,genesis,keygen,validator}; do
  $program -V || ok=false
done
$ok || {
  echo
  echo "Unable to locate required programs.  Try building them first with:"
  echo
  echo "  $ cargo build --all"
  echo
  exit 1
}

export RUST_LOG=${RUST_LOG:-solana=info,solana_runtime::message_processor=debug} # if RUST_LOG is unset, default to info
export RUST_BACKTRACE=1
pathDir=$HOME/.svmstationd
dataDir=$pathDir/keypair
ledgerDir=$pathDir/station-svm-chain

SOLANA_RUN_SH_CLUSTER_TYPE=${SOLANA_RUN_SH_CLUSTER_TYPE:-testnet}

set -x
if ! ./target/"$profile"/svm-station address; then
  echo Generating default keypair
  ./target/"$profile"/svm-station-keygen new --no-passphrase --force
fi
validator_identity="$dataDir/validator-identity.json"
if [[ -e $validator_identity ]]; then
  echo "Use existing validator keypair"
else
  ./target/"$profile"/svm-station-keygen new --no-passphrase -so "$validator_identity" --force
fi
validator_vote_account="$dataDir/validator-vote-account.json"
if [[ -e $validator_vote_account ]]; then
  echo "Use existing validator vote account keypair"
else
  ./target/"$profile"/svm-station-keygen new --no-passphrase -so "$validator_vote_account" --force
fi
validator_stake_account="$dataDir/validator-stake-account.json"
if [[ -e $validator_stake_account ]]; then
  echo "Use existing validator stake account keypair"
else
  ./target/"$profile"/svm-station-keygen new --no-passphrase -so "$validator_stake_account" --force
fi

if [[ -e "$ledgerDir"/genesis.bin || -e "$ledgerDir"/genesis.tar.bz2 ]]; then
  echo "Use existing genesis"
else

  download_path="$pathDir"/program
  upgradeableLoader=BPFLoaderUpgradeab1e11111111111111111111111

  fetch_program() {
    declare name=$1
    declare version=$2
    declare address=$3
    declare loader=$4

    declare so="$download_path/spl_$name-$version.so"

    if [[ $loader == "$upgradeableLoader" ]]; then
      genesis_args+=(--upgradeable-program "$address" "$loader" "$so" none)
    else
      genesis_args+=(--bpf-program "$address" "$loader" "$so")
    fi

    if [[ -r $so ]]; then
      return
    fi

    if [[ -r "$download_path"/.cache/solana-spl/$so ]]; then
      cp "$download_path"/.cache/solana-spl/"$so" "$so"
    else
      echo "Downloading $name $version"
      so_name="spl_${name//-/_}.so"
      (
        set -x
        curl -L --retry 5 --retry-delay 2 --retry-connrefused \
          -o "$so" \
          "https://github.com/solana-labs/solana-program-library/releases/download/$name-v$version/$so_name"
      )

      mkdir -p "$(dirname "$download_path"/.cache/solana-spl/"$so")"
      cp "$so" "$download_path"/.cache/solana-spl/"$so"
    fi

  }
  mkdir -p "$download_path"

  fetch_program memo  1.0.0 Memo1UhkJRfHyvLMcVucJwxXeuD728EqVDDwQDxFMNo BPFLoader1111111111111111111111111111111111
  fetch_program memo  3.0.0 MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr BPFLoader2111111111111111111111111111111111
  fetch_program associated-token-account 1.1.2 ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL BPFLoader2111111111111111111111111111111111
  fetch_program feature-proposal 1.0.0 Feat1YXHhH6t1juaWF74WLcfv4XoNocjXA6sPWHNgAse BPFLoader2111111111111111111111111111111111
  fetch_program token 3.5.0 TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA BPFLoader2111111111111111111111111111111111
  fetch_program token-2022 0.9.0 TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb BPFLoaderUpgradeab1e11111111111111111111111

 # shellcheck disable=SC2124
 SPL_GENESIS_ARGS=${genesis_args[@]}
  rm -rf "$download_path"/.cache

# shellcheck disable=SC2086
  ./target/"$profile"/svm-station-genesis \
  --ledger "$ledgerDir" \
  --hashes-per-tick sleep \
  --faucet-lamports 10000000000000000000 \
  --bootstrap-validator-lamports 100000000000000000 \
  --bootstrap-validator-stake-lamports 1000000000000000 \
  --bootstrap-validator  "$validator_identity" "$validator_vote_account" "$validator_stake_account" \
  --cluster-type testnet \
  --ticks-per-slot 64 \
  --slots-per-epoch 432000 \
  $SPL_GENESIS_ARGS
fi

abort() {
  set +e
  kill "$faucet" "$validator"
  wait "$validator"
}
trap abort INT TERM EXIT

./target/"$profile"/svm-station-faucet &
faucet=$!

# shellcheck disable=SC2086
./target/"$profile"/svm-station-validator \
  --identity "$validator_identity" \
  --vote-account "$validator_vote_account" \
  --ledger "$ledgerDir" \
  --rpc-port 8899 \
  --gossip-port 8001 \
  --snapshot-interval-slots 1000 \
  --no-incremental-snapshots \
  --rpc-faucet-address 127.0.0.1:9900 \
  --rpc-bind-address 0.0.0.0 \
  --bind-address 0.0.0.0 \
  --log  - \
  --no-poh-speed-test \
  --no-wait-for-vote-to-start-leader \
  --full-rpc-api \
  --allow-private-addr \
  --enable-rpc-transaction-history \
  --enable-extended-tx-metadata-storage \
  --require-tower \
   --geyser-plugin-config="svm-geyser/script/geysor.json" \
  --no-os-network-limits-test &
validator=$!

#  --geyser-plugin-config="svm-geyser/script/geysor.json" \
wait "$validator"
