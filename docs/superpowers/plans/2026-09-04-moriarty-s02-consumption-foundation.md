# S02 Parent Consumption Foundation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Execute exact-parent, two-slot consumption and cancellation bookkeeping in Quint without conferring signature or financial authority.

**Architecture:** A pure entry checker stores the complete generic parent descriptor, not a lossy identifier. A non-candidate shared-state harness explores first fill, second fill, and cancellation. Candidate semantics, signature checks, and separately authorized recovery remain separate obligations.

**Tech Stack:** Quint 0.32.0, installed Rust evaluator, existing pure `effects` types, repository Python verification.

## Global Constraints

- Follow the reviewed S02 model-comparison and observation-authorization designs under `docs/superpowers/specs/`.
- Preserve XML v1.3, every S01 normative byte, Core/swap/backend/E00, and scope `0.0.0-e00.2`.
- No direct TLA+/TLC workflow; no candidate result or S02 gate follows from this bookkeeping harness.
- Pure guards and updates remain separate. Every action assigns every variable. No blanket stutter.
- Typecheck then execute after each model action is added. Instantiate every constant. Discover scenario tests explicitly.
- The signed parent permits slots 1 and 2, each paying five units to Bob.
- `paid + remainingAllowance = 10`; remaining allowance is not spendable authority after cancellation.
- Cancellation only revokes remaining parent payment authority. It does not release money, erase value, or reuse the cancelled fill nonce for recovery.
- A prepared fill and cancellation bind the same consumption revision. The winner invalidates the loser.

### Task 1: Exact parent and residual bookkeeping

**Files:** Create `specs/quint/s02/consumption.qnt`, `consumption_harness.qnt`, and `consumption_test.qnt`. Append the scope note below to `specs/quint/s02/README.md`. Preserve command output and two complete terminal ITF paths in `evidence/s02-model-comparison/consumption/`. Write the tracked task report named by the controller.

**Interfaces:** Consume `Principal`, `Asset`, `Location` from `effects.qnt`. Produce generic `Parent[p]`, `Entry[p]`, `validEntry`, `canConsumeSlot`, `applyConsumeSlot`, `canCancelParent`, `applyCancelParent`, and `validResidual`. Parameter `p` carries complete policy content supplied by the caller; it is never interpreted here. A future caller must use the complete signed envelope, not a phase counter or hash stand-in. The harness's data record is only a local equality test fixture.

Cancellation claims the exact parent even before its first fill. It therefore cannot free the nonce for a different parent. Revisions equal the number of consumed slots plus one if cancelled; there is no wrap. This is one nonce entry, not an entire registry. Registry key uniqueness and authenticated ownership are separate caller obligations. No function below is a signing or execution verifier.

- [ ] **Step 1: Write the tests first and record the missing-module RED.**

Create `consumption_test.qnt`:

```quint
module consumption_test {
  import consumption_harness.* from "./consumption_harness"
  import consumption.* from "./consumption"

  run firstFillTest = init.then(fillFirst).expect(
    state.usedSlots == Set(1) and state.paid == 5 and state.remainingAllowance == 5 and safety)
  run fullFillTest = init.then(fillFirst).then(fillSecond).expect(
    state.usedSlots == Set(1, 2) and state.paid == 10 and state.remainingAllowance == 0 and safety)
  run cancelBeforeFillTest = init.then(cancelParent).expect(
    state.cancelled and state.paid == 0 and state.remainingAllowance == 10 and
    not(canConsumeSlot(PARENT, state, 1, state)))
  run cancelAfterFillTest = init.then(fillFirst).then(cancelParent).expect(
    state.cancelled and state.paid == 5 and state.remainingAllowance == 5 and
    not(canConsumeSlot(PARENT, state, 2, state)))
  run staleCancelTest = init.then(fillFirst).expect(
    not(canCancelParent(PARENT, state, INITIAL)) and canCancelParent(PARENT, state, state))
  run staleFillTest = init.then(cancelParent).expect(
    not(canConsumeSlot(PARENT, state, 1, INITIAL)))
  run duplicateSlotTest = init.then(fillFirst).expect(
    not(canConsumeSlot(PARENT, state, 1, state)))
  run outOfOrderTest = init.expect(not(canConsumeSlot(PARENT, state, 2, state)))
  run substitutedParentTest = {
    val changed = {...PARENT, policy: {...PARENT.policy, mechanismVersion: 2}}
    init.then(fillFirst).expect(not(canConsumeSlot(changed, state, 2, state)))
  }
  run alteredTermsTest = {
    val changed = {...PARENT, recipient: Alice}
    init.then(fillFirst).expect(not(canConsumeSlot(changed, state, 2, state)))
  }
  run enlargedResidualTest = init.then(fillFirst).expect(
    not(validResidual(PARENT, INITIAL, 1, {...state, remainingAllowance: 10})))
  run exactResidualTest = init.then(fillFirst).expect(validResidual(PARENT, INITIAL, 1, state))
  run badRevisionTest = init.then(fillFirst).expect(not(validEntry(PARENT, {...state, revision: 0})))
  run badPaidTest = init.then(fillFirst).expect(not(validEntry(PARENT, {...state, paid: 0})))
  run duplicateCancelTest = init.then(cancelParent).expect(not(canCancelParent(PARENT, state, state)))
  run invalidSlotTest = init.expect(not(canConsumeSlot(PARENT, state, 3, state)))
}
```

Run `/home/charl/.npm-global/bin/quint test specs/quint/s02/consumption_test.qnt --main consumption_test --match 'Test$'`. The missing import must fail. The final command must discover exactly sixteen tests.

- [ ] **Step 2: Add pure types and signatures, typecheck, then implement and exercise the functions.**

Create `consumption.qnt` using this code. Quint import/type syntax adjustments are permitted only when supported by an actual diagnostic and recorded in the report; do not weaken predicates.

```quint
module consumption {
  import effects.* from "./effects"
  type Domain = SwapDomain | InstallmentDomain
  type AuthorityKey = {domain: Domain, principal: Principal, nonce: int}
  type Parent[p] = {
    key: AuthorityKey, policy: p, source: Location, recipient: Principal,
    asset: Asset, budget: int, slots: int -> int,
  }
  type Claim[p] = Unclaimed | Claimed(Parent[p])
  type Entry[p] = {
    claim: Claim[p], usedSlots: Set[int], paid: int,
    remainingAllowance: int, cancelled: bool, revision: int,
  }
  pure def initialEntry(parent: Parent[p]): Entry[p] = {
    claim: Unclaimed, usedSlots: Set(), paid: 0,
    remainingAllowance: parent.budget, cancelled: false, revision: 0,
  }
  pure def validParent(parent: Parent[p]): bool =
    parent.key == {domain: InstallmentDomain, principal: Alice, nonce: 0} and
    parent.source == Escrow({owner: Alice, asset: TokenA}) and
    parent.recipient == Bob and parent.asset == TokenA and parent.budget == 10 and
    parent.slots == Map(1 -> 5, 2 -> 5)
  pure def slotQuantity(parent: Parent[p], slot: int): int =
    if (parent.slots.keys().contains(slot)) parent.slots.get(slot) else 0
  pure def validEntry(parent: Parent[p], entry: Entry[p]): bool = {
    val parentMatches = match entry.claim {
      | Unclaimed => entry == initialEntry(parent)
      | Claimed(actual) => actual == parent and (entry.usedSlots.size() > 0 or entry.cancelled)
    }
    validParent(parent) and parentMatches and
    entry.usedSlots.subseteq(Set(1, 2)) and
    (not(entry.usedSlots.contains(2)) or entry.usedSlots.contains(1)) and
    entry.paid == entry.usedSlots.fold(0, (total, slot) => total + slotQuantity(parent, slot)) and
    entry.remainingAllowance == parent.budget - entry.paid and
    entry.remainingAllowance >= 0 and
    entry.revision == entry.usedSlots.size() + (if (entry.cancelled) 1 else 0) and
    (not(entry.cancelled) or entry.remainingAllowance > 0)
  }
  pure def canConsumeSlot(parent: Parent[p], entry: Entry[p], slot: int, prepared: Entry[p]): bool =
    validEntry(parent, entry) and prepared == entry and not(entry.cancelled) and
    parent.slots.keys().contains(slot) and not(entry.usedSlots.contains(slot)) and
    (slot == 1 or entry.usedSlots.contains(1)) and
    slotQuantity(parent, slot) <= entry.remainingAllowance
  pure def applyConsumeSlot(parent: Parent[p], entry: Entry[p], slot: int): Entry[p] = {
    ...entry, claim: Claimed(parent), usedSlots: entry.usedSlots.union(Set(slot)),
    paid: entry.paid + slotQuantity(parent, slot),
    remainingAllowance: entry.remainingAllowance - slotQuantity(parent, slot),
    revision: entry.revision + 1,
  }
  pure def canCancelParent(parent: Parent[p], entry: Entry[p], prepared: Entry[p]): bool =
    validEntry(parent, entry) and prepared == entry and not(entry.cancelled) and entry.remainingAllowance > 0
  pure def applyCancelParent(parent: Parent[p], entry: Entry[p]): Entry[p] = {
    ...entry, claim: Claimed(parent), cancelled: true, revision: entry.revision + 1,
  }
  pure def validResidual(parent: Parent[p], before: Entry[p], slot: int, proposed: Entry[p]): bool =
    canConsumeSlot(parent, before, slot, before) and proposed == applyConsumeSlot(parent, before, slot)
}
```

Pure helpers produce candidate bookkeeping only. Their guards do not authenticate a policy, authorize cancellation, validate financial effects, or establish candidate-semantic legality. Use hand-built REPL expressions for initial entry, first fill, and cancellation before wiring each action.

- [ ] **Step 3: Build the harness one action at a time and run after each.**

Create `consumption_harness.qnt`. Start with types/initialization and `fillFirst`, then add `fillSecond`, then `cancelParent`. Run typecheck followed by a sampled run after each addition. Intermediate witness/invariant lists include only definitions already present.

```quint
module consumption_harness {
  import effects.* from "./effects"
  import consumption.* from "./consumption"
  type FixturePolicy = {mechanismVersion: int, acceptedDisclosure: bool}
  pure val PARENT: Parent[FixturePolicy] = {
    key: {domain: InstallmentDomain, principal: Alice, nonce: 0},
    policy: {mechanismVersion: 1, acceptedDisclosure: false},
    source: Escrow({owner: Alice, asset: TokenA}), recipient: Bob,
    asset: TokenA, budget: 10, slots: Map(1 -> 5, 2 -> 5),
  }
  pure val INITIAL: Entry[FixturePolicy] = initialEntry(PARENT)
  var state: Entry[FixturePolicy]
  action init = state' = INITIAL
  action fillFirst = all { canConsumeSlot(PARENT, state, 1, state), state' = applyConsumeSlot(PARENT, state, 1) }
  action fillSecond = all { canConsumeSlot(PARENT, state, 2, state), state' = applyConsumeSlot(PARENT, state, 2) }
  action cancelParent = all { canCancelParent(PARENT, state, state), state' = applyCancelParent(PARENT, state) }
  action step = any { fillFirst, fillSecond, cancelParent }
  val firstFilled = state.usedSlots == Set(1) and not(state.cancelled)
  val fullyFilled = state.usedSlots == Set(1, 2) and state.paid == 10
  val cancelledUnused = state.cancelled and state.paid == 0
  val cancelledResidual = state.cancelled and state.paid == 5
  val terminal = state.cancelled or state.remainingAllowance == 0
  val safety = validEntry(PARENT, state) and state.paid + state.remainingAllowance == 10 and
    (not(state.cancelled) or not(canConsumeSlot(PARENT, state, 1, state)) and not(canConsumeSlot(PARENT, state, 2, state))) and
    (terminal or canConsumeSlot(PARENT, state, 1, state) or canConsumeSlot(PARENT, state, 2, state) or canCancelParent(PARENT, state, state))
  val notCancelledUnused = not(cancelledUnused)
  val notCancelledResidual = not(cancelledResidual)
}
```

The tests explicitly import `effects.*` if Quint does not re-export `Alice` through the harness. Keep test discovery at `Test$`; wildcard discovery also selects imported non-test definitions in Quint 0.32.0.

- [ ] **Step 4: Run all tests and preserve sampled coverage plus complete cancellation paths.**

Run each of the three `quint typecheck` commands, then the sixteen-test command from Step 1. Run:

```text
/home/charl/.npm-global/bin/quint run specs/quint/s02/consumption_harness.qnt --main consumption_harness --invariant safety --witnesses firstFilled fullyFilled cancelledUnused cancelledResidual --seed 42 --max-samples 10000 --max-steps 4
/home/charl/.npm-global/bin/quint run specs/quint/s02/consumption_harness.qnt --main consumption_harness --invariant notCancelledUnused --seed 42 --max-samples 100 --max-steps 4 --out-itf evidence/s02-model-comparison/consumption/cancel-unused.itf.json
/home/charl/.npm-global/bin/quint run specs/quint/s02/consumption_harness.qnt --main consumption_harness --invariant notCancelledResidual --seed 42 --max-samples 100 --max-steps 4 --out-itf evidence/s02-model-comparison/consumption/cancel-residual.itf.json
```

The safety run must have no violation and all four witnesses above zero. The last two commands intentionally violate negated witnesses; those nonzero exits demonstrate reachability, not safety failure. Inspect both generated traces, including exact parent and financial-allowance counters. Preserve actual command arguments, exit codes, raw output, source hashes, tool versions, and generated ITF names in the report. Record JSON `--out` arguments if used to preserve structured output. No Apalache result follows from sampled execution.

- [ ] **Step 5: Append the scope note, verify regression boundaries, self-review, and commit.**

Append to `specs/quint/s02/README.md`:

```markdown
## Parent consumption foundation

`consumption.qnt` checks one exact-parent nonce entry for the fixed two-slot
installment workload. Its generic policy payload is compared structurally.
Slot ordering, exact residuals, cancellation, and stale prepared snapshots
are checked without wrapping revisions. The separate harness has no signing
event and transfers no money. Its policy record is an equality-test fixture,
not a complete signed envelope. These guards are bookkeeping prerequisites,
not authorization. Full caller-side signature, registry ownership, conditions,
candidate-semantic checks, and separately authorized refund recovery remain
required. A cancellation witness here does not satisfy S02 recovery coverage.
```

Run `/home/charl/Moriarty/.venv/bin/python -m pytest tests/test_s02_contract.py -q`, `/home/charl/Moriarty/.venv/bin/python scripts/validate_s01_intent_evidence.py`, and `git diff --check`. Verify no protected file changed. Write and force-track only the named task report in `.superpowers/sdd/`. Commit task-owned files using an explicit file list. Report source and evidence hashes; do not claim S02 completion or a candidate result.
