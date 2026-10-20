# Select a retained Push configuration family

Use this guide to prepare the public CI4 Push source interface with an explicit configuration family, and to interpret the related finite K context checks. This is an API example for source inspection/integration; it has not been compiled or run as a standalone example. It introduces no CLI command or native operation.

## Retain the original inputs

Keep the original `C6HostConfig`, admission `BoundsC` and `C6HostLimits` as distinct inputs. In particular, admission bounds need not equal `cfg.layout.bounds`. Select these from the invocation/configuration you intend to use; decoding a prover's metadata alone does not establish which family a verifier should accept.

The [public source](../../../crates/mori-backend/src/sherlock_machine/c6_push_step.rs) provides this constructor sequence. Here `cfg`, `admission_bounds`, `limits` and `pre` are already supplied typed values; the surrounding caller handles `C6PushStepError`.

```rust
use mori_backend::sherlock_machine::c6_push_step::{
    build_c6_push_step_circuit, C6PushFamilyParams,
    C6PushHostPrefixLanes, C6PushStatementBindingMode,
};

let family = C6PushFamilyParams::new(
    cfg.clone(), admission_bounds, limits,
)?;
let prefix = C6PushHostPrefixLanes::from_host_snapshot(
    &pre, &cfg, &limits,
)?;
let circuit = build_c6_push_step_circuit(
    family,
    prefix.meta.len(),
    prefix.prog.len(),
    prefix.meta_bounds_frames_offset,
    C6PushStatementBindingMode::LocalPublic,
    "example.push",
)?;
```

Use the complete public module path for the builder. The prefix supplies exact serialized lengths and the META admission-frames offset; it does not authenticate the selected family. The constructor retains its original inputs, derives bound META, caps/geometry and expected CTX, and refuses invalid host derivations. It does not raise caller bounds.

## Choose how to obtain the pair

For an already saved pair, `witness_from_host_pair(true, &pre, &post, &cfg, &limits)` encodes the supplied before/after observations without running the owner. For one new owner observation, `witness_from_owner_step(&admitted, &mut image, selection, &cfg, &limits)` calls the real A135/A140 recorder once. Keep that mutation and its returned recorder error distinct from a later circuit check.

A pair-derived witness retains admission bounds from its before snapshot. Compare the intended family and the retained witness inputs explicitly in the surrounding integration; the diagnostic checker does not authenticate verifier family selection. The builder's `LocalPublic`/`LedgerCall` mode selects statement plumbing, not target authorization.

## Inspect the bound composition

Follow the [reference](../reference/c6-retained-push-family-and-context.md) to check that bound META is called once, PROG/CTX parsing and expected-CTX binding share one emitter serial, and admission/layout bounds remain distinct. Preserve original absent/present-zero extension identity. Gate zero leaves the canonical/family bindings inactive even though unconditional lane ranges and pre/post equalities remain.

At the dated CI4 cut, two independent Muse opinions approve the bounded source/design/API and test source. The 24 Push/META tests and SDK path are **UNRUN**. Calling the non-proving checker would be a new SDK operation; this guide reports none. A family chosen by the caller or copied from a witness does not replace independent verifier family/invocation selection, authenticated source/IR reconstruction or native proof verification.

## Read K projection evidence at its own boundary

Use the [K result index](../../../models/sherlock/k/c6-positive-context-20261020/README.md) when inspecting the four positive configuration cases. Confirm `PASS_WHOLE_TYPED_BOTH` in the [actual result](../../results/sherlock-k-c6-positive-context-20261020/actual/ROOT-RESULT.json), which records the whole common JSON and typed AST comparison classification. The separate [final readback](../../results/sherlock-k-c6-positive-context-20261020/provenance/ROOT-ACTUAL-EXACT7-FINAL-READBACK.json) records phase, unit and barrier checks. Lane counts or a successful decoder alone are insufficient to establish the whole comparison.

The K result observes all five declared fields, with original extension presence and raw `u128` lifetime retained. It does not run the Rust codec, Push SDK checker or an owner transition. Its fields contain no K work clock; no runtime cost, degree, proof, target validation or full EARS acceptance follows. Continue through the [recursive-proof handoff guide](inspect-recursive-proof-handoff.md) only with that path's separately qualified evidence.
