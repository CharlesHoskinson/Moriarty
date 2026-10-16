# Verdict: APPROVE_WITH_SCOPE

**Scope.** This approval covers only the caller-explicit, fixed two-source diagnostic job: S01 `quote(4,9)` and S02 `inspect(7,10)`, fuel 0..35 and 0..33, under the producer's nine stated bounds. It is not a general or untrusted-input API claim.

**Conditions.** The approval takes effect only when mandatory conditions M1–M5 below are met. They change the run procedure only, not the source, bounds or authority. If any of them cannot be met exactly as written, treat this verdict as CHANGES_REQUIRED.

**What this vote is.** It is one substantive security vote. The Core build still needs the agreeing independent Muse nonsecurity vote before it starts.

## H1 judgment for this job: satisfied

H1 warns against exposing the unmetered evaluator to untrusted input. This job has no untrusted input, and the bounded consumer adds count admission on top of that:

- **Fixed inputs.** The inputs are compile-time constants: two hash-pinned repo fixtures, literal scalar arguments, no CLI arguments, no stdin and no network endpoint.
- **Ordering.** Every refusal check runs before `prepare_scalar_action` and before any `Machine` exists:
  - source shape and arguments
  - Core validation
  - a non-recursive forward precount
  - occurrences
  - nesting
  - records
  - slots
- **Occurrence precount.** The precount is a sound upper bound on adapter occurrences. It counts every let and emit tree unshared, which over-counts the adapter's early refusals.
- **Recursion depth.** The adapter's recursive `map_expression` depth is at most `ast_nesting_bound`. For the fixtures this is 4, and the hostile 40-let chain is admitted at 42.
- **Termination.** `initial_fuel + 1` records is an exact termination bound. Each pending non-publication step costs one fuel, and publication is free.
- **No unbounded runner.** `Machine::run` is never called.
- **Extra step probe.** The added `terminal_has_next` call runs after publication. `step()` returns `None` before any clone or mutation, so it is read-only.
- **Values stay scalar.** List and tree arguments are refused by discriminant, and the generated body is walked to confirm it uses only scalar forms. Derived Drop/Clone/PartialEq/Debug recursion is therefore limited to AST nesting, with no Tree recursion.
- **Process envelope.** Each step runs in a separate child with a 30 s timeout and a process-group kill.

**What stays in force.** The H1 warning stays unchanged on `Machine` and on the bounded API. The counts do not guarantee bytes, wall time or native stack. Using this API with anything else would need its own review: other inputs, other bounds, untrusted or network callers, or any CLI/LSP/MCP wiring. That review would need measured stack use and byte accounting.

## Source review: no blockers

I traced the hostile expectations by hand against `reference.rs`. This is analysis, not execution:

| Test | Expected trace and result |
|---|---|
| Frame cap 3 | `S01[..23]`, then 140 and 82; `Rejected(4)`, 10 fuel left |
| Overflow | `S01[..18]`, then 225 and 82; `Rejected(10)` |
| Short arity | 7 charged edges; `Rejected(8)` |
| Mistyped argument | 9 charged edges; `Rejected(8)` |
| Zero-input action | 129, 200, 204, 208, 135, 102, 185, then 80 |
| Fan-out DAG | 32,767 occurrences; with the caller bound raised, the adapter's own budget diagnostic |
| 40-let chain | 42/41 refused, 42 admitted, 161 occurrences |
| Source shape | 9000 expressions refused before Core validation, then the Core `bound` error |
| Expected extent | Matches; peak slots 74·11·9 = 7,326 |

Compile paths also check out:

- Nested `#[path]` resolves relative to `sherlock/` in both the lib build and the harness build.
- `pub use mori_core::*` supplies every `crate::` path both files use.
- `use super::*` reaches the parent's private imports.

Non-blocking notes:

1. **Harness copy.** The 14 tests exercise a second crate instance of the same bytes. The producer's evidence comes through the production export, checked by `check_source_observations.py`.
2. **Self-pinned figures.** The slot formula and the extent totals are pinned only by the author's own tests. They are not independent evidence.
3. **No merge approval.** The public `sherlock_bounded_source` export is approved for this build only. Merging it as a supported API or wiring it anywhere needs separate review.

## Trust boundary: accepted, with M2 and M3

I accept the following as **local trust, not supply-chain authentication**:

- the locally cached third-party rlibs and the `serde_derive`/`thiserror_impl` proc-macro `.so` files, whose source authenticity is unverified (the proc macros execute native code inside rustc)
- the installed rustc 1.99.0 sysroot
- the system linker

This is acceptable because the job holds zero authority, touches no keys, proving or network, takes fixed inputs and emits diagnostics only. The same artifacts already executed in the historical build class. This acceptance does not transfer to native, target or release evidence.

**Recommended, not required:** run the steps without network access, for example with `unshare -rn`, because linked third-party code executes under the user's account.

## Mandatory conditions

**M1. Snapshot completeness**
- Add `models/sherlock/k/main.k` (SHA-256 `b3746ff51dd7999b046ccb23237595605420adcfeebc0c04c0ad0f1396b11d41`) to the snapshot and the manifest. The step-6 checker reads and hash-checks this file, and it is missing from the manifest.
- Verify every manifest hash at step 0, before any compile, and abort on any mismatch.
- I reviewed the pasted text. This approval binds to the manifest hashes as stated, including `bounded_scalar_source.rs` = `cd7ad770…`.

**M2. Full dependency inventory**
- Before step 1 and after step 6, record the path, size and SHA-256 of every regular file in DECLARED_DEPS, not just the eight named artifacts.
- Require the pre and post inventories to be identical, and require the eight named entries to match the manifest.
- Reason: rustc resolves the transitive rlibs and the proc-macro `.so` files from that directory by crate hash. The eight hashes alone don't show which files were actually loaded. The directory is also a shared cargo target, which may contain an old `libmori_core-*` and may receive concurrent writes.
- Stronger acceptable alternative: copy the directory, minus any `libmori_core-*`, into a fresh read-only `ATTEMPT/deps`, inventory it, and use only that.

**M3. Toolchain identity**
- Record the resolved absolute rustc path and the output of `rustc -vV` (it must report release 1.99.0).
- Record SHA-256 hashes of:
  - the rustc binary
  - `librustc_driver-*.so`
  - the target `lib/rustlib/<host>/lib` inventory
  - the linker resolved via PATH (`cc`)
  - any bundled `rust-lld`/`gcc-ld` used

**M4. Harness exactness**
- Record and hash the generated harness bytes.
- Use a quoted `#[path = "…"]` literal, and check that ABS_SNAPSHOT contains only `[A-Za-z0-9/._-]`.
- Before step 3, `--list` must show exactly the 14 `bounded_candidate::tests::*` names and nothing else.
- Run with `--exact --test-threads=1`. Require 14 passed, 0 failed, 0 ignored and 0 measured. Stop if this does not hold.

**M5. Envelope as stated**
- Use `env -i` with only `PATH=/usr/bin:/bin`, `LANG=C.UTF-8` and a fresh `TMPDIR`. That means no `HOME`, `RUST*`, `CARGO*`, `RUSTC_WRAPPER` or `LD_*`.
- Run from a fresh `ATTEMPT` directory outside the worktree, and write outputs only there.
- Use no `-O`, no `-C debug-assertions` or `overflow-checks` override, no `-Z` flags and no `--cfg test` on step 1.
- Stop each dependent step at the first failure. No retries, no fallbacks and no raised limits. Any deviation needs a new preflight.

## Not covered

This approval does not cover:
- a merge or result approval
- the public export as a supported API
- any CLI, LSP, MCP or network path
- other inputs or bounds
- the full mori-core test suite
- the stream modules
- Candidate B schemas
- native, C4 or target gates

All counters and terminal attempts stay unchanged. The 14 normal tests give no output-safety claim. Raw captures need my publication review before any push, and stderr will contain `/home/charl/...` paths.
