# APPROVE_WITH_SCOPE

This is a static exact-candidate confirmation for final manifest `7f8cc109…3783a`. I ran nothing and hashed nothing. Every check below compares the pasted texts with each other.

## 1. Driver bytes and manifest delta

**`run_core.py` matches my driver word for word.** I compared it with the fence in my report:
- The docstring and imports match.
- Every constant matches: `R`, `A`, `S`, `CAND`, `RUST_SHA`, `RUST_COMMIT`, `DEPS`, `DM_SHA`, `PRIOR_CORE`, `HELPER_SHA`, `HELPERS`, `CHILDREN`, `PINNED`, `TOP`, `PRIOR_EVIDENCE`, `WIRING`, `WIRING_TAIL`, `PATCH_ADDED` and `SAFE`.
- The `result` template matches.
- All of `digest`, `require`, `member`, `load_helpers`, `toolchain`, `run`, `postflight` and `main` match.

I found no independent edits. The 18,994-byte size and the digest `81c079df…1336` are root's on-disk claims. The run binds them through the self-hash check and the postflight re-hash.

**The manifest delta is exactly one new row.** The 53 rows of `SOURCE_MANIFEST` appear in `FINAL_MANIFEST` with the same paths, digests, sizes and order. The 54th row is `run_core.py`.

**The driver's own reviewed-set check will accept this manifest.** The expected set is 6 `TOP` files, 12 `PRIOR_EVIDENCE` files and the 36 `candidate/` members from `CONFIG.snapshot_members`, which totals 54. All ten `PINNED` digests equal their manifest rows.

## 2. Public API wiring

**The projection is exactly BASE plus the reviewed patch.**
- Each `WIRING` anchor occurs once in BASE.
- BASE ends with `\nmod layout_a;\n`.
- BASE plus the two replacements plus `WIRING_TAIL` reproduces the candidate `mod.rs` text.
- The sizes agree: 1779 + 22 + 52 + 42 = 1895.
- The `+` lines of `future-wiring.patch`, including the empty `+` line, equal `PATCH_ADDED`.

**Only the wiring and the new module change.** The prior bounded-source-01 snapshot shows the following:
- Its compiled `mod.rs` was `340782cc…` (BASE).
- I checked the other 24 prior Core dep-info inputs against the final manifest one by one. All 24 digests are unchanged: the 9 crate-root files, `bounded_scalar_source`, `compare`, `reference`, `reference_display`, `reference_types`, `source_adapter`, `source_sponsor` and the 8 `stage/` files.

So this compile differs from the already-compiled production crate in two places only:
- `mod.rs`, which goes from `340782cc` to `bb0752ab`.
- The added `reference_stream.rs`, `90316dea`.

`reference_stream.rs` is byte-identical to the file compiled and tested in streaming-01.

**The resulting API is as reviewed.** It is:
- `mori_core::sherlock::Machine::visit_steps`, an inherent `pub` method on the already-public `Machine`.
- The re-exports `mori_core::sherlock::{VisitResult, VisitStop}`.

`use super::{Machine, Outcome, StepRecord}` resolves through the existing `pub use reference::Machine` and `pub use reference_types::*`. Production code does not wire up any test module. `reference_stream_tests` stays `cfg(test)` and is not in the closure.

## 3. Binding of the 8-test result and the prior run

**8-test result.** The Rust source the 8 tests compiled (`reference_stream.rs` and `reference_stream_tests.rs`) appears in both `PINNED` and the final manifest. The streaming-01 evidence set is in the manifest and gets re-hashed in postflight: the receipt `1721fa70…`, the `CANDIDATE.json` digest `73fb5864…`, the logs, the reports and `run_streaming.py`. My earlier APPROVE_WITH_SCOPE on that result still holds within its stated limits, which are standalone `#[path]` host evidence, normal-only inputs, and no production `cfg(test)`.

**Prior closure.** The prior 25 dep-info inputs all carry the `bounded-source-01/source/` prefix. With that prefix removed, they equal `CONFIG.compile_closure` minus `reference_stream.rs`. Prior `terminal_state` is `success`.

**Helper binding.** Two points need stating:
- The prior `result.json` does not record its driver's digest. The `HELPER_SHA` link therefore rests on root's observation that the copied file and the live `execute_bounded_source_01.py` are both `6fe358db…fd51`, 11,528 bytes. I accept that observation for this scope.
- The safety of this run does not depend on that link. I reviewed the seven extracted helper bodies directly in the pasted bytes:
  - None has decorators or default arguments.
  - None references the helper module's `W` or `R`.
  - Their free names are exactly the injected namespace: `A`, `S`, `result`, `manifest` and the stdlib modules.
  - `source_records` reads only the root it is given, either the snapshot or `CAND`. Nothing touches `W`, which is consistent with the statement that no W or shared sources changed.

**Prior `result.json` digest.** The driver reads this file live and records its digest; the file is not pinned in the manifest. Before the new result is attributed, the audit must show `prior_core_result.sha256 = dd95f3e4…efbb` and `bytes = 58037`.

## 4. Driver execution scope

**Exactly two children, through the extracted `child`:**
1. `rustc -vV`.
2. One `mori_core` rlib compile. Its argument vector is identical to the prior `01-compile-actual-core` apart from the `attempt-core-01` paths: edition 2024, `-L dependency=DEPS` and the same six sorted `--extern`s.

The compile has no `--test`, `--cfg test`, `-C` options, Cargo, SDK, model or native steps. `shutil.which` does not spawn a process.

**Caps and environment are unchanged from the prior run:**
- 30 s per child.
- `env -i` with the minimal PATH, LANG and TMPDIR.
- `start_new_session`, with `killpg` on timeout.
- Full inventories of `DEPS` and the toolchain/sysroot, taken before and after.
- No retry and no fallback.
- `A` must be fresh, the manifest digest is checked before `mkdir`, and `result.json` is written last and exclusively.

**`-I` keeps the script directory off `sys.path`.** A file planted in `R` cannot be imported.

## Conditions on this approval

1. **Invoke with absolute paths.** Use `/usr/bin/python3 -I -B /home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/streaming-02-production-preparation/run_core.py 7f8cc109227ad49cbd6660d51933f453e8a4bdd2910ffd84242ffc6c2273783a`. Run it once, against `attempt-core-01` only.
2. **Post-run attribution.** The result counts only if all 8 attribution conditions from my report hold, plus the following:
   - `result.helpers` lists the 7 expected helpers.
   - `compile_stderr_bytes` is 0. The prior compile of the 24 unchanged files produced empty stderr. The driver does not enforce this, so any nonzero value fails attribution until the auditor explains it.
   - `actual_core_library` is recorded.
3. **Failures.** Any failure is reported as a failure. A second attempt needs a fresh approval.

## Not covered

- I verified pasted text only, not on-disk bytes.
- This approval does not cover the future compiler result or the rlib it produces. An independent audit of the actual production result is still required.
- No consumer of `visit_steps` is approved. Under S1, no CLI, LSP, MCP, `/1`, `/2` or untrusted path may call it.
- There is no whole-crate `cfg(test)` build, no target acceptance and no K, Quint, Lean, Clean or circuit evidence.
- Native authority stays at 0. C4, counters and resource limits are unchanged.
- H1 is unchanged. That covers the unmetered typed host evaluator, snapshots, helpers and callbacks.
- Publishing the raw captures is a separate scope.
- Root alone applies `future-wiring.patch` and merges.

**Residuals I accept, as stated in my report:**
- The window between the interpreter reading the driver and the driver hashing itself.
- Proc-macro selection being pinned only by the full `DEPS` inventory.
- The parent process's own time being uncapped during inventory. That inventory hashed about 14.5 GB previously.
