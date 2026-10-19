//! Named mutual recursion through the public typed reference API.
//! This host example is not Moriarty source compilation or a proof artifact.
use mori_core::sherlock::*;
use std::sync::Arc;

fn lit(n: u128) -> ExprRef {
    Arc::new(Expr::Literal(Value::UInt(n)))
}

fn parity(name: &str, other: &str, base: bool) -> Function {
    let variable = Arc::new(Expr::Var("0".into()));
    Function {
        name: name.into(),
        parameters: vec![Parameter {
            name: "0".into(),
            ty: Type::UInt,
        }],
        return_type: Type::Bool,
        body: Arc::new(Expr::If(
            Arc::new(Expr::Binary(BinaryOp::Eq, variable.clone(), lit(0))),
            Arc::new(Expr::Literal(Value::Bool(base))),
            Arc::new(Expr::Call(
                other.into(),
                vec![Arc::new(Expr::Binary(BinaryOp::Sub, variable, lit(1)))],
            )),
        )),
    }
}

fn main() {
    let program = Arc::new(Program {
        functions: vec![parity("10", "11", true), parity("11", "10", false)],
        entry: Arc::new(Expr::Call("10".into(), vec![lit(6)])),
    });
    let mut machine = Machine::new(program, 500, 32);
    let result = machine.run();
    assert_eq!(result.outcome, Outcome::Done(Value::Bool(true)));
    println!(
        "{}: {:?}; {} charged steps",
        PROFILE,
        result.outcome,
        result.charged_steps()
    );
}
