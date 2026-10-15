//! Opus-authored hostile-resume tests (H1-H5, H8, H13, H17/H26, H25).
//! Written before execution; not yet run. Expected values are frozen from the
//! source read and DESIGN-OPUS-A2, not from any evaluator output, and are not
//! acceptance evidence by themselves.
//!
//! Each case perturbs exactly one field of a genuine Waiting continuation, so
//! the expected code does not depend on admission order beyond R0-R7.
use mori_core::stage::{self, Continuation, NamedValue, StageEvent, StageOutcome, StagedProgram};
use mori_core::{AssetId, Balance, Scenario, UInt, Value};
use proptest::prelude::*;

const ESCROW: &str = include_str!("../../../examples/core/continuations/escrow.mori");

fn uint(n: u128) -> Value {
    Value::UInt(UInt(n))
}
fn event(action: &str, label: &str, values: &[(&str, Value)]) -> StageEvent {
    StageEvent {
        format: stage::STAGE_EVENT_FORMAT.into(),
        action: action.into(),
        label: label.into(),
        values: values
            .iter()
            .map(|(name, value)| NamedValue {
                name: (*name).into(),
                value: value.clone(),
            })
            .collect(),
    }
}
fn ready(flag: bool) -> StageEvent {
    event("deliver", "delivery", &[("ready", Value::Bool(flag))])
}
fn opening(amount: u128) -> Scenario {
    let id = AssetId {
        network: "local".into(),
        issuer: "continuation-escrow-oracle".into(),
        reference: "PAY".into(),
        scale: 0,
    };
    let balance = |account: &str, amount: u128| Balance {
        account: account.into(),
        asset: id.clone(),
        amount: UInt(amount),
    };
    Scenario {
        inputs: [("amount".into(), uint(amount))].into(),
        balances: vec![
            balance("alice", 100),
            balance("bob", 0),
            balance("escrow", 0),
            balance("observer", 42),
        ],
        ..Scenario::default()
    }
}
/// A genuine escrow continuation at site 1, committed 5 (oracle `[5, 6]`).
fn pending(sp: &StagedProgram, cap: Option<UInt>) -> Continuation {
    match stage::start(sp, "deliver", opening(10), cap).unwrap() {
        StageOutcome::Waiting {
            advanced: true,
            continuation,
            ..
        } => continuation,
        other => panic!("expected an advanced waiting outcome, got {other:?}"),
    }
}
/// Resumes and requires a rejection with `code`, attempted 0 and the input's
/// committed value echoed unchanged.
fn rejects(sp: &StagedProgram, c: Continuation, e: StageEvent, code: &str) {
    let echoed = c.logical.committed;
    let refused = stage::resume(sp, c, e, None, None).expect_err(code);
    assert_eq!(refused.error.code, format!("mori::core::{code}"));
    assert_eq!((refused.attempted, refused.committed), (UInt(0), echoed));
}

#[test]
fn r0_r1_selected_action_format_and_profile_are_exact() {
    let sp = stage::load(ESCROW).unwrap();
    let c = pending(&sp, None);
    let refused = stage::resume(&sp, c.clone(), ready(true), Some("other"), None).unwrap_err();
    assert_eq!(refused.error.code, "mori::core::stage_site");
    assert_eq!(refused.attempted, UInt(0));
    assert!(stage::resume(&sp, c.clone(), ready(true), Some("deliver"), None).is_ok());
    let mut format = c.clone();
    format.format = "moriarty-continuation/2".into();
    rejects(&sp, format, ready(true), "stage_format");
    let mut profile = c;
    profile.logical.profile = format!("{}-forged", stage::LOGICAL_COST_PROFILE_2);
    rejects(&sp, profile, ready(true), "stage_format");
}

#[test]
fn h1_h2_program_and_plan_digests_bind_the_continuation() {
    let sp = stage::load(ESCROW).unwrap();
    let c = pending(&sp, None);
    let flip = |hex: &str| {
        let mut bytes = hex.as_bytes().to_vec();
        bytes[0] = if bytes[0] == b'0' { b'1' } else { b'0' };
        String::from_utf8(bytes).unwrap()
    };
    let mut program = c.clone();
    program.program_hash = flip(&c.program_hash);
    rejects(&sp, program, ready(true), "stage_program");
    let mut plan = c.clone();
    plan.plan_hash = flip(&c.plan_hash);
    rejects(&sp, plan, ready(true), "stage_program");
    // Non-canonical encodings of the right digest are not accepted.
    let mut upper = c.clone();
    upper.program_hash = c.program_hash.to_uppercase();
    if upper.program_hash != c.program_hash {
        rejects(&sp, upper, ready(true), "stage_program");
    }
    // H1: a continuation of a one-identifier edit cannot resume the original,
    // and the original cannot resume the edit.
    let edited = stage::load(&ESCROW.replace(
        "agreement ContinuationEscrowOracle",
        "agreement ContinuationEscrowOracleEdited",
    ))
    .unwrap();
    assert_ne!(edited.program_hash(), sp.program_hash());
    let foreign = pending(&edited, None);
    rejects(&sp, foreign, ready(true), "stage_program");
    rejects(&edited, c, ready(true), "stage_program");
}

#[test]
fn h3_only_a_planned_pending_site_is_admitted() {
    let sp = stage::load(ESCROW).unwrap();
    let c = pending(&sp, None);
    assert_eq!(c.site, 1);
    for site in [0, 2, u16::MAX] {
        let mut forged = c.clone();
        forged.site = site;
        rejects(&sp, forged, ready(true), "stage_site");
    }
    let mut action = c;
    action.action = "missing".into();
    rejects(&sp, action, ready(true), "stage_site");
}

#[test]
fn h4_captures_must_match_the_site_layout_exactly() {
    let sp = stage::load(ESCROW).unwrap();
    let c = pending(&sp, None);
    assert_eq!(c.captures, vec![NamedValue { name: "amount".into(), value: uint(10) }]);
    let mut renamed = c.clone();
    renamed.captures[0].name = "amount2".into();
    let mut retyped = c.clone();
    retyped.captures[0].value = Value::Bool(true);
    let mut dropped = c.clone();
    dropped.captures.clear();
    let mut extra = c.clone();
    extra.captures.push(NamedValue { name: "ready".into(), value: Value::Bool(true) });
    for forged in [renamed, retyped, dropped, extra] {
        rejects(&sp, forged, ready(true), "stage_capture");
    }
}

#[test]
fn h5_event_must_match_the_pending_await_exactly() {
    let sp = stage::load(ESCROW).unwrap();
    let c = pending(&sp, None);
    let mut format = ready(true);
    format.format = "moriarty-stage-event/2".into();
    let mut action = ready(true);
    action.action = "other".into();
    let cases = [
        format,
        action,
        event("deliver", "Delivery", &[("ready", Value::Bool(true))]),
        event("deliver", "delivery", &[("Ready", Value::Bool(true))]),
        event("deliver", "delivery", &[("ready", uint(1))]),
        event("deliver", "delivery", &[]),
        event(
            "deliver",
            "delivery",
            &[("ready", Value::Bool(true)), ("ready", Value::Bool(true))],
        ),
    ];
    for e in cases {
        rejects(&sp, c.clone(), e, "stage_event");
    }
}

#[test]
fn h8_committed_work_and_cap_cannot_be_forged() {
    let sp = stage::load(ESCROW).unwrap();
    let c = pending(&sp, Some(UInt(11)));
    for committed in [0, 4, 6, 11, u128::MAX] {
        let mut forged = c.clone();
        forged.logical.committed = UInt(committed);
        rejects(&sp, forged, ready(true), "stage_logical");
    }
    // Lowering the stored cap below the next segment refuses; the CLI cap
    // can never exceed the stored cap.
    let mut lowered = c.clone();
    lowered.logical.cap = Some(UInt(10));
    rejects(&sp, lowered, ready(true), "stage_logical");
    let above = stage::resume(&sp, c.clone(), ready(true), None, Some(UInt(12))).unwrap_err();
    assert_eq!(above.error.code, "mori::core::stage_logical");
    // Characterized limitation (unauthenticated): raising or removing the
    // stored cap is not detectable, since `cap` is outside both digests.
    let mut uncapped = c;
    uncapped.logical.cap = None;
    assert!(stage::resume(&sp, uncapped, ready(true), None, None).is_ok());
}

#[test]
fn c10_rejection_leaves_the_input_valid_for_a_corrected_retry() {
    let sp = stage::load(ESCROW).unwrap();
    let c = pending(&sp, None);
    let snapshot = c.clone();
    rejects(&sp, c.clone(), event("deliver", "delivery", &[]), "stage_event");
    assert_eq!(c, snapshot);
    match stage::resume(&sp, c, ready(true), None, None).unwrap() {
        StageOutcome::Complete { committed, attempted, .. } => {
            assert_eq!((committed, attempted), (UInt(11), UInt(6)));
        }
        other => panic!("expected complete, got {other:?}"),
    }
}

#[test]
fn h13_p3_duplicate_resume_is_an_untracked_characterized_limitation() {
    // No consumption state exists (P3): the same continuation resumes twice
    // with identical results. This documents the limitation; it is not a
    // replay-protection claim.
    let sp = stage::load(ESCROW).unwrap();
    let c = pending(&sp, None);
    let first = stage::resume(&sp, c.clone(), ready(true), None, None).unwrap();
    let second = stage::resume(&sp, c, ready(true), None, None).unwrap();
    assert_eq!(first, second);
    assert!(matches!(first, StageOutcome::Complete { .. }));
    // A fabricated idle scenario starts again, also untracked.
    assert_eq!(pending(&sp, None), pending(&sp, None));
}

#[test]
fn h25_emitted_continuations_round_trip_through_the_resume_decoder() {
    let sp = stage::load(ESCROW).unwrap();
    for cap in [None, Some(UInt(11)), Some(UInt(u128::MAX))] {
        let c = pending(&sp, cap);
        assert_eq!(c.format, stage::CONTINUATION_FORMAT);
        assert_eq!(c.logical.profile, stage::LOGICAL_COST_PROFILE_2);
        assert_eq!(c.logical.cap, cap);
        let text = serde_json::to_string(&c).unwrap();
        assert!(text.len() <= stage::MAX_CONTINUATION_BYTES);
        let decoded = stage::decode::continuation(&text).unwrap();
        assert_eq!(decoded, c);
        // The `cap` key is mandatory even when null.
        let missing = text.replacen("\"cap\":null,", "", 1);
        if missing != text {
            assert_eq!(stage::decode::continuation(&missing).unwrap_err().code, "mori::core::json");
        }
        let unknown = text.replacen('{', "{\"extra\":1,", 1);
        assert_eq!(stage::decode::continuation(&unknown).unwrap_err().code, "mori::core::json");
        assert!(stage::resume(&sp, decoded, ready(true), None, None).is_ok());
    }
}

// Literal source with independently counted cost: `let m = k * N` is one
// dispatch plus {k, N, *} = 4; `next n = n + m` is one plus {n, m, +} = 4.
fn literal_source(n: u128) -> String {
    format!(
        r#"language "moriarty/2"; agreement L {{
  state n: UInt = 0;
  action bump(k: UInt) {{
    let m = k * {n};
    next n = n + m;
    await go(ok: Bool) when ok;
    next n = n + {n};
  }}
}}"#
    )
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(32))]

    /// H17/H26 over small independent literal ranges: the plan is
    /// `[4 + 4 + 1, 1 + 1 + 4] = [9, 6]`, total 15, for every literal and
    /// input, each successful segment attempts exactly its cost, and the
    /// literal moves both digests (H1).
    #[test]
    fn h17_h26_static_cost_is_literal_independent(n in 0u128..1_000, k in 0u128..1_000) {
        let sp = stage::load(&literal_source(n)).unwrap();
        let plan = &sp.plan().actions[0];
        prop_assert_eq!(plan.segment_cost.clone(), vec![UInt(9), UInt(6)]);
        prop_assert_eq!(plan.committed_before.clone(), vec![UInt(0), UInt(9)]);
        prop_assert_eq!(plan.plan_total, UInt(15));
        let other = stage::load(&literal_source(n + 1)).unwrap();
        prop_assert_ne!(sp.program_hash(), other.program_hash());
        prop_assert_ne!(sp.plan_hash(), other.plan_hash());
        let scenario = Scenario {
            inputs: [("k".into(), uint(k))].into(),
            ..Scenario::default()
        };
        let c = match stage::start(&sp, "bump", scenario, Some(UInt(15))).unwrap() {
            StageOutcome::Waiting { advanced: true, continuation, attempted, .. } => {
                prop_assert_eq!(attempted, UInt(9));
                continuation
            }
            other => panic!("expected waiting, got {other:?}"),
        };
        prop_assert_eq!(c.logical.committed, UInt(9));
        let go = |ok: bool| event("bump", "go", &[("ok", Value::Bool(ok))]);
        match stage::resume(&sp, c.clone(), go(false), None, None).unwrap() {
            StageOutcome::Waiting { advanced: false, continuation, attempted, .. } => {
                prop_assert_eq!(attempted, UInt(2));
                prop_assert_eq!(&continuation, &c);
            }
            other => panic!("expected unchanged waiting, got {other:?}"),
        }
        match stage::resume(&sp, c, go(true), None, None).unwrap() {
            StageOutcome::Complete { segment, committed, attempted, .. } => {
                prop_assert_eq!((committed, attempted), (UInt(15), UInt(6)));
                prop_assert_eq!(&segment.execution.post.state["n"], &uint(k * n + n));
            }
            other => panic!("expected complete, got {other:?}"),
        }
    }
}
