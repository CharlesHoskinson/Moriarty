//! Compile-time refusals, `/2` admission-then-refusal with the real stage plan, the
//! target Emit policy and the pending claimant-bound ABI.
mod common;

use common::*;
use mori_core::UInt;
use mori_r1cs::staged::{SegmentEntry, SegmentExit, StoredCapParameter};
use mori_r1cs::*;

#[test]
fn staged_programs_are_admitted_by_stage_load_then_refused_with_their_plan() {
    let addendum = json(ADDENDUM);
    let hashes = &addendum["additional_expectations"]["program_hashes"];
    for (source, plan_json, program_key, plan_key, action, ranges) in [
        (DELIVERY2, DELIVERY2_PLAN, "delivery.mori (moriarty/2)", "delivery.mori plan_hash", "deliver", vec![(0u16, 2u16), (2, 5)]),
        (RESERVE2, RESERVE2_PLAN, "reserve.mori (moriarty/2)", "reserve.mori plan_hash", "allocate", vec![(0, 3), (3, 8), (8, 12)]),
    ] {
        let Err(CompileError::StagedNotLowered(plan)) = try_compile(source, options(action, CapBinding::NotBound)) else {
            panic!("a moriarty/2 source must be refused with its staged lowering plan");
        };
        assert_eq!(plan.language, "moriarty/2");
        assert_eq!(plan.semantics, "moriarty-core/2");
        assert_eq!(plan.stored_cap, StoredCapParameter::ExplicitPublicSlot);
        assert_eq!(plan.program_hash.hex(), hashes[program_key].as_str().unwrap()); // [ORACLE-derived]
        assert_eq!(plan.plan_hash.hex(), hashes[plan_key].as_str().unwrap());
        assert!(!plan.remaining.is_empty());
        // [ORACLE] costs and prefix sums from the frozen staged expected plan.
        let expected = json(plan_json);
        let ex = &expected["actions"][0];
        let a = &plan.actions[0];
        assert_eq!(a.action, action);
        assert_eq!(a.plan_total, UInt(uint(ex["plan_total"].as_str().unwrap())));
        assert_eq!(a.segments.len(), ex["segment_cost"].as_array().unwrap().len());
        for (i, s) in a.segments.iter().enumerate() {
            assert_eq!(s.segment_cost, UInt(uint(ex["segment_cost"][i].as_str().unwrap())));
            assert_eq!(s.committed_before, UInt(uint(ex["committed_before"][i].as_str().unwrap())));
            assert_eq!((s.statement_start, s.statement_end), ranges[i]); // [AUTHOR] ADDENDUM-1
            match (&s.entry, i) {
                (SegmentEntry::Start { .. }, 0) => assert!(s.not_advanced_attempted.is_none()),
                (SegmentEntry::Resume { site, .. }, _) => assert_eq!(usize::from(*site), i),
                _ => panic!("segment {i} entry"),
            }
            match (&s.exit, i + 1 == a.segments.len()) {
                (SegmentExit::Complete, true) | (SegmentExit::Await { .. }, false) => {}
                _ => panic!("segment {i} exit"),
            }
        }
    }
}

#[test]
fn staged_not_advanced_costs_match_the_staged_oracle() {
    // [ORACLE] false resumes: delivery site 1 attempts 2; reserve site 1 attempts 4, site 2 attempts 2.
    let costs = |source: &str, action: &str| -> Vec<Option<UInt>> {
        let Err(CompileError::StagedNotLowered(plan)) = try_compile(source, options(action, CapBinding::NotBound)) else { panic!() };
        plan.actions[0].segments.iter().map(|s| s.not_advanced_attempted).collect()
    };
    assert_eq!(costs(DELIVERY2, "deliver"), vec![None, Some(UInt(2))]);
    assert_eq!(costs(RESERVE2, "allocate"), vec![None, Some(UInt(4)), Some(UInt(2))]);
    // Emit is retained in the staged plan, never dropped.
    let Err(CompileError::StagedNotLowered(plan)) = try_compile(RESERVE2, options("allocate", CapBinding::NotBound)) else { panic!() };
    assert!(plan.actions[0].segments[2].has_emit);
}

#[test]
fn core_admission_refusals_are_core_errors() {
    let bad = r#"language "moriarty/1"; agreement A { action a() { requires x; } }"#;
    assert!(matches!(try_compile(bad, options("a", CapBinding::NotBound)), Err(CompileError::Core(_))));
    let profile = r#"language "moriarty/9"; agreement A { action a() { } }"#;
    assert!(matches!(try_compile(profile, options("a", CapBinding::NotBound)), Err(CompileError::Core(_))));
}

#[test]
fn unknown_action_external_duty_and_static_failures_are_refused() {
    assert!(matches!(try_compile(SELF_MOVE, options("nope", CapBinding::NotBound)), Err(CompileError::UnknownAction(_))));

    let external = r#"language "moriarty/1";
agreement DischargeOnly {
  asset PAY = { network: "local", issuer: "t", reference: "PAY", scale: 0 };
  action settle(amount: UInt) { discharge("x", amount); }
}
"#;
    assert!(matches!(
        try_compile(external, options("settle", CapBinding::NotBound)),
        Err(CompileError::Unsupported { construct: "external duty", .. })
    ));

    for (body, why) in [("requires 1 > 2;", "constant false predicate"), ("let x = 0 - 1;", "constant underflow")] {
        let src = format!("language \"moriarty/1\";\nagreement S {{\n  action a() {{ {body} }}\n}}\n");
        assert!(matches!(try_compile(&src, options("a", CapBinding::NotBound)), Err(CompileError::StaticFailure { .. })), "{why}");
    }

    // Conflicting source identities with no reads pass validate, but every simulate
    // refuses (runtime.rs:500-527): no success relation.
    let conflict = r#"language "moriarty/1";
agreement Conflict {
  asset U = { network: "local", issuer: "t", reference: "u", scale: 0 };
  action a(x: UInt) { liability("d", U, "p", "q", x); }
  action b(x: UInt) { liability("d", U, "p", "r", x); }
}
"#;
    assert!(matches!(try_compile(conflict, options("a", CapBinding::NotBound)), Err(CompileError::StaticFailure { .. })));
}

#[test]
fn represented_snapshot_bounds_are_checked_conservatively() {
    let src = format!(
        "language \"moriarty/1\";\nagreement Big {{\n  state n: UInt = 0;\n  asset U = {{ network: \"local\", issuer: \"t\", reference: \"u\", scale: 0 }};\n  action a() {{ {} }}\n}}\n",
        "next n = n; ".repeat(14)
    );
    let mut opts = options("a", CapBinding::NotBound);
    opts.frame = FrameCells::Include((0..4095).map(|i| CellRef { asset: "U".into(), account: format!("a{i}") }).collect());
    // (1 state + 4095 cells + 0 duties + 0 liability statements + 1) * (14 + 2) = 65,552 > 65,536.
    assert!(matches!(try_compile(&src, opts), Err(CompileError::Bound(_))));
    let mut unknown = options("a", CapBinding::NotBound);
    unknown.frame = FrameCells::Include(vec![CellRef { asset: "NOPE".into(), account: "x".into() }]);
    assert!(matches!(try_compile(&src, unknown), Err(CompileError::Option(_))));
}

#[test]
fn identity_inputs_are_mandatory() {
    let mut opts = options("move", CapBinding::NotBound);
    opts.compiler.core_closure_sha256 = Digest32([0; 32]);
    assert!(matches!(try_compile(SELF_MOVE, opts), Err(CompileError::Identity(_))));
}

#[test]
fn emit_is_lowered_but_refused_whole_program_by_the_initial_ledger_target() {
    // The relation lowers Emit (payroll pay/settle emit); the target policy refuses the
    // whole program, including its actions that do not emit.
    let pay = compile_ok(PAYROLL, options("pay", CapBinding::NotBound));
    assert!(matches!(initial_ledger_target_admits(pay.program()), Err(AdapterError::TargetRefusal(_))));
    let invoice = compile_ok(INVOICE, options("open", CapBinding::NotBound));
    assert!(initial_ledger_target_admits(invoice.program()).is_ok());
}

#[test]
fn claimant_bound_abi_v1_is_not_adopted_here() {
    let refused = compile_source::<ark_bls12_381::Fr>(INVOICE, options("open", CapBinding::NotBound), StatementAdapter::ClaimantBoundAbiV1Pending);
    assert!(matches!(refused, Err(CompileError::Adapter(AdapterError::Unadopted(_)))));
}

#[test]
fn coverage_rows_exist_for_every_construct() {
    use mori_r1cs::coverage::{Support, table};
    let rows = table();
    let constructs: Vec<_> = rows.iter().filter(|r| r.item.starts_with("Expr::") || r.item.starts_with("BinaryOp::") || r.item.starts_with("Statement::")).collect();
    assert_eq!(constructs.len(), 6 + 11 + 10);
    for r in constructs {
        let expected = if r.item == "Statement::Await" { Support::PlannedNotLowered } else { Support::Lowered };
        assert_eq!(r.support, expected, "{}", r.item);
    }
}
