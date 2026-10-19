//! UNRUN owner-authored `/3` runtime/admission source tests. Expectations
//! are hand-derived from the `/3` admission contract (REG3 agreement
//! between `validate_v3`, `decode::registry_for`/`canonical_boundary` and
//! `runtime::scenario_snapshot`, plus ordered start/resume behaviour),
//! not from any evaluator run. No hash or golden bytes are asserted.
use crate::stage::{self, StageEvent, StageOutcome};
use crate::{ExternalDutyDecl, Liability, Program, Scenario, UInt, Value};
use std::collections::BTreeMap;

const ASSET: &str =
    "asset A = { network: \"local\", issuer: \"i\", reference: \"A\", scale: 0 };";
const DUTY: &str = "duty \"loan\" = { asset: A, debtor: \"alice\", creditor: \"bob\" };";

fn v3(body: &str) -> stage::StagedProgram {
    stage::load_v3(&format!(
        "language \"moriarty/3\"; agreement P {{ {body} }}"
    ))
    .expect("/3 program loads")
}

fn v2(body: &str) -> stage::StagedProgram {
    stage::load(&format!(
        "language \"moriarty/2\"; agreement P {{ {body} }}"
    ))
    .expect("/2 program loads")
}

fn scenario(sp: &stage::StagedProgram, name: &str, debtor: &str, amount: u128) -> Scenario {
    Scenario {
        state: BTreeMap::new(),
        inputs: BTreeMap::new(),
        balances: Vec::new(),
        liabilities: vec![Liability {
            name: name.into(),
            asset: sp.program().assets[0].identity.clone(),
            debtor: debtor.into(),
            creditor: "bob".into(),
            amount: UInt(amount),
        }],
    }
}

fn empty_scenario() -> Scenario {
    Scenario {
        state: BTreeMap::new(),
        inputs: BTreeMap::new(),
        balances: Vec::new(),
        liabilities: Vec::new(),
    }
}

fn event(go: bool) -> StageEvent {
    StageEvent {
        format: stage::STAGE_EVENT_FORMAT.into(),
        action: "pay".into(),
        label: "ready".into(),
        values: vec![stage::NamedValue {
            name: "go".into(),
            value: Value::Bool(go),
        }],
    }
}

#[test]
fn core3_nonawaiting_duty_discharge_completes_and_present_zero_stays_present() {
    let sp = v3(&format!(
        "{ASSET} {DUTY} action pay() {{ discharge(\"loan\", 3); }}"
    ));
    let out = stage::start(&sp, "pay", scenario(&sp, "loan", "alice", 5), None)
        .expect("matching duty boundary starts");
    let StageOutcome::Complete {
        segment, residual, ..
    } = out
    else {
        panic!("complete")
    };
    assert_eq!(segment.execution.semantics, "moriarty-core/3");
    assert_eq!(segment.execution.post.liabilities.len(), 1);
    assert_eq!(segment.execution.post.liabilities[0].amount, UInt(2));
    assert_eq!(residual.len(), 1);
    assert_eq!(residual[0].amount, UInt(2));
    let out = stage::start(&sp, "pay", scenario(&sp, "loan", "alice", 3), None)
        .expect("present-zero duty boundary starts");
    let StageOutcome::Complete {
        segment, residual, ..
    } = out
    else {
        panic!("complete")
    };
    assert_eq!(segment.execution.post.liabilities.len(), 1);
    assert_eq!(segment.execution.post.liabilities[0].amount, UInt(0));
    assert!(residual.is_empty());
}

#[test]
fn core3_nonawaiting_wrong_duty_identity_is_refused_at_start_with_zero_attempted() {
    let sp = v3(&format!(
        "{ASSET} {DUTY} action pay() {{ discharge(\"loan\", 3); }}"
    ));
    let rejected = stage::start(&sp, "pay", scenario(&sp, "loan", "mallory", 5), None)
        .expect_err("debtor differs from the source-declared duty");
    assert_eq!(rejected.error.code, "mori::core::liability");
    assert_eq!(rejected.attempted, UInt(0));
    assert_eq!(rejected.committed, UInt(0));
    // F1: scenario_snapshot checks declared exact-asset membership before
    // any REG3 liability identity comparison, so a scale-1 AssetId that is
    // not declared yields `asset`, not `liability`. Legacy ordering is
    // preserved; only this UNRUN expectation is corrected.
    let mut scaled = scenario(&sp, "loan", "alice", 5);
    scaled.liabilities[0].asset.scale = 1;
    let rejected = stage::start(&sp, "pay", scaled, None)
        .expect_err("asset scale differs from every declared exact asset");
    assert_eq!(rejected.error.code, "mori::core::asset");
    assert_eq!(rejected.attempted, UInt(0));
}

#[test]
fn core3_nonawaiting_undeclared_incoming_liability_is_refused_at_start() {
    let sp = v3(&format!(
        "{ASSET} {DUTY} action pay() {{ discharge(\"loan\", 3); }}"
    ));
    let rejected = stage::start(&sp, "pay", scenario(&sp, "ghost", "alice", 5), None)
        .expect_err("undeclared incoming liability is outside REG3");
    assert_eq!(rejected.error.code, "mori::core::liability");
    assert_eq!(rejected.attempted, UInt(0));
    assert_eq!(rejected.committed, UInt(0));
}

#[test]
fn core3_awaiting_duty_waits_false_guard_is_unchanged_and_true_guard_completes() {
    let sp = v3(&format!(
        "{ASSET} {DUTY} action pay() {{ await ready(go: Bool) when go; discharge(\"loan\", 3); }}"
    ));
    let out = stage::start(&sp, "pay", scenario(&sp, "loan", "alice", 5), None)
        .expect("matching duty boundary starts and waits");
    let StageOutcome::Waiting { continuation, .. } = out else {
        panic!("waiting")
    };
    assert_eq!(continuation.boundary.liabilities[0].amount, UInt(5));
    let out = stage::resume(&sp, continuation.clone(), event(false), Some("pay"), None)
        .expect("false guard resumes as unchanged waiting");
    let StageOutcome::Waiting {
        advanced,
        continuation: again,
        ..
    } = out
    else {
        panic!("waiting")
    };
    assert!(!advanced);
    assert_eq!(again, continuation);
    let out = stage::resume(&sp, continuation, event(true), Some("pay"), None)
        .expect("true guard resumes the /3 continuation");
    let StageOutcome::Complete { segment, .. } = out else {
        panic!("complete")
    };
    assert_eq!(segment.execution.semantics, "moriarty-core/3");
    assert_eq!(segment.execution.post.liabilities[0].amount, UInt(2));
    assert_eq!(
        segment.execution.steps[0].snapshot.liabilities[0].amount,
        UInt(5)
    );
}

#[test]
fn core3_awaiting_wrong_duty_identity_is_refused_at_start_with_zero_attempted() {
    let sp = v3(&format!(
        "{ASSET} {DUTY} action pay() {{ await ready(go: Bool) when go; discharge(\"loan\", 3); }}"
    ));
    let rejected = stage::start(&sp, "pay", scenario(&sp, "loan", "mallory", 5), None)
        .expect_err("awaiting start uses the same REG3 identity check");
    assert_eq!(rejected.error.code, "mori::core::liability");
    assert_eq!(rejected.attempted, UInt(0));
    assert_eq!(rejected.committed, UInt(0));
}

#[test]
fn core3_statement_only_liability_remains_admissible_without_a_duty() {
    let sp = v3(&format!(
        "{ASSET} action pay() {{ liability(\"loan\", A, \"alice\", \"bob\", 1); }}"
    ));
    let out = stage::start(&sp, "pay", scenario(&sp, "loan", "alice", 5), None)
        .expect("statement-declared identity still matches");
    let StageOutcome::Complete { segment, .. } = out else {
        panic!("complete")
    };
    assert_eq!(segment.execution.post.liabilities[0].amount, UInt(6));
    let rejected = stage::start(&sp, "pay", scenario(&sp, "loan", "mallory", 5), None)
        .expect_err("statement identity conflict keeps its legacy refusal");
    assert_eq!(rejected.error.code, "mori::core::liability");
    assert_eq!(rejected.attempted, UInt(0));
}

#[test]
fn core2_nonawaiting_discharge_only_keeps_its_legacy_start_domain() {
    let sp = v2(&format!(
        "{ASSET} action pay() {{ discharge(\"loan\", 3); }}"
    ));
    let out = stage::start(&sp, "pay", scenario(&sp, "loan", "alice", 5), None)
        .expect("legacy /2 non-awaiting discharge still starts");
    let StageOutcome::Complete { segment, .. } = out else {
        panic!("complete")
    };
    assert_eq!(segment.execution.semantics, "moriarty-core/2");
    assert_eq!(segment.execution.post.liabilities[0].amount, UInt(2));
}

#[test]
fn core2_awaiting_discharge_only_is_still_refused_at_start_and_core2_resume_still_works() {
    let sp = v2(&format!(
        "{ASSET} action pay() {{ await ready(go: Bool) when go; discharge(\"loan\", 3); }}"
    ));
    let rejected = stage::start(&sp, "pay", scenario(&sp, "loan", "alice", 5), None)
        .expect_err("/2 awaiting boundary still needs a statement registry entry");
    assert_eq!(rejected.error.code, "mori::core::liability");
    assert_eq!(rejected.attempted, UInt(0));
    let sp = v2(&format!(
        "{ASSET} action pay() {{ await ready(go: Bool) when go; liability(\"loan\", A, \"alice\", \"bob\", 2); }}"
    ));
    let out = stage::start(&sp, "pay", empty_scenario(), None).expect("/2 start waits");
    let StageOutcome::Waiting { continuation, .. } = out else {
        panic!("waiting")
    };
    let out = stage::resume(&sp, continuation, event(true), Some("pay"), None)
        .expect("/2 resume still admitted through require_staged");
    let StageOutcome::Complete { segment, .. } = out else {
        panic!("complete")
    };
    assert_eq!(segment.execution.semantics, "moriarty-core/2");
    assert_eq!(segment.execution.post.liabilities[0].amount, UInt(2));
}

#[test]
fn manual_core1_program_with_duties_is_refused_by_validate_and_require_core1() {
    // F2: public fields permit this manual value without parser custody;
    // both public `/1` guards must refuse it with the profile identity
    // instead of allowing duties to reach serialization or hashing.
    let program = Program {
        language: crate::LANGUAGE.into(),
        semantics: crate::SEMANTICS.into(),
        name: "P".into(),
        states: Vec::new(),
        assets: Vec::new(),
        expressions: Vec::new(),
        actions: vec![crate::Action {
            name: "a".into(),
            inputs: Vec::new(),
            statements: Vec::new(),
        }],
        duties: vec![ExternalDutyDecl {
            name: "loan".into(),
            asset: "A".into(),
            debtor: "alice".into(),
            creditor: "bob".into(),
        }],
    };
    assert_eq!(
        crate::validate(&program).expect_err("manual /1 duties refused").code,
        "mori::core::profile"
    );
    assert_eq!(
        crate::stage::require_core1(&program)
            .expect_err("manual /1 duties refused at profile guard")
            .code,
        "mori::core::profile"
    );
    let mut empty = program.clone();
    empty.duties.clear();
    assert!(crate::validate(&empty).is_ok());
    assert!(crate::stage::require_core1(&empty).is_ok());
}
