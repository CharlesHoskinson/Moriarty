//! Source-wide liability identity admission through the real Core evaluator.
use mori_core::*;

const SOURCE: &str = r#"language "moriarty/1";
agreement Registry {
 asset USD = { network: "local", issuer: "issuer", reference: "usd", scale: 2 };
 asset NOTE = { network: "local", issuer: "issuer", reference: "note", scale: 0 };
 state touched: UInt = 0;
 action open(delta: UInt) { liability("delivery", USD, "seller", "buyer", delta); }
 action accrue(delta: UInt) { liability("delivery", USD, "seller", "buyer", delta); }
 action settle(delta: UInt) {
   next touched = touched + 1;
   emit "started"(touched);
   transfer(USD, "seller", "buyer", delta);
   discharge("delivery", delta);
 }
 action audit() { next touched = touched + 1; emit "audit"(touched); }
 action guarded() { requires false; }
}"#;

fn uint(n: u128) -> Value {
    Value::UInt(UInt(n))
}
fn scenario(p: &Program, residual: Option<u128>, delta: Option<u128>) -> Scenario {
    Scenario {
        inputs: delta
            .map(|n| [("delta".into(), uint(n))].into())
            .unwrap_or_default(),
        balances: ["seller", "buyer"]
            .into_iter()
            .map(|account| Balance {
                asset: p.assets[0].identity.clone(),
                account: account.into(),
                amount: UInt(if account == "seller" { 20 } else { 0 }),
            })
            .collect(),
        liabilities: residual
            .map(|n| {
                vec![Liability {
                    name: "delivery".into(),
                    asset: p.assets[0].identity.clone(),
                    debtor: "seller".into(),
                    creditor: "buyer".into(),
                    amount: UInt(n),
                }]
            })
            .unwrap_or_default(),
        ..Scenario::default()
    }
}
fn refusal(p: &Program, action: &str, s: &Scenario, step: Option<u16>) {
    let before = s.clone();
    let e = simulate(p, action, s).expect_err("substituted or missing duty must refuse");
    assert_eq!(e.code, "mori::core::liability");
    assert_eq!(
        e.step, step,
        "identity admission must precede action execution"
    );
    assert_eq!(*s, before, "refusal must preserve caller state and funds");
}

#[test]
fn declared_identity_substitution_refuses_before_every_action_even_when_untouched() {
    let p = parse(SOURCE).unwrap();
    let mut failures = Vec::new();
    for action in ["open", "accrue", "settle", "audit", "guarded"] {
        for member in ["asset", "debtor", "creditor"] {
            let delta = matches!(action, "open" | "accrue" | "settle").then_some(3);
            let mut s = scenario(&p, Some(10), delta);
            match member {
                "asset" => s.liabilities[0].asset = p.assets[1].identity.clone(),
                "debtor" => s.liabilities[0].debtor = "other-seller".into(),
                "creditor" => s.liabilities[0].creditor = "other-buyer".into(),
                _ => unreachable!(),
            }
            let before = s.clone();
            match simulate(&p, action, &s) {
                Err(e) if e.code == "mori::core::liability" && e.step.is_none() => {}
                Err(e) => failures.push(format!("{action}/{member}: {e}")),
                Ok(_) => failures.push(format!("{action}/{member}: wrongly accepted")),
            }
            assert_eq!(s, before);
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("; "));
}

#[test]
fn zero_tombstone_is_present_and_identity_is_still_immutable() {
    let p = parse(SOURCE).unwrap();
    let empty = scenario(&p, None, Some(0));
    let created = simulate(&p, "open", &empty).unwrap();
    assert!(created.pre.liabilities.is_empty());
    assert_eq!(created.post.liabilities[0].amount, UInt(0));
    let zero = scenario(&p, Some(0), Some(0));
    let settled = simulate(&p, "settle", &zero).unwrap();
    assert_eq!(settled.post.liabilities, zero.liabilities);
    assert_eq!(settled.effects.len(), 4);
    for member in ["asset", "debtor", "creditor"] {
        let mut changed = zero.clone();
        match member {
            "asset" => changed.liabilities[0].asset = p.assets[1].identity.clone(),
            "debtor" => changed.liabilities[0].debtor = "other".into(),
            "creditor" => changed.liabilities[0].creditor = "other".into(),
            _ => unreachable!(),
        }
        refusal(&p, "settle", &changed, None);
    }
    refusal(&p, "settle", &empty, Some(3));
}

#[test]
fn renamed_row_does_not_supply_the_declared_discharge_and_unknown_rows_are_preserved() {
    let p = parse(SOURCE).unwrap();
    let mut s = scenario(&p, Some(10), Some(3));
    s.liabilities[0].name = "external-duty".into();
    refusal(&p, "settle", &s, Some(3));
    s.inputs.clear();
    let audited = simulate(&p, "audit", &s).unwrap();
    assert_eq!(audited.post.liabilities, s.liabilities);
    assert_eq!(audited.post.state["touched"], uint(1));
    s.inputs.insert("delta".into(), uint(3));
    let opened = simulate(&p, "open", &s).unwrap();
    assert_eq!(opened.post.liabilities.len(), 2);
    assert!(opened.post.liabilities.contains(&s.liabilities[0]));
}

#[test]
fn valid_residuals_repeated_declarations_and_unrelated_actions_keep_existing_effects() {
    let p = parse(SOURCE).unwrap();
    for residual in [3, 10, u128::MAX] {
        let s = scenario(&p, Some(residual), Some(3));
        let before = s.clone();
        let e = simulate(&p, "settle", &s).unwrap();
        assert_eq!(e.post.liabilities[0].amount, UInt(residual - 3));
        assert_eq!(e.post.state["touched"], uint(1));
        assert_eq!(e.effects.len(), 4);
        assert!(matches!(e.effects[0], Effect::State { .. }));
        assert!(matches!(e.effects[1], Effect::Emit { .. }));
        assert!(matches!(
            e.effects[2],
            Effect::Transfer {
                amount: UInt(3),
                ..
            }
        ));
        assert!(
            matches!(e.effects[3], Effect::Liability { after: UInt(n), .. } if n == residual - 3)
        );
        replay(&p, "settle", &s, &e).unwrap();
        assert_eq!(s, before);
    }
    for action in ["open", "accrue"] {
        let e = simulate(&p, action, &scenario(&p, Some(10), Some(3))).unwrap();
        assert_eq!(e.post.liabilities[0].amount, UInt(13));
    }
    for residual in [None, Some(0), Some(u128::MAX)] {
        let s = scenario(&p, residual, None);
        assert_eq!(
            simulate(&p, "audit", &s).unwrap().post.liabilities,
            s.liabilities
        );
    }
    let s = scenario(&p, Some(2), Some(3));
    refusal(&p, "settle", &s, Some(3));
}

#[test]
fn exact_asset_components_and_duplicate_names_keep_existing_admission_rules() {
    let p = parse(SOURCE).unwrap();
    for member in ["network", "issuer", "reference", "scale"] {
        let mut s = scenario(&p, Some(10), None);
        match member {
            "network" => s.liabilities[0].asset.network = "other".into(),
            "issuer" => s.liabilities[0].asset.issuer = "other".into(),
            "reference" => s.liabilities[0].asset.reference = "other".into(),
            "scale" => s.liabilities[0].asset.scale = 3,
            _ => unreachable!(),
        }
        let before = s.clone();
        let e = simulate(&p, "audit", &s).unwrap_err();
        assert_eq!(e.code, "mori::core::asset");
        assert_eq!(e.step, None);
        assert_eq!(s, before);
    }
    let mut s = scenario(&p, Some(10), None);
    s.liabilities.push(s.liabilities[0].clone());
    refusal(&p, "audit", &s, None);
}

#[test]
fn conflicting_source_identity_refuses_at_admission_without_changing_parse_contract() {
    for replacement in [
        "liability(\"delivery\", NOTE, \"seller\", \"buyer\", delta)",
        "liability(\"delivery\", USD, \"other\", \"buyer\", delta)",
        "liability(\"delivery\", USD, \"seller\", \"other\", delta)",
    ] {
        let source = SOURCE.replace(
            "action accrue(delta: UInt) { liability(\"delivery\", USD, \"seller\", \"buyer\", delta); }",
            &format!("action accrue(delta: UInt) {{ {replacement}; }}"),
        );
        let p = parse(&source).unwrap();
        validate(&p).unwrap();
        refusal(&p, "audit", &scenario(&p, None, None), None);
    }
}

#[test]
fn discharge_only_core_program_keeps_supplied_identity_without_compiler_metadata() {
    let source = SOURCE.replace(
        "action open(delta: UInt) { liability(\"delivery\", USD, \"seller\", \"buyer\", delta); }",
        "",
    ).replace(
        "action accrue(delta: UInt) { liability(\"delivery\", USD, \"seller\", \"buyer\", delta); }",
        "",
    );
    let p = parse(&source).unwrap();
    let mut s = scenario(&p, Some(10), Some(3));
    s.liabilities[0].asset = p.assets[1].identity.clone();
    s.liabilities[0].debtor = "external-debtor".into();
    s.liabilities[0].creditor = "external-creditor".into();
    let e = simulate(&p, "settle", &s).unwrap();
    let mut expected = s.liabilities.clone();
    expected[0].amount = UInt(7);
    assert_eq!(e.post.liabilities, expected);
    let mut missing = s;
    missing.liabilities.clear();
    refusal(&p, "settle", &missing, Some(3));
}

#[test]
fn core_registry_keeps_general_declaration_bounds_without_compiler_sixteen_entry_limit() {
    let mut p = parse(SOURCE).unwrap();
    let statement = p.actions[0].statements[0].clone();
    for i in 0..17 {
        let mut added = statement.clone();
        if let Statement::Liability { name, .. } = &mut added {
            *name = format!("duty{i}");
        }
        p.actions[0].statements.push(added);
    }
    let e = simulate(&p, "open", &scenario(&p, None, Some(1))).unwrap();
    assert_eq!(e.post.liabilities.len(), 18);
}
