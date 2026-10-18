# R254 first-order supported-profile source proposal

The missing UIREQ-002 behavior is an explicit supported-profile report on the
actual admitted public result. The frozen producer already returns source and
execution identities, config bounds and correspondence obligations. It lacks a
single explicit grammar/domain/schedule/admission-gap/refusal inventory.

This R-only proposal adds `DraftCompileArtifact.supported_profile`, produced by
`draft_profile::report(&config.fields)` after observing the fresh admitted pair
and before moving that same config into the artifact. The output-only leaf
copies existing profile, grammar, all22 bounds, layout capacities, run fuel/stack
and all16 preprocessing extents. It adds no limits, defaults, guard, grammar
feature, admission rule, evaluator or native interface.

The exact producer predicate is: call `compile_draft_host` on the independent
`ProfileFragment` source with named calls, UInt128, Bool, Cons/Nil and Match;
serialize its returned `supported_profile`; compare every field with the literal
expected report. A second public call changes all22 bound values and selected
layout/run/preprocessing inputs to distinguish per-invocation projection from
a report detached from the caller's config. Both test sources are **UNRUN**.
The independent test/literal freeze precedes report-leaf authoring; no observed
red/green cycle is claimed under this source-only assignment.

| Original scenario observation | Returned field |
|---|---|
| Exact grammar and numeric domains | `grammar`, `numeric_domains`, source identities |
| Versioned execution profile and schedule | `execution_profile`, `schedule` |
| Code/data/frame/environment/serialization/helper-work bounds | `extents.bounds` plus layout/run/preprocessing extents and their scopes |
| Closed-body and unique-function admission work | `admission_scope`; owner checks remain explicit implementation work, while source typechecking is separately described |
| Higher-order closures and letrec extensions | `grammar.higher_order_closures`, `grammar.letrec` |
| Dedicated sorts versus generic nominal ADTs | `grammar.generic_nominal_adts` |
| AdmissionRejected/tooling Unsupported versus runtime outcomes | `dispositions`; actual supplied locators and separate parse diagnostics retain their existing meanings |

`ORIGINAL002-EVIDENCE.json` retains the full original clause/scenario and current
open evidence row without narrowing it. Native execution and a full semantic
proof are not prerequisites added by this metadata proposal. Runtime behavior,
physical H1 guarantees, public JSON size and serialization enforcement remain
unobserved or uncertified as stated in the report. Generic ADT and financial
extensions remain separate. No original002 closure or evidence-ledger edit occurs.

The candidate has **six files**: two source modifications (module registration
and returned-artifact field/wiring), one new report leaf, one external test and
two independent fixtures. `APPLY-RECIPE.json` lists exact before/after pins;
`CANDIDATE.PATCH` is the same change in unified format. `basis/` supplies full
source references needed for review. The PR185 24-file source membership remains
unchanged in W; `PR185-SOURCE-READBACK.json` records exact byte comparisons.

Root alone obtains the fresh designated exact-candidate audit, installs source,
selects any ordinary compile/test operation and decides adoption. Earlier R239
results remain historical for their exact bytes and do not execute this changed
candidate. `STATIC-VALIDATION.json` records data/source readback only. No source,
compiler, test, provider, model, native or Git execution was performed.
