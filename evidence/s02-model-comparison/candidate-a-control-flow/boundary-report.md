# Candidate A Task 3 independent boundary-test report

Status: completed independent boundary-test authorship. These are finite,
Rust-evaluator observations of the approved Task 3 API. They do not establish
exhaustive model checking, frozen-Python correspondence, transaction behavior,
authority integration, a Council result, or an S02 gate result.

## Scope

Owned files changed:

- `specs/quint/s02/candidate_a_boundary_test.qnt`
- `.superpowers/sdd/candidate-a-boundary-report.md`

No candidate core, type, program, existing-test, Python-reference, manifest,
or commit was changed by this task.

The new module independently covers:

- the 15 non-Close plus six Close-reduction boundary, including ordered warnings,
  canonical ordered refunds, count 21, and terminal quiescence within the
  fixed 22-call fold;
- the maximum 20-unit deposit at the potential boundary, then the exact Pay and
  terminal Close-refund result without arithmetic saturation;
- an expired `When` at `Time2` whose successor reduction retains `Time2`, rather
  than clamping the clock;
- ordered choice scanning: a later case accepts only after an earlier bounds
  mismatch, while a wrong chooser remains `no_matching_input`, not bounds error.

## RED

Before Task 3 was implemented, ran:

```text
quint test specs/quint/s02/candidate_a_boundary_test.qnt --backend=rust --seed=42
```

Exit code: `1`.

Author-reported diagnostic excerpt; complete raw RED terminal output was not archived:

```text
  candidate_a_boundary_test
    1) fixedTwentyTwoCallsReachTerminalQuiescenceTest failed after 1 test(s)
    2) maximumDepositPreservesPotentialAcrossInputAndReductionTest failed after 1 test(s)
    3) expiredWhenRetainsOriginalClockTest failed after 1 test(s)
    4) orderedChoiceScanSeparatesIdentityAndBoundsTest failed after 1 test(s)

  4 failed

  1) fixedTwentyTwoCallsReachTerminalQuiescenceTest:
       Error [QNT508]: Assertion failed
        at specs/quint/s02/candidate_a_boundary_test.qnt:31:5
    Use --seed=0x2a --match=fixedTwentyTwoCallsReachTerminalQuiescenceTest to repeat.
  2) maximumDepositPreservesPotentialAcrossInputAndReductionTest:
       Error [QNT508]: Assertion failed
        at specs/quint/s02/candidate_a_boundary_test.qnt:41:5
    Use --seed=0x2a --match=maximumDepositPreservesPotentialAcrossInputAndReductionTest to repeat.
  3) expiredWhenRetainsOriginalClockTest:
       Error [QNT508]: Assertion failed
        at specs/quint/s02/candidate_a_boundary_test.qnt:52:5
    Use --seed=0x2a --match=expiredWhenRetainsOriginalClockTest to repeat.
  4) orderedChoiceScanSeparatesIdentityAndBoundsTest:
       Error [QNT508]: Assertion failed
        at specs/quint/s02/candidate_a_boundary_test.qnt:61:5
    Use --seed=0x2a --match=orderedChoiceScanSeparatesIdentityAndBoundsTest to repeat.

  Use --verbosity=3 to show executions.
  Further debug with: quint test --verbosity=3 specs/quint/s02/candidate_a_boundary_test.qnt
error: Tests failed
```

Each failure was a `QNT508` assertion failure, not a parse or type error. The
base HEAD was `226cf56d93c2d99c2f7d94405b7f11c223bd80c5`; the Task 3 typed
scaffolding was dirty worktree content, not asserted to be contained by that
commit. Its hashes were:

```text
candidate_a_types.qnt 93ccf78860d69b51fbcbcdee18952d05d33810e4dc9ed34c05e06770b6e1bd17
candidate_a_programs.qnt b443fc0beec22b867e4d36c21714fbaee63950961776d21b2ebe9463e190ab66
candidate_a_core.qnt 948063f04cbf853314a984ef15d76563df72f7e0483a89252d57cbd6b8e2a267
candidate_a_core_test.qnt 42df4e4c11665f185efaf302b33418113302d9b2dead4574116d9e81af847767
```

This RED is a development observation against typed Task 3 scaffolding, not a
product defect. The exact original RED test source was not archived; it was
superseded by the test-construction correction below.

## Test-construction correction

The first post-implementation run passed three tests and failed the
maximum-deposit test. Its expected quiescent result stopped after Pay at N0 and
omitted the required Close refund of the retained 280. The expectation now
records the full terminal state: payments `[Bob 20, Alice 280]`, empty account,
and two reductions. This was a test-construction correction, not a source-model
defect.

## Final stable-source run

Command:

```text
quint test specs/quint/s02/candidate_a_boundary_test.qnt --backend=rust --seed=42
```

Exit code: `0`

Full terminal output:

```text
  candidate_a_boundary_test
    ok fixedTwentyTwoCallsReachTerminalQuiescenceTest passed 1 test(s)
    ok maximumDepositPreservesPotentialAcrossInputAndReductionTest passed 1 test(s)
    ok expiredWhenRetainsOriginalClockTest passed 1 test(s)
    ok orderedChoiceScanSeparatesIdentityAndBoundsTest passed 1 test(s)

  4 passing (169ms)
```

The source snapshot before and after this final run was identical:

```text
HEAD c8cedc1c0a343a5bb03ce4e14320cac62e2522fc
candidate_a_types.qnt c233d6775c65e5eec10eec0768bcfc5d8b64bb7f6601b8e652275456d45ac288
candidate_a_programs.qnt b443fc0beec22b867e4d36c21714fbaee63950961776d21b2ebe9463e190ab66
candidate_a_core.qnt 50f6dd731493510c606c82ce083c55d4d773985a02e43ecf1c22ec393a37d3c1
candidate_a_core_test.qnt 42df4e4c11665f185efaf302b33418113302d9b2dead4574116d9e81af847767
candidate_a_boundary_test.qnt 1bb9ff76c2cf7f6e73e72f3c3076b1ff490e1ed0e9cc90c0bb71d7e7c01ca623
```

`git diff --check -- specs/quint/s02/candidate_a_boundary_test.qnt` produced
no output and exited successfully.
