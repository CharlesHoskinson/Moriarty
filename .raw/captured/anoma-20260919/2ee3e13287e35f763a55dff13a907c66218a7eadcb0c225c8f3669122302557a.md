# AnomaPay ERC20 Resource

Resource logics and zero-knowledge circuits for the **AnomaPay ERC20 transfer
resource**, targeting the [Anoma Resource Machine (ARM)](https://anoma.net) on
the [RISC Zero](https://dev.risczero.com) zkVM.

This repository packages the witness types, resource-logic library, and RISC
Zero guest programs that prove the validity of token-transfer resources. It was
extracted from the AnomaPay backend into a standalone workspace so the proving
artifacts (guest ELFs, `ImageID`s, and the Rust APIs around them) can be
versioned and reused independently of the backend services that consume them.

There are two generations of the resource:

- **v1** — the original token-transfer resource (wrap / unwrap / transfer).
- **v2** — adds **migration** support (migrating a v1 resource to v2) on top of
  the v1 capabilities.

## Layout

Crates live at the repository root. The witness and library crates form one
Cargo workspace; each circuit is its own **excluded** workspace because RISC
Zero guests are cross-compiled to the `riscv32im-risc0-zkvm-elf` target and must
not share the host workspace's lockfile or profile.

```
.
├── transfer_witness/        # v1 witness data + resource-logic constraints
├── transfer_witness_v2/     # v2 witness data (depends on v1)
├── transfer_library/        # v1 host API: TransferLogic, guest ELF + ImageID
├── transfer_library_v2/     # v2 host API: TransferLogicV2 + migration tx builder
├── transfer_circuit/        # v1 RISC Zero guest program  (excluded workspace)
└── transfer_circuit_v2/     # v2 RISC Zero guest program  (excluded workspace)
```

### Dependency graph

```
transfer_witness ─────────────┬─────────────► transfer_library ───► transfer_circuit
        ▲                      │                      ▲                     │
        │                      │                      │                     │ (path deps,
        └── transfer_witness_v2 ──► transfer_library_v2 ──► transfer_circuit_v2
                                                            (guest embeds the library ELF)
```

- `transfer_witness` is the leaf; everything else builds on it.
- The `*_library` crates **embed the prebuilt guest ELF** (`include_bytes!`) and
  expose the matching `ImageID`, so a host can verify proofs without rebuilding
  the guest.
- The `*_circuit` crates are the guest *sources* — used to (re)produce those
  ELFs — and depend on the witness/library crates by relative path.

## Crates

### `transfer_witness`
Defines `TokenTransferWitness`: the full set of inputs needed to prove the
resource logic of a single consumed or created resource (the resource itself,
nullifier key, authorization signature, encryption info, forwarder/permit data,
label and value info). Implements the ARM `LogicCircuit` constraint function
that the guest executes, plus the Solidity-ABI (`alloy-sol-types`) encoding for
the EVM forwarder `Wrap`/`Unwrap` calldata. Authorization domain:
`TokenTransferAuthorization`.

### `transfer_witness_v2`
`TokenTransferWitnessV2` — the v2 witness, reusing the shared v1 building blocks
(`EncryptionInfo`, `LabelInfo`, `ValueInfo`, `PermitInfo`, …) and adding
`ForwarderInfoV2` with `MigrateInfo` to support migrating a v1 resource to v2.
Authorization domain: `TokenTransferAuthorizationV2`.

### `transfer_library`
Host-side API for v1 proving. `TransferLogic` wraps a witness and constructs
resource-logic proofs. Exposes:
- `TOKEN_TRANSFER_ELF` — the embedded guest binary
  (`elf/token-transfer-guest.bin`).
- `TOKEN_TRANSFER_ID` — the guest `ImageID` (a `Digest`) used for verification
  on- and off-chain.

### `transfer_library_v2`
v2 host-side API. `TransferLogicV2` plus `migrate_tx::construct_migrate_tx`,
which assembles a complete ARM `Transaction` (compliance units, logic proofs,
delta proof) that migrates a v1 resource to v2. Exposes `TOKEN_TRANSFER_V2_ELF`
and `TOKEN_TRANSFER_V2_ID` (`elf/token-transfer-guest-v2.bin`).

### `transfer_circuit` / `transfer_circuit_v2`
The RISC Zero guest programs (`token-transfer` / `token-transfer-v2`). Each
`methods/guest` main simply reads a witness, runs `witness.constrain()`, and
commits the resulting `LogicInstance`:

```rust
let witness: TokenTransferWitness = env::read();
let instance = witness.constrain().unwrap();
env::commit(&instance);
```

Their integration tests (`src/test.rs`) exercise wrap/unwrap/transfer (and, for
v2, migration) end to end. See each crate's `README.md` for how to reproducibly
build the ELF and `ImageID` with `cargo risczero build`.

## The guest ELF / `ImageID` model

`TOKEN_TRANSFER_ID` (and `TOKEN_TRANSFER_V2_ID`) is the `ImageID` of a **specific
committed build** of the guest ELF — not necessarily of a fresh
`cargo risczero build` at HEAD. The `ImageID` is a digest over the entire
compiled guest, so it changes whenever *any* transitive input changes
(`transfer_library`, the ARM libraries, the toolchain), even if the circuit
logic is byte-for-byte identical.

Because `TOKEN_TRANSFER_ID` is consumed on-chain (verifier / protocol-adapter
contracts) and off-chain, it is rotated **only when the proof semantics
change** — not on incidental dependency or toolchain bumps. When you do need to
rotate it, rebuild the guest, replace the committed `elf/*.bin`, update the
`ImageID` constant in the library crate, and coordinate the corresponding
contract redeployment. Full details are in
[`transfer_circuit/README.md`](transfer_circuit/README.md).

## Building and testing

Requires the toolchain pinned in [`rust-toolchain.toml`](rust-toolchain.toml)
(Rust **1.95**). Building the guests additionally needs the RISC Zero toolchain
(`cargo risczero`); see the circuit READMEs.

```bash
# Workspace crates (witness + library)
cargo build --workspace --all-targets
cargo test  --workspace

# Each circuit is a separate workspace — build/test from its own directory.
# RISC0_DEV_MODE=1 skips real proving so tests run fast (dev/CI only — it does
# NOT produce verifiable proofs).
cd transfer_circuit    && RISC0_DEV_MODE=1 cargo test -- --nocapture && cd ..
cd transfer_circuit_v2 && RISC0_DEV_MODE=1 cargo test -- --nocapture && cd ..
```

> The dev profile sets `opt-level = 3` for the whole workspace: running an
> unoptimized guest is dramatically slower.

## Continuous integration

[`.github/workflows/ci.yml`](.github/workflows/ci.yml) runs on pushes to `main`
and on pull requests:

- **Format** — `cargo fmt --all --check` for the workspace and each circuit
  (`rustfmt` `style_edition = "2024"`), plus `taplo fmt --check` for all
  `Cargo.toml`s.
- **Build & test** — `cargo build --workspace --all-targets`,
  `cargo test --workspace`, and a build + `RISC0_DEV_MODE=1` test of each
  circuit.

## Audits

Our software undergoes regular audits:

1. Informal Systems

   - Company Website: https://informal.systems
   - Commit ID: [03e60b64d9dc3845c55e34d1d0bef25392cb5b60](https://github.com/anoma/anomapay-erc20-forwarder/tree/03e60b64d9dc3845c55e34d1d0bef25392cb5b60)
   - Started: 2025-12-01
   - Finished: 2025-12-16
   - Last revised: 2025-12-19

   [📄 Audit Report (pdf)](./audits/2025-12-19_Informal_Systems_AnomaPay_Phase_I.pdf)

## Versioning

The workspace crates (`transfer_witness*`, `transfer_library*`) share the
version in `[workspace.package]` (currently `2.0.0`). The circuit crates are
versioned independently (currently `2.0.0`).

## License

GPL-3.0.
