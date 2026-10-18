
// CoreProgram: uniquely owned tree (Box/Vec, no Arc/Rc). SECURITY OWNER in
// shared::owner must supply iterative Drop (heap worklist) for CoreExpr/
// CoreTree; compiler Drop glue alone does NOT satisfy D-36. No Drop body
// is claimed. C8: CoreProgram/CoreExpr/CoreTree/CoreValue derive no Clone,
// PartialEq, Eq, Debug or Hash; any owner diagnostic traversal is iterative.
pub struct CoreProgram { pub functions: Vec<CoreFunction>, pub entry: CoreExpr }
pub struct CoreFunction { pub name: String, pub parameters: Vec<CoreParameter>,
    pub return_type: SortTag, pub body: CoreExpr }
pub struct CoreParameter { pub name: String, pub sort: SortTag }
pub enum CoreExpr { Literal { value: CoreValue }, Var { name: String },
    Let { name: String, value: Box<CoreExpr>, body: Box<CoreExpr> },
    Binary { op: BinaryOp, left: Box<CoreExpr>, right: Box<CoreExpr> },
    Unary { op: UnaryOp, operand: Box<CoreExpr> },
    If { cond: Box<CoreExpr>, then_expr: Box<CoreExpr>, else_expr: Box<CoreExpr> },
    Call { name: String, args: Vec<CoreExpr> },
    Match { scrutinee: Box<CoreExpr>, arms: Vec<CoreArm> } }
pub struct CoreArm { pub pattern: CorePattern, pub body: CoreExpr }
pub enum CorePattern { PVar { x: String }, PNil,
    PCons { x: String, y: String }, PLeaf { x: String },
    PBranch { x: String, y: String } }
pub enum CoreValue { UInt { n: i128_class::KIntC }, Bool { b: bool },
    List { elements: Vec<i128_class::KIntC> }, Tree { tree: CoreTree } }
pub mod i128_class {
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub enum KIntC { InRange(u128), Negative, AboveU128 }
}
pub enum CoreTree { Leaf { n: i128_class::KIntC },
    Branch { left: Box<CoreTree>, right: Box<CoreTree> } }
// From<&reference_types::Program>: #[cfg(test)] only, no Stage-2 entry point.

// ProgramView: Vec lengths ARE active lengths (no stored duplicate).
pub enum ExpressionRowC { Literal { constant: usize }, Var { binder: usize },
    Let { binder: usize, value: usize, body: usize },
    Binary { op: BinaryOp, left: usize, right: usize },
    Unary { op: UnaryOp, operand: usize },
    If { cond: usize, then_expr: usize, else_expr: usize },
    Call { function_name: usize, args: SliceC },
    Match { scrutinee: usize, arms: SliceC } }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct FunctionRowC { pub function_name: usize, pub params: SliceC, pub return_type: SortTag, pub body: usize }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ParameterRowC { pub binder: usize, pub sort: SortTag }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ArmRowC { pub pattern: usize, pub body: usize }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PatternRowC { PVar { x: usize }, PNil, PCons { x: usize, y: usize }, PLeaf { x: usize }, PBranch { x: usize, y: usize } }
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OccurrenceRowC { pub owner_is_function: bool, pub function: usize, pub path: SliceC }
    // owner_is_function=false is Entry with function=0; path slices occurrence_paths.
pub struct ProgramView {
    pub entry: usize,
    pub expressions: Vec<ExpressionRowC>, pub occurrences: Vec<OccurrenceRowC>, pub occurrence_paths: Vec<usize>,
    pub children: Vec<usize>, pub functions: Vec<FunctionRowC>,
    pub parameters: Vec<ParameterRowC>, pub arms: Vec<ArmRowC>,
    pub patterns: Vec<PatternRowC>,
    pub constants: Vec<RawConstRow>, pub constant_fields: Vec<usize>,
    pub names: Vec<String>,
}
    // Vec lengths are the sole active lengths. constants hold markerFree in RawConstRow; constant_fields are Const ids in field order.
impl ProgramView {
    pub fn active_expressions(&self) -> usize { self.expressions.len() }
    pub fn active_functions(&self) -> usize { self.functions.len() }
    pub fn active_names(&self) -> usize { self.names.len() }
}
// WidthTable: private, duplicate-free by construction (owner-only ctor).
pub struct WidthTable { index_widths: Vec<(String, u32)>, counter_widths: Vec<(String, u32)>, record_widths: Vec<(String, u32)> }
impl WidthTable {
    pub fn new_validated(index_widths: Vec<(String, u32)>, counter_widths: Vec<(String, u32)>, record_widths: Vec<(String, u32)>) -> Result<Self, String> {
        fn norm(mut e: Vec<(String, u32)>) -> Result<Vec<(String, u32)>, String> { e.sort_by(|a, b| a.0.cmp(&b.0)); for w in e.windows(2) { if w[0].0 == w[1].0 { return Err(w[0].0.clone()); } } Ok(e) }
        Ok(Self { index_widths: norm(index_widths)?, counter_widths: norm(counter_widths)?, record_widths: norm(record_widths)? })
    }
    fn get(e: &[(String, u32)], n: &str) -> u32 { match e.binary_search_by(|x| x.0.as_str().cmp(n)) { Ok(i) => e[i].1, Err(_) => 0 } }
    pub fn index_width(&self, n: &str) -> u32 { Self::get(&self.index_widths, n) }
    pub fn record_width(&self, n: &str) -> u32 { Self::get(&self.record_widths, n) }
    pub fn counter_width(&self, f: &CounterWidthField) -> u32 { Self::get(&self.counter_widths, match f { CounterWidthField::WorkCounter => "WorkCounter", CounterWidthField::HelperCounter => "HelperCounter", CounterWidthField::ReferenceCursor => "ReferenceCursor" }) }
    // Three R4 categories, absent=0. AD-2a checks WorkCounter,HelperCounter,ReferenceCursor in that order. Public only as validated input construction; no bounded approval is implied and HostOnly remains widths=None.
    // Raw pre-admission enumeration: each borrows one stored list whole, in stored
    // order and multiplicity; an absent name is simply absent (no lookup default).
    // No sort, copy, validity or admission is implied; owner_host observes admitted state.
    pub fn index_width_entries(&self) -> &[(String, u32)] { self.index_widths.as_slice() }
    pub fn counter_width_entries(&self) -> &[(String, u32)] { self.counter_widths.as_slice() }
    pub fn record_width_entries(&self) -> &[(String, u32)] { self.record_widths.as_slice() }
}
pub struct LayoutSchemaC { pub layout_version: (String, String),
    pub field_id: Option<(String, String)>, pub widths: Option<WidthTable>,
    pub scalar_layout_id: Option<(String, String)>, pub bounds: BoundsC,
    pub capacities: LayoutCapacitiesC,
    pub boundary_capacities_present: bool } // must be false in B-core (AD-6)

// ---------- errors / selection: no derives over raw, moved RawRuleId ----
pub struct HostViolation { pub kind: HostViolationKind,
    pub clause: Option<StepInvClauseId>, pub rule: Option<RuleIdC>,
    pub locator: Option<String> }
pub struct StepInvViolation { pub clause: StepInvClauseId,
    pub locator: Option<String>, pub detail: String }
pub struct ProgramViewViolation { pub check: ViewCheck, pub table: String,
    pub row: Option<usize>, pub detail: String }
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum NeededNatC { Exact(u128), AboveU128 }
    // Host carrier for schema api_types.NeededNat (CapacityNotAdmitted.needed).
    // Exact carries an exact Nat: a usize count widened to u128, a bitlen,
    // the AD-1/AD-3 minimum, or RunIntC::Nat(u128). AboveU128 is produced
    // only from RunIntC::NatAboveU128 at admitRun R3/R4, whose magnitude the
    // 3-class input never carried. It is never a saturated finite needed value.
pub enum Tooling { AdmissionRejected { reason: AdmissionReason,
        detail: String, locator: Option<LocatorC> },
    CapacityNotAdmitted { kind: CapacityKind, needed: NeededNatC, limit: u128,
        locator: Option<LocatorC> },
    UnsupportedFeature { feature: FeatureKind, locator: Option<LocatorC> } }
// Selected is the SINGLE authoritative dispatch fact. Families must not
// recompute rule, site, helper kind, event kind, eligibility, or visits.
pub struct Selected { pub rule: RuleIdC, pub event_kind: EventKind,
    pub site: Option<SiteTag>, pub helper_kind: Option<HelperKind>,
    pub helper_eligible: bool, pub helper_visits: u8 }
pub struct RunSummary { pub outcome: OutcomeC, pub events: u128,
    pub consumed_work: u128, pub helper_work: u128, pub helper_visits: u128,
    pub reference_cursor: u128 }
pub enum StepResult { Terminal, Event { event: StepEventC } }
    // Full R4 StepEventC above; sink and consumers receive StepEventC, not a 3-field reduction.

// ---------- Admitted / MachineImage: private, owner-child only ----------
struct AdmittedBody { identity: IdentityC, bounds: BoundsC, view: ProgramView, layout: LayoutSchemaC }
    // layout is retained for AD/R5/physical checks; bounds==layout.bounds by AD-0a and is the authoritative copy.
pub struct Admitted { body: Arc<AdmittedBody> }
impl Clone for Admitted {
    fn clone(&self) -> Self { Self { body: Arc::clone(&self.body) } } }
pub struct MachineImage {
    admission: Admitted, run_id: std::sync::Arc<RunIdC>, control: ControlC, control_origin: usize,
    env_root: EnvRootC, stack_limit: usize, outcome: OutcomeC,
    initial_fuel: u128, remaining_work: u128, consumed_work: u128,
    helper_work: u128, helper_visits: u128, reference_cursor: u128,
    alloc: AllocCounters, active: ActiveLengths,
    helper: Option<HelperControlC>, result: Option<HelperResultC>,
    data: Vec<RawDataRow>, data_fields: Vec<RefC>, env: Vec<EnvCellC>, frames: Vec<FrameRowC>,
    tasks: Vec<HelperTaskC>, value_slots: Vec<RefC>,
}
impl MachineImage {
    pub fn depth(&self) -> usize { self.frames.len() }
    pub fn task_live(&self) -> usize { self.tasks.len() }
    // depth and task_live are derived (frames.len(), tasks.len()); no stored copy exists.
    // `active` IS stored: owner_apply_validated rewrites it from the four Vec lengths
    // after every write, owner_validate_plan reads it for new ids, and check_step_inv
    // SI-05/SI-10 checks it against those lengths. No code reconciles or drops either;
    // owner_host observation exposes both (DEP-1).
}

