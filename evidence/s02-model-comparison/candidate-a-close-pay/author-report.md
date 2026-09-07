# Candidate A Tasks 1–2 implementation receipt

Scope: local downstream implementation of finite carriers, domain checks, the canonical swap node table, and one-step Close/Pay reductions only. This is author-side implementation evidence, not independent review, Python correspondence, model checking, Candidate A completion, integration, or Council acceptance.

Starting HEAD: `f933acab722117838ec638cf8939840bbeedffa4`.
Working directory for every command below:
`/home/charl/Moriarty/.worktrees/s01-audit-start`.
CLI: `/home/charl/.npm-global/bin/quint`, version `0.32.0`.
No shared source, existing installment files, manifests, or Git state were edited by this worker. Root's concurrently created Python reference-vector file is outside this unit and was not edited.

## Scope and deliberate temporary boundaries

- `candidate_a_types.qnt`: finite node, time, choice, input, state and result carriers; exact total-map, rank-decrease, scalar-domain, case-length, and balance-potential checks. Int/List carriers become finite only under these explicit admission checks.
- `candidate_a_programs.qnt`: zero state, Close graph, and literal frozen canonical swap graph. The installment graph remains Task 6; no installment interpreter was added here.
- `candidate_a_core.qnt`: only the Close/Pay clauses from `moriarty/core.py:228` and `:237`. Close picks the first positive account from the canonical list. Pay clamps requested payment exactly, emits exact warning records, preserves choices/minimum time, and counts one reduction even when nothing is paid.
- `candidate_a_core_test.qnt`: 15 domain/table tests and 11 reduction/diagnostic tests. Nine reduction tests compare quiescence or complete one-step result records; the other two distinguish unsupported and invalid-domain calls.
- `canReduceOnce` means the evaluator supports this admitted state. Empty Close is supported and quiescent; the guard does not assert a counted reduction exists.
- Root approved explicit `ReductionUnavailableA` for valid If/When during this increment. `ReductionOutsideDomainA` is separate for invalid finite-domain calls. Neither is a Core error, successful transaction, or false quiescence. Task 3 must remove unavailable results for every valid frozen constructor.
- Future reduction-bound and transaction result types are carriers only. No If/When execution, observation evaluation, input application, quiescence iteration, transaction rollback, projection, authority interpreter, or EvidenceValid production is implemented.

The sketch's record field `action` is a Quint reserved word. The first bootstrap typecheck failed on that parse error, and the field was renamed `caseAction` consistently before semantic tests. That syntax failure is not counted as behavioral RED. A tool-orchestration JavaScript typo also failed before execution and changed no files.

Skills used: Quint modeling/language, executing-plans, and test-driven development kept domain validity separate from semantics and required source-bound RED/GREEN. The checkpoint skill's session-database write is outside this worker's explicitly authorized files and was not performed; this scoped receipt carries the handoff and replayable measurements instead. Root retains session orchestration.

## Exact source-bound stages

Each RED directory was created with apply_patch before implementation of that stage. It contains an exact independent local import closure, including unchanged copies of effects/observations. RED commands ran directly against these preserved paths, not mutable live files. Do not modify the snapshots.

Task 1 scaffold typecheck (before domain/table logic):

```text
$ quint typecheck specs/quint/s02/candidate_a_core_test.qnt
[no terminal output]
exit_code: 0
```

Task 1 behavioral RED:

```text
$ quint test .superpowers/sdd/candidate-a-stages/task1-red/candidate_a_core_test.qnt --backend=rust --seed=42

  candidate_a_core_test
    1) finiteProgramDomainTest failed after 1 test(s)
    ok cycleRejectedByDomainTest passed 1 test(s)
    ok absentChoiceDiffersFromZeroTest passed 1 test(s)
    ok canonicalAccountOrderTest passed 1 test(s)
    2) canonicalSwapNodeTableTest failed after 1 test(s)
    3) canonicalSwapDomainTest failed after 1 test(s)
    4) finiteScalarMappingsTest failed after 1 test(s)
    ok missingNodeMapRejectedTest passed 1 test(s)
    ok oversizedCaseListRejectedTest passed 1 test(s)
    ok invalidConstantRejectedTest passed 1 test(s)
    ok invalidChoiceBoundsRejectedTest passed 1 test(s)
    5) statePotentialBoundaryTest failed after 1 test(s)
    ok malformedStateMapsRejectedTest passed 1 test(s)
    6) stateValueDomainTest failed after 1 test(s)
    7) inputDomainTest failed after 1 test(s)

  8 passing (302ms)
  7 failed

  1) finiteProgramDomainTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-stages/task1-red/candidate_a_core_test.qnt:7:33
        7:   run finiteProgramDomainTest = assert(validProgram(closeProgram)
                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        8:     and validState(closeProgram, emptyAState(N0)) and validInput(NoAInput))
           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=finiteProgramDomainTest to repeat.
  2) canonicalSwapNodeTableTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-stages/task1-red/candidate_a_core_test.qnt:23:36
        23:   run canonicalSwapNodeTableTest = assert(canonicalSwap.root == N6 and canonicalSwap.nodes ==
                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        24:     A_NODE_IDS.mapBy(_ => CloseA)
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        25:       .put(N1, PayA({account: {owner: Bob, asset: TokenB}, payee: Alice, amount: ConstantA(20), continuation: N0}))
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        26:       .put(N2, PayA({account: {owner: Alice, asset: TokenA}, payee: Bob, amount: ConstantA(10), continuation: N1}))
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        27:       .put(N3, IfA({observation: ChoiceEqualsA({id: SettleId, expected: 1}), thenNode: N2, elseNode: N0}))
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        28:       .put(N4, WhenA({cases: List({caseAction: ChoiceA({id: SettleId, chooser: Bob, lower: 0, upper: 1}),
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        29:         continuation: N3}), timeout: Time100, timeoutNode: N0}))
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        30:       .put(N5, WhenA({cases: List({caseAction: DepositA({account: {owner: Bob, asset: TokenB}, depositor: Bob,
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        31:         amount: ConstantA(20)}), continuation: N4}), timeout: Time100, timeoutNode: N0}))
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        32:       .put(N6, WhenA({cases: List({caseAction: DepositA({account: {owner: Alice, asset: TokenA}, depositor: Alice,
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        33:         amount: ConstantA(10)}), continuation: N5}), timeout: Time100, timeoutNode: N0})))
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=canonicalSwapNodeTableTest to repeat.
  3) canonicalSwapDomainTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-stages/task1-red/candidate_a_core_test.qnt:34:33
        34:   run canonicalSwapDomainTest = assert(validProgram(canonicalSwap) and validState(canonicalSwap, emptyAState(N6)))
                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=canonicalSwapDomainTest to repeat.
  4) finiteScalarMappingsTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-stages/task1-red/candidate_a_core_test.qnt:35:34
        35:   run finiteScalarMappingsTest = assert(A_TIMES.map(time => timeValue(time)) == Set(0, 1, 2, 100, 101)
                                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        36:     and A_CHOICE_IDS.map(id => choiceName(id)) == Set("settle", "fill1", "fill2", "recover", "other")
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        37:     and A_NODE_IDS.map(node => rank(node)) == 0.to(15))
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=finiteScalarMappingsTest to repeat.
  5) statePotentialBoundaryTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-stages/task1-red/candidate_a_core_test.qnt:53:5
        53:     assert(validState(canonicalSwap, allowed) and not(validState(canonicalSwap, excessive)))
                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=statePotentialBoundaryTest to repeat.
  6) stateValueDomainTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-stages/task1-red/candidate_a_core_test.qnt:59:5
        59:     assert(validState(closeProgram, {...empty, choices: empty.choices.put(SettleId, IntValue(0))})
                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        60:       and not(validState(closeProgram, {...empty, choices: empty.choices.put(SettleId, IntValue(3))}))
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        61:       and not(validState(closeProgram, {...empty, accounts: empty.accounts.put(aliceA, -1)})))
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=stateValueDomainTest to repeat.
  7) inputDomainTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-stages/task1-red/candidate_a_core_test.qnt:63:25
        63:   run inputDomainTest = assert(
                                    ^^^^^^^
        64:     validInput(PresentAInput(DepositInputA({account: aliceA, depositor: Mallory, quantity: 21})))
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        65:       and not(validInput(PresentAInput(DepositInputA({account: aliceA, depositor: Alice, quantity: 22}))))
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        66:       and validInput(PresentAInput(ChoiceInputA({id: OtherId, chooser: Mallory, chosen: -1})))
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        67:       and not(validInput(PresentAInput(ChoiceInputA({id: SettleId, chooser: Bob, chosen: 3})))))
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=inputDomainTest to repeat.


  Use --verbosity=3 to show executions.
  Further debug with: quint test --verbosity=3 .superpowers/sdd/candidate-a-stages/task1-red/candidate_a_core_test.qnt
error: Tests failed
exit_code: 1
```

The seven failed assertions exercise positive domain admission, the literal swap table, scalar mappings, the exact potential boundary, stored zero, and supplied input domains. Eight other checks passed against the stubs; they are not evidence that domain logic had already been implemented.

Task 1 GREEN, after domain/table implementation and before reduction stubs/tests were added:

```text
$ quint test specs/quint/s02/candidate_a_core_test.qnt --backend=rust --seed=42

  candidate_a_core_test
    ok finiteProgramDomainTest passed 1 test(s)
    ok cycleRejectedByDomainTest passed 1 test(s)
    ok absentChoiceDiffersFromZeroTest passed 1 test(s)
    ok canonicalAccountOrderTest passed 1 test(s)
    ok canonicalSwapNodeTableTest passed 1 test(s)
    ok canonicalSwapDomainTest passed 1 test(s)
    ok finiteScalarMappingsTest passed 1 test(s)
    ok missingNodeMapRejectedTest passed 1 test(s)
    ok oversizedCaseListRejectedTest passed 1 test(s)
    ok invalidConstantRejectedTest passed 1 test(s)
    ok invalidChoiceBoundsRejectedTest passed 1 test(s)
    ok statePotentialBoundaryTest passed 1 test(s)
    ok malformedStateMapsRejectedTest passed 1 test(s)
    ok stateValueDomainTest passed 1 test(s)
    ok inputDomainTest passed 1 test(s)

  15 passing (404ms)
exit_code: 0
```

The Task 1 GREEN types/programs are exactly the copies in `task2-red`. Its 15-test source is exactly `task1-red/candidate_a_core_test.qnt`.

Task 2 scaffold typecheck (before Close/Pay implementation):

```text
$ quint typecheck specs/quint/s02/candidate_a_core_test.qnt
[no terminal output]
exit_code: 0
```

Task 2 behavioral RED:

```text
$ quint test .superpowers/sdd/candidate-a-stages/task2-red/candidate_a_core_test.qnt --backend=rust --seed=42

  candidate_a_core_test
    1) closeEmptyQuiescentTest failed after 1 test(s)
    2) closeCanonicalRefundStepTest failed after 1 test(s)
    3) closeCanonicalNonAliceRefundTest failed after 1 test(s)
    4) payExactStepTest failed after 1 test(s)
    5) payRetainsExcessBalanceTest failed after 1 test(s)
    6) payPartialFourOfTenTest failed after 1 test(s)
    7) payFromMissingAccountTest failed after 1 test(s)
    8) payZeroWarningTest failed after 1 test(s)
    9) payNegativeWarningTest failed after 1 test(s)
    ok unsupportedIfWhenUnavailableTest passed 1 test(s)
    10) invalidDomainDistinctTest failed after 1 test(s)
    ok finiteProgramDomainTest passed 1 test(s)
    ok cycleRejectedByDomainTest passed 1 test(s)
    ok absentChoiceDiffersFromZeroTest passed 1 test(s)
    ok canonicalAccountOrderTest passed 1 test(s)
    ok canonicalSwapNodeTableTest passed 1 test(s)
    ok canonicalSwapDomainTest passed 1 test(s)
    ok finiteScalarMappingsTest passed 1 test(s)
    ok missingNodeMapRejectedTest passed 1 test(s)
    ok oversizedCaseListRejectedTest passed 1 test(s)
    ok invalidConstantRejectedTest passed 1 test(s)
    ok invalidChoiceBoundsRejectedTest passed 1 test(s)
    ok statePotentialBoundaryTest passed 1 test(s)
    ok malformedStateMapsRejectedTest passed 1 test(s)
    ok stateValueDomainTest passed 1 test(s)
    ok inputDomainTest passed 1 test(s)

  16 passing (898ms)
  10 failed

  1) closeEmptyQuiescentTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-stages/task2-red/candidate_a_core_test.qnt:14:33
        14:   run closeEmptyQuiescentTest = assert(canReduceOnce(closeProgram, emptyAState(N0))
                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        15:     and reduceOnce(closeProgram, emptyAState(N0)) == QuiescentA(emptyAState(N0)))
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=closeEmptyQuiescentTest to repeat.
  2) closeCanonicalRefundStepTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-stages/task2-red/candidate_a_core_test.qnt:21:5
        21:     assert(canReduceOnce(closeProgram, before) and reduceOnce(closeProgram, before) == expected)
                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=closeCanonicalRefundStepTest to repeat.
  3) closeCanonicalNonAliceRefundTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-stages/task2-red/candidate_a_core_test.qnt:26:5
        26:     assert(reduceOnce(closeProgram, before) == ReducedA({state: {...before, accounts: before.accounts.put(aliceB, 0)},
                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        27:       payments: List({source: aliceB, recipient: Alice, asset: TokenB, quantity: 5}), warnings: List(), reductions: 1}))
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=closeCanonicalNonAliceRefundTest to repeat.
  4) payExactStepTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-stages/task2-red/candidate_a_core_test.qnt:31:5
        31:     assert(canReduceOnce(payProgram(10), before) and reduceOnce(payProgram(10), before) == ReducedA({
                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        32:       state: {...before, continuation: N0, accounts: before.accounts.put(aliceA, 0)},
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        33:       payments: List({source: aliceA, recipient: Bob, asset: TokenA, quantity: 10}), warnings: List(), reductions: 1}))
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=payExactStepTest to repeat.
  5) payRetainsExcessBalanceTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-stages/task2-red/candidate_a_core_test.qnt:37:5
        37:     assert(reduceOnce(payProgram(5), before) == ReducedA({
                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        38:       state: {...before, continuation: N0, accounts: before.accounts.put(aliceA, 5)},
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        39:       payments: List({source: aliceA, recipient: Bob, asset: TokenA, quantity: 5}), warnings: List(), reductions: 1}))
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=payRetainsExcessBalanceTest to repeat.
  6) payPartialFourOfTenTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-stages/task2-red/candidate_a_core_test.qnt:43:5
        43:     assert(reduceOnce(payProgram(10), before) == ReducedA({
                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        44:       state: {...before, continuation: N0, accounts: before.accounts.put(aliceA, 0)},
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        45:       payments: List({source: aliceA, recipient: Bob, asset: TokenA, quantity: 4}),
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        46:       warnings: List({code: "partial_payment", requested: IntValue(10), paid: IntValue(4)}), reductions: 1}))
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=payPartialFourOfTenTest to repeat.
  7) payFromMissingAccountTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-stages/task2-red/candidate_a_core_test.qnt:50:5
        50:     assert(reduceOnce(payProgram(10), before) == ReducedA({state: {...before, continuation: N0}, payments: List(),
                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        51:       warnings: List({code: "partial_payment", requested: IntValue(10), paid: IntValue(0)}), reductions: 1}))
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=payFromMissingAccountTest to repeat.
  8) payZeroWarningTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-stages/task2-red/candidate_a_core_test.qnt:55:5
        55:     assert(reduceOnce(payProgram(0), before) == ReducedA({state: {...before, continuation: N0}, payments: List(),
                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        56:       warnings: List({code: "non_positive_payment", requested: IntValue(0), paid: IntValue(0)}), reductions: 1}))
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=payZeroWarningTest to repeat.
  9) payNegativeWarningTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-stages/task2-red/candidate_a_core_test.qnt:60:5
        60:     assert(reduceOnce(payProgram(-1), before) == ReducedA({state: {...before, continuation: N0}, payments: List(),
                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        61:       warnings: List({code: "non_positive_payment", requested: IntValue(-1), paid: IntValue(0)}), reductions: 1}))
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=payNegativeWarningTest to repeat.
  10) invalidDomainDistinctTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-stages/task2-red/candidate_a_core_test.qnt:71:5
        71:     assert(not(canReduceOnce(closeProgram, malformed))
                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        72:       and reduceOnce(closeProgram, malformed) == ReductionOutsideDomainA
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        73:       and reduceOnce(payProgram(21), fundedState(N1, 10)) == ReductionOutsideDomainA)
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=invalidDomainDistinctTest to repeat.


  Use --verbosity=3 to show executions.
  Further debug with: quint test --verbosity=3 .superpowers/sdd/candidate-a-stages/task2-red/candidate_a_core_test.qnt
error: Tests failed
exit_code: 1
```

All nine exact-result reduction assertions and the invalid-domain diagnostic assertion fail against the typed unavailable stub. The 15 earlier tests stay green. The explicit unsupported If/When test also passes because unavailable is its intended temporary result.

Task 2 first GREEN:

```text
$ quint test specs/quint/s02/candidate_a_core_test.qnt --backend=rust --seed=42

  candidate_a_core_test
    ok closeEmptyQuiescentTest passed 1 test(s)
    ok closeCanonicalRefundStepTest passed 1 test(s)
    ok closeCanonicalNonAliceRefundTest passed 1 test(s)
    ok payExactStepTest passed 1 test(s)
    ok payRetainsExcessBalanceTest passed 1 test(s)
    ok payPartialFourOfTenTest passed 1 test(s)
    ok payFromMissingAccountTest passed 1 test(s)
    ok payZeroWarningTest passed 1 test(s)
    ok payNegativeWarningTest passed 1 test(s)
    ok unsupportedIfWhenUnavailableTest passed 1 test(s)
    ok invalidDomainDistinctTest passed 1 test(s)
    ok finiteProgramDomainTest passed 1 test(s)
    ok cycleRejectedByDomainTest passed 1 test(s)
    ok absentChoiceDiffersFromZeroTest passed 1 test(s)
    ok canonicalAccountOrderTest passed 1 test(s)
    ok canonicalSwapNodeTableTest passed 1 test(s)
    ok canonicalSwapDomainTest passed 1 test(s)
    ok finiteScalarMappingsTest passed 1 test(s)
    ok missingNodeMapRejectedTest passed 1 test(s)
    ok oversizedCaseListRejectedTest passed 1 test(s)
    ok invalidConstantRejectedTest passed 1 test(s)
    ok invalidChoiceBoundsRejectedTest passed 1 test(s)
    ok statePotentialBoundaryTest passed 1 test(s)
    ok malformedStateMapsRejectedTest passed 1 test(s)
    ok stateValueDomainTest passed 1 test(s)
    ok inputDomainTest passed 1 test(s)

  26 passing (1209ms)
exit_code: 0
```

## Final author-side verification

No source changes followed the first Task 2 GREEN. A separate typecheck and final scoped test ran after the hash capture below:

```text
$ quint typecheck specs/quint/s02/candidate_a_core_test.qnt
[no terminal output]
exit_code: 0

$ quint test specs/quint/s02/candidate_a_core_test.qnt --backend=rust --seed=42

  candidate_a_core_test
    ok closeEmptyQuiescentTest passed 1 test(s)
    ok closeCanonicalRefundStepTest passed 1 test(s)
    ok closeCanonicalNonAliceRefundTest passed 1 test(s)
    ok payExactStepTest passed 1 test(s)
    ok payRetainsExcessBalanceTest passed 1 test(s)
    ok payPartialFourOfTenTest passed 1 test(s)
    ok payFromMissingAccountTest passed 1 test(s)
    ok payZeroWarningTest passed 1 test(s)
    ok payNegativeWarningTest passed 1 test(s)
    ok unsupportedIfWhenUnavailableTest passed 1 test(s)
    ok invalidDomainDistinctTest passed 1 test(s)
    ok finiteProgramDomainTest passed 1 test(s)
    ok cycleRejectedByDomainTest passed 1 test(s)
    ok absentChoiceDiffersFromZeroTest passed 1 test(s)
    ok canonicalAccountOrderTest passed 1 test(s)
    ok canonicalSwapNodeTableTest passed 1 test(s)
    ok canonicalSwapDomainTest passed 1 test(s)
    ok finiteScalarMappingsTest passed 1 test(s)
    ok missingNodeMapRejectedTest passed 1 test(s)
    ok oversizedCaseListRejectedTest passed 1 test(s)
    ok invalidConstantRejectedTest passed 1 test(s)
    ok invalidChoiceBoundsRejectedTest passed 1 test(s)
    ok statePotentialBoundaryTest passed 1 test(s)
    ok malformedStateMapsRejectedTest passed 1 test(s)
    ok stateValueDomainTest passed 1 test(s)
    ok inputDomainTest passed 1 test(s)

  26 passing (1064ms)
exit_code: 0
```

No stateful action was added in Tasks 1–2, so no simulated transaction trace or stateful reachability claim is made. These are deterministic pure tests, each with one execution. No full suite or `quint verify` command ran.

## Source hashes and unchanged-source check

Exact hash command ran both before and after final typecheck/test. The complete terminal outputs were byte-identical, including every RED snapshot and frozen input:

```text
$ sha256sum specs/quint/s02/candidate_a*.qnt .superpowers/sdd/candidate-a-stages/task1-red/*.qnt .superpowers/sdd/candidate-a-stages/task2-red/*.qnt moriarty/core.py moriarty/swap.py specs/quint/s02/effects.qnt specs/quint/s02/observations.qnt docs/superpowers/plans/2026-09-05-moriarty-s02-candidate-a-core.md /home/charl/.npm-global/bin/quint
742456caf169975d632e23e2e217c7586e89dc8f358a0b0903daaf25d15711c9  specs/quint/s02/candidate_a_core.qnt
c803545a878e03cd8675c1e7410995e12faf1a111d4fcba61207defd66282cda  specs/quint/s02/candidate_a_core_test.qnt
b443fc0beec22b867e4d36c21714fbaee63950961776d21b2ebe9463e190ab66  specs/quint/s02/candidate_a_programs.qnt
a2ad0fb7cd30bb0c53d80cf01bb76810033051f3d3f092d963676415cf5ee129  specs/quint/s02/candidate_a_types.qnt
4668525392fd1e4380cf87e22c7aceb870e325cff4f2b5adc030932cb171c0dd  .superpowers/sdd/candidate-a-stages/task1-red/candidate_a_core_test.qnt
bdb3d8eece863f2a72df1e7747c351e618de896e9be61f28ac6061fbaa123d59  .superpowers/sdd/candidate-a-stages/task1-red/candidate_a_programs.qnt
18c27d0e03e7e3cc09a2971f2d0b964c25bcb3b82a0483b0a7092667541efac2  .superpowers/sdd/candidate-a-stages/task1-red/candidate_a_types.qnt
dccc9208d9a42f01685000b1358f1d2f9684bf5e8ebe1d5deadd0c992557504c  .superpowers/sdd/candidate-a-stages/task1-red/effects.qnt
e44484ed9035e560ef9f4d012198671917750da42ab404022592db16cdd59bc3  .superpowers/sdd/candidate-a-stages/task1-red/observations.qnt
1f03188098afb10e5d281283bc7438355172c96152315cc049587aeedfa7a1fd  .superpowers/sdd/candidate-a-stages/task2-red/candidate_a_core.qnt
c803545a878e03cd8675c1e7410995e12faf1a111d4fcba61207defd66282cda  .superpowers/sdd/candidate-a-stages/task2-red/candidate_a_core_test.qnt
b443fc0beec22b867e4d36c21714fbaee63950961776d21b2ebe9463e190ab66  .superpowers/sdd/candidate-a-stages/task2-red/candidate_a_programs.qnt
a2ad0fb7cd30bb0c53d80cf01bb76810033051f3d3f092d963676415cf5ee129  .superpowers/sdd/candidate-a-stages/task2-red/candidate_a_types.qnt
dccc9208d9a42f01685000b1358f1d2f9684bf5e8ebe1d5deadd0c992557504c  .superpowers/sdd/candidate-a-stages/task2-red/effects.qnt
e44484ed9035e560ef9f4d012198671917750da42ab404022592db16cdd59bc3  .superpowers/sdd/candidate-a-stages/task2-red/observations.qnt
564a926779beb54bd16ff58481f5c950bae6fb50fcd5b0d7c9eb56427f83273b  moriarty/core.py
82e9e1da76af65706171049f0238953aca5333edec4aeed6caa43dbd57c79797  moriarty/swap.py
dccc9208d9a42f01685000b1358f1d2f9684bf5e8ebe1d5deadd0c992557504c  specs/quint/s02/effects.qnt
e44484ed9035e560ef9f4d012198671917750da42ab404022592db16cdd59bc3  specs/quint/s02/observations.qnt
cc8c66a1a4b94a788376dd9ba1649d6d485d00cb0c254020998a77cba3bf5860  docs/superpowers/plans/2026-09-05-moriarty-s02-candidate-a-core.md
ac12595b1cb7253feec93c79417615c6eb20fc3b6a3df35c5e3530b24e90a501  /home/charl/.npm-global/bin/quint
exit_code: 0
before_output_equals_after_output: true
```

## Handoff

Four new live Quint files and both RED snapshots are frozen for root's independent tests and source/evidence review. This worker made no commit. The next implementation is Task 3, not authorized by this receipt: evaluate ChoiceEquals with absent-versus-zero semantics, If/When reductions, and bounded quiescence. Input/transaction/rollback, installment execution, independent Python extraction/correspondence, and all acceptance gates remain open.
