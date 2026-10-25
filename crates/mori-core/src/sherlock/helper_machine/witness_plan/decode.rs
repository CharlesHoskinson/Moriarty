//! Iterative occurrence reconstruction and explicit payload dependency discovery.
use super::*;
use std::collections::{BTreeMap, BTreeSet};

struct Builder<'a> {
    input: &'a HostSnapshot,
    inputs: Vec<InputBinding>,
    reads: BTreeMap<InputPath, NodeId>,
    nodes: Vec<Node>,
}
impl<'a> Builder<'a> {
    fn add(
        &mut self,
        operation: Operation,
        deps: Vec<Dependency>,
        output_type: ValueType,
        at: Location,
    ) -> NodeId {
        let id = NodeId(self.nodes.len());
        self.nodes.push(Node {
            id,
            location: at,
            operation,
            dependencies: deps,
            output_type,
        });
        id
    }
    fn dependency(&self, id: NodeId) -> Dependency {
        Dependency {
            node: id,
            value_type: self.nodes[id.0].output_type,
        }
    }
    fn read(&mut self, path: InputPath, at: Location) -> Result<NodeId, WitnessFailure> {
        if let Some(id) = self.reads.get(&path) {
            return Ok(*id);
        }
        let value = dag::read(self.input, path, at)?;
        let value_type = value.source_type();
        let id = BindingId(self.inputs.len());
        self.inputs.push(InputBinding {
            id,
            path,
            value_type,
            immutable_value: value,
            location: at,
        });
        let node = self.add(
            Operation::Read(id),
            vec![],
            ValueType::Source(value_type),
            at,
        );
        self.reads.insert(path, node);
        Ok(node)
    }
    fn field(&self, name: &'static str, id: NodeId) -> FieldBinding {
        FieldBinding {
            field: name,
            dependency: self.dependency(id),
        }
    }
    fn bind(&mut self, source: NodeId, target: NodeId, at: Location) -> NodeId {
        self.add(
            Operation::Bind,
            vec![self.dependency(source), self.dependency(target)],
            self.nodes[target.0].output_type,
            at,
        )
    }
    fn sequence(
        &mut self,
        kind: SequenceKind,
        item_type: ValueType,
        ids: Vec<NodeId>,
        at: Location,
    ) -> NodeId {
        let deps = ids.into_iter().map(|id| self.dependency(id)).collect();
        self.add(
            Operation::Sequence { kind, item_type },
            deps,
            ValueType::Sequence(kind),
            at,
        )
    }
    fn indices(
        &self,
        start: usize,
        len: usize,
        table_len: usize,
        at: Location,
    ) -> Result<std::ops::Range<usize>, WitnessFailure> {
        // Empty semantic slices do not dereference a cursor; retain raw start.
        if len == 0 {
            return Ok(0..0);
        }
        let end = start
            .checked_add(len)
            .ok_or_else(|| fail(at, FailureKind::IndexOverflow))?;
        if end > table_len {
            return Err(fail(at, FailureKind::MissingInput));
        }
        Ok(start..end)
    }
    fn source_sequence(
        &mut self,
        kind: SequenceKind,
        ty: SourceType,
        paths: Vec<InputPath>,
        at: Location,
    ) -> Result<NodeId, WitnessFailure> {
        let ids = paths
            .into_iter()
            .map(|path| self.read(path, at))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(self.sequence(kind, ValueType::Source(ty), ids, at))
    }
    fn values_slice(&mut self, slice: SliceC, at: Location) -> Result<NodeId, WitnessFailure> {
        let indices = self.indices(
            slice.start,
            slice.len,
            self.input.image.value_slots.len(),
            at,
        )?;
        let mut ids = Vec::new();
        for index in indices {
            // Explicit Read also binds the ordered slice member, not only its target.
            let member = self.read(InputPath::ValueSlot(index), at)?;
            let value = self.value(self.input.image.value_slots[index], at)?;
            ids.push(self.bind(member, value, at));
        }
        Ok(self.sequence(SequenceKind::Values, ValueType::Value, ids, at))
    }
    fn expression_slice(&mut self, slice: SliceC, at: Location) -> Result<NodeId, WitnessFailure> {
        let indices = self.indices(
            slice.start,
            slice.len,
            self.input.admission.view.children.len(),
            at,
        )?;
        let mut ids = Vec::new();
        for index in indices {
            let member = self.read(InputPath::Child(index), at)?;
            let expression = self.read(
                InputPath::Expression(self.input.admission.view.children[index]),
                at,
            )?;
            ids.push(self.bind(member, expression, at));
        }
        Ok(self.sequence(
            SequenceKind::Expressions,
            ValueType::Source(SourceType::Expression),
            ids,
            at,
        ))
    }
    fn parameter_slice(&mut self, slice: SliceC, at: Location) -> Result<NodeId, WitnessFailure> {
        let indices = self.indices(
            slice.start,
            slice.len,
            self.input.admission.view.parameters.len(),
            at,
        )?;
        self.source_sequence(
            SequenceKind::Parameters,
            SourceType::Parameter,
            indices.map(InputPath::Parameter).collect(),
            at,
        )
    }
    fn arm_slice(&mut self, slice: SliceC, at: Location) -> Result<NodeId, WitnessFailure> {
        let indices = self.indices(
            slice.start,
            slice.len,
            self.input.admission.view.arms.len(),
            at,
        )?;
        self.source_sequence(
            SequenceKind::Arms,
            SourceType::Arm,
            indices.map(InputPath::Arm).collect(),
            at,
        )
    }

    fn value(&mut self, root: RefC, at: Location) -> Result<NodeId, WitnessFailure> {
        enum Work {
            Enter(RefC),
            Exit {
                reference: RefC,
                row: NodeId,
                fields: Vec<NodeId>,
            },
        }
        let mut work = vec![Work::Enter(root)];
        let mut ancestors = BTreeSet::new();
        let mut results = Vec::new();
        while let Some(item) = work.pop() {
            match item {
                Work::Enter(reference) => {
                    if !ancestors.insert(reference_key(reference)) {
                        return Err(fail(at, FailureKind::CyclicValue));
                    }
                    let row = self.read(
                        match reference {
                            RefC::Const(n) => InputPath::Constant(n),
                            RefC::Data(n) => InputPath::Data(n),
                        },
                        at,
                    )?;
                    let atom = match &self.nodes[row.0].operation {
                        Operation::Read(binding) => self.inputs[binding.0].immutable_value.clone(),
                        _ => return Err(fail(at, FailureKind::WrongOperation)),
                    };
                    let (arity, fields_slice) = shape(&atom, at)?;
                    let mut field_nodes = Vec::new();
                    let mut children = Vec::new();
                    if let Some((start, len)) = fields_slice {
                        if len != arity {
                            return Err(fail(
                                at,
                                FailureKind::WrongArity {
                                    expected: arity,
                                    actual: len,
                                },
                            ));
                        }
                        let table_len = match reference {
                            RefC::Const(_) => self.input.admission.view.constant_fields.len(),
                            RefC::Data(_) => self.input.image.data_fields.len(),
                        };
                        for index in self.indices(start, len, table_len, at)? {
                            let (path, child) = match reference {
                                RefC::Const(_) => (
                                    InputPath::ConstantField(index),
                                    RefC::Const(self.input.admission.view.constant_fields[index]),
                                ),
                                RefC::Data(_) => (
                                    InputPath::DataField(index),
                                    self.input.image.data_fields[index],
                                ),
                            };
                            field_nodes.push(self.read(path, at)?);
                            children.push(child);
                        }
                    }
                    work.push(Work::Exit {
                        reference,
                        row,
                        fields: field_nodes,
                    });
                    for child in children.into_iter().rev() {
                        work.push(Work::Enter(child));
                    }
                }
                Work::Exit {
                    reference,
                    row,
                    fields,
                } => {
                    let start = results
                        .len()
                        .checked_sub(fields.len())
                        .ok_or_else(|| fail(at, FailureKind::IncompleteBindings))?;
                    let children = results.split_off(start);
                    let mut deps = vec![self.dependency(row)];
                    for (field, child) in fields.iter().zip(children) {
                        deps.push(self.dependency(*field));
                        deps.push(self.dependency(child));
                    }
                    let id = self.add(
                        Operation::Value {
                            source: reference,
                            arity: fields.len(),
                        },
                        deps,
                        ValueType::Value,
                        at,
                    );
                    results.push(id);
                    ancestors.remove(&reference_key(reference));
                }
            }
        }
        if results.len() != 1 {
            return Err(fail(at, FailureKind::IncompleteBindings));
        }
        Ok(results[0])
    }
    fn environment(&mut self, root: EnvRootC, at: Location) -> Result<NodeId, WitnessFailure> {
        let mut chain = Vec::new();
        let mut seen = BTreeSet::new();
        let mut cursor = root;
        while let Some(index) = cursor {
            if !seen.insert(index) {
                return Err(fail(at, FailureKind::CyclicEnvironment));
            }
            let cell = *self
                .input
                .image
                .env
                .get(index)
                .ok_or_else(|| fail(at, FailureKind::MissingInput))?;
            chain.push((index, cell));
            cursor = cell.previous;
        }
        let mut previous = self.add(
            Operation::EmptyEnvironment,
            vec![],
            ValueType::Environment,
            at,
        );
        for (index, cell) in chain.into_iter().rev() {
            let raw = self.read(InputPath::Environment(index), at)?;
            let name = self.read(InputPath::Name(cell.binder), at)?;
            let value = self.value(cell.value, at)?;
            let deps = [raw, name, value, previous]
                .into_iter()
                .map(|id| self.dependency(id))
                .collect();
            previous = self.add(Operation::Environment, deps, ValueType::Environment, at);
        }
        Ok(previous)
    }
    fn payload(
        &mut self,
        path: InputPath,
        kind: PayloadKind,
        at: Location,
    ) -> Result<NodeId, WitnessFailure> {
        let raw = self.read(path, at)?;
        let Operation::Read(binding) = self.nodes[raw.0].operation else {
            return Err(fail(at, FailureKind::WrongOperation));
        };
        let atom = self.inputs[binding.0].immutable_value.clone();
        let mut fields = Vec::new();
        // Each match is exhaustive over current owner payload variants. Scalars,
        // status/None tags, cursors and origins also remain in the exact raw atom.
        match atom {
            InputValue::Frame(frame) => self.frame_fields(frame, &mut fields, at)?,
            InputValue::Control(control) => self.control_fields(control, &mut fields, at)?,
            InputValue::Helper(helper) => self.helper_fields(helper, &mut fields, at)?,
            InputValue::Task(task) => self.task_fields(task, &mut fields, at)?,
            InputValue::Result(result) => self.result_fields(result, &mut fields, at)?,
            InputValue::Outcome(outcome) => match outcome {
                OutcomeC::Halted { value } => self.value_field(&mut fields, "value", value, at)?,
                OutcomeC::Running
                | OutcomeC::Rejected { .. }
                | OutcomeC::Exhausted { .. }
                | OutcomeC::Waiting { .. } => (),
            },
            _ => return Err(fail(at, FailureKind::WrongOperation)),
        }
        let mut deps = vec![self.dependency(raw)];
        deps.extend(fields.iter().map(|f| f.dependency));
        Ok(self.add(
            Operation::Payload { kind, fields },
            deps,
            ValueType::Payload(kind),
            at,
        ))
    }
    fn source_field(
        &mut self,
        f: &mut Vec<FieldBinding>,
        name: &'static str,
        path: InputPath,
        at: Location,
    ) -> Result<(), WitnessFailure> {
        let id = self.read(path, at)?;
        f.push(self.field(name, id));
        Ok(())
    }
    fn value_field(
        &mut self,
        f: &mut Vec<FieldBinding>,
        name: &'static str,
        value: RefC,
        at: Location,
    ) -> Result<(), WitnessFailure> {
        let id = self.value(value, at)?;
        f.push(self.field(name, id));
        Ok(())
    }
    fn env_field(
        &mut self,
        f: &mut Vec<FieldBinding>,
        name: &'static str,
        value: EnvRootC,
        at: Location,
    ) -> Result<(), WitnessFailure> {
        let id = self.environment(value, at)?;
        f.push(self.field(name, id));
        Ok(())
    }
    fn values_field(
        &mut self,
        f: &mut Vec<FieldBinding>,
        name: &'static str,
        value: SliceC,
        at: Location,
    ) -> Result<(), WitnessFailure> {
        let id = self.values_slice(value, at)?;
        f.push(self.field(name, id));
        Ok(())
    }
    fn expressions_field(
        &mut self,
        f: &mut Vec<FieldBinding>,
        name: &'static str,
        value: SliceC,
        at: Location,
    ) -> Result<(), WitnessFailure> {
        let id = self.expression_slice(value, at)?;
        f.push(self.field(name, id));
        Ok(())
    }
    fn arms_field(
        &mut self,
        f: &mut Vec<FieldBinding>,
        name: &'static str,
        value: SliceC,
        at: Location,
    ) -> Result<(), WitnessFailure> {
        let id = self.arm_slice(value, at)?;
        f.push(self.field(name, id));
        Ok(())
    }
    fn field_path_slice(&mut self, slice: SliceC, at: Location) -> Result<NodeId, WitnessFailure> {
        let indices = self.indices(
            slice.start,
            slice.len,
            self.input.admission.view.field_paths.len(),
            at,
        )?;
        self.source_sequence(
            SequenceKind::Indices,
            SourceType::Index,
            indices.map(InputPath::FieldPath).collect(),
            at,
        )
    }
    fn constructor_fields(
        &mut self,
        type_id: TypeIdC,
        ctor_id: adt::CtorIdC,
        f: &mut Vec<FieldBinding>,
        at: Location,
    ) -> Result<(), WitnessFailure> {
        self.source_field(f, "type_id", InputPath::Type(type_id.0), at)?;
        self.source_field(f, "ctor_id", InputPath::Constructor(ctor_id.0), at)?;
        Ok(())
    }
    fn frame_fields(
        &mut self,
        frame: FrameRowC,
        f: &mut Vec<FieldBinding>,
        at: Location,
    ) -> Result<(), WitnessFailure> {
        match frame {
            FrameRowC::LetF {
                binder,
                body,
                saved_env,
                origin,
            } => {
                self.source_field(f, "binder", InputPath::Name(binder), at)?;
                self.source_field(f, "body", InputPath::Expression(body), at)?;
                self.env_field(f, "saved_env", saved_env, at)?;
                self.source_field(f, "origin", InputPath::Occurrence(origin), at)?;
            }
            FrameRowC::LeftF {
                right,
                saved_env,
                origin,
                ..
            } => {
                self.source_field(f, "right", InputPath::Expression(right), at)?;
                self.env_field(f, "saved_env", saved_env, at)?;
                self.source_field(f, "origin", InputPath::Occurrence(origin), at)?;
            }
            FrameRowC::RightF { left, origin, .. } => {
                self.value_field(f, "left", left, at)?;
                self.source_field(f, "origin", InputPath::Occurrence(origin), at)?;
            }
            FrameRowC::UnaryF { origin, .. } => {
                self.source_field(f, "origin", InputPath::Occurrence(origin), at)?
            }
            FrameRowC::IfF {
                then_expr,
                else_expr,
                saved_env,
                origin,
            } => {
                self.source_field(f, "then_expr", InputPath::Expression(then_expr), at)?;
                self.source_field(f, "else_expr", InputPath::Expression(else_expr), at)?;
                self.env_field(f, "saved_env", saved_env, at)?;
                self.source_field(f, "origin", InputPath::Occurrence(origin), at)?;
            }
            FrameRowC::MatchF {
                arms,
                saved_env,
                origin,
            } => {
                self.arms_field(f, "arms", arms, at)?;
                self.env_field(f, "saved_env", saved_env, at)?;
                self.source_field(f, "origin", InputPath::Occurrence(origin), at)?;
            }
            FrameRowC::ArgsF {
                function_name,
                completed,
                rest,
                saved_env,
                origin,
            } => {
                self.source_field(f, "function_name", InputPath::Name(function_name), at)?;
                self.values_field(f, "completed", completed, at)?;
                self.expressions_field(f, "rest", rest, at)?;
                self.env_field(f, "saved_env", saved_env, at)?;
                self.source_field(f, "origin", InputPath::Occurrence(origin), at)?;
            }
            FrameRowC::ConstructorFieldsF {
                type_id,
                ctor_id,
                field_type,
                completed,
                rest,
                saved_env,
                origin,
                ..
            } => {
                self.constructor_fields(type_id, ctor_id, f, at)?;
                self.source_field(f, "field_type", InputPath::Type(field_type.0), at)?;
                self.values_field(f, "completed", completed, at)?;
                self.expressions_field(f, "rest", rest, at)?;
                self.env_field(f, "saved_env", saved_env, at)?;
                self.source_field(f, "origin", InputPath::Occurrence(origin), at)?;
            }
            FrameRowC::ReturnF {
                saved_env, origin, ..
            }
            | FrameRowC::ScopeF { saved_env, origin } => {
                self.env_field(f, "saved_env", saved_env, at)?;
                self.source_field(f, "origin", InputPath::Occurrence(origin), at)?;
            }
        }
        Ok(())
    }
    fn control_fields(
        &mut self,
        control: ControlC,
        f: &mut Vec<FieldBinding>,
        at: Location,
    ) -> Result<(), WitnessFailure> {
        match control {
            ControlC::Eval { expr } => {
                self.source_field(f, "expr", InputPath::Expression(expr), at)?
            }
            ControlC::Return { value } => self.value_field(f, "value", value, at)?,
            ControlC::ScheduleNext { next, env, .. } => {
                let frame = self.payload(InputPath::ScheduledFrame, PayloadKind::Frame, at)?;
                f.push(self.field("frame", frame));
                self.source_field(f, "next", InputPath::Expression(next), at)?;
                self.env_field(f, "env", env, at)?;
            }
            ControlC::Invoke {
                function_name,
                args,
            } => {
                self.source_field(f, "function_name", InputPath::Name(function_name), at)?;
                self.values_field(f, "args", args, at)?;
            }
            ControlC::Enter { function, args } => {
                if let Some(index) = function {
                    self.source_field(f, "function", InputPath::Function(index), at)?;
                }
                self.values_field(f, "args", args, at)?;
            }
            ControlC::Bound { binding, body, .. } => {
                if let BoundBinding::Ok { env } = binding {
                    self.env_field(f, "binding", env, at)?;
                }
                self.source_field(f, "body", InputPath::Expression(body), at)?;
            }
            ControlC::Cases {
                value,
                arms,
                saved_env,
            } => {
                self.value_field(f, "value", value, at)?;
                self.arms_field(f, "arms", arms, at)?;
                self.env_field(f, "saved_env", saved_env, at)?;
            }
            ControlC::CaseResult {
                matching,
                value,
                body,
                rest,
                saved_env,
            } => {
                if let CaseMatching::Matched { env } = matching {
                    self.env_field(f, "matching", env, at)?;
                }
                self.value_field(f, "value", value, at)?;
                self.source_field(f, "body", InputPath::Expression(body), at)?;
                self.arms_field(f, "rest", rest, at)?;
                self.env_field(f, "saved_env", saved_env, at)?;
            }
            ControlC::PrimResult { result } => match result {
                PrimOutcome::Value { value } => self.value_field(f, "value", value, at)?,
                PrimOutcome::Error { .. } => (),
            },
            ControlC::Materialize {
                type_id,
                ctor_id,
                fields,
            } => {
                self.constructor_fields(type_id, ctor_id, f, at)?;
                self.values_field(f, "fields", fields, at)?;
            }
            ControlC::Await { expr, .. } => {
                self.source_field(f, "expr", InputPath::Expression(expr), at)?
            }
            ControlC::Error { .. } => (),
        }
        Ok(())
    }
    fn helper_fields(
        &mut self,
        helper: Option<HelperControlC>,
        f: &mut Vec<FieldBinding>,
        at: Location,
    ) -> Result<(), WitnessFailure> {
        let Some(helper) = helper else {
            return Ok(());
        };
        match helper {
            HelperControlC::Scalar {
                operand_a,
                operand_b,
                ..
            } => {
                self.value_field(f, "operand_a", operand_a, at)?;
                if let Some(value) = operand_b {
                    self.value_field(f, "operand_b", value, at)?;
                }
            }
            HelperControlC::EnvGet {
                binder,
                cursor,
                found,
                ..
            } => {
                self.source_field(f, "binder", InputPath::Name(binder), at)?;
                self.env_field(f, "cursor", cursor, at)?;
                if let Some(value) = found {
                    self.value_field(f, "found", value, at)?;
                }
            }
            HelperControlC::EnvPut {
                binder,
                value,
                base,
                new_root,
                ..
            } => {
                self.source_field(f, "binder", InputPath::Name(binder), at)?;
                self.value_field(f, "value", value, at)?;
                self.env_field(f, "base", base, at)?;
                self.env_field(f, "new_root", new_root, at)?;
            }
            HelperControlC::Find {
                function_name,
                found,
                ..
            } => {
                self.source_field(f, "function_name", InputPath::Name(function_name), at)?;
                if let Some(index) = found {
                    self.source_field(f, "found", InputPath::Function(index), at)?;
                }
            }
            HelperControlC::Bind {
                function,
                params,
                args,
                partial,
                dup_cursor,
                ..
            } => {
                self.source_field(f, "function", InputPath::Function(function), at)?;
                let parameters = self.parameter_slice(params, at)?;
                f.push(self.field("params", parameters));
                self.values_field(f, "args", args, at)?;
                self.env_field(f, "partial", partial, at)?;
                self.env_field(f, "dup_cursor", dup_cursor, at)?;
            }
            HelperControlC::Append {
                src, value, staged, ..
            } => {
                self.values_field(f, "src", src, at)?;
                self.value_field(f, "value", value, at)?;
                if let Some(value) = staged {
                    self.value_field(f, "staged", value, at)?;
                }
            }
            HelperControlC::Validate { root, .. } => self.value_field(f, "root", root, at)?,
            HelperControlC::Equal { left, right, .. } => {
                self.value_field(f, "left", left, at)?;
                self.value_field(f, "right", right, at)?;
            }
            HelperControlC::Match {
                scrutinee,
                arm,
                saved_env,
                partial,
                pat_tag,
                x,
                y,
                staged,
                phase,
                ..
            } => {
                self.value_field(f, "scrutinee", scrutinee, at)?;
                self.source_field(f, "arm", InputPath::Arm(arm), at)?;
                self.env_field(f, "saved_env", saved_env, at)?;
                self.env_field(f, "partial", partial, at)?;
                // Entry carries PVar/x=0 placeholders until the Arm read. They
                // are raw state, not resolved names before that read occurs.
                if phase != MatchPhase::Arm {
                    match pat_tag {
                        PatternTag::PVar | PatternTag::PLeaf => {
                            self.source_field(f, "x", InputPath::Name(x), at)?
                        }
                        PatternTag::PCons | PatternTag::PBranch => {
                            self.source_field(f, "x", InputPath::Name(x), at)?;
                            self.source_field(f, "y", InputPath::Name(y), at)?;
                        }
                        PatternTag::PNil
                        | PatternTag::WildcardExtensionReserved
                        | PatternTag::ConstructorExtensionReserved => (),
                    }
                }
                if let Some(value) = staged {
                    self.value_field(f, "staged", value, at)?;
                }
            }
            HelperControlC::Build {
                type_id,
                ctor_id,
                src,
                staged,
                node,
                ..
            } => {
                self.constructor_fields(type_id, ctor_id, f, at)?;
                self.values_field(f, "src", src, at)?;
                if let Some(value) = staged {
                    self.value_field(f, "staged", value, at)?;
                }
                if let Some(value) = node {
                    self.value_field(f, "node", value, at)?;
                }
            }
            HelperControlC::Plan {
                source_match,
                scrutinee,
                saved_env,
                partial,
                node,
                cur,
                path,
                arm,
                ..
            } => {
                self.source_field(f, "source_match", InputPath::Expression(source_match), at)?;
                self.value_field(f, "scrutinee", scrutinee, at)?;
                self.env_field(f, "saved_env", saved_env, at)?;
                self.env_field(f, "partial", partial, at)?;
                self.source_field(f, "node", InputPath::PlanNode(node), at)?;
                self.value_field(f, "cur", cur, at)?;
                let id = self.field_path_slice(path, at)?;
                f.push(self.field("path", id));
                if let Some(index) = arm {
                    self.source_field(f, "arm", InputPath::Arm(index), at)?;
                }
            }
        }
        Ok(())
    }
    fn task_fields(
        &mut self,
        task: HelperTaskC,
        f: &mut Vec<FieldBinding>,
        at: Location,
    ) -> Result<(), WitnessFailure> {
        match task {
            HelperTaskC::ValidateTask { r, .. } => self.value_field(f, "r", r, at)?,
            HelperTaskC::EqualTask { left, right, .. } => {
                self.value_field(f, "left", left, at)?;
                self.value_field(f, "right", right, at)?;
            }
            HelperTaskC::PairTask { binder, arg, .. } => {
                self.source_field(f, "binder", InputPath::Name(binder), at)?;
                self.value_field(f, "arg", arg, at)?;
            }
        }
        Ok(())
    }
    fn result_fields(
        &mut self,
        result: Option<HelperResultC>,
        f: &mut Vec<FieldBinding>,
        at: Location,
    ) -> Result<(), WitnessFailure> {
        let Some(result) = result else {
            return Ok(());
        };
        match result {
            HelperResultC::BuildDone { value }
            | HelperResultC::EnvFound { value }
            | HelperResultC::ScalarValue { value }
            | HelperResultC::EqualDone { value } => self.value_field(f, "value", value, at)?,
            HelperResultC::EnvUpdated { cell } => self.env_field(f, "cell", Some(cell), at)?,
            HelperResultC::FindFound { function } => {
                self.source_field(f, "function", InputPath::Function(function), at)?
            }
            HelperResultC::BindOk { env } | HelperResultC::MatchMatched { env } => {
                self.env_field(f, "env", env, at)?
            }
            HelperResultC::PlanMatched { arm, env } => {
                self.source_field(f, "arm", InputPath::Arm(arm), at)?;
                self.env_field(f, "env", env, at)?;
            }
            HelperResultC::PlanNoMatch => (),
            HelperResultC::AppendDone { seq } => self.values_field(f, "seq", seq, at)?,
            HelperResultC::Validated { .. }
            | HelperResultC::EnvMissing
            | HelperResultC::FindMissing
            | HelperResultC::BindBad
            | HelperResultC::ScalarError { .. }
            | HelperResultC::ScalarSelector { .. }
            | HelperResultC::ScalarTypeTest { .. }
            | HelperResultC::MatchNoMatch => (),
        }
        Ok(())
    }
}

fn reference_key(value: RefC) -> (u8, usize) {
    match value {
        RefC::Const(n) => (0, n),
        RefC::Data(n) => (1, n),
    }
}
fn shape(
    atom: &InputValue,
    at: Location,
) -> Result<(usize, Option<(usize, usize)>), WitnessFailure> {
    Ok(match atom {
        InputValue::Constant(value) => match value {
            ConstantSnapshot::Cons { fields, .. } | ConstantSnapshot::Branch { fields, .. } => {
                (2, Some((fields.start, fields.len)))
            }
            ConstantSnapshot::Leaf { fields, .. } => (1, Some((fields.start, fields.len))),
            ConstantSnapshot::Constructor { fields, .. } => {
                (fields.len, Some((fields.start, fields.len)))
            }
            ConstantSnapshot::U128 { .. }
            | ConstantSnapshot::Bool { .. }
            | ConstantSnapshot::Nil { .. }
            | ConstantSnapshot::U128OutOfRange { .. } => (0, None),
        },
        InputValue::Data(value) => match value {
            DataSnapshot::Cons { fields } | DataSnapshot::Branch { fields } => {
                (2, Some((fields.start, fields.len)))
            }
            DataSnapshot::Leaf { fields } => (1, Some((fields.start, fields.len))),
            DataSnapshot::Constructor { fields, .. } => {
                (fields.len, Some((fields.start, fields.len)))
            }
            DataSnapshot::U128 { .. } | DataSnapshot::Bool { .. } | DataSnapshot::Nil => (0, None),
        },
        _ => return Err(fail(at, FailureKind::WrongOperation)),
    })
}

pub(super) fn build(
    input: &HostSnapshot,
    lifetime: LifetimeUnit,
) -> Result<WitnessPlan, WitnessFailure> {
    let mut b = Builder {
        input,
        inputs: Vec::new(),
        reads: BTreeMap::new(),
        nodes: Vec::new(),
    };
    let at = Location::new("snapshot", None, "outputs");
    let metadata = b.read(InputPath::Metadata, at)?;
    let mut fields = Vec::new();
    let mut constants = Vec::new();
    for n in 0..input.admission.view.constants.len() {
        constants.push(b.value(RefC::Const(n), Location::new("constants", Some(n), "value"))?);
    }
    let id = b.sequence(SequenceKind::Values, ValueType::Value, constants, at);
    fields.push(b.field("constants", id));
    let id = b.source_sequence(
        SequenceKind::References,
        SourceType::Reference,
        (0..input.admission.view.constant_fields.len())
            .map(InputPath::ConstantField)
            .collect(),
        at,
    )?;
    fields.push(b.field("constant_fields", id));
    let mut data = Vec::new();
    for n in 0..input.image.data.len() {
        data.push(b.value(RefC::Data(n), Location::new("data", Some(n), "value"))?);
    }
    let id = b.sequence(SequenceKind::Values, ValueType::Value, data, at);
    fields.push(b.field("data", id));
    let id = b.source_sequence(
        SequenceKind::References,
        SourceType::Reference,
        (0..input.image.data_fields.len())
            .map(InputPath::DataField)
            .collect(),
        at,
    )?;
    fields.push(b.field("data_fields", id));
    let mut envs = Vec::new();
    for n in 0..input.image.env.len() {
        envs.push(b.environment(
            Some(n),
            Location::new("environment_cells", Some(n), "chain"),
        )?);
    }
    let id = b.sequence(SequenceKind::Environments, ValueType::Environment, envs, at);
    fields.push(b.field("environment_cells", id));
    let id = b.environment(
        input.image.env_root,
        Location::new("snapshot", None, "current_environment"),
    )?;
    fields.push(b.field("current_environment", id));
    let mut frames = Vec::new();
    // B stores bottom-to-top; decoded reference-facing frames are top-first.
    for n in (0..input.image.frames.len()).rev() {
        frames.push(b.payload(
            InputPath::Frame(n),
            PayloadKind::Frame,
            Location::new("frames", Some(n), "payload"),
        )?);
    }
    let id = b.sequence(
        SequenceKind::Frames,
        ValueType::Payload(PayloadKind::Frame),
        frames,
        at,
    );
    fields.push(b.field("frames", id));
    for (name, path, kind) in [
        ("control", InputPath::Control, PayloadKind::Control),
        ("helper", InputPath::Helper, PayloadKind::Helper),
    ] {
        let id = b.payload(path, kind, Location::new("snapshot", None, name))?;
        fields.push(b.field(name, id));
    }
    let mut tasks = Vec::new();
    for n in 0..input.image.tasks.len() {
        tasks.push(b.payload(
            InputPath::Task(n),
            PayloadKind::Task,
            Location::new("tasks", Some(n), "payload"),
        )?);
    }
    let id = b.sequence(
        SequenceKind::Tasks,
        ValueType::Payload(PayloadKind::Task),
        tasks,
        at,
    );
    fields.push(b.field("tasks", id));
    let id = b.payload(
        InputPath::Result,
        PayloadKind::Result,
        Location::new("snapshot", None, "result"),
    )?;
    fields.push(b.field("result", id));
    let id = b.values_slice(
        SliceC {
            start: 0,
            len: input.image.value_slots.len(),
        },
        Location::new("snapshot", None, "value_slots"),
    )?;
    fields.push(b.field("value_slots", id));
    let id = b.payload(
        InputPath::Outcome,
        PayloadKind::Outcome,
        Location::new("snapshot", None, "outcome"),
    )?;
    fields.push(b.field("outcome", id));
    let p = &input.admission.view;
    for (name, kind, ty, paths) in [
        (
            "names",
            SequenceKind::Names,
            SourceType::Name,
            (0..p.names.len()).map(InputPath::Name).collect(),
        ),
        (
            "expressions",
            SequenceKind::Expressions,
            SourceType::Expression,
            (0..p.expressions.len())
                .map(InputPath::Expression)
                .collect(),
        ),
        (
            "parameters",
            SequenceKind::Parameters,
            SourceType::Parameter,
            (0..p.parameters.len()).map(InputPath::Parameter).collect(),
        ),
        (
            "arms",
            SequenceKind::Arms,
            SourceType::Arm,
            (0..p.arms.len()).map(InputPath::Arm).collect(),
        ),
        (
            "patterns",
            SequenceKind::Patterns,
            SourceType::Pattern,
            (0..p.patterns.len()).map(InputPath::Pattern).collect(),
        ),
        (
            "occurrences",
            SequenceKind::Occurrences,
            SourceType::Occurrence,
            (0..p.occurrences.len())
                .map(InputPath::Occurrence)
                .collect(),
        ),
        (
            "occurrence_paths",
            SequenceKind::Indices,
            SourceType::Index,
            (0..p.occurrence_paths.len())
                .map(InputPath::OccurrencePath)
                .collect(),
        ),
        (
            "children",
            SequenceKind::Indices,
            SourceType::Index,
            (0..p.children.len()).map(InputPath::Child).collect(),
        ),
        (
            "functions",
            SequenceKind::Functions,
            SourceType::Function,
            (0..p.functions.len()).map(InputPath::Function).collect(),
        ),
        (
            "pattern_children",
            SequenceKind::Indices,
            SourceType::Index,
            (0..p.pattern_children.len())
                .map(InputPath::PatternChild)
                .collect(),
        ),
        (
            "types",
            SequenceKind::Types,
            SourceType::Type,
            (0..p.types.len()).map(InputPath::Type).collect(),
        ),
        (
            "type_args",
            SequenceKind::TypeIds,
            SourceType::TypeId,
            (0..p.type_args.len()).map(InputPath::TypeArg).collect(),
        ),
        (
            "constructors",
            SequenceKind::Constructors,
            SourceType::Constructor,
            (0..p.constructors.len())
                .map(InputPath::Constructor)
                .collect(),
        ),
        (
            "ctor_fields",
            SequenceKind::ConstructorFields,
            SourceType::ConstructorField,
            (0..p.ctor_fields.len())
                .map(InputPath::ConstructorField)
                .collect(),
        ),
        (
            "decl_param_counts",
            SequenceKind::Indices,
            SourceType::Index,
            (0..p.decl_param_counts.len())
                .map(InputPath::DeclParamCount)
                .collect(),
        ),
        (
            "decl_ctor_counts",
            SequenceKind::Indices,
            SourceType::Index,
            (0..p.decl_ctor_counts.len())
                .map(InputPath::DeclCtorCount)
                .collect(),
        ),
        (
            "decl_templates",
            SequenceKind::DeclTemplates,
            SourceType::DeclTemplate,
            (0..p.decl_templates.len())
                .map(InputPath::DeclTemplate)
                .collect(),
        ),
        (
            "plan_nodes",
            SequenceKind::PlanNodes,
            SourceType::PlanNode,
            (0..p.plan_nodes.len()).map(InputPath::PlanNode).collect(),
        ),
        (
            "plan_cases",
            SequenceKind::PlanCases,
            SourceType::PlanCase,
            (0..p.plan_cases.len()).map(InputPath::PlanCase).collect(),
        ),
        (
            "field_paths",
            SequenceKind::Indices,
            SourceType::Index,
            (0..p.field_paths.len()).map(InputPath::FieldPath).collect(),
        ),
    ] {
        let id = b.source_sequence(kind, ty, paths, at)?;
        fields.push(b.field(name, id));
    }
    let mut deps = vec![b.dependency(metadata)];
    deps.extend(fields.iter().map(|f| f.dependency));
    let mut live_outs: Vec<_> = fields.iter().map(|f| f.dependency.node).collect();
    let output = b.add(
        Operation::Snapshot { fields },
        deps,
        ValueType::Snapshot,
        at,
    );
    live_outs.push(output);
    let stable_topological_order = dag::order(&b.nodes)?;
    let definitions = dag::definitions(&b.nodes, &stable_topological_order, &live_outs, lifetime)?;
    Ok(WitnessPlan {
        format: "sherlock-host-witness-plan/1",
        lifetime,
        inputs: b.inputs,
        nodes: b.nodes,
        stable_topological_order,
        definitions,
        live_outs,
        output,
    })
}

fn shifted(node: &ValueNode, offset: usize, at: Location) -> Result<ValueNode, WitnessFailure> {
    let add = |index: usize| {
        index
            .checked_add(offset)
            .ok_or_else(|| fail(at, FailureKind::IndexOverflow))
    };
    Ok(match *node {
        ValueNode::UInt(n) => ValueNode::UInt(n),
        ValueNode::Bool(v) => ValueNode::Bool(v),
        ValueNode::Nil => ValueNode::Nil,
        ValueNode::U128OutOfRange => ValueNode::U128OutOfRange,
        ValueNode::Cons { head, tail } => ValueNode::Cons {
            head: add(head)?,
            tail: add(tail)?,
        },
        ValueNode::Leaf { value } => ValueNode::Leaf { value: add(value)? },
        ValueNode::Branch { left, right } => ValueNode::Branch {
            left: add(left)?,
            right: add(right)?,
        },
        ValueNode::Constructor {
            type_id,
            ctor_id,
            ref fields,
        } => ValueNode::Constructor {
            type_id,
            ctor_id,
            fields: fields
                .iter()
                .map(|index| add(*index))
                .collect::<Result<Vec<_>, _>>()?,
        },
    })
}
pub(super) fn evaluate_value(
    plan: &WitnessPlan,
    node: &Node,
    source: RefC,
    arity: usize,
    args: &[&SlotValue],
) -> Result<DecodedValue, WitnessFailure> {
    let at = node.location;
    let expected_len = arity
        .checked_mul(2)
        .and_then(|v| v.checked_add(1))
        .ok_or_else(|| fail(at, FailureKind::IndexOverflow))?;
    if args.len() != expected_len {
        return Err(fail(at, FailureKind::IncompleteBindings));
    }
    let Some(SlotValue::Source(atom)) = args.first().copied() else {
        return Err(fail(at, FailureKind::WrongOperation));
    };
    match (source, atom) {
        (RefC::Const(_), InputValue::Constant(_)) | (RefC::Data(_), InputValue::Data(_)) => (),
        _ => return Err(fail(at, FailureKind::WrongOperation)),
    }
    let row_node = plan
        .nodes
        .get(node.dependencies[0].node.0)
        .ok_or_else(|| fail(at, FailureKind::IncompleteBindings))?;
    let Operation::Read(binding) = row_node.operation else {
        return Err(fail(at, FailureKind::IncompleteBindings));
    };
    let expected_path = match source {
        RefC::Const(n) => InputPath::Constant(n),
        RefC::Data(n) => InputPath::Data(n),
    };
    if plan.inputs.get(binding.0).map(|b| b.path) != Some(expected_path) {
        return Err(fail(at, FailureKind::IncompleteBindings));
    }
    if shape(atom, at)?.0 != arity {
        return Err(fail(
            at,
            FailureKind::WrongArity {
                expected: shape(atom, at)?.0,
                actual: arity,
            },
        ));
    }
    let mut nodes = Vec::new();
    let mut roots = Vec::new();
    for (index, pair) in args[1..].chunks_exact(2).enumerate() {
        let [
            SlotValue::Source(InputValue::Reference(reference)),
            SlotValue::Value(value),
        ] = pair
        else {
            return Err(fail(at, FailureKind::WrongOperation));
        };
        let child_node = &plan.nodes[node.dependencies[index * 2 + 2].node.0];
        if !matches!(child_node.operation,Operation::Value { source: child,.. } if child == *reference)
        {
            return Err(fail(at, FailureKind::IncompleteBindings));
        }
        let offset = nodes.len();
        if value.root >= value.nodes.len() {
            return Err(fail(at, FailureKind::IncompleteBindings));
        }
        for child in &value.nodes {
            nodes.push(shifted(child, offset, at)?);
        }
        roots.push(
            value
                .root
                .checked_add(offset)
                .ok_or_else(|| fail(at, FailureKind::IndexOverflow))?,
        );
    }
    let tail = match atom {
        InputValue::Constant(ConstantSnapshot::U128 { n, .. })
        | InputValue::Data(DataSnapshot::U128 { n }) => ValueNode::UInt(*n),
        InputValue::Constant(ConstantSnapshot::Bool { b, .. })
        | InputValue::Data(DataSnapshot::Bool { b }) => ValueNode::Bool(*b),
        InputValue::Constant(ConstantSnapshot::Nil { .. })
        | InputValue::Data(DataSnapshot::Nil) => ValueNode::Nil,
        InputValue::Constant(ConstantSnapshot::U128OutOfRange { .. }) => ValueNode::U128OutOfRange,
        InputValue::Constant(ConstantSnapshot::Cons { .. })
        | InputValue::Data(DataSnapshot::Cons { .. }) => ValueNode::Cons {
            head: roots[0],
            tail: roots[1],
        },
        InputValue::Constant(ConstantSnapshot::Leaf { .. })
        | InputValue::Data(DataSnapshot::Leaf { .. }) => ValueNode::Leaf { value: roots[0] },
        InputValue::Constant(ConstantSnapshot::Branch { .. })
        | InputValue::Data(DataSnapshot::Branch { .. }) => ValueNode::Branch {
            left: roots[0],
            right: roots[1],
        },
        InputValue::Constant(ConstantSnapshot::Constructor {
            type_id, ctor_id, ..
        })
        | InputValue::Data(DataSnapshot::Constructor {
            type_id, ctor_id, ..
        }) => ValueNode::Constructor {
            type_id: *type_id,
            ctor_id: *ctor_id,
            fields: roots,
        },
        _ => return Err(fail(at, FailureKind::WrongOperation)),
    };
    let sort = |index: usize| match nodes.get(index) {
        Some(ValueNode::UInt(_)) => Some(SortTag::UInt),
        Some(ValueNode::Bool(_)) => Some(SortTag::Bool),
        Some(ValueNode::Nil | ValueNode::Cons { .. }) => Some(SortTag::List),
        Some(ValueNode::Leaf { .. } | ValueNode::Branch { .. }) => Some(SortTag::Tree),
        _ => None,
    };
    let required = match tail {
        ValueNode::Cons { head, tail } => vec![(head, SortTag::UInt), (tail, SortTag::List)],
        ValueNode::Leaf { value } => vec![(value, SortTag::UInt)],
        ValueNode::Branch { left, right } => vec![(left, SortTag::Tree), (right, SortTag::Tree)],
        _ => vec![],
    };
    for (index, expected) in required {
        let actual = sort(index);
        if actual != Some(expected) {
            return Err(fail(at, FailureKind::WrongValueSort { expected, actual }));
        }
    }
    let root = nodes.len();
    nodes.push(tail);
    Ok(DecodedValue { nodes, root })
}
