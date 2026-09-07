# Candidate A neutral projection and effect-extraction author report

Date: 2026-09-05. Classification: repository and experiment observations.
Scope: only the projection/extraction portion of plan Task 5. This is the
implementation author's report, not an independent review or Council gate.

## Ownership and approved interface

Created candidate_a_projection.qnt and candidate_a_projection_test.qnt only,
plus this report and candidate-a-projection-stages/. No core, types, programs,
shared foundations, harness, index, or HEAD changes were made by this author.
Other agents own concurrent harness/test files; they are not included here.
Observed inherited HEAD during RED and GREEN was
`18d8a5ca0ef5fe1434306e4a2a3b45ea744e1c9d`.

The root approved these exact pure interfaces before source was written:

- projectState(program, state): CoreStateObservation[AContinuation].
- projectInput(input): NeutralInput.
- projectResult(program, result): CoreResultObservation[AContinuation].
- extractCommittedEffects(program, before, input, now, result): AEffectExtraction.

The first three are total carrier translations, explicitly not validators.
State projection preserves the existing account entries, translates each
existing choice key to its exact string, preserves NoInt and IntValue(0),
binds the complete program plus continuation node, and decodes numeric time.
A missing choice map stays missing; it is not normalized into a valid state.
Result projection copies all fields, including arbitrary optional warning
payloads. Input projection preserves quantities outside the model domain too;
translation does not certify them.

AEffectExtraction is EffectsExtractedA(List[Transfer]) or one of
EffectResultMismatchA, EffectOutsideDomainA, EffectReductionBoundFailureA,
EffectInputPreconditionFailureA. The root approved this diagnostic-safe
deviation from the plan's bare List return. The harness author received the
same interface before coupling.

Extraction recomputes computeTransaction from the original request, and compares
the supplied complete result with the actual complete result before returning
EffectsExtractedA, including on rejection. A genuine rejected transaction has
empty effects. On success the pre-input reduction is recomputed to obtain its
payment prefix. The returned order is pre-input payment debits, then a credit
for the accepted deposit if present, then the remaining post-input payment
debits. Payments use the neutral paymentTransfer helper. No envelope or
EvidenceValid disposition is created. Lifecycle-only cancellation must not
call this transaction API; its separate adapter retains NoCoreProjection.

## Behavioral RED, then GREEN

The typed stub and all 26 tests were written before implementation. Scaffold
typecheck exited 0. The full seven-file import closure was copied to
`.superpowers/sdd/candidate-a-projection-stages/red/` before running:

```text
quint test .superpowers/sdd/candidate-a-projection-stages/red/candidate_a_projection_test.qnt --backend=rust --seed=42
```

Its terminal output was collected and inspected before changing implementation:
4 passing, 22 failed, exit 1. All 22 failures were QNT508 assertions, not parse
or missing-import errors. The four scaffold passes were noInputProjectionTest
and the three extraction domain diagnostics: the stubs already returned NoInput
or EffectOutsideDomainA. These four are not claimed as observed behavioral RED.
Exact command metadata and every returned output chunk are retained in
candidate-a-projection-stages/red-result.json.

After implementation, typecheck exited 0 and the unchanged 26-test module
passed on the first GREEN test invocation:

```text
quint typecheck specs/quint/s02/candidate_a_projection_test.qnt
quint test specs/quint/s02/candidate_a_projection_test.qnt --backend=rust --seed=42
```

Terminal result: 26 passing (1674ms), exit 0. No test-construction correction,
parse/type error, or assertion failure occurred between archived RED and first
GREEN. The seven-file GREEN closure is archived separately. The test source
hash is identical in RED, GREEN, and the live final file.

The inherited Core regression was also run without modification:

```text
quint test specs/quint/s02/candidate_a_core_test.qnt --backend=rust --seed=42
```

Terminal result: 75 passing (7544ms), exit 0. Its source hash is recorded below.
Raw complete chunks are in core-regression.json.

Receipt files under candidate-a-projection-stages/:

- scaffold-typecheck.json: initial command receipt plus terminal exit 0.
- red-result.json: initial command receipt plus complete terminal assertion RED.
- green-typecheck.json: initial command receipt plus terminal exit 0.
- green-result.json: initial command receipt plus complete 26-test GREEN.
- core-regression.json: initial command receipt plus complete 75-test GREEN.
- snapshots.json: hashes for all 14 archived source files and five receipts,
  plus the inherited Core test, frozen Python sources, and plan hashes.

## Coverage and limits

The 26 deterministic tests check full state/result fields, all five choice
strings, absence versus stored zero, all five numeric clocks, complete-program
continuation binding, malformed-map transparency, exact input payloads, and
accepted/rejected result mutation diagnostics. Literal complete expected
transactions check deposit without payment, deposit then immediate owner
refund, Pay1/deposit5/Pay5 effect ordering, speculative payment/warning rollback,
deadline supplied-input rollback, deadline no-input two-asset refund, and
choice settlement with stored choice. Expected effects are literal transfer
records, not constructed by calling the projection under test.

The carrier-only full-result test deliberately includes a non-Core warning
with NoInt payloads to establish non-validating copying, not to claim this is
an executable frozen Core result. Valid-domain state tests separately assert
validState and exact five-key neutral observations.

This unit has no stateful actions or harness, and no sampled/model-checking
run was performed by this author. No Python suite, independent decoder,
correspondence checker, authority integration, signature, lifecycle
cancellation/recovery implementation, or EvidenceValid production is claimed.
The 75 Core tests are inherited regression evidence, not new tests authored here.
Bound and input-precondition extraction diagnostic arms forward internal
evaluator outcomes; no claim is made that these arms are reachable from an
admitted transaction. There is no proof of exhaustive coverage.

Recomputing the Candidate A evaluator is not an independent semantic oracle.
The later independent frozen-Python checker must still verify raw results and
the separate projection/effect list. Independent source review by the root
remains pending at this handoff; this report does not self-certify correctness.

## Source and tool pins

Quint version observed with quint --version: 0.32.0.
Executable: /home/charl/.npm-global/bin/quint.
SHA-256: ac12595b1cb7253feec93c79417615c6eb20fc3b6a3df35c5e3530b24e90a501.
All tests used the Rust backend and seed 42. No separate Rust-binary hash
was collected by this author.

RED closure hashes (paths relative to candidate-a-projection-stages/red/):

```text
2e88dc47c653d94ba5111b8f6f7dd322859a5d046ad2e0ad7fb9665b0c38f543  candidate_a_projection.qnt
3a6eac6f132b86003d0973c0f6fd26af938b56a082a7a648de597c4f5a2c96a2  candidate_a_projection_test.qnt
e759d4c13032d83a4d839e08bef0fcf8272943792f73bb3ab7910357b0f28dec  candidate_a_core.qnt
40901738fd1749527761b624a84253da94bbc24c209d408dc2fd4e895daaae0b  candidate_a_types.qnt
b443fc0beec22b867e4d36c21714fbaee63950961776d21b2ebe9463e190ab66  candidate_a_programs.qnt
e44484ed9035e560ef9f4d012198671917750da42ab404022592db16cdd59bc3  observations.qnt
dccc9208d9a42f01685000b1358f1d2f9684bf5e8ebe1d5deadd0c992557504c  effects.qnt
```

GREEN/live closure hashes (paths relative to specs/quint/s02/, with exact
copies under candidate-a-projection-stages/green/):

```text
2eb66db8010a3d47ddba8f99d647326d2eb7bf3c9da5177c7dd2dcd88145452c  candidate_a_projection.qnt
3a6eac6f132b86003d0973c0f6fd26af938b56a082a7a648de597c4f5a2c96a2  candidate_a_projection_test.qnt
e759d4c13032d83a4d839e08bef0fcf8272943792f73bb3ab7910357b0f28dec  candidate_a_core.qnt
40901738fd1749527761b624a84253da94bbc24c209d408dc2fd4e895daaae0b  candidate_a_types.qnt
b443fc0beec22b867e4d36c21714fbaee63950961776d21b2ebe9463e190ab66  candidate_a_programs.qnt
e44484ed9035e560ef9f4d012198671917750da42ab404022592db16cdd59bc3  observations.qnt
dccc9208d9a42f01685000b1358f1d2f9684bf5e8ebe1d5deadd0c992557504c  effects.qnt
```

Additional inherited source pins:

```text
91482debc44658a6a55eb6f74b63253f715a9372ed19228e43b16b1261c12375  specs/quint/s02/candidate_a_core_test.qnt
564a926779beb54bd16ff58481f5c950bae6fb50fcd5b0d7c9eb56427f83273b  moriarty/core.py
82e9e1da76af65706171049f0238953aca5333edec4aeed6caa43dbd57c79797  moriarty/swap.py
cc8c66a1a4b94a788376dd9ba1649d6d485d00cb0c254020998a77cba3bf5860  docs/superpowers/plans/2026-09-05-moriarty-s02-candidate-a-core.md
```

Read-only closure check found all 38 import edges resolve across the two
seven-file snapshots; every GREEN source copy equals its live file. The five
dependency files other than the new projection/test are unchanged between
RED and GREEN. No source edits are planned after this handoff.
