//! Hand-counted primitive oracles through the real interpreter and replay APIs.
use mori_core::*;
use serde_json::json;

const COUNTER: &str = r#"language "moriarty/1"; agreement Counter {
state count: UInt = 0;
action run(amount: UInt) {
let total = count + amount;
requires total <= 100;
next count = total;
ensures count == total;
emit "count"(count);
} }"#;
const PAYMENT: &str = r#"language "moriarty/1"; agreement Payment {
asset A = {network: "local", issuer: "i", reference: "A", scale: 0};
action run(amount: UInt) {
transfer(A, "alice", "bob", amount);
fee(A, "alice", "treasury", 2);
} }"#;
fn counter() -> (Program, Scenario) {
    (
        parse(COUNTER).unwrap(),
        Scenario {
            inputs: [("amount".into(), Value::UInt(UInt(7)))].into(),
            ..Scenario::default()
        },
    )
}
fn payment() -> (Program, Scenario) {
    let p = parse(PAYMENT).unwrap();
    let s = Scenario {
        inputs: [("amount".into(), Value::UInt(UInt(7)))].into(),
        balances: [("alice", 20), ("bob", 0), ("treasury", 0)]
            .into_iter()
            .map(|(account, n)| Balance {
                account: account.into(),
                asset: p.assets[0].identity.clone(),
                amount: UInt(n),
            })
            .collect(),
        ..Scenario::default()
    };
    (p, s)
}
fn report(cost: &CostReport, cap: u128, used: u128) {
    assert_eq!(cost.profile, "moriarty-reference-logical-cost/1");
    assert_eq!(cost.cap, UInt(cap));
    assert_eq!(cost.used, UInt(used));
    assert_eq!(
        serde_json::to_value(cost).unwrap(),
        json!({
            "profile": "moriarty-reference-logical-cost/1", "cap": cap.to_string(), "used": used.to_string()
        })
    );
}
fn meter_error(error: &Error, step: u16) {
    assert_eq!(error.code, "mori::core::meter");
    assert_eq!(error.message, "cost plane cap exceeded");
    assert_eq!(error.step, Some(step));
}
#[test]
fn counter_exact_cap_and_atomic_refusals_use_independent_cost_16() {
    let (p, s) = counter();
    let before = s.clone();
    let ordinary = simulate(&p, "run", &s).unwrap();
    for cap in [16, u128::MAX] {
        let metered = simulate_metered(&p, "run", &s, UInt(cap)).unwrap();
        report(&metered.cost, cap, 16); // 4 + 4 + 2 + 4 + 2
        assert_eq!(metered.execution, ordinary);
        assert_eq!(
            serde_json::to_vec(&metered.execution).unwrap(),
            serde_json::to_vec(&ordinary).unwrap()
        );
    }
    for (cap, step) in [(0, 0), (15, 4)] {
        let refused = simulate_metered(&p, "run", &s, UInt(cap)).unwrap_err();
        report(&refused.cost, cap, cap);
        meter_error(&refused.error, step);
        let json = serde_json::to_value(&refused).unwrap();
        assert!(json.get("execution").is_none());
        assert!(json.get("effects").is_none());
        assert!(json.get("post").is_none());
        assert_eq!(s, before);
    }
    // Rejected/previous calls do not share budget or cached values.
    report(
        &simulate_metered(&p, "run", &s, UInt(16)).unwrap().cost,
        16,
        16,
    );
}
#[test]
fn payment_cost_4_retains_complete_effects_and_late_failure_rolls_back() {
    let (p, s) = payment();
    let before = s.clone();
    let ordinary = simulate(&p, "run", &s).unwrap();
    let metered = simulate_metered(&p, "run", &s, UInt(4)).unwrap();
    report(&metered.cost, 4, 4); // transfer=2, fee=2 (fee amount is not cost)
    assert_eq!(metered.execution, ordinary);
    assert!(matches!(
        ordinary.effects[0],
        Effect::Transfer {
            amount: UInt(7),
            ..
        }
    ));
    assert!(matches!(
        ordinary.effects[1],
        Effect::Fee {
            amount: UInt(2),
            ..
        }
    ));
    assert_eq!(
        ordinary
            .post
            .balances
            .iter()
            .map(|b| b.amount)
            .collect::<Vec<_>>(),
        vec![UInt(11), UInt(7), UInt(2)]
    );
    let refusal = simulate_metered(&p, "run", &s, UInt(3)).unwrap_err();
    report(&refusal.cost, 3, 3);
    meter_error(&refusal.error, 1);
    assert_eq!(s, before);
    assert!(
        serde_json::to_value(refusal)
            .unwrap()
            .get("execution")
            .is_none()
    );
}
#[test]
fn duty_reads_capture_current_state_and_cost_16_with_zero_tombstone() {
    let p = parse(
        r#"language "moriarty/1"; agreement Duties {
asset A = {network: "local", issuer: "i", reference: "A", scale: 0};
action run(amount: UInt) {
liability("d", A, "alice", "bob", amount);
let opening = liability_amount("d");
requires liability_present("d") and opening == amount;
discharge("d", amount);
ensures liability_amount("d") == 0;
} }"#,
    )
    .unwrap();
    let (_, s) = counter();
    let metered = simulate_metered(&p, "run", &s, UInt(16)).unwrap();
    report(&metered.cost, 16, 16); // 2 + 2 + 6 + 2 + 4
    assert_eq!(metered.execution, simulate(&p, "run", &s).unwrap());
    assert_eq!(metered.execution.post.liabilities.len(), 1);
    assert_eq!(metered.execution.post.liabilities[0].amount, UInt(0));
    assert_eq!(metered.execution.effects.len(), 2);
}
fn dag(distinct: bool, twice: bool) -> Program {
    let mut expressions = vec![Expr::Literal {
        value: Value::UInt(UInt(1)),
    }];
    if distinct {
        expressions.push(Expr::Literal {
            value: Value::UInt(UInt(1)),
        });
    }
    let root = expressions.len() as ExprId;
    expressions.push(Expr::Binary {
        op: BinaryOp::Add,
        lhs: 0,
        rhs: if distinct { 1 } else { 0 },
    });
    let mut statements = vec![Statement::Let {
        name: "first".into(),
        value: root,
    }];
    if twice {
        statements.push(Statement::Let {
            name: "second".into(),
            value: root,
        });
    }
    Program {
        language: LANGUAGE.into(),
        semantics: SEMANTICS.into(),
        name: "Dag".into(),
        states: vec![],
        assets: vec![],
        duties: vec![],
        expressions,
        actions: vec![Action {
            name: "run".into(),
            inputs: vec![],
            statements,
        }],
    }
}
#[test]
fn manual_ir_cache_hits_are_free_distinct_ids_charge_and_cache_resets() {
    for (distinct, twice, cost) in [(false, false, 3), (true, false, 4), (false, true, 6)] {
        let p = dag(distinct, twice);
        validate(&p).unwrap();
        let metered = simulate_metered(&p, "run", &Scenario::default(), UInt(cost)).unwrap();
        report(&metered.cost, cost, cost);
        assert_eq!(metered.execution.steps[0].value, Some(Value::UInt(UInt(2))));
        let refused =
            simulate_metered(&p, "run", &Scenario::default(), UInt(cost - 1)).unwrap_err();
        report(&refused.cost, cost - 1, cost - 1);
        meter_error(&refused.error, if twice { 1 } else { 0 });
    }
}
#[test]
fn eager_left_to_right_attempts_keep_failure_and_meter_precedence() {
    for (predicate, cap, used, code) in [
        ("false", 2, 2, "requires"),
        ("false and (0 - 1 == 0)", 7, 7, "underflow"),
        ("false and (0 - 1 == 0)", 6, 6, "meter"),
        ("not true", 3, 3, "requires"),
        ("true or (0 - 1 == 0)", 7, 7, "underflow"),
    ] {
        let p = parse(&format!("language \"moriarty/1\"; agreement Eager {{ action run() {{ requires {predicate}; }} }}")).unwrap();
        let refused = simulate_metered(&p, "run", &Scenario::default(), UInt(cap)).unwrap_err();
        report(&refused.cost, cap, used);
        assert_eq!(refused.error.code, format!("mori::core::{code}"));
        assert_eq!(refused.error.step, Some(0));
        if code == "meter" {
            meter_error(&refused.error, 0);
        } else {
            assert_eq!(
                refused.error,
                simulate(&p, "run", &Scenario::default()).unwrap_err()
            );
        }
    }
    // The left subtraction fails before the right addition can overflow.
    let p = parse("language \"moriarty/1\"; agreement Order { action run() { let x = (0 - 1) + (340282366920938463463374607431768211455 + 1); } }").unwrap();
    let err = simulate_metered(&p, "run", &Scenario::default(), UInt(5)).unwrap_err();
    report(&err.cost, 5, 5);
    assert_eq!(err.error.code, "mori::core::underflow");
}
#[test]
fn structural_action_and_input_refusals_keep_original_errors_at_zero_cost() {
    let (p, s) = counter();
    let mut invalid = p.clone();
    invalid.expressions.push(Expr::Not { value: u16::MAX });
    let mut wrong = s.clone();
    wrong.inputs.insert("amount".into(), Value::Bool(true));
    let mut extra = s.clone();
    extra.inputs.insert("extra".into(), Value::UInt(UInt(0)));
    for (program, action, scenario) in [
        (&invalid, "run", &s),
        (&p, "missing", &s),
        (&p, "run", &wrong),
        (&p, "run", &extra),
        (&p, "run", &Scenario::default()),
    ] {
        let ordinary = simulate(program, action, scenario).unwrap_err();
        let refused = simulate_metered(program, action, scenario, UInt(0)).unwrap_err();
        report(&refused.cost, 0, 0);
        assert_eq!(refused.error, ordinary);
        assert_eq!(refused.error.step, None);
    }
    let (p, s) = payment();
    let mut duplicate = s.clone();
    duplicate.balances.push(s.balances[0].clone());
    let mut missing = s.clone();
    missing.balances.pop();
    let mut undeclared = s.clone();
    undeclared.balances[0].asset.reference = "other".into();
    for scenario in [duplicate, missing, undeclared] {
        let refused = simulate_metered(&p, "run", &scenario, UInt(0)).unwrap_err();
        report(&refused.cost, 0, 0);
        assert_eq!(refused.error, simulate(&p, "run", &scenario).unwrap_err());
    }
}
#[test]
fn detailed_replay_enforces_trusted_cap_before_comparison_and_preserves_first_field() {
    let (p, s) = counter();
    // Independent complete record: values/snapshots/effects enumerated, no simulate-generated oracle.
    let pre = Snapshot {
        state: [("count".into(), Value::UInt(UInt(0)))].into(),
        balances: vec![],
        liabilities: vec![],
    };
    let post = Snapshot {
        state: [("count".into(), Value::UInt(UInt(7)))].into(),
        ..pre.clone()
    };
    let state = Effect::State {
        name: "count".into(),
        before: Value::UInt(UInt(0)),
        after: Value::UInt(UInt(7)),
    };
    let emit = Effect::Emit {
        name: "count".into(),
        value: Value::UInt(UInt(7)),
    };
    let candidate = Execution {
        semantics: SEMANTICS.into(),
        action: "run".into(),
        pre: pre.clone(),
        post: post.clone(),
        steps: p.actions[0]
            .statements
            .iter()
            .enumerate()
            .map(|(i, statement)| Step {
                index: i as u16,
                statement: statement.clone(),
                value: Some(if i == 1 || i == 3 {
                    Value::Bool(true)
                } else {
                    Value::UInt(UInt(7))
                }),
                snapshot: if i < 2 { pre.clone() } else { post.clone() },
                effects: match i {
                    2 => vec![state.clone()],
                    4 => vec![emit.clone()],
                    _ => vec![],
                },
            })
            .collect(),
        effects: vec![state, emit],
    };
    report(
        &replay_detailed_metered(&p, "run", &s, &candidate, UInt(16)).unwrap(),
        16,
        16,
    );
    replay_detailed(&p, "run", &s, &candidate).unwrap();
    let mut divergent = candidate.clone();
    divergent.steps[0].value = None;
    divergent
        .post
        .state
        .insert("count".into(), Value::UInt(UInt(99)));
    let refused = replay_detailed_metered(&p, "run", &s, &divergent, UInt(15)).unwrap_err();
    report(&refused.cost, 15, 15);
    let DetailedReplayError::Execution(error) = refused.error else {
        panic!("cap must precede comparison")
    };
    meter_error(&error, 4);
    let mismatch = replay_detailed_metered(&p, "run", &s, &divergent, UInt(16)).unwrap_err();
    report(&mismatch.cost, 16, 16);
    assert_eq!(
        mismatch.error,
        DetailedReplayError::Divergence(ReplayDifference {
            step: Some(0),
            field: "/steps/0/value".into()
        })
    );
    assert_eq!(
        mismatch.error,
        replay_detailed(&p, "run", &s, &divergent).unwrap_err()
    );
    // A scalar payload change has the existing nested RFC6901 path. Preserve
    // that diagnostic too, rather than flattening the comparator's semantics.
    divergent.steps[0].value = Some(Value::UInt(UInt(8)));
    let nested = replay_detailed_metered(&p, "run", &s, &divergent, UInt(16)).unwrap_err();
    report(&nested.cost, 16, 16);
    assert_eq!(
        nested.error,
        DetailedReplayError::Divergence(ReplayDifference {
            step: Some(0),
            field: "/steps/0/value/value".into()
        })
    );
    assert_eq!(
        nested.error,
        replay_detailed(&p, "run", &s, &divergent).unwrap_err()
    );
    // Cost authority lives outside Execution's unchanged exact schema.
    let mut forged = serde_json::to_value(candidate).unwrap();
    forged["cost"] = json!({"cap": "999", "used": "0"});
    assert!(serde_json::from_value::<Execution>(forged).is_err());
}
