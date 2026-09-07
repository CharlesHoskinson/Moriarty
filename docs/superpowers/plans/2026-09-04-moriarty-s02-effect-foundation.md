# S02 Complete-Effect Foundation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Execute a neutral Quint transfer/arithmetic foundation with multiplicity-sensitive policy checks and a deposit/refund witness harness.

**Architecture:** Pure transfer functions are shared infrastructure, not candidate execution semantics. A separate two-action harness exercises arithmetic; it is explicitly not one of the four architectures and has no signing authority. Integrate the approved observation/authorization supplement into the OpenSpec contract before later candidate logic.

**Tech Stack:** Quint 0.32.0, its installed Rust evaluator, Python/pytest for contract checks. Candidate model checking later uses Quint with Apalache 0.56.1.

## Global Constraints

- Follow the reviewed S02 model-comparison and observation-authorization designs under `docs/superpowers/specs/`.
- Preserve XML v1.3, every S01 normative byte, Core/swap/backend/E00, and scope `0.0.0-e00.2`.
- No direct TLA+/TLC workflow; no candidate result or S02 gate follows from this arithmetic harness.
- Pure guards and updates remain separate. Every action assigns every variable. No blanket stutter.
- Typecheck then execute after each model action is added. Instantiate every constant. Discover scenario tests explicitly.
- Preserve multiplicities and ordered financial transfers; wallet and escrow are different locations.
- An update function computes a candidate state only; it must never substitute for its enabling guard or authorize execution.
- Required recovery subscenarios remain additional coverage under existing registry IDs, not replacements for the registry's witness minimums.

### Task 1: Complete-transfer foundation and runnable witnesses

**Files:**

- Create `specs/quint/s02/effects.qnt`, `effects_harness.qnt`, `effects_test.qnt`, and `README.md`.
- Modify `openspec/changes/s02-model-comparison/design.md` and `specs/architecture-comparison/spec.md` to bind the approved supplement and recovery subscenarios.
- Extend `tests/test_s02_contract.py` with the supplement dependency test below.
- Preserve run evidence in `evidence/s02-model-comparison/foundation/` and a tracked task report.

**Interfaces:** Pure `Transfer`, `Ledger`, `canApply`, `applyTransfers`, `policyAllows`, and `totalAsset` are the outputs. Candidate-specific transitions, complete signed-policy verification, Core correspondence, and model checking are not implemented by this task.

- [ ] **Step 1: Add and run the failing contract test.**

```python
def test_s02_binds_observation_supplement_and_recovery_subscenarios():
    import hashlib
    supplement = ROOT / "docs/superpowers/specs/2026-09-04-moriarty-s02-observation-authorization-design.md"
    design = (CHANGE / "design.md").read_text()
    spec = (CHANGE / "specs/architecture-comparison/spec.md").read_text()
    assert supplement.name in design
    assert hashlib.sha256(supplement.read_bytes()).hexdigest() in design
    for identifier in (
        "cancel-wins/recovery-before-any-fill",
        "fill-wins/recovery-after-first-fill",
    ):
        assert identifier in design and identifier in spec
    assert "pre-sign" in spec
```

Run `/home/charl/Moriarty/.venv/bin/python -m pytest tests/test_s02_contract.py -q`.
The new test must fail because the approved supplement is not yet an explicit
contract dependency. Add its actual SHA-256, both required subscenario IDs,
and the pre-sign versus signed execution checks to the existing design and
S02-05/S02-09 scenarios. Preserve exactly ten scenarios and all original
minimums. State that each candidate and both profiles require the two recovery
paths and their final financial/authority states. Run the focused test GREEN.

- [ ] **Step 2: Write the Quint tests before their implementation.**

Create `effects_test.qnt` with the complete test module below. Run
`quint test specs/quint/s02/effects_test.qnt --main effects_test --match '.*'`;
record the expected missing-import RED. Subsequent behavioral checks must
exercise actual functions, not only import success.

```quint
module effects_test {
  import effects_harness.* from "./effects_harness"

  run depositTest = init.then(deposit).expect(
    balance(state.ledger, Wallet(Alice), TokenA) == 0 and
    balance(state.ledger, Escrow({owner: Alice, asset: TokenA}), TokenA) == 10)
  run refundTest = init.then(deposit).then(refund).expect(
    state.ledger == initialLedger(INITIAL_A, INITIAL_B) and safety)
  run missingLedgerTest = init.expect(not(canApply(Map(), List(depositEffect))))
  run nonpositiveTest = init.expect(
    not(validTransfer({...depositEffect, quantity: 0})) and
    not(canApply(state.ledger, List({...depositEffect, quantity: -1}))))
  run wrongAssetTest = init.expect(
    not(validTransfer({...depositEffect, asset: TokenB})))
  run duplicateEffectsTest = init.expect(
    not(policyAllows(List(), List(depositEffect), List(depositEffect, depositEffect))) and
    policyAllows(List(depositEffect, depositEffect), List(depositEffect, depositEffect), List(depositEffect, depositEffect)))
  run missingRequiredTest = init.expect(
    not(policyAllows(List(depositEffect), List(depositEffect), List())))
  run permutationTest = init.expect(
    policyAllows(List(depositEffect, refundEffect), List(depositEffect, refundEffect), List(refundEffect, depositEffect)))
  run temporaryOverdraftTest = {
    val out = {source: Wallet(Bob), destination: Wallet(Alice), asset: TokenA, quantity: 1}
    val back = {source: Wallet(Alice), destination: Wallet(Bob), asset: TokenA, quantity: 1}
    init.expect(
      applyTransfers(state.ledger, List(out, back)) == state.ledger and
      not(canApply(state.ledger, List(out, back))))
  }
  run extraEffectTest = {
    val extra = {source: Wallet(Alice), destination: Wallet(Mallory), asset: TokenA, quantity: 1}
    init.expect(not(policyAllows(List(depositEffect), List(depositEffect), List(depositEffect, extra))))
  }
}
```

- [ ] **Step 3: Implement the pure foundation and first executable action.**

`effects.qnt`:

```quint
module effects {
  type Principal = Alice | Bob | Mallory
  type Asset = TokenA | TokenB
  type Location = Wallet(Principal) | Escrow({owner: Principal, asset: Asset})
  type Transfer = {source: Location, destination: Location, asset: Asset, quantity: int}
  type Ledger = (Location, Asset) -> int

  pure val PRINCIPALS: Set[Principal] = Set(Alice, Bob, Mallory)
  pure val ASSETS: Set[Asset] = Set(TokenA, TokenB)
  pure val LOCATIONS: Set[Location] = PRINCIPALS.map(p => Wallet(p)).union(
    PRINCIPALS.map(p => ASSETS.map(a => Escrow({owner: p, asset: a}))).flatten())
  pure val LEDGER_KEYS: Set[(Location, Asset)] = LOCATIONS.map(l => ASSETS.map(a => (l, a))).flatten()

  pure def locationAccepts(location: Location, asset: Asset): bool = match location {
    | Wallet(_) => true
    | Escrow(account) => account.asset == asset
  }
  pure def balance(ledger: Ledger, location: Location, asset: Asset): int =
    if (ledger.keys().contains((location, asset))) ledger.get((location, asset)) else 0
  pure def initialLedger(aliceA: int, bobB: int): Ledger =
    LEDGER_KEYS.mapBy(_ => 0).put((Wallet(Alice), TokenA), aliceA).put((Wallet(Bob), TokenB), bobB)
  pure def validLedger(ledger: Ledger): bool =
    ledger.keys() == LEDGER_KEYS and
    LOCATIONS.forall(l => ASSETS.forall(a =>
      balance(ledger, l, a) >= 0 and (locationAccepts(l, a) or balance(ledger, l, a) == 0)))
  pure def validTransfer(effect: Transfer): bool =
    effect.quantity > 0 and effect.source != effect.destination and
    locationAccepts(effect.source, effect.asset) and locationAccepts(effect.destination, effect.asset)
  pure def applyOne(ledger: Ledger, effect: Transfer): Ledger = {
    val debited = ledger.put((effect.source, effect.asset), balance(ledger, effect.source, effect.asset) - effect.quantity)
    debited.put((effect.destination, effect.asset), balance(debited, effect.destination, effect.asset) + effect.quantity)
  }
  pure def applyTransfers(ledger: Ledger, effects: List[Transfer]): Ledger =
    effects.foldl(ledger, (current, effect) => applyOne(current, effect))
  pure def canApply(ledger: Ledger, effects: List[Transfer]): bool = {
    val checked = effects.foldl({ledger: ledger, ok: validLedger(ledger)}, (acc, effect) => {
      ledger: applyOne(acc.ledger, effect),
      ok: acc.ok and validTransfer(effect) and balance(acc.ledger, effect.source, effect.asset) >= effect.quantity,
    })
    checked.ok
  }
  pure def occurrences(effects: List[Transfer], effect: Transfer): int =
    effects.select(candidate => candidate == effect).length()
  pure def submultiset(left: List[Transfer], right: List[Transfer]): bool =
    left.foldl(true, (ok, effect) => ok and occurrences(left, effect) <= occurrences(right, effect))
  pure def validEffects(effects: List[Transfer]): bool =
    effects.foldl(true, (ok, effect) => ok and validTransfer(effect))
  pure def policyAllows(required: List[Transfer], allowed: List[Transfer], actual: List[Transfer]): bool =
    validEffects(required) and validEffects(allowed) and validEffects(actual) and
    submultiset(required, allowed) and submultiset(required, actual) and submultiset(actual, allowed)
  pure def totalAsset(ledger: Ledger, asset: Asset): int =
    LOCATIONS.fold(0, (total, location) => total + balance(ledger, location, asset))
}
```

Build `effects_harness.qnt` incrementally: first types/imports, initialization,
the deposit action, and its witness; typecheck then run. Only after that succeeds
add refund, its witness, and the remaining invariants. The final module is:

```quint
module effects_machine {
  import effects.* from "./effects"
  const INITIAL_A: int
  const INITIAL_B: int
  type HarnessState = {ledger: Ledger, phase: int, history: List[Transfer]}
  var state: HarnessState
  pure val depositEffect: Transfer = {source: Wallet(Alice), destination: Escrow({owner: Alice, asset: TokenA}), asset: TokenA, quantity: 10}
  pure val refundEffect: Transfer = {source: Escrow({owner: Alice, asset: TokenA}), destination: Wallet(Alice), asset: TokenA, quantity: 10}
  pure def canDeposit(s: HarnessState): bool = s.phase == 0 and canApply(s.ledger, List(depositEffect))
  pure def canRefund(s: HarnessState): bool = s.phase == 1 and canApply(s.ledger, List(refundEffect))
  action init = state' = {ledger: initialLedger(INITIAL_A, INITIAL_B), phase: 0, history: List()}
  action deposit = all {
    canDeposit(state),
    state' = {ledger: applyTransfers(state.ledger, List(depositEffect)), phase: 1, history: state.history.append(depositEffect)},
  }
  action refund = all {
    canRefund(state),
    state' = {ledger: applyTransfers(state.ledger, List(refundEffect)), phase: 2, history: state.history.append(refundEffect)},
  }
  action step = any {deposit, refund}
  val deposited = state.phase == 1
  val refunded = state.phase == 2
  val safety = validLedger(state.ledger) and
    totalAsset(state.ledger, TokenA) == INITIAL_A and totalAsset(state.ledger, TokenB) == INITIAL_B and
    (state.phase == 2 or canDeposit(state) or canRefund(state))
}
module effects_harness {
  import effects_machine(INITIAL_A = 10, INITIAL_B = 20).*
}
```

During the first-action run use `step = deposit` and the deposit witness; the
financially meaningful end is the deposited harness state. After adding refund,
use the complete `step` and terminal phase 2 above. Do not add a no-op action.
Quint syntax or import fixes discovered by actual execution are permitted;
record them and preserve the specified interfaces and predicates.

- [ ] **Step 4: Verify and preserve actual output.**

Run with `/home/charl/.npm-global/bin/quint`:

```bash
quint typecheck specs/quint/s02/effects.qnt
quint typecheck specs/quint/s02/effects_harness.qnt
quint typecheck specs/quint/s02/effects_test.qnt
quint test specs/quint/s02/effects_test.qnt --main effects_test --match '.*'
quint run specs/quint/s02/effects_harness.qnt --main effects_harness --invariant safety --witnesses deposited refunded --seed 42 --max-samples 10000 --max-steps 4
```

Expect ten discovered tests to pass and both witnesses to be reached. The
two-step terminal trace is intentional. Preserve actual full command output,
versions and source hashes in the task report, and use Quint's `--out-itf` option
on a deterministic run to preserve an actual deposit/refund path under
`evidence/s02-model-comparison/foundation/`. Record its exact generated name and
digest. A sampled run is not Apalache model checking; no S02 candidate gate passes.

Explain in `specs/quint/s02/README.md` the separate wallet/escrow projection,
guard/update API, prefix overdraft test, multiset semantics, explicit constants,
terminal harness, evidence paths, and exclusions (no full authorization,
candidate execution, Core correspondence, signing, proof, or ledger claim).

Run `/home/charl/Moriarty/.venv/bin/python -m pytest -q`, the read-only S01
validator, and `git diff --check`. Confirm frozen files did not change.

- [ ] **Step 5: Commit and request independent review.**

Commit task artifacts and the full report. Review actual Quint predicates and
output, not just Python text-presence tests. After approval, plan the full
authorization lifecycle and candidate-specific execution; do not substitute
this harness for the four required architectures or final model checking.
