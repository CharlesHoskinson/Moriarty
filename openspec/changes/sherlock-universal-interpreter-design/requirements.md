# Sherlock EARS requirements

FINAL DESIGN CANDIDATE B / SPECIFIED-ONLY; Superseded 2026-10-24: adoption and acceptance follow AGENTS.md (factory predicates + holdouts); no vote or reviewer approval is required. The 35 atomic clauses and WHEN/THEN scenarios are normative within this proposed package at [spec.md](specs/universal-interpreter/spec.md). This index does not duplicate or weaken their wording. All are unexecuted; financial/lifecycle extensions remain pending where stated. Existing UNI/MOR/MPLR/AEO/ZR/MNR and Opus-owned security clauses remain unchanged.

| ID | Behavior | Scenario |
|---|---|---|
| [UIREQ-001](specs/universal-interpreter/spec.md#requirement-uireq-001-fixed-interpreter-relation) | Fixed interpreter relation | Two distinct programs |
| [UIREQ-002](specs/universal-interpreter/spec.md#requirement-uireq-002-explicit-supported-profile) | Explicit supported profile | First-order fragment |
| [UIREQ-003](specs/universal-interpreter/spec.md#requirement-uireq-003-lexical-call-scope) | Lexical call scope | Caller-local preservation |
| [UIREQ-004](specs/universal-interpreter/spec.md#requirement-uireq-004-strict-evaluation-order) | Strict evaluation order | Nested arguments |
| [UIREQ-005](specs/universal-interpreter/spec.md#requirement-uireq-005-named-recursive-calls) | Named recursive calls | Self and mutual recursion |
| [UIREQ-006](specs/universal-interpreter/spec.md#requirement-uireq-006-exact-scalar-arithmetic) | Exact scalar arithmetic | Normal arithmetic |
| [UIREQ-007](specs/universal-interpreter/spec.md#requirement-uireq-007-typed-inductive-values) | Typed inductive values | Recursive list value |
| [UIREQ-008](specs/universal-interpreter/spec.md#requirement-uireq-008-selected-match-branch) | Selected match branch | Tree fold |
| [UIREQ-009](specs/universal-interpreter/spec.md#requirement-uireq-009-structural-value-meaning) | Structural value meaning | Repeated subtree |
| [UIREQ-010](specs/universal-interpreter/spec.md#requirement-uireq-010-explicit-pending-return) | Explicit pending return | Factorial multiplication |
| [UIREQ-011](specs/universal-interpreter/spec.md#requirement-uireq-011-complete-pure-result) | Complete pure result | Completed versus pending |
| [UIREQ-012](specs/universal-interpreter/spec.md#requirement-uireq-012-explicit-bounded-outcome) | Explicit bounded outcome | Finite recursive attempt |
| [UIREQ-013](specs/universal-interpreter/spec.md#requirement-uireq-013-versioned-work-observations) | Versioned work observations | Same computation in chunks |
| [UIREQ-014](specs/universal-interpreter/spec.md#requirement-uireq-014-complete-checkpoint-representation) | Complete checkpoint representation | Pending callee snapshot |
| [UIREQ-015](specs/universal-interpreter/spec.md#requirement-uireq-015-pending-return-resumption) | Pending-return resumption | Resume inner call |
| [UIREQ-016](specs/universal-interpreter/spec.md#requirement-uireq-016-chunk-transparency) | Chunk transparency | Boundary before return |
| [UIREQ-017](specs/universal-interpreter/spec.md#requirement-uireq-017-complete-financial-observation) | Complete financial observation | First paid prefix |
| [UIREQ-018](specs/universal-interpreter/spec.md#requirement-uireq-018-existing-await-prefix-preservation) | Existing await-prefix preservation | Await after partial payment |
| [UIREQ-019](specs/universal-interpreter/spec.md#requirement-uireq-019-local-segment-failure-outcome) | Local segment failure outcome | Second segment fails before commit |
| [UIREQ-020](specs/universal-interpreter/spec.md#requirement-uireq-020-target-phase-mapping) | Target phase mapping | Permitted retained fee |
| [UIREQ-021](specs/universal-interpreter/spec.md#requirement-uireq-021-declared-model-projection) | Declared model projection | Finite call-return comparison |
| [UIREQ-022](specs/universal-interpreter/spec.md#requirement-uireq-022-compiler-semantic-mapping) | Compiler semantic mapping | New supported program |
| [UIREQ-023](specs/universal-interpreter/spec.md#requirement-uireq-023-native-relation-inventory) | Native relation inventory | Fixed relation across programs |
| [UIREQ-024](specs/universal-interpreter/spec.md#requirement-uireq-024-optimization-preservation) | Optimization preservation | Frame or sharing optimization |
| [UIREQ-025](specs/universal-interpreter/spec.md#requirement-uireq-025-evidence-class-report) | Evidence-class report | Pure models only |
| [UIREQ-026](specs/universal-interpreter/spec.md#requirement-uireq-026-required-clean-correspondence-stage) | Required Clean correspondence stage | Recursive semantics and bounded step circuit |
| [UIREQ-027](specs/universal-interpreter/spec.md#requirement-uireq-027-indexed-program-image-provenance) | Indexed program-image provenance | Distinct occurrences and recursive references |
| [UIREQ-028](specs/universal-interpreter/spec.md#requirement-uireq-028-bounded-helper-microsteps) | Bounded helper microsteps | Lookup and equality cursors |
| [UIREQ-029](specs/universal-interpreter/spec.md#requirement-uireq-029-bounded-representation-domain) | Bounded representation domain | Complete pending-state encoding |
| [UIREQ-030](specs/universal-interpreter/spec.md#requirement-uireq-030-typed-arithmetic-component-api) | Typed arithmetic component API | Carry, borrow and full product |
| [UIREQ-031](specs/universal-interpreter/spec.md#requirement-uireq-031-separate-compilationexport-plans) | Separate compilation/export plans | Structured witness export and unsupported features |
| [UIREQ-032](specs/universal-interpreter/spec.md#requirement-uireq-032-earliest-stop-reference-slices) | Earliest-stop reference slices | Grouped pending return |
| [UIREQ-033](specs/universal-interpreter/spec.md#requirement-uireq-033-bounded-static-residualization) | Bounded static residualization | Three realization masks |
| [UIREQ-034](specs/universal-interpreter/spec.md#requirement-uireq-034-declaration-driven-ordered-matching) | Declaration-driven ordered matching | New recursive Schedule declaration |
| [UIREQ-035](specs/universal-interpreter/spec.md#requirement-uireq-035-deterministic-witness-dataflow) | Deterministic witness dataflow | Independent normal comparison baseline |

UIREQ-001–026 retain their IDs. The nine additions are functional requirements for Candidate B interfaces defined in [interfaces.md](interfaces.md); they do not add security clauses or establish implementation. The pre-existing canonical 160-clause language index and all ZR01–16/MNR01–08 obligations remain separate and unchanged.

Component/model scope and evidence gaps are in [traceability.md](traceability.md). The first twenty and sixteen additional normal demonstration contracts are in [demonstrators.md](demonstrators.md). No B acceptance task, trace, theorem or native result is marked passed; the three checked preparation tasks retain only their historical linked evidence.
