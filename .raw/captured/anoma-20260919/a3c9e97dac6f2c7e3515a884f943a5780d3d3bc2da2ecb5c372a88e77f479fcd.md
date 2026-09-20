# `transfer_library_v2`

The v2 host-side proving API for the **AnomaPay ERC20 transfer resource**. It is
the migration-capable counterpart of [`transfer_library`](../transfer_library):
same wrap/unwrap/transfer constructors, plus everything needed to **migrate a v1
resource to v2** — including a ready-made transaction builder.

It wraps [`transfer_witness_v2`](../transfer_witness_v2) (and reuses v1 witness
types) behind the ARM `LogicProver` trait.

## What it provides

### `TransferLogicV2`
A wrapper around a `TokenTransferWitnessV2` implementing `LogicProver`. It
mirrors `TransferLogic`'s constructors —
`consume_persistent_resource_logic`, `create_persistent_resource_logic`,
`mint_resource_logic_with_permit`, `burn_resource_logic` — and adds:

- **`migrate_resource_logic`** — builds the consumed-side logic for a migration:
  the v2 (ephemeral) resource being consumed plus the `MigrateInfo` describing
  the v1 resource being migrated (its resource, nullifier key, Merkle path, auth
  signature/keys, and the v1 forwarder address).

### `migrate_tx::construct_migrate_tx` ([`src/migrate_tx.rs`](src/migrate_tx.rs))
Assembles a complete, balanced ARM `Transaction` that migrates a v1 resource to
v2. Given the consumed, migrated, and created resource parameters it:

1. builds the action tree from the consumed nullifier and created commitment,
2. creates the compliance unit (Groth16),
3. proves the consumed-side (`migrate_resource_logic`) and created-side
   (`create_persistent_resource_logic`) resource logics,
4. assembles the action and generates the delta proof,

returning a verifiable `Transaction`.

### Embedded guest + image id
- `TOKEN_TRANSFER_V2_ELF` — the guest binary, embedded via `include_bytes!`
  from [`elf/token-transfer-guest-v2.bin`](elf/token-transfer-guest-v2.bin).
- `TOKEN_TRANSFER_V2_ID` — the guest `ImageID` (a `Digest`,
  `7da9a32dd1c2822fa7507bef6876354a6df81656a177fbe7e2980298bbc1f6c7`) used to
  verify proofs on- and off-chain.

> [!IMPORTANT]
> `TOKEN_TRANSFER_V2_ID` pins a **specific committed build** of the guest, not
> whatever `cargo risczero build` produces at HEAD. It is consumed by deployed
> contracts, so it is rotated only when the proof semantics change. See
> [`transfer_circuit_v2/README.md`](../transfer_circuit_v2/README.md) for how to
> reproduce it and when/how to update it.

## Testing

```bash
cargo test -p transfer_library_v2
```

`simple_migrate_test` in [`src/migrate_tx.rs`](src/migrate_tx.rs) builds and
verifies a migration transaction end to end. It is gated off on macOS and does
real proving — run it with `RISC0_DEV_MODE=1` for a fast (non-verifiable) pass.

See the [workspace README](../README.md) for the full picture.
