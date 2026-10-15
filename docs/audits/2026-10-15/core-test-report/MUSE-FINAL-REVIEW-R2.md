# Muse final nonsecurity review R2 — Core test report candidate

- Candidate: `95df73558ab4fb0b6d80618a76e88c3a52c38888`; base `9dffbb0a3f6996dbdae5030ed9bb0a5193d39d73`; checkout `/home/charl/Moriarty/.worktrees/core-test-report-20261015` (HEAD verified equal to candidate).
- Requested identity: fresh independent non-author Muse Wonder 1.0, high effort, nonsecurity. Returned identity: Muse Code powered by `muse-wonder-1.0`. Author: GPT-6.1 Sol high (implementation/normal tests); aggregate helper, unit and CLI security tests authored by Opus 5.5 and mechanically integrated by Sol. This review supplies no security verdict; all security analysis/judgment remains solely Opus 5.5.
- Method: read-only Git/source and public dossier JSON/gzip inspection. No tests were run by this reviewer, and no native/product/Cargo/SDK/VM/model/degree/key/proof/ledger/private-key/PK/SRS/witness/proving-cache operation was performed (native authority 0).

## Verdicts

- **SOURCE_VERDICT: APPROVE** — for the exact non-evidence scope below, nonsecurity functional integration/completeness/format only.
- **ACTUAL_RESULT_VERDICT: APPROVE** — for the recorded actual results below only. Neither verdict closes M10/U7/MC08/U0–U7, native/target qualification, or full financial conformance. Missing audit is not approval; a fresh Opus security verdict on this exact candidate remains separate and is not supplied here.

## Scope inspected

Seven hash-bound non-evidence files in `docs/audits/2026-10-15/core-test-report/scope.json` were each re-hashed and byte-counted; all seven match exactly: `ROADMAP.md` (16532, `0199b34e…`), `crates/mori/src/commands/test.rs` (7418, `4d8bc187…`), `crates/mori/tests/core_test_report.rs` (10521, `74e021ed…`), `docs/NATIVE-CORE.md` (33582, `bb987aac…`), consolidated spec (18102, `66f85468…`), traceability (21180, `d549f31a…`), and crosswalk (86380, `a361071a…`). Full source projection `c3624b8f0e439d61fe5c1bb57d8d06870cdde6cb0759c7dc2fc389f804913280` as recorded. Preserved `mori-core/src/json.rs` and `core_io.rs` were verified byte-identical to base; shared IR exporter/evaluator/signing/native files are unchanged in the diff.

Full current `test.rs` and `core_test_report.rs` were read in full, plus the base-to-candidate diff for the affected docs/EARS/traceability and the exact UNI-024 index change (only its `clauseSha256` refreshed to `46157c82…`, acceptance still `open`, no IDs/clauses/exits removed). Dossier README, independent expectations MD/JSON (manual section and memberships only for the large arrays), green/red-r3 results with gzip stdout, Opus unit selection/result (ownership/count only), `actual/SUMMARY.json`, `capture-normal-results.py`, compile-only result, follow-up CLI/unit receipts, `actual-r2/SUMMARY.json`, `final-normal-output-equality.json`, and `publication-verification-r2.json` were read.

## Source findings (nonsecurity)

1. **Report assembly is functionally correct.** [aggregate_report](/home/charl/Moriarty/.worktrees/core-test-report-20261015/crates/mori/src/commands/test.rs:89) exports every case through the unchanged `mori_core::json::export`, adds only fixed structural bytes, re-indents structural newlines only, builds the complete string before `println!` at [test.rs:76](/home/charl/Moriarty/.worktrees/core-test-report-20261015/crates/mori/src/commands/test.rs:76), and leaves evaluation, comparison, order, status, and exit selection at [test.rs:37](/home/charl/Moriarty/.worktrees/core-test-report-20261015/crates/mori/src/commands/test.rs:37) unchanged. Any per-case export error fails before any stdout is published.
2. **Delta since `69f3258d` is exactly as described.** The runtime prefix before `#[cfg(test)]` (4225 bytes, `101f661e…`) is byte-identical between `69f3258d` and this candidate (recomputed), and the current integration file retains all 8891 original normal-test bytes (`ed6c46c3…`) as an exact prefix (recomputed). Changes since `69f3258d` are only the Opus-authored additions: one unit test in `test.rs`, two CLI tests in `core_test_report.rs`, and the NATIVE-CORE reader-profile text. No new expected financial values were introduced; the frozen normal oracle digest `3911ef89…` is unchanged.
3. **Integration coverage is complete for the stated scope.** Six CLI tests in [core_test_report.rs](/home/charl/Moriarty/.worktrees/core-test-report-20261015/crates/mori/tests/core_test_report.rs:162): small byte-exact legacy output (28,093 bytes, pinned digest), 256-case large, mixed with mismatches at 0/127/255 and exit 1, 87-case FeePool frozen post/effects/order with trace presence, hostile-name escaping, and unexportable-final-case with empty stdout. Four filtered unit tests cover byte identity, >1 MiB round-trip, per-case oversize, and the depth boundary. The reader-profile documentation gap (former F1) is now covered by Opus's exact text at NATIVE-CORE; no security judgment is supplied on it here.
4. **Docs/EARS/traceability are accurate and bounded.** ROADMAP and traceability record local evidence only and keep M10/U7/MC08 open; the new spec scenario matches the implemented behavior, including the "when every case report is exportable" qualification in the guide. OpenSpec files are unchanged since `69f3258d`, so the retained 14-package strict pass and 11 validator tests remain applicable.

No nonsecurity source finding requires changes.

## Actual-result findings

Independently recomputed from the committed gzip receipts (not merely read from summaries):

- `actual-r2` small/large/mixed stdout each decompress to bytes byte-identical to the corresponding independent expected stdout: 28,093 (`b57043f1…`), 7,181,098 (`796d5657…`), and 7,181,101 (`8a967b85…`) bytes; mixed exit 1 with exactly mismatches 0/127/255 and complete actuals. FeePool stdout is 31,333,417 bytes (`4d72faed…`), status `passed`, 87/87 cases passed. All four are byte-identical to the initial capture; compiled binary `89ae66c4…` is unchanged before/after compile-only.
- Behavioral red-r3 is exact: 1 passed, 3 failed with `JSON exceeds 1 MiB` against unchanged production; the earlier 120 s compile timeout and test-only `LowerHex` error are preserved failed attempts, not reds or passes. Green is 4 passed; current follow-up CLI run is 6 passed; current filtered unit run is 4 passed with 21 filtered out (initial Opus run: 3 passed, 21 filtered). Raw gzip stdout preserves EOF blank lines; `git diff --check` on the full candidate passes without trimming raw data.
- `publication-verification-r2.json` independently confirms the current seven source hashes, runtime-prefix and test-prefix identity, gzip digests, and navigation; this reviewer's recomputation agrees.

## Limitations

Profiles exercised are `moriarty/1`, `moriarty-core/1`, and the `moriarty-tests/1` manifest only. Only the 87 accepted FeePool cases were newly captured; the original 162-case membership is preserved and the 75 rejected cases were not newly tested. FeePool trace checking is presence-only; the normal fixture checks every trace field. Repetitions do not establish full financial conformance. No native or target qualification exists; no parent M10/U7/MC08/U0–U7 exit closes. Opus-authored helper/security tests were reviewed for functional integration, completeness, and format only.

**SOURCE_VERDICT: APPROVE** and **ACTUAL_RESULT_VERDICT: APPROVE** for candidate `95df73558ab4fb0b6d80618a76e88c3a52c38888` within the scope and limitations above.
