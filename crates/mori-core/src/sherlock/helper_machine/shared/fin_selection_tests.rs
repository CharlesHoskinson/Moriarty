use super::fin_carrier::FinGenesisRequest;
use super::fin_selection::{FinRuleId, FinSelectedStatement, FinSelectionError, FinStatementKind};
use super::owner_host::{
    admit_fin_genesis_image, observe_fin_bound_image, prepare_fin_staged_source, select_fin_bound,
};
use super::{
    BoundsC, FinCapacitiesC, FinanceInput, HelperResultC, HostViolationKind, LayoutCapacitiesC,
    LayoutSchemaC, OutcomeC, OwnerEngine, RunIntC, StepInvClauseId,
};
use crate::{AssetId, Balance, Liability, Scenario, UInt};
use std::collections::BTreeMap;

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

fn asset(network: &str, issuer: &str, reference: &str, scale: u8) -> AssetId {
    AssetId {
        network: network.into(),
        issuer: issuer.into(),
        reference: reference.into(),
        scale,
    }
}

fn balance(account: &str, asset: AssetId, amount: u128) -> Balance {
    Balance {
        account: account.into(),
        asset,
        amount: UInt(amount),
    }
}

fn genesis_a() -> (super::Admitted, super::MachineImage) {
    let b = bounds();
    let unit = asset("local", "new-split-settlement-a", "UNIT", 0);
    let scenario = Scenario {
        state: BTreeMap::new(),
        inputs: BTreeMap::new(),
        balances: vec![
            balance("escrow", unit.clone(), 100),
            balance("beneficiary", unit.clone(), 0),
            balance("treasury", unit, 0),
        ],
        liabilities: Vec::new(),
    };
    let prepared = prepare_fin_staged_source(A_SRC, &layout(b), &caps()).expect("A");
    admit_fin_genesis_image(
        prepared,
        &b,
        &layout(b),
        &RunIntC::Nat(8),
        &RunIntC::Nat(64),
        &FinGenesisRequest {
            action: "release".into(),
            scenario,
            cap: None,
        },
    )
    .expect("A genesis")
}

fn genesis_b() -> (super::Admitted, super::MachineImage) {
    let b = bounds();
    let alt = asset("local", "new-refund-settlement-b", "ALT", 2);
    let scenario = Scenario {
        state: BTreeMap::new(),
        inputs: BTreeMap::new(),
        balances: vec![
            balance("vault", alt.clone(), 100),
            balance("recipient", alt.clone(), 0),
            balance("collector", alt, 0),
        ],
        liabilities: vec![Liability {
            name: "preexisting".into(),
            asset: asset("local", "new-refund-settlement-b", "ALT", 2),
            debtor: "vault".into(),
            creditor: "recipient".into(),
            amount: UInt(0),
        }],
    };
    let prepared = prepare_fin_staged_source(B_SRC, &layout(b), &caps()).expect("B");
    admit_fin_genesis_image(
        prepared,
        &b,
        &layout(b),
        &RunIntC::Nat(8),
        &RunIntC::Nat(64),
        &FinGenesisRequest {
            action: "settle".into(),
            scenario,
            cap: None,
        },
    )
    .expect("B genesis")
}

#[test]
fn mu2_a_genesis_selection_is_liability_literal17() {
    let (admitted, image) = genesis_a();
    let selection = select_fin_bound(&admitted, &image).expect("A select");
    assert_eq!(selection.header.format, "financial-selection/1");
    assert_eq!(selection.header.rule, FinRuleId::LiabilityApply);
    assert_eq!(selection.header.rule.retained_name(), "XFinLiabilityApply");
    assert_eq!(selection.header.statement_kind, FinStatementKind::Liability);
    assert_eq!(selection.header.action_name, "release");
    assert_eq!(selection.header.statement_cursor, 0);
    assert_eq!(selection.header.segment_index, 0);
    match &selection.statement {
        FinSelectedStatement::Liability {
            name,
            asset,
            debtor,
            creditor,
            amount_literal,
            amount_name,
            ..
        } => {
            assert_eq!(name, "due");
            assert_eq!(asset, "UNIT");
            assert_eq!(debtor, "escrow");
            assert_eq!(creditor, "beneficiary");
            assert_eq!(*amount_literal, Some(17));
            assert_eq!(*amount_name, None);
        }
        other => panic!("expected Liability selection, got {other:?}"),
    }
    let event = selection.event_header();
    assert_eq!(event.rule, FinRuleId::LiabilityApply);
    assert_eq!(event.boundary, super::BoundaryKind::None);
    assert_eq!(event.outcome, OutcomeC::Running);
}

#[test]
fn mu2_b_genesis_selection_is_let_literal9_not_liability() {
    let (admitted, image) = genesis_b();
    let selection = select_fin_bound(&admitted, &image).expect("B select");
    assert_eq!(selection.header.rule, FinRuleId::LetApply);
    assert_eq!(selection.header.rule.retained_name(), "XFinLetApply");
    assert_eq!(selection.header.statement_kind, FinStatementKind::Let);
    assert_eq!(selection.header.action_name, "settle");
    assert_ne!(selection.header.action_name, "release");
    match &selection.statement {
        FinSelectedStatement::Let {
            name,
            value_literal,
            value_name,
            ..
        } => {
            assert_eq!(name, "grant");
            assert_eq!(*value_literal, Some(9));
            assert_eq!(*value_name, None);
        }
        other => panic!("expected Let selection, got {other:?}"),
    }
}

#[test]
fn mu2_nonfirst_cursor_reads_actual_next_statement() {
    let (admitted, mut image) = genesis_a();
    image.fin.as_mut().expect("fin").exec.statement_cursor = 1;
    let selection = select_fin_bound(&admitted, &image).expect("A stmt1");
    assert_eq!(selection.header.rule, FinRuleId::TransferApply);
    assert_eq!(selection.header.statement_cursor, 1);
    match &selection.statement {
        FinSelectedStatement::Transfer {
            asset,
            from,
            to,
            amount_literal,
            ..
        } => {
            assert_eq!(asset, "UNIT");
            assert_eq!(from, "escrow");
            assert_eq!(to, "beneficiary");
            assert_eq!(*amount_literal, Some(6));
        }
        other => panic!("expected Transfer selection, got {other:?}"),
    }

    let (admitted, mut image) = genesis_b();
    image.fin.as_mut().expect("fin").exec.statement_cursor = 1;
    let selection = select_fin_bound(&admitted, &image).expect("B stmt1");
    assert_eq!(selection.header.rule, FinRuleId::LiabilityApply);
    match &selection.statement {
        FinSelectedStatement::Liability {
            name,
            asset,
            amount_literal,
            amount_name,
            ..
        } => {
            assert_eq!(name, "grant_due");
            assert_eq!(asset, "ALT");
            assert_eq!(*amount_literal, None);
            assert_eq!(amount_name.as_deref(), Some("grant"));
        }
        other => panic!("expected B Liability selection, got {other:?}"),
    }
}

#[test]
fn mu2_selection_plan_fields_come_from_one_sealed_prepared_authority() {
    // CE05 applicability to MU2: header plan values are read from the same
    // FinPreparedStagedSource carried by admission, whose FinPlan was
    // derived from its sealed StagePlan. No second caller plan is accepted.
    // The comparison uses the existing public readonly bound observation;
    // no test-only accessor is added.
    let (admitted, image) = genesis_a();
    let selection = select_fin_bound(&admitted, &image).expect("select");
    let obs = observe_fin_bound_image(&admitted, &image).expect("observe");
    let prepared = obs.admission.prepared;
    let header = &selection.header;

    // Selected pair itself: the observed financial state is the state the
    // header was read from.
    assert_eq!(obs.image.state.action_index, header.action_index);
    assert_eq!(
        obs.image.state.exec.statement_cursor,
        header.statement_cursor
    );
    assert_eq!(obs.image.state.exec.segment_index, header.segment_index);
    assert_eq!(obs.image.state.committed.0, header.committed_before);

    // Source correspondence through the sealed prepared source.
    let source_action = &prepared.program().actions[header.action_index];
    assert_eq!(header.action_name, source_action.name);
    assert_eq!(
        header.action_name,
        prepared.bound_source().actions()[header.action_index].text
    );
    assert!(matches!(
        &source_action.statements[header.statement_cursor],
        crate::Statement::Liability { .. }
    ));
    assert_eq!(header.statement_kind, FinStatementKind::Liability);

    // FinPlan correspondence: every plan field in the header is the
    // same-index field of the one sealed FinPlan.
    let fin_action = &prepared.fin_plan().actions[header.action_index];
    assert_eq!(header.action_name, fin_action.name);
    assert_eq!(
        header.committed_before,
        fin_action.committed_before[header.segment_index]
    );
    assert_eq!(
        header.segment_src_cost,
        fin_action.segment_src_cost[header.segment_index]
    );
    assert_eq!(header.plan_total, fin_action.plan_total);

    // StagePlan correspondence: the sealed stage plan from which that
    // FinPlan was derived carries the same action, costs and total.
    let stage_action = &prepared.stage_plan().actions[header.action_index];
    assert_eq!(stage_action.action, fin_action.name);
    assert_eq!(
        usize::from(stage_action.statement_count),
        source_action.statements.len()
    );
    assert_eq!(
        stage_action.committed_before[header.segment_index].0,
        header.committed_before
    );
    assert_eq!(
        stage_action.segment_cost[header.segment_index].0,
        header.segment_src_cost
    );
    assert_eq!(stage_action.plan_total.0, header.plan_total);
}

#[test]
fn mu2_wrong_admission_binding_precedes_financial_selection() {
    let (admitted_a, _image_a) = genesis_a();
    let (_admitted_b, image_b) = genesis_b();
    match select_fin_bound(&admitted_a, &image_b) {
        Err(FinSelectionError::Binding(error)) => {
            assert_eq!(error.kind, HostViolationKind::StepInvViolated);
            assert_eq!(error.clause, Some(StepInvClauseId::SI01));
        }
        Err(other) => panic!("expected binding refusal, got {other:?}"),
        Ok(_) => panic!("AB mismatch selected a financial statement"),
    }
}

#[test]
fn mu2_out_of_range_action_segment_and_cursor_refuse_without_mutation() {
    let (admitted, mut image) = genesis_b();
    image.fin.as_mut().expect("fin").action_index = 99;
    let before_attempt = image.fin.clone();
    assert!(matches!(
        select_fin_bound(&admitted, &image),
        Err(FinSelectionError::Invariant(
            super::fin_exec_state::FinExecInvariantError::PlanActionMissing { action_index: 99 }
        ))
    ));
    assert_eq!(image.fin, before_attempt);

    // K4: image2 and admitted2 are the pair returned by one producer
    // call. The previous shape paired image2 with the outer admitted
    // and therefore refused at PreImage::bind (SI01) before any segment
    // invariant could be reached.
    let (admitted2, mut image2) = genesis_b();
    image2.fin.as_mut().expect("fin").exec.segment_index = 99;
    let before_attempt2 = image2.fin.clone();
    assert!(matches!(
        select_fin_bound(&admitted2, &image2),
        Err(FinSelectionError::Invariant(
            super::fin_exec_state::FinExecInvariantError::SegmentOutOfRange { .. }
        ))
    ));
    assert_eq!(image2.fin, before_attempt2);

    // K5: B settle has exactly 11 source statements, counted from the
    // unchanged B_SRC above: let, liability, transfer, fee, discharge,
    // await, transfer, transfer, discharge, next, emit.
    let (admitted, mut image) = genesis_b();
    image.fin.as_mut().expect("fin").exec.statement_cursor = 99;
    let before_failed = image.fin.clone();
    assert!(matches!(
        select_fin_bound(&admitted, &image),
        Err(FinSelectionError::Invariant(
            super::fin_exec_state::FinExecInvariantError::CursorOutOfRange {
                cursor: 99,
                statement_count: 11
            }
        ))
    ));
    assert_eq!(image.fin, before_failed);
}

#[test]
fn mu2_pending_result_is_refused_before_statement_read() {
    // CE11 applicability to MU2: a stale staged result competes with the
    // statement at cursor; MU2 has no result-consumption rule, so it must
    // refuse rather than fall through to the statement or an old family.
    // FinSelectionError has no PartialEq (it carries the Debug-only
    // HostViolation), so the exact typed refusal is asserted by variant.
    let (admitted, mut image) = genesis_b();
    image.result = Some(HelperResultC::EnvMissing);
    let before = image.fin.clone();
    assert!(matches!(
        select_fin_bound(&admitted, &image),
        Err(FinSelectionError::PendingHelperOrResult)
    ));
    assert_eq!(image.fin, before);
}

#[test]
fn mu2_non_running_image_is_refused_before_statement_read() {
    // K7: select_fin accepts only live Running images. The Running value
    // in FinSelection::event_header is a prospective declared label for
    // a selection produced inside that bounded domain; it is not an
    // observed outcome and no StepInv claim is made here.
    let (admitted, mut image) = genesis_a();
    image.outcome = OutcomeC::Rejected { code: 1 };
    let before = image.fin.clone();
    assert!(matches!(
        select_fin_bound(&admitted, &image),
        Err(FinSelectionError::NonRunningOutcome {
            outcome: OutcomeC::Rejected { code: 1 }
        })
    ));
    assert_eq!(image.fin, before);
}

#[test]
fn mu2_pure_image_and_financial_step_remain_distinct() {
    let engine = super::owner_host::host_owner_engine();
    let (admitted, mut image) = genesis_a();
    match engine.step(&admitted, &mut image) {
        Err(error) => assert_eq!(
            error.kind,
            HostViolationKind::FinancialUnsupportedBeyondGenesis {
                op: super::FinUnsupportedOp::Step
            }
        ),
        Ok(_) => panic!("MU2 selection must not become Step/apply"),
    }
    let _ = FinanceInput::Absent;
}
