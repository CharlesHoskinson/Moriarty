//! Complete output-only owned snapshots and bounded UTF-8 wire writer.
//! Top-level containers (`HostSnapshot`, `AdmissionSnapshot`, `ProgramSnapshot`,
//! `ImageSnapshot`, `LayoutSnapshot`, normalized `StepHostSnapshot`
//! `{"image": ...}`, `CompleteStepRecord`, and `TerminalBoundary`) stream
//! field-by-field and slice-element-by-element into `BoundedJsonWriter`, while
//! `BoundedJsonWriter::write_owner_wire` materializes bounded single-row/leaf
//! `Value`s via `OwnerWire::owner_wire` after pre-checking `buf.len() < active_limit`.
//! Enforces `admission.bounds.serialization_bytes` per projection (`0` is a
//! real zero bound) and cumulative trace bounds validated through
//! `crate::json::parse_limited` without parallel trait duplication.
//! No import, semantic validation, native encoding or authenticity claim.
use super::wire::OwnerWire;
use crate::sherlock::helper_machine::complete_observation as obs;
use crate::sherlock::helper_machine::shared::host_implied_prefix::SegBase;
use serde_json::{Value, json};

/// Versioned complete-trace wire format: `initial` and `returned` carry full
/// `HostSnapshot` (`{"admission": ..., "image": ...}`), while intermediate
/// `records[i].before/after` and `terminal.before/after` carry normalized
/// `StepHostSnapshot` (`{"image": ...}`) sharing `initial["admission"]`.
pub const TRACE_FORMAT: &str = "sherlock-core1-complete-trace/2";

/// Dual serialization budget contract recorded in `complete_trace["serialization_budget"]`:
/// per-projection capacity is `admission.bounds.serialization_bytes`; cumulative
/// whole-trace capacity is `derive_trace_serialization_budget(admission.bounds)`;
/// step/terminal boundaries use `image-only-shared-admission`.
pub const TRACE_SERIALIZATION_BUDGET: &str = "per_projection:admission.bounds.serialization_bytes;whole_trace:derive_trace_serialization_budget(admission.bounds);step_boundaries:image-only-shared-admission";

impl OwnerWire for SegBase {
    fn owner_wire(&self) -> Value {
        json!({"active":self.active().owner_wire(),"alloc":self.alloc().owner_wire(),
        "consumed_work":self.consumed_work().owner_wire()})
    }
}
impl OwnerWire for obs::WidthsSnapshot {
    fn owner_wire(&self) -> Value {
        let obs::WidthsSnapshot {
            index_widths,
            counter_widths,
            record_widths,
        } = self;
        json!({"index_widths":index_widths.owner_wire(),"counter_widths":counter_widths.owner_wire(),"record_widths":record_widths.owner_wire()})
    }
}

impl OwnerWire for obs::LayoutSnapshot {
    fn owner_wire(&self) -> Value {
        let obs::LayoutSnapshot {
            layout_version,
            field_id,
            widths,
            scalar_layout_id,
            bounds,
            capacities,
            boundary_capacities_present,
        } = self;
        json!({"layout_version":layout_version.owner_wire(),"field_id":field_id.owner_wire(),"widths":widths.owner_wire(),"scalar_layout_id":scalar_layout_id.owner_wire(),"bounds":bounds.owner_wire(),"capacities":capacities.owner_wire(),"boundary_capacities_present":boundary_capacities_present.owner_wire()})
    }
}

impl OwnerWire for obs::ExpressionSnapshot {
    fn owner_wire(&self) -> Value {
        match self {
            Self::Literal { constant } => json!({"Literal":{"constant":constant.owner_wire()}}),
            Self::Var { binder } => json!({"Var":{"binder":binder.owner_wire()}}),
            Self::Let {
                binder,
                value,
                body,
            } => {
                json!({"Let":{"binder":binder.owner_wire(),"value":value.owner_wire(),"body":body.owner_wire()}})
            }
            Self::Binary { op, left, right } => {
                json!({"Binary":{"op":op.owner_wire(),"left":left.owner_wire(),"right":right.owner_wire()}})
            }
            Self::Unary { op, operand } => {
                json!({"Unary":{"op":op.owner_wire(),"operand":operand.owner_wire()}})
            }
            Self::If {
                cond,
                then_expr,
                else_expr,
            } => {
                json!({"If":{"cond":cond.owner_wire(),"then_expr":then_expr.owner_wire(),"else_expr":else_expr.owner_wire()}})
            }
            Self::Call {
                function_name,
                args,
            } => {
                json!({"Call":{"function_name":function_name.owner_wire(),"args":args.owner_wire()}})
            }
            Self::Match {
                scrutinee,
                arms,
                plan,
            } => {
                json!({"Match":{"scrutinee":scrutinee.owner_wire(),"arms":arms.owner_wire(),"plan":plan.owner_wire()}})
            }
            Self::Construct {
                type_id,
                ctor_id,
                fields,
                constant,
            } => {
                json!({"Construct":{"type_id":type_id.owner_wire(),"ctor_id":ctor_id.owner_wire(),"fields":fields.owner_wire(),"constant":constant.owner_wire()}})
            }
        }
    }
}

impl OwnerWire for obs::ConstFieldsSnapshot {
    fn owner_wire(&self) -> Value {
        let obs::ConstFieldsSnapshot { start, len } = self;
        json!({"start":start.owner_wire(),"len":len.owner_wire()})
    }
}

impl OwnerWire for obs::DataFieldsSnapshot {
    fn owner_wire(&self) -> Value {
        let obs::DataFieldsSnapshot { start, len } = self;
        json!({"start":start.owner_wire(),"len":len.owner_wire()})
    }
}

impl OwnerWire for obs::ConstantSnapshot {
    fn owner_wire(&self) -> Value {
        match self {
            Self::U128 { marker_free, n } => {
                json!({"U128":{"marker_free":marker_free.owner_wire(),"n":n.owner_wire()}})
            }
            Self::Bool { marker_free, b } => {
                json!({"Bool":{"marker_free":marker_free.owner_wire(),"b":b.owner_wire()}})
            }
            Self::Nil { marker_free } => json!({"Nil":{"marker_free":marker_free.owner_wire()}}),
            Self::Cons {
                marker_free,
                fields,
            } => {
                json!({"Cons":{"marker_free":marker_free.owner_wire(),"fields":fields.owner_wire()}})
            }
            Self::Leaf {
                marker_free,
                fields,
            } => {
                json!({"Leaf":{"marker_free":marker_free.owner_wire(),"fields":fields.owner_wire()}})
            }
            Self::Branch {
                marker_free,
                fields,
            } => {
                json!({"Branch":{"marker_free":marker_free.owner_wire(),"fields":fields.owner_wire()}})
            }
            Self::Constructor {
                marker_free,
                type_id,
                ctor_id,
                fields,
            } => {
                json!({"Constructor":{"marker_free":marker_free.owner_wire(),"type_id":type_id.owner_wire(),"ctor_id":ctor_id.owner_wire(),"fields":fields.owner_wire()}})
            }
            Self::U128OutOfRange { marker_free } => {
                json!({"U128OutOfRange":{"marker_free":marker_free.owner_wire()}})
            }
        }
    }
}

impl OwnerWire for obs::DataSnapshot {
    fn owner_wire(&self) -> Value {
        match self {
            Self::U128 { n } => json!({"U128":{"n":n.owner_wire()}}),
            Self::Bool { b } => json!({"Bool":{"b":b.owner_wire()}}),
            Self::Nil => json!("Nil"),
            Self::Cons { fields } => json!({"Cons":{"fields":fields.owner_wire()}}),
            Self::Leaf { fields } => json!({"Leaf":{"fields":fields.owner_wire()}}),
            Self::Branch { fields } => json!({"Branch":{"fields":fields.owner_wire()}}),
            Self::Constructor {
                type_id,
                ctor_id,
                fields,
            } => {
                json!({"Constructor":{"type_id":type_id.owner_wire(),"ctor_id":ctor_id.owner_wire(),"fields":fields.owner_wire()}})
            }
        }
    }
}

impl OwnerWire for obs::ProgramSnapshot {
    fn owner_wire(&self) -> Value {
        let obs::ProgramSnapshot {
            entry,
            expressions,
            occurrences,
            occurrence_paths,
            children,
            functions,
            parameters,
            arms,
            patterns,
            pattern_children,
            types,
            type_args,
            constructors,
            ctor_fields,
            decl_param_counts,
            decl_ctor_counts,
            decl_templates,
            plan_nodes,
            plan_cases,
            field_paths,
            constants,
            constant_fields,
            names,
        } = self;
        json!({"entry":entry.owner_wire(),"expressions":expressions.owner_wire(),"occurrences":occurrences.owner_wire(),"occurrence_paths":occurrence_paths.owner_wire(),"children":children.owner_wire(),"functions":functions.owner_wire(),"parameters":parameters.owner_wire(),"arms":arms.owner_wire(),"patterns":patterns.owner_wire(),"pattern_children":pattern_children.owner_wire(),"types":types.owner_wire(),"type_args":type_args.owner_wire(),"constructors":constructors.owner_wire(),"ctor_fields":ctor_fields.owner_wire(),"decl_param_counts":decl_param_counts.owner_wire(),"decl_ctor_counts":decl_ctor_counts.owner_wire(),"decl_templates":decl_templates.owner_wire(),"plan_nodes":plan_nodes.owner_wire(),"plan_cases":plan_cases.owner_wire(),"field_paths":field_paths.owner_wire(),"constants":constants.owner_wire(),"constant_fields":constant_fields.owner_wire(),"names":names.owner_wire()})
    }
}

impl OwnerWire for obs::AdmissionSnapshot {
    fn owner_wire(&self) -> Value {
        let obs::AdmissionSnapshot {
            identity,
            bounds,
            view,
            layout,
        } = self;
        json!({"identity":identity.owner_wire(),"bounds":bounds.owner_wire(),"view":view.owner_wire(),"layout":layout.owner_wire()})
    }
}

impl OwnerWire for obs::ImageSnapshot {
    fn owner_wire(&self) -> Value {
        let obs::ImageSnapshot {
            run_id,
            control,
            control_origin,
            env_root,
            stack_limit,
            outcome,
            initial_fuel,
            remaining_work,
            consumed_work,
            helper_work,
            helper_visits,
            reference_cursor,
            alloc,
            active,
            helper,
            result,
            seg_base,
            data,
            data_fields,
            env,
            frames,
            tasks,
            value_slots,
        } = self;
        json!({"run_id":run_id.owner_wire(),"control":control.owner_wire(),"control_origin":control_origin.owner_wire(),"env_root":env_root.owner_wire(),"stack_limit":stack_limit.owner_wire(),"outcome":outcome.owner_wire(),"initial_fuel":initial_fuel.owner_wire(),"remaining_work":remaining_work.owner_wire(),"consumed_work":consumed_work.owner_wire(),"helper_work":helper_work.owner_wire(),"helper_visits":helper_visits.owner_wire(),"reference_cursor":reference_cursor.owner_wire(),"alloc":alloc.owner_wire(),"active":active.owner_wire(),"helper":helper.owner_wire(),"result":result.owner_wire(),"seg_base":seg_base.owner_wire(),"data":data.owner_wire(),"data_fields":data_fields.owner_wire(),"env":env.owner_wire(),"frames":frames.owner_wire(),"tasks":tasks.owner_wire(),"value_slots":value_slots.owner_wire()})
    }
}

impl OwnerWire for obs::HostSnapshot {
    fn owner_wire(&self) -> Value {
        let obs::HostSnapshot { admission, image } = self;
        json!({"admission":admission.owner_wire(),"image":image.owner_wire()})
    }
}

/// Lossless step-boundary projection: `AdmissionSnapshot` is immutable across all
/// micro-steps of a run (`complete_observation::record_step` takes `&Admitted`) and
/// is retained in full on `complete_trace["initial"]` and `complete_trace["returned"]`.
pub(super) fn step_host_snapshot_wire(snapshot: &obs::HostSnapshot) -> Value {
    json!({"image": snapshot.image.owner_wire()})
}

/// Deterministic lossless reconstruction of a full `HostSnapshot` wire object
/// (`{"admission": ..., "image": ...}`) from the trace's shared `initial["admission"]`
/// and any normalized step or terminal boundary (`{"image": ...}`).
pub fn reconstruct_step_host_snapshot(shared_admission: &Value, step_boundary: &Value) -> Value {
    json!({
        "admission": shared_admission,
        "image": step_boundary["image"],
    })
}

impl OwnerWire for obs::CompleteStepRecord {
    fn owner_wire(&self) -> Value {
        let obs::CompleteStepRecord {
            before,
            event,
            after,
        } = self;
        json!({"before":step_host_snapshot_wire(before),"event":event.owner_wire(),"after":step_host_snapshot_wire(after)})
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum BoundedWireError {
    SerializationLimitExceeded {
        limit: usize,
    },
    OutputNotUtf8,
    Serialize {
        field: &'static str,
        message: String,
    },
}

/// Checked UTF-8 byte writer enforcing both per-projection capacity
/// (`admission.bounds.serialization_bytes`) via `with_projection_budget` and
/// cumulative trace capacity (`limit`). Zero is a real zero bound (not
/// unlimited). Every write checks cumulative byte length with `checked_add`
/// against `active_limit` before extending `buf`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct BoundedJsonWriter {
    buf: Vec<u8>,
    limit: usize,
    active_limit: usize,
    exceeded_limit: Option<usize>,
}

impl BoundedJsonWriter {
    pub(super) fn new(limit: usize) -> Self {
        Self {
            buf: Vec::new(),
            limit,
            active_limit: limit,
            exceeded_limit: None,
        }
    }
    pub(super) fn limit(&self) -> usize {
        self.limit
    }
    pub(super) fn as_bytes(&self) -> &[u8] {
        &self.buf
    }
    pub(super) fn checkpoint(&self) -> usize {
        self.buf.len()
    }
    pub(super) fn rollback(&mut self, checkpoint: usize) {
        self.buf.truncate(checkpoint);
        self.active_limit = self.limit;
        self.exceeded_limit = None;
    }
    pub(super) fn limit_error(&self) -> BoundedWireError {
        BoundedWireError::SerializationLimitExceeded {
            limit: self.exceeded_limit.unwrap_or(self.active_limit),
        }
    }
    pub(super) fn with_projection_budget<F>(
        &mut self,
        per_projection_limit: usize,
        f: F,
    ) -> Result<(), BoundedWireError>
    where
        F: FnOnce(&mut Self) -> Result<(), BoundedWireError>,
    {
        if per_projection_limit == 0 {
            self.exceeded_limit = Some(0);
            return Err(self.limit_error());
        }
        let prev_active = self.active_limit;
        let projection_ceiling = self.buf.len().saturating_add(per_projection_limit);
        self.active_limit = prev_active.min(projection_ceiling);
        let result = f(self);
        self.active_limit = prev_active;
        if result.is_err() && self.exceeded_limit.is_some() {
            if projection_ceiling <= prev_active {
                self.exceeded_limit = Some(per_projection_limit);
            } else {
                self.exceeded_limit = Some(prev_active);
            }
            return Err(self.limit_error());
        }
        result
    }
    pub(super) fn write_raw(&mut self, bytes: &[u8]) -> Result<(), BoundedWireError> {
        std::io::Write::write_all(self, bytes).map_err(|_| self.limit_error())
    }
    pub(super) fn write_serialize<T: ?Sized + serde::Serialize>(
        &mut self,
        value: &T,
        field: &'static str,
    ) -> Result<(), BoundedWireError> {
        if self.buf.len() >= self.active_limit {
            if self.exceeded_limit.is_none() {
                self.exceeded_limit = Some(self.active_limit);
            }
            return Err(self.limit_error());
        }
        serde_json::to_writer(&mut *self, value).map_err(|error| {
            if self.exceeded_limit.is_some() {
                self.limit_error()
            } else {
                BoundedWireError::Serialize {
                    field,
                    message: error.to_string(),
                }
            }
        })
    }
    pub(super) fn write_owner_wire<T: OwnerWire>(
        &mut self,
        value: &T,
        field: &'static str,
    ) -> Result<(), BoundedWireError> {
        if self.buf.len() >= self.active_limit {
            if self.exceeded_limit.is_none() {
                self.exceeded_limit = Some(self.active_limit);
            }
            return Err(self.limit_error());
        }
        let encoded = value.owner_wire();
        self.write_serialize(&encoded, field)
    }
    pub(super) fn write_owner_wire_slice<T: OwnerWire>(
        &mut self,
        slice: &[T],
        field: &'static str,
    ) -> Result<(), BoundedWireError> {
        self.write_raw(b"[")?;
        for (idx, item) in slice.iter().enumerate() {
            if idx > 0 {
                self.write_raw(b",")?;
            }
            self.write_owner_wire(item, field)?;
        }
        self.write_raw(b"]")
    }
    pub(super) fn write_layout_snapshot(
        &mut self,
        layout: &obs::LayoutSnapshot,
    ) -> Result<(), BoundedWireError> {
        let obs::LayoutSnapshot {
            layout_version,
            field_id,
            widths,
            scalar_layout_id,
            bounds,
            capacities,
            boundary_capacities_present,
        } = layout;
        self.write_raw(b"{\"boundary_capacities_present\":")?;
        self.write_owner_wire(
            boundary_capacities_present,
            "layout.boundary_capacities_present",
        )?;
        self.write_raw(b",\"bounds\":")?;
        self.write_owner_wire(bounds, "layout.bounds")?;
        self.write_raw(b",\"capacities\":")?;
        self.write_owner_wire(capacities, "layout.capacities")?;
        self.write_raw(b",\"field_id\":")?;
        self.write_owner_wire(field_id, "layout.field_id")?;
        self.write_raw(b",\"layout_version\":")?;
        self.write_owner_wire(layout_version, "layout.layout_version")?;
        self.write_raw(b",\"scalar_layout_id\":")?;
        self.write_owner_wire(scalar_layout_id, "layout.scalar_layout_id")?;
        self.write_raw(b",\"widths\":")?;
        self.write_owner_wire(widths, "layout.widths")?;
        self.write_raw(b"}")
    }
    pub(super) fn write_program_snapshot(
        &mut self,
        view: &obs::ProgramSnapshot,
    ) -> Result<(), BoundedWireError> {
        let obs::ProgramSnapshot {
            entry,
            expressions,
            occurrences,
            occurrence_paths,
            children,
            functions,
            parameters,
            arms,
            patterns,
            pattern_children,
            types,
            type_args,
            constructors,
            ctor_fields,
            decl_param_counts,
            decl_ctor_counts,
            decl_templates,
            plan_nodes,
            plan_cases,
            field_paths,
            constants,
            constant_fields,
            names,
        } = view;
        self.write_raw(b"{\"arms\":")?;
        self.write_owner_wire_slice(arms, "view.arms")?;
        self.write_raw(b",\"children\":")?;
        self.write_owner_wire_slice(children, "view.children")?;
        self.write_raw(b",\"constant_fields\":")?;
        self.write_owner_wire_slice(constant_fields, "view.constant_fields")?;
        self.write_raw(b",\"constants\":")?;
        self.write_owner_wire_slice(constants, "view.constants")?;
        self.write_raw(b",\"constructors\":")?;
        self.write_owner_wire_slice(constructors, "view.constructors")?;
        self.write_raw(b",\"ctor_fields\":")?;
        self.write_owner_wire_slice(ctor_fields, "view.ctor_fields")?;
        self.write_raw(b",\"decl_ctor_counts\":")?;
        self.write_owner_wire_slice(decl_ctor_counts, "view.decl_ctor_counts")?;
        self.write_raw(b",\"decl_param_counts\":")?;
        self.write_owner_wire_slice(decl_param_counts, "view.decl_param_counts")?;
        self.write_raw(b",\"decl_templates\":")?;
        self.write_owner_wire_slice(decl_templates, "view.decl_templates")?;
        self.write_raw(b",\"entry\":")?;
        self.write_owner_wire(entry, "view.entry")?;
        self.write_raw(b",\"expressions\":")?;
        self.write_owner_wire_slice(expressions, "view.expressions")?;
        self.write_raw(b",\"field_paths\":")?;
        self.write_owner_wire_slice(field_paths, "view.field_paths")?;
        self.write_raw(b",\"functions\":")?;
        self.write_owner_wire_slice(functions, "view.functions")?;
        self.write_raw(b",\"names\":")?;
        self.write_owner_wire_slice(names, "view.names")?;
        self.write_raw(b",\"occurrence_paths\":")?;
        self.write_owner_wire_slice(occurrence_paths, "view.occurrence_paths")?;
        self.write_raw(b",\"occurrences\":")?;
        self.write_owner_wire_slice(occurrences, "view.occurrences")?;
        self.write_raw(b",\"parameters\":")?;
        self.write_owner_wire_slice(parameters, "view.parameters")?;
        self.write_raw(b",\"pattern_children\":")?;
        self.write_owner_wire_slice(pattern_children, "view.pattern_children")?;
        self.write_raw(b",\"patterns\":")?;
        self.write_owner_wire_slice(patterns, "view.patterns")?;
        self.write_raw(b",\"plan_cases\":")?;
        self.write_owner_wire_slice(plan_cases, "view.plan_cases")?;
        self.write_raw(b",\"plan_nodes\":")?;
        self.write_owner_wire_slice(plan_nodes, "view.plan_nodes")?;
        self.write_raw(b",\"type_args\":")?;
        self.write_owner_wire_slice(type_args, "view.type_args")?;
        self.write_raw(b",\"types\":")?;
        self.write_owner_wire_slice(types, "view.types")?;
        self.write_raw(b"}")
    }
    pub(super) fn write_admission_snapshot(
        &mut self,
        admission: &obs::AdmissionSnapshot,
    ) -> Result<(), BoundedWireError> {
        let obs::AdmissionSnapshot {
            identity,
            bounds,
            view,
            layout,
        } = admission;
        self.write_raw(b"{\"bounds\":")?;
        self.write_owner_wire(bounds, "admission.bounds")?;
        self.write_raw(b",\"identity\":")?;
        self.write_owner_wire(identity, "admission.identity")?;
        self.write_raw(b",\"layout\":")?;
        self.write_layout_snapshot(layout)?;
        self.write_raw(b",\"view\":")?;
        self.write_program_snapshot(view)?;
        self.write_raw(b"}")
    }
    pub(super) fn write_image_snapshot(
        &mut self,
        image: &obs::ImageSnapshot,
    ) -> Result<(), BoundedWireError> {
        let obs::ImageSnapshot {
            run_id,
            control,
            control_origin,
            env_root,
            stack_limit,
            outcome,
            initial_fuel,
            remaining_work,
            consumed_work,
            helper_work,
            helper_visits,
            reference_cursor,
            alloc,
            active,
            helper,
            result,
            seg_base,
            data,
            data_fields,
            env,
            frames,
            tasks,
            value_slots,
        } = image;
        self.write_raw(b"{\"active\":")?;
        self.write_owner_wire(active, "image.active")?;
        self.write_raw(b",\"alloc\":")?;
        self.write_owner_wire(alloc, "image.alloc")?;
        self.write_raw(b",\"consumed_work\":")?;
        self.write_owner_wire(consumed_work, "image.consumed_work")?;
        self.write_raw(b",\"control\":")?;
        self.write_owner_wire(control, "image.control")?;
        self.write_raw(b",\"control_origin\":")?;
        self.write_owner_wire(control_origin, "image.control_origin")?;
        self.write_raw(b",\"data\":")?;
        self.write_owner_wire_slice(data, "image.data")?;
        self.write_raw(b",\"data_fields\":")?;
        self.write_owner_wire_slice(data_fields, "image.data_fields")?;
        self.write_raw(b",\"env\":")?;
        self.write_owner_wire_slice(env, "image.env")?;
        self.write_raw(b",\"env_root\":")?;
        self.write_owner_wire(env_root, "image.env_root")?;
        self.write_raw(b",\"frames\":")?;
        self.write_owner_wire_slice(frames, "image.frames")?;
        self.write_raw(b",\"helper\":")?;
        self.write_owner_wire(helper, "image.helper")?;
        self.write_raw(b",\"helper_visits\":")?;
        self.write_owner_wire(helper_visits, "image.helper_visits")?;
        self.write_raw(b",\"helper_work\":")?;
        self.write_owner_wire(helper_work, "image.helper_work")?;
        self.write_raw(b",\"initial_fuel\":")?;
        self.write_owner_wire(initial_fuel, "image.initial_fuel")?;
        self.write_raw(b",\"outcome\":")?;
        self.write_owner_wire(outcome, "image.outcome")?;
        self.write_raw(b",\"reference_cursor\":")?;
        self.write_owner_wire(reference_cursor, "image.reference_cursor")?;
        self.write_raw(b",\"remaining_work\":")?;
        self.write_owner_wire(remaining_work, "image.remaining_work")?;
        self.write_raw(b",\"result\":")?;
        self.write_owner_wire(result, "image.result")?;
        self.write_raw(b",\"run_id\":")?;
        self.write_owner_wire(run_id, "image.run_id")?;
        self.write_raw(b",\"seg_base\":")?;
        self.write_owner_wire(seg_base, "image.seg_base")?;
        self.write_raw(b",\"stack_limit\":")?;
        self.write_owner_wire(stack_limit, "image.stack_limit")?;
        self.write_raw(b",\"tasks\":")?;
        self.write_owner_wire_slice(tasks, "image.tasks")?;
        self.write_raw(b",\"value_slots\":")?;
        self.write_owner_wire_slice(value_slots, "image.value_slots")?;
        self.write_raw(b"}")
    }
    pub(super) fn write_host_snapshot(
        &mut self,
        snapshot: &obs::HostSnapshot,
    ) -> Result<(), BoundedWireError> {
        self.write_raw(b"{\"admission\":")?;
        self.write_admission_snapshot(&snapshot.admission)?;
        self.write_raw(b",\"image\":")?;
        self.write_image_snapshot(&snapshot.image)?;
        self.write_raw(b"}")
    }
    pub(super) fn write_step_host_snapshot(
        &mut self,
        snapshot: &obs::HostSnapshot,
    ) -> Result<(), BoundedWireError> {
        self.write_raw(b"{\"image\":")?;
        self.write_image_snapshot(&snapshot.image)?;
        self.write_raw(b"}")
    }
    pub(super) fn write_step_record(
        &mut self,
        record: &obs::CompleteStepRecord,
    ) -> Result<(), BoundedWireError> {
        self.write_raw(b"{\"after\":")?;
        self.write_step_host_snapshot(&record.after)?;
        self.write_raw(b",\"before\":")?;
        self.write_step_host_snapshot(&record.before)?;
        self.write_raw(b",\"event\":")?;
        self.write_owner_wire(&record.event, "record.event")?;
        self.write_raw(b"}")
    }
    pub(super) fn write_terminal_boundary(
        &mut self,
        before: &obs::HostSnapshot,
        after: &obs::HostSnapshot,
    ) -> Result<(), BoundedWireError> {
        self.write_raw(b"{\"after\":")?;
        self.write_step_host_snapshot(after)?;
        self.write_raw(b",\"before\":")?;
        self.write_step_host_snapshot(before)?;
        self.write_raw(b"}")
    }
    pub(super) fn parse_value(&self, field: &'static str) -> Result<Value, BoundedWireError> {
        if self.exceeded_limit.is_some() {
            return Err(self.limit_error());
        }
        parse_bounded_slice(&self.buf, self.limit, field)
    }
    pub(super) fn into_value(self, field: &'static str) -> Result<Value, BoundedWireError> {
        self.parse_value(field)
    }
}

impl std::io::Write for BoundedJsonWriter {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        self.write_all(bytes)?;
        Ok(bytes.len())
    }
    fn write_all(&mut self, bytes: &[u8]) -> std::io::Result<()> {
        let next = self
            .buf
            .len()
            .checked_add(bytes.len())
            .filter(|n| *n <= self.active_limit);
        match next {
            Some(_) => {
                self.buf.extend_from_slice(bytes);
                Ok(())
            }
            None => {
                if self.exceeded_limit.is_none() {
                    self.exceeded_limit = Some(self.active_limit);
                }
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "complete trace serialization limit exceeded",
                ))
            }
        }
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

pub(super) fn parse_bounded_slice(
    bytes: &[u8],
    limit: usize,
    field: &'static str,
) -> Result<Value, BoundedWireError> {
    let text = std::str::from_utf8(bytes).map_err(|_| BoundedWireError::OutputNotUtf8)?;
    crate::json::parse_limited(text, limit, "complete trace serialization limit exceeded").map_err(
        |error| BoundedWireError::Serialize {
            field,
            message: error.message,
        },
    )
}
