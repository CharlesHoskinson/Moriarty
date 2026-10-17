# DoctorWatson commands and reports

## Shell entry point

`scripts/doctorwatson.sh [--design|--host]` accepts zero arguments or exactly one listed flag; other arguments exit 2. Default runs `qa/check.py` and Python `test_*.py` discovery. `--host` additionally invokes `run_host.py`; `--design` additionally selects the explicit bounded TypeScript Quint workflow-model commands. The flags cannot be combined. [Script](../../../scripts/doctorwatson.sh).

`--host` reads `DOCTORWATSON_RUSTC` and `DOCTORWATSON_OUTPUT_DIR`. `--design` reads `DOCTORWATSON_NODE` and `DOCTORWATSON_QUINT`; missing installed tools exit 127. Design child commands have individual 30-second limits and a 256 MiB Node heap. Workflow-model results do not establish Sherlock interpreter results.

## Supplied-observation checker

`python3 models/sherlock/qa/check.py` accepts:

| Options | Contract |
| --- | --- |
| `--repo PATH` | Defaults to checkout root |
| `--requirement-evidence PATH` | Read the selected current EARS evidence sidecar; report declared status and computed currentness separately |
| `--observed-programs PATH --observed-prefixes PATH` | Both required together; existing literal document mode |
| `--family normal|extra-normal` | Selects literal comparison family; default normal |
| One of `--host-observations`, `--k-observations`, `--quint-observations`, `--lean-observations`, `--image-observations` | Complete supplied artifact; mutually exclusive with other layer flags and literal-document mode |

JSON exits: 0 `pass`, 1 `mismatch`, 2 malformed input. Fields include scope/acceptance/profile/counts, frozen/transcription records, first difference, supplied layers and `execution_layers`. `source_reference_facts` inventories explicit input byte reads only: dependency closure, reviewer reading and execution remain unestablished. Acceptance is `none`; checker execution layers remain `not-run`. Supplied comparison does not execute/authenticate a producer. [Checker source](../../../models/sherlock/qa/check.py).

The shell selects the [current evidence sidecar](../../../openspec/changes/sherlock-completion-convergence/current-evidence.json).
Its `requirement_evidence` report inventories the 35 original clauses/37 scenarios,
25 refinements/68 scenarios and declared retained families separately. Each row
preserves implementation, validation, acceptance and next-action declarations;
the checker verifies named membership and declared source/evidence currentness.
Changed or missing support excludes a closure from the current count and returns
a mismatch. Current open requirements can pass this integrity check. Matching
hashes do not establish semantic acceptance. See the [requirement checklist](../requirements-status.md)
and [complete report contract](../../DOCTORWATSON-CI.md#current-ears-evidence-report).
A bare checker call omits this report and keeps the existing corpus behavior.

## Host runner and files

`python3 -I -B models/sherlock/qa/run_host.py --rustc ABS --output-dir ABS [--realization host|image|stream]` requires normalized absolute paths and a fresh directory outside the checkout. Default host selects 14 tests; image has no suite (`not-selected`); stream selects eight `visit_steps` tests and compares through `--host-observations`. [Arguments](../../../models/sherlock/qa/run_host.py#L464). Shell `--host` still selects default host; there is no shell `--stream` flag.

The plan snapshots sources, compiles/runs producer/tests, compares, then rechecks sources; failure blocks dependents. Child deadline: 30 seconds; memory/disk/process counts unbounded. Files: `source/`, `build/`, `logs/`, `{host,image,stream}-observations.json`, `comparison.json`, `receipt.json`, `result.json`. Step states: `ok`, `failed`, `timeout`, `not-run`, `not-selected`. Stream's observer retains every record for comparison; it supplies no memory/allocation bound.

Root's 2026-10-17 receipts record default CI exit 0/241 Python tests and stream `success`: Rust 1.99.0 producer/test compilation, eight tests, producer, 216 complete-prefix comparisons across eight programs, and postflight with 26 source pins unchanged. Comparison is `pass`/`match`, authenticity `not-established`; startup and K/Quint/Lean/Clean/circuit/native layers were not executed. Receipt identities are in the corpus manifest.

`python3 models/sherlock/qa/analysis/work_summary.py INPUT` prints `sherlock-logical-work-summary/1` JSON with input digest; errors exit 2. Physical/economic costs remain unmeasured and A helper work unknown. [Summary CLI](../../../models/sherlock/qa/analysis/work_summary.py#L179).

See [comparison](../how-to/compare-complete-observations.md), [stream checks](../how-to/check-stream-realization.md), [new cases](../how-to/add-normal-case.md).
