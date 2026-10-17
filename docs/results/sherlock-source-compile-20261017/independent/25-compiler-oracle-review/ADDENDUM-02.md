# R25 addendum 02: correction02 narrow review

**Verdict: R20-F1 CLOSED FOR THE ESTABLISHED SELECTED NORMAL PROJECTIONS.** No further material correction was found in this additive packet. This is an ordinary non-author source/data review, not full runtime-oracle approval, observed B04 agreement, security judgment or native authorization. The original `REVIEW.md` and `findings.json` remain unchanged.

Reviewed correction freeze `f34e552376880e6379bc349439d6753263deb5e9e5497682211ed24e0f18cf07` and handoff `a43fd257ec6781c3193dcbf660a7458d78aa416a49eb1b72c282bde60a044c95`. All 41 sealed members match their hashes and lengths. The 34 original R20 members, original freeze `2a3fd342dcd33ac36ee482ab186c2e87b4ac20ab353b81279aa68ea16a5eb800` and handoff remain unchanged. The separate prior 333-carrier oracle retains all 38 members and freeze `88c6fba4b81f7326b34fb5cc759019e55dd6370fc9785ff5040ad080409cc2a2`. All 25 declared source/lineage pins match. Exact input paths, hashes and read extents are in `findings02.json`.

## Causal fix and retained events

The unchanged normative rows remain the authority: ReturnLeft rewrites LeftF to RightF and directly sets Eval(right); ReturnLet rewrites LetF to ScopeF and directly sets Eval(body). Neither creates ScheduleNext or changes depth. ReturnIfTrue pops IfF and directly sets Eval(thenExpr); it creates no ScopeF. Push consumes ScheduleNext. These effects justify deleting the phantom pushes rather than adjusting a total to agree with code.

Both specimens first differ at zero-based event ordinal 21. Event 20 ReturnLeft belongs to Binary Expr2, preserves depth 3 and selects right literal Expr4. The next valid projected event is Validate.entry at depth 3. Prefixes through 21 processed events are identical. After event 21, consumed work is still 22 in both schedules, but the corrected helper_work/reference_cursor are 11/11 rather than 10/12. The lineage records these exact before/after projections correctly.

Quote removes original Push ordinals 21,37,47,63. Eligibility removes Push 21,37,51,63,71,89,103,115,127,139 and Scope return 134. The selected And branch has two eager-Let scopes; its If consumes its own frame. The third original Scope had no creator. I checked the complete alignment, not a sample: all 71 Quote and 141 Eligibility retained projected events equal their original fields except ordinal. Helper blocks, helper order, payload allocations and terminal publication remain unchanged.

## Dependent counters and final state

| Selected quantity | Quote | Eligibility |
|---|---:|---:|
| Charged legacy / reference cursor | 29 | 58 |
| Helper work / visits | 41 / 13 | 82 / 29 |
| Consumed work | 70 | 140 |
| Events, including free publication | 71 | 141 |
| Remaining work from 1000 | 930 | 860 |
| Data rows / persistent environment cells | 2 / 2 | 2 / 7 |
| Logical frame peak / final depth | 4 / 0 | 7 / 0 |

Independent static arithmetic checked all 72 Quote and 142 Eligibility selected-counter prefixes against their event columns and normative work/helper/reference deltas. Every logical-depth prefix agrees with normative depth deltas and certified live frame intervals. Coverage counts and final summary/image counters agree with the corrected schedules.

Exactly seven normal-result leaves change per specimen: events_forwarded; summary events/consumed_work/reference_cursor; final-image consumed_work/reference_cursor/remaining_work. Quote changes 75→71 events, 74→70 work, 33→29 reference cursor, 926→930 fuel. Eligibility changes 152→141, 151→140, 69→58 and 849→860 respectively. All other normal-result leaves are identical. Final UInt82/Booltrue, Data1 result, arena payloads, environment cells, restored empty environment, empty active frames/tasks/value slots and other registers remain as previously reviewed.

## Certificate adequacy and limits

I read the new fixed-continuity checker and complete emitted certificates without running either. Their manually transcribed node anchors match the frozen prepared expression tables. Checked annotations cover the entire manual plan and event intervals. All 5/11 Push events have an immediate normative ScheduleNext-producing predecessor. All 4/10 affected direct continuations name the exact right/body/then target, with the retained literal/lookup helper block preceding its Eval completion when needed. All Let/Binary/If/Return frame intervals pair the intended creator, rewrite and pop; their nesting, top-at-rewrite/pop, every-prefix depth and zero final depth agree. Every Scope is paired with a Let, and If creates none.

This is adequate to reject the concrete old phantom-Push/excess-Scope defect and validate the stated fixed control/frame facts. The checker uses handwritten anchors, selected rows and helper-block skipping; it is not a general source evaluator or a proof of every helper/register transition. The complete retained-event comparison and previously established helper projection are necessary parts of this narrow conclusion. Certificate agreement does not supply omitted peripheral StepEvent fields or full intermediate images.

Both twelve-field ProgramViews, source-report projections, alpha adapter, config, identity/layout/genesis and all nine explicit omission entries remain unchanged. The compile-only availability clarification correctly reserves genesis for a direct admitted library preparation pair; it invents no compile-only observation. Eligibility admission remains conditional, including unresolved actual fresh-name bytes. The nine omissions remain: raw fresh names/NameIds; Core expression-ID mapping; report spelling; peripheral StepEvent fields; full per-event images; inactive storage tails; unmeasured resource text/slices; target descriptor; wrapper metadata wire details. A predicted depth certificate does not constitute observed complete StepEvent equality.

## Independence and next acceptance boundary

The corrective author had the original review, including its recount; this is not a blind second derivation claim. My acceptance follows the pinned normative row effects, unchanged source shapes, exact removal/alignment, prefix arithmetic and fixed frame certificates. Existing producer bodies do not replace those authorities. No R19 candidate, R23 opinion or new B04 output was read. The original source review remains the record for existing public API/type facts; fresh lineage source reads here were hashes only.

I ran only local file/JSON comparison, hashing and arithmetic over frozen data. I did not run the packet scripts, parser, compiler, Rust/Cargo, interpreter, model, SDK, native or provider. The packet records pre-observation timing and Root reports no new compile_host output existed before this recheck; that is retained provenance, not independent authentication of every process history. No observed subject agreement is claimed.

The concrete R20-F1 correction is now ready for Root's separately authorized observed comparisons against this exact sealed packet. Preserve actual raw fields and the explicit alpha mapping; compare only the established projections and report missing fields. Full raw-event/full-prefix-image, general compiler/interpreter, finance, target/native and U/M/MC/SP/G acceptance remain outside this disposition. No gate, policy, source or authority was changed.
