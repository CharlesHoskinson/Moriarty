# C6 ValidateField: source reference

This is uncompiled source for the experimental `sherlock` branch. It is not a release artifact or proof of execution. **RawSDK20 remains UNCOMPILED_UNRUN**: the eight ordinary and twelve raw tests have not been compiled or run for this source publication.

## API reference

The module covers the handwritten `BHelperValidateField` step. Its [emitter](../../../crates/mori-backend/src/sherlock_machine/c6_validate_field_step.rs) and [consumer](../../../crates/mori-backend/src/sherlock_machine/c6_validate_field_consumer.rs) expose these interfaces through `sherlock_machine`:

| Interface | Source role |
|---|---|
| `C6ValidateFieldStepEmitter::emit` and `emit_c6_validate_field_step_block` | Produce `C6ValidateFieldStepBlock`, containing instructions and receipt operands. |
| `witness_validate_field_from_host_pair` | Serialize independent pre/post host snapshots and derive disposition lanes from the retained admission bounds. |
| `witness_validate_field_from_owner_record` | Validate the record's internal event/snapshot consistency, then construct a witness. |
| `record_validate_field_step` | Call the fixed owner recorder once, then construct the witness. An error can leave the image advanced; rollback is not guaranteed. |
| `build_c6_validate_field_step_circuit` and `check_c6_validate_field_step_circuit` | Build the statement/IR consumer and provide the SDK `ProofPreimage::check` preprocessing route. |

The binding modes are `C6PushStatementBindingMode::LocalPublic` and `LedgerCall`. The returned type is `C6ValidateFieldStepBlock`; its constant-field operands are `const_field_count`, `const_field_found`, and `const_field_value`.

## Why the source is separate from validation

The host-pair constructor accepts independent post snapshots. The owner-record constructor performs additional host-side consistency checks. The recording wrapper first invokes the owner; these three routes have different responsibilities. Receipt operands also have documented gating conditions. Use the actual module contracts when composing another step.

Scoped source reviews recorded `APPROVE_SOURCE_WITH_LIMITS` with zero mandatory findings for the reviewed modules and their parent registration. The four Field modules are unchanged, and the parent adds only their module/test declarations and public reexports. This publication includes no registration for unpublished ScalarConstruct, ValidateEntry, or ValidateOccurrence backend modules.

Those source opinions establish no compilation or SDK/circuit-check result. Whole-Push composition, StepInv and formal correspondence, financial requirements, native proofs, circuit degree/performance, recursive proofs, stagenet settlement, release readiness, and EARS acceptance remain open.

## Select the future twenty tests

The [ordinary test module](../../../crates/mori-backend/src/sherlock_machine/c6_validate_field_step_tests.rs) contains eight written `#[test]` functions. The [raw test module](../../../crates/mori-backend/src/sherlock_machine/c6_validate_field_consumer_raw_tests.rs) contains twelve. A separately authorized future build can select those modules with:

```sh
cargo test -p mori-backend sherlock_machine::c6_validate_field_step_tests:: -- --list
cargo test -p mori-backend sherlock_machine::c6_validate_field_consumer_raw_tests:: -- --list
```

These commands require compilation even with `--list`; neither was executed for this publication. The counts describe source declarations, not observed test discovery or passes. Future execution needs its own exact-source receipt, actual command/exit and scoped interpretation of the result.
