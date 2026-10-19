# Proposed public nominal integration contract — revision 02

Status: **not adopted or implemented**. This document revises the integration
contract reviewed by fresh Opus design voters A and B. Both support the existing
API route; both require the amendments now stated here. A fresh Muse amendment
applicability review and Root's adoption disposition remain pending. The author
supplies no design vote. Security analysis stays with its designated owner.
This file changes no production, test, CLI, shared IR or existing public API.

The associated [three static judgments](nominal-static-judgments-20261019.md)
remain proposed. A real public body-typed pipeline is the intended resulting
feature. Keeping the public result permanently at `not-performed` would leave
that feature incomplete; current structural-only routes remain truthful until
the qualified connection is implemented and validated.

## Exact future public route

Use the existing actual public APIs in this order:

1. `parse_nominal_source_bounded(source, budget)` returns the owned
   `ParsedNominal { name, program, provenance }`, under the caller's explicit
   existing `DraftParseBudget`.
2. Run `prepare_ext_program_types(program, &config.layout, config.domain,
   locator.clone())`. Use **exactly** `config.layout` and `config.domain` from
   the eventual `ExtCompileConfig`; do not replace `layout.bounds`, infer a
   domain, copy admission guards or introduce defaults. Retain provenance and
   the complete unchanged `PreparedExtProgramSource` through the body check.
3. Borrow that exact preparation for `check_nominal_program_bodies(&prepared)`.
   Check all function declarations, including unused and later same-named
   declarations, then the closed entry. Calls use the first matching declared
   signature without unfolding. On failure, return the actual typed body error;
   owner admission has not yet run.
4. **I1 disposition: REPORT_ONLY_COMPLETED.** On body success discard the
   `NominalBodyTypeSummary` rather than export its type IDs. Those IDs name only
   the first preparation's table. Move the SAME unchanged `prepared.program`
   into `compile_ext_host(program, config, locator)`; no source rewrite, clone
   or second parser is needed. Do not interpret first-T3 IDs against the second
   preparation or admitted view. No table-equivalence theorem is assumed and no
   first-table clone is retained merely for summary reporting.
5. `compile_ext_host` calls existing `admit_ext_program`, which applies all its
   configuration checks, prepares T3 again, lowers C/P, validates V1–V17 and
   creates the actual `Admitted`. Only then does it call `admit_run` for genesis.
   Retain that actual artifact. No public staged `admit_prepared` API exists;
   do not fabricate one or wrap a pending ProgramView as admitted.

Partial movement of `prepared.program` uses the actual owned carrier, which has
no Drop implementation. The leftover first preparation is not a basis for
interpreting any second-pass IDs. The no-summary-ID contract avoids a new public
table-retention or equality obligation.

## Composite error order and compatibility

**I2: the first T3 and body check run BEFORE owner AD-0a, AD-1, AD-2 and AD-3.**
The existing raw `compile_ext_host` promise that its first preparation operation
is complete owner admission does not apply to this composed text wrapper.
No owner guard is copied, moved, weakened or bypassed. The resulting observable
first-error change and duplicate work require the existing owner/security and
resource dispositions before this route is qualified.

Owner admission still checks, in its current order:

- AD-0a: `config.layout.bounds == config.bounds`.
- AD-1: `bounds.step_cap >= 1`.
- AD-2: optional counter/index widths; every `widths: Some` path refuses,
  either at an earlier width check or finally with `recordWidths`.
- AD-3: `bounds.max_arity >= 2`.
- Owner preparation (second T3, C/P), V1–V17 and remaining owner checks,
  including AD-6 boundary-capacities absence, then genesis.

For a source whose first T3 succeeds, an illtyped body plus `step_cap = 0`
returns BodyType in this wrapper. Direct `compile_ext_host` instead returns
Preparation containing CapacityNotAdmitted/StepCap. For a welltyped source with
`widths: Some`, this route can perform full first T3 and body work before the
guaranteed owner refusal; an earlier first-T3/body error can stop it sooner.
If layout bounds differ from config bounds, use the supplied layout for the
first T3 and let AD-0a refuse at OwnerAdmission if reached. Body success alone
never qualifies that mismatched configuration as a successful compile.

Introduce a **separate new wrapper result and error enum** with exactly these
five stage payloads. This is an interface proposal, not a claim of callable code:

| Stage | Original typed payload / provenance |
| --- | --- |
| Parse | `DraftError` from bounded nominal parsing |
| Preparation | `ExtTypeBuildError` from the first `prepare_ext_program_types` |
| BodyType | `NominalBodyTypeError` from the actual borrowed checker |
| OwnerAdmission | `ExtTypeBuildError` carried by `ExtCompileFailure::Preparation`, covering AD-*, second T3, C/P and V1–V17 |
| Genesis | `Tooling` carried by `ExtCompileFailure::Genesis` |

Do not add variants to the existing externally exhaustive `ExtCompileFailure`
or `NominalSourceError`. Preserve their names, variants and existing consumers.
Translate their existing two variants into the new wrapper stages without
changing payloads. A second-T3 failure is OwnerAdmission, even if first T3 passed;
there is no retry or relabeling. If Debug is implemented, use the existing manual
approach for the non-Debug `ExtTypeBuildError`; do not discard its stored value.

The new result may carry a separate body disposition on owner/genesis failure;
the error enum still has the five stages above. Failures before completion must
not say completed. BodyType means the check ran and failed. OwnerAdmission or
Genesis after body success may say completed and retain their failed stage;
they never return a successful admitted artifact.

## Report and profile contract

Add body-check disposition through a **new wrapper report containing the actual
unchanged `ExtCompileReport` plus disposition and judgment/profile identity**.
This extends ordinary public reporting while preserving existing external
struct literals and exhaustive enum matches. It does not reinterpret or alter
the raw report. A qualified successful report must say completed under the
adopted judgments on this exact source, including all-arm binder uniqueness;
it must never say `not-performed`. The final integrator owns the concrete
serialization field name and new type names, without changing these semantics.

Direct raw `compile_ext_host` calls, existing `prepare_nominal_source_types_bounded`
and parsing/type-table preparation remain outside the body-typed qualification.
Retain their existing truthful structural-only or not-performed status. Public
developers gain no maintainer approval prerequisite. The selected wrapper is
`ExtDomain::GENERIC_ABSENT` only: typed finance is refused in first owner type
preparation; DedicatedSorts is refused by PassTSession::begin before this checker.
Do not advertise its defensive UnsupportedDomain arm as a newly supported route.

The selected profile applies the proposed parameter rule consistent with owner
V4 and the additional all-arm rule that restricts never-selectable unplanned
`cons(x,x)` / `branch(x,x)` arms. It is not equivalent to the broader raw owner
language. First-match signature lookup is an isolated checker fact; owner V3
still rejects duplicate function names. A program of two individually valid
same-named functions can pass body typing and fail OwnerAdmission/V3. Preserve
both outcomes instead of promising that body success implies admission.

## Actual location and span contract

The checker returns `SourceLocation { function_index, child_path }`.
`None` selects Entry; `Some(i)` selects function body i in the unchanged table.
Parser `collect` stores entry preorder before functions, while checking visits
functions first. Lookup must use `SourceOwner` and the exact path, not a shared
ordinal into those differently ordered traversals.

- Ordinary Expr children are in source order: Binary 0/1, Unary 0, Let value 0
  and body 1, If condition 0/then 1/else 2, Call/Construct argument or field
  indices. Match uses scrutinee 0 and arm body `arm + 1`.
- Map an expression error to the matching `ExpressionOccurrence` for that
  owner/path. SourceSpan byte offsets retain parser semantics. Parentheses
  widen the existing expression site's span; they add no path component.
- Pattern errors currently name the owning arm BODY path. Map that body span;
  do not claim an exact pattern/field span merely because separate
  `PatternOccurrence` records exist. Preserve the typed binder payload.
- A checker Literal path may have trailing `ExtValue` constructor field
  indices. Traverse the actual unchanged AST only until reaching Literal,
  truncate there and use that expression span. Do not traverse those suffix
  indices as Expr children or T3 LiteralCtor ordinals. Text parsing currently
  creates Core literals; this rule also covers the public AST checker contract.
- For `DuplicateParameter { index, .. }` at `Some(i), []`, if the matching
  function and parameter site exist, use
  `provenance.functions[i].parameters[index].binder.span`. Keep the original
  source location and typed name/index payload alongside the span.
- A general function-root error maps the function body expression at path `[]`;
  no return-type or parameter location is inferred from that alone. Global
  UnsupportedDomain/IncompletePreparation/Preparation errors have no precise
  expression-site guarantee despite their structural `None, []` location;
  expose no span unless an independently valid source mapping exists. For a
  missing owner/path/parameter record, return an absent span with the intact
  typed location; never fabricate a nearest span or change the error stage.

These conventions are confirmed by the complete actual nominal parser bodies
(`atom_inner`, `match_expression_inner`, `pattern_inner`, `collect`) and public
provenance structs. They are source evidence, not executed parser assertions.

## Deterministic error policy and future ordinary checks

Retain the current checker traversal, without claiming runtime evaluation order:
domain and T3-record consistency checks first; then functions in unchanged table
order, resolving each parameter and checking its duplicate as encountered,
inferring its body and comparing its return; then the closed entry. Unknown
function and arity/constructor metadata checks precede child inference. Children
are inferred left to right; operand/argument types are compared on completion.
An If infers condition and both branches before its condition/result comparison.
A Match infers its scrutinee, binds ALL patterns in arm order, then infers bodies
in arm order and compares results. Pattern binder order is source preorder.

Freeze the proposal's new boundary tests as **single-error tests**, asserting
exact kind, payload, function index and child path. Add separate stage-order
cases for bad body plus zero step cap, welltyped body plus zero step cap,
layout-bounds mismatch and optional-width refusal; their expected stage must
follow the composite order above. Any numeric width/arity values come from
explicit valid inspected configurations, not invented defaults. Retain both
normal public compiler link and its existing owner/genesis error coverage.

No tests are added or run by this revision. The existing 21 functions remain
unrun with their actual freeze history. Later ordinary validation must build the
FULL normal Core rlib and reuse the same test source through the external
consumer; a smaller substitute implementation or cfg(test) Core replay is not
equivalent evidence. Root must issue a new resource disposition before any
normal compile: the historical 30-wall/25-CPU/2GiB-AS/64MiB-file cut failed wall
at host load 31, and this candidate is unmeasured.

## Open obligations and actual sources

This two-T3 route repeats preparation. The checker clones Environment for child
and arm visits; deep lets can cost O(nodes × environment size), hence quadratic
in source size. Call signature lookup is linear per call. No parser budget
establishes a global bound on those costs, and there is no actual fit result.
The composite guard/error order and these facts go to the existing
owner/security/resource review; this language proposal copies no guards and
adds no cap, budget policy or resource authority.

Actual caller/callee sources:

- [Nominal parse/provenance](../../../crates/mori-core/src/sherlock/nominal_source.rs), public bounded parse, result/source structs and child construction/collection.
- [First preparation](../../../crates/mori-core/src/sherlock/helper_machine/shared/owner_prepare.rs), PreparedExtProgramSource, prepare_ext_program_types and PassTSession::begin.
- [Body predicate](../../../crates/mori-core/src/sherlock/nominal_body_typecheck.rs), summary/error definitions and deterministic traversal.
- [Actual raw compiler](../../../crates/mori-core/src/sherlock/compile_artifact/ext_compile.rs), config, exhaustive failure enum, artifact/report and compile_ext_host.
- [Owner admission](../../../crates/mori-core/src/sherlock/helper_machine/shared/owner_ext_admit.rs), AD ordering, prepare_ext_program, V3/V4/V16 and pv_malformed.
- [SourceLocation](../../../crates/mori-core/src/sherlock/draft_source.rs), function index and child path definitions.

Fresh amendment applicability review, Root/spec adoption, reviewed integration,
observed normal compile and exact public-consumer checks remain required.
Source correctness is separate from owner admission correctness, H1, compiler
correspondence, formal/circuit work, financial/lifecycle typing, native proof,
ledger/target behavior and all retained UIREQ/revision03 exits. No document,
opinion, hash or local passing case supplies those missing results.
