Both commands are **APPROVE_ORDINARY**, but only if root meets the preconditions in the report. I wrote the report to `R/OPUS-BACKEND-CHECK-CLASSIFICATION.md` (1,315 words). I ran neither command, and no source file changed.

| Command | Verdict |
|---|---|
| `--test stages` | APPROVE_ORDINARY. 6 tests: **5 run, 1 ignored** (the baseline comparator) |
| `--test stage_relation` | APPROVE_ORDINARY. 4 tests, 0 ignored |

**What the tests actually do:**
- **Staged compile:** Core `stage::load` (the same pinned pure path as the CORE classification) builds the IR. It is then serialized and reloaded through the public SDK parser `IrSource::load`, which is plain JSON parsing plus field-element encoding.
- **Not reachable:** nothing on these paths calls SDK check/model, the VM, keygen, prove, the verifier parameters, the prover-key cache or parameter fetching.
- **Legacy refusal:** `legacy_compilers_refuse_staged_programs` is refused by the first statement of `compile` (`lib.rs:366`), before any SDK call.
- **No file writes from the tests:** the output path argument is unused, and proptest/insta aren't used, so no regression or snapshot files get written.
- **No native test code compiled:** `task4_native_driver` and the library's test modules are `#[cfg(test)]`, so `--test` never compiles them.

**Things root needs to act on:**
1. **The argv doesn't clean the environment enough.** It only unsets `RUSTFLAGS` and `MORI_FEE_POOL_CAPTURE_DIR`. Root has to launch from a cleaned environment, clearing `MORI_*`, `MORIARTY_*`, `PROPTEST_*`, `CARGO_*` (except the target dir), the rustc/wrapper overrides and the C-compiler variables. The report lists every variable. `HOME` and `CODEX_HOME` stay unchanged.
2. **Six files need pins** (hashes are in the report): `counter.mori`, `payment.mori`, the two `.snap` files, `mori-lexer/Cargo.toml` and `rust-toolchain.toml`. Root should also run `git diff --quiet fe01bc8b` over lexer/diagnostics/span and the backend fixtures. My own run of that was blocked by the permission layer; an earlier diff showed the fixtures unchanged.
3. **The earlier CORE "SDK never linked" premise doesn't carry over.** mori-backend does link the SDK. One consequence: when the SDK compiles, rustc embeds the public verifier SRS file (`bls_midnight_2p14`) into the build. It is only decoded on verify paths, which these tests can't reach. I count this as ordinary. If you treat any compile-time embedding as an SRS action, both verdicts become DENY.
4. **Stop conditions:** the run must end with exactly `5 passed; 1 ignored` and `4 passed; 0 ignored`. Never pass `--ignored` or `--include-ignored`.

The other residuals (blst/aws-lc C build scripts, SDK networking code that is linked but never called, cargo-home lock writes) are listed in the report.

On model identity: this session produced no adapter report. My runtime says I'm `claude-opus-5-5`, but root should record whatever identity its own adapter returned.

None of this accepts the feature as correct or secure. The full candidate/result reviews by Muse and Opus are still owed, and native authority stays at zero.
