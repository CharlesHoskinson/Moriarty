//! Source-only MP1/MP2 tests: real `stage::load` via the owner-host seam on
//! the whole immutable A/B sources. No genesis, snapshot, or outcome claim.
use super::fin_carrier::{FinBindError, FinCapacitiesC};
use super::owner_host::prepare_fin_staged_source;
use super::{BoundsC, LayoutCapacitiesC, LayoutSchemaC};
use crate::{Expr, Statement, UInt, Value};

const A_SRC: &str = r##"language "moriarty/2";
agreement NewSplitSettlementA {
  asset UNIT = { network: "local", issuer: "new-split-settlement-a", reference: "UNIT", scale: 0 };
  asset HOLD = { network: "local", issuer: "preserved-reserve-a", reference: "HOLD", scale: 0 };
  state settled: UInt = 0;
  state audit_marker: UInt = 81;
  action release() {
    liability("due", UNIT, "escrow", "beneficiary", 17);
    transfer(UNIT, "escrow", "beneficiary", 6);
    fee(UNIT, "escrow", "treasury", 2);
    discharge("due", 6);
    next settled = 6;
    await unlock(ready: Bool) when ready;
    transfer(UNIT, "escrow", "beneficiary", 11);
    fee(UNIT, "escrow", "treasury", 1);
    discharge("due", 11);
    next settled = settled + 11;
    emit "settled_amount"(settled);
  }
}
"##;

const B_SRC: &str = r##"language "moriarty/2";
agreement NewRefundSettlementB {
  asset ALT = { network: "local", issuer: "new-refund-settlement-b", reference: "ALT", scale: 2 };
  asset HOLD = { network: "local", issuer: "preserved-reserve-b", reference: "HOLD", scale: 0 };
  state paid: UInt = 3;
  state marker: Bool = true;
  action settle() {
    let grant = 9;
    liability("grant_due", ALT, "vault", "recipient", grant);
    transfer(ALT, "vault", "recipient", 4);
    fee(ALT, "vault", "collector", 1);
    discharge("grant_due", 4);
    await priced(quote: UInt) when quote >= grant;
    transfer(ALT, "recipient", "vault", 2);
    transfer(ALT, "vault", "recipient", grant - 4);
    discharge("grant_due", grant - 4);
    next paid = paid + grant;
    emit "quoted"(quote);
  }
}
"##;

fn layout(name_rows: usize, name_bytes: usize) -> LayoutSchemaC {
    LayoutSchemaC {
        layout_version: ("t".into(), "1".into()),
        field_id: None,
        widths: None,
        scalar_layout_id: None,
        bounds: BoundsC::default(),
        capacities: LayoutCapacitiesC {
            name_rows,
            name_bytes,
            occurrence_path_slots: 64,
            value_slots: 64,
        },
        ext_capacities: None,
        boundary_capacities_present: false,
    }
}
fn caps() -> FinCapacitiesC {
    FinCapacitiesC {
        max_states: 8,
        max_assets: 8,
        max_balances: 64,
        max_liabilities: 64,
    }
}

#[test]
fn a_prepared_retains_whole_source_and_first_liability17() {
    let p = prepare_fin_staged_source(A_SRC, &layout(64, 4096), &caps()).expect("prepare A");
    assert_eq!(p.program().name, "NewSplitSettlementA");
    assert_eq!(p.program().states.len(), 2);
    assert_eq!(p.program().assets.len(), 2);
    assert_eq!(p.bound_source().states().len(), 2);
    assert_eq!(p.bound_source().assets().len(), 2);
    let action = p.action("release").expect("release action");
    assert_eq!(action.statements.len(), 11);
    assert!(p.action_inputs("release").expect("inputs").is_empty());
    assert!(!p.program().expressions.is_empty());
    match p.statement("release", 0).expect("first statement") {
        Statement::Liability {
            name,
            asset,
            debtor,
            creditor,
            amount,
        } => {
            assert_eq!(name, "due");
            assert_eq!(asset, "UNIT");
            assert_eq!(debtor, "escrow");
            assert_eq!(creditor, "beneficiary");
            assert_eq!(
                p.expression(*amount),
                Some(&Expr::Literal {
                    value: Value::UInt(UInt(17))
                })
            );
        }
        other => panic!("expected first Liability, got {other:?}"),
    }
    // Source-derived plan authority: FinPlan is the StagePlan, in order.
    let staged_plan = &p.stage_plan().actions[0];
    let fin_plan = &p.fin_plan().actions[0];
    assert_eq!(fin_plan.name, "release");
    assert_eq!(fin_plan.plan_total, staged_plan.plan_total.0);
    assert_eq!(
        fin_plan.segment_src_cost,
        staged_plan
            .segment_cost
            .iter()
            .map(|u| u.0)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        fin_plan.committed_before,
        staged_plan
            .committed_before
            .iter()
            .map(|u| u.0)
            .collect::<Vec<_>>()
    );
    let again =
        prepare_fin_staged_source(A_SRC, &layout(64, 4096), &caps()).expect("prepare A again");
    assert!(p.same_source(&again));
}

#[test]
fn b_prepared_let_grant9_binds_following_liability_by_name() {
    let p = prepare_fin_staged_source(B_SRC, &layout(64, 4096), &caps()).expect("prepare B");
    assert_eq!(p.program().name, "NewRefundSettlementB");
    let (let_name, let_value) = match p.statement("settle", 0).expect("let statement") {
        Statement::Let { name, value } => (name.clone(), *value),
        other => panic!("expected first Let, got {other:?}"),
    };
    assert_eq!(let_name, "grant");
    assert_eq!(
        p.expression(let_value),
        Some(&Expr::Literal {
            value: Value::UInt(UInt(9))
        })
    );
    match p.statement("settle", 1).expect("liability statement") {
        Statement::Liability {
            name,
            asset,
            debtor,
            creditor,
            amount,
        } => {
            assert_eq!(name, "grant_due");
            assert_eq!(asset, "ALT");
            assert_eq!(debtor, "vault");
            assert_eq!(creditor, "recipient");
            // Static source association only: the amount Expr::Name names the
            // preceding `let grant`; no runtime environment or provenance.
            assert_eq!(
                p.expression(*amount),
                Some(&Expr::Name {
                    name: let_name.clone()
                })
            );
        }
        other => panic!("expected Liability, got {other:?}"),
    }
}

#[test]
fn cross_source_distinction_is_preserved() {
    let a = prepare_fin_staged_source(A_SRC, &layout(64, 4096), &caps()).expect("A");
    let b = prepare_fin_staged_source(B_SRC, &layout(64, 4096), &caps()).expect("B");
    assert!(!a.same_source(&b));
    assert_ne!(a.program(), b.program());
    assert_ne!(a.program_hash(), b.program_hash());
    assert_ne!(a.extras(), b.extras());
    assert_eq!(a.bound_source().resolve_name("UNIT").is_ok(), true);
    assert_eq!(b.bound_source().resolve_name("ALT").is_ok(), true);
}

#[test]
fn staged_binder_refusals_are_preserved() {
    // A distinct interned spellings: program + 2 states + 2 assets + 1 action = 6.
    assert_eq!(
        prepare_fin_staged_source(A_SRC, &layout(2, 4096), &caps()).unwrap_err(),
        super::fin_carrier::FinStagedPrepareError::Bind(FinBindError::NameRowsExceeded {
            needed: 6,
            limit: 2
        })
    );
    match prepare_fin_staged_source(A_SRC, &layout(64, 4), &caps()).unwrap_err() {
        super::fin_carrier::FinStagedPrepareError::Bind(FinBindError::NameBytesExceeded {
            limit: 4,
            ..
        }) => {}
        other => panic!("expected NameBytesExceeded, got {other:?}"),
    }
    let mut small = caps();
    small.max_states = 0;
    assert_eq!(
        prepare_fin_staged_source(A_SRC, &layout(64, 4096), &small).unwrap_err(),
        super::fin_carrier::FinStagedPrepareError::Bind(FinBindError::CapacityExceeded("states"))
    );
    let mut small_assets = caps();
    small_assets.max_assets = 1;
    assert_eq!(
        prepare_fin_staged_source(B_SRC, &layout(64, 4096), &small_assets).unwrap_err(),
        super::fin_carrier::FinStagedPrepareError::Bind(FinBindError::CapacityExceeded("assets"))
    );
}

const COLLIDE_SRC: &str = r##"language "moriarty/2";
agreement Collide {
  asset go = { network: "n", issuer: "i", reference: "r", scale: 0 };
  state release: UInt = 0;
  action release() { next release = 1; }
  action go() { transfer(go, "a", "b", 1); }
  action Collide(x: UInt) { next release = x; }
}
"##;

const PROGRAM_STATE_SRC: &str = r##"language "moriarty/2";
agreement Same {
  asset coin = { network: "n", issuer: "i", reference: "r", scale: 0 };
  state Same: UInt = 0;
  action run() { next Same = 1; }
}
"##;

const PROGRAM_ASSET_SRC: &str = r##"language "moriarty/2";
agreement Same {
  asset Same = { network: "n", issuer: "i", reference: "r", scale: 0 };
  state count: UInt = 0;
  action run() { transfer(Same, "a", "b", 1); }
}
"##;

const SMALL_SRC: &str = r##"language "moriarty/2";
agreement P {
  asset A = { network: "n", issuer: "i", reference: "r", scale: 0 };
  state s: UInt = 0;
  action a() { next s = 1; }
}
"##;

#[test]
fn legal_cross_kind_collisions_resolve_by_typed_rows() {
    use super::fin_carrier::{FinAssetRowId, FinStateRowId, NameId};
    let p = prepare_fin_staged_source(COLLIDE_SRC, &layout(64, 4096), &caps())
        .expect("stage-legal collisions");
    let b = p.bound_source();
    // Distinct interned spellings sorted: Collide, go, release.
    assert_eq!(b.names(), ["Collide", "go", "release"]);
    assert_eq!(b.states()[0].row, FinStateRowId(0));
    assert_eq!(b.states()[0].text, "release");
    assert_eq!(b.states()[0].name, NameId(2));
    assert_eq!(b.assets()[0].row, FinAssetRowId(0));
    assert_eq!(b.assets()[0].text, "go");
    assert_eq!(b.assets()[0].name, NameId(1));
    assert_eq!(b.actions()[0].text, "release");
    assert_eq!(b.actions()[0].name, NameId(2));
    assert_eq!(b.actions()[1].text, "go");
    assert_eq!(b.actions()[1].name, NameId(1));
    assert_eq!(b.actions()[2].text, "Collide");
    assert_eq!(b.actions()[2].name, NameId(0));
    assert_eq!(b.resolve_name("release"), Ok(NameId(2)));
    let inputs = p.action_inputs("Collide").expect("inputs");
    assert_eq!(inputs.len(), 1);
    assert_eq!(inputs[0].name, "x");
    assert_eq!(inputs[0].ty, crate::Type::UInt);
    match p.statement("Collide", 0).expect("next") {
        Statement::Next { state, value } => {
            assert_eq!(state, "release");
            assert_eq!(p.expression(*value), Some(&Expr::Name { name: "x".into() }));
        }
        other => panic!("expected Next, got {other:?}"),
    }
}

#[test]
fn legal_program_declaration_collisions_resolve() {
    use super::fin_carrier::NameId;
    let p = prepare_fin_staged_source(PROGRAM_STATE_SRC, &layout(64, 4096), &caps())
        .expect("program/state");
    assert_eq!(p.program().name, "Same");
    assert_eq!(p.bound_source().states()[0].text, "Same");
    assert_eq!(p.bound_source().states()[0].name, NameId(0));
    assert_eq!(p.bound_source().resolve_name("Same"), Ok(NameId(0)));
    let p = prepare_fin_staged_source(PROGRAM_ASSET_SRC, &layout(64, 4096), &caps())
        .expect("program/asset");
    assert_eq!(p.program().name, "Same");
    assert_eq!(p.bound_source().assets()[0].text, "Same");
    assert_eq!(p.bound_source().assets()[0].name, NameId(0));
    assert_eq!(p.bound_source().resolve_name("Same"), Ok(NameId(0)));
}

#[test]
fn overlapping_refusals_pin_rows_bytes_state_asset_order() {
    // SMALL distinct spellings P, A, s, a: 4 rows and 4 bytes.
    let mut c = caps();
    c.max_states = 0;
    assert_eq!(
        prepare_fin_staged_source(SMALL_SRC, &layout(3, 4096), &c).unwrap_err(),
        super::fin_carrier::FinStagedPrepareError::Bind(FinBindError::NameRowsExceeded {
            needed: 4,
            limit: 3
        })
    );
    let mut c = caps();
    c.max_assets = 0;
    assert_eq!(
        prepare_fin_staged_source(SMALL_SRC, &layout(64, 3), &c).unwrap_err(),
        super::fin_carrier::FinStagedPrepareError::Bind(FinBindError::NameBytesExceeded {
            needed: 4,
            limit: 3
        })
    );
    let mut c = caps();
    c.max_states = 0;
    c.max_assets = 0;
    assert_eq!(
        prepare_fin_staged_source(SMALL_SRC, &layout(3, 3), &c).unwrap_err(),
        super::fin_carrier::FinStagedPrepareError::Bind(FinBindError::NameRowsExceeded {
            needed: 4,
            limit: 3
        })
    );
    assert_eq!(
        prepare_fin_staged_source(SMALL_SRC, &layout(64, 4096), &c).unwrap_err(),
        super::fin_carrier::FinStagedPrepareError::Bind(FinBindError::CapacityExceeded("states"))
    );
}

#[test]
fn whole_ab_hash_plan_await_captures_and_accessor_bounds() {
    for src in [A_SRC, B_SRC] {
        let p = prepare_fin_staged_source(src, &layout(64, 4096), &caps()).expect("prepare");
        assert_eq!(p.program_hash(), p.staged().program_hash());
        assert_eq!(p.plan_hash(), p.staged().plan_hash());
        assert_eq!(p.fin_plan().actions.len(), p.stage_plan().actions.len());
        for (f, s) in p.fin_plan().actions.iter().zip(&p.stage_plan().actions) {
            assert_eq!(f.name, s.action);
            assert_eq!(f.plan_total, s.plan_total.0);
            assert_eq!(
                f.segment_src_cost,
                s.segment_cost.iter().map(|u| u.0).collect::<Vec<_>>()
            );
            assert_eq!(
                f.committed_before,
                s.committed_before.iter().map(|u| u.0).collect::<Vec<_>>()
            );
        }
        assert_eq!(p.action("absent"), None);
        assert_eq!(p.action_inputs("absent"), None);
        assert_eq!(p.statement("absent", 0), None);
        assert_eq!(p.expression(u16::MAX), None);
    }
    let a = prepare_fin_staged_source(A_SRC, &layout(64, 4096), &caps()).expect("A");
    assert_eq!(a.program().assets[0].identity.reference, "UNIT");
    assert_eq!(a.program().assets[1].identity.reference, "HOLD");
    match a.statement("release", 5).expect("await") {
        Statement::Await {
            label,
            binders,
            guard,
        } => {
            assert_eq!(label, "unlock");
            assert_eq!(binders.len(), 1);
            assert_eq!(binders[0].name, "ready");
            assert_eq!(
                a.expression(*guard),
                Some(&Expr::Name {
                    name: "ready".into()
                })
            );
        }
        other => panic!("expected Await, got {other:?}"),
    }
    assert_eq!(a.statement("release", 99), None);
    let b = prepare_fin_staged_source(B_SRC, &layout(64, 4096), &caps()).expect("B");
    assert_eq!(b.stage_plan().actions[0].sites.len(), 1);
    assert_eq!(b.stage_plan().actions[0].sites[0].label, "priced");
    assert_eq!(b.stage_plan().actions[0].sites[0].captures.len(), 1);
    assert_eq!(b.stage_plan().actions[0].sites[0].captures[0].name, "grant");
}

#[test]
fn stage_error_via_one_and_same_source_context() {
    let v1 =
        "language \"moriarty/1\";\nagreement P { state s: UInt = 0; action a() { next s = 1; } }\n";
    assert!(matches!(
        prepare_fin_staged_source(v1, &layout(64, 4096), &caps()).unwrap_err(),
        super::fin_carrier::FinStagedPrepareError::Stage(_)
    ));
    let decl_dup = r##"language "moriarty/2";
agreement P {
  asset x = { network: "n", issuer: "i", reference: "r", scale: 0 };
  state x: UInt = 0;
  action a() { next x = 1; }
}
"##;
    assert!(matches!(
        prepare_fin_staged_source(decl_dup, &layout(64, 4096), &caps()).unwrap_err(),
        super::fin_carrier::FinStagedPrepareError::Stage(_)
    ));
    let a1 = prepare_fin_staged_source(A_SRC, &layout(64, 4096), &caps()).expect("a1");
    let a2 = prepare_fin_staged_source(A_SRC, &layout(32, 8192), &caps()).expect("a2");
    // Layout is not retained: this is source comparison, not context identity.
    assert!(a1.same_source(&a2));
    let mut c = caps();
    c.max_balances = 63;
    let a3 = prepare_fin_staged_source(A_SRC, &layout(64, 4096), &c).expect("a3");
    assert!(!a1.same_source(&a3));
}
