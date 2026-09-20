[![Contracts Tests](https://github.com/anoma/generic-call-forwarder/actions/workflows/contracts.yml/badge.svg)](https://github.com/anoma/generic-call-forwarder/actions/workflows/contracts.yml) [![soldeer.xyz](https://img.shields.io/badge/soldeer.xyz-anoma--generic--call--forwarder-blue?logo=ethereum)](https://soldeer.xyz/project/anoma--generic--call) [![License](https://img.shields.io/badge/license-MIT-blue)](https://raw.githubusercontent.com/anoma/generic-call-forwarder/refs/heads/main/contracts/LICENSE)

[![Bindings Tests](https://github.com/anoma/generic-call-forwarder/actions/workflows/bindings.yml/badge.svg)](https://github.com/anoma/generic-call-forwarder/actions/workflows/bindings.yml) [![crates.io](https://img.shields.io/badge/crates.io-anoma--generic--call--forwarder--bindings-blue?logo=rust)](https://crates.io/crates/anoma-generic-call-forwarder-bindings) [![License](https://img.shields.io/badge/license-MIT-blue)](https://raw.githubusercontent.com/anoma/anoma-generic-call-forwarder/refs/heads/main/bindings/LICENSE)

# Generic Call Forwarder

A forwarder contract written in Solidity allowing generic EVM calls to be triggered by resources using the [Anoma EVM protocol adapter](https://github.com/anoma/pa-evm).

## Project Structure

This monorepo is structured as follows:

```
.
├── audits
├── bindings
├── contracts
├── Cargo.lock
├── Cargo.toml
├── README.md
└── RELEASE_CHECKLIST.md
```

The [contracts](./contracts/) folder contains the contracts written in [Solidity](https://soliditylang.org/) as well as [Foundry forge](https://book.getfoundry.sh/forge/) tests and deploy scripts.

The [bindings](./bindings/) folder provides [Rust](https://www.rust-lang.org/) bindings for the conversion of Rust and [RISC Zero](https://risczero.com/) types into [EVM types](https://docs.soliditylang.org/en/latest/types.html) and exposes the deployment addresses on the different supported networks using the [alloy-rs](https://github.com/alloy-rs)
library.

## Audits

Our software undergoes regular [audits](./audits/):

1. Informal Systems
   - Company Website: https://informal.systems
   - Commit
     ID: [64c364974b51c31dabb5371e89762c037c9790bb](https://github.com/anoma/generic-call-forwarder/tree/64c364974b51c31dabb5371e89762c037c9790bb)
   - Started: 2026-06-15
   - Finished: 2026-06-19
   - Last revised: 2026-07-03

   [📄 Audit Report (pdf)](./audits/2026-07-03_Informal_Systems_Generic_Call_Resource_&_Forwarder.pdf)

## Security

If you believe you've found a security issue, we encourage you to notify us via Email
at [security@anoma.foundation](mailto:security@anoma.foundation).

Please do not use the issue tracker for security issues. We welcome working with you to resolve the issue promptly.
