# Candidate A Task 4 implementation receipt

Scope: author-side implementation of computeTransaction and complete Core-error rollback. This is not independent review, correspondence, a model-checking result, Candidate A completion, integration, or Council acceptance. No Task 5 projection or stateful harness was added.

Starting source HEAD: `d3f5dd6e60ca937dfc4840d34204bf00436bdc48`.
Root archived prior work concurrently; observed HEAD before final verification was `a0fb15f9a3d92c67cdac214bbd87e57ce788adfe`. This worker did not commit or alter Git state. The exact source hashes below bind this unit independently of root's concurrent commits.

Working directory for every command:
`/home/charl/Moriarty/.worktrees/s01-audit-start`.
CLI: `/home/charl/.npm-global/bin/quint`, version `0.32.0`.

Changes are limited to candidate_a_core.qnt, candidate_a_core_test.qnt, necessary diagnostic carriers in candidate_a_types.qnt, this report, and the new Task 4 RED closure. Shared foundations, frozen Python, program fixtures, archived reports, and earlier snapshots were not edited.

## Exact transaction boundary

The implementation follows frozen `moriarty/core.py:323-370`:

1. Reject invalid finite-domain calls with OutsideModelDomainA, outside the Python vocabulary.
2. Compare now with ORIGINAL minimumTime. A backward time returns the complete original state with time_before_state.
3. Copy minimumTime=now and perform pre-input quiescence.
4. With NoAInput, unchanged/quiescent Close returns contract_closed; quiescent When returns input_required even after speculative reductions; an actual reduction ending elsewhere commits its exact result.
5. With supplied input, pre-reduced Close returns contract_closed. The frozen non-When fallback is no_matching_input. Only the reduced unexpired When calls applyInput.
6. Any real input-stage Core error rolls back to ORIGINAL state, not the pre-reduced working state.
7. Successful input performs post-input quiescence. The transaction concatenates pre/post payments and warnings in order and adds only actual reduction counts.

Every Core rejection returns accepted=false, ORIGINAL continuation/accounts/choices/minimumTime, the exact error, empty ordered payments/warnings, and reductions=0. Success uses NoCoreError and the actual final state. The test-only expectedRollback helper is independent of the evaluator's result constructor.

Root approved `TransactionReductionBoundFailureA` and `TransactionInputPreconditionFailureA` before implementation. They remain distinct from OutsideModelDomainA and every frozen Core error. The evaluator propagates internal diagnostics without creating a fake rejected or successful Core result. A 200-case admitted literal fixture matrix requires TransactionComputedA in every case, including genuine rejected outcomes; it does not accept diagnostics as rejections.

These two diagnostics are expected to be unreachable for admitted programs under the established rank/domain constraints. This bounded test unit is not an exhaustive proof of that property.

## Test coverage

The unit contains 75 deterministic pure tests: 52 retained Task 1–3 tests plus 23 transaction tests. The new tests cover:

- All six actual Core errors: time_before_state, contract_closed, input_required, no_matching_input, choice_out_of_bounds, non_positive_deposit.
- Backward time and finite-domain admission precedence.
- Exact original-state rollback after speculative warning, payment, minimum-time update, continuation change, and no-input pre-reduction.
- Funded timeout with NoAInput commits the refund; supplied input at/after deadline rolls it back with contract_closed.
- Pre-Pay1, Deposit5, post-Pay5 gives ordered payments [Bob1, Bob5], zero final accounts, and reductions=2.
- Pre/post warning order, deposit with no Core payment, deposit followed by Close refund, input accepted into empty Close with reductions=0, and choice followed by actual If/Pay/refund reduction.
- Exact matching nonpositive deposit versus quantity mismatch.
- Ten admitted literal program values, five modeled times, and four input forms: 200 computations must produce real Core results.

The payment-order test does not claim an extracted effect trace. The future projection/extraction stage must place the deposit between the pre/post payment effects. No extraction was implemented here.

Quint modeling/language, executing-plans, TDD, and verification-before-completion guided typed scaffold, preserved behavioral RED, and fresh source-bound verification. No provider dispatch, Foreman/Council work, or external session database write was performed.

## Typed scaffold and preserved behavioral RED

First, the transaction wrapper and disabled typed function compiled with the retained tests:

```text
$ quint typecheck specs/quint/s02/candidate_a_core_test.qnt
[no terminal output]
exit_code: 0
```

The complete 75-test unit then typechecked against the disabled transaction scaffold:

```text
$ quint typecheck specs/quint/s02/candidate_a_core_test.qnt
[no terminal output]
exit_code: 0
```

Before adding transaction logic, apply_patch preserved the exact six-file import closure in `.superpowers/sdd/candidate-a-task4-stages/red/`. The semantic RED command ran against that snapshot directly. Neither the test source nor the wrapper types changed after this RED.

```text
$ sha256sum .superpowers/sdd/candidate-a-task4-stages/red/*.qnt
b49c596d1557f6ac2a4732417c569f9e6b2e2112a46c24883c98ccfc354b8b89  .superpowers/sdd/candidate-a-task4-stages/red/candidate_a_core.qnt
91482debc44658a6a55eb6f74b63253f715a9372ed19228e43b16b1261c12375  .superpowers/sdd/candidate-a-task4-stages/red/candidate_a_core_test.qnt
b443fc0beec22b867e4d36c21714fbaee63950961776d21b2ebe9463e190ab66  .superpowers/sdd/candidate-a-task4-stages/red/candidate_a_programs.qnt
40901738fd1749527761b624a84253da94bbc24c209d408dc2fd4e895daaae0b  .superpowers/sdd/candidate-a-task4-stages/red/candidate_a_types.qnt
dccc9208d9a42f01685000b1358f1d2f9684bf5e8ebe1d5deadd0c992557504c  .superpowers/sdd/candidate-a-task4-stages/red/effects.qnt
e44484ed9035e560ef9f4d012198671917750da42ab404022592db16cdd59bc3  .superpowers/sdd/candidate-a-task4-stages/red/observations.qnt
exit_code: 0
```

```text
$ quint test .superpowers/sdd/candidate-a-task4-stages/red/candidate_a_core_test.qnt --backend=rust --seed=42

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
    1) timeBeforeStateRollbackTest failed after 1 test(s)
    2) emptyCloseRollbackTest failed after 1 test(s)
    3) waitingWithoutInputRollbackTest failed after 1 test(s)
    4) wrongInputRollbackTest failed after 1 test(s)
    5) outOfBoundsRollbackTest failed after 1 test(s)
    6) nonpositiveDepositRollbackTest failed after 1 test(s)
    7) timeoutNoInputCommitsRefundTest failed after 1 test(s)
    8) timeoutWithInputRollsBackRefundTest failed after 1 test(s)
    9) speculativeWarningRollbackTest failed after 1 test(s)
    10) speculativePaymentRollbackTest failed after 1 test(s)
    11) speculativeNoInputRollbackTest failed after 1 test(s)
    12) beforeAfterReductionOrderTest failed after 1 test(s)
    13) beforeAfterWarningOrderTest failed after 1 test(s)
    14) depositWithoutPaymentTransactionTest failed after 1 test(s)
    15) depositThenCloseRefundTransactionTest failed after 1 test(s)
    16) choiceToEmptyCloseAcceptedTest failed after 1 test(s)
    17) choiceThenIfPaymentTransactionTest failed after 1 test(s)
    18) noInputActualReductionAcceptedTest failed after 1 test(s)
    19) noInputWarningAcceptedTest failed after 1 test(s)
    20) suppliedAfterPreCloseRollbackTest failed after 1 test(s)
    21) matchedZeroVersusMismatchedZeroTransactionTest failed after 1 test(s)
    ok transactionDomainDiagnosticTest passed 1 test(s)
    22) admittedTransactionFixturesNeverDiagnosticTest failed after 1 test(s)
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

  53 passing (6447ms)
  22 failed

  1) timeBeforeStateRollbackTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task4-stages/red/candidate_a_core_test.qnt:278:5
        278:     assert(computeTransaction(payProgram(0), before, NoAInput, Time1)
                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        279:       == expectedRollback(before, "time_before_state"))
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=timeBeforeStateRollbackTest to repeat.
  2) emptyCloseRollbackTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task4-stages/red/candidate_a_core_test.qnt:283:5
        283:     assert(computeTransaction(closeProgram, before, NoAInput, Time2)
                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        284:       == expectedRollback(before, "contract_closed"))
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=emptyCloseRollbackTest to repeat.
  3) waitingWithoutInputRollbackTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task4-stages/red/candidate_a_core_test.qnt:288:5
        288:     assert(computeTransaction(waitingProgram(List(choiceCase(0, 1, N1))), before, NoAInput, Time2)
                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        289:       == expectedRollback(before, "input_required"))
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=waitingWithoutInputRollbackTest to repeat.
  4) wrongInputRollbackTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task4-stages/red/candidate_a_core_test.qnt:293:5
        293:     assert(computeTransaction(waitingProgram(List(choiceCase(0, 1, N1))), before,
                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        294:       PresentAInput(chosenInput(Mallory, 1)), Time2) == expectedRollback(before, "no_matching_input"))
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=wrongInputRollbackTest to repeat.
  5) outOfBoundsRollbackTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task4-stages/red/candidate_a_core_test.qnt:298:5
        298:     assert(computeTransaction(waitingProgram(List(choiceCase(0, 1, N1))), before,
                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        299:       PresentAInput(chosenInput(Bob, 2)), Time2) == expectedRollback(before, "choice_out_of_bounds"))
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=outOfBoundsRollbackTest to repeat.
  6) nonpositiveDepositRollbackTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task4-stages/red/candidate_a_core_test.qnt:303:5
        303:     assert(Set(-1, 0).forall(quantity =>
                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        304:       computeTransaction(waitingProgram(List(depositCase(quantity, N1))), before,
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        305:         PresentAInput(depositInput(quantity)), Time2) == expectedRollback(before, "non_positive_deposit")))
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=nonpositiveDepositRollbackTest to repeat.
  7) timeoutNoInputCommitsRefundTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task4-stages/red/candidate_a_core_test.qnt:309:5
        309:     assert(computeTransaction(waitingProgram(List(choiceCase(0, 1, N1))), before, NoAInput, Time100)
                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        310:       == TransactionComputedA({accepted: true, state: {...before, continuation: N0,
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        311:         minimumTime: Time100, accounts: CORE_ACCOUNTS.mapBy(_ => 0)}, error: NoCoreError,
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        312:         payments: List({source: aliceA, recipient: Alice, asset: TokenA, quantity: 10}),
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        313:         warnings: List(), reductions: 2}))
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=timeoutNoInputCommitsRefundTest to repeat.
  8) timeoutWithInputRollsBackRefundTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task4-stages/red/candidate_a_core_test.qnt:317:5
        317:     assert(Set(Time100, Time101).forall(time =>
                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        318:       computeTransaction(waitingProgram(List(choiceCase(0, 1, N1))), before,
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        319:         PresentAInput(chosenInput(Bob, 1)), time) == expectedRollback(before, "contract_closed")))
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=timeoutWithInputRollsBackRefundTest to repeat.
  9) speculativeWarningRollbackTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task4-stages/red/candidate_a_core_test.qnt:326:5
        326:     assert(computeTransaction(program, before, PresentAInput(chosenInput(Mallory, 1)), Time2)
                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        327:       == expectedRollback(before, "no_matching_input"))
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=speculativeWarningRollbackTest to repeat.
  10) speculativePaymentRollbackTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task4-stages/red/candidate_a_core_test.qnt:331:5
        331:     assert(computeTransaction(payDepositPayProgram, before, PresentAInput(depositInput(10)), Time2)
                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        332:       == expectedRollback(before, "no_matching_input"))
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=speculativePaymentRollbackTest to repeat.
  11) speculativeNoInputRollbackTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task4-stages/red/candidate_a_core_test.qnt:336:5
        336:     assert(computeTransaction(warningDepositWarningProgram, before, NoAInput, Time2)
                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        337:       == expectedRollback(before, "input_required"))
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=speculativeNoInputRollbackTest to repeat.
  12) beforeAfterReductionOrderTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task4-stages/red/candidate_a_core_test.qnt:341:5
        341:     assert(computeTransaction(payDepositPayProgram, before, PresentAInput(depositInput(5)), Time2)
                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        342:       == TransactionComputedA({accepted: true, state: {...before, continuation: N0,
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        343:         minimumTime: Time2, accounts: CORE_ACCOUNTS.mapBy(_ => 0)}, error: NoCoreError,
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        344:         payments: List({source: aliceA, recipient: Bob, asset: TokenA, quantity: 1},
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        345:           {source: aliceA, recipient: Bob, asset: TokenA, quantity: 5}), warnings: List(), reductions: 2}))
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=beforeAfterReductionOrderTest to repeat.
  13) beforeAfterWarningOrderTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task4-stages/red/candidate_a_core_test.qnt:349:5
        349:     assert(computeTransaction(warningDepositWarningProgram, before, PresentAInput(depositInput(5)), Time2)
                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        350:       == TransactionComputedA({accepted: true, state: {...before, continuation: N0,
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        351:         minimumTime: Time2, accounts: CORE_ACCOUNTS.mapBy(_ => 0)}, error: NoCoreError,
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        352:         payments: List({source: aliceA, recipient: Bob, asset: TokenA, quantity: 5}),
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        353:         warnings: List({code: "non_positive_payment", requested: IntValue(0), paid: IntValue(0)},
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        354:           {code: "partial_payment", requested: IntValue(10), paid: IntValue(5)}), reductions: 2}))
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=beforeAfterWarningOrderTest to repeat.
  14) depositWithoutPaymentTransactionTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task4-stages/red/candidate_a_core_test.qnt:358:5
        358:     assert(computeTransaction(canonicalSwap, before, PresentAInput(depositInput(10)), Time1)
                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        359:       == TransactionComputedA({accepted: true, state: {...before, continuation: N5,
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        360:         minimumTime: Time1, accounts: before.accounts.put(aliceA, 10)}, error: NoCoreError,
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        361:         payments: List(), warnings: List(), reductions: 0}))
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=depositWithoutPaymentTransactionTest to repeat.
  15) depositThenCloseRefundTransactionTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task4-stages/red/candidate_a_core_test.qnt:365:5
        365:     assert(computeTransaction(waitingProgram(List(depositCase(5, N0))), before,
                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        366:       PresentAInput(depositInput(5)), Time2) == TransactionComputedA({accepted: true,
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        367:         state: {...before, continuation: N0, minimumTime: Time2}, error: NoCoreError,
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        368:         payments: List({source: aliceA, recipient: Alice, asset: TokenA, quantity: 5}),
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        369:         warnings: List(), reductions: 1}))
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=depositThenCloseRefundTransactionTest to repeat.
  16) choiceToEmptyCloseAcceptedTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task4-stages/red/candidate_a_core_test.qnt:373:5
        373:     assert(computeTransaction(waitingProgram(List(choiceCase(0, 1, N0))), before,
                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        374:       PresentAInput(chosenInput(Bob, 1)), Time2) == TransactionComputedA({accepted: true,
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        375:         state: {...before, continuation: N0, minimumTime: Time2,
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        376:           choices: before.choices.put(SettleId, IntValue(1))}, error: NoCoreError,
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        377:         payments: List(), warnings: List(), reductions: 0}))
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=choiceToEmptyCloseAcceptedTest to repeat.
  17) choiceThenIfPaymentTransactionTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task4-stages/red/candidate_a_core_test.qnt:385:5
        385:     assert(computeTransaction(program, before, PresentAInput(chosenInput(Bob, 1)), Time2)
                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        386:       == TransactionComputedA({accepted: true, state: {...before, continuation: N0, minimumTime: Time2,
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        387:         choices: before.choices.put(SettleId, IntValue(1)), accounts: CORE_ACCOUNTS.mapBy(_ => 0)},
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        388:         error: NoCoreError, payments: List({source: aliceA, recipient: Bob, asset: TokenA, quantity: 5},
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        389:           {source: aliceA, recipient: Alice, asset: TokenA, quantity: 5}), warnings: List(), reductions: 3}))
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=choiceThenIfPaymentTransactionTest to repeat.
  18) noInputActualReductionAcceptedTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task4-stages/red/candidate_a_core_test.qnt:393:5
        393:     assert(computeTransaction(zeroBranchProgram, before, NoAInput, Time2)
                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        394:       == TransactionComputedA({accepted: true, state: {...before, continuation: N1, minimumTime: Time2},
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        395:         error: NoCoreError, payments: List(), warnings: List(), reductions: 1}))
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=noInputActualReductionAcceptedTest to repeat.
  19) noInputWarningAcceptedTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task4-stages/red/candidate_a_core_test.qnt:399:5
        399:     assert(computeTransaction(payProgram(0), before, NoAInput, Time2)
                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        400:       == TransactionComputedA({accepted: true, state: {...before, continuation: N0, minimumTime: Time2},
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        401:         error: NoCoreError, payments: List(),
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        402:         warnings: List({code: "non_positive_payment", requested: IntValue(0), paid: IntValue(0)}), reductions: 1}))
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=noInputWarningAcceptedTest to repeat.
  20) suppliedAfterPreCloseRollbackTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task4-stages/red/candidate_a_core_test.qnt:406:5
        406:     assert(computeTransaction(payProgram(10), before, PresentAInput(depositInput(5)), Time2)
                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        407:       == expectedRollback(before, "contract_closed"))
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=suppliedAfterPreCloseRollbackTest to repeat.
  21) matchedZeroVersusMismatchedZeroTransactionTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task4-stages/red/candidate_a_core_test.qnt:411:5
        411:     assert(computeTransaction(waitingProgram(List(depositCase(10, N0))), before,
                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        412:       PresentAInput(depositInput(0)), Time2) == expectedRollback(before, "no_matching_input"))
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=matchedZeroVersusMismatchedZeroTransactionTest to repeat.
  22) admittedTransactionFixturesNeverDiagnosticTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task4-stages/red/candidate_a_core_test.qnt:422:56
        422:   run admittedTransactionFixturesNeverDiagnosticTest = assert(
                                                                    ^^^^^^^
        423:     Set(closeProgram, canonicalSwap, zeroBranchProgram, waitingProgram(List()),
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        424:       waitingProgram(List(choiceCase(0, 1, N1))), waitingProgram(List(depositCase(0, N0))),
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        425:       payProgram(0), payDepositPayProgram, warningDepositWarningProgram, maximalReductionProgram)
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        426:       .forall(program => A_TIMES.forall(time =>
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        427:         Set(NoAInput, PresentAInput(chosenInput(Bob, 1)), PresentAInput(depositInput(5)), PresentAInput(depositInput(0)))
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        428:           .forall(supplied => match computeTransaction(program, initialTransactionState(program.root, 1), supplied, time) {
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        429:             | TransactionComputedA(_) => true | _ => false
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        430:           }))))
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=admittedTransactionFixturesNeverDiagnosticTest to repeat.


  Use --verbosity=3 to show executions.
  Further debug with: quint test --verbosity=3 .superpowers/sdd/candidate-a-task4-stages/red/candidate_a_core_test.qnt
error: Tests failed
exit_code: 1
```

The 22 failures are 21 exact transaction-result assertions plus the admitted-workload no-diagnostic matrix. The 52 retained tests and one invalid-domain diagnostic test pass against the stub. This is behavioral RED, not an import/type/parse failure.

## First implementation GREEN

After implementation, the unchanged test source passed:

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
    ok timeBeforeStateRollbackTest passed 1 test(s)
    ok emptyCloseRollbackTest passed 1 test(s)
    ok waitingWithoutInputRollbackTest passed 1 test(s)
    ok wrongInputRollbackTest passed 1 test(s)
    ok outOfBoundsRollbackTest passed 1 test(s)
    ok nonpositiveDepositRollbackTest passed 1 test(s)
    ok timeoutNoInputCommitsRefundTest passed 1 test(s)
    ok timeoutWithInputRollsBackRefundTest passed 1 test(s)
    ok speculativeWarningRollbackTest passed 1 test(s)
    ok speculativePaymentRollbackTest passed 1 test(s)
    ok speculativeNoInputRollbackTest passed 1 test(s)
    ok beforeAfterReductionOrderTest passed 1 test(s)
    ok beforeAfterWarningOrderTest passed 1 test(s)
    ok depositWithoutPaymentTransactionTest passed 1 test(s)
    ok depositThenCloseRefundTransactionTest passed 1 test(s)
    ok choiceToEmptyCloseAcceptedTest passed 1 test(s)
    ok choiceThenIfPaymentTransactionTest passed 1 test(s)
    ok noInputActualReductionAcceptedTest passed 1 test(s)
    ok noInputWarningAcceptedTest passed 1 test(s)
    ok suppliedAfterPreCloseRollbackTest passed 1 test(s)
    ok matchedZeroVersusMismatchedZeroTransactionTest passed 1 test(s)
    ok transactionDomainDiagnosticTest passed 1 test(s)
    ok admittedTransactionFixturesNeverDiagnosticTest passed 1 test(s)
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

  75 passing (7546ms)
exit_code: 0
```

## Final frozen-source author verification

No source changes followed first GREEN. Separate standalone typecheck and scoped rerun:

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
    ok timeBeforeStateRollbackTest passed 1 test(s)
    ok emptyCloseRollbackTest passed 1 test(s)
    ok waitingWithoutInputRollbackTest passed 1 test(s)
    ok wrongInputRollbackTest passed 1 test(s)
    ok outOfBoundsRollbackTest passed 1 test(s)
    ok nonpositiveDepositRollbackTest passed 1 test(s)
    ok timeoutNoInputCommitsRefundTest passed 1 test(s)
    ok timeoutWithInputRollsBackRefundTest passed 1 test(s)
    ok speculativeWarningRollbackTest passed 1 test(s)
    ok speculativePaymentRollbackTest passed 1 test(s)
    ok speculativeNoInputRollbackTest passed 1 test(s)
    ok beforeAfterReductionOrderTest passed 1 test(s)
    ok beforeAfterWarningOrderTest passed 1 test(s)
    ok depositWithoutPaymentTransactionTest passed 1 test(s)
    ok depositThenCloseRefundTransactionTest passed 1 test(s)
    ok choiceToEmptyCloseAcceptedTest passed 1 test(s)
    ok choiceThenIfPaymentTransactionTest passed 1 test(s)
    ok noInputActualReductionAcceptedTest passed 1 test(s)
    ok noInputWarningAcceptedTest passed 1 test(s)
    ok suppliedAfterPreCloseRollbackTest passed 1 test(s)
    ok matchedZeroVersusMismatchedZeroTransactionTest passed 1 test(s)
    ok transactionDomainDiagnosticTest passed 1 test(s)
    ok admittedTransactionFixturesNeverDiagnosticTest passed 1 test(s)
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

  75 passing (7544ms)
exit_code: 0
```

No full suite, simulator, model checker, or Python correspondence command was run by this worker. Root's independent reruns/reviews are separate evidence.

## Exact source hashes

The following exact command ran before and after final typecheck/test. Both full terminal outputs were byte-identical:

```text
$ sha256sum specs/quint/s02/candidate_a_core.qnt specs/quint/s02/candidate_a_core_test.qnt specs/quint/s02/candidate_a_types.qnt specs/quint/s02/candidate_a_programs.qnt specs/quint/s02/effects.qnt specs/quint/s02/observations.qnt .superpowers/sdd/candidate-a-task4-stages/red/*.qnt moriarty/core.py moriarty/swap.py docs/superpowers/plans/2026-09-05-moriarty-s02-candidate-a-core.md /home/charl/.npm-global/bin/quint
e759d4c13032d83a4d839e08bef0fcf8272943792f73bb3ab7910357b0f28dec  specs/quint/s02/candidate_a_core.qnt
91482debc44658a6a55eb6f74b63253f715a9372ed19228e43b16b1261c12375  specs/quint/s02/candidate_a_core_test.qnt
40901738fd1749527761b624a84253da94bbc24c209d408dc2fd4e895daaae0b  specs/quint/s02/candidate_a_types.qnt
b443fc0beec22b867e4d36c21714fbaee63950961776d21b2ebe9463e190ab66  specs/quint/s02/candidate_a_programs.qnt
dccc9208d9a42f01685000b1358f1d2f9684bf5e8ebe1d5deadd0c992557504c  specs/quint/s02/effects.qnt
e44484ed9035e560ef9f4d012198671917750da42ab404022592db16cdd59bc3  specs/quint/s02/observations.qnt
b49c596d1557f6ac2a4732417c569f9e6b2e2112a46c24883c98ccfc354b8b89  .superpowers/sdd/candidate-a-task4-stages/red/candidate_a_core.qnt
91482debc44658a6a55eb6f74b63253f715a9372ed19228e43b16b1261c12375  .superpowers/sdd/candidate-a-task4-stages/red/candidate_a_core_test.qnt
b443fc0beec22b867e4d36c21714fbaee63950961776d21b2ebe9463e190ab66  .superpowers/sdd/candidate-a-task4-stages/red/candidate_a_programs.qnt
40901738fd1749527761b624a84253da94bbc24c209d408dc2fd4e895daaae0b  .superpowers/sdd/candidate-a-task4-stages/red/candidate_a_types.qnt
dccc9208d9a42f01685000b1358f1d2f9684bf5e8ebe1d5deadd0c992557504c  .superpowers/sdd/candidate-a-task4-stages/red/effects.qnt
e44484ed9035e560ef9f4d012198671917750da42ab404022592db16cdd59bc3  .superpowers/sdd/candidate-a-task4-stages/red/observations.qnt
564a926779beb54bd16ff58481f5c950bae6fb50fcd5b0d7c9eb56427f83273b  moriarty/core.py
82e9e1da76af65706171049f0238953aca5333edec4aeed6caa43dbd57c79797  moriarty/swap.py
cc8c66a1a4b94a788376dd9ba1649d6d485d00cb0c254020998a77cba3bf5860  docs/superpowers/plans/2026-09-05-moriarty-s02-candidate-a-core.md
ac12595b1cb7253feec93c79417615c6eb20fc3b6a3df35c5e3530b24e90a501  /home/charl/.npm-global/bin/quint
exit_code: 0
before_output_equals_after_output: true
```

## Handoff

Three modified live Quint files and the Task 4 RED closure are frozen for root's independent checks and source/evidence review. No commit was made. Task 5 projection, stateful canonical swap, ordered effect extraction, installment workloads, independent Python correspondence, and all acceptance/integration gates remain open.
