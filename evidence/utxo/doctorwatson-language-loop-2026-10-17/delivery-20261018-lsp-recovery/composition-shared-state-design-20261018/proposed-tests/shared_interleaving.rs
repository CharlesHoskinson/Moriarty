// Proposed future crates/mori-core/tests/shared_interleaving.rs.
// No current API/feature or missing expected fixture is supplied by this file.
#![cfg(feature = "core2-shared-interleaving-draft")]
use mori_core::{Snapshot, UInt, stage};
use mori_core::stage::shared::{Bounds, Session, Slot};
use serde::Deserialize;
use serde_json::Value;
use std::{fs, path::PathBuf};

#[derive(Deserialize)]
struct Case {
    name: String,
    steps: Vec<Operation>,
}

#[derive(Deserialize)]
#[serde(tag = "op", rename_all = "lowercase", deny_unknown_fields)]
enum Operation {
    Start { slot: usize },
    Resume { slot: usize, event: String },
}

#[test]
fn shared_head_preserves_pending_quotes_and_both_complete_financial_domains() {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../fixtures/core-shared-interleaving");
    let source = fs::read_to_string(dir.join("shared-deliveries.mori")).unwrap();
    let opening: Snapshot =
        serde_json::from_slice(&fs::read(dir.join("opening.json")).unwrap()).unwrap();
    let cases: Vec<Case> =
        serde_json::from_slice(&fs::read(dir.join("schedules.json")).unwrap()).unwrap();
    // Required independent complete reports are frozen before any Session runs.
    // Each array contains opening plus every complete invocation observation.
    let expected: Vec<Vec<Value>> = cases.iter().map(|case| {
        serde_json::from_slice(
            &fs::read(dir.join(format!("{}.reports.json", case.name))).unwrap(),
        ).unwrap()
    }).collect();
    let events: Vec<Vec<Option<stage::StageEvent>>> = cases.iter().map(|case| {
        case.steps.iter().map(|step| match step {
            Operation::Start { .. } => None,
            Operation::Resume { event, .. } => Some(stage::decode::event(
                &fs::read_to_string(dir.join(event)).unwrap(),
            ).unwrap()),
        }).collect()
    }).collect();
    let program = stage::load(&source).unwrap();
    for ((case, expected), events) in cases.into_iter().zip(expected).zip(events) {
        assert_eq!(expected.len(), case.steps.len() + 1, "{}", case.name);
        let mut session = Session::open(
            program.clone(),
            opening.clone(),
            [
                Slot { action: "deliver_a".into(), inputs: vec![], logical_cap: UInt(23) },
                Slot { action: "deliver_b".into(), inputs: vec![], logical_cap: UInt(23) },
            ],
            Bounds { attempted_cap: UInt(46), invocation_limit: 4 },
        ).unwrap();
        assert_eq!(session.report().unwrap(), expected[0], "{} opening", case.name);
        for (index, (step, event)) in case.steps.into_iter().zip(events).enumerate() {
            match step {
                Operation::Start { slot } => session.start(slot).unwrap(),
                Operation::Resume { slot, .. } => session.resume(slot, event.unwrap()).unwrap(),
            }
            // Whole equality covers current head, original history and Steps,
            // both frames/captures, all effects/duties, work and untouched rows.
            assert_eq!(session.report().unwrap(), expected[index + 1],
                "{} observation {}", case.name, index + 1);
        }
    }
}
