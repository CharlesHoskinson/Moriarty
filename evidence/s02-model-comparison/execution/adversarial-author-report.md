# Execution evidence adversarial test authorship

Date: 2026-09-05. Classification: repository observation and experiment observation.
Scope: the public `canVerify` and `canCommit` guards in the S02 common execution
boundary. This is test authorship, not an independent review or Council gate.

## Owned change

Added `specs/quint/s02/execution_adversarial_test.qnt`, initially 42 deterministic
`run` tests. The file imports the public modules and concrete harness. It does not
modify an implementation guard or generate cryptographic or effect evidence.
`EvidenceValid` abstracts an external verifier result. No candidate interpreter,
semantic correspondence, cryptographic security, or evidence-generation claim
follows from these tests.

Positive controls require both signing profiles to verify settlement and a
verified settlement to admit commitment. Negative cases cover absent, unavailable,
and invalid evidence; each omitted debit owner; signature token, signed policy,
and signer substitution; all attempt-binding fields; fresh evidence about a plan
that exceeds signed bounds; rejected Core projections; consumed generic swap
authority; and stale candidate, ledger, registry, or environment after verification.
Attempt substitutions independently affect the effect proof and each required
signature proof while retaining the original proposal. All environment fields
are tested separately.

Parent tests use explicitly constructed contexts. Positive controls check signing
and cancellation verification, then a current consumed parent with a valid
first-fill residual permitting second-fill verification. Negative cases cover
effects-free cancellation without the exact parent signature, invalid parent
signature, an invented Core projection, revision mismatch, first-slot replay,
and retained-parent token replacement. These contexts are not evidence of an
executed fill, cancellation, recovery, or complete signing lifecycle.

The proposal actor and the policy signer are distinct: a Bob proposal without
Alice's parent signature rejects; the positive fixture supplies Alice's signed
parent. These tests do not invent an actor-equals-signer requirement.

## Initial validation

Tool: `quint --version` returned `0.32.0`.

The first command was `quint typecheck specs/quint/s02/execution_adversarial_test.qnt`.
Its initial tool response had no output and an ongoing session identifier that
was not retained. The process was confirmed running by process inspection and
later exited. Its terminal status is unknown; no typecheck success is claimed
from that invocation.

The scoped test command is:

```text
quint test specs/quint/s02/execution_adversarial_test.qnt --main execution_adversarial_test --match 'Test$' --seed 0x502
```

Initial result: exit 1, 41 passing and 1 failing. All positive fixture controls
passed. The only failure was `cancellationCannotInventCoreProjectionTest`: a
freshly bound, accepted Core projection was admitted for an effects-free
cancellation. This contradicts the adopted lifecycle-only cancellation boundary.
The failure was immediately reported to the root implementation owner, who will
decide and implement the guard correction. This author made no source correction.

Captured test output (exit code 1):

```text

  execution_adversarial_test
    ok positiveEvidenceFixturesTest passed 1 test(s)
    ok positiveCommitFixtureTest passed 1 test(s)
    ok unavailableEffectProofTest passed 1 test(s)
    ok invalidEffectProofTest passed 1 test(s)
    ok omittedDebitOwnerTest passed 1 test(s)
    ok noSignatureProofsTest passed 1 test(s)
    ok unavailableSignatureProofTest passed 1 test(s)
    ok invalidSignatureProofTest passed 1 test(s)
    ok swappedSignatureTokenTest passed 1 test(s)
    ok swappedSignedPolicyTest passed 1 test(s)
    ok swappedSignerTest passed 1 test(s)
    ok actorSubstitutionTest passed 1 test(s)
    ok attemptIdSubstitutionTest passed 1 test(s)
    ok operationSubstitutionTest passed 1 test(s)
    ok inputSubstitutionTest passed 1 test(s)
    ok planSubstitutionTest passed 1 test(s)
    ok artifactSubstitutionTest passed 1 test(s)
    ok wholeObservationSubstitutionTest passed 1 test(s)
    ok predecessorSubstitutionTest passed 1 test(s)
    ok successorSubstitutionTest passed 1 test(s)
    ok effectsSubstitutionTest passed 1 test(s)
    ok displaySubstitutionTest passed 1 test(s)
    ok stateContextSubstitutionTest passed 1 test(s)
    ok ledgerContextSubstitutionTest passed 1 test(s)
    ok environmentContextSubstitutionTest passed 1 test(s)
    ok observationEffectUnavailableTest passed 1 test(s)
    ok observationEffectInvalidTest passed 1 test(s)
    ok freshEvidenceCannotExpandSignedAfterPlanTest passed 1 test(s)
    ok coreRejectedProjectionTest passed 1 test(s)
    ok consumedGenericSwapAuthorityTest passed 1 test(s)
    ok staleCandidateAfterVerificationTest passed 1 test(s)
    ok staleLedgerAfterVerificationTest passed 1 test(s)
    ok staleEnvironmentAfterVerificationTest passed 1 test(s)
    ok staleRegistryAfterVerificationTest passed 1 test(s)
    ok constructedParentCancellationPreconditionsTest passed 1 test(s)
    ok unauthorizedNoEffectCancellationTest passed 1 test(s)
    ok cancellationInvalidParentSignatureTest passed 1 test(s)
    1) cancellationCannotInventCoreProjectionTest failed after 1 test(s)
    ok constructedCurrentParentResidualPreconditionsTest passed 1 test(s)
    ok staleParentNonceRevisionTest passed 1 test(s)
    ok reusedFirstSlotWithCurrentResidualTest passed 1 test(s)
    ok substitutedRetainedParentTokenTest passed 1 test(s)

  41 passing (59406ms)
  1 failed

  1) cancellationCannotInventCoreProjectionTest:
       Error [QNT508]: Assertion failed
        at /home/charl/Moriarty/.worktrees/s01-audit-start/specs/quint/s02/execution_adversarial_test.qnt:167:5
        167:     assert(not(canVerify(withAttempt(cancelProposed, other), CancelAttempt, parentEvidence(other))))
                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Use --seed=0x502 --match=cancellationCannotInventCoreProjectionTest to repeat.


  Use --verbosity=3 to show executions.
  Further debug with: quint test --verbosity=3 specs/quint/s02/execution_adversarial_test.qnt
error: Tests failed
```

Final scoped validation is recorded below. No full suite, simulation, or model checking was run
by this author. The 42-case test source is preserved unchanged from the failing run.

Repository snapshot observed during the initial test:

```text
HEAD 194af05e5d42c20fdbe3d0c9650ebb8ce89f8686
execution.qnt sha256 4a04dd5222ad5ca5577d76dcba6fa9234482326b5cf9b9aca254686dc8662a54
execution_adversarial_test.qnt sha256 e4799beb38a7f0cc0ce0822fb852fce95c1a53d234c22286f827fddcadf962aa
```

The root agent owns shared implementation and harness changes. This author did
not stage, commit, move HEAD, modify manifests, or edit any shared source file.

## Rerun after source correction

Repository observation: the root owner changed `projectionAdmitsExecution` to
accept an operation and require `NoCoreProjection` for `OpCancelParent`. Other
operations retain the existing accepted-result rule. The test file is unchanged
from the failing run, as its SHA-256 confirms.

The root requested this explicit backend and seed for the final scoped command:

```text
quint test specs/quint/s02/execution_adversarial_test.qnt --backend=rust --seed=42
```

Source snapshot immediately before this command:

```text
HEAD 523ee1471aecb04cc8fb843245fcb5b238904ee5
execution.qnt sha256 55a52f037d83f426385966685f244a4814b71eb59f7cda79f7a47f2ec45feaed
execution_harness.qnt sha256 b548c4a4d0d2189b710ce28c6b383867deaa968ae0cf00049cc2bbf550149f5d
execution_adversarial_test.qnt sha256 e4799beb38a7f0cc0ce0822fb852fce95c1a53d234c22286f827fddcadf962aa
```

Experiment observation: terminal exit 0, all 42 tests pass. All three source
hashes above were rechecked after completion and remained identical. The test
source was unchanged between the RED and GREEN runs. No additional standalone
typecheck was run: the first invocation's exit is unknown, and the scoped test
command typechecks its imports. This author also ran `git diff --check --
specs/quint/s02/execution_adversarial_test.qnt`; it emitted no diagnostics.

Captured GREEN output (exit code 0):

```text

  execution_adversarial_test
    ok positiveEvidenceFixturesTest passed 1 test(s)
    ok positiveCommitFixtureTest passed 1 test(s)
    ok unavailableEffectProofTest passed 1 test(s)
    ok invalidEffectProofTest passed 1 test(s)
    ok omittedDebitOwnerTest passed 1 test(s)
    ok noSignatureProofsTest passed 1 test(s)
    ok unavailableSignatureProofTest passed 1 test(s)
    ok invalidSignatureProofTest passed 1 test(s)
    ok swappedSignatureTokenTest passed 1 test(s)
    ok swappedSignedPolicyTest passed 1 test(s)
    ok swappedSignerTest passed 1 test(s)
    ok actorSubstitutionTest passed 1 test(s)
    ok attemptIdSubstitutionTest passed 1 test(s)
    ok operationSubstitutionTest passed 1 test(s)
    ok inputSubstitutionTest passed 1 test(s)
    ok planSubstitutionTest passed 1 test(s)
    ok artifactSubstitutionTest passed 1 test(s)
    ok wholeObservationSubstitutionTest passed 1 test(s)
    ok predecessorSubstitutionTest passed 1 test(s)
    ok successorSubstitutionTest passed 1 test(s)
    ok effectsSubstitutionTest passed 1 test(s)
    ok displaySubstitutionTest passed 1 test(s)
    ok stateContextSubstitutionTest passed 1 test(s)
    ok ledgerContextSubstitutionTest passed 1 test(s)
    ok environmentContextSubstitutionTest passed 1 test(s)
    ok observationEffectUnavailableTest passed 1 test(s)
    ok observationEffectInvalidTest passed 1 test(s)
    ok freshEvidenceCannotExpandSignedAfterPlanTest passed 1 test(s)
    ok coreRejectedProjectionTest passed 1 test(s)
    ok consumedGenericSwapAuthorityTest passed 1 test(s)
    ok staleCandidateAfterVerificationTest passed 1 test(s)
    ok staleLedgerAfterVerificationTest passed 1 test(s)
    ok staleEnvironmentAfterVerificationTest passed 1 test(s)
    ok staleRegistryAfterVerificationTest passed 1 test(s)
    ok constructedParentCancellationPreconditionsTest passed 1 test(s)
    ok unauthorizedNoEffectCancellationTest passed 1 test(s)
    ok cancellationInvalidParentSignatureTest passed 1 test(s)
    ok cancellationCannotInventCoreProjectionTest passed 1 test(s)
    ok constructedCurrentParentResidualPreconditionsTest passed 1 test(s)
    ok staleParentNonceRevisionTest passed 1 test(s)
    ok reusedFirstSlotWithCurrentResidualTest passed 1 test(s)
    ok substitutedRetainedParentTokenTest passed 1 test(s)

  42 passing (73893ms)
```
