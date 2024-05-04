
![Project Logo](https://www.airchains.io/assets/logos/airchains-svm-rollup-full-logo.png) 

This repository contains the implementation for running a Solana-based chain using Airchains' rollup-as-a-service platform. Our solution leverages the robust features of Solana to provide a scalable and efficient rollup service.

## Table of Contents

- [Introduction](#introduction)
- [Building the Project](#building-the-project)
  - [Prerequisites](#prerequisites)
  - [1. Install rustc, cargo, and rustfmt](#1-install-rustc-cargo-and-rustfmt)
  - [2. Update Rust to the Latest Stable Version](#2-update-rust-to-the-latest-stable-version)
  - [3.  Building a Specific Release Branch](#3--building-a-specific-release-branch)
  - [4. Install Linux Dependencies](#4-install-linux-dependencies)
  - [5. Clone the Repository](#5-clone-the-repository)
  - [6. Build the Project](#6-build-the-project)
  - [7. Run the Project](#7-run-the-project)
- [Acknowledgments](#acknowledgments)
- [License](#license)

## Introduction

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

## Building the Project

The following instructions will guide you through setting up your development environment to build and run the Solana chain in the context of AirChains.

### Prerequisites

- Rust programming language
- Cargo (Rust's package manager)
- Rustfmt (Rust's code formatter)
- Solana CLI tools

### 1. Install rustc, cargo, and rustfmt

Start by installing Rust and its associated tools:

```bash
curl https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env
rustup component add rustfmt
```
### 2. Update Rust to the Latest Stable Version

```bash
rustup update
```

### 3.  Building a Specific Release Branch

For a specific release branch, check the required Rust version:

```bash
rustup install [SPECIFIC_RUST_VERSION]
```

### 4. Install Linux Dependencies

**For Ubuntu:**

```bash 
sudo apt-get update
sudo apt-get install libssl-dev libudev-dev pkg-config zlib1g-dev llvm clang cmake make libprotobuf-dev protobuf-compiler
```

**For Fedora:**

```bash
sudo dnf install openssl-devel systemd-devel pkg-config zlib-devel llvm clang cmake make protobuf-devel protobuf-compiler perl-core
```

### 5. Clone the Repository

   ```bash
   https://github.com/airchains-network/rollup-svm
   ```

### 6. Build the Project

   ```bash
   cd rollup-svm
   cargo build
   ```

### 7. Run the Project

   ```bash
    path/to/build/directory/svm-station-test-validator
   ```

## Running the Solana Chain

### Step 1 **Prepare Account**

Create the 4 keypair for solana you will need to run your solana chain.

This command generates a cryptographic key pair specifically for a node in the Solana blockchain network. The generated keys will be used to identify and authenticate the validator ,voter, staker on the network.

- #### chain faucet key
```shell
./target/debug/svm-station-keygen new --no-passphrase -so $HOME/.svmstationd/keys/faucet.json --force
```
- #### chain validator key

```shell
./target/debug/svm-station-keygen new --no-passphrase -so $HOME/.svmstationd/keys/validator-identity.json --force
```
- #### chain validator's voter key
```shell
./target/debug/svm-station-keygen new --no-passphrase -so $HOME/.svmstationd/keys/validator-vote-account.json --force
```

- #### chain validator's staker key

```shell
./target/debug/svm-station-keygen new --no-passphrase -so $HOME/.svmstationd/keys/validator-stake-account.json --force
```


### Step 2 **Initialize the Program**

fetch_spl_programs.sh is to automate the process of fetching the latest Solana Program Library (SPL) programs generating the necessary command-line arguments for installing them using the Solana blockchain's solana-genesis tool.

```shell
chmod +x fetch_spl_programs.sh
./fetch_spl_programs.sh &lt;path to program&gt;  # path to download spl program example : = "program"
```

### Step 3 **Create Genesis**

This is a command-line invocation of the solana-genesis tool, which is used for initializing a new Solana blockchain network.

Configuration :

- The command sets various configurations such as the number of hashes per tick, the amount of lamports in the faucet account, and the stake for the bootstrap validator
- It specifies the paths to JSON files containing the validator's identity, vote account, and stake account keys
- It specifies the directory where blockchain data will be stored.

Example : “chain/ledger”

Loading Programs :

- The command loads several BPF (Berkeley Packet Filter) programs into the blockchain network. These programs include the Token program, Memo program, Associated Token Account program, and Feature Proposal program from the Solana Program Library
- Each program is specified with its program ID, loader ID, and the path to its corresponding BPF program file.

```shell
  ./target/debug/svm-station-genesis \
  --ledger $HOME/.svmstationd/station-svm-chain/ \
  --hashes-per-tick sleep \
  --faucet-lamports 10000000000000000000 \
  --bootstrap-validator-lamports 100000000000000000 \
  --bootstrap-validator-stake-lamports 1000000000000000 \
  --bootstrap-validator  $HOME/.svmstationd/keys/validator-identity.json \
  $HOME/.svmstationd/keys/validator-vote-account.json \
  $HOME/.svmstationd/keys/validator-stake-account.json \
  --cluster-type testnet \
  --ticks-per-slot 44 \
  --slots-per-epoch 432000 \
  --bpf-program_ TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA BPFLoader2111111111111111111111111111111111 $HOME/.svmstationd/spl_token-3.5.0.so \
  --bpf-program TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb BPFLoaderUpgradeab1e11111111111111111111111111 $HOME/.svmstationd/spl_token-2022-0.9.0.so \
  --bpf-program Memo1UhkJRfHyvLMcVucJwxXeuD728EqVDDwQDxFMNo BPFLoader1111111111111111111111111111111111 $HOME/.svmstationd/spl_memo-1.0.0.so \
  --bpf-program MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr BPFLoader2111111111111111111111111111111111 $HOME/.svmstationd/spl_memo-3.0.0.so \
  --bpf-program ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL BPFLoader2111111111111111111111111111111111 $HOME/.svmstationd/spl_associated-token-account-1.1.2.so \
  --bpf-program Feat1YXHhH6t1juaWF74WLcfv4XoNocjXA6sPWHNgAse BPFLoader2111111111111111111111111111111111 $HOME/.svmstationd/spl_feature-proposal-1.0.0.so
```


### Step 4 **Start Node**

solana-validator runs the Solana validator node with specific configurations.

Identity and Vote Account : The validator's identity and vote account are specified via JSON files located in the ‘chain/keys’ directory.

Ledger Configuration: The directory where blockchain ledger data will be stored is specified as ‘chain/ledger’

Network Configuration:

- The gossip port is set to 8001, enabling communication between nodes.
- RPC (Remote Procedure Call) services are configured to listen on 0.0.0.0:8899 for incoming requests.
- The full RPC API is enabled, allowing clients to access various Solana RPC methods.
- The RPC faucet address is set to 127.0.0.1:9900, indicating the local address for the faucet service

```shell
./target/debug/svm-station-validator \
  --identity  $HOME/.svmstationd/keys/validator-identity.json \
  --vote-account  $HOME/.svmstationd/keys/validator-vote-account.json \
  --ledger  $HOME/.svmstationd/station-svm-chain/ \
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
```


## Acknowledgments

- This project is built upon the [Solana blockchain](https://github.com/solana-labs). We extend our gratitude to the Solana Labs and the community for their foundational work.

- Visit the official [Solana GitHub repository](https://github.com/solana-labs/solana) for more information about the Solana blockchain and its features.

## License

This project is licensed under the MIT License - see the LICENSE.md file for details.

*Disclaimer: The information in this repository is subject to change and may not always be up-to-date with the latest version of Solana or AirChains' platform.*
