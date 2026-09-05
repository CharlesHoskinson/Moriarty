# Candidate B native-obligation graph plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use `superpowers:subagent-driven-development` or `superpowers:executing-plans` task-by-task. This document is planning input only; it authorizes no implementation, selection, Council decision, or S02 gate claim.

**Goal:** Specify an independent finite Candidate B execution representation: a native obligation graph with guarded obligation discharge and a separately checked elaboration to the frozen E00 Core reference for the two required workloads.

**Architecture:** B represents executable meaning as graph nodes, prerequisite sets, native state facts, and emitted effects; no B module imports, invokes, or mirrors `candidate_a_core`. A separate, non-B Python checker elaborates each recorded B transition into a frozen `moriarty.core` contract/state/input/time vector and compares complete reference results with B's recorded raw outcome and ordered effects. The common authorization/envelope model remains a later consumer, not B's interpreter.

**Tech Stack:** Quint 0.32.0 with Rust sampling and Apalache final check; frozen `moriarty/core.py` and `moriarty/swap.py`; Python pytest checker; JSON/ITF evidence.

## Global constraints

- Preserve Core `0.0.0-e00.2`, its receipts, `requirements.json` status `specified-only`, `evidence: []`, and `selected_candidate: null` until later evidence work changes them under separate authority.
- Pin and recheck frozen `moriarty/core.py` SHA-256 `564a926779beb54bd16ff58481f5c950bae6fb50fcd5b0d7c9eb56427f83273b` and `moriarty/swap.py` SHA-256 `82e9e1da76af65706171049f0238953aca5333edec4aeed6caa43dbd57c79797`.
- B is not Candidate A relabeled: no import of `candidate_a_core`, no finite AST-node reduction evaluator, and no shared evaluator used by the B generator and its correspondence checker.
- B may use neutral effects/observations types only; it must not modify common authorization, execution, consumption, observation, or Candidate A files.
- Keep lifecycle cancellation outside frozen Core: it has `NoCoreProjection`; recovery requires the separate nonce-one authorization pipeline later, while B's graph provides the agreement-side refund effect only.
- Model Alice, Bob, Mallory, TokenA, TokenB, values `{0,1,5,10,20}`, times `{1,2,100,101}`, parent nonce `0`, recovery nonce `1`, and both `SignAfterResolve` and `SignBeforeResolve` explicitly. These are bounded corpus assumptions, not universal coverage.
- Candidate B must produce all twelve required predicate checks, eleven required witnesses, both closed recovery subscenarios, and every listed control disposition. A rejected operation is not a positive settlement witness.
- Final evidence must state sampled versus finite-state checked coverage, complete constants, initializer, step, invariants, witnesses, tool versions/hashes, source hashes, seeds, terminal outputs, and remaining limitations.
- Do not claim correspondence, model checking, architecture selection, Council acceptance, cryptographic authenticity, ledger execution, ACTUS completeness, or production feasibility before their named evidence exists.

## B representation and independent elaboration contract

`candidate_b_types.qnt` defines these finite, opaque identifiers before any graph action:

```quint
type BWorkload = SwapB | InstallmentB
type BNodeId = AliceDepositB | BobDepositB | SettleB | RefundB | DeadlineAliceOnlyB
  | DeadlineFundedSwapB | DeadlineEmptySwapB | FillOneB | FillTwoB | RecoverChoiceTenB | RecoverChoiceFiveB
  | DeadlineInstallmentFirstB | DeadlineInstallmentSecondB
type BChoiceId = SettleChoiceB | FillOneChoiceB | FillTwoChoiceB | RecoverChoiceB | OtherChoiceB
type BTime = BTime0 | BTime1 | BTime2 | BTime100 | BTime101
type BContinuation = SwapAwaitAliceDepositB | SwapAwaitBobDepositB | SwapAwaitDecisionB | SwapClosedB
  | InstallmentAwaitFirstB | InstallmentAwaitSecondB | InstallmentClosedB
type BAttemptStatus = NoAttemptB | ReadyAttemptB | CommittedAttemptB
  | RejectedCoreAttemptB(CoreError) | RejectedNativeAttemptB(BError)
type BError = BNativeGuardRejected | BInputOutsideFiniteDomain | BGraphInvariantFailure
type BEvent = BDeposited | BSettled | BRefunded | BTimedOut | BFirstFilled | BSecondFilled
  | BRecoveryChoice | BInputRejected | BGraphRejected
type BCoreTag = SwapAliceDepositTag | SwapBobDepositTag | SwapSettleTag | SwapRefundTag
  | SwapDeadlineNoInputTag | SwapDeadlineInputRollbackTag | FillOneTag | FillTwoTag
  | RecoverTenTag | RecoverFiveTag | NoCoreProjectionTag
type BInput = NoBInput | DepositB({account: CoreAccount, depositor: Principal, quantity: int})
  | ChoiceB({id: BChoiceId, chooser: Principal, value: int})
type BPredicate = RequiresDepositB | RequiresChoiceB | BeforeDeadlineB | AtOrAfterDeadlineB
  | RequiresBalanceB | RequiresUnusedBranchB
type BEffectTemplate = DepositTemplate({account: CoreAccount, depositor: Principal, quantity: int})
  | PaymentTemplate({account: CoreAccount, recipient: Principal, quantity: int})
  | RefundCurrentBalanceTemplate(CoreAccount)
type BObligation = {id: BNodeId, prerequisites: Set[BNodeId], predicates: Set[BPredicate],
  effectTemplates: List[BEffectTemplate], successor: BContinuation,
  excludes: Set[BNodeId], coreTag: BCoreTag}
type BGraph = {workload: BWorkload, root: BNodeId, obligations: BNodeId -> BObligation}
type BRawCoreResult = {accepted: bool, error: CoreError, warnings: List[CoreWarning],
  payments: List[CorePayment], state: CoreStateObservation[BContinuation], reductions: int}
type BState = {enabled: Set[BNodeId], discharged: Set[BNodeId], invalidated: Set[BNodeId],
  attempts: BNodeId -> BAttemptStatus,
  balances: CoreAccount -> int, choices: BChoiceId -> OptionalInt, time: BTime,
  continuation: BContinuation, events: List[BEvent], attemptHistory: List[{node: BNodeId,
    now: BTime, status: BAttemptStatus}]}
type BNativeOutcome = BComputed(BRawCoreResult) | BNativeDiagnostic(BError)
type BResult = {state: BState, transferEffects: List[Transfer], outcome: BNativeOutcome,
  event: BEvent, coreTag: BCoreTag}
type BElaborationRequest = {graph: BGraph, before: BState, input: BInput, now: BTime,
  transition: BNodeId}
```

The graph is a finite dependency DAG, not an AST: an obligation becomes enabled only when all declared prerequisites discharge, it is not invalidated by a competing successful branch, and its native predicates hold. `deriveEffects(state, templates)` resolves each template at discharge time: a deposit is wallet-to-escrow, a payment is escrow-to-wallet, and `RefundCurrentBalanceTemplate(account)` reads that exact current account balance and emits no transfer when it is zero. It also derives matching ordered `CorePayment` values and the complete `BRawCoreResult`; static deadline-payment lists are forbidden. `applyDischarge` changes exactly one obligation to `CommittedAttemptB`, updates balances/choices/continuation, invalidates its exact `excludes` set, and appends one event. A graph-core rejection such as a supplied input after deadline returns `BComputed` with frozen-shaped `contract_closed`, the original projected state/time, empty effects/warnings/payments, and zero reductions; it appends only a time-independent attempt event. A finite-domain/native-guard rejection returns `BNativeDiagnostic`, never a `CoreError`.

A failed attempt is recorded as `RejectedAttemptB(error)` but does not discharge or invalidate the obligation: the same ready node may retry with a different valid input. Attempt history is separate from projected Core state, so a rejection never changes projected continuation, balances, choices, or minimum time. Time advancement recomputes `enabled`; expired input nodes are removed, a deadline node is enabled for the actual phase, and a successful deadline node invalidates that phase's competing input nodes. A graph has one fixed root, a complete finite node map, no self-edge, no unknown dependency, no cycle by rank, and mutually exclusive branches declared symmetrically.

`BTime0` is the sole initializer time and is never a successful transaction `now`; every admitted transition takes an explicit `now` in `{BTime1,BTime2,BTime100,BTime101}`. A transition with `now < state.time` records `RejectedCoreAttemptB(CoreErrorCode("time_before_state"))`, retaining projected state and time. B admits input values `{-1,0,1,5,10,20,21}` so frozen non-positive-deposit and mismatch vectors remain representable; negative quantities are disabled for successful deposit effects. Values outside that set (for example 22) yield `BNativeDiagnostic(BInputOutsideFiniteDomain)`, not a Core error. The literal `DepositB({account: aliceA, depositor: Alice, quantity: -1})` is retained in `negativeMatchedDepositDiagnosticB`: it has the same account/depositor identity as its graph edge and elaborates to the frozen `non_positive_deposit` Core result, but it produces no successful transfer. This deliberately differs from the common successful-envelope time/value bounds `{1,2,100,101}` while preserving the frozen-Core error corpus.

The native graph has no parent-authority, signing, evidence, cancellation, or recovery-nonce state. Frozen agreement recovery choices are legal graph obligations before lifecycle cancellation, exactly as the adopted separation requires. Later common-boundary integration consumes B-derived complete observations through existing `authorization.qnt`/`execution.qnt` APIs; it alone determines cancellation race, nonce 0/1, signatures, evidence, and the closed recovery subscenarios.

The independently written elaborator has this input/output boundary, implemented outside Quint B source:

```text
elaborate_b_transition(request: BElaborationRequest) -> FrozenCoreVector | NoCoreProjection
run_frozen_core(vector: FrozenCoreVector) -> FrozenCoreResult
compare_b_record(record: BResult, reference: FrozenCoreResult | NoCoreProjection) -> list[Difference]
```

It constructs frozen Python `Close`, `Pay`, `If`, `When`, `Deposit`, and `Choice` values only from a fixed B tag-to-vector table, calls frozen `compute_transaction`, and compares `BRawCoreResult` field-for-field plus B's ordered `Transfer` list against independently derived deposit/payment effects. The checker must not import B's Quint source or share B's transition/guard code. Lifecycle cancellation records elaborated by the later common-boundary task use `NoCoreProjectionTag` only.

## File structure

| File | Responsibility |
| --- | --- |
| `specs/quint/s02/candidate_b_types.qnt` | Finite B graph/state/result carriers, validity and rank predicates. |
| `specs/quint/s02/candidate_b_graphs.qnt` | Literal native swap/installment graph tables and graph-only elaboration tags. |
| `specs/quint/s02/candidate_b_native.qnt` | Pure enable, guard, discharge, rejection, and native safety functions. |
| `specs/quint/s02/candidate_b_harness.qnt` | Stateful guarded trace actions, both profile initializers, witnesses, and invariants. |
| `specs/quint/s02/candidate_b_test.qnt` | Deterministic graph, race, recovery, and mutation vectors. |
| `scripts/export_s02_candidate_b_cases.py` | Serializes recorded B vectors; computes no expected outcome. |
| `scripts/check_s02_candidate_b_elaboration.py` | Independently builds frozen Python vectors, invokes the reference, and reports field differences. |
| `tests/test_s02_candidate_b_elaboration.py` | Checker tests and independent mutation controls. |
| `raw/experiments/s02-candidate-b-2026-09-05/` | Immutable command outputs, ITF traces, cases, checker report, and tool receipt. |
| `evidence/s02-candidate-b/evidence-manifest.json` | Hashes, commands, scope, coverage, and open limitations. |

## Required coverage matrix

| Registry item | B graph/check obligation |
| --- | --- |
| `complete-signed-effects`, `asset-conservation`, `nonnegative-balances` | Exact ordered effect list equals discharged obligation; native balance update preserves each asset total and nonnegativity. |
| `authorized-refunds`, `nonce-replay-exclusion`, `cancel-fill-exclusion`, `residual-authority-conservation` | B proves only agreement-side refund/remaining-balance facts. The later existing common-boundary integration must prove nonce 0/1, cancellation exclusion, and signed authority; do not attribute those lifecycle predicates to B alone. |
| `authority-bindings`, `rejection-preservation`, `display-is-not-authority`, `settlement-evidence-level`, `nonterminal-enabled` | B binds graph/tag/input/time/effects and preserves financial state on native rejection. A later existing-envelope task binds version/anchor/signatures/evidence/display; B's nonterminal state has a real enabled native action. |
| settlement, voluntary/deadline refund, deadline-input rollback, extra-effect rejection | Swap graph nodes and checker vectors, including exact deadline no-input commit versus deadline supplied-input rollback. |
| first residual, second completion | Installment graph nodes: `FillOneB` preserves exactly five and enables `FillTwoB` or agreement-legal `RecoverChoiceFiveB`; their effects are native ordered transfers/core records. |
| cancel-wins, fill-wins, after/before resolve execution | Explicit later integration obligation: feed B-derived observations into existing common authorization/execution harness under both profiles; retain the loser rejection and exact nonce/escrow outcomes there. |
| cancel-wins recovery, fill-wins recovery | Explicit later integration obligation: pair B's `RecoverChoiceTenB`/`RecoverChoiceFiveB` with lifecycle cancellation and existing nonce-one pipeline, then check the two closed registry terminals. |
| all 14 controls | Execute the control table in Task 5 below and record counterexample/redundant/blocked disposition with path and predicate. |

### Task 1: finite graph carriers and literal graph tables

**Files:** Create `candidate_b_types.qnt`, `candidate_b_graphs.qnt`, `candidate_b_test.qnt`.

**Interfaces:** `validBGraph(graph)`, `validBState(graph,state)`, `enabledNodes(graph,state)`, `canonicalSwapB`, `installmentB`, and tag-only `BElaborationRequest` records. No native action yet.

- [ ] Write failing tests for a missing node key, cycle, undeclared dependency, duplicate effect-template order, wrong swap deposit account/depositor/amount, absent choice versus stored zero, every initial/intermediate/closed continuation constructor, `BTime0` scope, and outside-finite-domain input diagnostic. Do not test a recovery nonce in B-native types.
- [ ] Run `quint test specs/quint/s02/candidate_b_test.qnt --backend=rust --seed=42`; expect assertion failures naming unavailable graph/domain functions, not parser errors.
- [ ] Implement complete finite types and graph validity before graph constructors. Initialize swap at `SwapAwaitAliceDepositB` and installment at `InstallmentAwaitFirstB`; use the matching closed continuations after every accepted terminal transition. Build swap nodes: `AliceDepositB` requires `DepositB({account:Alice/TokenA,depositor:Alice,quantity:10})`, derives wallet-to-escrow 10, and moves to `SwapAwaitBobDepositB`; `BobDepositB` derives Bob/TokenB 20 and moves to `SwapAwaitDecisionB`; `SettleB` requires both deposits plus `ChoiceB(settle,Bob,1)` and derives `[escrow Alice/A->Bob:10, escrow Bob/B->Alice:20]`; `RefundB` requires choice zero and derives current-balance refunds in canonical account order. `DeadlineAliceOnlyB` and `DeadlineFundedSwapB` are separately guarded at time 100 and use `RefundCurrentBalanceTemplate`, yielding respectively `[Alice:10]` and `[Alice:10,Bob:20]` (or empty only for an admitted zero-balance phase). Build installment nodes: `FillOneB` requires `ChoiceB(fill1,Bob,1)`, derives 5 to Bob, and moves to `InstallmentAwaitSecondB`; `FillTwoB` derives the second 5 and closes; `RecoverChoiceTenB`/`RecoverChoiceFiveB` require `ChoiceB(recover,Alice,1)` and derive the current 10/5 Alice refund. `DeadlineInstallmentFirstB` and `DeadlineInstallmentSecondB` use the same current-balance refund template at their own phases. For every timeout phase, also encode the supplied-input path as a native `BComputed` full rollback result, not as a deadline discharge. No cancellation node exists in the B agreement graph.
- [ ] Add equality tests for every graph node, prerequisite set, effect template, tag, time, amount, account/depositor identity, successor, exclusive set, and actual-balance-derived timeout result. Run the same command; expect all Task 1 tests passing.
- [ ] Record source hashes and RED/GREEN output in the task report; do not generate a Core result from B.

### Task 2: native graph transition semantics and safety floor

**Files:** Create `candidate_b_native.qnt`, `candidate_b_harness.qnt`; extend `candidate_b_test.qnt`.

**Interfaces:**

```quint
pure def canDischarge(graph: BGraph, state: BState, node: BNodeId, input: BInput, now: BTime): bool
pure def applyDischarge(graph: BGraph, state: BState, node: BNodeId, input: BInput, now: BTime): BResult
pure def canReject(graph: BGraph, state: BState, node: BNodeId): bool
pure def applyRejection(state: BState, node: BNodeId, reason: BError): BResult
pure def bSafety(graph: BGraph, state: BState): bool
```

- [ ] Write failing deterministic tests for both deposits, settlement, settle=0 refund, Alice-only/funded swap deadlines, supplied swap-deadline input rollback, first-fill residual, second-fill completion, first-stage/second-stage installment deadlines, supplied installment-deadline input rollback, and both agreement recovery-choice refunds. Include exact native `BRawCoreResult` fields and deposit `Transfer` effects.
- [ ] Write failing invariant tests for exact ordered transfers/payments, asset conservation, nonnegative balances, native remaining balance 10/5/0, rejection preservation, retry-after-rejection, exclusive-branch invalidation, expiry invalidation, and enabledness outside terminal state. Nonce and lifecycle authority predicates are reserved for the later common-boundary integration task.
- [ ] Implement guards separately from updates. Every B harness action assigns `bState'` and `lastResult'`; no no-op/stutter action is permitted. Profile state belongs only to the later common-API integration harness.
- [ ] Run `quint test specs/quint/s02/candidate_b_test.qnt --backend=rust --seed=42`; expect native vectors and invariant tests passing. Then run `quint run specs/quint/s02/candidate_b_harness.qnt --backend=rust --seed=42 --max-samples=100 --max-steps=16 --invariant=bSafety --witnesses settlement voluntaryRefund deadlineRefund firstInstallmentResidual secondInstallmentCompletion recoveryChoiceTen recoveryChoiceFive --verbosity=1`; require nonzero named witnesses and no invariant violation.

### Task 3: B-derived observation export and later common-boundary integration plan

**Files:** Extend `candidate_b_harness.qnt`, `candidate_b_test.qnt`.

**Interfaces:** `projectBObservation(result: BResult): CandidateObservation[...]` and `projectBPlanView(graph,node): PlannedOperation[...]`. The future integration calls existing `canPrepareSigning`, `canSign`, `canPropose`, `canVerify`, and `canCommit`; B defines none of those functions.

- [ ] Write RED tests that B observations bind the exact graph, node, input, time, native transfer list, full `BRawCoreResult`, and elaboration tag; substitution of any field changes the projection.
- [ ] Write a separate integration-plan test list, not B implementation: recovery unavailable before lifecycle cancellation; nonce one unused until signing; missing/invalid evidence rejected; wrong refund rejected; replay rejected; and both closed registry subscenarios under both profiles. Required-event lists remain membership requirements, not total orders.
- [ ] Implement only B projection data. Do not add B-local signing, cancellation, nonce, evidence, or authority records and do not duplicate `authorization.qnt`/`execution.qnt`.
- [ ] Defer the `cancelWins`, `fillWins`, `recoveryTen`, and `recoveryFive` sampled witnesses to the separately reviewed existing-common-API integration task; until then mark them required/open rather than attributing them to B.

### Task 4: independent frozen-Core elaboration checker

**Files:** Create exporter, checker, and checker test files listed above.

**Interfaces:** Export records `{case_id, graph_hash, transition, before, input, now, b_result, b_effects, core_tag}`. The checker owns its tag-to-frozen-Python-vector table and returns one field-level difference per mismatch.

- [ ] Write pytest RED tests for a good swap deposit, settled swap, voluntary refund, deadline no-input refund, deadline supplied-input rollback, fill one, fill two, recovery ten, and recovery five. Lifecycle cancellation `NoCoreProjection` records belong to the later common-boundary integration checker.
- [ ] Add corrupted record controls: changed continuation tag, payment order, asset, reduction count, absent-vs-zero choice, minimum time, error, and projection presence. Each test must assert checker exit nonzero and its named differing field.
- [ ] Implement the checker by constructing frozen Python objects and calling `compute_transaction`; do not import `candidate_a_core`, call a B Quint evaluator, or compare a generated expectation to itself.
- [ ] Run `python -m pytest tests/test_s02_candidate_b_elaboration.py -q` and the checker command over exported ITF cases. Preserve raw stdout/stderr, exit code, and source hashes. A checker mismatch blocks B correspondence evidence.

### Task 5: control table, final checks, and evidence handoff

**Files:** Extend `candidate_b_test.qnt`, checker tests, evidence manifest, and task report.

- [ ] Write and execute B-applicable controls: remove Bob-deposit prerequisite; substitute executed artifact; reverse deadline priority; replay fill one; expand residual; corrupt B elaboration; agreement-only advancement; intent-only consumption; corrupt extraction; substitute plan after signing; omit state verification; and corrupt an abstraction map. Keep `bridge-corruption` and `stale-bridge` as unresolved registry obligations pending controller disposition: the comparison design assigns controls by representation, but it does not itself authorize an N/A result. Do not claim all fourteen controls covered until that disposition is recorded.
- [ ] For each control, preserve all other premises and record either a reachable counterexample plus violated predicate, or a justified redundant-equivalent disposition naming the independent guard that still rejects it. A mere disabled unreachable action is not mutation evidence.
- [ ] Run deterministic tests, the bounded Rust sampled run, `quint verify specs/quint/s02/candidate_b_harness.qnt --backend=apalache` with explicit finite constants, and the independent checker. Preserve exact commands, versions, binary hashes, seeds, bounds, traces, and terminal outputs.
- [ ] Populate `evidence/s02-candidate-b/evidence-manifest.json` only from produced artifacts. Mark unperformed checks and all excluded claims open. Request independent spec/quality review before any authority integration.

## Falsification vectors and retained dissent

## Fourth-draft change disposition and executable finite boundary

The literal graph tables are total over `BNodeId`. `canonicalSwapB.root = AliceDepositB`;
`AliceDepositB -> BobDepositB -> SettleB/RefundB`, with timeout edges respectively
`DeadlineEmptySwapB` (no escrow), `DeadlineAliceOnlyB` (Alice/A=10), and
`DeadlineFundedSwapB` (Alice/A=10,Bob/B=20). `DeadlineEmptySwapB` emits `[]`, so
empty timeout is not represented by an Alice-only refund node. `installmentB.root =
FillOneB`; `FillOneB -> FillTwoB`, while `RecoverChoiceTenB` is enabled only before
FillOneB and `RecoverChoiceFiveB` only after it. Every unused table key is an explicit
closed/unreachable obligation with empty templates, no prerequisites, and no successor;
it cannot become enabled because `enabledNodes` also requires membership in the graph's
phase-node set.

The finite state bound is: 13 node IDs; at most 13 discharged and 13 invalidated IDs;
six Core accounts each in `0..20`; five choices each `NoInt|-1|0|1|2`; five times;
and at most 16 attempt-history/events in a harness trace. A graph validity check requires
unique keys, a complete table, rank-decreasing prerequisites/successor edges, templates
with positive successful quantities, and exclusive sets symmetric. `deriveEffects` reads
the current bounded account balance at timeout; it never selects a fixed refund list.

`applyDischarge(...,now)` first rejects backward time as the retained Core error above.
On a matched graph input at or after timeout it returns the frozen-shaped rejected Core
record, does not discharge/invalidate/update projected fields, and appends only an attempt
history entry. On a native domain/guard failure it returns `BNativeDiagnostic`; it is not
a `CoreError`. Thus a rejected attempt never blocks a legal later timeout or recovery
choice, and only a committed exclusive branch invalidates competitors.

Remaining unresolved items: B still covers a fixed corpus rather than arbitrary Core
programs; bridge-corruption and stale-bridge controls require a later controller
disposition and are not N/A; common cancellation, evidence, signatures, and nonce-one
recovery remain outside B. This fourth draft is planning input only, not implementation,
model-checking evidence, correspondence, adoption, or a Council result.

- A graph that admits settlement before Bob's deposit falsifies `RequiresDepositB`; a passing mutant means B is not an independent obligation graph.
- An elaboration tag that reverses timeout priority or payment order must differ from frozen Python output; a surviving mutant falsifies checker independence.
- A graph that treats absent `settle` as zero, clamps time, or commits a deadline-supplied input falsifies frozen-Core compatibility.
- A B recovery-choice record that refunds 10 after fill one, preserves a competing fill-two branch after a selected refund, or emits a noncanonical effect order falsifies B's agreement-side graph. A later common-boundary record that reuses nonce zero, consumes authority on rejected evidence, or leaves slot two authorized after recovery falsifies the closed recovery terminals.
- A B graph can encode the required frozen subset only through a fixed obligation/tag corpus. It does not prove that arbitrary E00 `If`/`When` trees, arbitrary case lists, or all integer/time values elaborate without a hidden interpreter. Preserve this as dissent/open question; expand the corpus only with a separately reviewed semantics decision.
- B's graph-native meaning and checker elaboration are intentionally two artifacts. The checker validates selected vectors, not a theorem that every graph equals frozen Core. A critical elaboration mutant survivor blocks B from claiming even bounded correspondence.

## Plan self-review

Tasks 1–2 define types before actions, use TDD RED/GREEN evidence, and cover B-native graph safety plus agreement-side recovery choices. Task 3 keeps lifecycle authorization deliberately in the existing common APIs and leaves the closed recovery terminals open until that separate integration task. Task 4 prevents B/A interpreter reuse and compares complete frozen results independently. Task 5 leaves the two bridge controls unresolved rather than claiming N/A coverage and distinguishes model checking from sampling. The plan deliberately leaves Council acceptance, architecture selection, production correspondence, cryptography, ledger execution, and unbounded completeness open.
