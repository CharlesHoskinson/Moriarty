use mori_core::*;

const COUNTER: &str = r#"language "moriarty/1";
agreement Counter {
 state count: UInt = 0;
 action increment(amount: UInt) {
   let total = count + amount;
   requires total <= 100;
   next count = total;
   ensures count == total;
   emit "count"(count);
 }
 action double() { next count = count * 2; }
}"#;
fn uint(n: u128) -> Value {
    Value::UInt(UInt(n))
}
#[test]
fn authored_counter_selects_actions_and_preserves_dependencies() {
    let p = parse(COUNTER).unwrap();
    let mut s = Scenario::default();
    s.inputs.insert("amount".into(), uint(7));
    let run = simulate(&p, "increment", &s).unwrap();
    assert_eq!(run.post.state["count"], uint(7));
    assert_eq!(run.steps.len(), 5);
    replay(&p, "increment", &s, &run).unwrap();
    s.state = run.post.state;
    s.inputs.clear();
    assert_eq!(
        simulate(&p, "double", &s).unwrap().post.state["count"],
        uint(14)
    );
}
#[test]
fn failed_predicate_leaves_original_scenario_unchanged() {
    let p = parse(COUNTER).unwrap();
    let mut s = Scenario::default();
    s.inputs.insert("amount".into(), uint(101));
    let original = s.clone();
    assert_eq!(
        simulate(&p, "increment", &s).unwrap_err().code,
        "mori::core::requires"
    );
    assert_eq!(s, original);
}

const PAYMENT: &str = r#"language "moriarty/1";
agreement ConditionalPayment {
 asset USD = { network: "preview", issuer: "issuer", reference: "usd", scale: 2 };
 state paid: UInt = 0;
 action settle(amount: UInt, approved: Bool) {
   requires approved and amount > 0;
   let charge = amount + 2;
   transfer(USD, "alice", "bob", amount);
   fee(USD, "alice", "treasury", 2);
   liability("delivery", USD, "bob", "alice", amount);
   next paid = paid + charge;
   ensures paid >= charge;
   emit "settled"(paid);
 }
 action finish(amount: UInt) { discharge("delivery", amount); }
}"#;
fn payment() -> (Program, Scenario) {
    let p = parse(PAYMENT).unwrap();
    let asset = p.assets[0].identity.clone();
    let mut s = Scenario::default();
    s.inputs.insert("amount".into(), uint(20));
    s.inputs.insert("approved".into(), Value::Bool(true));
    s.balances = vec![
        Balance {
            account: "alice".into(),
            asset: asset.clone(),
            amount: UInt(100),
        },
        Balance {
            account: "bob".into(),
            asset: asset.clone(),
            amount: UInt(0),
        },
        Balance {
            account: "treasury".into(),
            asset,
            amount: UInt(0),
        },
    ];
    (p, s)
}
#[test]
fn authored_payment_has_complete_ordered_effects_and_persistent_liability() {
    let (p, s) = payment();
    let run = simulate(&p, "settle", &s).unwrap();
    assert_eq!(run.post.state["paid"], uint(22));
    assert_eq!(run.effects.len(), 5);
    assert_eq!(
        run.post
            .balances
            .iter()
            .find(|b| b.account == "alice")
            .unwrap()
            .amount,
        UInt(78)
    );
    assert_eq!(
        run.post.balances.iter().map(|b| b.amount.0).sum::<u128>(),
        100
    );
    assert_eq!(run.post.liabilities[0].amount, UInt(20));
    let mut next = Scenario {
        state: run.post.state,
        balances: run.post.balances,
        liabilities: run.post.liabilities,
        ..Scenario::default()
    };
    next.inputs.insert("amount".into(), uint(7));
    assert_eq!(
        simulate(&p, "finish", &next).unwrap().post.liabilities[0].amount,
        UInt(13)
    );
}
#[test]
fn late_fee_failure_rolls_back_every_effect() {
    let (p, mut s) = payment();
    s.balances[0].amount = UInt(21);
    let before = s.clone();
    let e = simulate(&p, "settle", &s).unwrap_err();
    assert_eq!(e.code, "mori::core::balance");
    assert_eq!(e.step, Some(3));
    assert_eq!(s, before);
}
#[test]
fn replay_identifies_first_step_divergence_and_missing_effects() {
    let (p, s) = payment();
    let run = simulate(&p, "settle", &s).unwrap();
    let mut wrong = run.clone();
    wrong.steps[2].effects.clear();
    assert_eq!(replay(&p, "settle", &s, &wrong).unwrap_err().step, Some(2));
    let mut wrong = run;
    wrong.effects.pop();
    assert_eq!(
        replay(&p, "settle", &s, &wrong).unwrap_err().code,
        "mori::core::replay"
    );
}
#[test]
fn exact_input_and_balance_schemas_are_required() {
    let (p, mut s) = payment();
    s.inputs.remove("amount");
    assert_eq!(
        simulate(&p, "settle", &s).unwrap_err().code,
        "mori::core::input"
    );
    let (_, mut s) = payment();
    s.inputs.insert("unexpected".into(), uint(1));
    assert_eq!(
        simulate(&p, "settle", &s).unwrap_err().code,
        "mori::core::input"
    );
    let (_, mut s) = payment();
    s.inputs.insert("amount".into(), Value::Bool(true));
    assert_eq!(
        simulate(&p, "settle", &s).unwrap_err().code,
        "mori::core::input"
    );
    let (_, mut s) = payment();
    s.balances.pop();
    assert_eq!(
        simulate(&p, "settle", &s).unwrap_err().code,
        "mori::core::balance"
    );
    let (_, mut s) = payment();
    s.balances[1].asset.issuer = "other".into();
    assert_eq!(
        simulate(&p, "settle", &s).unwrap_err().code,
        "mori::core::asset"
    );
}
#[test]
fn checked_arithmetic_and_invalid_ir_refuse() {
    let p = parse(COUNTER).unwrap();
    let mut s = Scenario::default();
    s.inputs.insert("amount".into(), uint(u128::MAX));
    s.state.insert("count".into(), uint(1));
    assert_eq!(
        simulate(&p, "increment", &s).unwrap_err().code,
        "mori::core::overflow"
    );
    let mut p = p;
    p.expressions[0] = Expr::Not { value: 0 };
    assert_eq!(validate(&p).unwrap_err().code, "mori::core::expression");
    let bad = r#"language "moriarty/1"; agreement Bad { state x: UInt = 0; action run(){ next x = x - 1; } }"#;
    assert_eq!(
        simulate(&parse(bad).unwrap(), "run", &Scenario::default())
            .unwrap_err()
            .code,
        "mori::core::underflow"
    );
}
#[test]
fn unsupported_and_ill_typed_programs_refuse() {
    assert_eq!(
        parse(&COUNTER.replace(
            "let total = count + amount;",
            "let total = missing + amount;"
        ))
        .unwrap_err()
        .code,
        "mori::core::name"
    );
    assert_eq!(
        parse(&COUNTER.replace("requires total <= 100;", "requires total;"))
            .unwrap_err()
            .code,
        "mori::core::type"
    );
    assert_eq!(
        parse(&COUNTER.replace("next count = total;", "while true { }"))
            .unwrap_err()
            .code,
        "mori::core::unsupported"
    );
    assert_eq!(
        parse(&PAYMENT.replace("scale: 2", "scale: 39"))
            .unwrap_err()
            .code,
        "mori::core::asset"
    );
}
#[test]
fn quantities_are_canonical_decimal_strings_even_above_javascript_range() {
    let v = uint(u128::MAX);
    let encoded = serde_json::to_string(&v).unwrap();
    assert!(encoded.contains(&format!("\"{}\"", u128::MAX)));
    assert_eq!(serde_json::from_str::<Value>(&encoded).unwrap(), v);
    for text in ["1", "\"01\"", "\"-1\"", "\"1.0\"", "\"+1\""] {
        assert!(serde_json::from_str::<UInt>(text).is_err());
    }
    assert!(
        serde_json::from_str::<Scenario>(
            r#"{"state":{},"inputs":{},"balances":[],"liabilities":[],"ignored":true}"#
        )
        .is_err()
    );
}

#[test]
fn every_asset_has_independent_conservation_and_recipient_overflow_refuses() {
    let (p, mut s) = payment();
    s.balances[1].amount = UInt(u128::MAX);
    let original = s.clone();
    assert_eq!(
        simulate(&p, "settle", &s).unwrap_err().code,
        "mori::core::overflow"
    );
    assert_eq!(s, original);
    let extended=PAYMENT.replace("state paid:","asset EUR = { network: \"preview\", issuer: \"issuer\", reference: \"eur\", scale: 2 }; state paid:");
    let p = parse(&extended).unwrap();
    let eur = p.assets[1].identity.clone();
    let (_, mut s) = payment();
    s.balances.push(Balance {
        account: "alice".into(),
        asset: eur.clone(),
        amount: UInt(50),
    });
    s.balances.push(Balance {
        account: "bob".into(),
        asset: eur,
        amount: UInt(9),
    });
    let run = simulate(&p, "settle", &s).unwrap();
    assert_eq!(
        run.post
            .balances
            .iter()
            .filter(|b| b.asset.reference == "eur")
            .map(|b| b.amount.0)
            .sum::<u128>(),
        59
    );
    let extended = extended.replace(
        "reference: \"eur\", scale: 2",
        "reference: \"usd\", scale: 3",
    );
    assert_eq!(parse(&extended).unwrap_err().code, "mori::core::asset");
}
#[test]
fn postcondition_failure_is_atomic_and_initial_state_is_exact() {
    let p = parse(&COUNTER.replace("ensures count == total;", "ensures count > total;")).unwrap();
    let mut s = Scenario::default();
    s.inputs.insert("amount".into(), uint(7));
    let original = s.clone();
    let e = simulate(&p, "increment", &s).unwrap_err();
    assert_eq!(e.code, "mori::core::ensures");
    assert_eq!(e.step, Some(3));
    assert_eq!(s, original);
    s.state.insert("surplus".into(), uint(1));
    assert_eq!(
        simulate(&p, "increment", &s).unwrap_err().code,
        "mori::core::state"
    );
    s.state.clear();
    s.state.insert("count".into(), Value::Bool(false));
    assert_eq!(
        simulate(&p, "increment", &s).unwrap_err().code,
        "mori::core::state"
    );
    assert_eq!(
        simulate(&p, "missing", &s).unwrap_err().code,
        "mori::core::action"
    );
}
#[test]
fn liability_binding_and_partial_discharge_cannot_be_rewritten() {
    let (p, mut s) = payment();
    let asset = p.assets[0].identity.clone();
    s.liabilities.push(Liability {
        name: "delivery".into(),
        asset,
        debtor: "bob".into(),
        creditor: "mallory".into(),
        amount: UInt(5),
    });
    let original = s.clone();
    assert_eq!(
        simulate(&p, "settle", &s).unwrap_err().code,
        "mori::core::liability"
    );
    assert_eq!(s, original);
    s.inputs.remove("approved");
    s.inputs.insert("amount".into(), uint(6));
    assert_eq!(
        simulate(&p, "finish", &s).unwrap_err().code,
        "mori::core::liability"
    );
    s.inputs.insert("amount".into(), uint(5));
    s.liabilities[0].creditor = "alice".into();
    let run = simulate(&p, "finish", &s).unwrap();
    assert_eq!(run.post.liabilities[0].amount, UInt(0));
}
#[test]
fn deterministic_order_is_independent_of_balance_serialization() {
    let (p, s) = payment();
    let one = simulate(&p, "settle", &s).unwrap();
    let mut reversed = s;
    reversed.balances.reverse();
    assert_eq!(simulate(&p, "settle", &reversed).unwrap(), one);
    reversed.balances.push(reversed.balances[0].clone());
    assert_eq!(
        simulate(&p, "settle", &reversed).unwrap_err().code,
        "mori::core::balance"
    );
}
#[test]
fn shared_expression_dag_is_linear_and_bounded() {
    let mut p=parse(r#"language "moriarty/1"; agreement Shared { state value: UInt = 0; action run(){ next value = 0; } }"#).unwrap();
    p.expressions = vec![Expr::Literal { value: uint(0) }];
    for i in 1..64 {
        p.expressions.push(Expr::Binary {
            op: BinaryOp::Add,
            lhs: i - 1,
            rhs: i - 1,
        });
    }
    p.actions[0].statements = vec![Statement::Next {
        state: "value".into(),
        value: 63,
    }];
    assert_eq!(
        simulate(&p, "run", &Scenario::default())
            .unwrap()
            .post
            .state["value"],
        uint(0)
    );
    p.expressions.push(Expr::Binary {
        op: BinaryOp::Add,
        lhs: 63,
        rhs: 63,
    });
    p.expressions.push(Expr::Not { value: 64 });
    assert_eq!(validate(&p).unwrap_err().code, "mori::core::bound");
}
#[test]
fn malformed_identifiers_refuse() {
    let mut p = parse(COUNTER).unwrap();
    p.name = "path/../name".into();
    assert_eq!(validate(&p).unwrap_err().code, "mori::core::name");
}
#[test]
fn excessive_trace_refuses() {
    let (p, mut s) = payment();
    let asset = p.assets[0].identity.clone();
    for i in 0..4090 {
        s.balances.push(Balance {
            account: format!("account{i}"),
            asset: asset.clone(),
            amount: UInt(0),
        });
    }
    let mut p = p;
    let stmt = p.actions[0].statements.last().unwrap().clone();
    p.actions[0]
        .statements
        .extend(std::iter::repeat_n(stmt, 20));
    assert_eq!(
        simulate(&p, "settle", &s)
            .expect_err("trace budget must refuse")
            .code,
        "mori::core::bound"
    );
}
proptest::proptest! {
 #[test]
 fn arbitrary_source_never_panics(source in ".{0,2048}") { let _=parse(&source); }
}

#[test]
fn quoted_operators_cannot_become_syntax() {
    let source = r#"language "moriarty/1"; agreement Quoted { state x: UInt = 0; action run() { next x = 1 "+" 2; } }"#;
    assert!(
        parse(source).is_err(),
        "quoted plus must not become an operator"
    );
    let source =
        r#"language "moriarty/1"; agreement Quoted { action run() { requires 1 = "=" 1; } }"#;
    assert!(parse(source).is_err(), "quoted tail cannot form equality");
}
