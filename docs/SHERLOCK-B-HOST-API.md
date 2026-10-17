# Sherlock B host library API

This source landing exposes `mori_core::sherlock::helper_machine` beside the
existing A reference API. Its profile is `sherlock-helper-microstep/1`,
`DedicatedSorts`, finance `Absent`, HostOnly, genesis and one uncut segment.
It preserves the adopted source05 owner and family bodies, apart from the
recorded module paths. It does not replace `sherlock::Machine` or the Core
financial compiler/runtime.

The callable boundary is the fixed
`helper_machine::shared::owner_host::host_owner_engine()` constructor. Import
`helper_machine::shared::OwnerEngine` to call the returned engine's methods.
The constructor supplies the existing family implementation and owner
preparation functions. The `raw` and `shared` modules retain their source
visibility; the family implementation module remains private.

A consumer supplies a `shared::CoreProgram`, `BoundsC` and `LayoutSchemaC`.
Alternatively, `source_input_bridge::from_program(&sherlock::Program)` copies
an existing canonical A AST into `CoreProgram`. This bridge accepts the actual
A types exported by `mori_core::sherlock`; no second A type declaration is
compiled. It copies AST structure only. It is not a parser or financial source
lowering path and establishes no A/B execution correspondence.

This bridge copies finite shared ASTs occurrence by occurrence, unfolding shared structure rather than preserving sharing, with unmetered host allocation before any B admission cap applies; only trusted typed in-memory callers holding an already-constructed `sherlock::Program` are qualified to use it, and it makes no untrusted-input or Stage-2 claim.

Call `engine.admit(&program, &bounds, &layout)` and retain its returned
`Admitted`. Call `engine.admit_run(&admitted, &fuel, &stack_limit,
&FinanceInput::Absent)` to obtain a `MachineImage`. Fuel and stack limit use
`RunIntC`. Pass that same admitted handle and image to `engine.step` or
`engine.run`. `step` returns `StepResult`; `run` accepts a `FnMut(&StepEventC)`
sink and returns `RunSummary`. Their error type is `HostViolation`.
Admission methods return `Tooling`. Callers must handle these results; this
landing adds no default capacities, silent fallbacks or fixture selection.

`shared::owner_host::observe_bound_image(&admitted, &image)` provides the
existing borrowing observation API, returning `HostObservation` or
`HostViolation`. Observations borrow the admission and image. The existing
owner contract defines their fields; the API adds no wire encoding.

The source05 declarations retain historical proposal comments to preserve their
provenance. The module paths and callable surface documented here describe this
landing. `inputs/source_a_types.rs`, fixture builders, capture/JSON diagnostic
modules and the standalone `main.rs` are excluded from the production tree.
No dependency or crate-root declaration is needed beyond the public child in
`sherlock/mod.rs`; `mori-core` already publicly exports `sherlock`.

## Evidence and remaining boundary

The source basis is the 23-file source05 composition with SHA-256
`88a89fd55ac7ff1326d614f7e0216ffbcc4e353c47b64a4c1c036e3099db7019`.
Root's source-only adoption has SHA-256
`43b790f3082ff3b16c2ee3a99f0622fff07d07f33298b7d74620ed99d0ee71bd`.
The separate integration handoff records exact proposed file membership,
namespace authorship and fresh source audits. Those records describe source
preparation, not a compiler or execution result.

On 2026-10-17 UTC, an admitted SDK-free metadata-only check compiled the actual
public non-test Sherlock A+B module tree at historical commit
`06f7ff03a245e0ab6fae271f09cd75bf813f0330` under pinned rustc 1.99.0. Its version
child and one metadata/dep-info compile exited 0. The normalized compiler read
set is exactly 24 files: 23 production sources and a 67-byte root wrapper.
Compiler stderr retains six warnings. The [selected evidence](audits/2026-10-17/sherlock-production-metadata/README.md)
records exact source pins, declared normalization and the limited result.

This checks module/name/type closure for that exact historical snapshot. It is
not a whole-`mori-core` Cargo build, cfg(test) compilation, test execution or
program-runtime result. Later source changes neither affect nor inherit it.
The earlier source05 ALL13 dispatch incident remains not admitted runtime
evidence, and D01 retention supplies no result for this new check. Native
authority remains zero and C4 stays open.

There is no existing production B consumer in Moriarty. The explicit library API
is the integration delivered here; an application or compiler must separately
choose and supply the B input contract before it can use it. Generic ADTs,
finance-present execution, complete financial/lifecycle semantics, model and
Lean/Clean correspondence, circuits, native proof and target qualification remain
separate obligations. The independent ADT source proposal is not included.
All U0–U7, M1–M17, MC/SP/G and Sherlock acceptance requirements retain their
actual evidence status. This source landing establishes no full Sherlock or
financial-runtime acceptance.
