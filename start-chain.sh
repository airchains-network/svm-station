#!/bin/bash

set -e

# Prefer possible `cargo build` binaries over PATH binaries
script_dir="$(readlink -f "$(dirname "$0")")"
if [[ "$script_dir" =~ /scripts$ ]]; then
  cd "$script_dir/.."
else
  cd "$script_dir"
fi


profile=debug
if [[ -n $NDEBUG ]]; then
  profile=release
fi

for program in ./target/"$profile"/solana-{faucet,genesis,keygen,validator}; do
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
dataDir=$PWD/config/keypair
ledgerDir=$PWD/config/station-svm-chain

SOLANA_RUN_SH_CLUSTER_TYPE=${SOLANA_RUN_SH_CLUSTER_TYPE:-development}

set -x
if ! ./target/"$profile"/solana address; then
  echo Generating default keypair
  ./target/"$profile"/solana-keygen new --no-passphrase --force
fi
validator_identity="$dataDir/validator-identity.json"
if [[ -e $validator_identity ]]; then
  echo "Use existing validator keypair"
else
  ./target/"$profile"/solana-keygen new --no-passphrase -so "$validator_identity" --force
fi
validator_vote_account="$dataDir/validator-vote-account.json"
if [[ -e $validator_vote_account ]]; then
  echo "Use existing validator vote account keypair"
else
  ./target/"$profile"/solana-keygen new --no-passphrase -so "$validator_vote_account" --force
fi
validator_stake_account="$dataDir/validator-stake-account.json"
if [[ -e $validator_stake_account ]]; then
  echo "Use existing validator stake account keypair"
else
  ./target/"$profile"/solana-keygen new --no-passphrase -so "$validator_stake_account" --force
fi

if [[ -e "$ledgerDir"/genesis.bin || -e "$ledgerDir"/genesis.tar.bz2 ]]; then
  echo "Use existing genesis"
else
  ./fetch-spl.sh "config/program"
  if [[ -r spl-genesis-args.sh ]]; then
    SPL_GENESIS_ARGS=$(cat spl-genesis-args.sh)
  fi

  # shellcheck disable=SC2086
  ./target/"$profile"/solana-genesis \
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

./target/"$profile"/solana-faucet &
faucet=$!

#args=(
# --identity "$validator_identity"
# --vote-account "$validator_vote_account"
# --ledger "$ledgerDir"
# --rpc-port 8899
# --gossip-port 8001
# --snapshot-interval-slots 1000
# --no-incremental-snapshots
# --rpc-faucet-address 127.0.0.1:9900
# --rpc-bind-address 0.0.0.0
# --bind-address 0.0.0.0
# --log -
# --no-poh-speed-test
# --no-wait-for-vote-to-start-leader
# --full-rpc-api
# --allow-private-addr
# --enable-rpc-transaction-history
# --enable-extended-tx-metadata-storage
# --require-tower
# --no-os-network-limits-test
#
##  --identity "$validator_identity"
##  --vote-account "$validator_vote_account"
##  --ledger "$ledgerDir"
##  --no-poh-speed-test
##  --no-os-network-limits-test
##  --gossip-port 8001
##  --full-rpc-api
##  --rpc-bind-address 0.0.0.0
##  --bind-address 0.0.0.0
##  --rpc-port 8899
##  --rpc-faucet-address 127.0.0.1:9900
##  --log -
##  --enable-rpc-transaction-history
##  --enable-extended-tx-metadata-storage
##  --init-complete-file "$dataDir"/init-completed
##  --require-tower
##  --no-wait-for-vote-to-start-leader
##  --snapshot-interval-slots 1000
##  --no-incremental-snapshots
#)

# shellcheck disable=SC2086
./target/"$profile"/solana-validator \
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
  --log - \
  --no-poh-speed-test \
  --no-wait-for-vote-to-start-leader \
  --full-rpc-api \
  --allow-private-addr \
  --enable-rpc-transaction-history \
  --enable-extended-tx-metadata-storage \
  --require-tower \
  --no-os-network-limits-test &

wait "$validator"
