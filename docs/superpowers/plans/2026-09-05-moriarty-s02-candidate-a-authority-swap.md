# Candidate A Swap Authority Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Execute A3-S01–S08 with the canonical raw swap and frozen A authority checks under both signing profiles.

**Architecture:** Independent literal swap fixtures specify original requests, whole programs/maps, results, ordered effects and policy facts. Separate guarded helpers/harness drive sequential funding, nonce1 dispositions and retained rejection. Scenario state selects finite positive/refusal paths; neither tags nor generic string fixtures supply semantic results.

**Tech Stack:** Quint0.32.0/Rust; frozen common/A boundary source `d14cfea98a1c9213e5ef5f12c1a088f4e966083d`.

## Global Constraints

- Status: specified-only corrected A1 draft; no claim that embedded code typechecks or has run. Root independently reviews/adopts after A0 before implementation.
- Controlling authority: approved `deliverables/moriarty-candidate-a-completion-prompt-2026-09-05.xml`, semantic contract, A1 and A3-S01–S08.
- Keep canonical raw swap at N6/minimumTime0. Authority environment starts physicalTime1. Alice deposit at1 advances candidate minimumTime to1; explicit environment advance to2 precedes Bob deposit.
- Preserve full sixteen-node program, six account entries, five optional choices, original call, raw result/error/warnings/reductions, ordered payments/effects and exact context.
- Use canPrepareSigningAuthorityA, canSignAuthorityA, canVerifyAuthorityA and canCommitAuthorityA before unchanged common updates. Cancellation is outside this swap unit.
- EvidenceValid is a finite symbolic external premise. PreparedAttempt.actor is retained metadata, not authenticated sender identity. S08 wrong-actor means wrong Core depositor/chooser or signed.signer; do not change common semantics or claim metadata authentication.
- Empty timeout accepted Core/no effects is rejected by the common nonparent envelope. No invented signer/effect/authorized commit.
- One DispositionAttempt means a stale or rejected disposition cannot be overwritten with another disposition. Successful timeout and deadline refusal use separate scenarios.
- No common, Python, adapter/boundary, DB, evidence or other candidate edits. No implementation or commit during this plan-only task.
- Root owns A3 OpenSpec/EARS, evidence receipts, review/adoption and eventual commit. A3 worker owns only the four new modules below. Each task ends with a nonauthor source/spec/runtime gate; Council and separate model checking remain later work.

## Files, imports and interfaces

Create `specs/quint/s02/candidate_a_authority_swap_fixtures.qnt`, `candidate_a_authority_swap.qnt`, `candidate_a_authority_swap_harness.qnt`, `candidate_a_authority_swap_test.qnt`. Module names match file stems. Full preamble for each:

```quint
import effects.* from "./effects"
import consumption.* from "./consumption"
import observations.* from "./observations"
import policies.* from "./policies"
import authorization.* from "./authorization"
import execution.* from "./execution"
import candidate_a_types.* from "./candidate_a_types"
import candidate_a_programs.* from "./candidate_a_programs"
import candidate_a_core.* from "./candidate_a_core"
import candidate_a_projection.* from "./candidate_a_projection"
import candidate_a_authority_adapter.* from "./candidate_a_authority_adapter"
import candidate_a_authority_boundary.* from "./candidate_a_authority_boundary"
```

Lifecycle additionally imports fixtures; harness imports fixtures/lifecycle; test imports fixtures/lifecycle and later namespaced harness. No generic string harness imports. Read `candidate_a_harness.qnt` as source reference; there is no candidate_a_swap_harness module.

| Requirement | Deterministic coverage under both profiles | Expected result |
|---|---|---|
| S01 | fundingTest, all positive funded routes | Alice nonce0 → actual10 into aliceA; Bob nonce0 →20 into bobB; wallets debited only at commit |
| S02 | SettleS | fresh Alice/Bob nonce1 prepare/register under funded context; settle1 → N0/Bob A10/Alice B20 |
| S03 | RefundS | settle0 → ordered Alice A10 then Bob B20 refunds; both nonce1 keys consumed |
| S04 | TimeoutS at100/101, funded2 and Alice-only1 | NoInput → refunds10/20 or10; correct one/two authority keys |
| S05 | RefuseS at100/101 for chosen0/1 | genuine contract_closed, original N4/minTime2/accounts/choices retained, zero payments/warnings/effects/reductions |
| S06 | staleSigningTest/staleVerifiedTest | environment advance invalidates exact prepared snapshot/verified context, retained rejection and no money movement |
| S07 | TimeoutS funding0 at100/101 | actual accepted Core timeout with reductions1/noeffects; UnauthorizedEffect retained; candidate remains N6/minTime0 |
| S08 | mutationTest/identityBindingTest/wrongActorTest | wrong Core actor/signer/nonce, changed second operation, unused node/result/effects/facts fail frozen A boundary |

## Task 1: independent swap fixtures and compiling behavioral RED

**Files:** fixtures and test. **Produces:** ScenarioS, plannedS, planS, policyS, expectedS, actualS, unsignedS.

- [ ] Create the complete fixture below with RED defect: change only final settlement expected reductions from3 to2 in plannedS. Add literalFidelityTest, typecheck, then capture actual equality assertion failure/source closure before correcting it. Type/import errors never qualify as RED.
- [ ] Restore exact3 and run GREEN. Literal expected results are never adapted/projected from producer outputs.

```quint
type ModeS = SettleS | RefundS | TimeoutS(ATime) | RefuseS({now: ATime, chosen: int})
type ScenarioS = {funded: int, mode: ModeS}
pure val SCENARIOS_S: Set[ScenarioS] = Set(
  {funded: 2, mode: SettleS}, {funded: 2, mode: RefundS}).union(
  Set(0, 1, 2).map(n => Set(Time100, Time101).map(t => {funded: n, mode: TimeoutS(t)})).flatten()).union(
  Set(Time100, Time101).map(t => Set(0, 1).map(c => {funded: 2, mode: RefuseS({now: t, chosen: c})})).flatten())
pure val PROFILES_S = Set(SignAfterResolve, SignBeforeResolve)
pure def keyS(principal: Principal, nonce: int): AuthorityKey = {domain: SwapDomain, principal: principal, nonce: nonce}
pure val programS: AProgram = {root: N6, nodes: Map(
  N0 -> CloseA,
  N1 -> PayA({account: bobB, payee: Alice, amount: ConstantA(20), continuation: N0}),
  N2 -> PayA({account: aliceA, payee: Bob, amount: ConstantA(10), continuation: N1}),
  N3 -> IfA({observation: ChoiceEqualsA({id: SettleId, expected: 1}), thenNode: N2, elseNode: N0}),
  N4 -> WhenA({cases: List({caseAction: ChoiceA({id: SettleId, chooser: Bob, lower: 0, upper: 1}), continuation: N3}),
    timeout: Time100, timeoutNode: N0}),
  N5 -> WhenA({cases: List({caseAction: DepositA({account: bobB, depositor: Bob, amount: ConstantA(20)}), continuation: N4}),
    timeout: Time100, timeoutNode: N0}),
  N6 -> WhenA({cases: List({caseAction: DepositA({account: aliceA, depositor: Alice, amount: ConstantA(10)}), continuation: N5}),
    timeout: Time100, timeoutNode: N0}),
  N7 -> CloseA, N8 -> CloseA, N9 -> CloseA, N10 -> CloseA, N11 -> CloseA, N12 -> CloseA,
  N13 -> CloseA, N14 -> CloseA, N15 -> CloseA)}
pure def accountsS(a: int, b: int): CoreAccount -> int = Map(
  {owner: Alice, asset: TokenA} -> a, {owner: Alice, asset: TokenB} -> 0,
  {owner: Bob, asset: TokenA} -> 0, {owner: Bob, asset: TokenB} -> b,
  {owner: Mallory, asset: TokenA} -> 0, {owner: Mallory, asset: TokenB} -> 0)
pure def choicesS(choice: OptionalInt): AChoiceId -> OptionalInt = Map(SettleId -> choice,
  FirstFillId -> NoInt, SecondFillId -> NoInt, RecoveryId -> NoInt, OtherId -> NoInt)
pure def neutralChoicesS(choice: OptionalInt): str -> OptionalInt = Map("settle" -> choice,
  "fill1" -> NoInt, "fill2" -> NoInt, "recover" -> NoInt, "other" -> NoInt)
pure def beforeS(n: int): APredecessor = {program: programS, state: {
  continuation: if (n == 0) N6 else if (n == 1) N5 else N4,
  accounts: accountsS(if (n >= 1) 10 else 0, if (n == 2) 20 else 0),
  choices: choicesS(NoInt), minimumTime: if (n == 0) Time0 else if (n == 1) Time1 else Time2}}
pure def ledgerS(n: int): Ledger = LEDGER_KEYS.mapBy(_ => 0)
  .put((Wallet(Alice), TokenA), if (n == 0) 10 else 0)
  .put((Wallet(Bob), TokenB), if (n < 2) 20 else 0)
  .put((Escrow(aliceA), TokenA), if (n >= 1) 10 else 0)
  .put((Escrow(bobB), TokenB), if (n == 2) 20 else 0)
pure def environmentS(t: int): Environment = {physicalTime: t, anchor: 0, implementationVersion: 0, enforcementMechanism: 0}
pure def factsS(n: int, t: int): PolicyFacts = {
  ledger: ledgerS(n), environment: environmentS(t), parents: AUTHORITY_KEYS.mapBy(_ => ParentAbsent)}
pure def timeS(mode: ModeS): ATime = match mode {
  | SettleS => Time2 | RefundS => Time2 | TimeoutS(t) => t | RefuseS(r) => r.now }
pure def rejectedS(mode: ModeS): bool = match mode { | RefuseS(_) => true | _ => false }
pure def chosenS(mode: ModeS): OptionalInt = match mode {
  | SettleS => IntValue(1) | RefundS => IntValue(0) | RefuseS(r) => IntValue(r.chosen) | _ => NoInt }
pure def dispositionInputS(mode: ModeS): AInput = match chosenS(mode) {
  | NoInt => NoAInput | IntValue(c) => PresentAInput(ChoiceInputA({id: SettleId, chooser: Bob, chosen: c})) }
pure def neutralInputS(mode: ModeS): NeutralInput = match chosenS(mode) {
  | NoInt => NoInput | IntValue(c) => ChoiceLike({id: "settle", chooser: Bob, chosen: c}) }
pure def operationS(mode: ModeS): Operation = match mode {
  | SettleS => OpSettle | RefundS => OpVoluntaryRefund | TimeoutS(_) => OpDeadlineRefund
  | RefuseS(r) => if (r.chosen == 1) OpSettle else OpVoluntaryRefund }
pure def paymentS(account: CoreAccount, recipient: Principal, q: int): CorePayment = {
  source: account, recipient: recipient, asset: account.asset, quantity: q}
pure def transferS(account: CoreAccount, recipient: Principal, q: int): Transfer = {
  source: Escrow(account), destination: Wallet(recipient), asset: account.asset, quantity: q}
pure def projectedS(before: APredecessor, accepted: bool, node: NodeId, a: int, b: int,
  choice: OptionalInt, t: int, payments: List[CorePayment], reductions: int): CoreProjection[AContinuation] =
  CoreProjected({accepted: accepted, error: if (accepted) NoCoreError else CoreErrorCode("contract_closed"),
    warnings: List(), payments: payments, reductions: reductions, state: {
      accounts: accountsS(a, b), choices: neutralChoicesS(choice), continuation: {program: programS, node: node}, minimumTime: t}})
pure def fundingPlannedS(principal: Principal): AAuthorityPlanned = {
  val n = if (principal == Alice) 0 else 1
  val account = if (principal == Alice) aliceA else bobB
  val q = if (principal == Alice) 10 else 20
  val now = if (principal == Alice) Time1 else Time2
  val effect = {source: Wallet(principal), destination: Escrow(account), asset: account.asset, quantity: q}
  {predecessor: beforeS(n), proposedSuccessor: beforeS(n + 1),
    artifactAndCall: AgreementCallA({before: beforeS(n),
      input: PresentAInput(DepositInputA({account: account, depositor: principal, quantity: q})), now: now}),
    input: DepositLike({location: Escrow(account), depositor: principal, asset: account.asset, quantity: q}),
    operation: OpFund, transactionTime: timeValue(now), effects: List(effect), predecessorFacts: factsS(n, timeValue(now)),
    coreProjection: projectedS(beforeS(n), true, if (principal == Alice) N5 else N4, 10,
      if (principal == Alice) 0 else 20, NoInt, timeValue(now), List(), 0)}
}
pure def dispositionPlannedS(s: ScenarioS): AAuthorityPlanned = {
  val before = beforeS(s.funded)
  val rejected = rejectedS(s.mode)
  val settlement = s.mode == SettleS
  val now = timeS(s.mode)
  val payments = if (rejected or s.funded == 0) List() else if (s.funded == 1) List(paymentS(aliceA, Alice, 10))
    else if (settlement) List(paymentS(aliceA, Bob, 10), paymentS(bobB, Alice, 20))
    else List(paymentS(aliceA, Alice, 10), paymentS(bobB, Bob, 20))
  val effects = if (rejected or s.funded == 0) List() else if (s.funded == 1) List(transferS(aliceA, Alice, 10))
    else if (settlement) List(transferS(aliceA, Bob, 10), transferS(bobB, Alice, 20))
    else List(transferS(aliceA, Alice, 10), transferS(bobB, Bob, 20))
  val after = if (rejected) before else {program: programS, state: {continuation: N0,
    accounts: accountsS(0, 0), choices: choicesS(chosenS(s.mode)), minimumTime: now}}
  {predecessor: before, proposedSuccessor: after,
    artifactAndCall: AgreementCallA({before: before, input: dispositionInputS(s.mode), now: now}),
    input: neutralInputS(s.mode), operation: operationS(s.mode), transactionTime: timeValue(now),
    effects: effects, predecessorFacts: factsS(s.funded, timeValue(now)),
    coreProjection: if (rejected) projectedS(before, false, N4, 10, 20, NoInt, 2, List(), 0)
      else projectedS(before, true, N0, 0, 0, chosenS(s.mode), timeValue(now), payments,
        if (settlement or s.mode == RefundS) 3 else 1 + s.funded)}
}
pure def plannedS(id: AttemptId, s: ScenarioS): AAuthorityPlanned =
  if (id == FundingOneAttempt) fundingPlannedS(Alice)
  else if (id == FundingTwoAttempt) fundingPlannedS(Bob) else dispositionPlannedS(s)
pure def planS(id: AttemptId, s: ScenarioS): AResolvedPlan = {identity: SwapPlanA, operations: List(plannedS(id, s))}
pure def expectedS(id: AttemptId, s: ScenarioS): AAuthorityObservation = {
  val p = plannedS(id, s)
  {predecessor: p.predecessor, proposedSuccessor: p.proposedSuccessor, artifactAndCall: p.artifactAndCall,
    resolvedPlan: planS(id, s), input: p.input, transactionTime: p.transactionTime, effects: p.effects,
    coreProjection: p.coreProjection, effectEvidence: EvidenceValid, display: PublicDisplay,
    outcome: if (id != DispositionAttempt) FundingAccepted
      else if (rejectedS(s.mode)) Rejected(CoreRejected(CoreErrorCode("contract_closed")))
      else if (s.mode == SettleS) Settlement else if (s.mode == RefundS) VoluntaryRefund else DeadlineRefund}
}
pure def actualS(id: AttemptId, s: ScenarioS): AAuthorityAdaptation = {
  val p = plannedS(id, s)
  match p.artifactAndCall {
    | AgreementCallA(r) => adaptAuthorityA(r, p.operation, planS(id, s), PublicDisplay)
    | _ => AuthorityInvalidRequestA }
}
pure def policyS(id: AttemptId, s: ScenarioS, principal: Principal, profile: SigningProfile): AAuthorityPolicy = {
  val p = plannedS(id, s)
  val location = if (id != DispositionAttempt) Wallet(principal) else Escrow(if (principal == Alice) aliceA else bobB)
  val body: PolicyBody = {key: keyS(principal, if (id == DispositionAttempt) 1 else 0),
    debitLocations: Set(location), capabilities: Set(if (id == DispositionAttempt) DisposeCapability else FundCapability),
    disclosures: Set(), validFrom: p.transactionTime, validUntil: p.transactionTime,
    implementationVersion: 0, enforcementMechanism: 0,
    clauses: List({operation: p.operation, conditions: List(InputIs(p.input)),
      requiredEffects: p.effects, allowedEffects: p.effects, effectOrder: ExactOrder})}
  val plan = planS(id, s)
  {body: body, profile: profile, binding: if (profile == SignAfterResolve)
    AfterResolution({identity: plan, operations: plan.operations}) else BeforeResolution(AnyArtifactUnderMechanism)}
}
pure val unsignedS: AAuthorityExecution = {authority: {context: {candidate: beforeS(0),
  ledger: ledgerS(0), environment: environmentS(1), registry: AUTHORITY_KEYS.mapBy(_ => AuthorityUnused),
  parents: AUTHORITY_KEYS.mapBy(_ => ParentVacant)}, signing: AUTHORITY_KEYS.mapBy(_ => NoSigningCheck)},
  attempts: ATTEMPT_IDS.mapBy(_ => NoAttempt)}
pure def expectedKeysS(id: AttemptId, s: ScenarioS): Set[AuthorityKey] =
  if (id == FundingOneAttempt) Set(keyS(Alice, 0)) else if (id == FundingTwoAttempt) Set(keyS(Bob, 0))
  else if (rejectedS(s.mode) or s.funded == 0) Set()
  else if (s.funded == 1) Set(keyS(Alice, 1)) else Set(keyS(Alice, 1), keyS(Bob, 1))
```

Initial tests:

```quint
run literalFidelityTest = assert(programS == canonicalSwap and beforeS(0).state.minimumTime == Time0
  and unsignedS.authority.context.environment.physicalTime == 1 and SCENARIOS_S.forall(s =>
    Set(FundingOneAttempt, FundingTwoAttempt, DispositionAttempt).forall(id =>
      actualS(id, s) == AuthorityAdaptedA(expectedS(id, s))
      and authorityPlanMatchesA(planS(id, s))
      and requiredKeys(plannedS(id, s).operation, plannedS(id, s).effects) == expectedKeysS(id, s))))
run authorityTimeZeroTest = assert(adaptAuthorityA({before: beforeS(0), input: NoAInput, now: Time0},
  OpDeadlineRefund, planS(DispositionAttempt, {funded: 0, mode: TimeoutS(Time100)}), PublicDisplay) == AuthorityInvalidRequestA
  and validState(programS, beforeS(0).state))
```

The unused projectedS before parameter is intentional type stability for literal rollback comparisons, not an adapter dependency. Signed plan facts are expected execution predecessor facts; funding registration itself does not change their ledger/environment/ParentAbsent facts.

- [ ] Gate1: reviewer checks all literal paths, exact ordered refunds/payments, no-effect empty timeout, unchanged raw Time0, body/plan validity and complete expected authority-key sets. Root captures original RED/GREEN.

## Task 2: actual authority lifecycle and deterministic matrix

**Files:** lifecycle/tests. **Consumes:** fixture interfaces. **Produces:** CommandS, routeS, canCommandS, applyCommandS, prefixS, safetyS, financialTerminalS.

- [ ] Add RED variant in applyCommandS: CommitS initially returns state unchanged. Typecheck and run fundingTest; assert failure at the first ledger/candidate update, capture full source/output, then restore exact common applyCommit. Never record a fabricated historical RED.
- [ ] Add final helpers:

```quint
type CommandS = PrepareS({id: AttemptId, principal: Principal}) | SignS({id: AttemptId, principal: Principal})
  | ProposeS(AttemptId) | VerifyS(AttemptId) | CommitS(AttemptId) | AdvanceS(ATime)
  | RejectProposedS(AttemptId) | RejectVerifiedS(AttemptId)
pure def preparePairS(id: AttemptId, principal: Principal): List[CommandS] =
  List(PrepareS({id: id, principal: principal}), SignS({id: id, principal: principal}))
pure def pipelineS(id: AttemptId): List[CommandS] = List(ProposeS(id), VerifyS(id), CommitS(id))
pure def fundingRouteS(s: ScenarioS): List[CommandS] =
  (if (s.funded >= 1) preparePairS(FundingOneAttempt, Alice).concat(pipelineS(FundingOneAttempt)) else List())
  .concat(if (s.funded == 2) List(AdvanceS(Time2)).concat(preparePairS(FundingTwoAttempt, Bob)).concat(pipelineS(FundingTwoAttempt)) else List())
pure def routeS(s: ScenarioS): List[CommandS] = {
  val clock = if (Set(SettleS, RefundS).contains(s.mode)) List() else List(AdvanceS(timeS(s.mode)))
  val signing = if (rejectedS(s.mode) or s.funded == 0) List()
    else preparePairS(DispositionAttempt, Alice).concat(if (s.funded == 2) preparePairS(DispositionAttempt, Bob) else List())
  val disposition = if (rejectedS(s.mode) or s.funded == 0)
    List(ProposeS(DispositionAttempt), RejectProposedS(DispositionAttempt)) else pipelineS(DispositionAttempt)
  fundingRouteS(s).concat(clock).concat(signing).concat(disposition)
}
pure def evidenceS(attempt: AAuthorityAttempt, s: ScenarioS, p: SigningProfile): AAuthorityEvidence = {
  effect: {attempt: attempt, disposition: EvidenceValid},
  signatures: expectedKeysS(attempt.id, s).mapBy(key => {attempt: attempt, disposition: EvidenceValid,
    signed: {policy: policyS(attempt.id, s, key.principal, p), signer: key.principal, token: 0}})}
pure def canCommandS(state: AAuthorityExecution, s: ScenarioS, p: SigningProfile, cmd: CommandS): bool = match cmd {
  | PrepareS(c) => canPrepareSigningAuthorityA(state.authority, policyS(c.id, s, c.principal, p), c.principal)
  | SignS(c) => canSignAuthorityA(state.authority, policyS(c.id, s, c.principal, p), c.principal, 0)
  | AdvanceS(t) => validAuthorityExecutionA(state) and timeValue(t) > state.authority.context.environment.physicalTime
  | ProposeS(id) => match actualS(id, s) {
      | AuthorityAdaptedA(obs) => validAuthorityExecutionA(state) and canPropose(state, id, plannedS(id, s).operation, obs,
          if (id == FundingOneAttempt) Alice else Bob)
      | _ => false }
  | VerifyS(id) => match state.attempts.get(id) {
      | ProposedAttempt(a) => canVerifyAuthorityA(state, id, evidenceS(a, s, p)) | _ => false }
  | CommitS(id) => canCommitAuthorityA(state, id)
  | RejectProposedS(id) => match state.attempts.get(id) {
      | ProposedAttempt(a) => canRejectProposedAuthorityA(state, id, evidenceS(a, s, p)) | _ => false }
  | RejectVerifiedS(id) => canRejectVerifiedAuthorityA(state, id)
}
pure def applyCommandS(state: AAuthorityExecution, s: ScenarioS, p: SigningProfile, cmd: CommandS): AAuthorityExecution = match cmd {
  | PrepareS(c) => {...state, authority: applyPrepareSigning(state.authority, policyS(c.id, s, c.principal, p), c.principal)}
  | SignS(c) => {...state, authority: applySign(state.authority, policyS(c.id, s, c.principal, p), c.principal, 0)}
  | AdvanceS(t) => {...state, authority: {...state.authority, context: {...state.authority.context, environment: environmentS(timeValue(t))}}}
  | ProposeS(id) => match actualS(id, s) {
      | AuthorityAdaptedA(obs) => applyPropose(state, id, plannedS(id, s).operation, obs, if (id == FundingOneAttempt) Alice else Bob)
      | _ => state }
  | VerifyS(id) => match state.attempts.get(id) {
      | ProposedAttempt(a) => applyVerify(state, id, evidenceS(a, s, p)) | _ => state }
  | CommitS(id) => applyCommit(state, id)
  | RejectProposedS(id) => match state.attempts.get(id) {
      | ProposedAttempt(a) => applyRejectProposedAuthorityA(state, id, evidenceS(a, s, p)) | _ => state }
  | RejectVerifiedS(id) => applyRejectVerifiedAuthorityA(state, id)
}
pure def noPendingS(state: AAuthorityExecution): bool = ATTEMPT_IDS.forall(id => match state.attempts.get(id) {
  | ProposedAttempt(_) => false | VerifiedOperation(_) => false | _ => true })
  and AUTHORITY_KEYS.forall(key => match state.authority.signing.get(key) { | PreparedSigning(_) => false | _ => true })
pure def financialTerminalS(state: AAuthorityExecution): bool = noPendingS(state)
  and state.authority.context.candidate.state.continuation == N0
  and CORE_ACCOUNTS.forall(a => balance(state.authority.context.ledger, Escrow(a), a.asset) == 0)
pure def safetyS(state: AAuthorityExecution): bool = validAuthorityExecutionA(state)
  and totalAsset(state.authority.context.ledger, TokenA) == 10 and totalAsset(state.authority.context.ledger, TokenB) == 20
  and state.authority.context.parents == unsignedS.authority.context.parents
pure def nonCommitS(before: AAuthorityExecution, after: AAuthorityExecution, cmd: CommandS): bool = match cmd {
  | CommitS(id) => after == applyCommit(before, id)
  | AdvanceS(_) => after.authority.context.ledger == before.authority.context.ledger
      and after.authority.context.candidate == before.authority.context.candidate and after.attempts == before.attempts
      and after.authority.context.registry == before.authority.context.registry
  | SignS(_) => after.authority.context.candidate == before.authority.context.candidate
      and after.authority.context.ledger == before.authority.context.ledger and after.attempts == before.attempts
  | _ => after.authority.context == before.authority.context }
type PrefixS = {state: AAuthorityExecution, ok: bool, visited: List[CommandS]}
pure def executeListS(s: ScenarioS, p: SigningProfile, commands: List[CommandS]): PrefixS =
  commands.foldl({state: unsignedS, ok: safetyS(unsignedS), visited: List()}, (acc, cmd) => {
    val commandReady = canCommandS(acc.state, s, p, cmd)
    val after = if (commandReady) applyCommandS(acc.state, s, p, cmd) else acc.state
    {state: after, ok: acc.ok and commandReady and safetyS(after) and nonCommitS(acc.state, after, cmd),
      visited: if (commandReady) acc.visited.append(cmd) else acc.visited}
  })
pure def prefixS(s: ScenarioS, p: SigningProfile, count: int): PrefixS = executeListS(s, p, routeS(s).slice(0, count))
pure def expectedFinalLedgerS(s: ScenarioS): Ledger = if (s.mode == SettleS)
  LEDGER_KEYS.mapBy(_ => 0).put((Wallet(Bob), TokenA), 10).put((Wallet(Alice), TokenB), 20)
  else if (rejectedS(s.mode) or s.funded == 0) ledgerS(s.funded) else ledgerS(0)
```

No-op values in pure total update dispatchers are unreachable under matching guards. prefixS only counts success when ok is true. The harness action below disables on false guard. Unfunded timeouts need no signature bundle because requiredKeys is empty, and that is exactly why the envelope fails; do not register a fictitious disposition policy.

Add tests:

```quint
run fundingTest = assert(PROFILES_S.forall(p => {
  val s = {funded: 2, mode: SettleS}
  val signed = prefixS(s, p, 2)
  val alice = prefixS(s, p, 5)
  val bob = prefixS(s, p, 11)
  signed.ok and alice.ok and bob.ok
    and signed.state.authority.context.candidate == beforeS(0) and signed.state.authority.context.ledger == ledgerS(0)
    and alice.state.authority.context.candidate == beforeS(1) and alice.state.authority.context.ledger == ledgerS(1)
    and bob.state.authority.context.candidate == beforeS(2) and bob.state.authority.context.ledger == ledgerS(2)
    and Set(Alice, Bob).forall(owner => bob.state.authority.context.registry.get(keyS(owner, 0)) ==
      AuthorityConsumed({signed: {policy: policyS(if (owner == Alice) FundingOneAttempt else FundingTwoAttempt, s, owner, p),
        signer: owner, token: 0}, revision: 1}))
}))
run dispositionMatrixTest = assert(PROFILES_S.forall(p => SCENARIOS_S.forall(s => {
  val result = prefixS(s, p, routeS(s).length())
  val registered = prefixS(s, p, routeS(s).length() - (if (rejectedS(s.mode) or s.funded == 0) 2 else 3))
  val planned = dispositionPlannedS(s)
  result.ok and registered.ok and result.state.authority.context.ledger == expectedFinalLedgerS(s)
    and match result.state.attempts.get(DispositionAttempt) {
      | ExecutedOperation(done) => s.funded > 0 and not(rejectedS(s.mode))
          and financialTerminalS(result.state) and done.attempt.observation == expectedS(DispositionAttempt, s)
          and result.state.authority.context.candidate == planned.proposedSuccessor
          and done.evidence.signatures.keys() == expectedKeysS(DispositionAttempt, s)
          and expectedKeysS(DispositionAttempt, s).forall(key => result.state.authority.context.registry.get(key)
            == AuthorityConsumed({signed: done.evidence.signatures.get(key).signed, revision: 1}))
      | RejectedOperation(r) => {
          val attempt: AAuthorityAttempt = {id: DispositionAttempt, actor: Bob, operation: planned.operation,
            observation: expectedS(DispositionAttempt, s), context: registered.state.authority.context}
          val evidence: AAuthorityEvidence = {effect: {attempt: attempt, disposition: EvidenceValid}, signatures: Map()}
          not(financialTerminalS(result.state)) and noPendingS(result.state)
            and result.state.authority.context.candidate == beforeS(s.funded)
            and result.state == {...registered.state, attempts: registered.state.attempts.put(DispositionAttempt, RejectedOperation({
              attempt: attempt, evidence: evidence, observedContext: registered.state.authority.context,
              reason: if (rejectedS(s.mode)) CoreRejected(CoreErrorCode("contract_closed")) else UnauthorizedEffect,
              stage: VerificationBoundary}))}
        }
      | _ => false }
})))
run replayTest = assert(PROFILES_S.forall(p => SCENARIOS_S.forall(s => {
  val result = prefixS(s, p, routeS(s).length())
  result.ok and not(canCommitAuthorityA(result.state, DispositionAttempt))
    and not(canPropose(result.state, DispositionAttempt, dispositionPlannedS(s).operation, expectedS(DispositionAttempt, s), Bob))
})))
```

- [ ] Gate2: independent reviewer checks funding starts from rawTime0, each actual signing snapshot, required two-party disposition evidence, all12 scenarios under both profiles, exact refusal rollback and no false financial terminal. Root preserves receipts before any next edit.

## Task 3: stale bindings, adversarial controls and action harness

**Files:** lifecycle/test/harness. **Produces:** explicit stale routes and retained controls; positive action witnesses.

- [ ] For compiling RED temporarily replace canVerifyAuthorityA by canVerify in VerifyS. Run mutationTest with the unused-successor-node mutant below; capture genuine failure then restore A verification. Commit-time tests separately build structurally verified mutants and require canCommitAuthorityA false; common representation alone is not authority.
- [ ] Add stale route and controls. Put only `staleScenarioS` and `staleRouteS` in `candidate_a_authority_swap.qnt` so the harness can import them. Put `rejectionPreservedS`, `changedAttemptS`, `mutationsS` and every `run` below in `candidate_a_authority_swap_test.qnt`. No mutation below edits common modules; mutations are test inputs only.

```quint
pure val staleScenarioS: ScenarioS = {funded: 2, mode: SettleS}
pure val staleRouteS: List[CommandS] = fundingRouteS(staleScenarioS)
  .concat(preparePairS(DispositionAttempt, Alice)).concat(preparePairS(DispositionAttempt, Bob))
  .concat(List(ProposeS(DispositionAttempt), VerifyS(DispositionAttempt), AdvanceS(Time100), RejectVerifiedS(DispositionAttempt)))
pure def rejectionPreservedS(state: AAuthorityExecution, id: AttemptId, e: AAuthorityEvidence,
  expectedReason: RejectionReason): bool = canRejectProposedAuthorityA(state, id, e) and match state.attempts.get(id) {
    | ProposedAttempt(attempt) => applyRejectProposedAuthorityA(state, id, e) == {...state,
        attempts: state.attempts.put(id, RejectedOperation({attempt: attempt, evidence: e,
          observedContext: state.authority.context, reason: expectedReason, stage: VerificationBoundary}))}
    | _ => false }
pure def changedAttemptS(state: AAuthorityExecution, original: AAuthorityAttempt, changed: AAuthorityObservation): AAuthorityExecution =
  {...state, attempts: state.attempts.put(original.id, ProposedAttempt({...original, observation: changed}))}
run staleSigningTest = assert(PROFILES_S.forall(p => {
  val s = staleScenarioS
  val funded = prefixS(s, p, 11)
  val policy = policyS(DispositionAttempt, s, Alice, p)
  val prepared = applyPrepareSigning(funded.state.authority, policy, Alice)
  val advanced = {...prepared, context: {...prepared.context, environment: environmentS(100)}}
  funded.ok and canPrepareSigningAuthorityA(funded.state.authority, policy, Alice)
    and canSignAuthorityA(prepared, policy, Alice, 0)
    and not(canSignAuthorityA(advanced, policy, Alice, 0))
    and advanced.signing == prepared.signing and advanced.context.candidate == prepared.context.candidate
    and advanced.context.ledger == prepared.context.ledger and advanced.context.registry == prepared.context.registry
}))
run staleVerifiedTest = assert(PROFILES_S.forall(p => {
  val ready = executeListS(staleScenarioS, p, staleRouteS.slice(0, 17))
  val advanced = executeListS(staleScenarioS, p, staleRouteS.slice(0, 18))
  val rejected = executeListS(staleScenarioS, p, staleRouteS)
  ready.ok and advanced.ok and rejected.ok and not(financialTerminalS(rejected.state))
    and canCommitAuthorityA(ready.state, DispositionAttempt)
    and not(canCommitAuthorityA(advanced.state, DispositionAttempt))
    and advanced.state.attempts == ready.state.attempts and rejected.state.authority == advanced.state.authority
    and match ready.state.attempts.get(DispositionAttempt) {
      | VerifiedOperation(v) => rejected.state.attempts.get(DispositionAttempt) == RejectedOperation({
          attempt: v.attempt, evidence: v.evidence, observedContext: advanced.state.authority.context,
          reason: StaleBindings, stage: CommitBoundary})
      | _ => false }
}))
pure def mutationsS(obs: AAuthorityObservation): Set[AAuthorityObservation] = {
  val changedProgram = {...programS, nodes: programS.nodes.put(N15,
    PayA({account: aliceA, payee: Bob, amount: ConstantA(5), continuation: N0}))}
  val wrongResult = match obs.coreProjection {
    | CoreProjected(r) => CoreProjected({...r, reductions: r.reductions + 1})
    | NoCoreProjection => NoCoreProjection }
  Set({...obs, proposedSuccessor: {...obs.proposedSuccessor, program: changedProgram}},
    {...obs, effects: List(obs.effects.nth(1), obs.effects.nth(0))},
    {...obs, coreProjection: wrongResult},
    {...obs, input: ChoiceLike({id: "settle", chooser: Alice, chosen: 1})})
}
run mutationTest = assert(PROFILES_S.forall(p => {
  val s = staleScenarioS
  val proposed = prefixS(s, p, 16)
  proposed.ok and match proposed.state.attempts.get(DispositionAttempt) {
    | ProposedAttempt(a) => mutationsS(a.observation).forall(obs => {
        val changed = {...a, observation: obs}
        val bad = changedAttemptS(proposed.state, a, obs)
        val e = evidenceS(changed, s, p)
        val verified = {...bad, attempts: bad.attempts.put(DispositionAttempt, VerifiedOperation({attempt: changed, evidence: e}))}
        not(canCommandS(bad, s, p, VerifyS(DispositionAttempt)))
          and not(canCommitAuthorityA(verified, DispositionAttempt))
          and authorityRejectionReasonA(bad, changed, e) == UnauthorizedEffect
          and rejectionPreservedS(bad, DispositionAttempt, e, UnauthorizedEffect)
      })
    | _ => false }
}))
run identityBindingTest = assert(PROFILES_S.forall(p => {
  val s = staleScenarioS
  val original = fundingPlannedS(Alice)
  val second = {...original, proposedSuccessor: beforeS(0)}
  val plan = {identity: SwapPlanA, operations: List(original, second)}
  val policy = policyS(FundingOneAttempt, s, Alice, p)
  val withPlan = {...policy, binding: if (p == SignAfterResolve) AfterResolution({identity: plan, operations: plan.operations})
    else BeforeResolution(AnyArtifactUnderMechanism)}
  val obs = {...expectedS(FundingOneAttempt, s), resolvedPlan: plan}
  val signed = {...unsignedS, authority: applySign(applyPrepareSigning(unsignedS.authority, policy, Alice), policy, Alice, 0)}
  val proposed = applyPropose(signed, FundingOneAttempt, OpFund, obs, Alice)
  not(authorityPlanMatchesA(plan))
    and (if (p == SignAfterResolve) not(canPrepareSigningAuthorityA(unsignedS.authority, withPlan, Alice))
      else canPrepareSigningAuthorityA(unsignedS.authority, withPlan, Alice))
    and canPrepareSigningAuthorityA(unsignedS.authority, policy, Alice)
    and canSignAuthorityA(applyPrepareSigning(unsignedS.authority, policy, Alice), policy, Alice, 0)
    and canPropose(signed, FundingOneAttempt, OpFund, obs, Alice)
    and match proposed.attempts.get(FundingOneAttempt) {
      | ProposedAttempt(a) => not(canVerifyAuthorityA(proposed, FundingOneAttempt, evidenceS(a, s, p)))
      | _ => false }
}))
run wrongActorTest = assert(PROFILES_S.forall(p => {
  val s = staleScenarioS
  val proposed = prefixS(s, p, 16)
  val wrongCore = computeTransaction(programS, beforeS(2).state,
    PresentAInput(ChoiceInputA({id: SettleId, chooser: Alice, chosen: 1})), Time2)
  proposed.ok and wrongCore == TransactionComputedA({accepted: false, state: beforeS(2).state,
    error: CoreErrorCode("no_matching_input"), payments: List(), warnings: List(), reductions: 0})
    and match proposed.state.attempts.get(DispositionAttempt) {
      | ProposedAttempt(a) => {
          val e = evidenceS(a, s, p)
          val aliceKey = keyS(Alice, 1)
          val wrongSigner = {...e, signatures: e.signatures.put(aliceKey, {...e.signatures.get(aliceKey),
            signed: {...e.signatures.get(aliceKey).signed, signer: Mallory}})}
          val wrongNonce = {...e, signatures: e.signatures.keys().exclude(Set(aliceKey)).mapBy(k => e.signatures.get(k))
            .put(keyS(Alice, 0), {...e.signatures.get(aliceKey), signed: {
              policy: policyS(FundingOneAttempt, s, Alice, p), signer: Alice, token: 0}})}
          not(canVerifyAuthorityA(proposed.state, DispositionAttempt, wrongSigner))
            and not(canVerifyAuthorityA(proposed.state, DispositionAttempt, wrongNonce))
            and rejectionPreservedS(proposed.state, DispositionAttempt, wrongSigner, UnauthorizedEffect)
            and rejectionPreservedS(proposed.state, DispositionAttempt, wrongNonce, EvidenceMissing)
        } | _ => false }
}))
```

Also test stale policy facts without asserting all such facts fail before-resolution signing, which deliberately need not resolve:

```quint
run staleFactsTest = assert(PROFILES_S.forall(p => {
  val s = staleScenarioS
  val obs = expectedS(DispositionAttempt, s)
  val original = dispositionPlannedS(s)
  val changed = {...original, predecessorFacts: {...original.predecessorFacts, environment: environmentS(100)}}
  val plan = {identity: SwapPlanA, operations: List(changed)}
  val proposed = prefixS(s, p, 16)
  proposed.ok and not(authorityPlanMatchesA(plan)) and match proposed.state.attempts.get(DispositionAttempt) {
    | ProposedAttempt(a) => {
        val bad = {...a, observation: {...obs, resolvedPlan: plan}}
        val badState = {...proposed.state, attempts: proposed.state.attempts.put(DispositionAttempt, ProposedAttempt(bad))}
        not(canVerifyAuthorityA(badState, DispositionAttempt, evidenceS(bad, s, p)))
      } | _ => false }
}))
```

- [ ] Add harness. Scenario and profile choose positive/refusal routes; a separate stale flag adds the explicit verified-stale branch. Witnesses depend on history entries, never initialization facts.

```quint
var state: AAuthorityExecution
var scenario: ScenarioS
var profile: SigningProfile
var stale: bool
var cursor: int
var history: List[CommandS]
var lastAtomicS: bool
val selectedRouteS: List[CommandS] = if (stale) staleRouteS else routeS(scenario)
action init = {
  nondet selectedScenario = SCENARIOS_S.oneOf()
  nondet selectedProfile = PROFILES_S.oneOf()
  nondet selectedStale = Set(false, true).oneOf()
  all { state' = unsignedS, scenario' = if (selectedStale) staleScenarioS else selectedScenario,
    profile' = selectedProfile, stale' = selectedStale, cursor' = 0, history' = List(), lastAtomicS' = true }
}
action performS(cmd: CommandS): bool = all {
  cursor < selectedRouteS.length(), selectedRouteS.nth(cursor) == cmd,
  canCommandS(state, scenario, profile, cmd),
  state' = applyCommandS(state, scenario, profile, cmd), scenario' = scenario, profile' = profile, stale' = stale,
  cursor' = cursor + 1, history' = history.append(cmd),
  lastAtomicS' = nonCommitS(state, applyCommandS(state, scenario, profile, cmd), cmd),
}
action step = if (cursor < selectedRouteS.length()) performS(selectedRouteS.nth(cursor))
  else all { false, state' = state, scenario' = scenario, profile' = profile, stale' = stale, cursor' = cursor, history' = history, lastAtomicS' = lastAtomicS }
def seenS(cmd: CommandS): bool = history.foldl(false, (found, actual) => found or actual == cmd)
val swapSafetyS = safetyS(state) and lastAtomicS and cursor == history.length() and cursor <= selectedRouteS.length()
val completedS = cursor == selectedRouteS.length() and cursor > 0
val alicePreparedS = seenS(PrepareS({id: FundingOneAttempt, principal: Alice}))
val aliceSignedS = seenS(SignS({id: FundingOneAttempt, principal: Alice}))
val aliceProposedS = seenS(ProposeS(FundingOneAttempt))
val aliceVerifiedS = seenS(VerifyS(FundingOneAttempt))
val aliceCommittedS = seenS(CommitS(FundingOneAttempt))
val bobPreparedS = seenS(PrepareS({id: FundingTwoAttempt, principal: Bob}))
val bobSignedS = seenS(SignS({id: FundingTwoAttempt, principal: Bob}))
val bobProposedS = seenS(ProposeS(FundingTwoAttempt))
val bobVerifiedS = seenS(VerifyS(FundingTwoAttempt))
val bobCommittedS = seenS(CommitS(FundingTwoAttempt))
val dispositionAlicePreparedS = seenS(PrepareS({id: DispositionAttempt, principal: Alice}))
val dispositionAliceSignedS = seenS(SignS({id: DispositionAttempt, principal: Alice}))
val dispositionBobPreparedS = seenS(PrepareS({id: DispositionAttempt, principal: Bob}))
val dispositionBobSignedS = seenS(SignS({id: DispositionAttempt, principal: Bob}))
val dispositionProposedS = seenS(ProposeS(DispositionAttempt))
val dispositionVerifiedS = seenS(VerifyS(DispositionAttempt))
val dispositionCommittedS = seenS(CommitS(DispositionAttempt))
val proposedRejectedS = seenS(RejectProposedS(DispositionAttempt))
val verifiedRejectedS = seenS(RejectVerifiedS(DispositionAttempt))
val advanced2S = seenS(AdvanceS(Time2))
val advanced100S = seenS(AdvanceS(Time100))
val advanced101S = seenS(AdvanceS(Time101))
val afterCompletedS = completedS and profile == SignAfterResolve
val beforeCompletedS = completedS and profile == SignBeforeResolve
```

Add deterministic stateful smoke tests; pure forall matrix covers all24 ordinary scenario/profile pairs plus2 stale paths:

```quint
import candidate_a_authority_swap_harness as H from "./candidate_a_authority_swap_harness"
action initCaseS(s: ScenarioS, p: SigningProfile, isStale: bool): bool = all {
  H::state' = unsignedS, H::scenario' = s, H::profile' = p, H::stale' = isStale, H::cursor' = 0, H::history' = List(), H::lastAtomicS' = true }
run settleActionTest = initCaseS(staleScenarioS, SignAfterResolve, false)
  .then(18.reps(_ => H::step)).expect(H::completedS and H::swapSafetyS and financialTerminalS(H::state))
run refundActionTest = initCaseS({funded: 2, mode: RefundS}, SignBeforeResolve, false)
  .then(18.reps(_ => H::step)).expect(H::completedS and H::swapSafetyS and financialTerminalS(H::state))
run staleActionTest = initCaseS(staleScenarioS, SignAfterResolve, true)
  .then(19.reps(_ => H::step)).expect(H::completedS and H::verifiedRejectedS and not(financialTerminalS(H::state)))
run emptyTimeoutActionTest = initCaseS({funded: 0, mode: TimeoutS(Time100)}, SignBeforeResolve, false)
  .then(3.reps(_ => H::step)).expect(H::completedS and H::proposedRejectedS and not(financialTerminalS(H::state)))
run initialWitnessesFalseTest = H::init.expect(not(H::completedS or H::alicePreparedS or H::proposedRejectedS))
run boundsTest = assert(SCENARIOS_S.size() == 12 and SCENARIOS_S.forall(s => routeS(s).length() <= 19)
  and routeS({funded: 2, mode: TimeoutS(Time101)}).length() == 19 and staleRouteS.length() == 19)
```

### Bound and verification gate

Longest ordinary path: Alice prepare/sign/propose/verify/commit5 + advance2 one + Bob pipeline5 + advance deadline one + both nonce1 prepare/sign4 + disposition pipeline3 =19 transitions (20 states). Settle/refund18. Stale route replaces disposition commit by advance+reject, also19. Empty timeout3. Use max-steps22, margin3; do not confuse this with Core's internal22 iterations.

- [ ] Execute and capture source/tool/output receipts:

```bash
quint --version
quint typecheck specs/quint/s02/candidate_a_authority_swap_fixtures.qnt
quint typecheck specs/quint/s02/candidate_a_authority_swap.qnt
quint typecheck specs/quint/s02/candidate_a_authority_swap_harness.qnt
quint typecheck specs/quint/s02/candidate_a_authority_swap_test.qnt
quint test specs/quint/s02/candidate_a_authority_swap_test.qnt --backend=rust --seed=42 --match '.*Test'
quint run specs/quint/s02/candidate_a_authority_swap_harness.qnt --backend=rust --seed=42 --max-samples=100 --max-steps=22 --invariant=swapSafetyS --witnesses alicePreparedS aliceSignedS aliceProposedS aliceVerifiedS aliceCommittedS bobPreparedS bobSignedS bobProposedS bobVerifiedS bobCommittedS dispositionAlicePreparedS dispositionAliceSignedS dispositionBobPreparedS dispositionBobSignedS dispositionProposedS dispositionVerifiedS dispositionCommittedS proposedRejectedS verifiedRejectedS advanced2S advanced100S advanced101S afterCompletedS beforeCompletedS --verbosity=1
quint test specs/quint/s02/candidate_a_authority_boundary_test.qnt --backend=rust --seed=42
quint test specs/quint/s02/candidate_a_authority_adapter_test.qnt --backend=rust --seed=42
/home/charl/Moriarty/.venv/bin/python -m pytest -q
```

Start with100 samples. Only if a named action witness is absent, inspect the corresponding deterministic path, then rerun the same command with --max-samples=1000, retaining both receipts and naming the missing witness. Do not escalate once all witnesses are nonzero.

Expected: all typechecks/tests pass, no sampled violation, all22 action+2 profile witnesses nonzero. Space-separated witness arguments. Zero blocks the gate and requires trace inspection. No sampled run is a proof.

- [ ] Gate3: root's independent final source/spec/runtime review binds the exact four-file closure, all scenario/profile outcomes, original RED/GREEN, ordered effects, full rollback, stale retained records and no-effect refusal. Root maps S01–S08 to EARS/OpenSpec and may commit under existing authority only after acceptance. Council/model checking/export/correspondence remain separate.

## Output schema and limits

Root's receipt schema:
`{requirementId,scenario,profile,command,cwd,sourceCommit,sourceClosure:[{path,sha256}],tool:{path,version,sha256},exitCode,stdoutPath,stderrPath,tracePaths,expectedPredicate,observedResult,classification}`.
Trace rows retain original request, exact raw result/effects, observation, policy/signature evidence, contexts, parent/nonce registry and attempt cells. Classification distinguishes behavioral-red, type-error, green, sampled-no-violation and witness-count; never invent outcomes or hashes.

Compatibility risks: specified-only blocks need the compiling gates; generic variant/Map types and namespace assignments may need explicit frozen aliases. Only current source-compatible authority behavior is claimed: attempt.actor metadata can change without authentication effect if evidence is rebound; wrong Core actor or signed.signer fails. No new attempt IDs are permitted. A rejected DispositionAttempt can end a refusal workload while funds/agreement remain active; financialTerminalS requires actual N0/zero escrow and no pending records. The finite route workload excludes arbitrary interleavings; separate model checking must state its checked state space.
