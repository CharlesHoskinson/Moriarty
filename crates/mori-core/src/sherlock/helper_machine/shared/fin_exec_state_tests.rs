use super::fin_carrier::{FinAssetRowId, FinGenesisRequest, FinScalar, FinStateRowId};
use super::fin_exec_state::{
    FinExecInvariantError, check_fin_exec_genesis_invariant, check_fin_exec_invariant,
    validate_fin_action_plan,
};
use super::owner_host::{
    admit_fin_genesis_image, observe_fin_bound_image, prepare_fin_staged_source,
};
use super::{BoundsC, FinCapacitiesC, LayoutCapacitiesC, LayoutSchemaC, RunIntC};
use crate::{AssetId, Balance, Scenario, UInt, Value};
use std::collections::BTreeMap;

const A_SRC: &str = r##"language "moriarty/2";
agreement Alpha {
  asset UNIT = { network: "local", issuer: "alpha", reference: "UNIT", scale: 0 };
  state settled: UInt = 0;
  action release() {
    liability("due", UNIT, "escrow", "beneficiary", 17);
    next settled = 17;
  }
}
"##;

const B_SRC: &str = r##"language "moriarty/2";
agreement Beta {
  asset ALT = { network: "local", issuer: "beta", reference: "ALT", scale: 2 };
  state paid: UInt = 3;
  action settle() {
    let grant = 9;
    next paid = grant;
  }
}
"##;

const C_SRC: &str = r##"language "moriarty/2";
agreement Gamma {
  asset COIN = { network: "n2", issuer: "gamma", reference: "COIN", scale: 0 };
  state done: UInt = 0;
  action issue() {
    liability("owed", COIN, "vault", "holder", 23);
    next done = 23;
  }
}
"##;

fn bounds() -> BoundsC {
    BoundsC {
        max_arity: 2,
        frames: 64,
        max_reference_work: 1024,
        max_helper_work: 1024,
        step_cap: 1,
        serialization_bytes: 1_000_000,
        ..BoundsC::default()
    }
}

fn layout(b: BoundsC) -> LayoutSchemaC {
    LayoutSchemaC {
        layout_version: ("t".into(), "1".into()),
        field_id: None,
        widths: None,
        scalar_layout_id: None,
        bounds: b,
        capacities: LayoutCapacitiesC {
            name_rows: 64,
            name_bytes: 4096,
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

fn genesis(src: &str, action: &str, scenario: Scenario) -> (super::Admitted, super::MachineImage) {
    let b = bounds();
    let prepared = prepare_fin_staged_source(src, &layout(b), &caps()).expect("prepare");
    admit_fin_genesis_image(
        prepared,
        &b,
        &layout(b),
        &RunIntC::Nat(8),
        &RunIntC::Nat(64),
        &FinGenesisRequest {
            action: action.into(),
            scenario,
            cap: None,
        },
    )
    .expect("financial genesis")
}

fn prepared(src: &str) -> super::fin_carrier::FinPreparedStagedSource {
    let b = bounds();
    prepare_fin_staged_source(src, &layout(b), &caps()).expect("prepare")
}

fn balance(account: &str, asset: AssetId, amount: u128) -> Balance {
    Balance {
        account: account.into(),
        asset,
        amount: UInt(amount),
    }
}

#[test]
fn mu1_public_plan_length_zero_is_typed_refusal_not_panic() {
    // Exact constructible counterexample shape formerly reaching
    // committed_before[0] on an empty vector.
    let action = super::fin_carrier::FinActionPlan {
        name: "settle".into(),
        segment_src_cost: vec![1],
        committed_before: Vec::new(),
        plan_total: 1,
    };
    assert_eq!(
        validate_fin_action_plan(&action),
        Err(FinExecInvariantError::PlanLengthMismatch {
            segment_src_cost_len: 1,
            committed_before_len: 0,
        })
    );

    let prefix_bad = super::fin_carrier::FinActionPlan {
        name: "settle".into(),
        segment_src_cost: vec![1, 2],
        committed_before: vec![0, 9],
        plan_total: 3,
    };
    assert_eq!(
        validate_fin_action_plan(&prefix_bad),
        Err(FinExecInvariantError::PlanPrefixMismatch {
            segment_index: 1,
            expected: 1,
            actual: 9,
        })
    );

    let total_bad = super::fin_carrier::FinActionPlan {
        name: "settle".into(),
        segment_src_cost: vec![1],
        committed_before: vec![0],
        plan_total: 2,
    };
    assert_eq!(
        validate_fin_action_plan(&total_bad),
        Err(FinExecInvariantError::PlanTotalMismatch {
            expected: 1,
            actual: 2,
        })
    );

    let empty = super::fin_carrier::FinActionPlan {
        name: "settle".into(),
        segment_src_cost: Vec::new(),
        committed_before: Vec::new(),
        plan_total: 0,
    };
    assert_eq!(
        validate_fin_action_plan(&empty),
        Err(FinExecInvariantError::PlanEmptySegments)
    );
}

#[test]
fn mu1_genesis_a_and_c_satisfy_genesis_and_general_invariants() {
    for (src, action, reference, issuer) in [
        (A_SRC, "release", "UNIT", "alpha"),
        (C_SRC, "issue", "COIN", "gamma"),
    ] {
        let asset = AssetId {
            network: if issuer == "alpha" { "local" } else { "n2" }.into(),
            issuer: issuer.into(),
            reference: reference.into(),
            scale: 0,
        };
        let scenario = Scenario {
            state: BTreeMap::new(),
            inputs: BTreeMap::new(),
            balances: vec![
                balance("escrow", asset.clone(), 100),
                balance("vault", asset, 100),
            ],
            liabilities: Vec::new(),
        };
        let (admitted, image) = genesis(src, action, scenario);
        let obs = observe_fin_bound_image(&admitted, &image).expect("observe");
        let state = obs.image.state;
        assert_eq!(state.committed, UInt(0));
        assert_eq!(state.exec.statement_cursor, 0);
        assert_eq!(state.exec.segment_index, 0);
        assert_eq!(state.exec.attempted_in_segment, 0);
        assert!(state.exec.tentative_effects.is_empty());
        assert_eq!(state.exec.retained_env, state.inputs);
        assert_eq!(state.exec.tentative_snapshot, state.snapshot);
        assert_eq!(
            state
                .snapshot
                .to_core(state.snapshot.source())
                .expect("core"),
            state.core_snapshot
        );
        check_fin_exec_invariant(state, obs.admission.prepared).expect("general genesis invariant");
        check_fin_exec_genesis_invariant(state, obs.admission.prepared).expect("genesis invariant");
    }
}

#[test]
fn mu1_genesis_b_let_source_has_paid_3_and_no_tentative_effect() {
    let (admitted, image) = genesis(B_SRC, "settle", Scenario::default());
    let obs = observe_fin_bound_image(&admitted, &image).expect("observe");
    let state = obs.image.state;
    assert_eq!(state.snapshot.states(), [FinScalar::UInt(3)]);
    assert_eq!(state.exec.tentative_snapshot.states(), [FinScalar::UInt(3)]);
    assert_eq!(state.exec.statement_cursor, 0);
    assert!(state.exec.retained_env.is_empty());
    check_fin_exec_genesis_invariant(state, obs.admission.prepared).expect("genesis invariant");
}

#[test]
fn mu1_each_genesis_relation_mutation_is_rejected_without_strengthening_general() {
    // K6 earliest-refusal table, derived from the corrected source order
    // before any execution. check_fin_exec_genesis_invariant checks the
    // genesis-only conjuncts first, in this order: segment, cursor,
    // attempted, committed, tentative snapshot, tentative effects,
    // retained env; it then runs the unchanged all-state invariant.
    // check_fin_exec_invariant alone keeps its own order and, in
    // particular, reports EffectsAheadOfCursor for effects > cursor.
    //
    // fixture (single mutation from B genesis)   all-state earliest      genesis earliest
    // -----------------------------------------  ----------------------  ------------------------------
    // tentative snapshot = 99                    Ok                      GenesisTentativeNotCommitted
    // cursor = 1, no effects                     Ok                      GenesisCursorNotZero{cursor:1}
    // cursor = 0, one effect                     EffectsAheadOfCursor    GenesisTentativeEffectsNotEmpty
    // cursor = 1, one effect                     Ok                      GenesisCursorNotZero{cursor:1}
    // retained env += invented binding           RetainedEnvPrefixMismatch is NOT
    //                                            reported: prefix of the empty
    //                                            genesis inputs still holds, so
    //                                            all-state is Ok            GenesisRetainedEnvMismatch
    // segment_index = 99 (all-state call only)   SegmentOutOfRange       not asserted via genesis here
    // action_index = 99 (all-state call only)    PlanActionMissing       not asserted via genesis here
    // committed = 999 (all-state call only)      CommittedMismatch       not asserted via genesis here
    //
    // Under the previous genesis order (all-state first, then cursor
    // before effects) GenesisTentativeEffectsNotEmpty was unreachable:
    // cursor = 0 with an effect failed all-state EffectsAheadOfCursor,
    // and cursor >= 1 failed GenesisCursorNotZero first. The corrected
    // order above makes every genesis conjunct reachable while a state
    // is accepted only if both predicates pass.
    let (admitted, image) = genesis(B_SRC, "settle", Scenario::default());
    let obs = observe_fin_bound_image(&admitted, &image).expect("observe");
    let prepared = obs.admission.prepared.clone();
    let state = obs.image.state.clone();
    let bound = prepared.bound_source().clone();

    // Tentative divergence: valid general working state, invalid genesis.
    let mut bad = state.clone();
    super::fin_carrier::apply_set_state(
        &bound,
        &mut bad.exec.tentative_snapshot,
        FinStateRowId(0),
        FinScalar::UInt(99),
    )
    .expect("mutate tentative");
    assert!(check_fin_exec_invariant(&bad, &prepared).is_ok());
    assert_eq!(
        check_fin_exec_genesis_invariant(&bad, &prepared),
        Err(FinExecInvariantError::GenesisTentativeNotCommitted)
    );

    // Cursor only.
    let mut bad = state.clone();
    bad.exec.statement_cursor = 1;
    assert!(check_fin_exec_invariant(&bad, &prepared).is_ok());
    assert_eq!(
        check_fin_exec_genesis_invariant(&bad, &prepared),
        Err(FinExecInvariantError::GenesisCursorNotZero { cursor: 1 })
    );

    // Effects only, at the genesis cursor. The all-state invariant keeps
    // its framing refusal; the genesis predicate reports its own
    // stronger emptiness conjunct.
    let mut bad = state.clone();
    bad.exec
        .tentative_effects
        .push(super::fin_carrier::FinEffectC::Liability {
            name: "x".into(),
            before: 0,
            after: 1,
        });
    assert_eq!(
        check_fin_exec_invariant(&bad, &prepared),
        Err(FinExecInvariantError::EffectsAheadOfCursor)
    );
    assert_eq!(
        check_fin_exec_genesis_invariant(&bad, &prepared),
        Err(FinExecInvariantError::GenesisTentativeEffectsNotEmpty)
    );

    // Combined cursor + effect fixture formerly used here: the earliest
    // genesis refusal is the cursor conjunct, which precedes effects.
    let mut bad = state.clone();
    bad.exec
        .tentative_effects
        .push(super::fin_carrier::FinEffectC::Liability {
            name: "x".into(),
            before: 0,
            after: 1,
        });
    bad.exec.statement_cursor = 1;
    assert!(check_fin_exec_invariant(&bad, &prepared).is_ok());
    assert_eq!(
        check_fin_exec_genesis_invariant(&bad, &prepared),
        Err(FinExecInvariantError::GenesisCursorNotZero { cursor: 1 })
    );

    let mut bad = state.clone();
    bad.exec
        .retained_env
        .push(super::fin_carrier::FinInputBinding {
            name: "invented".into(),
            ty: crate::Type::UInt,
            value: Value::UInt(UInt(1)),
        });
    assert!(check_fin_exec_invariant(&bad, &prepared).is_ok());
    assert_eq!(
        check_fin_exec_genesis_invariant(&bad, &prepared),
        Err(FinExecInvariantError::GenesisRetainedEnvMismatch)
    );

    let mut bad = state.clone();
    bad.exec.segment_index = 99;
    assert!(matches!(
        check_fin_exec_invariant(&bad, &prepared),
        Err(FinExecInvariantError::SegmentOutOfRange { .. })
    ));

    let mut bad = state.clone();
    bad.action_index = 99;
    assert_eq!(
        check_fin_exec_invariant(&bad, &prepared),
        Err(FinExecInvariantError::PlanActionMissing { action_index: 99 })
    );

    let mut bad = state.clone();
    bad.committed = UInt(999);
    assert!(matches!(
        check_fin_exec_invariant(&bad, &prepared),
        Err(FinExecInvariantError::CommittedMismatch { .. })
    ));

    let _ = FinAssetRowId(0);
    let _ = admitted;
}

#[test]
fn mu1_invariant_uses_sealed_action_statement_count_not_caller_count() {
    let (admitted, image) = genesis(B_SRC, "settle", Scenario::default());
    let obs = observe_fin_bound_image(&admitted, &image).expect("observe");
    let prepared = obs.admission.prepared.clone();
    let mut bad = obs.image.state.clone();
    // Beta/settle has exactly two source statements. Cursor 2 is the
    // one-past-end position and remains representable; cursor 99 is not.
    bad.exec.statement_cursor = 2;
    assert!(check_fin_exec_invariant(&bad, &prepared).is_ok());
    bad.exec.statement_cursor = 99;
    assert_eq!(
        check_fin_exec_invariant(&bad, &prepared),
        Err(FinExecInvariantError::CursorOutOfRange {
            cursor: 99,
            statement_count: 2,
        })
    );
    let _ = admitted;
}
