//! Owned complete B host records around the existing fixed owner step.
//!
//! A diagnostic observer, not a new engine, guard, codec or transition validator.
//! Every snapshot copies the actual owner-bound observation. Admission/source,
//! native representation and source/transition authenticity are not established.
//! Host allocation and copying costs are unmeasured and outside B work counters.
use super::raw::{ConstRow, DataRow};
use super::shared::*;
use super::shared::host_implied_prefix::SegBase;
use super::shared::owner_host::{host_owner_engine, observe_bound_image, HostObservation};

fn clone_type_expr_iterative(root: &owner_ext_type_builder::TypeExprSrc) -> owner_ext_type_builder::TypeExprSrc {
    use owner_ext_type_builder::TypeExprSrc;
    enum Frame<'a> {
        Enter(&'a TypeExprSrc),
        BuildApply { name: String, arity: usize },
    }
    let mut work: Vec<Frame<'_>> = vec![Frame::Enter(root)];
    let mut out: Vec<TypeExprSrc> = Vec::new();
    while let Some(frame) = work.pop() {
        match frame {
            Frame::Enter(TypeExprSrc::UInt) => out.push(TypeExprSrc::UInt),
            Frame::Enter(TypeExprSrc::Bool) => out.push(TypeExprSrc::Bool),
            Frame::Enter(TypeExprSrc::List) => out.push(TypeExprSrc::List),
            Frame::Enter(TypeExprSrc::Tree) => out.push(TypeExprSrc::Tree),
            Frame::Enter(TypeExprSrc::Named { name }) => out.push(TypeExprSrc::Named { name: name.clone() }),
            Frame::Enter(TypeExprSrc::Param { name }) => out.push(TypeExprSrc::Param { name: name.clone() }),
            Frame::Enter(TypeExprSrc::Apply { name, args }) => {
                work.push(Frame::BuildApply { name: name.clone(), arity: args.len() });
                for arg in args.iter().rev() {
                    work.push(Frame::Enter(arg));
                }
            }
            Frame::BuildApply { name, arity } => {
                let split = out.len().saturating_sub(arity);
                let args = out.split_off(split);
                out.push(TypeExprSrc::Apply { name, args });
            }
        }
    }
    out.pop().unwrap_or(TypeExprSrc::UInt)
}

fn eq_type_expr_iterative(a: &owner_ext_type_builder::TypeExprSrc, b: &owner_ext_type_builder::TypeExprSrc) -> bool {
    use owner_ext_type_builder::TypeExprSrc;
    let mut stack: Vec<(&TypeExprSrc, &TypeExprSrc)> = vec![(a, b)];
    while let Some((x, y)) = stack.pop() {
        match (x, y) {
            (TypeExprSrc::UInt, TypeExprSrc::UInt)
            | (TypeExprSrc::Bool, TypeExprSrc::Bool)
            | (TypeExprSrc::List, TypeExprSrc::List)
            | (TypeExprSrc::Tree, TypeExprSrc::Tree) => {}
            (TypeExprSrc::Named { name: nx }, TypeExprSrc::Named { name: ny })
            | (TypeExprSrc::Param { name: nx }, TypeExprSrc::Param { name: ny }) => {
                if nx != ny { return false; }
            }
            (TypeExprSrc::Apply { name: nx, args: ax }, TypeExprSrc::Apply { name: ny, args: ay }) => {
                if nx != ny || ax.len() != ay.len() { return false; }
                for (cx, cy) in ax.iter().zip(ay.iter()).rev() {
                    stack.push((cx, cy));
                }
            }
            _ => return false,
        }
    }
    true
}

fn fmt_type_expr_iterative(root: &owner_ext_type_builder::TypeExprSrc, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    use owner_ext_type_builder::TypeExprSrc;
    enum FmtFrame<'a> {
        Enter(&'a TypeExprSrc),
        Text(&'static str),
    }
    let mut stack: Vec<FmtFrame<'_>> = vec![FmtFrame::Enter(root)];
    while let Some(frame) = stack.pop() {
        match frame {
            FmtFrame::Text(s) => f.write_str(s)?,
            FmtFrame::Enter(TypeExprSrc::UInt) => f.write_str("UInt")?,
            FmtFrame::Enter(TypeExprSrc::Bool) => f.write_str("Bool")?,
            FmtFrame::Enter(TypeExprSrc::List) => f.write_str("List")?,
            FmtFrame::Enter(TypeExprSrc::Tree) => f.write_str("Tree")?,
            FmtFrame::Enter(TypeExprSrc::Named { name }) => write!(f, "Named({:?})", name)?,
            FmtFrame::Enter(TypeExprSrc::Param { name }) => write!(f, "Param({:?})", name)?,
            FmtFrame::Enter(TypeExprSrc::Apply { name, args }) => {
                write!(f, "Apply({:?}, [", name)?;
                stack.push(FmtFrame::Text("])"));
                for (idx, arg) in args.iter().enumerate().rev() {
                    if idx > 0 {
                        stack.push(FmtFrame::Enter(arg));
                        stack.push(FmtFrame::Text(", "));
                    } else {
                        stack.push(FmtFrame::Enter(arg));
                    }
                }
            }
        }
    }
    Ok(())
}

impl Clone for owner_ext_type_builder::TypeExprSrc {
    fn clone(&self) -> Self { clone_type_expr_iterative(self) }
}
impl PartialEq for owner_ext_type_builder::TypeExprSrc {
    fn eq(&self, other: &Self) -> bool { eq_type_expr_iterative(self, other) }
}
impl Eq for owner_ext_type_builder::TypeExprSrc {}
impl std::fmt::Debug for owner_ext_type_builder::TypeExprSrc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { fmt_type_expr_iterative(self, f) }
}

impl Clone for owner_ext_type_builder::CtorFieldSrc {
    fn clone(&self) -> Self { Self { name: self.name.clone(), ty: clone_type_expr_iterative(&self.ty) } }
}
impl PartialEq for owner_ext_type_builder::CtorFieldSrc {
    fn eq(&self, other: &Self) -> bool { self.name == other.name && eq_type_expr_iterative(&self.ty, &other.ty) }
}
impl Eq for owner_ext_type_builder::CtorFieldSrc {}
impl std::fmt::Debug for owner_ext_type_builder::CtorFieldSrc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CtorFieldSrc").field("name", &self.name).field("ty", &self.ty).finish()
    }
}

impl Clone for owner_ext_type_builder::CtorDeclSrc {
    fn clone(&self) -> Self { Self { name: self.name.clone(), fields: self.fields.clone() } }
}
impl PartialEq for owner_ext_type_builder::CtorDeclSrc {
    fn eq(&self, other: &Self) -> bool { self.name == other.name && self.fields == other.fields }
}
impl Eq for owner_ext_type_builder::CtorDeclSrc {}
impl std::fmt::Debug for owner_ext_type_builder::CtorDeclSrc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CtorDeclSrc").field("name", &self.name).field("fields", &self.fields).finish()
    }
}

impl Clone for owner_ext_type_builder::TypeDeclSrc {
    fn clone(&self) -> Self {
        Self { name: self.name.clone(), parameters: self.parameters.clone(), ctors: self.ctors.clone() }
    }
}
impl PartialEq for owner_ext_type_builder::TypeDeclSrc {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.parameters == other.parameters && self.ctors == other.ctors
    }
}
impl Eq for owner_ext_type_builder::TypeDeclSrc {}
impl std::fmt::Debug for owner_ext_type_builder::TypeDeclSrc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypeDeclSrc")
            .field("name", &self.name)
            .field("parameters", &self.parameters)
            .field("ctors", &self.ctors)
            .finish()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WidthsSnapshot {
    pub index_widths: Vec<(String, u32)>,
    pub counter_widths: Vec<(String, u32)>,
    pub record_widths: Vec<(String, u32)>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LayoutSnapshot {
    pub layout_version: (String, String),
    pub field_id: Option<(String, String)>,
    pub widths: Option<WidthsSnapshot>,
    pub scalar_layout_id: Option<(String, String)>,
    pub bounds: BoundsC,
    pub capacities: LayoutCapacitiesC,
    pub boundary_capacities_present: bool,
}

/// Same field/variant payloads as actual ExpressionRowC; no AST evaluation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExpressionSnapshot {
    Literal { constant: usize },
    Var { binder: usize },
    Let { binder: usize, value: usize, body: usize },
    Binary { op: BinaryOp, left: usize, right: usize },
    Unary { op: UnaryOp, operand: usize },
    If { cond: usize, then_expr: usize, else_expr: usize },
    Call { function_name: usize, args: SliceC },
    Match { scrutinee: usize, arms: SliceC, plan: Option<usize> },
    Construct { type_id: TypeIdC, ctor_id: adt::CtorIdC, fields: SliceC, constant: Option<usize> },
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ConstFieldsSnapshot { pub start: usize, pub len: usize }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DataFieldsSnapshot { pub start: usize, pub len: usize }
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConstantSnapshot {
    U128 { marker_free: bool, n: u128 },
    Bool { marker_free: bool, b: bool },
    Nil { marker_free: bool },
    Cons { marker_free: bool, fields: ConstFieldsSnapshot },
    Leaf { marker_free: bool, fields: ConstFieldsSnapshot },
    Branch { marker_free: bool, fields: ConstFieldsSnapshot },
    Constructor { marker_free: bool, type_id: TypeIdC, ctor_id: adt::CtorIdC, fields: ConstFieldsSnapshot },
    U128OutOfRange { marker_free: bool },
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DataSnapshot {
    U128 { n: u128 },
    Bool { b: bool },
    Nil,
    Cons { fields: DataFieldsSnapshot },
    Leaf { fields: DataFieldsSnapshot },
    Branch { fields: DataFieldsSnapshot },
    Constructor { type_id: TypeIdC, ctor_id: adt::CtorIdC, fields: DataFieldsSnapshot },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProgramSnapshot {
    pub entry: usize,
    pub expressions: Vec<ExpressionSnapshot>,
    pub occurrences: Vec<OccurrenceRowC>,
    pub occurrence_paths: Vec<usize>,
    pub children: Vec<usize>,
    pub functions: Vec<FunctionRowC>,
    pub parameters: Vec<ParameterRowC>,
    pub arms: Vec<ArmRowC>,
    pub patterns: Vec<PatternRowC>,
    pub pattern_children: Vec<usize>,
    pub types: Vec<adt::TypeRowC>,
    pub type_args: Vec<TypeIdC>,
    pub constructors: Vec<adt_storage::CtorRowExt>,
    pub ctor_fields: Vec<adt_storage::CtorFieldRowExt>,
    pub decl_param_counts: Vec<usize>,
    pub decl_ctor_counts: Vec<usize>,
    pub decl_templates: Vec<owner_ext_type_builder::TypeDeclSrc>,
    pub plan_nodes: Vec<PlanNodeC>,
    pub plan_cases: Vec<Option<usize>>,
    pub field_paths: Vec<usize>,
    pub constants: Vec<ConstantSnapshot>,
    pub constant_fields: Vec<usize>,
    pub names: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdmissionSnapshot {
    pub identity: IdentityC,
    pub bounds: BoundsC,
    pub view: ProgramSnapshot,
    pub layout: LayoutSnapshot,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImageSnapshot {
    pub run_id: RunIdC,
    pub control: ControlC,
    pub control_origin: usize,
    pub env_root: EnvRootC,
    pub stack_limit: usize,
    pub outcome: OutcomeC,
    pub initial_fuel: u128,
    pub remaining_work: u128,
    pub consumed_work: u128,
    pub helper_work: u128,
    pub helper_visits: u128,
    pub reference_cursor: u128,
    pub alloc: AllocCounters,
    pub active: ActiveLengths,
    pub helper: Option<HelperControlC>,
    pub result: Option<HelperResultC>,
    pub seg_base: SegBase,
    pub data: Vec<DataSnapshot>,
    pub data_fields: Vec<RefC>,
    pub env: Vec<EnvCellC>,
    pub frames: Vec<FrameRowC>,
    pub tasks: Vec<HelperTaskC>,
    pub value_slots: Vec<RefC>,
}
impl ImageSnapshot {
    /// Derived exactly as the existing observation; no duplicate stored counter.
    pub fn depth(&self) -> usize { self.frames.len() }
    pub fn task_live(&self) -> usize { self.tasks.len() }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HostSnapshot {
    pub admission: AdmissionSnapshot,
    pub image: ImageSnapshot,
}

fn expression(value: &ExpressionRowC) -> ExpressionSnapshot {
    match value {
        ExpressionRowC::Literal { constant } => ExpressionSnapshot::Literal { constant:*constant },
        ExpressionRowC::Var { binder } => ExpressionSnapshot::Var { binder:*binder },
        ExpressionRowC::Let { binder, value, body } => ExpressionSnapshot::Let { binder:*binder, value:*value, body:*body },
        ExpressionRowC::Binary { op, left, right } => ExpressionSnapshot::Binary { op:*op, left:*left, right:*right },
        ExpressionRowC::Unary { op, operand } => ExpressionSnapshot::Unary { op:*op, operand:*operand },
        ExpressionRowC::If { cond, then_expr, else_expr } => ExpressionSnapshot::If { cond:*cond, then_expr:*then_expr, else_expr:*else_expr },
        ExpressionRowC::Call { function_name, args } => ExpressionSnapshot::Call { function_name:*function_name, args:*args },
        ExpressionRowC::Match { scrutinee, arms, plan } => ExpressionSnapshot::Match { scrutinee:*scrutinee, arms:*arms, plan:*plan },
        ExpressionRowC::Construct { type_id, ctor_id, fields, constant } => ExpressionSnapshot::Construct {
            type_id:*type_id, ctor_id:*ctor_id, fields:*fields, constant:*constant },
    }
}
fn constant(value: &ConstRow) -> ConstantSnapshot {
    match value {
        ConstRow::U128 { marker_free, n } => ConstantSnapshot::U128 { marker_free:*marker_free, n:*n },
        ConstRow::Bool { marker_free, b } => ConstantSnapshot::Bool { marker_free:*marker_free, b:*b },
        ConstRow::Nil { marker_free } => ConstantSnapshot::Nil { marker_free:*marker_free },
        ConstRow::Cons { marker_free, fields } => ConstantSnapshot::Cons { marker_free:*marker_free,
            fields:ConstFieldsSnapshot { start:fields.start.0, len:fields.len } },
        ConstRow::Leaf { marker_free, fields } => ConstantSnapshot::Leaf { marker_free:*marker_free,
            fields:ConstFieldsSnapshot { start:fields.start.0, len:fields.len } },
        ConstRow::Branch { marker_free, fields } => ConstantSnapshot::Branch { marker_free:*marker_free,
            fields:ConstFieldsSnapshot { start:fields.start.0, len:fields.len } },
        ConstRow::Constructor { marker_free, type_id, ctor_id, fields } => ConstantSnapshot::Constructor {
            marker_free:*marker_free, type_id:TypeIdC(type_id.0), ctor_id:adt::CtorIdC(ctor_id.0),
            fields:ConstFieldsSnapshot { start:fields.start.0, len:fields.len } },
        ConstRow::U128OutOfRange { marker_free } => ConstantSnapshot::U128OutOfRange { marker_free:*marker_free },
    }
}
fn data(value: &DataRow) -> DataSnapshot {
    match value {
        DataRow::U128 { n } => DataSnapshot::U128 { n:*n },
        DataRow::Bool { b } => DataSnapshot::Bool { b:*b },
        DataRow::Nil => DataSnapshot::Nil,
        DataRow::Cons { fields } => DataSnapshot::Cons { fields:DataFieldsSnapshot { start:fields.start.0, len:fields.len } },
        DataRow::Leaf { fields } => DataSnapshot::Leaf { fields:DataFieldsSnapshot { start:fields.start.0, len:fields.len } },
        DataRow::Branch { fields } => DataSnapshot::Branch { fields:DataFieldsSnapshot { start:fields.start.0, len:fields.len } },
        DataRow::Constructor { type_id, ctor_id, fields } => DataSnapshot::Constructor {
            type_id:TypeIdC(type_id.0), ctor_id:adt::CtorIdC(ctor_id.0),
            fields:DataFieldsSnapshot { start:fields.start.0, len:fields.len } },
    }
}

/// Copy every public field of the borrowed, bound host observation.
/// Private zero-sized _sealed markers are construction markers, not observed data.
/// Vec order/multiplicity, Option tags, raw references and marker_free remain exact.
pub fn copy_observation(value: HostObservation<'_>) -> HostSnapshot {
    let admission = value.admission;
    let layout = admission.layout;
    let view = admission.view;
    let image = value.image;
    let decl_templates = view.decl_templates.iter().map(|d| owner_ext_type_builder::TypeDeclSrc {
        name: d.name.clone(),
        parameters: d.parameters.clone(),
        ctors: d.ctors.iter().map(|c| owner_ext_type_builder::CtorDeclSrc {
            name: c.name.clone(),
            fields: c.fields.iter().map(|f| owner_ext_type_builder::CtorFieldSrc {
                name: f.name.clone(),
                ty: clone_type_expr_iterative(&f.ty),
            }).collect(),
        }).collect(),
    }).collect();
    HostSnapshot {
        admission:AdmissionSnapshot {
            identity:(*admission.identity).clone(), bounds:*admission.bounds,
            view:ProgramSnapshot {
                entry:view.entry, expressions:view.expressions.iter().map(expression).collect(),
                occurrences:view.occurrences.clone(), occurrence_paths:view.occurrence_paths.clone(),
                children:view.children.clone(), functions:view.functions.clone(),
                parameters:view.parameters.clone(), arms:view.arms.clone(), patterns:view.patterns.clone(),
                pattern_children:view.pattern_children.clone(), types:view.types.clone(),
                type_args:view.type_args.clone(), constructors:view.constructors.clone(),
                ctor_fields:view.ctor_fields.clone(), decl_param_counts:view.decl_param_counts.clone(),
                decl_ctor_counts:view.decl_ctor_counts.clone(), decl_templates,
                plan_nodes:view.plan_nodes.clone(), plan_cases:view.plan_cases.clone(),
                field_paths:view.field_paths.clone(),
                constants:view.constants.iter().map(constant).collect(),
                constant_fields:view.constant_fields.clone(), names:view.names.clone(),
            },
            layout:LayoutSnapshot {
                layout_version:(*layout.layout_version).clone(), field_id:layout.field_id.cloned(),
                widths:layout.widths.map(|widths| WidthsSnapshot {
                    index_widths:widths.index_widths.to_vec(), counter_widths:widths.counter_widths.to_vec(),
                    record_widths:widths.record_widths.to_vec(),
                }),
                scalar_layout_id:layout.scalar_layout_id.cloned(), bounds:*layout.bounds,
                capacities:*layout.capacities, boundary_capacities_present:layout.boundary_capacities_present,
            },
        },
        image:ImageSnapshot {
            run_id:(*image.run_id).clone(), control:*image.control, control_origin:image.control_origin,
            env_root:image.env_root, stack_limit:image.stack_limit, outcome:*image.outcome,
            initial_fuel:image.initial_fuel, remaining_work:image.remaining_work,
            consumed_work:image.consumed_work, helper_work:image.helper_work,
            helper_visits:image.helper_visits, reference_cursor:image.reference_cursor,
            alloc:*image.alloc, active:*image.active, helper:image.helper.copied(), result:image.result.copied(),
            seg_base:image.seg_base, data:image.data.iter().map(data).collect(),
            data_fields:image.data_fields.to_vec(), env:image.env.to_vec(), frames:image.frames.to_vec(),
            tasks:image.tasks.to_vec(), value_slots:image.value_slots.to_vec(),
        },
    }
}

pub fn capture(admitted: &Admitted, image: &MachineImage) -> Result<HostSnapshot, HostViolation> {
    observe_bound_image(admitted, image).map(copy_observation)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CompleteStepRecord {
    pub before: HostSnapshot,
    /// The actual complete owner event, including all counters/origins/locators.
    pub event: StepEventC,
    pub after: HostSnapshot,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ObservedStep {
    Event(CompleteStepRecord),
    /// Actual Terminal result has no fabricated event or inferred mutation.
    Terminal { before: HostSnapshot, after: HostSnapshot },
}
pub enum CaptureStepError {
    BeforeObservation(HostViolation),
    /// The unchanged owner failure plus both available actual boundaries.
    Step {
        violation: HostViolation,
        before: Box<HostSnapshot>,
        after: Result<Box<HostSnapshot>, HostViolation>,
    },
    /// A real event/terminal disposition occurred, but no complete post record exists.
    AfterObservation {
        violation: HostViolation,
        before: Box<HostSnapshot>,
        event: Option<StepEventC>,
    },
}

/// Capture owned pre-state before mutation, call the existing fixed owner once,
/// then observe/copy actual post-state. This explicit stepping API consumes the
/// caller's existing host-execution dependency; source integration does not admit it.
/// No sink, expected data, new dispatcher, clock conversion or run loop is involved.
pub fn record_step(admitted: &Admitted, image: &mut MachineImage)
    -> Result<ObservedStep, CaptureStepError>
{
    let before = capture(admitted, image).map_err(CaptureStepError::BeforeObservation)?;
    let engine = host_owner_engine();
    match engine.step(admitted, image) {
        Ok(StepResult::Event { event }) => {
            match capture(admitted, image) {
                Ok(after) => Ok(ObservedStep::Event(CompleteStepRecord { before, event, after })),
                Err(violation) => Err(CaptureStepError::AfterObservation {
                    violation, before:Box::new(before), event:Some(event),
                }),
            }
        }
        Ok(StepResult::Terminal) => {
            match capture(admitted, image) {
                Ok(after) => Ok(ObservedStep::Terminal { before, after }),
                Err(violation) => Err(CaptureStepError::AfterObservation {
                    violation, before:Box::new(before), event:None,
                }),
            }
        }
        Err(violation) => Err(CaptureStepError::Step {
            violation, before:Box::new(before), after:capture(admitted, image).map(Box::new),
        }),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContinuityBoundary {
    EmptyReturned,
    InitialBefore,
    Between { previous_record: usize, next_record: usize },
    FinalReturned { final_record: usize },
}
pub struct ContinuityDifference<'a> {
    pub boundary: ContinuityBoundary,
    pub expected: &'a HostSnapshot,
    pub observed: &'a HostSnapshot,
}

/// Full typed boundary equality only. A match does not assert that events are
/// valid, authenticate source, establish terminality or prove semantic refinement.
/// A present empty sequence compares initial and returned state directly.
pub fn first_discontinuity<'a>(initial: &'a HostSnapshot,
    records: &'a [CompleteStepRecord], returned: &'a HostSnapshot)
    -> Option<ContinuityDifference<'a>>
{
    if records.is_empty() {
        return (initial != returned).then_some(ContinuityDifference {
            boundary:ContinuityBoundary::EmptyReturned, expected:initial, observed:returned,
        });
    }
    if initial != &records[0].before {
        return Some(ContinuityDifference {
            boundary:ContinuityBoundary::InitialBefore, expected:initial, observed:&records[0].before,
        });
    }
    for index in 1..records.len() {
        if records[index - 1].after != records[index].before {
            return Some(ContinuityDifference {
                boundary:ContinuityBoundary::Between { previous_record:index - 1, next_record:index },
                expected:&records[index - 1].after, observed:&records[index].before,
            });
        }
    }
    let final_record = records.len() - 1;
    (&records[final_record].after != returned).then_some(ContinuityDifference {
        boundary:ContinuityBoundary::FinalReturned { final_record },
        expected:&records[final_record].after, observed:returned,
    })
}


/// Optional witness materialization around the SAME existing fixed-owner step.
/// The raw event/boundaries remain complete and unmodified. No run loop or
/// transition evaluator is introduced. All plan/decode failures are tooling
/// failures after the real step; raw boundaries survive and no retry is implied.
pub struct PlannedObservedStep {
    pub observed: ObservedStep,
    pub before: super::witness_plan::MaterializedSnapshot,
    pub after: super::witness_plan::MaterializedSnapshot,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WitnessSide { Before, After }
pub enum PlannedStepError {
    Owner(CaptureStepError),
    Materialization {
        side: WitnessSide,
        failure: super::witness_plan::WitnessFailure,
        observed: Box<ObservedStep>,
        completed_before: Option<Box<super::witness_plan::MaterializedSnapshot>>,
    },
}
/// Execute existing record_step exactly once, then materialize its OWN saved
/// boundaries. A tooling failure does not roll back or erase the actual step.
/// Original capture/record_step signatures, raw semantics and owner errors stay
/// unchanged. Cloning/decoding retains complete values; host costs remain H1 gaps.
pub fn record_step_with_plan(admitted: &Admitted, image: &mut MachineImage)
    -> Result<PlannedObservedStep, PlannedStepError>
{
    use super::witness_plan::{materialize_for, LifetimeUnit};
    let observed = record_step(admitted, image).map_err(PlannedStepError::Owner)?;
    let (before_raw, after_raw) = match &observed {
        ObservedStep::Event(record) => (&record.before, &record.after),
        ObservedStep::Terminal { before, after } => (before, after),
    };
    let before = match materialize_for(before_raw, LifetimeUnit::RecordBefore) {
        Ok(value) => value,
        Err(failure) => return Err(PlannedStepError::Materialization {
            side: WitnessSide::Before, failure, observed: Box::new(observed),
            completed_before: None,
        }),
    };
    let after = match materialize_for(after_raw, LifetimeUnit::RecordAfter) {
        Ok(value) => value,
        Err(failure) => return Err(PlannedStepError::Materialization {
            side: WitnessSide::After, failure, observed: Box::new(observed),
            completed_before: Some(Box::new(before)),
        }),
    };
    Ok(PlannedObservedStep { observed, before, after })
}
