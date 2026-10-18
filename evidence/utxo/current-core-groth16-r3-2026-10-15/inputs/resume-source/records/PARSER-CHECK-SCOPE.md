# Own parser-check scope (written before the check ran)

Author: claude-opus-5-5, evidence-repair resume source author, 2026-10-15.

## What runs

One command, once per check revision:

```
timeout 120 python3 -I <OWN>/tools/parser_selfcheck.py
```

`<OWN>` = `/home/charl/moriarty-eutxo-validation-2026-10-12/current-core-groth16-evidence-repair-resume-source-20261015`.

## Effects (complete)

- Sets `RLIMIT_CPU` 60 s and `RLIMIT_AS` 1 GiB on itself; wall bound 120 s by `timeout`.
- Reads exactly one code file, `<OWN>/operator-proposals/libtest_rows.py` (this author's own proposal; not candidate, operator or reviewer code), hashes it, and executes those exact bytes as a module with `compile()`/`exec()` in-process.
- Every log, request, RESULT and trace input is a synthetic string written by the check itself. No real run log, candidate file, frozen packet file or operator file is read.
- Writes only below `<OWN>/records/parser-check/` (created by the check; it refuses to start if that directory already exists) and one result file `<OWN>/records/PARSER-CHECK-RESULT.json` (created exclusively).
- No subprocess, network, Rust, Cargo, compiler, evaluator, harness, crypto, key, proof, Git or ledger call. No import of anything except the Python standard library and the hashed helper bytes.

## Test scope

| Case | Synthetic input | Expected |
|---|---|---|
| C1 inline first line | `--nocapture` stream shaped like the actual r2 log: `test NAME ... identity build_manifest_sha256 …` then two more identity lines then `ok` | outcome `ok`; three identity lines including the inline build-manifest line |
| C2 legacy defect reproduced | the same stream through a verbatim copy of the reviewed `run_pathb.py` `tests_of()` lines and the `startswith('identity ')` filter | legacy outcome is the first data row; legacy identity list lacks the inline line (N-04 reproduced on synthetic data) |
| C3 typed rows and summary | r3-style rows with OK/FAIL/OPEN and `[CLASS]`, a matching and a deliberately non-matching `summary` line | parsed counts equal the matching summary; the non-matching one is flagged |
| C4 failing test, nocapture | rows then `FAILED`, binary summary `FAILED` | outcome `FAILED`; no anomaly |
| C5 capture mode with failures section | `test a ... ok`, `test b ... FAILED`, `---- b stdout ----` block | outcomes recorded; failure section captured; no anomaly |
| C6 anomalies | open test at end of stream; header while a test is open; running count mismatch | each reported as an anomaly |
| C7 ignored with reason | `test c ... ignored, slow` | outcome `ignored, slow`; summary count check equal |
| C8 HW resolution | synthetic table map and trace | OK / FAIL / MISSING / OPEN statuses exactly as defined |
| C9 CLI analyze | synthetic run dir under `records/parser-check/` | first call creates `ANALYSIS-<UTC>.json`; second call creates a second file, nothing overwritten, index has two lines; `--out-dir` equal to the run dir refused (exit 2) without writing; a missing request file produces a dated `.error.json` and exit 3 |

A failed case is recorded verbatim in the result file; the check is not retried with edited expectations. These checks establish parser behaviour on synthetic text only. They are not evidence about any actual run, and they do not review the helper's effects for operator use.
