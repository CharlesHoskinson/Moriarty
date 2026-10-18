//! Normal literal-event/API tests, SOURCE_ONLY/UNRUN.
//! These events are hand-stated typed fixtures, not captured/certified engine
//! histories. No private O, engine, interpreter or source evaluator is used.
use super::*;
use std::sync::Arc;
use crate::sherlock::helper_machine::shared::{AllocDeltasC, BoundaryKind, DigestC,
    EventKind, ExpandedStepIdC, FinanceInput, GrammarDomain, IdentityC, LocatorC,
    OutcomeC, OutcomeTagC, RuleIdC, RunIdC, RunIntC, StepDisposition};

fn event(ordinal: u128, work: u8, helper: u8, visits: u8, cursor: u8,
    before: usize, after: usize) -> StepEventC
{
    // Metadata unused by the observer is explicit test data, not a claim that
    // a selected owner rule/plan produced these hand-stated relevant fields.
    let identity = IdentityC { profile_id: 2, grammar_domain: GrammarDomain::DedicatedSorts,
        finance: FinanceInput::Absent,
        layout_version: ("moriarty".into(), "sherlock-helper-microstep-hostonly/1".into()),
        layout_digest: DigestC::Unselected, bounds_digest: DigestC::Unselected,
        field_id: None, program_membership_digest: DigestC::Unselected };
    let run_id = Arc::new(RunIdC { identity, fuel_input: RunIntC::Nat(1000), stack_limit_input: RunIntC::Nat(32) });
    StepEventC { step_id: ExpandedStepIdC { run_id, ordinal },
        rule_id: RuleIdC::KMachineEvalLet, event_kind: EventKind::LegacyComplete,
        site: None, helper_kind: None, disposition: StepDisposition::Applied,
        work_delta: work, helper_work_delta: helper, helper_visits_delta: visits,
        reference_cursor_delta: cursor,
        alloc_deltas: AllocDeltasC { data_nodes: 0, data_fields: 0, environment_cells: 0 },
        value_slot_writes: 0, logical_depth_before: before, logical_depth_after: after,
        task_cursor_before: 0, task_cursor_after: 0,
        helper_phase_before: None, helper_phase_after: None,
        helper_cursor_before: None, helper_cursor_after: None, result_tag: None,
        origin_reference_cursor: None, source_expr_id: 0,
        source_occurrence: LocatorC::Field { name: "literal-event-fixture".into() },
        exhausted_kind: None, outcome_tag: OutcomeTagC::Running,
        outcome_payload: OutcomeC::Running, boundary: BoundaryKind::None }
}
fn exact(value: u128) -> CheckedTotal { CheckedTotal::Exact { value: Counter(value) } }

#[test]
fn zero_events_preserve_the_call_result_without_inventing_depth() {
    let mut deliveries = 0usize;
    let mut sink = |_event: &StepEventC| deliveries += 1;
    let (result, report) = collect_forwarded(&mut sink, |_forward| Ok::<_, ()>("original"));
    assert_eq!(result, Ok("original"));
    assert_eq!(deliveries, 0);
    assert_eq!(report.prefix_end, PrefixEnd::ReturnedOk);
    assert_eq!(report.event_count, exact(0));
    assert_eq!(report.maximum_observed_logical_depth, None);
    let json = serde_json::to_value(report).unwrap();
    assert!(json["maximum_observed_logical_depth"].is_null());
}

#[test]
fn multiple_events_keep_once_order_delivery_and_distinct_raw_delta_columns() {
    let events = [event(0, 1, 0, 0, 1, 0, 1), event(1, 1, 1, 1, 0, 1, 3),
        event(2, 0, 0, 0, 0, 3, 2)];
    let mut delivered = Vec::new();
    let mut sink = |event: &StepEventC| delivered.push(event.step_id.ordinal);
    let (result, report) = collect_forwarded(&mut sink, |forward| {
        for event in &events { forward(event); }
        Ok::<_, ()>(17u8)
    });
    assert_eq!(result, Ok(17));
    assert_eq!(delivered, vec![0, 1, 2]);
    assert_eq!(report.event_count, exact(3));
    assert_eq!(report.work_delta_sum, exact(2));
    assert_eq!(report.helper_work_delta_sum, exact(1));
    assert_eq!(report.helper_visits_delta_sum, exact(1));
    assert_eq!(report.reference_cursor_delta_sum, exact(1));
    assert_eq!(report.maximum_observed_logical_depth, Some(3));
    assert_eq!(report.arithmetic_status, ArithmeticStatus::Exact);
}

#[test]
fn an_original_error_keeps_the_delivered_partial_prefix_and_error_object() {
    struct Marker { tag: u8 }
    let error = Box::new(Marker { tag: 7 });
    let original_address = error.as_ref() as *const Marker;
    let events = [event(0, 1, 0, 0, 1, 2, 4), event(1, 1, 1, 1, 0, 4, 3)];
    let mut delivered = Vec::new();
    let mut sink = |event: &StepEventC| delivered.push(event.step_id.ordinal);
    let (result, report) = collect_forwarded(&mut sink, |forward| {
        for event in &events { forward(event); }
        Err::<(), _>(error)
    });
    let returned = result.err().unwrap();
    assert_eq!(returned.tag, 7);
    assert_eq!(returned.as_ref() as *const Marker, original_address);
    assert_eq!(delivered, vec![0, 1]);
    assert_eq!(report.prefix_end, PrefixEnd::ReturnedErr);
    assert_eq!(report.event_count, exact(2));
    assert_eq!(report.maximum_observed_logical_depth, Some(4));
}

#[test]
fn u128_arithmetic_is_lossless_and_overflow_is_an_explicit_incomplete_prefix() {
    // Test-only seeded diagnostic state E03. This is not imported owner state
    // and does not assert a real prior event history of this length.
    let mut accounting = EventAccounting {
        event_count: exact(u128::MAX - 1), work_delta_sum: exact(u128::MAX - 1),
        helper_work_delta_sum: exact(0), helper_visits_delta_sum: exact(0),
        reference_cursor_delta_sum: exact(0), maximum_observed_logical_depth: None,
    };
    accounting.observe(&event(0, 1, 1, 1, 0, 1, 2));
    let before_overflow = accounting.snapshot(PrefixEnd::Open);
    let json = serde_json::to_value(before_overflow.event_count).unwrap();
    assert_eq!(json["status"], "exact");
    assert_eq!(json["value"], u128::MAX.to_string());
    accounting.observe(&event(1, 1, 0, 0, 1, 2, 3));
    let report = accounting.snapshot(PrefixEnd::ReturnedErr);
    assert_eq!(report.event_count, CheckedTotal::Overflow { exact_prefix: Counter(u128::MAX) });
    assert_eq!(report.work_delta_sum, CheckedTotal::Overflow { exact_prefix: Counter(u128::MAX) });
    assert_eq!(report.arithmetic_status, ArithmeticStatus::IncompleteOverflow);
    assert_eq!(report.helper_work_delta_sum, exact(1));
    assert_eq!(report.reference_cursor_delta_sum, exact(1));
    assert_eq!(report.maximum_observed_logical_depth, Some(3));
    // Later events continue to update depth/other exact columns; overflow is
    // never silently relabelled a complete/saturated u128 total.
    accounting.observe(&event(2, 0, 0, 0, 1, 3, 5));
    let later = accounting.snapshot(PrefixEnd::Open);
    assert_eq!(later.event_count, report.event_count);
    assert_eq!(later.reference_cursor_delta_sum, exact(2));
    assert_eq!(later.maximum_observed_logical_depth, Some(5));
}
