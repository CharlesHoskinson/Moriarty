# generic_call_library

Host-side prover for the `GenericCall` resource logic in the
[Generic Call Resource](https://github.com/anoma/generic-call-resource)
application.

This crate wraps [`generic_call_witness`](../generic_call_witness/) in a
`GenericCallLogic` type that implements `anoma-rm-risc0`'s `LogicProver`, and
embeds the compiled RISC0 guest ELF and its image ID so transactions can
generate resource-logic proofs.

## What it provides

- **`GenericCallLogic`** — wraps a `GenericCallWitness` and implements
  `LogicProver` (exposing the guest ELF as the proving key and the image ID as
  the verifying key).
- **`GenericCallLogic::consumed_ephemeral_resource_logic`** — build the logic
  for a consumed ephemeral resource (requires a nullifier key).
- **`GenericCallLogic::created_ephemeral_resource_logic`** — build the logic for
  a created (balancing) ephemeral resource.
- **`GENERIC_CALL_ELF`** / **`GENERIC_CALL_ID`** — the embedded guest binary and
  its image ID.
- Re-exports **`GenericCall`** from `generic_call_witness` for convenience.

## Example

```rust
use generic_call_library::{GenericCall, GenericCallLogic};

let logic = GenericCallLogic::consumed_ephemeral_resource_logic(
    resource,
    action_tree_root,
    nf_key,
    forwarder_addr,
    vec![GenericCall {
        to: target_addr.to_vec(),
        value: 0,
        data: calldata,
    }],
);
// `logic` implements `LogicProver` and can be used to generate a proof.
```

## Regenerating the guest ELF and image ID

The embedded ELF (`elf/generic-call-guest.bin`) and `GENERIC_CALL_ID` come from
building [`generic_call_circuit`](../generic_call_circuit/) with the RISC0
toolchain. After changing the witness or circuit, rebuild the guest, copy the
output ELF into `elf/`, and update `GENERIC_CALL_ID` in
[`src/lib.rs`](src/lib.rs) with the printed image ID.

## License

GPL-3.0
