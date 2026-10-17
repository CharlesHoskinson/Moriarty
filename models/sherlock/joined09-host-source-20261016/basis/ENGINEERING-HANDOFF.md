# Engineering handoff: B-core finite helper machine, revision R2

**Author:** Opus 5.5 (`claude-opus-5-5`, high effort).

**Status:** proposed contract, specified only. It takes effect only if the consequential choices in RESOLUTION.md §8 receive two substantive agreeing votes on these bytes. Votes on the earlier review-candidate bytes do not transfer. Nothing here grants execution, native, schema-freeze or acceptance authority. Native authority is 0 and C4 is unchanged.

**Targeted correction R2-T1.** This revision changes only the admission binding (S1), admission classification (S2) and the binding implementation instructions in §2, §4, §5 and §8. No R2 vote covers these bytes. Stage 2 stays blocked until a fresh Opus review of actual implementation bytes; this text grants no native or Stage 2 permission.

**Normative inputs:**
- [`schema-supplement.json`](schema-supplement.json): types, row language, dispatch, state space, StepInv, admission and API types.
- [`rows-supplement.json`](rows-supplement.json): every row.
- [`catalog-revision-overlay.json`](catalog-revision-overlay.json): the schedule revision B-CORE-R1.

Where this prose and the JSON disagree, the JSON governs. Report every disagreement as a defect; do not resolve it silently.

## 1. What to build, and what it is not

Build a deterministic host-side state machine for profile `sherlock-helper-microstep/1` with `grammarDomain = DedicatedSorts`:

- capacities come from runtime `Bounds` and `LayoutCapacities` parameters;
- a run goes from genesis to a published Outcome in one uncut segment;
- each charged event executes one of the **96 Enabled step rows** (37 legacy, 58 helper, 1 bookkeeping);
- the 5 administrative IDs are realized by genesis (Startup) or by precedence P1/P2 (Publish). They have no plan function.

This build is a **pure finite helper core**. It is not Candidate B, not generic ADT support, not finance support, not chunking, not a circuit and not a product deliverable. The user's full Sherlock goal and every obligation in §9 remain open after it.

## 2. Consumer boundary (answers Muse F-11(d))

The earlier bytes registered the module as `#[cfg(test)]` and also called it a "callable API". Both statements cannot hold for a real consumer, so this revision separates three stages.

| Stage | Registration in `crates/mori-core/src/sherlock/mod.rs` (I applies) | Consumer | Limitation |
|---|---|---|---|
| **1. Source preparation** | `#[cfg(test)] mod helper_machine;` | Unit tests in `helper_machine/*_tests.rs` only | **Not callable outside `cargo test`.** No example, library user, CLI, Fenrir, circuit or comparator can call it. Stage 1 alone does not deliver a real callable B. |
| **2. Host library consumer** | `pub mod helper_machine;` (no `cfg(test)`), plus a re-export block in `sherlock/mod.rs` of exactly: `CoreProgram`, `Bounds`, `LayoutSchema`, `LayoutCapacities`, `RunInt`, `FinanceInput`, `Admitted`, `MachineImage`, `StepEvent`, `StepResult`, `RunSummary`, `Tooling`, `HostViolation`, `StepInvViolation`, `prepare_core`, `check_program_view`, `admit`, `admit_run`, `step`, `run`, `check_step_inv` | (a) a new I-owned example `crates/mori-core/examples/sherlock_helper_machine.rs`, mirroring the existing public consumer `examples/sherlock_reference.rs` (reference README.md:71-76); (b) T's B comparator in `step_record.rs`/`compare.rs` (plan Phase 1) | Trusted, typed, in-memory callers only. No file, CLI, network or deserialization entry point. |
| **3. Untrusted or CLI input** | none in this handoff | e.g. `mori simulate` | Blocked under H1 (MCL design.md:364-368; composition delta: H1's general untrusted-input restriction remains in force). Needs a separate Opus decision. |

**My security conditions for Stage 2.** All must hold in the implementation bytes and be checked by a fresh Opus review of those bytes:

- **C1** No entry point accepts bytes, files, a CLI or the network. Programs arrive only as `CoreProgram` values. (R2-T1: the `From<&reference_types::Program>` conversion is not a Stage 2 entry point. Reference programs are `Arc` DAGs, so unfolding one can allocate exponentially before any capacity check. It stays in Stage 1 `cfg(test)` code on test-authored inputs until a bounded, capacity-refusing form is specified and reviewed; schema `core_program_input.from_reference_types_stage_status`.)
- **C2** No panics on any input reachable through the public API. Every index is checked, all arithmetic is checked, and there is no `unwrap` on caller-influenced data. Failures are `Tooling` or `HostViolation`.
- **C3** Memory grows only with active lengths. Tables are filled lazily up to their capacities and are never preallocated to `Bounds` values.
- **C4** `run` follows the stop rule in schema `api_types.RunSummary.stop_rule`: binding check first; no `step` call on a non-Running image; after each `Event`, call the sink once and stop if the outcome is no longer Running. It therefore makes at most `pre.remainingWork + 1` step calls (`initialFuel + 1` from genesis).
- **C5** No `unsafe`, no global or static mutable state, no I/O, no time and no randomness.
- **C6** `Admitted` and `MachineImage` have private fields. `Admitted` can only be built by `admit`, and a `MachineImage` only by `admit_run` or `step`. No caller-built `ProgramView` or image reaches `step`.
- **C7** `check_step_inv` is documented as a diagnostic.
- **C8** (R2-T1, extended by R3-T1 D-36 for Opus1 R3-2 and Opus2 R3-L1, R3-L2.) No native recursion whose depth grows with caller-controlled nesting or value depth, per schema `step_inv.traversal_rule`. This covers `prepare_core`, `check_program_view`, `check_step_inv`, any conversion into `CoreProgram`, and any host decoding or materialization out of an image (`decodeStep`, `materializeStep`, comparator or example output), which must either preserve sharing (memoized per row ID) or refuse incrementally at a declared capacity. The recursive `CoreProgram`, `CoreExpr` and `CoreTree` carriers must be uniquely owned trees with no shared subterms (`Arc`/`Rc` DAG sharing is forbidden), so `pass_B_names` work is linear in the caller's allocation before C1's `codeRows` check. Compiler-generated `Clone`, `PartialEq`, `Debug` and `Hash` on recursive types must be written iteratively or not implemented; because Drop glue cannot be opted out of for an owning recursive heap carrier, `Drop` MUST be an explicit iterative impl unless children are flat arena indices. Value traversals in diagnostics are memoized, so a shared DAG costs work polynomial in active lengths. Use explicit heap worklists; do not add a depth refusal.
- **C9** (R2-T1; Opus2 S1.) `MachineImage` holds a private handle to the exact `Admitted` that `admit_run` consumed. `step`, `select`, `run` and `check_step_inv` check that binding before anything else (schema precedence AB) and fail closed with `HostViolation{StepInvViolated, SI-01}` and no mutation. `Admitted` is a cheap handle; cloning it shares the admission and never deep-copies.

Stage 2 is the smallest change that gives a real host consumer. It does not lift H1 and adds no developer permission requirement. Moriarty stays permissionless. The B-core library is a host evaluator for trusted programs, not public compiler integration.

**Write set (implementer):** `crates/mori-core/src/sherlock/helper_machine/` only, with files `mod.rs`, `types.rs`, `prepare.rs`, `admit.rs`, `dispatch.rs`, `legacy_rows.rs`, `helper_rows.rs`, `step.rs`, `step_inv.rs` and separate `*_tests.rs` files.

**Ownership versus the plan's file map (answers Opus2 L-8; nonsecurity, I decides):** the plan assigns `program_image.rs`/`admission.rs` to P and `step_record.rs` to T. Those paths currently hold A-profile proposals (`mod.rs:39-41`). I recommend that I record `helper_machine/{prepare,admit}.rs` as P-owned and `helper_machine/{dispatch,legacy_rows,helper_rows,step,step_inv}.rs` as owned by one named implementer. T keeps the B comparator in its own files. This is an integrator decision (OPEN-I-2).

**Untouched:** `reference_types.rs` (read only), `helper_schedule.rs` and `helper_call_adapter.rs`. The last two remain unadopted historical proposals; do not reuse their phases as normative.

**Oracles:** O authors expected states, events and work prefixes independently, from rows-supplement and the overlay, after I-B-EVENT-ADOPTION and before any comparison (`SHERLOCK-IMPLEMENTATION-PLAN.md:87-89`).

**Hostile cases:** S (Opus) authors them; HB-01..HB-47 are in RESOLUTION.md §11, and the S1/S2 regression contracts are in `REPRODUCTION-CONTRACTS.json` (unexecuted). Building the mutation harness is nonsecurity work, but pass/fail judgment stays with S.

**Every-phase gate (Opus1 F8):** no B-specific claim without the applicable Opus-authored B001–B009 and HB cases as its gate (plan:111). Normal runs alone never establish a B-core claim.

## 3. Data model (Rust sketch; names bind, representation does not)

```rust
// types.rs: indices are usize (no hidden u32 narrowing, Muse F-11(c)); work is u128.
pub struct Bounds {            // all 22 interfaces.md fields (Muse F-11(b))
    pub code_rows: usize, pub function_rows: usize, pub code_child_slots: usize,
    pub parameter_slots: usize, pub pattern_rows: usize, pub match_plan_rows: usize,
    pub match_arm_rows: usize, pub type_rows: usize, pub constructor_rows: usize,
    pub constructor_field_slots: usize, pub max_arity: usize, pub constant_nodes: usize,
    pub constant_field_slots: usize, pub data_nodes: usize, pub data_field_slots: usize,
    pub environment_cells: usize, pub frames: usize, pub helper_tasks: usize,
    pub serialization_bytes: usize, pub max_reference_work: u128,
    pub max_helper_work: u128, pub step_cap: u128,
}
pub struct LayoutCapacities { pub name_rows: usize, pub name_bytes: usize,
    pub occurrence_path_slots: usize, pub value_slots: usize }          // D-12, D-08R
pub struct LayoutSchema { pub layout_version: Id, pub field_id: Option<Id>,
    pub widths: Option<WidthTable>, pub scalar_layout_id: Option<Id>,
    pub bounds: Bounds, pub capacities: LayoutCapacities,
    pub boundary_capacities: Option<BoundaryCapacities> }              // must be None (AD-6)
pub struct Id { pub namespace: String, pub name: String }
pub enum Digest { Unselected, Selected { format: Id, bytes: Vec<u8> } } // never fabricate Selected
pub struct Identity { pub profile: ProfileId, pub grammar_domain: GrammarDomain,
    pub finance: FinanceInput, pub layout_version: Id, pub layout_digest: Digest,
    pub bounds_digest: Digest, pub field_id: Option<Id>, pub program_membership_digest: Digest }
pub struct RunId { pub identity: Identity, pub fuel_input: RunInt, pub stack_limit_input: RunInt }
pub struct ExpandedStepId { pub run_id: Arc<RunId>, pub ordinal: u128 }
pub enum RunInt { Negative, Nat(u128), NatAboveU128 }                  // Opus1 F10
pub enum KInt { InRange(u128), Negative, AboveU128 }                   // D-13 literals

// CoreProgram mirrors schema core_program_input (uniquely owned tree carrier, no shared subterms); #[cfg(test)] impl From<&reference_types::Program>.
pub enum CoreValue { UInt(KInt), Bool(bool), List(Vec<KInt>), Tree(CoreTree) }

#[derive(Clone, Copy, PartialEq, Eq)] pub enum ValueRef { Const(usize), Data(usize) }
#[derive(Clone, Copy, PartialEq, Eq)] pub struct ValSlice { pub start: usize, pub len: usize } // (0,0) empty
pub type EnvRoot = Option<usize>;
pub enum LogicalKind { UInt, Bool, Nil, Cons, Leaf, Branch, Marker }   // D-23: all tests use kind()

pub enum ConstRow { U128(u128), Bool(bool), Nil, Cons{fields: usize}, Leaf{fields: usize},
    Branch{fields: usize}, U128OutOfRange }                            // + marker_free: bool per row
pub enum DataRow  { U128(u128), Bool(bool), Nil, Cons{fields: usize}, Leaf{fields: usize},
    Branch{fields: usize} }                                            // no Seq row (D-08R)
pub struct EnvCell { pub binder: usize, pub value: ValueRef, pub previous: Option<usize> }

pub enum FrameRow { LetF{binder, body, saved_env, origin}, LeftF{op, right, saved_env, origin},
    RightF{op, left: ValueRef, origin}, UnaryF{op, origin}, IfF{then_expr, else_expr, saved_env, origin},
    MatchF{arms: Slice, saved_env, origin}, ArgsF{function_name, completed: ValSlice, rest: Slice, saved_env, origin},
    ReturnF{return_type: Sort, saved_env, origin}, ScopeF{saved_env, origin} }

// Muse F-01: one discriminator per sum, so the successful empty binding/match stays distinct.
pub enum BoundBinding { Bad, Ok { env: EnvRoot } }
pub enum CaseMatching { NoMatch, Matched { env: EnvRoot } }
pub enum PrimOutcome  { Value(ValueRef), Error(u8 /* 10 | 11 */) }
pub enum Control { Eval(usize), Return(ValueRef),
    ScheduleNext { frame: FrameRow, next: usize, env: EnvRoot },
    Invoke { function_name: usize, args: ValSlice },
    Enter  { function: Option<usize>, args: ValSlice },
    Bound  { binding: BoundBinding, return_type: Sort, body: usize },
    Cases  { value: ValueRef, arms: Slice, saved_env: EnvRoot },
    CaseResult { matching: CaseMatching, value: ValueRef, body: usize, rest: Slice, saved_env: EnvRoot },
    PrimResult(PrimOutcome), Error(u8 /* 1..=11 */) }

pub enum ScalarStaged { None, U128Value(u128), BoolValue(bool), PendingNode(NodeKind),
    PrimError(u8), Selector(BoolSelector), TypeTest(bool) }
pub type ResumeId = usize; // R3-T1 D-35 (Muse R3-F01): core host carrier; no Resume table in B-core
pub struct HelperCommon { pub site: SiteTag, pub origin_reference_cursor: u128,
    pub parent_resume: Option<ResumeId> /* always None in core; Some(_) is HostViolation SI-08 */ }
pub enum HelperControl { Scalar{..}, EnvGet{..}, EnvPut{..}, Find{..}, Bind{..},
    Append{..}, Validate{..}, Equal{..}, Match{..} }   // fields exactly as schema records.HelperControl
pub enum HelperTask { Validate{r: ValueRef, visit: TaskVisit, cursor: usize},
    Equal{l: ValueRef, r: ValueRef, visit: TaskVisit, cursor: usize},
    Pair{binder: usize, sort: Sort, arg: ValueRef} }
pub enum HelperResult { Validated(bool), EnvFound(ValueRef), EnvMissing, EnvUpdated(usize),
    FindFound(usize), FindMissing, BindOk(EnvRoot), BindBad, AppendDone(ValSlice),
    ScalarValue(ValueRef), ScalarError(u8), ScalarSelector(BoolSelector), ScalarTypeTest(bool),
    EqualDone(ValueRef), MatchMatched(EnvRoot), MatchNoMatch }
pub enum Outcome { Running, Halted(ValueRef), Rejected(u8), Exhausted(ExhaustReason) }

#[derive(Clone)] pub struct Admitted { body: Arc<AdmittedBody> }   // private (C6); Clone shares the body (C9)
struct AdmittedBody { identity: Identity, view: ProgramView, bounds: Bounds,
    layout: LayoutSchema, encoding: Encoding /* HostOnly | Bounded */ }
// binding check (precedence AB): Arc::ptr_eq(&a.body, &img.admission.body)

pub struct MachineImage {     // private fields (C6)
    admission: Admitted /* R2-T1 D-32; identity is admission.body.identity */,
    run_id: Arc<RunId> /* schema machine_image.run_id; set once by admit_run */,
    control: Control, control_origin: usize, env_root: EnvRoot,
    depth: usize, stack_limit: usize, outcome: Outcome,
    initial_fuel: u128, remaining_work: u128, consumed_work: u128,
    helper_work: u128, helper_visits: u128, reference_cursor: u128,
    alloc: Alloc /* data_nodes, data_fields, environment_cells */,
    seg_base: SegBase /* all zero in B-core */,
    helper: Option<HelperControl>, result: Option<HelperResult>,
    data: Vec<DataRow>, data_fields: Vec<ValueRef>, env: Vec<EnvCell>,
    value_slots: Vec<ValueRef>, frames: Vec<FrameRow>, tasks: Vec<HelperTask>,
}   // Vec length = physical active length; capacities enforced from Bounds/LayoutCapacities (C3)
```

**Host integers.** `u128` holds UInt128 exactly. `checked_add`, `checked_sub` and `checked_mul` on `u128` give error 10 exactly when the mathematical result is outside `[0, 2^128)`.

**Widths.** The schema's width formulas govern bounded encodings only. With `layout.widths = None`, `Admitted` is tagged `HostOnly` and no encoder accepts it.

## 4. Callable API (aligned with the schema; answers Muse F-03, F-08)

```rust
pub fn prepare_core(p: &CoreProgram, b: &Bounds, l: &LayoutSchema) -> Result<ProgramView, Tooling>;
pub fn check_program_view(v: &ProgramView, b: &Bounds, l: &LayoutSchema)
    -> Result<(), ProgramViewViolation>;                                   // AD-5, V1..V8
pub fn admit(p: &CoreProgram, b: &Bounds, l: &LayoutSchema) -> Result<Admitted, Tooling>; // AD-1..AD-6 (AD-0 retired)
pub fn admit_run(a: &Admitted, fuel: RunInt, stack_limit: RunInt, finance: FinanceInput)
    -> Result<MachineImage, Tooling>;                // R1..R6; the image keeps a clone of `a` (C9)
// The four functions below run the admission-binding check AB first (C9).
pub fn select(a: &Admitted, img: &MachineImage) -> Result<Selected, HostViolation>;
pub fn step(a: &Admitted, img: &mut MachineImage) -> Result<StepResult, HostViolation>;
pub fn run(a: &Admitted, img: &mut MachineImage, sink: &mut dyn FnMut(&StepEvent))
    -> Result<RunSummary, HostViolation>;                // stop rule C4: <= pre.remaining_work + 1 steps
pub fn check_step_inv(a: &Admitted, img: &MachineImage) -> Result<(), Vec<StepInvViolation>>;

pub enum Tooling {
    AdmissionRejected   { reason: AdmissionReason, detail: String, locator: Option<Locator> },
    CapacityNotAdmitted { kind: CapacityKind, needed: NeededNat, limit: u128, locator: Option<Locator> },
    UnsupportedFeature  { feature: FeatureKind, locator: Option<Locator> },
}
pub enum NeededNat { Exact(u128), AboveU128 }   // schema api_types.NeededNat; AboveU128 only at admitRun R3/R4
pub enum CapacityKind { CodeRows, FunctionRows, CodeChildSlots, ParameterSlots, PatternRows,
    MatchArmRows, ConstantNodes, ConstantFieldSlots, MaxArity, NameRows, NameBytes,
    OccurrencePathSlots, ValueSlots, MaxReferenceWork, Frames, SerializationBytes,
    PhysicalBoundaryEnvelope, StepCap, CounterWidth { field: CounterWidthField } } // R2-T1 D-31
pub enum CounterWidthField { WorkCounter, HelperCounter, ReferenceCursor }
pub enum AdmissionReason { LayoutWidthTooSmall, ProgramViewMalformed, GrammarDomainMismatch,
    ProfileMismatch, LayoutBoundsMismatch }              // AD-0a adds LayoutBoundsMismatch (R3-T1 D-34)
pub struct StepInvViolation { pub clause: StepInvClauseId, pub locator: Option<String>,
    pub detail: String }
pub struct HostViolation { pub kind: HostViolationKind, pub clause: Option<StepInvClauseId>,
    pub rule: Option<RuleId>, pub locator: Option<String> }
pub struct Selected { pub rule: RuleId, pub event_kind: EventKind, pub site: Option<SiteTag>,
    pub helper_kind: Option<HelperKind>, pub helper_eligible: bool, pub helper_visits: u8 }
pub enum StepResult { Terminal, Event(StepEvent) }    // Terminal only when outcome != Running
```

- `Tooling` and `HostViolation` are never an `Outcome`, an error code or an `Exhausted` reason.
- `HostViolation` means the pre-image is outside StepInv or a guard is undefined (C-INV territory). `step` returns it **without mutating** the image.
- `prepare_core` follows the deterministic three-pass order in schema `admission.prepare_core.deterministic_order` (answers Muse F-10): features, then names, then emission C1–C6, refusing at the first overflow in that order.
- `admit` follows schema `admission.admit.checks_in_order` exactly (R2-T1 D-31, R3-T1 D-34). First, AD-0a checks `layout.bounds == *b` (structural equality over all 22 `Bounds` fields), refusing a mismatch with `AdmissionRejected{reason: LayoutBoundsMismatch, detail: "layout.bounds != bounds", locator: Some(Field{name: "layout.bounds"})}`; every unqualified `Bounds` read in preparation, width checks, admission and runtime denotes `b`. Then `stepCap = 0` is `CapacityNotAdmitted{StepCap}`, a short WorkCounter, HelperCounter or ReferenceCursor width is `CapacityNotAdmitted{CounterWidth{field}}`, `maxArity < 2` is `CapacityNotAdmitted{MaxArity}`, `codeRows = 0` is C1's `CapacityNotAdmitted{CodeRows}`, and a present `boundaryCapacities` is `UnsupportedFeature{BoundarySection}`. Calling `select(a, img)` when `img.outcome != Running` returns `Err(HostViolation{kind: StepInvViolated, clause: Some(SI-18), rule: None, locator: Some("select on non-Running image")})`. The `Tooling` error type is an explicit amendment of the erratum signature `Result<Admitted, CapacityNotAdmitted>` (schema `admission.admit.error_type_amendment`).

## 5. `step`: normative order (answers Opus2 L-3)

```text
if !Arc::ptr_eq(&a.body, &img.admission.body)     -> Err(HostViolation{StepInvViolated, SI-01}) // AB (C9)
                                                     // checked before any other read; img untouched
if img.outcome != Running                         -> Ok(Terminal)                            // P0
if helper.is_none() && result.is_none() {
    Error(c)                                      -> outcome = Rejected(c); emit Publish     // P1
    Return(v) && depth == 0                       -> outcome = Halted(v);   emit Publish     // P1
}                                                    // P1 never moves referenceCursor (D-09R)
if remaining_work == 0                            -> outcome = Exhausted(Fuel); emit Publish // P2
let e    = select(a, img)?;                          // pure; HostViolation leaves img untouched
let plan = rows[e].plan(a, img)?;                    // pure; reads only the pre-image
                                                     // also computes cap_refusal (P6 order) and, if
                                                     // it is None, checks every planned write's
                                                     // physical position: PhysicalCapacityExceeded
if e.helper_eligible && helper_work == max_helper_work
                                                  -> outcome = Exhausted(HelperWork);
                                                     emit(e, RefusedHelperWorkUncharged)     // P3
remaining_work -= 1; consumed_work += 1; if e.helper_eligible { helper_work += 1 }           // P4
for kind in [DataNodes, DataFields, EnvironmentCells, HelperTasks] {                          // P6
    if plan.final_delta(kind) > 0 && counter(kind) + plan.final_delta(kind) > cap(kind)
                                                  -> outcome = Exhausted(kind); emit(e, RefusedCapCharged)
}
plan.apply(img);                                     // effects in row order (row_language)
helper_visits += e.helper_visits;
if e.event_kind == LegacyComplete { reference_cursor += 1 }                                   // P7
emit(e, Applied)
```

- One `plan` function per Enabled RuleId: 37 in `legacy_rows.rs`, 59 in `helper_rows.rs`. Disabled and ExtensionOnly IDs have no plan; reaching one is `HostViolation{DisabledRuleReached | ExtensionRuleReached}`.
- `plan` evaluates the row's `effects` against the pre-image, computes the final delta vector (base plus every `when` add whose guard holds) and the full write set, then returns. `apply` is infallible.
- **Selection.** Helper groups and legacy groups are tested in the listed order with each row's `select_when`; entry rows come from `helperPlan`. Implement them as exhaustive `match`es whose only fallback arm returns `HostViolation{SelectUndefined}` (schema `dispatch.totality`). No wildcard arm may silently pick a row. With a staged result, only a legacy row whose `select_when` reads `r` is eligible (R2-T1, Opus1 R2-L1); a row that ignores `r` is never selected over a stale result.
- **Physical capacity (R2-T1, Opus1 R2-L5).** Every physical capacity check is part of the pure `plan` over the pre-image, never `apply`. It applies only when no P6 kind would refuse, so a valid logical exhaustion (for example `alloc.dataNodes = bounds.dataNodes` at `Scalar.exit`) stays `Exhausted(DataNodes)` and never becomes a HostViolation. `apply` is infallible.

## 6. Static conformance the implementation must make checkable

These are specified checks; no outcome is predicted.

1. The `RuleId` enum has exactly the 112 strings of schema `enums.RuleId`, in order.
2. Each Enabled RuleId has exactly one plan. Its declared base delta vector, guard-keyed adds, `cap_kinds_possible`, `helperVisits` and `eventKind` equal the row's. The 11-entry vector order is schema `row_language.deltas.vector_order`.
3. Each helper group and legacy group lists the same RuleIds, in the same order, as schema `dispatch.helper_groups` and `dispatch.legacy_groups`.
4. `check_step_inv` holds on both genesis images and after every Applied event in every normal run. Its failure is reported as a defect, never treated as an Outcome.
5. `referenceCursor = consumedWork − helperWork` after every event (SI-04).
6. `active.valueSlots ≤ consumedWork` after every event (SI-23).

## 7. Author illustrations (hand-derived from the rows; not oracles)

O must derive expectations independently. These exist only to make the contract concrete. "cursor" is `referenceCursor`.

| Program (entry only unless stated) | Inputs | Event sequence | Final |
|---|---|---|---|
| literal `5` | F ≥ 5, any L, valueSlots ≥ F | Validate.entry, Validate.occurrence, Validate.empty, Validate.exit, eval.literal.valid, publish.value | Halted(Const 5); consumed 5, helperWork 4, helperVisits 1, cursor 1 |
| literal `5` | F = 2 | Validate.entry, Validate.occurrence, publish.exhausted | Exhausted(Fuel); consumed 2, helperWork 2, cursor 0; Validate helper live, status Pending, taskLive 0 |
| variable `x`, empty env | F ≥ 4 | EnvGet.entry, EnvGet.empty, EnvGet.failure, eval.variable.absent, publish.error | Rejected(3); consumed 4, helperWork 3, helperVisits 0, cursor 1 |
| `let x = 1 in x` | F ≥ 2, L = 0 | eval.let, push.refuse, publish.error | Rejected(4) (resource limit); consumed 2, helperWork 0, cursor 2 |
| `f(7)` with `f(a: UInt): UInt = a` | F ≥ 33, L ≥ 1, valueSlots ≥ F, environmentCells ≥ 1, helperTasks ≥ 1 | eval.call.arguments, push.accept, Validate.entry, Validate.occurrence, Validate.empty, Validate.exit, eval.literal.valid, Append.entry, Append.empty, Append.tail, Append.exit, arguments.final, Find.entry, Find.row, Find.exit, invoke.find, Bind.entry, Bind.pair, Bind.empty, Bind.type, Bind.insert, Bind.exit, enter.bind, bound.valid, push.accept, EnvGet.entry, EnvGet.cell, EnvGet.exit, eval.variable.present, Scalar.entry, Scalar.tag, Scalar.exit, return.call.valid, publish.value | Halted(Const 7); consumed 33, helperWork 23, helperVisits 8, cursor 10; alloc.environmentCells 1, alloc.dataNodes 0, active.valueSlots 1 |

The last row has 10 legacy completions, which equals the 10 charged K-A events for the same program, so 33 = 10 + 23. It illustrates the `W_B = W_A + H_B` target for one completed run. It is not evidence for that relation.

## 8. Rows-to-code notes

- The textual expressions in the rows parse to the AST in schema `row_language.ast`. An implementer may hand-translate them, but every translation must preserve guard order and the read-pre/write-post discipline.
- `kind()` replaces every raw tag comparison. `Const Bool(true)` equals `Data Bool(true)` (HB-28).
- **Value slots.** `Append.copy` and `Append.tail` each write one slot at `active.valueSlots`. Slots are never rewritten. No P6 kind covers them because R5 admits `valueSlots ≥ fuel`. If a planned slot write would still exceed capacity, `plan` reports `HostViolation{PhysicalCapacityExceeded}` before P3, never after P4, and never as an outcome (R2-T1).
- **Tasks.** At most one top rewrite, then at most one push or pop, per event.

## 9. Retained obligations this build does not discharge

A host build of B-core closes none of these:

- generic declaration-driven ADTs (`b.coverage.*`, UIREQ-007/008/034, B-T10);
- nested helpers with resume rows;
- finance (UISEC-007/008, B008, UIREQ-014..020, B-T12, Core/2);
- canonical `project` and `decodeCanonical` beyond genesis, ChunkStore, constrained ProjectRel and Link, B002 codec laws, the L03 decision;
- StepInv genesis and preservation proofs;
- an independent B comparator over StepEvents (the existing `compare.rs` is A-only);
- K, Quint and Lean B projections and the `W_B = W_A + H_B` target;
- the Clean step circuit and native export; L04 statement, exposure and field-embedding binding;
- every U, M, MC, SP, G, ZR, MNR and OSR exit; the ACTUS and DeFi denominators; composition; Preview, Fenrir and Midnight stagenet evidence.

The exact interfaces the parallel full-B extension consumes are listed in schema `extension_interfaces` X-01..X-12.
