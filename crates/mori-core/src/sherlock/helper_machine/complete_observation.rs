//! Owned complete B host records around the existing fixed owner step.
//!
//! A diagnostic observer, not a new engine, guard, codec or transition validator.
//! Every snapshot copies the actual owner-bound observation. Admission/source,
//! native representation and source/transition authenticity are not established.
//! Host allocation and copying costs are unmeasured and outside B work counters.
use super::raw::{ConstRow, DataRow};
use super::shared::*;
use super::shared::host_implied_prefix::SegBase;
use super::shared::owner_host::{
    financial_presence, host_owner_engine, observe_bound_image, observe_fin_bound_image,
    HostObservation,
};
use serde_json::json;

fn clone_type_expr_iterative(root: &owner_ext_type_builder::TypeExprSrc) -> owner_ext_type_builder::TypeExprSrc {
    use owner_ext_type_builder::TypeExprSrc;
    // Leaf clones need only their value (and owned name), not two work Vecs.
    // Keep Apply on the original iterative path, including deep applications.
    match root {
        TypeExprSrc::UInt => return TypeExprSrc::UInt,
        TypeExprSrc::Bool => return TypeExprSrc::Bool,
        TypeExprSrc::List => return TypeExprSrc::List,
        TypeExprSrc::Tree => return TypeExprSrc::Tree,
        TypeExprSrc::Named { name } => return TypeExprSrc::Named { name: name.clone() },
        TypeExprSrc::Param { name } => return TypeExprSrc::Param { name: name.clone() },
        TypeExprSrc::Apply { .. } => {}
    }
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

#[derive(Clone, Debug)]
pub struct FinAdmissionSnapshot {
    pub identity: IdentityC,
    pub bounds: BoundsC,
    pub layout: LayoutSnapshot,
    pub ext_capacities: Option<ExtCapacitiesC>,
    pub prepared: super::shared::fin_carrier::FinPreparedStagedSource,
}

#[derive(Clone, Debug)]
pub struct FinHostSnapshot {
    admission: FinAdmissionSnapshot,
    image: ImageSnapshot,
    state: FinImageState,
}
impl FinHostSnapshot {
    pub fn admission(&self) -> &FinAdmissionSnapshot { &self.admission }
    pub fn image(&self) -> &ImageSnapshot { &self.image }
    pub fn state(&self) -> &FinImageState { &self.state }
}

/// Complete owned financial genesis capture. AB binding happens inside
/// observe_fin_bound_image before any retained field is copied.
pub fn capture_fin(
    admitted: &Admitted,
    image: &MachineImage,
) -> Result<FinHostSnapshot, HostViolation> {
    let obs = observe_fin_bound_image(admitted, image)?;
    let l = obs.admission.layout;
    let layout = LayoutSnapshot {
        layout_version: l.layout_version.clone(),
        field_id: l.field_id.clone(),
        widths: l.widths.as_ref().map(|w| WidthsSnapshot {
            index_widths: w.index_width_entries().to_vec(),
            counter_widths: w.counter_width_entries().to_vec(),
            record_widths: w.record_width_entries().to_vec(),
        }),
        scalar_layout_id: l.scalar_layout_id.clone(),
        bounds: l.bounds,
        capacities: l.capacities,
        boundary_capacities_present: l.boundary_capacities_present,
    };
    let img = obs.image;
    Ok(FinHostSnapshot {
        admission: FinAdmissionSnapshot {
            identity: obs.admission.identity.clone(),
            bounds: *obs.admission.bounds,
            layout,
            ext_capacities: l.ext_capacities,
            prepared: obs.admission.prepared.clone(),
        },
        image: ImageSnapshot {
            run_id: img.run_id.clone(),
            control: *img.control,
            control_origin: img.control_origin,
            env_root: img.env_root,
            stack_limit: img.stack_limit,
            outcome: *img.outcome,
            initial_fuel: img.initial_fuel,
            remaining_work: img.remaining_work,
            consumed_work: img.consumed_work,
            helper_work: img.helper_work,
            helper_visits: img.helper_visits,
            reference_cursor: img.reference_cursor,
            alloc: *img.alloc,
            active: *img.active,
            helper: img.helper.copied(),
            result: img.result.copied(),
            seg_base: img.seg_base,
            data: img.data.iter().map(data).collect(),
            data_fields: img.data_fields.to_vec(),
            env: img.env.to_vec(),
            frames: img.frames.to_vec(),
            tasks: img.tasks.to_vec(),
            value_slots: img.value_slots.to_vec(),
        },
        state: img.state.clone(),
    })
}

fn fin_bounds_json(b: &BoundsC) -> serde_json::Value {
    json!({
        "code_rows": b.code_rows.to_string(),
        "function_rows": b.function_rows.to_string(),
        "code_child_slots": b.code_child_slots.to_string(),
        "parameter_slots": b.parameter_slots.to_string(),
        "pattern_rows": b.pattern_rows.to_string(),
        "match_plan_rows": b.match_plan_rows.to_string(),
        "match_arm_rows": b.match_arm_rows.to_string(),
        "type_rows": b.type_rows.to_string(),
        "constructor_rows": b.constructor_rows.to_string(),
        "constructor_field_slots": b.constructor_field_slots.to_string(),
        "max_arity": b.max_arity.to_string(),
        "constant_nodes": b.constant_nodes.to_string(),
        "constant_field_slots": b.constant_field_slots.to_string(),
        "data_nodes": b.data_nodes.to_string(),
        "data_field_slots": b.data_field_slots.to_string(),
        "environment_cells": b.environment_cells.to_string(),
        "frames": b.frames.to_string(),
        "helper_tasks": b.helper_tasks.to_string(),
        "serialization_bytes": b.serialization_bytes.to_string(),
        "max_reference_work": b.max_reference_work.to_string(),
        "max_helper_work": b.max_helper_work.to_string(),
        "step_cap": b.step_cap.to_string(),
    })
}

fn fin_digest_json(d: &DigestC) -> serde_json::Value {
    match d {
        DigestC::Unselected => json!("Unselected"),
        DigestC::Selected { format, bytes } => json!({
            "Selected": { "format": format, "bytes": bytes },
        }),
    }
}

fn fin_scalar_json(v: &super::shared::fin_carrier::FinScalar) -> serde_json::Value {
    match v {
        super::shared::fin_carrier::FinScalar::UInt(n) => json!({ "UInt": n.to_string() }),
        super::shared::fin_carrier::FinScalar::Bool(b) => json!({ "Bool": b }),
    }
}

#[derive(Debug)]
pub enum FinCaptureJsonError {
    ActionIndexOutOfRange { action_index: usize, action_count: usize },
    SnapshotSourceMismatch,
    StateCountMismatch { snapshot_states: usize, bound_states: usize },
    BalanceAssetRowOutOfRange { row: usize },
    LiabilityAssetRowOutOfRange { row: usize },
    SerializationLimitExceeded { limit: usize },
    OutputNotUtf8,
    Serialize { field: &'static str, error: serde_json::Error },
}

fn fin_widths_json(w: &Option<WidthsSnapshot>) -> serde_json::Value {
    match w {
        None => serde_json::Value::Null,
        Some(w) => json!({
            "index_widths": w.index_widths,
            "counter_widths": w.counter_widths,
            "record_widths": w.record_widths,
        }),
    }
}

fn fin_ext_capacities_json(e: &Option<ExtCapacitiesC>) -> serde_json::Value {
    match e {
        None => serde_json::Value::Null,
        Some(e) => json!({
            "plan_case_slots": e.plan_case_slots,
            "field_path_slots": e.field_path_slots,
            "pattern_child_slots": e.pattern_child_slots,
            "input_rows": e.input_rows,
            "asset_rows": e.asset_rows,
            "state_rows": e.state_rows,
            "account_rows": e.account_rows,
            "liability_rows": e.liability_rows,
            "await_site_rows": e.await_site_rows,
            "await_binder_slots": e.await_binder_slots,
            "balance_cells": e.balance_cells,
            "journal_rows": e.journal_rows,
            "pending_op_rows": e.pending_op_rows,
            "observations_per_invocation": e.observations_per_invocation,
            "max_lifetime_work": e.max_lifetime_work.to_string(),
            "resume_rows": e.resume_rows,
        }),
    }
}

struct BoundedJsonWriter {
    buf: Vec<u8>,
    limit: usize,
    exceeded: bool,
}

impl BoundedJsonWriter {
    fn new(limit: usize) -> Self {
        Self { buf: Vec::new(), limit, exceeded: false }
    }
    fn limit_error(&self) -> FinCaptureJsonError {
        FinCaptureJsonError::SerializationLimitExceeded { limit: self.limit }
    }
}

impl std::io::Write for BoundedJsonWriter {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        self.write_all(bytes)?;
        Ok(bytes.len())
    }
    fn write_all(&mut self, bytes: &[u8]) -> std::io::Result<()> {
        let next = self.buf.len().checked_add(bytes.len()).filter(|n| *n <= self.limit);
        match next {
            Some(_) => {
                self.buf.extend_from_slice(bytes);
                Ok(())
            }
            None => {
                self.exceeded = true;
                Err(std::io::Error::new(std::io::ErrorKind::Other, "fin capture serialization limit exceeded"))
            }
        }
    }
    fn flush(&mut self) -> std::io::Result<()> { Ok(()) }
}

fn bw(w: &mut BoundedJsonWriter, bytes: &[u8]) -> Result<(), FinCaptureJsonError> {
    std::io::Write::write_all(w, bytes).map_err(|_| w.limit_error())
}
fn bw_json_string(w: &mut BoundedJsonWriter, value: &str, field: &'static str) -> Result<(), FinCaptureJsonError> {
    serde_json::to_writer(&mut *w, value).map_err(|error| {
        if w.exceeded { w.limit_error() } else { FinCaptureJsonError::Serialize { field, error } }
    })
}
fn bw_serialize<T: ?Sized + serde::Serialize>(w: &mut BoundedJsonWriter, value: &T, field: &'static str) -> Result<(), FinCaptureJsonError> {
    serde_json::to_writer(&mut *w, value).map_err(|error| {
        if w.exceeded { w.limit_error() } else { FinCaptureJsonError::Serialize { field, error } }
    })
}
fn bw_display(w: &mut BoundedJsonWriter, value: &dyn std::fmt::Display) -> Result<(), FinCaptureJsonError> {
    use std::io::Write as _;
    write!(w, "{}", value).map_err(|_| w.limit_error())
}

struct DebugEscapeWriter<'a> {
    writer: &'a mut BoundedJsonWriter,
}
impl std::fmt::Write for DebugEscapeWriter<'_> {
    fn write_str(&mut self, value: &str) -> std::fmt::Result {
        for ch in value.chars() {
            let result = match ch {
                '"' => bw(self.writer, b"\\\""),
                '\\' => bw(self.writer, b"\\\\"),
                '\n' => bw(self.writer, b"\\n"),
                '\r' => bw(self.writer, b"\\r"),
                '\t' => bw(self.writer, b"\\t"),
                '\u{0008}' => bw(self.writer, b"\\b"),
                '\u{000C}' => bw(self.writer, b"\\f"),
                ch if (ch as u32) < 0x20 => {
                    let mut hex = [0u8; 6];
                    hex[0] = b'\\';
                    hex[1] = b'u';
                    let n = ch as u32;
                    hex[2] = b"0123456789abcdef"[(n >> 12) as usize & 0xF];
                    hex[3] = b"0123456789abcdef"[(n >> 8) as usize & 0xF];
                    hex[4] = b"0123456789abcdef"[(n >> 4) as usize & 0xF];
                    hex[5] = b"0123456789abcdef"[(n & 0xF) as usize];
                    bw(self.writer, &hex)
                }
                ch => {
                    let mut utf8 = [0u8; 4];
                    bw(self.writer, ch.encode_utf8(&mut utf8).as_bytes())
                }
            };
            if result.is_err() {
                return Err(std::fmt::Error);
            }
        }
        Ok(())
    }
}

fn bw_debug_string(w: &mut BoundedJsonWriter, value: &dyn std::fmt::Debug, field: &'static str) -> Result<(), FinCaptureJsonError> {
    use std::fmt::Write as _;
    bw(w, b"\"")?;
    let mut adapter = DebugEscapeWriter { writer: w };
    if write!(adapter, "{:?}", value).is_err() {
        if w.exceeded {
            return Err(w.limit_error());
        }
        return Err(FinCaptureJsonError::Serialize {
            field,
            error: serde_json::Error::io(std::io::Error::new(std::io::ErrorKind::Other, "debug formatting failed")),
        });
    }
    bw(w, b"\"")
}

fn bw_bounds(w: &mut BoundedJsonWriter, b: &BoundsC) -> Result<(), FinCaptureJsonError> {
    bw(w, b"{")?;
    let fields: [(&str, String); 22] = [
        ("code_rows", b.code_rows.to_string()), ("function_rows", b.function_rows.to_string()),
        ("code_child_slots", b.code_child_slots.to_string()), ("parameter_slots", b.parameter_slots.to_string()),
        ("pattern_rows", b.pattern_rows.to_string()), ("match_plan_rows", b.match_plan_rows.to_string()),
        ("match_arm_rows", b.match_arm_rows.to_string()), ("type_rows", b.type_rows.to_string()),
        ("constructor_rows", b.constructor_rows.to_string()), ("constructor_field_slots", b.constructor_field_slots.to_string()),
        ("max_arity", b.max_arity.to_string()), ("constant_nodes", b.constant_nodes.to_string()),
        ("constant_field_slots", b.constant_field_slots.to_string()), ("data_nodes", b.data_nodes.to_string()),
        ("data_field_slots", b.data_field_slots.to_string()), ("environment_cells", b.environment_cells.to_string()),
        ("frames", b.frames.to_string()), ("helper_tasks", b.helper_tasks.to_string()),
        ("serialization_bytes", b.serialization_bytes.to_string()), ("max_reference_work", b.max_reference_work.to_string()),
        ("max_helper_work", b.max_helper_work.to_string()), ("step_cap", b.step_cap.to_string()),
    ];
    for (index, (name, value)) in fields.iter().enumerate() {
        if index > 0 { bw(w, b",")?; }
        bw_json_string(w, name, "bounds.key")?;
        bw(w, b":")?;
        bw_json_string(w, value, "bounds.value")?;
    }
    bw(w, b"}")
}

fn bw_digest(w: &mut BoundedJsonWriter, d: &DigestC) -> Result<(), FinCaptureJsonError> {
    match d {
        DigestC::Unselected => bw_json_string(w, "Unselected", "identity.digest"),
        DigestC::Selected { format, bytes } => {
            bw(w, b"{\"Selected\":{\"format\":")?;
            bw_serialize(w, format, "identity.digest.format")?;
            bw(w, b",\"bytes\":")?;
            bw_serialize(w, bytes, "identity.digest.bytes")?;
            bw(w, b"}}")
        }
    }
}

fn bw_scalar(w: &mut BoundedJsonWriter, v: &super::shared::fin_carrier::FinScalar) -> Result<(), FinCaptureJsonError> {
    match v {
        super::shared::fin_carrier::FinScalar::UInt(n) => {
            bw(w, b"{\"UInt\":")?;
            bw_json_string(w, &n.to_string(), "fin_scalar")?;
            bw(w, b"}")
        }
        super::shared::fin_carrier::FinScalar::Bool(value) => {
            bw(w, b"{\"Bool\":")?;
            bw_display(w, value)?;
            bw(w, b"}")
        }
    }
}

fn bw_widths(w: &mut BoundedJsonWriter, widths: &Option<WidthsSnapshot>) -> Result<(), FinCaptureJsonError> {
    let Some(widths) = widths else { return bw(w, b"null"); };
    bw(w, b"{\"index_widths\":")?;
    bw_serialize(w, &widths.index_widths, "layout.widths.index_widths")?;
    bw(w, b",\"counter_widths\":")?;
    bw_serialize(w, &widths.counter_widths, "layout.widths.counter_widths")?;
    bw(w, b",\"record_widths\":")?;
    bw_serialize(w, &widths.record_widths, "layout.widths.record_widths")?;
    bw(w, b"}")
}

fn bw_ext_capacities(w: &mut BoundedJsonWriter, ext: &Option<ExtCapacitiesC>) -> Result<(), FinCaptureJsonError> {
    let Some(e) = ext else { return bw(w, b"null"); };
    bw(w, b"{")?;
    let fields: [(&str, String); 16] = [
        ("plan_case_slots", e.plan_case_slots.to_string()), ("field_path_slots", e.field_path_slots.to_string()),
        ("pattern_child_slots", e.pattern_child_slots.to_string()), ("input_rows", e.input_rows.to_string()),
        ("asset_rows", e.asset_rows.to_string()), ("state_rows", e.state_rows.to_string()),
        ("account_rows", e.account_rows.to_string()), ("liability_rows", e.liability_rows.to_string()),
        ("await_site_rows", e.await_site_rows.to_string()), ("await_binder_slots", e.await_binder_slots.to_string()),
        ("balance_cells", e.balance_cells.to_string()), ("journal_rows", e.journal_rows.to_string()),
        ("pending_op_rows", e.pending_op_rows.to_string()), ("observations_per_invocation", e.observations_per_invocation.to_string()),
        ("max_lifetime_work", e.max_lifetime_work.to_string()), ("resume_rows", e.resume_rows.to_string()),
    ];
    for (index, (name, value)) in fields.iter().enumerate() {
        if index > 0 { bw(w, b",")?; }
        bw_json_string(w, name, "layout.ext_capacities.key")?;
        bw(w, b":")?;
        if *name == "max_lifetime_work" {
            bw_json_string(w, value, "layout.ext_capacities.max_lifetime_work")?;
        } else {
            bw(w, value.as_bytes())?;
        }
    }
    bw(w, b"}")
}

/// Bounded diagnostic serialization of the complete financial genesis
/// capture. The retained limit is `admission.bounds.serialization_bytes`;
/// zero is a real zero limit, not unlimited. All program, stage-plan,
/// core-snapshot, input, configuration, geometry, vector, Debug-string,
/// and escaped UTF-8 output bytes are written through the checked writer
/// before the returned `String` exists. No `serde_json::Value`, DOM, or
/// `format!("{:?}", ...)` intermediate is constructed. This is not a C6,
/// native, checkpoint, canonical, or otherwise adopted wire format.
pub fn fin_capture_json(s: &FinHostSnapshot) -> Result<String, FinCaptureJsonError> {
    let prepared = &s.admission.prepared;
    let bound = prepared.bound_source();
    let identity = &s.admission.identity;
    let actions = &prepared.fin_plan().actions;
    let action = actions.get(s.state.action_index).ok_or(
        FinCaptureJsonError::ActionIndexOutOfRange {
            action_index: s.state.action_index,
            action_count: actions.len(),
        },
    )?;
    if s.state.snapshot.source() != bound {
        return Err(FinCaptureJsonError::SnapshotSourceMismatch);
    }
    if s.state.snapshot.states().len() != bound.states().len() {
        return Err(FinCaptureJsonError::StateCountMismatch {
            snapshot_states: s.state.snapshot.states().len(),
            bound_states: bound.states().len(),
        });
    }
    for balance in s.state.snapshot.balances() {
        if balance.asset.0 >= bound.assets().len() {
            return Err(FinCaptureJsonError::BalanceAssetRowOutOfRange { row: balance.asset.0 });
        }
    }
    for liability in s.state.snapshot.liabilities() {
        if liability.asset.0 >= bound.assets().len() {
            return Err(FinCaptureJsonError::LiabilityAssetRowOutOfRange { row: liability.asset.0 });
        }
    }

    let mut w = BoundedJsonWriter::new(s.admission.bounds.serialization_bytes);
    bw(&mut w, b"{\"format\":\"financial-genesis-capture/1\",\"identity\":{\"profile_id\":")?;
    bw_display(&mut w, &identity.profile_id)?;
    bw(&mut w, b",\"grammar_domain\":")?;
    bw_debug_string(&mut w, &identity.grammar_domain, "identity.grammar_domain")?;
    bw(&mut w, b",\"finance\":")?;
    bw_debug_string(&mut w, &identity.finance, "identity.finance")?;
    bw(&mut w, b",\"layout_version\":")?;
    bw_serialize(&mut w, &identity.layout_version, "identity.layout_version")?;
    bw(&mut w, b",\"layout_digest\":")?;
    bw_digest(&mut w, &identity.layout_digest)?;
    bw(&mut w, b",\"bounds_digest\":")?;
    bw_digest(&mut w, &identity.bounds_digest)?;
    bw(&mut w, b",\"field_id\":")?;
    bw_serialize(&mut w, &identity.field_id, "identity.field_id")?;
    bw(&mut w, b",\"program_membership_digest\":")?;
    bw_digest(&mut w, &identity.program_membership_digest)?;
    bw(&mut w, b"},\"bounds\":")?;
    bw_bounds(&mut w, &s.admission.bounds)?;
    bw(&mut w, b",\"layout\":{\"layout_version\":")?;
    bw_serialize(&mut w, &s.admission.layout.layout_version, "layout.layout_version")?;
    bw(&mut w, b",\"field_id\":")?;
    bw_serialize(&mut w, &s.admission.layout.field_id, "layout.field_id")?;
    bw(&mut w, b",\"widths\":")?;
    bw_widths(&mut w, &s.admission.layout.widths)?;
    bw(&mut w, b",\"scalar_layout_id\":")?;
    bw_serialize(&mut w, &s.admission.layout.scalar_layout_id, "layout.scalar_layout_id")?;
    bw(&mut w, b",\"bounds\":")?;
    bw_bounds(&mut w, &s.admission.layout.bounds)?;
    bw(&mut w, b",\"capacities\":{\"name_rows\":")?;
    bw_json_string(&mut w, &s.admission.layout.capacities.name_rows.to_string(), "layout.capacities")?;
    bw(&mut w, b",\"name_bytes\":")?;
    bw_json_string(&mut w, &s.admission.layout.capacities.name_bytes.to_string(), "layout.capacities")?;
    bw(&mut w, b",\"occurrence_path_slots\":")?;
    bw_json_string(&mut w, &s.admission.layout.capacities.occurrence_path_slots.to_string(), "layout.capacities")?;
    bw(&mut w, b",\"value_slots\":")?;
    bw_json_string(&mut w, &s.admission.layout.capacities.value_slots.to_string(), "layout.capacities")?;
    bw(&mut w, b"},\"ext_capacities\":")?;
    bw_ext_capacities(&mut w, &s.admission.ext_capacities)?;
    bw(&mut w, b",\"boundary_capacities_present\":")?;
    bw_display(&mut w, &s.admission.layout.boundary_capacities_present)?;
    bw(&mut w, b"},\"prepared\":{\"program\":")?;
    bw_serialize(&mut w, prepared.program(), "prepared.program")?;
    bw(&mut w, b",\"stage_plan\":")?;
    bw_serialize(&mut w, prepared.stage_plan(), "prepared.stage_plan")?;
    bw(&mut w, b",\"program_hash\":")?;
    bw_serialize(&mut w, &prepared.program_hash().to_vec(), "prepared.program_hash")?;
    bw(&mut w, b",\"plan_hash\":")?;
    bw_serialize(&mut w, &prepared.plan_hash().to_vec(), "prepared.plan_hash")?;
    bw(&mut w, b",\"bound_names\":")?;
    bw_serialize(&mut w, bound.names(), "prepared.bound_names")?;
    bw(&mut w, b",\"bound_states\":[")?;
    for (index, state) in bound.states().iter().enumerate() {
        if index > 0 { bw(&mut w, b",")?; }
        bw(&mut w, b"{\"row\":")?;
        bw_display(&mut w, &state.row.0)?;
        bw(&mut w, b",\"name_id\":")?;
        bw_display(&mut w, &state.name.0)?;
        bw(&mut w, b",\"text\":")?;
        bw_json_string(&mut w, &state.text, "prepared.bound_states.text")?;
        bw(&mut w, b",\"ty\":")?;
        bw_debug_string(&mut w, &state.ty, "prepared.bound_states.ty")?;
        bw(&mut w, b",\"initial\":")?;
        bw_scalar(&mut w, &state.initial)?;
        bw(&mut w, b"}")?;
    }
    bw(&mut w, b"],\"bound_assets\":[")?;
    for (index, asset) in bound.assets().iter().enumerate() {
        if index > 0 { bw(&mut w, b",")?; }
        bw(&mut w, b"{\"row\":")?;
        bw_display(&mut w, &asset.row.0)?;
        bw(&mut w, b",\"name_id\":")?;
        bw_display(&mut w, &asset.name.0)?;
        bw(&mut w, b",\"text\":")?;
        bw_json_string(&mut w, &asset.text, "prepared.bound_assets.text")?;
        bw(&mut w, b",\"identity\":{\"network\":")?;
        bw_json_string(&mut w, &asset.identity.network, "prepared.bound_assets.identity")?;
        bw(&mut w, b",\"issuer\":")?;
        bw_json_string(&mut w, &asset.identity.issuer, "prepared.bound_assets.identity")?;
        bw(&mut w, b",\"reference\":")?;
        bw_json_string(&mut w, &asset.identity.reference, "prepared.bound_assets.identity")?;
        bw(&mut w, b",\"scale\":")?;
        bw_display(&mut w, &asset.identity.scale)?;
        bw(&mut w, b"}}")?;
    }
    bw(&mut w, b"],\"bound_actions\":[")?;
    for (index, action) in bound.actions().iter().enumerate() {
        if index > 0 { bw(&mut w, b",")?; }
        bw(&mut w, b"{\"name_id\":")?;
        bw_display(&mut w, &action.name.0)?;
        bw(&mut w, b",\"text\":")?;
        bw_json_string(&mut w, &action.text, "prepared.bound_actions.text")?;
        bw(&mut w, b"}")?;
    }
    bw(&mut w, b"],\"bound_capacities\":{\"max_states\":")?;
    bw_display(&mut w, &bound.capacities().max_states)?;
    bw(&mut w, b",\"max_assets\":")?;
    bw_display(&mut w, &bound.capacities().max_assets)?;
    bw(&mut w, b",\"max_balances\":")?;
    bw_display(&mut w, &bound.capacities().max_balances)?;
    bw(&mut w, b",\"max_liabilities\":")?;
    bw_display(&mut w, &bound.capacities().max_liabilities)?;
    bw(&mut w, b"},\"fin_plan\":{\"actions\":[")?;
    for (index, action) in actions.iter().enumerate() {
        if index > 0 { bw(&mut w, b",")?; }
        bw(&mut w, b"{\"name\":")?;
        bw_json_string(&mut w, &action.name, "prepared.fin_plan.name")?;
        bw(&mut w, b",\"segment_src_cost\":[")?;
        for (item, value) in action.segment_src_cost.iter().enumerate() {
            if item > 0 { bw(&mut w, b",")?; }
            bw_json_string(&mut w, &value.to_string(), "prepared.fin_plan.segment_src_cost")?;
        }
        bw(&mut w, b"],\"committed_before\":[")?;
        for (item, value) in action.committed_before.iter().enumerate() {
            if item > 0 { bw(&mut w, b",")?; }
            bw_json_string(&mut w, &value.to_string(), "prepared.fin_plan.committed_before")?;
        }
        bw(&mut w, b"],\"plan_total\":")?;
        bw_json_string(&mut w, &action.plan_total.to_string(), "prepared.fin_plan.plan_total")?;
        bw(&mut w, b"}")?;
    }
    bw(&mut w, b"]}},\"genesis\":{\"action_index\":")?;
    bw_display(&mut w, &s.state.action_index)?;
    bw(&mut w, b",\"action\":")?;
    bw_json_string(&mut w, &action.name, "genesis.action")?;
    bw(&mut w, b",\"inputs\":[")?;
    for (index, input) in s.state.inputs.iter().enumerate() {
        if index > 0 { bw(&mut w, b",")?; }
        bw(&mut w, b"{\"name\":")?;
        bw_json_string(&mut w, &input.name, "genesis.inputs.name")?;
        bw(&mut w, b",\"ty\":")?;
        bw_debug_string(&mut w, &input.ty, "genesis.inputs.ty")?;
        bw(&mut w, b",\"value\":")?;
        bw_serialize(&mut w, &input.value, "genesis.inputs.value")?;
        bw(&mut w, b"}")?;
    }
    bw(&mut w, b"],\"snapshot\":{\"states\":[")?;
    for (index, value) in s.state.snapshot.states().iter().enumerate() {
        if index > 0 { bw(&mut w, b",")?; }
        bw_scalar(&mut w, value)?;
    }
    bw(&mut w, b"],\"balances\":[")?;
    for (index, balance) in s.state.snapshot.balances().iter().enumerate() {
        if index > 0 { bw(&mut w, b",")?; }
        bw(&mut w, b"{\"asset_row\":")?;
        bw_display(&mut w, &balance.asset.0)?;
        bw(&mut w, b",\"account\":")?;
        bw_json_string(&mut w, &balance.account, "genesis.snapshot.balances.account")?;
        bw(&mut w, b",\"amount\":")?;
        bw_json_string(&mut w, &balance.amount.to_string(), "genesis.snapshot.balances.amount")?;
        bw(&mut w, b"}")?;
    }
    bw(&mut w, b"],\"liabilities\":[")?;
    for (index, liability) in s.state.snapshot.liabilities().iter().enumerate() {
        if index > 0 { bw(&mut w, b",")?; }
        bw(&mut w, b"{\"name\":")?;
        bw_json_string(&mut w, &liability.name, "genesis.snapshot.liabilities.name")?;
        bw(&mut w, b",\"asset_row\":")?;
        bw_display(&mut w, &liability.asset.0)?;
        bw(&mut w, b",\"debtor\":")?;
        bw_json_string(&mut w, &liability.debtor, "genesis.snapshot.liabilities.debtor")?;
        bw(&mut w, b",\"creditor\":")?;
        bw_json_string(&mut w, &liability.creditor, "genesis.snapshot.liabilities.creditor")?;
        bw(&mut w, b",\"amount\":")?;
        bw_json_string(&mut w, &liability.amount.to_string(), "genesis.snapshot.liabilities.amount")?;
        bw(&mut w, b"}")?;
    }
    bw(&mut w, b"]},\"core_snapshot\":")?;
    bw_serialize(&mut w, &s.state.core_snapshot, "genesis.core_snapshot")?;
    bw(&mut w, b",\"cap\":")?;
    match s.state.cap {
        Some(cap) => bw_json_string(&mut w, &cap.0.to_string(), "genesis.cap")?,
        None => bw(&mut w, b"null")?,
    }
    bw(&mut w, b",\"committed\":")?;
    bw_json_string(&mut w, &s.state.committed.0.to_string(), "genesis.committed")?;
    bw(&mut w, b"},\"image\":{\"run_id\":")?;
    bw_debug_string(&mut w, &s.image.run_id, "image.run_id")?;
    bw(&mut w, b",\"control\":")?;
    bw_debug_string(&mut w, &s.image.control, "image.control")?;
    bw(&mut w, b",\"control_origin\":")?;
    bw_json_string(&mut w, &s.image.control_origin.to_string(), "image.control_origin")?;
    bw(&mut w, b",\"env_root\":")?;
    bw_debug_string(&mut w, &s.image.env_root, "image.env_root")?;
    bw(&mut w, b",\"stack_limit\":")?;
    bw_json_string(&mut w, &s.image.stack_limit.to_string(), "image.stack_limit")?;
    bw(&mut w, b",\"outcome\":")?;
    bw_debug_string(&mut w, &s.image.outcome, "image.outcome")?;
    bw(&mut w, b",\"initial_fuel\":")?;
    bw_json_string(&mut w, &s.image.initial_fuel.to_string(), "image.initial_fuel")?;
    bw(&mut w, b",\"remaining_work\":")?;
    bw_json_string(&mut w, &s.image.remaining_work.to_string(), "image.remaining_work")?;
    bw(&mut w, b",\"consumed_work\":")?;
    bw_json_string(&mut w, &s.image.consumed_work.to_string(), "image.consumed_work")?;
    bw(&mut w, b",\"helper_work\":")?;
    bw_json_string(&mut w, &s.image.helper_work.to_string(), "image.helper_work")?;
    bw(&mut w, b",\"helper_visits\":")?;
    bw_json_string(&mut w, &s.image.helper_visits.to_string(), "image.helper_visits")?;
    bw(&mut w, b",\"reference_cursor\":")?;
    bw_json_string(&mut w, &s.image.reference_cursor.to_string(), "image.reference_cursor")?;
    bw(&mut w, b",\"alloc\":")?;
    bw_debug_string(&mut w, &s.image.alloc, "image.alloc")?;
    bw(&mut w, b",\"active\":")?;
    bw_debug_string(&mut w, &s.image.active, "image.active")?;
    bw(&mut w, b",\"seg_base\":{\"active\":")?;
    bw_debug_string(&mut w, &s.image.seg_base.active(), "image.seg_base.active")?;
    bw(&mut w, b",\"alloc\":")?;
    bw_debug_string(&mut w, &s.image.seg_base.alloc(), "image.seg_base.alloc")?;
    bw(&mut w, b",\"consumed_work\":")?;
    let seg_consumed = s.image.seg_base.consumed_work();
    bw_debug_string(&mut w, &seg_consumed, "image.seg_base.consumed_work")?;
    bw(&mut w, b"},\"helper\":")?;
    bw_debug_string(&mut w, &s.image.helper, "image.helper")?;
    bw(&mut w, b",\"result\":")?;
    bw_debug_string(&mut w, &s.image.result, "image.result")?;
    bw(&mut w, b",\"data\":")?;
    bw_debug_string(&mut w, &s.image.data, "image.data")?;
    bw(&mut w, b",\"data_fields\":")?;
    bw_debug_string(&mut w, &s.image.data_fields, "image.data_fields")?;
    bw(&mut w, b",\"env\":")?;
    bw_debug_string(&mut w, &s.image.env, "image.env")?;
    bw(&mut w, b",\"frames\":")?;
    bw_debug_string(&mut w, &s.image.frames, "image.frames")?;
    bw(&mut w, b",\"tasks\":")?;
    bw_debug_string(&mut w, &s.image.tasks, "image.tasks")?;
    bw(&mut w, b",\"value_slots\":")?;
    bw_debug_string(&mut w, &s.image.value_slots, "image.value_slots")?;
    bw(&mut w, b"}}")?;
    if w.exceeded {
        return Err(w.limit_error());
    }
    String::from_utf8(w.buf).map_err(|_| FinCaptureJsonError::OutputNotUtf8)
}

#[cfg(test)]
mod fin_capture_writer_tests {
    use super::*;

    #[test]
    fn bounded_debug_string_escapes_utf8_and_controls() {
        let mut writer = BoundedJsonWriter::new(64);
        bw_debug_string(&mut writer, &"a\"\\\n\u{1}é", "test.debug").expect("bounded debug");
        assert_eq!(
            String::from_utf8(writer.buf).expect("utf8"),
            serde_json::to_string(&format!("{:?}", "a\"\\\n\u{1}é")).expect("expected JSON")
        );
        let mut tight = BoundedJsonWriter::new(3);
        assert!(matches!(
            bw_debug_string(&mut tight, &"abcdef", "test.debug"),
            Err(FinCaptureJsonError::SerializationLimitExceeded { limit: 3 })
        ));
    }
}

pub fn capture(admitted: &Admitted, image: &MachineImage) -> Result<HostSnapshot, HostViolation> {
    if financial_presence(admitted, image)? {
        return Err(HostViolation {
            kind: super::shared::HostViolationKind::FinancialUnsupportedBeyondGenesis {
                op: super::shared::FinUnsupportedOp::Capture,
            },
            clause: None,
            rule: None,
            locator: Some("financial complete capture beyond genesis".to_string()),
        });
    }
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
