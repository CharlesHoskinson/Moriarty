# Candidate A Task 6 author implementation report

Scope: literal two-When installment agreement fixture and a separate pure cancellation adapter.
This is a local implementation and bounded simulation receipt, not independent correspondence,
authority/signature integration, Candidate A completion, candidate selection, or Council acceptance.

## Frozen design and scope

Fixture ID: `installment-two-when-v1`. N4 chooses first fill or recovery.
First fill reaches N3 (Pay Alice A 5 to Bob) and stops at unexpired N2.
A separate second-fill transaction reaches N1 (Pay Alice A 5 to Bob), then N0 Close.
N4 and N2 each also accept Alice recovery directly to N0; their timeout is 100,
with timeout continuation N0. Unused nodes remain Close. No frozen constructor or
Core semantics was changed. Canonical swap source text is unchanged.

Initial agreement has Alice A 10, other accounts zero, all choices absent,
continuation N4, and minimumTime Time2. This is deliberately different from
the independent Python fixture's initial minimum time 1. The root approved
Time2; any later checker must compute from the actual exported before-state.
The neutral ledger starts with Alice A escrow 10 and all wallet balances zero.

The shared state is `installmentTrace`, containing agreement, ledger, and at most
three complete records. Records retain original program/state/input/time,
before/after ledger, and either complete raw result/projection/effects or a distinct
computation/extraction/ledger diagnostic. Computation and extraction are real
Candidate A calls. Diagnostics are not Core rejects, witnesses, or safe terminal states.

The initial schedule permits first-fill/recovery at Time2 or NoInput/recovery at
Time100. After first fill, second-fill/recovery at Time2 or NoInput/recovery at
Time100 is permitted. After deadline recovery rejection, only actual NoInput
deadline cleanup is enabled. Close has no enabled action. No blanket stutter.
All outcome witnesses are derived from recorded actual computations, not initializer labels.

`installmentTraceSafety` checks bounded replay from the exact initial state,
request/program/state/ledger chains, actual raw results, exact producer projections
and extraction, escrow-account coupling, conservation, direct account/payment
arithmetic, complete rollback, absence of diagnostics, and terminal/enabledness.
These checks reuse the local evaluator/projector and are not an independent
Python correspondence proof. Tamper tests cover ledger, request, reduction count,
projection, effects, and diagnostics.

Cancellation is a pure identity view over predecessor/proposed successor, with
empty effects and NoCoreProjection. It does not call computeTransaction,
produce any accepted/rejected Core result, or appear among transaction ITF records.
Recovery is agreement-legal without cancellation; envelope authorization is deferred.

## Test-first source closure

Eight source files were copied with apply_patch into
`.superpowers/sdd/candidate-a-task6-stages/red/` before any Task 6 logic.
The installment program was a Close-only stub at N4; action guards and safety
were false, state updates unchanged, and cancellation returned an explicitly
non-Core unavailable variant. The final implementation removes that variant.
The 23-test source was unchanged between RED and GREEN. One initial fixture
test passed in RED; 22 graph/action/cancellation/semantic tests failed.
The exact RED run below uses the copied closure, not the mutable live path.

RED closure hashes:

```text
e759d4c13032d83a4d839e08bef0fcf8272943792f73bb3ab7910357b0f28dec  .superpowers/sdd/candidate-a-task6-stages/red/candidate_a_core.qnt
6e64b315c291132a7e70c921c94f7876e1aaf0a36f4eae3eeac980f375f23483  .superpowers/sdd/candidate-a-task6-stages/red/candidate_a_installment_harness.qnt
6ce3ed1077f46c5aabbaf4449548adb39b3916935d4021acf76af1ebd2a80f8f  .superpowers/sdd/candidate-a-task6-stages/red/candidate_a_installment_test.qnt
4a7514acec471781290184066af7bf58449bfeaf8d6a7f07879a6e8ea2c7edbe  .superpowers/sdd/candidate-a-task6-stages/red/candidate_a_programs.qnt
2eb66db8010a3d47ddba8f99d647326d2eb7bf3c9da5177c7dd2dcd88145452c  .superpowers/sdd/candidate-a-task6-stages/red/candidate_a_projection.qnt
40901738fd1749527761b624a84253da94bbc24c209d408dc2fd4e895daaae0b  .superpowers/sdd/candidate-a-task6-stages/red/candidate_a_types.qnt
dccc9208d9a42f01685000b1358f1d2f9684bf5e8ebe1d5deadd0c992557504c  .superpowers/sdd/candidate-a-task6-stages/red/effects.qnt
e44484ed9035e560ef9f4d012198671917750da42ab404022592db16cdd59bc3  .superpowers/sdd/candidate-a-task6-stages/red/observations.qnt
```

Final source hashes, checked before final typecheck/sampling and again after
all target exports, were identical:

```text
40901738fd1749527761b624a84253da94bbc24c209d408dc2fd4e895daaae0b  specs/quint/s02/candidate_a_types.qnt
bf814bce2924cafac5836a171b52a4c9bdba43e083fe7129bcdeadd810c42d5e  specs/quint/s02/candidate_a_programs.qnt
e759d4c13032d83a4d839e08bef0fcf8272943792f73bb3ab7910357b0f28dec  specs/quint/s02/candidate_a_core.qnt
2eb66db8010a3d47ddba8f99d647326d2eb7bf3c9da5177c7dd2dcd88145452c  specs/quint/s02/candidate_a_projection.qnt
dccc9208d9a42f01685000b1358f1d2f9684bf5e8ebe1d5deadd0c992557504c  specs/quint/s02/effects.qnt
e44484ed9035e560ef9f4d012198671917750da42ab404022592db16cdd59bc3  specs/quint/s02/observations.qnt
5912655f3f8b35e7f85602099851fea9bdddf6a28493964ddc834e53b098194e  specs/quint/s02/candidate_a_installment_harness.qnt
6ce3ed1077f46c5aabbaf4449548adb39b3916935d4021acf76af1ebd2a80f8f  specs/quint/s02/candidate_a_installment_test.qnt
```

Frozen references, executable entry point, and actual ITF artifact hashes:

```text
47b7d680092cca11534ff3fec20f101c526f3b8d2e7e43a5efcea6a5773b1219  .superpowers/sdd/candidate-a-task6-stages/samples/refund-five.itf.json
2c6c6e926a9aa0043de26ce291abba70a769ea06e284f3120245e4cbb7d37ac0  .superpowers/sdd/candidate-a-task6-stages/samples/refund-ten.itf.json
b55f8c819ca1b4ff835bb599f93653e86a4a1f4383e84a6d7dba2e705ef2f7c3  .superpowers/sdd/candidate-a-task6-stages/samples/residual-deadline-cleanup.itf.json
62d7f2e8821214663a274751724dc58093fedb11a0bdd81eb852485bcca84114  .superpowers/sdd/candidate-a-task6-stages/samples/two-fills.itf.json
564a926779beb54bd16ff58481f5c950bae6fb50fcd5b0d7c9eb56427f83273b  moriarty/core.py
82e9e1da76af65706171049f0238953aca5333edec4aeed6caa43dbd57c79797  moriarty/swap.py
9d91935a09382e68dd6ccf24b565ca243d4b0524acf0a4af79999ee830de2633  tests/test_s02_candidate_a_installment_reference.py
cc8c66a1a4b94a788376dd9ba1649d6d485d00cb0c254020998a77cba3bf5860  docs/superpowers/plans/2026-09-05-moriarty-s02-candidate-a-core.md
ac12595b1cb7253feec93c79417615c6eb20fc3b6a3df35c5e3530b24e90a501  /home/charl/.npm-global/bin/quint
```

The frozen Python 22-vector reference was inspected as workload guidance,
not executed as a correspondence check by this author. Pure tests include
deadline 100/101 commit versus rollback, wrong chooser/phase, choice bounds,
time-before-state, and input-required at both When boundaries.

## Typed scaffold

Command:

```sh
quint typecheck specs/quint/s02/candidate_a_installment_test.qnt
```

Terminal exit: 0.

```text
(no terminal output)
```

## Typed full behavioral test unit before implementation

Command:

```sh
quint typecheck specs/quint/s02/candidate_a_installment_test.qnt
```

Terminal exit: 0.

```text
(no terminal output)
```

## Behavioral RED against immutable copied source

Command:

```sh
quint test .superpowers/sdd/candidate-a-task6-stages/red/candidate_a_installment_test.qnt --backend=rust --seed=42
```

Terminal exit: 1.

```text

  candidate_a_installment_test
    1) exactInstallmentNodeTableTest failed after 1 test(s)
    ok initialFixtureNoWitnessTest passed 1 test(s)
    2) initialEnabledSafetyTest failed after 1 test(s)
    3) firstFillStopsAtSecondWhenTest failed after 1 test(s)
    4) secondFillCompletesTest failed after 1 test(s)
    5) recoveryChoiceRefundsTenTest failed after 1 test(s)
    6) recoveryChoiceRefundsFiveTest failed after 1 test(s)
    7) initialDeadlineRefundTest failed after 1 test(s)
    8) residualDeadlineRefundTest failed after 1 test(s)
    9) installmentTimeoutPriorityTest failed after 1 test(s)
    10) residualTimeoutPriorityTest failed after 1 test(s)
    11) initialRejectedCleanupRetainsAttemptTest failed after 1 test(s)
    12) residualRejectedCleanupRetainsAttemptTest failed after 1 test(s)
    13) cancellationHasNoCoreResultTest failed after 1 test(s)
    14) residualCancellationHasNoCoreResultTest failed after 1 test(s)
    15) recoveryLegalWithoutCancellationTest failed after 1 test(s)
    16) deadline101CommitAndRollbackTest failed after 1 test(s)
    17) invalidInstallmentRequestsRollbackTest failed after 1 test(s)
    18) onlyCleanupAfterRejectedRecoveryTest failed after 1 test(s)
    19) terminalNoActionTest failed after 1 test(s)
    20) installmentLedgerTamperDetectedTest failed after 1 test(s)
    21) installmentRecordTamperDetectedTest failed after 1 test(s)
    22) installmentDiagnosticNeverWitnessTest failed after 1 test(s)

  1 passing (1172ms)
  22 failed

  1) exactInstallmentNodeTableTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task6-stages/red/candidate_a_installment_test.qnt:21:39
        21:   run exactInstallmentNodeTableTest = assert(installmentProgram == {root: N4, nodes: closeProgram.nodes
                                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        22:     .put(N1, PayA({account: aliceA, payee: Bob, amount: ConstantA(5), continuation: N0}))
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        23:     .put(N2, WhenA({cases: List(
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        24:       {caseAction: ChoiceA({id: SecondFillId, chooser: Bob, lower: 1, upper: 1}), continuation: N1},
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        25:       {caseAction: ChoiceA({id: RecoveryId, chooser: Alice, lower: 1, upper: 1}), continuation: N0}),
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        26:       timeout: Time100, timeoutNode: N0}))
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        27:     .put(N3, PayA({account: aliceA, payee: Bob, amount: ConstantA(5), continuation: N2}))
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        28:     .put(N4, WhenA({cases: List(
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        29:       {caseAction: ChoiceA({id: FirstFillId, chooser: Bob, lower: 1, upper: 1}), continuation: N3},
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        30:       {caseAction: ChoiceA({id: RecoveryId, chooser: Alice, lower: 1, upper: 1}), continuation: N0}),
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        31:       timeout: Time100, timeoutNode: N0}))})
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=exactInstallmentNodeTableTest to repeat.
  2) initialEnabledSafetyTest:
       Error [QNT508]: Expect condition does not hold true
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task6-stages/red/candidate_a_installment_test.qnt:37:34
        37:   run initialEnabledSafetyTest = init.expect(installmentTraceSafety and not(installmentTerminal)
                                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        38:     and canRequest(installmentTrace, fillOneInput, Time2)
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        39:     and canRequest(installmentTrace, recoveryInput, Time2)
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        40:     and not(canRequest(installmentTrace, fillTwoInput, Time2)))
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=initialEnabledSafetyTest to repeat.
  3) firstFillStopsAtSecondWhenTest:
       Error [QNT508]: Cannot continue to "expect"
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task6-stages/red/candidate_a_installment_test.qnt:41:40
        41:   run firstFillStopsAtSecondWhenTest = init.then(fillFirst).expect(firstAgreementFill
                                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        42:     and installmentTraceSafety and not(installmentTerminal) and installmentTrace.records.length() == 1
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        43:     and installmentTrace.agreement == expectedAfterFirst
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        44:     and match installmentTrace.records.head().payload {
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        45:       | InstallmentComputedA(payload) => payload.raw == {accepted: true, state: expectedAfterFirst, error: NoCoreError,
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        46:           payments: List({source: aliceA, recipient: Bob, asset: TokenA, quantity: 5}), warnings: List(), reductions: 1}
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        47:           and payload.effects == List({source: Escrow(aliceA), destination: Wallet(Bob), asset: TokenA, quantity: 5})
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        48:       | _ => false
            ^^^^^^^^^^^^^^^^^^
        49:     })
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=firstFillStopsAtSecondWhenTest to repeat.
  4) secondFillCompletesTest:
       Error [QNT513]: Cannot continue in `then` because the highlighted expression evaluated to false
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task6-stages/red/candidate_a_installment_test.qnt:50:43
        50:   run secondFillCompletesTest = init.then(fillFirst).then(fillSecond).expect(firstAgreementFill
                                                      ^^^^^^^^^
    Use --seed=0x2a --match=secondFillCompletesTest to repeat.
  5) recoveryChoiceRefundsTenTest:
       Error [QNT508]: Cannot continue to "expect"
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task6-stages/red/candidate_a_installment_test.qnt:60:38
        60:   run recoveryChoiceRefundsTenTest = init.then(recoverTen).expect(agreementRefundTen
                                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        61:     and installmentTraceSafety and installmentTerminal and match installmentTrace.records.head().payload {
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        62:       | InstallmentComputedA(payload) => payload.raw == expectedRefund(installmentInitial.agreement, 10, Time2, true)
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        63:       | _ => false
            ^^^^^^^^^^^^^^^^^^
        64:     })
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=recoveryChoiceRefundsTenTest to repeat.
  6) recoveryChoiceRefundsFiveTest:
       Error [QNT513]: Cannot continue in `then` because the highlighted expression evaluated to false
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task6-stages/red/candidate_a_installment_test.qnt:65:49
        65:   run recoveryChoiceRefundsFiveTest = init.then(fillFirst).then(recoverFive).expect(agreementRefundFive
                                                            ^^^^^^^^^
    Use --seed=0x2a --match=recoveryChoiceRefundsFiveTest to repeat.
  7) initialDeadlineRefundTest:
       Error [QNT508]: Cannot continue to "expect"
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task6-stages/red/candidate_a_installment_test.qnt:72:35
        72:   run initialDeadlineRefundTest = init.then(timeoutTen).expect(agreementDeadlineTen
                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        73:     and installmentTraceSafety and installmentTerminal and match installmentTrace.records.head().payload {
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        74:       | InstallmentComputedA(payload) => payload.raw == expectedRefund(installmentInitial.agreement, 10, Time100, false)
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        75:       | _ => false
            ^^^^^^^^^^^^^^^^^^
        76:     })
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=initialDeadlineRefundTest to repeat.
  8) residualDeadlineRefundTest:
       Error [QNT513]: Cannot continue in `then` because the highlighted expression evaluated to false
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task6-stages/red/candidate_a_installment_test.qnt:77:46
        77:   run residualDeadlineRefundTest = init.then(fillFirst).then(timeoutFive).expect(agreementDeadlineFive
                                                         ^^^^^^^^^
    Use --seed=0x2a --match=residualDeadlineRefundTest to repeat.
  9) installmentTimeoutPriorityTest:
       Error [QNT508]: Cannot continue to "expect"
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task6-stages/red/candidate_a_installment_test.qnt:82:40
        82:   run installmentTimeoutPriorityTest = init.then(rejectRecoveryTen).expect(
                                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        83:     agreementDeadlineRecoveryTenRejected and installmentTraceSafety and not(installmentTerminal)
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        84:       and installmentTrace.agreement == installmentInitial.agreement
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        85:       and installmentTrace.ledger == installmentInitial.ledger)
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=installmentTimeoutPriorityTest to repeat.
  10) residualTimeoutPriorityTest:
       Error [QNT513]: Cannot continue in `then` because the highlighted expression evaluated to false
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task6-stages/red/candidate_a_installment_test.qnt:86:47
        86:   run residualTimeoutPriorityTest = init.then(fillFirst).then(rejectRecoveryFive).expect(
                                                          ^^^^^^^^^
    Use --seed=0x2a --match=residualTimeoutPriorityTest to repeat.
  11) initialRejectedCleanupRetainsAttemptTest:
       Error [QNT513]: Cannot continue in `then` because the highlighted expression evaluated to false
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task6-stages/red/candidate_a_installment_test.qnt:91:60
        91:   run initialRejectedCleanupRetainsAttemptTest = init.then(rejectRecoveryTen).then(timeoutTen).expect(
                                                                       ^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=initialRejectedCleanupRetainsAttemptTest to repeat.
  12) residualRejectedCleanupRetainsAttemptTest:
       Error [QNT513]: Cannot continue in `then` because the highlighted expression evaluated to false
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task6-stages/red/candidate_a_installment_test.qnt:94:61
        94:   run residualRejectedCleanupRetainsAttemptTest = init.then(fillFirst).then(rejectRecoveryFive).then(timeoutFive).expect(
                                                                        ^^^^^^^^^
    Use --seed=0x2a --match=residualRejectedCleanupRetainsAttemptTest to repeat.
  13) cancellationHasNoCoreResultTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task6-stages/red/candidate_a_installment_test.qnt:97:41
        97:   run cancellationHasNoCoreResultTest = assert(cancellationAdapter(installmentInitial.agreement)
                                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        98:     == CancellationAdaptedA({predecessor: {program: installmentProgram, state: installmentInitial.agreement},
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        99:       proposedSuccessor: {program: installmentProgram, state: installmentInitial.agreement},
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        100:       effects: List(), coreProjection: NoCoreProjection}))
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=cancellationHasNoCoreResultTest to repeat.
  14) residualCancellationHasNoCoreResultTest:
       Error [QNT508]: Cannot continue to "expect"
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task6-stages/red/candidate_a_installment_test.qnt:101:49
        101:   run residualCancellationHasNoCoreResultTest = init.then(fillFirst).expect(
                                                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        102:     cancellationAdapter(installmentTrace.agreement) == CancellationAdaptedA({
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        103:       predecessor: {program: installmentProgram, state: expectedAfterFirst},
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        104:       proposedSuccessor: {program: installmentProgram, state: expectedAfterFirst},
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        105:       effects: List(), coreProjection: NoCoreProjection}))
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=residualCancellationHasNoCoreResultTest to repeat.
  15) recoveryLegalWithoutCancellationTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task6-stages/red/candidate_a_installment_test.qnt:106:46
        106:   run recoveryLegalWithoutCancellationTest = assert(computeTransaction(installmentProgram,
                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        107:     installmentInitial.agreement, recoveryInput, Time2)
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        108:       == TransactionComputedA(expectedRefund(installmentInitial.agreement, 10, Time2, true)))
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=recoveryLegalWithoutCancellationTest to repeat.
  16) deadline101CommitAndRollbackTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task6-stages/red/candidate_a_installment_test.qnt:109:42
        109:   run deadline101CommitAndRollbackTest = assert(Set(installmentInitial.agreement, expectedAfterFirst).forall(before =>
                                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        110:     computeTransaction(installmentProgram, before, NoAInput, Time101)
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        111:       == TransactionComputedA(expectedRefund(before, before.accounts.get(aliceA), Time101, false))
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        112:     and computeTransaction(installmentProgram, before, recoveryInput, Time101)
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        113:       == expectedRejection(before, "contract_closed")))
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=deadline101CommitAndRollbackTest to repeat.
  17) invalidInstallmentRequestsRollbackTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task6-stages/red/candidate_a_installment_test.qnt:114:48
        114:   run invalidInstallmentRequestsRollbackTest = assert(Set(installmentInitial.agreement, expectedAfterFirst).forall(before => {
                                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        115:     val phase = if (before.continuation == N4) FirstFillId else SecondFillId
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        116:     val wrongPhase = if (before.continuation == N4) fillTwoInput else fillOneInput
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        117:     val correctInput = if (before.continuation == N4) fillOneInput else fillTwoInput
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        118:     computeTransaction(installmentProgram, before, wrongPhase, Time2) == expectedRejection(before, "no_matching_input")
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        119:       and computeTransaction(installmentProgram, before, PresentAInput(ChoiceInputA({id: phase, chooser: Alice, chosen: 1})), Time2)
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        120:         == expectedRejection(before, "no_matching_input")
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        121:       and computeTransaction(installmentProgram, before, PresentAInput(ChoiceInputA({id: phase, chooser: Bob, chosen: 0})), Time2)
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        122:         == expectedRejection(before, "choice_out_of_bounds")
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        123:       and computeTransaction(installmentProgram, before, correctInput, Time0) == expectedRejection(before, "time_before_state")
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        124:       and computeTransaction(installmentProgram, before, NoAInput, Time2) == expectedRejection(before, "input_required")
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        125:   }))
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=invalidInstallmentRequestsRollbackTest to repeat.
  18) onlyCleanupAfterRejectedRecoveryTest:
       Error [QNT513]: Cannot continue in `then` because the highlighted expression evaluated to false
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task6-stages/red/candidate_a_installment_test.qnt:126:56
        126:   run onlyCleanupAfterRejectedRecoveryTest = init.then(fillFirst).then(rejectRecoveryFive).expect(
                                                                    ^^^^^^^^^
    Use --seed=0x2a --match=onlyCleanupAfterRejectedRecoveryTest to repeat.
  19) terminalNoActionTest:
       Error [QNT513]: Cannot continue in `then` because the highlighted expression evaluated to false
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task6-stages/red/candidate_a_installment_test.qnt:131:40
        131:   run terminalNoActionTest = init.then(fillFirst).then(fillSecond).expect(
                                                    ^^^^^^^^^
    Use --seed=0x2a --match=terminalNoActionTest to repeat.
  20) installmentLedgerTamperDetectedTest:
       Error [QNT508]: Cannot continue to "expect"
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task6-stages/red/candidate_a_installment_test.qnt:133:45
        133:   run installmentLedgerTamperDetectedTest = init.then(fillFirst).expect(not(traceSafety({...installmentTrace,
                                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        134:     ledger: installmentTrace.ledger.put((Wallet(Mallory), TokenA), 1)})))
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=installmentLedgerTamperDetectedTest to repeat.
  21) installmentRecordTamperDetectedTest:
       Error [QNT508]: Cannot continue to "expect"
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task6-stages/red/candidate_a_installment_test.qnt:135:45
        135:   run installmentRecordTamperDetectedTest = init.then(fillFirst).expect(match installmentTrace.records.head().payload {
                                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        136:     | InstallmentComputedA(payload) => {
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        137:         val original = installmentTrace.records.head()
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        138:         val badRaw = {...original, payload: InstallmentComputedA({...payload, raw: {...payload.raw, reductions: 0}})}
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        139:         val badProjection = {...original, payload: InstallmentComputedA({...payload,
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        140:           projection: {...payload.projection, reductions: 0}})}
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        141:         val badEffects = {...original, payload: InstallmentComputedA({...payload, effects: List()})}
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        142:         val badRequest = {...original, request: {...original.request, input: fillTwoInput}}
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        143:         Set(badRaw, badProjection, badEffects, badRequest).forall(changed =>
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        144:           not(traceSafety({...installmentTrace, records: List(changed)})))
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        145:       }
             ^^^^^^^
        146:     | _ => false
             ^^^^^^^^^^^^^^^^
        147:   })
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=installmentRecordTamperDetectedTest to repeat.
  22) installmentDiagnosticNeverWitnessTest:
       Error [QNT508]: Cannot continue to "expect"
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task6-stages/red/candidate_a_installment_test.qnt:148:47
        148:   run installmentDiagnosticNeverWitnessTest = init.then(fillFirst).expect(match installmentTrace.records.head().payload {
                                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        149:     | InstallmentComputedA(payload) => {
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        150:         val original = installmentTrace.records.head()
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        151:         val badComputation = {...original, payload: InstallmentComputationDiagnosticA(OutsideModelDomainA)}
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        152:         val badLedger = {...original, payload: InstallmentLedgerDiagnosticA(payload)}
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        153:         Set(badComputation, badLedger).forall(changed => {
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        154:           val bad = {...installmentTrace, records: List(changed)}
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        155:           not(traceSafety(bad)) and not(traceTerminal(bad)) and not(recorded(bad, fillOneInput, Time2, N4, true))
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        156:         })
             ^^^^^^^^^^
        157:       }
             ^^^^^^^
        158:     | _ => false
             ^^^^^^^^^^^^^^^^
        159:   })
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=installmentDiagnosticNeverWitnessTest to repeat.


  Use --verbosity=3 to show executions.
  Further debug with: quint test --verbosity=3 .superpowers/sdd/candidate-a-task6-stages/red/candidate_a_installment_test.qnt
error: Tests failed
```

## Intermediate development failure disclosure

The first implementation test command exposed a mechanical identifier-replacement
mistake: canonicalSwap had become canonicalInstallment before the intended rename.
This parser failure is not the behavioral RED and no behavioral test was weakened.
The eight references were corrected to installmentProgram before the following GREEN.

## Intermediate parser failure

Command:

```sh
quint test specs/quint/s02/candidate_a_installment_test.qnt --backend=rust --seed=42
```

Terminal exit: 1.

```text

 Error [QNT404]: Name 'canonicalInstallment' not found

  at /home/charl/Moriarty/.worktrees/s01-audit-start/specs/quint/s02/candidate_a_installment_harness.qnt:47:67
  47:       | InstallmentComputedA(payload) => entry.request.program == canonicalInstallment and entry.request.input == input
                                                                        ^^^^^^^^^^^^^^^^^^^^


 Error [QNT404]: Name 'canonicalInstallment' not found

  at /home/charl/Moriarty/.worktrees/s01-audit-start/specs/quint/s02/candidate_a_installment_harness.qnt:73:40
  73:     val evaluated = computeTransaction(canonicalInstallment, s.agreement, input, now)
                                             ^^^^^^^^^^^^^^^^^^^^


 Error [QNT404]: Name 'canonicalInstallment' not found

  at /home/charl/Moriarty/.worktrees/s01-audit-start/specs/quint/s02/candidate_a_installment_harness.qnt:76:42
  76:           val projection = projectResult(canonicalInstallment, raw)
                                               ^^^^^^^^^^^^^^^^^^^^


 Error [QNT404]: Name 'canonicalInstallment' not found

  at /home/charl/Moriarty/.worktrees/s01-audit-start/specs/quint/s02/candidate_a_installment_harness.qnt:77:51
  77:           val extracted = extractCommittedEffects(canonicalInstallment, s.agreement, input, now, raw)
                                                        ^^^^^^^^^^^^^^^^^^^^


 Error [QNT404]: Name 'canonicalInstallment' not found

  at /home/charl/Moriarty/.worktrees/s01-audit-start/specs/quint/s02/candidate_a_installment_harness.qnt:109:24
  109:     if (not(validState(canonicalInstallment, agreement)) or not(validLedger(ledger))) false
                              ^^^^^^^^^^^^^^^^^^^^


 Error [QNT404]: Name 'canonicalInstallment' not found

  at /home/charl/Moriarty/.worktrees/s01-audit-start/specs/quint/s02/candidate_a_installment_harness.qnt:121:28
  121:         if (not(validState(canonicalInstallment, entry.request.before))
                                  ^^^^^^^^^^^^^^^^^^^^


 Error [QNT404]: Name 'canonicalInstallment' not found

  at /home/charl/Moriarty/.worktrees/s01-audit-start/specs/quint/s02/candidate_a_installment_harness.qnt:122:29
  122:           or not(validState(canonicalInstallment, payload.raw.state))) false
                                   ^^^^^^^^^^^^^^^^^^^^


 Error [QNT404]: Name 'canonicalInstallment' not found

  at /home/charl/Moriarty/.worktrees/s01-audit-start/specs/quint/s02/candidate_a_installment_harness.qnt:123:39
  123:         else entry.request.program == canonicalInstallment and entry.request.before == prefix.agreement
                                             ^^^^^^^^^^^^^^^^^^^^

error: parsing failed
```

## GREEN unchanged 23-test unit

Command:

```sh
quint test specs/quint/s02/candidate_a_installment_test.qnt --backend=rust --seed=42
```

Terminal exit: 0.

```text

  candidate_a_installment_test
    ok exactInstallmentNodeTableTest passed 1 test(s)
    ok initialFixtureNoWitnessTest passed 1 test(s)
    ok initialEnabledSafetyTest passed 1 test(s)
    ok firstFillStopsAtSecondWhenTest passed 1 test(s)
    ok secondFillCompletesTest passed 1 test(s)
    ok recoveryChoiceRefundsTenTest passed 1 test(s)
    ok recoveryChoiceRefundsFiveTest passed 1 test(s)
    ok initialDeadlineRefundTest passed 1 test(s)
    ok residualDeadlineRefundTest passed 1 test(s)
    ok installmentTimeoutPriorityTest passed 1 test(s)
    ok residualTimeoutPriorityTest passed 1 test(s)
    ok initialRejectedCleanupRetainsAttemptTest passed 1 test(s)
    ok residualRejectedCleanupRetainsAttemptTest passed 1 test(s)
    ok cancellationHasNoCoreResultTest passed 1 test(s)
    ok residualCancellationHasNoCoreResultTest passed 1 test(s)
    ok recoveryLegalWithoutCancellationTest passed 1 test(s)
    ok deadline101CommitAndRollbackTest passed 1 test(s)
    ok invalidInstallmentRequestsRollbackTest passed 1 test(s)
    ok onlyCleanupAfterRejectedRecoveryTest passed 1 test(s)
    ok terminalNoActionTest passed 1 test(s)
    ok installmentLedgerTamperDetectedTest passed 1 test(s)
    ok installmentRecordTamperDetectedTest passed 1 test(s)
    ok installmentDiagnosticNeverWitnessTest passed 1 test(s)

  23 passing (1754ms)
```

## Final standalone typecheck

Command:

```sh
quint typecheck specs/quint/s02/candidate_a_installment_test.qnt
```

Terminal exit: 0.

```text
(no terminal output)
```

## Bounded safety and all nine witnesses

Command:

```sh
quint run specs/quint/s02/candidate_a_installment_harness.qnt --backend=rust --seed=42 --max-samples=100 --max-steps=8 --invariant=installmentTraceSafety --witnesses firstAgreementFill secondAgreementFill agreementRefundTen agreementRefundFive agreementDeadlineTen agreementDeadlineFive agreementDeadlineRecoveryTenRejected agreementDeadlineRecoveryFiveRejected installmentTerminal --verbosity=1
```

Terminal exit: 0.

```text
[ok] No violation found (584ms at 171 traces/second).
Witnesses:
firstAgreementFill was witnessed in 20 trace(s) out of 100 explored (20.00%)
secondAgreementFill was witnessed in 6 trace(s) out of 100 explored (6.00%)
agreementRefundTen was witnessed in 35 trace(s) out of 100 explored (35.00%)
agreementRefundFive was witnessed in 6 trace(s) out of 100 explored (6.00%)
agreementDeadlineTen was witnessed in 45 trace(s) out of 100 explored (45.00%)
agreementDeadlineFive was witnessed in 8 trace(s) out of 100 explored (8.00%)
agreementDeadlineRecoveryTenRejected was witnessed in 18 trace(s) out of 100 explored (18.00%)
agreementDeadlineRecoveryFiveRejected was witnessed in 3 trace(s) out of 100 explored (3.00%)
installmentTerminal was witnessed in 100 trace(s) out of 100 explored (100.00%)
Use --seed=0x3ba --backend=rust to reproduce.
```

The CLI prints reproduction seed 0x3ba although the invoked command used seed42;
both are preserved exactly. This run explored 100 finite traces, not exhaustive
model checking. Each of the eight action witnesses was positive; all 100 reached terminal.

## Actual target ITF exports

Each following command deliberately negates its reachability witness. Exit 1 is
the expected target reach, not an installmentTraceSafety violation. Complete raw
ITF files retain the full original requests, accounts, choices, times, raw outputs,
projection, effects, and ledger chain. Compact extraction below is for inspection
only; it is not a replacement for those raw files.

## Actual trace: two-fills

Command:

```sh
quint run specs/quint/s02/candidate_a_installment_harness.qnt --backend=rust --seed=42 --max-samples=100 --max-steps=8 --invariant='not(secondAgreementFill)' --out-itf=.superpowers/sdd/candidate-a-task6-stages/samples/two-fills.itf.json --verbosity=1
```

Terminal exit: 1.

```text
[violation] Found an issue (88ms at 80 traces/second).
error: Invariant violated
```

## Actual trace: refund-ten

Command:

```sh
quint run specs/quint/s02/candidate_a_installment_harness.qnt --backend=rust --seed=42 --max-samples=100 --max-steps=8 --invariant='not(agreementRefundTen)' --out-itf=.superpowers/sdd/candidate-a-task6-stages/samples/refund-ten.itf.json --verbosity=1
```

Terminal exit: 1.

```text
[violation] Found an issue (74ms at 27 traces/second).
error: Invariant violated
```

## Actual trace: refund-five

Command:

```sh
quint run specs/quint/s02/candidate_a_installment_harness.qnt --backend=rust --seed=42 --max-samples=100 --max-steps=8 --invariant='not(agreementRefundFive)' --out-itf=.superpowers/sdd/candidate-a-task6-stages/samples/refund-five.itf.json --verbosity=1
```

Terminal exit: 1.

```text
[violation] Found an issue (128ms at 203 traces/second).
error: Invariant violated
```

## Actual trace: residual-deadline-cleanup

Command:

```sh
quint run specs/quint/s02/candidate_a_installment_harness.qnt --backend=rust --seed=42 --max-samples=100 --max-steps=8 --invariant='not(agreementDeadlineRecoveryFiveRejected and installmentTerminal)' --out-itf=.superpowers/sdd/candidate-a-task6-stages/samples/residual-deadline-cleanup.itf.json --verbosity=1
```

Terminal exit: 1.

```text
[violation] Found an issue (121ms at 240 traces/second).
error: Invariant violated
```

## Bounded inspection of actual exported records

Command:

```sh
jq '{file:input_filename,states:(.states|length),records:(.states[-1].installmentTrace.records | map({before:.request.before.continuation.tag,input:(.request.input.value.value.id.tag // .request.input.tag),now:.request.now.tag,payload:.payload.tag,accepted:.payload.value.raw.accepted,error:.payload.value.raw.error,after:.payload.value.raw.state.continuation.tag,reductions:.payload.value.raw.reductions["#bigint"],payments:(.payload.value.raw.payments|map({recipient:.recipient.tag,quantity:.quantity["#bigint"]})),effects:(.payload.value.effects|map({destination:.destination.value.tag,quantity:.quantity["#bigint"]}))}))}' .superpowers/sdd/candidate-a-task6-stages/samples/*.itf.json
```

Terminal exit: 0.

```text
{
  "file": ".superpowers/sdd/candidate-a-task6-stages/samples/refund-five.itf.json",
  "states": 3,
  "records": [
    {
      "before": "N4",
      "input": "FirstFillId",
      "now": "Time2",
      "payload": "InstallmentComputedA",
      "accepted": true,
      "error": {
        "tag": "NoCoreError",
        "value": {
          "#tup": []
        }
      },
      "after": "N2",
      "reductions": "1",
      "payments": [
        {
          "recipient": "Bob",
          "quantity": "5"
        }
      ],
      "effects": [
        {
          "destination": "Bob",
          "quantity": "5"
        }
      ]
    },
    {
      "before": "N2",
      "input": "RecoveryId",
      "now": "Time2",
      "payload": "InstallmentComputedA",
      "accepted": true,
      "error": {
        "tag": "NoCoreError",
        "value": {
          "#tup": []
        }
      },
      "after": "N0",
      "reductions": "1",
      "payments": [
        {
          "recipient": "Alice",
          "quantity": "5"
        }
      ],
      "effects": [
        {
          "destination": "Alice",
          "quantity": "5"
        }
      ]
    }
  ]
}
{
  "file": ".superpowers/sdd/candidate-a-task6-stages/samples/refund-ten.itf.json",
  "states": 2,
  "records": [
    {
      "before": "N4",
      "input": "RecoveryId",
      "now": "Time2",
      "payload": "InstallmentComputedA",
      "accepted": true,
      "error": {
        "tag": "NoCoreError",
        "value": {
          "#tup": []
        }
      },
      "after": "N0",
      "reductions": "1",
      "payments": [
        {
          "recipient": "Alice",
          "quantity": "10"
        }
      ],
      "effects": [
        {
          "destination": "Alice",
          "quantity": "10"
        }
      ]
    }
  ]
}
{
  "file": ".superpowers/sdd/candidate-a-task6-stages/samples/residual-deadline-cleanup.itf.json",
  "states": 4,
  "records": [
    {
      "before": "N4",
      "input": "FirstFillId",
      "now": "Time2",
      "payload": "InstallmentComputedA",
      "accepted": true,
      "error": {
        "tag": "NoCoreError",
        "value": {
          "#tup": []
        }
      },
      "after": "N2",
      "reductions": "1",
      "payments": [
        {
          "recipient": "Bob",
          "quantity": "5"
        }
      ],
      "effects": [
        {
          "destination": "Bob",
          "quantity": "5"
        }
      ]
    },
    {
      "before": "N2",
      "input": "RecoveryId",
      "now": "Time100",
      "payload": "InstallmentComputedA",
      "accepted": false,
      "error": {
        "tag": "CoreErrorCode",
        "value": "contract_closed"
      },
      "after": "N2",
      "reductions": "0",
      "payments": [],
      "effects": []
    },
    {
      "before": "N2",
      "input": "NoAInput",
      "now": "Time100",
      "payload": "InstallmentComputedA",
      "accepted": true,
      "error": {
        "tag": "NoCoreError",
        "value": {
          "#tup": []
        }
      },
      "after": "N0",
      "reductions": "2",
      "payments": [
        {
          "recipient": "Alice",
          "quantity": "5"
        }
      ],
      "effects": [
        {
          "destination": "Alice",
          "quantity": "5"
        }
      ]
    }
  ]
}
{
  "file": ".superpowers/sdd/candidate-a-task6-stages/samples/two-fills.itf.json",
  "states": 3,
  "records": [
    {
      "before": "N4",
      "input": "FirstFillId",
      "now": "Time2",
      "payload": "InstallmentComputedA",
      "accepted": true,
      "error": {
        "tag": "NoCoreError",
        "value": {
          "#tup": []
        }
      },
      "after": "N2",
      "reductions": "1",
      "payments": [
        {
          "recipient": "Bob",
          "quantity": "5"
        }
      ],
      "effects": [
        {
          "destination": "Bob",
          "quantity": "5"
        }
      ]
    },
    {
      "before": "N2",
      "input": "SecondFillId",
      "now": "Time2",
      "payload": "InstallmentComputedA",
      "accepted": true,
      "error": {
        "tag": "NoCoreError",
        "value": {
          "#tup": []
        }
      },
      "after": "N0",
      "reductions": "1",
      "payments": [
        {
          "recipient": "Bob",
          "quantity": "5"
        }
      ],
      "effects": [
        {
          "destination": "Bob",
          "quantity": "5"
        }
      ]
    }
  ]
}
```

## Handoff

Author checks are complete for this bounded unit: standalone typecheck, 23 tests,
100 sampled traces with all nine witnesses, and four source-pinned raw ITF targets.
Root independent execution/source review and the independently owned exporter/checker
remain separate evidence. No archived Task 1–5 receipts or other agent-owned sources
were edited; no commit, full-suite run, or model-checker proof was performed here.

