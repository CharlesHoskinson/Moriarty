use mori_core::policy::{
    Cost, FeeCapability, LocalObservation, SponsorReservation, SponsorScope, SponsoredRequest,
    WorkBudget, prepare_sponsored_execution,
};
use mori_core::{Error, Program, Scenario, UInt, Value, parse, simulate};

const COUNTER: &str = r#"language "moriarty/1";
agreement Counter {
 state count: UInt = 0;
 action increment(amount: UInt) {
   let total = count + amount;
   requires total <= 100;
   next count = total;
   ensures count == total;
   emit "count"(count);
 }
 action double() { next count = count * 2; }
}"#;

const OTHER: &str = r#"language "moriarty/1";
agreement Other {
 state count: UInt = 0;
 action increment(amount: UInt) { next count = amount; }
}"#;

fn scope() -> SponsorScope {
    SponsorScope {
        grant: "grant".into(),
        network: "local".into(),
        anchor: "anchor".into(),
        program: "Counter".into(),
        action: "increment".into(),
        beneficiary: "alice".into(),
    }
}

fn new_grant(cap: u128) -> FeeCapability {
    let scope = scope();
    FeeCapability::new(
        &scope.grant,
        &scope.network,
        &scope.anchor,
        &scope.program,
        &scope.action,
        &scope.beneficiary,
        UInt(cap),
    )
}

fn new_budget() -> WorkBudget {
    let mut budget = WorkBudget::new(Cost {
        logical: UInt(10),
        dust: UInt(10),
        service: UInt(10),
        external: UInt(10),
    });
    budget
        .reserve_recovery(mori_core::policy::CostPlane::Logical, UInt(4))
        .unwrap();
    budget
}

fn logical(amount: u128) -> Cost {
    Cost {
        logical: UInt(amount),
        ..Cost::default()
    }
}

fn program() -> Program {
    let program = parse(COUNTER).unwrap();
    assert_eq!(program.name, "Counter");
    program
}

fn scenario_with(amount: u128) -> Scenario {
    let mut scenario = Scenario::default();
    scenario
        .inputs
        .insert("amount".into(), Value::UInt(UInt(amount)));
    scenario
}

fn request<'a>(
    candidate: &'a str,
    scope: &'a SponsorScope,
    program: &'a Program,
    scenario: &'a Scenario,
    dust: u128,
    ordinary: Cost,
    observation: LocalObservation,
) -> SponsoredRequest<'a> {
    SponsoredRequest {
        candidate,
        scope,
        dust: UInt(dust),
        ordinary,
        program,
        action: "increment",
        scenario,
        observation,
    }
}

fn sponsor_error(error: Error, message: &str) {
    assert_eq!(error.code, "mori::core::sponsor");
    assert_eq!(error.message, message);
    assert_eq!(error.step, None);
}

#[test]
fn source_success_matches_evaluator_and_keeps_recovery_slice() {
    let program = program();
    let scenario = scenario_with(7);
    let scope = scope();
    let mut grant = new_grant(10);
    let mut budget = new_budget();
    let prepared = prepare_sponsored_execution(
        &mut grant,
        &mut budget,
        request(
            "attempt-a",
            &scope,
            &program,
            &scenario,
            5,
            logical(2),
            LocalObservation::Hold,
        ),
    )
    .unwrap();
    assert_eq!(
        prepared.execution,
        simulate(&program, "increment", &scenario).unwrap()
    );
    assert_eq!(prepared.execution.post.state["count"], Value::UInt(UInt(7)));
    assert_eq!(scenario.inputs["amount"], Value::UInt(UInt(7)));
    assert_eq!(
        grant.reservation("attempt-a"),
        Some(SponsorReservation::Reserved(UInt(5)))
    );
    assert_eq!(grant.spent(), UInt(0));
    assert_eq!(grant.available().unwrap(), UInt(5));
    assert_eq!(budget.ordinary_used().logical, UInt(2));
    assert_eq!(budget.ordinary_used().dust, UInt(0));
    assert_eq!(budget.ordinary_used().service, UInt(0));
    assert_eq!(budget.ordinary_used().external, UInt(0));
    assert_eq!(budget.recovery_reserved().logical, UInt(4));
    assert_eq!(budget.recovery_used(), Cost::default());
}

#[test]
fn evaluator_failure_restores_sponsor_and_budget_atomically() {
    let program = program();
    let scenario = scenario_with(101);
    let scope = scope();
    let mut grant = new_grant(10);
    let mut budget = new_budget();
    let grant_before = grant.clone();
    let budget_before = budget.clone();
    let error = prepare_sponsored_execution(
        &mut grant,
        &mut budget,
        request(
            "attempt-a",
            &scope,
            &program,
            &scenario,
            5,
            logical(2),
            LocalObservation::Settle {
                actual: UInt(4),
                authenticated: true,
            },
        ),
    )
    .unwrap_err();
    assert_eq!(error.code, "mori::core::requires");
    assert_eq!(error.message, "Action predicate evaluated to false.");
    assert_eq!(error.step, Some(1));
    assert_eq!(grant, grant_before);
    assert_eq!(budget, budget_before);
    assert_eq!(grant.reservation("attempt-a"), None);
    let retry = scenario_with(3);
    let prepared = prepare_sponsored_execution(
        &mut grant,
        &mut budget,
        request(
            "attempt-a",
            &scope,
            &program,
            &retry,
            5,
            logical(2),
            LocalObservation::Hold,
        ),
    )
    .unwrap();
    assert_eq!(prepared.execution.post.state["count"], Value::UInt(UInt(3)));
    assert_eq!(
        grant.reservation("attempt-a"),
        Some(SponsorReservation::Reserved(UInt(5)))
    );
    assert_eq!(budget.ordinary_used().logical, UInt(2));
    assert_eq!(budget.recovery_reserved().logical, UInt(4));
}

#[test]
fn interleaved_candidates_stop_at_the_same_cap_boundary() {
    let program = program();
    let scenario = scenario_with(1);
    let scope = scope();
    let mut grant = new_grant(10);
    let mut budget = new_budget();
    for (candidate, dust) in [("attempt-a", 6u128), ("attempt-b", 4)] {
        prepare_sponsored_execution(
            &mut grant,
            &mut budget,
            request(
                candidate,
                &scope,
                &program,
                &scenario,
                dust,
                logical(1),
                LocalObservation::Hold,
            ),
        )
        .unwrap();
    }
    assert_eq!(grant.available().unwrap(), UInt(0));
    assert_eq!(grant.spent(), UInt(0));
    assert_eq!(budget.ordinary_used().logical, UInt(2));
    let at_cap = grant.clone();
    let budget_at_cap = budget.clone();
    let error = prepare_sponsored_execution(
        &mut grant,
        &mut budget,
        request(
            "attempt-c",
            &scope,
            &program,
            &scenario,
            1,
            logical(1),
            LocalObservation::Hold,
        ),
    )
    .unwrap_err();
    sponsor_error(error, "insufficient sponsor capacity");
    assert_eq!(grant, at_cap);
    assert_eq!(budget, budget_at_cap);
    assert_eq!(grant.reservation("attempt-c"), None);

    let mut exact = new_grant(10);
    let mut exact_budget = new_budget();
    prepare_sponsored_execution(
        &mut exact,
        &mut exact_budget,
        request(
            "attempt-a",
            &scope,
            &program,
            &scenario,
            10,
            logical(1),
            LocalObservation::Hold,
        ),
    )
    .unwrap();
    let filled = exact.clone();
    let filled_budget = exact_budget.clone();
    let error = prepare_sponsored_execution(
        &mut exact,
        &mut exact_budget,
        request(
            "attempt-b",
            &scope,
            &program,
            &scenario,
            1,
            logical(1),
            LocalObservation::Hold,
        ),
    )
    .unwrap_err();
    sponsor_error(error, "insufficient sponsor capacity");
    assert_eq!(exact, filled);
    assert_eq!(exact_budget, filled_budget);
    assert_eq!(exact.available().unwrap(), UInt(0));
}

#[test]
fn timeout_cannot_release_without_authenticated_unused_premise() {
    let program = program();
    let scenario = scenario_with(4);
    let scope = scope();
    let mut grant = new_grant(10);
    let mut budget = new_budget();
    prepare_sponsored_execution(
        &mut grant,
        &mut budget,
        request(
            "attempt-a",
            &scope,
            &program,
            &scenario,
            7,
            logical(2),
            LocalObservation::Timeout,
        ),
    )
    .unwrap();
    assert_eq!(
        grant.reservation("attempt-a"),
        Some(SponsorReservation::Unknown(UInt(7)))
    );
    assert_eq!(grant.spent(), UInt(0));
    assert_eq!(grant.available().unwrap(), UInt(3));
    assert_eq!(budget.ordinary_used().logical, UInt(2));
    assert_eq!(budget.recovery_reserved().logical, UInt(4));
    let held = grant.clone();
    let held_budget = budget.clone();
    sponsor_error(
        grant.release_unused("attempt-a").unwrap_err(),
        "cannot release an unknown or finalized spend",
    );
    sponsor_error(
        grant
            .release_authenticated_unused("attempt-a", false)
            .unwrap_err(),
        "authenticated unused observation required",
    );
    sponsor_error(
        grant
            .settle_authenticated("attempt-a", UInt(1), false)
            .unwrap_err(),
        "authenticated settlement observation required",
    );
    assert_eq!(grant, held);
    assert_eq!(budget, held_budget);
    let error = prepare_sponsored_execution(
        &mut grant,
        &mut budget,
        request(
            "attempt-b",
            &scope,
            &program,
            &scenario,
            4,
            logical(1),
            LocalObservation::Hold,
        ),
    )
    .unwrap_err();
    sponsor_error(error, "insufficient sponsor capacity");
    assert_eq!(grant, held);
    assert_eq!(budget, held_budget);
    grant
        .release_authenticated_unused("attempt-a", true)
        .unwrap();
    assert_eq!(
        grant.reservation("attempt-a"),
        Some(SponsorReservation::Released)
    );
    assert_eq!(grant.spent(), UInt(0));
    assert_eq!(grant.available().unwrap(), UInt(10));
    assert_eq!(budget.ordinary_used().logical, UInt(2));
    sponsor_error(
        grant
            .reserve(
                "attempt-a",
                "local",
                "anchor",
                "Counter",
                "increment",
                "alice",
                UInt(1),
            )
            .unwrap_err(),
        "reservation identity already consumed",
    );
}

#[test]
fn scope_and_overflow_refusals_keep_every_counter() {
    let program = program();
    let other = parse(OTHER).unwrap();
    let scenario = scenario_with(2);
    let scope = scope();
    let mut grant = new_grant(10);
    let mut budget = new_budget();
    prepare_sponsored_execution(
        &mut grant,
        &mut budget,
        request(
            "attempt-a",
            &scope,
            &program,
            &scenario,
            3,
            logical(1),
            LocalObservation::Hold,
        ),
    )
    .unwrap();
    let saved_grant = grant.clone();
    let saved_budget = budget.clone();
    let mut attempts: Vec<(SponsorScope, &Program, &str, &str, u128)> = Vec::new();
    let mut wrong_anchor = scope.clone();
    wrong_anchor.anchor = "other-anchor".into();
    let mut wrong_network = scope.clone();
    wrong_network.network = "other-network".into();
    let mut wrong_program = scope.clone();
    wrong_program.program = "Other".into();
    let mut wrong_action = scope.clone();
    wrong_action.action = "double".into();
    let mut wrong_beneficiary = scope.clone();
    wrong_beneficiary.beneficiary = "bob".into();
    let mut wrong_grant = scope.clone();
    wrong_grant.grant = "other-grant".into();
    let mut empty_beneficiary = scope.clone();
    empty_beneficiary.beneficiary.clear();
    attempts.push((wrong_anchor, &program, "increment", "attempt-b", 1));
    attempts.push((wrong_network, &program, "increment", "attempt-b", 1));
    attempts.push((wrong_program, &program, "increment", "attempt-b", 1));
    attempts.push((wrong_action, &program, "double", "attempt-b", 1));
    attempts.push((wrong_beneficiary, &program, "increment", "attempt-b", 1));
    attempts.push((wrong_grant, &program, "increment", "attempt-b", 1));
    attempts.push((empty_beneficiary, &program, "increment", "attempt-b", 1));
    attempts.push((scope.clone(), &other, "increment", "attempt-b", 1));
    attempts.push((scope.clone(), &program, "increment", "", 1));
    attempts.push((scope.clone(), &program, "increment", "attempt-a", 1));
    for (candidate_scope, candidate_program, action, candidate, dust) in attempts {
        let error = prepare_sponsored_execution(
            &mut grant,
            &mut budget,
            SponsoredRequest {
                candidate,
                scope: &candidate_scope,
                dust: UInt(dust),
                ordinary: logical(1),
                program: candidate_program,
                action,
                scenario: &scenario,
                observation: LocalObservation::Hold,
            },
        )
        .unwrap_err();
        assert_eq!(error.code, "mori::core::sponsor");
        assert_eq!(error.step, None);
        assert_eq!(grant, saved_grant);
        assert_eq!(budget, saved_budget);
    }
    sponsor_error(
        prepare_sponsored_execution(
            &mut grant,
            &mut budget,
            request(
                "attempt-b",
                &scope,
                &program,
                &scenario,
                0,
                logical(1),
                LocalObservation::Hold,
            ),
        )
        .unwrap_err(),
        "dust reservation must be positive",
    );
    assert_eq!(grant, saved_grant);
    assert_eq!(budget, saved_budget);

    let mut wide = WorkBudget::new(Cost {
        logical: UInt(u128::MAX),
        dust: UInt(1),
        service: UInt(1),
        external: UInt(1),
    });
    wide.charge_ordinary(mori_core::policy::CostPlane::Logical, UInt(1))
        .unwrap();
    let mut open = new_grant(10);
    let open_before = open.clone();
    let wide_before = wide.clone();
    let error = prepare_sponsored_execution(
        &mut open,
        &mut wide,
        request(
            "attempt-z",
            &scope,
            &program,
            &scenario,
            1,
            logical(u128::MAX),
            LocalObservation::Hold,
        ),
    )
    .unwrap_err();
    assert_eq!(error.code, "mori::core::meter");
    assert_eq!(error.message, "cost counter overflow");
    assert_eq!(error.step, None);
    assert_eq!(open, open_before);
    assert_eq!(wide, wide_before);
    assert_eq!(wide.ordinary_used().logical, UInt(1));
}

#[test]
fn settlement_below_reservation_debits_actual_once() {
    let program = program();
    let scenario = scenario_with(8);
    let scope = scope();
    let mut grant = new_grant(20);
    let mut budget = new_budget();
    let prepared = prepare_sponsored_execution(
        &mut grant,
        &mut budget,
        SponsoredRequest {
            candidate: "attempt-a",
            scope: &scope,
            dust: UInt(9),
            ordinary: Cost {
                logical: UInt(1),
                dust: UInt(1),
                service: UInt(0),
                external: UInt(0),
            },
            program: &program,
            action: "increment",
            scenario: &scenario,
            observation: LocalObservation::Settle {
                actual: UInt(4),
                authenticated: true,
            },
        },
    )
    .unwrap();
    assert_eq!(
        prepared.execution,
        simulate(&program, "increment", &scenario).unwrap()
    );
    assert_eq!(
        grant.reservation("attempt-a"),
        Some(SponsorReservation::Settled(UInt(4)))
    );
    assert_eq!(grant.spent(), UInt(4));
    assert_eq!(grant.available().unwrap(), UInt(16));
    assert_eq!(budget.ordinary_used().logical, UInt(1));
    assert_eq!(budget.ordinary_used().dust, UInt(1));
    assert_eq!(budget.recovery_reserved().logical, UInt(4));
    let settled = grant.clone();
    let settled_budget = budget.clone();
    sponsor_error(
        grant
            .settle_authenticated("attempt-a", UInt(4), true)
            .unwrap_err(),
        "reservation is absent or finalized",
    );
    assert_eq!(grant, settled);
    assert_eq!(budget, settled_budget);
    grant
        .reserve(
            "attempt-b",
            "local",
            "anchor",
            "Counter",
            "increment",
            "alice",
            UInt(16),
        )
        .unwrap();
    assert!(
        grant
            .reserve(
                "attempt-c",
                "local",
                "anchor",
                "Counter",
                "increment",
                "alice",
                UInt(1),
            )
            .is_err()
    );
    assert_eq!(grant.spent(), UInt(4));

    let mut later = new_grant(20);
    let mut later_budget = new_budget();
    prepare_sponsored_execution(
        &mut later,
        &mut later_budget,
        request(
            "attempt-a",
            &scope,
            &program,
            &scenario,
            9,
            logical(1),
            LocalObservation::Timeout,
        ),
    )
    .unwrap();
    later
        .settle_authenticated("attempt-a", UInt(4), true)
        .unwrap();
    assert_eq!(
        later.reservation("attempt-a"),
        Some(SponsorReservation::Settled(UInt(4)))
    );
    assert_eq!(later.spent(), UInt(4));
    assert_eq!(later.available().unwrap(), UInt(16));
    assert_eq!(later_budget.ordinary_used().logical, UInt(1));
    assert_eq!(later_budget.ordinary_used().dust, UInt(0));

    let mut over = new_grant(20);
    let mut over_budget = new_budget();
    let over_grant = over.clone();
    let over_budget_before = over_budget.clone();
    sponsor_error(
        prepare_sponsored_execution(
            &mut over,
            &mut over_budget,
            SponsoredRequest {
                candidate: "attempt-a",
                scope: &scope,
                dust: UInt(9),
                ordinary: logical(1),
                program: &program,
                action: "increment",
                scenario: &scenario,
                observation: LocalObservation::Settle {
                    actual: UInt(10),
                    authenticated: true,
                },
            },
        )
        .unwrap_err(),
        "settled DUST exceeds reserved authority",
    );
    sponsor_error(
        prepare_sponsored_execution(
            &mut over,
            &mut over_budget,
            SponsoredRequest {
                candidate: "attempt-a",
                scope: &scope,
                dust: UInt(9),
                ordinary: logical(1),
                program: &program,
                action: "increment",
                scenario: &scenario,
                observation: LocalObservation::Settle {
                    actual: UInt(4),
                    authenticated: false,
                },
            },
        )
        .unwrap_err(),
        "authenticated settlement observation required",
    );
    assert_eq!(over, over_grant);
    assert_eq!(over_budget, over_budget_before);
}

#[test]
fn ordinary_prepare_cannot_consume_the_recovery_slice() {
    let program = program();
    let scenario = scenario_with(1);
    let scope = scope();
    let mut grant = new_grant(10);
    let mut budget = new_budget();
    let grant_before = grant.clone();
    let budget_before = budget.clone();
    let error = prepare_sponsored_execution(
        &mut grant,
        &mut budget,
        request(
            "attempt-a",
            &scope,
            &program,
            &scenario,
            2,
            logical(7),
            LocalObservation::Hold,
        ),
    )
    .unwrap_err();
    assert_eq!(error.code, "mori::core::meter");
    assert_eq!(
        error.message,
        "ordinary work would consume reserved recovery"
    );
    assert_eq!(grant, grant_before);
    assert_eq!(budget, budget_before);
    prepare_sponsored_execution(
        &mut grant,
        &mut budget,
        request(
            "attempt-a",
            &scope,
            &program,
            &scenario,
            2,
            logical(6),
            LocalObservation::Hold,
        ),
    )
    .unwrap();
    assert_eq!(budget.ordinary_used().logical, UInt(6));
    assert_eq!(budget.recovery_reserved().logical, UInt(4));
    assert_eq!(budget.recovery_used().logical, UInt(0));
    assert_eq!(grant.spent(), UInt(0));
    assert_eq!(grant.available().unwrap(), UInt(8));
}
