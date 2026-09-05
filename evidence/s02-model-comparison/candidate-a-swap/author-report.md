# Candidate A Task 5 canonical-swap harness receipt

## Scope and design approval

Repository observation: this worker implemented only new candidate_a_harness.qnt and candidate_a_test.qnt. Root approved the trace carriers and bounded schedule before logic, correcting Bob's deposit time to Time2. Starting source milestone supplied by root: `ea35cada6bf84a016ec568834242628ee8ccfa76`. Concurrent root work moved observed HEAD to `d4a714ce6013e8237ff9ef07b6b661c52a421e8c` before final verification. This worker made no commit.

The projection author separately owns candidate_a_projection.qnt/projection_test.qnt. Its approved API was confirmed before coupling; the harness behavioral RED and all GREEN checks use projection source SHA `2eb66db8010a3d47ddba8f99d647326d2eb7bf3c9da5177c7dd2dcd88145452c`. No evaluator, types, programs, shared foundations, archived reports, or prior snapshots were edited by this worker.

Working directory for all commands:
`/home/charl/Moriarty/.worktrees/s01-audit-start`.
Quint entry point: `/home/charl/.npm-global/bin/quint`, version `0.32.0`.

Repository observation: this is an agreement-only shared-state model. Its neutral ledger tracks wallet/escrow accounting, not signing, authority, policy approval, or signature evidence. Init contains empty canonicalSwap at N6 and wallets Alice/TokenA=10, Bob/TokenB=20. No funded agreement or outcome is preselected.

Approved bounded schedule:

- Empty N6: actual Alice Deposit10 at Time1, or NoInput timeout at Time100.
- After Alice, N5: actual Bob Deposit20 at Time2, or Alice-only NoInput timeout at Time100.
- After both, N4: settle1 or settle0 at Time2; NoInput timeout at Time100; or supplied settle1 at Time100.
- After the deadline-input rejection, retain the rejected record and enable only an actual NoInput timeout at Time100. That final transaction refunds both owners.
- At most four transaction records; terminal Close has no enabled action. There is no blanket stutter.

Each SwapTraceRecord retains the original complete program/state/input/time, before/after ledger, actual complete raw result, projection, and ordered effects. Computation, extraction, and ledger-inapplicability diagnostics have distinct payload constructors. They preserve diagnostic details, fail traceSafety, do not satisfy outcome witnesses, and do not become successful terminal states or Core rejections.

Source fact/inference: coreTraceSafety checks exact trace chaining and schedule, valid finite states, neutral-ledger validity/escrow coupling/conservation, actual deposit-minus-payment account arithmetic, exact rollback, ordered applicable effects, no diagnostics, and enabled-versus-terminal status. Rechecking the producer evaluator/projection/extraction is local consistency checking, not independent Python correspondence.

Quint modeling/language and TDD required approved plain shared-state carriers, typed stubs, preserved behavioral RED, guarded thin actions, and a witness for each major action. The simulation skill distinguishes desired witness reachability from safety invariants. No provider dispatch, Foreman/Council action, authority integration, or installment model was added.

## Typed scaffold and behavioral RED

A bootstrap parse error was caused by a parameterized action missing an explicit bool return annotation. It was corrected before typed/behavioral evidence; the first error output was truncated and is not treated as a complete receipt or semantic RED.

The scaffold then typechecked:

```text
$ quint typecheck specs/quint/s02/candidate_a_test.qnt
[no terminal output]
exit_code: 0
```

After all 19 tests were present and the projection implementation was frozen:

```text
$ quint typecheck specs/quint/s02/candidate_a_test.qnt
[no terminal output]
exit_code: 0
```

Before harness logic, apply_patch preserved the exact eight-file import closure in `.superpowers/sdd/candidate-a-task5-harness-stages/red/`. The actual RED command ran against this snapshot, including the implemented frozen projection, not projection stubs. The test source remained unchanged afterward.

```text
$ sha256sum .superpowers/sdd/candidate-a-task5-harness-stages/red/*.qnt
e759d4c13032d83a4d839e08bef0fcf8272943792f73bb3ab7910357b0f28dec  .superpowers/sdd/candidate-a-task5-harness-stages/red/candidate_a_core.qnt
8bd5208437697b25facc611500c893be51bdc6e245013d7a1fa38497602b4528  .superpowers/sdd/candidate-a-task5-harness-stages/red/candidate_a_harness.qnt
b443fc0beec22b867e4d36c21714fbaee63950961776d21b2ebe9463e190ab66  .superpowers/sdd/candidate-a-task5-harness-stages/red/candidate_a_programs.qnt
2eb66db8010a3d47ddba8f99d647326d2eb7bf3c9da5177c7dd2dcd88145452c  .superpowers/sdd/candidate-a-task5-harness-stages/red/candidate_a_projection.qnt
7906400286288d3419ea22b48d424c68f21b5842fa8349ac156df26286077d25  .superpowers/sdd/candidate-a-task5-harness-stages/red/candidate_a_test.qnt
40901738fd1749527761b624a84253da94bbc24c209d408dc2fd4e895daaae0b  .superpowers/sdd/candidate-a-task5-harness-stages/red/candidate_a_types.qnt
dccc9208d9a42f01685000b1358f1d2f9684bf5e8ebe1d5deadd0c992557504c  .superpowers/sdd/candidate-a-task5-harness-stages/red/effects.qnt
e44484ed9035e560ef9f4d012198671917750da42ab404022592db16cdd59bc3  .superpowers/sdd/candidate-a-task5-harness-stages/red/observations.qnt
exit_code: 0
```

```text
$ quint test .superpowers/sdd/candidate-a-task5-harness-stages/red/candidate_a_test.qnt --backend=rust --seed=42

  candidate_a_test
    ok initialTraceEmptyTest passed 1 test(s)
    1) initialScheduleTest failed after 1 test(s)
    2) initialSafetyTest failed after 1 test(s)
    3) depositRecordsActualRequestTest failed after 1 test(s)
    4) swapSettlementTraceTest failed after 1 test(s)
    5) swapVoluntaryRefundTraceTest failed after 1 test(s)
    6) swapAliceOnlyTimeoutTest failed after 1 test(s)
    7) swapEmptyTimeoutTest failed after 1 test(s)
    8) swapFundedTimeoutCommitTest failed after 1 test(s)
    9) swapFundedTimeoutInputRollbackTest failed after 1 test(s)
    10) rejectedAttemptRetainedAfterCleanupTest failed after 1 test(s)
    11) depositWithoutPaymentEffectTest failed after 1 test(s)
    12) rollbackRecordExactTest failed after 1 test(s)
    13) terminalHasNoEnabledActionTest failed after 1 test(s)
    14) rejectionOnlyCleanupEnabledTest failed after 1 test(s)
    15) wrongBobTimeDisabledTest failed after 1 test(s)
    16) ledgerTamperDetectedTest failed after 1 test(s)
    17) tracePayloadTamperDetectedTest failed after 1 test(s)
    18) diagnosticNeverWitnessOrTerminalTest failed after 1 test(s)

  1 passing (831ms)
  18 failed

  1) initialScheduleTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task5-harness-stages/red/candidate_a_test.qnt:12:29
        12:   run initialScheduleTest = assert(canRequest(swapInitial, aliceFunding, Time1)
                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        13:     and canRequest(swapInitial, NoAInput, Time100)
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        14:     and not(canRequest(swapInitial, bobFunding, Time2)))
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=initialScheduleTest to repeat.
  2) initialSafetyTest:
       Error [QNT508]: Expect condition does not hold true
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task5-harness-stages/red/candidate_a_test.qnt:15:27
        15:   run initialSafetyTest = init.expect(coreTraceSafety and not(swapTerminal))
                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=initialSafetyTest to repeat.
  3) depositRecordsActualRequestTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task5-harness-stages/red/candidate_a_test.qnt:18:5
        18:     assert(changed.records.length() == 1 and changed.agreement.continuation == N5
                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        19:       and changed.records.head().request == requestFor(swapInitial, aliceFunding, Time1))
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=depositRecordsActualRequestTest to repeat.
  4) swapSettlementTraceTest:
       Error [QNT513]: Cannot continue in `then` because the highlighted expression evaluated to false
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task5-harness-stages/red/candidate_a_test.qnt:21:43
        21:   run swapSettlementTraceTest = init.then(depositAlice).then(depositBob).then(settleSwap)
                                                      ^^^^^^^^^^^^
    Use --seed=0x2a --match=swapSettlementTraceTest to repeat.
  5) swapVoluntaryRefundTraceTest:
       Error [QNT513]: Cannot continue in `then` because the highlighted expression evaluated to false
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task5-harness-stages/red/candidate_a_test.qnt:25:48
        25:   run swapVoluntaryRefundTraceTest = init.then(depositAlice).then(depositBob).then(refundSwap)
                                                           ^^^^^^^^^^^^
    Use --seed=0x2a --match=swapVoluntaryRefundTraceTest to repeat.
  6) swapAliceOnlyTimeoutTest:
       Error [QNT513]: Cannot continue in `then` because the highlighted expression evaluated to false
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task5-harness-stages/red/candidate_a_test.qnt:28:44
        28:   run swapAliceOnlyTimeoutTest = init.then(depositAlice).then(timeoutAlice)
                                                       ^^^^^^^^^^^^
    Use --seed=0x2a --match=swapAliceOnlyTimeoutTest to repeat.
  7) swapEmptyTimeoutTest:
       Error [QNT508]: Cannot continue to "expect"
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task5-harness-stages/red/candidate_a_test.qnt:31:30
        31:   run swapEmptyTimeoutTest = init.then(timeoutEmpty)
                                         ^^^^^^^^^^^^^^^^^^^^^^^
        32:     .expect(swapEmptyTimedOut and coreTraceSafety and swapTerminal
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        33:       and swapTrace.records.length() == 1 and swapTrace.ledger == initialLedger(10, 20))
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=swapEmptyTimeoutTest to repeat.
  8) swapFundedTimeoutCommitTest:
       Error [QNT513]: Cannot continue in `then` because the highlighted expression evaluated to false
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task5-harness-stages/red/candidate_a_test.qnt:34:47
        34:   run swapFundedTimeoutCommitTest = init.then(depositAlice).then(depositBob).then(timeoutFunded)
                                                          ^^^^^^^^^^^^
    Use --seed=0x2a --match=swapFundedTimeoutCommitTest to repeat.
  9) swapFundedTimeoutInputRollbackTest:
       Error [QNT513]: Cannot continue in `then` because the highlighted expression evaluated to false
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task5-harness-stages/red/candidate_a_test.qnt:37:54
        37:   run swapFundedTimeoutInputRollbackTest = init.then(depositAlice).then(depositBob).then(timeoutInput)
                                                                 ^^^^^^^^^^^^
    Use --seed=0x2a --match=swapFundedTimeoutInputRollbackTest to repeat.
  10) rejectedAttemptRetainedAfterCleanupTest:
       Error [QNT513]: Cannot continue in `then` because the highlighted expression evaluated to false
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task5-harness-stages/red/candidate_a_test.qnt:42:59
        42:   run rejectedAttemptRetainedAfterCleanupTest = init.then(depositAlice).then(depositBob)
                                                                      ^^^^^^^^^^^^
    Use --seed=0x2a --match=rejectedAttemptRetainedAfterCleanupTest to repeat.
  11) depositWithoutPaymentEffectTest:
       Error [QNT508]: Cannot continue to "expect"
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task5-harness-stages/red/candidate_a_test.qnt:46:41
        46:   run depositWithoutPaymentEffectTest = init.then(depositAlice).expect(aliceDeposited and coreTraceSafety
                                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        47:     and swapTrace.records.length() == 1 and match swapTrace.records.head().payload {
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        48:       | SwapComputedA(payload) => payload.raw.payments == List() and payload.raw.reductions == 0
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        49:           and payload.effects == List({source: Wallet(Alice), destination: Escrow(aliceA), asset: TokenA, quantity: 10})
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        50:       | _ => false
            ^^^^^^^^^^^^^^^^^^
        51:     })
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=depositWithoutPaymentEffectTest to repeat.
  12) rollbackRecordExactTest:
       Error [QNT513]: Cannot continue in `then` because the highlighted expression evaluated to false
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task5-harness-stages/red/candidate_a_test.qnt:52:43
        52:   run rollbackRecordExactTest = init.then(depositAlice).then(depositBob).then(timeoutInput).expect(
                                                      ^^^^^^^^^^^^
    Use --seed=0x2a --match=rollbackRecordExactTest to repeat.
  13) terminalHasNoEnabledActionTest:
       Error [QNT513]: Cannot continue in `then` because the highlighted expression evaluated to false
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task5-harness-stages/red/candidate_a_test.qnt:60:50
        60:   run terminalHasNoEnabledActionTest = init.then(depositAlice).then(depositBob).then(settleSwap)
                                                             ^^^^^^^^^^^^
    Use --seed=0x2a --match=terminalHasNoEnabledActionTest to repeat.
  14) rejectionOnlyCleanupEnabledTest:
       Error [QNT513]: Cannot continue in `then` because the highlighted expression evaluated to false
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task5-harness-stages/red/candidate_a_test.qnt:62:51
        62:   run rejectionOnlyCleanupEnabledTest = init.then(depositAlice).then(depositBob).then(timeoutInput).expect(
                                                              ^^^^^^^^^^^^
    Use --seed=0x2a --match=rejectionOnlyCleanupEnabledTest to repeat.
  15) wrongBobTimeDisabledTest:
       Error [QNT508]: Cannot continue to "expect"
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task5-harness-stages/red/candidate_a_test.qnt:65:34
        65:   run wrongBobTimeDisabledTest = init.then(depositAlice).expect(
                                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        66:     not(canRequest(swapTrace, bobFunding, Time1)) and canRequest(swapTrace, bobFunding, Time2))
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=wrongBobTimeDisabledTest to repeat.
  16) ledgerTamperDetectedTest:
       Error [QNT508]: Cannot continue to "expect"
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task5-harness-stages/red/candidate_a_test.qnt:67:34
        67:   run ledgerTamperDetectedTest = init.then(depositAlice).expect(not(traceSafety({...swapTrace,
                                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        68:     ledger: swapTrace.ledger.put((Wallet(Mallory), TokenA), 1)})))
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=ledgerTamperDetectedTest to repeat.
  17) tracePayloadTamperDetectedTest:
       Error [QNT508]: Cannot continue to "expect"
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task5-harness-stages/red/candidate_a_test.qnt:69:40
        69:   run tracePayloadTamperDetectedTest = init.then(depositAlice).expect(match swapTrace.records.head().payload {
                                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        70:     | SwapComputedA(payload) => {
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        71:         val original = swapTrace.records.head()
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        72:         val changedRaw = {...original, payload: SwapComputedA({...payload, raw: {...payload.raw, reductions: 1}})}
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        73:         val changedProjection = {...original, payload: SwapComputedA({...payload,
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        74:           projection: {...payload.projection, reductions: 1}})}
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        75:         val changedEffects = {...original, payload: SwapComputedA({...payload, effects: List()})}
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        76:         val changedRequest = {...original, request: {...original.request, input: bobFunding}}
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        77:         Set(changedRaw, changedProjection, changedEffects, changedRequest).forall(changed =>
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        78:           not(traceSafety({...swapTrace, records: List(changed)})))
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        79:       }
            ^^^^^^^
        80:     | _ => false
            ^^^^^^^^^^^^^^^^
        81:   })
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=tracePayloadTamperDetectedTest to repeat.
  18) diagnosticNeverWitnessOrTerminalTest:
       Error [QNT508]: Cannot continue to "expect"
        at /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-task5-harness-stages/red/candidate_a_test.qnt:82:46
        82:   run diagnosticNeverWitnessOrTerminalTest = init.then(depositAlice).expect(match swapTrace.records.head().payload {
                                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        83:     | SwapComputedA(payload) => {
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        84:         val original = swapTrace.records.head()
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        85:         val failedComputation = {...original, payload: SwapComputationDiagnosticA(TransactionReductionBoundFailureA)}
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        86:         val failedLedger = {...original, payload: SwapLedgerDiagnosticA(payload)}
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        87:         Set(failedComputation, failedLedger).forall(changed => {
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        88:           val bad = {...swapTrace, records: List(changed)}
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        89:           not(traceSafety(bad)) and not(traceTerminal(bad)) and not(recorded(bad, aliceFunding, Time1, N6, true))
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        90:         })
            ^^^^^^^^^^
        91:       }
            ^^^^^^^
        92:     | _ => false
            ^^^^^^^^^^^^^^^^
        93:   })
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=diagnosticNeverWitnessOrTerminalTest to repeat.


  Use --verbosity=3 to show executions.
  Further debug with: quint test --verbosity=3 .superpowers/sdd/candidate-a-task5-harness-stages/red/candidate_a_test.qnt
error: Tests failed
exit_code: 1
```

Experiment observation: one initial empty-trace/no-witness check passes. Eighteen fail against disabled guard/update/property stubs, including direct initial-schedule and actual-record assertions. Stateful tests fail because expected actions are disabled (QNT513/QNT508), not because imports/types are absent. These failures are scoped behavioral RED for the harness, not evaluator regressions.

During implementation, a typed lambda parameter caused a parse failure. Moving the annotation onto the fold's initial value corrected it without changing intended behavior. The complete terminal output from `quint test specs/quint/s02/candidate_a_test.qnt --backend=rust --seed=42` is preserved in `candidate-a-task5-harness-stages/implementation-parse-error.txt`; exit 1, compile-only development evidence.

## First GREEN and smoke execution

```text
$ quint test specs/quint/s02/candidate_a_test.qnt --backend=rust --seed=42

  candidate_a_test
    ok initialTraceEmptyTest passed 1 test(s)
    ok initialScheduleTest passed 1 test(s)
    ok initialSafetyTest passed 1 test(s)
    ok depositRecordsActualRequestTest passed 1 test(s)
    ok swapSettlementTraceTest passed 1 test(s)
    ok swapVoluntaryRefundTraceTest passed 1 test(s)
    ok swapAliceOnlyTimeoutTest passed 1 test(s)
    ok swapEmptyTimeoutTest passed 1 test(s)
    ok swapFundedTimeoutCommitTest passed 1 test(s)
    ok swapFundedTimeoutInputRollbackTest passed 1 test(s)
    ok rejectedAttemptRetainedAfterCleanupTest passed 1 test(s)
    ok depositWithoutPaymentEffectTest passed 1 test(s)
    ok rollbackRecordExactTest passed 1 test(s)
    ok terminalHasNoEnabledActionTest passed 1 test(s)
    ok rejectionOnlyCleanupEnabledTest passed 1 test(s)
    ok wrongBobTimeDisabledTest passed 1 test(s)
    ok ledgerTamperDetectedTest passed 1 test(s)
    ok tracePayloadTamperDetectedTest passed 1 test(s)
    ok diagnosticNeverWitnessOrTerminalTest passed 1 test(s)

  19 passing (1482ms)
exit_code: 0

$ quint run specs/quint/s02/candidate_a_harness.qnt --backend=rust --seed=42 --max-samples=1 --max-steps=1 --invariant=coreTraceSafety --verbosity=1
[ok] No violation found (80ms at 13 traces/second).
Use --seed=0x2a --backend=rust to reproduce.
exit_code: 0
```

The 19 tests include all six plan-named paths/properties, empty timeout, deadline-rejection cleanup, exact rejection record, terminal enabledness, Time2-only Bob funding, initial non-vacuity, and ledger/raw/projection/effect/request/diagnostic mutation controls. Constructed mutation controls are not additional executed Core scenarios.

## First bounded sampling run

```text
$ quint run specs/quint/s02/candidate_a_harness.qnt --backend=rust --seed=42 --max-samples=100 --max-steps=8 --invariant=coreTraceSafety --witnesses swapSettled swapVoluntaryRefunded swapDeadlineCommitted swapDeadlineInputRejected aliceDeposited bobDeposited swapAliceOnlyTimedOut swapEmptyTimedOut --verbosity=1
[ok] No violation found (773ms at 129 traces/second).
Witnesses:
swapSettled was witnessed in 2 trace(s) out of 100 explored (2.00%)
swapVoluntaryRefunded was witnessed in 5 trace(s) out of 100 explored (5.00%)
swapDeadlineCommitted was witnessed in 9 trace(s) out of 100 explored (9.00%)
swapDeadlineInputRejected was witnessed in 6 trace(s) out of 100 explored (6.00%)
aliceDeposited was witnessed in 48 trace(s) out of 100 explored (48.00%)
bobDeposited was witnessed in 16 trace(s) out of 100 explored (16.00%)
swapAliceOnlyTimedOut was witnessed in 32 trace(s) out of 100 explored (32.00%)
swapEmptyTimedOut was witnessed in 52 trace(s) out of 100 explored (52.00%)
Use --seed=0x79a --backend=rust to reproduce.
exit_code: 0
```

Experiment observation: no counterexample across these 100 sampled runs (maximum eight steps), with all eight major action witnesses reached. This is not exhaustive verification. Intended traces finish after one to four transaction actions; early stopping at terminal Close is by design, not hidden stuttering.

The command supplied seed 42. The tool itself printed reproduction seed 0x79a in the multi-sample output; both are preserved exactly rather than silently substituting one for the other.

## Concrete raw ITF reaching traces

The following commands deliberately negate desired witnesses. Their exit 1 is expected witness reachability, not a coreTraceSafety failure:

```text
$ quint run specs/quint/s02/candidate_a_harness.qnt --backend=rust --seed=42 --max-samples=100 --max-steps=8 --invariant='not(swapSettled)' --out-itf=.superpowers/sdd/candidate-a-task5-harness-stages/samples/settlement.itf.json --verbosity=1
[violation] Found an issue (127ms at 142 traces/second).
error: Invariant violated
exit_code: 1

$ quint run specs/quint/s02/candidate_a_harness.qnt --backend=rust --seed=42 --max-samples=100 --max-steps=8 --invariant='not(swapDeadlineInputRejected and swapTerminal)' --out-itf=.superpowers/sdd/candidate-a-task5-harness-stages/samples/rejection-cleanup.itf.json --verbosity=1
[violation] Found an issue (116ms at 69 traces/second).
error: Invariant violated
exit_code: 1
```

The complete raw ITF payloads are preserved, not replaced by the summary below. A first overly broad jq display was terminal-truncated, so the bounded inspection command below was used to inspect the relevant actual records completely. Neither jq command changes the files.

```text
$ jq '{states: (.states|length), finalRecords: [.states[-1].swapTrace.records[] | {before: .request.before.continuation.tag, input: .request.input, now: .request.now.tag, payload: .payload.tag, accepted: .payload.value.raw.accepted, error: .payload.value.raw.error, successor: .payload.value.raw.state.continuation.tag, reductions: .payload.value.raw.reductions, payments: [.payload.value.raw.payments[] | {recipient: .recipient.tag, asset: .asset.tag, quantity: .quantity}], effects: [.payload.value.effects[] | {source: .source.tag, destination: .destination.tag, asset: .asset.tag, quantity: .quantity}]}]}' .superpowers/sdd/candidate-a-task5-harness-stages/samples/settlement.itf.json .superpowers/sdd/candidate-a-task5-harness-stages/samples/rejection-cleanup.itf.json
{
  "states": 4,
  "finalRecords": [
    {
      "before": "N6",
      "input": {
        "tag": "PresentAInput",
        "value": {
          "tag": "DepositInputA",
          "value": {
            "account": {
              "asset": {
                "tag": "TokenA",
                "value": {
                  "#tup": []
                }
              },
              "owner": {
                "tag": "Alice",
                "value": {
                  "#tup": []
                }
              }
            },
            "depositor": {
              "tag": "Alice",
              "value": {
                "#tup": []
              }
            },
            "quantity": {
              "#bigint": "10"
            }
          }
        }
      },
      "now": "Time1",
      "payload": "SwapComputedA",
      "accepted": true,
      "error": {
        "tag": "NoCoreError",
        "value": {
          "#tup": []
        }
      },
      "successor": "N5",
      "reductions": {
        "#bigint": "0"
      },
      "payments": [],
      "effects": [
        {
          "source": "Wallet",
          "destination": "Escrow",
          "asset": "TokenA",
          "quantity": {
            "#bigint": "10"
          }
        }
      ]
    },
    {
      "before": "N5",
      "input": {
        "tag": "PresentAInput",
        "value": {
          "tag": "DepositInputA",
          "value": {
            "account": {
              "asset": {
                "tag": "TokenB",
                "value": {
                  "#tup": []
                }
              },
              "owner": {
                "tag": "Bob",
                "value": {
                  "#tup": []
                }
              }
            },
            "depositor": {
              "tag": "Bob",
              "value": {
                "#tup": []
              }
            },
            "quantity": {
              "#bigint": "20"
            }
          }
        }
      },
      "now": "Time2",
      "payload": "SwapComputedA",
      "accepted": true,
      "error": {
        "tag": "NoCoreError",
        "value": {
          "#tup": []
        }
      },
      "successor": "N4",
      "reductions": {
        "#bigint": "0"
      },
      "payments": [],
      "effects": [
        {
          "source": "Wallet",
          "destination": "Escrow",
          "asset": "TokenB",
          "quantity": {
            "#bigint": "20"
          }
        }
      ]
    },
    {
      "before": "N4",
      "input": {
        "tag": "PresentAInput",
        "value": {
          "tag": "ChoiceInputA",
          "value": {
            "chooser": {
              "tag": "Bob",
              "value": {
                "#tup": []
              }
            },
            "chosen": {
              "#bigint": "1"
            },
            "id": {
              "tag": "SettleId",
              "value": {
                "#tup": []
              }
            }
          }
        }
      },
      "now": "Time2",
      "payload": "SwapComputedA",
      "accepted": true,
      "error": {
        "tag": "NoCoreError",
        "value": {
          "#tup": []
        }
      },
      "successor": "N0",
      "reductions": {
        "#bigint": "3"
      },
      "payments": [
        {
          "recipient": "Bob",
          "asset": "TokenA",
          "quantity": {
            "#bigint": "10"
          }
        },
        {
          "recipient": "Alice",
          "asset": "TokenB",
          "quantity": {
            "#bigint": "20"
          }
        }
      ],
      "effects": [
        {
          "source": "Escrow",
          "destination": "Wallet",
          "asset": "TokenA",
          "quantity": {
            "#bigint": "10"
          }
        },
        {
          "source": "Escrow",
          "destination": "Wallet",
          "asset": "TokenB",
          "quantity": {
            "#bigint": "20"
          }
        }
      ]
    }
  ]
}
{
  "states": 5,
  "finalRecords": [
    {
      "before": "N6",
      "input": {
        "tag": "PresentAInput",
        "value": {
          "tag": "DepositInputA",
          "value": {
            "account": {
              "asset": {
                "tag": "TokenA",
                "value": {
                  "#tup": []
                }
              },
              "owner": {
                "tag": "Alice",
                "value": {
                  "#tup": []
                }
              }
            },
            "depositor": {
              "tag": "Alice",
              "value": {
                "#tup": []
              }
            },
            "quantity": {
              "#bigint": "10"
            }
          }
        }
      },
      "now": "Time1",
      "payload": "SwapComputedA",
      "accepted": true,
      "error": {
        "tag": "NoCoreError",
        "value": {
          "#tup": []
        }
      },
      "successor": "N5",
      "reductions": {
        "#bigint": "0"
      },
      "payments": [],
      "effects": [
        {
          "source": "Wallet",
          "destination": "Escrow",
          "asset": "TokenA",
          "quantity": {
            "#bigint": "10"
          }
        }
      ]
    },
    {
      "before": "N5",
      "input": {
        "tag": "PresentAInput",
        "value": {
          "tag": "DepositInputA",
          "value": {
            "account": {
              "asset": {
                "tag": "TokenB",
                "value": {
                  "#tup": []
                }
              },
              "owner": {
                "tag": "Bob",
                "value": {
                  "#tup": []
                }
              }
            },
            "depositor": {
              "tag": "Bob",
              "value": {
                "#tup": []
              }
            },
            "quantity": {
              "#bigint": "20"
            }
          }
        }
      },
      "now": "Time2",
      "payload": "SwapComputedA",
      "accepted": true,
      "error": {
        "tag": "NoCoreError",
        "value": {
          "#tup": []
        }
      },
      "successor": "N4",
      "reductions": {
        "#bigint": "0"
      },
      "payments": [],
      "effects": [
        {
          "source": "Wallet",
          "destination": "Escrow",
          "asset": "TokenB",
          "quantity": {
            "#bigint": "20"
          }
        }
      ]
    },
    {
      "before": "N4",
      "input": {
        "tag": "PresentAInput",
        "value": {
          "tag": "ChoiceInputA",
          "value": {
            "chooser": {
              "tag": "Bob",
              "value": {
                "#tup": []
              }
            },
            "chosen": {
              "#bigint": "1"
            },
            "id": {
              "tag": "SettleId",
              "value": {
                "#tup": []
              }
            }
          }
        }
      },
      "now": "Time100",
      "payload": "SwapComputedA",
      "accepted": false,
      "error": {
        "tag": "CoreErrorCode",
        "value": "contract_closed"
      },
      "successor": "N4",
      "reductions": {
        "#bigint": "0"
      },
      "payments": [],
      "effects": []
    },
    {
      "before": "N4",
      "input": {
        "tag": "NoAInput",
        "value": {
          "#tup": []
        }
      },
      "now": "Time100",
      "payload": "SwapComputedA",
      "accepted": true,
      "error": {
        "tag": "NoCoreError",
        "value": {
          "#tup": []
        }
      },
      "successor": "N0",
      "reductions": {
        "#bigint": "3"
      },
      "payments": [
        {
          "recipient": "Alice",
          "asset": "TokenA",
          "quantity": {
            "#bigint": "10"
          }
        },
        {
          "recipient": "Bob",
          "asset": "TokenB",
          "quantity": {
            "#bigint": "20"
          }
        }
      ],
      "effects": [
        {
          "source": "Escrow",
          "destination": "Wallet",
          "asset": "TokenA",
          "quantity": {
            "#bigint": "10"
          }
        },
        {
          "source": "Escrow",
          "destination": "Wallet",
          "asset": "TokenB",
          "quantity": {
            "#bigint": "20"
          }
        }
      ]
    }
  ]
}
exit_code: 0
```

Experiment observation: settlement has four states (init plus three transactions), with Bob10/TokenA then Alice20/TokenB payments. Rejection-cleanup has five states; its third transaction is rejected contract_closed with successor N4, reductions0, empty payments/effects. Its fourth transaction refunds Alice10/TokenA then Bob20/TokenB and reaches N0. Full choices, minimum time, program, projections, and ledgers remain in the ITF files.

## Final frozen-source verification

No source changes followed the first GREEN. Final separate typecheck, test, and bounded sampling rerun:

```text
$ quint typecheck specs/quint/s02/candidate_a_test.qnt
[no terminal output]
exit_code: 0

$ quint test specs/quint/s02/candidate_a_test.qnt --backend=rust --seed=42

  candidate_a_test
    ok initialTraceEmptyTest passed 1 test(s)
    ok initialScheduleTest passed 1 test(s)
    ok initialSafetyTest passed 1 test(s)
    ok depositRecordsActualRequestTest passed 1 test(s)
    ok swapSettlementTraceTest passed 1 test(s)
    ok swapVoluntaryRefundTraceTest passed 1 test(s)
    ok swapAliceOnlyTimeoutTest passed 1 test(s)
    ok swapEmptyTimeoutTest passed 1 test(s)
    ok swapFundedTimeoutCommitTest passed 1 test(s)
    ok swapFundedTimeoutInputRollbackTest passed 1 test(s)
    ok rejectedAttemptRetainedAfterCleanupTest passed 1 test(s)
    ok depositWithoutPaymentEffectTest passed 1 test(s)
    ok rollbackRecordExactTest passed 1 test(s)
    ok terminalHasNoEnabledActionTest passed 1 test(s)
    ok rejectionOnlyCleanupEnabledTest passed 1 test(s)
    ok wrongBobTimeDisabledTest passed 1 test(s)
    ok ledgerTamperDetectedTest passed 1 test(s)
    ok tracePayloadTamperDetectedTest passed 1 test(s)
    ok diagnosticNeverWitnessOrTerminalTest passed 1 test(s)

  19 passing (1290ms)
exit_code: 0

$ quint run specs/quint/s02/candidate_a_harness.qnt --backend=rust --seed=42 --max-samples=100 --max-steps=8 --invariant=coreTraceSafety --witnesses swapSettled swapVoluntaryRefunded swapDeadlineCommitted swapDeadlineInputRejected aliceDeposited bobDeposited swapAliceOnlyTimedOut swapEmptyTimedOut --verbosity=1
[ok] No violation found (765ms at 131 traces/second).
Witnesses:
swapSettled was witnessed in 2 trace(s) out of 100 explored (2.00%)
swapVoluntaryRefunded was witnessed in 5 trace(s) out of 100 explored (5.00%)
swapDeadlineCommitted was witnessed in 9 trace(s) out of 100 explored (9.00%)
swapDeadlineInputRejected was witnessed in 6 trace(s) out of 100 explored (6.00%)
aliceDeposited was witnessed in 48 trace(s) out of 100 explored (48.00%)
bobDeposited was witnessed in 16 trace(s) out of 100 explored (16.00%)
swapAliceOnlyTimedOut was witnessed in 32 trace(s) out of 100 explored (32.00%)
swapEmptyTimedOut was witnessed in 52 trace(s) out of 100 explored (52.00%)
Use --seed=0x79a --backend=rust to reproduce.
exit_code: 0
```

## Exact source/artifact hashes

This exact command ran before and after final typecheck/test/sampling. The complete terminal outputs were byte-identical:

```text
$ sha256sum specs/quint/s02/candidate_a_harness.qnt specs/quint/s02/candidate_a_test.qnt specs/quint/s02/candidate_a_projection.qnt specs/quint/s02/candidate_a_core.qnt specs/quint/s02/candidate_a_types.qnt specs/quint/s02/candidate_a_programs.qnt specs/quint/s02/effects.qnt specs/quint/s02/observations.qnt .superpowers/sdd/candidate-a-task5-harness-stages/red/*.qnt .superpowers/sdd/candidate-a-task5-harness-stages/samples/*.itf.json moriarty/core.py moriarty/swap.py /home/charl/.npm-global/bin/quint
0651108d66e40e0295f7ef568665257bee59b93300dae1f7dd6aad684ce8429c  specs/quint/s02/candidate_a_harness.qnt
7906400286288d3419ea22b48d424c68f21b5842fa8349ac156df26286077d25  specs/quint/s02/candidate_a_test.qnt
2eb66db8010a3d47ddba8f99d647326d2eb7bf3c9da5177c7dd2dcd88145452c  specs/quint/s02/candidate_a_projection.qnt
e759d4c13032d83a4d839e08bef0fcf8272943792f73bb3ab7910357b0f28dec  specs/quint/s02/candidate_a_core.qnt
40901738fd1749527761b624a84253da94bbc24c209d408dc2fd4e895daaae0b  specs/quint/s02/candidate_a_types.qnt
b443fc0beec22b867e4d36c21714fbaee63950961776d21b2ebe9463e190ab66  specs/quint/s02/candidate_a_programs.qnt
dccc9208d9a42f01685000b1358f1d2f9684bf5e8ebe1d5deadd0c992557504c  specs/quint/s02/effects.qnt
e44484ed9035e560ef9f4d012198671917750da42ab404022592db16cdd59bc3  specs/quint/s02/observations.qnt
e759d4c13032d83a4d839e08bef0fcf8272943792f73bb3ab7910357b0f28dec  .superpowers/sdd/candidate-a-task5-harness-stages/red/candidate_a_core.qnt
8bd5208437697b25facc611500c893be51bdc6e245013d7a1fa38497602b4528  .superpowers/sdd/candidate-a-task5-harness-stages/red/candidate_a_harness.qnt
b443fc0beec22b867e4d36c21714fbaee63950961776d21b2ebe9463e190ab66  .superpowers/sdd/candidate-a-task5-harness-stages/red/candidate_a_programs.qnt
2eb66db8010a3d47ddba8f99d647326d2eb7bf3c9da5177c7dd2dcd88145452c  .superpowers/sdd/candidate-a-task5-harness-stages/red/candidate_a_projection.qnt
7906400286288d3419ea22b48d424c68f21b5842fa8349ac156df26286077d25  .superpowers/sdd/candidate-a-task5-harness-stages/red/candidate_a_test.qnt
40901738fd1749527761b624a84253da94bbc24c209d408dc2fd4e895daaae0b  .superpowers/sdd/candidate-a-task5-harness-stages/red/candidate_a_types.qnt
dccc9208d9a42f01685000b1358f1d2f9684bf5e8ebe1d5deadd0c992557504c  .superpowers/sdd/candidate-a-task5-harness-stages/red/effects.qnt
e44484ed9035e560ef9f4d012198671917750da42ab404022592db16cdd59bc3  .superpowers/sdd/candidate-a-task5-harness-stages/red/observations.qnt
ea1e95f9932fe8aeacffb5d351c18266ae80f0f3224876f0ffa99bacc7a9fab9  .superpowers/sdd/candidate-a-task5-harness-stages/samples/rejection-cleanup.itf.json
c1cb048b3c93df49513629a6ba46ff786113634894a084dc99d485619506801d  .superpowers/sdd/candidate-a-task5-harness-stages/samples/settlement.itf.json
564a926779beb54bd16ff58481f5c950bae6fb50fcd5b0d7c9eb56427f83273b  moriarty/core.py
82e9e1da76af65706171049f0238953aca5333edec4aeed6caa43dbd57c79797  moriarty/swap.py
ac12595b1cb7253feec93c79417615c6eb20fc3b6a3df35c5e3530b24e90a501  /home/charl/.npm-global/bin/quint
exit_code: 0
before_output_equals_after_output: true
```

## Handoff and open scope

Two new live harness/test files and the RED/ITF artifacts are frozen for root's independent tests and source/evidence review. No commit was made. No full suite or model checker ran in this worker. Independent Python extraction/correspondence, installment agreement traces, authority integration, Candidate A completion, and all acceptance/Council gates remain open.
