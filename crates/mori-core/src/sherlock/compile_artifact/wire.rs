//! Complete explicit diagnostic wire views of the frozen public owner records.
//! This does not define a native codec, canonical checkpoint or authenticity layer.
use serde_json::{json, Value};
use std::sync::Arc;
use crate::sherlock::helper_machine::shared::*;
use crate::sherlock::helper_machine::shared::owner_host::{HostObservation, LayoutObservation, WidthTableObservation, ImageObservation};
use crate::sherlock::helper_machine::core_action_consumer::{ConsumerProfile, HostGuardError, HostMeasure, SourceHostBounds};
use crate::sherlock::helper_machine::raw::{ConstRow as RawConstRow, DataRow as RawDataRow, ConstFieldSlice, ConstFieldPos, FieldSlice, FieldPos};

pub trait OwnerWire { fn owner_wire(&self) -> Value; }
impl OwnerWire for str { fn owner_wire(&self) -> Value { Value::String(self.to_owned()) } }
macro_rules! decimal_wire { ($($ty:ty),+ $(,)?) => { $(
    impl OwnerWire for $ty { fn owner_wire(&self) -> Value { Value::String(self.to_string()) } }
)+ }; }
decimal_wire!(usize, u128, i64);
macro_rules! json_wire { ($($ty:ty),+ $(,)?) => { $(
    impl OwnerWire for $ty { fn owner_wire(&self) -> Value { json!(self) } }
)+ }; }
json_wire!(bool, u8, u32, String, crate::Type);
impl<T: OwnerWire> OwnerWire for Option<T> { fn owner_wire(&self) -> Value {
    self.as_ref().map_or(Value::Null, OwnerWire::owner_wire)
} }
impl<T: OwnerWire> OwnerWire for [T] { fn owner_wire(&self) -> Value {
    Value::Array(self.iter().map(OwnerWire::owner_wire).collect())
} }
impl<T: OwnerWire> OwnerWire for Vec<T> { fn owner_wire(&self) -> Value { self.as_slice().owner_wire() } }
impl<T: OwnerWire> OwnerWire for Arc<T> { fn owner_wire(&self) -> Value { self.as_ref().owner_wire() } }
impl<T: OwnerWire + ?Sized> OwnerWire for &T { fn owner_wire(&self) -> Value { T::owner_wire(*self) } }
impl<A: OwnerWire, B: OwnerWire> OwnerWire for (A, B) { fn owner_wire(&self) -> Value {
    Value::Array(vec![self.0.owner_wire(), self.1.owner_wire()])
} }

impl OwnerWire for ActiveLengths {
    fn owner_wire(&self) -> Value {
        let ActiveLengths { data, data_fields, env, value_slots } = self;
        json!({"data": data.owner_wire(), "data_fields": data_fields.owner_wire(), "env": env.owner_wire(), "value_slots": value_slots.owner_wire()})
    }
}

impl OwnerWire for AdmissionReason {
    fn owner_wire(&self) -> Value {
        match self {
            Self::LayoutWidthTooSmall => json!("LayoutWidthTooSmall"),
            Self::ProgramViewMalformed => json!("ProgramViewMalformed"),
            Self::GrammarDomainMismatch => json!("GrammarDomainMismatch"),
            Self::ProfileMismatch => json!("ProfileMismatch"),
            Self::LayoutBoundsMismatch => json!("LayoutBoundsMismatch"),
        }
    }
}

impl OwnerWire for AllocCounters {
    fn owner_wire(&self) -> Value {
        let AllocCounters { data_nodes, data_fields, environment_cells } = self;
        json!({"data_nodes": data_nodes.owner_wire(), "data_fields": data_fields.owner_wire(), "environment_cells": environment_cells.owner_wire()})
    }
}

impl OwnerWire for AppendPhase {
    fn owner_wire(&self) -> Value {
        match self {
            Self::Copy => json!("Copy"),
            Self::Tail => json!("Tail"),
            Self::Done => json!("Done"),
        }
    }
}

impl OwnerWire for ArmRowC {
    fn owner_wire(&self) -> Value {
        let ArmRowC { pattern, body } = self;
        json!({"pattern": pattern.owner_wire(), "body": body.owner_wire()})
    }
}

impl OwnerWire for BinaryOp {
    fn owner_wire(&self) -> Value {
        match self {
            Self::Add => json!("Add"),
            Self::Sub => json!("Sub"),
            Self::Mul => json!("Mul"),
            Self::Eq => json!("Eq"),
            Self::Lt => json!("Lt"),
            Self::Cons => json!("Cons"),
            Self::Branch => json!("Branch"),
        }
    }
}

impl OwnerWire for BindPhase {
    fn owner_wire(&self) -> Value {
        match self {
            Self::Pairing => json!("Pairing"),
            Self::UnwindType => json!("UnwindType"),
            Self::UnwindDup => json!("UnwindDup"),
            Self::UnwindInsert => json!("UnwindInsert"),
            Self::Propagate => json!("Propagate"),
            Self::Finished => json!("Finished"),
        }
    }
}

impl OwnerWire for BindStatus {
    fn owner_wire(&self) -> Value {
        match self {
            Self::Ok => json!("Ok"),
            Self::Bad => json!("Bad"),
        }
    }
}

impl OwnerWire for BoolSelector {
    fn owner_wire(&self) -> Value {
        match self {
            Self::True => json!("True"),
            Self::False => json!("False"),
            Self::NotBool => json!("NotBool"),
        }
    }
}

impl OwnerWire for BoundBinding {
    fn owner_wire(&self) -> Value {
        match self {
            Self::Bad => json!("Bad"),
            Self::Ok { env } => json!({"Ok": {"env": env.owner_wire()}}),
        }
    }
}

impl OwnerWire for BoundsC {
    fn owner_wire(&self) -> Value {
        let BoundsC { code_rows, function_rows, code_child_slots, parameter_slots, pattern_rows, match_plan_rows, match_arm_rows, type_rows, constructor_rows, constructor_field_slots, max_arity, constant_nodes, constant_field_slots, data_nodes, data_field_slots, environment_cells, frames, helper_tasks, serialization_bytes, max_reference_work, max_helper_work, step_cap } = self;
        json!({"code_rows": code_rows.owner_wire(), "function_rows": function_rows.owner_wire(), "code_child_slots": code_child_slots.owner_wire(), "parameter_slots": parameter_slots.owner_wire(), "pattern_rows": pattern_rows.owner_wire(), "match_plan_rows": match_plan_rows.owner_wire(), "match_arm_rows": match_arm_rows.owner_wire(), "type_rows": type_rows.owner_wire(), "constructor_rows": constructor_rows.owner_wire(), "constructor_field_slots": constructor_field_slots.owner_wire(), "max_arity": max_arity.owner_wire(), "constant_nodes": constant_nodes.owner_wire(), "constant_field_slots": constant_field_slots.owner_wire(), "data_nodes": data_nodes.owner_wire(), "data_field_slots": data_field_slots.owner_wire(), "environment_cells": environment_cells.owner_wire(), "frames": frames.owner_wire(), "helper_tasks": helper_tasks.owner_wire(), "serialization_bytes": serialization_bytes.owner_wire(), "max_reference_work": max_reference_work.owner_wire(), "max_helper_work": max_helper_work.owner_wire(), "step_cap": step_cap.owner_wire()})
    }
}

impl OwnerWire for CapacityKind {
    fn owner_wire(&self) -> Value {
        match self {
            Self::CodeRows => json!("CodeRows"),
            Self::FunctionRows => json!("FunctionRows"),
            Self::CodeChildSlots => json!("CodeChildSlots"),
            Self::ParameterSlots => json!("ParameterSlots"),
            Self::PatternRows => json!("PatternRows"),
            Self::MatchArmRows => json!("MatchArmRows"),
            Self::ConstantNodes => json!("ConstantNodes"),
            Self::ConstantFieldSlots => json!("ConstantFieldSlots"),
            Self::MaxArity => json!("MaxArity"),
            Self::NameRows => json!("NameRows"),
            Self::NameBytes => json!("NameBytes"),
            Self::OccurrencePathSlots => json!("OccurrencePathSlots"),
            Self::ValueSlots => json!("ValueSlots"),
            Self::MaxReferenceWork => json!("MaxReferenceWork"),
            Self::Frames => json!("Frames"),
            Self::SerializationBytes => json!("SerializationBytes"),
            Self::PhysicalBoundaryEnvelope => json!("PhysicalBoundaryEnvelope"),
            Self::StepCap => json!("StepCap"),
            Self::CounterWidth { field } => json!({"CounterWidth": {"field": field.owner_wire()}}),
        }
    }
}

impl OwnerWire for CaseMatching {
    fn owner_wire(&self) -> Value {
        match self {
            Self::NoMatch => json!("NoMatch"),
            Self::Matched { env } => json!({"Matched": {"env": env.owner_wire()}}),
        }
    }
}

impl OwnerWire for ConstFieldPos {
    fn owner_wire(&self) -> Value {
        self.0.owner_wire()
    }
}

impl OwnerWire for ConstFieldSlice {
    fn owner_wire(&self) -> Value {
        let ConstFieldSlice { start, len } = self;
        json!({"start": start.owner_wire(), "len": len.owner_wire()})
    }
}

impl OwnerWire for ConsumerProfile {
    fn owner_wire(&self) -> Value {
        match self {
            Self::SherlockKA1 => json!("SherlockKA1"),
            Self::SherlockQuintA1 => json!("SherlockQuintA1"),
            Self::SherlockHelperMicrostep1 => json!("SherlockHelperMicrostep1"),
        }
    }
}

impl OwnerWire for ControlC {
    fn owner_wire(&self) -> Value {
        match self {
            Self::Eval { expr } => json!({"Eval": {"expr": expr.owner_wire()}}),
            Self::Return { value } => json!({"Return": {"value": value.owner_wire()}}),
            Self::ScheduleNext { frame, next, env } => json!({"ScheduleNext": {"frame": frame.owner_wire(), "next": next.owner_wire(), "env": env.owner_wire()}}),
            Self::Invoke { function_name, args } => json!({"Invoke": {"function_name": function_name.owner_wire(), "args": args.owner_wire()}}),
            Self::Enter { function, args } => json!({"Enter": {"function": function.owner_wire(), "args": args.owner_wire()}}),
            Self::Bound { binding, return_type, body } => json!({"Bound": {"binding": binding.owner_wire(), "return_type": return_type.owner_wire(), "body": body.owner_wire()}}),
            Self::Cases { value, arms, saved_env } => json!({"Cases": {"value": value.owner_wire(), "arms": arms.owner_wire(), "saved_env": saved_env.owner_wire()}}),
            Self::CaseResult { matching, value, body, rest, saved_env } => json!({"CaseResult": {"matching": matching.owner_wire(), "value": value.owner_wire(), "body": body.owner_wire(), "rest": rest.owner_wire(), "saved_env": saved_env.owner_wire()}}),
            Self::PrimResult { result } => json!({"PrimResult": {"result": result.owner_wire()}}),
            Self::Error { code } => json!({"Error": {"code": code.owner_wire()}}),
            Self::Materialize { type_id, ctor_id, fields } => json!({"Materialize": {"type_id": type_id.owner_wire(), "ctor_id": ctor_id.owner_wire(), "fields": fields.owner_wire()}}),
        }
    }
}

impl OwnerWire for CounterWidthField {
    fn owner_wire(&self) -> Value {
        match self {
            Self::WorkCounter => json!("WorkCounter"),
            Self::HelperCounter => json!("HelperCounter"),
            Self::ReferenceCursor => json!("ReferenceCursor"),
        }
    }
}

impl OwnerWire for DigestC {
    fn owner_wire(&self) -> Value {
        match self {
            Self::Unselected => json!("Unselected"),
            Self::Selected { format, bytes } => json!({"Selected": {"format": format.owner_wire(), "bytes": bytes.owner_wire()}}),
        }
    }
}

impl OwnerWire for EnvCellC {
    fn owner_wire(&self) -> Value {
        let EnvCellC { binder, value, previous } = self;
        json!({"binder": binder.owner_wire(), "value": value.owner_wire(), "previous": previous.owner_wire()})
    }
}

impl OwnerWire for EventKind {
    fn owner_wire(&self) -> Value {
        match self {
            Self::LegacyComplete => json!("LegacyComplete"),
            Self::HelperEnter => json!("HelperEnter"),
            Self::HelperVisit => json!("HelperVisit"),
            Self::HelperExit => json!("HelperExit"),
            Self::HelperFail => json!("HelperFail"),
            Self::Bookkeeping => json!("Bookkeeping"),
            Self::Publish => json!("Publish"),
            Self::Startup => json!("Startup"),
        }
    }
}

impl OwnerWire for ExhaustReason {
    fn owner_wire(&self) -> Value {
        match self {
            Self::Fuel => json!("Fuel"),
            Self::DataNodes => json!("DataNodes"),
            Self::DataFields => json!("DataFields"),
            Self::EnvironmentCells => json!("EnvironmentCells"),
            Self::HelperTasks => json!("HelperTasks"),
            Self::HelperWork => json!("HelperWork"),
            Self::SerializationBytes => json!("SerializationBytes"),
        }
    }
}

impl OwnerWire for ExpressionRowC {
    fn owner_wire(&self) -> Value {
        match self {
            Self::Literal { constant } => json!({"Literal": {"constant": constant.owner_wire()}}),
            Self::Var { binder } => json!({"Var": {"binder": binder.owner_wire()}}),
            Self::Let { binder, value, body } => json!({"Let": {"binder": binder.owner_wire(), "value": value.owner_wire(), "body": body.owner_wire()}}),
            Self::Binary { op, left, right } => json!({"Binary": {"op": op.owner_wire(), "left": left.owner_wire(), "right": right.owner_wire()}}),
            Self::Unary { op, operand } => json!({"Unary": {"op": op.owner_wire(), "operand": operand.owner_wire()}}),
            Self::If { cond, then_expr, else_expr } => json!({"If": {"cond": cond.owner_wire(), "then_expr": then_expr.owner_wire(), "else_expr": else_expr.owner_wire()}}),
            Self::Call { function_name, args } => json!({"Call": {"function_name": function_name.owner_wire(), "args": args.owner_wire()}}),
            Self::Match { scrutinee, arms, plan } => json!({"Match": {"scrutinee": scrutinee.owner_wire(), "arms": arms.owner_wire(), "plan": plan.owner_wire()}}),
            Self::Construct { type_id, ctor_id, fields, constant } => json!({"Construct": {"type_id": type_id.owner_wire(), "ctor_id": ctor_id.owner_wire(), "fields": fields.owner_wire(), "constant": constant.owner_wire()}}),
        }
    }
}

impl OwnerWire for FeatureKind {
    fn owner_wire(&self) -> Value {
        match self {
            Self::GenericAdt => json!("GenericAdt"),
            Self::FinancialSyntax => json!("FinancialSyntax"),
            Self::FinancialInput => json!("FinancialInput"),
            Self::BoundedEncodingWithoutWidths => json!("BoundedEncodingWithoutWidths"),
            Self::BoundarySection => json!("BoundarySection"),
        }
    }
}

impl OwnerWire for FieldPos {
    fn owner_wire(&self) -> Value {
        self.0.owner_wire()
    }
}

impl OwnerWire for FieldSlice {
    fn owner_wire(&self) -> Value {
        let FieldSlice { start, len } = self;
        json!({"start": start.owner_wire(), "len": len.owner_wire()})
    }
}

impl OwnerWire for FinanceInput {
    fn owner_wire(&self) -> Value {
        match self {
            Self::Absent => json!("Absent"),
            Self::TypedExtensionReserved => json!("TypedExtensionReserved"),
        }
    }
}

impl OwnerWire for FrameRowC {
    fn owner_wire(&self) -> Value {
        match self {
            Self::LetF { binder, body, saved_env, origin } => json!({"LetF": {"binder": binder.owner_wire(), "body": body.owner_wire(), "saved_env": saved_env.owner_wire(), "origin": origin.owner_wire()}}),
            Self::LeftF { op, right, saved_env, origin } => json!({"LeftF": {"op": op.owner_wire(), "right": right.owner_wire(), "saved_env": saved_env.owner_wire(), "origin": origin.owner_wire()}}),
            Self::RightF { op, left, origin } => json!({"RightF": {"op": op.owner_wire(), "left": left.owner_wire(), "origin": origin.owner_wire()}}),
            Self::UnaryF { op, origin } => json!({"UnaryF": {"op": op.owner_wire(), "origin": origin.owner_wire()}}),
            Self::IfF { then_expr, else_expr, saved_env, origin } => json!({"IfF": {"then_expr": then_expr.owner_wire(), "else_expr": else_expr.owner_wire(), "saved_env": saved_env.owner_wire(), "origin": origin.owner_wire()}}),
            Self::MatchF { arms, saved_env, origin } => json!({"MatchF": {"arms": arms.owner_wire(), "saved_env": saved_env.owner_wire(), "origin": origin.owner_wire()}}),
            Self::ArgsF { function_name, completed, rest, saved_env, origin } => json!({"ArgsF": {"function_name": function_name.owner_wire(), "completed": completed.owner_wire(), "rest": rest.owner_wire(), "saved_env": saved_env.owner_wire(), "origin": origin.owner_wire()}}),
            Self::ReturnF { return_type, saved_env, origin } => json!({"ReturnF": {"return_type": return_type.owner_wire(), "saved_env": saved_env.owner_wire(), "origin": origin.owner_wire()}}),
            Self::ScopeF { saved_env, origin } => json!({"ScopeF": {"saved_env": saved_env.owner_wire(), "origin": origin.owner_wire()}}),
            Self::ConstructorFieldsF { type_id, ctor_id, index, field_type, checked, completed, rest, saved_env, origin } => json!({"ConstructorFieldsF": {"type_id": type_id.owner_wire(), "ctor_id": ctor_id.owner_wire(), "index": index.owner_wire(), "field_type": field_type.owner_wire(), "checked": checked.owner_wire(), "completed": completed.owner_wire(), "rest": rest.owner_wire(), "saved_env": saved_env.owner_wire(), "origin": origin.owner_wire()}}),
        }
    }
}

impl OwnerWire for FunctionRowC {
    fn owner_wire(&self) -> Value {
        let FunctionRowC { function_name, params, return_type, body } = self;
        json!({"function_name": function_name.owner_wire(), "params": params.owner_wire(), "return_type": return_type.owner_wire(), "body": body.owner_wire()})
    }
}

impl OwnerWire for GrammarDomain {
    fn owner_wire(&self) -> Value {
        match self {
            Self::DedicatedSorts => json!("DedicatedSorts"),
            Self::GenericNominalExtensionReserved => json!("GenericNominalExtensionReserved"),
        }
    }
}

impl OwnerWire for HelperCommon {
    fn owner_wire(&self) -> Value {
        let HelperCommon { site, origin_reference_cursor, parent_resume } = self;
        json!({"site": site.owner_wire(), "origin_reference_cursor": origin_reference_cursor.owner_wire(), "parent_resume": parent_resume.owner_wire()})
    }
}

impl OwnerWire for HelperControlC {
    fn owner_wire(&self) -> Value {
        match self {
            Self::Scalar { common, mode, operand_a, operand_b, expected, phase, reg_a, staged } => json!({"Scalar": {"common": common.owner_wire(), "mode": mode.owner_wire(), "operand_a": operand_a.owner_wire(), "operand_b": operand_b.owner_wire(), "expected": expected.owner_wire(), "phase": phase.owner_wire(), "reg_a": reg_a.owner_wire(), "staged": staged.owner_wire()}}),
            Self::EnvGet { common, binder, cursor, found, status } => json!({"EnvGet": {"common": common.owner_wire(), "binder": binder.owner_wire(), "cursor": cursor.owner_wire(), "found": found.owner_wire(), "status": status.owner_wire()}}),
            Self::EnvPut { common, binder, value, base, new_root, status } => json!({"EnvPut": {"common": common.owner_wire(), "binder": binder.owner_wire(), "value": value.owner_wire(), "base": base.owner_wire(), "new_root": new_root.owner_wire(), "status": status.owner_wire()}}),
            Self::Find { common, function_name, row_cursor, found, status } => json!({"Find": {"common": common.owner_wire(), "function_name": function_name.owner_wire(), "row_cursor": row_cursor.owner_wire(), "found": found.owner_wire(), "status": status.owner_wire()}}),
            Self::Bind { common, function, params, args, p_cursor, a_cursor, partial, dup_cursor, phase, status } => json!({"Bind": {"common": common.owner_wire(), "function": function.owner_wire(), "params": params.owner_wire(), "args": args.owner_wire(), "p_cursor": p_cursor.owner_wire(), "a_cursor": a_cursor.owner_wire(), "partial": partial.owner_wire(), "dup_cursor": dup_cursor.owner_wire(), "phase": phase.owner_wire(), "status": status.owner_wire()}}),
            Self::Append { common, src, value, src_cursor, dest_start, staged, phase } => json!({"Append": {"common": common.owner_wire(), "src": src.owner_wire(), "value": value.owner_wire(), "src_cursor": src_cursor.owner_wire(), "dest_start": dest_start.owner_wire(), "staged": staged.owner_wire(), "phase": phase.owner_wire()}}),
            Self::Validate { common, root, status } => json!({"Validate": {"common": common.owner_wire(), "root": root.owner_wire(), "status": status.owner_wire()}}),
            Self::Equal { common, left, right, status } => json!({"Equal": {"common": common.owner_wire(), "left": left.owner_wire(), "right": right.owner_wire(), "status": status.owner_wire()}}),
            Self::Match { common, scrutinee, arm, saved_env, partial, pat_tag, x, y, field_idx, staged, phase, status } => json!({"Match": {"common": common.owner_wire(), "scrutinee": scrutinee.owner_wire(), "arm": arm.owner_wire(), "saved_env": saved_env.owner_wire(), "partial": partial.owner_wire(), "pat_tag": pat_tag.owner_wire(), "x": x.owner_wire(), "y": y.owner_wire(), "field_idx": field_idx.owner_wire(), "staged": staged.owner_wire(), "phase": phase.owner_wire(), "status": status.owner_wire()}}),
            Self::Plan { common, source_match, scrutinee, saved_env, partial, node, phase, after_walk, cur, path, path_cursor, arm, status } => json!({"Plan": {"common": common.owner_wire(), "source_match": source_match.owner_wire(), "scrutinee": scrutinee.owner_wire(), "saved_env": saved_env.owner_wire(), "partial": partial.owner_wire(), "node": node.owner_wire(), "phase": phase.owner_wire(), "after_walk": after_walk.owner_wire(), "cur": cur.owner_wire(), "path": path.owner_wire(), "path_cursor": path_cursor.owner_wire(), "arm": arm.owner_wire(), "status": status.owner_wire()}}),
            Self::Build { common, type_id, ctor_id, src, src_cursor, field_start, staged, node, phase } => json!({"Build": {"common": common.owner_wire(), "type_id": type_id.owner_wire(), "ctor_id": ctor_id.owner_wire(), "src": src.owner_wire(), "src_cursor": src_cursor.owner_wire(), "field_start": field_start.owner_wire(), "staged": staged.owner_wire(), "node": node.owner_wire(), "phase": phase.owner_wire()}}),
        }
    }
}

impl OwnerWire for BuildPhase {
    fn owner_wire(&self) -> Value {
        match self {
            Self::Copy => json!("Copy"),
            Self::Done => json!("Done"),
        }
    }
}

impl OwnerWire for PlanPhase {
    fn owner_wire(&self) -> Value {
        match self {
            Self::Walk => json!("Walk"),
            Self::Test => json!("Test"),
            Self::Bind => json!("Bind"),
            Self::Ready => json!("Ready"),
            Self::NoMatchReady => json!("NoMatchReady"),
            Self::Done => json!("Done"),
        }
    }
}

impl OwnerWire for PlanAfterWalk {
    fn owner_wire(&self) -> Value {
        match self {
            Self::Test => json!("Test"),
            Self::Bind => json!("Bind"),
        }
    }
}

impl OwnerWire for HelperKind {
    fn owner_wire(&self) -> Value {
        match self {
            Self::Scalar => json!("Scalar"),
            Self::EnvGet => json!("EnvGet"),
            Self::EnvPut => json!("EnvPut"),
            Self::Find => json!("Find"),
            Self::Bind => json!("Bind"),
            Self::Append => json!("Append"),
            Self::Validate => json!("Validate"),
            Self::Equal => json!("Equal"),
            Self::Match => json!("Match"),
            Self::Plan => json!("Plan"),
            Self::Build => json!("Build"),
        }
    }
}

impl OwnerWire for HelperResultC {
    fn owner_wire(&self) -> Value {
        match self {
            Self::Validated { ok } => json!({"Validated": {"ok": ok.owner_wire()}}),
            Self::EnvFound { value } => json!({"EnvFound": {"value": value.owner_wire()}}),
            Self::EnvMissing => json!("EnvMissing"),
            Self::EnvUpdated { cell } => json!({"EnvUpdated": {"cell": cell.owner_wire()}}),
            Self::FindFound { function } => json!({"FindFound": {"function": function.owner_wire()}}),
            Self::FindMissing => json!("FindMissing"),
            Self::BindOk { env } => json!({"BindOk": {"env": env.owner_wire()}}),
            Self::BindBad => json!("BindBad"),
            Self::AppendDone { seq } => json!({"AppendDone": {"seq": seq.owner_wire()}}),
            Self::ScalarValue { value } => json!({"ScalarValue": {"value": value.owner_wire()}}),
            Self::ScalarError { code } => json!({"ScalarError": {"code": code.owner_wire()}}),
            Self::ScalarSelector { sel } => json!({"ScalarSelector": {"sel": sel.owner_wire()}}),
            Self::ScalarTypeTest { ok } => json!({"ScalarTypeTest": {"ok": ok.owner_wire()}}),
            Self::EqualDone { value } => json!({"EqualDone": {"value": value.owner_wire()}}),
            Self::MatchMatched { env } => json!({"MatchMatched": {"env": env.owner_wire()}}),
            Self::MatchNoMatch => json!("MatchNoMatch"),
            Self::PlanMatched { arm, env } => json!({"PlanMatched": {"arm": arm.owner_wire(), "env": env.owner_wire()}}),
            Self::PlanNoMatch => json!("PlanNoMatch"),
            Self::BuildDone { value } => json!({"BuildDone": {"value": value.owner_wire()}}),
        }
    }
}

impl OwnerWire for HelperTaskC {
    fn owner_wire(&self) -> Value {
        match self {
            Self::ValidateTask { r, visit, cursor } => json!({"ValidateTask": {"r": r.owner_wire(), "visit": visit.owner_wire(), "cursor": cursor.owner_wire()}}),
            Self::EqualTask { left, right, visit, cursor } => json!({"EqualTask": {"left": left.owner_wire(), "right": right.owner_wire(), "visit": visit.owner_wire(), "cursor": cursor.owner_wire()}}),
            Self::PairTask { binder, sort, arg } => json!({"PairTask": {"binder": binder.owner_wire(), "sort": sort.owner_wire(), "arg": arg.owner_wire()}}),
        }
    }
}

impl OwnerWire for HostGuardError {
    fn owner_wire(&self) -> Value {
        match self {
            Self::InvalidHostBounds { field } => json!({"InvalidHostBounds": {"field": field.owner_wire()}}),
            Self::ProfileRejected { supplied } => json!({"ProfileRejected": {"supplied": supplied.owner_wire()}}),
            Self::GrammarRejected { supplied } => json!({"GrammarRejected": {"supplied": supplied.owner_wire()}}),
            Self::FinanceRejected { supplied } => json!({"FinanceRejected": {"supplied": supplied.owner_wire()}}),
            Self::Exceeded { measure, required, bound } => json!({"Exceeded": {"measure": measure.owner_wire(), "required": required.owner_wire(), "bound": bound.owner_wire()}}),
            Self::CountOverflow { measure } => json!({"CountOverflow": {"measure": measure.owner_wire()}}),
            Self::NoSuchAction { action_index, action_count } => json!({"NoSuchAction": {"action_index": action_index.owner_wire(), "action_count": action_count.owner_wire()}}),
            Self::MissingInput { name } => json!({"MissingInput": {"name": name.owner_wire()}}),
            Self::TypeMismatch { name, declared, supplied } => json!({"TypeMismatch": {"name": name.owner_wire(), "declared": declared.owner_wire(), "supplied": supplied.owner_wire()}}),
            Self::ExtraInput { name } => json!({"ExtraInput": {"name": name.owner_wire()}}),
            Self::PreparationContract { detail } => json!({"PreparationContract": {"detail": detail.owner_wire()}}),
        }
    }
}

impl OwnerWire for HostMeasure {
    fn owner_wire(&self) -> Value {
        match self {
            Self::SourceActions => json!("SourceActions"),
            Self::SourceExpressions => json!("SourceExpressions"),
            Self::SourceStatements => json!("SourceStatements"),
            Self::ActionInputs => json!("ActionInputs"),
            Self::SuppliedInputs => json!("SuppliedInputs"),
            Self::ProjectionOccurrences => json!("ProjectionOccurrences"),
            Self::GeneratedAstNodes => json!("GeneratedAstNodes"),
            Self::AstNesting => json!("AstNesting"),
            Self::InputNameBytes => json!("InputNameBytes"),
            Self::TotalInputNameBytes => json!("TotalInputNameBytes"),
        }
    }
}

impl OwnerWire for HostViolation {
    fn owner_wire(&self) -> Value {
        let HostViolation { kind, clause, rule, locator } = self;
        json!({"kind": kind.owner_wire(), "clause": clause.owner_wire(), "rule": rule.owner_wire(), "locator": locator.owner_wire()})
    }
}

impl OwnerWire for HostViolationKind {
    fn owner_wire(&self) -> Value {
        match self {
            Self::FinancialUnsupportedBeyondGenesis { op } => match op {
                FinUnsupportedOp::Step => json!("FinancialUnsupportedBeyondGenesis.Step"),
                FinUnsupportedOp::Run => json!("FinancialUnsupportedBeyondGenesis.Run"),
                FinUnsupportedOp::Select => json!("FinancialUnsupportedBeyondGenesis.Select"),
                FinUnsupportedOp::Capture => json!("FinancialUnsupportedBeyondGenesis.Capture"),
                FinUnsupportedOp::Codec => json!("FinancialUnsupportedBeyondGenesis.Codec"),
                FinUnsupportedOp::ObservePure => json!("FinancialUnsupportedBeyondGenesis.ObservePure"),
                FinUnsupportedOp::StepInv => json!("FinancialUnsupportedBeyondGenesis.StepInv"),
            },
            Self::StepInvViolated => json!("StepInvViolated"),
            Self::SelectUndefined => json!("SelectUndefined"),
            Self::ReadOutOfBounds => json!("ReadOutOfBounds"),
            Self::WrongVariant => json!("WrongVariant"),
            Self::DisabledRuleReached => json!("DisabledRuleReached"),
            Self::ExtensionRuleReached => json!("ExtensionRuleReached"),
            Self::PhysicalCapacityExceeded => json!("PhysicalCapacityExceeded"),
            Self::ArithmeticUndefined => json!("ArithmeticUndefined"),
        }
    }
}

impl OwnerWire for IdentityC {
    fn owner_wire(&self) -> Value {
        let IdentityC { profile_id, grammar_domain, finance, layout_version, layout_digest, bounds_digest, field_id, program_membership_digest } = self;
        json!({"profile_id": profile_id.owner_wire(), "grammar_domain": grammar_domain.owner_wire(), "finance": finance.owner_wire(), "layout_version": layout_version.owner_wire(), "layout_digest": layout_digest.owner_wire(), "bounds_digest": bounds_digest.owner_wire(), "field_id": field_id.owner_wire(), "program_membership_digest": program_membership_digest.owner_wire()})
    }
}

impl OwnerWire for LayoutCapacitiesC {
    fn owner_wire(&self) -> Value {
        let LayoutCapacitiesC { name_rows, name_bytes, occurrence_path_slots, value_slots } = self;
        json!({"name_rows": name_rows.owner_wire(), "name_bytes": name_bytes.owner_wire(), "occurrence_path_slots": occurrence_path_slots.owner_wire(), "value_slots": value_slots.owner_wire()})
    }
}

impl OwnerWire for LocatorC {
    fn owner_wire(&self) -> Value {
        match self {
            Self::Occurrence { owner_is_function, function, ast_path } => json!({"Occurrence": {"owner_is_function": owner_is_function.owner_wire(), "function": function.owner_wire(), "ast_path": ast_path.owner_wire()}}),
            Self::Table { table, row } => json!({"Table": {"table": table.owner_wire(), "row": row.owner_wire()}}),
            Self::Field { name } => json!({"Field": {"name": name.owner_wire()}}),
        }
    }
}

impl OwnerWire for MatchPhase {
    fn owner_wire(&self) -> Value {
        match self {
            Self::Arm => json!("Arm"),
            Self::Tag => json!("Tag"),
            Self::Distinct => json!("Distinct"),
            Self::Field => json!("Field"),
            Self::Bind => json!("Bind"),
            Self::Ready => json!("Ready"),
            Self::NoMatchReady => json!("NoMatchReady"),
            Self::Done => json!("Done"),
        }
    }
}

impl OwnerWire for NeededNatC {
    fn owner_wire(&self) -> Value {
        match self {
            Self::Exact(item0) => json!({"Exact": item0.owner_wire()}),
            Self::AboveU128 => json!("AboveU128"),
        }
    }
}

impl OwnerWire for NodeKind {
    fn owner_wire(&self) -> Value {
        match self {
            Self::Cons => json!("Cons"),
            Self::Leaf => json!("Leaf"),
            Self::Branch => json!("Branch"),
        }
    }
}

impl OwnerWire for OccurrenceRowC {
    fn owner_wire(&self) -> Value {
        let OccurrenceRowC { owner_is_function, function, path } = self;
        json!({"owner_is_function": owner_is_function.owner_wire(), "function": function.owner_wire(), "path": path.owner_wire()})
    }
}

impl OwnerWire for OutcomeC {
    fn owner_wire(&self) -> Value {
        match self {
            Self::Running => json!("Running"),
            Self::Halted { value } => json!({"Halted": {"value": value.owner_wire()}}),
            Self::Rejected { code } => json!({"Rejected": {"code": code.owner_wire()}}),
            Self::Exhausted { reason } => json!({"Exhausted": {"reason": reason.owner_wire()}}),
        }
    }
}

impl OwnerWire for ParameterRowC {
    fn owner_wire(&self) -> Value {
        let ParameterRowC { binder, sort } = self;
        json!({"binder": binder.owner_wire(), "sort": sort.owner_wire()})
    }
}

impl OwnerWire for PatternRowC {
    fn owner_wire(&self) -> Value {
        match self {
            Self::PVar { x } => json!({"PVar": {"x": x.owner_wire()}}),
            Self::PNil => json!("PNil"),
            Self::PCons { x, y } => json!({"PCons": {"x": x.owner_wire(), "y": y.owner_wire()}}),
            Self::PLeaf { x } => json!({"PLeaf": {"x": x.owner_wire()}}),
            Self::PBranch { x, y } => json!({"PBranch": {"x": x.owner_wire(), "y": y.owner_wire()}}),
            Self::Wildcard => json!("Wildcard"),
            Self::Constructor { type_id, ctor_id, children } => json!({"Constructor": {"type_id": type_id.owner_wire(), "ctor_id": ctor_id.owner_wire(), "children": children.owner_wire()}}),
        }
    }
}


impl OwnerWire for adt::CtorIdC {
    fn owner_wire(&self) -> Value { self.0.owner_wire() }
}
impl OwnerWire for adt::TypeRowC {
    fn owner_wire(&self) -> Value {
        let Self { name_id, decl, args, ctor_start, ctor_len } = self;
        json!({"name_id": name_id.owner_wire(), "decl": decl.owner_wire(), "args": args.owner_wire(), "ctor_start": ctor_start.owner_wire(), "ctor_len": ctor_len.owner_wire()})
    }
}
impl OwnerWire for adt_storage::CtorRowExt {
    fn owner_wire(&self) -> Value {
        let Self { owning_type, name_id, source_order, field_start, field_len } = self;
        json!({"owning_type": owning_type.owner_wire(), "name_id": name_id.owner_wire(), "source_order": source_order.owner_wire(), "field_start": field_start.owner_wire(), "field_len": field_len.owner_wire()})
    }
}
impl OwnerWire for adt_storage::CtorFieldRowExt {
    fn owner_wire(&self) -> Value {
        let Self { name_id, type_id } = self;
        json!({"name_id": name_id.owner_wire(), "type_id": type_id.owner_wire()})
    }
}
impl OwnerWire for owner_ext_type_builder::DeclIdxC {
    fn owner_wire(&self) -> Value { self.0.owner_wire() }
}
impl OwnerWire for owner_ext_type_builder::TypeExprSrc {
    fn owner_wire(&self) -> Value {
        use owner_ext_type_builder::TypeExprSrc;
        enum Frame<'a> {
            Enter(&'a TypeExprSrc),
            BuildApply { name: &'a str, arity: usize },
        }
        let mut work: Vec<Frame<'_>> = vec![Frame::Enter(self)];
        let mut out: Vec<Value> = Vec::new();
        while let Some(frame) = work.pop() {
            match frame {
                Frame::Enter(TypeExprSrc::UInt) => out.push(json!("UInt")),
                Frame::Enter(TypeExprSrc::Bool) => out.push(json!("Bool")),
                Frame::Enter(TypeExprSrc::List) => out.push(json!("List")),
                Frame::Enter(TypeExprSrc::Tree) => out.push(json!("Tree")),
                Frame::Enter(TypeExprSrc::Named { name }) => out.push(json!({"Named": {"name": name.owner_wire()}})),
                Frame::Enter(TypeExprSrc::Param { name }) => out.push(json!({"Param": {"name": name.owner_wire()}})),
                Frame::Enter(TypeExprSrc::Apply { name, args }) => {
                    work.push(Frame::BuildApply { name: name.as_str(), arity: args.len() });
                    for arg in args.iter().rev() {
                        work.push(Frame::Enter(arg));
                    }
                }
                Frame::BuildApply { name, arity } => {
                    let split = out.len().saturating_sub(arity);
                    let args = out.split_off(split);
                    out.push(json!({"Apply": {"name": name.owner_wire(), "args": Value::Array(args)}}));
                }
            }
        }
        out.pop().unwrap_or(Value::Null)
    }
}
impl OwnerWire for owner_ext_type_builder::CtorFieldSrc {
    fn owner_wire(&self) -> Value {
        let Self { name, ty } = self;
        json!({"name": name.owner_wire(), "ty": ty.owner_wire()})
    }
}
impl OwnerWire for owner_ext_type_builder::CtorDeclSrc {
    fn owner_wire(&self) -> Value {
        let Self { name, fields } = self;
        json!({"name": name.owner_wire(), "fields": fields.owner_wire()})
    }
}
impl OwnerWire for owner_ext_type_builder::TypeDeclSrc {
    fn owner_wire(&self) -> Value {
        let Self { name, parameters, ctors } = self;
        json!({"name": name.owner_wire(), "parameters": parameters.owner_wire(), "ctors": ctors.owner_wire()})
    }
}
impl OwnerWire for owner_ext_type_builder::DeclNameIds {
    fn owner_wire(&self) -> Value {
        let Self { type_name_id, ctor_name_ids, field_name_ids } = self;
        json!({"type_name_id": type_name_id.owner_wire(), "ctor_name_ids": ctor_name_ids.owner_wire(), "field_name_ids": field_name_ids.owner_wire()})
    }
}
impl OwnerWire for owner_ext_type_builder::PassBTypeNameIds {
    fn owner_wire(&self) -> Value {
        json!({"name_count": self.name_count.owner_wire(), "decls": self.decls.owner_wire(), "owner_verified": self.is_owner_verified().owner_wire()})
    }
}
impl OwnerWire for owner_ext_type_builder::ExtTypeCapKind {
    fn owner_wire(&self) -> Value {
        match self {
            Self::TypeRows => json!("TypeRows"),
            Self::ConstructorRows => json!("ConstructorRows"),
            Self::ConstructorFieldSlots => json!("ConstructorFieldSlots"),
            Self::PatternChildSlots => json!("PatternChildSlots"),
            Self::MatchPlanRows => json!("MatchPlanRows"),
            Self::PlanCaseSlots => json!("PlanCaseSlots"),
            Self::FieldPathSlots => json!("FieldPathSlots"),
            Self::InputRows => json!("InputRows"),
            Self::AssetRows => json!("AssetRows"),
            Self::StateRows => json!("StateRows"),
            Self::AccountRows => json!("AccountRows"),
            Self::LiabilityRows => json!("LiabilityRows"),
            Self::AwaitSiteRows => json!("AwaitSiteRows"),
            Self::AwaitBinderSlots => json!("AwaitBinderSlots"),
            Self::BalanceCells => json!("BalanceCells"),
            Self::JournalRows => json!("JournalRows"),
            Self::PendingOpRows => json!("PendingOpRows"),
            Self::ObservationsPerInvocation => json!("ObservationsPerInvocation"),
            Self::MaxLifetimeWork => json!("MaxLifetimeWork"),
            Self::ResumeRows => json!("ResumeRows"),
            Self::EffectOrdinalWidth => json!("EffectOrdinalWidth"),
            Self::SegmentOrdinalWidth => json!("SegmentOrdinalWidth"),
        }
    }
}
impl OwnerWire for owner_ext_type_builder::ExtTypeRejectReason {
    fn owner_wire(&self) -> Value {
        match self {
            Self::DuplicateTypeName => json!("DuplicateTypeName"),
            Self::ReservedTypeName => json!("ReservedTypeName"),
            Self::UnknownType => json!("UnknownType"),
            Self::TypeArity => json!("TypeArity"),
            Self::ConstructorArity => json!("ConstructorArity"),
            Self::DuplicateCtorName => json!("DuplicateCtorName"),
            Self::DuplicateFieldName => json!("DuplicateFieldName"),
            Self::UnknownCtor => json!("UnknownCtor"),
            Self::BuiltinConstruct => json!("BuiltinConstruct"),
            Self::PatternArity => json!("PatternArity"),
            Self::DuplicatePatternBinder => json!("DuplicatePatternBinder"),
            Self::ConstantFieldType => json!("ConstantFieldType"),
        }
    }
}
impl OwnerWire for owner_ext_type_builder::ExtTypeFeatureKind {
    fn owner_wire(&self) -> Value {
        match self {
            Self::TypeVariableInSignature => json!("TypeVariableInSignature"),
        }
    }
}
impl OwnerWire for owner_ext_type_builder::ExtTypeBuildError {
    fn owner_wire(&self) -> Value {
        match self {
            Self::CoreTooling(t) => json!({"CoreTooling": t.owner_wire()}),
            Self::ExtCapacityNotAdmitted { kind, needed, limit, locator } => json!({"ExtCapacityNotAdmitted": {"kind": kind.owner_wire(), "needed": needed.owner_wire(), "limit": limit.owner_wire(), "locator": locator.owner_wire()}}),
            Self::ExtAdmissionRejected { reason, detail, locator } => json!({"ExtAdmissionRejected": {"reason": reason.owner_wire(), "detail": detail.owner_wire(), "locator": locator.owner_wire()}}),
            Self::ExtUnsupportedFeature { feature, locator } => json!({"ExtUnsupportedFeature": {"feature": feature.owner_wire(), "locator": locator.owner_wire()}}),
            Self::Host(h) => json!({"Host": h.owner_wire()}),
        }
    }
}
impl OwnerWire for PlanNodeC {
    fn owner_wire(&self) -> Value {
        match self {
            Self::Switch { source_match, path, type_id, cases, default } => json!({"Switch": {"source_match": source_match.owner_wire(), "path": path.owner_wire(), "type_id": type_id.owner_wire(), "cases": cases.owner_wire(), "default": default.owner_wire()}}),
            Self::Bind { source_match, path, binder, next } => json!({"Bind": {"source_match": source_match.owner_wire(), "path": path.owner_wire(), "binder": binder.owner_wire(), "next": next.owner_wire()}}),
            Self::Select { source_match, arm } => json!({"Select": {"source_match": source_match.owner_wire(), "arm": arm.owner_wire()}}),
            Self::NoMatch { source_match } => json!({"NoMatch": {"source_match": source_match.owner_wire()}}),
        }
    }
}

impl OwnerWire for PatternTag {
    fn owner_wire(&self) -> Value {
        match self {
            Self::PVar => json!("PVar"),
            Self::PNil => json!("PNil"),
            Self::PCons => json!("PCons"),
            Self::PLeaf => json!("PLeaf"),
            Self::PBranch => json!("PBranch"),
            Self::WildcardExtensionReserved => json!("WildcardExtensionReserved"),
            Self::ConstructorExtensionReserved => json!("ConstructorExtensionReserved"),
        }
    }
}

impl OwnerWire for PrimOutcome {
    fn owner_wire(&self) -> Value {
        match self {
            Self::Value { value } => json!({"Value": {"value": value.owner_wire()}}),
            Self::Error { code } => json!({"Error": {"code": code.owner_wire()}}),
        }
    }
}

impl OwnerWire for ProgramView {
    fn owner_wire(&self) -> Value {
        let ProgramView {
            entry, expressions, occurrences, occurrence_paths, children, functions, parameters,
            arms, patterns, pattern_children, types, type_args, constructors, ctor_fields,
            decl_param_counts, decl_ctor_counts, decl_templates, plan_nodes, plan_cases,
            field_paths, constants, constant_fields, names,
        } = self;
        json!({
            "entry": entry.owner_wire(), "expressions": expressions.owner_wire(),
            "occurrences": occurrences.owner_wire(), "occurrence_paths": occurrence_paths.owner_wire(),
            "children": children.owner_wire(), "functions": functions.owner_wire(),
            "parameters": parameters.owner_wire(), "arms": arms.owner_wire(),
            "patterns": patterns.owner_wire(), "pattern_children": pattern_children.owner_wire(),
            "types": types.owner_wire(), "type_args": type_args.owner_wire(),
            "constructors": constructors.owner_wire(), "ctor_fields": ctor_fields.owner_wire(),
            "decl_param_counts": decl_param_counts.owner_wire(),
            "decl_ctor_counts": decl_ctor_counts.owner_wire(),
            "decl_templates": decl_templates.owner_wire(),
            "plan_nodes": plan_nodes.owner_wire(), "plan_cases": plan_cases.owner_wire(),
            "field_paths": field_paths.owner_wire(), "constants": constants.owner_wire(),
            "constant_fields": constant_fields.owner_wire(), "names": names.owner_wire(),
        })
    }
}

impl OwnerWire for RawConstRow {
    fn owner_wire(&self) -> Value {
        match self {
            Self::U128 { marker_free, n } => json!({"U128": {"marker_free": marker_free.owner_wire(), "n": n.owner_wire()}}),
            Self::Bool { marker_free, b } => json!({"Bool": {"marker_free": marker_free.owner_wire(), "b": b.owner_wire()}}),
            Self::Nil { marker_free } => json!({"Nil": {"marker_free": marker_free.owner_wire()}}),
            Self::Cons { marker_free, fields } => json!({"Cons": {"marker_free": marker_free.owner_wire(), "fields": fields.owner_wire()}}),
            Self::Leaf { marker_free, fields } => json!({"Leaf": {"marker_free": marker_free.owner_wire(), "fields": fields.owner_wire()}}),
            Self::Branch { marker_free, fields } => json!({"Branch": {"marker_free": marker_free.owner_wire(), "fields": fields.owner_wire()}}),
            Self::Constructor { marker_free, type_id, ctor_id, fields } => json!({"Constructor": {"marker_free": marker_free.owner_wire(), "type_id": TypeIdC(type_id.0).owner_wire(), "ctor_id": adt::CtorIdC(ctor_id.0).owner_wire(), "fields": fields.owner_wire()}}),
            Self::U128OutOfRange { marker_free } => json!({"U128OutOfRange": {"marker_free": marker_free.owner_wire()}}),
        }
    }
}

impl OwnerWire for RawDataRow {
    fn owner_wire(&self) -> Value {
        match self {
            Self::U128 { n } => json!({"U128": {"n": n.owner_wire()}}),
            Self::Bool { b } => json!({"Bool": {"b": b.owner_wire()}}),
            Self::Nil => json!("Nil"),
            Self::Cons { fields } => json!({"Cons": {"fields": fields.owner_wire()}}),
            Self::Leaf { fields } => json!({"Leaf": {"fields": fields.owner_wire()}}),
            Self::Branch { fields } => json!({"Branch": {"fields": fields.owner_wire()}}),
            Self::Constructor { type_id, ctor_id, fields } => json!({"Constructor": {"type_id": TypeIdC(type_id.0).owner_wire(), "ctor_id": adt::CtorIdC(ctor_id.0).owner_wire(), "fields": fields.owner_wire()}}),
        }
    }
}

impl OwnerWire for RefC {
    fn owner_wire(&self) -> Value {
        match self {
            Self::Const(item0) => json!({"Const": item0.owner_wire()}),
            Self::Data(item0) => json!({"Data": item0.owner_wire()}),
        }
    }
}

impl OwnerWire for RuleIdC {
    fn owner_wire(&self) -> Value {
        match self {
            Self::KMachineEvalLiteralValid => json!("KMachineEvalLiteralValid"),
            Self::KMachineEvalLiteralInvalid => json!("KMachineEvalLiteralInvalid"),
            Self::KMachineEvalVariablePresent => json!("KMachineEvalVariablePresent"),
            Self::KMachineEvalVariableAbsent => json!("KMachineEvalVariableAbsent"),
            Self::KMachineEvalLet => json!("KMachineEvalLet"),
            Self::KMachineEvalBinary => json!("KMachineEvalBinary"),
            Self::KMachineEvalUnary => json!("KMachineEvalUnary"),
            Self::KMachineEvalIf => json!("KMachineEvalIf"),
            Self::KMachineEvalMatch => json!("KMachineEvalMatch"),
            Self::KMachineEvalCallEmpty => json!("KMachineEvalCallEmpty"),
            Self::KMachineEvalCallArguments => json!("KMachineEvalCallArguments"),
            Self::KMachinePushAccept => json!("KMachinePushAccept"),
            Self::KMachinePushRefuse => json!("KMachinePushRefuse"),
            Self::KMachineReturnLet => json!("KMachineReturnLet"),
            Self::KMachineReturnLeft => json!("KMachineReturnLeft"),
            Self::KMachineReturnRight => json!("KMachineReturnRight"),
            Self::KMachineReturnUnary => json!("KMachineReturnUnary"),
            Self::KMachineReturnIfTrue => json!("KMachineReturnIfTrue"),
            Self::KMachineReturnIfFalse => json!("KMachineReturnIfFalse"),
            Self::KMachineReturnIfInvalid => json!("KMachineReturnIfInvalid"),
            Self::KMachineReturnMatch => json!("KMachineReturnMatch"),
            Self::KMachineReturnArgumentsNext => json!("KMachineReturnArgumentsNext"),
            Self::KMachineReturnArgumentsFinal => json!("KMachineReturnArgumentsFinal"),
            Self::KMachineReturnCallValid => json!("KMachineReturnCallValid"),
            Self::KMachineReturnCallInvalid => json!("KMachineReturnCallInvalid"),
            Self::KMachineReturnScope => json!("KMachineReturnScope"),
            Self::KMachineInvokeFind => json!("KMachineInvokeFind"),
            Self::KMachineEnterMissing => json!("KMachineEnterMissing"),
            Self::KMachineEnterBind => json!("KMachineEnterBind"),
            Self::KMachineBoundInvalid => json!("KMachineBoundInvalid"),
            Self::KMachineBoundValid => json!("KMachineBoundValid"),
            Self::KMachineCasesEmpty => json!("KMachineCasesEmpty"),
            Self::KMachineCasesTry => json!("KMachineCasesTry"),
            Self::KMachineCasesMiss => json!("KMachineCasesMiss"),
            Self::KMachineCasesHit => json!("KMachineCasesHit"),
            Self::KMachinePrimitiveValue => json!("KMachinePrimitiveValue"),
            Self::KMachinePrimitiveError => json!("KMachinePrimitiveError"),
            Self::BHelperScalarEntry => json!("BHelperScalarEntry"),
            Self::BHelperScalarTag => json!("BHelperScalarTag"),
            Self::BHelperScalarArithmetic => json!("BHelperScalarArithmetic"),
            Self::BHelperScalarConstruct => json!("BHelperScalarConstruct"),
            Self::BHelperScalarExit => json!("BHelperScalarExit"),
            Self::BHelperScalarFailure => json!("BHelperScalarFailure"),
            Self::BHelperEnvGetEntry => json!("BHelperEnvGetEntry"),
            Self::BHelperEnvGetCell => json!("BHelperEnvGetCell"),
            Self::BHelperEnvGetEmpty => json!("BHelperEnvGetEmpty"),
            Self::BHelperEnvGetExit => json!("BHelperEnvGetExit"),
            Self::BHelperEnvGetFailure => json!("BHelperEnvGetFailure"),
            Self::BHelperEnvPutEntry => json!("BHelperEnvPutEntry"),
            Self::BHelperEnvPutCell => json!("BHelperEnvPutCell"),
            Self::BHelperEnvPutInsert => json!("BHelperEnvPutInsert"),
            Self::BHelperEnvPutEmpty => json!("BHelperEnvPutEmpty"),
            Self::BHelperEnvPutExit => json!("BHelperEnvPutExit"),
            Self::BHelperEnvPutFailure => json!("BHelperEnvPutFailure"),
            Self::BHelperFindEntry => json!("BHelperFindEntry"),
            Self::BHelperFindRow => json!("BHelperFindRow"),
            Self::BHelperFindEmpty => json!("BHelperFindEmpty"),
            Self::BHelperFindExit => json!("BHelperFindExit"),
            Self::BHelperFindFailure => json!("BHelperFindFailure"),
            Self::BHelperBindEntry => json!("BHelperBindEntry"),
            Self::BHelperBindPair => json!("BHelperBindPair"),
            Self::BHelperBindEmpty => json!("BHelperBindEmpty"),
            Self::BHelperBindArity => json!("BHelperBindArity"),
            Self::BHelperBindType => json!("BHelperBindType"),
            Self::BHelperBindDuplicate => json!("BHelperBindDuplicate"),
            Self::BHelperBindInsert => json!("BHelperBindInsert"),
            Self::BHelperBindPropagate => json!("BHelperBindPropagate"),
            Self::BHelperBindExit => json!("BHelperBindExit"),
            Self::BHelperBindFailure => json!("BHelperBindFailure"),
            Self::BHelperAppendEntry => json!("BHelperAppendEntry"),
            Self::BHelperAppendCell => json!("BHelperAppendCell"),
            Self::BHelperAppendEmpty => json!("BHelperAppendEmpty"),
            Self::BHelperAppendTail => json!("BHelperAppendTail"),
            Self::BHelperAppendCopy => json!("BHelperAppendCopy"),
            Self::BHelperAppendExit => json!("BHelperAppendExit"),
            Self::BHelperAppendFailure => json!("BHelperAppendFailure"),
            Self::BHelperValidateEntry => json!("BHelperValidateEntry"),
            Self::BHelperValidateOccurrence => json!("BHelperValidateOccurrence"),
            Self::BHelperValidateField => json!("BHelperValidateField"),
            Self::BHelperValidateDeclaration => json!("BHelperValidateDeclaration"),
            Self::BHelperValidateEmpty => json!("BHelperValidateEmpty"),
            Self::BHelperValidateInvalid => json!("BHelperValidateInvalid"),
            Self::BHelperValidateExit => json!("BHelperValidateExit"),
            Self::BHelperValidateFailure => json!("BHelperValidateFailure"),
            Self::BHelperEqualEntry => json!("BHelperEqualEntry"),
            Self::BHelperEqualPair => json!("BHelperEqualPair"),
            Self::BHelperEqualField => json!("BHelperEqualField"),
            Self::BHelperEqualEmpty => json!("BHelperEqualEmpty"),
            Self::BHelperEqualDifferent => json!("BHelperEqualDifferent"),
            Self::BHelperEqualExit => json!("BHelperEqualExit"),
            Self::BHelperEqualFailure => json!("BHelperEqualFailure"),
            Self::BHelperMatchEntry => json!("BHelperMatchEntry"),
            Self::BHelperMatchArm => json!("BHelperMatchArm"),
            Self::BHelperMatchOccurrence => json!("BHelperMatchOccurrence"),
            Self::BHelperMatchField => json!("BHelperMatchField"),
            Self::BHelperMatchBinding => json!("BHelperMatchBinding"),
            Self::BHelperMatchDistinct => json!("BHelperMatchDistinct"),
            Self::BHelperMatchDiscard => json!("BHelperMatchDiscard"),
            Self::BHelperMatchNext => json!("BHelperMatchNext"),
            Self::BHelperMatchSelected => json!("BHelperMatchSelected"),
            Self::BHelperMatchEmpty => json!("BHelperMatchEmpty"),
            Self::BHelperMatchExit => json!("BHelperMatchExit"),
            Self::BHelperMatchFailure => json!("BHelperMatchFailure"),
            Self::BBookkeepingTaskEnqueue => json!("BBookkeepingTaskEnqueue"),
            Self::BBookkeepingTaskRelease => json!("BBookkeepingTaskRelease"),
            Self::BBookkeepingResumeStore => json!("BBookkeepingResumeStore"),
            Self::BBookkeepingResumeRestore => json!("BBookkeepingResumeRestore"),
            Self::KAdminStartupValid => json!("KAdminStartupValid"),
            Self::KAdminStartupInvalid => json!("KAdminStartupInvalid"),
            Self::KAdminPublishValue => json!("KAdminPublishValue"),
            Self::KAdminPublishError => json!("KAdminPublishError"),
            Self::KAdminPublishExhausted => json!("KAdminPublishExhausted"),
            Self::XAdtEvalConstructNullary => json!("XAdtEvalConstructNullary"),
            Self::XAdtEvalConstructFields => json!("XAdtEvalConstructFields"),
            Self::XAdtReturnCtorFieldsTyped => json!("XAdtReturnCtorFieldsTyped"),
            Self::XAdtReturnCtorFieldsMistyped => json!("XAdtReturnCtorFieldsMistyped"),
            Self::XAdtReturnCtorFieldsNext => json!("XAdtReturnCtorFieldsNext"),
            Self::XAdtReturnCtorFieldsFinal => json!("XAdtReturnCtorFieldsFinal"),
            Self::XAdtMaterializeDone => json!("XAdtMaterializeDone"),
            Self::XAdtCasesPlanHit => json!("XAdtCasesPlanHit"),
            Self::XAdtCasesPlanMiss => json!("XAdtCasesPlanMiss"),
            Self::XAdtHelperBuildEntry => json!("XAdtHelperBuildEntry"),
            Self::XAdtHelperBuildCell => json!("XAdtHelperBuildCell"),
            Self::XAdtHelperBuildField => json!("XAdtHelperBuildField"),
            Self::XAdtHelperBuildExit => json!("XAdtHelperBuildExit"),
            Self::XAdtHelperPlanEntry => json!("XAdtHelperPlanEntry"),
            Self::XAdtHelperPlanWalk => json!("XAdtHelperPlanWalk"),
            Self::XAdtHelperPlanSwitch => json!("XAdtHelperPlanSwitch"),
            Self::XAdtHelperPlanBind => json!("XAdtHelperPlanBind"),
            Self::XAdtHelperPlanSelect => json!("XAdtHelperPlanSelect"),
            Self::XAdtHelperPlanNoMatch => json!("XAdtHelperPlanNoMatch"),
            Self::XAdtHelperPlanExit => json!("XAdtHelperPlanExit"),
            Self::XAdtHelperPlanFailure => json!("XAdtHelperPlanFailure"),
        }
    }
}

impl OwnerWire for RunIdC {
    fn owner_wire(&self) -> Value {
        let RunIdC { identity, fuel_input, stack_limit_input } = self;
        json!({"identity": identity.owner_wire(), "fuel_input": fuel_input.owner_wire(), "stack_limit_input": stack_limit_input.owner_wire()})
    }
}

impl OwnerWire for RunIntC {
    fn owner_wire(&self) -> Value {
        match self {
            Self::Negative => json!("Negative"),
            Self::Nat(item0) => json!({"Nat": item0.owner_wire()}),
            Self::NatAboveU128 => json!("NatAboveU128"),
        }
    }
}

impl OwnerWire for RunSummary {
    fn owner_wire(&self) -> Value {
        let RunSummary { outcome, events, consumed_work, helper_work, helper_visits, reference_cursor } = self;
        json!({"outcome": outcome.owner_wire(), "events": events.owner_wire(), "consumed_work": consumed_work.owner_wire(), "helper_work": helper_work.owner_wire(), "helper_visits": helper_visits.owner_wire(), "reference_cursor": reference_cursor.owner_wire()})
    }
}

impl OwnerWire for ScalarMode {
    fn owner_wire(&self) -> Value {
        match self {
            Self::Add => json!("Add"),
            Self::Sub => json!("Sub"),
            Self::Mul => json!("Mul"),
            Self::Lt => json!("Lt"),
            Self::Cons => json!("Cons"),
            Self::Branch => json!("Branch"),
            Self::Leaf => json!("Leaf"),
            Self::Not => json!("Not"),
            Self::TestBool => json!("TestBool"),
            Self::TestType => json!("TestType"),
        }
    }
}

impl OwnerWire for ScalarPhase {
    fn owner_wire(&self) -> Value {
        match self {
            Self::First => json!("First"),
            Self::Second => json!("Second"),
            Self::Done => json!("Done"),
        }
    }
}

impl OwnerWire for ScalarStaged {
    fn owner_wire(&self) -> Value {
        match self {
            Self::None => json!("None"),
            Self::U128Value { n } => json!({"U128Value": {"n": n.owner_wire()}}),
            Self::BoolValue { b } => json!({"BoolValue": {"b": b.owner_wire()}}),
            Self::PendingNode { tag } => json!({"PendingNode": {"tag": tag.owner_wire()}}),
            Self::PrimError { code } => json!({"PrimError": {"code": code.owner_wire()}}),
            Self::Selector { sel } => json!({"Selector": {"sel": sel.owner_wire()}}),
            Self::TypeTest { ok } => json!({"TypeTest": {"ok": ok.owner_wire()}}),
        }
    }
}

impl OwnerWire for Selected {
    fn owner_wire(&self) -> Value {
        let Selected { rule, event_kind, site, helper_kind, helper_eligible, helper_visits } = self;
        json!({"rule": rule.owner_wire(), "event_kind": event_kind.owner_wire(), "site": site.owner_wire(), "helper_kind": helper_kind.owner_wire(), "helper_eligible": helper_eligible.owner_wire(), "helper_visits": helper_visits.owner_wire()})
    }
}

impl OwnerWire for SiteTag {
    fn owner_wire(&self) -> Value {
        match self {
            Self::EvalLiteral => json!("EvalLiteral"),
            Self::EvalVar => json!("EvalVar"),
            Self::EvalLet => json!("EvalLet"),
            Self::EvalBinary => json!("EvalBinary"),
            Self::EvalUnary => json!("EvalUnary"),
            Self::EvalIf => json!("EvalIf"),
            Self::EvalMatch => json!("EvalMatch"),
            Self::EvalCall => json!("EvalCall"),
            Self::Push => json!("Push"),
            Self::RetLet => json!("RetLet"),
            Self::RetLeft => json!("RetLeft"),
            Self::RetRight => json!("RetRight"),
            Self::RetUnary => json!("RetUnary"),
            Self::RetIf => json!("RetIf"),
            Self::RetMatch => json!("RetMatch"),
            Self::RetArgs => json!("RetArgs"),
            Self::RetCall => json!("RetCall"),
            Self::RetScope => json!("RetScope"),
            Self::Invoke => json!("Invoke"),
            Self::Enter => json!("Enter"),
            Self::Bound => json!("Bound"),
            Self::Cases => json!("Cases"),
            Self::CaseResult => json!("CaseResult"),
            Self::Prim => json!("Prim"),
            Self::EvalConstruct => json!("EvalConstruct"),
            Self::RetCtorFields => json!("RetCtorFields"),
            Self::Materialize => json!("Materialize"),
        }
    }
}

impl OwnerWire for SliceC {
    fn owner_wire(&self) -> Value {
        let SliceC { start, len } = self;
        json!({"start": start.owner_wire(), "len": len.owner_wire()})
    }
}

impl OwnerWire for TypeIdC {
    fn owner_wire(&self) -> Value {
        match *self {
            Self::UInt => json!("UInt"),
            Self::Bool => json!("Bool"),
            Self::List => json!("List"),
            Self::Tree => json!("Tree"),
            _ => json!({"TypeId": self.0.owner_wire()}),
        }
    }
}

impl OwnerWire for SortTag {
    fn owner_wire(&self) -> Value {
        match self {
            Self::UInt => json!("UInt"),
            Self::Bool => json!("Bool"),
            Self::List => json!("List"),
            Self::Tree => json!("Tree"),
        }
    }
}

impl OwnerWire for SourceHostBounds {
    fn owner_wire(&self) -> Value {
        let SourceHostBounds { max_source_actions, max_source_expressions, max_source_statements, max_action_inputs, max_supplied_inputs, max_projection_occurrences, max_generated_ast_nodes, max_ast_nesting, max_input_name_bytes, max_total_input_name_bytes } = self;
        json!({"max_source_actions": max_source_actions.owner_wire(), "max_source_expressions": max_source_expressions.owner_wire(), "max_source_statements": max_source_statements.owner_wire(), "max_action_inputs": max_action_inputs.owner_wire(), "max_supplied_inputs": max_supplied_inputs.owner_wire(), "max_projection_occurrences": max_projection_occurrences.owner_wire(), "max_generated_ast_nodes": max_generated_ast_nodes.owner_wire(), "max_ast_nesting": max_ast_nesting.owner_wire(), "max_input_name_bytes": max_input_name_bytes.owner_wire(), "max_total_input_name_bytes": max_total_input_name_bytes.owner_wire()})
    }
}

impl OwnerWire for StepInvClauseId {
    fn owner_wire(&self) -> Value {
        match self {
            Self::SI01 => json!("SI01"),
            Self::SI02 => json!("SI02"),
            Self::SI03 => json!("SI03"),
            Self::SI04 => json!("SI04"),
            Self::SI05 => json!("SI05"),
            Self::SI06 => json!("SI06"),
            Self::SI07 => json!("SI07"),
            Self::SI08 => json!("SI08"),
            Self::SI09 => json!("SI09"),
            Self::SI10 => json!("SI10"),
            Self::SI11 => json!("SI11"),
            Self::SI12 => json!("SI12"),
            Self::SI13 => json!("SI13"),
            Self::SI14 => json!("SI14"),
            Self::SI15 => json!("SI15"),
            Self::SI16 => json!("SI16"),
            Self::SI17 => json!("SI17"),
            Self::SI18 => json!("SI18"),
            Self::SI19 => json!("SI19"),
            Self::SI20 => json!("SI20"),
            Self::SI21 => json!("SI21"),
            Self::SI22 => json!("SI22"),
            Self::SI23 => json!("SI23"),
            Self::SI24 => json!("SI24"),
        }
    }
}

impl OwnerWire for TaskVisitC {
    fn owner_wire(&self) -> Value {
        match self {
            Self::Unvisited => json!("Unvisited"),
            Self::Fields => json!("Fields"),
        }
    }
}

impl OwnerWire for Tooling {
    fn owner_wire(&self) -> Value {
        match self {
            Self::AdmissionRejected { reason, detail, locator } => json!({"AdmissionRejected": {"reason": reason.owner_wire(), "detail": detail.owner_wire(), "locator": locator.owner_wire()}}),
            Self::CapacityNotAdmitted { kind, needed, limit, locator } => json!({"CapacityNotAdmitted": {"kind": kind.owner_wire(), "needed": needed.owner_wire(), "limit": limit.owner_wire(), "locator": locator.owner_wire()}}),
            Self::UnsupportedFeature { feature, locator } => json!({"UnsupportedFeature": {"feature": feature.owner_wire(), "locator": locator.owner_wire()}}),
        }
    }
}

impl OwnerWire for TriStatus {
    fn owner_wire(&self) -> Value {
        match self {
            Self::Pending => json!("Pending"),
            Self::Positive => json!("Positive"),
            Self::Negative => json!("Negative"),
        }
    }
}

impl OwnerWire for UnaryOp {
    fn owner_wire(&self) -> Value {
        match self {
            Self::Leaf => json!("Leaf"),
            Self::Not => json!("Not"),
        }
    }
}

impl OwnerWire for LogicalKind {
    fn owner_wire(&self) -> Value {
        match self {
            Self::UInt => json!("UInt"),
            Self::Bool => json!("Bool"),
            Self::Nil => json!("Nil"),
            Self::Cons => json!("Cons"),
            Self::Leaf => json!("Leaf"),
            Self::Branch => json!("Branch"),
            Self::Marker => json!("Marker"),
            Self::ConstructorExtensionReserved => json!("ConstructorExtensionReserved"),
            Self::Constructor { type_id, ctor_id } => json!({"Constructor": {"type_id": type_id.owner_wire(), "ctor_id": ctor_id.owner_wire()}}),
        }
    }
}

impl OwnerWire for FamilyId {
    fn owner_wire(&self) -> Value {
        match self {
            Self::Legacy => json!("Legacy"),
            Self::Scalar => json!("Scalar"),
            Self::EnvGet => json!("EnvGet"),
            Self::EnvPut => json!("EnvPut"),
            Self::Find => json!("Find"),
            Self::Bind => json!("Bind"),
            Self::Append => json!("Append"),
            Self::Validate => json!("Validate"),
            Self::Equal => json!("Equal"),
            Self::Match => json!("Match"),
            Self::BookkeepingRelease => json!("BookkeepingRelease"),
            Self::Build => json!("Build"),
            Self::Plan => json!("Plan"),
        }
    }
}

impl OwnerWire for BoundaryKind {
    fn owner_wire(&self) -> Value {
        match self {
            Self::None => json!("None"),
            Self::Terminal => json!("Terminal"),
            Self::TechnicalCut => json!("TechnicalCut"),
            Self::SourceSuspensionExtensionReserved => json!("SourceSuspensionExtensionReserved"),
        }
    }
}

impl OwnerWire for HelperResultTagC {
    fn owner_wire(&self) -> Value {
        match self {
            Self::Validated => json!("Validated"),
            Self::EnvFound => json!("EnvFound"),
            Self::EnvMissing => json!("EnvMissing"),
            Self::EnvUpdated => json!("EnvUpdated"),
            Self::FindFound => json!("FindFound"),
            Self::FindMissing => json!("FindMissing"),
            Self::BindOk => json!("BindOk"),
            Self::BindBad => json!("BindBad"),
            Self::AppendDone => json!("AppendDone"),
            Self::ScalarValue => json!("ScalarValue"),
            Self::ScalarError => json!("ScalarError"),
            Self::ScalarSelector => json!("ScalarSelector"),
            Self::ScalarTypeTest => json!("ScalarTypeTest"),
            Self::EqualDone => json!("EqualDone"),
            Self::MatchMatched => json!("MatchMatched"),
            Self::MatchNoMatch => json!("MatchNoMatch"),
            Self::PlanMatched => json!("PlanMatched"),
            Self::PlanNoMatch => json!("PlanNoMatch"),
            Self::BuildDone => json!("BuildDone"),
        }
    }
}

impl OwnerWire for OutcomeTagC {
    fn owner_wire(&self) -> Value {
        match self {
            Self::Running => json!("Running"),
            Self::Halted => json!("Halted"),
            Self::Rejected => json!("Rejected"),
            Self::Exhausted => json!("Exhausted"),
        }
    }
}

impl OwnerWire for HelperPhaseSummaryC {
    fn owner_wire(&self) -> Value {
        match self {
            Self::Scalar(p) => json!({"Scalar": p.owner_wire()}),
            Self::Tri(s) => json!({"Tri": s.owner_wire()}),
            Self::Bind(p) => json!({"Bind": p.owner_wire()}),
            Self::Append(p) => json!({"Append": p.owner_wire()}),
            Self::Match(p) => json!({"Match": p.owner_wire()}),
            Self::Build(p) => json!({"Build": p.owner_wire()}),
            Self::Plan(p) => json!({"Plan": p.owner_wire()}),
        }
    }
}

impl OwnerWire for AllocDeltasC {
    fn owner_wire(&self) -> Value {
        let Self { data_nodes, data_fields, environment_cells } = self;
        json!({"data_nodes": data_nodes.owner_wire(), "data_fields": data_fields.owner_wire(), "environment_cells": environment_cells.owner_wire()})
    }
}

impl OwnerWire for StepDisposition {
    fn owner_wire(&self) -> Value {
        match self {
            Self::Applied => json!("Applied"),
            Self::Published => json!("Published"),
            Self::ExhaustedFuelUncharged => json!("ExhaustedFuelUncharged"),
            Self::RefusedHelperWorkUncharged => json!("RefusedHelperWorkUncharged"),
            Self::RefusedCapCharged => json!("RefusedCapCharged"),
        }
    }
}

impl OwnerWire for ExpandedStepIdC {
    fn owner_wire(&self) -> Value {
        let Self { run_id, ordinal } = self;
        json!({"run_id": run_id.owner_wire(), "ordinal": ordinal.owner_wire()})
    }
}

impl OwnerWire for StepEventC {
    fn owner_wire(&self) -> Value {
        let Self {
            step_id, rule_id, event_kind, site, helper_kind, disposition, work_delta,
            helper_work_delta, helper_visits_delta, reference_cursor_delta, alloc_deltas,
            value_slot_writes, logical_depth_before, logical_depth_after, task_cursor_before,
            task_cursor_after, helper_phase_before, helper_phase_after, helper_cursor_before,
            helper_cursor_after, result_tag, origin_reference_cursor, source_expr_id,
            source_occurrence, exhausted_kind, outcome_tag, outcome_payload, boundary,
        } = self;
        json!({
            "step_id": step_id.owner_wire(), "rule_id": rule_id.owner_wire(),
            "event_kind": event_kind.owner_wire(), "site": site.owner_wire(),
            "helper_kind": helper_kind.owner_wire(), "disposition": disposition.owner_wire(),
            "work_delta": work_delta.owner_wire(), "helper_work_delta": helper_work_delta.owner_wire(),
            "helper_visits_delta": helper_visits_delta.owner_wire(),
            "reference_cursor_delta": reference_cursor_delta.owner_wire(),
            "alloc_deltas": alloc_deltas.owner_wire(), "value_slot_writes": value_slot_writes.owner_wire(),
            "logical_depth_before": logical_depth_before.owner_wire(),
            "logical_depth_after": logical_depth_after.owner_wire(),
            "task_cursor_before": task_cursor_before.owner_wire(),
            "task_cursor_after": task_cursor_after.owner_wire(),
            "helper_phase_before": helper_phase_before.owner_wire(),
            "helper_phase_after": helper_phase_after.owner_wire(),
            "helper_cursor_before": helper_cursor_before.owner_wire(),
            "helper_cursor_after": helper_cursor_after.owner_wire(),
            "result_tag": result_tag.owner_wire(),
            "origin_reference_cursor": origin_reference_cursor.owner_wire(),
            "source_expr_id": source_expr_id.owner_wire(),
            "source_occurrence": source_occurrence.owner_wire(),
            "exhausted_kind": exhausted_kind.owner_wire(),
            "outcome_tag": outcome_tag.owner_wire(),
            "outcome_payload": outcome_payload.owner_wire(),
            "boundary": boundary.owner_wire(),
        })
    }
}

impl OwnerWire for ViewCheck {
    fn owner_wire(&self) -> Value {
        match self {
            Self::V1 => json!("V1"), Self::V2 => json!("V2"), Self::V3 => json!("V3"), Self::V4 => json!("V4"),
            Self::V5 => json!("V5"), Self::V6 => json!("V6"), Self::V7 => json!("V7"), Self::V8 => json!("V8"),
        }
    }
}

impl OwnerWire for StepInvViolation {
    fn owner_wire(&self) -> Value {
        let Self { clause, locator, detail } = self;
        json!({"clause": clause.owner_wire(), "locator": locator.owner_wire(), "detail": detail.owner_wire()})
    }
}

impl OwnerWire for ProgramViewViolation {
    fn owner_wire(&self) -> Value {
        let Self { check, table, row, detail } = self;
        json!({"check": check.owner_wire(), "table": table.owner_wire(), "row": row.owner_wire(), "detail": detail.owner_wire()})
    }
}

impl OwnerWire for GuardId {
    fn owner_wire(&self) -> Value {
        match self {
            Self::GScalarAllocNode => json!("GScalarAllocNode"),
            Self::GScalarFields1 => json!("GScalarFields1"),
            Self::GScalarFields2 => json!("GScalarFields2"),
            Self::GValidateOccLeaf => json!("GValidateOccLeaf"),
            Self::GValidateFieldNontail => json!("GValidateFieldNontail"),
            Self::GEqualPairLeaf => json!("GEqualPairLeaf"),
            Self::GEqualFieldNontail => json!("GEqualFieldNontail"),
            Self::GBindTypeReject => json!("GBindTypeReject"),
            Self::GBindDupHit => json!("GBindDupHit"),
            Self::GBuildLast => json!("GBuildLast"),
        }
    }
}

impl OwnerWire for CapKind {
    fn owner_wire(&self) -> Value {
        match self {
            Self::DataNodes => json!("DataNodes"),
            Self::DataFields => json!("DataFields"),
            Self::EnvironmentCells => json!("EnvironmentCells"),
            Self::HelperTasks => json!("HelperTasks"),
        }
    }
}

impl OwnerWire for ConstructorNodeWriteC {
    fn owner_wire(&self) -> Value {
        let Self { type_id, ctor_id, fields } = self;
        json!({"type_id": type_id.owner_wire(), "ctor_id": ctor_id.owner_wire(), "fields": fields.owner_wire()})
    }
}

pub fn layout(value: &LayoutObservation<'_>) -> Value {
    json!({"layout_version": value.layout_version.owner_wire(),
        "field_id": value.field_id.owner_wire(),
        "widths": value.widths.as_ref().map(widths),
        "scalar_layout_id": value.scalar_layout_id.owner_wire(),
        "bounds": value.bounds.owner_wire(), "capacities": value.capacities.owner_wire(),
        "boundary_capacities_present": value.boundary_capacities_present})
}
fn widths(value: &WidthTableObservation<'_>) -> Value {
    json!({"index_widths": value.index_widths.owner_wire(),
        "counter_widths": value.counter_widths.owner_wire(),
        "record_widths": value.record_widths.owner_wire()})
}
pub fn admission(value: &HostObservation<'_>) -> Value {
    json!({"status":"admitted", "representation":"HostOnly",
        "identity": value.admission.identity.owner_wire(),
        "bounds": value.admission.bounds.owner_wire(),
        "program_view": value.admission.view.owner_wire(), "layout": layout(&value.admission.layout)})
}
pub fn image(value: &ImageObservation<'_>) -> Value {
    json!({
        "run_id": value.run_id.owner_wire(),
        "control": value.control.owner_wire(),
        "control_origin": value.control_origin.owner_wire(),
        "env_root": value.env_root.owner_wire(),
        "stack_limit": value.stack_limit.owner_wire(),
        "outcome": value.outcome.owner_wire(),
        "initial_fuel": value.initial_fuel.owner_wire(),
        "remaining_work": value.remaining_work.owner_wire(),
        "consumed_work": value.consumed_work.owner_wire(),
        "helper_work": value.helper_work.owner_wire(),
        "helper_visits": value.helper_visits.owner_wire(),
        "reference_cursor": value.reference_cursor.owner_wire(),
        "alloc": value.alloc.owner_wire(),
        "active": value.active.owner_wire(),
        "helper": value.helper.owner_wire(),
        "result": value.result.owner_wire(),
        "data": value.data.owner_wire(),
        "data_fields": value.data_fields.owner_wire(),
        "env": value.env.owner_wire(),
        "frames": value.frames.owner_wire(),
        "tasks": value.tasks.owner_wire(),
        "value_slots": value.value_slots.owner_wire(),
        "seg_base": {"active":value.seg_base.active().owner_wire(),
            "alloc":value.seg_base.alloc().owner_wire(),
            "consumed_work":value.seg_base.consumed_work().owner_wire()}
    })
}

// Nominal preparation reports use the same explicit, output-only owner wire.
impl OwnerWire for PreparationReportC {
    fn owner_wire(&self) -> Value {
        let PreparationReportC { work_map } = self;
        json!({"work_map": work_map.owner_wire()})
    }
}

impl OwnerWire for WorkMapEntryC {
    fn owner_wire(&self) -> Value {
        let WorkMapEntryC { source_match, live_arms, field_path, events_per_visit } = self;
        json!({"source_match": source_match.owner_wire(), "live_arms": live_arms.owner_wire(),
            "field_path": field_path.owner_wire(), "events_per_visit": events_per_visit.owner_wire()})
    }
}

#[cfg(test)]
mod nominal_preparation_tests {
    use super::*;
    #[test]
    fn preparation_report_retains_nonempty_ordered_work_map() {
        let report=PreparationReportC {work_map:vec![
            WorkMapEntryC {source_match:9,live_arms:vec![2,0],field_path:vec![1,3],
                events_per_visit:vec![RuleIdC::KMachineEvalMatch,RuleIdC::KMachineCasesTry]},
            WorkMapEntryC {source_match:4,live_arms:vec![],field_path:vec![],events_per_visit:vec![]},
        ]};
        assert_eq!(report.owner_wire(),json!({"work_map":[
            {"source_match":"9","live_arms":["2","0"],"field_path":["1","3"],
             "events_per_visit":["KMachineEvalMatch","KMachineCasesTry"]},
            {"source_match":"4","live_arms":[],"field_path":[],"events_per_visit":[]}
        ]}));
    }
}
