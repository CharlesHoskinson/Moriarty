# Muse final nonsecurity review — Core test report candidate

- Candidate: `69f3258db08c5949d9a643828b5cebf8453e2591`; base `9dffbb0a3f6996dbdae5030ed9bb0a5193d39d73`; checkout `/home/charl/Moriarty/.worktrees/core-test-report-20261015` (HEAD verified equal to candidate).
- Requested identity: fresh independent non-author Muse Wonder 1.0, high effort, nonsecurity. Returned identity: Muse Code powered by `muse-wonder-1.0`. Author: GPT-6.1 Sol high (implementation/normal tests); helper and its three unit tests authored by Opus 5.5 (`claude-opus-5-5`) and mechanically integrated by Sol. This review supplies no security verdict; all security analysis/judgment remains solely Opus 5.5.
- Method: read-only Git/source and public dossier JSON/gzip inspection. No tests were run by this reviewer, and no native/product/Cargo/SDK/VM/key/proof/ledger operation was performed (native authority 0).

## Verdicts

- **SOURCE_VERDICT: APPROVE** — for the exact non-evidence scope below, nonsecurity functional integration/completeness/format only.
- **ACTUAL_RESULT_VERDICT: APPROVE** — for the recorded actual results below only. Neither verdict closes M10/U7/MC08/U0–U7, native/target qualification, or full financial conformance. Missing audit is not approval; Opus final security review of this exact candidate/results remains separate and outstanding from this report's scope.

## Scope inspected

Seven hash-bound non-evidence files in `docs/audits/2026-10-15/core-test-report/scope.json` were each re-hashed and byte-counted; all seven match exactly: `ROADMAP.md` (16532, `0199b34e…`), `crates/mori/src/commands/test.rs` (6565, `e3b1668a…`), `crates/mori/tests/core_test_report.rs` (8891, `ed6c46c3…`), `docs/NATIVE-CORE.md` (33335, `6e18049b…`), consolidated spec (18102, `66f85468…`), traceability (21180, `d549f31a…`), crosswalk (86380, `a361071a…`). Recorded source projection `c3f64d702756f6dc1a9faa3e1aa6a611ce9a22e84b2d9849b1bbacf0b04d8620` is as listed in scope (per-file hashes independently verified). Preserved files `mori-core/src/json.rs`, `commands/core_io.rs`, both OpenSpec program/workflow JSON and `sprints.json` are byte-identical to base; shared IR exporter/`core_io`/evaluator/signing/native paths show no diff. `git diff --check base..candidate` passes. Full current `test.rs`, full new `core_test_report.rs`, affected docs/EARS/traceability, dossier README, independent expectations MD/JSON (manual expectation and FeePool membership sections targeted, multi-MB repeated arrays not dumped), green/red-r3 results and gzipped stdout, Opus unit selection/result, `actual/SUMMARY.json`, `capture-normal-results.py`, and compile-only result were inspected.

## Source findings (nonsecurity)

1. Callsite preserves semantics: evaluation, comparison, order, counting and exit selection in `test.rs:50-82` are unchanged; only the whole-document `core_io::print` is replaced by `aggregate_report(failed==0, reports)?` plus `println!` (`test.rs:76-77`), so the final newline and exit 0/1 behavior are retained.
2. `aggregate_report` (`test.rs:89-118`) exports every case through the unchanged `mori_core::json::export`, adds only fixed structural bytes, re-indents raw newlines (structural only in pretty JSON), builds the full string before any stdout write, and propagates any per-case export error with no partial report. Field order is taken from the same `serde_json::Map` construction as the legacy wrapper. Functional integration of the Opus-authored helper/tests is exact apart from the recorded rustfmt layout/trailing-comma change; no security judgment is made here.
3. Integration tests (`core_test_report.rs:162-248`) independently freeze the normal execution (alice 100→75, bob 0→25, sixteen observer 42, duty 11, two steps, one Transfer), pin small stdout at 28093 bytes / SHA-256 `b57043f1…`, require complete 256-case large and mixed reports (mismatches only at 0/127/255, exit 1), and check the 87 accepted FeePool cases for exact frozen post/effects/order with trace presence only. Frozen normal expected values are unchanged; the two earliest author records (`AUTHOR-TEST-ONLY-READY`, `ORACLE-SOURCE-CHECK`) carry the previous test hash (`0c10e7a6…`, 8586 bytes) superseded only by formatting plus the bytewise digest-compatibility fix before the observed red.
4. Docs/EARS/traceability are correctly bounded: ROADMAP and `docs/NATIVE-CORE.md` describe local reporting evidence only; the new spec scenario (spec.md:267-269) states the aggregate SHALL behavior; traceability M10 and parents U7/M10/MC08 remain open. The crosswalk change is exactly the single UNI-024 `clauseSha256` refresh (`5513cd34…` → `46157c82…`) with `acceptance: open` retained; no ID, clause, or exit was removed.

No nonsecurity defect requiring changes was found in this scope.

## Actual-result findings

- Behavioral red (red-r3): 1 passed / 3 failed against unchanged production, each failure showing successful evaluation followed by `JSON exceeds 1 MiB`; exit 101. Green: 4/4 integration tests pass, exit 0. The earlier 120 s compile timeout (exit 124) and test-only `LowerHex` compile error are preserved failed attempts, not passes or behavioral reds.
- Opus filtered unit tests: selection lists exactly the three `report_tests` names; execution passes 3, with 21 other binary unit tests filtered out (ownership/count only reviewed here).
- Independent actual captures byte-match the frozen independent expectations (verified by decompression in this review): small 28093 bytes exact; large 7,181,098 bytes; mixed 7,181,101 bytes, exit 1, mismatches exactly 0/127/255, complete actuals retained. FeePool stdout 31,333,417 bytes, exit 0, exact original frozen post/effects/order and trace presence. Original 162 FeePool membership (87 accepted / 75 rejected) is preserved; the 75 rejected were not newly tested, and repetitions do not establish full financial conformance.
- OpenSpec: 14 packages strict-pass; planning verify records 160 requirements / 12 sprints / 24 gates; 11 validator tests pass. Raw Rust stdout gzip receipts (`green`, `red-r3`, `opus-unit`) decompress to the recorded byte counts/digests, preserving original EOF blank lines. No plugin change is in this candidate; PR 51 plugin 0.1.1 is separate. Opus initial classification timed out at 700 s and the snapshot retry completed in 264 s; neither is treated as final candidate approval here.

## Limitations

Profiles are local `moriarty/1`, `moriarty-core/1`, `moriarty-tests/1` manifest only. There is no native, target, Preview/Fenrir, proof, ledger, or private-material evidence, and none was sought. FeePool trace checking is presence-only; normal trace fields are fully checked. This approval covers only the aggregate-report repair and its recorded local results.
