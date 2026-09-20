# Reproducibly generate proving and verifying keys (ELF and ImageID)

You may generate different ELFs and ImageIDs on different machines and environments. To reproduce the same output and publicly verify that the ELF and ImageID correspond to the transfer circuit source code, use the tool and command below.

## Reproducing the `TOKEN_TRANSFER_ID` committed in `transfer_library`

The `TOKEN_TRANSFER_ID` constant in [`transfer_library/src/lib.rs`](../transfer_library/src/lib.rs) is the `ImageID` of a specific historical build of the guest ELF — *not* of whatever `cargo risczero build` produces from this repo at HEAD (see [ImageID mismatch with `transfer_library`](#imageid-mismatch-with-transfer_library) below).

To reproduce that exact value, check out commit `ec5f9bc0466feb5abf2da5ad7d9a5c365a4d0a8f` of the `anomapay-backend` repo (this codebase's pre-multichain ancestor, where the path was `transfer_circuit/...`) and run:

```bash
cargo risczero build --manifest-path transfer_circuit/methods/guest/Cargo.toml
```

which reproduces:

```bash
ELFs ready at:
ImageID: bc12323668c37c3d381ca798f11116f35fb1639d12239b29da7810df3985e7ad
transfer_circuit/methods/guest/target/riscv32im-risc0-zkvm-elf/docker/token-transfer-guest.bin
```

The same value is also embedded as the committed guest binary at `transfer_library/elf/token-transfer-guest.bin` in this repo.

## Building from the current repo HEAD

To build the circuit from this repo at HEAD, run:

```bash
cargo risczero build --manifest-path transfer_circuit/methods/guest/Cargo.toml
```

The `ImageID` produced will generally **not** equal `TOKEN_TRANSFER_ID`. That divergence is intentional — see the next section.

## ImageID mismatch with `transfer_library`

Running `cargo risczero build` against this repo at HEAD will produce an `ImageID` that does not match the `TOKEN_TRANSFER_ID = bc12323668c37c3d381ca798f11116f35fb1639d12239b29da7810df3985e7ad` currently committed in [`transfer_library/src/lib.rs`](../transfer_library/src/lib.rs). This is expected: the committed `TOKEN_TRANSFER_ID` (and the matching `transfer_library/elf/token-transfer-guest.bin`) corresponds to the historical build reproduced in the [section above](#reproducing-the-token_transfer_id-committed-in-transfer_library), not to a fresh build of the current source tree.

### Why the values drift

The `ImageID` is a digest over the entire compiled guest binary, which transitively pins:

- the circuit source in this crate (`transfer_circuit`),
- `transfer_library` (the resource-logic definitions shared with the host/backend),
- the ARM libraries (`anoma_rm_risc0`, `anoma_rm_risc0_gadgets`, `transfer_witness`, …),
- the toolchain and any other transitive dependency.

Because `transfer_library` is shared by both the guest circuit and the host/backend, *any* change to it — including a version bump that does not touch resource-logic behavior — re-links the guest and produces a new `ImageID`. The same is true for ARM library upgrades. The circuit's own source code can be byte-for-byte identical and the `ImageID` will still change.

### Why we do not bump `TOKEN_TRANSFER_ID` on every rebuild

`TOKEN_TRANSFER_ID` is consumed off-chain (app-level checks) and on-chain (the deployed protocol adapter / verifier contracts). Rotating it requires:

1. redeploying the protocol-adapter contracts on every supported chain, and
2. updating the `ID in PA` reference used by downstream integrations.

We only pay that cost when the **circuit logic itself** changes — i.e., when the proof semantics change. Dependency or toolchain bumps that leave the logic unchanged are intentionally allowed to leave `TOKEN_TRANSFER_ID` and the committed `token-transfer-guest.bin` on the older build, even though `cargo risczero build` would now produce a different `ImageID`.

If you need the two values to agree (e.g., before a release that does change circuit logic), update both together:

1. Rebuild with the command above.
2. Replace `transfer_library/elf/token-transfer-guest.bin` with the freshly built ELF.
3. Update `TOKEN_TRANSFER_ID` in `transfer_library/src/lib.rs` to the new `ImageID`.
4. Coordinate the corresponding protocol-adapter redeployment and PA update.


Note: The unstable feature of risc0-zkvm currently causes issues in circuits. This can be temporarily fixed by manually updating the tool. The problem will be fully resolved in the next release of RISC Zero.

```bash
cargo install --force --git https://github.com/risc0/risc0 --tag v3.0.3 -Fexperimental cargo-risczero
```
