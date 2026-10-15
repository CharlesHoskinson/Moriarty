//! Private host-consumer proposal for builder-produced ProgramImageDraft tables.
//!
//! Transitions fetch ExprId rows and ordered function/parameter/arm tables.
//! Recursive source expressions are reconstructed once for Snapshot observations
//! only; they never drive evaluation. Constants decode once to the existing
//! typed runtime Values. There is no existing Machine execution dependency.
//!
//! This preserves the frozen K-A schedule, including unmetered pure helpers and
//! host allocation. It selects no B schema, capacity, admission/error policy,
//! native layout or public input path. Arbitrary hand-built/malformed images are
//! outside this proposal's domain. Host usize positions remain host metadata.

use super::program_image::{ConstantPayload, ExprId, ExprPayload, ProgramImageDraft, TableSlice};
use super::reference_types::*;
use std::sync::Arc;

#[derive(Clone)]
enum ImageFrame {
    Let(Name, ExprId, Environment),
    Left(BinaryOp, ExprId, Environment),
    Right(BinaryOp, Value),
    Unary(UnaryOp),
    If(ExprId, ExprId, Environment),
    Match(TableSlice, Environment),
    Args(Name, Vec<Value>, TableSlice, Environment),
    Return(Type, Environment),
    Scope(Environment),
}

#[derive(Clone)]
enum ImageControl {
    Eval(ExprId),
    Return(Value),
    Push(ImageFrame, Box<ImageControl>, Environment),
    Invoke(Name, Vec<Value>),
    Enter(Option<usize>, Vec<Value>),
    Bound(Option<Environment>, Type, ExprId),
    Cases(Value, TableSlice, Environment),
    CaseResult(Option<Environment>, Value, ExprId, TableSlice, Environment),
    PrimitiveResult(Result<Value, u8>),
    Error(u8),
}

/// Host-only source candidate. Integrator wiring is limited to cfg(test).
/// No arbitrary-state importer or observer-controlled transition flags exist.
pub(super) struct ImageMachine {
    image: Arc<ProgramImageDraft>,
    constants: Vec<Value>,
    observations: SourceObservations,
    control: Option<ImageControl>,
    environment: Environment,
    frames: Vec<ImageFrame>,
    stack_limit: usize,
    remaining_fuel: u128,
    initial_fuel: u128,
    outcome: Outcome,
}

impl ImageMachine {
    /// Requires an unchanged image produced by prepare_program_image. This
    /// constructor deliberately adds no policy for arbitrary malformed tables.
    pub(super) fn new(image: Arc<ProgramImageDraft>, fuel: u128, stack_limit: usize) -> Self {
        let constants = decode_constants(&image);
        let observations = SourceObservations::new(&image, &constants);
        Self {
            control: Some(ImageControl::Eval(image.entry)),
            image,
            constants,
            observations,
            environment: Environment::new(),
            frames: Vec::new(),
            stack_limit,
            remaining_fuel: fuel,
            initial_fuel: fuel,
            outcome: Outcome::Pending,
        }
    }

    pub(super) fn snapshot(&self) -> Snapshot {
        Snapshot {
            program: self.observations.program.clone(),
            control: self.control.as_ref().map(|control| self.observe_control(control)),
            environment: self.environment.clone(),
            frames: self.frames.iter().map(|frame| self.observe_frame(frame)).collect(),
            stack_limit: self.stack_limit,
            remaining_fuel: self.remaining_fuel,
            initial_fuel: self.initial_fuel,
            outcome: self.outcome.clone(),
        }
    }

    /// One charged K-A rule, or free terminal publication. Snapshot conversion
    /// and cloning are unmetered host work, as in the existing A reference.
    pub(super) fn step(&mut self) -> Option<StepRecord> {
        if self.outcome != Outcome::Pending {
            return None;
        }
        let control = self.control.clone()?;
        let before = self.snapshot();
        let (rule_start_line, charge) = match control {
            ImageControl::Return(value) if self.frames.is_empty() => {
                self.control = None;
                self.outcome = Outcome::Done(value);
                (80, 0)
            }
            ImageControl::Error(code) => {
                self.control = None;
                self.outcome = Outcome::Rejected(code);
                (82, 0)
            }
            _ if self.remaining_fuel == 0 => {
                self.control = None;
                self.outcome = Outcome::Exhausted;
                (84, 0)
            }
            control => {
                let (line, next) = self.transition(control);
                self.control = Some(next);
                self.remaining_fuel -= 1;
                (line, 1)
            }
        };
        Some(StepRecord { rule_start_line, charge, before, after: self.snapshot() })
    }

    pub(super) fn run(&mut self) -> Snapshot {
        while self.step().is_some() {}
        self.snapshot()
    }

    fn transition(&mut self, control: ImageControl) -> (usize, ImageControl) {
        match control {
            ImageControl::Eval(id) => self.evaluate(id),
            ImageControl::Return(value) => self.return_value(value),
            ImageControl::Push(frame, next, environment) => {
                if self.frames.len() < self.stack_limit {
                    self.frames.insert(0, frame);
                    self.environment = environment;
                    (135, *next)
                } else {
                    (140, ImageControl::Error(4))
                }
            }
            ImageControl::Invoke(name, values) => {
                // Preserve source order and first match, including duplicate names.
                let found = self.image.functions.iter().position(|function| function.name == name);
                (200, ImageControl::Enter(found, values))
            }
            ImageControl::Enter(None, _) => (202, ImageControl::Error(7)),
            ImageControl::Enter(Some(index), values) => {
                let function = &self.image.functions[index];
                let parameters = &self.image.ordered_parameters[
                    function.parameters.start..function.parameters.start + function.parameters.len];
                (204, ImageControl::Bound(bind(parameters, &values), function.return_type, function.body))
            }
            ImageControl::Bound(None, _, _) => (206, ImageControl::Error(8)),
            ImageControl::Bound(Some(environment), ty, body) => {
                let frame = ImageFrame::Return(ty, self.environment.clone());
                (208, push(frame, ImageControl::Eval(body), environment))
            }
            ImageControl::Cases(_, arms, _) if arms.len == 0 => (212, ImageControl::Error(9)),
            ImageControl::Cases(value, arms, environment) => {
                let arm = &self.image.match_arms[arms.start];
                let bindings = matching(&arm.pattern, &value, &environment);
                (214, ImageControl::CaseResult(bindings, value, arm.body, tail(arms), environment))
            }
            ImageControl::CaseResult(None, value, _, arms, environment) => {
                (217, ImageControl::Cases(value, arms, environment))
            }
            ImageControl::CaseResult(Some(bindings), _, body, _, environment) => {
                (220, push(ImageFrame::Scope(environment), ImageControl::Eval(body), bindings))
            }
            ImageControl::PrimitiveResult(Ok(value)) => (223, ImageControl::Return(value)),
            ImageControl::PrimitiveResult(Err(code)) => (225, ImageControl::Error(code)),
            ImageControl::Error(_) => unreachable!("error publication precedes charged rules"),
        }
    }

    fn evaluate(&self, id: ExprId) -> (usize, ImageControl) {
        let row = &self.image.expressions[id.0];
        let children = &self.image.ordered_children[
            row.children.start..row.children.start + row.children.len];
        let environment = &self.environment;
        match &row.payload {
            ExprPayload::Literal(constant) => (102, ImageControl::Return(self.constants[constant.0].clone())),
            ExprPayload::Var(name) => match environment.get(name) {
                Some(value) => (108, ImageControl::Return(value.clone())),
                None => (111, ImageControl::Error(3)),
            },
            ExprPayload::Let(name) => {
                let frame = ImageFrame::Let(name.clone(), children[1], environment.clone());
                (114, push(frame, ImageControl::Eval(children[0]), environment.clone()))
            }
            ExprPayload::Binary(op) => {
                let frame = ImageFrame::Left(*op, children[1], environment.clone());
                (117, push(frame, ImageControl::Eval(children[0]), environment.clone()))
            }
            ExprPayload::Unary(op) => {
                (120, push(ImageFrame::Unary(*op), ImageControl::Eval(children[0]), environment.clone()))
            }
            ExprPayload::If => {
                let frame = ImageFrame::If(children[1], children[2], environment.clone());
                (123, push(frame, ImageControl::Eval(children[0]), environment.clone()))
            }
            ExprPayload::Match(arms) => {
                let frame = ImageFrame::Match(*arms, environment.clone());
                (126, push(frame, ImageControl::Eval(children[0]), environment.clone()))
            }
            ExprPayload::Call(name) if children.is_empty() => {
                (129, ImageControl::Invoke(name.clone(), Vec::new()))
            }
            ExprPayload::Call(name) => {
                let frame = ImageFrame::Args(name.clone(), Vec::new(), tail(row.children), environment.clone());
                (131, push(frame, ImageControl::Eval(children[0]), environment.clone()))
            }
        }
    }

    fn return_value(&mut self, value: Value) -> (usize, ImageControl) {
        // Frames remain top first. Replacement rules keep their logical depth;
        // failing if/return type tests keep every frame and saved environment.
        match self.frames[0].clone() {
            ImageFrame::Let(name, body, environment) => {
                self.frames[0] = ImageFrame::Scope(environment.clone());
                self.environment = environment;
                self.environment.insert(name, value);
                (145, ImageControl::Eval(body))
            }
            ImageFrame::Left(op, right, environment) => {
                self.frames[0] = ImageFrame::Right(op, value);
                self.environment = environment;
                (149, ImageControl::Eval(right))
            }
            ImageFrame::Right(op, left) => {
                self.frames.remove(0);
                (153, ImageControl::PrimitiveResult(binary_result(op, left, value)))
            }
            ImageFrame::Unary(op) => {
                self.frames.remove(0);
                (157, ImageControl::PrimitiveResult(unary_result(op, value)))
            }
            ImageFrame::If(yes, no, environment) => match value {
                Value::Bool(condition) => {
                    self.frames.remove(0);
                    self.environment = environment;
                    if condition { (161, ImageControl::Eval(yes)) }
                    else { (165, ImageControl::Eval(no)) }
                }
                _ => (169, ImageControl::Error(5)),
            },
            ImageFrame::Match(arms, environment) => {
                self.frames.remove(0);
                self.environment = environment.clone();
                (173, ImageControl::Cases(value, arms, environment))
            }
            ImageFrame::Args(name, mut values, rest, environment) => {
                values.push(value);
                self.environment = environment.clone();
                if rest.len == 0 {
                    self.frames.remove(0);
                    (181, ImageControl::Invoke(name, values))
                } else {
                    let next = self.image.ordered_children[rest.start];
                    self.frames[0] = ImageFrame::Args(name, values, tail(rest), environment);
                    (177, ImageControl::Eval(next))
                }
            }
            ImageFrame::Return(ty, environment) => {
                if has_type(&value, ty) {
                    self.frames.remove(0);
                    self.environment = environment;
                    (185, ImageControl::Return(value))
                } else {
                    (190, ImageControl::Error(6))
                }
            }
            ImageFrame::Scope(environment) => {
                self.frames.remove(0);
                self.environment = environment;
                (195, ImageControl::Return(value))
            }
        }
    }

    fn observe_frame(&self, frame: &ImageFrame) -> Frame {
        let source = &self.observations;
        match frame {
            ImageFrame::Let(name, body, environment) => Frame::Let(name.clone(), source.expression(*body), environment.clone()),
            ImageFrame::Left(op, right, environment) => Frame::Left(*op, source.expression(*right), environment.clone()),
            ImageFrame::Right(op, value) => Frame::Right(*op, value.clone()),
            ImageFrame::Unary(op) => Frame::Unary(*op),
            ImageFrame::If(yes, no, environment) => Frame::If(source.expression(*yes), source.expression(*no), environment.clone()),
            ImageFrame::Match(arms, environment) => Frame::Match(source.arms(&self.image, *arms), environment.clone()),
            ImageFrame::Args(name, values, rest, environment) => Frame::Args(
                name.clone(), values.clone(), source.children(&self.image, *rest), environment.clone()),
            ImageFrame::Return(ty, environment) => Frame::Return(*ty, environment.clone()),
            ImageFrame::Scope(environment) => Frame::Scope(environment.clone()),
        }
    }

    fn observe_control(&self, control: &ImageControl) -> Control {
        let source = &self.observations;
        match control {
            ImageControl::Eval(id) => Control::Eval(source.expression(*id)),
            ImageControl::Return(value) => Control::Return(value.clone()),
            ImageControl::Push(frame, next, environment) => Control::Push(
                self.observe_frame(frame), Box::new(self.observe_control(next)), environment.clone()),
            ImageControl::Invoke(name, values) => Control::Invoke(name.clone(), values.clone()),
            ImageControl::Enter(index, values) => Control::Enter(
                index.map(|index| source.program.functions[index].clone()), values.clone()),
            ImageControl::Bound(bindings, ty, body) => Control::Bound(bindings.clone(), *ty, source.expression(*body)),
            ImageControl::Cases(value, arms, environment) => Control::Cases(
                value.clone(), source.arms(&self.image, *arms), environment.clone()),
            ImageControl::CaseResult(bindings, value, body, rest, environment) => Control::CaseResult(
                bindings.clone(), value.clone(), source.expression(*body), source.arms(&self.image, *rest), environment.clone()),
            ImageControl::PrimitiveResult(result) => Control::PrimitiveResult(result.clone()),
            ImageControl::Error(code) => Control::Error(*code),
        }
    }
}

fn push(frame: ImageFrame, next: ImageControl, environment: Environment) -> ImageControl {
    ImageControl::Push(frame, Box::new(next), environment)
}

/// Only applied to a nonempty ordered child or arm slice.
fn tail(slice: TableSlice) -> TableSlice {
    TableSlice { start: slice.start + 1, len: slice.len - 1 }
}

/// Immutable observation cache: reverse preorder resolves children before their
/// parents in builder-produced images. No recursive AST traversal or execution
/// is used to construct it, and the transition methods never read this cache.
struct SourceObservations {
    expressions: Vec<ExprRef>,
    program: Arc<Program>,
}

impl SourceObservations {
    fn new(image: &ProgramImageDraft, constants: &[Value]) -> Self {
        let mut expressions: Vec<Option<ExprRef>> = vec![None; image.expressions.len()];
        for index in (0..image.expressions.len()).rev() {
            let row = &image.expressions[index];
            let children = &image.ordered_children[row.children.start..row.children.start + row.children.len];
            let child = |position: usize| expressions[children[position].0].as_ref()
                .expect("builder child follows its parent").clone();
            let expression = match &row.payload {
                ExprPayload::Literal(id) => Expr::Literal(constants[id.0].clone()),
                ExprPayload::Var(name) => Expr::Var(name.clone()),
                ExprPayload::Let(name) => Expr::Let(name.clone(), child(0), child(1)),
                ExprPayload::Binary(op) => Expr::Binary(*op, child(0), child(1)),
                ExprPayload::Unary(op) => Expr::Unary(*op, child(0)),
                ExprPayload::If => Expr::If(child(0), child(1), child(2)),
                ExprPayload::Call(name) => Expr::Call(name.clone(), (0..children.len()).map(child).collect()),
                ExprPayload::Match(arms) => Expr::Match(child(0), image.match_arms[
                    arms.start..arms.start + arms.len].iter().map(|arm| Arm {
                        pattern: arm.pattern.clone(),
                        body: expressions[arm.body.0].as_ref().expect("builder arm follows parent").clone(),
                    }).collect()),
            };
            expressions[index] = Some(Arc::new(expression));
        }
        let expressions: Vec<ExprRef> = expressions.into_iter()
            .map(|expression| expression.expect("all builder rows reconstructed")).collect();
        let functions = image.functions.iter().map(|function| Function {
            name: function.name.clone(),
            parameters: image.ordered_parameters[
                function.parameters.start..function.parameters.start + function.parameters.len].to_vec(),
            return_type: function.return_type,
            body: expressions[function.body.0].clone(),
        }).collect();
        let program = Arc::new(Program { functions, entry: expressions[image.entry.0].clone() });
        Self { expressions, program }
    }

    fn expression(&self, id: ExprId) -> ExprRef {
        self.expressions[id.0].clone()
    }

    fn children(&self, image: &ProgramImageDraft, slice: TableSlice) -> Vec<ExprRef> {
        image.ordered_children[slice.start..slice.start + slice.len].iter()
            .map(|id| self.expression(*id)).collect()
    }

    fn arms(&self, image: &ProgramImageDraft, slice: TableSlice) -> Vec<Arm> {
        image.match_arms[slice.start..slice.start + slice.len].iter().map(|arm| Arm {
            pattern: arm.pattern.clone(), body: self.expression(arm.body),
        }).collect()
    }
}

/// Normal builder constants are preorder too. The typed runtime's recursive
/// Values/Tree remain host values; this is neither a bounded arena nor native IR.
fn decode_constants(image: &ProgramImageDraft) -> Vec<Value> {
    let mut values: Vec<Option<Value>> = vec![None; image.constants.len()];
    for index in (0..image.constants.len()).rev() {
        let row = &image.constants[index];
        let fields = &image.constant_fields[row.fields.start..row.fields.start + row.fields.len];
        let field = |position: usize| values[fields[position].0].as_ref()
            .expect("builder constant child follows parent");
        let value = match row.payload {
            ConstantPayload::UInt(value) => Value::UInt(value),
            ConstantPayload::Bool(value) => Value::Bool(value),
            ConstantPayload::List => Value::List((0..fields.len()).map(|position| match field(position) {
                Value::UInt(value) => *value,
                _ => unreachable!("builder list fields are UInt"),
            }).collect()),
            ConstantPayload::TreeLeaf(value) => Value::Tree(Tree::Leaf(value)),
            ConstantPayload::TreeBranch => match (field(0), field(1)) {
                (Value::Tree(left), Value::Tree(right)) => Value::Tree(Tree::Branch(
                    Arc::new(left.clone()), Arc::new(right.clone()))),
                _ => unreachable!("builder branch fields are trees"),
            },
        };
        values[index] = Some(value);
    }
    values.into_iter().map(|value| value.expect("all builder constants decoded")).collect()
}

// Mechanical K-A pure helpers, kept local rather than calling reference::Machine
// or its helpers. These deliberately share the existing Value/Environment types
// and mirror reference.rs's rule decomposition; that reduces independence.
// Independent literal normal oracles remain the comparison authority.
fn has_type(value: &Value, ty: Type) -> bool {
    matches!((value, ty),
        (Value::UInt(_), Type::UInt) | (Value::Bool(_), Type::Bool)
        | (Value::List(_), Type::List) | (Value::Tree(_), Type::Tree))
}

fn binary_result(op: BinaryOp, left: Value, right: Value) -> Result<Value, u8> {
    match (op, left, right) {
        (BinaryOp::Add, Value::UInt(a), Value::UInt(b)) => a.checked_add(b).map(Value::UInt).ok_or(10),
        (BinaryOp::Sub, Value::UInt(a), Value::UInt(b)) => a.checked_sub(b).map(Value::UInt).ok_or(10),
        (BinaryOp::Mul, Value::UInt(a), Value::UInt(b)) => a.checked_mul(b).map(Value::UInt).ok_or(10),
        (BinaryOp::Lt, Value::UInt(a), Value::UInt(b)) => Ok(Value::Bool(a < b)),
        (BinaryOp::Eq, left, right) => Ok(Value::Bool(left == right)),
        (BinaryOp::Cons, Value::UInt(head), Value::List(mut tail)) => {
            tail.insert(0, head);
            Ok(Value::List(tail))
        }
        (BinaryOp::Branch, Value::Tree(left), Value::Tree(right)) => {
            Ok(Value::Tree(Tree::Branch(Arc::new(left), Arc::new(right))))
        }
        _ => Err(11),
    }
}

fn unary_result(op: UnaryOp, value: Value) -> Result<Value, u8> {
    match (op, value) {
        (UnaryOp::Leaf, Value::UInt(value)) => Ok(Value::Tree(Tree::Leaf(value))),
        (UnaryOp::Not, Value::Bool(value)) => Ok(Value::Bool(!value)),
        _ => Err(11),
    }
}

fn bind(parameters: &[Parameter], values: &[Value]) -> Option<Environment> {
    if parameters.len() != values.len() { return None; }
    let mut environment = Environment::new();
    for (parameter, value) in parameters.iter().zip(values).rev() {
        if !has_type(value, parameter.ty) || environment.contains_key(&parameter.name) {
            return None;
        }
        environment.insert(parameter.name.clone(), value.clone());
    }
    Some(environment)
}

fn matching(pattern: &Pattern, value: &Value, saved: &Environment) -> Option<Environment> {
    let mut environment = saved.clone();
    match (pattern, value) {
        (Pattern::Var(name), value) => { environment.insert(name.clone(), value.clone()); }
        (Pattern::Nil, Value::List(values)) if values.is_empty() => {}
        (Pattern::Cons(head, tail), Value::List(values)) if head != tail && !values.is_empty() => {
            environment.insert(head.clone(), Value::UInt(values[0]));
            environment.insert(tail.clone(), Value::List(values[1..].to_vec()));
        }
        (Pattern::Leaf(name), Value::Tree(Tree::Leaf(value))) => {
            environment.insert(name.clone(), Value::UInt(*value));
        }
        (Pattern::Branch(left, right), Value::Tree(Tree::Branch(a, b))) if left != right => {
            environment.insert(left.clone(), Value::Tree(a.as_ref().clone()));
            environment.insert(right.clone(), Value::Tree(b.as_ref().clone()));
        }
        _ => return None,
    }
    Some(environment)
}

#[cfg(test)]
#[path = "image_reference_tests.rs"]
mod tests;
