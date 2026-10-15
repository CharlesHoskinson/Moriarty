I approve the `--lib` command as ordinary (APPROVE_ORDINARY), provided root meets the preconditions in the report. The verdict is in `OPUS-FRAME-CHECK-CLASSIFICATION.md`, which is the only file I wrote. I edited no source, ran no Cargo, build or test, and native authority is still 0. I couldn't load the develop/AGENTS status because the shell refused the combined command.

**What the `--lib` build adds compared with the earlier `--test` approvals:**
- **Test code:** besides the six frame tests, it compiles 8 tests in `proving::tests` and 6 in `signed_relation::tests`. It also compiles `proving::task4_native_driver`, which is pulled in at `proving.rs:1777-1779`.
- **Native code that gets linked:** that driver contains the real native experiment, `task4_native_resource_gated_entry` (`task4_native_driver.rs:1127-1129`). This test is marked `#[ignore]` and reads `MORIARTY_TASK4_CASE`. The driver also adds test hooks into the keygen, prove, verify and parameter-loading paths (`proving.rs:476/483/505/614/623`, `parameters.rs:42/105`).
- **New compile-time data:** 21 example and fixture files are now embedded via `include_str!`/`include_bytes!`. They're data, not code, but they aren't pinned yet.

**Why none of that native code runs:**
- The filter `stage::frame::tests::` matches only the six frame tests. The other tests are named `proving::tests::*`, `signed_relation::tests::*` and `proving::task4_native_driver::task4_*`, so none of them match.
- The command has no `--ignored` or `--include-ignored`, so the native entry can't be selected.
- No dependency runs code at startup. `rusty-fork` is present through proptest, but only forking proptest tests use it, and none are selected.

**What the six tests actually run:** `mori_core::stage::load` on a string (pure parsing, which doesn't reach the file-reading code in `decode.rs`). They also call the frame functions and `Builder::public` (`lib.rs:624-657`), which only builds data structures in memory. There's no SDK, VM, proof, keygen or SRS access, and no env, fs or process use.

**Preconditions:**
1. **Pins:** keep the existing backend pins, with only `stages.rs` changed to `217c75af…`. Also re-check `frame.rs` (`d13bb9f7…`), `task4_native_driver.rs` and `rust-toolchain.toml` before and after the run.
2. **New pins:** add the 21 embedded data files listed in the report, re-hashed by root.
3. **Command:** run the exact argv from `FRAME-PROPOSED-CHECKS.json` once, with no `--ignored` and no retry.
4. **Environment:** use the same cleaned environment as backend C1–C7, and make sure `MORIARTY_TASK4_CASE` is unset. Confirm there is no Cargo config in parent directories or `CARGO_HOME`, and no symlink leading outside the worktree.
5. **Expected result:** `6 passed; 0 failed; 0 ignored; 17 filtered out`. Any other result, or any task4 or parameter output or capture directory appearing, means stop and diagnose.
6. **Record:** log the run under a new check ID and keep the earlier evidence.

**Still open:**
- **Build-time code:** third-party build scripts and proc macros remain unreviewed, as before.
- **Barrier against native runs:** the native code is linked into this test binary, and only the filter and the missing `--ignored` stop it running. Changing either needs a new classification.
- **Owed reviews:** the six tests are structural checks I wrote, with no independent oracle, so a pass doesn't show the frames are sound. Full Muse and Opus reviews of the candidate and the results are still owed.
