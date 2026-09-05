# Candidate A authority adapter takeover

Date: 2026-09-05. Scope: canonical authority adapter plan Task 1 only.

## Outcome and authorship

The inherited adapter and one-action harness are unchanged. This takeover removed one unused test helper and added ten supplemental tests to the five inherited grouped tests. All fifteen tests pass. Both standalone typechecks pass. The Rust harness run checked 100 samples, found no adapterBindingSafety violation, and reached adapterProducedFirstFill in all 100. The full Python suite was run once and passed all 441 tests.

This is implementation-author verification. The takeover author also authored earlier Candidate A evaluator/harness units. This is not independent source review, cross-provider review, Council acceptance, or an execution attestation. Root coordinates the required nonauthor review.

The implementation derives non-cancellation observations from actual computeTransaction, extraction, and projection. Cancellation uses the separate lifecycle identity rule: unchanged admitted agreement, NoAInput, empty effects, NoCoreProjection, and EvidenceValid. That tag does not replace Core derivation or claim cryptography. Plans and display labels are carried verbatim and are not authorization checks.

No authority boundary, plan fidelity, signing, verification, commit, registry, parent lifecycle, or Candidate A completion claim is made. The harness witness means an observation was produced; it does not mean funds were committed. The sampler is bounded and repeats a deterministic one-action path, not an exhaustive state-space proof.

## Requirements covered

The inherited tests cover first fill, cancellation identity, deposit, second fill, recovery of ten/five, timeout, deadline rejection, invalid operation/time/program diagnostics, and input/effect/outcome/successor substitutions.

The supplemental tests cover:

- Deposit has exact neutral input and effect, no Core payments or warnings, zero reductions, and the actual successor.
- NoInput deadline refund has exact payment and two reductions.
- Supplied deadline recovery preserves the original continuation, whole program, accounts, all choices including present zero and negative values, and minimumTime. It returns contract_closed, empty ordered outputs, and zero reductions.
- Cancellation rejects supplied fill/recovery input. Malformed states and inputs remain diagnostics, not Core rejections.
- Binding rejects predecessor, unused artifact node, artifact call time, transactionTime, absent Core projection, and evidence-status mutations.
- Binding rejects reductions, warnings, error, accepted flag, payments, and projected minimumTime mutations.
- Actual swap settlement produces two ordered payments. Reversed payments and reversed effects fail binding.
- Cancellation positive binding succeeds; nonempty effects, invented accepted Core output, agreement-call substitution, and operation substitution fail.
- An empty alternative plan and alternate display survive unchanged. This is deliberately not a plan-admissibility assertion.

## Historical RED and evidence limitations

The previous worker's directory `.superpowers/sdd/candidate-a-authority-adapter-receipts/` remains unchanged. Its initial hashes are retained in `historical-receipts-before.sha256`.

- `first-fill-red.txt` reports QNT404 missing names, exit 1. This is a parser/name-resolution failure, not a meaningful behavioral RED.
- `first-fill-green.txt` reports a non-Boolean pure fixture discovered as a Test and a Rust panic. The first-fill test passed, but the aggregate command exited 1. The filename is not a passing receipt.
- `cancellation-red.txt` is the actual behavioral RED: first fill passed, cancellation identity failed with QNT511, one passing and one failing, exit 1.
- `cancellation-green.txt` has two passing tests, exit 0, after the inherited EvidenceValid cancellation correction.
- `coverage.txt` has only two discovered tests, exit 0. It is not five-test coverage.
- `coverage-discovered.txt` has the five named inherited tests passing, exit 0.
- The two empty historical typecheck logs have no paired terminal-exit receipt. They are not independently asserted as successful here.
- Historical cancellation RED/GREEN source hashes exist, but no corresponding source-byte snapshots were found. This takeover does not reconstruct or invent them. These logs and hashes are weaker than an immutable full RED source closure.

The ten takeover tests are supplemental assertions against already implemented behavior. They passed on their first batch run. They are not claimed as original REDs, and the implementation was not weakened or temporarily broken to manufacture a RED.

## Fresh verification

Working directory: `/home/charl/Moriarty/.worktrees/s01-audit-start`.
Takeover base HEAD: `93fce82250cf6f0f68114b4d2b5c373cc9b7390d`.
Quint version: `0.32.0`.
All commands below reached terminal exit 0. Existing process state was checked before dispatch; no duplicate validation was started. Source hashes were identical before and after validation.

```text
quint typecheck specs/quint/s02/candidate_a_authority_adapter_test.qnt
quint typecheck specs/quint/s02/candidate_a_authority_adapter_harness.qnt
quint test specs/quint/s02/candidate_a_authority_adapter_test.qnt --backend=rust --seed=42
quint run specs/quint/s02/candidate_a_authority_adapter_harness.qnt --backend=rust --seed=42 --max-samples=100 --max-steps=2 --invariant=adapterBindingSafety --witnesses adapterProducedFirstFill --verbosity=1
/home/charl/Moriarty/.venv/bin/python -m pytest -q
```

The five JSON receipts preserve command, cwd, initial output chunk, terminal output chunk, and terminal exit code. Matching text files concatenate the actual output chunks, without replacing or omitting terminal output. The standalone typecheck text files are empty because successful Quint typecheck printed no output. Full nonempty terminal output:

### Adapter tests

```text

  candidate_a_authority_adapter_test
    ok firstFillActualInputTest passed 1 test(s)
    ok cancellationIdentityEvidenceTest passed 1 test(s)
    ok agreementScenarioCoverageTest passed 1 test(s)
    ok rejectionAndDiagnosticCoverageTest passed 1 test(s)
    ok bindingMutationControlsTest passed 1 test(s)
    ok depositHasNoCorePaymentsTest passed 1 test(s)
    ok timeoutProjectionTwoReductionsTest passed 1 test(s)
    ok suppliedDeadlineRecoveryFullRollbackTest passed 1 test(s)
    ok cancellationRejectsSuppliedInputTest passed 1 test(s)
    ok malformedStateAndInputDiagnosticsTest passed 1 test(s)
    ok predecessorArtifactAndTimeBindingTest passed 1 test(s)
    ok completeCoreProjectionBindingTest passed 1 test(s)
    ok actualTwoPaymentOrderBindingTest passed 1 test(s)
    ok cancellationBindingAndInventedCoreControlsTest passed 1 test(s)
    ok unvalidatedPlanAndDisplayCarriedVerbatimTest passed 1 test(s)

  15 passing (24120ms)
```

### Harness

```text
[ok] No violation found (1969ms at 51 traces/second).
Witnesses:
adapterProducedFirstFill was witnessed in 100 trace(s) out of 100 explored (100.00%)
Use --seed=0x2a --backend=rust to reproduce.
```

### Python suite

```text
........................................................................ [ 16%]
........................................................................ [ 32%]
........................................................................ [ 48%]
........................................................................ [ 65%]
........................................................................ [ 81%]
........................................................................ [ 97%]
.........                                                                [100%]
441 passed in 15.27s
```

## Source closure and retained receipts

Receipts: `.superpowers/sdd/candidate-a-authority-adapter-takeover-receipts/`.

The `final-source/` directory contains the exact thirteen Quint files in the adapter/test/harness import closure. Their snapshot hashes equal the live source hashes. The before/after hash lists also pin frozen Python Core/swap, the adopted design and plan, and the Quint executable entry. These source files and results are not authenticated evidence of external execution.

```text
b59779d5e2e7f1bf0a14952dfe1cd3ab70bcfe813210d690abbb3e3205a6df61  specs/quint/s02/authorization.qnt
3f3b093f4c718dd08eda38e610de700d0a24138beb82fd7b2b12dcf9d300bda8  specs/quint/s02/candidate_a_authority_adapter.qnt
d7b54410ec0ed2ce8de0b63e5f13dca685f91298730e33342759bf52aad9dd23  specs/quint/s02/candidate_a_authority_adapter_harness.qnt
029b2dfd24380f385c961a3145d52528339ee78b9a4d63b9009e7770f2077cf8  specs/quint/s02/candidate_a_authority_adapter_test.qnt
e759d4c13032d83a4d839e08bef0fcf8272943792f73bb3ab7910357b0f28dec  specs/quint/s02/candidate_a_core.qnt
bf814bce2924cafac5836a171b52a4c9bdba43e083fe7129bcdeadd810c42d5e  specs/quint/s02/candidate_a_programs.qnt
2eb66db8010a3d47ddba8f99d647326d2eb7bf3c9da5177c7dd2dcd88145452c  specs/quint/s02/candidate_a_projection.qnt
40901738fd1749527761b624a84253da94bbc24c209d408dc2fd4e895daaae0b  specs/quint/s02/candidate_a_types.qnt
3dd07f3c5f274aff4fce4de9c245aa0b1f1fe7f68e29e5de95a0b1c42a00f93b  specs/quint/s02/consumption.qnt
dccc9208d9a42f01685000b1358f1d2f9684bf5e8ebe1d5deadd0c992557504c  specs/quint/s02/effects.qnt
cf52f55ad1810548b7c45e32552b977a30fc8dd937dfbd01d35829b6beec8928  specs/quint/s02/execution.qnt
e44484ed9035e560ef9f4d012198671917750da42ab404022592db16cdd59bc3  specs/quint/s02/observations.qnt
0886619203c3de16f2326e27e9d4ae2ddfda8c54742c231dd4e824397c5fa9fe  specs/quint/s02/policies.qnt
564a926779beb54bd16ff58481f5c950bae6fb50fcd5b0d7c9eb56427f83273b  moriarty/core.py
82e9e1da76af65706171049f0238953aca5333edec4aeed6caa43dbd57c79797  moriarty/swap.py
5dd9bcaf6b6d7d4269b09fbd9557b6e2d1200b8729a5d3c1290be0e6c2265e27  docs/superpowers/plans/2026-09-05-moriarty-s02-candidate-a-authority-adapter.md
04cd2ed28f9c794823282b982de7d7a5b30c67e8e2d6df95ef18365ab19dddbd  docs/superpowers/specs/2026-09-05-moriarty-s02-candidate-a-authority-design.md
ac12595b1cb7253feec93c79417615c6eb20fc3b6a3df35c5e3530b24e90a501  /home/charl/.npm-global/bin/quint
```

The checkpoint skill's Foreman database action was not used because root explicitly prohibited Foreman work and limited write ownership. The scoped receipts provide the handoff instead. The verification skill required terminal exits and unchanged hashes before completion.

Authorized source commit: `e84f737`. Only the three owned Quint files were committed. The exact full commit identity, paths, terminal commit output, and unchanged post-commit source hashes are in `commit.json`. The report and receipts remain outside that commit for root-controlled archival.
