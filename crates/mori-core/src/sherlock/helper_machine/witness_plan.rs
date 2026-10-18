//! Deterministic host materialization of existing B observations.
//!
//! This read-only plan does not evaluate source expressions, select owner rules,
//! modify an image, emit constraints or authenticate supplied snapshots. Plans
//! bind exact immutable observation inputs. Existing uncached entrypoints execute
//! every node once per call. Optional memoization and last-use/streaming APIs
//! retain the same operations and complete raw/decoded boundaries, with separate
//! logical host diagnostics. Physical allocation/copy/time costs and H1 remain
//! unmeasured; no native, resource, A/B-clock or correspondence claim is made.
use super::complete_observation::{
    AdmissionSnapshot, ConstantSnapshot, DataSnapshot, ExpressionSnapshot, HostSnapshot,
};
use super::shared::*;
use super::shared::host_implied_prefix::SegBase;

#[path = "witness_plan/dag.rs"]
mod dag;
#[path = "witness_plan/decode.rs"]
mod decode;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct NodeId(pub usize);
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BindingId(pub usize);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LifetimeUnit { Snapshot, RecordBefore, RecordAfter }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Location {
    pub section: &'static str,
    pub row: Option<usize>,
    pub field: &'static str,
}
impl Location {
    pub fn new(section: &'static str, row: Option<usize>, field: &'static str) -> Self {
        Self { section, row, field }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FailureKind {
    MissingInput, InputChanged, IndexOverflow, CyclicValue, CyclicEnvironment,
    WrongArity { expected: usize, actual: usize },
    WrongValueSort { expected: SortTag, actual: Option<SortTag> },
    MissingDependency { dependency: NodeId },
    WrongType { expected: ValueType, actual: ValueType },
    InvalidNodeId, CyclicDependencies, UnstableOrder, IncompleteBindings,
    IncompleteOutputs, WrongOperation,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WitnessFailure {
    pub location: Location,
    pub node: Option<NodeId>,
    pub kind: FailureKind,
}
pub(super) fn fail(location: Location, kind: FailureKind) -> WitnessFailure {
    WitnessFailure { location, node: None, kind }
}

/// One flat, fully unfolded occurrence tree. Child indices precede parents;
/// repeated raw references produce repeated semantic occurrences, not sharing.
/// Vec ownership avoids recursive construction, traversal and recursive Drop.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValueNode {
    UInt(u128), Bool(bool), Nil, U128OutOfRange,
    Cons { head: usize, tail: usize }, Leaf { value: usize },
    Branch { left: usize, right: usize },
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DecodedValue { pub nodes: Vec<ValueNode>, pub root: usize }
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EnvironmentEntry {
    pub binder: usize,
    pub name: String,
    pub value: DecodedValue,
}
/// Exact newest-to-oldest chain, retaining duplicate binders and shadowed cells.
/// An empty successful binding remains distinct from a failed BoundBinding.
pub type DecodedEnvironment = Vec<EnvironmentEntry>;

/// Full non-table image fields; tables have explicit separate output bindings.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Metadata {
    pub admission: AdmissionSnapshot,
    pub run_id: RunIdC,
    pub control_origin: usize,
    pub env_root: EnvRootC,
    pub stack_limit: usize,
    pub initial_fuel: u128,
    pub remaining_work: u128,
    pub consumed_work: u128,
    pub helper_work: u128,
    pub helper_visits: u128,
    pub reference_cursor: u128,
    pub alloc: AllocCounters,
    pub active: ActiveLengths,
    pub seg_base: SegBase,
    /// data/data_fields/env/frames/tasks/value_slots, in that order.
    pub table_lengths: [usize; 6],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum InputPath {
    Metadata, Constant(usize), ConstantField(usize), Data(usize), DataField(usize),
    Environment(usize), Name(usize), Expression(usize), Function(usize),
    Parameter(usize), Arm(usize), Pattern(usize), Occurrence(usize),
    OccurrencePath(usize), Child(usize), Frame(usize), ScheduledFrame,
    Control, Helper, Task(usize), Result, Outcome, ValueSlot(usize),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SourceType {
    Metadata, Constant, Data, Reference, Environment, Name, Expression, Function,
    Parameter, Arm, Pattern, Occurrence, Index, Frame, Control, Helper, Task,
    Result, Outcome,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InputValue {
    Metadata(Metadata), Constant(ConstantSnapshot), Data(DataSnapshot),
    Reference(RefC), Environment(EnvCellC), Name(String),
    Expression(ExpressionSnapshot), Function(FunctionRowC), Parameter(ParameterRowC),
    Arm(ArmRowC), Pattern(PatternRowC), Occurrence(OccurrenceRowC), Index(usize),
    Frame(FrameRowC), Control(ControlC), Helper(Option<HelperControlC>),
    Task(HelperTaskC), Result(Option<HelperResultC>), Outcome(OutcomeC),
}
impl InputValue {
    pub fn source_type(&self) -> SourceType {
        match self {
            Self::Metadata(_) => SourceType::Metadata, Self::Constant(_) => SourceType::Constant,
            Self::Data(_) => SourceType::Data, Self::Reference(_) => SourceType::Reference,
            Self::Environment(_) => SourceType::Environment, Self::Name(_) => SourceType::Name,
            Self::Expression(_) => SourceType::Expression, Self::Function(_) => SourceType::Function,
            Self::Parameter(_) => SourceType::Parameter, Self::Arm(_) => SourceType::Arm,
            Self::Pattern(_) => SourceType::Pattern, Self::Occurrence(_) => SourceType::Occurrence,
            Self::Index(_) => SourceType::Index, Self::Frame(_) => SourceType::Frame,
            Self::Control(_) => SourceType::Control, Self::Helper(_) => SourceType::Helper,
            Self::Task(_) => SourceType::Task, Self::Result(_) => SourceType::Result,
            Self::Outcome(_) => SourceType::Outcome,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PayloadKind { Frame, Control, Helper, Task, Result, Outcome }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SequenceKind { Values, Environments, Frames, Tasks, Names, Expressions, Parameters, Arms, Patterns, Occurrences, Indices, References }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValueType {
    Source(SourceType), Value, Environment, Payload(PayloadKind),
    Sequence(SequenceKind), Snapshot,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NamedValue { pub field: &'static str, pub value: SlotValue }
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DecodedPayload {
    pub kind: PayloadKind,
    /// Exact tag and all scalar/cursor/origin/status fields, never normalized.
    pub raw: InputValue,
    pub fields: Vec<NamedValue>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DecodedSnapshot { pub metadata: Metadata, pub fields: Vec<NamedValue> }
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SlotValue {
    Source(InputValue), Value(DecodedValue), Environment(DecodedEnvironment),
    Payload(DecodedPayload), Sequence { kind: SequenceKind, items: Vec<SlotValue> },
    Snapshot(DecodedSnapshot),
}
impl SlotValue {
    pub fn value_type(&self) -> ValueType {
        match self {
            Self::Source(value) => ValueType::Source(value.source_type()),
            Self::Value(_) => ValueType::Value, Self::Environment(_) => ValueType::Environment,
            Self::Payload(value) => ValueType::Payload(value.kind),
            Self::Sequence { kind, .. } => ValueType::Sequence(*kind), Self::Snapshot(_) => ValueType::Snapshot,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InputBinding {
    pub id: BindingId,
    pub path: InputPath,
    pub value_type: SourceType,
    pub immutable_value: InputValue,
    pub location: Location,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Dependency { pub node: NodeId, pub value_type: ValueType }
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldBinding { pub field: &'static str, pub dependency: Dependency }
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Operation {
    Read(BindingId),
    /// A raw member/reference and its resolved target, in that order.
    Bind,
    /// Row, then ordered (field-reference, decoded-child) pairs.
    Value { source: RefC, arity: usize },
    EmptyEnvironment,
    /// Environment-cell, name, decoded-value, previous-environment.
    Environment,
    Sequence { kind: SequenceKind, item_type: ValueType },
    /// First dependency is the exact raw payload; fields are remaining inputs.
    Payload { kind: PayloadKind, fields: Vec<FieldBinding> },
    /// First dependency is metadata; every remaining output is named explicitly.
    Snapshot { fields: Vec<FieldBinding> },
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Node {
    pub id: NodeId,
    pub location: Location,
    pub operation: Operation,
    pub dependencies: Vec<Dependency>,
    pub output_type: ValueType,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Definition {
    pub value: NodeId,
    pub producer: NodeId,
    /// Ordered consumer edges, including repeated uses within one consumer.
    pub consumers: Vec<NodeId>,
    pub last_use_order: usize,
    pub lifetime: LifetimeUnit,
    pub live_out: bool,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WitnessPlan {
    pub format: &'static str,
    pub lifetime: LifetimeUnit,
    pub inputs: Vec<InputBinding>,
    pub nodes: Vec<Node>,
    pub stable_topological_order: Vec<NodeId>,
    pub definitions: Vec<Definition>,
    pub live_outs: Vec<NodeId>,
    pub output: NodeId,
}
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct HostDiagnostics {
    pub evaluated_nodes: usize,
    pub input_reads: usize,
    pub value_constructions: usize,
    pub environment_constructions: usize,
    pub payload_constructions: usize,
    /// Only child nodes copied within Value operations, not all host clones.
    pub copied_value_nodes: usize,
    /// Only prior-chain entries copied within Environment operations.
    pub copied_environment_entries: usize,
    /// Scratch slots at publication: all nodes in the uncached baseline,
    /// potentially fewer after last-use. Not allocated bytes/prover memory.
    pub retained_slots: usize,
    pub measured_host_bytes: Option<usize>,
    pub measured_host_time_ns: Option<u128>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WitnessEvaluation {
    pub decoded: DecodedSnapshot,
    pub diagnostics: HostDiagnostics,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MaterializedSnapshot {
    pub raw: HostSnapshot,
    pub plan: WitnessPlan,
    pub evaluation: WitnessEvaluation,
}

pub fn construct_plan(input: &HostSnapshot) -> Result<WitnessPlan, WitnessFailure> {
    construct_plan_for(input, LifetimeUnit::Snapshot)
}
pub fn construct_plan_for(input: &HostSnapshot, lifetime: LifetimeUnit)
    -> Result<WitnessPlan, WitnessFailure>
{
    decode::build(input, lifetime)
}
pub fn evaluate_uncached(plan: &WitnessPlan, input: &HostSnapshot)
    -> Result<WitnessEvaluation, WitnessFailure>
{
    dag::evaluate(plan, input)
}
pub fn materialize(input: &HostSnapshot) -> Result<MaterializedSnapshot, WitnessFailure> {
    materialize_for(input, LifetimeUnit::Snapshot)
}
pub fn materialize_for(input: &HostSnapshot, lifetime: LifetimeUnit)
    -> Result<MaterializedSnapshot, WitnessFailure>
{
    let plan = construct_plan_for(input, lifetime)?;
    let evaluation = evaluate_uncached(&plan, input)?;
    Ok(MaterializedSnapshot { raw: input.clone(), plan, evaluation })
}

// Optional host scheduling strategies reuse the same validated DAG operations.
#[path = "witness_plan/strategy.rs"]
mod strategy;
pub use strategy::{
    evaluate_last_use, evaluate_memoized, evaluate_with_storage,
    SlotStorage, StrategyDiagnostics, StrategyEvaluation, WitnessCache,
};
#[path = "witness_plan/stream.rs"]
mod stream;
pub use stream::{
    materialize_with_storage, record_step_with_storage, stream_owner_steps,
    StrategyMaterializedSnapshot, StrategyObservedStep, StrategyStepError,
    WitnessRecordCache, StreamAction, StreamStop,
};
