use super::fin_carrier::{
    FinAssetRowId, FinGenesisError, FinGenesisRequest, FinRunError, FinScalar,
};
use super::owner_host::{
    admit_fin_genesis_image, observe_fin_bound_image, prepare_fin_staged_source,
};
use super::{
    ActiveLengths, AllocCounters, BoundsC, CapacityKind, ControlC, DigestC, ExtCapacitiesC,
    FinCapacitiesC, FinUnsupportedOp, FinanceInput, HostViolationKind, LayoutCapacitiesC,
    LayoutSchemaC, LocatorC, NeededNatC, OutcomeC, OwnerEngine, PreImageView, RunIntC,
    StepInvClauseId, Tooling, WidthTable,
};
use crate::{AssetId, Balance, Liability, Scenario, UInt, Value};
use std::collections::BTreeMap;

const SRC: &str = r##"language "moriarty/2";
agreement GenesisSynthetic {
  asset COIN = { network: "local", issuer: "synthetic", reference: "COIN", scale: 0 };
  state count: UInt = 0;
  state flag: Bool = false;
  action open(amount: UInt) { next count = amount; }
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

fn layout(bounds: BoundsC) -> LayoutSchemaC {
    LayoutSchemaC {
        layout_version: ("t".into(), "1".into()),
        field_id: None,
        widths: None,
        scalar_layout_id: None,
        bounds,
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

fn coin() -> AssetId {
    AssetId {
        network: "local".into(),
        issuer: "synthetic".into(),
        reference: "COIN".into(),
        scale: 0,
    }
}

fn scenario() -> Scenario {
    Scenario {
        state: BTreeMap::new(),
        inputs: BTreeMap::from([("amount".to_string(), Value::UInt(UInt(7)))]),
        balances: vec![
            Balance {
                account: "alice".into(),
                asset: coin(),
                amount: UInt(10),
            },
            Balance {
                account: "bob".into(),
                asset: coin(),
                amount: UInt(3),
            },
        ],
        liabilities: vec![Liability {
            name: "due".into(),
            asset: coin(),
            debtor: "d".into(),
            creditor: "c".into(),
            amount: UInt(0),
        }],
    }
}

fn request(action: &str, scenario: Scenario, cap: Option<UInt>) -> FinGenesisRequest {
    FinGenesisRequest {
        action: action.into(),
        scenario,
        cap,
    }
}

fn genesis() -> (super::Admitted, super::MachineImage) {
    let b = bounds();
    let prepared = prepare_fin_staged_source(SRC, &layout(b), &caps()).expect("prepare");
    admit_fin_genesis_image(
        prepared,
        &b,
        &layout(b),
        &RunIntC::Nat(8),
        &RunIntC::Nat(64),
        &request("open", scenario(), Some(UInt(2))),
    )
    .expect("actual financial genesis")
}

fn pure_admitted(with_function: bool) -> (super::Admitted, super::MachineImage) {
    let mut b = bounds();
    b.code_rows = 3;
    b.function_rows = 1;
    b.code_child_slots = 1;
    b.parameter_slots = 1;
    b.constant_nodes = 1;
    let lit = |n: u128| super::CoreExpr::Literal {
        value: super::CoreValue::UInt {
            n: super::i128_class::KIntC::InRange(n),
        },
    };
    let program = if with_function {
        super::CoreProgram {
            functions: vec![super::CoreFunction {
                name: "identity".into(),
                parameters: vec![super::CoreParameter {
                    name: "x".into(),
                    sort: super::SortTag::UInt,
                }],
                return_type: super::SortTag::UInt,
                body: super::CoreExpr::Var { name: "x".into() },
            }],
            entry: super::CoreExpr::Call {
                name: "identity".into(),
                args: vec![lit(2)],
            },
        }
    } else {
        super::CoreProgram {
            functions: Vec::new(),
            entry: lit(7),
        }
    };
    let engine = super::owner_host::host_owner_engine();
    let admitted = engine
        .admit(&program, &b, &layout(b))
        .expect("pure admission");
    let image = engine
        .admit_run(
            &admitted,
            &RunIntC::Nat(8),
            &RunIntC::Nat(64),
            &FinanceInput::Absent,
        )
        .expect("pure genesis image");
    (admitted, image)
}

fn genesis_capture_with_limit(
    limit: usize,
) -> crate::sherlock::helper_machine::complete_observation::FinHostSnapshot {
    let mut b = bounds();
    b.serialization_bytes = limit;
    let prepared = prepare_fin_staged_source(SRC, &layout(b), &caps()).expect("prepare");
    let (admitted, image) = admit_fin_genesis_image(
        prepared,
        &b,
        &layout(b),
        &RunIntC::Nat(8),
        &RunIntC::Nat(64),
        &request("open", scenario(), Some(UInt(2))),
    )
    .expect("actual financial genesis");
    crate::sherlock::helper_machine::complete_observation::capture_fin(&admitted, &image)
        .expect("financial capture")
}

fn expected_bounds_json(b: &BoundsC) -> serde_json::Value {
    serde_json::json!({
        "code_rows": b.code_rows.to_string(),
        "function_rows": b.function_rows.to_string(),
        "code_child_slots": b.code_child_slots.to_string(),
        "parameter_slots": b.parameter_slots.to_string(),
        "pattern_rows": b.pattern_rows.to_string(),
        "match_plan_rows": b.match_plan_rows.to_string(),
        "match_arm_rows": b.match_arm_rows.to_string(),
        "type_rows": b.type_rows.to_string(),
        "constructor_rows": b.constructor_rows.to_string(),
        "constructor_field_slots": b.constructor_field_slots.to_string(),
        "max_arity": b.max_arity.to_string(),
        "constant_nodes": b.constant_nodes.to_string(),
        "constant_field_slots": b.constant_field_slots.to_string(),
        "data_nodes": b.data_nodes.to_string(),
        "data_field_slots": b.data_field_slots.to_string(),
        "environment_cells": b.environment_cells.to_string(),
        "frames": b.frames.to_string(),
        "helper_tasks": b.helper_tasks.to_string(),
        "serialization_bytes": b.serialization_bytes.to_string(),
        "max_reference_work": b.max_reference_work.to_string(),
        "max_helper_work": b.max_helper_work.to_string(),
        "step_cap": b.step_cap.to_string(),
    })
}

fn expected_digest_json(d: &DigestC) -> serde_json::Value {
    match d {
        DigestC::Unselected => serde_json::json!("Unselected"),
        DigestC::Selected { format, bytes } => serde_json::json!({
            "Selected": { "format": format, "bytes": bytes },
        }),
    }
}

fn expected_scalar_json(v: &FinScalar) -> serde_json::Value {
    match v {
        FinScalar::UInt(n) => serde_json::json!({ "UInt": n.to_string() }),
        FinScalar::Bool(value) => serde_json::json!({ "Bool": value }),
    }
}

fn expected_fin_capture_json(
    captured: &crate::sherlock::helper_machine::complete_observation::FinHostSnapshot,
) -> serde_json::Value {
    let admission = captured.admission();
    let image = captured.image();
    let state = captured.state();
    let prepared = &admission.prepared;
    let bound = prepared.bound_source();
    let identity = &admission.identity;
    let actions = &prepared.fin_plan().actions;
    let action = &actions[state.action_index];
    let widths = match &admission.layout.widths {
        None => serde_json::Value::Null,
        Some(w) => serde_json::json!({
            "index_widths": w.index_widths,
            "counter_widths": w.counter_widths,
            "record_widths": w.record_widths,
        }),
    };
    let ext_capacities = match &admission.ext_capacities {
        None => serde_json::Value::Null,
        Some(e) => serde_json::json!({
            "plan_case_slots": e.plan_case_slots,
            "field_path_slots": e.field_path_slots,
            "pattern_child_slots": e.pattern_child_slots,
            "input_rows": e.input_rows,
            "asset_rows": e.asset_rows,
            "state_rows": e.state_rows,
            "account_rows": e.account_rows,
            "liability_rows": e.liability_rows,
            "await_site_rows": e.await_site_rows,
            "await_binder_slots": e.await_binder_slots,
            "balance_cells": e.balance_cells,
            "journal_rows": e.journal_rows,
            "pending_op_rows": e.pending_op_rows,
            "observations_per_invocation": e.observations_per_invocation,
            "max_lifetime_work": e.max_lifetime_work.to_string(),
            "resume_rows": e.resume_rows,
        }),
    };
    let mut inputs = Vec::with_capacity(state.inputs.len());
    for input in &state.inputs {
        inputs.push(serde_json::json!({
            "name": input.name,
            "ty": format!("{:?}", input.ty),
            "value": serde_json::to_value(&input.value).expect("expected input value"),
        }));
    }
    serde_json::json!({
        "format": "financial-genesis-capture/1",
        "identity": {
            "profile_id": identity.profile_id,
            "grammar_domain": format!("{:?}", identity.grammar_domain),
            "finance": format!("{:?}", identity.finance),
            "layout_version": identity.layout_version,
            "layout_digest": expected_digest_json(&identity.layout_digest),
            "bounds_digest": expected_digest_json(&identity.bounds_digest),
            "field_id": identity.field_id,
            "program_membership_digest": expected_digest_json(&identity.program_membership_digest),
        },
        "bounds": expected_bounds_json(&admission.bounds),
        "layout": {
            "layout_version": admission.layout.layout_version,
            "field_id": admission.layout.field_id,
            "widths": widths,
            "scalar_layout_id": admission.layout.scalar_layout_id,
            "bounds": expected_bounds_json(&admission.layout.bounds),
            "capacities": {
                "name_rows": admission.layout.capacities.name_rows.to_string(),
                "name_bytes": admission.layout.capacities.name_bytes.to_string(),
                "occurrence_path_slots": admission.layout.capacities.occurrence_path_slots.to_string(),
                "value_slots": admission.layout.capacities.value_slots.to_string(),
            },
            "ext_capacities": ext_capacities,
            "boundary_capacities_present": admission.layout.boundary_capacities_present,
        },
        "prepared": {
            "program": serde_json::to_value(prepared.program()).expect("expected program"),
            "stage_plan": serde_json::to_value(prepared.stage_plan()).expect("expected stage plan"),
            "program_hash": prepared.program_hash().to_vec(),
            "plan_hash": prepared.plan_hash().to_vec(),
            "bound_names": bound.names(),
            "bound_states": bound.states().iter().map(|row| serde_json::json!({
                "row": row.row.0,
                "name_id": row.name.0,
                "text": row.text,
                "ty": format!("{:?}", row.ty),
                "initial": expected_scalar_json(&row.initial),
            })).collect::<Vec<_>>(),
            "bound_assets": bound.assets().iter().map(|row| serde_json::json!({
                "row": row.row.0,
                "name_id": row.name.0,
                "text": row.text,
                "identity": {
                    "network": row.identity.network,
                    "issuer": row.identity.issuer,
                    "reference": row.identity.reference,
                    "scale": row.identity.scale,
                },
            })).collect::<Vec<_>>(),
            "bound_actions": bound.actions().iter().map(|row| serde_json::json!({
                "name_id": row.name.0,
                "text": row.text,
            })).collect::<Vec<_>>(),
            "bound_capacities": {
                "max_states": bound.capacities().max_states,
                "max_assets": bound.capacities().max_assets,
                "max_balances": bound.capacities().max_balances,
                "max_liabilities": bound.capacities().max_liabilities,
            },
            "fin_plan": {
                "actions": actions.iter().map(|row| serde_json::json!({
                    "name": row.name,
                    "segment_src_cost": row.segment_src_cost.iter().map(|n| n.to_string()).collect::<Vec<_>>(),
                    "committed_before": row.committed_before.iter().map(|n| n.to_string()).collect::<Vec<_>>(),
                    "plan_total": row.plan_total.to_string(),
                })).collect::<Vec<_>>(),
            },
        },
        "genesis": {
            "action_index": state.action_index,
            "action": action.name,
            "inputs": inputs,
            "snapshot": {
                "states": state.snapshot.states().iter().map(expected_scalar_json).collect::<Vec<_>>(),
                "balances": state.snapshot.balances().iter().map(|row| serde_json::json!({
                    "asset_row": row.asset.0, "account": row.account, "amount": row.amount.to_string(),
                })).collect::<Vec<_>>(),
                "liabilities": state.snapshot.liabilities().iter().map(|row| serde_json::json!({
                    "name": row.name, "asset_row": row.asset.0, "debtor": row.debtor,
                    "creditor": row.creditor, "amount": row.amount.to_string(),
                })).collect::<Vec<_>>(),
            },
            "core_snapshot": serde_json::to_value(&state.core_snapshot).expect("expected core snapshot"),
            "cap": state.cap.map(|cap| cap.0.to_string()),
            "committed": state.committed.0.to_string(),
        },
        "image": {
            "run_id": format!("{:?}", image.run_id),
            "control": format!("{:?}", image.control),
            "control_origin": image.control_origin.to_string(),
            "env_root": format!("{:?}", image.env_root),
            "stack_limit": image.stack_limit.to_string(),
            "outcome": format!("{:?}", image.outcome),
            "initial_fuel": image.initial_fuel.to_string(),
            "remaining_work": image.remaining_work.to_string(),
            "consumed_work": image.consumed_work.to_string(),
            "helper_work": image.helper_work.to_string(),
            "helper_visits": image.helper_visits.to_string(),
            "reference_cursor": image.reference_cursor.to_string(),
            "alloc": format!("{:?}", image.alloc),
            "active": format!("{:?}", image.active),
            "seg_base": {
                "active": format!("{:?}", image.seg_base.active()),
                "alloc": format!("{:?}", image.seg_base.alloc()),
                "consumed_work": format!("{:?}", image.seg_base.consumed_work()),
            },
            "helper": format!("{:?}", image.helper),
            "result": format!("{:?}", image.result),
            "data": format!("{:?}", image.data),
            "data_fields": format!("{:?}", image.data_fields),
            "env": format!("{:?}", image.env),
            "frames": format!("{:?}", image.frames),
            "tasks": format!("{:?}", image.tasks),
            "value_slots": format!("{:?}", image.value_slots),
        },
    })
}

#[test]
fn actual_owner_admission_genesis_bound_observation_capture_and_json() {
    let b = bounds();
    let prepared = prepare_fin_staged_source(SRC, &layout(b), &caps()).expect("prepare");
    let program_hash = prepared.program_hash();
    let plan_hash = prepared.plan_hash();
    let (admitted, image) = admit_fin_genesis_image(
        prepared,
        &b,
        &layout(b),
        &RunIntC::Nat(8),
        &RunIntC::Nat(64),
        &request("open", scenario(), Some(UInt(2))),
    )
    .expect("actual financial genesis");
    let obs = observe_fin_bound_image(&admitted, &image).expect("bound financial observation");
    let state = obs.image.state;
    assert_eq!(state.action_index, 0);
    assert_eq!(state.inputs.len(), 1);
    assert_eq!(state.inputs[0].name, "amount");
    assert_eq!(state.inputs[0].ty, crate::Type::UInt);
    assert_eq!(state.inputs[0].value, Value::UInt(UInt(7)));
    assert_eq!(
        state.snapshot.states(),
        [FinScalar::UInt(0), FinScalar::Bool(false)]
    );
    assert_eq!(state.snapshot.balances().len(), 2);
    assert_eq!(state.snapshot.balances()[0].asset, FinAssetRowId(0));
    assert_eq!(state.snapshot.balances()[0].account, "alice");
    assert_eq!(state.snapshot.balances()[0].amount, 10);
    assert_eq!(state.snapshot.balances()[1].account, "bob");
    assert_eq!(state.snapshot.balances()[1].amount, 3);
    assert_eq!(state.snapshot.liabilities().len(), 1);
    assert_eq!(state.snapshot.liabilities()[0].name, "due");
    assert_eq!(state.snapshot.liabilities()[0].amount, 0);
    assert_eq!(
        state.core_snapshot.state,
        BTreeMap::from([
            ("count".to_string(), Value::UInt(UInt(0))),
            ("flag".to_string(), Value::Bool(false)),
        ])
    );
    assert_eq!(state.cap, Some(UInt(2)));
    assert_eq!(state.committed, UInt(0));
    assert_eq!(obs.admission.prepared.program_hash(), program_hash);
    assert_eq!(obs.admission.prepared.plan_hash(), plan_hash);
    assert_eq!(obs.admission.prepared.fin_plan().actions[0].plan_total, 2);
    assert_eq!(obs.admission.layout.bounds, b);
    assert_eq!(obs.image.control, &ControlC::Eval { expr: 0 });
    assert_eq!(obs.image.outcome, &OutcomeC::Running);
    assert_eq!(obs.image.initial_fuel, 8);
    assert_eq!(obs.image.remaining_work, 8);
    assert_eq!(obs.image.consumed_work, 0);
    assert!(obs.image.frames.is_empty());
    assert!(obs.image.tasks.is_empty());
    assert!(obs.image.data.is_empty());
    assert!(obs.image.env.is_empty());
    assert!(obs.image.value_slots.is_empty());

    let captured =
        crate::sherlock::helper_machine::complete_observation::capture_fin(&admitted, &image)
            .expect("financial capture");
    assert_eq!(captured.admission().prepared.program_hash(), program_hash);
    assert_eq!(captured.admission().prepared.plan_hash(), plan_hash);
    assert_eq!(captured.admission().layout.bounds, b);
    assert_eq!(captured.state().snapshot, state.snapshot);
    assert_eq!(captured.image().control, ControlC::Eval { expr: 0 });
    let text = crate::sherlock::helper_machine::complete_observation::fin_capture_json(&captured)
        .expect("financial capture JSON");
    let value: serde_json::Value =
        serde_json::from_str(&text).expect("financial capture JSON text");
    assert_eq!(value["genesis"]["action"], "open");
    assert_eq!(value["genesis"]["inputs"][0]["value"]["value"], "7");
    assert_eq!(value["genesis"]["snapshot"]["balances"][0]["amount"], "10");
    assert_eq!(value["genesis"]["snapshot"]["balances"][1]["amount"], "3");
    assert_eq!(
        value["genesis"]["snapshot"]["liabilities"][0]["amount"],
        "0"
    );
    assert_eq!(value["genesis"]["cap"], "2");
    assert_eq!(value["genesis"]["committed"], "0");
    assert_eq!(
        value["prepared"]["fin_plan"]["actions"][0]["plan_total"],
        "2"
    );
}

#[test]
fn actual_admission_configuration_and_rebind_precedence() {
    let b = bounds();
    let engine = super::owner_host::host_owner_engine();
    let prepared = prepare_fin_staged_source(SRC, &layout(b), &caps()).expect("prepare");
    match engine.admit_fin(prepared, &BoundsC::default(), &layout(BoundsC::default())) {
        Err(Tooling::CapacityNotAdmitted {
            kind,
            needed,
            limit,
            ..
        }) => {
            assert_eq!(kind, CapacityKind::StepCap);
            assert_eq!(needed, NeededNatC::Exact(1));
            assert_eq!(limit, 0);
        }
        Err(_) => panic!("expected step-cap tooling refusal"),
        Ok(_) => panic!("expected step-cap refusal"),
    }
    let prepared = prepare_fin_staged_source(SRC, &layout(b), &caps()).expect("prepare");
    let mut small_rows = layout(b);
    small_rows.capacities.name_rows = 2;
    match engine.admit_fin(prepared, &b, &small_rows) {
        Err(Tooling::CapacityNotAdmitted {
            kind,
            needed,
            limit,
            ..
        }) => {
            assert_eq!(kind, CapacityKind::NameRows);
            assert_eq!(needed, NeededNatC::Exact(5));
            assert_eq!(limit, 2);
        }
        Err(_) => panic!("expected name-rows tooling refusal"),
        Ok(_) => panic!("expected name-rows refusal"),
    }
    let prepared = prepare_fin_staged_source(SRC, &layout(b), &caps()).expect("prepare");
    let mut small_bytes = layout(b);
    small_bytes.capacities.name_bytes = 4;
    match engine.admit_fin(prepared, &b, &small_bytes) {
        Err(Tooling::CapacityNotAdmitted {
            kind,
            needed,
            limit,
            ..
        }) => {
            assert_eq!(kind, CapacityKind::NameBytes);
            assert_eq!(needed, NeededNatC::Exact(33));
            assert_eq!(limit, 4);
        }
        Err(_) => panic!("expected name-bytes tooling refusal"),
        Ok(_) => panic!("expected name-bytes refusal"),
    }
    let prepared = prepare_fin_staged_source(SRC, &layout(b), &caps()).expect("prepare");
    let mut width_layout = layout(b);
    width_layout.widths = Some(
        WidthTable::new_validated(Vec::new(), vec![("WorkCounter".to_string(), 0)], Vec::new())
            .expect("width table"),
    );
    match engine.admit_fin(prepared, &b, &width_layout) {
        Err(Tooling::CapacityNotAdmitted {
            kind,
            needed,
            limit,
            ..
        }) => {
            assert_eq!(
                kind,
                CapacityKind::CounterWidth {
                    field: super::CounterWidthField::WorkCounter,
                }
            );
            assert_eq!(needed, NeededNatC::Exact(11));
            assert_eq!(limit, 0);
        }
        Err(_) => panic!("expected counter-width tooling refusal"),
        Ok(_) => panic!("expected counter-width refusal"),
    }
    let prepared = prepare_fin_staged_source(SRC, &layout(b), &caps()).expect("prepare");
    let mut boundary_layout = layout(b);
    boundary_layout.boundary_capacities_present = true;
    match engine.admit_fin(prepared, &b, &boundary_layout) {
        Err(Tooling::UnsupportedFeature { feature, .. }) => {
            assert_eq!(feature, super::FeatureKind::BoundarySection);
        }
        Err(_) => panic!("expected boundary tooling refusal"),
        Ok(_) => panic!("expected boundary refusal"),
    }
}

#[test]
fn actual_genesis_refusal_precedence_and_run_input_contract() {
    let admit = |action: &str, scenario: Scenario, cap: Option<UInt>| {
        let b = bounds();
        let prepared = prepare_fin_staged_source(SRC, &layout(b), &caps()).expect("prepare");
        admit_fin_genesis_image(
            prepared,
            &b,
            &layout(b),
            &RunIntC::Nat(8),
            &RunIntC::Nat(64),
            &request(action, scenario, cap),
        )
    };
    match admit("absent", Scenario::default(), None) {
        Err(FinRunError::Genesis(FinGenesisError::UnknownAction(name))) => {
            assert_eq!(name, "absent")
        }
        Err(_) => panic!("expected unknown-action genesis refusal"),
        Ok(_) => panic!("expected unknown-action refusal"),
    }
    let mut missing = scenario();
    missing.inputs.clear();
    match admit("open", missing, None) {
        Err(FinRunError::Genesis(FinGenesisError::Scenario(e))) => {
            assert_eq!(e.code, "mori::core::input")
        }
        Err(_) => panic!("expected input genesis refusal"),
        Ok(_) => panic!("expected input refusal"),
    }
    let mut bad_asset = scenario();
    bad_asset.balances[0].asset.network = "other".into();
    match admit("open", bad_asset, None) {
        Err(FinRunError::Genesis(FinGenesisError::Scenario(e))) => {
            assert_eq!(e.code, "mori::core::asset")
        }
        Err(_) => panic!("expected asset genesis refusal"),
        Ok(_) => panic!("expected asset refusal"),
    }
    match admit("open", scenario(), Some(UInt(1))) {
        Err(FinRunError::Genesis(FinGenesisError::CapExceeded { plan_total, cap })) => {
            assert_eq!((plan_total, cap), (2, 1));
        }
        Err(_) => panic!("expected cap genesis refusal"),
        Ok(_) => panic!("expected cap refusal"),
    }
    let b = bounds();
    let prepared = prepare_fin_staged_source(SRC, &layout(b), &caps()).expect("prepare");
    let engine = super::owner_host::host_owner_engine();
    let admitted = engine
        .admit_fin(prepared, &b, &layout(b))
        .expect("admit fin");
    match engine.admit_fin_run(
        &admitted,
        &RunIntC::Negative,
        &RunIntC::Nat(64),
        &request("open", scenario(), Some(UInt(2))),
    ) {
        Err(FinRunError::Genesis(FinGenesisError::GenesisInvalid)) => {}
        Err(_) => panic!("expected genesis-invalid refusal"),
        Ok(_) => panic!("expected genesis-invalid refusal"),
    }
    match engine.admit_fin_run(
        &admitted,
        &RunIntC::NatAboveU128,
        &RunIntC::Nat(64),
        &request("open", scenario(), Some(UInt(2))),
    ) {
        Err(FinRunError::Tooling(Tooling::CapacityNotAdmitted { kind, needed, .. })) => {
            assert_eq!(kind, CapacityKind::MaxReferenceWork);
            assert_eq!(needed, NeededNatC::AboveU128);
        }
        Err(_) => panic!("expected run-tooling refusal"),
        Ok(_) => panic!("expected run-tooling refusal"),
    }
}

#[test]
fn actual_public_entries_and_consumers_refuse_alternate_paths() {
    let (admitted, mut image) = genesis();
    let engine = super::owner_host::host_owner_engine();
    for finance in [FinanceInput::Absent, FinanceInput::TypedExtensionReserved] {
        match engine.admit_run(&admitted, &RunIntC::Nat(8), &RunIntC::Nat(64), &finance) {
            Err(Tooling::UnsupportedFeature { feature, locator }) => {
                assert_eq!(feature, super::FeatureKind::FinancialInput);
                assert_eq!(locator, None);
            }
            Err(_) => panic!("expected financial-input tooling refusal"),
            Ok(_) => panic!("public admit_run manufactured an alternate image"),
        }
    }
    let pre = super::PreImage::bind(&admitted, &image).expect("bind");
    match engine.select(&pre) {
        Err(e) => assert_eq!(
            e.kind,
            HostViolationKind::FinancialUnsupportedBeyondGenesis {
                op: FinUnsupportedOp::Select
            }
        ),
        Ok(_) => panic!("expected select refusal"),
    }
    match engine.check_step_inv(&pre) {
        Err(v) => {
            assert_eq!(v.len(), 1);
            assert_eq!(v[0].clause, StepInvClauseId::SI01);
        }
        Ok(()) => panic!("financial StepInv must not report success"),
    }
    match super::owner_host::observe_bound_image(&admitted, &image) {
        Err(e) => assert_eq!(
            e.kind,
            HostViolationKind::FinancialUnsupportedBeyondGenesis {
                op: FinUnsupportedOp::ObservePure
            }
        ),
        Ok(_) => panic!("expected pure-observation refusal"),
    }
    match crate::sherlock::helper_machine::complete_observation::capture(&admitted, &image) {
        Err(e) => assert_eq!(
            e.kind,
            HostViolationKind::FinancialUnsupportedBeyondGenesis {
                op: FinUnsupportedOp::Capture
            }
        ),
        Ok(_) => panic!("expected capture refusal"),
    }
    let limits = crate::sherlock::helper_machine::c6_phys_host::C6HostLimits {
        max_total_lanes: 1_000_000,
        max_state_lanes: 1_000_000,
        max_metadata_bytes: 1_000_000,
        max_string_bytes: 65_536,
        max_width_entries: 4096,
    };
    match crate::sherlock::helper_machine::c6_phys_host::capture_host(
        &admitted,
        &image,
        &layout(bounds()),
        crate::sherlock::helper_machine::c6_phys::C6CodecBudgets {
            type_expr_nodes: 4096,
            template_bytes: 4096,
        },
        &limits,
    ) {
        Err(crate::sherlock::helper_machine::c6_phys_host::C6HostCaptureError::Observation(e)) => {
            assert_eq!(
                e.kind,
                HostViolationKind::FinancialUnsupportedBeyondGenesis {
                    op: FinUnsupportedOp::Codec
                }
            );
        }
        Err(_) => panic!("expected C6 observation refusal"),
        Ok(_) => panic!("expected C6 codec refusal"),
    }
    let _ = &mut image;
}

#[test]
fn actual_step_and_run_refuse_beyond_genesis_and_preserve_every_genesis_field() {
    let (admitted, mut image) = genesis();
    let before_fin = image.fin.clone().expect("fin state");
    let before_run_id = (*image.run_id).clone();
    let engine = super::owner_host::host_owner_engine();
    let step_error = match engine.step(&admitted, &mut image) {
        Err(e) => e,
        Ok(_) => panic!("step must refuse"),
    };
    assert_eq!(
        step_error.kind,
        HostViolationKind::FinancialUnsupportedBeyondGenesis {
            op: FinUnsupportedOp::Step
        }
    );
    let mut events = 0usize;
    let run_error = match engine.run(&admitted, &mut image, &mut |_| events += 1) {
        Err(e) => e,
        Ok(_) => panic!("run must refuse"),
    };
    assert_eq!(
        run_error.kind,
        HostViolationKind::FinancialUnsupportedBeyondGenesis {
            op: FinUnsupportedOp::Run
        }
    );
    assert_eq!(events, 0);
    let after_fin = image.fin.as_ref().expect("fin state");
    assert_eq!(after_fin.action_index, before_fin.action_index);
    assert_eq!(after_fin.inputs, before_fin.inputs);
    assert_eq!(after_fin.snapshot, before_fin.snapshot);
    assert_eq!(after_fin.core_snapshot, before_fin.core_snapshot);
    assert_eq!(after_fin.cap, before_fin.cap);
    assert_eq!(after_fin.committed, before_fin.committed);
    assert_eq!(*image.run_id, before_run_id);
    assert_eq!(image.stack_limit, 64);
    assert_eq!(
        image.seg_base(),
        super::host_implied_prefix::SegBase::genesis_uncut()
    );
    assert_eq!(image.alloc, AllocCounters::default());
    assert_eq!(image.active, ActiveLengths::default());
    assert_eq!(image.control, ControlC::Eval { expr: 0 });
    assert_eq!(image.control_origin, 0);
    assert_eq!(image.env_root, None);
    assert_eq!(image.outcome, OutcomeC::Running);
    assert_eq!(image.initial_fuel, 8);
    assert_eq!(image.remaining_work, 8);
    assert_eq!(image.consumed_work, 0);
    assert_eq!(image.helper_work, 0);
    assert_eq!(image.helper_visits, 0);
    assert_eq!(image.reference_cursor, 0);
    assert_eq!(image.helper, None);
    assert_eq!(image.result, None);
    assert!(image.data.is_empty());
    assert!(image.data_fields.is_empty());
    assert!(image.env.is_empty());
    assert!(image.frames.is_empty());
    assert!(image.tasks.is_empty());
    assert!(image.value_slots.is_empty());
}

#[test]
fn financial_capture_retains_ext_capacities_and_seg_base() {
    let b = bounds();
    let ext = ExtCapacitiesC {
        plan_case_slots: 11,
        field_path_slots: 12,
        pattern_child_slots: 13,
        input_rows: 14,
        asset_rows: 15,
        state_rows: 16,
        account_rows: 17,
        liability_rows: 18,
        await_site_rows: 19,
        await_binder_slots: 20,
        balance_cells: 21,
        journal_rows: 22,
        pending_op_rows: 23,
        observations_per_invocation: 24,
        max_lifetime_work: 25,
        resume_rows: 26,
    };
    let mut retained = layout(b);
    retained.ext_capacities = Some(ext);
    let prepared = prepare_fin_staged_source(SRC, &retained, &caps()).expect("prepare");
    let (admitted, image) = admit_fin_genesis_image(
        prepared,
        &b,
        &retained,
        &RunIntC::Nat(8),
        &RunIntC::Nat(64),
        &request("open", scenario(), Some(UInt(2))),
    )
    .expect("actual financial genesis");
    let captured =
        crate::sherlock::helper_machine::complete_observation::capture_fin(&admitted, &image)
            .expect("financial capture");
    assert_eq!(captured.admission().ext_capacities, Some(ext));
    assert_eq!(captured.admission().layout.widths, None);
    let text = crate::sherlock::helper_machine::complete_observation::fin_capture_json(&captured)
        .expect("financial capture JSON");
    let value: serde_json::Value =
        serde_json::from_str(&text).expect("financial capture JSON text");
    assert_eq!(value["layout"]["widths"], serde_json::Value::Null);
    assert_eq!(value["layout"]["ext_capacities"]["plan_case_slots"], 11);
    assert_eq!(value["layout"]["ext_capacities"]["field_path_slots"], 12);
    assert_eq!(value["layout"]["ext_capacities"]["pattern_child_slots"], 13);
    assert_eq!(value["layout"]["ext_capacities"]["input_rows"], 14);
    assert_eq!(value["layout"]["ext_capacities"]["asset_rows"], 15);
    assert_eq!(value["layout"]["ext_capacities"]["state_rows"], 16);
    assert_eq!(value["layout"]["ext_capacities"]["account_rows"], 17);
    assert_eq!(value["layout"]["ext_capacities"]["liability_rows"], 18);
    assert_eq!(value["layout"]["ext_capacities"]["await_site_rows"], 19);
    assert_eq!(value["layout"]["ext_capacities"]["await_binder_slots"], 20);
    assert_eq!(value["layout"]["ext_capacities"]["balance_cells"], 21);
    assert_eq!(value["layout"]["ext_capacities"]["journal_rows"], 22);
    assert_eq!(value["layout"]["ext_capacities"]["pending_op_rows"], 23);
    assert_eq!(
        value["layout"]["ext_capacities"]["observations_per_invocation"],
        24
    );
    assert_eq!(value["layout"]["ext_capacities"]["max_lifetime_work"], "25");
    assert_eq!(value["layout"]["ext_capacities"]["resume_rows"], 26);
    assert!(value["image"]["seg_base"].is_object());
    assert!(value["image"]["seg_base"]["active"].is_string());
    assert!(value["image"]["seg_base"]["alloc"].is_string());
    assert!(value["image"]["seg_base"]["consumed_work"].is_string());
    assert_eq!(value["prepared"]["bound_states"][0]["text"], "count");
    assert_eq!(
        value["prepared"]["bound_states"][0]["initial"],
        serde_json::json!({ "UInt": "0" })
    );
    assert_eq!(value["prepared"]["bound_states"][1]["text"], "flag");
    assert_eq!(value["prepared"]["bound_assets"][0]["text"], "COIN");
    assert_eq!(value["prepared"]["bound_actions"][0]["text"], "open");
}

#[test]
fn financial_capture_sealed_read_only_observability_is_preserved() {
    let (admitted, image) = genesis();
    let captured =
        crate::sherlock::helper_machine::complete_observation::capture_fin(&admitted, &image)
            .expect("financial capture");
    assert_eq!(captured.state().action_index, 0);
    assert_eq!(captured.state().inputs.len(), 1);
    assert_eq!(captured.image().control, ControlC::Eval { expr: 0 });
    assert_eq!(captured.admission().bounds, bounds());
    assert!(
        crate::sherlock::helper_machine::complete_observation::fin_capture_json(&captured).is_ok()
    );
}

#[test]
fn pure_admission_admit_fin_run_returns_not_financial_admission() {
    let (admitted, _image) = pure_admitted(false);
    let engine = super::owner_host::host_owner_engine();
    match engine.admit_fin_run(
        &admitted,
        &RunIntC::Nat(8),
        &RunIntC::Nat(64),
        &request("open", scenario(), None),
    ) {
        Err(FinRunError::Genesis(FinGenesisError::NotFinancialAdmission)) => {}
        Err(_) => panic!("expected NotFinancialAdmission"),
        Ok(_) => panic!("pure admission manufactured a financial image"),
    }
}

#[test]
fn pure_admission_typed_extension_reserved_admit_run_refuses() {
    let (admitted, _image) = pure_admitted(false);
    let engine = super::owner_host::host_owner_engine();
    match engine.admit_run(
        &admitted,
        &RunIntC::Nat(8),
        &RunIntC::Nat(64),
        &FinanceInput::TypedExtensionReserved,
    ) {
        Err(Tooling::UnsupportedFeature { feature, .. }) => {
            assert_eq!(feature, super::FeatureKind::FinancialInput)
        }
        Err(_) => panic!("expected FinancialInput refusal"),
        Ok(_) => panic!("pure admission plus reserved marker manufactured an image"),
    }
}

#[test]
fn admitted_function_count_financial_typed_error_and_pure_exact() {
    let (fin_admitted, fin_image) = genesis();
    let fin_pre = super::PreImage::bind(&fin_admitted, &fin_image).expect("bind financial");
    match fin_pre.admitted_function_count() {
        Err(error) => assert_eq!(
            error.kind,
            HostViolationKind::FinancialUnsupportedBeyondGenesis {
                op: FinUnsupportedOp::StepInv
            }
        ),
        Ok(count) => panic!("financial count silently returned {count}"),
    }
    let (pure_zero_admitted, pure_zero_image) = pure_admitted(false);
    let pure_zero_pre =
        super::PreImage::bind(&pure_zero_admitted, &pure_zero_image).expect("bind pure");
    assert_eq!(
        pure_zero_pre.admitted_function_count().expect("pure count"),
        0
    );
    let (pure_fn_admitted, pure_fn_image) = pure_admitted(true);
    let pure_fn_pre =
        super::PreImage::bind(&pure_fn_admitted, &pure_fn_image).expect("bind pure function");
    assert_eq!(
        pure_fn_pre
            .admitted_function_count()
            .expect("pure function count"),
        1
    );
}

#[test]
fn record_width_layout_reaches_record_widths_admission_refusal() {
    let b = bounds();
    let mut retained = layout(b);
    retained.widths = Some(
        WidthTable::new_validated(
            [
                "ExprId",
                "ChildPos",
                "FunctionRowId",
                "FunctionCursor",
                "ParamPos",
                "ArmId",
                "ArmPos",
                "PatternRowId",
                "ConstId",
                "ConstFieldPos",
                "DataId",
                "FieldPos",
                "EnvId",
                "NameId",
                "OccPathPos",
                "ChildPosition",
                "ValueSlotPos",
                "ArityCount",
                "Depth",
                "TaskCount",
            ]
            .into_iter()
            .map(|name| (name.to_string(), 128))
            .collect(),
            ["WorkCounter", "HelperCounter", "ReferenceCursor"]
                .into_iter()
                .map(|name| (name.to_string(), 128))
                .collect(),
            Vec::new(),
        )
        .expect("width table"),
    );
    let prepared = prepare_fin_staged_source(SRC, &layout(b), &caps()).expect("prepare");
    let engine = super::owner_host::host_owner_engine();
    match engine.admit_fin(prepared, &b, &retained) {
        Err(Tooling::AdmissionRejected {
            reason,
            detail,
            locator,
        }) => {
            assert_eq!(reason, super::AdmissionReason::LayoutWidthTooSmall);
            assert_eq!(detail, "recordWidths");
            assert_eq!(
                locator,
                Some(LocatorC::Field {
                    name: "layout.widths.recordWidths".into()
                })
            );
        }
        Err(_) => panic!("expected recordWidths admission refusal"),
        Ok(_) => panic!("record-width layout was admitted"),
    }
}

#[test]
fn capture_and_c6_refuse_financial_before_clone_and_ab_first() {
    let (fin_admitted, fin_image) = genesis();
    match crate::sherlock::helper_machine::complete_observation::capture(&fin_admitted, &fin_image)
    {
        Err(error) => assert_eq!(
            error.kind,
            HostViolationKind::FinancialUnsupportedBeyondGenesis {
                op: FinUnsupportedOp::Capture
            }
        ),
        Ok(_) => panic!("financial pure capture succeeded"),
    }
    let limits = crate::sherlock::helper_machine::c6_phys_host::C6HostLimits {
        max_total_lanes: 1_000_000,
        max_state_lanes: 1_000_000,
        max_metadata_bytes: 1_000_000,
        max_string_bytes: 65_536,
        max_width_entries: 4096,
    };
    let budgets = crate::sherlock::helper_machine::c6_phys::C6CodecBudgets {
        type_expr_nodes: 4096,
        template_bytes: 4096,
    };
    match crate::sherlock::helper_machine::c6_phys_host::capture_host(
        &fin_admitted,
        &fin_image,
        &layout(bounds()),
        budgets,
        &limits,
    ) {
        Err(crate::sherlock::helper_machine::c6_phys_host::C6HostCaptureError::Observation(
            error,
        )) => {
            assert_eq!(
                error.kind,
                HostViolationKind::FinancialUnsupportedBeyondGenesis {
                    op: FinUnsupportedOp::Codec
                }
            );
        }
        Err(_) => panic!("expected C6 observation refusal"),
        Ok(_) => panic!("financial C6 capture succeeded"),
    }
    let (pure_admitted_value, pure_image) = pure_admitted(false);
    assert!(
        crate::sherlock::helper_machine::complete_observation::capture(
            &pure_admitted_value,
            &pure_image
        )
        .is_ok()
    );
    match crate::sherlock::helper_machine::complete_observation::capture(&fin_admitted, &pure_image)
    {
        Err(error) => {
            assert_eq!(error.kind, HostViolationKind::StepInvViolated);
            assert_eq!(error.clause, Some(StepInvClauseId::SI01));
        }
        Ok(_) => panic!("AB mismatch was treated as a capture refusal or success"),
    }
    match crate::sherlock::helper_machine::c6_phys_host::capture_host(
        &fin_admitted,
        &pure_image,
        &layout(bounds()),
        budgets,
        &limits,
    ) {
        Err(crate::sherlock::helper_machine::c6_phys_host::C6HostCaptureError::Observation(
            error,
        )) => {
            assert_eq!(error.kind, HostViolationKind::StepInvViolated);
            assert_eq!(error.clause, Some(StepInvClauseId::SI01));
        }
        Err(_) => panic!("expected C6 AB observation refusal"),
        Ok(_) => panic!("AB mismatch was treated as a C6 refusal or success"),
    }
}

#[test]
fn fin_capture_zero_insufficient_and_exact_serialization_budgets() {
    let working_limit = 1_000_000usize;
    let working = genesis_capture_with_limit(working_limit);
    let expected = expected_fin_capture_json(&working);
    let expected_text = serde_json::to_string(&expected).expect("expected JSON");
    let actual_text =
        crate::sherlock::helper_machine::complete_observation::fin_capture_json(&working)
            .expect("working-budget JSON");
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&actual_text).expect("actual JSON"),
        expected
    );
    assert_eq!(actual_text.len(), expected_text.len());
    let base = expected_text.len() - 2 * working_limit.to_string().len();
    let exact = (1usize..=3_000_000)
        .find(|candidate| *candidate == base + 2 * candidate.to_string().len())
        .expect("deterministic exact serialization budget");
    for limit in [0usize, 1, exact - 1] {
        let captured = genesis_capture_with_limit(limit);
        match crate::sherlock::helper_machine::complete_observation::fin_capture_json(&captured) {
            Err(crate::sherlock::helper_machine::complete_observation::FinCaptureJsonError::SerializationLimitExceeded { limit: got }) => assert_eq!(got, limit),
            Err(_) => panic!("expected serialization-limit refusal for {limit}"),
            Ok(_) => panic!("insufficient serialization budget {limit} succeeded"),
        }
    }
    let captured = genesis_capture_with_limit(exact);
    let text = crate::sherlock::helper_machine::complete_observation::fin_capture_json(&captured)
        .expect("exact-budget JSON");
    assert_eq!(text.len(), exact);
}
