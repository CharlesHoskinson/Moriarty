# `transfer_witness`

The v1 witness crate for the **AnomaPay ERC20 transfer resource**. It defines
the data a prover feeds into the RISC Zero guest and the resource-logic
constraint function the guest runs over it. This is the leaf crate of the
workspace —  `transfer_library`, and the circuits all
build on it.

It is `no_std`-friendly (depends on `anoma-rm-risc0` with `default-features =
false`) so the same constraint code compiles for both the host and the guest.

## What it provides

### `TokenTransferWitness`
The full set of inputs needed to prove the resource logic of a **single**
consumed or created resource:

| field | purpose |
| --- | --- |
| `resource` | the ARM `Resource` this witness is about |
| `is_consumed` | whether the resource is consumed (nullified) or created (committed) |
| `action_tree_root` | the action tree root that authorization signs over |
| `nf_key` | nullifier key (required for consumed resources) |
| `auth_sig` | authorization signature (required for consumed persistent resources) |
| `encryption_info` | keys/nonce for the resource + discovery ciphertexts |
| `forwarder_info` | EVM forwarder call data (`Wrap`/`Unwrap` + permit) |
| `label_info` | forwarder address + ERC20 token address (defines the label) |
| `value_info` | authorization + encryption public keys (defines the value) |

### `LogicCircuit` implementation
`TokenTransferWitness::constrain()` is the function the guest executes. It
branches on the resource kind and enforces the corresponding rules:

- **Ephemeral resource** → validates the label, builds the EVM forwarder
  `Wrap` (consumed) or `Unwrap` (created) calldata, and emits it as the
  `external_payload`.
- **Consumed persistent resource** → verifies the authorization signature over
  the action tree root under the `TokenTransferAuthorization` domain.
- **Created persistent resource** → validates the label, encrypts the resource
  payload for the recipient, and emits the resource + discovery ciphertexts.

It returns the `LogicInstance` (tag, `is_consumed`, root, app data) the guest
commits.

### Solidity ABI encoding ([`call_type.rs`](src/call_type.rs))
`CallType` (`Wrap`/`Unwrap`) plus `encode_wrap_forwarder_input` /
`encode_unwrap_forwarder_input`, which `alloy-sol-types`–encode the EVM
forwarder calldata (including the Permit2 `WrapData`) that the ARM places in the
action's external payload.

## Where it's used

- [`transfer_library`](../transfer_library) wraps this witness behind
  `TransferLogic` to build proofs on the host.
- [`transfer_circuit`](../transfer_circuit) reads a `TokenTransferWitness` in
  the guest and calls `constrain()`.

## Testing

```bash
cargo test -p transfer_witness
```

The unit tests in [`src/call_type.rs`](src/call_type.rs) exercise the forwarder
calldata encoding.

See the [workspace README](../README.md) for the full picture.
