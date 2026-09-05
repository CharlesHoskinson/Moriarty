# Candidate A authority adapter implementation plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development to implement this plan task-by-task. Steps use checkbox syntax for tracking.

**Goal:** bind actual Candidate A computations to complete common observation and request carriers without implementing authority commitment in this unit.

**Architecture:** a pure adapter calls the existing interpreter and projection/extraction functions, carries full typed requests/plans, and rejects substituted observations by recomputation and equality. Cancellation is a distinct non-Core path. Signing, policy, verification and commit integration follow as separate reviewed units.

**Tech Stack:** Quint 0.32.0, Rust test backend, frozen Python regression suite.

## Global Constraints

- Preserve frozen Core/swap and scope bytes.
- Keep the agreement interpreter and shared signing/policy/commit implementation unchanged.
- Signature authenticity remains a finite symbolic external premise.
- Actual Core rejection and evaluator diagnostics remain distinct.
- Cancellation changes authority only: unchanged agreement, empty effects, NoCoreProjection.
- Valid cancellation derives EvidenceValid from its checked lifecycle identity/empty effects, not from a fabricated Core extraction; non-cancellation requires actual extraction.
- A plan ID alone proves no fidelity; carry the complete AResolvedPlan payload.
- This adapter carries plans; it does not validate plan fidelity, sign, verify or commit.
- Native review is not Council acceptance, model checking or S02 completion.

## Task 1: complete typed request and observation adapter

**Files:** create `specs/quint/s02/candidate_a_authority_adapter.qnt`,
`specs/quint/s02/candidate_a_authority_adapter_test.qnt`, and
`specs/quint/s02/candidate_a_authority_adapter_harness.qnt`.

**Interfaces:** consume existing computeTransaction, extractCommittedEffects,
projectInput, projectResult, validOperation and validState. Produce the exact
carrier aliases from the adjacent adopted authority design, plus:

```quint
pure def adaptAuthorityA(request: AAuthorityRequest, op: Operation,
  plan: AResolvedPlan, display: DisplayProjection): AAuthorityAdaptation
pure def authorityObservationMatchesA(op: Operation,
  obs: AAuthorityObservation): bool
```

- [ ] Read `docs/superpowers/specs/2026-09-05-moriarty-s02-candidate-a-authority-design.md` through Adapter contract. Preserve the type sketch exactly. Read existing imported modules, not generic string fixtures. Add declarations first and typecheck them before logic.
- [ ] Add the following first actual assertion, with imports of effects, observations, policies, candidate_a_types/programs and the new adapter. Stub adaptAuthorityA with AuthorityInvalidRequestA only after this test is written. Retain the complete source/import closure and typecheck/test receipt for behavioral RED before replacing the stub.

```quint
pure val emptyPlanForAdapterTest: AResolvedPlan = {
  identity: InstallmentPlanA, operations: List()}
pure val firstRequest: AAuthorityRequest = {
  before: {program: installmentProgram,
    state: {...emptyAState(N4), accounts: CORE_ACCOUNTS.mapBy(_ => 0).put(aliceA, 10),
      minimumTime: Time2}},
  input: PresentAInput(ChoiceInputA({id: FirstFillId, chooser: Bob, chosen: 1})),
  now: Time2}
pure val parentKeyA = {domain: InstallmentDomain, principal: Alice, nonce: 0}
run firstFillActualInputTest = {
  match adaptAuthorityA(firstRequest, OpFillSlot({parent: parentKeyA, slot: 1}),
    emptyPlanForAdapterTest, PublicDisplay) {
    | AuthorityAdaptedA(obs) => all {
        obs.input == ChoiceLike({id: "fill1", chooser: Bob, chosen: 1}),
        obs.proposedSuccessor.state.continuation == N2,
        obs.proposedSuccessor.state.accounts.get(aliceA) == 5,
        obs.effects == List({source: Escrow(aliceA), destination: Wallet(Bob),
          asset: TokenA, quantity: 5}),
        authorityObservationMatchesA(OpFillSlot({parent: parentKeyA, slot: 1}), obs),
      }
    | _ => false
  }
}
```

The empty plan is intentionally unvalidated adapter data, never an admissible
execution plan. This unit must preserve even malformed plan payloads verbatim;
the next boundary unit checks the one-to-four-operation plan domain/fidelity.

- [ ] Implement adaptation with this decision tree. Require validOperation and now != Time0. On cancellation require validState and NoAInput, emit the unchanged/no-effects/NoCoreProjection observation with CancellationCallA. Otherwise evaluate computeTransaction; preserve every noncomputed diagnostic in AuthorityComputationDiagnosticA. On computed raw, call extractCommittedEffects; preserve every failed extraction in AuthorityExtractionDiagnosticA. On extracted effects emit the complete observation with AgreementCallA(request), actual projected input/time/result, actual successor and ordered effects. Error outcome is Rejected(CoreRejected(raw.error)); accepted outcome is the exact operation-label table in the design. Do not derive money from outcome labels.
- [ ] Implement the binding check exactly as follows; the adapter's checks enforce call-kind validity and invalid lifecycle input rejection.

```quint
pure def authorityObservationMatchesA(op: Operation, obs: AAuthorityObservation): bool =
  match obs.artifactAndCall {
    | AgreementCallA(request) => adaptAuthorityA(request, op, obs.resolvedPlan, obs.display)
        == AuthorityAdaptedA(obs)
    | CancellationCallA(call) => adaptAuthorityA({before: call.before, input: NoAInput,
        now: call.now}, op, obs.resolvedPlan, obs.display) == AuthorityAdaptedA(obs)
  }
```

- [ ] Add separate deterministic assertions for actual Alice deposit10/no payments, second fill5/Close, recovery10 and recovery5, NoInput timeout2reductions/refund10, supplied recovery at deadline exact rollback, cancellation unchanged/no Core result, cancellation with supplied input invalid, Time0 invalid, invalid operation invalid, malformed program/state/input diagnostic. Observe behavioral RED for each new behavior before implementing that behavior. Add output mutation controls for neutral input, predecessor, successor, program/unused-node, projection reductions, warnings, error/accepted bit, payment order and effects; each altered observation must fail equality. Matching an honest adapter result is positive evidence of adapter consistency only.
- [ ] Wire a finite one-action harness with `var adapted: bool` and `var observation: AAuthorityAdaptation`; init has adapted=false and AuthorityInvalidRequestA. The guarded adapt action requires not(adapted), assigns the actual first-fill adaptation and adapted=true. `adapterProducedFirstFill` requires adapted and checks the actual stored input/successor/effects; `adapterBindingSafety` requires not(adapted) or a stored matching observation. There is no stutter. This witness demonstrates adaptation, not signing or payment commitment. Keep harness fixture definitions local rather than importing a test module.
- [ ] Run and retain each terminal receipt:

```bash
quint typecheck specs/quint/s02/candidate_a_authority_adapter_test.qnt
quint test specs/quint/s02/candidate_a_authority_adapter_test.qnt --backend=rust --seed=42
quint typecheck specs/quint/s02/candidate_a_authority_adapter_harness.qnt
quint run specs/quint/s02/candidate_a_authority_adapter_harness.qnt --backend=rust --seed=42 --max-samples=100 --max-steps=2 --invariant=adapterBindingSafety --witnesses adapterProducedFirstFill --verbosity=1
/home/charl/Moriarty/.venv/bin/python -m pytest -q
```

Expected: typechecks/test suite exit0; positive adapter witness in every sampled
one-action trace and no invariant violation; unchanged Python regressions pass.
Preserve exact tested source hashes, tool identity and raw output. Report the
actual observed counts, not these expectations. No Apalache run is part of this unit.

- [ ] Self-review, commit only the three owned source/test/harness files, and write the requested report with source closure and RED/GREEN receipts in `.superpowers/sdd/`. Request nonauthor task review before beginning authority boundary implementation. Root will archive verified receipts, not reconstruct historical failures.

## Plan self-review

The unit covers the adopted type/adapter section only. It does not quietly
implement later signing, plan-fidelity, execution or lifecycle stages. Each
observation field is either computed from the actual request/result or explicitly
retained unvalidated metadata. The supplied plans may be empty in these pure
tests because this unit grants no authority. The finite harness stops after one
actual adaptation; no financial or S02 completion witness is claimed.
