//! Host adapter for the immutable helper proposal; normal comparisons only.
//! Preparation and observation may allocate/clone real host values; they are
//! outside helper events. This is an A boundary projection with H diagnostics,
//! not B budget/exhaustion semantics, schema, adoption or refinement evidence.
use super::program_image::{ExprId, ExprPayload, ProgramImageDraft, SourceLocator, SourceOwner};
use super::reference_types::*;
use std::sync::Arc;

pub(super) use super::helper_schedule as helper;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum Gap {
    NotPendingInvoke,
    ImageSourceAssociation,
    ExplicitOriginAssociation,
    MissingRetainedHandle,
    Helper(helper::Unsupported),
}

pub(super) struct AdapterEvent {
    pub helper_event: helper::Event,
    /// Only legacy completion events produce a charged A record.
    pub reference_completion: Option<StepRecord>,
}

/// All helper handles are indices into this retained context. Distinct handle
/// namespaces each have one real program/image/origin/caller-environment slot;
/// frames and values are real retained sequences. No usize invents an object.
pub(super) struct CallContext {
    original: Snapshot,
    image: Arc<ProgramImageDraft>,
    origin: (ExprId, SourceLocator),
    names: Vec<Name>,
    functions: Vec<helper::FunctionRow>,
    parameters: Vec<helper::Parameter>,
    arguments: Vec<helper::Argument>,
    values: Vec<Value>,
    pairs: Vec<Option<helper::PairTask>>,
    cells: Vec<Option<helper::EnvCell>>,
    result_slots: [Option<helper::LegacyControl>; 1],
    state: helper::CallHelper,
}

impl CallContext {
    /// Domain: an unchanged builder-produced image of `snapshot.program`, a
    /// real pending Invoke boundary, and its explicitly supplied source Call ID.
    /// Snapshot currently has no occurrence field; this API does not infer one
    /// from dynamic values or pretend to establish causal provenance.
    pub(super) fn new(image: Arc<ProgramImageDraft>, snapshot: Snapshot, origin: ExprId)
        -> Result<Self, Gap>
    {
        let (invoked_name, values) = match &snapshot.control {
            Some(Control::Invoke(name, values)) if snapshot.outcome == Outcome::Pending =>
                (name.clone(), values.clone()),
            _ => return Err(Gap::NotPendingInvoke),
        };
        let reference_cursor = snapshot.checked_charged_steps().ok_or(Gap::NotPendingInvoke)?;
        if image.profile_id != PROFILE || image.functions.len() != snapshot.program.functions.len() {
            return Err(Gap::ImageSourceAssociation);
        }
        let locator = image.source_map.get(origin.0).ok_or(Gap::ExplicitOriginAssociation)?.clone();
        match image.expressions.get(origin.0).map(|row| &row.payload) {
            Some(ExprPayload::Call(name)) if name == &invoked_name => (),
            _ => return Err(Gap::ExplicitOriginAssociation),
        }
        match source_expression(&snapshot.program, &locator)?.as_ref() {
            Expr::Call(name, _) if name == &invoked_name => (),
            _ => return Err(Gap::ExplicitOriginAssociation),
        }
        let mut names = Vec::new();
        let mut functions = Vec::new();
        // Whole-table host preparation only. No selected-row lookup happens here.
        for (index, row) in image.functions.iter().enumerate() {
            let source = &snapshot.program.functions[index];
            let end = row.parameters.start.checked_add(row.parameters.len)
                .ok_or(Gap::ImageSourceAssociation)?;
            let parameters = image.ordered_parameters.get(row.parameters.start..end)
                .ok_or(Gap::ImageSourceAssociation)?;
            let body_source = image.source_map.get(row.body.0).ok_or(Gap::ImageSourceAssociation)?;
            if row.source_table_index != index || row.name != source.name
                || row.return_type != source.return_type || parameters != source.parameters.as_slice()
                || body_source.owner != SourceOwner::Function(index) || !body_source.ast_path.is_empty()
            { return Err(Gap::ImageSourceAssociation); }
            functions.push(helper::FunctionRow {
                source_table_index: row.source_table_index,
                name: intern(&mut names, &row.name),
                parameters: helper::Slice { start: row.parameters.start, len: row.parameters.len },
                return_sort: sort(row.return_type), body: helper::ExprId(row.body.0),
            });
        }
        let parameters = image.ordered_parameters.iter().map(|parameter| helper::Parameter {
            name: intern(&mut names, &parameter.name), sort: sort(parameter.ty),
        }).collect();
        let arguments = values.iter().enumerate().map(|(index, value)| helper::Argument {
            value: helper::ValueRef(index), sort: value_sort(value),
        }).collect();
        let arguments_slice = helper::Slice { start: 0, len: values.len() };
        let boundary = helper::Boundary {
            reference_profile: PROFILE,
            program_handle: 0, source_binding_handle: 0, control_origin_handle: 0,
            control: helper::LegacyControl::Invoke {
                name: intern(&mut names, &invoked_name), arguments: arguments_slice },
            caller_environment_handle: 0,
            frames_top_first: helper::Slice { start: 0, len: snapshot.frames.len() },
            stack_limit: snapshot.stack_limit,
            reference_initial_fuel: snapshot.initial_fuel,
            reference_remaining_fuel: snapshot.remaining_fuel,
            outcome: helper::BoundaryOutcome::Pending,
        };
        let state = helper::CallHelper::begin(boundary, reference_cursor, 0).map_err(Gap::Helper)?;
        let scratch_len = values.len();
        Ok(Self { original: snapshot, image, origin: (origin, locator), names, functions,
            parameters, arguments, values, pairs: vec![None; scratch_len],
            cells: vec![None; scratch_len], result_slots: [None], state })
    }

    pub(super) fn work(&self) -> helper::Work { self.state.work }
    pub(super) fn proposed_profile(&self) -> &'static str { helper::PROPOSED_PROFILE }
    pub(super) fn resolve_program(&self, handle: usize) -> Result<&Arc<Program>, Gap> {
        if handle == 0 { Ok(&self.original.program) } else { Err(Gap::MissingRetainedHandle) }
    }
    pub(super) fn resolve_source(&self, handle: usize) -> Result<&ProgramImageDraft, Gap> {
        if handle == 0 { Ok(self.image.as_ref()) } else { Err(Gap::MissingRetainedHandle) }
    }
    pub(super) fn resolve_origin(&self, handle: usize) -> Result<&(ExprId, SourceLocator), Gap> {
        if handle == 0 { Ok(&self.origin) } else { Err(Gap::MissingRetainedHandle) }
    }
    pub(super) fn resolve_environment(&self, handle: usize) -> Result<&Environment, Gap> {
        if handle == 0 { Ok(&self.original.environment) } else { Err(Gap::MissingRetainedHandle) }
    }
    pub(super) fn resolve_frames(&self, slice: helper::Slice) -> Result<&[Frame], Gap> {
        let end = slice.start.checked_add(slice.len).ok_or(Gap::MissingRetainedHandle)?;
        self.original.frames.get(slice.start..end).ok_or(Gap::MissingRetainedHandle)
    }
    pub(super) fn resolve_value(&self, value: helper::ValueRef) -> Result<&Value, Gap> {
        self.values.get(value.0).ok_or(Gap::MissingRetainedHandle)
    }
    pub(super) fn resolve_name(&self, name: helper::NameId) -> Result<&Name, Gap> {
        self.names.get(name.0).ok_or(Gap::MissingRetainedHandle)
    }
    pub(super) fn resolve_result_slot(&self, slot: usize)
        -> Result<&Option<helper::LegacyControl>, Gap>
    {
        self.result_slots.get(slot).ok_or(Gap::MissingRetainedHandle)
    }
    fn resolve_function(&self, function: helper::FunctionRow) -> Result<Function, Gap> {
        let row = self.functions.get(function.source_table_index).ok_or(Gap::MissingRetainedHandle)?;
        if *row != function { return Err(Gap::MissingRetainedHandle); }
        self.original.program.functions.get(function.source_table_index).cloned()
            .ok_or(Gap::MissingRetainedHandle)
    }
    fn resolve_body(&self, body: helper::ExprId) -> Result<ExprRef, Gap> {
        let locator = self.image.source_map.get(body.0).ok_or(Gap::MissingRetainedHandle)?;
        source_expression(&self.original.program, locator)
    }
    fn resolve_arguments(&self, slice: helper::Slice) -> Result<Vec<Value>, Gap> {
        let end = slice.start.checked_add(slice.len).ok_or(Gap::MissingRetainedHandle)?;
        self.arguments.get(slice.start..end).ok_or(Gap::MissingRetainedHandle)?
            .iter().map(|argument| self.resolve_value(argument.value).cloned()).collect()
    }
    /// Host observation only: map materialization/cloning is deliberately not a
    /// helper event, native operation, or claimed bounded B/circuit computation.
    fn resolve_binding(&self, binding: helper::Binding) -> Result<Option<Environment>, Gap> {
        let helper::Binding::Fresh(mut cursor) = binding else { return Ok(None); };
        let mut environment = Environment::new();
        let mut remaining = self.cells.len();
        while let Some(id) = cursor {
            remaining = remaining.checked_sub(1).ok_or(Gap::MissingRetainedHandle)?;
            let cell = self.cells.get(id.0).copied().flatten().ok_or(Gap::MissingRetainedHandle)?;
            environment.insert(self.resolve_name(cell.name)?.clone(), self.resolve_value(cell.value)?.clone());
            cursor = cell.previous;
        }
        Ok(Some(environment))
    }
    fn materialize(&self, boundary: helper::Boundary) -> Result<Snapshot, Gap> {
        if boundary.reference_profile != PROFILE { return Err(Gap::MissingRetainedHandle); }
        self.resolve_source(boundary.source_binding_handle)?;
        self.resolve_origin(boundary.control_origin_handle)?;
        let control = match boundary.control {
            helper::LegacyControl::Invoke { name, arguments } =>
                Control::Invoke(self.resolve_name(name)?.clone(), self.resolve_arguments(arguments)?),
            helper::LegacyControl::Enter { function, arguments } => Control::Enter(
                function.map(|row| self.resolve_function(row)).transpose()?, self.resolve_arguments(arguments)?),
            helper::LegacyControl::Bound { binding, return_sort, body } =>
                Control::Bound(self.resolve_binding(binding)?, unsort(return_sort), self.resolve_body(body)?),
        };
        Ok(Snapshot {
            program: self.resolve_program(boundary.program_handle)?.clone(), control: Some(control),
            environment: self.resolve_environment(boundary.caller_environment_handle)?.clone(),
            frames: self.resolve_frames(boundary.frames_top_first)?.to_vec(),
            stack_limit: boundary.stack_limit, remaining_fuel: boundary.reference_remaining_fuel,
            initial_fuel: boundary.reference_initial_fuel, outcome: Outcome::Pending,
        })
    }
    pub(super) fn snapshot(&self) -> Result<Snapshot, Gap> { self.materialize(self.state.boundary) }
    pub(super) fn resume_snapshot(&self) -> Result<Snapshot, Gap> {
        self.resolve_result_slot(self.state.resume.result_slot)?;
        self.materialize(self.state.resume.saved_pre_state)
    }

    /// Delegate one event to the unchanged helper. Only separately completed
    /// legacy rules emit real records, at A charge1 with exactly one A fuel unit.
    /// H is reported through work()/helper_event, never added to A fuel/work.
    pub(super) fn step(&mut self) -> Result<Option<AdapterEvent>, Gap> {
        let prior = self.state.boundary;
        let event = self.state.step(&helper::Tables { functions: &self.functions,
            parameters: &self.parameters, arguments: &self.arguments },
            &mut helper::Scratch { pairs: &mut self.pairs, cells: &mut self.cells })
            .map_err(Gap::Helper)?;
        let Some(event) = event else { return Ok(None); };
        let reference_completion = if event.reference_cursor_delta == 1 {
            self.result_slots[self.state.resume.result_slot] = Some(self.state.boundary.control);
            let line = match event.kind {
                helper::EventKind::CompleteFind => 200,
                helper::EventKind::CompleteBind => 204,
                _ => return Err(Gap::MissingRetainedHandle),
            };
            Some(StepRecord { rule_start_line: line, charge: 1,
                before: self.materialize(prior)?, after: self.snapshot()? })
        } else { None };
        Ok(Some(AdapterEvent { helper_event: event, reference_completion }))
    }
}

// Name interning and path resolution are host preparation/observation, not
// helper steps. String spelling/equality is preserved, with no new admission.
fn intern(names: &mut Vec<Name>, name: &str) -> helper::NameId {
    if let Some(index) = names.iter().position(|existing| existing == name) {
        helper::NameId(index)
    } else {
        let index = names.len(); names.push(name.into()); helper::NameId(index)
    }
}
fn sort(ty: Type) -> helper::Sort {
    match ty { Type::UInt => helper::Sort::UInt, Type::Bool => helper::Sort::Bool,
        Type::List => helper::Sort::List, Type::Tree => helper::Sort::Tree }
}
fn unsort(ty: helper::Sort) -> Type {
    match ty { helper::Sort::UInt => Type::UInt, helper::Sort::Bool => Type::Bool,
        helper::Sort::List => Type::List, helper::Sort::Tree => Type::Tree }
}
fn value_sort(value: &Value) -> helper::Sort {
    match value { Value::UInt(_) => helper::Sort::UInt, Value::Bool(_) => helper::Sort::Bool,
        Value::List(_) => helper::Sort::List, Value::Tree(_) => helper::Sort::Tree }
}
fn source_expression(program: &Program, locator: &SourceLocator) -> Result<ExprRef, Gap> {
    let mut expression = match locator.owner {
        SourceOwner::Entry => program.entry.clone(),
        SourceOwner::Function(index) => program.functions.get(index)
            .ok_or(Gap::ImageSourceAssociation)?.body.clone(),
    };
    for &child in &locator.ast_path {
        expression = match expression.as_ref() {
            Expr::Let(_, left, right) | Expr::Binary(_, left, right) => match child {
                0 => left.clone(), 1 => right.clone(), _ => return Err(Gap::ImageSourceAssociation),
            },
            Expr::Unary(_, operand) if child == 0 => operand.clone(),
            Expr::If(condition, yes, no) => match child {
                0 => condition.clone(), 1 => yes.clone(), 2 => no.clone(),
                _ => return Err(Gap::ImageSourceAssociation),
            },
            Expr::Call(_, arguments) => arguments.get(child).ok_or(Gap::ImageSourceAssociation)?.clone(),
            Expr::Match(scrutinee, _) if child == 0 => scrutinee.clone(),
            Expr::Match(_, arms) => arms.get(child - 1).ok_or(Gap::ImageSourceAssociation)?.body.clone(),
            _ => return Err(Gap::ImageSourceAssociation),
        };
    }
    Ok(expression)
}
