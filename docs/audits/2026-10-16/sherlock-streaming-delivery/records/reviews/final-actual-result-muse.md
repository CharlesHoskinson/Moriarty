# Final independent nonsecurity audit — Muse Wonder 1.0, high

## Verdicts

**SOURCE / ACTUAL RESULT: APPROVE_WITH_SCOPE — no nonsecurity blockers.**

**PUBLICATION PACKAGE: APPROVE_WITH_SCOPE — no nonsecurity blockers.**

These are consistency, source-contract, result-attribution, and package-accuracy verdicts only. They are not a substitute security or publication-safety judgment. Opus retains ownership of those judgments.

I ran no tools, compiler, tests, hashing, retries, or other execution in this review, as instructed. All checks below are cross-checks of the exact supplied texts, manifests, receipts, results, and reports.

## Exact identities checked

- Production candidate / `SOURCE_MANIFEST_SHA256`: `7f8cc109227ad49cbd6660d51933f453e8a4bdd2910ffd84242ffc6c2273783a`.
- Public package / `PUBLIC_PACKAGE_SHA256`: `667e00c5232288de9b5b38e4321b6b9d428da0fb19e7ec413d4a5e101806cac7`.
- Production result: `56154a66fb9a6575cee9201338d1a4bbbb49a99a56481a35079d83afc8973e93`, 28,685 bytes.
- Production rlib, retained locally and not published: `dff1c647ef75602afd9e7635bc57c529dc06697f85dcfdfab4a061b5c622f4e4`, 15,688,664 bytes.
- Prior audited Core result bound by the production result: `dd95f3e4cf38127e28362acf0242429a94385f9fe07af0c17e21a4e09780efbb`, 58,037 bytes.
- Recorded Git base: `f7ca8ed381daf986be651ba65745310334994bbf`. This review does not claim that base already contained the bounded Core commit or the streaming module wiring.
- Production `mod.rs`: `bb0752abf4065903fcd74cccd7c62783b88a17fc706b72c63e6b9e77e0f8aa39`, 1,895 bytes.
- Base `mod.rs`: `340782cc3bc0d7d95b71377df3f338f47e7a4caf00608f628c45cd0106f88887`, 1,779 bytes.
- `reference_stream.rs`: `90316dea89d6528e17ffdb991d8cf4b226e12714f3a1ed4db06de8c72821f923`, 2,879 bytes.
- `reference_stream_tests.rs`: `4e8b66bec3ef365b2fc73c7c65f8d677604cbaac7f9ceb9ae1677f0e9828cdfc`, 16,424 bytes.
- `future-wiring.patch`: `0fcab12809c47dd1d8435996065abee1d90e0a05f520ffecb9b481c4bc1a3eaa`, 512 bytes.
- Streaming receipt: `1721fa709d6a8baa21320bdef924481a7b1de49ce7e52b3e0ca71199d6bfb6e8`, 25,249 bytes.

## 1. Source and public API contract

The projected public wiring is internally exact.

- `BASE-mod.rs` plus the wiring in `future-wiring.patch` reproduces the candidate `mod.rs`: private `mod reference_stream;` is added, `pub use reference_stream::{VisitResult, VisitStop};` is added beside `pub use reference::Machine;`, and `#[cfg(test)] mod reference_stream_tests;` is appended as the tail.
- `lib.rs` retains `pub mod sherlock;` and is byte-identified as `df45168fa300562f532577739731c257d016a387fc943b25975ae46e3968a2fa`.
- The externally reachable API is therefore `mori_core::sherlock::Machine::visit_steps`, with `mori_core::sherlock::{VisitResult, VisitStop}` naming its result and stop types. The private `reference_stream` module does not hide the inherent `Machine` method or the re-exported types.
- `reference_stream.rs` implements the stated contract: each `step()` record is borrowed by the callback and dropped before advancing; no history is retained; terminal publication takes precedence over an observer `Break`; zero budget returns `Budget` without stepping or calling the observer, including on a terminal machine; a positive budget on a terminal machine returns `Terminal` with zero events; later calls resume the same machine.
- The production compile closure contains `reference_stream.rs` but no `reference_stream_tests.rs` or other `_tests.rs` closure member. The tests therefore remain separately audited standalone evidence, not production `cfg(test)` evidence.
- No change is shown to counters, native authority, C4, schemas, profiles, defaults, bounded prior candidate bytes, or other production files beyond the stated streaming wiring and new stream module.

## 2. Actual production result attribution

All 8 owner-attribution conditions, plus the 3 additional conditions, are satisfied by the supplied actual result. The 14 operator predicates in `COMPILE-ATTRIBUTION.json` are also all `true`, with overlap among the sets.

Owner conditions 1–8:

1. Outer exit 0 with no traceback: satisfied. `COMPILE-ATTRIBUTION.json` records `outer_exit: 0` and `traceback: false`.
2. Read-only success result: satisfied as attested; `result.json` records `terminal_state: "success"`.
3. Expected and after-manifest identity: satisfied. Both are `7f8cc109227ad49cbd6660d51933f453e8a4bdd2910ffd84242ffc6c2273783a`.
4. All five postflights true with no recorded postflight error: satisfied — dependencies, tools, snapshot, manifest, and exactly-two-children are all `true`.
5. Exactly two successful children with matching argv: satisfied. They are `00-rustc-version` and `01-compile-actual-core`, each exit 0 and not timed out. Observed times are 0.016 s and 1.620 s; total elapsed is 16.324 s.
6. Exact 26-file dep-info closure: satisfied. The dep-info inputs are the 26 files in `CONFIG.json`, including the real `lib.rs`, projected `mod.rs`, and `reference_stream.rs`.
7. Candidate, snapshot, before, and after source maps match the pins: satisfied. The four source maps are recorded equal to the manifest identities.
8. Prior Core result binding: satisfied. `prior_core_result.sha256` is `dd95f3e4cf38127e28362acf0242429a94385f9fe07af0c17e21a4e09780efbb` at 58,037 bytes.

Additional conditions 9–11:

9. Exactly the 7 expected helpers are recorded: `sha`, `record`, `inventory`, `write_json`, `child`, `source_records`, and `depinfo`.
10. `compile_stderr_bytes` is 0, and the published compile stderr/stdout captures are empty.
11. `actual_core_library` is recorded and matches the retained rlib identity above.

The prior 25 dep-info inputs plus `reference_stream.rs` account for the exact 26-file production closure. The closure difference from the prior production crate is the projected `mod.rs` bytes and the added `reference_stream.rs`; the other prior closure bytes are unchanged.

## 3. Actual 8-test result binding

The separately audited 8-test result remains bound to the same source bytes now in the production candidate. It was not rerun in the production compile, and this review does not present it as rerun.

- `run-tests.stdout` lists exactly the 8 named `reference_stream_tests`, all `ok`, with `8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`.
- Compile and run stderr are empty.
- The receipt records the six steps as `ok`, terminal state `success`, and binds result to receipt `1721fa70…e6b8`.
- The tested `reference_stream.rs` and `reference_stream_tests.rs` hashes match the production candidate manifest rows and the `cfg(test)` registration is present in the projected `mod.rs`.
- Its scope remains standalone leaf-module Std-host evidence on fixed normal inputs. Whole-crate `cfg(test)` was not compiled, and no generic consumer or native target acceptance is established.

## 4. Public package consistency

The frozen package at destination `docs/audits/2026-10-16/sherlock-streaming-delivery` is approved only as the exact 41-file package identified by `667e00c5232288de9b5b38e4321b6b9d428da0fb19e7ec413d4a5e101806cac7`.

- `PUBLIC-FILES.json`, `README.md`, `RESULT.json`, `SOURCE-BINDINGS.json`, the wiring patch, direct production-compile records, the six prior plain review reports, and the streaming-test records agree on candidate, result, receipt, closure, library-exclusion, and scope identities.
- The qualified README and `RESULT.json` correctly separate the two observations: standalone 8-test evidence and a fresh production rlib compile. They do not claim a production test rerun, an rlib consumer run, whole-crate `cfg(test)` compilation, or target acceptance.
- The rlib, test binary, compiled objects, cached binaries, provider raw output, private Fenrir material, keys, and witness material are not in the package manifest.
- No full source archive is duplicated. New source files remain repo-candidate material, while full production-closure hashes are published in `SOURCE-BINDINGS.json` and `ACTUAL-LIBRARY-CLOSURE.json`.
- Raw absolute runtime paths are retained only as provenance, consistent with the stated scope.
- Complete before/after dependency and toolchain inventory files remain local. The published `INVENTORY-SUMMARY.json` records identical pair digests (`35d50cc5…c790` for dependencies and `c4a40b7b…f911` for toolchain), 2,326 dependency members, the selected 8 dependency records, and all 75 toolchain records. This review therefore relies on those attestations and selected identities; it does not independently see all 2,326 raw dependency rows.
- Integration wording is consistent: Root must merge the bounded Core prerequisite first, compare all 26 captured closure bytes allowing only the old `mod.rs` to become the exact projected patch result, and only then apply the streaming module patch. It does not claim the capture base already contained the bounded API commit or that the stream module was already wired.
- Trust limits are explicit and not overstated: local tools and cached Rust artifacts, unestablished transitive source authenticity, no memory/disk/process-count cap, uncapped parent inventory time, and H1 unmetered helpers, snapshots, callbacks, and native stack behavior.

## Limits retained by both approvals

This approval covers finite host observations and the one actual production compile only. The new API remains an H1 typed host diagnostic. No CLI, LSP, MCP, source adapter, untrusted connection, generic consumer, or native/target use is approved.

A local captured pass closes no K, Quint, Lean, Clean, handwritten-circuit, native, target, U, M, MC, SP, or G exit. Native authority remains 0.

## Redaction and publication note

No redaction is required for this report if it is published verbatim: it repeats only digests, byte counts, timings, verdicts, and absolute runtime paths already present in the frozen public package, and it contains no credentials, provider raw output, private Fenrir material, keys, or witness material.

If Root adds this report as a 42nd file inside the destination, Root must update `PUBLIC-FILES.json`/the package manifest and issue a newly frozen package digest. That would be a package-manifest revision, not a change to the currently approved exact 41-file package.
