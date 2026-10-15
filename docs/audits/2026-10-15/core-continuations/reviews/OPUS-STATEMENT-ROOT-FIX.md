# Opus 5.5 statement_root fix and cli-normal-01 rerun classification

Author and security owner: Opus 5.5 (high). Native authority stays at 0. Nothing was compiled, tested or applied.

## Diagnosis

`checks/cli-normal-01` exited 101 after 35.8 s, while compiling `mori-ledger` and before any of the 7 tests ran. The error is E0004 at `crates/mori-ledger/src/relation/mod.rs:271`: `statement_root(s) -> ExprId` doesn't cover `Statement::Await`. PR #53 added this provenance helper. The earlier legacy-refusals patch covered the compiler arms (`compiler.rs:512,521` → `layout::core1_refusal()`) but missed this helper. The run reported source pins and baseline unchanged, no fixture leftovers and 0 native actions.

The only caller is `validate_statement_emissions` (line 320). Its only caller in turn is `compile_source_static_roles`, which already runs `layout::require_core1(validated.program())?` (line 360) before the compiler and before the validator. On the real path an `Await` program gets the core1 refusal first, so the helper never sees `Await`. The helper still has to be total, and it must not invent an `ExprId` or panic.

## Patch

`R/OPUS-STATEMENT-ROOT-FIX.patch`, sha256 `3a3a942c131a3d1fbe51dd2b456228ab748133d8b42ff85046579bbe5eac8dc0`.

- Pre-image: `crates/mori-ledger/src/relation/mod.rs` sha256 `19f7c4cb6e18fa95a4449154955aad3c50aef1c0b2b25eb5b9d43d3539c52b0a`. This equals the current CLI pin and already includes the prior `+require_core1` line.
- Changes:
  - `statement_root` now returns `Result<ExprId>`, with `S::Await { .. } => return Err(layout::core1_refusal())`. The other 9 arms are unchanged, wrapped in `Ok(...)`.
  - The caller changes to `e.expression != statement_root(s)?`.
- Semantics: `Await` gets the same typed core1 refusal the compiler arms and the entry guard use. There's no new refusal class, no panic or `todo!` and no made-up expression. The error leaves through the validator's existing `Result<()>`, before artifact or manifest construction and before any SDK call. `layout` and `Result` are already imported (lines 6–8), and `core1_refusal` is `pub(crate)`.
- /1 byte behavior: for every non-`Await` statement, the comparison gives the same bool as before. Short-circuit order is the same (`e.statement != index` is still checked first). Manifest, IR and emission bytes are untouched, so /1 provenance output can't change.
- I checked the hunk headers by hand (`-266,17 +266,18`, `-317,7 +318,7`). `git apply --check` needed approval and **did not run**.
- Post-image hash: not computed, because I'm not allowed to write source or scratch files. Root must record sha256 of `mori-ledger/src/relation/mod.rs` after applying the patch and use it as the only refreshed pin.

No test was added. The path is unreachable behind `require_core1`, and a direct unit test would mean a new test target and a new classification. That's out of scope here.

## Rerun classification: APPROVE_ORDINARY

This is for rerunning the exact `CLI-PROPOSED-CHECKS.json` argv (`cargo +1.99.0 --offline --locked test -p mori --test core_continuations -j2 -- --test-threads=1` under the same flock, timeout and env wrapper), on these conditions:

- **D1:** apply only this patch, byte for byte. Refresh only the `crates/mori-ledger/src/relation/mod.rs` pin, from `19f7c4cb…` to the observed post-image hash. All other 112 pins and the `fe01bc8b` baseline diff must match before and after the run.
- **D2:** all prior CLI conditions C1–C6 still apply unchanged, including the env scrub list, TMPDIR, the single serial run with no retry, P5/P7 reuse only and the fixture-leftover check. The earlier residuals also still apply: the missing Cargo.toml and `rust-toolchain.toml` pins, registry build scripts not enumerated, and lowering bodies read only through grep.
- **D3:** this is a rerun after a diagnosed source fix, not a repeat of the unchanged run. If the build fails again, stop and get a diagnosis. Don't retry.

Why this is ordinary: the change adds no I/O, env read, process spawn, SDK, proving, key or ledger call. It only makes a private pure function total. The 7 CLI tests reach `compile_staged` (backend), not `relation::compile_source_static_roles`, so the new error arm isn't on their runtime path at all. Its only effect is that `mori-ledger` compiles. Even if something did reach that path, `require_core1` refuses first.

## Expected outcome

- Expected: E0004 gone, and `mori-ledger` builds. The build then moves on to `mori-backend`-dependent and `mori` CLI crates that haven't compiled yet in this run.
- Unknown: whether later passes or downstream crates (the `mori` binary, the `core_continuations` test crate) hit their own compile errors. rustc stopped at this one. Any new failure is a separate diagnosis, not a reason to retry.
- If it builds, the 7 tests run under the previous classification. Their pass or fail isn't predicted here.

## Source classes touched

- Changed: `mori-ledger` legacy source-static relation provenance (crate-private helper plus one caller expression).
- Not changed: mori-core, so the 32 core passes keep their existing scope. Also unchanged: backend, CLI commands, H28 sources and pins, test sources and examples.

## Not executed / caveats

- No cargo, build, test, fmt, `git apply` or hash of the post-image.
- No pass of the full source closure for other non-exhaustive `Statement` matches outside the files I grepped. A repo-wide grep for `statement_root` found only these two sites.
- This approves the call path only. It is not candidate acceptance. Fresh Muse and Opus code and result reviews of the full candidate, including this patch, are still owed.
- Elapsed time was probably over the 220 s budget.
