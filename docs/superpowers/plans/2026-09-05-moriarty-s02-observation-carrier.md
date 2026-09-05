# S02 Observation Carrier Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement the lossless neutral observation carrier and executable structural checks required by the delegated common-foundation decision.

**Architecture:** One pure observation module imports only the existing effect types. A concrete two-step harness exercises a declared deposit observation, not a candidate execution interpreter. Separate deterministic tests challenge emission rules, optional values, rejection preservation, and ordered payment projection.

**Tech Stack:** Quint 0.32.0; later common integration uses the already requested Quint/Apalache model checks.

## Global Constraints

- Controlling decision: `docs/superpowers/specs/2026-09-05-moriarty-s02-common-design-decision.md`.
- Core remains `0.0.0-e00.2`; do not modify `moriarty/core.py`, `moriarty/swap.py`, `effects.qnt`, or `consumption.qnt`.
- XML SHA-256: `86b80dd1cbd14d1e5759988be9f619355495fc10c4e2fb6b6d9162367670ddcd`.
- Preserve complete ordered effects, Core payments and warnings, exact error strings, optional integer values, accounts, choices, continuation, minimum time, and reductions.
- No Core interpreter, candidate A–D implementation, signature authority, correspondence oracle, or Council verdict is supplied by this unit.
- No new user approval is needed; the user delegated the design decision. Implementation acceptance remains separate.

## Task 1: Neutral carrier and emission checks

**Files:** Create `specs/quint/s02/observations.qnt`, `observations_harness.qnt`, and `observations_test.qnt`.

**Interfaces:** `validFrozenCoreError(error)`, `validFrozenCoreWarning(warning)`,
`validCoreState(allowedChoiceIds, state)`, `rejectionPreserved(before, result, effects)`,
`validCoreObservation(allowedChoiceIds, before, result, effects)`,
`paymentTransfer(payment)`, and `projectionSupportsCoreEvidence(projection)`.
All are pure. Generic continuation parameters use inferred lowercase type variables.

- [ ] Write concrete scenario tests first. Use a total six-account map, a single
  `settle` choice key, neutral continuation strings, and these declared fixtures:

```quint
pure val before: CoreStateObservation[str] = {
  accounts: CORE_ACCOUNTS.mapBy(_ => 0),
  choices: Map("settle" -> NoInt), continuation: "waiting", minimumTime: 0,
}
pure val deposit: Transfer = {
  source: Wallet(Alice), destination: Escrow({owner: Alice, asset: TokenA}),
  asset: TokenA, quantity: 10,
}
pure val acceptedDeposit: CoreResultObservation[str] = {
  accepted: true, error: NoCoreError, warnings: List(), payments: List(),
  state: {...before, accounts: before.accounts.put({owner: Alice, asset: TokenA}, 10), minimumTime: 1},
  reductions: 0,
}
pure val rejected: CoreResultObservation[str] = {
  accepted: false, error: CoreErrorCode("no_matching_input"),
  warnings: List(), payments: List(), state: before, reductions: 0,
}
```

  Assertions must cover: all six exact errors and unknown error rejection;
  nonpositive warning with requested <= 0 and paid 0; partial warning with
  requested > 0 and 0 <= paid < requested; absent warning operands rejected;
  NoInt distinct from IntValue(0); extra/missing choice or account keys rejected;
  negative balances rejected; accepted deposit with zero reductions and no Core
  payment accepted; rejected transaction with changed state/time, warnings,
  payments, effects, or reductions rejected; accepted/error inconsistency rejected;
  payment asset mismatch and nonpositive quantity rejected; ordered escrow payout
  projection mismatch rejected; NoCoreProjection cannot support CoreResultEvidence.

- [ ] Add carrier declarations and false-returning guard stubs only. The carrier
  matches the old sketch's complete Core records plus these corrected sums:

```quint
type CoreProjection[c] = NoCoreProjection | CoreProjected(CoreResultObservation[c])
type RejectionReason = CoreRejected(CoreError) | UnauthorizedEffect | StaleBindings
  | EvidenceMissing | ConsumptionConflict
type NeutralInput = NoInput
  | DepositLike({location: Location, depositor: Principal, asset: Asset, quantity: int})
  | ChoiceLike({id: str, chooser: Principal, chosen: int})
type DisplayProjection = PublicDisplay | AlternateDisplay
```

  `CandidateObservation[s,c,a,p]` retains complete predecessor/successor, artifact,
  resolved plan, transactionTime, effects, outcome and effectEvidence, and adds
  neutral input and display while making coreProjection optional. None of these
  type declarations interprets the generic payloads.

- [ ] Run `quint typecheck specs/quint/s02/observations_test.qnt`, then
  `quint test specs/quint/s02/observations_test.qnt --match 'Test$'`.
  The meaningful RED must be failed positive assertions against false stubs,
  not a missing import, parser failure, or nonexistent test selector.

- [ ] Replace stubs with the exact structural predicates below. Preserve string
  carriers; do not coerce unknown values. A Core payment injects to an escrow-to-
  wallet transfer; wallet-to-escrow deposit occurrences do not inject to payments.

```quint
pure def validFrozenCoreError(error: CoreError): bool = match error {
  | NoCoreError => true
  | CoreErrorCode(code) => Set("time_before_state", "contract_closed", "input_required",
      "no_matching_input", "choice_out_of_bounds", "non_positive_deposit").contains(code)
}
pure def validFrozenCoreWarning(w: CoreWarning): bool = match w.requested {
  | NoInt => false
  | IntValue(requested) => match w.paid {
      | NoInt => false
      | IntValue(paid) => (w.code == "non_positive_payment" and requested <= 0 and paid == 0)
          or (w.code == "partial_payment" and requested > 0 and paid >= 0 and paid < requested)
    }
}
pure def validCoreState(ids: Set[str], s: CoreStateObservation[c]): bool =
  s.accounts.keys() == CORE_ACCOUNTS and s.accounts.keys().forall(k => s.accounts.get(k) >= 0)
    and s.choices.keys() == ids
pure def paymentTransfer(p: CorePayment): Transfer = {
  source: Escrow(p.source), destination: Wallet(p.recipient), asset: p.asset, quantity: p.quantity,
}
pure def validCorePayment(p: CorePayment): bool = p.asset == p.source.asset and p.quantity > 0
pure def isEscrowTransfer(t: Transfer): bool = match t.source {
  | Wallet(_) => false
  | Escrow(_) => true
}
pure def rejectionPreserved(before: CoreStateObservation[c], r: CoreResultObservation[c],
                            transfers: List[Transfer]): bool =
  not(r.accepted) and r.state == before and r.warnings == List() and r.payments == List()
    and transfers == List() and r.reductions == 0
pure def validCoreObservation(ids: Set[str], before: CoreStateObservation[c],
                             r: CoreResultObservation[c], transfers: List[Transfer]): bool =
  validCoreState(ids, before) and validCoreState(ids, r.state) and validFrozenCoreError(r.error)
    and (r.accepted == (r.error == NoCoreError)) and r.reductions >= 0
    and r.warnings.foldl(true, (ok, w) => ok and validFrozenCoreWarning(w))
    and r.payments.foldl(true, (ok, p) => ok and validCorePayment(p))
    and validEffects(transfers)
    and (if (r.accepted)
      r.state.minimumTime >= before.minimumTime
        and r.payments.foldl(List(), (acc, p) => acc.append(paymentTransfer(p)))
          == transfers.select(t => isEscrowTransfer(t))
      else rejectionPreserved(before, r, transfers))
pure def projectionSupportsCoreEvidence(projection: CoreProjection[c]): bool = match projection {
  | NoCoreProjection => false
  | CoreProjected(_) => true
}
```

  The last predicate means an applicable carrier exists, not that correspondence
  evidence exists. Authorization must later require that independent evidence.
  The structural validator deliberately does not decide whether a candidate
  computed the correct continuation, balances, or reduction count.

- [ ] Add a minimal concrete harness that first checks the deposit fixture and
  then checks the rejection fixture, with state `{phase: int, valid: bool}`.
  `init` sets phase 0 and valid true. The first action requires phase 0 and writes
  phase 1 with `validCoreObservation(ids,before,acceptedDeposit,List(deposit))`;
  the second requires phase 1 and writes phase 2 with the conjunction of the
  previous valid flag and `validCoreObservation(ids,before,rejected,List())`.
  `step` is exactly those two guarded actions. Witnesses are `phase == 1` and
  `phase == 2`; safety is the state-dependent valid flag. Phase 2 is terminal.
  This harness tests predicates on declared examples, not candidate transitions.

- [ ] Run typecheck, all deterministic tests, and
  `quint run specs/quint/s02/observations_harness.qnt --main observations_harness --invariant safety --witnesses checkedDeposit checkedRejection --max-samples 1000 --max-steps 3 --seed 42`.
  Require both witnesses, no invariant violation, and explicit terminal stopping.
- [ ] Run the unchanged effect and consumption tests plus `git diff --check`.
  Review the predicates against the frozen Core source, especially zero-reduction
  accepted deposits and full rollback. Commit only this unit and its test evidence.

## Follow-on work (not completion criteria for this carrier unit)

Branch policies, neutral plan views, signing, authorization, cancellation races,
recovery, finite-domain checks of the full common machine, independent review,
Quint/Apalache evidence, and A–D remain required. This unit must not be reported
as their completion or as a Core correspondence result.
