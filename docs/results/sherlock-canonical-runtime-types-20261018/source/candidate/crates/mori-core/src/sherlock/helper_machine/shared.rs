//! helper_machine::shared — PROPOSED v2, not installed, not adopted.
//! Root placement proposal (bounded, no wildcard edit):
//!   crates/mori-core/src/sherlock/helper_machine/mod.rs        (Root/I only)
//!   crates/mori-core/src/sherlock/helper_machine/raw.rs        (frozen raw-104, unmutated)
//!   crates/mori-core/src/sherlock/helper_machine/shared.rs     (this file)
//!   crates/mori-core/src/sherlock/helper_machine/shared/owner.rs
//!        (child of `shared`; the ONLY module that may construct PreImage
//!         internals, OwnerGuards, ValidatedPlan, Admitted, MachineImage)
//!   crates/mori-core/src/sherlock/helper_machine/families/{legacy,scalar,
//!        env_get,env_put,find,bind,append,validate,equal,match,bookkeeping}.rs
//!        (siblings of `shared`, NOT descendants; private fields in `shared`
//!         are therefore inaccessible to family bodies)
//! Family bodies are given no typed means to admit, charge, set Outcome,
//!   or mutate MachineImage. A family body is ordinary reviewed Rust: it can
//!   loop, allocate, or return Err(HostViolation); the owner surfaces that Err
//!   with the image untouched. It cannot publish an Outcome.
//! Callable surface = trait declarations below (future S-owned bodies in
//!   shared::owner) + the small implemented constructors/normalizers/leaves
//!   in this file. No engine, admission, StepInv, or apply body exists yet.

use std::sync::Arc;
use super::raw::{ConstRow as RawConstRow, DataRow as RawDataRow, RuleId as RawRuleId};

// ---------- plain shared enums: Copy, no raw-104 types inside ----------
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GrammarDomain { DedicatedSorts, GenericNominalExtensionReserved }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FinanceInput { Absent, TypedExtensionReserved }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SortTag { UInt, Bool, List, Tree }
// Runtime signatures share the canonical identity of the existing ADT tables.
pub use adt::TypeIdC;
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BinaryOp { Add, Sub, Mul, Eq, Lt, Cons, Branch }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum UnaryOp { Leaf, Not }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PatternTag { PVar, PNil, PCons, PLeaf, PBranch,
    WildcardExtensionReserved, ConstructorExtensionReserved } // L1 restored
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BoundaryKind { None, Terminal, TechnicalCut,
    SourceSuspensionExtensionReserved } // L1 restored; still refused in B-core
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LogicalKind { UInt, Bool, Nil, Cons, Leaf, Branch, Marker,
    ConstructorExtensionReserved }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ScalarMode { Add, Sub, Mul, Lt, Cons, Branch, Leaf, Not, TestBool, TestType }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ScalarPhase { First, Second, Done }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NodeKind { Cons, Leaf, Branch }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BoolSelector { True, False, NotBool }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TriStatus { Pending, Positive, Negative }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BindPhase { Pairing, UnwindType, UnwindDup, UnwindInsert, Propagate, Finished }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BindStatus { Ok, Bad }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AppendPhase { Copy, Tail, Done }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MatchPhase { Arm, Tag, Distinct, Field, Bind, Ready, NoMatchReady, Done }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum HelperKind { Scalar, EnvGet, EnvPut, Find, Bind, Append, Validate, Equal, Match }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FamilyId { Legacy, Scalar, EnvGet, EnvPut, Find, Bind, Append,
    Validate, Equal, Match, BookkeepingRelease }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EventKind { LegacyComplete, HelperEnter, HelperVisit, HelperExit,
    HelperFail, Bookkeeping, Publish, Startup }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SiteTag { EvalLiteral, EvalVar, EvalLet, EvalBinary, EvalUnary, EvalIf,
    EvalMatch, EvalCall, Push, RetLet, RetLeft, RetRight, RetUnary, RetIf,
    RetMatch, RetArgs, RetCall, RetScope, Invoke, Enter, Bound, Cases,
    CaseResult, Prim }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum StepInvClauseId { SI01, SI02, SI03, SI04, SI05, SI06, SI07, SI08,
    SI09, SI10, SI11, SI12, SI13, SI14, SI15, SI16, SI17, SI18, SI19, SI20,
    SI21, SI22, SI23, SI24 }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ViewCheck { V1, V2, V3, V4, V5, V6, V7, V8 }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CounterWidthField { WorkCounter, HelperCounter, ReferenceCursor }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CapacityKind { CodeRows, FunctionRows, CodeChildSlots, ParameterSlots,
    PatternRows, MatchArmRows, ConstantNodes, ConstantFieldSlots, MaxArity,
    NameRows, NameBytes, OccurrencePathSlots, ValueSlots, MaxReferenceWork,
    Frames, SerializationBytes, PhysicalBoundaryEnvelope, StepCap,
    CounterWidth { field: CounterWidthField } }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FeatureKind { GenericAdt, FinancialSyntax, FinancialInput,
    BoundedEncodingWithoutWidths, BoundarySection }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AdmissionReason { LayoutWidthTooSmall, ProgramViewMalformed,
    GrammarDomainMismatch, ProfileMismatch, LayoutBoundsMismatch }
    // GrammarDomainMismatch/ProfileMismatch: reserved, no B-core producer.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum HostViolationKind { StepInvViolated, SelectUndefined, ReadOutOfBounds,
    WrongVariant, DisabledRuleReached, ExtensionRuleReached,
    PhysicalCapacityExceeded, ArithmeticUndefined }
    // R4 enum only. Owner maps a foreign family RuleIdC and a rejected plan
    // to WrongVariant{clause:None,rule:None,locator:Some("foreign rule for family"|"invalid plan")}.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GuardId { GScalarAllocNode, GScalarFields1, GScalarFields2,
    GValidateOccLeaf, GValidateFieldNontail, GEqualPairLeaf,
    GEqualFieldNontail, GBindTypeReject, GBindDupHit }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CapKind { DataNodes, DataFields, EnvironmentCells, HelperTasks }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ExhaustReason { Fuel, DataNodes, DataFields, EnvironmentCells,
    HelperTasks, HelperWork, SerializationBytes }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum StepDisposition { Applied, Published, ExhaustedFuelUncharged,
    RefusedHelperWorkUncharged, RefusedCapCharged }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RunIntC { Negative, Nat(u128), NatAboveU128 }
    // Negative fuel/stack: genesis-invalid R2. NatAboveU128 fuel: CapacityNotAdmitted{MaxReferenceWork}; stack: {Frames}. Never bool+u128.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TaskVisitC { Unvisited, Fields }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RuleIdC { KMachineEvalLiteralValid,KMachineEvalLiteralInvalid,KMachineEvalVariablePresent,KMachineEvalVariableAbsent,KMachineEvalLet,KMachineEvalBinary,KMachineEvalUnary,KMachineEvalIf,KMachineEvalMatch,KMachineEvalCallEmpty,KMachineEvalCallArguments,KMachinePushAccept,KMachinePushRefuse,KMachineReturnLet,KMachineReturnLeft,KMachineReturnRight,KMachineReturnUnary,KMachineReturnIfTrue,KMachineReturnIfFalse,KMachineReturnIfInvalid,KMachineReturnMatch,KMachineReturnArgumentsNext,KMachineReturnArgumentsFinal,KMachineReturnCallValid,KMachineReturnCallInvalid,KMachineReturnScope,KMachineInvokeFind,KMachineEnterMissing,KMachineEnterBind,KMachineBoundInvalid,KMachineBoundValid,KMachineCasesEmpty,KMachineCasesTry,KMachineCasesMiss,KMachineCasesHit,KMachinePrimitiveValue,KMachinePrimitiveError,BHelperScalarEntry,BHelperScalarTag,BHelperScalarArithmetic,BHelperScalarConstruct,BHelperScalarExit,BHelperScalarFailure,BHelperEnvGetEntry,BHelperEnvGetCell,BHelperEnvGetEmpty,BHelperEnvGetExit,BHelperEnvGetFailure,BHelperEnvPutEntry,BHelperEnvPutCell,BHelperEnvPutInsert,BHelperEnvPutEmpty,BHelperEnvPutExit,BHelperEnvPutFailure,BHelperFindEntry,BHelperFindRow,BHelperFindEmpty,BHelperFindExit,BHelperFindFailure,BHelperBindEntry,BHelperBindPair,BHelperBindEmpty,BHelperBindArity,BHelperBindType,BHelperBindDuplicate,BHelperBindInsert,BHelperBindPropagate,BHelperBindExit,BHelperBindFailure,BHelperAppendEntry,BHelperAppendCell,BHelperAppendEmpty,BHelperAppendTail,BHelperAppendCopy,BHelperAppendExit,BHelperAppendFailure,BHelperValidateEntry,BHelperValidateOccurrence,BHelperValidateField,BHelperValidateDeclaration,BHelperValidateEmpty,BHelperValidateInvalid,BHelperValidateExit,BHelperValidateFailure,BHelperEqualEntry,BHelperEqualPair,BHelperEqualField,BHelperEqualEmpty,BHelperEqualDifferent,BHelperEqualExit,BHelperEqualFailure,BHelperMatchEntry,BHelperMatchArm,BHelperMatchOccurrence,BHelperMatchField,BHelperMatchBinding,BHelperMatchDistinct,BHelperMatchDiscard,BHelperMatchNext,BHelperMatchSelected,BHelperMatchEmpty,BHelperMatchExit,BHelperMatchFailure,BBookkeepingTaskEnqueue,BBookkeepingTaskRelease,BBookkeepingResumeStore,BBookkeepingResumeRestore,KAdminStartupValid,KAdminStartupInvalid,KAdminPublishValue,KAdminPublishError,KAdminPublishExhausted }
    // Copy shared rule carrier in R4 RuleId order. Owner alone maps &RawRuleId<->RuleIdC exhaustively; raw pin unchanged. Families match RuleIdC exhaustively; for a foreign RuleIdC they use the sole foreign-rule rendering at FamilyPlans and do not copy that RuleIdC into HostViolation.rule.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum DigestC { Unselected, Selected { format: (String, String), bytes: Vec<u8> } }
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IdentityC { pub profile_id: u8, pub grammar_domain: GrammarDomain, pub finance: FinanceInput, pub layout_version: (String, String), pub layout_digest: DigestC, pub bounds_digest: DigestC, pub field_id: Option<(String, String)>, pub program_membership_digest: DigestC }
    // profile_id: owner maps 0=sherlock-k-a/1,1=sherlock-quint-a/1,2=sherlock-helper-microstep/1; B-core requires 2.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RunIdC { pub identity: IdentityC, pub fuel_input: RunIntC, pub stack_limit_input: RunIntC }
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ExpandedStepIdC { pub run_id: std::sync::Arc<RunIdC>, pub ordinal: u128 }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum HelperResultTagC { Validated,EnvFound,EnvMissing,EnvUpdated,FindFound,FindMissing,BindOk,BindBad,AppendDone,ScalarValue,ScalarError,ScalarSelector,ScalarTypeTest,EqualDone,MatchMatched,MatchNoMatch }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum OutcomeTagC { Running, Halted, Rejected, Exhausted }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum HelperPhaseSummaryC { Scalar(ScalarPhase), Tri(TriStatus), Bind(BindPhase), Append(AppendPhase), Match(MatchPhase) }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct AllocDeltasC { pub data_nodes: i64, pub data_fields: i64, pub environment_cells: i64 }
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LocatorC { Occurrence { owner_is_function: bool, function: usize, ast_path: Vec<usize> }, Table { table: String, row: usize }, Field { name: String } }
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StepEventC { pub step_id: ExpandedStepIdC, pub rule_id: RuleIdC, pub event_kind: EventKind, pub site: Option<SiteTag>, pub helper_kind: Option<HelperKind>, pub disposition: StepDisposition, pub work_delta: u8, pub helper_work_delta: u8, pub helper_visits_delta: u8, pub reference_cursor_delta: u8, pub alloc_deltas: AllocDeltasC, pub value_slot_writes: u8, pub logical_depth_before: usize, pub logical_depth_after: usize, pub task_cursor_before: usize, pub task_cursor_after: usize, pub helper_phase_before: Option<HelperPhaseSummaryC>, pub helper_phase_after: Option<HelperPhaseSummaryC>, pub helper_cursor_before: Option<usize>, pub helper_cursor_after: Option<usize>, pub result_tag: Option<HelperResultTagC>, pub origin_reference_cursor: Option<u128>, pub source_expr_id: usize, pub source_occurrence: LocatorC, pub exhausted_kind: Option<ExhaustReason>, pub outcome_tag: OutcomeTagC, pub outcome_payload: OutcomeC, pub boundary: BoundaryKind }

// ---------- shared Copy carriers (never derived over frozen raw-104) ----
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct BoundsC { pub code_rows: usize, pub function_rows: usize,
    pub code_child_slots: usize, pub parameter_slots: usize,
    pub pattern_rows: usize, pub match_plan_rows: usize,
    pub match_arm_rows: usize, pub type_rows: usize,
    pub constructor_rows: usize, pub constructor_field_slots: usize,
    pub max_arity: usize, pub constant_nodes: usize,
    pub constant_field_slots: usize, pub data_nodes: usize,
    pub data_field_slots: usize, pub environment_cells: usize,
    pub frames: usize, pub helper_tasks: usize,
    pub serialization_bytes: usize, pub max_reference_work: u128,
    pub max_helper_work: u128, pub step_cap: u128 } // all 22, schema #/bounds
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct LayoutCapacitiesC { pub name_rows: usize, pub name_bytes: usize,
    pub occurrence_path_slots: usize, pub value_slots: usize }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RefC { Const(usize), Data(usize) } // shared owned ref; owner maps to raw
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SliceC { pub start: usize, pub len: usize }
impl SliceC {
    pub fn normalized(start: usize, len: usize) -> Self {
        if len == 0 { Self { start: 0, len: 0 } } else { Self { start, len } }
    }
    pub fn empty() -> Self { Self { start: 0, len: 0 } }
}
pub type ValSliceC = SliceC; pub type FieldSliceC = SliceC;
pub type EnvRootC = Option<usize>;
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct AllocCounters { pub data_nodes: usize, pub data_fields: usize,
    pub environment_cells: usize }
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct ActiveLengths { pub data: usize, pub data_fields: usize,
    pub env: usize, pub value_slots: usize }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct HelperCommon { pub site: SiteTag, pub origin_reference_cursor: u128,
    pub parent_resume: Option<usize> }
    // ResumeId HOST carrier = usize in Option; always None in B-core,
    // Some(_) = SI-08. Bounded width DEFERRED to X-05, not frozen here.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ScalarStaged { None, U128Value { n: u128 }, BoolValue { b: bool },
    PendingNode { tag: NodeKind }, PrimError { code: u8 }, // 10|11, owner-checked
    Selector { sel: BoolSelector }, TypeTest { ok: bool } }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum HelperControlC {
    Scalar { common: HelperCommon, mode: ScalarMode, operand_a: RefC,
        operand_b: Option<RefC>, expected: Option<TypeIdC>,
        phase: ScalarPhase, reg_a: u128, staged: ScalarStaged },
    EnvGet { common: HelperCommon, binder: usize, cursor: Option<usize>,
        found: Option<RefC>, status: TriStatus },
    EnvPut { common: HelperCommon, binder: usize, value: RefC, base: EnvRootC,
        new_root: Option<usize>, status: TriStatus },
    Find { common: HelperCommon, function_name: usize, row_cursor: usize,
        found: Option<usize>, status: TriStatus },
    Bind { common: HelperCommon, function: usize, params: SliceC, args: ValSliceC,
        p_cursor: usize, a_cursor: usize, partial: EnvRootC,
        dup_cursor: Option<usize>, phase: BindPhase, status: BindStatus },
    Append { common: HelperCommon, src: ValSliceC, value: RefC,
        src_cursor: usize, dest_start: usize, staged: Option<RefC>,
        phase: AppendPhase },
    Validate { common: HelperCommon, root: RefC, status: TriStatus },
    Equal { common: HelperCommon, left: RefC, right: RefC, status: TriStatus },
    Match { common: HelperCommon, scrutinee: RefC, arm: usize,
        saved_env: EnvRootC, partial: EnvRootC, pat_tag: PatternTag,
        x: usize, y: usize, field_idx: u8, staged: Option<RefC>,
        phase: MatchPhase, status: TriStatus },
}
// FrameRow: exactly 9 core variants. ConstructorFields is NOT a variant:
// a reserved ConstructorFields tag at any owner boundary is refused
// HostViolation{WrongVariant}/C-INV, its payload is never carried here.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FrameRowC {
    LetF { binder: usize, body: usize, saved_env: EnvRootC, origin: usize },
    LeftF { op: BinaryOp, right: usize, saved_env: EnvRootC, origin: usize },
    RightF { op: BinaryOp, left: RefC, origin: usize },
    UnaryF { op: UnaryOp, origin: usize },
    IfF { then_expr: usize, else_expr: usize, saved_env: EnvRootC, origin: usize },
    MatchF { arms: SliceC, saved_env: EnvRootC, origin: usize },
    ArgsF { function_name: usize, completed: ValSliceC, rest: SliceC,
        saved_env: EnvRootC, origin: usize },
    ReturnF { return_type: TypeIdC, saved_env: EnvRootC, origin: usize },
    ScopeF { saved_env: EnvRootC, origin: usize },
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BoundBinding { Bad, Ok { env: EnvRootC } }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CaseMatching { NoMatch, Matched { env: EnvRootC } }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PrimOutcome { Value { value: RefC }, Error { code: u8 } }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ControlC { Eval { expr: usize }, Return { value: RefC },
    ScheduleNext { frame: FrameRowC, next: usize, env: EnvRootC },
    Invoke { function_name: usize, args: ValSliceC },
    Enter { function: Option<usize>, args: ValSliceC },
    Bound { binding: BoundBinding, return_type: TypeIdC, body: usize },
    Cases { value: RefC, arms: SliceC, saved_env: EnvRootC },
    CaseResult { matching: CaseMatching, value: RefC, body: usize,
        rest: SliceC, saved_env: EnvRootC },
    PrimResult { result: PrimOutcome }, Error { code: u8 } }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum HelperTaskC {
    ValidateTask { r: RefC, visit: TaskVisitC, cursor: usize },
    EqualTask { left: RefC, right: RefC, visit: TaskVisitC, cursor: usize },
    PairTask { binder: usize, sort: TypeIdC, arg: RefC },
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum HelperResultC { Validated { ok: bool }, EnvFound { value: RefC },
    EnvMissing, EnvUpdated { cell: usize }, FindFound { function: usize },
    FindMissing, BindOk { env: EnvRootC }, BindBad, AppendDone { seq: ValSliceC },
    ScalarValue { value: RefC }, ScalarError { code: u8 },
    ScalarSelector { sel: BoolSelector }, ScalarTypeTest { ok: bool },
    EqualDone { value: RefC }, MatchMatched { env: EnvRootC }, MatchNoMatch }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum OutcomeC { Running, Halted { value: RefC }, Rejected { code: u8 },
    Exhausted { reason: ExhaustReason } }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct EnvCellC { pub binder: usize, pub value: RefC, pub previous: EnvRootC }

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
pub struct FunctionRowC { pub function_name: usize, pub params: SliceC, pub return_type: TypeIdC, pub body: usize }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ParameterRowC { pub binder: usize, pub sort: TypeIdC }
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

// ---------- concrete readonly PreImage + OwnerGuards (SEC-03) ----------
pub struct PreImage<'a> { admitted: &'a Admitted, img: &'a MachineImage }
impl<'a> PreImage<'a> {
    // AB FIRST: only Arc::ptr_eq is read before this returns. No table,
    // register, or program read precedes the binding check.
    pub fn bind(admitted: &'a Admitted, img: &'a MachineImage)
        -> Result<Self, HostViolation> {
        if Arc::ptr_eq(&admitted.body, &img.admission.body) {
            Ok(Self { admitted, img })
        } else {
            Err(HostViolation { kind: HostViolationKind::StepInvViolated,
                clause: Some(StepInvClauseId::SI01), rule: None,
                locator: Some("admission binding".to_string()) })
        }
    }
}
pub struct OwnerGuards { _sealed: () }
impl OwnerGuards {
    fn new_owner() -> Self { Self { _sealed: () } } // visible only in `shared`
    // descendants (shared::owner); families (siblings) cannot construct it.
}

// Borrowed raw-row adapter for the expanded-13 leaves (M1). This module
// re-authors the leaves as pub fns; it assumes NO old private visibility.
pub mod pure_leaves {
    use super::{LogicalKind, RawConstRow, RawDataRow};
    use super::TypeIdC as SortTag;
    #[derive(Clone, Copy)]
    pub enum BorrowedValueRow<'a> {
        Const(&'a super::RawConstRow), Data(&'a super::RawDataRow) }
    pub fn borrowed_kind(r: BorrowedValueRow<'_>) -> LogicalKind {
        match r {
            BorrowedValueRow::Const(c) => match c {
                RawConstRow::U128 { .. } => LogicalKind::UInt,
                RawConstRow::Bool { .. } => LogicalKind::Bool,
                RawConstRow::Nil { .. } => LogicalKind::Nil,
                RawConstRow::Cons { .. } => LogicalKind::Cons,
                RawConstRow::Leaf { .. } => LogicalKind::Leaf,
                RawConstRow::Branch { .. } => LogicalKind::Branch,
                RawConstRow::U128OutOfRange { .. } => LogicalKind::Marker,
            },
            BorrowedValueRow::Data(d) => match d {
                RawDataRow::U128 { .. } => LogicalKind::UInt,
                RawDataRow::Bool { .. } => LogicalKind::Bool,
                RawDataRow::Nil => LogicalKind::Nil,
                RawDataRow::Cons { .. } => LogicalKind::Cons,
                RawDataRow::Leaf { .. } => LogicalKind::Leaf,
                RawDataRow::Branch { .. } => LogicalKind::Branch,
            },
        }
    }
    pub fn borrowed_u128(r: BorrowedValueRow<'_>) -> Option<u128> {
        match r { BorrowedValueRow::Const(RawConstRow::U128 { n, .. }) => Some(*n),
            BorrowedValueRow::Data(RawDataRow::U128 { n }) => Some(*n),
            _ => None } }
    pub fn borrowed_bool(r: BorrowedValueRow<'_>) -> Option<bool> {
        match r { BorrowedValueRow::Const(RawConstRow::Bool { b, .. }) => Some(*b),
            BorrowedValueRow::Data(RawDataRow::Bool { b }) => Some(*b),
            _ => None } }
    pub fn shallow_differ(a: BorrowedValueRow<'_>, b: BorrowedValueRow<'_>) -> bool {
        borrowed_kind(a) != borrowed_kind(b)
            || borrowed_u128(a) != borrowed_u128(b)
            || borrowed_bool(a) != borrowed_bool(b) }
    pub fn arity_of(k: LogicalKind) -> Option<usize> {
        match k { LogicalKind::UInt | LogicalKind::Bool | LogicalKind::Nil => Some(0),
            LogicalKind::Cons | LogicalKind::Branch => Some(2),
            LogicalKind::Leaf => Some(1),
            LogicalKind::Marker | LogicalKind::ConstructorExtensionReserved => None } }
    pub fn sort_of_kind(k: LogicalKind) -> Option<SortTag> {
        match k { LogicalKind::UInt => Some(SortTag::UInt),
            LogicalKind::Bool => Some(SortTag::Bool),
            LogicalKind::Nil | LogicalKind::Cons => Some(SortTag::List),
            LogicalKind::Leaf | LogicalKind::Branch => Some(SortTag::Tree),
            LogicalKind::Marker | LogicalKind::ConstructorExtensionReserved => None } }
    // Marker/reserved arity or sort = None here; the OWNER maps None to
    // HostViolation{ArithmeticUndefined} / staged PrimError as the row requires.
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub enum CheckedArith { Value { n: u128 }, Overflow }
    pub fn checked_add(a: u128, b: u128) -> CheckedArith { match a.checked_add(b) { Some(n) => CheckedArith::Value { n }, None => CheckedArith::Overflow } }
    pub fn checked_sub(a: u128, b: u128) -> CheckedArith { match a.checked_sub(b) { Some(n) => CheckedArith::Value { n }, None => CheckedArith::Overflow } }
    pub fn checked_mul(a: u128, b: u128) -> CheckedArith { match a.checked_mul(b) { Some(n) => CheckedArith::Value { n }, None => CheckedArith::Overflow } }
    // Owner maps CheckedArith::Overflow only to staged PrimError{code:10}; no None-to-false and no other mapping.
}

// ---------- family-facing checked view: NO raw table escape (SEC-06) ---
pub trait PreImageView {
    fn control(&self) -> &ControlC;
    fn control_origin(&self) -> usize;
    fn env_root(&self) -> EnvRootC;
    fn depth(&self) -> usize;
    fn task_live(&self) -> usize;
    fn stack_limit(&self) -> usize;
    fn outcome(&self) -> &OutcomeC;
    fn initial_fuel(&self) -> u128;
    fn remaining_work(&self) -> u128;
    fn consumed_work(&self) -> u128;
    fn helper_work(&self) -> u128;
    fn helper_visits(&self) -> u128;
    fn reference_cursor(&self) -> u128;
    fn helper(&self) -> Option<&HelperControlC>;
    fn result(&self) -> Option<&HelperResultC>;
    fn top_frame(&self) -> Option<&FrameRowC>;
    fn top_task(&self) -> Option<&HelperTaskC>;
    fn active(&self) -> ActiveLengths;
    fn alloc(&self) -> AllocCounters;
    // All keys by reference with C carriers; all reads checked, Err never panic. No view()/ProgramView escape.
    fn expr(&self, id: &usize) -> Result<&ExpressionRowC, HostViolation>;
    fn child(&self, pos: &usize) -> Result<usize, HostViolation>;
    fn function_row(&self, id: &usize) -> Result<&FunctionRowC, HostViolation>;
    fn parameter(&self, pos: &usize) -> Result<&ParameterRowC, HostViolation>;
    fn arm(&self, id: &usize) -> Result<&ArmRowC, HostViolation>;
    fn pattern(&self, id: &usize) -> Result<&PatternRowC, HostViolation>;
    fn occurrence(&self, id: &usize) -> Result<&OccurrenceRowC, HostViolation>;
    fn occurrence_path(&self, s: &SliceC) -> Result<&[usize], HostViolation>;
    fn constant_borrowed(&self, id: &usize) -> Result<pure_leaves::BorrowedValueRow<'_>, HostViolation>;
    fn constant_field(&self, id: &usize) -> Result<usize, HostViolation>;
    fn marker_free(&self, id: &usize) -> Result<bool, HostViolation>;
    fn deref_borrowed(&self, r: &RefC) -> Result<pure_leaves::BorrowedValueRow<'_>, HostViolation>;
    fn field_at(&self, r: &RefC, i: &usize) -> Result<RefC, HostViolation>;
    fn env_cell(&self, id: &usize) -> Result<&EnvCellC, HostViolation>;
    fn value_slot(&self, pos: &usize) -> Result<RefC, HostViolation>;
    fn kind_of(&self, r: &RefC) -> Result<LogicalKind, HostViolation>;
    fn u128_of(&self, r: &RefC) -> Result<u128, HostViolation>;
    fn bool_of(&self, r: &RefC) -> Result<bool, HostViolation>;
    fn checked_arity(&self, k: &LogicalKind) -> Result<usize, HostViolation>;
    // pure arity_of None (Marker/ConstructorExtensionReserved) maps only here to ArithmeticUndefined.
    fn checked_sort(&self, k: &LogicalKind) -> Result<TypeIdC, HostViolation>;
    // pure sort_of_kind None maps only here to ArithmeticUndefined for structural sort consumers.
    fn scalar_test_type_ok(&self, r: &RefC, expected: &TypeIdC) -> Result<bool, HostViolation>;
    // Sole Scalar TestType seam: kind Marker/ConstructorExtensionReserved yields Ok(false), UInt/Bool/Nil/Cons/Leaf/Branch compares checked_sort; families never map Option themselves.
    fn admitted_function_count(&self) -> usize;
    // Sole function-table length seam: owner returns admitted view functions.len(); no other implementation and no zero fallback is permitted. Other tables remain readable only through the checked row methods above.
}
// Owner-only whole-view access lives ONLY in shared::owner (not this trait,
// not given to families): owner code uses PreImage fields directly.

pub trait GuardEvaluator {
    fn eval_guard(&self, g: &GuardId, pre: &dyn PreImageView)
        -> Result<bool, HostViolation>;
    fn eval_select_when(&self, rule: &RuleIdC, pre: &dyn PreImageView)
        -> Result<bool, HostViolation>;
    fn reads_result_root(&self, rule: &RuleIdC) -> bool; // AST Root{r}
}
// OwnerGuards implements GuardEvaluator in shared::owner (future body); families receive only &dyn GuardEvaluator.

// ---------- plan: writes authoritative, delta is owner-checked assertion --
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Write<T> { Keep, Set(T) }
impl<T> Default for Write<T> { fn default() -> Self { Write::Keep } }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FrameWrite { Push(FrameRowC), RewriteTop(FrameRowC), Pop }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TaskOp { None, Push(HelperTaskC), Pop }
impl Default for TaskOp { fn default() -> Self { TaskOp::None } }
// Per-variant staging (SEC-05): no DataRow.fields exists to ignore.
// Owner constructs the row + FieldSlice itself: len 0 -> (0,0),
// else start = pre.active.data_fields, fields written before node (G1).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AllocDataWrite { U128 { n: u128 }, Bool { b: bool }, Nil,
    Cons { fields: [RefC; 2] }, Leaf { field: RefC }, Branch { fields: [RefC; 2] } }
impl AllocDataWrite {
    pub fn field_len(&self) -> usize {
        match self { AllocDataWrite::U128 { .. } | AllocDataWrite::Bool { .. }
            | AllocDataWrite::Nil => 0,
            AllocDataWrite::Leaf { .. } => 1,
            AllocDataWrite::Cons { .. } | AllocDataWrite::Branch { .. } => 2 } }
}
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct PlanDelta { // allocation/depth/task/slot assertion only
    pub alloc_data_nodes: i64, pub alloc_data_fields: i64,
    pub alloc_environment_cells: i64, pub depth: i64, pub task_live: i64,
    pub active_value_slots: i64 }
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct PlannedWrites {
    pub control: Write<ControlC>, pub control_origin: Write<usize>,
    pub env_root: Write<EnvRootC>, pub result: Write<Option<HelperResultC>>,
    pub helper: Write<Option<HelperControlC>>, pub frame: Option<FrameWrite>,
    pub task_rewrite_top: Option<HelperTaskC>, pub task_op: TaskOp,
    pub alloc_data: Option<AllocDataWrite>, pub alloc_env: Option<EnvCellC>,
    pub alloc_value_slot: Option<RefC> }
pub struct EffectPlan { pub delta_assert: PlanDelta, pub writes: PlannedWrites }
    // No rule/event/eligible/visits/cap list: Selected (owner) is sole source;
// owner derives CapKinds from delta_assert; charges/visits/cursor fire once.

// ValidatedPlan: opaque, constructible only inside `shared` descendants.
pub struct ValidatedPlan { writes: PlannedWrites, delta: PlanDelta,
    cap_first: Option<CapKind>, rule: RuleIdC }
impl ValidatedPlan {
    fn from_validated(w: PlannedWrites, d: PlanDelta, c: Option<CapKind>, rule: RuleIdC) -> Self {
        Self { writes: w, delta: d, cap_first: c, rule } } // shared::owner only
    pub fn rule(&self) -> RuleIdC { self.rule }
}

// ---------- family contract (Sol, pure row effects only) ----------------
// Every method receives the authoritative Selected. A method presented a
// foreign RuleId MUST return HostViolation { kind: HostViolationKind::WrongVariant, clause: None, rule: None, locator: Some("foreign rule for family".to_string()) }.
// The same literal rendering applies to private family free functions. Families never
// recompute site/helper-plan/dispatch, charges, caps, or refusals.
pub trait FamilyPlans {
    fn plan_legacy(&self, sel: &Selected, pre: &dyn PreImageView, g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation>;
    fn plan_scalar(&self, sel: &Selected, pre: &dyn PreImageView, g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation>;
    fn plan_env_get(&self, sel: &Selected, pre: &dyn PreImageView, g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation>;
    fn plan_env_put(&self, sel: &Selected, pre: &dyn PreImageView, g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation>;
    fn plan_find(&self, sel: &Selected, pre: &dyn PreImageView, g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation>;
    fn plan_bind(&self, sel: &Selected, pre: &dyn PreImageView, g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation>;
    fn plan_append(&self, sel: &Selected, pre: &dyn PreImageView, g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation>;
    fn plan_validate(&self, sel: &Selected, pre: &dyn PreImageView, g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation>;
    fn plan_equal(&self, sel: &Selected, pre: &dyn PreImageView, g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation>;
    fn plan_match(&self, sel: &Selected, pre: &dyn PreImageView, g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation>;
    fn plan_bookkeeping_release(&self, sel: &Selected, pre: &dyn PreImageView, g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation>;
}
// Sol invokes owner eval_guard/eval_select_when through &dyn GuardEvaluator without implementing guards or selection.
// Foreign RuleIdC for the family (sole rendering): HostViolation { kind: HostViolationKind::WrongVariant, clause: None, rule: None, locator: Some("foreign rule for family".to_string()) }.

// ---------- security-owner contract (future S-owned bodies, PS order) ---
pub trait OwnerEngine {
    fn prepare_core(&self, p: &CoreProgram, b: &BoundsC, l: &LayoutSchemaC)
        -> Result<ProgramView, Tooling>;
    fn check_program_view(&self, v: &ProgramView, b: &BoundsC, l: &LayoutSchemaC)
        -> Result<(), ProgramViewViolation>;
    fn admit(&self, p: &CoreProgram, b: &BoundsC, l: &LayoutSchemaC)
        -> Result<Admitted, Tooling>;
    fn admit_run(&self, a: &Admitted, fuel: &RunIntC, stack_limit: &RunIntC, finance: &FinanceInput)
        -> Result<MachineImage, Tooling>;
    fn rule_id_of(&self, raw: &RawRuleId) -> RuleIdC;
    fn raw_rule_id(&self, rule: &RuleIdC) -> RawRuleId;
    fn select(&self, pre: &PreImage<'_>) -> Result<Selected, HostViolation>;
    fn family_of(&self, rule: &RawRuleId) -> Option<FamilyId>; // dispatcher;
    // None for Disabled/ExtensionOnly -> Disabled/ExtensionRuleReached.
    // PS plan validation, P6 cap refusal, PS physical check and P7 apply are
    // NOT trait methods (F-TRAIT-1): no caller-authored Selected/EffectPlan
    // reaches them. owner_step alone runs AB,P0,P1,P2,PS,P3,P4,P6,P7 over the
    // owner-private free functions, so images mutate only through step/run.
    fn step(&self, a: &Admitted, img: &mut MachineImage)
        -> Result<StepResult, HostViolation>; // AB,P0,P1,P2,PS,P3,P4,P6,P7
    fn run(&self, a: &Admitted, img: &mut MachineImage,
        sink: &mut dyn FnMut(&StepEventC)) -> Result<RunSummary, HostViolation>;
    fn check_step_inv(&self, pre: &PreImage<'_>)
        -> Result<(), Vec<StepInvViolation>>;
}

// Root module placement only; owner body above unchanged.
pub mod adt;
pub mod adt_storage;
pub mod ext_pass_b_walk;
pub mod ext_source_traversal;
pub mod nominal_declaration_order;
pub mod owner_ext_prepare_types;
pub mod owner_ext_source_program;
pub mod owner_ext_t3_uses;
pub mod owner_ext_type_builder;
mod owner_prepare;
mod owner_runtime;
pub mod owner_host;
pub mod host_implied_prefix;

pub use owner_prepare::{
    prepare_ext_program_types, prepare_nominal_declarations, prepare_program_nominal_declarations,
    PassTOutput, PassTSession, PreparedDeclarationScope, PreparedExtProgramSource,
    PreparedNominalDeclarations, run_pass_t,
};
