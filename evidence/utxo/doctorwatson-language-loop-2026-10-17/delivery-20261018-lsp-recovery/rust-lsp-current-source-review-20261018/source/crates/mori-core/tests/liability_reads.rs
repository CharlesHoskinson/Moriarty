use mori_core::*;
const HEADER: &str = r#"language "moriarty/1"; agreement Reads {
asset A = { network: "local", issuer: "i", reference: "A", scale: 0 };
state x: UInt = 0;
action declare() { liability("d", A, "debtor", "creditor", 0); }
"#;
fn program(body: &str) -> Program {
    parse(&format!("{HEADER} action read() {{ {body} }} }}")).unwrap()
}
#[test]
fn missing_and_present_zero_reads_are_distinct() {
    let p = program(r#"requires not liability_present("d"); requires liability_amount("d") == 0;"#);
    let absent = simulate(&p, "read", &Scenario::default()).unwrap();
    assert!(absent.effects.is_empty());
    let zero = simulate(&p, "declare", &Scenario::default()).unwrap();
    let s = Scenario {
        liabilities: zero.post.liabilities,
        ..Scenario::default()
    };
    let p = program(
        r#"requires liability_present("d"); requires liability_amount("d") == 0; discharge("d", 0); ensures liability_present("d");"#,
    );
    simulate(&p, "read", &s).unwrap();
}
#[test]
fn reads_use_ordered_snapshot_and_let_captures_it() {
    let p = program(
        r#"requires liability_amount("d") == 0;
        liability("d", A, "debtor", "creditor", 7);
        let saved = liability_amount("d"); discharge("d", 3);
        next x = liability_amount("d");
        ensures saved == 7 and x == 4 and liability_amount("d") == 4;
        discharge("d", liability_amount("d"));
        ensures liability_present("d") and liability_amount("d") == 0;"#,
    );
    let e = simulate(&p, "read", &Scenario::default()).unwrap();
    assert_eq!(e.post.state["x"], Value::UInt(UInt(4)));
    replay(&p, "read", &Scenario::default(), &e).unwrap();
}
#[test]
fn literal_identity_and_type_diagnostics_are_stable() {
    for expression in [
        "liability_amount()",
        "liability_amount(d)",
        "liability_amount(7)",
        "liability_present(\"d\", \"d\")",
    ] {
        let source = format!("{HEADER} action read() {{ let value = {expression}; }} }}");
        assert_eq!(parse(&source).unwrap_err().code, "mori::core::syntax");
    }
    for name in ["", "unknown"] {
        let source =
            format!("{HEADER} action read() {{ let value = liability_amount(\"{name}\"); }} }}");
        assert_eq!(parse(&source).unwrap_err().code, "mori::core::liability");
    }
    let source = format!("{HEADER} action read() {{ requires liability_amount(\"d\"); }} }}");
    assert_eq!(parse(&source).unwrap_err().code, "mori::core::type");
    let source = format!("{HEADER} action read() {{ next x = liability_present(\"d\"); }} }}");
    assert_eq!(parse(&source).unwrap_err().code, "mori::core::type");
    let source = format!(
        "{HEADER} action conflict() {{ liability(\"d\", A, \"other\", \"creditor\", 0); }} action read() {{ requires liability_present(\"d\"); }} }}"
    );
    assert_eq!(parse(&source).unwrap_err().code, "mori::core::liability");
}
#[test]
fn intrinsic_spellings_still_work_as_ordinary_names() {
    let p = program(
        "let liability_amount = 7; let liability_present = true; next x = liability_amount; requires liability_present;",
    );
    assert_eq!(
        simulate(&p, "read", &Scenario::default())
            .unwrap()
            .post
            .state["x"],
        Value::UInt(UInt(7))
    );
}
#[test]
fn exact_amounts_and_shared_expression_handles_refresh_each_statement() {
    let mut p = program(
        r#"let old = liability_amount("d"); discharge("d", 0); next x = liability_amount("d");"#,
    );
    let shared = match p.actions[1].statements[0] {
        Statement::Let { value, .. } => value,
        _ => unreachable!(),
    };
    if let Statement::Next { value, .. } = &mut p.actions[1].statements[2] {
        *value = shared;
    }
    for n in [0, 1, 7, u128::MAX] {
        let s = Scenario {
            liabilities: vec![Liability {
                name: "d".into(),
                asset: p.assets[0].identity.clone(),
                debtor: "debtor".into(),
                creditor: "creditor".into(),
                amount: UInt(n),
            }],
            ..Scenario::default()
        };
        let e = simulate(&p, "read", &s).unwrap();
        assert_eq!(e.post.state["x"], Value::UInt(UInt(n)));
    }
    if let Statement::Discharge { amount, .. } = &mut p.actions[1].statements[1] {
        p.expressions.push(Expr::Literal {
            value: Value::UInt(UInt(3)),
        });
        *amount = (p.expressions.len() - 1) as ExprId;
    }
    let s = Scenario {
        liabilities: vec![Liability {
            name: "d".into(),
            asset: p.assets[0].identity.clone(),
            debtor: "debtor".into(),
            creditor: "creditor".into(),
            amount: UInt(7),
        }],
        ..Scenario::default()
    };
    assert_eq!(
        simulate(&p, "read", &s).unwrap().post.state["x"],
        Value::UInt(UInt(4))
    );
}

#[test]
fn unused_and_deserialized_read_leaves_require_declared_bounded_identity() {
    for name in ["unknown".to_string(), "".to_string(), "x".repeat(1025)] {
        let mut p = program("requires true;");
        p.expressions.push(
            serde_json::from_value(serde_json::json!({"kind":"LiabilityAmount", "name":name}))
                .unwrap(),
        );
        assert_eq!(validate(&p).unwrap_err().code, "mori::core::liability");
    }
    let p =
        program(r#"liability("d", A, "debtor", "creditor", 0); ensures liability_present("d");"#);
    simulate(&p, "read", &Scenario::default()).unwrap();
}

#[test]
fn incoming_identity_substitutions_refuse_reads_before_the_first_step() {
    let p = program(r#"requires liability_present("d"); next x = liability_amount("d");"#);
    let original = Liability {
        name: "d".into(),
        asset: p.assets[0].identity.clone(),
        debtor: "debtor".into(),
        creditor: "creditor".into(),
        amount: UInt(7),
    };
    for member in [
        "network",
        "issuer",
        "reference",
        "scale",
        "debtor",
        "creditor",
        "duplicate",
    ] {
        let mut row = original.clone();
        match member {
            "network" => row.asset.network = "other".into(),
            "issuer" => row.asset.issuer = "other".into(),
            "reference" => row.asset.reference = "other".into(),
            "scale" => row.asset.scale = 1,
            "debtor" => row.debtor = "other".into(),
            "creditor" => row.creditor = "other".into(),
            "duplicate" => {}
            _ => unreachable!(),
        }
        let mut s = Scenario {
            liabilities: vec![row],
            ..Scenario::default()
        };
        if member == "duplicate" {
            s.liabilities.push(original.clone());
        }
        let before = s.clone();
        let error = simulate(&p, "read", &s).unwrap_err();
        let expected = if matches!(member, "network" | "issuer" | "reference" | "scale") {
            "mori::core::asset"
        } else {
            "mori::core::liability"
        };
        assert_eq!(error.code, expected);
        assert_eq!(error.step, None);
        assert_eq!(s, before);
    }
}
