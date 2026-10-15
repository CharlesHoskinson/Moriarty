# Security review: `mori test` aggregate report repair

**Candidate:** `69f3258db08c…` (base `9dffbb0a…`)
**Projection:** `c3f64d70…`

**Basis:** I worked only from the source and evidence supplied in this message. I used no tools and wrote no plan file, because you asked for no tools or execution. I did not recompute any digest, read any repository file or run anything. Every hash below is as recorded in your evidence.

**Scope:**
- `aggregate_report` and its callsite in `crates/mori/src/commands/test.rs`.
- The three `report_tests` unit tests.
- `crates/mori/tests/core_test_report.rs`.
- Their interaction with the unchanged `mori_core::json::{parse, export}`, `core_io::operation` and `Shell`.
- The red, green, unit and capture evidence for those files.

**Not in scope:** evaluator correctness, manifest path policy, native, SDK or ledger paths, MCP/LSP, plugins, and parent exits. U7, M10 and MC08 stay open.

---

## SOURCE_VERDICT: APPROVE (narrow)

### Properties I checked against the exact code

1. **Variable bytes come only from `export`.**
   - Each case `Value` passes through the unchanged `mori_core::json::export`.
   - The helper adds only fixed ASCII:
     - braces and brackets;
     - `,` and newlines;
     - 2- or 4-space indentation;
     - the keys `"status"` and `"cases"`;
     - the literals `"passed"` and `"failed"`.
   - No caller-controlled byte bypasses `export`.

2. **Re-indenting cannot change any value.**
   - `export` writes ASCII only. Every ASCII control except `\n` becomes `\uXXXX`, and DEL (0x7f) counts as a control.
   - `serde_json` pretty output escapes `\n` inside strings.
   - So every raw `\n` in a case export is structural. Adding four spaces after it changes only whitespace.

3. **Small reports are byte-identical wherever legacy succeeded.** If the legacy whole-document export succeeded, the new output is the same bytes. The reasons:
   - The escaping loop maps each character independently of its context.
   - `serde_json`'s pretty formatter writes a nested value as its standalone form plus the parent indentation.
   - Separators match: `[` then `\n    `, then `,\n    `, then `\n  ]`.
   - Wrapper key order comes from the same `serde_json::Map` type as the legacy `json!` call, so it matches with or without `preserve_order`.
   - If the whole document passed `parse`, each case passes too, because each case is a smaller, shallower subdocument.

4. **Fail-closed publication.**
   - Every case is exported before `println!`. Any `export` error returns through `?`, so stdout gets no bytes and stderr gets a report sanitized by `display::text`.
   - Nothing is truncated, dropped or reordered. Comparison, status and exit selection are untouched.
   - If writing to stdout fails, `println!` panics with possibly partial output and exit 101. That matches legacy behaviour, and stdout is not promised to be atomic.

5. **Terminal safety.**
   - Stdout is ASCII with no control bytes except `\n`.
   - Bidi characters (U+202E), C1 controls (U+0085), ESC and surrogate pairs appear as visible escapes.
   - The stderr status line contains only counts. Case names never reach stderr.

6. **No collateral changes.** The 256-case manifest limit and the per-case 1 MiB, depth-32 and 65,536-node limits still apply. `json.rs` and `core_io.rs` are byte-identical to base, per `scope.json`. No other command changed.

### Findings

**F1 (Low, contract/documentation). Test output is no longer a reader-profile document, at any size.**
- Each case is now checked alone at depth 0. Inside the aggregate it sits two levels deeper.
- So `mori test` can now print reports that legacy rejected, even below 1 MiB. These reports fail `mori_core::json::parse`:
  - a case whose content reaches relative depth 31 or 32, giving aggregate depth up to 34;
  - a report whose total nodes exceed 65,536 while each case stays under the limit.
- The earlier Opus contract mentioned depth 34 but not the node widening, and the candidate's docs mention neither.
- This is not exploitable in the supplied source, because no reader of `mori test` stdout was supplied. It should still be stated so no future consumer treats the report as reader-profile input. The exact sentence is below.
- It does not block approval.

**F2 (Correction to the earlier Opus resource analysis).** The earlier claim was "Values plus one aggregate String". From the exact code, the peak is larger:

| Component | Bound |
|---|---|
| Remaining `reports` Values | Same as legacy; shrinks as the loop advances |
| Per iteration, inside `export` | Unescaped pretty `json` (≤1 MiB on success) and a transient `Value` from `parse(&json)`. Then the `display` string (≤1 MiB + 12 bytes) and a transient `Value` from `parse(&display)` |
| Per iteration, retained | The returned `case` String (≤1,048,575 bytes) lives at the same time as the `case.replace(...)` temporary (≤3L) until `push_str` copies it. The original case `Value` is shadowed, not dropped, until the iteration ends |
| `out` | Final length ≤ 256 × (3,145,723 + 6) + ~40 ≈ 805 MB. Amortized doubling can push capacity to about 2× length, and a reallocating copy can briefly hold both buffers |

- The 3× factor holds because pretty JSON never starts with a newline or contains two in a row. So a case has at most (L−1)/2 newlines and grows to at most L + 4n ≤ 3L.
- The theoretical adversarial peak is therefore about 1.5–2.5 GB of heap.
- This is a constant-factor increase over legacy, not new unbounded growth. Legacy already serialized the whole unescaped report before rejecting it, so its peak was the same order of magnitude.
- Reaching the bound requires 256 attacker-chosen cases, each near the 1 MiB cap. That attacker already controls the evaluated agreement and arbitrary manifest paths.
- `mori test` on an untrusted directory is not a protected boundary in this candidate.
- The largest observed report is 31.3 MB, from FeePool.
- I accept this with no new cap. It is residual R2.

**F3 (Informational, pre-existing, unchanged).**
- An export failure exits 1, the same code as an expectation mismatch. The stderr line "Tested N passed; 0 failed" is printed before aggregation, so it can appear just before an error. The red mixed run shows exactly this collision: exit 1 matched both ways and only the empty stdout exposed it. Consumers must require complete parseable stdout.
- Unrestricted `source` and `scenario` paths, including absolute paths and `..`, plus FIFO blocking, predate this change. The only new effect is that more already-computed results are now published.

### Security test coverage

The three unit tests are adequate for the helper:
- byte identity with hostile Unicode, controls, empty containers, both statuses and 1–3 cases;
- a 256-case aggregate over 1 MiB that is ASCII with no controls and round-trips in order;
- both per-case cap paths (raw size over 1 MiB, and escaped size over 1 MiB), each failing on the last case.

They do not cover:
- the per-case depth and node limits inside the aggregate;
- fail-closed behaviour with empty stdout at the CLI level;
- hostile-name escaping at the CLI level.

By inspection, the code paths for all three are the same as for the covered cases. The tests below would turn those inspections into regressions. Adopting them is recommended but does not block approval.

---

## Optional corrective patch (authored by me)

**R1. Documentation for F1.** In `docs/NATIVE-CORE.md`, add this right after "Small reports retain their existing pretty JSON format.":

```text
The report is ASCII with every string escaped, but it is not a bounded
`mori_core::json` reader-profile document: it may exceed 1 MiB, depth 32 or
65,536 nodes. Each case is exported and bounded individually.
```

Only that file's hash in `scope.json` changes. Spec, clause and crosswalk hashes are unaffected.

**T1. Add to `report_tests` in `test.rs`.** Pins the per-case depth limit and documents the widening in F1.

```rust
    #[test]
    fn per_case_depth_limit_applies_and_aggregate_is_not_reader_profile() {
        // Case root is depth 0 under export, so the innermost scalar of
        // `actual` sits at nested + 1 and the reader accepts depth 32.
        let deep = |nested: usize| {
            let actual = (0..nested).fold(json!(0), |inner, _| json!([inner]));
            vec![json!({"name": "deep", "passed": true, "actual": actual})]
        };
        let cases = deep(31);
        assert!(mori_core::json::export(&whole(true, &cases)).is_err());
        let text = aggregate_report(true, cases.clone()).unwrap();
        assert!(mori_core::json::parse(&text).is_err());
        assert_eq!(serde_json::from_str::<Value>(&text).unwrap(), whole(true, &cases));
        assert!(aggregate_report(true, deep(32)).is_err());
    }
```

**T2 and T3. Add to `crates/mori/tests/core_test_report.rs`.**

```rust
const HOSTILE: &str = "x\n}\"\\\r\u{1b}[31m\u{7f}\u{85}\u{202e}\u{1f9ea}";

fn rename(project: &Project, index: usize, name: Value) {
    let path = project.0.join("mori-tests.json");
    let mut manifest: Value = serde_json::from_slice(&fs::read(path).unwrap()).unwrap();
    manifest["cases"][index]["name"] = name;
    project.write_json("mori-tests.json", &manifest);
}

#[test]
fn large_report_escapes_hostile_case_name_once() {
    let project = Project::normal(256, false);
    rename(&project, 128, json!(HOSTILE));
    let output = run(&project);
    let report = check_output(&output, 0, 256, 0);
    assert!(output.stdout.len() > 1_048_576);
    assert!(
        output
            .stdout
            .iter()
            .all(|&b| b == b'\n' || (b.is_ascii() && !b.is_ascii_control()))
    );
    let mut expected = expected_report(256, false);
    expected["cases"][128]["name"] = json!(HOSTILE);
    assert_eq!(report, expected);
}

#[test]
fn unexportable_final_case_publishes_no_partial_report() {
    // 350,000 raw UTF-8 bytes keep the manifest under 1 MiB; each U+0085
    // exports as six ASCII bytes, so only the final case exceeds its cap.
    let project = Project::normal(128, false);
    rename(&project, 127, json!("\u{85}".repeat(175_000)));
    let output = run(&project);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(1), "{stderr}");
    assert!(stderr.contains("128 passed; 0 failed"), "{stderr}");
    assert!(stderr.contains("mori::core::json"), "{stderr}");
    assert!(!stderr.contains('\u{85}'));
    assert!(output.stdout.is_empty());
}
```

**Size reasoning for T2:**
- 128 cases at about 2.9 KB each, plus 350 KB of name, gives a manifest of about 723 KB. That is under 1 MiB and far under 65,536 nodes.
- The case's unescaped pretty text is about 378 KB, so it passes the first `parse`.
- Escaped, it is about 1.08 MB, so the `display` cap rejects it.
- The assertion uses only the `mori::core::json` prefix, because the renderer may wrap the longer message.

**Command classification if these are adopted.** Pure Core with no native reach. Use only the already-cleared commands:
1. The focused `--test core_test_report` run. It should report 6 passed.
2. The `--bin mori -- --list commands::test::report_tests::` run. It should list exactly these four names:
   - `any_individually_oversized_case_fails_before_output`
   - `full_256_aggregate_exceeds_whole_cap_and_round_trips_safely`
   - `small_aggregate_is_byte_identical_to_whole_export`
   - `per_case_depth_limit_applies_and_aggregate_is_not_reader_profile`
3. The filtered unit run.

Same toolchain, lock, offline and locked settings, and job limits as before.

---

## ACTUAL_RESULT_VERDICT: APPROVE (narrow)

### Consistency checks I did by hand on the supplied evidence

**Raw stdout byte counts match their receipts exactly:**

| File | Receipt bytes | Reconstructed from text |
|---|---|---|
| `green.stdout` | 380 | 17 + 266 + 1 + 95 + 1 |
| `opus-unit.stdout` | 402 | 17 + 288 + 1 + 95 + 1 |
| `opus-unit-list` stdout | 293 | three `: test` lines + `\n3 tests, 0 benchmarks\n` |

This confirms the retained final blank lines and that exactly three tests existed in the filter.

**Output sizes are structurally consistent:**
- Mixed (7,181,101) minus large (7,181,098) is exactly 3 bytes, which is three `true` → `false` changes.
- Solving small (28,093) against large gives an integer per-case size of 28,050 bytes with one extra separator byte per case. That matches 256 identical-length names.
- FeePool averages about 360 KB per case, so every case is under the per-case cap.

**Binary provenance timeline is consistent:**

| Event | UTC time |
|---|---|
| Oracle file mtime (matches the stated freeze time) | 00:38:07.92 |
| Integration test mtime | 01:02:44 |
| `test.rs` mtime | 01:04:30.57 |
| Green run | 01:04:59.5 – 01:05:24.5 |
| Binary mtime (inside the green run) | 01:05:08.24 |
| Capture script mtime | 01:07:03 |
| Compile-only `--no-run` (no-op, 0.33 s) | starts 01:07:42 |

- Binary hash and mtime are identical before and after compile-only.
- The capture checks the binary hash before and after every invocation.
- Source hashes are identical before compile, after compile and after all invocations.

**Red evidence is consistent with legacy behaviour.** All cases were evaluated, then `JSON exceeds 1 MiB` with exit 1 and empty stdout. The small case passed. The failed attempts are retained and not counted as reds.

**Oracle integrity.**
- The oracle file was frozen before the product ran. The capture checks the oracle hash and the frozen input hashes before and after.
- Large and mixed results are compared by whole JSON value. Small is compared by exact bytes and a pinned SHA-256.
- Duplicate keys would be masked by both `json.loads` and `serde_json::Value`. By construction they cannot occur: the two wrapper keys are fixed, and every case passes `parse`, which rejects duplicates.

### Residuals and material limits

- **L1.** I recomputed no hashes and did not see the tree for `69f3258d`. The capture ran on an uncommitted working tree at base HEAD. It is tied to the candidate only through per-file SHA-256s, which match `scope.json`. The projection hash `c3f64d70…` is unverified by me. The `red-r3-result.json` receipt was not supplied.
- **L2.** `openspec/current-requirement-crosswalk.json` does not appear in the capture's `git_status`, so its hash refresh came after the capture. It is metadata only and is not compiled; acceptable.
- **L3.** FeePool coverage:
  - only the 87 accepted cases ran;
  - only `post`, `effects` and order are compared to the frozen corpus;
  - trace checks are presence only;
  - the 75 rejected cases were not run;
  - the corpus is a repository artifact, not independently derived except for the arithmetic subset.
- **L4.** Only one `serde_json` feature configuration was exercised. Key-order equivalence in other configurations rests on my source argument.
- **R2.** Memory per F2 is bounded but large in the adversarial worst case. No cap was added. Untrusted test directories are not a protected boundary.
- **R3.** Startup and build:
  - The constructor search covers `Cargo.lock` crate names only. A raw `.init_array` link section inside a dependency would not appear there.
  - Pinned third-party build scripts and proc-macros were not inspected. That is native code run at build time, not an SDK operation.
  - `mori-core` has Midnight dev-dependencies. Its normal dependencies, the ones linked into `mori`, do not include Midnight.
  - The `mori` binary links `mori-ledger` and `mori-backend`, but the `test` path does not call them.
- **R4.** `mcp.rs`, `lsp.rs` and other possible consumers of test output were not supplied. F1's impact assumes none of them reads `mori test` stdout as reader-profile input.
- **R5.** One case over its individual cap still suppresses the whole report, failing closed. This is a functional residual for Muse, not a security issue.

Neither verdict certifies native, ledger, target or milestone status, or broad security. U7, M10 and MC08 and all parent exits stay open.
