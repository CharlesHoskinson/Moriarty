//! SOURCE ONLY: designated-owner exact closure classification precedes execution.
//! Original UIREQ003/004/005/006 positive acceptance examples, typed host A only.
//! Expected results/work/checkpoints are handwritten from the frozen K-A rules.
//! No actual interpreter output or helper evaluator supplies expected values.

use mori_core::sherlock::{
    BinaryOp, Control, Environment, Expr, ExprRef, Frame, Function, K_SOURCE_SHA256, Machine,
    Outcome, PROFILE, Parameter, Program, Snapshot, StepRecord, Type, Value,
};
use std::sync::Arc;

const FUEL: u128 = 512;
const STACK: usize = 32;
const MAX_EVENTS: usize = 513;
const GENERIC_RULES: &[usize] = &[
    80, 102, 108, 114, 117, 123, 129, 131, 135, 145, 149, 153, 161, 165, 177, 181, 185, 195, 200,
    204, 208, 223,
];
fn u(n: u128) -> ExprRef {
    Arc::new(Expr::Literal(Value::UInt(n)))
}
fn b(value: bool) -> ExprRef {
    Arc::new(Expr::Literal(Value::Bool(value)))
}
fn var(name: &str) -> ExprRef {
    Arc::new(Expr::Var(name.into()))
}
fn binary(op: BinaryOp, left: ExprRef, right: ExprRef) -> ExprRef {
    Arc::new(Expr::Binary(op, left, right))
}
fn call(name: &str, args: Vec<ExprRef>) -> ExprRef {
    Arc::new(Expr::Call(name.into(), args))
}
fn let_in(name: &str, value: ExprRef, body: ExprRef) -> ExprRef {
    Arc::new(Expr::Let(name.into(), value, body))
}
fn function(name: &str, params: &[&str], result: Type, body: ExprRef) -> Function {
    Function {
        name: name.into(),
        parameters: params
            .iter()
            .map(|name| Parameter {
                name: (*name).into(),
                ty: Type::UInt,
            })
            .collect(),
        return_type: result,
        body,
    }
}
fn env(bindings: &[(&str, u128)]) -> Environment {
    bindings
        .iter()
        .map(|(name, value)| ((*name).into(), Value::UInt(*value)))
        .collect()
}

// This helper captures ACTUAL observations only. It never computes an expected
// value, expected charge count, call ordering or rule schedule.
struct Trace {
    program: Arc<Program>,
    initial: Snapshot,
    records: Vec<StepRecord>,
    returned: Snapshot,
}
fn observe_fixed(program: Program) -> Trace {
    let program = Arc::new(program);
    let mut machine = Machine::new(program.clone(), FUEL, STACK);
    let initial = machine.snapshot();
    let records = machine.run_steps(MAX_EVENTS);
    let returned = machine.snapshot();
    Trace {
        program,
        initial,
        records,
        returned,
    }
}
fn assert_completed(trace: &Trace, value: Value, charged: u128) {
    assert_eq!(PROFILE, "sherlock-k-a/1");
    assert_eq!(
        K_SOURCE_SHA256,
        "b3746ff51dd7999b046ccb23237595605420adcfeebc0c04c0ad0f1396b11d41"
    );
    let initial = Snapshot {
        program: trace.program.clone(),
        control: Some(Control::Eval(trace.program.entry.clone())),
        environment: Environment::new(),
        frames: vec![],
        stack_limit: STACK,
        remaining_fuel: FUEL,
        initial_fuel: FUEL,
        outcome: Outcome::Pending,
    };
    let final_state = Snapshot {
        program: trace.program.clone(),
        control: None,
        environment: Environment::new(),
        frames: vec![],
        stack_limit: STACK,
        remaining_fuel: FUEL - charged,
        initial_fuel: FUEL,
        outcome: Outcome::Done(value.clone()),
    };
    assert_eq!(
        trace.initial, initial,
        "complete independent post-startup initial state"
    );
    assert_eq!(
        trace.returned, final_state,
        "complete hand-derived final state"
    );
    let count = usize::try_from(charged).expect("small frozen literal work count");
    assert_eq!(trace.records.len(), count + 1, "one final free publication");
    for (index, record) in trace.records.iter().enumerate() {
        assert!(
            GENERIC_RULES.contains(&record.rule_start_line),
            "unchanged generic rule basis"
        );
        assert_eq!(record.charge, if index < count { 1 } else { 0 });
        assert_eq!(
            record.before.program, trace.program,
            "complete global code table is retained"
        );
        assert_eq!(record.after.program, trace.program);
        assert_eq!(record.before.stack_limit, STACK);
        assert_eq!(record.after.stack_limit, STACK);
        assert_eq!(record.before.initial_fuel, FUEL);
        assert_eq!(record.after.initial_fuel, FUEL);
        assert_eq!(
            record.before.remaining_fuel,
            FUEL - index.min(count) as u128
        );
        assert_eq!(
            record.after.remaining_fuel,
            FUEL - (index + 1).min(count) as u128
        );
        assert_eq!(record.before.outcome, Outcome::Pending);
        assert_eq!(
            record.after.outcome,
            if index < count {
                Outcome::Pending
            } else {
                Outcome::Done(value.clone())
            }
        );
    }
    let publication = trace.records.last().expect("frozen nonempty trace");
    assert_eq!(publication.rule_start_line, 80);
    assert_eq!(publication.before.control, Some(Control::Return(value)));
    assert_eq!(publication.after, final_state);
}
fn rules(trace: &Trace) -> Vec<usize> {
    trace
        .records
        .iter()
        .map(|record| record.rule_start_line)
        .collect()
}

// Selected raw typed call checkpoints, not an evaluator/model or full-trace oracle.
#[derive(Debug, PartialEq, Eq)]
enum CallEvent {
    Invoke(String, Vec<Value>),
    Returned(Environment, Value),
}
fn call_events(trace: &Trace) -> Vec<CallEvent> {
    let mut result = Vec::new();
    for record in &trace.records {
        if record.rule_start_line == 200 {
            match &record.before.control {
                Some(Control::Invoke(name, args)) => {
                    result.push(CallEvent::Invoke(name.clone(), args.clone()))
                }
                _ => panic!("generic invoke has its raw typed control"),
            }
        }
        if record.rule_start_line == 185 {
            assert!(matches!(
                record.before.frames.first(),
                Some(Frame::Return(_, _))
            ));
            match &record.before.control {
                Some(Control::Return(value)) => result.push(CallEvent::Returned(
                    record.before.environment.clone(),
                    value.clone(),
                )),
                _ => panic!("generic return has its raw typed value"),
            }
        }
    }
    result
}

#[test]
fn uireq003_callee_local_x2_preserves_caller_x7() {
    let callee_body = let_in("x", u(2), var("x"));
    let trace = observe_fixed(Program {
        functions: vec![function("callee", &[], Type::UInt, callee_body.clone())],
        entry: let_in(
            "x",
            u(7),
            let_in("ignored", call("callee", vec![]), var("x")),
        ),
    });
    assert_completed(&trace, Value::UInt(7), 22);
    assert_eq!(
        rules(&trace),
        vec![
            114, 135, 102, 145, 114, 135, 129, 200, 204, 208, 135, 114, 135, 102, 145, 108, 195,
            185, 145, 108, 195, 195, 80,
        ]
    );
    let entered = &trace.records[10];
    assert_eq!(
        entered.after.environment,
        Environment::new(),
        "callee starts with only its empty parameters"
    );
    assert_eq!(entered.after.control, Some(Control::Eval(callee_body)));
    assert_eq!(
        entered.after.frames.first(),
        Some(&Frame::Return(Type::UInt, env(&[("x", 7)])))
    );
    assert_eq!(
        call_events(&trace),
        vec![
            CallEvent::Invoke("callee".into(), vec![]),
            CallEvent::Returned(Environment::new(), Value::UInt(2)),
        ]
    );
    assert_eq!(trace.records[15].before.environment, env(&[("x", 2)]));
    assert_eq!(
        trace.records[15].after.control,
        Some(Control::Return(Value::UInt(2)))
    );
    assert_eq!(
        trace.records[17].after.environment,
        env(&[("x", 7)]),
        "call return restores caller bindings"
    );
    assert_eq!(
        trace.records[19].before.environment,
        env(&[("ignored", 2), ("x", 7)])
    );
    assert_eq!(
        trace.records[19].before.control,
        Some(Control::Eval(var("x")))
    );
    assert_eq!(
        trace.records[19].after.control,
        Some(Control::Return(Value::UInt(7)))
    );
}

#[test]
fn uireq004_double_completes_before_increment_starts_and_add_returns8() {
    let trace = observe_fixed(Program {
        functions: vec![
            function(
                "add",
                &["left", "right"],
                Type::UInt,
                binary(BinaryOp::Add, var("left"), var("right")),
            ),
            function(
                "double",
                &["n"],
                Type::UInt,
                binary(BinaryOp::Mul, var("n"), u(2)),
            ),
            function(
                "increment",
                &["n"],
                Type::UInt,
                binary(BinaryOp::Add, var("n"), u(1)),
            ),
        ],
        entry: call(
            "add",
            vec![call("double", vec![u(2)]), call("increment", vec![u(3)])],
        ),
    });
    assert_completed(&trace, Value::UInt(8), 48);
    assert_eq!(
        call_events(&trace),
        vec![
            CallEvent::Invoke("double".into(), vec![Value::UInt(2)]),
            CallEvent::Returned(env(&[("n", 2)]), Value::UInt(4)),
            CallEvent::Invoke("increment".into(), vec![Value::UInt(3)]),
            CallEvent::Returned(env(&[("n", 3)]), Value::UInt(4)),
            CallEvent::Invoke("add".into(), vec![Value::UInt(4), Value::UInt(4)]),
            CallEvent::Returned(env(&[("left", 4), ("right", 4)]), Value::UInt(8)),
        ],
        "exact ordered multiplicity: each operand runs once, from left to right"
    );
}

fn factorial_program() -> Program {
    let body = Arc::new(Expr::If(
        binary(BinaryOp::Eq, var("n"), u(0)),
        u(1),
        binary(
            BinaryOp::Mul,
            var("n"),
            call("factorial", vec![binary(BinaryOp::Sub, var("n"), u(1))]),
        ),
    ));
    Program {
        functions: vec![function("factorial", &["n"], Type::UInt, body)],
        entry: call("factorial", vec![u(3)]),
    }
}
#[test]
fn uireq005_factorial3_uses_generic_calls_and_returns6() {
    let trace = observe_fixed(factorial_program());
    assert_completed(&trace, Value::UInt(6), 113);
    assert_eq!(
        call_events(&trace),
        vec![
            CallEvent::Invoke("factorial".into(), vec![Value::UInt(3)]),
            CallEvent::Invoke("factorial".into(), vec![Value::UInt(2)]),
            CallEvent::Invoke("factorial".into(), vec![Value::UInt(1)]),
            CallEvent::Invoke("factorial".into(), vec![Value::UInt(0)]),
            CallEvent::Returned(env(&[("n", 0)]), Value::UInt(1)),
            CallEvent::Returned(env(&[("n", 1)]), Value::UInt(1)),
            CallEvent::Returned(env(&[("n", 2)]), Value::UInt(2)),
            CallEvent::Returned(env(&[("n", 3)]), Value::UInt(6)),
        ]
    );
}
fn parity_program(entry: &str, n: u128) -> Program {
    let even_body = Arc::new(Expr::If(
        binary(BinaryOp::Eq, var("n"), u(0)),
        b(true),
        call("odd", vec![binary(BinaryOp::Sub, var("n"), u(1))]),
    ));
    let odd_body = Arc::new(Expr::If(
        binary(BinaryOp::Eq, var("n"), u(0)),
        b(false),
        call("even", vec![binary(BinaryOp::Sub, var("n"), u(1))]),
    ));
    Program {
        functions: vec![
            function("even", &["n"], Type::Bool, even_body),
            function("odd", &["n"], Type::Bool, odd_body),
        ],
        entry: call(entry, vec![u(n)]),
    }
}
#[test]
fn uireq005_even4_uses_mutual_calls_and_returns_true() {
    let trace = observe_fixed(parity_program("even", 4));
    assert_completed(&trace, Value::Bool(true), 120);
    assert_eq!(
        call_events(&trace),
        vec![
            CallEvent::Invoke("even".into(), vec![Value::UInt(4)]),
            CallEvent::Invoke("odd".into(), vec![Value::UInt(3)]),
            CallEvent::Invoke("even".into(), vec![Value::UInt(2)]),
            CallEvent::Invoke("odd".into(), vec![Value::UInt(1)]),
            CallEvent::Invoke("even".into(), vec![Value::UInt(0)]),
            CallEvent::Returned(env(&[("n", 0)]), Value::Bool(true)),
            CallEvent::Returned(env(&[("n", 1)]), Value::Bool(true)),
            CallEvent::Returned(env(&[("n", 2)]), Value::Bool(true)),
            CallEvent::Returned(env(&[("n", 3)]), Value::Bool(true)),
            CallEvent::Returned(env(&[("n", 4)]), Value::Bool(true)),
        ]
    );
}
#[test]
fn uireq005_odd5_uses_mutual_calls_and_returns_true() {
    let trace = observe_fixed(parity_program("odd", 5));
    assert_completed(&trace, Value::Bool(true), 145);
    assert_eq!(
        call_events(&trace),
        vec![
            CallEvent::Invoke("odd".into(), vec![Value::UInt(5)]),
            CallEvent::Invoke("even".into(), vec![Value::UInt(4)]),
            CallEvent::Invoke("odd".into(), vec![Value::UInt(3)]),
            CallEvent::Invoke("even".into(), vec![Value::UInt(2)]),
            CallEvent::Invoke("odd".into(), vec![Value::UInt(1)]),
            CallEvent::Invoke("even".into(), vec![Value::UInt(0)]),
            CallEvent::Returned(env(&[("n", 0)]), Value::Bool(true)),
            CallEvent::Returned(env(&[("n", 1)]), Value::Bool(true)),
            CallEvent::Returned(env(&[("n", 2)]), Value::Bool(true)),
            CallEvent::Returned(env(&[("n", 3)]), Value::Bool(true)),
            CallEvent::Returned(env(&[("n", 4)]), Value::Bool(true)),
            CallEvent::Returned(env(&[("n", 5)]), Value::Bool(true)),
        ]
    );
}
fn scalar_case(op: BinaryOp, left: u128, right: u128, result: Value) {
    let trace = observe_fixed(Program {
        functions: vec![],
        entry: binary(op, u(left), u(right)),
    });
    assert_completed(&trace, result.clone(), 7);
    assert_eq!(rules(&trace), vec![117, 135, 102, 149, 102, 153, 223, 80]);
    assert_eq!(
        trace.records[5].after.control,
        Some(Control::PrimitiveResult(Ok(result.clone())))
    );
    assert_eq!(
        trace.records[6].after.control,
        Some(Control::Return(result))
    );
}
#[test]
fn uireq006_add2_3_returns5() {
    scalar_case(BinaryOp::Add, 2, 3, Value::UInt(5));
}
#[test]
fn uireq006_sub7_2_returns5() {
    scalar_case(BinaryOp::Sub, 7, 2, Value::UInt(5));
}
#[test]
fn uireq006_mul3_4_returns12() {
    scalar_case(BinaryOp::Mul, 3, 4, Value::UInt(12));
}
#[test]
fn uireq006_lt2_3_returns_true() {
    scalar_case(BinaryOp::Lt, 2, 3, Value::Bool(true));
}
