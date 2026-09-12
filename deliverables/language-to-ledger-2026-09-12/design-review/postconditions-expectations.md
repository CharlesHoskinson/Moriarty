# Independent financial postcondition design and expected results

Scope: recommendation and specified-only expected results before implementation; not candidate approval, K correspondence, financial ledger acceptance, or an empirical result for the proposed feature. Reviewer: delegated GPT-6 Astra context, independently inspecting current source. Initial assigned baseline: `dc9d516eabb5757591e6ef73a7544639cbe55134`; shared main moved during inspection to `229545fada12c151ed5693f4e6cebdab956bd222`. No implementation files were changed by this reviewer.

Startup: read repository AGENTS and installed moriarty-dev development skill and execution-focus reference, refreshed guarded status. Status reported stale SP01 binding/candidate inputs and unavailable accounting/live resource state; no pending transactions. Those findings block dependent registered dispatch, not this authorized design review. Latest task-specific routing supplied by root is Grok 4.6 high author and fresh Astra medium result audit.

## Inspected implementation facts

Read the complete current `src/successor/financial-agreement-source-v3.ts`, `financial-agreement-source-compiler.ts`, `financial-expression-v1.ts` (the shared Core /1 and /2 implementation), `financial-expression-v2.ts`, `funded-expression-source-v1.ts`, `repayment.ts`, financial source lowering and source API files. Paths in this document are relative to `experiments/moriarty-language/` unless otherwise qualified. Also inspected complete current repayment fixture, source/3 specification and pertinent repayment specification sections. These are implementation observations, not graph claims.

- The Machine holds typed nodes, local values, staged ordinary writes, descriptors and one runtime work counter. Its existing `run` reduces the complete action, including every `Ensure`, before returning `ExpressionPrepared`.
- `completeFundedPreparation` subsequently maps descriptors, debits expression work, calls `prepareRepayment` once, checks nominal-unit compatibility and returns both posts. Thus an explicit continuation is necessary to inspect candidate financial post-state before publication.
- Six existing financial reads always use the same owned admitted pre-state. Ordinary `post` is permitted only within Ensure. Old Core /1 excludes financial constructors; Core /2 excludes the proposed post constructors.
- Source/3 statically checks every action before selection, admits the complete financial state before runtime guards, checks work equality, then admits selected snapshots. Source state faults retain kernel rejection envelopes, whereas standalone Core /2 currently converts state faults to expression envelopes. This historical difference must stay frozen.
- Kernel admission checks all rows, closed shapes, capacity 128, uniqueness, canonical UInt128 values, allowance/work sum bounds, obligation principal+accrued=outstanding, status and conversion invariants.
- `prepareRepayment` copies its admitted input, executes ordered Transfer/Repay, charges one unit per action and publishes only a candidate. It permits Transfer-only batches; an empty batch is invalid. The funded adapter rejects no descriptors as bare `EMPTY_BATCH`, before entering the kernel.
- Closure reserve is a separate counter. Ordinary `remaining` can reach zero while reserve stays 16. There is no requirement that remaining must stay greater than reserve.

## Chosen architecture

Recommendation, agreed with root during this review: add `moriarty-financial-agreement-source/4` and `moriarty-financial-expression-contract/3`. Keep existing versions' constructors, reserved names, APIs and rejection priorities unchanged. Source/4 uses the same declaration grammar and existing `ensures` suffix, with these six new generics:

| Intrinsic | Result | Candidate lookup |
| --- | --- | --- |
| `post_outstanding<Cash>(id)` | `Quantity<Units<Cash,1>,0>` | obligation.outstanding |
| `post_principal<Cash>(id)` | same Quantity | obligation.principal |
| `post_accrued<Cash>(id)` | same Quantity | obligation.accrued |
| `post_balance<Cash>(party)` | `Amount<Cash>` | exact party/asset balance |
| `post_allowance_remaining<Cash>(party)` | same Amount | allowance.remaining |
| `post_allowance_spent<Cash>(party)` | same Amount | allowance.spent |

Each accepts exactly one simple declared unit/asset symbol and one Text expression, evaluated once. Choose six dedicated closed constructors `ReadPostOutstanding`, `ReadPostPrincipal`, `ReadPostAccrued`, `ReadPostBalance`, `ReadPostAllowanceRemaining`, `ReadPostAllowanceSpent`, with the existing `unit` or `asset` and `identity` operand shapes. Reserve new names only in source/4. Post constructors are statically permitted only under an Ensure condition; outside that context reject `TYPE_POST_SCOPE`, including dead branches and unused actions. Quantity range and all lookup/domain errors match existing financial reads. Existing unprefixed reads retain financial PRE semantics even in the delayed suffix. Ordinary `post.field` retains staged ordinary post semantics.

Implement one owned typed-machine continuation: action prefix, tentative kernel execution, suffix, publication. Preserve original Core statement indices, source spans, local values, types, source identity and work. The continuation is an implementation detail, never a caller-controlled callback or resumable authority token. It must not rerun any prefix statement or ensure. The financial post context can only come from the actual kernel candidate for this invocation.

### Public API

Source factory `createFinancialAgreementSourceV4()` has `elaborate(source)`, `check(source)` and `evaluate(source, actionName, snapshotsCanonicalJSON, repaymentStateJSON)`. All runtime arguments are primitive strings. Keep the versioned source artifact shape, declaration-order action artifacts and per-action expression `staticWorkBound`; no aggregate invocation bound. Evaluation returns the existing `FundedExpressionResult` union.

Core factory `createFinancialExpressionContractV3(schemaCanonicalJSON, financialPreStateJSON?)` returns frozen `check(requestCanonicalJSON)` and `evaluate(requestCanonicalJSON)`. Its request retains the existing closed fields `contract, source, core, Pre, Args, Obs, workInitial`. It accepts no financialPost, parsed-state object, effect receipt, callback or caller-selected continuation. `evaluate` is integrated funded action evaluation and returns `FundedExpressionResult`, not a pure `ExpressionPrepared` success.

`check` validates structure, schema and whole-Core typing without financial state; it can retain standalone-expression checking for tooling. A statically valid standalone expression passed to integrated `evaluate` rejects `TYPE_ACTION_REQUIRED` with synthetic span, empty path and workUsed 0. A valid action with absent context rejects `FINANCIAL_CONTEXT_REQUIRED` with that expression envelope, after static checking and before snapshot runtime admission. Invalid action Core beats missing context. With supplied context, admission faults are passed through exactly as the funded source API: bare `INPUT_SCHEMA`/`INPUT_BOUND` transport faults, kernel `{status,code,actionIndex:null}` state faults. Pin this new Core /3 distinction explicitly; do not change old Core /2 envelopes.

### Evaluation order and accounting

1. Source primitive/bounds, parse, declarations/protected operation binding and all-action static checks; then exact mandatory action selection. Core entry performs its corresponding full request/schema/Core static checks first.
2. Require Core action form and bound context; admit complete owned financial PRE with the existing state-admission rules. No caller snapshot can replace a financial value.
3. Check work equality, preserving source/3 compound-invalid priority, then selected ordinary snapshot shape/domain/bounds. Failures before reductions report no runtime work.
4. Run the selected non-Ensure prefix once. Preserve all locals and staged writes/descriptors. Check ordinary aggregate post bound before financial preparation; this is bounded validation, not expression reduction work.
5. Map descriptors and reject empty batch `EMPTY_BATCH`; debit actual prefix work in the owned financial state, prepare the existing repayment kernel once and require successful nominal-unit matching. Kernel failure wins over every suffix failure. Retain existing kernel code/actionIndex envelope and expose no tentative posts or effects.
6. Bind financial POST only to that candidate. Decrease the machine's remaining budget by N, the successful kernel action count, without resetting its initial counter. Reduce all ensures in source order, including ordinary-only ones, once. Every failure discards all candidate output.
7. Debit actual suffix expression work in candidate work; publish ordinary post, full financialPost, effects and consistent workRemaining only if all checks succeed.

For successful execution with initial remaining R, prior spent S, prefix reductions B, kernel actions N and suffix reductions Q: remaining=`R-B-N-Q`, spent=`S+B+N+Q`, reserve unchanged. Each read costs one reduction plus the actually evaluated identity child. And/Or/Select keep exact dynamic short-circuit costs. Static work bound counts all expression nodes, includes prefix and suffix, excludes N, and is not a pre-debit reservation.

Expression errors during prefix report actual prefix reductions. After successful kernel execution, suffix `workUsed` includes B+N+reductions actually performed in the suffix, including the failing charged node when applicable. Kernel rejection keeps its existing envelope and does not gain a speculative committed work counter. Every rejection has no `post`, `financialPost`, `effects`, `descriptors` or success workRemaining. WorkUsed is diagnostic work, not publication of a changed input budget.

## Independent numeric fixture and work oracle

Recommendation: clone current `financial-state-payment.mori` under /4, keeping each existing action body and ordinary ensure exactly, and append these six ensures to `repay_remaining`:

```
ensures post_outstanding<Cash>("Due100") == quantity<Units<Cash,1>,0>(0);
ensures post_principal<Cash>("Due100") == quantity<Units<Cash,1>,0>(0);
ensures post_accrued<Cash>("Due100") == quantity<Units<Cash,1>,0>(0);
ensures post_balance<Cash>("Payer") == amount<Cash>(0);
ensures post_allowance_remaining<Cash>("Payer") == amount<Cash>(0);
ensures post_allowance_spent<Cash>("Payer") == amount<Cash>(100);
```

Independent manual node counts: each Quantity comparison costs Ensure 1 + Eq 1 + post-read 1 + Text 1 + LitQuantity 1 = 5. Each Amount comparison costs the same except `amount<Cash>(literal)` lowers to ConstructAmount+LitUInt, so it costs 6. New suffix contribution is 3*5+3*6=33, not 30.

A read-only local probe of current /3 elaboration reproduced these baseline statement counts (after correcting a bug in the probe's object traversal; no product failure):

| Action | Prefix B | Existing ordinary ensure | Existing total expression bound |
| --- | ---: | ---: | ---: |
| repay | 37 | 6 | 43 |
| repay_installment | 39 | 6 | 45 |
| repay_remaining | 41 | 6 | 47 |

With the six new ensures above, repay_remaining has expression bound 80 and successful total debit 82 (B41+N2+Q39). Start with the existing work 256/spent0/reserve16 and debt100/payer100/lender0/allowance100: final remaining174/spent82/reserve16; ordinary paid100, debt principal/accrued/outstanding0/0/0 and Settled, balances payer0/lender100, allowance remaining0/spent100. Both tombstone lists gain the exact passed ID once; two effects retain complete Transfer/Repayment values and order; unrelated rows and metadata remain byte-for-value equivalent.

For the existing sequence with new six checks added only to the last action, pay30 costs45, installment20 costs47, repay_remaining50 costs82. Remaining is211,164,82; spent45,92,174. Final financial/ordinary values are the same complete repayment values. A fourth remaining-payment call fails the positive-payment guard after 10 prefix reductions, with no effects and no new IDs.

A partial interest control uses principal80/accrued20/outstanding100, AccrualFirst, conversion1, payment30. Reuse current `repay` prefix and append six analogous checks for outstanding70, principal70, accrued0, payer70, allowanceRemaining70, allowanceSpent30. Total debit is37+2+39=78; remaining178/spent78 from256. Repayment effect discharges principal10/accrued20 and nominal/settlement30. PrincipalFirst instead leaves50/20/70; ProRata leaves56/14/70 with discharged24/6. These are specified expectations from kernel arithmetic, not executions of proposed /4.

### Exact work boundaries for the full-payment fixture

| Initial remaining R | Expected result |
| ---: | --- |
| 82 | Success, remaining0, spent increased82, reserve16 untouched |
| 81 | Kernel succeeds privately; last suffix expression exhausts work; WORK_EXHAUSTED/workUsed81; no candidate output |
| 43 | Prefix41 and kernel2 complete privately; first Ensure cannot reduce; WORK_EXHAUSTED/workUsed43 |
| 42 | Prefix succeeds; kernel preflight INSUFFICIENT_WORK/actionIndex null; no suffix evaluation |
| 41 | Same kernel INSUFFICIENT_WORK result |
| 40 | Prefix WORK_EXHAUSTED/workUsed40 before kernel |

Set snapshots.workInitial to each R and keep the state invariant valid. Giving any amount of closureReserve cannot turn these failures into ordinary-work success. In particular, do not reserve the static suffix bound up front: `ensures true or <validly typed expensive/missing post read>` must succeed with only Ensure+Or+LitBool=3 actual suffix reductions when that is the available budget.

## Required adversarial and rollback cases

All rows are specified-only requirements for the author and fresh final reviewer.

| Case | Exact predicate |
| --- | --- |
| Deliberately false first financial ensure after otherwise valid full payment | ENSURES_FAILED at that Ensure span/path, workUsed54 for the fixture (prefix41+kernel2+ordinaryEnsure6+firstQuantityEnsure5); no posts/effects/new IDs |
| False ordinary ensure and insufficient payer funds | Funding rejection before suffix under /4; old /3 keeps its historical ensure-before-kernel priority |
| Late false final financial ensure | Correct previous suffix nodes execute once; false final Amount comparison gives ENSURES_FAILED/workUsed82; all tentative financial and ordinary changes disappear |
| Prefix `outstanding` captured in local and compared with post read | Local retains pre-value100 while post value0; no prefix replay and unprefixed read in suffix still100 |
| Valid missing pre receiver balance, then Transfer creates receiver row | post_balance(receiver) reads newly created amount; no implicit pre row is fabricated |
| Missing candidate obligation/balance/allowance | MISSING_OBLIGATION/MISSING_BALANCE/MISSING_ALLOWANCE at actual post-read node; missing never equals zero |
| Bad identity including valid prefix followed by newline | INVALID_IDENTIFIER with a genuine full-string identifier check, not regex dollar-anchor prefix acceptance |
| Wrong declared nominal unit | Static unknown unit TYPE_NAME; existing but mismatched obligation denomination NOMINAL_UNIT at runtime |
| Candidate Quantity exceeds signed128 maximum | ARITH_RANGE; no truncation or unsigned reinterpretation. Amount still admits full UInt128 |
| Invalid unrelated financial row or duplicate row and false guard | Full-state admission error first, no expression reductions |
| Bad unselected action, bad selector, bad state | Static source error first; with valid source bad selector before state; valid selector bad state before guard/suffix |
| Missing Core context and invalid action Core | Static type/shape error first; valid action missing context FINANCIAL_CONTEXT_REQUIRED/workUsed0 |
| Post read in Let, Require, Next, Emit, or standalone expression | TYPE_POST_SCOPE statically, including skipped branches/unselected actions |
| Post read in skipped suffix branch | No lookup error and no read/identity debit; all branches still statically checked |
| Empty descriptor action with false ensure | EMPTY_BATCH before suffix; do not convert it to ordinary-only success |
| Transfer-only descriptor batch | Legal kernel preparation; postcondition can observe balances changing and debt unchanged |
| Repay with missing same-step Transfer | TRANSFER_NOT_IN_STEP at repayment action; suffix cannot supply a substitute funding observation |
| Existing transfer/allocation tombstone reused | Existing kernel DUPLICATE priority and actionIndex; no suffix or externally changed state |
| Transfer succeeds but Repay fails | No transfer effect, sender debit, allowance debit or tombstone escapes |
| Caller mutates elaborated schema/Core before evaluate | Recompilation from primitive source prevents influence |
| Candidate or callback added to Core request | Closed request INPUT_SCHEMA; cannot authenticate a forged financial post |
| Older versions see new constructors/names | Old Core TYPE_CONSTRUCTOR including dead branches; old source keeps prior legal identifier use and rejection priorities |
| CLI actual /4 success and semantic failure | Existing action/snapshot/state flags; no --schema; success output profile/action correct; failure exit1, stderr rejection only, empty stdout |

Full-state rollback comparisons must include spent/remaining/reserve, all obligations and metadata, allowance gross-spent accounting, balances, tombstones, ordinary fields, output effects and input strings. Repeated evaluation of unchanged primitive inputs is deterministic and cannot leak prior machine state. Host-observed prepared projections remain unauthenticated and confer no permission to bypass ledger admission.

## Alternatives considered

Chosen: explicit post_* family, versioned semantics, existing Ensure suffix delayed in the new version, one integrated funded Core entry. It has a small closed surface, supports comparing pre and post in one expression, and keeps financial provenance under the interpreter's control.

Rejected: reinterpret existing unprefixed reads as post-state in ensures. It silently changes existing /3 meaning and prevents clear same-expression pre/post comparisons.

Rejected: add caller-supplied financialPost, callbacks, boolean success flags, or synthetic Pre/Obs fields. Each allows the asserted postcondition to detach from the prepared transition.

Rejected: evaluate the entire action a second time after repayment or rebuild it with precomputed literal reads. That repeats guards/effects, breaks locals, source paths, short-circuiting and work accounting.

Rejected: run old ensures first then only newly detected financial ensures afterward. Splitting by expression contents makes source-order error and work behavior surprising, and requires special categorization when an ensure mixes ordinary and financial reads. All ensures run after preparation only in the new version.

Rejected: duplicate the complete machine or immediately add a public general continuation API. Share existing implementation behind version gates; keep runtime continuation private.

Rejected: debit the static full-action bound before execution. Dynamic skipped branches must remain uncharged; empty/failed suffixes cannot become a successful commit.

## Constraints for the next origination/accrual item

Recommendations only; no implementation in this review.

1. Current source protected operation binding requires exactly Transfer and Repay and same asset/nominal unit. Current kernel Action is exactly Transfer|Repay and state shape is closed. Originate and Accrue cannot be quietly added to old source/Core/kernel versions. Extend under explicit successor versions and keep old error contracts.
2. Origination must be a real source-emitted operation reaching a versioned kernel action, with a closed typed obligation payload, fresh obligation ID, checked capacity/invariants and explicit initial status. A lifecycle must distinguish recording an obligation from disbursing money; where a loan is represented as funded origination, bind a same-candidate Transfer to creditor/debtor, settlement asset and exact nominal conversion. Do not mint an unfunded financial balance from an ordinary NextWrite or fixture setup.
3. State/work/allowance initialization remains a separate provenance problem. Define the trusted local seed explicitly and retain unauthenticated scope; a source origination demo that starts with lender balance/allowance is acceptable only if that seed is stated, not described as authenticated account creation.
4. Accrual must update accrued and outstanding together using checked integer arithmetic and explicit rounding. Decide simple versus compounding, principal base versus outstanding base, period representation, rate sign, day-count/time source and replay identity before authoring. An unrestricted caller-provided increment only demonstrates an explicit adjustment, not authenticated interest accrual or ACTUS conformance.
5. Existing state has no accrual replay IDs, period cursor or obligation birth provenance. Any claimed replay-safe accrual needs versioned persistent metadata and a bounded tombstone/cursor rule. UsedTransferIds or UsedAllocationIds must not silently stand in for accrual history.
6. Decide settled-obligation accrual explicitly. Recommended minimal lifecycle forbids positive accrual on a settled obligation; do not silently resurrect debt. Reject zero/negative nominal origination or accrual according to the chosen closed action contract and retain residual duties.
7. New event kinds must share the same prefix/kernel/ensure continuation and total work accounting. Same-call pre reads cannot observe a just-originated obligation; post reads can. A later action invocation uses the complete accepted local post as its new pre. No second ad hoc financial mutation path.
8. Lifecycle expected arithmetic should be fixed independently before implementation, e.g. disburse100, accrue10 under a specifically defined one-period rule, pay30 AccrualFirst leaves principal80/accrued0/outstanding80, then computed remaining80 settles. Check conservation, all payer/lender balances, both parties' gross allowance debits, replay IDs and cumulative work. This example is only a proposed arithmetic oracle until the new source/kernel contract chooses its accrual formula.
9. New local lifecycle success does not establish K coverage, formal correspondence, authenticated Docker/Preview execution, PCD, fee-aware net goals or aggregate roadmap acceptance. Those remain separate later stages and preserve the existing resource/admission stops.
