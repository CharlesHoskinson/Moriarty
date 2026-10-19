# Compile a body-typed nominal source

The BW47b284b1 public Core cut subsequently passed Root's full normal Core and
external27 qualification. The selected CLI integration subsequently passed its own normal Core/real-handler24
qualification; see [the CLI guide](compile-body-typed-nominal-cli.md). The callable public Core entry is
`mori_core::sherlock_compile_artifact::nominal_body_compile::compile_nominal_source_host_bounded`.
It takes source text, the existing explicit `DraftParseBudget`, an existing
`ExtCompileConfig` and optional owner `LocatorC`. Public developers need no
maintainer approval. The explicit selected CLI profile has a separately observed21-test handler cut;
its qualification remains independent of the BW library observations.

Use `ExtDomain::GENERIC_ABSENT`. Supply the actual layout, bounds, fuel and stack;
the wrapper supplies no defaults. It parses bounded text, prepares the same owned
program with the supplied layout/domain, checks every function body and closed
entry, discards the first type summary, then moves the unchanged program into
`compile_ext_host`. The raw compiler performs full owner admission, including a
second T3, and creates the real genesis image. It performs no interpreter steps.

On success, `artifact.report()` wraps the unchanged `ExtCompileReport` under
`owner`. `source_profile` is `moriarty-sherlock-nominal-body-typed/1` and
`body_check.status` is `completed`. The named judgments are Eq-Any, unique
parameters in every declaration, and unique binders in every arm. No first-T3
type IDs are reported. `artifact.owner()` borrows the real raw artifact;
`into_parts()` returns its ownership together with source name and provenance.

Handle `NominalBodyCompileFailure.error` by its five distinct variants:
`Parse(DraftError)`, `Preparation(ExtTypeBuildError)`,
`BodyType(NominalBodyTypeError)`, `OwnerAdmission(ExtTypeBuildError)` and
`Genesis(Tooling)`. `span` is optional exact source provenance for body errors;
the original typed location and error remain available. Pattern binder errors
map the arm body. Duplicate parameter errors map the repeated parameter binder.
Literal value-field paths truncate at the actual literal expression.
Missing, invalid or ambiguous sites have no span.

`body_check_completed()` is true for owner/genesis failures reached after body
success. These failures do not carry a successful artifact. An earlier error
never reports completed typing. Raw `compile_ext_host` and structural nominal
preparation retain their existing qualification and interfaces.

The wrapper checks bodies before owner AD-0a/1/2/3. Thus an illtyped body plus
zero step cap fails at BodyType; a welltyped body reaches the owner StepCap
refusal. Layout mismatch and every widths:Some path retain actual owner
refusals after first preparation/body work if reached. First preparation uses
the supplied layout even when its bounds differ from config.bounds. Do not
infer owner admissibility from body success: duplicate function names can pass
this checker and fail owner V3.

This profile adds no financial/lifecycle support, new ISA, native proof,
compiler correspondence or requirement closure. First T3 is repeated; cloned
environments can cause quadratic checking work and named-call lookup is linear.
Parser limits do not bound that total work. The exact BW27 and NW21 checks passed after their applicable source reviews and
Root dispositions. A new source cut needs its own applicable qualification.
