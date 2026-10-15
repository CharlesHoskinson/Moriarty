use super::reference_types::*;
use std::sync::Arc;

/// Host reference evaluator for the frozen `sherlock-k-a/1` profile on the typed common domain.
///
/// # Security Warning (H1 — Unmetered Host Evaluator)
/// This machine is a host diagnostic reference with zero native authority and no host time,
/// heap, or native stack admission bounds. Do not expose it to untrusted inputs.
pub struct Machine {
    pub(super) state: Snapshot,
}

impl Machine {
    pub fn new(program: Arc<Program>, fuel: u128, stack_limit: usize) -> Self {
        Self {
            state: Snapshot {
                control: Some(Control::Eval(program.entry.clone())),
                program,
                environment: Environment::new(),
                frames: vec![],
                stack_limit,
                remaining_fuel: fuel,
                initial_fuel: fuel,
                outcome: Outcome::Pending,
            },
        }
    }

    pub fn snapshot(&self) -> Snapshot {
        self.state.clone()
    }

    /// Executes at most one `sherlock-k-a/1` transition or free terminal publication.
    ///
    /// Clones the full [`Snapshot`] twice (`before` and `after`), including environments and
    /// lists. Unmetered on host resources; not safe for untrusted input (H1).
    pub fn step(&mut self) -> Option<StepRecord> {
        if self.state.outcome != Outcome::Pending {
            return None;
        }
        let control = self.state.control.clone()?;
        let before = self.snapshot();

        // K terminal publication precedes exhaustion, even at zero fuel.
        let (rule_start_line, charge) = match control {
            Control::Return(value) if self.state.frames.is_empty() => {
                self.state.control = None;
                self.state.outcome = Outcome::Done(value);
                (80, 0)
            }
            Control::Error(code) => {
                self.state.control = None;
                self.state.outcome = Outcome::Rejected(code);
                (82, 0)
            }
            _ if self.state.remaining_fuel == 0 => {
                self.state.control = None;
                self.state.outcome = Outcome::Exhausted;
                (84, 0)
            }
            control => {
                let (line, next) = self.transition(control);
                self.state.control = Some(next);
                self.state.remaining_fuel -= 1;
                (line, 1)
            }
        };
        Some(StepRecord {
            rule_start_line,
            charge,
            before,
            after: self.snapshot(),
        })
    }

    // One charged machine rule from the pinned main.k. Pure helpers below do
    // not generate steps or consume fuel in the A profile.
    fn transition(&mut self, control: Control) -> (usize, Control) {
        match control {
            Control::Eval(expr) => self.evaluate(expr),
            Control::Push(frame, next, environment) => {
                if self.state.frames.len() < self.state.stack_limit {
                    self.state.frames.insert(0, frame);
                    self.state.environment = environment;
                    (135, *next)
                } else {
                    (140, Control::Error(4))
                }
            }
            Control::Return(value) => self.return_value(value),
            Control::Invoke(name, values) => {
                let function = self
                    .state
                    .program
                    .functions
                    .iter()
                    .find(|function| function.name == name)
                    .cloned();
                (200, Control::Enter(function, values))
            }
            Control::Enter(None, _) => (202, Control::Error(7)),
            Control::Enter(Some(function), values) => {
                let bindings = bind(&function.parameters, &values);
                (
                    204,
                    Control::Bound(bindings, function.return_type, function.body),
                )
            }
            Control::Bound(None, _, _) => (206, Control::Error(8)),
            Control::Bound(Some(environment), ty, body) => {
                let frame = Frame::Return(ty, self.state.environment.clone());
                (208, push(frame, Control::Eval(body), environment))
            }
            Control::Cases(_, arms, _) if arms.is_empty() => (212, Control::Error(9)),
            Control::Cases(value, mut arms, environment) => {
                let arm = arms.remove(0);
                let bindings = matching(&arm.pattern, &value, &environment);
                (
                    214,
                    Control::CaseResult(bindings, value, arm.body, arms, environment),
                )
            }
            Control::CaseResult(None, value, _, arms, environment) => {
                (217, Control::Cases(value, arms, environment))
            }
            Control::CaseResult(Some(bindings), _, body, _, environment) => (
                220,
                push(Frame::Scope(environment), Control::Eval(body), bindings),
            ),
            Control::PrimitiveResult(Ok(value)) => (223, Control::Return(value)),
            Control::PrimitiveResult(Err(code)) => (225, Control::Error(code)),
            // Error publication is handled by step without a fuel charge.
            Control::Error(_) => unreachable!("error publication is administrative"),
        }
    }

    fn evaluate(&self, expr: ExprRef) -> (usize, Control) {
        let environment = &self.state.environment;
        match expr.as_ref() {
            // Every value representable by this typed API satisfies uiValid.
            Expr::Literal(value) => (102, Control::Return(value.clone())),
            Expr::Var(name) => match environment.get(name) {
                Some(value) => (108, Control::Return(value.clone())),
                None => (111, Control::Error(3)),
            },
            Expr::Let(name, value, body) => {
                let frame = Frame::Let(name.clone(), body.clone(), environment.clone());
                (
                    114,
                    push(frame, Control::Eval(value.clone()), environment.clone()),
                )
            }
            Expr::Binary(op, left, right) => {
                let frame = Frame::Left(*op, right.clone(), environment.clone());
                (
                    117,
                    push(frame, Control::Eval(left.clone()), environment.clone()),
                )
            }
            Expr::Unary(op, value) => (
                120,
                push(
                    Frame::Unary(*op),
                    Control::Eval(value.clone()),
                    environment.clone(),
                ),
            ),
            Expr::If(condition, yes, no) => {
                let frame = Frame::If(yes.clone(), no.clone(), environment.clone());
                (
                    123,
                    push(frame, Control::Eval(condition.clone()), environment.clone()),
                )
            }
            Expr::Match(value, arms) => {
                let frame = Frame::Match(arms.clone(), environment.clone());
                (
                    126,
                    push(frame, Control::Eval(value.clone()), environment.clone()),
                )
            }
            Expr::Call(name, args) if args.is_empty() => {
                (129, Control::Invoke(name.clone(), vec![]))
            }
            Expr::Call(name, args) => {
                let frame = Frame::Args(
                    name.clone(),
                    vec![],
                    args[1..].to_vec(),
                    environment.clone(),
                );
                (
                    131,
                    push(frame, Control::Eval(args[0].clone()), environment.clone()),
                )
            }
        }
    }

    fn return_value(&mut self, value: Value) -> (usize, Control) {
        // The empty-stack return was already published by step. Replacements
        // retain depth; only source rules with a stack rewrite to S pop it.
        match self.state.frames[0].clone() {
            Frame::Let(name, body, environment) => {
                self.state.frames[0] = Frame::Scope(environment.clone());
                self.state.environment = environment;
                self.state.environment.insert(name, value);
                (145, Control::Eval(body))
            }
            Frame::Left(op, right, environment) => {
                self.state.frames[0] = Frame::Right(op, value);
                self.state.environment = environment;
                (149, Control::Eval(right))
            }
            Frame::Right(op, left) => {
                self.state.frames.remove(0);
                (
                    153,
                    Control::PrimitiveResult(binary_result(op, left, value)),
                )
            }
            Frame::Unary(op) => {
                self.state.frames.remove(0);
                (157, Control::PrimitiveResult(unary_result(op, value)))
            }
            Frame::If(yes, no, environment) => match value {
                Value::Bool(condition) => {
                    self.state.frames.remove(0);
                    self.state.environment = environment;
                    if condition {
                        (161, Control::Eval(yes))
                    } else {
                        (165, Control::Eval(no))
                    }
                }
                _ => (169, Control::Error(5)),
            },
            Frame::Match(arms, environment) => {
                self.state.frames.remove(0);
                self.state.environment = environment.clone();
                (173, Control::Cases(value, arms, environment))
            }
            Frame::Args(name, mut values, mut rest, environment) => {
                values.push(value);
                self.state.environment = environment.clone();
                if rest.is_empty() {
                    self.state.frames.remove(0);
                    (181, Control::Invoke(name, values))
                } else {
                    let next = rest.remove(0);
                    self.state.frames[0] = Frame::Args(name, values, rest, environment);
                    (177, Control::Eval(next))
                }
            }
            Frame::Return(ty, environment) => {
                if has_type(&value, ty) {
                    self.state.frames.remove(0);
                    self.state.environment = environment;
                    (185, Control::Return(value))
                } else {
                    (190, Control::Error(6))
                }
            }
            Frame::Scope(environment) => {
                self.state.frames.remove(0);
                self.state.environment = environment;
                (195, Control::Return(value))
            }
        }
    }

    /// Runs the machine until a terminal [`Outcome`] (`Done`, `Rejected`, or `Exhausted`) is
    /// published.
    ///
    /// # Security Warning (H1 — DoS and Native Stack Overflow Risks)
    /// This method is an **unmetered** host reference evaluator and **must not** be connected to
    /// the CLI, LSP, MCP, the `/1` or `/2` source compiler adapter, or any untrusted/network-reachable
    /// input path until host time, memory, and nesting bounds are enforced:
    /// - `fuel: u128` and `stack_limit: usize` have no host ceiling.
    /// - Every step clones the full [`Snapshot`] twice (`before`/`after`) and `run()` discards
    ///   both copies; every pushed [`Frame`] clones its [`Environment`]; `BinaryOp::Cons` is $O(n)$.
    /// - Derived `Drop`, `Clone`, and `PartialEq` impls (as well as diagnostic K rendering)
    ///   recurse over `Arc<Expr>` and `Arc<Tree>`. Deep caller-supplied ASTs or runtime-built
    ///   `Branch`/`Leaf` trees can overflow the native call stack and abort the process.
    pub fn run(&mut self) -> Snapshot {
        while self.step().is_some() {}
        self.snapshot()
    }
}

fn push(frame: Frame, control: Control, environment: Environment) -> Control {
    Control::Push(frame, Box::new(control), environment)
}

// main.k:238-245. Type tests concern the value's sort, not its contents.
fn has_type(value: &Value, ty: Type) -> bool {
    matches!(
        (value, ty),
        (Value::UInt(_), Type::UInt)
            | (Value::Bool(_), Type::Bool)
            | (Value::List(_), Type::List)
            | (Value::Tree(_), Type::Tree)
    )
}

// main.k:250-272. Translate uiChecked's range failure to error 10 using
// checked arithmetic on the typed U128 input domain.
fn binary_result(op: BinaryOp, left: Value, right: Value) -> Result<Value, u8> {
    match (op, left, right) {
        (BinaryOp::Add, Value::UInt(a), Value::UInt(b)) => {
            a.checked_add(b).map(Value::UInt).ok_or(10)
        }
        (BinaryOp::Sub, Value::UInt(a), Value::UInt(b)) => {
            a.checked_sub(b).map(Value::UInt).ok_or(10)
        }
        (BinaryOp::Mul, Value::UInt(a), Value::UInt(b)) => {
            a.checked_mul(b).map(Value::UInt).ok_or(10)
        }
        (BinaryOp::Lt, Value::UInt(a), Value::UInt(b)) => Ok(Value::Bool(a < b)),
        (BinaryOp::Eq, a, b) => Ok(Value::Bool(a == b)),
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

// main.k:273-278.
fn unary_result(op: UnaryOp, value: Value) -> Result<Value, u8> {
    match (op, value) {
        (UnaryOp::Leaf, Value::UInt(value)) => Ok(Value::Tree(Tree::Leaf(value))),
        (UnaryOp::Not, Value::Bool(value)) => Ok(Value::Bool(!value)),
        _ => Err(11),
    }
}

// main.k:290-298. uiBind builds its tail first, and duplicate names or
// argument count/type mismatches make the complete binding fail.
fn bind(parameters: &[Parameter], values: &[Value]) -> Option<Environment> {
    if parameters.len() != values.len() {
        return None;
    }
    let mut environment = Environment::new();
    for (parameter, value) in parameters.iter().zip(values).rev() {
        if !has_type(value, parameter.ty) || environment.contains_key(&parameter.name) {
            return None;
        }
        environment.insert(parameter.name.clone(), value.clone());
    }
    Some(environment)
}

// main.k:300-319. Each successful pattern extends the saved environment;
// equal names in Cons/Branch are a miss, including on matching value kinds.
fn matching(pattern: &Pattern, value: &Value, saved: &Environment) -> Option<Environment> {
    let mut environment = saved.clone();
    match (pattern, value) {
        (Pattern::Var(name), value) => {
            environment.insert(name.clone(), value.clone());
        }
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
