//! Plan structure, stable scheduling and one uncached evaluation.
use super::*;
use std::collections::BTreeSet;

pub(super) const OUTPUT_FIELDS: &[&str] = &[
    "constants", "constant_fields", "data", "data_fields", "environment_cells",
    "current_environment", "frames", "control", "helper", "tasks", "result",
    "value_slots", "outcome", "names", "expressions", "parameters", "arms",
    "patterns", "occurrences", "occurrence_paths", "children",
];

pub(super) fn read(input: &HostSnapshot, path: InputPath, at: Location)
    -> Result<InputValue, WitnessFailure>
{
    let p = &input.admission.view;
    let i = &input.image;
    let missing = || fail(at, FailureKind::MissingInput);
    Ok(match path {
        InputPath::Metadata => InputValue::Metadata(Metadata {
            admission: input.admission.clone(), run_id: i.run_id.clone(),
            control_origin: i.control_origin, env_root: i.env_root, stack_limit: i.stack_limit,
            initial_fuel: i.initial_fuel, remaining_work: i.remaining_work,
            consumed_work: i.consumed_work, helper_work: i.helper_work,
            helper_visits: i.helper_visits, reference_cursor: i.reference_cursor,
            alloc: i.alloc, active: i.active, seg_base: i.seg_base,
            table_lengths: [i.data.len(), i.data_fields.len(), i.env.len(), i.frames.len(), i.tasks.len(), i.value_slots.len()],
        }),
        InputPath::Constant(n) => InputValue::Constant(p.constants.get(n).ok_or_else(missing)?.clone()),
        InputPath::ConstantField(n) => InputValue::Reference(RefC::Const(*p.constant_fields.get(n).ok_or_else(missing)?)),
        InputPath::Data(n) => InputValue::Data(i.data.get(n).ok_or_else(missing)?.clone()),
        InputPath::DataField(n) => InputValue::Reference(*i.data_fields.get(n).ok_or_else(missing)?),
        InputPath::Environment(n) => InputValue::Environment(*i.env.get(n).ok_or_else(missing)?),
        InputPath::Name(n) => InputValue::Name(p.names.get(n).ok_or_else(missing)?.clone()),
        InputPath::Expression(n) => InputValue::Expression(p.expressions.get(n).ok_or_else(missing)?.clone()),
        InputPath::Function(n) => InputValue::Function(*p.functions.get(n).ok_or_else(missing)?),
        InputPath::Parameter(n) => InputValue::Parameter(*p.parameters.get(n).ok_or_else(missing)?),
        InputPath::Arm(n) => InputValue::Arm(*p.arms.get(n).ok_or_else(missing)?),
        InputPath::Pattern(n) => InputValue::Pattern(*p.patterns.get(n).ok_or_else(missing)?),
        InputPath::Occurrence(n) => InputValue::Occurrence(p.occurrences.get(n).ok_or_else(missing)?.clone()),
        InputPath::OccurrencePath(n) => InputValue::Index(*p.occurrence_paths.get(n).ok_or_else(missing)?),
        InputPath::Child(n) => InputValue::Index(*p.children.get(n).ok_or_else(missing)?),
        InputPath::Frame(n) => InputValue::Frame(*i.frames.get(n).ok_or_else(missing)?),
        InputPath::ScheduledFrame => match i.control {
            ControlC::ScheduleNext { frame, .. } => InputValue::Frame(frame),
            _ => return Err(missing()),
        },
        InputPath::Control => InputValue::Control(i.control),
        InputPath::Helper => InputValue::Helper(i.helper),
        InputPath::Task(n) => InputValue::Task(*i.tasks.get(n).ok_or_else(missing)?),
        InputPath::Result => InputValue::Result(i.result),
        InputPath::Outcome => InputValue::Outcome(i.outcome),
        InputPath::ValueSlot(n) => InputValue::Reference(*i.value_slots.get(n).ok_or_else(missing)?),
    })
}

/// Kahn's algorithm, choosing the smallest ready ID, retains duplicate edges.
/// This order is independent of hash-map iteration or previous evaluations.
pub(super) fn order(nodes: &[Node]) -> Result<Vec<NodeId>, WitnessFailure> {
    let at = Location::new("plan", None, "dependencies");
    let mut counts = vec![0usize; nodes.len()];
    let mut uses = vec![Vec::new(); nodes.len()];
    for (index, node) in nodes.iter().enumerate() {
        if node.id != NodeId(index) { return Err(fail(node.location, FailureKind::InvalidNodeId)); }
        for dep in &node.dependencies {
            let source = nodes.get(dep.node.0).ok_or_else(|| fail(node.location,
                FailureKind::MissingDependency { dependency: dep.node }))?;
            if source.output_type != dep.value_type {
                return Err(fail(node.location, FailureKind::WrongType {
                    expected: dep.value_type, actual: source.output_type,
                }));
            }
            counts[index] = counts[index].checked_add(1).ok_or_else(|| fail(at, FailureKind::IndexOverflow))?;
            uses[dep.node.0].push(NodeId(index));
        }
    }
    let mut ready = BTreeSet::new();
    for (index, count) in counts.iter().enumerate() { if *count == 0 { ready.insert(NodeId(index)); } }
    let mut result = Vec::new();
    while let Some(id) = ready.iter().next().copied() {
        ready.remove(&id);
        result.push(id);
        for next in &uses[id.0] {
            counts[next.0] -= 1;
            if counts[next.0] == 0 { ready.insert(*next); }
        }
    }
    if result.len() != nodes.len() { return Err(fail(at, FailureKind::CyclicDependencies)); }
    Ok(result)
}

pub(super) fn definitions(nodes: &[Node], order: &[NodeId], live_outs: &[NodeId], lifetime: LifetimeUnit)
    -> Result<Vec<Definition>, WitnessFailure>
{
    let mut positions = vec![0usize; nodes.len()];
    for (position, id) in order.iter().enumerate() {
        let item = positions.get_mut(id.0).ok_or_else(|| fail(Location::new("plan", None, "order"), FailureKind::InvalidNodeId))?;
        *item = position;
    }
    let mut result: Vec<_> = nodes.iter().map(|node| Definition {
        value: node.id, producer: node.id, consumers: Vec::new(),
        last_use_order: positions[node.id.0], lifetime, live_out: live_outs.contains(&node.id),
    }).collect();
    for id in order {
        for dep in &nodes[id.0].dependencies {
            result[dep.node.0].consumers.push(*id);
            result[dep.node.0].last_use_order = result[dep.node.0].last_use_order.max(positions[id.0]);
        }
    }
    for id in live_outs {
        let value = result.get_mut(id.0).ok_or_else(|| fail(Location::new("plan", None, "live_outs"), FailureKind::InvalidNodeId))?;
        // Boundary live-outs remain available through final publication.
        value.last_use_order = order.len();
    }
    Ok(result)
}

fn node_failure(node: &Node, kind: FailureKind) -> WitnessFailure {
    WitnessFailure { location: node.location, node: Some(node.id), kind }
}
fn field_values(node: &Node, fields: &[FieldBinding], args: &[&SlotValue])
    -> Result<Vec<NamedValue>, WitnessFailure>
{
    if node.dependencies.len() != fields.len() + 1 || args.len() != fields.len() + 1 {
        return Err(node_failure(node, FailureKind::IncompleteBindings));
    }
    let mut result = Vec::new();
    for (offset, field) in fields.iter().enumerate() {
        if node.dependencies[offset + 1] != field.dependency {
            return Err(node_failure(node, FailureKind::IncompleteBindings));
        }
        result.push(NamedValue { field: field.field, value: (*args[offset + 1]).clone() });
    }
    Ok(result)
}

fn operation(plan: &WitnessPlan, input: &HostSnapshot, node: &Node, args: &[&SlotValue], diagnostics: &mut HostDiagnostics)
    -> Result<SlotValue, WitnessFailure>
{
    let wrong = || node_failure(node, FailureKind::WrongOperation);
    Ok(match &node.operation {
        Operation::Read(id) => {
            if !args.is_empty() { return Err(wrong()); }
            let binding = plan.inputs.get(id.0).ok_or_else(|| node_failure(node, FailureKind::MissingInput))?;
            if binding.id != *id { return Err(node_failure(node, FailureKind::IncompleteBindings)); }
            let value = read(input, binding.path, binding.location)?;
            if value.source_type() != binding.value_type {
                return Err(node_failure(node, FailureKind::WrongType {
                    expected: ValueType::Source(binding.value_type), actual: ValueType::Source(value.source_type()),
                }));
            }
            if value != binding.immutable_value { return Err(node_failure(node, FailureKind::InputChanged)); }
            diagnostics.input_reads += 1;
            SlotValue::Source(value)
        }
        Operation::Bind => {
            if args.len() != 2 { return Err(wrong()); }
            let SlotValue::Source(_) = args[0] else { return Err(wrong()); };
            (*args[1]).clone()
        }
        Operation::Value { source, arity } => {
            let value = decode::evaluate_value(plan, node, *source, *arity, args)?;
            diagnostics.value_constructions += 1;
            let copied = value.nodes.len().checked_sub(1).ok_or_else(wrong)?;
            diagnostics.copied_value_nodes = diagnostics.copied_value_nodes.checked_add(copied)
                .ok_or_else(|| node_failure(node, FailureKind::IndexOverflow))?;
            SlotValue::Value(value)
        }
        Operation::EmptyEnvironment => {
            if !args.is_empty() { return Err(wrong()); }
            SlotValue::Environment(Vec::new())
        }
        Operation::Environment => {
            let [SlotValue::Source(InputValue::Environment(cell)), SlotValue::Source(InputValue::Name(name)), SlotValue::Value(value), SlotValue::Environment(previous)] = args else {
                return Err(wrong());
            };
            let mut entries = Vec::new();
            entries.push(EnvironmentEntry { binder: cell.binder, name: name.clone(), value: value.clone() });
            entries.extend(previous.iter().cloned());
            diagnostics.environment_constructions += 1;
            diagnostics.copied_environment_entries = diagnostics.copied_environment_entries.checked_add(previous.len())
                .ok_or_else(|| node_failure(node, FailureKind::IndexOverflow))?;
            SlotValue::Environment(entries)
        }
        Operation::Sequence { kind, item_type } => {
            for value in args { if value.value_type() != *item_type { return Err(wrong()); } }
            SlotValue::Sequence { kind: *kind, items: args.iter().map(|v| (*v).clone()).collect() }
        }
        Operation::Payload { kind, fields } => {
            let Some(SlotValue::Source(raw)) = args.first().copied() else { return Err(wrong()); };
            let expected = match kind {
                PayloadKind::Frame => SourceType::Frame, PayloadKind::Control => SourceType::Control,
                PayloadKind::Helper => SourceType::Helper, PayloadKind::Task => SourceType::Task,
                PayloadKind::Result => SourceType::Result, PayloadKind::Outcome => SourceType::Outcome,
            };
            if raw.source_type() != expected { return Err(wrong()); }
            diagnostics.payload_constructions += 1;
            SlotValue::Payload(DecodedPayload { kind: *kind, raw: raw.clone(), fields: field_values(node, fields, args)? })
        }
        Operation::Snapshot { fields } => {
            let Some(SlotValue::Source(InputValue::Metadata(metadata))) = args.first().copied() else { return Err(wrong()); };
            if fields.iter().map(|f| f.field).collect::<Vec<_>>() != OUTPUT_FIELDS {
                return Err(node_failure(node, FailureKind::IncompleteOutputs));
            }
            SlotValue::Snapshot(DecodedSnapshot { metadata: metadata.clone(), fields: field_values(node, fields, args)? })
        }
    })
}

pub(super) fn evaluate(plan: &WitnessPlan, input: &HostSnapshot)
    -> Result<WitnessEvaluation, WitnessFailure>
{
    let at = Location::new("plan", None, "structure");
    if plan.format != "sherlock-host-witness-plan/1" { return Err(fail(at, FailureKind::WrongOperation)); }
    let schedule = order(&plan.nodes)?;
    if schedule != plan.stable_topological_order { return Err(fail(at, FailureKind::UnstableOrder)); }
    if plan.definitions != definitions(&plan.nodes, &schedule, &plan.live_outs, plan.lifetime)? {
        return Err(fail(at, FailureKind::IncompleteBindings));
    }
    let output = plan.nodes.get(plan.output.0).ok_or_else(|| fail(at, FailureKind::IncompleteOutputs))?;
    let Operation::Snapshot { fields } = &output.operation else { return Err(fail(at, FailureKind::IncompleteOutputs)); };
    let mut expected_live: Vec<_> = fields.iter().map(|f| f.dependency.node).collect();
    expected_live.push(plan.output);
    if plan.live_outs != expected_live { return Err(fail(at, FailureKind::IncompleteOutputs)); }
    // Input declarations must be indexed once and have exactly one Read producer.
    let mut reads = vec![0usize; plan.inputs.len()];
    for node in &plan.nodes {
        if let Operation::Read(id) = node.operation {
            let count = reads.get_mut(id.0).ok_or_else(|| node_failure(node, FailureKind::MissingInput))?;
            *count += 1;
        }
    }
    if reads.iter().any(|count| *count != 1) { return Err(fail(at, FailureKind::IncompleteBindings)); }
    let mut slots: Vec<Option<SlotValue>> = vec![None; plan.nodes.len()];
    let mut diagnostics = HostDiagnostics::default();
    for id in schedule {
        let node = &plan.nodes[id.0];
        let mut args = Vec::new();
        for dependency in &node.dependencies {
            let value = slots.get(dependency.node.0).and_then(Option::as_ref)
                .ok_or_else(|| node_failure(node, FailureKind::MissingDependency { dependency: dependency.node }))?;
            if value.value_type() != dependency.value_type {
                return Err(node_failure(node, FailureKind::WrongType { expected: dependency.value_type, actual: value.value_type() }));
            }
            args.push(value);
        }
        let result = operation(plan, input, node, &args, &mut diagnostics)?;
        if result.value_type() != node.output_type {
            return Err(node_failure(node, FailureKind::WrongType { expected: node.output_type, actual: result.value_type() }));
        }
        slots[id.0] = Some(result);
        diagnostics.evaluated_nodes += 1;
    }
    diagnostics.retained_slots = slots.len();
    let Some(SlotValue::Snapshot(decoded)) = slots.get_mut(plan.output.0).and_then(Option::take) else {
        return Err(fail(at, FailureKind::IncompleteOutputs));
    };
    Ok(WitnessEvaluation { decoded, diagnostics })
}
