# Candidate A Task 3 implementation receipt

Scope: author-side implementation of If/When reduction, a fixed 22-slot quiescence fold, and ordered input application. No computeTransaction, projection, policy interpreter, authority integration, Python correspondence, model-checking, Candidate A completion, or Council claim.

Task 1–2 source baseline supplied by root: `068b7cdd6d15bbb28659f56c062c8f93bccd290a`.
Prior evidence milestone: `982bdd8db4ab42b9bd9c40e451cdb283a94201ee`.
Root performed concurrent independent work; observed HEAD before final verification was `c8cedc1c0a343a5bb03ce4e14320cac62e2522fc`. This worker did not commit or alter Git state. Hashes below bind this unit independently of those root commits.

Working directory for all commands:
`/home/charl/Moriarty/.worktrees/s01-audit-start`.
Quint: `/home/charl/.npm-global/bin/quint`, version `0.32.0`.
Changes are confined to candidate_a_core.qnt, candidate_a_core_test.qnt, necessary explicit diagnostic carriers in candidate_a_types.qnt, this report, and the new Task 3 RED snapshot directory. The archived Task 1–2 report/snapshots and shared foundations were not changed.

## Interfaces and exact scope

Root approved the following refinement before behavior was implemented:

- `AInputEvaluation = InputAppliedA(AInputResult) | InputOutsideDomainA | InputPreconditionFailureA`.
- `AReductionEvaluation` adds `QuiescenceOutsideDomainA`, distinct from `ReductionBoundFailureA`.
- `ReductionUnavailableA` was removed after the typed RED snapshot: every admitted Close, Pay, If, and When now has an implemented one-step reduction.
- `evaluateObservation` has the explicit internal precondition of an admitted observation and total choice map. It compares against IntValue(expected), so NoInt is never stored zero.
- `applyInput` is a restricted internal modeling API, not a changed public frozen-Core transaction result. Its precondition is valid supplied input on a valid unexpired When after quiescence. A non-When or expired When gets InputPreconditionFailureA. Bad finite data gets InputOutsideDomainA. The Task 4 caller must reduce first, enforce source ordering, and handle NoAInput separately.
- Real input successes and the three possible input-stage Core errors are wrapped in InputAppliedA. Errors preserve the original input-stage state; this is not yet whole-transaction rollback.
- Input scanning is an ordered List fold. It stops after the first accepting case or exact matching nonpositive deposit. A matching choice bounds failure records a flag but permits a later case to accept. A wrong chooser/id does not set that flag.
- The quiescence fold has 22 ordered slots. It preserves completed accumulators; actual reductions, not slots, determine the count. It appends payments/warnings in execution order. It returns bound failure if the last slot did not observe quiescence, and a separate domain diagnostic if an admitted-state condition fails.

Source locators used: frozen core.py:214 (optional observation), :255 (If), :263 (expired When), :282 (ordered input), and :220 (reduction loop). The frozen Python hashes below remain unchanged.

## Verification scope and limits

The 52 deterministic pure tests consist of 25 retained Task 1–2 tests, one strengthened former temporary-unsupported test, and 26 added Task 3 tests. Tests compare exact states, errors, ordered payments/warnings, and reduction counts where applicable.

The bound witness executes 15 descending Pay0 nodes followed by six canonical account refunds: 21 reductions, 15 exact warnings, six ordered payments, and the 22nd slot observing quiescence. The literal-fixture test checks 11 program values at each of the five modeled times. These bounded tests are not exhaustive verification over every admissible program or a cross-language correspondence result.

Deadline checks cover time below, equal to, and after 100. The deadline-plus-input test checks pure timeout/refund reduction and the input-precondition diagnostic, not a public transaction or rollback result. Task 4 remains unimplemented.

Quint modeling/language, executing-plans, TDD, and verification-before-completion guided the typed scaffold, preserved RED closure, and fresh source-bound checks. No new provider dispatch, Foreman work, or external session-state write was performed.

## Typed scaffold and behavioral RED

First, the new wrapper declarations and disabled typed functions typechecked with the retained tests:

```text
$ quint typecheck specs/quint/s02/candidate_a_core_test.qnt
[no terminal output]
exit_code: 0
```

While authoring the added tests, one typecheck found use of Set.map on a List. Both expected-list constructions were changed to ordered foldl before semantic RED. This type error is not classified as behavioral RED.

The complete added test unit then typechecked against stubs:

```text
$ quint typecheck specs/quint/s02/candidate_a_core_test.qnt
[no terminal output]
exit_code: 0
```

Before implementation, apply_patch copied the exact six-file local import closure to `.superpowers/sdd/candidate-a-task3-stages/red/`. The actual RED command executed that preserved snapshot directly. The test source was not subsequently changed.

```text
$ sha256sum .superpowers/sdd/candidate-a-task3-stages/red/*.qnt specs/quint/s02/candidate_a*.qnt
948063f04cbf853314a984ef15d76563df72f7e0483a89252d57cbd6b8e2a267  .superpowers/sdd/candidate-a-task3-stages/red/candidate_a_core.qnt
42df4e4c11665f185efaf302b33418113302d9b2dead4574116d9e81af847767  .superpowers/sdd/candidate-a-task3-stages/red/candidate_a_core_test.qnt
b443fc0beec22b867e4d36c21714fbaee63950961776d21b2ebe9463e190ab66  .superpowers/sdd/candidate-a-task3-stages/red/candidate_a_programs.qnt
93ccf78860d69b51fbcbcdee18952d05d33810e4dc9ed34c05e06770b6e1bd17  .superpowers/sdd/candidate-a-task3-stages/red/candidate_a_types.qnt
dccc9208d9a42f01685000b1358f1d2f9684bf5e8ebe1d5deadd0c992557504c  .superpowers/sdd/candidate-a-task3-stages/red/effects.qnt
e44484ed9035e560ef9f4d012198671917750da42ab404022592db16cdd59bc3  .superpowers/sdd/candidate-a-task3-stages/red/observations.qnt
948063f04cbf853314a984ef15d76563df72f7e0483a89252d57cbd6b8e2a267  specs/quint/s02/candidate_a_core.qnt
42df4e4c11665f185efaf302b33418113302d9b2dead4574116d9e81af847767  specs/quint/s02/candidate_a_core_test.qnt
b443fc0beec22b867e4d36c21714fbaee63950961776d21b2ebe9463e190ab66  specs/quint/s02/candidate_a_programs.qnt
93ccf78860d69b51fbcbcdee18952d05d33810e4dc9ed34c05e06770b6e1bd17  specs/quint/s02/candidate_a_types.qnt
exit_code: 0
```

```text
$ quint test .superpowers/sdd/candidate-a-task3-stages/red/candidate_a_core_test.qnt --backend=rust --seed=42

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
    1) allFrozenConstructorsSupportedTest failed after 1 test(s)
    ok invalidDomainDistinctTest passed 1 test(s)
    2) ifAbsentEqualsZeroFalseTest failed after 1 test(s)
    3) ifStoredZeroTrueTest failed after 1 test(s)
    4) ifStoredMismatchFalseTest failed after 1 test(s)
    5) whenBeforeDeadlineQuiescentTest failed after 1 test(s)
    6) timeoutAtDeadlineStepTest failed after 1 test(s)
    7) timeoutAfterDeadlineStepTest failed after 1 test(s)
    8) quiescentWaitingZeroCountTest failed after 1 test(s)
    9) quiescentEmptyCloseZeroCountTest failed after 1 test(s)
    10) reductionBoundCompleteTest failed after 1 test(s)
    11) quiescencePreservesPaymentWarningOrderTest failed after 1 test(s)
    12) deadlineReductionRefundsBeforeInputTest failed after 1 test(s)
    13) literalFixturesBoundCompleteTest failed after 1 test(s)
    14) firstAcceptingCaseOrderTest failed after 1 test(s)
    15) laterChoiceCaseCanAcceptTest failed after 1 test(s)
    16) choiceBoundsMismatchOnlyAfterScanTest failed after 1 test(s)
    17) wrongChooserNoMatchingInputTest failed after 1 test(s)
    18) wrongChoiceIdNoMatchingInputTest failed after 1 test(s)
    19) matchedNonpositiveDepositTest failed after 1 test(s)
    20) mismatchedNonpositiveDepositTest failed after 1 test(s)
    21) matchingDepositAddsAndAdvancesTest failed after 1 test(s)
    22) depositAtPotentialBoundaryTest failed after 1 test(s)
    23) firstMatchingDepositOrderTest failed after 1 test(s)
    24) wrongDepositIdentityNoMatchingInputTest failed after 1 test(s)
    25) emptyOrWrongKindNoMatchingInputTest failed after 1 test(s)
    ok inputPreconditionDiagnosticTest passed 1 test(s)
    26) task3DomainDiagnosticTest failed after 1 test(s)
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

  26 passing (2970ms)
  26 failed

  1) allFrozenConstructorsSupportedTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task3-stages/red/candidate_a_core_test.qnt:63:44
        63:   run allFrozenConstructorsSupportedTest = assert(
                                                       ^^^^^^^
        64:     validState(canonicalSwap, emptyAState(N3)) and validState(canonicalSwap, emptyAState(N6))
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        65:       and canReduceOnce(canonicalSwap, emptyAState(N3))
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        66:       and canReduceOnce(canonicalSwap, emptyAState(N6)))
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=allFrozenConstructorsSupportedTest to repeat.
  2) ifAbsentEqualsZeroFalseTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task3-stages/red/candidate_a_core_test.qnt:93:5
        93:     assert(not(evaluateObservation(ChoiceEqualsA({id: SettleId, expected: 0}), before))
                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        94:       and reduceOnce(zeroBranchProgram, before) == ReducedA({
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        95:         state: {...before, continuation: N0}, payments: List(), warnings: List(), reductions: 1}))
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=ifAbsentEqualsZeroFalseTest to repeat.
  3) ifStoredZeroTrueTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task3-stages/red/candidate_a_core_test.qnt:99:5
        99:     assert(evaluateObservation(ChoiceEqualsA({id: SettleId, expected: 0}), before)
                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        100:       and reduceOnce(zeroBranchProgram, before) == ReducedA({
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        101:         state: {...before, continuation: N1}, payments: List(), warnings: List(), reductions: 1}))
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=ifStoredZeroTrueTest to repeat.
  4) ifStoredMismatchFalseTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task3-stages/red/candidate_a_core_test.qnt:105:5
        105:     assert(reduceOnce(zeroBranchProgram, before) == ReducedA({
                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        106:       state: {...before, continuation: N0}, payments: List(), warnings: List(), reductions: 1}))
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=ifStoredMismatchFalseTest to repeat.
  5) whenBeforeDeadlineQuiescentTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task3-stages/red/candidate_a_core_test.qnt:110:5
        110:     assert(reduceOnce(waitingProgram(List()), before) == QuiescentA(before))
                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=whenBeforeDeadlineQuiescentTest to repeat.
  6) timeoutAtDeadlineStepTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task3-stages/red/candidate_a_core_test.qnt:114:5
        114:     assert(reduceOnce(waitingProgram(List()), before) == ReducedA({
                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        115:       state: {...before, continuation: N0}, payments: List(), warnings: List(), reductions: 1}))
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=timeoutAtDeadlineStepTest to repeat.
  7) timeoutAfterDeadlineStepTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task3-stages/red/candidate_a_core_test.qnt:119:5
        119:     assert(reduceOnce(waitingProgram(List()), before) == ReducedA({
                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        120:       state: {...before, continuation: N0}, payments: List(), warnings: List(), reductions: 1}))
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=timeoutAfterDeadlineStepTest to repeat.
  8) quiescentWaitingZeroCountTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task3-stages/red/candidate_a_core_test.qnt:124:5
        124:     assert(reduceToQuiescence(waitingProgram(List()), before) == ReductionCompleteA({
                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        125:       state: before, payments: List(), warnings: List(), reductions: 0}))
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=quiescentWaitingZeroCountTest to repeat.
  9) quiescentEmptyCloseZeroCountTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task3-stages/red/candidate_a_core_test.qnt:127:42
        127:   run quiescentEmptyCloseZeroCountTest = assert(reduceToQuiescence(closeProgram, emptyAState(N0))
                                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        128:     == ReductionCompleteA({state: emptyAState(N0), payments: List(), warnings: List(), reductions: 0}))
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=quiescentEmptyCloseZeroCountTest to repeat.
  10) reductionBoundCompleteTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task3-stages/red/candidate_a_core_test.qnt:135:5
        135:     assert(validState(maximalReductionProgram, before)
                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        136:       and reduceToQuiescence(maximalReductionProgram, before) == ReductionCompleteA({
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        137:         state: emptyAState(N0), payments: expectedPayments, warnings: expectedWarnings, reductions: 21}))
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=reductionBoundCompleteTest to repeat.
  11) quiescencePreservesPaymentWarningOrderTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task3-stages/red/candidate_a_core_test.qnt:143:5
        143:     assert(reduceToQuiescence(program, before) == ReductionCompleteA({
                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        144:       state: {...before, continuation: N0, accounts: CORE_ACCOUNTS.mapBy(_ => 0)},
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        145:       payments: List({source: aliceA, recipient: Bob, asset: TokenA, quantity: 4},
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        146:         {source: bobB, recipient: Bob, asset: TokenB, quantity: 7}),
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        147:       warnings: List({code: "non_positive_payment", requested: IntValue(-1), paid: IntValue(0)},
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        148:         {code: "partial_payment", requested: IntValue(10), paid: IntValue(4)}), reductions: 3}))
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=quiescencePreservesPaymentWarningOrderTest to repeat.
  12) deadlineReductionRefundsBeforeInputTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task3-stages/red/candidate_a_core_test.qnt:153:5
        153:     assert(reduceToQuiescence(program, before) == ReductionCompleteA({
                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        154:       state: {...before, continuation: N0, accounts: CORE_ACCOUNTS.mapBy(_ => 0)},
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        155:       payments: List({source: aliceA, recipient: Alice, asset: TokenA, quantity: 10}),
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        156:       warnings: List(), reductions: 2})
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        157:       and applyInput(program, before, chosenInput(Bob, 1)) == InputPreconditionFailureA)
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=deadlineReductionRefundsBeforeInputTest to repeat.
  13) literalFixturesBoundCompleteTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task3-stages/red/candidate_a_core_test.qnt:159:42
        159:   run literalFixturesBoundCompleteTest = assert(
                                                      ^^^^^^^
        160:     Set(closeProgram, canonicalSwap, zeroBranchProgram, waitingProgram(List()), payProgram(-1),
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        161:       payProgram(0), payProgram(1), payProgram(5), payProgram(10), payProgram(20), maximalReductionProgram)
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        162:       .forall(program => A_TIMES.forall(time => match reduceToQuiescence(program,
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        163:         {...emptyAState(program.root), minimumTime: time}) {
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        164:           | ReductionCompleteA(_) => true | _ => false
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        165:         })))
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=literalFixturesBoundCompleteTest to repeat.
  14) firstAcceptingCaseOrderTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task3-stages/red/candidate_a_core_test.qnt:169:5
        169:     assert(canApplyInput(program, before, chosenInput(Bob, 1))
                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        170:       and applyInput(program, before, chosenInput(Bob, 1)) == InputAppliedA({
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        171:         state: {...before, continuation: N1, choices: before.choices.put(SettleId, IntValue(1))}, error: NoCoreError}))
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=firstAcceptingCaseOrderTest to repeat.
  15) laterChoiceCaseCanAcceptTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task3-stages/red/candidate_a_core_test.qnt:176:5
        176:     assert(applyInput(program, before, chosenInput(Bob, 1)) == InputAppliedA({
                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        177:       state: {...before, continuation: N2, choices: before.choices.put(SettleId, IntValue(1))}, error: NoCoreError}))
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=laterChoiceCaseCanAcceptTest to repeat.
  16) choiceBoundsMismatchOnlyAfterScanTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task3-stages/red/candidate_a_core_test.qnt:182:5
        182:     assert(applyInput(program, before, chosenInput(Bob, 2)) == InputAppliedA({
                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        183:       state: before, error: CoreErrorCode("choice_out_of_bounds")}))
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=choiceBoundsMismatchOnlyAfterScanTest to repeat.
  17) wrongChooserNoMatchingInputTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task3-stages/red/candidate_a_core_test.qnt:187:5
        187:     assert(applyInput(waitingProgram(List(choiceCase(0, 1, N1))), before, chosenInput(Mallory, 2))
                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        188:       == InputAppliedA({state: before, error: CoreErrorCode("no_matching_input")}))
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=wrongChooserNoMatchingInputTest to repeat.
  18) wrongChoiceIdNoMatchingInputTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task3-stages/red/candidate_a_core_test.qnt:192:5
        192:     assert(applyInput(waitingProgram(List(choiceCase(0, 1, N1))), before,
                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        193:       ChoiceInputA({id: OtherId, chooser: Bob, chosen: 2}))
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        194:       == InputAppliedA({state: before, error: CoreErrorCode("no_matching_input")}))
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=wrongChoiceIdNoMatchingInputTest to repeat.
  19) matchedNonpositiveDepositTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task3-stages/red/candidate_a_core_test.qnt:198:5
        198:     assert(Set(-1, 0).forall(quantity => applyInput(waitingProgram(List(depositCase(quantity, N1))),
                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        199:       before, depositInput(quantity)) == InputAppliedA({
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        200:         state: before, error: CoreErrorCode("non_positive_deposit")})))
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=matchedNonpositiveDepositTest to repeat.
  20) mismatchedNonpositiveDepositTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task3-stages/red/candidate_a_core_test.qnt:204:5
        204:     assert(applyInput(waitingProgram(List(depositCase(10, N1))), before, depositInput(0))
                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        205:       == InputAppliedA({state: before, error: CoreErrorCode("no_matching_input")}))
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=mismatchedNonpositiveDepositTest to repeat.
  21) matchingDepositAddsAndAdvancesTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task3-stages/red/candidate_a_core_test.qnt:209:5
        209:     assert(applyInput(waitingProgram(List(depositCase(10, N1))), before, depositInput(10))
                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        210:       == InputAppliedA({state: {...before, continuation: N1, accounts: before.accounts.put(aliceA, 14)},
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        211:         error: NoCoreError}))
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=matchingDepositAddsAndAdvancesTest to repeat.
  22) depositAtPotentialBoundaryTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task3-stages/red/candidate_a_core_test.qnt:217:5
        217:     assert(validState(program, before) and validState(program, expected)
                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        218:       and applyInput(program, before, depositInput(20)) == InputAppliedA({state: expected, error: NoCoreError}))
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=depositAtPotentialBoundaryTest to repeat.
  23) firstMatchingDepositOrderTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task3-stages/red/candidate_a_core_test.qnt:222:5
        222:     assert(applyInput(waitingProgram(List(depositCase(5, N1), depositCase(5, N2))), before, depositInput(5))
                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        223:       == InputAppliedA({state: {...before, continuation: N1, accounts: before.accounts.put(aliceA, 5)},
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        224:         error: NoCoreError}))
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=firstMatchingDepositOrderTest to repeat.
  24) wrongDepositIdentityNoMatchingInputTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task3-stages/red/candidate_a_core_test.qnt:229:5
        229:     assert(Set(DepositInputA({account: aliceA, depositor: Mallory, quantity: 10}),
                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        230:       DepositInputA({account: bobB, depositor: Alice, quantity: 10})).forall(supplied =>
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        231:         applyInput(program, before, supplied) == InputAppliedA({
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        232:           state: before, error: CoreErrorCode("no_matching_input")})))
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=wrongDepositIdentityNoMatchingInputTest to repeat.
  25) emptyOrWrongKindNoMatchingInputTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task3-stages/red/candidate_a_core_test.qnt:236:5
        236:     assert(Set(waitingProgram(List()), waitingProgram(List(depositCase(10, N1)))).forall(program =>
                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        237:       applyInput(program, before, chosenInput(Bob, 1)) == InputAppliedA({
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        238:         state: before, error: CoreErrorCode("no_matching_input")}))
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        239:       and applyInput(waitingProgram(List(choiceCase(0, 1, N1))), before, depositInput(10))
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        240:         == InputAppliedA({state: before, error: CoreErrorCode("no_matching_input")}))
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=emptyOrWrongKindNoMatchingInputTest to repeat.
  26) task3DomainDiagnosticTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task3-stages/red/candidate_a_core_test.qnt:255:5
        255:     assert(reduceToQuiescence(program, invalidState) == QuiescenceOutsideDomainA
                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        256:       and applyInput(program, invalidState, chosenInput(Bob, 1)) == InputOutsideDomainA
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        257:       and applyInput(program, emptyAState(N3), chosenInput(Bob, 3)) == InputOutsideDomainA
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        258:       and not(canApplyInput(program, emptyAState(N3), depositInput(22)))
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        259:       and applyInput(program, emptyAState(N3), depositInput(22)) == InputOutsideDomainA)
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=task3DomainDiagnosticTest to repeat.


  Use --verbosity=3 to show executions.
  Further debug with: quint test --verbosity=3 .superpowers/sdd/candidate-a-task3-stages/red/candidate_a_core_test.qnt
error: Tests failed
exit_code: 1
```

The 26 failures are assertion failures against disabled If/When, observation, quiescence, and input implementations. The 25 retained tests and explicit input-precondition diagnostic pass against the stubs. This is behavioral RED, not a missing import or compile failure.

## Implementation compile correction and first GREEN

The initial implemented source used two reserved local names. This parsing failure happened after the preserved RED and is not a second semantic RED. Exact output:

```text
$ quint test specs/quint/s02/candidate_a_core_test.qnt --backend=rust --seed=42

 Error [QNT101]: Built-in name 'next' is redefined in module 'candidate_a_core'

  at /home/charl/Moriarty/.worktrees/s01-audit-start/specs/quint/s02/candidate_a_core.qnt:71:15
  71:             | ReducedA(next) => {...acc, result: {state: next.state,
                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  72:                 payments: next.payments.foldl(acc.result.payments, (items, item) => items.append(item)),
      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  73:                 warnings: next.warnings.foldl(acc.result.warnings, (items, item) => items.append(item)),
      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  74:                 reductions: acc.result.reductions + next.reductions}}
      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


 Error [QNT101]: Built-in name 'item' is redefined in module 'candidate_a_core'

  at /home/charl/Moriarty/.worktrees/s01-audit-start/specs/quint/s02/candidate_a_core.qnt:72:76
  72:                 payments: next.payments.foldl(acc.result.payments, (items, item) => items.append(item)),
                                                                                 ^^^^


 Error [QNT101]: Built-in name 'item' is redefined in module 'candidate_a_core'

  at /home/charl/Moriarty/.worktrees/s01-audit-start/specs/quint/s02/candidate_a_core.qnt:73:76
  73:                 warnings: next.warnings.foldl(acc.result.warnings, (items, item) => items.append(item)),
                                                                                 ^^^^

error: parsing failed
exit_code: 1
```

Renamed local bindings to delta/payment/warning, without changing behavior. Then:

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
    ok allFrozenConstructorsSupportedTest passed 1 test(s)
    ok invalidDomainDistinctTest passed 1 test(s)
    ok ifAbsentEqualsZeroFalseTest passed 1 test(s)
    ok ifStoredZeroTrueTest passed 1 test(s)
    ok ifStoredMismatchFalseTest passed 1 test(s)
    ok whenBeforeDeadlineQuiescentTest passed 1 test(s)
    ok timeoutAtDeadlineStepTest passed 1 test(s)
    ok timeoutAfterDeadlineStepTest passed 1 test(s)
    ok quiescentWaitingZeroCountTest passed 1 test(s)
    ok quiescentEmptyCloseZeroCountTest passed 1 test(s)
    ok reductionBoundCompleteTest passed 1 test(s)
    ok quiescencePreservesPaymentWarningOrderTest passed 1 test(s)
    ok deadlineReductionRefundsBeforeInputTest passed 1 test(s)
    ok literalFixturesBoundCompleteTest passed 1 test(s)
    ok firstAcceptingCaseOrderTest passed 1 test(s)
    ok laterChoiceCaseCanAcceptTest passed 1 test(s)
    ok choiceBoundsMismatchOnlyAfterScanTest passed 1 test(s)
    ok wrongChooserNoMatchingInputTest passed 1 test(s)
    ok wrongChoiceIdNoMatchingInputTest passed 1 test(s)
    ok matchedNonpositiveDepositTest passed 1 test(s)
    ok mismatchedNonpositiveDepositTest passed 1 test(s)
    ok matchingDepositAddsAndAdvancesTest passed 1 test(s)
    ok depositAtPotentialBoundaryTest passed 1 test(s)
    ok firstMatchingDepositOrderTest passed 1 test(s)
    ok wrongDepositIdentityNoMatchingInputTest passed 1 test(s)
    ok emptyOrWrongKindNoMatchingInputTest passed 1 test(s)
    ok inputPreconditionDiagnosticTest passed 1 test(s)
    ok task3DomainDiagnosticTest passed 1 test(s)
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

  52 passing (3909ms)
exit_code: 0
```

## Final frozen-source author verification

No source changes followed the first GREEN. Separate standalone typecheck and scoped test:

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
    ok allFrozenConstructorsSupportedTest passed 1 test(s)
    ok invalidDomainDistinctTest passed 1 test(s)
    ok ifAbsentEqualsZeroFalseTest passed 1 test(s)
    ok ifStoredZeroTrueTest passed 1 test(s)
    ok ifStoredMismatchFalseTest passed 1 test(s)
    ok whenBeforeDeadlineQuiescentTest passed 1 test(s)
    ok timeoutAtDeadlineStepTest passed 1 test(s)
    ok timeoutAfterDeadlineStepTest passed 1 test(s)
    ok quiescentWaitingZeroCountTest passed 1 test(s)
    ok quiescentEmptyCloseZeroCountTest passed 1 test(s)
    ok reductionBoundCompleteTest passed 1 test(s)
    ok quiescencePreservesPaymentWarningOrderTest passed 1 test(s)
    ok deadlineReductionRefundsBeforeInputTest passed 1 test(s)
    ok literalFixturesBoundCompleteTest passed 1 test(s)
    ok firstAcceptingCaseOrderTest passed 1 test(s)
    ok laterChoiceCaseCanAcceptTest passed 1 test(s)
    ok choiceBoundsMismatchOnlyAfterScanTest passed 1 test(s)
    ok wrongChooserNoMatchingInputTest passed 1 test(s)
    ok wrongChoiceIdNoMatchingInputTest passed 1 test(s)
    ok matchedNonpositiveDepositTest passed 1 test(s)
    ok mismatchedNonpositiveDepositTest passed 1 test(s)
    ok matchingDepositAddsAndAdvancesTest passed 1 test(s)
    ok depositAtPotentialBoundaryTest passed 1 test(s)
    ok firstMatchingDepositOrderTest passed 1 test(s)
    ok wrongDepositIdentityNoMatchingInputTest passed 1 test(s)
    ok emptyOrWrongKindNoMatchingInputTest passed 1 test(s)
    ok inputPreconditionDiagnosticTest passed 1 test(s)
    ok task3DomainDiagnosticTest passed 1 test(s)
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

  52 passing (3932ms)
exit_code: 0
```

No full Python/Quint suite, simulator, or model checker was run by this worker. Independent root checks/review are separate evidence.

## Exact source hashes

This exact command ran before and after the final typecheck/test. Its terminal outputs were byte-identical, including the RED closure, unchanged program table/imports, frozen Python, plan, and Quint entry point:

```text
$ sha256sum specs/quint/s02/candidate_a_core.qnt specs/quint/s02/candidate_a_core_test.qnt specs/quint/s02/candidate_a_types.qnt specs/quint/s02/candidate_a_programs.qnt specs/quint/s02/effects.qnt specs/quint/s02/observations.qnt .superpowers/sdd/candidate-a-task3-stages/red/*.qnt moriarty/core.py moriarty/swap.py docs/superpowers/plans/2026-09-05-moriarty-s02-candidate-a-core.md /home/charl/.npm-global/bin/quint
50f6dd731493510c606c82ce083c55d4d773985a02e43ecf1c22ec393a37d3c1  specs/quint/s02/candidate_a_core.qnt
42df4e4c11665f185efaf302b33418113302d9b2dead4574116d9e81af847767  specs/quint/s02/candidate_a_core_test.qnt
c233d6775c65e5eec10eec0768bcfc5d8b64bb7f6601b8e652275456d45ac288  specs/quint/s02/candidate_a_types.qnt
b443fc0beec22b867e4d36c21714fbaee63950961776d21b2ebe9463e190ab66  specs/quint/s02/candidate_a_programs.qnt
dccc9208d9a42f01685000b1358f1d2f9684bf5e8ebe1d5deadd0c992557504c  specs/quint/s02/effects.qnt
e44484ed9035e560ef9f4d012198671917750da42ab404022592db16cdd59bc3  specs/quint/s02/observations.qnt
948063f04cbf853314a984ef15d76563df72f7e0483a89252d57cbd6b8e2a267  .superpowers/sdd/candidate-a-task3-stages/red/candidate_a_core.qnt
42df4e4c11665f185efaf302b33418113302d9b2dead4574116d9e81af847767  .superpowers/sdd/candidate-a-task3-stages/red/candidate_a_core_test.qnt
b443fc0beec22b867e4d36c21714fbaee63950961776d21b2ebe9463e190ab66  .superpowers/sdd/candidate-a-task3-stages/red/candidate_a_programs.qnt
93ccf78860d69b51fbcbcdee18952d05d33810e4dc9ed34c05e06770b6e1bd17  .superpowers/sdd/candidate-a-task3-stages/red/candidate_a_types.qnt
dccc9208d9a42f01685000b1358f1d2f9684bf5e8ebe1d5deadd0c992557504c  .superpowers/sdd/candidate-a-task3-stages/red/effects.qnt
e44484ed9035e560ef9f4d012198671917750da42ab404022592db16cdd59bc3  .superpowers/sdd/candidate-a-task3-stages/red/observations.qnt
564a926779beb54bd16ff58481f5c950bae6fb50fcd5b0d7c9eb56427f83273b  moriarty/core.py
82e9e1da76af65706171049f0238953aca5333edec4aeed6caa43dbd57c79797  moriarty/swap.py
cc8c66a1a4b94a788376dd9ba1649d6d485d00cb0c254020998a77cba3bf5860  docs/superpowers/plans/2026-09-05-moriarty-s02-candidate-a-core.md
ac12595b1cb7253feec93c79417615c6eb20fc3b6a3df35c5e3530b24e90a501  /home/charl/.npm-global/bin/quint
exit_code: 0
before_output_equals_after_output: true
```

## Handoff and open work

The three modified live Quint files and the RED closure are frozen for root's independent checks and source/evidence review. No commit was made by this worker. Full transaction source order and rollback are Task 4 and require separate authorization. Projection, stateful workloads, independent Python extraction/correspondence, and all acceptance/integration gates remain open.
