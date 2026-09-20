# Generic Call Resource

An [Anoma Resource Machine](https://github.com/anoma/anoma-rm-risc0) (ARM)
application that lets an Anoma transaction trigger arbitrary EVM calls.

A `GenericCall` resource encodes one or more EVM calls. When the resource is
consumed/created, the protocol adapter forwards those calls to the on-chain
`GenericCallForwarder` contract, which executes them on the target EVM. The
resource logic is a RISC0 zero-knowledge circuit that proves the forwarded
calldata matches the resource's committed `label_ref` and `value_ref`.

## How it works

A `GenericCall` resource is **always ephemeral** and binds two references:

- `label_ref = hash(forwarder_addr)` — pins the resource to a specific
  `GenericCallForwarder` contract.
- `value_ref = hash(abi_encode(calls))` — pins the resource to the exact set of
  EVM calls (`(address to, uint256 value, bytes data)[]`).

When the resource is constrained, the circuit:

1. Asserts the resource is ephemeral.
2. Recomputes `label_ref` and `value_ref` from the witness and checks them
   against the resource.
3. Computes the resource's nullifier (if consumed) or commitment (if created).
4. Emits the ABI-encoded forwarder calldata as an external payload blob (with
   `DeletionCriterion::Immediately`) for the protocol adapter to deliver to the
   forwarder contract.

## Crates

| Crate | Description |
| --- | --- |
| [`generic_call_witness`](generic_call_witness/) | `no_std`-friendly witness library: the `GenericCall` type and the `LogicCircuit` constraint logic that runs inside the zkVM guest. |
| [`generic_call_library`](generic_call_library/) | Host-side prover. Wraps the witness in `GenericCallLogic` (`LogicProver`) and embeds the compiled guest ELF and image ID. |
| [`generic_call_circuit`](generic_call_circuit/) | The RISC0 guest binary and integration tests. Building it produces the ELF and image ID consumed by `generic_call_library`. Excluded from the workspace. |

## Building and testing

The workspace crates build with the standard toolchain:

```sh
cargo build --workspace --all-targets
```

The circuit is a separate package (it needs the RISC0 toolchain) and is built
and tested from its own directory:

```sh
cd generic_call_circuit
cargo build --all-targets
cargo test -- --nocapture
```

### Regenerating the guest ELF and image ID

`generic_call_library` embeds a prebuilt guest ELF
(`generic_call_library/elf/generic-call-guest.bin`) and its image ID
(`GENERIC_CALL_ID`). After changing the witness/circuit, rebuild the guest with
the RISC0 toolchain, copy the resulting ELF into `generic_call_library/elf/`,
and update `GENERIC_CALL_ID` in [`generic_call_library/src/lib.rs`](generic_call_library/src/lib.rs)
with the printed image ID.

## Audits

Our software undergoes regular [audits](./audits/):

1. Informal Systems
   - Company Website: https://informal.systems
   - Commit
     ID: [4d561b11846fce692df49dbb73a5def97939e236](https://github.com/anoma/generic-call-resource/tree/4d561b11846fce692df49dbb73a5def97939e236)
   - Started: 2026-06-15
   - Finished: 2026-06-19
   - Last revised: 2026-07-03

   [📄 Audit Report (pdf)](./audits/2026-07-03_Informal_Systems_Generic_Call_Resource_&_Forwarder.pdf)

## Security

If you believe you've found a security issue, we encourage you to notify us via Email
at [security@anoma.foundation](mailto:security@anoma.foundation).

Please do not use the issue tracker for security issues. We welcome working with you to resolve the issue promptly.

## License

GPL-3.0
