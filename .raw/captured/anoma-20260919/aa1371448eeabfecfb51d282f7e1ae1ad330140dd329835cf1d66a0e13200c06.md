# `transfer_library`

The v1 host-side proving API for the **AnomaPay ERC20 transfer resource**. It
wraps [`transfer_witness`](../transfer_witness) behind the ARM `LogicProver`
trait and ships the prebuilt guest so hosts can prove and verify without
rebuilding the circuit.

## What it provides

### `TransferLogic`
A thin wrapper around a `TokenTransferWitness` that implements
`LogicProver` (`proving_key`, `verifying_key`, `witness`), so calling
`.prove(...)` produces a resource-logic proof. Besides the generic `new(...)`,
it offers constructors for each operation, so callers don't have to assemble the
witness fields by hand:

| constructor | resource produced |
| --- | --- |
| `consume_persistent_resource_logic` | consume an existing persistent resource (verifies the owner's auth signature) |
| `create_persistent_resource_logic` | create a persistent resource (encrypts payload for the recipient) |
| `mint_resource_logic_with_permit` | ephemeral **wrap** (mint) leg, carrying the Permit2 signature |
| `burn_resource_logic` | ephemeral **unwrap** (burn) leg |

### Embedded guest + image id
- `TOKEN_TRANSFER_ELF` — the guest binary, embedded via `include_bytes!` from
  [`elf/token-transfer-guest.bin`](elf/token-transfer-guest.bin).
- `TOKEN_TRANSFER_ID` — the guest `ImageID` (a `Digest`,
  `bc12323668c37c3d381ca798f11116f35fb1639d12239b29da7810df3985e7ad`) used to
  verify proofs on- and off-chain.

> [!IMPORTANT]
> `TOKEN_TRANSFER_ID` pins a **specific committed build** of the guest, not
> whatever `cargo risczero build` produces at HEAD. It is consumed by deployed
> contracts, so it is rotated only when the proof semantics change — not on
> incidental dependency or toolchain bumps. See
> [`transfer_circuit/README.md`](../transfer_circuit/README.md) for how to
> reproduce it and when/how to update it.

Unlike the witness crate, this crate builds `anoma-rm-risc0` with the
`transaction` and `prove` features — it is a **host** dependency, not part of
the guest.

## Testing

```bash
cargo test -p transfer_library
```

See the [workspace README](../README.md) for the full picture and
[`transfer_library_v2`](../transfer_library_v2) for the migration-capable
version.
