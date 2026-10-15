# Security review: `mori test` aggregate report, candidate `95df7355`

**Candidate:** `95df73558ab4fb0b6d80618a76e88c3a52c38888`
**Base:** `9dffbb0a3f6996dbdae5030ed9bb0a5193d39d73`
**Projection:** `c3624b8f…` (as supplied, not recomputed)

**Basis:** You asked for no tools or execution, so I used none and wrote no plan file. I worked only from the source and evidence in your message. I recomputed no hash and read no repository file. This review covers the full applicable candidate, not just the change since `69f3258d`. My earlier `69f` approvals are not relied on as approval here.

---

## SOURCE_VERDICT: APPROVE (narrow)

### Production path: `run` → `aggregate_report`

I re-derived each property from the current source:

1. **Only `export` produces variable bytes.**
   - Every case `Value` goes through the unchanged `mori_core::json::export` (`json.rs` is byte-identical to base, hash `941aa74b…`).
   - The helper adds only fixed ASCII: braces, brackets, commas, newlines, indentation, the two keys and the literals `passed` and `failed`.
2. **Re-indenting is value-preserving.**
   - Exported text is ASCII. Every control character except `\n` is escaped, including DEL.
   - `serde_json` pretty output escapes newlines inside strings, so every raw `\n` is structural.
3. **Legacy byte identity holds wherever the whole-document export succeeded.**
   - Each character is escaped independently of context.
   - Nested pretty output equals the standalone form plus the parent indentation.
   - Separators match legacy.
   - Key order comes from the same `serde_json::Map` type, with or without `preserve_order`.
4. **Publication fails closed.**
   - All cases are exported before `println!`, and any error returns through `?`. Stdout gets nothing.
   - The error message goes through `display::text`.
   - Comparison, status, order and exit selection are untouched.
5. **Terminal safety.**
   - Stdout is ASCII with no controls except `\n`.
   - The stderr status line carries counts only. Case names never reach stderr.
6. **No new trust surface.**
   - Dispatch `Command::Test → test::run` has no preceding setup (per the `main.rs`, `mod.rs` and `cli.rs` sources you supplied).
   - `core_io.rs`, the evaluator, signing and native code are unchanged.
   - The 256-case, per-file 1 MiB, depth-32 and 65,536-node input bounds are retained.
   - The production prefix before `#[cfg(test)]` is unchanged from `69f`. An unchanged executable image hash (`89ae66…`) after a `cfg(test)`-only edit is consistent with that.

### New tests, checked by hand against the exact code

- **T1 (depth, unit).**
  - Under per-case `export`, `deep(31)` puts the innermost scalar at depth 32. The reader rejects only depths above 32, so it is accepted.
  - In the whole document it sits at depth 34, so legacy `export` errs and `parse` of the aggregate errs. Both assertions are correct.
  - `serde_json::from_str` has a recursion limit of 128, so the round-trip succeeds.
  - `deep(32)` puts the scalar at depth 33, so the per-case export errs.
  - This pins both the retained per-case depth bound and the documented widening.
- **T2 (hostile name, CLI).**
  - When the manifest is written, `serde_json` escapes `\n`, `\r` and ESC, and leaves DEL, U+0085, U+202E and the emoji raw. The input reader accepts these.
  - Names are unique, and the comparison does not depend on the name.
  - The byte predicate rejects DEL (`is_ascii_control`), so it really forces escaping.
  - Whole-`Value` equality shows the name is escaped exactly once.
- **T3 (unexportable final case, CLI).**
  - The size arithmetic holds. 128 cases at about 2.9 KB each plus 350,000 raw bytes gives a manifest of about 723 KB, under 1 MiB.
  - Case 127's raw pretty text is about 378 KB, so it passes the first `parse`.
  - Escaped, 175,000 × 6 = 1,050,000 bytes, which alone exceeds 1,048,576. The `display` cap therefore rejects it whatever the rest of the case contains.
  - Cases 0–126 export successfully first, so empty stdout proves there is no partial publication.
  - Asserting on the short `mori::core::json` prefix is robust to renderer wrapping, and the no-raw-U+0085 assertion on stderr is sound.
- **Original four CLI tests and three unit tests:** retained unchanged. My prior analysis still applies.

### Resolution of my initial findings

- **F1 (reader-profile widening): resolved.**
  - My R1 sentence is present verbatim in `docs/NATIVE-CORE.md`.
  - T1 pins the depth side.
  - The node-count widening is documented but not tested. By inspection it is the same per-case path. This is non-blocking residual N1.
- **T2 and T3 coverage gaps: resolved.** Hostile-name escaping and empty stdout on failure are now CLI regressions.
- **F3 (export failure exits 1, the same code as a mismatch): open, pre-existing, mitigated by documentation.**
  - The guide's qualifier "When every case report is exportable" correctly limits the status and exit promise.
  - It does not say what happens otherwise. Optional sentence (D1):
    > "If any case report cannot be exported, no report is written and the command exits 1; consumers must require complete parseable stdout."
  - Non-blocking.

### New low or informational findings

- **S1 (Low, spec wording).** The spec says "Smaller reports SHALL retain their existing pretty JSON bytes".
  - Reports under 1 MiB that legacy rejected for depth or node count have no existing bytes to retain. Such reports exist, because nodes can exceed 65,536 well under 1 MiB.
  - Suggested wording: "reports the previous whole-document export accepted SHALL retain those exact bytes."
  - Not a security defect.
- **S2 (Informational, record consistency).** `AUTHOR-OPUS-ADDITIONS.json` is stale in three ways:
  - It records `NATIVE-CORE.md` at 33,544 bytes with hash `e876…`, while `scope.json` records 33,582 bytes with hash `bb98…`. The 38-byte difference is exactly the length of Sol's "When every case report is exportable, " qualifier, so the records agree once that disclosed edit is accounted for.
  - It names `OPUS-FINAL-REVIEW.md`.
  - It says "execution pending root".
  - Treat `scope.json` as authoritative. Optionally annotate the additions record as pre-qualifier.
- **S3 (Informational, test-only).** `Project::new` uses a predictable temp-dir name.
  - `create_dir` fails if the path already exists, so a hostile local user can only cause a test failure, not take over the directory.
  - `remove_dir_all` is symlink-safe in current std.
  - No production impact.

---

## ACTUAL_RESULT_VERDICT: APPROVE (narrow)

### Hand consistency checks on the supplied evidence

**Raw test stdout byte counts match their receipts exactly:**

| Receipt | Bytes | Reconstruction | Shows |
|---|---|---|---|
| CLI run | 500 | 1 + 16 + 386 + 1 + 95 + 1 | Six named tests; final blank lines retained |
| Unit run | 509 | 1 + 16 + 395 + 1 + 95 + 1 | Four tests; "21 filtered out" |
| Unit list | 394 | 371 + 1 + 22 | Exactly four names, matching `review-followup-unit-selection.json` |

The listed names equal the T1 name plus my three original unit-test names.

**Binary provenance timeline.** Times are derived from `mtime_ns` offsets against the oracle's 00:38:07.92 UTC.

| Event | UTC time |
|---|---|
| `test.rs` and integration test written | 01:20:55 |
| CLI test run | 01:21:53.70 – 01:22:18.32 |
| Binary mtime (inside the CLI run, so rebuilt there) | 01:21:57.51 |
| Capture compile-only `--no-run` (no-op) | 01:22:35.3 – 01:22:36.2 |

- The binary hash and mtime are identical before and after the compile-only step.
- The capture checks the binary hash before and after every invocation.
- The named source hashes are identical at all three checkpoints and equal `4d8bc187…` and `74e021ed…`.
- The image `89ae66…` equals the initially captured image. That is consistent with a production prefix that has not changed.

**Output identity.**
- Small: the captured stdout hash `b57043f1…` equals the SHA-256 pinned in the committed test and the test's 28,093-byte length. This cross-checks the frozen oracle, the test and the actual capture.
- Mixed minus large is 3 bytes, exactly three `true` → `false` changes.
- `final-normal-output-equality.json` records raw-byte identity with the initial capture for all four suites.
- The frozen oracle `3911ef…` is checked before the run, and the frozen inputs are checked before and after.

**FeePool (your mathematical correction, accepted).**
- My earlier statement inferred per-case cap compliance from the roughly 360 KB average. That was a non sequitur: an average below the cap does not bound every case.
- The correct basis is the source plus the result. `aggregate_report` returns an error if any case fails `export`. The actual run exited 0 with complete parseable stdout, and the integration test independently asserts 87 cases with matching posts and effects. So every one of the 87 cases was individually exportable in this run.
- Nothing is claimed for other inputs.

**Red evidence (unchanged).**
- All cases were evaluated, then the run hit `JSON exceeds 1 MiB` with exit 1.
- The mixed red fails on missing stdout after its exit and count assertions passed. That is exactly the F3 collision, discriminated only by stdout.

### Residuals and limits (explicit, not broadened)

- **L1 (hash binding).** I recomputed no hash. The capture ran on an uncommitted tree at HEAD `69f3258d`.
  - It binds to `95df7355` only through per-file SHA-256s, which match `scope.json`.
  - `NATIVE-CORE.md` and `scope.json` are not in the capture's source set. They are documentation and metadata, not compiled.
  - The projection hash and the separate root and Muse checks are accepted as supplied.
- **L2 (FeePool scope).** Only the 87 accepted FeePool cases ran.
  - Only `post`, `effects`, order and name are compared, and trace checks are presence only.
  - The 75 rejected cases are preserved membership only and were not newly executed.
  - The normal fixture's trace is checked literally in full.
- **L3 (test scope).** The 21 other binary unit tests, `fee_pool_component`, `init` and broad suites were not run.
  - The T2 and T3 hostile and failure paths are covered by the cargo CLI run only, not by the `actual-r2` capture.
- **R2 (memory).** My F2 heap bound, an adversarial worst case of about 1.5–2.5 GB, is source analysis, not a measured heap.
  - It is unchanged because production is unchanged.
  - No cap was added. Untrusted test directories are not a protected boundary.
- **R3 (build).** Pinned third-party build scripts and proc-macros were not inspected, and only one `serde_json` feature configuration was exercised.
- **R4 (consumers).** No consumer of `mori test` stdout was supplied. MCP, LSP and other readers must not treat the report as reader-profile input (F1, now documented).
- **N1, D1, S1–S3:** non-blocking, as above.

Native authority remains zero. Neither verdict covers any SDK relation, check, model or degree, VM, key, proof, ledger, private material, cache or target qualification. Neither verdict closes a milestone: U0–U7, M10, MC08 and every parent exit remain open. Both approvals are limited to the `mori test` aggregate report path at this exact SHA and the supplied results.
