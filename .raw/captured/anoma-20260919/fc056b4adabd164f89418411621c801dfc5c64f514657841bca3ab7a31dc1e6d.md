# `transfer_witness_v2`

The v2 witness crate for the **AnomaPay ERC20 transfer resource**. It mirrors
[`transfer_witness`](../transfer_witness) — same wrap/unwrap/transfer logic —
and adds **migration** support for moving a v1 resource to v2.

It reuses the v1 building blocks directly (`EncryptionInfo`, `LabelInfo`,
`ValueInfo`, `PermitInfo`, `ResourceWithLabel`, and the `calculate_*` helpers),
so v2 only adds what changes: the forwarder shape and the migration call type.

## What's new relative to v1

### `TokenTransferWitnessV2`
Same fields as `TokenTransferWitness`, except `forwarder_info` is replaced by
`forwarder_info_v2: Option<ForwarderInfoV2>`.

### `ForwarderInfoV2`
```rust
pub struct ForwarderInfoV2 {
    pub call_type: CallTypeV2,               // Wrap | Unwrap | Migrate
    pub ethereum_account_addr: Option<Vec<u8>>, // not needed for Migrate
    pub permit_info: Option<PermitInfo>,
    pub migrate_info: Option<MigrateInfo>,   // present only for Migrate
}
```

### `MigrateInfo`
The extra data a `Migrate` call proves about the **v1 resource being migrated**:
its `Resource`, nullifier key, a Merkle `path` from the v1 commitment tree
(proving the resource existed), the owner's authorization signature and
`ValueInfo`, and the **v1** forwarder address used in its label.

### `CallTypeV2` ([`call_type_v2.rs`](src/call_type_v2.rs))
Adds `Migrate` to `Wrap`/`Unwrap`, plus `encode_migrate_forwarder_input`, which
ABI-encodes the `MigrateV1Data` calldata (nullifier, v1 commitment-tree root, v1
logic ref, v1 forwarder address) for the EVM forwarder.

## The migration constraint

When `constrain()` runs on an ephemeral resource with `CallTypeV2::Migrate`, it
enforces (in [`src/lib.rs`](src/lib.rs)):

- migration must be triggered by a **consumed** resource;
- the migrated v1 resource is **non-ephemeral**;
- its `value_ref` matches its `ValueInfo`, and the owner's authorization
  signature over the action tree root verifies under
  `TokenTransferAuthorizationV2`;
- its quantity equals the v2 resource's quantity;
- its `label_ref` matches `sha2(v1_forwarder_addr, erc20_token_addr)`;

then it derives the migrated resource's nullifier and Merkle root and emits the
`Migrate` forwarder calldata as the external payload.

`Wrap` and `Unwrap` behave as in v1.

## Where it's used

- [`transfer_library_v2`](../transfer_library_v2) wraps this witness behind
  `TransferLogicV2` and builds the migration transaction.
- [`transfer_circuit_v2`](../transfer_circuit_v2) reads a
  `TokenTransferWitnessV2` in the guest and calls `constrain()`.

## Testing

```bash
cargo test -p transfer_witness_v2
```

See the [workspace README](../README.md) for the full picture.
