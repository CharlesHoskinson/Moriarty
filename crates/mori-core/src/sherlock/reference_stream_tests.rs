//! Independent ordinary expectations for borrowed streaming observations.
//! N01/N02 typed states and N05 K terms transcribe the frozen normal corpus;
//! no expected result comes from step(), run_steps(), or another execution.
//! Source preparation only: compilation/execution remain UNRUN pending preflight.

use super::normal_oracle_inputs::{normal_program, uint};
use super::reference_display::{KDisplay, environment_to_k};
use super::reference_stream::VisitStop;
use super::*;
use std::ops::ControlFlow;
use std::sync::Arc;

fn assert_profile() {
    assert_eq!(PROFILE, "sherlock-k-a/1");
    assert_eq!(
        K_SOURCE_SHA256,
        "b3746ff51dd7999b046ccb23237595605420adcfeebc0c04c0ad0f1396b11d41"
    );
}

// The caller supplies every changing field explicitly. These constructors
// assemble literal states; they do not execute a semantic transition.
fn typed_state(
    program: &Arc<Program>,
    initial_fuel: u128,
    remaining_fuel: u128,
    control: Option<Control>,
    frames: Vec<Frame>,
    outcome: Outcome,
) -> Snapshot {
    Snapshot {
        program: program.clone(),
        control,
        environment: Environment::new(),
        frames,
        stack_limit: 8,
        remaining_fuel,
        initial_fuel,
        outcome,
    }
}

fn n01_states() -> Vec<Snapshot> {
    let program = Arc::new(Program {
        functions: vec![],
        entry: uint(7),
    });
    vec![
        typed_state(
            &program,
            1,
            1,
            Some(Control::Eval(uint(7))),
            vec![],
            Outcome::Pending,
        ),
        typed_state(
            &program,
            1,
            0,
            Some(Control::Return(Value::UInt(7))),
            vec![],
            Outcome::Pending,
        ),
        typed_state(&program, 1, 0, None, vec![], Outcome::Done(Value::UInt(7))),
    ]
}

fn n02_states() -> Vec<Snapshot> {
    let program = Arc::new(Program {
        functions: vec![],
        entry: Arc::new(Expr::Binary(BinaryOp::Add, uint(2), uint(3))),
    });
    let left = Frame::Left(BinaryOp::Add, uint(3), Environment::new());
    let right = Frame::Right(BinaryOp::Add, Value::UInt(2));
    vec![
        typed_state(
            &program,
            7,
            7,
            Some(Control::Eval(program.entry.clone())),
            vec![],
            Outcome::Pending,
        ),
        typed_state(
            &program,
            7,
            6,
            Some(Control::Push(
                left.clone(),
                Box::new(Control::Eval(uint(2))),
                Environment::new(),
            )),
            vec![],
            Outcome::Pending,
        ),
        typed_state(
            &program,
            7,
            5,
            Some(Control::Eval(uint(2))),
            vec![left.clone()],
            Outcome::Pending,
        ),
        typed_state(
            &program,
            7,
            4,
            Some(Control::Return(Value::UInt(2))),
            vec![left],
            Outcome::Pending,
        ),
        typed_state(
            &program,
            7,
            3,
            Some(Control::Eval(uint(3))),
            vec![right.clone()],
            Outcome::Pending,
        ),
        typed_state(
            &program,
            7,
            2,
            Some(Control::Return(Value::UInt(3))),
            vec![right],
            Outcome::Pending,
        ),
        typed_state(
            &program,
            7,
            1,
            Some(Control::PrimitiveResult(Ok(Value::UInt(5)))),
            vec![],
            Outcome::Pending,
        ),
        typed_state(
            &program,
            7,
            0,
            Some(Control::Return(Value::UInt(5))),
            vec![],
            Outcome::Pending,
        ),
        typed_state(&program, 7, 0, None, vec![], Outcome::Done(Value::UInt(5))),
    ]
}

const N02_RULES: &[usize] = &[117, 135, 102, 149, 102, 153, 223, 80];
const N02_CHARGES: &[u8] = &[1, 1, 1, 1, 1, 1, 1, 0];

fn assert_record(
    record: &StepRecord,
    before: &Snapshot,
    after: &Snapshot,
    rule: usize,
    charge: u8,
) {
    assert_eq!(&record.before, before, "complete borrowed pre-state");
    assert_eq!(&record.after, after, "complete borrowed post-state");
    assert_eq!(record.rule_start_line, rule, "source rule order");
    assert_eq!(record.charge, charge, "charge including free publication");
}

#[test]
fn visit_n01_full_typed_records_and_terminal_precedence() {
    assert_profile();
    let expected = n01_states();
    let mut machine = Machine::new(normal_program("N01"), 1, 8);
    assert_eq!(machine.snapshot(), expected[0]);
    let mut position = 0;
    let result = machine.visit_steps(2, |record: &StepRecord| {
        assert_record(
            record,
            &expected[position],
            &expected[position + 1],
            [102, 80][position],
            [1, 0][position],
        );
        position += 1;
        if position == 2 {
            ControlFlow::Break(())
        } else {
            ControlFlow::Continue(())
        }
    });
    assert_eq!(position, 2);
    assert_eq!(result.events, 2);
    // Publication wins over both this exact budget and the observer's Break.
    assert_eq!(result.stop, VisitStop::Terminal);
    assert_eq!(machine.snapshot(), expected[2]);
}

#[test]
fn visit_n02_full_typed_records_across_budgets() {
    assert_profile();
    let expected = n02_states();
    for budget in [1, 2, 3, 8, 16] {
        let mut machine = Machine::new(normal_program("N02"), 7, 8);
        let mut position = 0;
        // Retention belongs to this observer. The API only borrows each record.
        let mut retained = Vec::new();
        assert_eq!(machine.snapshot(), expected[0]);
        loop {
            let start = position;
            let result = machine.visit_steps(budget, |record: &StepRecord| {
                assert_record(
                    record,
                    &expected[position],
                    &expected[position + 1],
                    N02_RULES[position],
                    N02_CHARGES[position],
                );
                retained.push(record.clone());
                position += 1;
                ControlFlow::Continue(())
            });
            assert_eq!(result.events, position - start);
            assert_eq!(machine.snapshot(), expected[position]);
            if position == 8 {
                assert_eq!(result.stop, VisitStop::Terminal);
                break;
            }
            assert_eq!(result.events, budget);
            assert_eq!(result.stop, VisitStop::Budget);
        }
        assert_eq!(retained.len(), 8);
        for (index, record) in retained.iter().enumerate() {
            assert_record(
                record,
                &expected[index],
                &expected[index + 1],
                N02_RULES[index],
                N02_CHARGES[index],
            );
        }
    }
}

#[test]
fn visit_zero_budget_is_inert_before_and_after_terminal() {
    let expected = n01_states();
    let mut machine = Machine::new(normal_program("N01"), 1, 8);
    let mut callbacks = 0;
    let zero = machine.visit_steps(0, |_: &StepRecord| {
        callbacks += 1;
        ControlFlow::Break(())
    });
    assert_eq!(zero.events, 0);
    assert_eq!(zero.stop, VisitStop::Budget);
    assert_eq!(callbacks, 0);
    assert_eq!(machine.snapshot(), expected[0]);

    let completed = machine.visit_steps(2, |_| ControlFlow::Continue(()));
    assert_eq!(completed.events, 2);
    assert_eq!(completed.stop, VisitStop::Terminal);
    let zero_terminal = machine.visit_steps(0, |_: &StepRecord| {
        callbacks += 1;
        ControlFlow::Break(())
    });
    assert_eq!(zero_terminal.events, 0);
    assert_eq!(zero_terminal.stop, VisitStop::Budget);
    assert_eq!(callbacks, 0);
    assert_eq!(machine.snapshot(), expected[2]);
}

#[test]
fn visit_positive_budget_on_terminal_has_no_callback() {
    let expected = n01_states();
    let mut machine = Machine::new(normal_program("N01"), 1, 8);
    let completed = machine.visit_steps(3, |_| ControlFlow::Continue(()));
    assert_eq!(completed.events, 2);
    assert_eq!(completed.stop, VisitStop::Terminal);
    for budget in [1, 8] {
        let result = machine.visit_steps(budget, |_: &StepRecord| -> ControlFlow<()> {
            panic!("terminal machine must not call the observer");
        });
        assert_eq!(result.events, 0);
        assert_eq!(result.stop, VisitStop::Terminal);
        assert_eq!(machine.snapshot(), expected[2]);
    }
}

#[test]
fn visit_observer_break_resumes_after_delivered_record() {
    let expected = n02_states();
    for cut in 1..=7 {
        // Check Observer precedence both inside and exactly at the budget.
        for budget in [cut, 8] {
            let mut machine = Machine::new(normal_program("N02"), 7, 8);
            let mut position = 0;
            let stopped = machine.visit_steps(budget, |record: &StepRecord| {
                assert_record(
                    record,
                    &expected[position],
                    &expected[position + 1],
                    N02_RULES[position],
                    N02_CHARGES[position],
                );
                position += 1;
                if position == cut {
                    ControlFlow::Break(())
                } else {
                    ControlFlow::Continue(())
                }
            });
            assert_eq!(stopped.events, cut);
            assert_eq!(stopped.stop, VisitStop::Observer);
            assert_eq!(position, cut);
            assert_eq!(machine.snapshot(), expected[cut]);
            let resumed = machine.visit_steps(8, |record: &StepRecord| {
                assert_record(
                    record,
                    &expected[position],
                    &expected[position + 1],
                    N02_RULES[position],
                    N02_CHARGES[position],
                );
                position += 1;
                ControlFlow::Continue(())
            });
            assert_eq!(resumed.events, 8 - cut);
            assert_eq!(resumed.stop, VisitStop::Terminal);
            assert_eq!(position, 8);
            assert_eq!(machine.snapshot(), expected[8]);
        }
    }
}

#[test]
fn visit_budget_cut_preserves_pending_zero_fuel_publication() {
    let expected = n02_states();
    let mut machine = Machine::new(normal_program("N02"), 7, 8);
    let mut position = 0;
    let cut = machine.visit_steps(7, |record: &StepRecord| {
        assert_record(
            record,
            &expected[position],
            &expected[position + 1],
            N02_RULES[position],
            1,
        );
        position += 1;
        ControlFlow::Continue(())
    });
    assert_eq!(cut.events, 7);
    assert_eq!(cut.stop, VisitStop::Budget);
    assert_eq!(machine.snapshot(), expected[7]);
    let zero = machine.visit_steps(0, |_: &StepRecord| -> ControlFlow<()> {
        panic!("zero budget cannot publish the pending outcome");
    });
    assert_eq!(zero.events, 0);
    assert_eq!(zero.stop, VisitStop::Budget);
    assert_eq!(machine.snapshot(), expected[7]);
    let published = machine.visit_steps(1, |record: &StepRecord| {
        assert_record(record, &expected[7], &expected[8], 80, 0);
        ControlFlow::Break(())
    });
    assert_eq!(published.events, 1);
    assert_eq!(published.stop, VisitStop::Terminal);
    assert_eq!(machine.snapshot(), expected[8]);
}

#[test]
fn visit_zero_fuel_exhaustion_is_one_free_terminal_record() {
    // The ordinary N01 fuel-zero prefix is frozen in normal-prefixes.json.
    let program = normal_program("N01");
    let before = typed_state(
        &program,
        0,
        0,
        Some(Control::Eval(uint(7))),
        vec![],
        Outcome::Pending,
    );
    let after = typed_state(&program, 0, 0, None, vec![], Outcome::Exhausted);
    let mut machine = Machine::new(program, 0, 8);
    let mut callbacks = 0;
    let result = machine.visit_steps(8, |record: &StepRecord| {
        callbacks += 1;
        assert_record(record, &before, &after, 84, 0);
        ControlFlow::Break(())
    });
    assert_eq!(callbacks, 1);
    assert_eq!(result.events, 1);
    assert_eq!(result.stop, VisitStop::Terminal);
    assert_eq!(machine.snapshot(), after);
}

// Selected complete boundary terms copied from the independent frozen N05 JSON.
struct FrozenN05State {
    charged_steps: u128,
    remaining_fuel: u128,
    control: &'static str,
    environment: &'static str,
    frames: &'static [&'static str],
}
const N05_CODE: &str = "uiFunction(uiFun(11, uiParam(0, uiBoolT(), uiNoParams()), uiUIntT(), uiIf(uiVar(0), uiCall(11, uiExpr(uiLiteral(uiB(false)), uiNoExprs())), uiLiteral(uiU(6)))), uiNoFuns())";
const N05_ENTRY: &str = "uiCall(11, uiExpr(uiLiteral(uiB(true)), uiNoExprs()))";
const N05_RULES: &[usize] = &[
    131, 135, 102, 181, 200, 204, 208, 135, 123, 135, 108, 161, 131, 135, 102, 181, 200, 204, 208,
    135, 123, 135, 108, 165, 102, 185, 185, 80,
];
const N05_CUTS: &[FrozenN05State] = &[
    FrozenN05State {
        charged_steps: 0,
        remaining_fuel: 27,
        control: "uiEval(uiCall(11, uiExpr(uiLiteral(uiB(true)), uiNoExprs())))",
        environment: ".Map",
        frames: &[],
    },
    FrozenN05State {
        charged_steps: 19,
        remaining_fuel: 8,
        control: "uiPush(uiReturnF(uiUIntT(), 0 |-> uiB(true)), uiEval(uiIf(uiVar(0), uiCall(11, uiExpr(uiLiteral(uiB(false)), uiNoExprs())), uiLiteral(uiU(6)))), 0 |-> uiB(false))",
        environment: "0 |-> uiB(true)",
        frames: &["uiReturnF(uiUIntT(), .Map)"],
    },
    FrozenN05State {
        charged_steps: 20,
        remaining_fuel: 7,
        control: "uiEval(uiIf(uiVar(0), uiCall(11, uiExpr(uiLiteral(uiB(false)), uiNoExprs())), uiLiteral(uiU(6))))",
        environment: "0 |-> uiB(false)",
        frames: &[
            "uiReturnF(uiUIntT(), 0 |-> uiB(true))",
            "uiReturnF(uiUIntT(), .Map)",
        ],
    },
    FrozenN05State {
        charged_steps: 22,
        remaining_fuel: 5,
        control: "uiEval(uiVar(0))",
        environment: "0 |-> uiB(false)",
        frames: &[
            "uiIfF(uiCall(11, uiExpr(uiLiteral(uiB(false)), uiNoExprs())), uiLiteral(uiU(6)), 0 |-> uiB(false))",
            "uiReturnF(uiUIntT(), 0 |-> uiB(true))",
            "uiReturnF(uiUIntT(), .Map)",
        ],
    },
    FrozenN05State {
        charged_steps: 25,
        remaining_fuel: 2,
        control: "uiRet(uiU(6))",
        environment: "0 |-> uiB(false)",
        frames: &[
            "uiReturnF(uiUIntT(), 0 |-> uiB(true))",
            "uiReturnF(uiUIntT(), .Map)",
        ],
    },
    FrozenN05State {
        charged_steps: 26,
        remaining_fuel: 1,
        control: "uiRet(uiU(6))",
        environment: "0 |-> uiB(true)",
        frames: &["uiReturnF(uiUIntT(), .Map)"],
    },
    FrozenN05State {
        charged_steps: 27,
        remaining_fuel: 0,
        control: "uiRet(uiU(6))",
        environment: ".Map",
        frames: &[],
    },
];

fn assert_n05_state(actual: &Snapshot, expected: &FrozenN05State, published: bool) {
    assert_eq!(actual.program.to_k(), N05_CODE, "complete frozen code");
    assert_eq!(actual.program.entry.to_k(), N05_ENTRY, "frozen entry");
    assert_eq!(
        actual
            .control
            .as_ref()
            .map(KDisplay::to_k)
            .unwrap_or_else(|| ".K".into()),
        if published { ".K" } else { expected.control },
        "complete control, including saved caller environments"
    );
    assert_eq!(environment_to_k(&actual.environment), expected.environment);
    let frames: Vec<String> = actual.frames.iter().map(KDisplay::to_k).collect();
    assert_eq!(
        frames, expected.frames,
        "every top-first frame and caller environment"
    );
    assert_eq!(actual.depth(), expected.frames.len());
    assert_eq!(actual.stack_limit, 8);
    assert_eq!(actual.initial_fuel, 27);
    assert_eq!(actual.remaining_fuel, expected.remaining_fuel);
    assert_eq!(actual.charged_steps(), expected.charged_steps);
    assert_eq!(
        actual.outcome,
        if published {
            Outcome::Done(Value::UInt(6))
        } else {
            Outcome::Pending
        }
    );
}

#[test]
fn visit_n05_recursive_cut_keeps_full_caller_environments() {
    assert_profile();
    let mut machine = Machine::new(normal_program("N05"), 27, 8);
    let mut previous = machine.snapshot();
    assert_n05_state(&previous, &N05_CUTS[0], false);
    let mut position = 0;
    for cut in &N05_CUTS[1..] {
        let budget = cut.charged_steps as usize - position;
        let result = machine.visit_steps(budget, |record: &StepRecord| {
            // Contiguity supplements the frozen expectations; it is not the oracle.
            assert_eq!(record.before, previous);
            assert_eq!(record.rule_start_line, N05_RULES[position]);
            assert_eq!(record.charge, 1);
            if let Some(expected) = N05_CUTS
                .iter()
                .find(|state| state.charged_steps == position as u128)
            {
                assert_n05_state(&record.before, expected, false);
            }
            position += 1;
            if let Some(expected) = N05_CUTS
                .iter()
                .find(|state| state.charged_steps == position as u128)
            {
                assert_n05_state(&record.after, expected, false);
            }
            previous = record.after.clone();
            ControlFlow::Continue(())
        });
        assert_eq!(result.events, budget);
        assert_eq!(result.stop, VisitStop::Budget);
        assert_n05_state(&machine.snapshot(), cut, false);
        assert_eq!(machine.snapshot(), previous);
    }
    assert_eq!(position, 27);
    let boundary = &N05_CUTS[6];
    let publication = machine.visit_steps(8, |record: &StepRecord| {
        assert_eq!(record.before, previous);
        assert_eq!(record.rule_start_line, N05_RULES[27]);
        assert_eq!(record.charge, 0);
        assert_n05_state(&record.before, boundary, false);
        assert_n05_state(&record.after, boundary, true);
        position += 1;
        ControlFlow::Break(())
    });
    assert_eq!(publication.events, 1);
    assert_eq!(publication.stop, VisitStop::Terminal);
    assert_eq!(position, 28);
    assert_n05_state(&machine.snapshot(), boundary, true);
}
