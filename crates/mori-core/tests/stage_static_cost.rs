//! Static-cost and `/2` validation tests (H17, H26, H22 core).
//!
//! H17: a successful segment's attempted work equals the plan's
//! `segment_cost`, for random inputs and boundaries, through the normal
//! `load`/`start`/`resume` path. If any case fails, C3 is void (A11 F2).
use mori_core::stage::{
    self, Continuation, NamedValue, StageEvent, StageOutcome, StageSegment, StagedProgram,
};
use mori_core::{AssetId, Balance, Liability, Scenario, UInt, Value};
use proptest::prelude::*;
use sha2::{Digest, Sha256};

const ESCROW: &str = include_str!("../../../examples/core/continuations/escrow.mori");
const RESERVE: &str = include_str!("../../../examples/core/continuations/reserve.mori");
// DESIGN-OPUS §7 E2: captured let versus current state.
const CAPTURE: &str = r#"language "moriarty/2";
agreement CaptureVersusState {
  state level: UInt = 0;
  action a(x: UInt) {
    let y = x + 1;
    next level = y;
    await go(z: UInt) when z == y;
    next level = level + z;
  }
}"#;
const CONSECUTIVE: &str = r#"language "moriarty/2"; agreement Consecutive {
  state count: UInt = 0;
  action run(x: UInt) {
    let captured = x + 1;
    next count = captured;
    await first(y: UInt) when y == captured;
    await second(ready: Bool) when ready;
    next count = count + y;
  }
}"#;
const PLAIN: &str = r#"language "moriarty/2"; agreement Plain {
  state n: UInt = 0;
  action bump(k: UInt) {
    let m = k * 2;
    next n = n + m;
  }
}"#;

fn uint(n: u128) -> Value {
    Value::UInt(UInt(n))
}
fn asset(issuer: &str, reference: &str) -> AssetId {
    AssetId {
        network: "local".into(),
        issuer: issuer.into(),
        reference: reference.into(),
        scale: 0,
    }
}
fn balance(id: &AssetId, account: &str, amount: u128) -> Balance {
    Balance {
        account: account.into(),
        asset: id.clone(),
        amount: UInt(amount),
    }
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
fn costs(sp: &StagedProgram, action: usize) -> (Vec<u128>, Vec<u128>, u128) {
    let plan = &sp.plan().actions[action];
    (
        plan.segment_cost.iter().map(|c| c.0).collect(),
        plan.committed_before.iter().map(|c| c.0).collect(),
        plan.plan_total.0,
    )
}
/// (advanced, segment, continuation, attempted)
fn waiting(outcome: StageOutcome) -> (bool, Option<StageSegment>, Continuation, UInt) {
    match outcome {
        StageOutcome::Waiting {
            advanced,
            segment,
            continuation,
            attempted,
            ..
        } => (advanced, segment, continuation, attempted),
        other => panic!("expected waiting, got {other:?}"),
    }
}
/// (segment, committed, attempted)
fn complete(outcome: StageOutcome) -> (StageSegment, UInt, UInt) {
    match outcome {
        StageOutcome::Complete {
            segment,
            committed,
            attempted,
            ..
        } => (segment, committed, attempted),
        other => panic!("expected complete, got {other:?}"),
    }
}

// ---------------------------------------------------------------- static plans

#[test]
fn static_plans_match_independent_oracle_counts() {
    for (source, expected) in [
        (ESCROW, (vec![5, 6], vec![0, 5], 11)),
        (RESERVE, (vec![11, 15, 10], vec![0, 11, 26], 36)),
        (CAPTURE, (vec![7, 8], vec![0, 7], 15)),
        (CONSECUTIVE, (vec![7, 5, 6], vec![0, 7, 12], 18)),
        (PLAIN, (vec![8], vec![0], 8)),
    ] {
        let sp = stage::load(source).unwrap();
        assert_eq!(costs(&sp, 0), expected, "{source}");
    }
}

#[test]
fn plan_layout_is_prefix_monotone_inputs_first_and_excludes_states() {
    let sp = stage::load(RESERVE).unwrap();
    let action = &sp.plan().actions[0];
    assert_eq!(action.statement_count, 12);
    let names = |k: usize| -> Vec<&str> {
        action.sites[k]
            .captures
            .iter()
            .map(|s| s.name.as_str())
            .collect()
    };
    assert_eq!(names(0), vec!["extra", "held"]);
    assert_eq!(names(1), vec!["extra", "held", "quote"]);
    assert_eq!(
        action
            .sites
            .iter()
            .map(|s| (s.site, s.ordinal))
            .collect::<Vec<_>>(),
        vec![(1, 3), (2, 8)]
    );
    assert!(
        action.sites[1]
            .captures
            .starts_with(&action.sites[0].captures)
    );
    for site in &action.sites {
        assert!(
            site.captures
                .iter()
                .all(|s| !matches!(s.name.as_str(), "basis" | "allocated" | "marker"))
        );
    }
}

#[test]
fn digests_are_canonical_deterministic_and_bind_every_literal() {
    let a = stage::load(ESCROW).unwrap();
    let b = stage::load(ESCROW).unwrap();
    assert_eq!(a.program_hash(), b.program_hash());
    assert_eq!(a.plan_hash(), b.plan_hash());
    assert_ne!(a.program_hash(), a.plan_hash());
    // Same formula as the backend `/3` program hash.
    let canonical = mori_core::intent::canonical(a.program()).unwrap();
    let expected: [u8; 32] = Sha256::digest(&canonical).into();
    assert_eq!(a.program_hash(), expected);
    assert_eq!(
        a.plan().program_hash,
        mori_core::intent::hex(&a.program_hash())
    );
    assert_eq!(a.plan().format, stage::STAGE_PLAN_FORMAT);
    let canonical_plan = mori_core::intent::canonical(a.plan()).unwrap();
    let expected_plan: [u8; 32] = Sha256::digest(&canonical_plan).into();
    assert_eq!(a.plan_hash(), expected_plan);
    // H1 precondition: one edited literal moves both digests.
    let edited = stage::load(&ESCROW.replace("scale: 0", "scale: 1")).unwrap();
    assert_ne!(edited.program_hash(), a.program_hash());
    assert_ne!(edited.plan_hash(), a.plan_hash());
}

// ------------------------------------------------------------- H26 deterministic

#[test]
fn h26_every_admitted_segment_meters_exactly_its_static_cost() {
    let sp = stage::load(CONSECUTIVE).unwrap();
    let (cost, before, total) = costs(&sp, 0);
    let opening = Scenario {
        inputs: [("x".into(), uint(4))].into(),
        ..Scenario::default()
    };
    let (advanced, segment, first, attempted) =
        waiting(stage::start(&sp, "run", opening, None).unwrap());
    assert!(advanced);
    assert_eq!(attempted.0, cost[0]);
    assert_eq!(first.logical.committed.0, before[1]);
    let segment = segment.unwrap();
    assert_eq!(
        segment.frame_pre,
        stage::PendingFrame {
            site: 0,
            captures: vec![]
        }
    );
    assert_eq!(segment.frame_post.site, 1);
    // False guard: one dispatch plus the guard's three nodes; nothing commits.
    let (advanced, segment, same, attempted) = waiting(
        stage::resume(
            &sp,
            first.clone(),
            event("run", "first", &[("y", uint(6))]),
            None,
            None,
        )
        .unwrap(),
    );
    assert!(!advanced && segment.is_none());
    assert_eq!(attempted, UInt(4));
    assert_eq!(same, first);
    let (_, _, second, attempted) = waiting(
        stage::resume(
            &sp,
            first,
            event("run", "first", &[("y", uint(5))]),
            None,
            None,
        )
        .unwrap(),
    );
    assert_eq!(attempted.0, cost[1]);
    assert_eq!(second.logical.committed.0, before[2]);
    let (segment, committed, attempted) = complete(
        stage::resume(
            &sp,
            second,
            event("run", "second", &[("ready", Value::Bool(true))]),
            None,
            None,
        )
        .unwrap(),
    );
    assert_eq!(attempted.0, cost[2]);
    assert_eq!(committed.0, total);
    assert_eq!(
        segment.frame_post,
        stage::PendingFrame {
            site: 0,
            captures: vec![]
        }
    );
}

#[test]
fn h26_zero_await_action_completes_with_its_single_static_segment() {
    let sp = stage::load(PLAIN).unwrap();
    let opening = Scenario {
        inputs: [("k".into(), uint(21))].into(),
        ..Scenario::default()
    };
    let (segment, committed, attempted) =
        complete(stage::start(&sp, "bump", opening, Some(UInt(8))).unwrap());
    assert_eq!((committed, attempted), (UInt(8), UInt(8)));
    assert_eq!(
        segment.frame_pre,
        stage::PendingFrame {
            site: 0,
            captures: vec![]
        }
    );
    assert_eq!(segment.frame_post, segment.frame_pre);
    assert_eq!(segment.execution.post.state["n"], uint(42));
}

#[test]
fn start_cap_below_plan_total_refuses_before_evaluation() {
    let sp = stage::load(ESCROW).unwrap();
    let id = asset("continuation-escrow-oracle", "PAY");
    let opening = Scenario {
        inputs: [("amount".into(), uint(10))].into(),
        balances: vec![
            balance(&id, "alice", 100),
            balance(&id, "bob", 0),
            balance(&id, "escrow", 0),
        ],
        ..Scenario::default()
    };
    let refused = stage::start(&sp, "deliver", opening.clone(), Some(UInt(10))).unwrap_err();
    assert_eq!(refused.error.code, "mori::core::stage_logical");
    assert_eq!((refused.attempted, refused.committed), (UInt(0), UInt(0)));
    assert!(stage::start(&sp, "deliver", opening, Some(UInt(11))).is_ok());
}

// ------------------------------------------------------------------ H17 property

fn escrow_opening(
    amount: u128,
    alice_extra: u128,
    escrow: u128,
    bob: u128,
    release: Option<u128>,
) -> Scenario {
    let id = asset("continuation-escrow-oracle", "PAY");
    Scenario {
        state: Default::default(),
        inputs: [("amount".into(), uint(amount))].into(),
        balances: vec![
            balance(&id, "alice", amount + alice_extra),
            balance(&id, "bob", bob),
            balance(&id, "escrow", escrow),
            balance(&id, "observer", 42),
        ],
        liabilities: release
            .map(|owed| Liability {
                name: "release".into(),
                asset: id.clone(),
                debtor: "escrow".into(),
                creditor: "bob".into(),
                amount: UInt(owed),
            })
            .into_iter()
            .collect(),
    }
}

fn reserve_opening(basis: u128, extra: u128, reserve: u128, owed: u128) -> Scenario {
    let id = asset("continuation-reserve-oracle", "UNIT");
    Scenario {
        state: [
            ("allocated".into(), uint(0)),
            ("basis".into(), uint(basis)),
            ("marker".into(), uint(77)),
        ]
        .into(),
        inputs: [("extra".into(), uint(extra))].into(),
        balances: vec![
            balance(&id, "observer", 42),
            balance(&id, "reserve", reserve),
            balance(&id, "target", 0),
            balance(&id, "treasury", 0),
        ],
        liabilities: vec![Liability {
            name: "allocation".into(),
            asset: id,
            debtor: "reserve".into(),
            creditor: "target".into(),
            amount: UInt(owed),
        }],
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]

    #[test]
    fn h17_escrow_successful_segments_attempt_exactly_segment_cost(
        amount in 0u128..1_000_000_000,
        alice_extra in 0u128..1_000_000,
        escrow in 0u128..1_000_000,
        bob in 0u128..1_000_000,
        release in proptest::option::of(0u128..1_000_000),
        cap_slack in proptest::option::of(0u128..100),
        false_first in any::<bool>(),
    ) {
        let sp = stage::load(ESCROW).unwrap();
        let (cost, before, total) = costs(&sp, 0);
        let cap = cap_slack.map(|slack| UInt(total + slack));
        let opening = escrow_opening(amount, alice_extra, escrow, bob, release);
        let (advanced, _, mut pending, attempted) =
            waiting(stage::start(&sp, "deliver", opening, cap).unwrap());
        prop_assert!(advanced);
        prop_assert_eq!(attempted.0, cost[0]);
        prop_assert_eq!(pending.logical.committed.0, before[1]);
        prop_assert_eq!(pending.logical.cap, cap);
        if false_first {
            let (advanced, _, same, attempted) = waiting(stage::resume(
                &sp, pending.clone(), event("deliver", "delivery", &[("ready", Value::Bool(false))]),
                None, None).unwrap());
            prop_assert!(!advanced);
            prop_assert_eq!(attempted, UInt(2));
            prop_assert_eq!(&same, &pending);
            pending = same;
        }
        let (_, committed, attempted) = complete(stage::resume(
            &sp, pending, event("deliver", "delivery", &[("ready", Value::Bool(true))]),
            None, None).unwrap());
        prop_assert_eq!(attempted.0, cost[1]);
        prop_assert_eq!(committed.0, total);
    }

    #[test]
    fn h17_capture_successful_segments_attempt_exactly_segment_cost(
        x in 0u128..1_000_000_000_000,
        opening_level in 0u128..1_000_000_000_000,
        edited_level in proptest::option::of(0u128..1_000_000_000_000),
        miss in 1u128..1_000,
    ) {
        let sp = stage::load(CAPTURE).unwrap();
        let (cost, before, total) = costs(&sp, 0);
        let opening = Scenario {
            state: [("level".into(), uint(opening_level))].into(),
            inputs: [("x".into(), uint(x))].into(),
            ..Scenario::default()
        };
        let (_, _, mut pending, attempted) = waiting(stage::start(&sp, "a", opening, None).unwrap());
        prop_assert_eq!(attempted.0, cost[0]);
        prop_assert_eq!(pending.logical.committed.0, before[1]);
        // The suffix reads the current boundary; any well-typed level is admitted.
        if let Some(level) = edited_level {
            pending.boundary.state.insert("level".into(), uint(level));
        }
        let (advanced, _, _, attempted) = waiting(stage::resume(
            &sp, pending.clone(), event("a", "go", &[("z", uint(x + 1 + miss))]), None, None).unwrap());
        prop_assert!(!advanced);
        prop_assert_eq!(attempted, UInt(4));
        let (segment, committed, attempted) = complete(stage::resume(
            &sp, pending, event("a", "go", &[("z", uint(x + 1))]), None, None).unwrap());
        prop_assert_eq!(attempted.0, cost[1]);
        prop_assert_eq!(committed.0, total);
        let level = edited_level.unwrap_or(x + 1);
        prop_assert_eq!(&segment.execution.post.state["level"], &uint(level + x + 1));
    }

    #[test]
    fn h17_reserve_successful_segments_attempt_exactly_segment_cost(
        basis in 5u128..=40,
        extra in 0u128..=10,
        reserve_extra in 0u128..1_000_000,
        owed in 0u128..1_000_000,
        quote_offset in 0u128..=50,
        false_approval in any::<bool>(),
        false_settlement in any::<bool>(),
    ) {
        let sp = stage::load(RESERVE).unwrap();
        let (cost, before, total) = costs(&sp, 0);
        let held = basis + extra;
        let quote = held + quote_offset % (51 - held);
        let opening = reserve_opening(basis, extra, held + extra + reserve_extra, owed);
        let (_, _, mut first, attempted) =
            waiting(stage::start(&sp, "allocate", opening, Some(UInt(total))).unwrap());
        prop_assert_eq!(attempted.0, cost[0]);
        prop_assert_eq!(first.logical.committed.0, before[1]);
        if false_approval && held > 0 {
            let (advanced, _, same, attempted) = waiting(stage::resume(
                &sp, first.clone(), event("allocate", "approval", &[("quote", uint(held - 1))]),
                None, None).unwrap());
            prop_assert!(!advanced);
            prop_assert_eq!(attempted, UInt(4));
            first = same;
        }
        let (advanced, _, mut second, attempted) = waiting(stage::resume(
            &sp, first, event("allocate", "approval", &[("quote", uint(quote))]), None, None).unwrap());
        prop_assert!(advanced);
        prop_assert_eq!(attempted.0, cost[1]);
        prop_assert_eq!(second.logical.committed.0, before[2]);
        if false_settlement {
            let (advanced, _, same, attempted) = waiting(stage::resume(
                &sp, second.clone(), event("allocate", "settlement", &[("ready", Value::Bool(false))]),
                None, None).unwrap());
            prop_assert!(!advanced);
            prop_assert_eq!(attempted, UInt(2));
            second = same;
        }
        let (_, committed, attempted) = complete(stage::resume(
            &sp, second, event("allocate", "settlement", &[("ready", Value::Bool(true))]),
            None, Some(UInt(total))).unwrap());
        prop_assert_eq!(attempted.0, cost[2]);
        prop_assert_eq!(committed.0, total);
    }
}

// --------------------------------------------------------- H22 core validation

fn refused(source: &str) -> mori_core::Error {
    stage::load(source).expect_err(source)
}
fn action_source(body: &str) -> String {
    format!(
        r#"language "moriarty/2"; agreement V {{
  asset PAY = {{ network: "local", issuer: "v", reference: "PAY", scale: 0 }};
  state s: UInt = 0;
  action a(i: UInt) {{ {body} }}
}}"#
    )
}

#[test]
fn h22_await_validation_codes() {
    let cases: Vec<(String, &str)> = vec![
        (
            action_source("await p(r: Bool) when r; await p(q: Bool) when q;"),
            "duplicate",
        ),
        (action_source("await p(s: UInt) when s == 0;"), "duplicate"),
        (action_source("await p(i: UInt) when i == 0;"), "duplicate"),
        (
            action_source("let l = i; await p(l: UInt) when l == 0;"),
            "duplicate",
        ),
        (
            action_source("await p(r: Bool) when r; await q(r: Bool) when r;"),
            "duplicate",
        ),
        (
            action_source("await p(r: Bool, r: Bool) when r;"),
            "duplicate",
        ),
        (action_source("await p(PAY: Bool) when PAY;"), "duplicate"),
        (
            action_source("await p(r: Bool) when r; let r = 1;"),
            "duplicate",
        ),
        (action_source("await p(r: UInt) when r;"), "type"),
        (action_source("await p(r: UInt) when r + 1;"), "type"),
        (action_source("await p() when unknown;"), "name"),
        (
            action_source("await p() when r; await q(r: Bool) when r;"),
            "name",
        ),
        (action_source("let await = 1;"), "duplicate"),
        (action_source("let when = 1;"), "duplicate"),
        (action_source("await when(r: Bool) when r;"), "duplicate"),
        (
            action_source("await p(await: Bool) when true;"),
            "duplicate",
        ),
    ];
    for (source, code) in cases {
        let error = refused(&source);
        assert_eq!(error.code, format!("mori::core::{code}"), "{source}");
    }
}

#[test]
fn h22_await_and_layout_bounds() {
    let awaits: String = (0..17)
        .map(|k| format!("await w{k}() when true; "))
        .collect();
    assert_eq!(refused(&action_source(&awaits)).code, "mori::core::bound");
    let sixteen: String = (0..16)
        .map(|k| format!("await w{k}() when true; "))
        .collect();
    assert_eq!(
        stage::load(&action_source(&sixteen))
            .unwrap()
            .plan()
            .actions[0]
            .sites
            .len(),
        16
    );
    let binders: Vec<String> = (0..17).map(|k| format!("b{k}: Bool")).collect();
    let too_many = action_source(&format!("await p({}) when true;", binders.join(", ")));
    assert_eq!(refused(&too_many).code, "mori::core::bound");
    let binders16 = action_source(&format!("await p({}) when true;", binders[..16].join(", ")));
    assert!(stage::load(&binders16).is_ok());
    // 256 inputs fill the layout; one more let before an await exceeds it.
    let inputs: Vec<String> = (0..256).map(|k| format!("i{k}: UInt")).collect();
    let wide = |body: &str| {
        format!(
            r#"language "moriarty/2"; agreement W {{ action a({}) {{ {body} }} }}"#,
            inputs.join(", ")
        )
    };
    assert_eq!(
        stage::load(&wide("await p() when true;"))
            .unwrap()
            .plan()
            .actions[0]
            .sites[0]
            .captures
            .len(),
        256
    );
    assert_eq!(
        refused(&wide("let z = 1; await p() when true;")).code,
        "mori::core::bound"
    );
}

#[test]
fn requires_stays_a_predicate_and_profiles_stay_isolated() {
    let sp = stage::load(&action_source("requires i > 0;")).unwrap();
    assert!(sp.plan().actions[0].sites.is_empty());
    assert!(matches!(
        sp.program().actions[0].statements[0],
        mori_core::Statement::Requires { .. }
    ));
    // `/1` source cannot load as staged, and `/2` source fails legacy parse.
    let legacy =
        r#"language "moriarty/1"; agreement L { state s: UInt = 0; action a() { next s = 1; } }"#;
    assert_eq!(refused(legacy).code, "mori::core::profile");
    let staged = action_source("next s = i;");
    assert_eq!(
        mori_core::parse(&staged).unwrap_err().code,
        "mori::core::profile"
    );
    // /1 keeps await and when as ordinary names (C6).
    let names = r#"language "moriarty/1"; agreement L { state s: UInt = 0; action a() { let await = 1; let when = await; next s = when; } }"#;
    assert!(mori_core::parse(names).is_ok());
    // Profile helpers keep the legacy error identity.
    let core1 = mori_core::parse(legacy).unwrap();
    assert!(stage::require_core1(&core1).is_ok());
    assert_eq!(
        stage::require_core2(&core1).unwrap_err().code,
        "mori::core::profile"
    );
    let core2 = stage::load(&staged).unwrap();
    assert!(stage::require_core2(core2.program()).is_ok());
    assert_eq!(
        stage::require_core1(core2.program()).unwrap_err().code,
        "mori::core::profile"
    );
    let mut relabelled = core2.program().clone();
    relabelled.language = mori_core::LANGUAGE.into();
    relabelled.semantics = mori_core::SEMANTICS.into();
    relabelled.actions[0]
        .statements
        .push(mori_core::Statement::Await {
            label: "x".into(),
            binders: vec![],
            guard: 0,
        });
    let error = stage::require_core1(&relabelled).unwrap_err();
    assert_eq!(error.code, "mori::core::profile");
}

/// `/2` validation keeps every legacy constraint with identical identity:
/// the same body under `/1` `parse` and `/2` `load` fails the same way.
#[test]
fn legacy_constraints_have_identical_errors_under_both_profiles() {
    let bodies = [
        "transfer(NOPE, \"a\", \"b\", i);",
        "next s = true;",
        "next missing = 1;",
        "let s = 1;",
        "let x = 1; let x = 2;",
        "requires i;",
        "transfer(PAY, \"\", \"b\", i);",
        "liability(\"\", PAY, \"d\", \"c\", i);",
        "discharge(\"\", i);",
        "emit \"\"(i);",
        "requires liability_present(\"orphan\");",
        "transfer(PAY, \"a\", \"b\", true);",
        "let y = i + true;",
        "requires not i;",
    ];
    for body in bodies {
        let staged = action_source(body);
        let legacy = staged.replace("moriarty/2", "moriarty/1");
        let v1 = mori_core::parse(&legacy).expect_err(body);
        let v2 = refused(&staged);
        assert_eq!(
            (&v2.code, &v2.message, v2.step),
            (&v1.code, &v1.message, v1.step),
            "{body}"
        );
    }
}
