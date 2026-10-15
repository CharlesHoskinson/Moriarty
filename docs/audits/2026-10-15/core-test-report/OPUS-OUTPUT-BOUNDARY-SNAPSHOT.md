# U7/M10/MC08: security decision and helper for the `mori test` aggregate

**Decision: approved within the supplied source.** The approach is a private aggregate helper in `crates/mori/src/commands/test.rs`. I ran no checks; everything below comes from reading the source you supplied. You asked for no tools, so I did not write a plan file.

## Implementation contract

1. **Variable content comes only from the unchanged exporter.** Each complete case `Value` goes through `mori_core::json::export`. That call keeps its existing parse, depth and node limits, its 1 MiB per-document cap and its ASCII/control escaping. The helper adds only fixed structural bytes: braces, brackets, `", "`, newlines, indentation, the `"status"`/`"cases"` keys and the literal `"passed"`/`"failed"`. It does no escaping of its own.
2. **Small outputs stay byte-identical to the legacy output.** This holds for three reasons:
   - Escaping is per-character and context-free, so escaping each case separately gives the same bytes as escaping the whole document.
   - `serde_json`'s pretty formatter writes nested values exactly as standalone values plus the parent indentation.
   - Wrapper key order comes from `serde_json::Map` itself, not hardcoding. It is the same Map type the legacy `json!` wrapper used, so it is correct with or without `preserve_order`.
3. **Re-indenting is safe.** Pretty JSON never puts a raw newline inside a string, and `export` escapes all controls except `'\n'`. So every raw `'\n'` in a case export is structural, and indenting after it cannot change any string value.
4. **Everything is checked before stdout is touched.** All cases are exported and the full aggregate is built in memory before anything is written. Any case export failure returns `Err` with no stdout. Nothing is truncated, dropped or reordered, and status and exit code are unchanged. The final newline comes from the existing `println!`.
5. **Stdout errors behave as before.** If stdout fails mid-write (for example a broken pipe), `println!` panics exactly as it does today. Stdout is not promised to be atomic.
6. **Existing limits are unchanged.** The 256-case manifest limit and the per-case 1 MiB export cap still apply. The shared `mori_core::json` code is not modified.
7. **The aggregate is no longer a reader-profile document.** Above 1 MiB it is not readable by `mori_core::json::parse`, which is intended. Each case passes reader-profile validation on its own. Inside the aggregate a case sits two levels deeper, so total depth is at most 34. The legacy whole-document check had limited case depth to 30.

## Aggregate resource bound (derived, not a new cap)

- Each case export is at most 1,048,575 bytes, enforced by the unchanged `export`.
- Pretty JSON never starts with a newline and never has two newlines in a row. So a case has at most half as many newlines as bytes.
- Re-indenting adds exactly 4 bytes per newline. An indented case is therefore under 3 × its exported size, which is under 3 MiB.
- **Aggregate size is under 256 × (3 MiB + 6) + 40 bytes, about 768 MiB, as the theoretical worst case.** This follows only from the existing 256-case and per-case 1 MiB limits.
- A realistic indentation overhead is about 2×. The observed legitimate maximum is about 7.18 MB.
- I add no smaller cap, because any cap below this bound could reject legitimate 256-case reports.
- Peak memory is the `Value`s the command already holds (as in legacy) plus one aggregate `String`. Each case's exported text is dropped right after it is appended.

## Exact helper and security tests (Opus-authored; append to `test.rs`)

```rust
/// Pretty report identical to exporting the whole value, without applying the
/// single-document 1 MiB cap to the aggregate. Every case is exported, bounded
/// and escaped by the unchanged mori_core::json::export; only trusted
/// structural bytes are added here. Nothing is written until all cases export.
fn aggregate_report(passed: bool, mut cases: Vec<serde_json::Value>) -> super::Result<String> {
    // Field order follows serde_json::Map exactly as the legacy json! wrapper did.
    let mut order = serde_json::Map::new();
    order.insert("status".into(), serde_json::Value::Null);
    order.insert("cases".into(), serde_json::Value::Null);
    let mut out = String::from("{");
    for (field, key) in order.keys().enumerate() {
        out.push_str(if field == 0 { "\n  " } else { ",\n  " });
        if key == "status" {
            out.push_str(if passed {
                r#""status": "passed""#
            } else {
                r#""status": "failed""#
            });
            continue;
        }
        out.push_str(r#""cases": ["#);
        let count = cases.len();
        for (index, case) in std::mem::take(&mut cases).into_iter().enumerate() {
            let case = mori_core::json::export(&case).map_err(core_io::operation)?;
            out.push_str(if index == 0 { "\n    " } else { ",\n    " });
            // Pretty JSON escapes newlines inside strings, so every raw newline
            // is structural and takes the enclosing array-element indentation.
            out.push_str(&case.replace('\n', "\n    "));
        }
        out.push_str(if count == 0 { "]" } else { "\n  ]" });
    }
    out.push_str("\n}");
    Ok(out)
}

#[cfg(test)]
mod report_tests {
    use super::aggregate_report;
    use serde_json::{Value, json};

    const HOSTILE: &str = "x\n}\"\\\r\u{1b}[31m\u{7f}\u{85}\u{202e}\u{1f9ea}";

    fn case(index: usize, padding: usize) -> Value {
        json!({
            "name": format!("case-{index:03}{HOSTILE}"),
            "passed": index % 2 == 0,
            "actual": {"Ok": {
                "pad": "\u{202e}".repeat(padding),
                "empty": [],
                "none": {},
                "nested": [[{"k": [1, -2, 3.5, null, true, HOSTILE]}]]
            }}
        })
    }

    fn whole(passed: bool, cases: &[Value]) -> Value {
        json!({"status": if passed { "passed" } else { "failed" }, "cases": cases})
    }

    #[test]
    fn small_aggregate_is_byte_identical_to_whole_export() {
        for passed in [true, false] {
            for count in [1, 2, 3] {
                let cases: Vec<_> = (0..count).map(|index| case(index, 1)).collect();
                let legacy = mori_core::json::export(&whole(passed, &cases)).unwrap();
                assert_eq!(aggregate_report(passed, cases).unwrap(), legacy);
            }
        }
    }

    #[test]
    fn full_256_aggregate_exceeds_whole_cap_and_round_trips_safely() {
        let cases: Vec<_> = (0..256).map(|index| case(index, 1024)).collect();
        let expected = whole(false, &cases);
        assert!(mori_core::json::export(&expected).is_err());
        let text = aggregate_report(false, cases).unwrap();
        assert!(text.len() > 1_048_576);
        assert!(text.is_ascii());
        assert!(text.bytes().all(|b| b == b'\n' || !b.is_ascii_control()));
        assert!(text.ends_with("\n}"));
        assert_eq!(serde_json::from_str::<Value>(&text).unwrap(), expected);
    }

    #[test]
    fn any_individually_oversized_case_fails_before_output() {
        // Raw pretty size over 1 MiB, then raw size under 1 MiB but escaped size over it.
        for pad in [json!("x".repeat(1_048_576)), json!("\u{202e}".repeat(200_000))] {
            let mut cases: Vec<_> = (0..256).map(|index| case(index, 1)).collect();
            cases[255]["actual"]["Ok"]["pad"] = pad;
            assert!(mori_core::json::export(&cases[255]).is_err());
            assert!(aggregate_report(true, cases).is_err());
        }
    }
}
```

**What the tests must cover:**
- Byte-for-byte equality with the legacy whole-document export, covering both statuses, 1 to 3 cases, hostile Unicode and controls, and empty containers.
- A 256-case aggregate over 1 MiB that is fully ASCII, contains no controls except newline, and round-trips exactly in order.
- Per-case cap enforcement on both exporter paths, with no `Ok` returned even when the failing case is the last one.

**Callsite for Sol (not security-sensitive).** Replace the `core_io::print(...)` call after `shell.status(...)` with:

```rust
let report = aggregate_report(failed == 0, reports)?;
println!("{report}");
```

Keep the existing order: status first, then the aggregate, then the exit code. Your existing `crates/mori/tests/core_test_report.rs` serves as the CLI-level regression test.

## Command classification

**1. `cargo +1.99.0 --offline --locked test -p mori --test core_test_report -j2 -- --test-threads=1`: cleared as ordinary compilation plus pure Core reference execution, with the conditions below.**
- At run time the test spawns only `CARGO_BIN_EXE_mori --color never test <tmpdir>`.
- Per the supplied source, `test::run` reaches only `core_io` reads, `mori_core::{parse, simulate, json}` and `operation`.
- `mori-core` has no Midnight or SDK dependency, so it cannot reach the native SDK.
- The fee-pool test only feeds example files through the same `test` subcommand.
- Linking `mori-backend`, `mori-ledger` and the `midnight-*` dev-dependencies is build/link only and does not authorize calling them.
- **Conditions and residuals:**
  - **Source needed:** `crates/mori/src/commands/mod.rs` (the `run` dispatch and its `Test` arm), `cli.rs` (global argument handling) and `Shell::new`. These must confirm that the `test` subcommand performs no setup work before `test::run`. Without them, this clearance is conditional.
  - **Life-before-main:** check that no linked crate registers startup constructors (`ctor`, `inventory`, `linkme`, `.init_array`). A read-only search of `Cargo.lock` for these is enough; otherwise this remains a residual assumption.
  - **Build-time residual:** compiling runs third-party build scripts and proc-macros from the pinned dependencies, including the SDK crates. The git-tracked workspace has no `build.rs`, but these scripts have not been freshly inspected. That is native code execution at build time, not an SDK operation. If your policy requires inspection, you need each locked build-script crate's `build.rs` from the registry source at its locked version. `cargo metadata --offline --locked` (no build scripts run) can list them.

**2. Formatting check: cleared.** Run `rustfmt +1.99.0 --check --edition <workspace edition> crates/mori/src/commands/test.rs crates/mori/tests/core_test_report.rs`. It reads only; there is no build, linking or build-script execution. `rustfmt.toml` is picked up automatically.

**3. New unit tests (separate command): cleared with the same conditions as command 1.** The unit tests live in the binary crate, so `--test core_test_report` does not run them. First run `cargo +1.99.0 --offline --locked test -p mori --bin mori -j2 -- --list commands::test::report_tests::` and confirm exactly three names are listed. Then run `cargo +1.99.0 --offline --locked test -p mori --bin mori -j2 -- commands::test::report_tests:: --test-threads=1`. It compiles every unit test in the binary but runs only the filtered ones, which call only `aggregate_report` and `mori_core::json`.

**4. `init` test: not cleared.** Its source and the commands it spawns (the `check`/`simulate` paths) were not supplied. To clear it, I need the test function in `crates/mori/tests/init.rs` plus the source of every subcommand it invokes.

**Not cleared:**
- `fullfee_pool_component`, any broad cargo suite, and any native, SDK, ledger or proving operation.
- Running a previously built binary. Its provenance against the unchanged product source is not established.
