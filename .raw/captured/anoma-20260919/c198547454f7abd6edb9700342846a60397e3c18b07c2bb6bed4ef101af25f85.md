[![Contracts Tests](https://github.com/anoma/generic-call-forwarder/actions/workflows/contracts.yml/badge.svg)](https://github.com/anoma/generic-call-forwarder/actions/workflows/contracts.yml) [![soldeer.xyz](https://img.shields.io/badge/soldeer.xyz-anoma--generic--call--forwarder-blue?logo=ethereum)](https://soldeer.xyz/project/anoma-generic-call-forwarder) [![License](https://img.shields.io/badge/license-MIT-blue)](https://raw.githubusercontent.com/anoma/generic-call-forwarder/refs/heads/main/bindings/LICENSE)

# Generic Call Forwarder Contract

The forwarder contract written in Solidity that allows generic calls to be executed via the [Anoma EVM protocol adapter](https://github.com/anoma/pa-evm).

## Prerequisites

1. Get an up-to-date version of [Foundry](https://github.com/foundry-rs/foundry) with

   ```sh
   curl -L https://foundry.paradigm.xyz | sh
   foundryup
   ```

2. Optionally, to lint the contracts, install [solhint](https://github.com/protofire/solhint) using a JS package manager such as [Bun](https://bun.com/) with

   ```sh
   curl -fsSL https://bun.sh/install | sh
   bun install
   ```

3. Optionally, for static analysis, install [Slither](https://github.com/crytic/slither) with

   ```sh
   python3 -m pip install slither-analyzer
   ```

   or brew

   ```sh
   brew install slither-analyzer
   ```

## Usage

#### Installation

Change the directory to the `contracts` folder with `cd contracts` and run

```sh
forge soldeer install
```

#### Build

To compile the contracts, run

```sh
forge build
```

#### Tests & Coverage

To run the tests, run

```sh
forge test
```

To show the coverage report, run

```sh
forge coverage
```

Append the

- `--no-match-coverage "(script|test)"` to exclude scripts, tests, and drafts,
- `--report lcov` to generate the `lcov.info` file that can be used by code review tooling.

#### Linting & Static Analysis

As a prerequisite, install the

- `solhint` linter (see https://github.com/protofire/solhint)
- `slither` static analyzer (see https://github.com/crytic/slither)

To run the linter and static analyzer, run

```sh
bunx solhint --config .solhint.json 'src/**/*.sol' && \
bunx solhint --config .solhint.other.json 'script/**/*.sol' 'test/**/*.sol' && \
slither .
```

#### Rust Bindings

To regenerate the Rust bindings (see the [forge bind](https://getfoundry.sh/forge/reference/bind/) documentation), run

```sh
forge bind \
  --select '^(GenericCallForwarder|GenericCallForwarderV2|GenericCallForwarderV3|IProtocolAdapterSpecific|ILogicRefSpecific|IEmergencyMigratable)$' \
  --bindings-path ../bindings/src/generated/ \
  --module \
  --overwrite
```

#### Documentation

Run

```sh
forge doc
```

#### Deployment

To simulate deployment on sepolia, run

```sh
forge script script/DeployGenericCallForwarder.s.sol:DeployGenericCallForwarder \
  --sig "run(bool,address,bytes32,address)" <IS_TEST_DEPLOYMENT> <PROTOCOL_ADAPTER> <CARRIER_LOGIC_REF> <EMERGENCY_COMMITTEE> \
  --rpc-url sepolia
```

Append the

- `--broadcast` flag to deploy to the network
- `--verify` flag for subsequent contract verification (Sourcify by default; set `ETHERSCAN_API_KEY` to also verify on Etherscan)
- `--slow` flag to add 15 seconds of waiting time between verification attempts
- `--account <ACCOUNT_NAME>` flag to use a previously imported keystore (see `cast wallet --help` for more info)

#### Block Explorer Verification

For post-deployment verification on **Sourcify** run

```sh
forge verify-contract \
   <ADDRESS> \
   src/GenericCallForwarder.sol:GenericCallForwarder \
   --chain sepolia \
   --verifier sourcify
```

For **Etherscan** (requires `ETHERSCAN_API_KEY`) run

```sh
forge verify-contract \
   <ADDRESS> \
   src/GenericCallForwarder.sol:GenericCallForwarder \
   --chain sepolia \
   --verifier etherscan
```
