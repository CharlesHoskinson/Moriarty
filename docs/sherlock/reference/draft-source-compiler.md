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
`draft_contract` and `draft_report`. Parser/checker internals are not public
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
There is no draft runtime observation argument. Target JSON is retained as a
descriptor plus an exact-byte digest; it does not select a network/backend.

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
Final post-result review and Root adoption for original UIREQ-022 remain pending.
These compile-only tables establish no runtime 7/8 result, error trace,
evaluation-order/resource-prefix observation or circuit semantic proof.

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
