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
PATH=$PWD/target/$profile:$PATH

for program in solana-{faucet,genesis,keygen,validator}; do
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
dataDir=$PWD/config/"$(basename "$0" .sh)"
ledgerDir=$PWD/config/station-svm-chain

SOLANA_RUN_SH_CLUSTER_TYPE=${SOLANA_RUN_SH_CLUSTER_TYPE:-development}

set -x
if ! solana address; then
  echo Generating default keypair
  solana-keygen new --no-passphrase --force
fi
validator_identity="$dataDir/validator-identity.json"
if [[ -e $validator_identity ]]; then
  echo "Use existing validator keypair"
else
  solana-keygen new --no-passphrase -so "$validator_identity" --force
fi
validator_vote_account="$dataDir/validator-vote-account.json"
if [[ -e $validator_vote_account ]]; then
  echo "Use existing validator vote account keypair"
else
  solana-keygen new --no-passphrase -so "$validator_vote_account" --force
fi
validator_stake_account="$dataDir/validator-stake-account.json"
if [[ -e $validator_stake_account ]]; then
  echo "Use existing validator stake account keypair"
else
  solana-keygen new --no-passphrase -so "$validator_stake_account" --force
fi

if [[ -e "$ledgerDir"/genesis.bin || -e "$ledgerDir"/genesis.tar.bz2 ]]; then
  echo "Use existing genesis"
else
  ./fetch-spl.sh "$ledgerDir/program"
  if [[ -r spl-genesis-args.sh ]]; then
    SPL_GENESIS_ARGS=$(cat spl-genesis-args.sh)
  fi

  # shellcheck disable=SC2086
  solana-genesis \
    --hashes-per-tick sleep \
    --faucet-lamports 10000000000000000000 \
    --bootstrap-validator-lamports 1000000000000000000 \
    --bootstrap-validator-stake-lamports 10000000000000000 \
    --bootstrap-validator \
      "$validator_identity" \
      "$validator_vote_account" \
      "$validator_stake_account" \
    --ledger "$ledgerDir" \
    --cluster-type testnet \
    $SPL_GENESIS_ARGS \
    --ticks-per-slot 64 \
    --slots-per-epoch 432000 \
    $SOLANA_RUN_SH_GENESIS_ARGS
fi

abort() {
  set +e
  kill "$faucet" "$validator"
  wait "$validator"
}
trap abort INT TERM EXIT

solana-faucet &
faucet=$!

args=(
  --identity "$validator_identity"
  --vote-account "$validator_vote_account"
  --ledger "$ledgerDir"
  --no-poh-speed-test
  --no-os-network-limits-test
  --gossip-port 8001
  --full-rpc-api
  --rpc-bind-address 0.0.0.0
  --bind-address 0.0.0.0
  --rpc-port 8899
  --rpc-faucet-address 127.0.0.1:9900
  --log -
  --enable-rpc-transaction-history
  --enable-extended-tx-metadata-storage
  --init-complete-file "$dataDir"/init-completed
  --require-tower
  --no-wait-for-vote-to-start-leader
  --snapshot-interval-slots 1000
  --no-incremental-snapshots
)
# shellcheck disable=SC2086
solana-validator "${args[@]}" $SOLANA_RUN_SH_VALIDATOR_ARGS &
validator=$!

wait "$validator"
