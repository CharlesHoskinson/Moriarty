## APPROVE_WITH_SCOPE

No nonsecurity blockers were found in the supplied source delta or in the actual 8-test result. The proposed production compile is **not run and is not authorized by this review**: execution requires the exact Opus-authored final driver and a later fresh confirmation of its exact bytes, followed by an independent audit of the actual compile result.

### Source and public-export audit

The delta is exactly the stated wiring in `candidate/crates/mori-core/src/sherlock/mod.rs` relative to `BASE-mod.rs`, matching `future-wiring.patch`:

- `mod reference_stream;` is added as a private sibling of `reference` and `reference_types`.
- `pub use reference_stream::{VisitResult, VisitStop};` is added beside the existing `pub use reference::Machine;` and `pub use reference_types::*;`.
- `#[cfg(test)] mod reference_stream_tests;` is registered only under test configuration.

This is sufficient for a real external API: `mori_core::sherlock::Machine::visit_steps` is a public inherent method on the already-public `Machine`, and its return and stop types are reachable as `mori_core::sherlock::{VisitResult, VisitStop}`. The private module declaration does not hide the inherent method or the re-exported types. No `lib.rs`, schema, admission, default, profile, native-authority, C4, resource, or counter change is introduced.

### Streaming contract consistency

`reference_stream.rs` (2,879 bytes, SHA-256 `90316dea…f923`, unchanged from the tested capture) is consistent with the unchanged `Machine::step` semantics in `reference.rs`:

- Zero budget returns `{0, Budget}` without stepping or invoking the observer, both before and after termination.
- A positive budget on an already-terminal machine returns `{0, Terminal}` with no callback, because `step()` returns `None`.
- Terminal publication is checked before the observer’s `Break`, so terminal takes precedence over both observer stop and exact budget exhaustion; the delivering event is counted and the record was presented to the observer first.
- Observer stop takes effect only after a delivered nonterminal record, leaves the machine at that record’s after-state, and later calls resume the same machine, including a pending zero-fuel free publication.
- Zero-fuel exhaustion is one free terminal record (rule 84, charge 0), distinct from the zero-budget inert case.
- The caller budget is grouping only; callbacks, helpers, and snapshot clones remain unmetered under the stated H1 warning, and terminal publication, fuel, and `A`-step behavior are not altered.

### Actual 8-test result audit

The prior capture is internally consistent for its stated standalone scope: `run-tests.stdout` lists exactly the eight named `reference_stream_tests`, all `ok`, with `8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`; both stderr logs are empty; the receipt records all six steps `ok`, exit 0, terminal `success`; and `ATTRIBUTION.json` reports all eight attribution checks true, including manifest before/after equality and result-to-receipt binding against the approved manifest `73fb5864…a4b1`. The tested bytes for `reference_stream.rs` and `reference_stream_tests.rs` (`4e8b66be…dfc`, 16,424 bytes) are the same bytes present in the current candidate manifest. N01/N02 complete typed records, N05 selected full caller-boundary snapshots, all N02 observer cuts and resumption, and zero/terminal precedence are therefore supported as finite normal-only evidence.

That evidence remains scoped to the standalone harness that compiled the unwired modules by path. It does not establish production module compilation, constructor startup beyond `Machine::new`’s post-startup state, or any K, Quint, Lean, Clean, circuit, native, ledger, U, M, MC, SP, or G closure. The stale `UNRUN` wording retained in the test-file preparation docs is a documentation residue only; editing it now would break the SHA-bound capture and is not a blocker to this scoped verdict.

### Proposed production-compile disposition

The described compile class is nonsecurity-consistent as a plan: compile the snapshot’s actual `crates/mori-core/src/lib.rs` with the projected `mod.rs` as `mori_core` rlib on edition 2024, use the same dependency directory and the exact six `--extern` entries and two pinned proc-macro artifacts from `dependency-manifest.json`, run no `cfg(test)`, test umbrella, test execution, SDK, crypto, model-engine, proof, native, or ledger action, and require the actual dep-info to match the exact 26-file closure in `CONFIG.json` (the prior 25 inputs plus `reference_stream.rs`). Inventory pre/post, source/snapshot postflight, manifest-digest binding, and no-success-on-outer-traceback are correctly stated as requirements for the forthcoming driver rather than as observed facts.

**Execution disposition: DEFERRED, not approved.** `bounded_source_driver.py` as supplied is the prior job’s top-level driver and must not be imported or run for this job; the new driver does not yet exist in the reviewed manifest. Opus must author any guard changes and the final driver; this review makes no substitute security judgment on guards, trust in the local cached artifact directory or sysroot, or execution safety. A fresh specific approval of the exact final driver bytes is required before the one-shot compile, and the resulting dep-info, inventories, and postflight records require their own actual independent audit afterward.
