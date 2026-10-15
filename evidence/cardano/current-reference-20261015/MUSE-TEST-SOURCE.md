# Verdict: PASS_TEST_SOURCE_SCOPED

This is a nonsecurity, source-only review of the proposed genuine current Moriarty `/2` staged-reference consumer tests. It validates the proposed test source and fixture design only; it makes no claim that the tests compile, run, pass, or establish ledger, runtime, proof, production, or security success.

Reviewer identity and limits: this review was performed by Meta Muse Wonder 1.0 (`muse-wonder-1.0`) at the requested high-effort setting, with no tools, execution, network access, edits, compilation, test runs, formatter runs, Cargo commands, or serializer invocation. Security judgments are reserved to Opus and are not made here. Production closure `fd44e4ff` / currentness revision 2 is treated as already source-reviewed context, not re-proven by this review.

## Scope reviewed

The proposed addition consists of:

- `proposed/crates/mori-cardano/tests/metadata-consumer/src/staged.rs` (`2b8766f3a209db110fcd21228c677f957f5ffb3a00acd6cd664417e15b0ca63b`)
- 27 byte-copied staged fixtures under `proposed/crates/mori-cardano/tests/staged-fixtures/`
- `MODULE-DECLARATION.patch` (`45a13dcd8dd39e439f6b702f7bd00557bdafa72e655b268623d9e9f9b0650269`), adding only `#[cfg(test)] mod staged;` beside the existing consumer modules
- Independent oracle freeze `4de032c53dee0d125ee2f3012b5aa1f1fd6f78be1a8b42b1d9796b32b188bb88`, which predates the proposed test source

The proposed tests are not yet applied or executed. Prior `/1` positives remain evidence about the old tree until rerun against the current production tree; this review does not upgrade them into current-tree results.

## Public API and serde wire checks

The proposed test uses the actual public staged wrapper surface:

- `prepare_staged_reference(raw_source)` to construct the reference context from `.mori` source.
- `context.metadata()` for complete staged source metadata.
- `context.start_reference(action, scenario, Some(UInt(11)))` for delivery.
- `context.start_reference(action, scenario, Some(UInt(36)))` for reserve.
- `context.resume_reference(actual_continuation, event, None, None)` for every resume.

Those signatures match the production staged wrapper basis supplied in the frozen source: `prepare_staged_reference(source: &str)`, `metadata(&self)`, `start_reference(&self, action: &str, scenario: Scenario, cap: Option<UInt>)`, and `resume_reference(&self, continuation: stage::Continuation, event: stage::StageEvent, action: Option<&str>, cap: Option<UInt>)`.

Scenario and event inputs are deserialized through their actual typed interfaces rather than through hand-built stand-ins:

- Start scenarios infer `mori_core::Scenario`.
- Resume events infer `mori_core::stage::StageEvent`.
- Returned pending values are used as typed `mori_core::stage::Continuation` values.
- Outcomes remain typed `mori_core::stage::StageOutcome` values until serialized whole for comparison.

The wire variants match the frozen source. `StageOutcome` is an externally tagged serialized enum, so `{"Waiting": {...}}` and `{"Complete": {...}}` are the correct expected top-level shapes. `StageSegment`, `Execution`, `Step`, `Statement`, `Expr`, `Effect`, `Value`, `UInt`, `Continuation`, `StageLogical`, `PendingFrame`, and plan/capture structures serialize with the field names used in the fixtures. In particular, `UInt` serializes as canonical decimal strings, `Value` uses `{"type": ..., "value": ...}`, statements and expressions use `kind` tagging, effects use `kind` tagging, and capture origins serialize as `Input`, `Let`, and `Event`.

The tests correctly avoid deserializing `/2` `Program` or `Execution` through the `/1`-only shared deserialization route. Metadata, program, and plan are compared as complete serialized JSON values. That is the right boundary because the shared `/1` decode path intentionally rejects the `Await` statement form.

No hidden helper stand-in was found. The helpers only parse frozen fixture text into `serde_json::Value`, extract the returned `Continuation` from an actual `StageOutcome::Waiting`, compare that actual continuation against the independent continuation fixture, and return the actual continuation for the next call. They do not synthesize expected outputs from actual results, do not rebuild a continuation from fixture JSON, do not call the production implementation to generate goldens, and do not invoke the oracle serializer.

## Complete expected-field consumption

The proposed assertions consume complete expected fields rather than selected summaries:

- `metadata()` compares the full serialized `StagedSourceMetadataV1` value against the complete expected metadata JSON, then additionally compares the nested `program` and `plan` values against their separate complete frozen files.
- `assert_outcome()` serializes each successful `StageOutcome` whole and compares it to the complete independent outcome JSON without removing fields.
- `continuation()` serializes each actual pending continuation whole and compares it to the complete independent continuation fixture before returning it.

Because comparisons are `serde_json::Value` equality, object key ordering is not part of the predicate, but exact field membership, all scalar values, numeric/string representations, and array ordering are included. This covers state, balances, liabilities, captures, frame pre/post values, execution pre/post snapshots, per-step original statement ordinals, per-step snapshots, per-step effects, aggregate effects, residuals, attempted work, committed work where present, labels, hashes, cost profile, program structure, plan sites, segment costs, committed-before vectors, and plan totals.

## Chained continuation behavior

The chained-call requirement is satisfied by construction.

Delivery does:

1. start with cap `11`;
2. assert the complete start outcome;
3. extract and fully compare the actual returned continuation;
4. pass that same actual continuation into the false resume with `None` action and `None` cap;
5. assert the complete false-wait outcome;
6. extract and fully compare the actual returned unchanged continuation;
7. pass that same actual continuation into the true resume;
8. assert the complete completion outcome.

Reserve does:

1. start with cap `36`;
2. assert the complete start outcome;
3. pass the actual site-1 continuation into the first false resume;
4. assert the complete first false-wait outcome and pass its actual returned continuation onward;
5. pass that actual continuation into the first true resume reaching site 2;
6. assert the complete first-true waiting outcome;
7. pass the actual site-2 continuation into the second false resume;
8. assert the complete second false-wait outcome and pass its actual returned continuation onward;
9. pass that actual continuation into the second true resume;
10. assert the complete completion outcome, including the final `Emit`.

These are chained actual-returned continuations, not independent fixture-injected replays. The fixture continuation is used only as the expected comparison value. If the actual continuation differed, the test fails before resume instead of substituting the frozen continuation.

The fixed start caps `11` and `36`, start actions `deliver` and `allocate`, and null resume action/cap are exactly the invocation contract in `INVOCATIONS.json`. The eight outcome files correspond to the eight frozen invocations: three delivery outcomes and five reserve outcomes.

## Oracle arithmetic and schema review

I checked the oracle derivations against the frozen production source basis, including parser expression indexing, plan capture construction, stage segment accounting, runtime snapshot/effect construction, false-guard behavior, residual filtering, and wrapper metadata field names.

Delivery arithmetic is internally correct:

- Expressions are five name nodes in statement order.
- Start work is `2 + 2 + 1 = 5`.
- True completion resume work is `2 + 2 + 2 = 6`.
- Plan total is `11`; committed-before is `[0, 5]`.
- The funding transfer moves alice `100 -> 90` and escrow `0 -> 10`.
- The `release` liability goes `0 -> 10`, then after transfer and discharge goes `10 -> 0`.
- The discharged duty remains present at zero in the final snapshot while residual is empty because residual filters positive liabilities only.
- The false resume attempts `2`, publishes no segment, preserves committed `5`, preserves the continuation, and leaves residual `release = 10`.
- Final balances are alice `90`, escrow `0`, bob `10`, observer `42`; asset total remains `142`.

Reserve arithmetic is internally correct:

- The manual expression table has 22 nodes and matches parser append order, including binary operands before their operator and no state-initial literals in the expression array.
- Start segment cost is `4 + 4 + 2 + 1 = 11`.
- First true segment cost is `4 + 2 + 2 + 2 + 4 + 1 = 15`.
- Final segment cost is `2 + 4 + 2 + 2 = 10`.
- Plan total is `36`; committed-before is `[0, 11, 26]`.
- `held` is captured as `42`, `basis` becomes `35`, and `allocation` becomes `5 + 42 = 47`.
- First false approval attempts `4` and preserves committed `11`.
- First true approval records source ordinal `3`, sets `allocated = 42`, transfers `42`, discharges allocation `47 -> 5`, and sets `basis = 35 + 43 = 78`.
- Site-2 captures preserve `extra = 2`, `held = 42`, and `quote = 43` in that order; `held` is not recomputed from the later `basis`.
- Second false settlement attempts `2` and preserves committed `26`.
- Final completion charges fee `2`, emits `settled_quote = 43`, leaves reserve `56`, target `42`, treasury `2`, observer `42`, and keeps `allocation` present at `5` in both snapshot and residual.
- Liability reconciliation is opening `5` + accrued `42` - discharged `42` = closing `5`.
- Reserve gross debit is `42 + 2 = 44`; unspent reserve units are `56`, correctly described as a source balance rather than measured protocol dust or service fees.

Plan captures also match the source rule: inputs come first in declaration order, lets append with source statement ordinal, and event binders append only after their defining site layout has been recorded. Delivery site 1 captures only `amount`; reserve site 1 captures `extra` and `held`; reserve site 2 adds `quote` with event origin from the approval await. Guard steps retain original source statement ordinals and unchanged pre-boundary snapshots; await entry has no step; completion frames return to idle site `0` with empty captures.

The expected metadata and outcome files consistently repeat the same delivery `program_hash` / `plan_hash` pair and the same reserve pair across metadata, continuations, and outcomes. Under the no-tools constraint of this review, I did not recompute SHA-256 digests or rerun the serializer; the hash values were reviewed for internal consistency against the frozen provenance material supplied, and the oracle records state that the serializer was copied as provenance and not invoked for the proposed test. The falsifiable design remains complete JSON equality against those frozen values once an authorized run occurs.

## Required limits on the verdict

This pass is scoped only to the proposed test source and fixtures as specified. It does not establish that the consumer compiles, that the fixture directory placement is integrated, that the module declaration has been applied, that any test passes, that production behavior matches the oracle at runtime, that the production 95-file closure still has a runtime result on the current tree, or that any `/1`, staged, CLI, native, proof, Preprod, custody, authentication, consent, ledger, or security predicate holds.

Before any execution, the already-required Opus current-tree permission remains pending. After authorized integration, the tests still need to be run before any pass/fail result can be claimed.

Final verdict: **PASS_TEST_SOURCE_SCOPED**.