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
the latest existing reader records CURRENT eleven adopted originals out of 35,
24 open originals and all 25 refinements open, with no findings, after reviewed
R315/R316 reporting compatibility. The earlier CURRENT10/stale025 snapshot is
preserved as a negative historical cut; this is restored currentness, not a new
requirement adoption. Reading the
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

## Simulate source integration: observed consumers, CLI UNRUN

The installed R293 combined fourteen-file source shares the production simulate `Args`
and pure selection with the public CLI, and adds an output-only report around
the installed public runtime API. R300 returned SUCCESS with a complete
25,013-byte source/operation opinion approving the fourteen-file composition,
mechanical extraction and specified R299/R301 ordinary operations. Root installed
the exact fourteen files after R311 source/result review. R310 now supplies the
new report consumer observation below. R315 returned SUCCESS, Root installed
its exact six test/fixture files and verified all twenty source/test files;
reviewed R316 reporting-document compatibility was applied. R322 later observed
the real selected handler, production file reads and JSON export/printing through
a direct consumer on two fixed inputs. The enclosing root CLI/global dispatch
has not run. Earlier R279 public Rust runtime observations remain separate evidence.

The intended command is:

```text
mori simulate SOURCE --execution-profile sherlock-helper-microstep/1 --source-profile moriarty-sherlock-draft/1 --entry --execution-config CONFIG --target TARGET
```

Each input is explicit; there is no source-profile autodetection. The selected
route refuses action/scenario, resume/event, trace, logical-cap and signed replay
profile/authority/at/signature arguments. Omission of the new selection flags
preserves legacy routing in the installed source. These are source contracts,
not new command/refusal observations or a legacy-handler regression result.
The developer API/command carries no maintainer reviewer-permission requirement.

Public module: `mori_core::sherlock_compile_artifact::draft_simulation_report`.
Its API is:

```rust
pub fn report(run: &DraftHostRun)
    -> Result<DraftSimulationReport<'_>, CompileFailure>;
```

`DraftSimulationReport<'a>` contains `format`, `qualification`,
`preparation: &'a DraftCompileArtifact` and `execution: ExecutionReport`.
The format is `sherlock-host-simulation/1`; qualification is HostOnly
execution without proof/native/target qualification. `preparation` preserves the
unstepped genesis snapshot. `execution` carries `evaluation_performed`,
`completed`, actual owner `summary`, decimal-string `events_forwarded`,
`event_capture: "not-retained"`, actual post-run `image` and `resource_report`.
Neither the report API nor the command stores a full event vector or supplies
an exact independent B-clock/prefix oracle. JSON size/encoding and host
allocation costs remain unmeasured; no new output cap/default is introduced.

The installed handler source reads source/config/target using existing `core_io`, calls
`run_draft_host` once with a non-retaining callback, obtains the report and prints
JSON to stdout. It selects exit zero only for actual `Halted`; `Rejected`,
`Exhausted` and unexpected `Running` select failure. Tooling/input/observation
failures keep existing structured `CompileFailure` metadata where printing
succeeds. A later serialization failure uses `run.failure(...)` to retain actual
completion/count metadata; failed output I/O is not a guarantee of a printed
JSON failure. A successful report wrapper is not a manufactured Halted result.
R310 now observes the report API and JSON serialization in its external Rust
consumer. R322's direct selected-handler processes later exited successfully and
persisted complete output for both fixed programs; negative/error exit behavior
and the enclosing CLI remain UNRUN.

The external Rust test consumes the same production Clap `Args` via
`augment_args`/`FromArgMatches`, then calls shared pure `selection`. Its frozen
24 rows cover explicit draft selection, legacy argument shapes, conflicts and
missing/unknown selection inputs. R299 actually passed all seventeen phases,
including this one test over 24 rows (0.006984103 seconds), with all 1,611 input
pins unchanged. This is not 24 public CLI executions. The test does not construct
the enclosing `Cli`, apply quiet/verbose/color options, dispatch a handler, read
files or invoke runtime/report serialization.

R301 separately compiled the normal Core composition containing the new report
module: actual exit zero in 4.090251816 seconds, all 559 pins unchanged and nine
compiler warnings. Its original capture stays FAIL_STOP because Root compared
two `families/../fragments` dep-info aliases literally. The additive
ROOT-DEPINFO-ALIAS-CORRECTION binds normalized paths to the same frozen bytes/
hashes and records NORMAL_CORE_COMPILE_PASS_NO_RERUN. No compiler failure or
passing-prefix retry is inferred. Compilation checks module/type composition;
this compile result does not itself establish report values. R310 separately
passed three phases: external consumer compilation, exact one-test listing and
one new test looping ChainTotal/BranchTotal, with all 457 input pins and generated
outputs unchanged. That test calls real `run_draft_host`, `report`, serialization
to bytes and JSON parsing. It asserts independently frozen UInt 7/8 and source
identities, complete same-run preparation, distinct genesis/post-run state,
actual forwarded/callback/summary/resource bindings, and explicit unproved/
unmeasured/not-retained qualifications. Raw stdout records test status, not a
persisted report or full event trace. No exact new count oracle, physical cost
measurement, complete B prefix or semantic proof is supplied.

R300's raw coverage predicate also remains false: reported line counts used
LF-count plus one on all 81 pages, while the observer compared LF-count directly.
Root's additive convention correction preserves raw observations, exact byte
counts and explicit complete-source declarations; counts alone do not prove
semantic comprehension. R315 returned SUCCESS and Root adopted the scoped
R310 report result, installed the exact six consumer files and verified all
twenty source/test files. Two reviewed R316 document-reference operations restored
the existing reader to CURRENT11/35 with 24 originals and 25 refinements open,
no findings and no new requirement acceptance. The prior CURRENT10/stale025
negative remains unchanged. Enclosing root CLI/global dispatch, compile
`--observe` and file publication still lack observation, so B04 remains unchecked. See the
[source and receipt archive](../../results/sherlock-simulate-report-20261018/README.md).
This documentation update executes no reader or subject code.

### Real selected-handler reports: observed

R322 passed four serial phases without a retry: helper compilation, ChainTotal
handler execution, BranchTotal handler execution and a saved-files-only check.
The direct consumer parses genuine production Args and runs the complete selected
handler with production source/config/target reads, `run_draft_host`, report,
`mori_core::json::export` and `println!`. The unchanged production 1 MiB rules
apply. All 1,997 input pins matched before and after. Compilation emitted no
diagnostics; dep-info names ten exact members: nine Rust files and one
`include_str!` JSON fixture. R325's earlier nine-total wording remains alongside
the additive count correction; source and argv did not change.

Complete stdout JSON is preserved: 59,814 bytes for ChainTotal and 64,160 for
BranchTotal, with both exits zero and empty stderr. The separate verifier reads
those two saved outputs and frozen R324 predicates; it does not run the owner
again. It passed independent UInt 7/8 and source/config/target identities,
genesis/post-state separation, declared keysets and same-output counter relations.
R324 discloses earlier report-implementation exposure and incidental declaration
conversion context; it did not inspect this handler/harness or new outputs before
freezing expectations. These are bounded checks, without independently expected
complete tables, exact B-clock totals or an event/intermediate-state trace.

The seven-file extraction preserves public `Report` signatures and error text/source chains
but changes the private diagnostic's concrete downcast type; the helper does not
compile `core_io`/parent dispatch or establish full-crate downcast compatibility.
Full root CLI/global/outer dispatch, negative-handler/default-Core coverage,
compile `--observe`, file publication, H1/formal/native/target and whole B04 remain
separate. This observation proposes no new EARS adoption or developer approval gate.

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
