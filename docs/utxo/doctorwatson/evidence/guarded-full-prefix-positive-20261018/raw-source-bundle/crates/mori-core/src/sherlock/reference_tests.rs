use super::*;
use std::sync::Arc;

#[test]
fn literal_publishes_at_exact_zero_remaining_fuel() {
    let program = Arc::new(Program {
        functions: vec![],
        entry: Arc::new(Expr::Literal(Value::UInt(7))),
    });
    let mut machine = Machine::new(program, 1, 8);
    let evaluated = machine.step().expect("literal step");
    assert_eq!(evaluated.rule_start_line, 102);
    assert_eq!(evaluated.charge, 1);
    assert_eq!(evaluated.after.remaining_fuel, 0);
    assert_eq!(
        evaluated.after.control,
        Some(Control::Return(Value::UInt(7)))
    );
    let published = machine.step().expect("free publication");
    assert_eq!(published.rule_start_line, 80);
    assert_eq!(published.charge, 0);
    assert_eq!(published.after.outcome, Outcome::Done(Value::UInt(7)));
    assert!(machine.step().is_none());
}
