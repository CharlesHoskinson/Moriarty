# S02 installment adversarial test report

Status: completed targeted Quint test authorship. This report is an experiment
observation for the scoped test module only. It does not claim candidate
semantics, cryptographic proof validity, a Council result, recovery evidence,
or an S02 gate result.

## Scope

Owned files changed:

- `specs/quint/s02/installment_adversarial_test.qnt`
- `.superpowers/sdd/installment-adversarial-report.md`

No shared source, fixture, harness, existing test, manifest, index, or commit
was changed by this task.

The new module uses reachable harness actions to check both signing profiles:

- cancellation wins before payment, leaves escrow at 10, and prevents the
  losing fill from committing;
- first fill wins, prevents the losing cancellation from committing, and enables
  the exact residual slot-two action only after stale-loser rejection;
- recovery is unavailable before cancellation, uses the distinct recovery key
  with nonce 1, and remains unused after checking but before signing;
- completed recovery cannot replay.

Separately labeled constructed mutations check recovery verification guards for
missing proof, invalid proof, wrong refund amount, and wrong refund recipient.
They are not lifecycle traces. `EvidenceValid` remains a trusted external
verifier abstraction.

## Test-construction correction

The first authored version asserted `canProposeSecondFill` immediately after
`commitFirstFill`. Focused inspection found that the harness deliberately keeps
the verified initial cancellation pending until `rejectInitialCancellation`.
The assertion was moved after that reachable rejection action. The
wrong-recipient mutation was also corrected to preserve the attempted refund
quantity, so a future five-unit path does not conflate recipient and amount.
These were test-construction corrections, not source-model defects.

## Stable source pins

Before authoring and before the final run:

```text
HEAD 7e5697fa50ad6800ec84d44b108752e4055fb89e
installment_fixtures.qnt e6c54da829a67acef0c9359071363c66924e7ed4c17489fbeaea0281af39f10c
installment_harness.qnt a0ed856d8e9f1ccd6f7fb0408fde270da341aba4c9a311cf1285d561726920da
execution.qnt cf52f55ad1810548b7c45e32552b977a30fc8dd937dfbd01d35829b6beec8928
```

After the final run, the three shared source hashes and HEAD were identical.
The authored test hash was:

```text
installment_adversarial_test.qnt fd4f4abf92f1622d6077de227dd99de523441e798f3bf5ca34121fe9001f20c1
```

## Final Rust run

Command:

```text
quint test specs/quint/s02/installment_adversarial_test.qnt --backend=rust --seed=42
```

Exit code: `0`

Full terminal output:

```text
  installment_adversarial_test
    ok cancellationWinnerAfterGuardTest passed 1 test(s)
    ok cancellationWinnerBeforeGuardTest passed 1 test(s)
    ok fillWinnerAfterGuardTest passed 1 test(s)
    ok fillWinnerBeforeGuardTest passed 1 test(s)
    ok recoveryUnavailableBeforeCancellationAfterTest passed 1 test(s)
    ok recoveryUnavailableBeforeCancellationBeforeTest passed 1 test(s)
    ok recoveryNonceUnusedUntilSigningTest passed 1 test(s)
    ok recoveryProofGuardsTest passed 1 test(s)
    ok recoveredAuthorityCannotReplayTest passed 1 test(s)

  9 passing (26036ms)
```

`git diff --check -- specs/quint/s02/installment_adversarial_test.qnt` produced
no output and exited successfully.
