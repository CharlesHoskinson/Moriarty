**APPROVE — scoped fresh source/applicability approval of candidate `6d4125dc2f58a554c185b08bf55f324d8d41820a`, with no mandatory source changes.**

This approves only the new selected body-typed CLI glue for Root’s next ordinary disposition. It does not build or run the successor, approve a resource fit, or qualify native, target, formal, circuit, financial, semantic-correspondence, H1, or full revision03 closure. All 21 new handler tests remain UNRUN. A new Root ordinary disposition and exact successor normal-Core compilation, handler checks, listing, and individual test execution remain required.

## Traced public chain

The actual chain is intact:

1. The public dispatcher in [compile.rs](/home/charl/Moriarty/.worktrees/sherlock-nominal-body-typed-cli-20261019/crates/mori/src/commands/compile.rs:13) routes explicit Sherlock selection to the selected handler before either native path.
2. [compile_sherlock.rs](/home/charl/Moriarty/.worktrees/sherlock-nominal-body-typed-cli-20261019/crates/mori/src/commands/compile_sherlock.rs) validates `sherlock-helper-microstep/1`, computes the separate `body_typed_nominal` flag from `NOMINAL_BODY_TYPED_PROFILE`, then calls the unchanged public `select_source`.
3. [draft_contract.rs](/home/charl/Moriarty/.worktrees/sherlock-nominal-body-typed-cli-20261019/crates/mori-core/src/sherlock/compile_artifact/draft_contract.rs) recognizes both nominal strings through the pre-existing `SourceSelection::NominalEntry` variant. No public `SourceSelection`, `ExtCompileFailure`, `NominalSourceError`, or other exhaustive public variant set gains a variant.
4. Only `NominalEntry` plus the separate typed flag reaches `SelectedSherlockArtifact::NominalBodyTyped`; structural nominal still reaches the unchanged structural command.
5. [nominal_body_command.rs](/home/charl/Moriarty/.worktrees/sherlock-nominal-body-typed-cli-20261019/crates/mori-core/src/sherlock/compile_artifact/nominal_body_command.rs) decodes the same `NominalConfig`, uses its exact parser-budget and owner projections, and calls the unchanged qualified BW wrapper.
6. That wrapper performs bounded parse, first `prepare_ext_program_types` with the supplied layout and `GENERIC_ABSENT`, all-body and closed-entry checking, discards the first summary, moves the same program into `compile_ext_host`, and therefore into actual owner admission, second T3, and genesis.
7. The handler serializes the completed artifact before creating the output directory and publishing through the existing `publish_new`; no successful artifact is returned or published on a program refusal.

## Rust, serde, and owner-wire compatibility

No source-level visibility, borrow, or type defect was found in the inline glue:

- Widening `NominalPreprocessing::parser_budget` and `NominalConfig::owner` to `pub(super)` is sufficient for the sibling `nominal_body_command` module under `compile_artifact`; their bodies are unchanged apart from visibility.
- `NominalConfig` remains owned for later serialization: `parser_budget` consumes the `Copy` preprocessing value, while `owner()` borrows the config.
- `artifact.report()` and `artifact.name()` are simultaneous immutable borrows. The report correctly wraps the unchanged raw `ExtCompileReport`; `report.owner.owner` is the actual `HostObservation` passed to the existing `wire::admission`.
- `SourceLocation::from(&error.source)` and `SourceSpan::from(...)` connect the same underlying `sherlock_draft` location/span types already used by the BW wrapper and draft report. Optional spans serialize as JSON numbers or `null`.
- Body-error projection preserves every checker kind. Canonical builtin types use the existing `TypeIdC` owner wire—`UInt`, `Bool`, `List`, and `Tree`—while non-builtin IDs retain the existing `TypeId` record convention. Integer payloads that owner wire conventions represent as decimal strings remain strings.
- Parse, first Preparation, BodyType, OwnerAdmission, and Genesis retain distinct stages and original typed payloads. First-preparation and owner-admission `ExtTypeBuildError` values are not relabelled into each other.

## Completed-body truthfulness

The completed flag is not manufactured by the CLI:

- The private `NominalBodyTyped` handler variant can be constructed only after `nominal_body_command::compile` returns a successful artifact, which itself requires the BW artifact whose private fields have no public unchecked constructor.
- Program failures map `OwnerAdmission` and `Genesis` to body status `completed`, `BodyType` to `failed`, and Parse/Preparation to `not-performed`.
- Adopted judgment IDs are emitted only for status `completed`.
- Runtime `CompileFailure.completed` and `evaluation_performed` remain `false`; successful output likewise reports evaluation false, correspondence unproved, native and target unrun, and export unsupported.
- Later serialization/publication failures through the private variant retain the already-observed `source_profile` and `body_typechecking`, without claiming runtime completion. An observation failure inside the new command after successful body checking also truthfully retains completed body status.
- Structural nominal output remains `body_typechecking: "not-performed"` and does not acquire `source_profile`.

## Span and diagnostic preservation

The command does not invent or re-map locations. It converts only the wrapper’s already computed optional span:

- Duplicate parameters retain the second-binder mapping supplied by the wrapper.
- Duplicate arm binders retain the arm-body path and body span, not a guessed pattern-field span.
- Parentheses do not add a checker path component.
- Literal constructor-field suffixes remain governed by the wrapper’s existing literal truncation behavior.
- Missing, ambiguous, invalid, and global sites retain no span.
- The complete checker location and kind remain beside the optional display span.

These are source-level traces against the unchanged BW wrapper and its separate BW30 qualification; they do not transfer that qualification to the new CLI glue.

## Tests and provenance

The 21 expectations are meaningful at source level and cover the requested discriminating cases:

- Two nontrivial recursive/generic positives through the actual selected handler and publication path.
- The actual owner tables, function membership, recursive types, source name, profile, judgments, config identity, and unsupported export/native status.
- An unused bad body, exact second-binder span, unplanned-arm duplicate binder, Eq-Any, a bad Eq operand with parenthesis/path behavior, recursive-call argument checking, and unused or later declarations through the checker’s existing traversal.
- All five program stages, body-before-step-cap ordering, supplied limits and layout/domain behavior, owner V3 after successful duplicate-function body checks, structural regression, selection, observe refusal, unknown profile version, and late publication metadata.

Provenance is stated truthfully and must remain so:

- Initial 20 selected-handler expectations were frozen at `d3aed04c`.
- The public selector assertion at `ba03f7ee` also predates production wiring.
- The 21st publication-failure expectation was frozen at `1f1f5c27` after initial wiring but before its metadata-retention implementation.
- The UInt/Bool expectation correction at `d5f40565` occurred after initial production wiring and before any execution. It correctly uses existing canonical owner-wire labels, but it must not be described as wholly pre-production.
- No expected output was shown to be regenerated from candidate output.
- Existing BW27 and CW five-handler/two-artifact expectations remain confined to their exact prior cuts.

## Documentation and claims

The CLI guide, BW guide, adoption record, command references, explanations, and the added UIREQ-002/UIREQ-027 scenario agree with the actual source within their stated scopes. In particular, they do not claim that:

- BW30 qualifies the new handler or its 21 tests.
- CW structural qualification qualifies this successor.
- body success implies owner admissibility.
- parser budgets bound duplicate-T3/checker total work.
- first-preparation type IDs can be interpreted against second-preparation tables.
- body completion implies runtime, native, target, proof, financial, or whole-requirement success.

## Mandatory changes

None.

## Remaining required gates

Root must still issue the new ordinary resource disposition and run the exact successor qualification: full normal compilation/handler checks for the changed crate and the 21 named tests, without substituting the BW27, old CW, reduced-Core, or `cfg(test)` results. Security/owner judgment remains with the designated reviewer; this nonsecurity source approval supplies none.