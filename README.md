
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
    path/to/build/directory/air-solana
   ```

## Acknowledgments

- This project is built upon the [Solana blockchain](https://github.com/solana-labs). We extend our gratitude to the Solana Labs and the community for their foundational work.

- Visit the official [Solana GitHub repository](https://github.com/solana-labs/solana) for more information about the Solana blockchain and its features.

## License

This project is licensed under the MIT License - see the LICENSE.md file for details.

*Disclaimer: The information in this repository is subject to change and may not always be up-to-date with the latest version of Solana or AirChains' platform.*
