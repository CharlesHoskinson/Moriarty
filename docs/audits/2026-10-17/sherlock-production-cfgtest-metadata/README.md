# Production Sherlock cfg(test) metadata observation — 2026-10-17 UTC

One admitted local SDK-free operation metadata-typechecked the canonical Sherlock
module graph at [commit `bea43580`](https://github.com/CharlesHoskinson/Moriarty/tree/bea435805ee08b09d8a903dc5fbf630241b4a6d1).
The version child and the single `rustc --test --emit=dep-info,metadata` child
exited 0; the latter took 2.890 seconds and emitted 13 warnings. All four
operation steps completed successfully. **Zero tests executed.**

The exact read set contains 72 Rust files: 71 unchanged repository files and the
67-byte [wrapper](wrapper/lib.rs). It retains all 18 canonical parent `cfg(test)`
registrations, the real `layout_a/mod.rs` and its children, path aliases and oracle
include fragments. This includes the production owner regression module's 21
non-ignored cells and ignored N01 body. The [source membership](source-membership.json)
pins every file. This later observation qualifies the name/type analysis of the
[source-only regression attachment](../../2026-10-16/sherlock-production-owner-regressions/README.md);
it executes no assertions and changes no source bytes.

The [selected result](result.selected.json) records the consumed command shape,
child exits, independent exact dep-info check, 235 pre/post binding checks and 72
verified attempt source pins. The metadata output existed and was **0 bytes**;
no usable serialized-library artifact or downstream use is claimed. The
[full normalized compiler diagnostics](compile.stderr.normalized.txt) remove only
the absolute snapshot source-directory prefix. This public selection is an
unsigned local summary; exact receipts, admission controls, reports and provider
captures remain private. It supplies no third-party producer authentication.

Qualification is **metadata-typechecked, UNEXECUTED**, for this tiny-root,
edition-2024 Sherlock graph. It establishes no codegen, monomorphization-time,
post-mono const-eval or link result; no actual full-Core crate-root, Cargo,
workspace or dev-dependency build; and no integration test, doctest or test pass.
The sysroot, libtest, compiler libraries and host dependencies remain unhashed;
there is no sandbox or same-UID mutation protection.

N01/full96 remains ignored, unimplemented and UNRUN. H05 holder coverage remains
partial; H06 P6-cap and physical-overflow precedence remain unstepped. Hostile-test
execution eligibility, full B/Sherlock, correspondence and parent acceptance stay
open. No SDK, model/VM, circuit, proof, native or ledger action is qualified.
Native authority remains zero; C4, counters and limits are unchanged. The one-shot
attempt is consumed and was not rerun. Historical captures retain their original
scope and status.
