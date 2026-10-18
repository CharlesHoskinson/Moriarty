//! Hostile-resume tests (H1-H5, H8, H13, H17/H26, H25). Expected values come
//! from the design, not from any evaluator output.
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
    assert_eq!(
        c.captures,
        vec![NamedValue {
            name: "amount".into(),
            value: uint(10)
        }]
    );
    let mut renamed = c.clone();
    renamed.captures[0].name = "amount2".into();
    let mut retyped = c.clone();
    retyped.captures[0].value = Value::Bool(true);
    let mut dropped = c.clone();
    dropped.captures.clear();
    let mut extra = c.clone();
    extra.captures.push(NamedValue {
        name: "ready".into(),
        value: Value::Bool(true),
    });
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
    rejects(
        &sp,
        c.clone(),
        event("deliver", "delivery", &[]),
        "stage_event",
    );
    assert_eq!(c, snapshot);
    match stage::resume(&sp, c, ready(true), None, None).unwrap() {
        StageOutcome::Complete {
            committed,
            attempted,
            ..
        } => {
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
            assert_eq!(
                stage::decode::continuation(&missing).unwrap_err().code,
                "mori::core::json"
            );
        }
        let unknown = text.replacen('{', "{\"extra\":1,", 1);
        assert_eq!(
            stage::decode::continuation(&unknown).unwrap_err().code,
            "mori::core::json"
        );
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

// F1 controls H6, H7, H9, H10, H11, H12, H14 and H24, then optional F2
// coverage. Codes, messages, steps and meter counts come from the design and an
// independent recount of the escrow and E2 plans, not from evaluator output.
use mori_core::{Effect, Error};

/// Requires a refusal before evaluation (attempted 0, committed echoed) with
/// `code`, and returns the error for message checks.
fn refused_before_evaluation(
    sp: &StagedProgram,
    c: Continuation,
    e: StageEvent,
    code: &str,
) -> Error {
    let echoed = c.logical.committed;
    let refused = stage::resume(sp, c, e, None, None).expect_err(code);
    assert_eq!(refused.error.code, format!("mori::core::{code}"));
    assert_eq!((refused.attempted, refused.committed), (UInt(0), echoed));
    refused.error
}
fn balance_of<'a>(c: &'a mut Continuation, account: &str) -> &'a mut Balance {
    c.boundary
        .balances
        .iter_mut()
        .find(|b| b.account == account)
        .unwrap_or_else(|| panic!("no boundary balance for {account:?}"))
}
/// Positive control: the genuine site-1 boundary (oracle balances 90/0/10/42,
/// `release` escrow->bob 10), so every mutation below changes a real field.
fn assert_escrow_boundary(c: &Continuation) {
    let balances: Vec<(&str, u128)> = c
        .boundary
        .balances
        .iter()
        .map(|b| (b.account.as_str(), b.amount.0))
        .collect();
    assert_eq!(
        balances,
        vec![("alice", 90), ("bob", 0), ("escrow", 10), ("observer", 42)]
    );
    let l = &c.boundary.liabilities;
    assert_eq!(l.len(), 1);
    assert_eq!(
        (
            l[0].name.as_str(),
            l[0].debtor.as_str(),
            l[0].creditor.as_str(),
            l[0].amount
        ),
        ("release", "escrow", "bob", UInt(10))
    );
}
fn completes(sp: &StagedProgram, c: Continuation) -> stage::StageSegment {
    match stage::resume(sp, c, ready(true), None, None).unwrap() {
        StageOutcome::Complete {
            segment,
            committed,
            attempted,
            ..
        } => {
            assert_eq!((committed, attempted), (UInt(11), UInt(6)));
            segment
        }
        other => panic!("expected complete, got {other:?}"),
    }
}
fn assert_duplicate_key(error: Error) {
    assert_eq!(error.code, "mori::core::json");
    assert!(
        error.message.starts_with("duplicate decoded key"),
        "{error:?}"
    );
}
fn assert_non_canonical(error: Error) {
    assert_eq!(error.code, "mori::core::json");
    assert!(
        error
            .message
            .contains("expected canonical unsigned decimal string"),
        "{error:?}"
    );
}
const READY_TRUE_JSON: &str = r#"{"format":"moriarty-stage-event/1","action":"deliver","label":"delivery","values":[{"name":"ready","value":{"type":"Bool","value":true}}]}"#;

#[test]
fn h6_duplicate_json_keys_are_refused_by_the_strict_stage_decoders() {
    let sp = stage::load(ESCROW).unwrap();
    let c = pending(&sp, None);
    let text = serde_json::to_string(&c).unwrap();
    assert_eq!(stage::decode::continuation(&text).unwrap(), c);
    let committed = r#""committed":"5""#;
    let site = r#""site":1,"#;
    assert_eq!(text.matches(committed).count(), 1);
    assert_eq!(text.matches(site).count(), 1);
    for forged in [
        text.replacen(committed, r#""committed":"5","committed":"11""#, 1),
        text.replacen(site, r#""site":1,"site":1,"#, 1),
        // A JSON escape decodes to the same key and is still a duplicate.
        text.replacen(site, r#""site":1,"site":2,"#, 1),
    ] {
        assert_ne!(forged, text);
        assert_duplicate_key(stage::decode::continuation(&forged).unwrap_err());
    }

    assert_eq!(stage::decode::event(READY_TRUE_JSON).unwrap(), ready(true));
    let flag = r#""value":true"#;
    let label = r#""label":"delivery","#;
    let name = r#""name":"ready","#;
    for (needle, replacement) in [
        // H6 as frozen: `ready` true, then false.
        (flag, r#""value":true,"value":false"#),
        (flag, r#""value":true,"value":false"#),
        (label, r#""label":"delivery","label":"delivery","#),
        (name, r#""name":"ready","name":"ready","#),
    ] {
        assert_eq!(READY_TRUE_JSON.matches(needle).count(), 1);
        let forged = READY_TRUE_JSON.replacen(needle, replacement, 1);
        assert_duplicate_key(stage::decode::event(&forged).unwrap_err());
    }
}

#[test]
fn h7_non_canonical_uint_strings_are_refused_on_decode() {
    let sp = stage::load(ESCROW).unwrap();
    let c = pending(&sp, None);
    let text = serde_json::to_string(&c).unwrap();
    assert_eq!(stage::decode::continuation(&text).unwrap(), c);
    for (needle, replacement) in [
        (
            r#"{"name":"amount","value":{"type":"UInt","value":"10"}}"#,
            r#"{"name":"amount","value":{"type":"UInt","value":"010"}}"#,
        ),
        (r#""committed":"5""#, r#""committed":"05""#),
        (r#""amount":"42""#, r#""amount":"042""#),
        (
            r#""marker":{"type":"UInt","value":"77"}"#,
            r#""marker":{"type":"UInt","value":"077"}"#,
        ),
    ] {
        assert_eq!(text.matches(needle).count(), 1, "{needle}");
        let forged = text.replacen(needle, replacement, 1);
        assert_non_canonical(stage::decode::continuation(&forged).unwrap_err());
    }

    let flag = r#"{"type":"Bool","value":true}"#;
    assert_eq!(READY_TRUE_JSON.matches(flag).count(), 1);
    let uint_event =
        |raw: &str| READY_TRUE_JSON.replace(flag, &format!(r#"{{"type":"UInt","value":"{raw}"}}"#));
    for raw in ["010", "00", "+10", "1_0", " 10", ""] {
        assert_non_canonical(stage::decode::event(&uint_event(raw)).unwrap_err());
    }
    // The canonical spelling decodes; admission then refuses the UInt for the
    // Bool binder at R5, a separate check from decoding.
    let canonical = stage::decode::event(&uint_event("10")).unwrap();
    assert_eq!(
        canonical,
        event("deliver", "delivery", &[("ready", uint(10))])
    );
    refused_before_evaluation(&sp, c, canonical, "stage_event");
}

#[test]
fn h9_boundary_balances_must_be_strictly_ascending_and_unique() {
    let sp = stage::load(ESCROW).unwrap();
    let c = pending(&sp, None);
    assert_escrow_boundary(&c);
    let mut swapped = c.clone();
    swapped.boundary.balances.swap(0, 1);
    let mut reversed = c.clone();
    reversed.boundary.balances.reverse();
    let mut duplicated = c.clone();
    let escrow = duplicated.boundary.balances[2].clone();
    duplicated.boundary.balances.insert(3, escrow);
    let mut conflicting = c.clone();
    let mut zero = conflicting.boundary.balances[2].clone();
    zero.amount = UInt(0);
    conflicting.boundary.balances.insert(2, zero);
    for forged in [swapped, reversed, duplicated, conflicting] {
        let error = refused_before_evaluation(&sp, forged, ready(true), "balance");
        assert_eq!(error.message, "Balances are not canonical");
    }
    let mut missing = c.clone();
    missing.boundary.balances.retain(|b| b.account != "escrow");
    let error = refused_before_evaluation(&sp, missing, ready(true), "balance");
    assert_eq!(
        error.message,
        "Missing explicit balance for \"escrow\" and asset \"PAY\"."
    );
    completes(&sp, c);
}

#[test]
fn h10_boundary_liability_identity_must_match_the_source_registry() {
    let sp = stage::load(ESCROW).unwrap();
    let c = pending(&sp, None);
    assert_escrow_boundary(&c);
    let mut debtor = c.clone();
    debtor.boundary.liabilities[0].debtor = "alice".into();
    let mut creditor = c.clone();
    creditor.boundary.liabilities[0].creditor = "observer".into();
    for forged in [debtor, creditor] {
        let error = refused_before_evaluation(&sp, forged, ready(true), "liability");
        assert_eq!(
            error.message,
            "Existing liability identity differs for \"release\"."
        );
    }
    // A name absent from the source registry is refused the same way.
    let mut renamed = c.clone();
    renamed.boundary.liabilities[0].name = "ghost".into();
    let error = refused_before_evaluation(&sp, renamed, ready(true), "liability");
    assert_eq!(
        error.message,
        "Existing liability identity differs for \"ghost\"."
    );
    completes(&sp, c);
}

#[test]
fn h11_drained_escrow_fails_the_true_resume_at_step_3_then_a_corrected_retry_completes() {
    let sp = stage::load(ESCROW).unwrap();
    let c = pending(&sp, None);
    assert_escrow_boundary(&c);
    let mut drained = c.clone();
    balance_of(&mut drained, "escrow").amount = UInt(0);
    // R7 admits the drained boundary; the failure is the transfer at
    // ordinal 3 after guard (1 + 1) and transfer (1 + 1) were attempted.
    let refused = stage::resume(&sp, drained.clone(), ready(true), None, None).unwrap_err();
    assert_eq!(refused.error.code, "mori::core::balance");
    assert_eq!(
        refused.error.message,
        "Insufficient balance for \"escrow\"."
    );
    assert_eq!(refused.error.step, Some(3));
    assert_eq!((refused.attempted, refused.committed), (UInt(4), UInt(5)));
    // A false guard on the same drained boundary never reaches the transfer.
    match stage::resume(&sp, drained.clone(), ready(false), None, None).unwrap() {
        StageOutcome::Waiting {
            advanced: false,
            segment: None,
            continuation,
            attempted,
            ..
        } => {
            assert_eq!(attempted, UInt(2));
            assert_eq!(continuation, drained);
        }
        other => panic!("expected unchanged waiting, got {other:?}"),
    }
    // Corrected retry: the same input with escrow back at 10.
    let mut corrected = drained;
    balance_of(&mut corrected, "escrow").amount = UInt(10);
    assert_eq!(corrected, c);
    let segment = completes(&sp, corrected);
    let post: Vec<(&str, u128)> = segment
        .execution
        .post
        .balances
        .iter()
        .map(|b| (b.account.as_str(), b.amount.0))
        .collect();
    assert_eq!(
        post,
        vec![("alice", 90), ("bob", 10), ("escrow", 0), ("observer", 42)]
    );
}

#[test]
fn h12_removed_release_fails_at_step_4_and_publishes_no_transfer_or_segment() {
    let sp = stage::load(ESCROW).unwrap();
    let c = pending(&sp, None);
    assert_escrow_boundary(&c);
    let mut removed = c.clone();
    removed.boundary.liabilities.clear();
    // guard 2 + transfer 2 + discharge 2 attempted; the discharge fails.
    let refused = stage::resume(&sp, removed, ready(true), None, None).unwrap_err();
    assert_eq!(refused.error.code, "mori::core::liability");
    assert_eq!(
        refused.error.message,
        "Unknown residual liability \"release\"."
    );
    assert_eq!(refused.error.step, Some(4));
    assert_eq!((refused.attempted, refused.committed), (UInt(6), UInt(5)));
    // The rejection carries only error and work; the transfer that ran on the
    // evaluator's clone is not published.
    let published = serde_json::to_value(&refused).unwrap();
    let keys: std::collections::BTreeSet<&str> = published
        .as_object()
        .unwrap()
        .keys()
        .map(String::as_str)
        .collect();
    assert_eq!(
        keys,
        std::collections::BTreeSet::from(["attempted", "committed", "error"])
    );
    assert!(!published.to_string().contains("Transfer"));
    // Positive control: the genuine boundary does publish that transfer.
    let segment = completes(&sp, c);
    assert!(segment.execution.effects.iter().any(|effect| matches!(
        effect,
        Effect::Transfer { from, to, amount, .. }
            if from == "escrow" && to == "bob" && *amount == UInt(10)
    )));
}

/// DESIGN-OPUS E2: capture `y` versus current state `level`.
const E2: &str = r#"language "moriarty/2"; agreement CaptureVsState {
  state level: UInt = 0;
  action run(x: UInt) {
    let y = x + 1;
    next level = y;
    await go(z: UInt) when z == y;
    next level = level + z;
  }
}"#;

#[test]
fn h14_tampered_capture_is_accepted_as_a_characterized_unauthenticated_limitation() {
    // Characterization only, not protection: captures and the boundary are
    // unauthenticated (`authentication:none`) and unconsumed
    // (`consumption:untracked`), so a tampered `y` is accepted locally.
    assert!(
        STAGE_LABELS_EXPECTED
            .iter()
            .all(|label| stage::STAGE_LABELS.contains(label))
    );
    let sp = stage::load(E2).unwrap();
    assert_eq!(sp.plan().actions[0].segment_cost, vec![UInt(7), UInt(8)]);
    let opening = Scenario {
        inputs: [("x".into(), uint(4))].into(),
        ..Scenario::default()
    };
    let c = match stage::start(&sp, "run", opening, None).unwrap() {
        StageOutcome::Waiting {
            advanced: true,
            continuation,
            attempted,
            ..
        } => {
            assert_eq!(attempted, UInt(7));
            continuation
        }
        other => panic!("expected waiting, got {other:?}"),
    };
    assert_eq!(
        c.captures,
        vec![
            NamedValue {
                name: "x".into(),
                value: uint(4)
            },
            NamedValue {
                name: "y".into(),
                value: uint(5)
            },
        ]
    );
    assert_eq!(c.boundary.state["level"], uint(5));
    let go = |z: u128| event("run", "go", &[("z", uint(z))]);
    // Untampered, z = 6 does not satisfy the guard.
    match stage::resume(&sp, c.clone(), go(6), None, None).unwrap() {
        StageOutcome::Waiting {
            advanced: false,
            continuation,
            attempted,
            ..
        } => {
            assert_eq!(attempted, UInt(4));
            assert_eq!(continuation, c);
        }
        other => panic!("expected unchanged waiting, got {other:?}"),
    }
    let mut tampered = c;
    tampered.captures[1].value = uint(6);
    tampered.boundary.state.insert("level".into(), uint(100));
    match stage::resume(&sp, tampered, go(6), None, None).unwrap() {
        StageOutcome::Complete {
            segment,
            committed,
            attempted,
            ..
        } => {
            assert_eq!((committed, attempted), (UInt(15), UInt(8)));
            assert_eq!(segment.execution.pre.state["level"], uint(100));
            assert_eq!(segment.execution.post.state["level"], uint(106));
        }
        other => panic!("expected the tampered capture to be accepted, got {other:?}"),
    }
}
const STAGE_LABELS_EXPECTED: [(&str, &str); 3] = [
    ("qualification", "local-reference-stage"),
    ("authentication", "none"),
    ("consumption", "untracked"),
];

const MIB: usize = 1_048_576;
fn padded(text: &str, len: usize) -> String {
    assert!(text.len() <= len);
    let mut out = text.to_owned();
    out.push_str(&" ".repeat(len - text.len()));
    out
}
fn assert_too_large(error: Error, message: &str) {
    assert_eq!(error.code, "mori::core::json");
    assert_eq!(error.message, message);
}

#[test]
fn h24_stage_size_caps_apply_before_parse_and_the_legacy_ceiling_is_unchanged() {
    assert_eq!(stage::MAX_CONTINUATION_BYTES, 4 * MIB);
    assert_eq!(stage::MAX_EVENT_BYTES, 65_536);
    let sp = stage::load(ESCROW).unwrap();
    let c = pending(&sp, None);
    let text = serde_json::to_string(&c).unwrap();
    // Whitespace padding keeps the text valid JSON, so the only difference
    // between the accepted and refused case is one byte over the cap.
    assert_eq!(
        stage::decode::continuation(&padded(&text, 4 * MIB)).unwrap(),
        c
    );
    for over in [padded(&text, 4 * MIB + 1), "[".repeat(4 * MIB + 1)] {
        assert_too_large(
            stage::decode::continuation(&over).unwrap_err(),
            "Continuation JSON exceeds 4 MiB",
        );
    }
    // The legacy 1 MiB ceiling does not apply to a stage continuation.
    assert_eq!(
        stage::decode::continuation(&padded(&text, MIB + 1)).unwrap(),
        c
    );

    assert_eq!(
        stage::decode::event(&padded(READY_TRUE_JSON, 65_536)).unwrap(),
        ready(true)
    );
    for over in [padded(READY_TRUE_JSON, 65_537), "{".repeat(65_537)] {
        assert_too_large(
            stage::decode::event(&over).unwrap_err(),
            "Stage event JSON exceeds 64 KiB",
        );
    }

    // `/1` keeps its ceiling and message, including for a stage-sized text.
    assert!(mori_core::json::parse(&padded("{}", MIB)).is_ok());
    for over in [padded("{}", MIB + 1), padded(&text, 4 * MIB)] {
        assert_too_large(
            mori_core::json::parse(&over).unwrap_err(),
            "JSON exceeds 1 MiB",
        );
    }
}

/// A file under the system temp directory, removed on drop. Created
/// exclusively, so an existing file or symlink at the path is never written
/// through or later removed.
struct TempInput(std::path::PathBuf);
impl TempInput {
    fn new(tag: &str, bytes: &[u8]) -> Self {
        use std::io::Write;
        let path = std::env::temp_dir().join(format!(
            "mori-core-stage-hostile-{}-{tag}",
            std::process::id()
        ));
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .unwrap();
        let fixture = Self(path);
        file.write_all(bytes).unwrap();
        fixture
    }
}
impl Drop for TempInput {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

#[test]
fn h24_read_bounded_refuses_oversized_stage_files_before_decoding() {
    use stage::decode::{CONTINUATION_TOO_LARGE, EVENT_TOO_LARGE, read_bounded};
    let event_at_cap = TempInput::new("event-cap", padded(READY_TRUE_JSON, 65_536).as_bytes());
    let read = read_bounded(&event_at_cap.0, stage::MAX_EVENT_BYTES, EVENT_TOO_LARGE).unwrap();
    assert_eq!(stage::decode::event(&read).unwrap(), ready(true));
    let event_over = TempInput::new("event-over", padded(READY_TRUE_JSON, 65_537).as_bytes());
    assert_too_large(
        read_bounded(&event_over.0, stage::MAX_EVENT_BYTES, EVENT_TOO_LARGE).unwrap_err(),
        "Stage event JSON exceeds 64 KiB",
    );

    let sp = stage::load(ESCROW).unwrap();
    let text = serde_json::to_string(&pending(&sp, None)).unwrap();
    let continuation_over =
        TempInput::new("continuation-over", padded(&text, 4 * MIB + 1).as_bytes());
    assert_too_large(
        read_bounded(
            &continuation_over.0,
            stage::MAX_CONTINUATION_BYTES,
            CONTINUATION_TOO_LARGE,
        )
        .unwrap_err(),
        "Continuation JSON exceeds 4 MiB",
    );

    let not_utf8 = TempInput::new("not-utf8", &[0xff]);
    assert_too_large(
        read_bounded(&not_utf8.0, stage::MAX_EVENT_BYTES, EVENT_TOO_LARGE).unwrap_err(),
        "Stage input is not UTF-8",
    );
}

#[test]
fn f2_start_requires_exactly_the_typed_action_inputs() {
    let sp = stage::load(ESCROW).unwrap();
    let with_inputs = |inputs: Vec<(&str, Value)>| Scenario {
        inputs: inputs.into_iter().map(|(n, v)| (n.into(), v)).collect(),
        ..opening(10)
    };
    let cases = [
        (
            vec![("amount", Value::Bool(true))],
            "Missing or wrong type for input \"amount\".",
        ),
        (
            vec![],
            "Scenario must provide exactly the selected action's inputs.",
        ),
        (
            vec![("amount", uint(10)), ("extra", uint(1))],
            "Scenario must provide exactly the selected action's inputs.",
        ),
        (
            vec![("other", uint(10))],
            "Missing or wrong type for input \"amount\".",
        ),
    ];
    for (inputs, message) in cases {
        let refused = stage::start(&sp, "deliver", with_inputs(inputs), None).unwrap_err();
        assert_eq!(refused.error.code, "mori::core::input");
        assert_eq!(refused.error.message, message);
        assert_eq!((refused.attempted, refused.committed), (UInt(0), UInt(0)));
    }
    assert!(
        stage::start(
            &sp,
            "deliver",
            with_inputs(vec![("amount", uint(10))]),
            None
        )
        .is_ok()
    );
}

#[test]
fn f2_r7_requires_exactly_the_declared_typed_states() {
    let sp = stage::load(ESCROW).unwrap();
    let c = pending(&sp, None);
    assert_eq!(c.boundary.state.len(), 1);
    let mut missing = c.clone();
    missing.boundary.state.clear();
    let mut extra = c.clone();
    extra.boundary.state.insert("ghost".into(), uint(0));
    for forged in [missing, extra] {
        let error = refused_before_evaluation(&sp, forged, ready(true), "state");
        assert_eq!(
            error.message,
            "Boundary must provide every declared state value and no other."
        );
    }
    let mut renamed = c.clone();
    let marker = renamed.boundary.state.remove("marker").unwrap();
    renamed.boundary.state.insert("markers".into(), marker);
    let error = refused_before_evaluation(&sp, renamed, ready(true), "state");
    assert_eq!(error.message, "Missing state \"marker\".");
    let mut retyped = c.clone();
    retyped
        .boundary
        .state
        .insert("marker".into(), Value::Bool(true));
    let error = refused_before_evaluation(&sp, retyped, ready(true), "state");
    assert_eq!(error.message, "Wrong type for state \"marker\".");
    completes(&sp, c);
}

const PAIR: &str = r#"language "moriarty/2";
agreement LiabilityPair {
  asset PAY = { network: "local", issuer: "liability-pair", reference: "PAY", scale: 0 };
  state marker: UInt = 0;
  action owe(n: UInt) {
    liability("alpha", PAY, "debtor", "creditor", n);
    liability("beta", PAY, "debtor", "creditor", n);
    await go(ok: Bool) when ok;
    discharge("beta", n);
  }
}"#;

#[test]
fn f2_r7_requires_strictly_ascending_unique_liabilities() {
    // Strict order is load-bearing: liability reads use a binary search.
    let sp = stage::load(PAIR).unwrap();
    let opening = Scenario {
        inputs: [("n".into(), uint(3))].into(),
        ..Scenario::default()
    };
    let c = match stage::start(&sp, "owe", opening, None).unwrap() {
        StageOutcome::Waiting {
            advanced: true,
            continuation,
            ..
        } => continuation,
        other => panic!("expected waiting, got {other:?}"),
    };
    let names: Vec<&str> = c
        .boundary
        .liabilities
        .iter()
        .map(|l| l.name.as_str())
        .collect();
    assert_eq!(names, vec!["alpha", "beta"]);
    let go = || event("owe", "go", &[("ok", Value::Bool(true))]);
    let mut inverted = c.clone();
    inverted.boundary.liabilities.swap(0, 1);
    let mut duplicated = c.clone();
    let alpha = duplicated.boundary.liabilities[0].clone();
    duplicated.boundary.liabilities.insert(1, alpha);
    for forged in [inverted, duplicated] {
        let error = refused_before_evaluation(&sp, forged, go(), "liability");
        assert_eq!(error.message, "Liabilities are not canonical");
    }
    assert!(matches!(
        stage::resume(&sp, c, go(), None, None).unwrap(),
        StageOutcome::Complete { .. }
    ));
}
