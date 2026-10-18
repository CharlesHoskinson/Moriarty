//! Positive all-of parent-to-successor behavior through the proposed public API.
use mori_core::{
    Scenario, UInt,
    stage::{self, join_draft::{self as join, Availability, JoinOutcome, Phase}},
};
use serde_json::Value;
use std::{collections::BTreeMap, fs, path::{Path, PathBuf}};

fn fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../fixtures/conditional-join-all-of")
}
fn input(name: &str) -> String {
    fs::read_to_string(fixtures().join("input").join(name)).unwrap()
}
fn expected(name: &str) -> Value {
    serde_json::from_slice(&fs::read(fixtures().join("expected").join(name)).unwrap()).unwrap()
}
fn program() -> join::JoinProgram {
    let document = input("document.mori");
    let recipient = input("recipient.mori");
    let settlement = input("settlement.mori");
    join::load(&input("release-all.join.mori"), &[
        ("document.mori", &document),
        ("recipient.mori", &recipient),
        ("settlement.mori", &settlement),
    ]).unwrap()
}
fn scenario(name: &str) -> Scenario {
    serde_json::from_str(&input(name)).unwrap()
}
fn event(name: &str) -> stage::StageEvent {
    stage::decode::event(&input(name)).unwrap()
}
fn checkpoint(outcome: JoinOutcome, fixture: &str, phase: Phase) -> join::JoinContinuation {
    assert_eq!(join::outcome_report(&outcome), expected(fixture));
    let continuation = match outcome {
        JoinOutcome::ParentsPending { continuation }
        | JoinOutcome::ReadyToEnter { continuation }
        | JoinOutcome::SuccessorPending { continuation } => continuation,
        JoinOutcome::Complete { .. } => panic!("expected pending join or handoff"),
    };
    assert_eq!(join::view(&continuation).phase, phase);
    continuation
}
fn attach(plan: &join::JoinProgram, trace: &str) -> join::JoinContinuation {
    let document = join::prepare_parent(plan, "document",
        scenario("document.scenario.json"), Some(UInt(50))).unwrap();
    let recipient = join::prepare_parent(plan, "recipient",
        scenario("recipient.scenario.json"), Some(UInt(50))).unwrap();
    assert_eq!(join::parent_report(&document), expected("parents/document-start.json"));
    assert_eq!(join::parent_report(&recipient), expected("parents/recipient-start.json"));
    checkpoint(join::attach(plan, [document, recipient], BTreeMap::new(),
        Some(UInt(50)), "release-example-1").unwrap(),
        &format!("{trace}/attach.json"), Phase::ParentsPending)
}
fn finish(plan: &join::JoinProgram, ready: join::JoinContinuation, trace: &str)
    -> join::JoinCompletion
{
    let ready_view = join::view(&ready);
    let boundary = ready_view.handoff_boundary.unwrap();
    let parent_frames = ready_view.current_frame.parents;
    let successor = checkpoint(join::enter(plan, ready).unwrap(),
        &format!("{trace}/enter.json"), Phase::SuccessorPending);
    let entered_view = join::view(&successor);
    assert_eq!(entered_view.current_frame.parents, parent_frames);
    assert_eq!(entered_view.current_frame.successor.as_ref().unwrap().boundary, boundary);
    let completed = join::resume_successor(plan, successor, event("handoff-true.event.json")).unwrap();
    assert_eq!(join::outcome_report(&completed), expected(&format!("{trace}/complete.json")));
    let JoinOutcome::Complete { completion } = completed else {
        panic!("successor must complete");
    };
    assert_eq!(join::completion_view(&completion).phase, Phase::Complete);
    assert_eq!(join::completion_view(&completion).current_frame.parents, parent_frames);
    completion
}

#[test]
fn complete_join_plan_retains_all_three_programs_and_filled_stage_plans() {
    let plan = program();
    assert_eq!(join::plan_report(&plan), expected("join-plan.json"));
}

#[test]
fn available_false_results_do_not_satisfy_all_of_and_do_not_drop_parent_frames() {
    let plan = program();
    let trace = "both-observed-false";
    let pending = attach(&plan, trace);
    let original_document = join::parent_continuation(&pending, "document").unwrap().clone();
    let original_recipient = join::parent_continuation(&pending, "recipient").unwrap().clone();
    let original_frames = join::view(&pending).current_frame.parents;
    let pending = checkpoint(join::resume_parent(&plan, pending, "document",
        event("document-false.event.json")).unwrap(),
        &format!("{trace}/document-false.json"), Phase::ParentsPending);
    assert_eq!(join::parent_continuation(&pending, "document"), Some(&original_document));
    assert_eq!(join::parent_continuation(&pending, "recipient"), Some(&original_recipient));
    assert_eq!(join::view(&pending).current_frame.parents, original_frames);
    let pending = checkpoint(join::resume_parent(&plan, pending, "recipient",
        event("recipient-false.event.json")).unwrap(),
        &format!("{trace}/recipient-false.json"), Phase::ParentsPending);
    let observed = join::view(&pending);
    assert_eq!(observed.availability, [Availability::Observed, Availability::Observed]);
    assert!(observed.availability_satisfied);
    assert!(!observed.success_satisfied);
    assert_eq!(observed.current_frame.parents, original_frames);
    let pending = checkpoint(join::resume_parent(&plan, pending, "document",
        event("document-true.event.json")).unwrap(),
        &format!("{trace}/document-true.json"), Phase::ParentsPending);
    let completed_document = join::view(&pending).current_frame.parents[0].clone();
    let ready = checkpoint(join::resume_parent(&plan, pending, "recipient",
        event("recipient-true.event.json")).unwrap(),
        &format!("{trace}/recipient-true.json"), Phase::ReadyToEnter);
    assert_eq!(join::view(&ready).current_frame.parents[0], completed_document);
    let _completion = finish(&plan, ready, trace);
}

#[test]
fn both_arrival_orders_reach_equal_complete_frames_with_distinct_full_histories() {
    let plan = program();
    let mut terminals = Vec::new();
    for (trace, first, first_event, second, second_event) in [
        ("document-first", "document", "document-true.event.json", "recipient", "recipient-true.event.json"),
        ("recipient-first", "recipient", "recipient-true.event.json", "document", "document-true.event.json"),
    ] {
        let pending = attach(&plan, trace);
        let pending = checkpoint(join::resume_parent(&plan, pending, first, event(first_event)).unwrap(),
            &format!("{trace}/first.json"), Phase::ParentsPending);
        let ready = checkpoint(join::resume_parent(&plan, pending, second, event(second_event)).unwrap(),
            &format!("{trace}/second.json"), Phase::ReadyToEnter);
        terminals.push(join::completion_view(&finish(&plan, ready, trace)));
    }
    assert_eq!(terminals[0].current_frame, terminals[1].current_frame);
    assert_eq!(terminals[0].effective_boundary, terminals[1].effective_boundary);
    assert_eq!(terminals[0].work.current_committed, terminals[1].work.current_committed);
    assert_eq!(terminals[0].work.successor_committed, terminals[1].work.successor_committed);
    assert_eq!(terminals[0].work.core_committed_total, terminals[1].work.core_committed_total);
    // Whole reports were compared independently for each order; no history is sorted.
}
