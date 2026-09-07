# Candidate A producer/export author report

Date: 2026-09-05. Classification: repository and experiment observations.
Scope: the producer/export portion of Candidate A plan Task 7. This report
does not establish independent correspondence or Council acceptance.

## Handoff and ownership

Source commit, made by the root (not this author):
`02a4e94b604760e54b04f94c0dc089ede0de938b`.

Owned source files are scripts/export_s02_candidate_a_cases.py,
tests/test_s02_candidate_a_export.py, specs/quint/s02/candidate_a_cases.qnt,
and specs/quint/s02/candidate_a_cases_test.qnt. No evaluator, types, programs,
projection, harness, checker, other tests, index, or HEAD edits were made by
this author. The root separately committed the four reviewed source/test files.

The complete generated bundle is at
`.superpowers/sdd/candidate-a-export-stages/final/`:

- cases.json: 53 exported cases, 76 explicit provenance occurrences.
- inputs/: 14 fresh raw ITF files and source-bindings.json.
- source/: the exact 12 bound source files plus the two producer test files.

All 14 source/test copies were compared byte-for-byte through hashes with the
source commit above. The snapshot manifest records 64 artifact pins.
The root plans to archive this bundle byte-exact under
evidence/s02-model-comparison/candidate-a-correspondence/export.
This report does not claim that archive is already committed.

## Approved schema and trust boundary

Schema version 1 has exactly source_pins and cases beside schema_version.
Each case contains case_id, fixture_id, trace_id, step_index, request, result,
projection, effects, and provenance. Result is the BARE ATransactionResult,
not a TransactionComputedA wrapper. The other three selected raw fields are
also copied directly from the ITF request/computed payload.

Each provenance occurrence contains trace_id, state_index, step_index,
input_path, and input_sha256. trace_id is the relative input filename, not
the ITF variable name. The input variable is read from document.vars[0]:
swapTrace, installmentTrace, or diagnosticCases. Normal computed tags are
SwapComputedA, InstallmentComputedA, and CaseComputedA, respectively.

Repeated prefixes are deduplicated only within one input trace, at the same
record index. Every occurrence is retained in provenance. Shrinking histories,
conflicting full prefix records, duplicate case IDs, unknown variables,
diagnostics, missing/malformed selected payloads, unknown fixture IDs,
source/input hash mismatches, undeclared files, missing files, and unsafe paths
fail export. Different trace files remain separate, even for equal requests.

The exporter requires source-bindings.json with exact schema_version,
source_pins, and input_pins fields. Eight base source pins are mandatory;
the four optional paths are the three generating modules and this exporter.
The observed generator's pin is mandatory. Declared source bytes are verified
against --source-root, and the complete actual ITF inventory and bytes are
verified against input_pins before export. Input provenance is constructed
from those actual files and indexed records, never from caller-supplied
provenance labels. Existing output files are not overwritten.

The exporter performs structural copying/validation only. It neither decodes
semantic symbols nor computes expected results, normalizes maps, reorders
payment/effect lists, repairs fields, or imports the candidate/reference
evaluator or checker. Original raw files remain unchanged and hash-bound;
JSON whitespace is not claimed to be preserved in the reserialized wrapper.
No separate neutralInput projection is exported by this unit. That projection
is therefore not independently compared by these producer outputs.

## Finite diagnostic request corpus

The 24 literal requests cover all six frozen Core errors, both warning classes
with exact zero/negative/partial payloads, pre-Pay1/deposit5/post-Pay5 ordering,
speculative payment/warning rollback, deposit matching and immediate refund,
absence versus stored-zero choice branches, ordered disjoint/overlapping case
selection, wrong chooser, and all-account/all-choice projection.

Program identities come from the approved 13-fixture catalog, not arbitrary
producer graphs accepted as an oracle. The diagnostic module uses 12 of these;
the installment trace module supplies installment-two-when-v1. All unused
nodes remain CloseA and all 16 node entries belong to the complete identity.
The independent checker must reconstruct each table without importing producer
helpers and compare unused nodes as well as reachable continuations.

Every corpus payload is obtained through actual computeTransaction,
projectResult, and extractCommittedEffects. Evaluator/extraction diagnostics
are retained as distinct variants, never fabricated as accepted/rejected Core
results. Initialization computes all 24 vectors. step is disabled. These
initialization-generated vectors are NOT stateful witnesses.

## Observed RED and GREEN

Python: the initial exporter stub and the complete 30-test module were
archived under python-red/scripts and python-red/tests before implementation.
The archived command was run from python-red/:

```text
/home/charl/Moriarty/.venv/bin/python -m pytest -c /dev/null tests/test_s02_candidate_a_export.py -q
```

Terminal RED: 29 failed, 1 passed, exit 1. The unchanged output-file protection
already passed. Two success-path tests reported IndexError because the stub
returned an empty cases list; the remaining failures were assertions or
missing expected ExportError. No missing import was called behavioral RED.
The /dev/null pytest configuration caused two cache-provider permission
warnings; both are preserved in the full raw receipt, not hidden or called
product defects. The complete 60,823-character output is archived; no
tool-output truncation marker occurs in the receipt.

After implementation, the unchanged 30 tests passed, exit 0:

```text
/home/charl/Moriarty/.venv/bin/python -m pytest tests/test_s02_candidate_a_export.py -q
```

Quint: scaffold typecheck exited 0. The eight-file full import closure was
archived under quint-red/ before running:

```text
quint test .superpowers/sdd/candidate-a-export-stages/quint-red/candidate_a_cases_test.qnt --backend=rust --seed=42
```

Terminal RED: 1 passed, 7 failed, exit 1; every failure was QNT508.
Literal-domain validity passed before implementation. Terminal RED was
collected before implementing computeCase. Afterwards, typecheck and the
unchanged eight tests passed, exit 0:

```text
quint typecheck specs/quint/s02/candidate_a_cases_test.qnt
quint test specs/quint/s02/candidate_a_cases_test.qnt --backend=rust --seed=42
```

No test weakening or construction correction was needed between either
archived RED and its first GREEN. Python malformed-input/source/dedup mutation
controls are included in the 30-test result. They use synthetic structural
records, deliberately not claimed to be semantic reference cases.

## Fresh generation and export results

Every generating command, initial process handle, complete returned output,
and terminal exit is preserved in its generation-*.json receipt. All runs use
Rust, seed 42, and the same frozen source closure. The 13 stateful runs check
the NEGATION of a desired recorded outcome, with max-samples=100 and
max-steps=8. Their exit 1 indicates the desired recorded state was reached;
it is not a failed coreTraceSafety check. No sampled safety result is claimed
for these targeted generation commands.

The diagnostic generation uses max-samples=1, max-steps=0, and
--invariant=corpusComplete; it exited 0. It is initialization evaluation only.

| Raw input | States | Exported records |
| --- | ---: | ---: |
| diagnostic-corpus.itf.json | 1 | 24 |
| installment-deadline-five-cleanup.itf.json | 4 | 3 |
| installment-deadline-five.itf.json | 3 | 2 |
| installment-deadline-ten-cleanup.itf.json | 3 | 2 |
| installment-deadline-ten.itf.json | 2 | 1 |
| installment-refund-five.itf.json | 3 | 2 |
| installment-refund-ten.itf.json | 2 | 1 |
| installment-two-fills.itf.json | 3 | 2 |
| swap-alice-timeout.itf.json | 3 | 2 |
| swap-deadline-cleanup.itf.json | 5 | 4 |
| swap-empty-timeout.itf.json | 2 | 1 |
| swap-funded-timeout.itf.json | 4 | 3 |
| swap-settlement.itf.json | 4 | 3 |
| swap-voluntary-refund.itf.json | 4 | 3 |

The stateful traces contribute 29 transaction records; the diagnostic file
contributes 24. All observed raw payloads use their normal computed tag.
The deadline-cleanup traces retain the rejected transaction and subsequent
separate no-input refund. Direct deadline-10/deadline-5 paths are also present.

Final exporter command, exit 0, reported 53 cases:

```text
/home/charl/Moriarty/.venv/bin/python scripts/export_s02_candidate_a_cases.py --itf-dir .superpowers/sdd/candidate-a-export-stages/final/inputs --source-root .superpowers/sdd/candidate-a-export-stages/final/source --out .superpowers/sdd/candidate-a-export-stages/final/cases.json
```

A read-only author spot check re-opened all 76 provenance occurrences and
compared the actual indexed raw request/result/projection/effects and input
SHA-256 with the exported fields: no discrepancies. This is an author
structural check, not the independent semantic checker.

## Pins and limitations

Python 3.13.14, pytest 9.1.1, Quint 0.32.0.
Python executable SHA-256:
396f548d3fc6ef4f52a81b2c6b51bb3f00d3e60c82081e1a80f66cfeba90b939.
Quint executable SHA-256:
ac12595b1cb7253feec93c79417615c6eb20fc3b6a3df35c5e3530b24e90a501.
Separate Rust evaluator binary/package closure hashes were not collected.

Bound source hashes:

```text
564a926779beb54bd16ff58481f5c950bae6fb50fcd5b0d7c9eb56427f83273b  moriarty/core.py
82e9e1da76af65706171049f0238953aca5333edec4aeed6caa43dbd57c79797  moriarty/swap.py
e759d4c13032d83a4d839e08bef0fcf8272943792f73bb3ab7910357b0f28dec  specs/quint/s02/candidate_a_core.qnt
40901738fd1749527761b624a84253da94bbc24c209d408dc2fd4e895daaae0b  specs/quint/s02/candidate_a_types.qnt
bf814bce2924cafac5836a171b52a4c9bdba43e083fe7129bcdeadd810c42d5e  specs/quint/s02/candidate_a_programs.qnt
2eb66db8010a3d47ddba8f99d647326d2eb7bf3c9da5177c7dd2dcd88145452c  specs/quint/s02/candidate_a_projection.qnt
dccc9208d9a42f01685000b1358f1d2f9684bf5e8ebe1d5deadd0c992557504c  specs/quint/s02/effects.qnt
e44484ed9035e560ef9f4d012198671917750da42ab404022592db16cdd59bc3  specs/quint/s02/observations.qnt
0651108d66e40e0295f7ef568665257bee59b93300dae1f7dd6aad684ce8429c  specs/quint/s02/candidate_a_harness.qnt
5912655f3f8b35e7f85602099851fea9bdddf6a28493964ddc834e53b098194e  specs/quint/s02/candidate_a_installment_harness.qnt
03da5af65cd4af2fa2d7e23f87ea43d9212f8ac9edef5dfe86e02ee5c79705cb  specs/quint/s02/candidate_a_cases.qnt
8efb311ae3a00d320637278adf358d25052534f73f225b50cdbaba5d2ed6f04c  scripts/export_s02_candidate_a_cases.py
```

snapshot-manifest.json contains hashes for RED sources, source-commit-matched
GREEN sources/tests, command/result receipts, raw inputs, source bindings,
and the exported cases file. Existing earlier Task5/Task6 ITFs and receipts
were not replaced; these are fresh generated artifacts.

No exhaustive model checking, stateful safety sampling, Python full-suite
regression, authority integration, lifecycle cancellation transaction,
EvidenceValid production, or independent correspondence result is claimed
by this author report. Cancellation remains outside the transaction export
schema and must not be fabricated as a Core result. The independent checker
is owned by another agent; its source review, semantic comparisons, raw
provenance linkage checks, and mutation evidence are separate artifacts.
