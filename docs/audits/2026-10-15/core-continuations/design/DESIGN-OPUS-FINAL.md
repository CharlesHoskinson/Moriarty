# Opus 5.5 final security amendment A1: U3 continuations (baseline `fe01bc8b…`)

**Status.** This is a design amendment only. Nothing was executed, read or hashed. It amends DESIGN-OPUS.md (C1–C10, P1–P5, H1–H23) and DESIGN-OPUS-CLARIFICATION.md. Where they conflict, this document controls. Any implementation-plan text that contradicts it is void (MF2). It closes no exit, and allnative0 and fullparentexits are preserved.

## A0. Corrections

1. **Relocation.** Clarification §3.2's ledger `relation/stage_{context,frame,artifact}.rs` and §3.3's `relation/stage_lower.rs` move to `crates/mori-backend/src/stage/{context,frame,artifact,lower}.rs`, plus `stage/mod.rs`. mori-ledger receives no stage emitter, lowerer or type. It receives only the refusal hunks (A9) and the `source_closure` hunk (A2).
2. **`/4` compiler identity withdrawn.** Clarification §3.4's last bullet ("`/4` artifacts record the same closure") is deleted.
   - The backend `/3` `Artifact` has no closure field, and nothing proves `/4` (native = 0).
   - Importing ledger's `pub(crate)` ledger-relative `include_str!` list into the backend would need an unverified cross-crate refactor.
   - `/4` identity is therefore exactly: format/abi/relation strings, `program_hash`, `plan_hash` and the per-circuit context hashes.
   - The closure coverage requirement (A2) no longer covers backend or CLI files.
3. **MF3 frame representation is frozen.** `Continuation.{site,captures}` *is* the local frame of the executing action. `/4` pending roles are its zero-padded projection. At start, `pre_site == 0` is locally assumed, not checked (P3). Preservation of the frames of other actions is a ledger premise under gate `pending-frame-predecessor-binding`.

## A1. Compatibility whitelist by actual stack

| Stack | Whitelist | Rule |
|---|---|---|
| `/1` runtime: Execution JSON, logical-cost/1, error code/message/offset/step, `/1` IR decode errors (H23) | **empty** | byte-identical |
| backend `compile`/`compile_mode` → `/3` `artifact.json` + every `.zkir` | **empty** | byte-identical; no snapshot or golden may change |
| ledger native-call public manifests/records containing `proving::source_closure()` or a value derived from it | Step A list | Recompute per clarification §1.2 Step C; masking is forbidden |
| CLI usage errors for existing `/1` invocations | **empty** | byte-identical |

Basis for the empty backend list, from the supplied source:
- The `Artifact`, `CircuitArtifact` and `Resources` fields carry no closure.
- None of the 18 `context_hash` elements is the closure.
- `json(program)` stays byte-stable because adding an enum variant does not alter how existing variants serialize.

Step A must still confirm by reading. See F1.

## A2. `source_closure` hunk (`crates/mori-ledger/src/proving.rs`)

- Add a comma after the current last entry.
- Then append exactly these lines, in this order:

```rust
include_str!("../../mori-core/src/stage/admit.rs"),
include_str!("../../mori-core/src/stage/cost.rs"),
include_str!("../../mori-core/src/stage/decode.rs"),
include_str!("../../mori-core/src/stage/digest.rs"),
include_str!("../../mori-core/src/stage/mod.rs"),
include_str!("../../mori-core/src/stage/plan.rs"),
include_str!("../../mori-core/src/stage/profile.rs"),
include_str!("../../mori-core/src/stage/validate.rs")
```

- No other new core or lexer files are permitted. `run_segment` lives in the existing `runtime.rs`, and `ir::v1` is an inline module in `ir.rs`. Sol creates no `stage_runtime.rs`.
- **Coverage test** (`mori-ledger/tests/source_closure_coverage.rs`). It walks `mori-core/src`, `mori-lexer/src` and `mori-ledger/src` recursively. Every `.rs` file must be listed or appear in the reasoned exclusion list, and `stage/*` files can never be excluded.

## A3. Core types (mori-core, final)

```rust
// stage/plan.rs (Sol builds; Opus validates). Serialize only: no Deserialize for StagePlan/ActionPlan/AwaitSite.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Origin { Input, Let, Event }
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)] #[serde(deny_unknown_fields)]
pub struct CaptureSlot { pub name: String, pub ty: Type, pub origin: Origin,
    pub ordinal: u16 } // Input: declaration index; Let: the let's statement ordinal; Event: the defining await's statement ordinal
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)] #[serde(deny_unknown_fields)]
pub struct StageBinder { pub name: String, pub ty: Type }
pub struct AwaitSite { pub site: u16 /*1..=N*/, pub ordinal: u16, pub label: String,
    pub binders: Vec<StageBinder>, pub captures: Vec<CaptureSlot> }
pub struct ActionPlan { pub action: String, pub statement_count: u16, pub sites: Vec<AwaitSite>,
    pub segment_cost: Vec<UInt> /*N+1*/, pub committed_before: Vec<UInt> /*N+1, [0]=0*/, pub plan_total: UInt }
pub struct StagePlan { pub format: String /*"moriarty-stage-plan/1"*/, pub program_hash: String /*64 lc hex*/,
    pub actions: Vec<ActionPlan> /*every program action, declaration order, incl. zero-await*/ }

// stage/mod.rs (Opus). Sealed: no public constructor, no Deserialize, no Default.
pub struct StagedProgram { program: Program, plan: StagePlan, program_hash: [u8; 32], plan_hash: [u8; 32] }
impl StagedProgram {
    pub fn program(&self) -> &Program;  pub fn plan(&self) -> &StagePlan;
    pub fn program_hash(&self) -> [u8; 32];  pub fn plan_hash(&self) -> [u8; 32];
}
pub fn load(src: &str) -> Result<StagedProgram, Error>;
```

**`load` sequence.**
1. Sol `parser::parse_v2(src)`, which is syntax-only. It has `pub(crate)` visibility and is called only by `load`.
2. `profile::require_core2`
3. `validate::validate_v2`
4. Sol `plan::build`, which leaves the cost vectors empty.
5. `validate::plan_invariants`. Sites are numbered 1..N. Ordinals are strictly increasing. Captures are prefix-monotone (`captures(k)` is a prefix of `captures(k+1)`). Captures exclude state names and include all inputs first. Widths are ≤ 256.
6. `cost::fill` uses checked arithmetic; overflow → `bound`. `plan_total = committed_before[N] + segment_cost[N]`.
7. `digest::{program_hash, plan_hash}` use `intent::canonical`.

**Parser rule (MF6).**
- `parser::parse` and legacy `validate` are unchanged and remain `/1`-only. A `moriarty/2` source therefore fails every legacy consumer with the existing profile error. This makes refusal structural.
- Sol adds `parser::language(src) -> Result<&'static str, Error>`, which returns `LANGUAGE_1` or `LANGUAGE_2`. The CLI uses it only to route. A `/1` file then follows the exact baseline path.

**Profile helpers (Opus).** `pub fn require_core1(&Program) -> Result<(), Error>` and `require_core2`. Both use the existing profile error identity.

**Runtime (Sol, `runtime.rs`).** The single evaluator is seeded as follows. It never decides admission.

```rust
pub enum SegmentEntry<'a> { Start { inputs: &'a [NamedValue] },
    Resume { site: u16, captures: &'a [NamedValue], event: &'a [NamedValue] } }
pub enum SegmentRun { NotAdvanced,
    Ran { execution: Execution, exit_site: u16, captures_out: Vec<NamedValue> } }
pub(crate) fn run_segment(sp: &StagedProgram, action: usize, entry: SegmentEntry<'_>,
    boundary: Snapshot, meter: &mut Meter) -> Result<SegmentRun, Error>;
```

**Admission (Opus, `stage/admit.rs`).** This is the only caller of `run_segment`.

```rust
pub const STAGE_LABELS: [(&str, &str); 3] = [("qualification", "local-reference-stage"),
    ("authentication", "none"), ("consumption", "untracked")];
pub fn start(sp: &StagedProgram, action: &str, scenario: Scenario, cap: Option<UInt>)
    -> Result<StageOutcome, StageRejection>;
pub fn resume(sp: &StagedProgram, c: Continuation, e: StageEvent, cli_action: Option<&str>,
    cli_cap: Option<UInt>) -> Result<StageOutcome, StageRejection>;
```

- **Start** validates the scenario with the existing `scenario_snapshot`; legacy sorting is acceptable for a fresh scenario. It then applies the plan-total cap preflight.
- **Resume** applies R1–R7 in order, and adds R0: a `cli_action` that is present must equal `c.action`, else `stage_site`.
- **Meter.** Admit creates the meter with limit `segment_cost[j]`. After a `Ran`, it requires `meter.used == segment_cost[j]`. Otherwise it rejects with `stage_logical` and the message "static cost premise violated" (H26).
- **Output round-trip (H25).** Before returning `Waiting`, admit serializes the new continuation with `serde_json::to_string`. It requires length ≤ `MAX_CONTINUATION_BYTES` and `decode::continuation(text)? == c`. Otherwise it rejects with `bound`. No continuation may be emitted that cannot be resumed.
- `StageOutcome`, `StageRejection`, `StageSegment`, `PendingFrame`, `Continuation`, `StageEvent`, `StageLogical` and `NamedValue` are exactly as frozen previously. Sol renders output and must insert `STAGE_LABELS` verbatim.

**Zero-await actions and cap (MF5).** Clarification §2.4 and §2.5 apply as written. A plan whose `committed_before.len() != segment_cost.len()` is rejected by `plan_invariants`.

## A4. Bounded strict parse (MF7, no `/1` limit raise)

**`json.rs` hunk.**
- The existing `parse` body moves into `fn parse_inner(text, max: usize, too_large: &'static str)` unchanged. The depth (32) and node (65,536) limits are unchanged.
- The public functions become:

```rust
pub fn parse(text: &str) -> Result<Value, Error> { parse_inner(text, MAX_BYTES, "JSON exceeds 1 MiB") }
pub fn parse_limited(text: &str, max: usize, too_large: &'static str) -> Result<Value, Error> { parse_inner(text, max, too_large) }
```

**`stage/decode.rs`.**

```rust
pub const MAX_CONTINUATION_BYTES: usize = 4 * 1_048_576;
pub const MAX_EVENT_BYTES: usize = 65_536;
pub fn read_bounded(path: &Path, max: usize, too_large: &'static str) -> Result<String, Error>;
pub fn continuation(text: &str) -> Result<Continuation, Error>;
pub fn event(text: &str) -> Result<StageEvent, Error>;
pub fn canonical_boundary(sp: &StagedProgram, action: usize, captures: &[NamedValue], b: &Snapshot) -> Result<(), Error>;
```

- **`read_bounded`** reads through `Read::take(max + 1)`, so there is no metadata TOCTOU. If more than `max` bytes are read it fails with `json` and `too_large`. Non-UTF-8 → `json`.
- **`continuation` and `event`** run `parse_limited`, then `serde_json::from_value` with `deny_unknown_fields`.
- **Node limit.** The 65,536-node limit is retained at 4 MiB. Rejection fails closed, and H25 guarantees that emitted continuations stay within it.
- **Clap usage conflicts** (MF9 matrix) fire before any `read_bounded`.

**`canonical_boundary` (R7).** This is a literal replacement of `scenario_snapshot` for resume. It never re-sorts.
- **(a) States.** The key set must equal the program's states exactly, each value of its declared type. If the representation is a `Vec`, it must be strictly ascending by name.
- **(b) Balances.** Strictly ascending by (asset, account), which also rejects duplicates. The key set must equal exactly the whole-action key set that `scenario_snapshot` derives. Failure → `balance`, with message "Balances are not canonical" for order.
- **(c) Liabilities.** Strictly ascending by name. Each name must be in the whole-program registry, and its identity must equal the registry entry, else `liability` "Existing liability identity differs". Order failure → `liability` "Liabilities are not canonical".
- **(d) Inputs.** If the legacy Snapshot carries an inputs component, it must equal the `Origin::Input` prefix of `captures`. Otherwise this check is vacuous (U-1).
- **(e) Trace cells.** The trace-cell formula uses the whole action's `statement_count` and its added-liability count, with the same code and message as `scenario_snapshot`.

## A5. Segment shape and charging (MF8 pin)

- **Reaching an await.** Charges its single statement charge and stops before the guard. The start or waiting Execution excludes the exit Await step.
- **Resume.** Dispatch is that same single charge. The guard is evaluated with a fresh per-statement cache.
  - If the guard is true, the first Step is `{index: ordinal, statement: Await, value: Bool(true), snapshot: pre, effects: []}`, with no second charge.
  - If the guard is false → `NotAdvanced`. The residual comes from the unchanged boundary.
- **`Execution.semantics`** is `"moriarty-core/2"`.
- **Rejected suffix.** attempted is the partial meter count. H11 = 4 and H12 = 6 stand.

## A6. `/4` types (`mori-backend/src/stage/artifact.rs`, Opus)

These are separate fields, not a nested `CircuitArtifact`. No `/3` type changes. Leaf types `PublicBinding`, `EffectDescriptor`, `BalanceDescriptor`, `LiabilityDescriptor`, `Gate` and `Target` are reused unmodified.

```rust
pub const STAGED_ARTIFACT_FORMAT: &str = "moriarty-zkir-artifact/4";
pub const STAGED_ABI_VERSION: &str = "moriarty-public-abi/4";
pub const STAGED_RELATION_VERSION: &str = "moriarty-zkir-relation/4";
pub const STAGE_CONTEXT_FORMAT: &str = "moriarty-stage-context/1";
pub const LOGICAL_ENFORCEMENT: &str = "not-circuit-enforced-not-dust";
pub const MAX_STAGED_CIRCUITS: usize = 999;
pub const STAGE_GATES: [&str; 4] = ["pending-frame-predecessor-binding", "stage-event-authentication",
    "stage-authority-and-consumption", "arbitrary-witness-stage-soundness"];

#[derive(Debug, Clone, Serialize, Deserialize)] #[serde(deny_unknown_fields)]
pub struct PendingPositions { pub pre_site: u32, pub pre_captures: Vec<u32> /*L*/,
    pub events: Vec<u32>, pub post_site: u32, pub post_captures: Vec<u32> /*L*/ }
#[derive(Debug, Clone, Serialize, Deserialize)] #[serde(deny_unknown_fields)]
pub struct StagedCircuit {
    pub action: String, pub segment: u16, pub entry_site: u16, pub exit_site: u16,
    pub zkir: String, pub context_hash: String,
    pub capture_width: u16, pub capture_in: Vec<CaptureSlot>, pub capture_out: Vec<CaptureSlot>,
    pub event: Vec<StageBinder>, pub public_inputs: Vec<PublicBinding>,
    pub pending_positions: PendingPositions, pub instruction_count: u32,
    pub effect_frame: Vec<EffectDescriptor>, pub balance_frame: Vec<BalanceDescriptor>,
    pub liability_frame: Vec<LiabilityDescriptor>, pub replay_steps: u32 }
#[derive(Debug, Clone, Serialize, Deserialize)] #[serde(deny_unknown_fields)]
pub struct SegmentLogical { pub segment: u16, pub segment_logical: UInt, pub committed_before: UInt }
#[derive(Debug, Clone, Serialize, Deserialize)] #[serde(deny_unknown_fields)]
pub struct ActionLogical { pub action: String, pub plan_total: UInt, pub segments: Vec<SegmentLogical> }
#[derive(Debug, Clone, Serialize, Deserialize)] #[serde(deny_unknown_fields)]
pub struct StagedResources { pub instructions: u32, pub zkir_bytes: u32, pub artifact_bytes: u32,
    pub public_values: u32, pub max_replay_steps: u32,
    #[serde(deserialize_with = "decode_registry_entries")] pub liability_registry_entries: u32,
    #[serde(deserialize_with = "decode_registry_ceiling")] pub max_liability_registry_entries: u32,
    pub logical_profile: String, pub logical_enforcement: String, pub logical: Vec<ActionLogical> }
#[derive(Debug, Clone, Serialize, Deserialize)] #[serde(try_from = "StagedArtifactFields")]
pub struct StagedArtifact { pub format: String, pub abi: String, pub relation: String,
    pub authorization_mode: String, pub program_hash: String, pub plan_hash: String,
    pub semantics: String, pub qualification: String, pub authentication: String,
    pub consumption: String, pub target: Target, pub circuits: Vec<StagedCircuit>,
    pub resources: StagedResources, pub gates: Vec<Gate> }
// StagedArtifactFields: same fields, #[serde(deny_unknown_fields)], private. No From/Into with Artifact.
```

**Field rules.**
- **Removed fields.** `circuit_k`, `fraud_max_k` and `cost_measured` are omitted because nothing is measured.
- **Gates.** The six `/3` predicates come first, in `/3` order, then `STAGE_GATES`. **All ten have status `"unperformed"`.** Under-claiming is deliberate.
- **Totals.** `instructions`, `zkir_bytes` and `public_values` are sums over all circuits. `max_replay_steps` is the max. The existing `/3` instruction, byte and public-value caps apply to the sums. More than `MAX_STAGED_CIRCUITS` circuits → the existing resource-cap error identity.
- **`artifact_bytes`.** Use the identical procedure `/3` uses for its envelope fixed point, applied to the `/4` serialization. If `/3` has no bounded procedure, iterate from 0 at most 4 times and refuse if the value is not stable.
- **Filenames.** The global circuit index orders actions by declaration, then segments ascending. Filenames are `{index:03}-{action}.s{segment}.zkir`. The manifest is published last via `publish_artifact`.

**Decoder (`TryFrom<StagedArtifactFields>`).**
1. Exact `/4` strings, `authorization_mode == "unsigned" == target.authorization_mode()`, `semantics == SEMANTICS_2`, the three labels equal `STAGE_LABELS`, `logical_profile == LOGICAL_COST_PROFILE_2`, and `logical_enforcement == LOGICAL_ENFORCEMENT`.
2. `program_hash`, `plan_hash` and every `context_hash` pass `decode_digest`.
3. Per action, segments are exactly 0..=N and contiguous. `entry_site = segment == 0 ? 0 : segment`. `exit_site = segment == N ? 0 : segment + 1`.
4. `pending_positions` are in range and pairwise distinct. They are disjoint from every other declared position. Their roles, names, types and constants match A7. Their lengths match L and the binder count. Segment 0 has no event entries.
5. `capture_in.len() == len_entry` and `capture_out.len() == len_exit`, both ≤ L. Every slot equals the width-L layout prefix.
6. Advance segments contain no `input` role.
7. The gate list equals the constant list exactly.
8. `logical` agrees with the circuits per action and segment.

`/3` `decode_version` rejects `/4` strings, and the `/4` decoder rejects `/3` strings (H20).

## A7. Frame API and role rules (`stage/frame.rs` Opus, `stage/lower.rs` Sol)

```rust
pub(crate) struct SegmentLayout { pub action: usize, pub segment: u16, pub entry_site: u16, pub exit_site: u16,
    pub statements: Range<usize>, pub guard: Option<ExprId>, pub slots: Vec<CaptureSlot> /*width L*/,
    pub len_in: u16, pub len_out: u16, pub event: Vec<StageBinder>, pub replay_steps: u32 }
pub(crate) fn layouts(sp: &StagedProgram) -> Result<Vec<SegmentLayout>, Error>; // global index order
pub(crate) struct PreFrame { pub captures: Vec<(String, Wire)> /*len_in*/, pub events: Vec<(String, Wire)> }
pub(crate) fn emit_pre(b: &mut Builder, bindings: &mut Vec<PublicBinding>, l: &SegmentLayout,
    pos: &mut PendingPositions) -> Result<PreFrame, Error>;
pub(crate) fn assert_guard(b: &mut Builder, guard: Wire) -> Result<(), Error>;
pub(crate) fn emit_post(b: &mut Builder, bindings: &mut Vec<PublicBinding>, l: &SegmentLayout,
    live: &[(&str, Wire)], pos: &mut PendingPositions) -> Result<(), Error>;
```

`Wire` means the existing Builder wire handle type.

**Layout.**
- L is the length of `captures(N)`, or 0 when N = 0, and `slots = captures(N)`. Prefix-monotonicity fixes each slot's name and type across sites.
- **Padding.** A slot i ≥ `len` is typed `slots[i].ty`, valued 0 and has constant `Some("0")`. A Bool padding slot is a typed Bool equal to false.
- **`statements`.**
  - Segment 0 covers `0..ord(1)`, or `0..statement_count` when N = 0.
  - Advance k covers `ord(k)+1..ord(k+1)`, or to the end. Its guard is that of site k.
- **`replay_steps`** = (exit 0 ? `statement_count` : `ord(exit)`) − (entry 0 ? 0 : `ord(entry)`). This counts actual emitted steps, including the true entry Await and excluding the exit Await. Test: equals the local `Execution.steps.len()`.

**Roles.**

| Position | Role | Name | Type | `constant` | Constraint |
|---|---|---|---|---|---|
| pre-pending block, inserted where `lower_action` finishes the pre-liability block | `pre_pending_site` | action | UInt | entry decimal | asserted equal to entry constant |
| | `pre_pending_capture` × L | `slots[i].name` | `slots[i].ty` | `None` live / `Some("0")` padding | range or boolean check; padding asserted 0 |
| | `event` × B | binder name | binder type | `None` | range or boolean check |
| post-pending block, inserted immediately after the last post-liability position | `post_pending_site` | action | UInt | exit decimal | `expected` = constant wire |
| | `post_pending_capture` × L | `slots[i].name` | `slots[i].ty` | `None` live / `Some("0")` padding | live: `expected = Some(live wire)`; padding: `expected` = constant-0 wire |

- **Output membership.** Pending roles are ordinary `PublicBinding`s emitted through `Builder::public`, so they are transcript entries exactly like the existing post roles. No additional Output entries are added.
- **`emit_post` checks.** `live` must equal `slots[0..len_out]` by name and order, and each wire's type must match. Otherwise it fails with `artifact`.
- **Bool checks.** `frame.rs` emits `ConstrainToBoolean` explicitly for Bool slots and events if `Builder::public` does not.
- **Lowerer restrictions.**
  - The lowerer never calls `Builder::public` for these five roles (H27, introspection).
  - The advance environment binds captures and binders from `PreFrame`, and states from `pre_state`.
  - The start `live` list comes from the input and let wires.
  - Balance and liability frames are whole-action and use the full registry.
  - `effect_frame` holds only this segment's effects, with original ordinals.
  - The context constants 0/1 are derived from the `/4` context hash exactly as `/3` derives them from its own.
- **Entry point.** `pub fn compile_staged(src: &str, target: &Target, out: &Path) -> Result<StagedArtifact, Error>` lives in `stage/lower.rs`. It calls `mori_core::stage::load`. A signed Target → `unsupported` before `create_dir`.
- **`/3` refactor.** `lower_action` may become range-parameterised only if `/3` calls it with the full range and its bytes stay identical (A1).

## A8. `/4` context preimage (`stage/context.rs`, Opus)

```rust
pub(crate) fn stage_context_hash(sp: &StagedProgram, target: &Target, registry: &[LiabilityIdentity],
    l: &SegmentLayout, bindings: &[PublicBinding], effect_frame: &[EffectDescriptor],
    balance_frame: &[BalanceDescriptor], liability_frame: &[LiabilityDescriptor]) -> Result<[u8; 32], Error>;
```

It uses the same crate `digest(&[Value])` and `json` helper as `/3`. The elements are ordered:

1. `STAGE_CONTEXT_FORMAT`
2. `"unsigned"`
3. `STAGED_ARTIFACT_FORMAT`
4. `STAGED_ABI_VERSION`
5. `STAGED_RELATION_VERSION`
6. `SEMANTICS_2`
7. `LOGICAL_COST_PROFILE_2`
8. `MAX_LIABILITY_REGISTRY_ENTRIES`
9. `MAX_AWAITS_PER_ACTION`
10. `MAX_EVENT_BINDERS`
11. `MAX_CAPTURE_SLOTS`
12. `registry`
13. `program`
14. `plan`
15. `target`
16. action name
17. `segment`
18. `entry_site`
19. `exit_site`
20. `L`
21. `slots`
22. `len_in`
23. `len_out`
24. `event`
25. projection: `/3` `Projection {position, role, name, ty, asset}`, excluding constants, as in `/3`
26. `effect_frame`
27. `balance_frame`
28. `liability_frame`
29. `replay_steps`

The `/3` `context_hash` is untouched. Test: `STAGE_CONTEXT_FORMAT != CONTEXT_FORMAT`, plus H21.

## A9. Refusals and forced match arms (integrator checklist)

- **Await arms.** Every exhaustive match that gains a `Statement::Await` arm returns a typed profile refusal with that crate's error identity. `unreachable!`, panic and `_ =>` fallthrough are forbidden. The arms are in:
  - backend `proving.rs` and the `signed_relation.rs` label scan;
  - ledger `ops.rs`, `relation/compiler.rs` (both matches) and `signed/mod.rs`;
  - CLI `inspect.rs` (`statement_rule`, `expression_constraints`).
- **`layout.rs`.** `admit_bindings_inner` gains an explicit Await refusal arm, and its callers call `require_core1` first.
- **Explicit `require_core1` at entry.** This is defense in depth over the structural parse refusal. It applies to:
  - ledger `compile_source_static_roles`, `static_compile::compile`, `run_static`/`CompileCall`, signed `admit` and preparation;
  - backend `compile`, `compile_signed` and `replay::generate`.
  - Tests pin the exact error identity, including backend's `mori::backend::program` wrapping.

## A10. Added hostile tests (Opus-authored)

- **H24.** `parse` still reports "JSON exceeds 1 MiB" at 1 MiB + 1. A continuation at 4 MiB + 1 and an event at 65,537 bytes are rejected before parse.
- **H25.** Waiting continuations round-trip.
- **H26.** Meter used equals `segment_cost`.
- **H27.** Pending roles are emitted only via `frame.rs`.
- **H28.** Closure coverage.

## A11. Unresolved conditions (stated explicitly, not evidenced)

No fatal condition is known. The following are **conditionally fatal**. If one is found, implementation stops and reports it; the contract is not reinterpreted.

- **F1.** Step A finds any backend `/3` byte that depends on the closure or on a changed constant. The empty whitelist then fails.
- **F2.** H17 or H26 fails. C3 is then void.
- **F3.** The Builder cannot produce a constant-valued `expected` wire, or the insertion points in A7 do not exist in `lower_action`'s order.
- **F4.** `json(program)` or `program_hash` changes for any non-Await program.

**Inspection premises to report:**
- **U-1.** The Snapshot inputs component.
- **U-2.** The `/3` `artifact_bytes` procedure.
- **U-3.** The `/3` gate predicate list.
- **U-4.** The `Wire` and Builder constant API names.

## A12. Sign-off

I sign off, as sole security owner, on the frozen security contract. It consists of:
- DESIGN-OPUS.md;
- DESIGN-OPUS-CLARIFICATION.md as amended by A0;
- this amendment A1;
- Sol's revised vote, read subject to the above.

The sign-off is conditional solely on two things:
- Sol adopts A0–A11 verbatim;
- a fresh Muse nonsecurity design approval of the amended packet.

No further Opus design round is required unless bytes or constraints change.

This is not milestone acceptance. These remain mandatory:
- the full-candidate and actual-results Muse audits;
- my code and test review;
- my pre-execution closure classification, which also covers the Step B baseline fixture build.

No execution is approved by this document.
