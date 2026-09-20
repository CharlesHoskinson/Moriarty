[![Bindings Tests](https://github.com/anoma/anomapay-erc20-forwarder/actions/workflows/bindings.yml/badge.svg)](https://github.com/anoma/anomapay-erc20-forwarder/actions/workflows/bindings.yml) [![crates.io](https://img.shields.io/badge/crates.io-anomapay--erc20--forwarder--bindings-blue?logo=rust)](https://crates.io/crates/anoma-pa-evm-bindings) [![License](https://img.shields.io/badge/license-MIT-blue)](https://raw.githubusercontent.com/anoma/anomapay-erc20-forwarder/refs/heads/main/bindings/LICENSE)

# AnomaPay ERC20 Forwarder Bindings

This package provides [Rust](https://www.rust-lang.org/) bindings to the Solidity contracts and exposes the deployment addresses on the different supported networks using the [alloy-rs](https://github.com/alloy-rs)
library.

## Prerequisites

1. Get an up-to-date version of [Rust](https://www.rust-lang.org/) with

   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Install [RISC Zero `rzup`](https://github.com/risc0/risc0) with

   ```sh
   curl -L https://risczero.com/install | sh
   ```

3. Install the RISC Zero version 3.0.3 with

   ```sh
   rzup install cargo-risczero 3.0.3
   ```

## Usage

### Build

Run

```sh
cargo build
```

### Test

To test the build, run

```sh
cargo test
```
