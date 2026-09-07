# Candidate A finite Core interpreter implementation plan

> For agentic workers: use superpowers:executing-plans for the implementation tasks below. This document authorizes no new provider dispatch, integration, or semantic motion.

**Goal:** implement a finite, independently checked Quint representation of the frozen Moriarty E00 agreement semantics for Candidate A, with canonical swap and two-installment agreement fixtures.

**Architecture:** a pure agreement evaluator operates on finite constructor nodes, accounts, choices, and minimum time. A separate projection adapter exports complete results; the existing common authorization boundary remains a consumer and is not reimplemented. An independently written Python checker invokes the pinned reference semantics and checks the model's raw results and extracted projections.

**Tech stack:** repository Quint 0.32.0 tooling; Python frozen dataclass semantics; pytest; JSON/ITF trace records. Pin actual executable versions and hashes when execution begins.

Status: local downstream implementation plan, specified-only. This plan neither integrates Candidate A nor selects A–D. Council acceptance and all S02 package gates remain separate/open.

## Global constraints and immutable source locators

Repository observation at planning HEAD `7e5697fa50ad6800ec84d44b108752e4055fb89e`:

| Input | SHA-256 |
| --- | --- |
| `moriarty/core.py` | `564a926779beb54bd16ff58481f5c950bae6fb50fcd5b0d7c9eb56427f83273b` |
| `moriarty/swap.py` | `82e9e1da76af65706171049f0238953aca5333edec4aeed6caa43dbd57c79797` |
| `evidence/semantic-scope/moriarty-core-0.0.0-e00.2.json` | `9bee72cb3a71962128ce5ead07b0912a3f54b1d813d59f629852631057e36c7a` |
| `docs/superpowers/specs/2026-09-04-moriarty-s02-model-comparison-design.md` | `75e242cabdde75fc77b6512bf8949715ad8f289311f32663d04da3c600856ab9` |
| `docs/superpowers/specs/2026-09-05-moriarty-s02-common-design-decision.md` | `d45d0073a9350ecf7478f65143766bf0c49e90dcde373b43a4b7d059cec93deb` |
| `specs/quint/s02/observations.qnt` | `e44484ed9035e560ef9f4d012198671917750da42ab404022592db16cdd59bc3` |

The two Python hashes match the immutable input entries in
`evidence/s01-intent-theorem-freeze/evidence-manifest.json:41` and `:51`.
The semantic-scope record names repository commit
`006c4d91ed09c0a89261861b6e7203b3efa3e2df`, version `0.0.0-e00.2`,
and an evidence-only motion. Do not confuse the current planning HEAD with that
historical scope commit.

Source facts to preserve:

| Source locator | Obligation |
| --- | --- |
| `moriarty/core.py:30`, `:35`, `:44`, `:66` | Exactly Constant, ChoiceEquals, Deposit/Choice, and Close/Pay/If/When; no Notify, Let, Assert, arithmetic expression, cancellation, or recovery constructor. |
| `moriarty/core.py:123` | Positive sparse accounts, canonical ordering, unique account/choice keys, nonnegative minimum time. |
| `moriarty/core.py:157` | Missing account reads zero; missing choice reads None, not zero. |
| `moriarty/core.py:179` | Preserve every Payment, Warning, ReductionResult, and TransactionResult field. |
| `moriarty/core.py:214` | ChoiceEquals compares optional lookup with an integer; an absent choice never equals zero. |
| `moriarty/core.py:220` | Close refunds one canonical account per reduction; Pay clamps payment, emits exact warnings, then advances; If and expired When each count once. |
| `moriarty/core.py:282` | Ordered first accepting case; matching nonpositive deposit rejects; an out-of-bounds choice can fall through to a later accepting case. |
| `moriarty/core.py:316` | Clock check, pre-input reduction, optional input, post-input reduction, whole-transaction rollback. |
| `moriarty/swap.py:65`, `:78` | Alice/alice, Bob/bob, TokenA=(aa,A), TokenB=(bb,B), quantities 10/20, deadline 100, choice id settle, and exact canonical topology. |
| comparison design `:82` | A is an agreement evaluator plus an independent envelope; it is not another label over the common execution fixture. |
| adopted design `:25`, `:35`, `:49`, `:59` | Complete observations, exact opaque bindings, common evidence/commit boundary, lifecycle-only cancellation, independent actual recovery effects. |

All source inspection in this planning task is read-only. Existing installment
files remain unchanged. No implementation, tests, simulation, or model checking
of Candidate A has run.

## Proposed file responsibilities

Create only when this downstream implementation is started:

| File | Responsibility |
| --- | --- |
| `specs/quint/s02/candidate_a_types.qnt` | Finite syntax, state/input/result carriers and domain checks. Imports neutral types from effects/observations only. |
| `specs/quint/s02/candidate_a_core.qnt` | Pure evaluation, one reduction, quiescent reduction, ordered input application, and transaction rollback. |
| `specs/quint/s02/candidate_a_programs.qnt` | Fixed canonical swap, installment, and diagnostic node maps; no interpreter. |
| `specs/quint/s02/candidate_a_core_test.qnt` | Deterministic semantic vectors with complete expected results. |
| `specs/quint/s02/candidate_a_projection.qnt` | Complete raw-state/result and ordered-effect projection; no policy decision and no EvidenceValid production. |
| `specs/quint/s02/candidate_a_harness.qnt` | Stateful bounded transaction traces and exportable request/result records; accepted updates and explicitly recorded rejected attempts. |
| `specs/quint/s02/candidate_a_test.qnt` | Canonical swap and installment traces, boundary witnesses, adapter mutations. |
| `scripts/export_s02_candidate_a_cases.py` | Parse Quint ITF records without computing semantics; preserve raw model results and their projection. |
| `scripts/check_s02_candidate_a_correspondence.py` | Independently decode input/program/state, invoke frozen Python, compare every result/projection field, and check source pins. |
| `tests/test_s02_candidate_a_correspondence.py` | Checker tests and independently corrupted input/result/abstraction-map controls. |

No task edits frozen Core/swap, shared policy/authorization/execution modules,
the scope record, or immutable receipts. Final authority integration needs its
own reviewed task and acceptance decision.

## Exact proposed finite carriers

Reuse `Principal`, `Asset`, `CoreAccount`, `CorePayment`, `CoreWarning`,
`CoreError`, and `OptionalInt` from the neutral common modules. The following
is an interface sketch, not a new language extension:

```quint
type NodeId = N0 | N1 | N2 | N3 | N4 | N5 | N6 | N7
  | N8 | N9 | N10 | N11 | N12 | N13 | N14 | N15
type AChoiceId = SettleId | FirstFillId | SecondFillId | RecoveryId | OtherId
type ATime = Time0 | Time1 | Time2 | Time100 | Time101
type AValue = ConstantA(int)
type AObservation = ChoiceEqualsA({id: AChoiceId, expected: int})
type AAction = DepositA({account: CoreAccount, depositor: Principal, amount: AValue})
  | ChoiceA({id: AChoiceId, chooser: Principal, lower: int, upper: int})
type ACase = {action: AAction, continuation: NodeId}
type ANode = CloseA
  | PayA({account: CoreAccount, payee: Principal, amount: AValue, continuation: NodeId})
  | IfA({observation: AObservation, thenNode: NodeId, elseNode: NodeId})
  | WhenA({cases: List[ACase], timeout: ATime, timeoutNode: NodeId})
type AProgram = {root: NodeId, nodes: NodeId -> ANode}
type AState = {continuation: NodeId, accounts: CoreAccount -> int,
  choices: AChoiceId -> OptionalInt, minimumTime: ATime}
type ASuppliedInput =
  DepositInputA({account: CoreAccount, depositor: Principal, quantity: int})
  | ChoiceInputA({id: AChoiceId, chooser: Principal, chosen: int})
type AInput = NoAInput | PresentAInput(ASuppliedInput)
type AReduction = {state: AState, payments: List[CorePayment],
  warnings: List[CoreWarning], reductions: int}
type AReductionStep = QuiescentA(AState) | ReducedA(AReduction)
type AReductionEvaluation = ReductionCompleteA(AReduction) | ReductionBoundFailureA
type AInputResult = {state: AState, error: CoreError}
type ATransactionResult = {accepted: bool, state: AState, error: CoreError,
  payments: List[CorePayment], warnings: List[CoreWarning], reductions: int}
type ATransactionEvaluation = TransactionComputedA(ATransactionResult) | OutsideModelDomainA
type APredecessor = {program: AProgram, state: AState}
type AContinuation = {program: AProgram, node: NodeId}
```

Finite instantiation checks are essential: an `int` alias or unbounded List
does not itself make the model finite.

- All sixteen node keys are present. Rank is N0=0 through N15=15; every Pay,
  If, case, and timeout successor has strictly smaller rank. Unused nodes are
  CloseA. This represents finite acyclic source trees using shared nodes,
  without recursive Quint types.
- Cases are ordered Lists of length 0, 1, or 2. Preserve duplicates and order.
- Constants range over {-1,0,1,5,10,20}; choice bounds/expected values and stored
  choices range over {-1,0,1,2}, with lower <= upper. Supplied deposit quantities
  range over {-1,0,1,5,10,20,21}; the extra 21 is an exact mismatch probe.
- Time values decode exactly as {0,1,2,100,101}. Time0 is initialization/diagnostic
  minimum time; authorized common-envelope transactions retain its existing
  {1,2,100,101} environment constraint. Do not add physical-time equality to
  the pure Core evaluator: frozen Core only checks now >= minimum time.
- Accounts have all six Principal x Asset keys. Initial diagnostic balances are
  in 0..20; canonical swap starts all zero; installment starts Alice/TokenA=10
  and all other accounts zero. Internal arithmetic remains exact, never clamped
  to satisfy a model bound. A conservative reachable bound is 340 per account.
  Check the domain potential `balance + 20 * rank(continuation) <= 340` for
  every account. A deposit increases balance by at most 20 while decreasing rank
  at least one, so the potential is non-increasing. No overflow is encoded as
  a Core error.
- All five choice keys are present; NoInt represents Python absence. IntValue(0)
  represents an actual stored zero. Their exported names are exactly
  settle, fill1, fill2, recover, other. No identifier has interpreted string
  structure inside the evaluator.
- Canonical account iteration is the fixed List
  [Alice/A, Alice/B, Bob/A, Bob/B, Mallory/A, Mallory/B], matching decoded Python
  ordering by party name, token policy id, and asset name. Do not use set
  iteration or oneOf to choose a Close refund.
- The full program travels in predecessor and continuation bindings. A node id
  without its program is not a complete agreement identity.

These are bounded-corpus assumptions, not claims that all Python integers,
contracts, or input sequences are covered.

## Pure interfaces and their exact behavior

```text
timeValue(time: ATime): int
choiceName(id: AChoiceId): str
rank(node: NodeId): int
validProgram(program: AProgram): bool
validState(program: AProgram, state: AState): bool
validInput(input: AInput): bool
evaluateObservation(observation: AObservation, state: AState): bool
reduceOnce(program: AProgram, state: AState): AReductionStep
reduceToQuiescence(program: AProgram, state: AState): AReductionEvaluation
applyInput(program: AProgram, state: AState, supplied: ASuppliedInput): AInputResult
computeTransaction(program: AProgram, before: AState, supplied: AInput, now: ATime): ATransactionEvaluation
projectState(program: AProgram, state: AState): CoreStateObservation[AContinuation]
projectResult(program: AProgram, result: ATransactionResult): CoreResultObservation[AContinuation]
projectInput(input: AInput): NeutralInput
extractCommittedEffects(program: AProgram, before: AState, input: AInput, now: ATime, result: ATransactionResult): List[Transfer]
```

All definitions above are `pure def` in Quint. `applyInput` has the explicit
precondition that the current node is an unexpired WhenA; it is not another
transaction entry point.

Reduction rules, in source order:

1. CloseA with no positive accounts is quiescent. Otherwise select the first
   positive account in the canonical List, zero that account, emit exactly one
   payment to its owner in its own asset, retain CloseA, and count one reduction.
2. PayA: requested = its ConstantA quantity; paid = min(max(requested,0),balance).
   requested <= 0 emits non_positive_payment with requested and paid=0.
   Otherwise paid < requested emits partial_payment with those exact integers.
   Emit a payment only when paid > 0; subtract it from the source account; advance
   unconditionally to continuation and count one reduction.
3. IfA evaluates ChoiceEqualsA against optional choice lookup, selects one
   successor, and counts one reduction.
4. WhenA with minimumTime >= timeout advances to timeoutNode and counts one;
   otherwise it is quiescent. Input is not examined by reduction.

Implement reduction iteration with a fixed ordered fold of 22 calls, retaining a
quiescent result once found. At most 15 strictly descending non-Close reductions
plus six Close refunds are possible; the extra call observes quiescence. Count
actual reductions, not fold iterations. This internal pure fold is not a stutter
action. Assert ReductionBoundFailureA is unreachable for every admitted program;
it is a model failure, never a seventh Core error or a successful partial result.

Ordered input application scans cases from left to right:

- Deposit matches account, depositor, and exact quantity. Only a matching
  nonpositive deposit yields non_positive_deposit. A zero supplied against a
  positive required amount yields no_matching_input.
- Choice matches id and chooser. An in-range value updates the choice and follows
  that first accepting case. A matching out-of-range value records a bounds
  mismatch and continues scanning: a later matching case may still accept.
- When no case accepts, return choice_out_of_bounds iff a matching Choice
  produced a bounds mismatch; otherwise return no_matching_input.
- Input application itself adds zero reductions, warnings, or Core payments.

Transaction evaluation exactly follows `core.py:323`:

1. Validate model-domain inputs separately. OutsideModelDomainA does not pretend
   to be a Python transaction result.
2. If now < original minimumTime, reject with time_before_state.
3. Copy original state with minimumTime=now and reduce to quiescence.
4. With no input: quiescent empty Close with zero reductions/payments/warnings
   rejects contract_closed; quiescent When rejects input_required; other reduced
   outcomes accept.
5. With input: if the pre-reduced continuation is not When, reject contract_closed
   for Close (the source fallback is no_matching_input for any other form).
   Otherwise apply one input, reject its error if any, and reduce the successor.
6. On success concatenate before/after ordered payments and warnings and add
   their actual reduction counts. On every rejection return the original state,
   including original continuation and minimumTime, with empty payments/warnings,
   reductions=0, accepted=false, and the exact error.

The only Core error values are time_before_state, contract_closed, input_required,
no_matching_input, choice_out_of_bounds, and non_positive_deposit.
No fuel error, authorization error, cancellation marker, or signer result enters
that vocabulary.

## Canonical program tables and expected traces

Use separate AProgram values; node ids are local to the equality-bound program.
Populate unused nodes as CloseA.

Canonical swap, root N6:

| Node | Exact content |
| --- | --- |
| N0 | Close |
| N1 | Pay Bob/TokenB 20 to Alice, next N0 |
| N2 | Pay Alice/TokenA 10 to Bob, next N1 |
| N3 | If settle == 1, then N2, else N0 |
| N4 | When [Choice(settle,Bob,0..1) -> N3], timeout 100 -> N0 |
| N5 | When [Deposit(Bob/TokenB,Bob,20) -> N4], timeout 100 -> N0 |
| N6 | When [Deposit(Alice/TokenA,Alice,10) -> N5], timeout 100 -> N0 |

Expected independent comparison vectors:

- Alice deposit at 1: accepted, N5, Alice account 10, minimumTime 1, no choices,
  no payments/warnings, reductions 0. Bob deposit at 2: accepted, N4, both accounts
  10/20, minimumTime 2, reductions 0.
- Choice settle=1 at 2: accepted, Close, choice stored as 1, accounts empty,
  ordered payments [Alice-account -> Bob TokenA 10, Bob-account -> Alice TokenB 20],
  warnings empty, reductions 3 (If plus two Pays).
- Choice settle=0 at 2: accepted, Close, choice stored as 0, ordered owner refunds
  [Alice TokenA 10, Bob TokenB 20], reductions 3 (If plus two Close refunds).
- From funded N4/minimumTime2, no input at 100: accepted owner refunds in that
  same order, minimumTime100, reductions3 (timeout plus two Close refunds).
- From the identical funded predecessor, supplied settle=1 at 100: rejected
  contract_closed, original N4/minimumTime2/accounts/choices restored, empty
  payments/warnings, reductions0. The speculative timeout refund never commits.
- Alice-only funded N5, no input at 100: accepted refund10, reductions2.
  Unfunded N6 timeout: accepted with no payment, reductions1. Neither an empty
  payment list nor an envelope rejection may be relabeled as Core failure.

Proposed installment composition, root N4, prefunded Alice/TokenA=10:

| Node | Exact content |
| --- | --- |
| N0 | Close |
| N1 | Pay Alice/TokenA 5 to Bob, next N0 |
| N2 | When [Choice(fill2,Bob,1..1) -> N1, Choice(recover,Alice,1..1) -> N0], timeout100 -> N0 |
| N3 | Pay Alice/TokenA 5 to Bob, next N2 |
| N4 | When [Choice(fill1,Bob,1..1) -> N3, Choice(recover,Alice,1..1) -> N0], timeout100 -> N0 |

This is a local proposed workload composition using existing constructors, not a
new frozen-Core feature. The two separate When boundaries are essential:
sequential Pay5/Pay5 without the intermediate When would pay ten in one
transaction (`tests/test_core_semantics.py:130`).

At time2, fill1 yields Bob5, Alice account5, continuation N2, stored fill1=1,
and reductions1. Fill2 then yields Bob's second5, no account funds, Close,
stored fill2=1, reductions1. The recover choice at N4 or N2 yields an owner
refund10 or5 through actual Close reduction, with recover=1 and reductions1.
At time100, use no input to execute the timeout refund; a supplied recover
choice follows the same timeout-first rollback rule as the swap.

Separate lifecycle cancellation has no Core transaction: retain the agreement
predecessor and use NoCoreProjection. The proposed refund choice is deliberately
agreement-legal before lifecycle cancellation; the common envelope must reject
its recovery operation until the parent is cancelled and nonce-one authority is
freshly checked, signed, verified, and consumed. This demonstrates the distinct
agreement and authorization boundaries. Do not fabricate a Core error when that
envelope check fails.

The existing generic installment fixture uses NoInput. It is not a Candidate A
codec: future candidate-specific policies must bind the actual ChoiceLike
fill1/fill2/recover inputs and actual projected predecessor facts. This adaptation
is a policy-fixture task, not a change to the common authorization interpreter.

## Independent Python correspondence and extraction boundary

The producer must export a record with exact program bytes/node map, original
AState, AInput, now, raw ATransactionResult, and separately projected
CoreResultObservation plus ordered effects. Include schema version, case id,
fixture id, source/tool hashes, and trace-step id. Never export only accepted
booleans or outcome labels.

The Python checker is a separate implementation:

1. Independently decode principal/asset/choice/time/node symbols. Decode zero
   account entries by omission into canonical sparse tuples. Decode NoInt by
   omission; preserve IntValue(0). Sort Python accounts/choices by their actual
   frozen dataclass/string order; reject duplicate/missing finite keys in export.
2. Independently decode the graph to frozen Python constructors. For swap,
   require structural equality with `canonical_swap(SwapParameters.example())`.
   For installment, construct the independently specified two-When Python tree
   above and require equality. Do not accept a producer-supplied arbitrary graph
   as the expected workload merely because Python can execute it.
3. Call frozen `compute_transaction` on the independently decoded original
   contract, state, input, and now. Compare accepted/error, complete sparse
   accounts, complete choices, decoded continuation tree, minimum time, ordered
   payments, full warning records including optional fields, and reductions.
4. Independently derive effects from the actual reference execution. On rejection
   effects are empty. For accepted transactions, call the pinned Python
   reduce_to_quiescence on the original contract and a copy of state with
   min_time=now to identify the pre-input payment prefix; require it to be a prefix
   of the successful compute_transaction payment list. Emit that prefix's escrow
   debits, then the accepted deposit's Wallet(depositor)->Escrow(account) credit
   if present, then the remaining post-input payment debits. This preserves
   operational effect order when a Pay occurs before input application. The
   producer's extractCommittedEffects takes the original program/state/input/time
   for the same reason: the final payment list alone loses the deposit insertion
   point. Accepted deposit is an effect even when no Core payment is emitted.
   A Deposit immediately followed by Close has deposit then refund effects.
5. Compare independently derived fields/effects to the candidate projection.
   Do not import the producer's decoder, candidate projection helpers, or
   `moriarty.intent.effects_from_payments` into this independent checker.
   Sharing frozen dataclass definitions and invoking the pinned reference
   function is the intended oracle boundary; sharing the mapping under test is not.
6. Reject mismatching frozen hashes before interpreting any case. Preserve the
   exact command, input record, raw output, exit code, seed, and tool hashes.
   A successful comparison covers those records only, not all programs or
   production correspondence.

The common `validCoreObservation` validator at observations.qnt:74 checks
structural/emission consistency. It cannot derive a continuation, reduction
count, or balance transition. EvidenceValid remains the external verification
disposition; neither this tag nor a successful structural validator is the
agreement evaluator or an independent correspondence result.

## Incremental TDD tasks

Each task ends with its named deterministic tests, a reachable state witness
where it adds an action, and a hash-pinned receipt. Keep the failing assertion
and observed failure before changing behavior. Do not classify a missing import
as a semantic regression. No unperformed step is marked reproduced.

### Task 1: finite syntax/state and independent symbol codec contract

Files: create candidate_a_types.qnt, candidate_a_programs.qnt, and
candidate_a_core_test.qnt.

- [ ] Write exact tests named finiteProgramDomainTest, cycleRejectedByDomainTest,
  absentChoiceDiffersFromZeroTest, canonicalAccountOrderTest, and
  canonicalSwapNodeTableTest.
- [ ] Assert total node/account/choice maps; a N1->N1 edge fails validProgram;
  NoInt != IntValue(0); the canonical account list order is the six-key order
  above; the swap root/node map equals the seven-row table.
- [ ] Run `quint typecheck specs/quint/s02/candidate_a_core_test.qnt`, then
  `quint test specs/quint/s02/candidate_a_core_test.qnt --backend=rust --seed=42`.
  Record the deliberate behavioral RED, implement finite checks and literal
  maps, then require all five tests to pass.
- [ ] Do not add a recursive contract type, an unchecked integer node id, or a
  guessed semantic result to make these tests pass.

### Task 2: Close and Pay one-step reduction

Files: create candidate_a_core.qnt; extend candidate_a_core_test.qnt.

- [ ] Add closeEmptyQuiescentTest, closeCanonicalRefundStepTest,
  payExactStepTest, payPartialFourOfTenTest, payFromMissingAccountTest,
  payZeroWarningTest, and payNegativeWarningTest.
- [ ] Exact expectations: Close with Alice3/Bob7 refunds Alice3 first and retains
  Close; Pay10 with balance4 emits paid4, partial_payment(10,4), zero balance,
  successor continuation, and one reduction; missing source emits no payment,
  partial_payment(10,0); Pay0 and Pay-1 emit non_positive_payment(0,0) and
  non_positive_payment(-1,0), respectively, no payment, and still advance once.
- [ ] Observe RED using the same scoped typecheck/test commands; implement only
  these reduceOnce branches; require all new and retained tests to pass.

### Task 3: If, When, quiescence, and ordered input application

Files: extend candidate_a_core.qnt and candidate_a_core_test.qnt.

- [ ] Add ifAbsentEqualsZeroFalseTest, ifStoredZeroTrueTest,
  whenBeforeDeadlineQuiescentTest, timeoutAtDeadlineStepTest,
  timeoutAfterDeadlineStepTest, reductionBoundCompleteTest,
  firstAcceptingCaseOrderTest, laterChoiceCaseCanAcceptTest,
  choiceBoundsMismatchOnlyAfterScanTest, and matchedNonpositiveDepositTest.
- [ ] Use two Choice cases with the same id/chooser: first bound0..0, second1..1;
  chosen1 must select the second. With overlapping0..1 cases, chosen1 selects
  the first continuation. Wrong chooser produces no_matching_input, not
  choice_out_of_bounds. Matching Deposit0 rejects non_positive_deposit;
  supplied0 against Deposit10 rejects no_matching_input.
- [ ] Observe RED, implement If/When and the ordered case fold, then require
  tests to pass. Add the fixed 22-call reduction fold and assert its domain
  failure branch is never used on every literal fixture.

### Task 4: transaction decision tree and complete rollback

Files: extend candidate_a_core.qnt and candidate_a_core_test.qnt.

- [ ] Add timeBeforeStateRollbackTest, emptyCloseRollbackTest,
  waitingWithoutInputRollbackTest, wrongInputRollbackTest,
  outOfBoundsRollbackTest, nonpositiveDepositRollbackTest,
  timeoutNoInputCommitsRefundTest, timeoutWithInputRollsBackRefundTest,
  speculativeWarningRollbackTest, and beforeAfterReductionOrderTest.
- [ ] Every rejecting expected result is the complete original state plus exact
  error, empty ordered payments/warnings, and zero reductions. For
  speculativeWarningRollbackTest use Pay0->waiting When and an invalid supplied
  input: the pre-input non_positive_payment warning must disappear on rejection.
- [ ] For beforeAfterReductionOrderTest use Alice/TokenA initial balance1 and
  Pay1 to Bob -> When Deposit5 by Alice -> Pay5 to Bob -> Close, with supplied
  Deposit5 at time2. The accepted result has payments [Bob1,Bob5], empty accounts,
  reductions2, and minimumTime2. Complete effects are [pre-input Pay1, Deposit5,
  post-input Pay5]; input itself adds no reduction.
- [ ] Observe RED; implement computeTransaction in the exact source order; pass
  the complete-result assertions without weakening clock or rollback fields.

### Task 5: stateful canonical swap and full neutral projection

Files: create candidate_a_projection.qnt, candidate_a_harness.qnt, candidate_a_test.qnt.

- [ ] Add swapSettlementTraceTest, swapVoluntaryRefundTraceTest,
  swapAliceOnlyTimeoutTest, swapFundedTimeoutCommitTest,
  swapFundedTimeoutInputRollbackTest, and depositWithoutPaymentEffectTest.
- [ ] Execute actual separate transactions using the expected vectors above.
  Store original request and result per bounded trace slot. Accepted transitions
  update agreement state; rejected transitions retain it and record the exact
  rejection. Limit the trace schedule explicitly; no unconditional stutter.
- [ ] Implement projectState/projectResult/projectInput with every named field.
  extractCommittedEffects returns empty for rejected results and inserts the
  successful deposit credit between pre-input and post-input payment effects.
  It must preserve the two-payment/deposit ordering vector from Task 4.
- [ ] Run the core and trace test modules separately with Rust seed42. Run
  `quint run specs/quint/s02/candidate_a_harness.qnt --backend=rust --seed=42 --max-samples=100 --max-steps=8 --invariant=coreTraceSafety --witnesses swapSettled swapVoluntaryRefunded swapDeadlineCommitted swapDeadlineInputRejected --verbosity=1`.
  Each witness is an actual recorded transaction outcome, never an initializer
  label. coreTraceSafety must check conservation with deposits, exact rejection
  preservation, valid finite state, and finite-domain completion.

### Task 6: two-installment agreement and recovery projections

Files: extend candidate_a_programs.qnt, candidate_a_core_test.qnt, candidate_a_test.qnt.

- [ ] Add firstFillStopsAtSecondWhenTest, secondFillCompletesTest,
  recoveryChoiceRefundsTenTest, recoveryChoiceRefundsFiveTest,
  installmentTimeoutPriorityTest, and cancellationHasNoCoreResultTest.
- [ ] Assert first fill pays exactly five and stops at N2 with five retained;
  second fill requires a second transaction. Assert both recover-choice
  executions derive their actual Close payment, choices, continuation, and
  reduction count from the agreement evaluator.
- [ ] cancellationHasNoCoreResultTest tests the pure lifecycle projection:
  unchanged agreement state, empty effects, NoCoreProjection. Do not call
  computeTransaction or fabricate accepted=true for this operation.
- [ ] Require all specified traces to pass, then sample them with witnesses
  firstAgreementFill, secondAgreementFill, agreementRefundTen, agreementRefundFive.
  This step establishes only Core/composition and projection behavior; it does
  not integrate common signatures, cancellation authority, or nonce consumption.

### Task 7: independent reference comparison and mutation controls

Files: create both named Python export/checker scripts and the checker test module.

- [ ] Export complete raw cases for every semantic error/warning, both deadline
  paths, each canonical swap outcome, and both installment/refund lengths.
  The exporter performs serialization only, not expected-result construction.
- [ ] Write checker tests named test_frozen_source_hash_required,
  test_absent_choice_is_not_zero, test_continuation_mapping_mutation_rejected,
  test_payment_order_mutation_rejected, test_asset_mapping_mutation_rejected,
  test_reduction_count_mutation_rejected, test_deposit_effect_omission_rejected,
  test_deposit_insertion_order_mutation_rejected,
  test_partial_warning_payload_mutation_rejected, and
  test_deadline_priority_mutation_rejected.
- [ ] Each mutation changes a compared field or mapping while retaining the
  original expected workload and reference semantics. Show one exact differing
  field and nonzero checker exit. A node-id change that decodes to the same
  identical Close tree is an equivalent mapping, not a critical surviving mutant;
  use a structurally different continuation for the critical control.
- [ ] Run `python -m pytest tests/test_s02_candidate_a_correspondence.py -q`.
  Export ITF cases under `raw/experiments/s02-candidate-a-core-2026-09-05/`.
  The exporter CLI is
  `python scripts/export_s02_candidate_a_cases.py --itf-dir raw/experiments/s02-candidate-a-core-2026-09-05 --out raw/experiments/s02-candidate-a-core-2026-09-05/cases.json`.
  Then preserve the full output from
  `python scripts/check_s02_candidate_a_correspondence.py --cases raw/experiments/s02-candidate-a-core-2026-09-05/cases.json --report raw/experiments/s02-candidate-a-core-2026-09-05/comparison-report.json`;
  exit0 requires all source pins, codec checks, and field comparisons to pass.
  Missing exports, unknown tags, domain failures, or any differing field exit1.
- [ ] Independently compare the separately exported projection and effect list
  with reference-derived values; comparing only raw model transactions would
  leave a corrupt extraction adapter untested.

### Task 8: evidence handoff and separate integration boundary

Files: create `evidence/s02-candidate-a-core/evidence-manifest.json` with immutable
raw artifacts under `raw/experiments/s02-candidate-a-core-2026-09-05/`; create
`.superpowers/sdd/candidate-a-implementation-report.md`. These are future
implementation outputs, not files produced by this planning task.

- [ ] Record exact source closure, code/tool hashes, command lines, seed/domain
  bounds, raw outputs, exits, witness counts, and all mutation dispositions.
- [ ] Recheck frozen Python/scope hashes after tests. Mark completed empirical
  checks precisely and all absent Apalache/model-checking evidence as open.
- [ ] Submit the semantic unit for independent review before any authority
  integration. Integration must equality-bind APredecessor, full program,
  transaction input/time, artifact, plan, actual result, and actual effects to
  existing canPropose/canVerify/canCommit APIs. It must use actual candidate-specific
  inputs and both signing profiles, not reuse generic NoInput fixtures as evidence.
- [ ] Keep agreement rejection separate from envelope rejection. Any future
  capability-free refund, stale plan, or missing evidence control uses the shared
  boundary and cannot be repaired by changing this frozen-Core evaluator.
- [ ] Final S02 model checking, all A–D comparisons, package validation, Council
  acceptance, and candidate selection remain outside this semantic-unit plan.

## Plan self-review

The proposed types cover every frozen constructor/action/value/observation and
every observable result field. The source tables and task vectors cover ordered
refunds, absent choices, all six errors, both warning classes, input scan order,
the timeout-priority pair, speculative rollback, and exact reduction counting.
Both installment payments have distinct When boundaries. Cancellation adds no
Core operation; recovery uses an existing Choice/Close composition. Common
authorization and EvidenceValid remain outside semantic derivation.

No code or candidate evidence was produced while writing this plan. The only
artifact changed by this planning task is this document.

## Architect disposition

The root architect read the frozen Core and swap sources in full and inspected
this concrete plan. Adopt the finite type sketch and the first semantic-unit
tasks for local implementation under the already delegated design authority.
The proposed installment topology uses existing constructors and remains an
experimental workload composition, not a change to frozen Core. No additional
human type-sketch vote is required for these specified local tasks. This does
not waive the still-open Council and integration boundaries.

Implement Task 1 and Task 2 first, with executable typechecking and observed
behavioral RED/GREEN evidence. Preserve historical scaffold source and exact
terminal receipts rather than reconstructing them later. No full Candidate A
or correspondence claim follows from those initial tasks.
