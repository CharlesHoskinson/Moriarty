//! Duty presence (absent versus present-zero), discharge, self-transfer, frame cells
//! and the per-statement DAG memo.
mod common;

use common::*;
use mori_core::{Action, BinaryOp, Expr, Program, StateDecl, Statement, UInt};
use mori_r1cs::*;

#[test]
fn present_zero_is_not_absence() {
    // T9 [AUTHOR] on [ORACLE] invoice-finish: post invoice is present with amount 0.
    let rel = compile_ok(INVOICE, options("finish", CapBinding::PublicCap));
    let oracle = oracle_case("invoice-finish");
    let honest = oracle_values(&rel, &oracle, Some(oracle.cap));
    let invoice = rel.plan().duty_index("invoice").unwrap();
    assert_eq!(honest.post.duties[invoice], DutyValue { present: true, amount: UInt(0) });
    assert!(accepts(&rel, honest.clone()));
    let mut absent = honest;
    absent.post.duties[invoice] = DutyValue { present: false, amount: UInt(0) };
    assert!(!accepts(&rel, absent));
}

/// A fresh minimal program for the isolated canonicality check (T6, strengthened).
const CANON: &str = r#"language "moriarty/1";
agreement DutyCanon {
  asset U = { network: "fresh-unit", issuer: "canon", reference: "u", scale: 0 };
  action touch(x: UInt) {
    liability("d", U, "a", "b", x);
  }
}
"#;

fn canon_claim(pre_present: bool, pre: u128, x: u128) -> TransitionValues {
    TransitionValues {
        inputs: vec![u(x)],
        pre: BodyValues { states: vec![], cells: vec![], duties: vec![DutyValue { present: pre_present, amount: UInt(pre) }] },
        post: BodyValues { states: vec![], cells: vec![], duties: vec![DutyValue { present: true, amount: UInt(pre + x) }] },
        effects: vec![EffectValues::Duty { before: UInt(pre), after: UInt(pre + x) }],
        cap: None,
    }
}

#[test]
fn absent_duty_with_a_hidden_amount_is_refused_in_isolation() {
    let rel = compile_ok(CANON, options("touch", CapBinding::NotBound));
    // Controls: absent pre accrues from 0; a present pre of 5 accrues from 5.
    assert!(accepts(&rel, canon_claim(false, 0, 3)));
    assert!(accepts(&rel, canon_claim(true, 5, 3)));
    // [HOSTILE] the present-5 claim with only the pre presence bit flipped to absent,
    // in both the public slot and the entry witness. Every other constraint still
    // holds, so only `(1 - present) * amount = 0` can refuse it.
    let at = offset(&rel, "pre.duty.d.present");
    let mut a = assignment(&rel, canon_claim(true, 5, 3));
    a.public[at] = fr(0);
    let out = run(&rel, a, Tamper::none().set("entry/duty0.present#0", fr(0)));
    assert_eq!(out.overrides_used, 1);
    assert!(!out.satisfied);
    // The claimant encoder never produces such a claim.
    assert!(rel.assignment::<ark_bls12_381::Fr>(canon_claim(false, 5, 3)).is_err());
}

#[test]
fn discharge_requires_presence_and_bounded_amount() {
    let settle = compile_ok(PAYROLL, options("settle", CapBinding::NotBound));
    let duty = settle.plan().duty_index("benefit").unwrap();
    let employer = settle.plan().cells.iter().position(|c| c.account == "employer").unwrap();
    let worker = settle.plan().cells.iter().position(|c| c.account == "worker").unwrap();
    let state = |closed: bool| vec![b(closed), u(2), u(3)]; // closed, period, rate (by name)
    let mut cells = vec![UInt(0); 2];
    cells[employer] = UInt(78);
    cells[worker] = UInt(25);
    let claim = |amount: u128, present: bool, opening: u128| {
        let mut post = cells.clone();
        post[employer] = UInt(78u128.wrapping_sub(amount));
        post[worker] = UInt(25 + amount);
        let mut pre_d = vec![DutyValue { present: false, amount: UInt(0) }];
        pre_d[duty] = DutyValue { present, amount: UInt(opening) };
        let closing = opening.wrapping_sub(amount);
        TransitionValues {
            inputs: vec![u(amount)],
            pre: BodyValues { states: state(false), cells: cells.clone(), duties: pre_d },
            post: BodyValues { states: state(closing == 0), cells: post, duties: vec![DutyValue { present: true, amount: UInt(closing) }] },
            effects: vec![
                EffectValues::Duty { before: UInt(opening), after: UInt(closing) },
                EffectValues::Amount(UInt(amount)),
                EffectValues::State { before: b(false), after: b(closing == 0) },
                EffectValues::Emit(b(closing == 0)),
            ],
            cap: None,
        }
    };
    assert!(accepts(&settle, claim(7, true, 7))); // P2
    assert!(accepts(&settle, claim(3, true, 7))); // P3
    assert!(!accepts(&settle, claim(8, true, 7))); // P6: discharge exceeds residual
    // P7: benefit absent. The wrapping claim (closing = 0 - 1) is not even encodable as
    // a canonical post duty, so claim the "short-circuit" post with closing 0 instead.
    let mut p7 = claim(0, false, 0);
    p7.inputs = vec![u(1)];
    p7.post.cells[employer] = UInt(77);
    p7.post.cells[worker] = UInt(26);
    p7.effects[1] = EffectValues::Amount(UInt(1));
    assert!(!accepts(&settle, p7));
}

fn move_claim(balance: u128, amount: u128) -> TransitionValues {
    TransitionValues {
        inputs: vec![u(amount)],
        pre: BodyValues { states: vec![], cells: vec![UInt(balance)], duties: vec![] },
        post: BodyValues { states: vec![], cells: vec![UInt(balance)], duties: vec![] },
        effects: vec![EffectValues::Amount(UInt(amount))],
        cap: None,
    }
}

#[test]
fn self_transfer_keeps_the_debit_check() {
    let rel = compile_ok(SELF_MOVE, options("move", CapBinding::NotBound));
    assert_eq!(rel.plan().cells.len(), 1);
    assert_eq!(rel.plan().logical_work, UInt(2));
    assert!(accepts(&rel, move_claim(5, 5))); // S1
    assert!(!accepts(&rel, move_claim(5, 6))); // S2
    assert!(accepts(&rel, move_claim(0, 0))); // S3
}

#[test]
fn extra_frame_cells_are_preserved() {
    let mut opts = options("open", CapBinding::PublicCap);
    opts.frame = FrameCells::Include(vec![CellRef { asset: "CASH".into(), account: "spectator".into() }]);
    let rel = compile_ok(INVOICE, opts);
    assert_eq!(rel.layout().field_count, 74); // 70 + pre/post spectator UInt
    let oracle = oracle_case("invoice-open");
    let values = oracle_values(&rel, &oracle, Some(oracle.cap));
    let spectator = rel.plan().cells.iter().position(|c| c.account == "spectator").unwrap();
    assert_eq!(values.pre.cells[spectator], UInt(41));
    assert!(accepts(&rel, values.clone()));
    let mut moved = values;
    moved.post.cells[spectator] = UInt(42);
    assert!(!accepts(&rel, moved));
}

#[test]
fn shared_dag_nodes_are_reevaluated_per_statement() {
    // D1 [AUTHOR]: manual IR the parser never produces; admitted by the same validate.
    let program = Program {
        language: mori_core::LANGUAGE.into(),
        semantics: mori_core::SEMANTICS.into(),
        name: "DagShare".into(),
        states: vec![StateDecl { name: "s".into(), ty: mori_core::Type::UInt, initial: u(1) }],
        assets: vec![],
        expressions: vec![Expr::Name { name: "s".into() }, Expr::Binary { op: BinaryOp::Add, lhs: 0, rhs: 0 }],
        actions: vec![Action {
            name: "twice".into(),
            inputs: vec![],
            statements: vec![Statement::Next { state: "s".into(), value: 1 }, Statement::Next { state: "s".into(), value: 1 }],
        }],
    };
    let admitted = admit_program(program).expect("Core validate admits the DAG");
    let rel = mori_r1cs::compile::<ark_bls12_381::Fr>(admitted, options("twice", CapBinding::NotBound), StatementAdapter::DirectFieldV0).unwrap();
    assert_eq!(rel.plan().logical_work, UInt(6));
    let claim = |post: u128, mid: u128| TransitionValues {
        inputs: vec![],
        pre: BodyValues { states: vec![u(1)], cells: vec![], duties: vec![] },
        post: BodyValues { states: vec![u(post)], cells: vec![], duties: vec![] },
        effects: vec![EffectValues::State { before: u(1), after: u(mid) }, EffectValues::State { before: u(mid), after: u(post) }],
        cap: None,
    };
    assert!(accepts(&rel, claim(4, 2)));
    // A lowering that reused statement 0's node value would claim 2 -> 2.
    assert!(!accepts(&rel, claim(2, 2)));
}
