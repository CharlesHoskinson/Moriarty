# Opus backend security completion: detail (source only)

Author: Opus 5.5 (claude-opus-5-5), security author. Date: 2026-10-14.
Status: SOURCE_WRITTEN. Nothing was compiled, formatted, tested, modelled or proved.
No approval is inferred. Full U/M/MC/SP/G remain open.

## Files

| Path | State |
|---|---|
| `crates/mori-backend/src/stage/mod.rs` | Kept as the prior job wrote it. `frame` and `context` are `pub(crate)`. `PendingPositions` is re-exported. `require_core1` and `core1_refusal` give the typed refusals. |
| `crates/mori-backend/src/stage/artifact.rs` | Kept as written. It is the strict `/4` decoder (N0/N1/A2). Fields follow A6, and the decoder is `deny_unknown_fields` plus `try_from` validation. `pending_positions` is a required nullable key. There is no conversion to `/3`. |
| `crates/mori-backend/src/stage/frame.rs` | Kept as written. It is the only emitter of the five pending roles, at fixed positions. The pre/post site constants are the entry/exit sites. Padding slots are constrained to `0`. Live post captures equal the computed wires. A false guard has no circuit. N = 0 emits no roles. It includes `#[cfg(test)]` Builder introspection. |
| `crates/mori-backend/src/stage/context.rs` | Kept as written. It is the 29-element context (C9) under `moriarty-stage-context/1`, with no compiler identity. It refuses a signed Target. |
| `crates/mori-backend/tests/stage_relation.rs` | New. Four focused tests; see below. |
| `OPUS-BACKEND-SHARED.patch` | Verbatim unified hunks for Sol. |

## Shared hunks (Sol applies; I did not edit these paths)

- `lib.rs`: `stage::require_core1` goes first in `compile`, `compile_signed` and `compile_mode`. That puts the profile refusal ahead of the target checks. The `/3` lowering match gets an explicit `Statement::Await` arm that returns `stage::core1_refusal`. Staged segment ranges skip every Await ordinal, so the arm is reached only by `/3` callers.
- `signed_relation.rs`: `admit` calls `require_core1` first and its match gets an exhaustive Await arm. `reconstruct_statement` calls `require_core1` after its parse. This covers replay `generate`, `replay` and `challenge`, which all go through `reconstruct`.
- `stage/lower.rs`: a signed Target is refused with `unsupported` before the load, before any work and before any publication directory. The CLI publication path must keep calling `compile_staged` before it reserves the output directory. I did not check that here.

## Tests (`stage_relation.rs`; DO NOT RUN under this job)

1. `pending_roles_occur_only_at_recorded_positions` (H27): pending roles appear exactly at the recorded positions. Site constants match the entry/exit, padding is `"0"`, live and event slots are unconstrained-constant, and N = 0 has no frame.
2. `contexts_are_per_circuit_and_gates_unperformed`: every circuit has a distinct context, and all ten gates are `unperformed`, including the four stage gates.
3. `decoder_refuses_forged_frames_and_fields` (H20/H21): the honest artifact decodes. Six forgeries must be refused: a wrong post-site constant, a moved pending position, a zero-await circuit claiming a frame, an absent nullable key, an unknown field, and a performed gate.
4. `legacy_compilers_refuse_staged_programs` (H18): `/3` `compile` returns the typed `mori::backend::program` refusal for moriarty/2. This depends on the shared hunk.

None of these tests use IrSource, SDK check/model, VM or proving. The mapping of tests to H numbers is my reading, not an audited classification.

## Remaining qualifiers

- Not compiled. The test file assumes these are public: `Error.code`, `PublicBinding.{role,constant}`, `Gate.{predicate,status}`, `StagedCircuit` fields, `StagedProgram::program()` and `Target::ledger9`. A mismatch is a compile-only fix.
- Test 4 fails until Sol applies the `lib.rs` hunk.
- `proving.rs:735/982` match on `Statement`. I did not check whether they have catch-alls; if not, they need an Await arm. Sol should confirm when compiling.
- The `/3` byte identity and the EMPTY whitelist are preserved: I changed no snapshots and no `/3` paths.
- The authenticated ledger binding and all ten gates are unperformed. This work draws no conclusion about host security, event authentication, stage authority or consumption.
