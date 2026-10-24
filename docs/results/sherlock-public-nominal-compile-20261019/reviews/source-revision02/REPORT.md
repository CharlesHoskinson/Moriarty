# Review: nominal `mori compile` seam, revision02

**Reviewer:** Opus 5.5 HIGH, non-author.

**Method.** I read only the supplied text. I ran no commands, Git, compiler, tests, providers, native tools or SDK, and opened no expected bodies. Every statement below is a static reading of the supplied text unless it is explicitly labelled as a receipt fact taken from that text. Nothing here is an execution observation.

## Source verdict: APPROVE (scoped)

- **Scope of approval.** I found no blocking source defect in the revision02 candidate for its stated predicate. The predicate is: the explicit nominal selection on the actual selected handler reaches the bounded parser, then `compile_nominal_host`, then publication, with honest dispositions.
- **What earlier reviews still mean.** The earlier `SOURCE_REQUIRES_CHANGES` (Opus7445) remains a historical record of revision01. This verdict covers revision02 only.
- **What this verdict is not.** It does not make any operation eligible and is not a resource disposition.
- **Conditions.** The approval depends on two dependency facts that live outside the supplied files (C1, C2 below).
- **Before merge.** Three documentation corrections are needed (D1–D3). They do not block the ordinary operations.

### Pre-operation citation conditions

These are not defects in the candidate. If either citation fails, the verdict becomes REQUIRES_CHANGES with the stated fix.

**C1. `mori_core::json::export` signature.** `crates/mori-core/src/json.rs` is not in the manifest.

- The new test 06 requires `export(&CompileFailure)` to return `Result<S, E>`:
  - `S` must deref to `str`, because of `display::text(&diagnostic)`. In practice `S = String`.
  - `E` must implement `Debug`, because of `.unwrap()`.
- The handler only requires `S: Display` and `E: Display`, so the handler's own use does not establish either requirement.
- **Action:** cite the exact signature before op02.

**C2. `mori_diagnostics::Report` Display.** Test 06 asserts `error.to_string() == "mori::command: …"`.

- This holds only if `Report`'s non-alternate `Display` prints just the wrapped error's `Display`, as `miette::Report` does. It must not add a code, help text or a fancy-render prefix.
- `mori_diagnostics` is not supplied.
- **Action:** cite the type or alias. If it renders more than that, compare `format!("{}", …)` against the inner text instead.

## Specific checks

| Check | Finding (static) |
|---|---|
| **B1: trait implementation fields, widths, order** | Correct. `owner_ext_admit.rs` defines `PreparationReportC { work_map: Vec<WorkMapEntryC> }` and `WorkMapEntryC { source_match: usize, live_arms: Vec<usize>, field_path: Vec<usize>, events_per_visit: Vec<RuleIdC> }`.<br>• All fields are `pub`.<br>• Both types are re-exported through `shared.rs` and reach `wire.rs` via `use …::shared::*`.<br>• The new impls destructure without `..`, so any later field change becomes a compile error. That is the right convention.<br>• `usize` maps to a decimal string (`decimal_wire!`). `Vec<T>` maps to an array. `RuleIdC` maps to its variant name.<br>• No coherence conflict with the `&T` or `Vec<T>` blanket impls.<br>• In `nominal_command.rs:137` the call on `&PreparationReportC` resolves to the new impl.<br>• Key order: `json!` builds a `serde_json::Map`, which is a BTreeMap because `preserve_order` is not in the workspace features. Output keys are therefore sorted. This matches every other `OwnerWire` projection. |
| **Every pre-existing wire byte** | Consistent with append-only.<br>• The hunk is `@@ -1482,3 +1482,37 @@`: 3 unchanged context lines and 34 pure `+` lines. I counted the added lines and got 34.<br>• 76,607 − 75,164 = 1,443 appended bytes, which is plausible for that text.<br>• I cannot recompute hashes. The claim that the 75,164-byte prefix is byte-identical, including the 23-field `ProgramView` projection, is author-attested and statically consistent with the patch. |
| **Non-empty report unit test** | Meaningful as a serialization check. It covers two ordered entries, preserved non-ascending `live_arms` [2,0], a non-empty path, named events, and empty members.<br>• `use super::*` picks up the parent's `json!` import and the `shared::*` names.<br>• The events it uses (`KMachineEvalMatch`) are never produced by Pass P. That is acceptable because the test is projection-only.<br>• It is `#[cfg(test)]`, so it is correctly absent from the normal rlib and from all 11 operations.<br>• To run it, a Core `cfg(test)` build is needed. That build's recorded consumer-compatibility inventory (27 sites plus 6 `TypeIdC` mismatches, dated Oct 19) must be resolved first if it applies to this base. |
| **Rendered diagnostic test 06** | Now discriminating.<br>• The handler's MissingEntry payload is built by `json!` with insertion order `selection_error, source_profile, entry, action`. The test builds the same keys in the same order, and the observe payload is `{"message":…}` on both sides.<br>• The `Value`s are equal under either map backend. Since the expected value goes through the same typed `CompileFailure` → `json::export` → `display::text` path, field order and escaping match.<br>• Pointing at a missing config proves that selection and observe refusal happen before config I/O. Otherwise the stage would be `config`.<br>• `!out.exists()` proves no directory was reserved.<br>• Remaining premises are C1 and C2. |
| **Cargo / direct-rustc fixture portability** | Correct.<br>• `option_env!("CARGO_MANIFEST_DIR")` is a compile-time value. Under Cargo it is the package root, giving `tests/fixtures/…`.<br>• Under direct rustc with the listed env (PATH, LANG, TMPDIR only), it is absent, `file!()` is the absolute argv path, and the assertion holds.<br>• Cargo does not auto-build `tests/support/*.rs`, so the capture file creates no new Cargo target.<br>• Root must use a genuinely clean environment (`env -i`-equivalent). A leaked `CARGO_MANIFEST_DIR` would silently redirect fixtures. Check that `consumer.d` records `# env-dep:CARGO_MANIFEST_DIR` with no value. |
| **`SourceSelection` exhaustiveness and unknown-profile tests** | Adequate.<br>• Active consumers are `compile_sherlock.rs` (a full three-arm `match`) and `simulate_selection.rs` (an or-pattern). `draft_contract_tests.rs` only uses `matches!`.<br>• Any exhaustive match would have to name the enum, so the text search is sufficient.<br>• No active test treats `nominal/1` as unknown. The new test uses `nominal/2`.<br>• Unrecorded behaviour change: simulating with the nominal profile previously gave a structured `UnknownProfile`. It now gives `input("select", "…supports the explicit dedicated draft entry")`. This is not tested, so record it as a known change. |
| **`hash_bytes` / hex binding** | Confirmed against the supplied `intent.rs`: `hex(&Sha256::digest(bytes))`, each byte formatted `{b:02x}`, no prefix.<br>• Cross-check: the AUTHOR-EXPECTATIONS digests `35158e22…` and `ac840364…` equal the BEFORE-PINS file hashes of the two fixtures.<br>• `read_source` hashes the UTF-8 bytes unchanged. Prior finding 4 is resolved. |
| **Amendment of the two historical dependency docs** | Explicit and honest at the record level. Before-images and pins are retained, the original six-file record and worktree are frozen, and the four code/test/fixture dependencies match the six-file pins. See D1 for one wording problem. |
| **New capture: same handler and serializer** | Same production source bytes (`cli_args/compile.rs`, `source_io`, `publish_io`, `compile_sherlock`, `display`, `styles`, `status_output`), linked to the same new `mori_core` rlib.<br>• It emits `fs::read(NEWDIR/sherlock-artifact.json)` bytes plus one newline, with no projection or re-serialization, and reads no expected file.<br>• It is a separately compiled binary. It is not the backend-linked `mori` CLI, and it bypasses `commands/compile.rs::run` and the real top-level clap parser.<br>• The artifact holds no output path, pointer or map-ordering nondeterminism, so its bytes do not depend on NEWDIR. |
| **Proposed caps unchanged; prior failed cut** | The caps are identical to the attempt that failed at the 30-second wall on a strict subset, and this candidate adds work. Running it unchanged would be a blind retry and is **not eligible**. See the operation table. |

## Prior-review items

| Item | Status |
|---|---|
| B1 | Resolved, statically. |
| 1. Fixture paths | Resolved. |
| 2. Test 06 not discriminating | Resolved, subject to C1 and C2. |
| 3. Artifacts not saved | Addressed by the capture, which is unrun. The tests still never inspect plan, case, field-path or preparation tables. |
| 4. Digest format | Resolved. |
| 5. Stage documentation | Partly addressed; see D2. |
| 6. Ill-typed body published as admitted | Documented limitation. Not added. Acceptable as optional. |
| 7. Stale docs | Amended; see D1 and D3. |
| 8. Merge preconditions | Addressed by the searches. Static completeness only. |

## Pre-merge documentation corrections (not operation blockers)

**D1.** `nominal-text-source.md` dropped "complete nominal admitted image/MatchPlan" from the "remain open" list.

- No nominal-text admitted image or MatchPlan has been observed. Removing the item reads as progress the evidence does not support.
- **Fix:** restore it, for example as "observed nominal admitted image/MatchPlan".

**D2.** The reference and explanation docs say `admit` covers B/T, C/P and V checks. It also carries the admission preconditions:

- AD-0a/AD-1/AD-2/AD-3/AD-6 refusals from `admit_ext_program`, such as `step_cap` 0 → `CapacityNotAdmitted StepCap`, and `max_arity` < 2.
- The finance-syntax refusal in Pass B.

The docs should name these.

**D3.** Humanizer nits:

- Over-long lines in `nominal-text-source.md` (around line 40) and `nominal-public-command.md` (line 18).
- The present tense "retains complete published artifacts" for an unrun capture in `nominal-compile-command.md` would read better as a prospective statement.

## Static note on the preparation wire for mutual-generic

This is my own derivation from the compiler source. It is **not** an oracle and not independent semantic evidence.

- `unwrap`'s match is expression 7, with arm row 0.
- Pass P should emit four nodes, in order Switch → Bind → Select → NoMatch.
- `work_map` should therefore be:
  - `{7,[0],[],[Switch]}`
  - `{7,[0],[0],[Walk,Bind]}`
  - `{7,[0],[],[Select]}`
  - `{7,[],[],[NoMatch]}`
- `plan_cases` should be `[1]` and `field_paths` should be `[0]`.
- wrapped-countdown should give `{"work_map":[]}`.

No test and no existing expectation checks the mutual-generic `preparation` section. The capture will be its first and only observation. Record that gap explicitly when the results are dispositioned.

## Per-operation opinion

All operations remain unrun. No Root resource disposition is selected. This review authorizes no operation.

Conditions common to all eleven:

- Run serially and stop at the first failure. No retry and no prefix replay.
- Use fresh, empty `build/`, `tmp/` and `output/` parents. NEWDIRs must be absent.
- Pin every source, tool and extern input before and after each operation.
- Capture full streams. Pin outputs only after exit 0 with complete streams.

| Op | Opinion and exact conditions |
|---|---|
| **01** normal Core compile | **Not eligible as specified.**<br>1. Root must choose a *changed* resource disposition, justified by the actual receipts of the prior failed attempt (wall time, CPU consumed, and memory/output if recorded). Identical `30s / --cpu=25 / --as=2GiB / --fsize=64MiB` on a strict superset of that work cannot run.<br>2. Check `--as=2GiB` against rustc's virtual-memory reservation for codegen threads.<br>3. Check `--fsize=64MiB` against expected `libmori_core` rlib size at `opt-level=0, debuginfo=0`, using earlier successful builds.<br>4. Re-pin `k256, mori_lexer_r191, serde, serde_json, sha2, thiserror`. Confirm that the `serde_derive` and `thiserror_impl` proc-macro dylibs on `-L 24-deps` were built by the exact 1.99.0 toolchain. Confirm that r191 was built against the `mori_diagnostics`, `mori_span` and `soa_rs` instances on `-L`, with no duplicate same-SVH candidates across `-L` directories.<br>5. Inspect `core.d` and confirm it contains the six revision02-relevant sources, including `wire.rs` at `4421605b`. |
| **02** handler-test link | Conditional on 01 exit 0 and C1.<br>• The extern set must equal the 882cba recipe except for `mori_core`.<br>• `serde_json`/`serde` must be the identical files `mori_core` was built against. Test 06 builds `CompileFailure { actual_error: serde_json::json!(…) }`, so a second instance is a type error.<br>• `clap` and `anstyle` must be the same instances; `clap_derive` must be on `-L 299`.<br>• `CARGO_MANIFEST_DIR` must be absent (check the `consumer.d` env-dep).<br>• FSIZE must fit the test executable.<br>• Align the wrapper order with 01 (timeout outermost). This is not functionally required, but it keeps receipts uniform. |
| **03** `--list` | The list must be exactly the five `expectedTestNames` and nothing else. Low risk. |
| **04** recursive positive | `TMPDIR` must exist. Expect `1 passed / 0 failed / 0 ignored / 0 measured / 4 filtered out`. The digest assertion is now meaningful. The test checks shape, counts and names only; no full-table equality, and the artifact is deleted. |
| **05** mutual-generic positive | Same as 04. This is the first nominal Construct→MatchPlan path through this compiler, and the expectations are author-derived. `preparation` is not asserted. |
| **06** selection refusal | Depends on C1 and C2. If both hold, it is a real discriminating check that refusal precedes config I/O. A failure here localizes to Report/export rendering, not to selection. |
| **07** structured payloads | Unchanged. Calls `nominal_command::compile` directly and does not observe handler rendering. |
| **08** selection contract | A pure `select_source` check. It does **not** establish old Core/draft artifact byte compatibility. |
| **09** capture link | Conditional on 01 and the 02 extern conditions, plus `anstream` and its transitive deps on `-L 299`.<br>• It is a binary crate (`main`), not `--test`.<br>• Compare `capture.d` with `consumer.d`: the `#[path]` production sources must be byte-identical. That comparison is the source-level basis for the "same handler and serializer" claim.<br>• FSIZE must fit the binary. |
| **10 / 11** capture runs | Each needs:<br>• Exit 0.<br>• Exactly one "Prepared" status line on stderr.<br>• NEWDIR containing exactly `sherlock-artifact.json`, with no leftover `.mori-proof-*.tmp`.<br>• `stdout == file bytes + "\n"`, verified by Root.<br><br>Expected artifacts are about 15–30 KB, so the 131,072-byte stream cap should not truncate. Any truncation, a panic (exit 101, for example from a wrong argument count) or partial publication means stop with no comparison.<br><br>**Capture full-freeze**, before **any** read of a protected expected or derivation body:<br>1. Hash, fsync and freeze (mode 0400) both artifacts, both stdout/stderr streams, both binaries and all dep-info files.<br>2. Freeze the wire-to-snapshot normalization map *before* comparing. It must be derived from `wire.rs` source, not from the actual output. This covers decimal strings, `TypeIdC` as "UInt" vs `{"TypeId":…}`, `RawConstRow` vs `ConstantSnapshot`, and the `TypeExprSrc` encoding.<br>3. Only wrapped-countdown has an existing independent full-table expectation. The mutual-generic capture is a full observation, not independent approval.<br>4. The CLI fixture's `layout_version` (`nominal-public-command`) differs from the mori-core test's, so identity comparison must not be conflated with `program_view` comparison.<br><br>Reading `AUTHOR-EXPECTATIONS.json` in ops 04/05 is not a protected read. |

Also outside the eleven and still unrun:

- The frozen dependency's three mori-core tests, including the only full `ProgramSnapshot` equality test (wrapped-countdown).
- The `wire.rs` `cfg(test)` unit test.

## Preserved limitations (unchanged by revision02)

- **Unchanged areas:** no security-owned guard, parser budget, typechecker depth, existing profile or config, or K/Quint/Clean-Lean/circuit/native source changed.
- **Representation:** HostOnly only.
- **Plans:** witness and constraint plans are missing; native export is unsupported; native and target evidence are unrun.
- **Typing:** `body_typechecking: not-performed`, and an ill-typed body can still be published as admitted.
- **Source metadata:** source name and spans are absent; semantics are `null` and unqualified.
- **Wire:** the 23-field `program_view` wire is retained.

**Not certified by this review or by any of the eleven operations, even if all succeed:**

- body typing
- independent full-table equality
- old-profile artifact byte compatibility
- Cargo or DoctorWatson integration
- backend-linked CLI
- native, target, EARS, B04 or H1 acceptance