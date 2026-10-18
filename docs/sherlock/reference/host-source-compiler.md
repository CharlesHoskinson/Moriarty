# HostOnly source compiler API and artifact

Historical Core1 result cut, 2026-10-17: the production Core library builds and has produced two fixed compile-only captures. CLI integration is source in draft PR170; the CLI binary has not been built/executed, and the explicit observation branch has not been executed in this result. Mandatory R27 source corrections remain pending. These positive results do not approve those corrections. See the [exact result package](../../results/sherlock-source-compile-20261017/).

This page retains the Core1 scalar artifact/1 contract and its historical result
cut. The separate [dedicated draft compiler](draft-source-compiler.md) now has a
real public compile-only body for typed functions, nested calls and list/tree
constructors, with artifact/2 and a distinct configuration. That source candidate
has now compiled through the current Core library and passed both public fixture
tests. See its [2026-10-18 result](../../results/sherlock-public-draft-compiler-20261018/README.md);
that result does not rerun or broaden the historical Core1 captures below.

## Public Rust entry points

Module: `mori_core::sherlock_compile_artifact`, registered in [lib.rs](../../../crates/mori-core/src/lib.rs#L11).

```rust
pub const EXECUTION_PROFILE: &str = "sherlock-helper-microstep/1";

pub fn parse_config(source: &str)
    -> Result<config::ExecutionConfig, CompileFailure>;

pub fn compile_host(
    source: &str,
    action_name: &str,
    config_source: &str,
    target_source: &str,
    observe: bool,
) -> Result<CompileArtifact, CompileFailure>;
```

[Exact declarations and implementation](../../../crates/mori-core/src/sherlock/compile_artifact.rs#L165). `config_source` is complete [sherlock-host-config/1 JSON](host-source-config.md). `target_source` is parsed JSON retained as a descriptor and exact-byte digest; it does not select a native network or SDK backend.

The selected source action must have no parameters. The route uses the public Core/1 parser, current scalar source projection, a real zero-argument call to the generated function, and the existing B owner admission. Lets/operators remain in the program tables; the compiler does not evaluate a source result and replace its structure with a literal. An emit contributes its scalar value to this projection, not an executed financial effect. The lower-level [prepare_scalar_action_host](../../../crates/mori-core/src/sherlock/helper_machine/core_action_consumer.rs#L81) returns `CoreActionPrepared { preparation, admitted, image }`; `observe()` on that pair borrows actual owner-bound fields without executing B steps.

`observe=false` prepares/admit-runs that pair and returns `Observation::NotRequested { evaluation_performed: false }`; no owner run occurs. `observe=true` selects the separate run consumer and serializes the returned run's own admission/image pair. That branch exists in source but has no execution result in this package.

Current scope excludes parameterized actions, Core/2, state/assets/financial effects, source functions/recursion, generic ADTs and non-scalar source values. These remain broader Sherlock requirements. HostOnly admission is not a bounded native encoding, persistent checkpoint, witness/constraint export, proof or stagenet result.

The draft preflight correction checks configuration/global source extents before
action selection and selected input/name extents before the unsupported-parameter
diagnostic. Five owner-authored regression functions typecheck; execution and
fresh combined review remain pending. The [correction record](../../results/sherlock-compiler-preflight-20261017/README.md)
preserves both the original API compilation failure and later integration-check
limitations. This does not extend the supported input policy.

## Artifact: `sherlock-host-artifact/1`

`CompileArtifact` is a serializable, output-only record; there is no executable-image reopen/import API. [Complete declarations](../../../crates/mori-core/src/sherlock/compile_artifact.rs#L35):

| Field | Representation |
| --- | --- |
| `format`, `qualification`, `execution_profile` | Format above; `host-only-scalar-value-projection; no effects/constraints/native/target qualification`; B profile |
| `source` | Language, semantics, agreement, exact action, decimal-string action index, exact source-byte digest |
| `input_policy` | `kind: "parameter-free"`, empty declared/supplied arrays |
| `config` | All supplied typed configuration fields flattened with `file_digest` |
| `target` | `{ descriptor, file_digest }` |
| `preparation` | Existing `ScalarActionReport`, preserving projected occurrence/source locations |
| `admission` | Actual admitted identity, program view, bounds and layout from the owner-bound pair |
| `plans` | Witness/constraint: `status: "not-implemented"`, `artifact: null`; export: `status: "unsupported"`, `artifact: null`; each has its reason |
| `observation` | Compile-only: `{ "status": "not-requested", "evaluation_performed": false }`; source observation branch additionally carries actual summary, forwarded-event count, image and resource report |
| `evidence` | Preparation/observation disposition; `native: "unrun"`, `target: "unrun"`, `acceptance: "none"` |

The private wire module projects real owner records. ProgramView's twelve fields are `entry`, `expressions`, `occurrences`, `occurrence_paths`, `children`, `functions`, `parameters`, `arms`, `patterns`, `constants`, `constant_fields`, `names`. Struct keys/enum variant names follow the typed records, vectors preserve order/multiplicity, owner integer fields use decimal strings, and optional values preserve null/present distinctions. See [ProgramView projection](../../../crates/mori-core/src/sherlock/compile_artifact/wire.rs#L596), [admission/image projection](../../../crates/mori-core/src/sherlock/compile_artifact/wire.rs#L968). This output projection is not a public native ABI.

`CompileFailure` exposes `stage`, `actual_error`, `evaluation_performed`, decimal-string `events_forwarded`, `completed`; [declaration](../../../crates/mori-core/src/sherlock/compile_artifact.rs#L96). Failure records and completed semantic outcomes remain distinct. This page makes no guard or diagnostic acceptance claim; the pending R27 correction has its separate source/review owner.

## CLI source contract — unbuilt

[compile.rs](../../../crates/mori/src/commands/compile.rs#L9) adds `--execution-profile`, `--action`, `--execution-config` and `--observe` to the existing compile arguments. Only `sherlock-helper-microstep/1` is selected by this source branch. Action/config are required with selection; target and a new output directory remain required. Omitted execution selection follows ordinary Core compilation. Selected source wiring publishes `sherlock-artifact.json`, not ordinary `artifact.json`/ZKIR artifacts. These declarations are not a demonstrated CLI invocation or installation result; use the [observed library path](../how-to/prepare-host-source-artifact.md).

## Observed result boundary

The direct production rlib build exited 0; seven selected positive checks passed. Earlier Cargo check/build timeouts remain recorded as timeouts. Actual `compile_host(..., false)` captures for FixedQuote/FixedEligibility match all twelve selected ProgramView fields and selected envelope/identity/layout/source/config/target predicates. Eligibility uses only the sealed hygienic-name bijection. The reports retain `comparison_complete=false`: admission bounds, full preparation values, some wrapper/evidence values and all runtime outcomes remain unselected. The advisory ordinary review supports that limited publication, not source/fix approval. The [result package](../../results/sherlock-source-compile-20261017/) retains precise inputs, receipts, provenance limits and unselected fields.
