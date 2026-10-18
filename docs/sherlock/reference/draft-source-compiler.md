# Dedicated draft source compiler

The public Rust draft route prepares a closed program entry as admitted HostOnly
B tables and an unstepped genesis image. It preserves source calls, constructors
and ordered match arms. Observed on 2026-10-18: current Core and the external
public consumer compiled, and both public fixture tests passed. The CLI path
remains unrun. See the [observed result](../../results/sherlock-public-draft-compiler-20261018/README.md).
The existing [Core1 scalar action route](host-source-compiler.md) has its own
configuration, artifact format and historical results.

## Public API and supported source

Module: `mori_core::sherlock_compile_artifact`.

```rust
pub fn parse_draft_config(source: &str)
    -> Result<draft_contract::DraftExecutionConfig, CompileFailure>;

pub fn compile_draft_host(
    source: &str,
    config_source: &str,
    target_source: &str,
) -> Result<draft_report::DraftCompileArtifact, CompileFailure>;
```

The [public module](../../../crates/mori-core/src/lib.rs) exports the
[compiler](../../../crates/mori-core/src/sherlock/compile_artifact.rs),
`draft_contract`, `draft_report` and the installed `draft_profile` report types.
Parser/checker internals are not public
client entry points. `parse_draft_config` decodes the distinct wire shape;
successful decoding alone does not establish program or owner admission.

Source uses `language "moriarty-sherlock-draft/1"`, a named `program`, typed
`fn` declarations and one closed `entry`. Its semantics identity is
`moriarty-sherlock-dedicated/1`; the B execution profile is
`sherlock-helper-microstep/1`. Supported sorts are `UInt`, `Bool`, `List` and
`Tree`. Expressions include literals, variables, let, if, named calls, match,
`+`, `-`, `*`, `==`, `<`, `cons`, `branch`, `leaf` and `not`. Patterns are
variable, nil, cons, leaf and branch. Named recursive calls remain call nodes;
preparation neither unfolds recursion nor replaces a program with an evaluated
literal. All function bodies are checked before the entry.

The two supplied programs exercise nested calls with dedicated list/tree
constructors and matching. This scope adds no generic ADT grammar, financial
effects, external entry parameters, Core2 continuation lowering or native export.
`compile_draft_host` has no runtime observation argument; execution uses the
separate public `run_draft_host` API below. Target JSON is retained as a
descriptor plus an exact-byte digest; it does not select a network/backend.

## Supported-profile report

The R254 source overlay adds `DraftCompileArtifact.supported_profile` with
format `sherlock-supported-profile/1`. Its producer calls
`draft_profile::report(&config.fields)` on the same config identity carried by
this admitted artifact. `compile_draft_host` keeps the three arguments above.
The report is output metadata: it neither evaluates expressions nor adds a
grammar feature, admission check, guard, resource default or native interface.

Observed R256 validation passed all five phases: normal Core compilation
(10.166786 seconds), public consumer compilation, exact two-test listing and
both exact supported-profile tests. All 723 selected inputs remained unchanged.
The first test compares the complete returned report; the second changes all
22 bounds and selected layout/run/preprocessing values and checks that actual
invocation. Both calls use the same ProfileFragment source; they are not two
new source programs. R257's fresh review approved the supplied 40 complete
bodies/55 pages and the exact five-phase plan, with its coverage and owner-gap
limits. Root withheld the inaccurate R263 result reply and retained it unchanged.
Fresh R272 literal-result review accepted the exact observations and reporting
scope; Root subsequently adopted original UIREQ-002. See the
[profile result and adoption](../../results/sherlock-supported-profile-20261018/README.md).
No B runtime, CLI publication, physical H1 or native result follows from these
report/preparation observations. Earlier public fixtures retain their exact
historical producer results.

Original UIREQ-022 remains adopted. With original UIREQ-002 now adopted,
the current evidence check records 11 accepted originals out of 35 and 24 open. Reading the returned
public report requires no reviewer permission; these review/adoption statuses
describe project evidence, not developer authorization.

The complete returned inventory is:

| Report member | Source contract |
| --- | --- |
| `format`, `source_language`, `source_semantics`, `execution_profile`, `qualification` | Report `/1`, draft language `/1`, dedicated semantics `/1`, selected B helper-microstep `/1`; HostOnly prepared/admitted, runtime and semantic correspondence unobserved |
| `grammar` | Selected `DedicatedSorts` domain, lexical variables, closed entry and immutable source-ordered named function table; expression/pattern/constructor inventory and extension dispositions below |
| `numeric_domains` | Dedicated value domains, checked arithmetic, structural equality and no source-visible pointer identity |
| `schedule` | Selected versioned profile, charges, helper families, distinct counters and unmeasured physical costs |
| `extents` | Actual caller bounds, layout, fuel, stack and preprocessing; serialization/frame/work scope qualifications |
| `admission_scope` | Source typecheck extent; explicit owner closed-body/unique-function work; pure `Absent` finance |
| `dispositions` | Source preparation failures, owner tooling failures, runtime outcomes, separate HostViolation and unsupported native export |

The grammar inventory is `Literal`, `Var`, `Let`, `If`, `Call`, `Match`,
`Binary.Add`, `Binary.Sub`, `Binary.Mul`, `Binary.Eq`, `Binary.Lt`,
`Binary.Cons`, `Binary.Branch`, `Unary.Leaf`, `Unary.Not`; patterns are `Var`,
`Nil`, `Cons`, `Leaf`, `Branch`. Constructors are `Nil`, `Cons(UInt128,List)`,
`Leaf(UInt128)` and `Branch(Tree,Tree)`. Named calls use the first declaration;
self and mutual recursion remain call nodes, without source-recursion unrolling.
The selected semantics use strict left-to-right operands, arguments and
constructor fields, and the first matching source arm with ordered binders and
one scrutinee evaluation. Preparation reports those semantics without running them.
Higher-order closures and `letrec` are unsupported extensions. Generic nominal
ADTs require a separate declaration/constructor extension; dedicated List/Tree
sorts do not provide it.

`UInt` is checked UInt128 in `[0,340282366920938463463374607431768211455]`;
add/sub/mul do not wrap. `Bool` is false/true. Lists are finite immutable UInt128
lists; trees are finite immutable binary trees with UInt128 leaf payloads.
Equality is structural over the dedicated value sum; different variants compare
false. Pointer identity is not source-visible and cyclic values are unsupported.

The selected `sherlock-helper-microstep/1` schedule reports charge `1` for a
legacy completion and each explicit helper transition, and `0` for startup,
terminal and error publication. Helper families are `Scalar`, `EnvGet`, `EnvPut`,
`Find`, `Bind`, `Append`, `Validate`, `Equal`, `Match`. `consumed_work` includes
charged helper work; `helper_work`, `helper_visits` and `reference_cursor` remain
distinct. The A `sherlock-k-a/1` reference leaves pure helpers unmetered; equal
A/B fuel and runtime correspondence are not claimed. Host heap/stack/time,
circuit rows/degree and proof/target costs are unmeasured.

Source checking covers lexical variables, signatures and sorts in all function
bodies and entry under `max_type_depth`; calls inspect signatures without
recursive callee descent. The report explicitly retains owner closed-body and
unique-function admission as implementation work. First-declaration lookup is
not a unique-name guarantee. Transfers, fees, liabilities, wait and resume are
outside the dedicated pure `Absent` grammar.

## Configuration and independent extents

`DraftExecutionConfig` requires `format`, `profile`, `grammar`, `finance`,
`bounds`, `layout`, `fuel`, `stack` and `preprocessing`. Fixed tags are
`sherlock-draft-host-config/1`, `sherlock-helper-microstep/1`, `DedicatedSorts`
and `Absent`. Bounds/layout/run-input types are shared with the
[HostOnly configuration](host-source-config.md); the draft uses `preprocessing`
instead of that route's `host` object. Numeric extents are required canonical
nonnegative decimal strings (`UsizeDecimal`), not JSON numbers. No numerical
default is supplied; unknown preprocessing fields are refused.

The complete [draft config fixture](../../../crates/mori-core/tests/fixtures/draft-compile/config.json)
contains all 16 fields below. Its numerical values are specimen inputs, not
recommended caps or a process-wide resource policy.

| Preprocessing field | Extent consumed by the draft pipeline |
| --- | --- |
| `max_source_bytes` | UTF-8 source bytes before lexing |
| `max_tokens` | Tokens including EOF, excluding comments |
| `max_functions` | Function count, also forwarded to copy |
| `max_parameters_per_function` | Per-function parameters, also forwarded to copy |
| `max_total_parameters` | Total parameters, also forwarded to copy |
| `max_ast_nodes` | Parser Expr + Pattern occurrences, without pointer deduplication |
| `max_parse_depth` | Active parser descent bodies; parentheses consume depth |
| `max_structural_ast_depth` | Expr root-to-leaf depth, leaf one |
| `max_type_depth` | Structural Expr inference depth; body/entry root one |
| `max_copy_depth` | Independent structural CoreExpr copy depth, root one |
| `max_copy_expr_nodes` | Total copied CoreExpr occurrences across entry and bodies |
| `max_copy_match_arms` | Total copied match arms and their patterns |
| `max_copy_name_bytes` | Total UTF-8 bytes cloned for names across the copied program |
| `max_copy_list_elements` | Total elements copied from literal List values |
| `max_copy_tree_nodes` | Total nodes copied from literal Tree values |
| `max_copy_tree_depth` | Depth within each literal Tree, root one |

The report's `extents.bounds` contains all 22 existing caller fields:

| Capacity group | Fields |
| --- | --- |
| Code and declarations | `code_rows`, `function_rows`, `code_child_slots`, `parameter_slots`, `pattern_rows`, `match_plan_rows`, `match_arm_rows`, `type_rows`, `constructor_rows`, `constructor_field_slots`, `max_arity` |
| Constants and data | `constant_nodes`, `constant_field_slots`, `data_nodes`, `data_field_slots` |
| Environment and continuation storage | `environment_cells`, `frames`, `helper_tasks` |
| Image/checkpoint serialization | `serialization_bytes` |
| Logical work | `max_reference_work`, `max_helper_work`, `step_cap` |

The first 19 bounds use canonical decimal strings checked into `usize`; the
three work fields use `u128`. These are capacities from this invocation, not
measured usage. `serialization_bytes` describes owner image/checkpoint capacity;
the report certifies no bound on public JSON size or encoding costs. `frames`
is logical image capacity, while `stack` is the separately supplied logical run
stack limit. The three work bounds and supplied run `fuel` remain distinct.

`extents.layout` preserves `layout_version` (two strings), required explicit
nulls `field_id`, `widths`, `scalar_layout_id`, the four `capacities` fields `name_rows`,
`name_bytes`, `occurrence_path_slots`, `value_slots`, and
`boundary_capacities_present`. Capacities use `UsizeDecimal`; the HostOnly
fixture has no boundary capacities. `extents.fuel` and `extents.stack` retain
`RunInput`, tagged `Nat` with a canonical UInt128 decimal `value`, `Negative`
or `NatAboveU128`; the fixture selects `Nat`. These input forms do not create a
new admission policy. The report copies the supplied layout/run/preprocessing
values rather than substituting fixture constants or inferring physical bounds.

The final six copy fields are separate required inputs.
`DraftPreprocessingExtents::parser_budget()` forwards the eight parser extents;
`copy_budget()` forwards the exact ten `SourceCopyBudget` fields. The checker
receives `max_type_depth.0` unchanged. List/tree literal-copy budgets do not count
future runtime values produced by `cons`/`leaf`/`branch` expressions. Original
lexer hard limits still apply independently of supplied extents.

These checks cover their named preparation operations. Recursive destruction of
A values, transient allocations during B destruction, owner admission/preparation allocations,
report/provenance/diagnostic/JSON allocations and global host time/heap/stack
admission remain separate H1 limitations. A finite caller extent is not a
general guarantee for untrusted operation.

## Artifact and diagnostics

`DraftCompileArtifact` serializes as `sherlock-host-artifact/2` with qualification
`HostOnly dedicated draft preparation; no constraints or native export`.

| Field | Content |
| --- | --- |
| `source` | Language, semantics, program, `{ "kind": "entry" }` selection and exact source digest |
| `execution_profile`, `input_policy` | B profile; `closed-entry` with empty declared/supplied inputs |
| `config`, `target` | Complete typed config/descriptor and exact-byte file digests |
| `preparation` | Function/parameter sorts, full provenance, checked type summary and owner join description |
| `supported_profile` | Complete grammar/domain/schedule inventory and this invocation’s extents, admission gaps and dispositions; two R256 public producer tests passed; original UIREQ-002 reporting scope adopted after R272 review |
| `correspondence` | Named construct, value, failure, order, resource and finance obligations |
| `admission`, `genesis` | Views projected from the same newly admitted, unstepped pair |
| `plans` | Witness/constraint `missing`, export `unsupported`; artifacts null |
| `observation` | `not-requested`, `evaluation_performed: false` |
| `evidence` | Preparation `performed`, host observation `genesis-only`, native/target `unrun`, acceptance `unestablished` |

`admission.program_view` retains all twelve owner tables: entry, expressions,
occurrences, occurrence_paths, children, functions, parameters, arms, patterns,
constants, constant_fields and names. Provenance retains spans, child paths,
expression kinds and pattern binders, in entry preorder then original function
body order. The artifact is output-only; no executable-image reopen API is added.

`CompileFailure` retains `stage`, `actual_error`, `evaluation_performed`,
`events_forwarded` and `completed`. Draft failures report false, `"0"` and false
for the last three. Stages distinguish `config`, `parse`, `typecheck`, `copy`,
`admit`, `admit-run` and projection `observation`. Source/type/copy errors retain
their own diagnostic payloads; owner admission failures retain `Tooling`, and
projection failure retains `HostViolation`. These are not manufactured runtime
outcomes. Copy budget errors preserve `resource`, decimal-string `limit` and
nullable decimal-string `observed`, `function_index`, `child_index`.

The report names parse/typecheck/copy preparation failures and owner
`AdmissionRejected`, `CapacityNotAdmitted`, `UnsupportedFeature` as tooling
failures, preserving a locator when supplied. Unsupported syntax can instead
produce a located parse diagnostic. These remain outside runtime `Outcome`:
`Running`, `Halted`, `Rejected`, `Exhausted`. In particular, runtime
`Rejected(error4)` frame refusal and `Exhausted(kind)` limits are not tooling
admission or unsupported-feature decisions. `HostViolation` is a separate owner
invariant/observation failure, never a successful application outcome. Reporting
these distinctions adds no execution evidence or global H1 guarantee.

## Correspondence output and test boundary

`draft_report::correspondence::SemanticCorrespondenceReport` has mapping ID
`moriarty-sherlock-draft-to-b-obligations`, version `1`, status `named-unproved`.
Its 26 ordered obligations cover Program/Function, every supported Expr family
and operator, all five patterns and four sorts. Each has `id`, `construct`,
`target`, `returned_values`, `defined_failures`, `order`,
`resource_obligation`, `effects_and_duties` and `status`. The report also carries
`resources`, `finance` and `proof` objects.

Resource mapping distinguishes the A charged-rule clock from B consumed/helper
work, helper visits and reference cursor. Equal fuel or helper schedules are not
claimed. Effects and residual duties are inapplicable for this pure
`FinanceInput::Absent` profile. Semantic correspondence is `unproved`; runtime
value/failure/order/resource comparisons are unobserved by compile-only
preparation. Constraint/witness plans and native export remain absent.

The two [public consumer tests](../../../crates/mori-core/tests/sherlock_draft_compile.rs)
passed through the canonical external public API consumer. ChainTotal uses the
177-byte source fixture; BranchTotal uses the 211-byte fixture. Each checks all
four sealed R115 comparison fields: source digest, full source identity, full
preparation and all twelve ProgramView tables, including every ordered nested
cell. They also check the complete correspondence object with its 26 obligations
and resource/finance/proof records, plus config, target, input-policy, plan,
observation and evidence metadata. `CORRESPONDENCE-EXPECTED.json` is an ordinary
author reference contract, separately checked in the fresh source review; it is
not an independently sealed extension of R115.

All five public-check phases passed: normal current Core compilation (5.247679
seconds), external consumer compilation, exact two-test listing, and each test.
The separate bounded-copy check passed all nine phases, including seven exact
tests. The fresh current-source review approved its supplied 64 pages/52 bodies.
Original UIREQ-022 was adopted after corrected final result review; the new
R254 supported-profile field has two new passing R256 tests. Root adopted original UIREQ-002 after fresh R272 literal-result review,
without rerunning the passing tests.
These compile-only tables establish no runtime 7/8 result, error trace,
evaluation-order/resource-prefix observation or circuit semantic proof.


## Public draft runtime API

The installed dedicated-source runtime interface is under
`mori_core::sherlock_compile_artifact::draft_execution`:

```rust
pub fn run_draft_host(
    source: &str,
    config_source: &str,
    target_source: &str,
    sink: &mut dyn FnMut(&StepEventC),
) -> Result<DraftHostRun, CompileFailure>;
```

Its `DraftHostRun` contains `preparation: DraftCompileArtifact` and
`completed: CompletedRun`, with `observe()` and `resource_report()` methods.
The source forwards actual full `StepEventC` callbacks from the fixed B owner's
run of the same admitted pair. The preparation snapshot remains genesis-only;
a run wrapper returning `Ok` would not itself mean `Halted`, since completed
owner outcomes can include `Rejected` or `Exhausted`. Callback retention is
caller-owned. The unstepped preparation snapshot is not relabelled as runtime
evidence; post-run state is read from `completed`, `observe()` and
`resource_report()`. The observed cases below establish no global H1 guarantee.

The [observed public runtime result](../../results/sherlock-public-runtime-20261018/README.md)
records four passing R279 phases: external consumer compilation against the
unchanged R264 Core library, exact two-test listing, ChainTotal and BranchTotal.
All 636 selected pins remained unchanged. The independently frozen tests assert
actual owner-bound Halted/Return UInt values 7 and 8, all twelve ordered
ProgramView tables, recursive call order and 3/5 invoke/bind/return completions,
plus relational event/ordinal/counter/resource conservation. Raw stdout records
libtest status. Complete event payloads, numeric counter totals, full intermediate
state prefixes and an independent exact B-clock oracle were not persisted.

[Root adopted the scoped result](../../results/sherlock-public-runtime-20261018/ROOT-ADOPTION.json)
after successful fresh R281 operation review and R286 result/installation review,
and installed exactly three source files and seven test/fixture files. Dated
SOURCE_ONLY/UNRUN author comments remain unchanged as historical metadata; they
do not describe the later observed test cut. This runtime installation adds no
original EARS adoption to the retained eleven-original baseline.

Earlier failures remain history. Root first miscompared the alphabetic test list
as an ordered sequence; the exact set/count correction did not rerun its passing
compile/list prefix. ChainTotal then refused before stepping because value-slot
capacity 1024 was below supplied fuel 4096, and BranchTotal was not run. The
independent input correction changed only value slots to 4096, the config-hash
literal and expected-input authorship metadata; source semantics and assertions
were unchanged. R279 reused the passed Core build. Five owner cleanup/allocation/
report/global-H1 gaps, caller-owned event retention and uncaught sink-panic scope
remain; finite logical bounds and these two observations do not certify physical
resources, semantic proof or arbitrary-input execution.

[Root's status correction](../../results/sherlock-supported-profile-20261018/ROOT-RUNTIME-REVIEW-STATUS-CORRECTION.json)
records that the prior runtime reviewer exited CLI 0 but returned terminal
`ERROR`. Its visible opinion is retained without adoption; it is not a
successful approval. These project execution records create no public developer
permission rule.
The later R279/R286 observations supply these two host results. Full semantic
correspondence, defined-failure/full-prefix coverage, CLI execution, circuit proof
and native/target outcomes remain separate unobserved requirements.

## Explicit CLI selection

The source CLI path selects draft preparation only with
`--execution-profile sherlock-helper-microstep/1`,
`--source-profile moriarty-sherlock-draft/1`, `--entry`,
`--execution-config`, `--target` and a new `--out` directory. It publishes
`sherlock-artifact.json`. Draft `--observe` and `--action` selection are refused;
omitted draft selection preserves the existing Core action route. The CLI path
is unrun here. For a concrete two-program client, see
[compile dedicated draft fixtures](../how-to/compile-draft-program.md).


Original UIREQ-022 was subsequently adopted after corrected final result review.
See the [Root adoption](../../results/sherlock-public-draft-compiler-20261018/ROOT-ORIGINAL022-ADOPTION.json).
This accepts the original named-mapping report clause. All 26 semantic
correspondence obligations remain unproved; runtime, CLI, witness/constraint,
native and target evidence retain their separate requirements.
