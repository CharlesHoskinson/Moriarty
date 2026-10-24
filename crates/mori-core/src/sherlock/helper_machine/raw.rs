//! PROVISIONAL, UNINSTALLED raw type preparation from the pinned B-core R2 proposal.
//! No schema freeze, adoption, product capability, bounded layout or wire ABI.
//! All fields are unvalidated data. No admission tokens, MachineImage, checks,
//! constructors enforcing policy, defaults, conversions or behavioral methods.
//! Compilation and all execution are UNRUN. See TYPE-MAPPING.json for exact
//! provenance and omitted dependent closures. Public visibility here does not
//! install or export this file in Moriarty.

// schema /primitive_types/Text
pub type Text = String;

// schema /primitive_types/Bool
pub type Bool = bool;

// schema /primitive_types/UInt128
pub type UInt128 = u128;

// schema /primitive_types/ErrorCode; raw carrier only, no range check.
pub struct ErrorCode(pub u8);

// schema /primitive_types/Id/record
pub struct Id {
    pub namespace: Text,
    pub name: Text,
}

// schema /primitive_types/Digest/variant
pub enum Digest {
    Unselected,
    Selected { format: Id, bytes: Vec<u8> },
}

// schema /primitive_types/RunInt/variant
pub enum RunInt {
    Negative,
    Nat { value: u128 },
    NatAboveU128,
}

// schema /primitive_types/KInt/variant
pub enum KInt {
    InRange { value: UInt128 },
    Negative,
    AboveU128,
}

// schema /derived_widths/ExprId; raw carrier only, no range check.
pub struct ExprId(pub usize);

// schema /derived_widths/ChildPos; raw carrier only, no range check.
pub struct ChildPos(pub usize);

// schema /derived_widths/FunctionRowId; raw carrier only, no range check.
pub struct FunctionRowId(pub usize);

// schema /derived_widths/FunctionCursor; raw carrier only, no range check.
pub struct FunctionCursor(pub usize);

// schema /derived_widths/ParamPos; raw carrier only, no range check.
pub struct ParamPos(pub usize);

// schema /derived_widths/ArmId; raw carrier only, no range check.
pub struct ArmId(pub usize);

// schema /derived_widths/ArmPos; raw carrier only, no range check.
pub struct ArmPos(pub usize);

// schema /derived_widths/PatternRowId; raw carrier only, no range check.
pub struct PatternRowId(pub usize);

// schema /derived_widths/ConstId; raw carrier only, no range check.
pub struct ConstId(pub usize);

// schema /derived_widths/ConstFieldPos; raw carrier only, no range check.
pub struct ConstFieldPos(pub usize);

// schema /derived_widths/DataId; raw carrier only, no range check.
pub struct DataId(pub usize);

// schema /derived_widths/FieldPos; raw carrier only, no range check.
pub struct FieldPos(pub usize);

// schema /derived_widths/EnvId; raw carrier only, no range check.
pub struct EnvId(pub usize);

// schema /derived_widths/NameId; raw carrier only, no range check.
pub struct NameId(pub usize);

// schema /derived_widths/OccPathPos; raw carrier only, no range check.
pub struct OccPathPos(pub usize);

// schema /derived_widths/ChildPosition; raw carrier only, no range check.
pub struct ChildPosition(pub usize);

// schema /derived_widths/ValueSlotPos; raw carrier only, no range check.
pub struct ValueSlotPos(pub usize);

// schema /derived_widths/ArityCount; raw carrier only, no range check.
pub struct ArityCount(pub usize);

// schema /derived_widths/Depth; raw carrier only, no range check.
pub struct Depth(pub usize);

// schema /derived_widths/StackLimit; raw carrier only, no range check.
pub struct StackLimit(pub usize);

// schema /derived_widths/TaskCount; raw carrier only, no range check.
pub struct TaskCount(pub usize);

// schema /derived_widths/WorkCounter; raw carrier only, no range check.
pub struct WorkCounter(pub u128);

// schema /derived_widths/HelperCounter; raw carrier only, no range check.
pub struct HelperCounter(pub u128);

// schema /derived_widths/ReferenceCursor; raw carrier only, no range check.
pub struct ReferenceCursor(pub u128);

// schema /enums/ProfileId
pub enum ProfileId {
    SherlockKA1,
    SherlockQuintA1,
    SherlockHelperMicrostep1,
}

// schema /enums/GrammarDomain
pub enum GrammarDomain {
    DedicatedSorts,
    GenericNominalExtensionReserved,
}

// schema /enums/FinanceInput
pub enum FinanceInput {
    Absent,
    TypedExtensionReserved,
}

// schema /enums/SortTag
pub enum SortTag {
    UInt,
    Bool,
    List,
    Tree,
}

// schema /enums/BinaryOp
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Eq,
    Lt,
    Cons,
    Branch,
}

// schema /enums/UnaryOp
pub enum UnaryOp {
    Leaf,
    Not,
}

// schema /enums/ExprTag
pub enum ExprTag {
    Literal,
    Var,
    Let,
    Binary,
    Unary,
    If,
    Call,
    Match,
    ConstructExtensionReserved,
}

// schema /enums/PatternTag
pub enum PatternTag {
    PVar,
    PNil,
    PCons,
    PLeaf,
    PBranch,
    WildcardExtensionReserved,
    ConstructorExtensionReserved,
}

// schema /enums/LogicalKind
pub enum LogicalKind {
    UInt,
    Bool,
    Nil,
    Cons,
    Leaf,
    Branch,
    Marker,
    ConstructorExtensionReserved,
}

// schema /enums/ConstTag
pub enum ConstTag {
    U128,
    Bool,
    Nil,
    Cons,
    Leaf,
    Branch,
    ConstructorExtensionReserved,
    U128OutOfRange,
}

// schema /enums/DataTag
pub enum DataTag {
    U128,
    Bool,
    Nil,
    Cons,
    Leaf,
    Branch,
    ConstructorExtensionReserved,
}

// schema /enums/FrameTag
pub enum FrameTag {
    LetF,
    LeftF,
    RightF,
    UnaryF,
    IfF,
    MatchF,
    ArgsF,
    ReturnF,
    ScopeF,
    ConstructorFieldsExtensionReserved,
}

// schema /enums/ControlTag
pub enum ControlTag {
    Eval,
    Return,
    ScheduleNext,
    Invoke,
    Enter,
    Bound,
    Cases,
    CaseResult,
    PrimResult,
    Error,
}

// schema /enums/HelperKind
pub enum HelperKind {
    Scalar,
    EnvGet,
    EnvPut,
    Find,
    Bind,
    Append,
    Validate,
    Equal,
    Match,
}

// schema /enums/SiteTag
pub enum SiteTag {
    EvalLiteral,
    EvalVar,
    EvalLet,
    EvalBinary,
    EvalUnary,
    EvalIf,
    EvalMatch,
    EvalCall,
    Push,
    RetLet,
    RetLeft,
    RetRight,
    RetUnary,
    RetIf,
    RetMatch,
    RetArgs,
    RetCall,
    RetScope,
    Invoke,
    Enter,
    Bound,
    Cases,
    CaseResult,
    Prim,
}

// schema /enums/ScalarMode
pub enum ScalarMode {
    Add,
    Sub,
    Mul,
    Lt,
    Cons,
    Branch,
    Leaf,
    Not,
    TestBool,
    TestType,
}

// schema /enums/ScalarPhase
pub enum ScalarPhase {
    First,
    Second,
    Done,
}

// schema /enums/ScalarStagedTag
pub enum ScalarStagedTag {
    None,
    U128Value,
    BoolValue,
    PendingNode,
    PrimError,
    Selector,
    TypeTest,
}

// schema /enums/NodeKind
pub enum NodeKind {
    Cons,
    Leaf,
    Branch,
}

// schema /enums/BoolSelector
pub enum BoolSelector {
    True,
    False,
    NotBool,
}

// schema /enums/TriStatus
pub enum TriStatus {
    Pending,
    Positive,
    Negative,
}

// schema /enums/BindPhase
pub enum BindPhase {
    Pairing,
    UnwindType,
    UnwindDup,
    UnwindInsert,
    Propagate,
    Finished,
}

// schema /enums/BindStatus
pub enum BindStatus {
    Ok,
    Bad,
}

// schema /enums/AppendPhase
pub enum AppendPhase {
    Copy,
    Tail,
    Done,
}

// schema /enums/TaskVisit
pub enum TaskVisit {
    Unvisited,
    Fields,
}

// schema /enums/MatchPhase
pub enum MatchPhase {
    Arm,
    Tag,
    Distinct,
    Field,
    Bind,
    Ready,
    NoMatchReady,
    Done,
}

// schema /enums/HelperTaskTag
pub enum HelperTaskTag {
    ValidateTask,
    EqualTask,
    PairTask,
}

// schema /enums/HelperResultTag
pub enum HelperResultTag {
    Validated,
    EnvFound,
    EnvMissing,
    EnvUpdated,
    FindFound,
    FindMissing,
    BindOk,
    BindBad,
    AppendDone,
    ScalarValue,
    ScalarError,
    ScalarSelector,
    ScalarTypeTest,
    EqualDone,
    MatchMatched,
    MatchNoMatch,
}

// schema /enums/OutcomeTag
pub enum OutcomeTag {
    Running,
    Halted,
    Rejected,
    Exhausted,
}

// schema /enums/ExhaustReason
pub enum ExhaustReason {
    Fuel,
    DataNodes,
    DataFields,
    EnvironmentCells,
    HelperTasks,
    HelperWork,
    SerializationBytes,
}

// schema /enums/StepDisposition
pub enum StepDisposition {
    Applied,
    Published,
    ExhaustedFuelUncharged,
    RefusedHelperWorkUncharged,
    RefusedCapCharged,
}

// schema /enums/EventStatus
pub enum EventStatus {
    Enabled,
    AdministrativeBoundary,
    Disabled,
    ExtensionOnly,
}

// schema /enums/EventKind
pub enum EventKind {
    LegacyComplete,
    HelperEnter,
    HelperVisit,
    HelperExit,
    HelperFail,
    Bookkeeping,
    Publish,
    Startup,
}

// schema /enums/BoundaryKind
pub enum BoundaryKind {
    None,
    Terminal,
    TechnicalCut,
    SourceSuspensionExtensionReserved,
}

// schema /enums/RuleId
pub enum RuleId {
    KMachineEvalLiteralValid,
    KMachineEvalLiteralInvalid,
    KMachineEvalVariablePresent,
    KMachineEvalVariableAbsent,
    KMachineEvalLet,
    KMachineEvalBinary,
    KMachineEvalUnary,
    KMachineEvalIf,
    KMachineEvalMatch,
    KMachineEvalCallEmpty,
    KMachineEvalCallArguments,
    KMachinePushAccept,
    KMachinePushRefuse,
    KMachineReturnLet,
    KMachineReturnLeft,
    KMachineReturnRight,
    KMachineReturnUnary,
    KMachineReturnIfTrue,
    KMachineReturnIfFalse,
    KMachineReturnIfInvalid,
    KMachineReturnMatch,
    KMachineReturnArgumentsNext,
    KMachineReturnArgumentsFinal,
    KMachineReturnCallValid,
    KMachineReturnCallInvalid,
    KMachineReturnScope,
    KMachineInvokeFind,
    KMachineEnterMissing,
    KMachineEnterBind,
    KMachineBoundInvalid,
    KMachineBoundValid,
    KMachineCasesEmpty,
    KMachineCasesTry,
    KMachineCasesMiss,
    KMachineCasesHit,
    KMachinePrimitiveValue,
    KMachinePrimitiveError,
    BHelperScalarEntry,
    BHelperScalarTag,
    BHelperScalarArithmetic,
    BHelperScalarConstruct,
    BHelperScalarExit,
    BHelperScalarFailure,
    BHelperEnvGetEntry,
    BHelperEnvGetCell,
    BHelperEnvGetEmpty,
    BHelperEnvGetExit,
    BHelperEnvGetFailure,
    BHelperEnvPutEntry,
    BHelperEnvPutCell,
    BHelperEnvPutInsert,
    BHelperEnvPutEmpty,
    BHelperEnvPutExit,
    BHelperEnvPutFailure,
    BHelperFindEntry,
    BHelperFindRow,
    BHelperFindEmpty,
    BHelperFindExit,
    BHelperFindFailure,
    BHelperBindEntry,
    BHelperBindPair,
    BHelperBindEmpty,
    BHelperBindArity,
    BHelperBindType,
    BHelperBindDuplicate,
    BHelperBindInsert,
    BHelperBindPropagate,
    BHelperBindExit,
    BHelperBindFailure,
    BHelperAppendEntry,
    BHelperAppendCell,
    BHelperAppendEmpty,
    BHelperAppendTail,
    BHelperAppendCopy,
    BHelperAppendExit,
    BHelperAppendFailure,
    BHelperValidateEntry,
    BHelperValidateOccurrence,
    BHelperValidateField,
    BHelperValidateDeclaration,
    BHelperValidateEmpty,
    BHelperValidateInvalid,
    BHelperValidateExit,
    BHelperValidateFailure,
    BHelperEqualEntry,
    BHelperEqualPair,
    BHelperEqualField,
    BHelperEqualEmpty,
    BHelperEqualDifferent,
    BHelperEqualExit,
    BHelperEqualFailure,
    BHelperMatchEntry,
    BHelperMatchArm,
    BHelperMatchOccurrence,
    BHelperMatchField,
    BHelperMatchBinding,
    BHelperMatchDistinct,
    BHelperMatchDiscard,
    BHelperMatchNext,
    BHelperMatchSelected,
    BHelperMatchEmpty,
    BHelperMatchExit,
    BHelperMatchFailure,
    BBookkeepingTaskEnqueue,
    BBookkeepingTaskRelease,
    BBookkeepingResumeStore,
    BBookkeepingResumeRestore,
    KAdminStartupValid,
    KAdminStartupInvalid,
    KAdminPublishValue,
    KAdminPublishError,
    KAdminPublishExhausted,
    XAdtEvalConstructNullary,
    XAdtEvalConstructFields,
    XAdtReturnCtorFieldsTyped,
    XAdtReturnCtorFieldsMistyped,
    XAdtReturnCtorFieldsNext,
    XAdtReturnCtorFieldsFinal,
    XAdtMaterializeDone,
    XAdtCasesPlanHit,
    XAdtCasesPlanMiss,
    XAdtHelperBuildEntry,
    XAdtHelperBuildCell,
    XAdtHelperBuildField,
    XAdtHelperBuildExit,
    XAdtHelperPlanEntry,
    XAdtHelperPlanWalk,
    XAdtHelperPlanSwitch,
    XAdtHelperPlanBind,
    XAdtHelperPlanSelect,
    XAdtHelperPlanNoMatch,
    XAdtHelperPlanExit,
    XAdtHelperPlanFailure,
}

// schema /bounds/fields
pub struct Bounds {
    pub code_rows: usize,
    pub function_rows: usize,
    pub code_child_slots: usize,
    pub parameter_slots: usize,
    pub pattern_rows: usize,
    pub match_plan_rows: usize,
    pub match_arm_rows: usize,
    pub type_rows: usize,
    pub constructor_rows: usize,
    pub constructor_field_slots: usize,
    pub max_arity: usize,
    pub constant_nodes: usize,
    pub constant_field_slots: usize,
    pub data_nodes: usize,
    pub data_field_slots: usize,
    pub environment_cells: usize,
    pub frames: usize,
    pub helper_tasks: usize,
    pub serialization_bytes: usize,
    pub max_reference_work: u128,
    pub max_helper_work: u128,
    pub step_cap: u128,
}

// schema /layout_schema/LayoutCapacities
pub struct LayoutCapacities {
    pub name_rows: usize,
    pub name_bytes: usize,
    pub occurrence_path_slots: usize,
    pub value_slots: usize,
}

// schema /records/ValueRef/variant
pub enum ValueRef {
    Const { id: ConstId },
    Data { id: DataId },
}

// schema /records/ValSlice/record
pub struct ValSlice {
    pub start: ValueSlotPos,
    pub len: ArityCount,
}

// schema /records/ExprSlice/record
pub struct ExprSlice {
    pub start: ChildPos,
    pub len: ArityCount,
}

// schema /records/ArmSlice/record
pub struct ArmSlice {
    pub start: ArmPos,
    pub len: ArmPos,
}

// schema /records/ParamSlice/record
pub struct ParamSlice {
    pub start: ParamPos,
    pub len: ArityCount,
}

// schema /records/FieldSlice/record
pub struct FieldSlice {
    pub start: FieldPos,
    pub len: usize,
}

// schema /records/ConstFieldSlice/record
pub struct ConstFieldSlice {
    pub start: ConstFieldPos,
    pub len: usize,
}

// schema /records/OccPathSlice/record
pub struct OccPathSlice {
    pub start: OccPathPos,
    pub len: OccPathPos,
}

// schema /records/EnvRoot/type
pub type EnvRoot = Option<EnvId>;

// schema /records/EnvCell/record
pub struct EnvCell {
    pub binder: NameId,
    pub value: ValueRef,
    pub previous: Option<EnvId>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct TypeId(pub usize);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CtorId(pub usize);

// schema /records/ConstRow
pub enum ConstRow {
    U128 {
        marker_free: Bool,
        n: UInt128,
    },
    Bool {
        marker_free: Bool,
        b: Bool,
    },
    Nil {
        marker_free: Bool,
    },
    Cons {
        marker_free: Bool,
        fields: ConstFieldSlice,
    },
    Leaf {
        marker_free: Bool,
        fields: ConstFieldSlice,
    },
    Branch {
        marker_free: Bool,
        fields: ConstFieldSlice,
    },
    Constructor {
        marker_free: Bool,
        type_id: TypeId,
        ctor_id: CtorId,
        fields: ConstFieldSlice,
    },
    U128OutOfRange {
        marker_free: Bool,
    },
}

// schema /records/DataRow/variant
pub enum DataRow {
    U128 {
        n: UInt128,
    },
    Bool {
        b: Bool,
    },
    Nil,
    Cons {
        fields: FieldSlice,
    },
    Leaf {
        fields: FieldSlice,
    },
    Branch {
        fields: FieldSlice,
    },
    Constructor {
        type_id: TypeId,
        ctor_id: CtorId,
        fields: FieldSlice,
    },
}

// schema /records/FieldSlot/type
pub type FieldSlot = ValueRef;

// schema /records/ValueSlot/type
pub type ValueSlot = ValueRef;

// schema /records/ControlRow/BoundBinding/variant
pub enum BoundBinding {
    Bad,
    Ok { env: EnvRoot },
}

// schema /records/ControlRow/CaseMatching/variant
pub enum CaseMatching {
    NoMatch,
    Matched { env: EnvRoot },
}

// schema /records/ControlRow/PrimOutcome/variant
pub enum PrimOutcome {
    Value { value: ValueRef },
    Error { code: ErrorCode },
}

// schema /records/HelperControl/ScalarStaged/variant
pub enum ScalarStaged {
    None,
    U128Value { n: UInt128 },
    BoolValue { b: Bool },
    PendingNode { tag: NodeKind },
    PrimError { code: ErrorCode },
    Selector { sel: BoolSelector },
    TypeTest { ok: Bool },
}

// schema /records/HelperTask/variants
pub enum HelperTask {
    ValidateTask {
        r#ref: ValueRef,
        visit: TaskVisit,
        cursor: ArityCount,
    },
    EqualTask {
        left: ValueRef,
        right: ValueRef,
        visit: TaskVisit,
        cursor: ArityCount,
    },
    PairTask {
        binder: NameId,
        sort: SortTag,
        arg: ValueRef,
    },
}

// schema /records/HelperResult/variants
pub enum HelperResult {
    Validated { ok: Bool },
    EnvFound { value: ValueRef },
    EnvMissing,
    EnvUpdated { cell: EnvId },
    FindFound { function: FunctionRowId },
    FindMissing,
    BindOk { env: EnvRoot },
    BindBad,
    AppendDone { seq: ValSlice },
    ScalarValue { value: ValueRef },
    ScalarError { code: ErrorCode },
    ScalarSelector { sel: BoolSelector },
    ScalarTypeTest { ok: Bool },
    EqualDone { value: ValueRef },
    MatchMatched { env: EnvRoot },
    MatchNoMatch,
}

// schema /records/Outcome/variants
pub enum Outcome {
    Running,
    Halted { value: ValueRef },
    Rejected { code: ErrorCode },
    Exhausted { reason: ExhaustReason },
}

// schema /program_image_view/tables/prog.expressions/row/variant
pub enum ExpressionRow {
    Literal {
        constant: ConstId,
    },
    Var {
        binder: NameId,
    },
    Let {
        binder: NameId,
        value: ExprId,
        body: ExprId,
    },
    Binary {
        op: BinaryOp,
        left: ExprId,
        right: ExprId,
    },
    Unary {
        op: UnaryOp,
        operand: ExprId,
    },
    If {
        cond: ExprId,
        then_expr: ExprId,
        else_expr: ExprId,
    },
    Call {
        function_name: NameId,
        args: ExprSlice,
    },
    Match {
        scrutinee: ExprId,
        arms: ArmSlice,
    },
}

// schema /identity/SourceOccurrenceId/record/owner
pub enum OccurrenceOwner {
    Entry,
    Function { index: FunctionRowId },
}

// schema /program_image_view/tables/prog.occurrences/row
pub struct OccurrenceRow {
    pub owner: OccurrenceOwner,
    pub path: OccPathSlice,
}

// schema /program_image_view/tables/prog.functions/row
pub struct FunctionRow {
    pub function_name: NameId,
    pub params: ParamSlice,
    pub return_type: SortTag,
    pub body: ExprId,
}

// schema /program_image_view/tables/prog.parameters/row
pub struct ParameterRow {
    pub binder: NameId,
    pub sort: SortTag,
}

// schema /program_image_view/tables/prog.arms/row
pub struct ArmRow {
    pub pattern: PatternRowId,
    pub body: ExprId,
}

// schema /program_image_view/tables/prog.patterns/row/variant
pub enum PatternRow {
    PVar { x: NameId },
    PNil,
    PCons { x: NameId, y: NameId },
    PLeaf { x: NameId },
    PBranch { x: NameId, y: NameId },
}

// schema /program_image_view/tables/prog.names/row
pub struct NameRow {
    pub bytes: Text,
}

// schema /core_program_input/CorePattern/variant
pub enum CorePattern {
    PVar { x: Text },
    PNil,
    PCons { x: Text, y: Text },
    PLeaf { x: Text },
    PBranch { x: Text, y: Text },
}

// schema /program_image_view/header/activeLengths
pub struct ProgramActiveLengths {
    pub expressions: usize,
    pub occurrences: usize,
    pub occurrence_paths: usize,
    pub children: usize,
    pub functions: usize,
    pub parameters: usize,
    pub arms: usize,
    pub patterns: usize,
    pub constants: usize,
    pub constant_fields: usize,
    pub names: usize,
}

// schema /program_image_view/header
pub struct ProgramHeader {
    pub entry: ExprId,
    pub active_lengths: ProgramActiveLengths,
}

// schema /identity/Identity/record
pub struct Identity {
    pub profile_id: ProfileId,
    pub grammar_domain: GrammarDomain,
    pub finance: FinanceInput,
    pub layout_version: Id,
    pub layout_digest: Digest,
    pub bounds_digest: Digest,
    pub field_id: Option<Id>,
    pub program_membership_digest: Digest,
}

// schema /identity/RunId/record
pub struct RunId {
    pub identity: Identity,
    pub fuel_input: RunInt,
    pub stack_limit_input: RunInt,
}

// schema /identity/SourceOccurrenceId/record
pub struct SourceOccurrenceId {
    pub program_source_digest: Digest,
    pub owner: OccurrenceOwner,
    pub ast_path: Vec<ChildPosition>,
}
