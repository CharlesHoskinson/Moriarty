# R20 compiler normal-oracle review

**Verdict: REQUEST_CHANGES — the frozen normal runtime event/work projections are incorrect.** The exact Quote ProgramView, alpha Eligibility ProgramView, scalar mathematical values, explicit configuration and typed identity/genesis projections receive scoped structural agreement. Do not use this seal as an approved runtime-work oracle or full B04 acceptance oracle.

Reviewed seal: `2a3fd342dcd33ac36ee482ab186c2e87b4ac20ab353b81279aa68ea16a5eb800`. All 34 sealed files and all 21 declared source pins match their recorded hashes/lengths. W source version at review: `1a0f5ea322d4b2ebb6587dbb9fd24bb946434fd7`; individual source hashes and exact content-read extent are recorded in `findings.json`. I applied current AGENTS/product/develop guidance and read guarded status without following its historical campaign queue. No R19 compiler candidate, new B04 output or R23 opinion was read. No Rust, Cargo, interpreter, model, SDK/native, provider or subject test ran; no W/Git/private edits occurred. Hashing, JSON inspection and fixed arithmetic below are audit operations, not subject execution.

## Blocking finding R20-F1: impossible extra Push transitions

Both handwritten `manual_plan` lists insert `push.accept` immediately after `return.left` and `return.let`. The pinned normative **03-rows-supplement.json** says:

- `k.machine.return.left` rewrites the top LeftF to RightF, sets control directly to `Eval(top.right)` and preserves depth.
- `k.machine.return.let` rewrites the top LetF to ScopeF, sets control directly to `Eval(top.body)` and preserves depth.
- `k.machine.push.accept` consumes a `ScheduleNext` control. Neither preceding transition produces one.

Therefore the next selected event cannot be Push. In each specimen the first concrete divergence is frozen event **ordinal 21**, after `return.left`: the next right operand is a literal and requires `Validate.entry`, whereas the frozen oracle says `KMachinePushAccept`. Quote contains four such invalid Push events at ordinals **21,37,47,63**. Eligibility contains ten at **21,37,51,63,71,89,103,115,127,139**.

Eligibility has an additional error around ordinals 126–134. Normative `return.if.true` pops IfF and sets `Eval(thenExpr)` directly; it creates no new ScopeF or ScheduleNext. The oracle nevertheless adds Push at127 and then three Scope returns at132–134. Only the two scopes of the two eager And Lets remain there. One of those three Scope events is extra. This is eleven excess legacy events in total for Eligibility.

Existing public `fragments/legacy.proposed.rs:147–180,201–212,266–289` independently exposes the same disagreement: Push handles ScheduleNext, Let/Left rewrite then Eval, If pops then Eval, and Scope pops an actual ScopeF. These implementation facts corroborate the diagnosis; they are **not** the oracle authority. The correction follows the pinned normative row effects and source AST, without executing or matching a producer.

The arithmetic consequences are:

| Field | Quote sealed | Quote normative recount | Eligibility sealed | Eligibility normative recount |
|---|---:|---:|---:|---:|
| Reference cursor / charged legacy events | 33 | **29** | 69 | **58** |
| Helper work | 41 | **41** | 82 | **82** |
| Helper visits | 13 | **13** | 29 | **29** |
| Consumed work | 74 | **70** | 151 | **140** |
| Events including free publication | 75 | **71** | 152 | **141** |
| Remaining fuel from1000 | 926 | **930** | 849 | **860** |

These are independently reasoned proposed corrections, not observed runtime outputs or permission to overwrite sealed data. Regenerate the manual event lists and every affected ordinal, prefix counter, final image counter, RunSummary, coverage count and derivation from the corrected normative sequence. Preserve the original seal and issue a new dated freeze for fresh review. Do not change expected cells to fit R19 or any future actual output.

Independent recount: Quote has entry/function machinery5, first Let+Mul+binding10, second Let+Add+binding10, final variable1, two Scope returns2, return-type completion1 =29 legacy events. Eligibility has the same entry5, low binding10, high's eager Gt+binding20, eligible's eager And+binding18, final variable1, outer Scope returns3 and return-type completion1 =58. Helper blocks remain Quote `12+8+6+6+3+3+3=41`; Eligibility `16+8+21+25+3+3+3+3=82`. Removing these phantom legacy events changes no helper traversal or allocated values/environments.

`LITERAL-CHECKS.json`/`INPUT-CHECKS.json` do not catch this: `check_inputs.py` checks each named row's individual delta and sums, but not whether the previous row produces that row's required control/frame shape. A per-row catalogue membership pass cannot validate this sequence. Preserve those checks as narrow static evidence; add a meaningful fixed normal continuity check derived from the normative row contracts when correcting this oracle.

## Structural and value findings

Quote's twelve fields are complete for the actual DedicatedSorts ProgramView: entry, expressions, occurrences, occurrence_paths, children, functions, parameters, arms, patterns, constants, constant_fields and names. Its entry Call0 targets the zero-parameter `quote` function with body1; the ten preorder rows retain both lets and Mul/Add. Names `[base,quote,total]`, constants `[7,11,5]`, nine body occurrences and sixteen path slots agree with the frozen input and normative preparation ordering. Empty tables/slices remain explicit. No A-profile declaration tables should be added to this B view.

Eligibility's 23 expressions,22 body occurrences,76 path slots and five constants `[3,9,9,3,false]` agree with the declared eager Gt/And templates. Gt preserves source operand evaluation before reversing comparison references; And evaluates both operands before its If. The unevaluated false branch remains in the prepared constant table. Eight names consist of four source names plus four fresh-role placeholders. The bijective alpha adapter is suitable for this comparison: bind roles at Let rows6,8,14,16; preserve four source names; rewrite all NameId references consistently; preserve ExprIds/constants/paths/indices and all raw actual fields. Placeholder spelling/bytes establish neither actual fresh spelling nor NamesBytes admission.

The normal arithmetic is independently clear: `7*11+5=82` and `3<9`, `9>3`, eager conjunction =true. Normative Scalar.exit allocates two data nodes in each specimen. Quote's two environment cells and Eligibility's seven cells, including retained unreachable hygienic-let cells and their previous links, agree with persistent newest-first environments. The final result reference is Data1; the function restores the empty outer environment; helper/result/active frames/tasks/value slots are empty. Full final image **counter values** still require the R20-F1 correction. I do not approve the present complete final JSON simply because its structural/value subset agrees.

The config is an explicit transformation of the old public input-only envelope, not a new policy default: fuel1000, stack32, all stated bounds/capacities/host extents, one shared bounds object, null field/width/scalar layout descriptors and false boundary capacities. Decimal strings fit stated carrier widths. Typed HostOnly identity has profile_id2 and explicit Unselected digests; valid genesis is Eval0/Running, empty active slices, zero work/allocation/segment-base values, remaining=initial=1000. Neither hashes nor these identity fields authenticate a native statement. Actual admission and raw fresh-name extent remain conditional.

Source-report projections properly avoid inventing raw Core parser IDs or synthetic spelling. Existing `source_adapter.rs`/owner bodies were read to establish API/field facts and expose disagreements only; the mapping-template lineage is author-owned source/spec material, not independent proof that implementation preserves source semantics. This correlation remains a comparison limitation.

## Preserved omissions and comparison boundary

All nine explicit omission groups remain: raw fresh names/NameIds; raw Core parser IDs; source-report node_kind/symbol_domain spelling; peripheral StepEvent fields; complete per-event images; inactive backing tails; resource-report unmeasured text/host slices; unsupplied target descriptor; new wrapper metadata wire details. The packet supplies projections, not complete raw StepEvent or complete prefix-image evidence. The target must be separately supplied; this review cannot close a real CLI invocation predicate.

Also distinguish library genesis comparison from public compile-only artifact comparison. FIRST's compile-only artifact exposes admission identity/view/layout and `observation:not-requested`; it does not declare a genesis image there. `EXPECTED-IDENTITY-LAYOUT-GENESIS.json` is a useful direct preparation-pair expectation, but its genesis portion cannot silently become an observed compile-only artifact field. Keep that availability limit inside the declared wire/metadata comparison boundary; do not add a fabricated observation.

The structural compile-only subset may inform bounded preparation work, with exact raw fields retained and the admitted result conditional as stated. The runtime event/work subset needs correction before it is used to judge actual outputs. Full B04 observe acceptance, missing field/prefix obligations, all U/M/MC/SP/G gates, security ownership and native authority remain unchanged.
