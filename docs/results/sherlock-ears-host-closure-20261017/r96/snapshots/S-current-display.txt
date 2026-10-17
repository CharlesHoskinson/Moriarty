//! Diagnostic rendering in the frozen `sherlock-k-a/1` vocabulary.
//!
//! Names are printed verbatim as opaque host symbols. Only canonical
//! signed-decimal names belong to the common K comparison domain. This display
//! is not a parser, persistence format, state import, or native commitment.

use super::reference_types::{
    Arm, BinaryOp, Control, Environment, Expr, Frame, Function, Outcome, Pattern, Program, Tree,
    Type, UnaryOp, Value,
};

/// Render a typed reference observation using the frozen K constructors.
pub trait KDisplay {
    fn to_k(&self) -> String;
}

// Each head may contain several comma-separated fields (parameters and cases).
// Open all cons constructors in source order, then close them after the nil.
fn sequence<T>(items: &[T], cons: &str, nil: &str, head: impl Fn(&T) -> String) -> String {
    let mut output = String::new();
    for item in items {
        output.push_str(cons);
        output.push('(');
        output.push_str(&head(item));
        output.push_str(", ");
    }
    output.push_str(nil);
    output.extend(std::iter::repeat(')').take(items.len()));
    output
}

fn expressions_to_k(expressions: &[super::reference_types::ExprRef]) -> String {
    sequence(expressions, "uiExpr", "uiNoExprs()", |expr| expr.to_k())
}

fn values_to_k(values: &[Value]) -> String {
    sequence(values, "uiValue", "uiNoValues()", KDisplay::to_k)
}

fn cases_to_k(arms: &[Arm]) -> String {
    sequence(arms, "uiCase", "uiNoCases()", |arm| {
        format!("{}, {}", arm.pattern.to_k(), arm.body.to_k())
    })
}

/// Map entries follow ascending host `String` order; K maps are unordered.
pub fn environment_to_k(environment: &Environment) -> String {
    if environment.is_empty() {
        return ".Map".into();
    }
    environment
        .iter()
        .map(|(name, value)| format!("{} |-> {}", name, value.to_k()))
        .collect::<Vec<_>>()
        .join(" ")
}

impl KDisplay for Program {
    /// Render only the function table, corresponding to the K `code` cell.
    fn to_k(&self) -> String {
        sequence(&self.functions, "uiFunction", "uiNoFuns()", KDisplay::to_k)
    }
}

impl KDisplay for Function {
    fn to_k(&self) -> String {
        let parameters = sequence(&self.parameters, "uiParam", "uiNoParams()", |parameter| {
            format!("{}, {}", parameter.name, parameter.ty.to_k())
        });
        format!(
            "uiFun({}, {}, {}, {})",
            self.name,
            parameters,
            self.return_type.to_k(),
            self.body.to_k()
        )
    }
}

impl KDisplay for Type {
    fn to_k(&self) -> String {
        match self {
            Self::UInt => "uiUIntT()",
            Self::Bool => "uiBoolT()",
            Self::List => "uiListT()",
            Self::Tree => "uiTreeT()",
        }
        .into()
    }
}

impl KDisplay for BinaryOp {
    fn to_k(&self) -> String {
        match self {
            Self::Add => "uiAdd()",
            Self::Sub => "uiSub()",
            Self::Mul => "uiMul()",
            Self::Eq => "uiEq()",
            Self::Lt => "uiLt()",
            Self::Cons => "uiConsOp()",
            Self::Branch => "uiBranchOp()",
        }
        .into()
    }
}

impl KDisplay for UnaryOp {
    fn to_k(&self) -> String {
        match self {
            Self::Leaf => "uiLeafOp()",
            Self::Not => "uiNotOp()",
        }
        .into()
    }
}

impl KDisplay for Tree {
    fn to_k(&self) -> String {
        match self {
            Self::Leaf(value) => format!("uiLeaf(uiU({}))", value),
            Self::Branch(left, right) => format!("uiBranch({}, {})", left.to_k(), right.to_k()),
        }
    }
}

impl KDisplay for Value {
    fn to_k(&self) -> String {
        match self {
            Self::UInt(value) => format!("uiU({})", value),
            Self::Bool(value) => format!("uiB({})", value),
            Self::List(values) => sequence(values, "uiCons", "uiNil()", |value| {
                format!("uiU({})", value)
            }),
            Self::Tree(tree) => tree.to_k(),
        }
    }
}

impl KDisplay for Pattern {
    fn to_k(&self) -> String {
        match self {
            Self::Var(name) => format!("uiPVar({})", name),
            Self::Nil => "uiPNil()".into(),
            Self::Cons(head, tail) => format!("uiPCons({}, {})", head, tail),
            Self::Leaf(name) => format!("uiPLeaf({})", name),
            Self::Branch(left, right) => format!("uiPBranch({}, {})", left, right),
        }
    }
}

impl KDisplay for Expr {
    fn to_k(&self) -> String {
        match self {
            Self::Literal(value) => format!("uiLiteral({})", value.to_k()),
            Self::Var(name) => format!("uiVar({})", name),
            Self::Let(name, value, body) => {
                format!("uiLet({}, {}, {})", name, value.to_k(), body.to_k())
            }
            Self::Binary(op, left, right) => {
                format!("uiBinary({}, {}, {})", op.to_k(), left.to_k(), right.to_k())
            }
            Self::Unary(op, expr) => format!("uiUnary({}, {})", op.to_k(), expr.to_k()),
            Self::If(condition, yes, no) => {
                format!("uiIf({}, {}, {})", condition.to_k(), yes.to_k(), no.to_k())
            }
            Self::Call(name, arguments) => {
                format!("uiCall({}, {})", name, expressions_to_k(arguments))
            }
            Self::Match(expr, arms) => format!("uiMatch({}, {})", expr.to_k(), cases_to_k(arms)),
        }
    }
}

impl KDisplay for Frame {
    fn to_k(&self) -> String {
        match self {
            Self::Let(name, body, environment) => format!(
                "uiLetF({}, {}, {})",
                name,
                body.to_k(),
                environment_to_k(environment)
            ),
            Self::Left(op, right, environment) => format!(
                "uiLeftF({}, {}, {})",
                op.to_k(),
                right.to_k(),
                environment_to_k(environment)
            ),
            Self::Right(op, left) => format!("uiRightF({}, {})", op.to_k(), left.to_k()),
            Self::Unary(op) => format!("uiUnaryF({})", op.to_k()),
            Self::If(yes, no, environment) => format!(
                "uiIfF({}, {}, {})",
                yes.to_k(),
                no.to_k(),
                environment_to_k(environment)
            ),
            Self::Match(arms, environment) => format!(
                "uiMatchF({}, {})",
                cases_to_k(arms),
                environment_to_k(environment)
            ),
            Self::Args(name, values, expressions, environment) => format!(
                "uiArgsF({}, {}, {}, {})",
                name,
                values_to_k(values),
                expressions_to_k(expressions),
                environment_to_k(environment)
            ),
            Self::Return(ty, environment) => format!(
                "uiReturnF({}, {})",
                ty.to_k(),
                environment_to_k(environment)
            ),
            Self::Scope(environment) => format!("uiScopeF({})", environment_to_k(environment)),
        }
    }
}

impl KDisplay for Control {
    fn to_k(&self) -> String {
        match self {
            Self::Eval(expr) => format!("uiEval({})", expr.to_k()),
            Self::Return(value) => format!("uiRet({})", value.to_k()),
            Self::Push(frame, control, environment) => format!(
                "uiPush({}, {}, {})",
                frame.to_k(),
                control.to_k(),
                environment_to_k(environment)
            ),
            Self::Invoke(name, values) => format!("uiInvoke({}, {})", name, values_to_k(values)),
            Self::Enter(function, values) => {
                let function = function
                    .as_ref()
                    .map_or_else(|| "uiMissingFun()".into(), KDisplay::to_k);
                format!("uiEnter({}, {})", function, values_to_k(values))
            }
            Self::Bound(bindings, ty, expr) => {
                let bindings = bindings.as_ref().map_or_else(
                    || "uiBadBinding()".into(),
                    |environment| format!("uiBindings({})", environment_to_k(environment)),
                );
                format!("uiBound({}, {}, {})", bindings, ty.to_k(), expr.to_k())
            }
            Self::Cases(value, arms, environment) => format!(
                "uiCases({}, {}, {})",
                value.to_k(),
                cases_to_k(arms),
                environment_to_k(environment)
            ),
            Self::CaseResult(bindings, value, expr, arms, environment) => {
                let bindings = bindings.as_ref().map_or_else(
                    || "uiNoMatch()".into(),
                    |environment| format!("uiMatched({})", environment_to_k(environment)),
                );
                format!(
                    "uiCaseResult({}, {}, {}, {}, {})",
                    bindings,
                    value.to_k(),
                    expr.to_k(),
                    cases_to_k(arms),
                    environment_to_k(environment)
                )
            }
            Self::PrimitiveResult(result) => {
                let primitive = match result {
                    Ok(value) => format!("uiPrimValue({})", value.to_k()),
                    Err(code) => format!("uiPrimError({})", code),
                };
                format!("uiPrimResult({})", primitive)
            }
            Self::Error(code) => format!("uiError({})", code),
        }
    }
}

impl KDisplay for Outcome {
    fn to_k(&self) -> String {
        match self {
            Self::Pending => "uiPending()".into(),
            Self::Done(value) => format!("uiDone({})", value.to_k()),
            Self::Rejected(code) => format!("uiRejected({})", code),
            Self::Exhausted => "uiExhausted()".into(),
        }
    }
}
