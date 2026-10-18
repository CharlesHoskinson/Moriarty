# One new complete reserve-settlement comparison

Status: specified-only, missing independent complete expected artifacts. This is an increment on the existing `crates/mori-core/tests/continuations.rs` consumer, not a standalone verifier, new model, scheduler or proof.

## Existing source and invocation

Use the exact `examples/core/continuations/reserve.mori` at main `588d811dbbd9e88d22d3ebe90b0544672870caa0`, SHA-256 `260f50443d31e58f62d83c8f7f00f3c110e3c116d216feb93f680f0b92224530`. Use original opening `extra=UInt2`, allocation duty5, complete balance rows observer42/reserve100/target0/treasury0, state basis40/allocated0/marker77. Preserve original cap36, source plan segment costs11/15/10, and action `allocate`; do not borrow PR212's no-cap assumptions.

Call the actual `stage::load`/`stage::start` and pass its actual continuation into `stage::resume` with original `approval` event quote43. Pass that actual resulting second-boundary continuation into `stage::resume` with original `settlement` event ready:true. Do not synthesize a continuation from expected JSON or reload a model-produced state as the Rust input. Starting/approval are necessary input construction for the newly checked final suffix, not a campaign to refresh their already recorded passing prefix counts.

The concrete consumer check is `stage::outcome_report(Ok(completed))` against a new independent full expected report. Retain the typed segment before rendering to compare the actual incoming frame/boundary to its `frame_pre`/`execution.pre`. A proposed test name is `reserve_settlement_complete_report_matches_independent_finite_projection` in the existing test file.

## Independent input custody before implementation

A fresh non-author must derive and freeze these artifacts from the source, adopted normative observation/M01 clauses and public DTO definitions, with exact source and derivation pins. It must not run or import the parser, evaluator, model, candidate adapter or source computation to generate expected values.

1. Full Program for the exact entire source, with declaration order and fresh whole-source expression allocation. Include every raw Statement and numeric ExprId; do not retain only selected roots or substitute prose `source_statement`.
2. Filled StagePlan and applicable existing local identity values under the separately adopted M03 premise. Preserve the owner text; this task makes no new identity/authentication judgment.
3. Complete second-boundary and final ordinary report JSON. Include root qualifications and exact key sets; complete segment action and execution action/semantics; typed frame_pre/frame_post; raw Step.statement, typed value, index, complete snapshot and delta for every suffix step; complete pre/post/effects and residual rows; attempted/committed UInt strings.
4. Complete independently expected finite model `Observation` after the direct two-event path, and a field-by-field model/Rust projection table with all static financial identities supplied by the frozen source. Explicitly list Rust fields absent from the model, checking those against the concrete independent report rather than pretending the model specified them.

`INHERITED-EXPECTATIONS.json` supplies original independent arithmetic and conceptual step facts, not the missing artifacts. Its first and final rows must not be changed to match future actual output. Source-exposed prior design is disclosed in the adopted record; implementation output still cannot become independent expected data.

## Expected finite suffix facts already specified

The pending input frame has site2 and ordered typed captures extra:UInt2, held:UInt42, quote:UInt43. It has balances observer42/reserve58/target42/treasury0; state allocated42/basis78/marker77; and a present allocation liability5 (CREDIT asset, reserve debtor, target creditor). The site and ordered capture names/types are explicit source-static data, not interchangeable with the model's `[2,42,43]` number list.

| Source ordinal | Executed statement | Typed result | New ordered delta | Complete snapshot |
| --- | --- | --- | --- | --- |
| 8 | true resumed Await settlement | Bool true | [] | Complete second-boundary snapshot |
| 9 | requires quote≤50 | Bool true | [] | Same complete snapshot |
| 10 | fee CREDIT reserve treasury extra | UInt2 | One exact Fee(CREDIT,reserve,treasury,2) | observer42/reserve56/target42/treasury2; all state/liability rows preserved |
| 11 | emit settled_quote(quote) | UInt43 | One exact Emit(settled_quote,UInt43) | Same complete post-fee snapshot |

The row describes inherited semantics; raw IR objects/ExprIds remain to be independently frozen. Execution.effects must be exactly `[Fee2,Emit43]` in that order with full identifiers. Final `frame_post` is site0/captures[]; local frames are not padded circuit ABI slots. Root status is complete, attempted:"10", committed:"36", with complete residual allocation5. Root continuation and advanced members are absent for this complete result; qualification:"local-reference-stage", authentication:"none", consumption:"untracked" remain explicit.

Suffix financials are gross debit2, fees2, net reserve−2/treasury+2/target0/observer0, liability opening5/closing5 with no creation/accrual/discharge, dust0; remaining reserve56 is custody, not dust. Across the direct entire path, gross debit44, transfers42, fee2, net reserve−44/target+42/treasury+2/observer0, original liability5 + accrued42 − discharged42 = residual5. Preserve complete affected and protected state and asset `(local,continuation-reserve-oracle,UNIT,0)`.

Direct-path cumulative attempted work36 equals committed36, while final invocation attempted10 remains separate. The reference logical profile is `moriarty-reference-logical-cost/2`; these quantities say nothing about circuit costs, DUST, service fees, proof costs or runtime. A false observation would change cumulative attempted work, so it is excluded from this exact direct-path predicate rather than silently normalized away.

## Complete common-domain projection

The existing model's final expected `c` is `reserveDone`: specimen Reserve; complete four-account map; complete state map; dutyPresent:true/dutyAmount5; site0; captures[]; committed36. Its `last` has status Complete, attempted10, advanced:true, hasSegment:true, newContinuation:false, effects `[FeeAmount(2),EmitQuote(43)]`, sourceSteps `[8,9,10,11]`, errorSteps[]. `attemptedTotal` is36 for this direct path. Check every member of the independently frozen finite observation, including all map key sets and ordered lists.

The mapping must explicitly bind FeeAmount to the source asset/from/to identity and EmitQuote to `settled_quote` and UInt43. Do not equate bare amounts with full financial effects. The model has no raw Step.statement/ExprIds, per-Step snapshots, wire row order, full frames, report metadata or canonical Program/Plan identity. Those fields remain concrete independent-report obligations; their absence is a model abstraction boundary, not evidence of agreement. No general abstraction theorem follows.

Actual model trace output for this mapping has not been captured in this task. Historical 9-test/32-sample records do not establish the two-sided comparison and must not be replayed just to update a dashboard. Any future permitted observation must bind the actual selected model source/attempt and compare it with the frozen observation, retaining missing fields or mismatches. A Rust-only test against the formal-source interpretation is finite source-informed QA until the formal result side is actually observed.

## Rejection and scope boundary

This selected case is a normally admitted positive business path. Do not discard rejection obligations: keep `normal_business_guard_rejects_only_the_final_attempted_segment` and the model's `reserveBusinessRejectionTest` separate and unchanged. They retain quote51, error step9, attempted6/committed26, no published segment/effects/replacement continuation, and prior boundary basis86 retained. A complete error/report expectation for that separate path still lacks independently selected full message/offset/DTO and model mapping; GAP-07 remains designated-owner unassessed. This ordinary task supplies no new refusal/admission precedence, cap authenticity, hostile case or security judgment. Unexpected rejection of the selected positive invocation is failure, not an alternative success result.

Do not claim public CLI byte compatibility, K correspondence, full UInt128 semantics, private/recursive/native/target acceptance or a parent exit. PR205/208/212's complete fixtures stay intact and are not transplanted to this source. No changes or tests execute until the required independent inputs and applicable runtime admission actually exist. Source-only independent derivation and narrow test preparation can proceed now.
