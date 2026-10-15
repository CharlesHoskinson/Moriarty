I wrote the patch and the handoff, but nothing was built, tested or applied. The sandbox needed approval for `git apply --check` and it wasn't granted, so I checked the hunk headers by hand.

**Files written (no source changed):**
- `R/OPUS-STATEMENT-ROOT-FIX.patch`, sha256 `3a3a942c131a3d1fbe51dd2b456228ab748133d8b42ff85046579bbe5eac8dc0`
- `R/OPUS-STATEMENT-ROOT-FIX.md`, the handoff

**Fix:** The patch applies to `crates/mori-ledger/src/relation/mod.rs` as it stands now, sha256 `19f7c4cb…`, which matches the current CLI pin.
- `statement_root` now returns `Result<ExprId>`.
- `Await` returns `layout::core1_refusal()`, the same typed refusal used by the compiler's `Await` arms (`compiler.rs:512,521`) and the entry guard. No new refusal class, no panic or `todo!`, no made-up expression.
- The only caller, `validate_statement_emissions`, now uses `statement_root(s)?`. The refusal goes out through its existing `Result`, before any artifact is built or the SDK is called.
- For every non-`Await` statement the comparison gives the same result in the same order, so /1 provenance bytes can't change.
- Programs containing `Await` are already refused by `require_core1` at line 360, before this helper runs. The change just makes the helper handle every case.

**Rerun classification: APPROVE_ORDINARY** for the exact `CLI-PROPOSED-CHECKS.json` command. This covers the call path only, not candidate acceptance. Conditions:
- **D1:** apply only this patch, byte for byte. The only pin you refresh is `relation/mod.rs`, to the new hash. I couldn't compute that hash, so you need to record it after applying. The other 112 pins and the `fe01bc8b` baseline must match before and after the run.
- **D2:** C1–C6 and all earlier residual caveats stay as they were.
- **D3:** if the build fails again, stop and diagnose; don't retry.

It's ordinary because the change adds no I/O, environment reads, processes, SDK, proving, key or ledger calls. The 7 CLI tests go through `compile_staged` in the backend and never reach this helper.

**What should happen:** the E0004 error goes away and `mori-ledger` compiles. Crates further down (the `mori` binary and the test crate) haven't been compiled yet, so they could still fail on their own errors. That would need a separate diagnosis.

**Not done:**
- No cargo, `git apply` or hash of the patched file.
- No search of the wider source for other incomplete `Statement` matches; I only grepped for this helper.
- No test added: the path can't be reached past `require_core1`, and a test would need a new classification.
- No core changes, so the 32 core passes keep their scope.
- Fresh full Muse and Opus code and results reviews are still owed.
- I probably went over the 220-second time limit.
