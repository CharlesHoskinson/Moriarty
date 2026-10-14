# Candidate B native foundation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking. Root reviews this plan before authorizing execution; this document author does not implement it.

**Goal:** Build the native B carriers, three literal graph maps, generic validation and structural graph-status inspection with deterministic foundation tests.

**Architecture:** Four Quint files separate carriers, library data, generic foundation functions and tests. Only neutral effects/observations are imported from existing candidates' shared substrate. Status inspection validates its predecessor before classification; evaluation, Core mapping and authority integration remain separate units.

**Tech Stack:** Existing Quint 0.32.0 CLI, Rust test backend, Bash `script`/`tar` for retained command receipts and exact source closures. No new dependency.

## Global Constraints

- Classification: proposed implementation plan, specified-only; none of the code or expected results below has been executed by this plan author.
- Controlling design: `docs/superpowers/specs/2026-09-05-moriarty-s02-candidate-b-native-graph-design.md`, SHA-256 `2cc795c4ad3a83c3b7b5ecf6ff39ef0af486a0ae1a864fa2fe3e0b24fe2f70ac`, adopted in `fe011a1` with disposition `evidence/s02-model-comparison/candidate-b-native-design-review/review-and-adoption.md`.
- Read `AGENTS.md`, `WIKI_SCHEMA.md` and `raw/assignments/modernizing-marlowe-assignment-2026-09-02.xml` before implementation. Preserve frozen Core/swap, scope and common authority artifacts.
- "Native modules may import neutral `effects` and `observations` data types, but not candidate_a_core, candidate_a_projection or a reference interpreter."
- "All graph maps have exactly thirteen keys." "The deadline is BTime100 for this finite model."
- "Native balances range 0 through 280." Deposit quantities are `{-1,0,1,5,10,20,21}`; choices and choice bounds are `{-1,0,1,2}`; each declared payment quantity is `{5,10,20}` and a rule has at most two ordered payments.
- "The swap semantic clock is not the nonzero clock of the authority fixtures." Initial swap time0, negative diagnostic time1, installment time2.
- "Generic graph admission deliberately does not guarantee non-locking." "BOpen is structural availability, not a claim that a permitted input or solvent payment exists."
- Root disposition for this plan: independently in-domain reversed choice bounds remain admitted; do not add `lower <= upper`. This permits an unsatisfiable choice obligation, not a successful choice execution. Library intervals remain unchanged.
- A graph-status diagnostic is distinct from a valid graph's status. Never classify invalid predecessors as financially complete or silently replace missing maps with complete maps.
- No evaluator, mapping, effect execution, authority adapter, model checking, Council, architecture selection or Foreman development belongs to this unit.
- The test fixtures below do not establish that a discharge history is reachable. The next evaluator unit must execute the required nonlibrary discharge witnesses and preserve diagnostics as failures of its safety predicate.

---

## Files and review boundaries

| New file | Responsibility |
| --- | --- |
| `specs/quint/s02/candidate_b_types.qnt` | Adopted carriers, explicit finite domains/order, zero-state constructor |
| `specs/quint/s02/candidate_b_graphs.qnt` | Exactly three library graphs and their literal initial predecessors |
| `specs/quint/s02/candidate_b_foundation.qnt` | Generic graph/state/input predicates, safe node-set helpers and checked structural status |
| `specs/quint/s02/candidate_b_foundation_test.qnt` | Initialization, literal-row, malformed-domain and structural-status tests |

Only these four `.qnt` files enter implementation commits. The plan file is owned by the plan author; evidence goes under a new `.superpowers/sdd/b-foundation-*` receipt directory, with root responsible for permanent evidence adoption. Preserve any existing A worktree edits.

Task 1 supplies types/data and their own tests. Task 2 adds generic validation and malformed-map tests. Task 3 adds status classification. Each task retains its own behavioral RED and GREEN before a source-only commit. Review the complete unit before starting native evaluation.

## Receipt procedure used by each task

Run these commands from `/home/charl/Moriarty/.worktrees/s01-audit-start`. At execution time, use the existing isolated worktree after checking its ownership and current changes. Read the using-git-worktrees skill if a new worktree is required. This plan does not require a second checkout.

- [ ] Verify the controlling bytes and installed tool identity:

```bash
git status --short
sha256sum docs/superpowers/specs/2026-09-05-moriarty-s02-candidate-b-native-graph-design.md
quint --version
```

Expected design hash is the one above; the established CLI version is 0.32.0. A mismatch is a review input, not permission to alter the design or install another tool silently.

- [ ] Define this shell function in the execution session. It copies only the current unit's source closure before running checks. It never manufactures a RED by changing a correct assertion. No helper file is added to the repository.

```bash
capture_b_foundation() {
  foundation_stage="$1"
  mkdir -p .superpowers/sdd
  foundation_receipt=$(mktemp -d ".superpowers/sdd/b-foundation-${foundation_stage}.XXXXXX")
  for foundation_source in \
    specs/quint/s02/effects.qnt \
    specs/quint/s02/observations.qnt \
    specs/quint/s02/candidate_b_types.qnt \
    specs/quint/s02/candidate_b_graphs.qnt \
    specs/quint/s02/candidate_b_foundation.qnt \
    specs/quint/s02/candidate_b_foundation_test.qnt
  do
    if test -f "$foundation_source"; then
      cp --parents "$foundation_source" "$foundation_receipt"
    fi
  done
  tar -cf "$foundation_receipt/source-closure.tar" -C "$foundation_receipt" specs
  script -q -e -c "cd '$foundation_receipt' && command -v quint && quint --version && node --version && rg --files specs | sort | xargs sha256sum" "$foundation_receipt/source-manifest.log"
  script -q -e -c "quint typecheck '$foundation_receipt/specs/quint/s02/candidate_b_foundation_test.qnt'" "$foundation_receipt/typecheck.log"
  script -q -e -c "quint test '$foundation_receipt/specs/quint/s02/candidate_b_foundation_test.qnt' --backend=rust --seed=42" "$foundation_receipt/test.log"
}
```

`script -e` preserves the command's exit status in the receipt and returns it. Inspect both logs: a valid behavioral RED requires typecheck exit0 and test exit nonzero from an intended Boolean expectation, not an undefined symbol, parser error, missing import or evaluator crash. The final command makes the function return the test exit code. Do not wrap expected RED in `|| true` and describe it as passing. Retain failed tool invocations separately if they occur, repair tooling and record the actual behavioral RED before GREEN. Every invocation uses a new directory, so later source cannot overwrite the earlier closure.

## Task 1: Native carriers and literal initial graphs

**Files:** Create `candidate_b_types.qnt`, `candidate_b_graphs.qnt` and `candidate_b_foundation_test.qnt` under `specs/quint/s02/`.

**Interfaces:** Consumes `Principal`, `Transfer`, `CoreAccount`, `OptionalInt` and `CORE_ACCOUNTS` from neutral modules. Produces all adopted native carriers, `B_NODE_ORDER`, `B_NODE_IDS`, `B_CHOICE_IDS`, finite quantity/time sets, `timeValueB(BTime): int`, `emptyBState(BTime): BState`, `swapGraphB`, `installmentGraphB`, `negativeDepositGraphB`, `swapInitialB`, `installmentInitialB`, `negativeInitialB`. The initial values have type `BPredecessor`, not ledger or authority state.

- [ ] Add the carrier file exactly as follows and typecheck it before any functional implementation. These are the request/result interfaces the next native evaluator must consume and produce; declaring results now does not implement evaluation.

```quint
module candidate_b_types {
  import effects.* from "./effects"
  import observations.* from "./observations"

  type BTime = BTime0 | BTime1 | BTime2 | BTime100 | BTime101
  type BChoiceId = BSettle | BFill1 | BFill2 | BRecover | BOther
  type BNodeId = BDepositAlice | BDepositBob | BSettleSwap | BRefundSwap
    | BTimeoutEmpty | BTimeoutAlice | BTimeoutFunded
    | BFirstFill | BSecondFill | BRecoverTen | BRecoverFive
    | BTimeoutTen | BTimeoutFive
  type BInput = BNoInput
    | BDepositInput({account: CoreAccount, depositor: Principal, quantity: int})
    | BChoiceInput({id: BChoiceId, chooser: Principal, chosen: int})
  type BTrigger = BDepositTrigger({account: CoreAccount, depositor: Principal, quantity: int})
    | BChoiceTrigger({id: BChoiceId, chooser: Principal, lower: int, upper: int})
    | BTimeoutTrigger
  type BPayment = {source: CoreAccount, recipient: Principal, quantity: int}
  type BTransferRule = BDepositTransfer | BPayTransfers(List[BPayment]) | BRefundAll
  type BNode = BInactive | BObligation({requires: Set[BNodeId], excludes: Set[BNodeId],
    trigger: BTrigger, transferRule: BTransferRule})
  type BGraph = {deadline: BTime, nodes: BNodeId -> BNode}
  type BState = {accounts: CoreAccount -> int, choices: BChoiceId -> OptionalInt,
    minimumTime: BTime, discharged: Set[BNodeId], excluded: Set[BNodeId]}
  type BPredecessor = {graph: BGraph, state: BState}
  type BRequest = {before: BPredecessor, input: BInput, now: BTime}
  type BRejection = BTimeBeforeState | BClosed | BInputRequired | BNoMatchingInput
    | BChoiceOutOfBounds | BNonPositiveDeposit | BInsufficientBalance | BExclusionConflict
  type BDiagnostic = BInvalidGraph | BInvalidState | BInputOutsideDomain | BResultOutsideDomain
  type BSuccess = {request: BRequest, node: BNodeId, after: BState,
    effects: List[Transfer], payments: List[BPayment]}
  type BFailure = {request: BRequest, reason: BRejection}
  type BExecution = BAccepted(BSuccess) | BRejected(BFailure)
    | BDiagnosticResult({request: BRequest, reason: BDiagnostic})
  type BGraphStatus = BOpen | BFinanciallyComplete | BExhausted | BStranded
  // A foundation inspection result, not an execution or Core mapping result.
  type BStatusInspection = BStatus(BGraphStatus)
    | BStatusDiagnostic({before: BPredecessor, reason: BDiagnostic})

  pure val B_NODE_ORDER: List[BNodeId] = List(BDepositAlice, BDepositBob,
    BSettleSwap, BRefundSwap, BTimeoutEmpty, BTimeoutAlice, BTimeoutFunded,
    BFirstFill, BSecondFill, BRecoverTen, BRecoverFive, BTimeoutTen, BTimeoutFive)
  pure val B_NODE_IDS: Set[BNodeId] = B_NODE_ORDER.foldl(Set(), (acc, id) => acc.union(Set(id)))
  pure val B_CHOICE_IDS: Set[BChoiceId] = Set(BSettle, BFill1, BFill2, BRecover, BOther)
  pure val B_TIMES: Set[BTime] = Set(BTime0, BTime1, BTime2, BTime100, BTime101)
  pure val B_DEPOSIT_QUANTITIES: Set[int] = Set(-1, 0, 1, 5, 10, 20, 21)
  pure val B_CHOICE_VALUES: Set[int] = Set(-1, 0, 1, 2)
  pure val B_PAYMENT_QUANTITIES: Set[int] = Set(5, 10, 20)
  pure val B_ACCOUNT_ORDER: List[CoreAccount] = List(
    {owner: Alice, asset: TokenA}, {owner: Alice, asset: TokenB},
    {owner: Bob, asset: TokenA}, {owner: Bob, asset: TokenB},
    {owner: Mallory, asset: TokenA}, {owner: Mallory, asset: TokenB})
  pure def timeValueB(time: BTime): int = match time {
    | BTime0 => 0 | BTime1 => 1 | BTime2 => 2 | BTime100 => 100 | BTime101 => 101
  }
  pure def emptyBState(time: BTime): BState = {
    accounts: CORE_ACCOUNTS.mapBy(_ => 0), choices: B_CHOICE_IDS.mapBy(_ => NoInt),
    minimumTime: time, discharged: Set(), excluded: Set()}
}
```

```bash
quint typecheck specs/quint/s02/candidate_b_types.qnt
```

Expected exit0. No runtime or reachability claim follows from this check.

- [ ] Write these literal and initialization tests before supplying the real graph rows. All test names end in `Test`; fixtures and helper predicates do not. `rowEqualsB` uses complete node equality, so a missing exclusion, wrong payment order or ignored trigger field fails.

```quint
module candidate_b_foundation_test {
  import effects.* from "./effects"
  import observations.* from "./observations"
  import candidate_b_types.* from "./candidate_b_types"
  import candidate_b_graphs.* from "./candidate_b_graphs"

  pure def rowEqualsB(g: BGraph, id: BNodeId, req: Set[BNodeId], exc: Set[BNodeId],
      trigger: BTrigger, rule: BTransferRule): bool =
    if (not(g.nodes.keys().contains(id))) false else g.nodes.get(id) == BObligation({
      requires: req, excludes: exc, trigger: trigger, transferRule: rule})
  pure val swapIds = Set(BDepositAlice, BDepositBob, BSettleSwap, BRefundSwap,
    BTimeoutEmpty, BTimeoutAlice, BTimeoutFunded)
  pure val installmentIds = Set(BFirstFill, BSecondFill, BRecoverTen, BRecoverFive,
    BTimeoutTen, BTimeoutFive)
  run completeLibraryMapsTest = all {
    List(swapGraphB, installmentGraphB, negativeDepositGraphB).foldl(true, (ok, g) =>
      ok and g.deadline == BTime100 and g.nodes.keys() == B_NODE_IDS),
    B_NODE_IDS.exclude(swapIds).forall(id => swapGraphB.nodes.get(id) == BInactive),
    B_NODE_IDS.exclude(installmentIds).forall(id => installmentGraphB.nodes.get(id) == BInactive),
    B_NODE_IDS.exclude(Set(BDepositAlice, BTimeoutEmpty)).forall(id =>
      negativeDepositGraphB.nodes.get(id) == BInactive),
    B_NODE_ORDER.length() == 13, B_NODE_IDS.size() == 13,
    B_ACCOUNT_ORDER.length() == 6,
    B_ACCOUNT_ORDER.foldl(Set(), (acc, a) => acc.union(Set(a))) == CORE_ACCOUNTS,
    B_ACCOUNT_ORDER == List({owner: Alice, asset: TokenA}, {owner: Alice, asset: TokenB},
      {owner: Bob, asset: TokenA}, {owner: Bob, asset: TokenB},
      {owner: Mallory, asset: TokenA}, {owner: Mallory, asset: TokenB}),
    B_TIMES.map(t => timeValueB(t)) == Set(0, 1, 2, 100, 101),
  }
  run swapLiteralRowsTest = all {
    rowEqualsB(swapGraphB, BDepositAlice, Set(), Set(BTimeoutEmpty),
      BDepositTrigger({account: bAliceA, depositor: Alice, quantity: 10}), BDepositTransfer),
    rowEqualsB(swapGraphB, BDepositBob, Set(BDepositAlice), Set(BTimeoutAlice),
      BDepositTrigger({account: bBobB, depositor: Bob, quantity: 20}), BDepositTransfer),
    rowEqualsB(swapGraphB, BSettleSwap, Set(BDepositAlice, BDepositBob),
      Set(BRefundSwap, BTimeoutFunded), BChoiceTrigger({id: BSettle, chooser: Bob, lower: 1, upper: 1}),
      BPayTransfers(List({source: bAliceA, recipient: Bob, quantity: 10},
        {source: bBobB, recipient: Alice, quantity: 20}))),
    rowEqualsB(swapGraphB, BRefundSwap, Set(BDepositAlice, BDepositBob),
      Set(BSettleSwap, BTimeoutFunded), BChoiceTrigger({id: BSettle, chooser: Bob, lower: 0, upper: 0}), BRefundAll),
    rowEqualsB(swapGraphB, BTimeoutEmpty, Set(),
      Set(BDepositAlice, BDepositBob, BSettleSwap, BRefundSwap, BTimeoutAlice, BTimeoutFunded),
      BTimeoutTrigger, BRefundAll),
    rowEqualsB(swapGraphB, BTimeoutAlice, Set(BDepositAlice),
      Set(BDepositBob, BSettleSwap, BRefundSwap, BTimeoutFunded), BTimeoutTrigger, BRefundAll),
    rowEqualsB(swapGraphB, BTimeoutFunded, Set(BDepositAlice, BDepositBob),
      Set(BSettleSwap, BRefundSwap), BTimeoutTrigger, BRefundAll),
  }
  run installmentLiteralRowsTest = all {
    rowEqualsB(installmentGraphB, BFirstFill, Set(), Set(BRecoverTen, BTimeoutTen),
      BChoiceTrigger({id: BFill1, chooser: Bob, lower: 1, upper: 1}),
      BPayTransfers(List({source: bAliceA, recipient: Bob, quantity: 5}))),
    rowEqualsB(installmentGraphB, BSecondFill, Set(BFirstFill), Set(BRecoverFive, BTimeoutFive),
      BChoiceTrigger({id: BFill2, chooser: Bob, lower: 1, upper: 1}),
      BPayTransfers(List({source: bAliceA, recipient: Bob, quantity: 5}))),
    rowEqualsB(installmentGraphB, BRecoverTen, Set(),
      Set(BFirstFill, BSecondFill, BRecoverFive, BTimeoutTen, BTimeoutFive),
      BChoiceTrigger({id: BRecover, chooser: Alice, lower: 1, upper: 1}), BRefundAll),
    rowEqualsB(installmentGraphB, BRecoverFive, Set(BFirstFill), Set(BSecondFill, BTimeoutFive),
      BChoiceTrigger({id: BRecover, chooser: Alice, lower: 1, upper: 1}), BRefundAll),
    rowEqualsB(installmentGraphB, BTimeoutTen, Set(),
      Set(BFirstFill, BSecondFill, BRecoverTen, BRecoverFive, BTimeoutFive), BTimeoutTrigger, BRefundAll),
    rowEqualsB(installmentGraphB, BTimeoutFive, Set(BFirstFill),
      Set(BSecondFill, BRecoverFive), BTimeoutTrigger, BRefundAll),
  }
  run negativeLiteralRowsTest = all {
    rowEqualsB(negativeDepositGraphB, BDepositAlice, Set(), Set(BTimeoutEmpty),
      BDepositTrigger({account: bAliceA, depositor: Alice, quantity: -1}), BDepositTransfer),
    rowEqualsB(negativeDepositGraphB, BTimeoutEmpty, Set(), Set(BDepositAlice),
      BTimeoutTrigger, BRefundAll),
  }
  run initialStatesTest = all {
    swapInitialB == {graph: swapGraphB, state: emptyBState(BTime0)},
    negativeInitialB == {graph: negativeDepositGraphB, state: emptyBState(BTime1)},
    installmentInitialB.graph == installmentGraphB,
    installmentInitialB.state == {...emptyBState(BTime2),
      accounts: CORE_ACCOUNTS.mapBy(_ => 0).put(bAliceA, 10)},
  }
}
```

- [ ] Add this compiling, intentionally incomplete literal-data scaffold. It is the behavioral RED source, not a passing implementation. Retain it with the unchanged tests before replacing it.

```quint
module candidate_b_graphs {
  import effects.* from "./effects"
  import observations.* from "./observations"
  import candidate_b_types.* from "./candidate_b_types"
  pure val bAliceA: CoreAccount = {owner: Alice, asset: TokenA}
  pure val bBobB: CoreAccount = {owner: Bob, asset: TokenB}
  pure val inactiveGraphB: BGraph = {deadline: BTime100, nodes: B_NODE_IDS.mapBy(_ => BInactive)}
  pure val swapGraphB: BGraph = inactiveGraphB
  pure val installmentGraphB: BGraph = inactiveGraphB
  pure val negativeDepositGraphB: BGraph = inactiveGraphB
  pure val swapInitialB: BPredecessor = {graph: swapGraphB, state: emptyBState(BTime0)}
  pure val installmentInitialB: BPredecessor = {graph: installmentGraphB, state: emptyBState(BTime2)}
  pure val negativeInitialB: BPredecessor = {graph: negativeDepositGraphB, state: emptyBState(BTime1)}
}
```

```bash
capture_b_foundation red-literals
```

Expected: typecheck succeeds; all three row tests and installment initialization assertion fail. An empty map is not the scaffold: the map must already contain all thirteen inactive keys.

- [ ] Replace the scaffold graph file with this exact data implementation. It supplies no evaluator, lookup fallback or reference call.

```quint
module candidate_b_graphs {
  import effects.* from "./effects"
  import observations.* from "./observations"
  import candidate_b_types.* from "./candidate_b_types"
  pure val bAliceA: CoreAccount = {owner: Alice, asset: TokenA}
  pure val bBobB: CoreAccount = {owner: Bob, asset: TokenB}
  pure val inactiveGraphB: BGraph = {deadline: BTime100, nodes: B_NODE_IDS.mapBy(_ => BInactive)}
  pure def obligationB(req: Set[BNodeId], exc: Set[BNodeId], trigger: BTrigger,
      rule: BTransferRule): BNode = BObligation({requires: req, excludes: exc,
        trigger: trigger, transferRule: rule})
  pure val swapGraphB: BGraph = {deadline: BTime100, nodes: inactiveGraphB.nodes
    .put(BDepositAlice, obligationB(Set(), Set(BTimeoutEmpty),
      BDepositTrigger({account: bAliceA, depositor: Alice, quantity: 10}), BDepositTransfer))
    .put(BDepositBob, obligationB(Set(BDepositAlice), Set(BTimeoutAlice),
      BDepositTrigger({account: bBobB, depositor: Bob, quantity: 20}), BDepositTransfer))
    .put(BSettleSwap, obligationB(Set(BDepositAlice, BDepositBob), Set(BRefundSwap, BTimeoutFunded),
      BChoiceTrigger({id: BSettle, chooser: Bob, lower: 1, upper: 1}),
      BPayTransfers(List({source: bAliceA, recipient: Bob, quantity: 10},
        {source: bBobB, recipient: Alice, quantity: 20}))))
    .put(BRefundSwap, obligationB(Set(BDepositAlice, BDepositBob), Set(BSettleSwap, BTimeoutFunded),
      BChoiceTrigger({id: BSettle, chooser: Bob, lower: 0, upper: 0}), BRefundAll))
    .put(BTimeoutEmpty, obligationB(Set(),
      Set(BDepositAlice, BDepositBob, BSettleSwap, BRefundSwap, BTimeoutAlice, BTimeoutFunded),
      BTimeoutTrigger, BRefundAll))
    .put(BTimeoutAlice, obligationB(Set(BDepositAlice),
      Set(BDepositBob, BSettleSwap, BRefundSwap, BTimeoutFunded), BTimeoutTrigger, BRefundAll))
    .put(BTimeoutFunded, obligationB(Set(BDepositAlice, BDepositBob),
      Set(BSettleSwap, BRefundSwap), BTimeoutTrigger, BRefundAll))}
  pure val installmentGraphB: BGraph = {deadline: BTime100, nodes: inactiveGraphB.nodes
    .put(BFirstFill, obligationB(Set(), Set(BRecoverTen, BTimeoutTen),
      BChoiceTrigger({id: BFill1, chooser: Bob, lower: 1, upper: 1}),
      BPayTransfers(List({source: bAliceA, recipient: Bob, quantity: 5}))))
    .put(BSecondFill, obligationB(Set(BFirstFill), Set(BRecoverFive, BTimeoutFive),
      BChoiceTrigger({id: BFill2, chooser: Bob, lower: 1, upper: 1}),
      BPayTransfers(List({source: bAliceA, recipient: Bob, quantity: 5}))))
    .put(BRecoverTen, obligationB(Set(),
      Set(BFirstFill, BSecondFill, BRecoverFive, BTimeoutTen, BTimeoutFive),
      BChoiceTrigger({id: BRecover, chooser: Alice, lower: 1, upper: 1}), BRefundAll))
    .put(BRecoverFive, obligationB(Set(BFirstFill), Set(BSecondFill, BTimeoutFive),
      BChoiceTrigger({id: BRecover, chooser: Alice, lower: 1, upper: 1}), BRefundAll))
    .put(BTimeoutTen, obligationB(Set(),
      Set(BFirstFill, BSecondFill, BRecoverTen, BRecoverFive, BTimeoutFive), BTimeoutTrigger, BRefundAll))
    .put(BTimeoutFive, obligationB(Set(BFirstFill), Set(BSecondFill, BRecoverFive), BTimeoutTrigger, BRefundAll))}
  pure val negativeDepositGraphB: BGraph = {deadline: BTime100, nodes: inactiveGraphB.nodes
    .put(BDepositAlice, obligationB(Set(), Set(BTimeoutEmpty),
      BDepositTrigger({account: bAliceA, depositor: Alice, quantity: -1}), BDepositTransfer))
    .put(BTimeoutEmpty, obligationB(Set(), Set(BDepositAlice), BTimeoutTrigger, BRefundAll))}
  pure val swapInitialB: BPredecessor = {graph: swapGraphB, state: emptyBState(BTime0)}
  pure val installmentInitialB: BPredecessor = {graph: installmentGraphB,
    state: {...emptyBState(BTime2), accounts: CORE_ACCOUNTS.mapBy(_ => 0).put(bAliceA, 10)}}
  pure val negativeInitialB: BPredecessor = {graph: negativeDepositGraphB, state: emptyBState(BTime1)}
}
```

- [ ] Capture GREEN; require all five tests to pass, then commit these three source files only.

```bash
capture_b_foundation green-literals
git add specs/quint/s02/candidate_b_types.qnt specs/quint/s02/candidate_b_graphs.qnt specs/quint/s02/candidate_b_foundation_test.qnt
git diff --cached --stat
git commit --only -m "feat: add native B carriers and literal graph library" -- specs/quint/s02/candidate_b_types.qnt specs/quint/s02/candidate_b_graphs.qnt specs/quint/s02/candidate_b_foundation_test.qnt
```

Inspect the staged diff before committing; other workers' staged files must not enter this commit.

## Task 2: Generic graph, state and input validation

**Files:** Create `specs/quint/s02/candidate_b_foundation.qnt`; extend `specs/quint/s02/candidate_b_foundation_test.qnt`.

**Interfaces:** Consumes Task 1's carriers and neutral sets. Produces `validBGraph(graph: BGraph): bool`, `validBState(before: BPredecessor): bool`, `validBInput(input: BInput): bool`, `activeNodesB(graph: BGraph): Set[BNodeId]`, `prerequisitesB(node: BNode): Set[BNodeId]`, `exclusionsB(node: BNode): Set[BNodeId]`, `nodeRankB(id: BNodeId): int`, and `declaredExclusionsB(graph: BGraph, discharged: Set[BNodeId]): Set[BNodeId]`. None of these invokes A or applies input.

- [ ] Add `import candidate_b_foundation.* from "./candidate_b_foundation"` to the test module. Append these fixtures and tests inside its existing module. They cover valid nonlibrary data as well as each rejection dimension. Missing-key mutations use a filtered map constructor, not undefined lookups.

```quint
  pure val depositOnlyB: BGraph = {deadline: BTime100, nodes: inactiveGraphB.nodes.put(BDepositAlice,
    obligationB(Set(), Set(), BDepositTrigger({account: bAliceA, depositor: Alice, quantity: 10}), BDepositTransfer))}
  pure val reversedChoiceB: BGraph = {deadline: BTime100, nodes: inactiveGraphB.nodes.put(BFirstFill,
    obligationB(Set(), Set(), BChoiceTrigger({id: BOther, chooser: Mallory, lower: 2, upper: -1}), BPayTransfers(List())))}
  pure def oneRowB(node: BNode): BGraph = {deadline: BTime100,
    nodes: inactiveGraphB.nodes.put(BDepositAlice, node)}
  pure val fundedStateB: BState = {...emptyBState(BTime0),
    accounts: CORE_ACCOUNTS.mapBy(_ => 0).put(bAliceA, 10),
    discharged: Set(BDepositAlice), excluded: Set(BTimeoutEmpty)}
  run admittedGraphsAndStatesTest = all {
    validBGraph(swapGraphB), validBGraph(installmentGraphB), validBGraph(negativeDepositGraphB),
    validBGraph(depositOnlyB), validBGraph(reversedChoiceB), validBGraph(inactiveGraphB),
    validBState(swapInitialB), validBState(installmentInitialB), validBState(negativeInitialB),
    validBState({graph: swapGraphB, state: fundedStateB}),
    validBState({graph: depositOnlyB, state: {...emptyBState(BTime101),
      accounts: CORE_ACCOUNTS.mapBy(_ => 280), choices: B_CHOICE_IDS.mapBy(_ => IntValue(-1))}}),
  }
  run graphMapAndReferencesTest = all {
    not(validBGraph({...swapGraphB, nodes: Map()})),
    not(validBGraph({...swapGraphB, nodes: B_NODE_IDS.exclude(Set(BTimeoutFive))
      .mapBy(id => swapGraphB.nodes.get(id))})),
    not(validBGraph({...swapGraphB, deadline: BTime101})),
    not(validBGraph(oneRowB(obligationB(Set(BDepositBob), Set(), BTimeoutTrigger, BRefundAll)))),
    not(validBGraph(oneRowB(obligationB(Set(BDepositAlice), Set(), BTimeoutTrigger, BRefundAll)))),
    not(validBGraph(oneRowB(obligationB(Set(), Set(BDepositBob), BTimeoutTrigger, BRefundAll)))),
    not(validBGraph(oneRowB(obligationB(Set(), Set(BDepositAlice), BTimeoutTrigger, BRefundAll)))),
    not(validBGraph({deadline: BTime100, nodes: inactiveGraphB.nodes
      .put(BDepositAlice, obligationB(Set(BDepositBob), Set(), BTimeoutTrigger, BRefundAll))
      .put(BDepositBob, obligationB(Set(), Set(), BTimeoutTrigger, BRefundAll))})),
  }
  run graphTriggerAndPaymentDomainsTest = all {
    B_DEPOSIT_QUANTITIES.forall(q => validBGraph(oneRowB(obligationB(Set(), Set(),
      BDepositTrigger({account: bAliceA, depositor: Mallory, quantity: q}), BDepositTransfer)))),
    not(validBGraph(oneRowB(obligationB(Set(), Set(),
      BDepositTrigger({account: bAliceA, depositor: Alice, quantity: 2}), BDepositTransfer)))),
    not(validBGraph(oneRowB(obligationB(Set(), Set(),
      BDepositTrigger({account: bAliceA, depositor: Alice, quantity: 10}), BRefundAll)))),
    not(validBGraph(oneRowB(obligationB(Set(), Set(), BTimeoutTrigger, BDepositTransfer)))),
    not(validBGraph(oneRowB(obligationB(Set(), Set(),
      BChoiceTrigger({id: BOther, chooser: Bob, lower: 0, upper: 3}), BRefundAll)))),
    not(validBGraph(oneRowB(obligationB(Set(), Set(),
      BChoiceTrigger({id: BOther, chooser: Bob, lower: 0, upper: 1}), BDepositTransfer)))),
    not(validBGraph(oneRowB(obligationB(Set(), Set(),
      BChoiceTrigger({id: BOther, chooser: Bob, lower: 0, upper: 1}),
      BPayTransfers(List({source: bAliceA, recipient: Bob, quantity: 1})))))),
    not(validBGraph(oneRowB(obligationB(Set(), Set(),
      BChoiceTrigger({id: BOther, chooser: Bob, lower: 0, upper: 1}),
      BPayTransfers(List({source: bAliceA, recipient: Bob, quantity: 5},
        {source: bAliceA, recipient: Bob, quantity: 10}, {source: bAliceA, recipient: Bob, quantity: 20})))))),
  }
  run stateMapsAndDomainsTest = all {
    not(validBState({...swapInitialB, state: {...swapInitialB.state, accounts: Map()}})),
    not(validBState({...swapInitialB, state: {...swapInitialB.state, choices: Map()}})),
    not(validBState({...swapInitialB, state: {...swapInitialB.state,
      accounts: CORE_ACCOUNTS.exclude(Set(bBobB)).mapBy(_ => 0)}})),
    not(validBState({...swapInitialB, state: {...swapInitialB.state,
      choices: B_CHOICE_IDS.exclude(Set(BOther)).mapBy(_ => NoInt)}})),
    not(validBState({...swapInitialB, state: {...swapInitialB.state,
      accounts: swapInitialB.state.accounts.put(bAliceA, -1)}})),
    not(validBState({...swapInitialB, state: {...swapInitialB.state,
      accounts: swapInitialB.state.accounts.put(bAliceA, 281)}})),
    not(validBState({...swapInitialB, state: {...swapInitialB.state,
      choices: swapInitialB.state.choices.put(BOther, IntValue(3))}})),
    not(validBState({graph: {...swapGraphB, nodes: Map()}, state: {...emptyBState(BTime0), accounts: Map()}})),
  }
  run dependencyAndExclusionStateTest = all {
    not(validBState({...swapInitialB, state: {...emptyBState(BTime0), discharged: Set(BFirstFill)}})),
    not(validBState({...swapInitialB, state: {...emptyBState(BTime0), excluded: Set(BFirstFill)}})),
    not(validBState({...swapInitialB, state: {...emptyBState(BTime0),
      discharged: Set(BDepositBob), excluded: Set(BTimeoutAlice)}})),
    not(validBState({...swapInitialB, state: {...fundedStateB, excluded: Set()}})),
    not(validBState({...swapInitialB, state: {...fundedStateB,
      excluded: Set(BTimeoutEmpty, BTimeoutAlice)}})),
    not(validBState({...swapInitialB, state: {...emptyBState(BTime0),
      discharged: Set(BDepositAlice, BTimeoutEmpty),
      excluded: Set(BDepositAlice, BDepositBob, BSettleSwap, BRefundSwap,
        BTimeoutEmpty, BTimeoutAlice, BTimeoutFunded)}})),
  }
  run inputDomainsTest = all {
    validBInput(BNoInput),
    B_DEPOSIT_QUANTITIES.forall(q => validBInput(BDepositInput({account: bBobB, depositor: Mallory, quantity: q}))),
    B_CHOICE_VALUES.forall(q => validBInput(BChoiceInput({id: BOther, chooser: Mallory, chosen: q}))),
    not(validBInput(BDepositInput({account: bAliceA, depositor: Alice, quantity: 2}))),
    not(validBInput(BChoiceInput({id: BSettle, chooser: Bob, chosen: 3}))),
  }
```

- [ ] Create the foundation module with the following behavioral scaffold, then capture RED. Inputs such as deposit -1 are in domain even though their eventual evaluation may reject them; validation must not decide that semantic outcome.

```quint
module candidate_b_foundation {
  import effects.* from "./effects"
  import observations.* from "./observations"
  import candidate_b_types.* from "./candidate_b_types"
  pure def validBGraph(graph: BGraph): bool = false
  pure def validBState(before: BPredecessor): bool = false
  pure def validBInput(input: BInput): bool = false
}
```

```bash
capture_b_foundation red-validation
```

Expected: typecheck exit0; admitted graph/state and input-domain tests fail on actual Boolean results. Keep the original five literal tests passing.

- [ ] Replace the scaffold foundation module with this complete validation implementation. The helpers that read maps iterate actual key sets. The public validators use sequential `if` guards before fixed-domain lookups; do not replace them with conjunctions that rely on backend short-circuit behavior.

```quint
module candidate_b_foundation {
  import effects.* from "./effects"
  import observations.* from "./observations"
  import candidate_b_types.* from "./candidate_b_types"

  pure def nodeRankB(id: BNodeId): int = match id {
    | BDepositAlice => 0 | BDepositBob => 1 | BSettleSwap => 2 | BRefundSwap => 3
    | BTimeoutEmpty => 4 | BTimeoutAlice => 5 | BTimeoutFunded => 6
    | BFirstFill => 7 | BSecondFill => 8 | BRecoverTen => 9 | BRecoverFive => 10
    | BTimeoutTen => 11 | BTimeoutFive => 12
  }
  pure def prerequisitesB(node: BNode): Set[BNodeId] = match node {
    | BInactive => Set() | BObligation(row) => row.requires
  }
  pure def exclusionsB(node: BNode): Set[BNodeId] = match node {
    | BInactive => Set() | BObligation(row) => row.excludes
  }
  pure def activeNodesB(graph: BGraph): Set[BNodeId] = graph.nodes.keys().filter(id =>
    match graph.nodes.get(id) { | BInactive => false | BObligation(_) => true })
  pure def declaredExclusionsB(graph: BGraph, discharged: Set[BNodeId]): Set[BNodeId] =
    discharged.intersect(graph.nodes.keys()).fold(Set(), (acc, id) =>
      acc.union(exclusionsB(graph.nodes.get(id))))
  pure def validBPayment(payment: BPayment): bool = CORE_ACCOUNTS.contains(payment.source)
    and PRINCIPALS.contains(payment.recipient) and B_PAYMENT_QUANTITIES.contains(payment.quantity)
  pure def validChoiceRuleB(rule: BTransferRule): bool = match rule {
    | BDepositTransfer => false
    | BRefundAll => true
    | BPayTransfers(payments) => payments.length() <= 2
        and payments.foldl(true, (ok, payment) => ok and validBPayment(payment))
  }
  pure def validTriggerRuleB(trigger: BTrigger, rule: BTransferRule): bool = match trigger {
    | BDepositTrigger(d) => CORE_ACCOUNTS.contains(d.account) and PRINCIPALS.contains(d.depositor)
        and B_DEPOSIT_QUANTITIES.contains(d.quantity) and rule == BDepositTransfer
    | BChoiceTrigger(c) => B_CHOICE_IDS.contains(c.id) and PRINCIPALS.contains(c.chooser)
        and B_CHOICE_VALUES.contains(c.lower) and B_CHOICE_VALUES.contains(c.upper)
        and validChoiceRuleB(rule)
    | BTimeoutTrigger => rule == BRefundAll
  }
  pure def validBGraph(graph: BGraph): bool =
    if (graph.nodes.keys() != B_NODE_IDS) false
    else if (graph.deadline != BTime100) false
    else {
      val active = activeNodesB(graph)
      B_NODE_IDS.forall(id => match graph.nodes.get(id) {
        | BInactive => true
        | BObligation(row) => row.requires.forall(parent =>
            active.contains(parent) and nodeRankB(parent) < nodeRankB(id))
            and row.excludes.forall(other => active.contains(other) and other != id)
            and validTriggerRuleB(row.trigger, row.transferRule)
      })
    }
  pure def validStoredChoiceB(value: OptionalInt): bool = match value {
    | NoInt => true | IntValue(chosen) => B_CHOICE_VALUES.contains(chosen)
  }
  pure def validBState(before: BPredecessor): bool =
    if (not(validBGraph(before.graph))) false
    else if (before.state.accounts.keys() != CORE_ACCOUNTS) false
    else if (before.state.choices.keys() != B_CHOICE_IDS) false
    else {
      val state = before.state
      val active = activeNodesB(before.graph)
      if (not(state.discharged.forall(id => active.contains(id)))) false
      else if (not(state.excluded.forall(id => active.contains(id)))) false
      else and {
        CORE_ACCOUNTS.forall(a => state.accounts.get(a) >= 0 and state.accounts.get(a) <= 280),
        B_CHOICE_IDS.forall(id => validStoredChoiceB(state.choices.get(id))),
        B_TIMES.contains(state.minimumTime),
        state.discharged.intersect(state.excluded) == Set(),
        state.discharged.forall(id => prerequisitesB(before.graph.nodes.get(id))
          .forall(parent => state.discharged.contains(parent))),
        state.excluded == declaredExclusionsB(before.graph, state.discharged),
      }
    }
  pure def validBInput(input: BInput): bool = match input {
    | BNoInput => true
    | BDepositInput(d) => CORE_ACCOUNTS.contains(d.account) and PRINCIPALS.contains(d.depositor)
        and B_DEPOSIT_QUANTITIES.contains(d.quantity)
    | BChoiceInput(c) => B_CHOICE_IDS.contains(c.id) and PRINCIPALS.contains(c.chooser)
        and B_CHOICE_VALUES.contains(c.chosen)
  }
}
```

Native sum types already prevent extra enum keys or unknown principals/assets. Complete-map tests therefore exercise omissions; invalid integer fields exercise out-of-domain values. An admitted state is not required to have library balances, historical choices or an established reachable discharge trace. `declaredExclusionsB` is lookup-total on malformed input but is only used as a semantic union after validation has established the relevant node domain.

- [ ] Capture GREEN, require all eleven tests to pass, then commit the validation task's two files only.

```bash
capture_b_foundation green-validation
git add specs/quint/s02/candidate_b_foundation.qnt specs/quint/s02/candidate_b_foundation_test.qnt
git diff --cached --stat
git commit --only -m "feat: validate generic native B graph and state domains" -- specs/quint/s02/candidate_b_foundation.qnt specs/quint/s02/candidate_b_foundation_test.qnt
```

## Task 3: Structural graph status and invalid-predecessor inspection

**Files:** Extend `specs/quint/s02/candidate_b_foundation.qnt` and `specs/quint/s02/candidate_b_foundation_test.qnt`.

**Interfaces:** Consumes `validBGraph`, `validBState`, `activeNodesB`, `prerequisitesB` and Task 1's carriers. Produces `remainingNodesB(before: BPredecessor): Set[BNodeId]`, `frontierB(before: BPredecessor): Set[BNodeId]`, `graphStatusB(before: BPredecessor): BGraphStatus` with the adopted valid-predecessor semantic precondition, and total `inspectGraphStatusB(before: BPredecessor): BStatusInspection`. The future evaluator must validate graph then state then input, use `frontierB` only after successful validation, and check each computed successor via `validBState({graph: request.before.graph, state: after})`.

- [ ] Append these tests and hand-constructed state fixtures to the existing test module. A fixture's `discharged` field is test data, not an executed obligation. No action or transition is fabricated.

```quint
  pure val depositDischargedB: BPredecessor = {graph: depositOnlyB,
    state: {...emptyBState(BTime0), accounts: CORE_ACCOUNTS.mapBy(_ => 0).put(bAliceA, 10),
      discharged: Set(BDepositAlice)}}
  pure val excludedPrerequisiteGraphB: BGraph = {deadline: BTime100, nodes: inactiveGraphB.nodes
    .put(BDepositAlice, obligationB(Set(), Set(BDepositBob),
      BChoiceTrigger({id: BOther, chooser: Alice, lower: 1, upper: 1}), BPayTransfers(List())))
    .put(BDepositBob, obligationB(Set(), Set(), BTimeoutTrigger, BRefundAll))
    .put(BSettleSwap, obligationB(Set(BDepositBob), Set(), BTimeoutTrigger, BRefundAll))}
  pure val exhaustedFixtureB: BPredecessor = {graph: excludedPrerequisiteGraphB,
    state: {...emptyBState(BTime0), discharged: Set(BDepositAlice), excluded: Set(BDepositBob)}}
  pure val settledFixtureB: BPredecessor = {graph: swapGraphB,
    state: {...emptyBState(BTime0), choices: B_CHOICE_IDS.mapBy(_ => NoInt).put(BSettle, IntValue(1)),
      discharged: Set(BDepositAlice, BDepositBob, BSettleSwap),
      excluded: Set(BTimeoutEmpty, BTimeoutAlice, BRefundSwap, BTimeoutFunded)}}
  run initialFrontiersTest = all {
    frontierB(swapInitialB) == Set(BDepositAlice, BTimeoutEmpty),
    frontierB(installmentInitialB) == Set(BFirstFill, BRecoverTen, BTimeoutTen),
    frontierB(negativeInitialB) == Set(BDepositAlice, BTimeoutEmpty),
    inspectGraphStatusB(swapInitialB) == BStatus(BOpen),
    inspectGraphStatusB(installmentInitialB) == BStatus(BOpen),
    inspectGraphStatusB(negativeInitialB) == BStatus(BOpen),
  }
  run structuralStatusFixturesTest = all {
    validBState(depositDischargedB), inspectGraphStatusB(depositDischargedB) == BStatus(BStranded),
    validBState(exhaustedFixtureB), inspectGraphStatusB(exhaustedFixtureB) == BStatus(BExhausted),
    remainingNodesB(exhaustedFixtureB) == Set(BSettleSwap), frontierB(exhaustedFixtureB) == Set(),
    validBState(settledFixtureB), inspectGraphStatusB(settledFixtureB) == BStatus(BFinanciallyComplete),
    inspectGraphStatusB({graph: inactiveGraphB, state: emptyBState(BTime0)}) == BStatus(BFinanciallyComplete),
    inspectGraphStatusB({...exhaustedFixtureB, state: {...exhaustedFixtureB.state,
      accounts: CORE_ACCOUNTS.mapBy(_ => 0).put(bAliceA, 1)}}) == BStatus(BStranded),
    inspectGraphStatusB({graph: reversedChoiceB, state: emptyBState(BTime0)}) == BStatus(BOpen),
    inspectGraphStatusB({graph: depositOnlyB, state: emptyBState(BTime101)}) == BStatus(BOpen),
  }
  run invalidStatusInspectionTest = {
    val badGraph: BPredecessor = {graph: {...swapGraphB, nodes: Map()},
      state: {...emptyBState(BTime0), accounts: Map()}}
    val badState: BPredecessor = {...swapInitialB, state: {...emptyBState(BTime0), accounts: Map()}}
    all {
      inspectGraphStatusB(badGraph) == BStatusDiagnostic({before: badGraph, reason: BInvalidGraph}),
      inspectGraphStatusB(badState) == BStatusDiagnostic({before: badState, reason: BInvalidState}),
    }
  }
  run orderAndExclusionHelpersTest = all {
    B_NODE_ORDER.foldl({position: 0, ok: true}, (acc, id) =>
      {position: acc.position + 1, ok: acc.ok and nodeRankB(id) == acc.position}).ok,
    activeNodesB(swapGraphB) == swapIds,
    declaredExclusionsB(swapGraphB, Set(BDepositAlice, BDepositBob)) == Set(BTimeoutEmpty, BTimeoutAlice),
    remainingNodesB(settledFixtureB) == Set(),
  }
```

- [ ] Add these lookup-total helper implementations and the intentionally incomplete status definitions inside the foundation module, retaining all existing validation functions. The `graphStatusB` scaffold must return BOpen until the failing classification expectations have been captured.

```quint
  pure def remainingNodesB(before: BPredecessor): Set[BNodeId] = activeNodesB(before.graph)
    .exclude(before.state.discharged).exclude(before.state.excluded)
  pure def frontierB(before: BPredecessor): Set[BNodeId] = remainingNodesB(before).filter(id =>
    prerequisitesB(before.graph.nodes.get(id)).forall(parent => before.state.discharged.contains(parent)))
  pure def graphStatusB(before: BPredecessor): BGraphStatus = BOpen
  pure def inspectGraphStatusB(before: BPredecessor): BStatusInspection =
    if (not(validBGraph(before.graph))) BStatusDiagnostic({before: before, reason: BInvalidGraph})
    else if (not(validBState(before))) BStatusDiagnostic({before: before, reason: BInvalidState})
    else BStatus(graphStatusB(before))
```

```bash
capture_b_foundation red-status
```

Expected: typecheck exit0; `structuralStatusFixturesTest` fails because BOpen differs from the required BStranded/BExhausted/BFinanciallyComplete results. The prior eleven tests and invalid-predecessor inspection continue to pass.

- [ ] Replace only `graphStatusB` with the complete adopted ordered classification:

```quint
  // Semantic precondition: validBState(before). Use inspectGraphStatusB at an unchecked boundary.
  pure def graphStatusB(before: BPredecessor): BGraphStatus =
    if (frontierB(before).size() > 0) BOpen
    else if (before.state.accounts.keys().exists(a => before.state.accounts.get(a) > 0)) BStranded
    else if (remainingNodesB(before).size() > 0) BExhausted
    else BFinanciallyComplete
```

Every raw helper remains lookup-total because frontier members are actual graph keys and account iteration uses actual account keys. Only the checked inspection is an API for arbitrary predecessors; the raw status function has no semantic admission claim outside its documented precondition. No synthetic status fallback or money movement is introduced.

- [ ] Capture GREEN and verify every final file plus the import boundary. Fifteen deterministic tests should be discovered; record the actual count and every exit code rather than assuming this expectation.

```bash
capture_b_foundation green-status
quint typecheck specs/quint/s02/candidate_b_types.qnt
quint typecheck specs/quint/s02/candidate_b_graphs.qnt
quint typecheck specs/quint/s02/candidate_b_foundation.qnt
quint typecheck specs/quint/s02/candidate_b_foundation_test.qnt
rg -n '^  import ' specs/quint/s02/candidate_b_types.qnt specs/quint/s02/candidate_b_graphs.qnt specs/quint/s02/candidate_b_foundation.qnt specs/quint/s02/candidate_b_foundation_test.qnt
git diff --check
```

Expected: all typechecks exit0, fifteen tests pass, and imports contain only the four native modules plus `effects`/`observations`. No `quint run` harness or Apalache job belongs here: these are pure foundation functions and explicit initialization/status fixtures. Requiring a simulated discharge in this unit would implement the evaluator prematurely.

- [ ] Commit only this task's two source files after inspecting staged scope:

```bash
git add specs/quint/s02/candidate_b_foundation.qnt specs/quint/s02/candidate_b_foundation_test.qnt
git diff --cached --stat
git commit --only -m "feat: classify native B graph status with checked inspection" -- specs/quint/s02/candidate_b_foundation.qnt specs/quint/s02/candidate_b_foundation_test.qnt
```

## Source-only review handoff

- [ ] Send root the final source commit(s), four exact source hashes, all three RED/GREEN receipt-directory paths, each terminal typecheck/test exit code and the actual test count. Include `effects.qnt` and `observations.qnt` hashes from the tested closure. Preserve raw outputs and both intentionally incomplete and final source bytes; an after-the-fact recreated RED is not evidence.
- [ ] Explain the tested predicates precisely: complete literal graphs, declared-domain admission/rejection, state structural validity and graph-status classification of explicit fixtures. Do not claim native discharge reachability, state preservation under evaluation, money movement, Core correspondence, authority integration, universal generic non-locking, Council approval or S02 completion.
- [ ] Request root's nonauthor review of the source-only implementation unit before beginning evaluation. The plan author hands off this document for plan review now and performs no `.qnt` edit or job in the planning turn.

Next-unit contract is fixed: `evaluateB(request: BRequest): BExecution` must use this foundation's validation order, original-request diagnostics, finite domains, fixed `B_NODE_ORDER` and `frontierB`. It must implement the adopted error priority and check complete successors through `validBState`; no evaluator body, mapper type or authority code is part of this plan.

## Plan self-review and root review questions

- Coverage: adopted carrier sketch, all thirteen row positions, all three library maps, negative declared deposit, swap semantic time0, optional choice distinction, complete maps, graph reference/order constraints, transfer-rule compatibility, finite input/state domains and all four graph statuses each have concrete code and assertions above.
- Boundary: full native request/result types are declared for the next evaluator. Core mapping result types are intentionally absent. The additional `BStatusInspection` is a foundation-only diagnostic sum carrying its original predecessor and grants no execution authority.
- Type consistency: `validBState` and status functions accept `BPredecessor`; next-unit successor validation therefore passes the unchanged graph with the computed state. `BPayment` declaration validation constrains rule quantities only; it does not constrain future refund-output quantities to `{5,10,20}`.
- Lookup review: graph and state validators guard complete maps before domain lookups; helper traversal uses actual keys. Malformed maps have negative tests and typed inspection diagnostics.
- Root review question: confirm the explicit reversed-interval admission test and the valid-predecessor status interface plus checked inspection wrapper. Root has directed preserving reversed in-domain intervals; this plan adds no `lower <= upper` admission rule.
- Deferred design obligations are outside this first unit: native positive/error execution, actual Time0 settlement, generic discharge/stranded/exhausted witnesses, successor-preservation checks, library correspondence and semantic mutations, and common authority integration. Their absence is a scoped dependency, not a foundation success claim.

Plan saved for root review. Execution remains pending that review under the delegated workflow; no additional user permission question is introduced.
