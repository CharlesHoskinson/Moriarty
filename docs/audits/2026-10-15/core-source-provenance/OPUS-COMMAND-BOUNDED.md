# Opus 5.5 security decision: execution-path classification for runs A–D (core-source-provenance)

**Scope.** This decision classifies the actual execution effects of four exact commands. It is not candidate approval, result approval, IR or semantics review, or oracle recomputation. I ran no tools and verified no hashes. All hashes are root-supplied.

- Native authority remains ZERO (gross 4334 / remaining 0).
- All U/M/MC/SP/G gates remain open.
- The H2–H4 expected outcomes, and every test expectation, are **unreviewed** and need the later full security audit.
- The missing canonical author receipt for H2–H4 is not supplied by this decision.

**Premise, explicitly retained.** OPUS-COMMAND-REVIEW is my premise for all **unchanged** source and the SDK runtime closure:

- the trace of `main`, `compile_call`, `static_compile::{compile,reopen}` and `publish_new`;
- `InMemoryDB` storage only;
- tagged decode only after the byte-equality check;
- no VM, check, keygen, prove, degree, ledger, network or private material;
- residuals R1–R7 recorded and **not waived**, including the third-party build scripts, proc macros, crypto internals, storage-core gaps, std/clap/anstream, and linked-but-unreached code.

I did not reinspect any of that. I reviewed only the supplied changed source and the C1–C3 material.

## Decisions

| Run | Decision |
|---|---|
| **A** exact normal CLI test, with baseline | **APPROVE ORDINARY**, subject to the conditions below |
| **B** `provenance_security_` (H2/H3/H4) | **APPROVE ORDINARY** (effects only), subject to the conditions below |
| **C** exact mori-ledger IR-structure test | **APPROVE ORDINARY**, subject to the conditions below |
| **D** `relation::emission_tests::` (5 unit tests) | **APPROVE ORDINARY**, subject to the conditions below |

I found no REJECT and no NEED-SOURCE. No reachable forbidden effect appears in the supplied deltas.

## Prior conditions C1–C3: reassessment

**C1 (final `relation/mod.rs`, hash `b7ecafb3…`, 22467 B): satisfied on the supplied text.**

- The only new imports are the type imports `Instruction`, `Operand` and `ExprId`.
- `statement_root` and `validate_statement_emissions` are pure. They use slices, `Vec`, `partition_point` and `binary_search`. They do no I/O, env access, process work, hashing, relation check, VM work or proving.
- Work is O(I + F log F) under the existing `compiled-work-bound`. No limit changed.
- The only call site is `compile_source_static_roles`, which runs at both export and reopen recompile.
- `compile`, `compile_bindings`, `check`, `admit`, `Manifest`, `Group` and the `Builder::op` refusals are unchanged in the diff. The refusal code is `source-static-emission`.
- The `compiler.rs` and `arithmetic.rs` deltas are bookkeeping only: `Builder.dynamic`, the `StatementEmission` push and tuple arity. Instruction and group emission is unchanged.
- `mod.rs` differs from Opus's authored record (`2c0dcd20…`, 22451 B). In the supplied text, the validator differs only by line-wrapping. The test module is first seen here. This decision binds to root's `b7ecafb3…`.
- `mapped_role_static_io.rs` is now `7e99fdff…` (41372 B), so the prior byte-identity requirement is replaced by my review of the supplied current text.

**C2 (baseline): satisfied, conditionally.**

- The removed (baseline-only) lines in the supplied current diff are signatures, tuple arity, two imports, the `/1` format literal and `c.statement(statement)?;`. None performs I/O, reads env, starts a process or executes SDK code.
- **Condition:** root's statement "95df→eb25 diff … exit0" must mean **empty output** (for example `--exit-code` or `--quiet`). A plain `git diff` exits 0 even when the trees differ.
- The baseline binary's provenance (built from 95df, PR52) rests on root's claim and on the pin.

**C3 (CLI startup): satisfied.**

- `cli.rs` has no `#[arg(env=…)]`. Its defaults are only `color="auto"`, which is overridden by `--color never`.
- `Color → ColorChoice` is a pure mapping.
- `display::text`, `CliError` (a thiserror derive) and `styles` (constants) perform no I/O or process work.
- I accept root's search result: the only other default is the literal `default_value_t=9` in `proof_io.rs`, and there are no env defaults or function defaults.
- `shell::Verbosity` was not supplied. It is covered by the premise.

## Common conditions (each run)

- Run exactly as stated:
  - `flock -w120` on the stated lock;
  - `timeout --signal=TERM --kill-after=10s 300s`;
  - the stated `CARGO_TARGET_DIR`;
  - `+1.99.0` already installed, so no auto-install;
  - `--offline --locked -j2`;
  - `env -u MORI_FEE_POOL_CAPTURE_DIR`;
  - no RUSTFLAGS or CARGO_ENCODED_RUSTFLAGS;
  - no other `MORI_*` variables;
  - no feature or profile change.
- Root re-pins all supplied source hashes before each run. Any drift voids this classification for that run.
- Stop on failure. Any nonzero exit, timeout, panic or unexpected run count is a stop and report. Do not edit expectations, retry with a larger budget, or fix forward. I recommend halting the remaining runs as well.
- Run-shape check: only the stated number of tests may **run**, whether they pass or fail.

## Per-run conditions and residuals

**A**

- `MORI_PROVENANCE_BASELINE_BINARY` is set for this run only, to the literal absolute path.
- The baseline SHA-256 `89ae66c4…` is checked before and after the run. A mismatch voids the evidence.
- The cargo rebuild of `target/debug/mori` does not touch the separate copy.
- Exactly 1 test runs; by my reading of the file, 9 are filtered out.
- Both child processes inherit the test environment, including `MORI_PROVENANCE_BASELINE_BINARY`. That variable is inert in the product per C3.
- Writes stay inside the fixture (`out/`, `baseline-out/`). The comparison covers `action-000000.ir.bin` bytes only.
- mori-ledger is built as a dependency, so `emission_tests` is neither compiled nor run.

**B**

- `MORI_PROVENANCE_BASELINE_BINARY` must be **unset**.
- The filter is a substring match. Exactly 3 tests must run; by my reading, 7 are filtered out.
- Actual effects:
  - 18 bounded candidate invocations: export, reopen and inspect;
  - in-test rewrites of the fixture's `source.mori` and `bindings.json`;
  - public `codec::hash` and `object_hash`;
  - H4's in-place `std::fs::write` of its own fixture manifest and marker.
- Those writes follow any hard link. Under the premise, `publish_new` links only inside the fixture's `out/`, so the effect stays within the fixture.
- A tampered reopen strictly parses attacker-shaped marker JSON under the existing caps. The tampered manifest is only compared with the fresh compile's bytes and never decoded, per the premise.
- No H5, `unsigned` or signed invocation occurs. Here `refused()` runs only `--inspect-static` in mapped mode.

**C**

- Exactly 1 test runs; by my reading, 8 are filtered out.
- Execution is in-process and in-memory only:
  - `bindings()`: public k256 point decode, `UserAddress::from`, hashes;
  - `static_compile::compile` (no publish or file write);
  - JSON projection;
  - `decode_artifact::<IrSource>` on product-emitted bytes (tagged decode, R3);
  - structural comparisons only.
- `Dir` is unused, so there is no filesystem write.
- `limits()` values are caps only; no degree, SRS or key work is reached, per the premise.
- The lib is built without `cfg(test)`.

**D**

- Exactly 5 tests run.
- The tests construct `Statement`, `Instruction`, `Group` and `StatementEmission` values and `Fr::from(u128)`, then call the pure validator. Nothing else runs.
- **New residual R8 (compile and link scope).**
  - `--lib` compiles every `#[cfg(test)]` module in mori-ledger; the other modules were not supplied. They are compiled but not run, because the filter matches only this module.
  - Compile-time effects are limited to rustc, the existing proc macros (R2) and any `include_*!` reads.
  - The lib-test binary links mori-ledger's dependencies plus `futures` and libtest. Any life-before-main constructor in a linked crate (for example `ctor`- or `inventory`-style) was not inspected. Recorded, not waived.

**R1 extension (C, D).** Under `-p mori-ledger`, resolver-2 feature selection may differ from the `-p mori` graph. That can produce new compile units and rerun third-party build scripts. This stays offline and locked; it is recorded, not waived.

## Possible early-failure points (fail closed, not confirmed defects)

These are the specific code errors you asked me to report separately. None are fixed or tuned here.

1. **D:** `(error.layer, error.code)` requires the `Error::layer` field to be visible from `relation::emission_tests`. The `Error` source was not supplied. If the field is not visible, the build fails before any test runs.
2. **B/H4:** the text edits assume `call_artifacts::json_bytes` writes pretty JSON with a `"key": value` layout. The `reseal` byte-equality assumes the product marker equals `serde_json::to_vec_pretty(Value)` with the same key order. Neither the source of `json_bytes` nor the marker writer was supplied. A mismatch panics before any tampered invocation.

## Not approved here

- Candidate source, results, the oracle, expectation correctness, H1–H5 semantics, compatibility consequences (format `/2`, `compiler_hash` and `marker.build` churn), native residuals, broader suites and ordinary or signed modes.
- Fresh Muse/Opus candidate and actual-result reviews are still required after ordinary verification.
