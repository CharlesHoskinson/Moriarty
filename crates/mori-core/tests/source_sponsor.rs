//! Independent primitive hand counts through the exact-source consumer.
use mori_core::*;

const COUNTER: &str = r#"language "moriarty/1"; agreement Counter {
state count: UInt = 0;
action run(amount: UInt) {
let total = count + amount;
requires total <= 100;
next count = total;
ensures count == total;
emit "count"(count);
} }"#;
const PAYMENT: &str = r#"language "moriarty/1"; agreement Payment {
asset A = {network: "local", issuer: "i", reference: "A", scale: 0};
action run(amount: UInt) {
transfer(A, "alice", "bob", amount);
fee(A, "alice", "treasury", 2);
} }"#;
const EMPTY: &str = r#"language "moriarty/1"; agreement Empty { action run() {} }"#;
const FALSE: &str =
    r#"language "moriarty/1"; agreement False { action run() { requires false; } }"#;
fn config(total: u128, recovery: u128, per_call: u128, dust: u128) -> LocalSourceSponsorConfig {
    LocalSourceSponsorConfig {
        grant: "grant".into(),
        network: "local".into(),
        anchor: "anchor".into(),
        action: "run".into(),
        beneficiary: "solver".into(),
        dust_cap: UInt(dust),
        logical_total: UInt(total),
        logical_recovery_reserved: UInt(recovery),
        logical_per_call: UInt(per_call),
    }
}
fn owner(
    source: &str,
    total: u128,
    recovery: u128,
    per_call: u128,
    dust: u128,
) -> LocalSourceSponsor {
    LocalSourceSponsor::from_trusted_local_source(source, config(total, recovery, per_call, dust))
        .unwrap()
}
fn counter_scenario(amount: u128) -> Scenario {
    Scenario {
        inputs: [("amount".into(), Value::UInt(UInt(amount)))].into(),
        ..Scenario::default()
    }
}
fn prepare(
    owner: &mut LocalSourceSponsor,
    source: &str,
    candidate: &str,
    scenario: &Scenario,
    dust: u128,
) -> Result<SourceSponsoredExecution, SourceSponsoredError> {
    let scope = owner.scope().clone();
    owner.prepare(SourceSponsoredRequest {
        candidate,
        scope: &scope,
        source,
        scenario,
        dust_reservation: UInt(dust),
    })
}
fn report(cost: &CostReport, cap: u128, used: u128) {
    assert_eq!(cost.profile, "moriarty-reference-logical-cost/1");
    assert_eq!(cost.cap, UInt(cap));
    assert_eq!(cost.used, UInt(used));
}
fn accounting(
    owner: &LocalSourceSponsor,
    used: u128,
    remaining: u128,
    available: u128,
    spent: u128,
) {
    assert_eq!(owner.logical_used(), UInt(used));
    assert_eq!(owner.ordinary_remaining().unwrap(), UInt(remaining));
    assert_eq!(owner.dust_available().unwrap(), UInt(available));
    assert_eq!(owner.dust_spent(), UInt(spent));
}
fn refusal(error: &SourceSponsoredError, code: &str, step: Option<u16>, cap: u128, used: u128) {
    assert_eq!(error.error.code, format!("mori::core::{code}"));
    assert_eq!(error.error.step, step);
    report(
        error.cost.as_ref().expect("entered real metered consumer"),
        cap,
        used,
    );
}
#[test]
fn counter_exact_cap_preserves_complete_execution_and_reserves_economics() {
    let mut sponsor = owner(COUNTER, 32, 0, 16, 10);
    let scenario = counter_scenario(7);
    let before = scenario.clone();
    let ordinary = simulate(&parse(COUNTER).unwrap(), "run", &scenario).unwrap();
    let result = prepare(&mut sponsor, COUNTER, "a", &scenario, 1);
    assert!(result.is_ok(), "exact cap must execute: {result:?}");
    let result = result.unwrap();
    report(&result.cost, 16, 16); // 4+4+2+4+2, independently counted.
    assert_eq!(result.execution, ordinary);
    assert_eq!(
        serde_json::to_vec(&result.execution).unwrap(),
        serde_json::to_vec(&ordinary).unwrap()
    );
    assert_eq!(result.candidate, "a");
    assert_eq!(&result.scope, sponsor.scope());
    accounting(&sponsor, 16, 16, 9, 0);
    assert_eq!(scenario, before);
}
#[test]
fn counter_cap15_retains_failed_work_without_hold_and_keeps_id_reusable() {
    let mut sponsor = owner(COUNTER, 31, 0, 15, 10);
    let scenario = counter_scenario(7);
    let before = scenario.clone();
    let error = prepare(&mut sponsor, COUNTER, "a", &scenario, 1).unwrap_err();
    refusal(&error, "meter", Some(4), 15, 15);
    assert_eq!(error.error.message, "cost plane cap exceeded");
    accounting(&sponsor, 15, 16, 10, 0);
    // Same candidate reaches the interpreter again rather than duplicate-ID rejection.
    refusal(
        &prepare(&mut sponsor, COUNTER, "a", &scenario, 1).unwrap_err(),
        "meter",
        Some(4),
        15,
        15,
    );
    accounting(&sponsor, 30, 1, 10, 0);
    assert_eq!(scenario, before);
}
#[test]
fn counter_repeated_failed_id_exhausts_only_ordinary_work() {
    let mut sponsor = owner(COUNTER, 20, 4, 15, 10);
    let scenario = counter_scenario(7);
    let before = scenario.clone();
    for (cap, step, cumulative) in [(15, 4, 15), (1, 0, 16), (0, 0, 16)] {
        refusal(
            &prepare(&mut sponsor, COUNTER, "a", &scenario, 1).unwrap_err(),
            "meter",
            Some(step),
            cap,
            cap,
        );
        accounting(&sponsor, cumulative, 16 - cumulative, 10, 0);
        assert_eq!(scenario, before);
    }
}
#[test]
fn counter_predicate_failure_charges8_and_same_id_retry_uses_remaining() {
    let mut sponsor = owner(COUNTER, 20, 0, 16, 10);
    let scenario = counter_scenario(101);
    let before = scenario.clone();
    let first = prepare(&mut sponsor, COUNTER, "a", &scenario, 1).unwrap_err();
    refusal(&first, "requires", Some(1), 16, 8);
    assert_eq!(
        first.error,
        simulate(&parse(COUNTER).unwrap(), "run", &scenario).unwrap_err()
    );
    accounting(&sponsor, 8, 12, 10, 0);
    refusal(
        &prepare(&mut sponsor, COUNTER, "a", &scenario, 1).unwrap_err(),
        "requires",
        Some(1),
        12,
        8,
    );
    accounting(&sponsor, 16, 4, 10, 0);
    refusal(
        &prepare(&mut sponsor, COUNTER, "a", &scenario, 1).unwrap_err(),
        "meter",
        Some(1),
        4,
        4,
    );
    accounting(&sponsor, 20, 0, 10, 0);
    assert_eq!(scenario, before);
}
fn preflight(error: &SourceSponsoredError) {
    assert_eq!(error.error.code, "mori::core::sponsor");
    assert_eq!(error.error.offset, None);
    assert_eq!(error.error.step, None);
    assert_eq!(error.cost, None);
}
#[test]
fn binding_matches_independent_exact_five_field_sha256_oracle() {
    let sponsor = owner(COUNTER, 16, 0, 16, 10);
    let binding = &sponsor.scope().binding;
    assert_eq!(binding.profile, "moriarty-local-source-sponsor/1");
    assert_eq!(binding.language, "moriarty/1");
    assert_eq!(binding.semantics, "moriarty-core/1");
    assert_eq!(
        binding.logical_cost_profile,
        "moriarty-reference-logical-cost/1"
    );
    assert_eq!(COUNTER.len(), 212);
    assert_eq!(
        binding.source_hash,
        "8e89db83018345a771224e3b20218eb06eb905562d4d3bc5f239b3aff41f07b0"
    );
    assert_eq!(
        binding.program_binding,
        "9554a52379f1d916364f33ea98ed41c27bafa4f2aaa8aef4bea85113ecb43451"
    );
}
#[test]
fn binding_all11_diagnostic_fields_refuse_before_invalid_scenario() {
    let mut sponsor = owner(COUNTER, 32, 0, 16, 10);
    let invalid = Scenario::default();
    let before = invalid.clone();
    for field in 0..11 {
        let mut scope = sponsor.scope().clone();
        let value = match field {
            0 => &mut scope.grant,
            1 => &mut scope.network,
            2 => &mut scope.anchor,
            3 => &mut scope.action,
            4 => &mut scope.beneficiary,
            5 => &mut scope.binding.profile,
            6 => &mut scope.binding.language,
            7 => &mut scope.binding.semantics,
            8 => &mut scope.binding.logical_cost_profile,
            9 => &mut scope.binding.source_hash,
            10 => &mut scope.binding.program_binding,
            _ => unreachable!(),
        };
        value.push_str("-changed");
        let error = sponsor
            .prepare(SourceSponsoredRequest {
                candidate: "a",
                scope: &scope,
                source: COUNTER,
                scenario: &invalid,
                dust_reservation: UInt(1),
            })
            .unwrap_err();
        preflight(&error);
        accounting(&sponsor, 0, 32, 10, 0);
        assert_eq!(invalid, before);
    }
    prepare(&mut sponsor, COUNTER, "a", &counter_scenario(7), 1).unwrap();
    accounting(&sponsor, 16, 16, 9, 0);
}
#[test]
fn binding_raw_source_body_space_comment_crlf_and_language_are_distinct() {
    let mut sponsor = owner(COUNTER, 32, 0, 16, 10);
    let scenario = counter_scenario(7);
    let before = scenario.clone();
    for changed in [
        COUNTER.replace("<= 100", "<= 99"),
        format!("{COUNTER} "),
        format!("{COUNTER}\n// same names"),
        COUNTER.replace('\n', "\r\n"),
        COUNTER.replace("moriarty/1", "moriarty/2"),
    ] {
        preflight(&prepare(&mut sponsor, &changed, "a", &scenario, 1).unwrap_err());
        accounting(&sponsor, 0, 32, 10, 0);
        assert_eq!(scenario, before);
    }
    // Binding metadata forged from the replacement cannot replace private authority.
    let changed = COUNTER.replace("<= 100", "<= 99");
    let other = owner(&changed, 32, 0, 16, 10);
    let error = sponsor
        .prepare(SourceSponsoredRequest {
            candidate: "a",
            scope: other.scope(),
            source: &changed,
            scenario: &scenario,
            dust_reservation: UInt(1),
        })
        .unwrap_err();
    preflight(&error);
    prepare(&mut sponsor, COUNTER, "a", &scenario, 1).unwrap();
}
#[test]
fn preflight_capacity_and_zero_reservation_precede_false_predicate() {
    let mut sponsor = owner(FALSE, 8, 0, 2, 1);
    let scenario = Scenario::default();
    let before = scenario.clone();
    let insufficient = prepare(&mut sponsor, FALSE, "a", &scenario, 2).unwrap_err();
    preflight(&insufficient);
    assert_eq!(insufficient.error.message, "insufficient sponsor capacity");
    accounting(&sponsor, 0, 8, 1, 0);
    preflight(&prepare(&mut sponsor, FALSE, "a", &scenario, 0).unwrap_err());
    accounting(&sponsor, 0, 8, 1, 0);
    let reached = prepare(&mut sponsor, FALSE, "a", &scenario, 1).unwrap_err();
    refusal(&reached, "requires", Some(0), 2, 2);
    assert_eq!(
        reached.error,
        simulate(&parse(FALSE).unwrap(), "run", &scenario).unwrap_err()
    );
    accounting(&sponsor, 2, 6, 1, 0);
    assert_eq!(scenario, before);
}
#[test]
fn preflight_consumed_id_precedes_bad_input_and_empty_id_is_unused() {
    let mut sponsor = owner(COUNTER, 48, 0, 16, 10);
    let valid = counter_scenario(7);
    preflight(&prepare(&mut sponsor, COUNTER, "", &valid, 1).unwrap_err());
    accounting(&sponsor, 0, 48, 10, 0);
    prepare(&mut sponsor, COUNTER, "a", &valid, 1).unwrap();
    let invalid = Scenario::default();
    let before = invalid.clone();
    let error = prepare(&mut sponsor, COUNTER, "a", &invalid, 1).unwrap_err();
    preflight(&error);
    assert_eq!(error.error.message, "reservation identity already consumed");
    accounting(&sponsor, 16, 32, 9, 0);
    assert_eq!(invalid, before);
    prepare(&mut sponsor, COUNTER, "b", &valid, 1).unwrap();
    accounting(&sponsor, 32, 16, 8, 0);
}
#[test]
fn normalization_errors_preserve_original_diagnostics_zero_report_and_reusable_id() {
    let program = parse(COUNTER).unwrap();
    let mut sponsor = owner(COUNTER, 20, 4, u128::MAX, 10);
    let mut extra = counter_scenario(7);
    extra.inputs.insert("extra".into(), Value::UInt(UInt(0)));
    let wrong = Scenario {
        inputs: [("amount".into(), Value::Bool(true))].into(),
        ..Scenario::default()
    };
    for scenario in [Scenario::default(), extra, wrong] {
        let before = scenario.clone();
        let ordinary = simulate(&program, "run", &scenario).unwrap_err();
        let error = prepare(&mut sponsor, COUNTER, "a", &scenario, 1).unwrap_err();
        report(error.cost.as_ref().unwrap(), 16, 0);
        assert_eq!(error.error, ordinary);
        assert_eq!(error.error.step, None);
        accounting(&sponsor, 0, 16, 10, 0);
        assert_eq!(scenario, before);
    }
    prepare(&mut sponsor, COUNTER, "a", &counter_scenario(7), 1).unwrap();
    accounting(&sponsor, 16, 0, 9, 0);
}
#[test]
fn constructor_rejects_each_empty_scope_missing_action_invalid_source_and_recovery_over_total() {
    for field in 0..5 {
        let mut cfg = config(16, 0, 16, 10);
        match field {
            0 => cfg.grant.clear(),
            1 => cfg.network.clear(),
            2 => cfg.anchor.clear(),
            3 => cfg.action.clear(),
            4 => cfg.beneficiary.clear(),
            _ => unreachable!(),
        }
        let result = LocalSourceSponsor::from_trusted_local_source(COUNTER, cfg);
        assert!(result.is_err(), "empty scope field {field} must refuse");
        assert_eq!(result.unwrap_err().code, "mori::core::sponsor");
    }
    let mut cfg = config(16, 0, 16, 10);
    cfg.action = "missing".into();
    assert_eq!(
        LocalSourceSponsor::from_trusted_local_source(COUNTER, cfg)
            .unwrap_err()
            .code,
        "mori::core::sponsor"
    );
    for invalid in [
        "invalid".to_string(),
        COUNTER.replace("moriarty/1", "moriarty/2"),
    ] {
        let expected = parse(&invalid).unwrap_err();
        assert_eq!(
            LocalSourceSponsor::from_trusted_local_source(&invalid, config(16, 0, 16, 10))
                .unwrap_err(),
            expected
        );
    }
    assert_eq!(
        LocalSourceSponsor::from_trusted_local_source(COUNTER, config(3, 4, 16, 10))
            .unwrap_err()
            .code,
        "mori::core::sponsor"
    );
}
fn payment_scenario(amount: u128, alice: u128) -> Scenario {
    let program = parse(PAYMENT).unwrap();
    Scenario {
        inputs: [("amount".into(), Value::UInt(UInt(amount)))].into(),
        balances: [("alice", alice), ("bob", 0), ("treasury", 0)]
            .into_iter()
            .map(|(account, amount)| Balance {
                account: account.into(),
                asset: program.assets[0].identity.clone(),
                amount: UInt(amount),
            })
            .collect(),
        ..Scenario::default()
    }
}
#[test]
fn payment_cost4_preserves_both_economic_effects_and_gross_debit9() {
    let mut sponsor = owner(PAYMENT, 8, 0, 4, 10);
    let scenario = payment_scenario(7, 20);
    let before = scenario.clone();
    let ordinary = simulate(&parse(PAYMENT).unwrap(), "run", &scenario).unwrap();
    let result = prepare(&mut sponsor, PAYMENT, "a", &scenario, 1).unwrap();
    report(&result.cost, 4, 4); // transfer statement+name; fee statement+literal.
    assert_eq!(result.execution, ordinary);
    assert_eq!(
        serde_json::to_vec(&result.execution).unwrap(),
        serde_json::to_vec(&ordinary).unwrap()
    );
    assert_eq!(result.execution.effects.len(), 2);
    assert!(matches!(
        result.execution.effects[0],
        Effect::Transfer {
            amount: UInt(7),
            ..
        }
    ));
    assert!(matches!(
        result.execution.effects[1],
        Effect::Fee {
            amount: UInt(2),
            ..
        }
    ));
    assert_eq!(
        result
            .execution
            .post
            .balances
            .iter()
            .map(|b| (&*b.account, b.amount))
            .collect::<Vec<_>>(),
        vec![("alice", UInt(11)), ("bob", UInt(7)), ("treasury", UInt(2))]
    );
    assert_eq!(
        scenario.balances[0].amount.0 - result.execution.post.balances[0].amount.0,
        9
    );
    accounting(&sponsor, 4, 4, 9, 0); // reservation, not settled DUST.
    assert_eq!(scenario, before);
}
#[test]
fn payment_cap3_late_meter_failure_rolls_back_economics_but_retains_work() {
    let mut sponsor = owner(PAYMENT, 8, 0, 3, 10);
    let scenario = payment_scenario(7, 20);
    let before = scenario.clone();
    let error = prepare(&mut sponsor, PAYMENT, "a", &scenario, 1).unwrap_err();
    refusal(&error, "meter", Some(1), 3, 3);
    accounting(&sponsor, 3, 5, 10, 0);
    assert_eq!(scenario, before);
    // Reusing a failed id enters the interpreter and is charged again.
    refusal(
        &prepare(&mut sponsor, PAYMENT, "a", &scenario, 1).unwrap_err(),
        "meter",
        Some(1),
        3,
        3,
    );
    accounting(&sponsor, 6, 2, 10, 0);
}
#[test]
fn payment_insufficient_fee_after_transfer_keeps_full_error_and_id_can_succeed_later() {
    let mut sponsor = owner(PAYMENT, 8, 0, 4, 10);
    let scenario = payment_scenario(7, 8);
    let before = scenario.clone();
    let expected = simulate(&parse(PAYMENT).unwrap(), "run", &scenario).unwrap_err();
    let error = prepare(&mut sponsor, PAYMENT, "a", &scenario, 1).unwrap_err();
    report(error.cost.as_ref().unwrap(), 4, 4);
    assert_eq!(error.error, expected);
    assert_eq!(error.error.step, Some(1));
    accounting(&sponsor, 4, 4, 10, 0);
    assert_eq!(scenario, before);
    let retry = payment_scenario(0, 8);
    let retry_before = retry.clone();
    let result = prepare(&mut sponsor, PAYMENT, "a", &retry, 1).unwrap();
    report(&result.cost, 4, 4);
    assert_eq!(
        result.execution,
        simulate(&parse(PAYMENT).unwrap(), "run", &retry).unwrap()
    );
    accounting(&sponsor, 8, 0, 9, 0);
    assert_eq!(retry, retry_before);
}
#[test]
fn eager_underflow_and_requires_failure_preserve_charge_before_error_precedence() {
    let eager = r#"language "moriarty/1"; agreement Eager { action run() { requires false and (0 - 1 == 0); } }"#;
    let scenario = Scenario::default();
    let before = scenario.clone();
    for (source, cap, code) in [
        (eager, 7, "underflow"),
        (eager, 6, "meter"),
        (FALSE, 1, "meter"),
        (FALSE, 2, "requires"),
    ] {
        let mut sponsor = owner(source, 20, 0, cap, 10);
        let error = prepare(&mut sponsor, source, "a", &scenario, 1).unwrap_err();
        refusal(&error, code, Some(0), cap, cap);
        if code != "meter" {
            assert_eq!(
                error.error,
                simulate(&parse(source).unwrap(), "run", &scenario).unwrap_err()
            );
        } else {
            assert_eq!(error.error.message, "cost plane cap exceeded");
        }
        accounting(&sponsor, cap, 20 - cap, 10, 0);
        assert_eq!(scenario, before);
    }
}
#[test]
fn unknown_hold_keeps_capacity_and_settlement_releases_only_unused_dust() {
    let mut sponsor = owner(COUNTER, 64, 0, 16, 10);
    let scenario = counter_scenario(7);
    let before = scenario.clone();
    prepare(&mut sponsor, COUNTER, "a", &scenario, 10).unwrap();
    accounting(&sponsor, 16, 48, 0, 0);
    sponsor.mark_unknown("a").unwrap();
    accounting(&sponsor, 16, 48, 0, 0);
    let insufficient = prepare(&mut sponsor, COUNTER, "b", &Scenario::default(), 1).unwrap_err();
    preflight(&insufficient);
    assert_eq!(insufficient.error.message, "insufficient sponsor capacity");
    let release = sponsor.release_unused("a").unwrap_err();
    assert_eq!(
        release.message,
        "cannot release an unknown or finalized spend"
    );
    accounting(&sponsor, 16, 48, 0, 0);
    sponsor
        .settle_from_trusted_local_observation("a", UInt(7))
        .unwrap();
    accounting(&sponsor, 16, 48, 3, 7);
    let repeat = sponsor
        .settle_from_trusted_local_observation("a", UInt(7))
        .unwrap_err();
    assert_eq!(repeat.message, "reservation is absent or finalized");
    preflight(&prepare(&mut sponsor, COUNTER, "a", &scenario, 1).unwrap_err());
    accounting(&sponsor, 16, 48, 3, 7);
    assert_eq!(scenario, before);
    // B's denied reservation was discarded; next real call retains the per-call cap16.
    let next = prepare(&mut sponsor, COUNTER, "b", &scenario, 1).unwrap();
    report(&next.cost, 16, 16);
    accounting(&sponsor, 32, 32, 2, 7);
}
#[test]
fn release_unused_restores_dust_preserves_work_and_keeps_successful_id_consumed() {
    let mut sponsor = owner(COUNTER, 52, 4, 16, 10);
    let scenario = counter_scenario(7);
    prepare(&mut sponsor, COUNTER, "a", &scenario, 1).unwrap();
    accounting(&sponsor, 16, 32, 9, 0);
    sponsor.release_unused("a").unwrap();
    accounting(&sponsor, 16, 32, 10, 0);
    let reused = prepare(&mut sponsor, COUNTER, "a", &scenario, 1).unwrap_err();
    preflight(&reused);
    assert_eq!(
        reused.error.message,
        "reservation identity already consumed"
    );
    accounting(&sponsor, 16, 32, 10, 0);
    let next = prepare(&mut sponsor, COUNTER, "b", &scenario, 1).unwrap();
    report(&next.cost, 16, 16);
    accounting(&sponsor, 32, 16, 9, 0);
}
#[test]
fn interleaved_pending_unknown_settled_released_holds_preserve_unrelated_accounting() {
    let mut sponsor = owner(COUNTER, 104, 4, 16, 10);
    let scenario = counter_scenario(7);
    let before = scenario.clone();
    prepare(&mut sponsor, COUNTER, "a", &scenario, 4).unwrap();
    prepare(&mut sponsor, COUNTER, "b", &scenario, 6).unwrap();
    accounting(&sponsor, 32, 68, 0, 0);
    sponsor.mark_unknown("a").unwrap();
    accounting(&sponsor, 32, 68, 0, 0);
    assert!(sponsor.release_unused("a").is_err());
    assert!(
        sponsor
            .settle_from_trusted_local_observation("a", UInt(5))
            .is_err()
    );
    accounting(&sponsor, 32, 68, 0, 0);
    sponsor
        .settle_from_trusted_local_observation("a", UInt(3))
        .unwrap();
    accounting(&sponsor, 32, 68, 1, 3);
    sponsor.release_unused("b").unwrap();
    accounting(&sponsor, 32, 68, 7, 3);
    for id in ["a", "b", "absent"] {
        assert!(sponsor.mark_unknown(id).is_err());
        accounting(&sponsor, 32, 68, 7, 3);
        assert!(sponsor.release_unused(id).is_err());
        accounting(&sponsor, 32, 68, 7, 3);
        assert!(
            sponsor
                .settle_from_trusted_local_observation(id, UInt(0))
                .is_err()
        );
        accounting(&sponsor, 32, 68, 7, 3);
    }
    for id in ["a", "b"] {
        let error = prepare(&mut sponsor, COUNTER, id, &Scenario::default(), 1).unwrap_err();
        preflight(&error);
        assert_eq!(error.error.message, "reservation identity already consumed");
        accounting(&sponsor, 32, 68, 7, 3);
    }
    assert_eq!(scenario, before);
}
#[test]
fn maximal_uint_pending_and_settled_accounting_never_wraps() {
    let max = u128::MAX;
    let scenario = Scenario::default();
    let before = scenario.clone();
    let mut sponsor = owner(EMPTY, max, max, 0, max);
    accounting(&sponsor, 0, 0, max, 0);
    let result = prepare(&mut sponsor, EMPTY, "a", &scenario, max).unwrap();
    report(&result.cost, 0, 0);
    assert_eq!(
        result.execution,
        simulate(&parse(EMPTY).unwrap(), "run", &scenario).unwrap()
    );
    accounting(&sponsor, 0, 0, 0, 0);
    preflight(&prepare(&mut sponsor, EMPTY, "b", &scenario, 1).unwrap_err());
    accounting(&sponsor, 0, 0, 0, 0);
    sponsor
        .settle_from_trusted_local_observation("a", UInt(max))
        .unwrap();
    accounting(&sponsor, 0, 0, 0, max);
    assert_eq!(scenario, before);

    let mut split = owner(EMPTY, max, max, 0, max);
    prepare(&mut split, EMPTY, "a", &scenario, max - 1).unwrap();
    accounting(&split, 0, 0, 1, 0);
    assert!(
        split
            .settle_from_trusted_local_observation("a", UInt(max))
            .is_err()
    );
    accounting(&split, 0, 0, 1, 0);
    prepare(&mut split, EMPTY, "b", &scenario, 1).unwrap();
    accounting(&split, 0, 0, 0, 0);
    preflight(&prepare(&mut split, EMPTY, "c", &scenario, 1).unwrap_err());
    accounting(&split, 0, 0, 0, 0);
    split
        .settle_from_trusted_local_observation("a", UInt(max - 1))
        .unwrap();
    accounting(&split, 0, 0, 0, max - 1);
    split
        .settle_from_trusted_local_observation("b", UInt(1))
        .unwrap();
    accounting(&split, 0, 0, 0, max);
    assert!(
        split
            .settle_from_trusted_local_observation("b", UInt(1))
            .is_err()
    );
    accounting(&split, 0, 0, 0, max);
    assert_eq!(scenario, before);
}
#[test]
fn two_counter_calls_reset_interpreter_meter_but_share_lifetime_allowance() {
    let scenario = counter_scenario(7);
    let before = scenario.clone();
    for total in [32, 31] {
        let mut sponsor = owner(COUNTER, total, 0, 16, 2);
        let first = prepare(&mut sponsor, COUNTER, "a", &scenario, 1).unwrap();
        report(&first.cost, 16, 16);
        accounting(&sponsor, 16, total - 16, 1, 0);
        let second = prepare(&mut sponsor, COUNTER, "b", &scenario, 1);
        if total == 32 {
            let second = second.unwrap();
            report(&second.cost, 16, 16);
            assert_eq!(first.execution, second.execution);
            accounting(&sponsor, 32, 0, 0, 0);
        } else {
            refusal(&second.unwrap_err(), "meter", Some(4), 15, 15);
            accounting(&sponsor, 31, 0, 1, 0);
            refusal(
                &prepare(&mut sponsor, COUNTER, "b", &scenario, 1).unwrap_err(),
                "meter",
                Some(0),
                0,
                0,
            );
            accounting(&sponsor, 31, 0, 1, 0);
        }
        assert_eq!(scenario, before);
    }
}
#[test]
fn fresh_trusted_owners_have_independent_local_authority_and_work() {
    let mut first = owner(COUNTER, 16, 0, 16, 1);
    let mut second = owner(COUNTER, 16, 0, 16, 1);
    assert_eq!(first.scope(), second.scope());
    let scenario = counter_scenario(7);
    prepare(&mut first, COUNTER, "a", &scenario, 1).unwrap();
    accounting(&first, 16, 0, 0, 0);
    accounting(&second, 0, 16, 1, 0);
    prepare(&mut second, COUNTER, "a", &scenario, 1).unwrap();
    second
        .settle_from_trusted_local_observation("a", UInt(1))
        .unwrap();
    accounting(&first, 16, 0, 0, 0);
    accounting(&second, 16, 0, 0, 1);
    first.release_unused("a").unwrap();
    accounting(&first, 16, 0, 1, 0);
    accounting(&second, 16, 0, 0, 1);
}
#[test]
fn successful_work_exhaustion_protects_recovery_after_each_economic_lifecycle() {
    let scenario = counter_scenario(7);
    let before = scenario.clone();
    for lifecycle in ["release", "unknown", "settle"] {
        let mut sponsor = owner(COUNTER, 20, 4, u128::MAX, 10);
        let result = prepare(&mut sponsor, COUNTER, "a", &scenario, 1).unwrap();
        report(&result.cost, 16, 16);
        accounting(&sponsor, 16, 0, 9, 0);
        let (available, spent) = match lifecycle {
            "release" => {
                sponsor.release_unused("a").unwrap();
                (10, 0)
            }
            "unknown" => {
                sponsor.mark_unknown("a").unwrap();
                (9, 0)
            }
            "settle" => {
                sponsor
                    .settle_from_trusted_local_observation("a", UInt(1))
                    .unwrap();
                (9, 1)
            }
            _ => unreachable!(),
        };
        accounting(&sponsor, 16, 0, available, spent);
        for _ in 0..2 {
            let refused = prepare(&mut sponsor, COUNTER, "b", &scenario, 1).unwrap_err();
            refusal(&refused, "meter", Some(0), 0, 0);
            accounting(&sponsor, 16, 0, available, spent);
        }
        assert_eq!(scenario, before);
    }
}
#[test]
fn zero_cap_empty_action_success_differs_from_positive_work_and_zero_dust_capacity() {
    let scenario = Scenario::default();
    let mut empty = owner(EMPTY, 0, 0, 0, 1);
    report(
        &prepare(&mut empty, EMPTY, "a", &scenario, 1).unwrap().cost,
        0,
        0,
    );
    accounting(&empty, 0, 0, 0, 0);
    empty.release_unused("a").unwrap();
    accounting(&empty, 0, 0, 1, 0);
    preflight(&prepare(&mut empty, EMPTY, "a", &scenario, 1).unwrap_err());
    let mut positive = owner(COUNTER, 0, 0, 0, 1);
    refusal(
        &prepare(&mut positive, COUNTER, "a", &counter_scenario(7), 1).unwrap_err(),
        "meter",
        Some(0),
        0,
        0,
    );
    accounting(&positive, 0, 0, 1, 0);
    let mut no_dust = owner(EMPTY, 0, 0, 0, 0);
    preflight(&prepare(&mut no_dust, EMPTY, "a", &scenario, 1).unwrap_err());
    preflight(&prepare(&mut no_dust, EMPTY, "a", &scenario, 0).unwrap_err());
    accounting(&no_dust, 0, 0, 0, 0);
}
