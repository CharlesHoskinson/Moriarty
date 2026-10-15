# Classification: ordinary host check (non-native), conditional

This is based only on the four supplied files. I ran nothing and verified no hashes.

**Verdict:** Both commands fit ordinary host Rust checks. Nothing in the closure touches any of the excluded actions: SDK, relation/circuit, model, degree/k, VM, transcript, keygen/prove/native-verify, ledger, deploy, key, SRS or cache. Native authority stays zero, and this grants no new authority.

## Basis

**Compile step**
- Plain `rustc --test` on a crate root that uses only std.
- No `extern crate`, proc macros, `build.rs`, `include!`/`include_bytes!`, `env!`/`option_env!`, `#[path]`, or `unsafe`.
- The derives are all built-in.
- `mod reference;` and `mod reference_types;` resolve next to the root file in `crates/mori-core/src/sherlock/`. `reference_tests` is pulled in only under `cfg(test)`. So the four files are the complete closure.
- No compile-time code execution beyond rustc and the libtest harness from the toolchain.

**Run step**
- The test builds an in-memory `Program`, calls `Machine::new`, then `step()`.
- `step()` hits `unimplemented!("Sherlock reference transitions")` and panics before any assertion runs.
- There is no filesystem, network, process or environment access.
- `PROFILE` and `K_SOURCE_SHA256` are inert string constants.

**Expected result:** a RED failure for the intended reason. Expect a panic `not implemented: Sherlock reference transitions` and a nonzero test-binary exit (101), not a compile error or setup fault. A different failure (compile error, assertion mismatch, timeout) is not this RED result and needs re-review before it is interpreted.

## Exact scope and conditions

1. The file bytes match the stated SHA256 values exactly, and no other files exist in `crates/mori-core/src/sherlock/` that the closure would pull in. Only the declared `mod`s are loaded.
2. The binary is the stated absolute-path 1.99.0 toolchain `rustc`, unmodified and invoked directly (no rustup proxy, wrapper or `RUSTC_WRAPPER`). No extra flags beyond those listed.
3. The working directory is the repo root that contains `crates/mori-core/...`, and `RUNTIME` expands to `/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-reference-20261015`.
4. **Stale-binary guard:** run the second command only if the first exited 0 in this session. Alternatively, confirm `RUNTIME/red-reference-tests` did not exist before compiling. Otherwise a pre-existing, unreviewed binary could be executed.
5. Run with `--exact reference_tests::literal_publishes_at_exact_zero_remaining_fuel --nocapture` only, with no other filters, `--include-ignored`, or harness flags. Use `timeout 60s` for each command separately.
6. Outputs are written only to `RUNTIME/`. rustc may create temp files there. Nothing is written into the source tree or any cache.
7. Compiler warnings such as `dead_code` are benign. Warnings promoted to errors would change the result and need re-review.

## Not covered

- Any byte change to these four files, including the future implementation of `step()`. Each change needs fresh source inspection before running.
- Any added module, dependency, Cargo invocation, build script or external crate.
- Future traces or `StepRecord` outputs. Whatever actually consumes them needs its own classification. This verdict does not extend to any consumer, comparison against K, or any native path.
- Acceptance of the implementation or of the test's expected values (102/80 rule lines, charge semantics). This is a safety classification only, not a correctness review.

No campaign, counters or authority are created or changed.
