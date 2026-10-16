# Final review: Sherlock streaming export (source, actual result, publication)

**Reviewer:** Opus 5.5 high, the designated security owner. This is a static review of the pasted texts only. I ran nothing and hashed nothing. Every digest and size below is Root's on-disk claim, and I checked them only against each other.

| Verdict | Decision | Binding digest |
|---|---|---|
| Source and actual result | **APPROVE_WITH_SCOPE** | candidate `7f8cc109227ad49cbd6660d51933f453e8a4bdd2910ffd84242ffc6c2273783a`; result `56154a66fb9a6575cee9201338d1a4bbbb49a99a56481a35079d83afc8973e93` (28,685 bytes) |
| Publication | **APPROVE_WITH_SCOPE** | package `667e00c5232288de9b5b38e4321b6b9d428da0fb19e7ec413d4a5e101806cac7` (41 files, 247,940 bytes) |

I found no blockers.

---

## 1. Source and public API

**The production delta is unchanged from the approved review.**
- `sherlock/mod.rs` changes from `340782cc` to `bb0752ab`. That equals BASE plus `future-wiring.patch`.
- I checked the patch hunk headers against BASE line numbers:
  - `@@ -7,10 +7,12 @@` covers BASE lines 7–16.
  - `@@ -66,3 +68,6 @@` covers lines 66–68, ending at `mod layout_a;`.
  - The patch therefore applies cleanly to BASE at the repository-relative path.
- `reference_stream.rs` (`90316dea`) is new. It is byte-identical to the file used in the 8-test capture.
- The other 24 closure files are byte-identical to the prior bounded-source-01 Core inputs.
- `lib.rs` stays at `df45168f`.

**The API matches what was reviewed.** It adds:
- `mori_core::sherlock::Machine::visit_steps`.
- The re-exports `mori_core::sherlock::{VisitResult, VisitStop}`.

It adds no new authority, because `step`, `run_steps` and `run` are already public. My earlier findings S1–S5 stand unchanged:
- **S1 is a binding condition:** nothing on a CLI, LSP, MCP, `/1`/`/2` source-adapter or other untrusted path may call `visit_steps`.
- **S2:** an observer panic leaves the machine in the delivered post-state with no `VisitResult`. Info only.
- **S3:** `VisitResult` and `VisitStop` are exhaustive public types. Nonsecurity note.
- **S4:** stale "UNRUN" header text stays frozen.
- **S5:** coverage is normal-only. The gap is retained.

**H1 is unchanged.** `max_events` only groups events. Snapshot clones, helpers, callbacks, allocation and native stack remain unmetered.

## 2. Actual production compile: 8 attribution conditions plus 3 additional conditions

| # | Condition | Observed in the pasted record | Status |
|---|---|---|---|
| 1 | Outer exit 0, no traceback | `COMPILE-ATTRIBUTION`: exit 0, traceback false. This is operator-observed, since `result.json` cannot record it by design. | ✓ (attested) |
| 2 | `result.json` is 0444 and records `success` | `terminal_state: success`. The 0444 mode is the operator predicate `readonly_success_result: true`. | ✓ (mode attested) |
| 3 | expected manifest = manifest after = approved digest | Both are `7f8cc109…3783a`. | ✓ |
| 4 | All 5 postflights true; no `error` or `postflight_errors` key | All five are `true`. I enumerated the top-level keys: neither error key is present. | ✓ |
| 5 | Exactly 2 named steps, exit 0, not timed out, argv as specified | Step 1 is `env -i PATH LANG TMPDIR rustc -vV`. Step 2 is `--edition=2024 --crate-name mori_core --crate-type rlib <S>/lib.rs -L dependency=DEPS --emit=link,dep-info -o …` with six `--extern`s in sorted order (k256, mori_lexer, serde, serde_json, sha2, thiserror). The paths equal `dependency-manifest.json`. There is no `--test`, `--cfg test` or `-C`. | ✓ |
| 6 | dep-info inputs are exactly the 26 snapshot paths | I counted 26. They are sorted and equal `CONFIG.compile_closure` under `attempt-core-01/source/`. `01-core.d` lists the same 26 files and no test module. | ✓ |
| 7 | candidate/snapshot before/after equal the pins | All four maps have 36 keys and are identical. I spot-checked them against `CANDIDATE.json`, including `mod.rs`, `reference_stream.rs`, `lib.rs`, `runtime.rs`, `stage/validate.rs`, `source_adapter.rs` and the oracles. | ✓ |
| 8 | Prior Core result is bound | `prior_core_result` = `dd95f3e4…efbb`, 58,037 bytes. | ✓ |
| +1 | `result.helpers` lists the 7 helpers | sha, record, inventory, write_json, child, source_records, depinfo. Their recorded line spans (13–18, 20–21, 23–24, 26–28, 30–52, 54–55, 57–63) match `bounded_source_driver.py` when I count lines. The segment hashes are not independently verified. | ✓ |
| +2 | `compile_stderr_bytes` = 0 | 0. The stderr file is the empty-string digest. There were no warnings of any kind. | ✓ |
| +3 | `actual_core_library` recorded | `dff1c647…f4e4`, 15,688,664 bytes. It matches `RESULT.json` and `COMPILE-ATTRIBUTION`. | ✓ |

The 14 operator predicates in `COMPILE-ATTRIBUTION` are all true. They map onto the rows above.

**Further consistency checks:**
- The before/after inventory pairs are byte-identical: dependencies `35d50cc5…`, 2,326 files and 14,539,190,934 bytes; toolchain `c4a40b7b…`, 75 records.
- I counted the 75 toolchain records. `rustc`, `python3.14` and `dependency-manifest.json` match their pinned digests.
- The 8 selected dependency records equal `dependency-manifest.json`.
- The timeline is plausible:
  - `rustc -vV` started at 17:33:42.52.
  - The compile started at 17:33:51.55. The 9 s gap includes the 14.5 GB inventory.
  - Total elapsed time was 16.32 s.

**The 8-test result is separately audited and was not rerun.**
- Its source bytes are the same rows in this candidate: `reference_stream.rs` `90316dea`, `reference_stream_tests.rs` `4e8b66be`.
- The `#[cfg(test)] mod reference_stream_tests;` registration is present in the projected `mod.rs`.
- Whole-crate `cfg(test)` was **never compiled**. The 8-test evidence stays standalone `#[path]` std-host, normal-only.

**Residuals accepted again:**
- The window between the interpreter reading the driver and the driver hashing itself.
- Proc-macro selection being pinned only by the full DEPS inventory. dep-info does not record which `.so` was loaded.
- No time cap on the parent's inventory work.
- No memory, disk or process-count cap.
- Trust in the local cached artifacts, sysroot, linker, libc, Python stdlib and kernel.
- Unestablished transitive source authenticity.

## 3. Integration conditions (Root only)

1. Merge the bounded Core prerequisite (CorePR2) first.
2. In the live checkout, compare all 26 closure rows against `SOURCE-BINDINGS.json` before applying the streaming patch:
   - the 24 prior rows unchanged;
   - `lib.rs` = `df45168f`;
   - `mod.rs` = BASE `340782cc`.

   After applying the patch, `mod.rs` must equal `bb0752ab`. A Git merge alone does not establish this.
3. Commit `reference_stream.rs` (`90316dea`, 2,879 bytes) **and** `reference_stream_tests.rs` (`4e8b66be`, 16,424 bytes) together with the patch. The patch declares the test module but does not create the file. Leaving it out would break any later `cfg(test)` build.
4. Do not claim that `f7ca8ed3` contained the bounded API, or that W was already wired.
5. Any first whole-crate `cfg(test)` build or `cargo test` after the merge is new, unaudited evidence. This verdict does not cover it.
6. Change nothing in:
   - the bounded prior candidate;
   - counters, native authority (stays 0), C4, caps, schema, profiles or defaults.

## 4. Publication audit

**Inventory.**
- `PUBLIC-FILES.json` lists 40 entries. With `PUBLIC-FILES.json` itself, that is 41, matching the package manifest.
- Every digest I compared agrees across `PUBLIC-FILES.json`, the package manifest and the source/result manifests.
- The byte sum is exactly 247,940.

**Exclusions confirmed.** The package contains none of the following:
- the rlib, test binary, cached dependency binaries or compiled objects;
- full raw inventories;
- provider raw output;
- Fenrir, key or witness material, credentials or tokens;
- the user's email.

The child environments contain only PATH, LANG and TMPDIR, so no inherited environment leaked. The binaries appear only as digests.

**Accepted disclosures.**
- Absolute paths under `/home/charl/...` are kept as provenance, as instructed.
- Host software identities are disclosed: Python 3.14.4, gcc-15, ld.bfd, uutils `env` and the rustc commit.

Both are low-sensitivity fingerprinting.

**Executable text in the records cannot run from the published location.**
- `run_core.py` refuses to run unless `__file__` equals `R/run_core.py`.
- `bounded_source_driver.py` has hardcoded absolute paths and asserts a fresh attempt directory.

Neither file has a `test_*` name.

**Claims in README, RESULT and SOURCE-BINDINGS are accurate and correctly scoped.**
- The two observations are kept separate.
- The compile took 1.62 s, stderr was empty, dep-info lists 26 files, and all five postflights passed.
- The README states no production `cfg(test)` build and no consumer.
- The H1 limits and the child and parent caps are stated.
- Native authority is 0 and no U, M, MC, SP or G exit is closed.
- The README states the base honestly: it says the bounded API was *not* committed at `f7ca8ed3`.

**Non-blocking advisories.** These do not require a refreeze.
- **A1.** `SOURCE-BINDINGS.integration_condition` says "25 unchanged source rows". The precise wording is "24 unchanged prior rows plus the new `reference_stream.rs`". The listed hashes are exact, so this does not change what the integrator compares.
- **A2.** The link `../sherlock-bounded-source-delivery/README.md` resolves only after the prerequisite package lands. Check it after the merge.
- **A3.** The header of `reference_stream_tests.rs` still says "UNRUN". This is frozen by the hash binding. Fix it only in a later change that has its own evidence.
- **A4.** In `COMPILE-ATTRIBUTION`, the field named `compiler_child` holds the `rustc -vV` step. This is a naming nit.

**Freeze conditions.**
- Publish exactly these 41 files with these digests at `docs/audits/2026-10-16/sherlock-streaming-delivery`. The package digest must remain `667e00c5…6cac7`.
- One change is pre-approved: adding this report, and Muse's companion report, verbatim under `records/reviews/`, together with only the matching new `PUBLIC-FILES.json` rows.
- Any other change requires a fresh review.

**This report needs no redaction.** It contains no secrets, private material or user email.

## 5. Not covered and gaps retained

- On-disk bytes, file modes, outer exit and traceback status are operator attestations.
- No consumer of `visit_steps` or of the rlib is approved. S1 binds any future caller.
- The following remain open:
  - whole-crate `cfg(test)`;
  - untrusted or hostile inputs;
  - the rule-82 `Rejected`, `Rejected(4)` and mid-run `Exhausted` paths through the visitor;
  - target and native acceptance.
- No K, Quint, Lean, Clean, handwritten-circuit, native or target evidence exists.
- No U, M, MC, SP or G exit closes.
- This verdict does not supply the separate Muse nonsecurity verdict on package accuracy.
