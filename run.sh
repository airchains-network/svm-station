##!/usr/bin/env bash
#
#cat <<'EOF'
#
#  WARNING! LEGACY SHELL SCRIPT
#
#  You almost certainly do not want to run this script!
#
#  If you are a dapp developer and looking for a way to run a local validator, please
#  see https://docs.solana.com/developing/test-validator
#
#  If you are a prospective validator, please see https://docs.solana.com/running-validator
#
#  If you are a core developer, many apologies for what you're about to endure, but
#  you may be in the right place.  This script is now located at `./scripts/run.sh`.
#  Please update whatever docs lead you here to reflect this change
#
#EOF

./target/release/solana-genesis \
    --enable-warmup-epochs \
    --bootstrap-stake-authorized-pubkey DQESfNE1ghknkotD1niT5r3EusjppaQcNZfQABUmHMi4 \
    --bootstrap-validator FxdRfAAoo4Gttqu4taQcKNEGhDXhezhT2DQt4wqW48CG CaTZDT8TmxPk4gVK6pdwGZhA2LdgXUAQ4VZS9t84N7XK DQESfNE1ghknkotD1niT5r3EusjppaQcNZfQABUmHMi4 \
    --bootstrap-validator-lamports 500000000000 \
    --bootstrap-validator-stake-lamports 5000000000 \
    --cluster-type development \
    --faucet-lamports 999999999999999 \
    --faucet-pubkey test-ledger/faucet-keypair.json \
    --fee-burn-percentage 50 \
    --hashes-per-tick auto \
    --inflation none \
    --lamports-per-byte-year 3480 \
    --ledger test-ledger \
    --max-genesis-archive-unpacked-size 10485760 \
    --rent-burn-percentage 50 \
    --rent-exemption-threshold 2 \
    --slots-per-epoch 141444 \
    --target-lamports-per-signature 10000 \
    --target-signatures-per-slot 20000 \
    --vote-commission-percentage 100

# ./target/release/solana-validator \
#     --full-rpc-api \
#     --tpu-enable-udp \
#     --identity test-ledger/validator-keypair.json \
#     --ledger test-ledger \
#     --log test-ledger/log.log \
#     --no-incremental-snapshots \
#     --allow-private-addr \
#     --dynamic-port-range 8000-8020 \
#     --rpc-port 8899 \
#     --rpc-faucet-address 127.0.0.1:9900 \
#     --no-poh-speed-test \
#     --tower test-ledger \
#     --tower-storage file



# exec ./solana-validator \
#     --identity account/i.json \
#     --vote-account account/v.json \
#     --known-validator DKfNptH8qhh2A7HSsJsYMg4PQQW1muTsB6j1DfRokfhF \
#     --only-known-rpc \
#     --ledger new \
#     --rpc-port 3000 \
#     --snapshot-interval-slots 200 \
#     --no-incremental-snapshots \
#     --rpc-faucet-address 127.0.0.1:9900 \
#     --no-poh-speed-test \
#     --no-os-network-limits-test \
#     --no-wait-for-vote-to-start-leader \
#     --full-rpc-api \
#     --log new/log.log \
#     --entrypoint 127.0.0.1:1024 \
#     --expected-genesis-hash 2ZU4yo6A4Jzd3PGpWxuoTjFzVNcTDA4WmLYNL1J99sdH \
#     --allow-private-addr \
#     --private-rpc \
#     --dynamic-port-range 8000-8020 \