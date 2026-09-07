# Stateful installment lifecycle implementation

Date: 2026-09-05. Classification: repository observation, experiment observation,
and bounded implementation design. This is implementation work, not an independent
review or a Council gate.

## Scope and shape

Owned files are the new `installment_fixtures.qnt`, `installment_harness.qnt`,
`installment_test.qnt`, and this scratch report. The shared execution and rejection
boundaries belong to the root implementation owner. No shared source, manifest,
commit, or HEAD changes are made by this author.

The harness uses shared state: one `ExecutionState[str,str,str,str]` and a fixed
signing-profile variable. Both profiles initialize unsigned, with escrow directly
prefunded with ten TokenA units, environment time 2, all authority keys unused,
all parent cells vacant, and all attempts absent. This is the declared prefunded
fixture, not a funding execution trace.

The after-resolution parent plan includes fill one, fill two, cancellation before
any fill, and cancellation after fill one. Both signing profiles share the same
canonical parent body. Plan predecessor facts include the exact ledger and parent
facts corresponding to each branch. The opaque plan identity is equality-bound;
these fixtures do not establish adapter fidelity.

Required lifecycle: prepare/sign the parent, propose and verify both first-fill
and initial cancellation against revision zero, commit one winner, explicitly
reject the stale race loser, then either finish slot two or separately prepare,
sign, verify, and commit recovery of ten or five under nonce one after cancellation.
Each action uses the corresponding public guard and pure update. No blanket
stutter is intended. Nonterminal enabledness must use exactly the actual action
guards. Successful terminality requires completed payment or executed recovery
and no proposed or verified attempts remaining.

`EvidenceValid` in fresh evidence constructors abstracts an external verifier
result. The harness does not implement cryptography, derive effects, interpret
Core, instantiate A–D candidates, or prove semantic correspondence. All final
simulation claims will be limited to the sampled traces and exact parameters.

## Development validation

Development-only bootstrap command:
`quint test specs/quint/s02/installment_test.qnt --backend=rust --seed=42`.
It exited 1 because the newly referenced harness did not exist (QNT013, QNT405,
followed by unresolved exported-name diagnostics). This is an expected scaffold
error, not a behavioral regression result.

The first behavioral RED used disabled, type-correct action scaffolding:

```text
quint test specs/quint/s02/installment_test.qnt --backend=rust --seed=42 --match=prepareParentAfterTest
```

Terminal exit: 1. Full terminal output:

```text

  installment_test
    1) prepareParentAfterTest failed after 1 test(s)

  1 failed

  1) prepareParentAfterTest:
       Error [QNT508]: Cannot continue to "expect"
        at /home/charl/Moriarty/.worktrees/s01-audit-start/specs/quint/s02/installment_test.qnt:4:32
        4:   run prepareParentAfterTest = initAfter.then(prepareParent).expect(parentChecked)
                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=prepareParentAfterTest to repeat.


  Use --verbosity=3 to show executions.
  Further debug with: quint test --verbosity=3 specs/quint/s02/installment_test.qnt
error: Tests failed
```

This failed because `prepareParent` was deliberately disabled, before its
implementation. Shared rejection source was being developed in parallel; this
early run is development-only and is not presented as a final frozen receipt.

Stage-one owned-source hashes (parent signing and initial race only):

```text
installment_fixtures.qnt d29f9c9b00f686d42d0084e0ab01fcc3b53a6b5cdad28434840da7405f72ff0d
installment_harness.qnt 7ae190a1fd6fd81c6562e0cf7d9ff028d0aecb9b2c9c7568b22b05f118779fa2
installment_test.qnt 605cae9fc029fbd7da51565d69734bc18559eaff02a5fed7765affb1a2a72d2c
```

Stage-one typecheck command `quint typecheck specs/quint/s02/installment_test.qnt`
exited 0 with empty output. The scoped test command was
`quint test specs/quint/s02/installment_test.qnt --backend=rust --seed=42`;
it exited 0 with this terminal output:

```text

  installment_test
    ok prepareParentAfterTest passed 1 test(s)
    ok prepareParentBeforeTest passed 1 test(s)
    ok signParentAfterTest passed 1 test(s)
    ok signParentBeforeTest passed 1 test(s)
    ok prepareRaceAfterTest passed 1 test(s)
    ok prepareRaceBeforeTest passed 1 test(s)

  6 passing (11899ms)
```

Development sampled-run command:

```text
quint run specs/quint/s02/installment_harness.qnt --backend=rust --seed=42 --max-samples=20 --max-steps=8 --invariant=setupSafety --witnesses parentChecked parentRegistered firstFillProposed initialCancellationProposed firstFillVerified initialCancellationVerified racePrepared --verbosity=1
```

Exit 0, full output:

```text
[ok] No violation found (2956ms at 7 traces/second).
Witnesses:
parentChecked was witnessed in 20 trace(s) out of 20 explored (100.00%)
parentRegistered was witnessed in 20 trace(s) out of 20 explored (100.00%)
firstFillProposed was witnessed in 20 trace(s) out of 20 explored (100.00%)
initialCancellationProposed was witnessed in 20 trace(s) out of 20 explored (100.00%)
firstFillVerified was witnessed in 20 trace(s) out of 20 explored (100.00%)
initialCancellationVerified was witnessed in 20 trace(s) out of 20 explored (100.00%)
racePrepared was witnessed in 20 trace(s) out of 20 explored (100.00%)
Use --seed=0x36e --backend=rust to reproduce.
```

All seven initial-action targets were reached in all twenty traces. The invariant
at this increment checked valid state and unchanged prefunded money only. This
was development sampling, not a safety confirmation or full lifecycle evidence.
## Second behavioral increment

The second behavioral RED command was:

```text
quint test specs/quint/s02/installment_test.qnt --backend=rust --seed=42 --match=cancellationWinsRecoveryTenAfterTest
```

Exit 1, full output:

```text

  installment_test
    1) cancellationWinsRecoveryTenAfterTest failed after 1 test(s)

  1 failed

  1) cancellationWinsRecoveryTenAfterTest:
       Error [QNT513]: Cannot continue in `then` because the highlighted expression evaluated to false
        at /home/charl/Moriarty/.worktrees/s01-audit-start/specs/quint/s02/installment_test.qnt:30:11
        30:     .then(commitInitialCancellation).then(rejectFirstFill)
                      ^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x2a --match=cancellationWinsRecoveryTenAfterTest to repeat.


  Use --verbosity=3 to show executions.
  Further debug with: quint test --verbosity=3 specs/quint/s02/installment_test.qnt
error: Tests failed
```

The trace reached the prepared race, then failed at the deliberately disabled
`commitInitialCancellation` scaffold. The subsequent implementation adds all
winner, loser-rejection, successor-branch, and nonce-one recovery actions.

A subsequent typecheck found a local naming error: `enabled` is a Quint built-in
(QNT101 at installment_harness.qnt:135). The witness alias was renamed
`installmentEnabled`; no guard or behavior changed in that correction.

The full invariant now combines valid execution state (including exact
registry/parent revision coupling), conservation of every asset, parent payments
and escrow residuals, preservation of cancelled parent state through recovery,
original evidence retained in classified race rejection records, explicit
terminality, and the exact disjunction of actual pure action guards. Slot two and
fresh cancellation are alternate successor proposals after the initial race
loser is resolved; an additional concurrent slot-two race is outside this fixture.
## Full lifecycle validation snapshot

Final-unit typecheck command:

```text
quint typecheck specs/quint/s02/installment_test.qnt
```

Exit 0 with empty output. The following complete import/source hashes were
captured at the start of the full test and sampled-run commands:

```text
HEAD 4f3bb750927187f3b465cc4418993e5df9f152bf
dccc9208d9a42f01685000b1358f1d2f9684bf5e8ebe1d5deadd0c992557504c  specs/quint/s02/effects.qnt
3dd07f3c5f274aff4fce4de9c245aa0b1f1fe7f68e29e5de95a0b1c42a00f93b  specs/quint/s02/consumption.qnt
e44484ed9035e560ef9f4d012198671917750da42ab404022592db16cdd59bc3  specs/quint/s02/observations.qnt
0886619203c3de16f2326e27e9d4ae2ddfda8c54742c231dd4e824397c5fa9fe  specs/quint/s02/policies.qnt
00b8867f566035a5d1ac16de5155d98c58a2f2e63a481b5900f8653015481c09  specs/quint/s02/policies_harness.qnt
b59779d5e2e7f1bf0a14952dfe1cd3ab70bcfe813210d690abbb3e3205a6df61  specs/quint/s02/authorization.qnt
cf52f55ad1810548b7c45e32552b977a30fc8dd937dfbd01d35829b6beec8928  specs/quint/s02/execution.qnt
e6c54da829a67acef0c9359071363c66924e7ed4c17489fbeaea0281af39f10c  specs/quint/s02/installment_fixtures.qnt
a0ed856d8e9f1ccd6f7fb0408fde270da341aba4c9a311cf1285d561726920da  specs/quint/s02/installment_harness.qnt
0609a869541454f772e7fcfa2b847b466b6241b38e9e28d08b3ec040b32a1af1  specs/quint/s02/installment_test.qnt
```

The twelve-test command is:

```text
quint test specs/quint/s02/installment_test.qnt --backend=rust --seed=42
```

The bounded sampled-run command is:

```text
quint run specs/quint/s02/installment_harness.qnt --backend=rust --seed=42 --max-samples=100 --max-steps=20 --invariant=installmentSafety --witnesses parentChecked parentRegistered firstFillProposed initialCancellationProposed firstFillVerified initialCancellationVerified racePrepared firstFillCommitted initialCancellationCommitted firstFillRejected initialCancellationRejected secondFillProposed secondFillVerified secondFillCommitted freshCancellationProposed freshCancellationVerified freshCancellationCommitted recoveryChecked recoveryRegistered recoveryProposed recoveryVerified recoveryCommitted completedPaymentAfter completedPaymentBefore recoveryTenAfter recoveryTenBefore recoveryFiveAfter recoveryFiveBefore --verbosity=1
```

The sample bound was chosen for an initial full-lifecycle execution check and
witness coverage. It is not an exhaustive check, a safety confirmation, or a
liveness proof. No model checker or unrelated full suite is run by this author.
Final test exit 0, full terminal output:

```text

  installment_test
    ok prepareParentAfterTest passed 1 test(s)
    ok prepareParentBeforeTest passed 1 test(s)
    ok signParentAfterTest passed 1 test(s)
    ok signParentBeforeTest passed 1 test(s)
    ok prepareRaceAfterTest passed 1 test(s)
    ok prepareRaceBeforeTest passed 1 test(s)
    ok completedPaymentAfterTest passed 1 test(s)
    ok completedPaymentBeforeTest passed 1 test(s)
    ok cancellationWinsRecoveryTenAfterTest passed 1 test(s)
    ok cancellationWinsRecoveryTenBeforeTest passed 1 test(s)
    ok fillWinsRecoveryFiveAfterTest passed 1 test(s)
    ok fillWinsRecoveryFiveBeforeTest passed 1 test(s)

  12 passing (36639ms)
```

Sampled-run exit 0, full terminal output:

```text
[ok] No violation found (27238ms at 4 traces/second).
Witnesses:
parentChecked was witnessed in 100 trace(s) out of 100 explored (100.00%)
parentRegistered was witnessed in 100 trace(s) out of 100 explored (100.00%)
firstFillProposed was witnessed in 100 trace(s) out of 100 explored (100.00%)
initialCancellationProposed was witnessed in 100 trace(s) out of 100 explored (100.00%)
firstFillVerified was witnessed in 100 trace(s) out of 100 explored (100.00%)
initialCancellationVerified was witnessed in 100 trace(s) out of 100 explored (100.00%)
racePrepared was witnessed in 100 trace(s) out of 100 explored (100.00%)
firstFillCommitted was witnessed in 49 trace(s) out of 100 explored (49.00%)
initialCancellationCommitted was witnessed in 51 trace(s) out of 100 explored (51.00%)
firstFillRejected was witnessed in 51 trace(s) out of 100 explored (51.00%)
initialCancellationRejected was witnessed in 49 trace(s) out of 100 explored (49.00%)
secondFillProposed was witnessed in 21 trace(s) out of 100 explored (21.00%)
secondFillVerified was witnessed in 21 trace(s) out of 100 explored (21.00%)
secondFillCommitted was witnessed in 21 trace(s) out of 100 explored (21.00%)
freshCancellationProposed was witnessed in 28 trace(s) out of 100 explored (28.00%)
freshCancellationVerified was witnessed in 28 trace(s) out of 100 explored (28.00%)
freshCancellationCommitted was witnessed in 28 trace(s) out of 100 explored (28.00%)
recoveryChecked was witnessed in 79 trace(s) out of 100 explored (79.00%)
recoveryRegistered was witnessed in 79 trace(s) out of 100 explored (79.00%)
recoveryProposed was witnessed in 79 trace(s) out of 100 explored (79.00%)
recoveryVerified was witnessed in 79 trace(s) out of 100 explored (79.00%)
recoveryCommitted was witnessed in 79 trace(s) out of 100 explored (79.00%)
completedPaymentAfter was witnessed in 11 trace(s) out of 100 explored (11.00%)
completedPaymentBefore was witnessed in 10 trace(s) out of 100 explored (10.00%)
recoveryTenAfter was witnessed in 23 trace(s) out of 100 explored (23.00%)
recoveryTenBefore was witnessed in 28 trace(s) out of 100 explored (28.00%)
recoveryFiveAfter was witnessed in 11 trace(s) out of 100 explored (11.00%)
recoveryFiveBefore was witnessed in 17 trace(s) out of 100 explored (17.00%)
Use --seed=0x75d5 --backend=rust to reproduce.
```

Experiment observation: all twelve deterministic tests pass; all twenty-eight
witnesses are reached; no invariant violation occurs in these one hundred
sampled traces. The six terminal outcome counts sum to one hundred. This is
bounded empirical evidence, not proof over all states, cryptographic verification,
Core correspondence, or A–D feasibility. All ten source/import hashes above were
rechecked after completion and remained identical.

## Outcome coverage and limits

The following three outcomes are exercised as actual action traces under both
signing profiles:

| Outcome | Alice wallet | Bob wallet | Escrow | Parent revision / paid / remaining | Recovery nonce one |
| --- | ---: | ---: | ---: | --- | --- |
| Both fills complete | 0 | 10 | 0 | 2 / 10 / 0, not cancelled | unused |
| Initial cancellation, recovery ten | 10 | 0 | 0 | 1 / 0 / 10, cancelled | consumed at revision 1 |
| First fill, fresh cancellation, recovery five | 5 | 5 | 0 | 2 / 5 / 5, cancelled | consumed at revision 1 |

The retained parent allowance remains ten or five after recovery; financial
completion is established by the executed recovery and ledger, while the parent
entry and nonce-zero registry record remain unchanged. The original losing
verified attempt remains in a `RejectedOperation` record with its original
evidence, observed context, `StaleBindings` reason, and `CommitBoundary` stage.

The harness models the specified initial race and a single selected successor
branch. It does not model another slot-two/fresh-cancellation race, funding
transactions, environment evolution, signature unavailability, malicious adapters,
candidate resolution interpreters, exhaustive model checking, or broader witness
inventory acceptance. No unrelated source files or regression suites were changed
or rerun by this author.

