//! Profile /2 (`sherlock-helper-microstep/2`) suspension slice 1: the Waiting
//! outcome, Await control and LogicalCap exhaust reason, their profile gating
//! in the step invariants (SI-01/SI-02/SI-20, design note §3.2 and SI-27), and
//! the not-steppable Waiting rule (§6.2). The resume relation is a later slice.

use super::{
    BoundsC, ControlC, CoreExpr, CoreProgram, CoreValue, ExhaustReason, FinanceInput,
    LayoutCapacitiesC, LayoutSchemaC, OutcomeC, OwnerEngine, PROFILE_ID_MICROSTEP_1,
    PROFILE_ID_MICROSTEP_2, PROFILE_MICROSTEP_2, PreImage, RunIntC, StepInvClauseId, StepResult,
};
use std::sync::Arc;

fn bounds() -> BoundsC {
    BoundsC {
        max_arity: 2,
        frames: 64,
        max_reference_work: 1024,
        max_helper_work: 1024,
        step_cap: 1,
        serialization_bytes: 1_000_000,
        code_rows: 3,
        function_rows: 1,
        code_child_slots: 1,
        parameter_slots: 1,
        constant_nodes: 1,
        ..BoundsC::default()
    }
}

fn layout(bounds: BoundsC) -> LayoutSchemaC {
    LayoutSchemaC {
        layout_version: ("t".into(), "1".into()),
        field_id: None,
        widths: None,
        scalar_layout_id: None,
        bounds,
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

/// A pure literal program admitted and started; `profile` is written into the
/// admission identity before the genesis image binds it.
fn image(profile: u8) -> (super::Admitted, super::MachineImage) {
    let program = CoreProgram {
        functions: Vec::new(),
        entry: CoreExpr::Literal {
            value: CoreValue::UInt {
                n: super::i128_class::KIntC::InRange(7),
            },
        },
    };
    let b = bounds();
    let engine = super::owner_host::host_owner_engine();
    let mut admitted = engine.admit(&program, &b, &layout(b)).expect("admission");
    Arc::get_mut(&mut admitted.body)
        .expect("sole admission owner")
        .identity
        .profile_id = profile;
    let img = engine
        .admit_run(
            &admitted,
            &RunIntC::Nat(8),
            &RunIntC::Nat(64),
            &FinanceInput::Absent,
        )
        .expect("genesis image");
    (admitted, img)
}

fn suspend(img: &mut super::MachineImage, site: u16) {
    img.outcome = OutcomeC::Waiting { site };
    img.control = ControlC::Await {
        site,
        expr: img.control_origin,
    };
}

fn clauses(a: &super::Admitted, img: &super::MachineImage) -> Vec<StepInvClauseId> {
    let pre = PreImage::bind(a, img).expect("bind");
    match super::owner_host::host_owner_engine().check_step_inv(&pre) {
        Ok(()) => Vec::new(),
        Err(v) => v.into_iter().map(|x| x.clause).collect(),
    }
}

#[test]
fn profile_constants_are_distinct_and_named() {
    assert_eq!(PROFILE_ID_MICROSTEP_1, 2);
    assert_eq!(PROFILE_ID_MICROSTEP_2, 3);
    assert_eq!(PROFILE_MICROSTEP_2, "sherlock-helper-microstep/2");
}

#[test]
fn genesis_images_pass_step_invariants_under_both_profiles() {
    for profile in [PROFILE_ID_MICROSTEP_1, PROFILE_ID_MICROSTEP_2] {
        let (a, img) = image(profile);
        assert_eq!(clauses(&a, &img), Vec::new(), "profile {profile}");
    }
    // Any other profile id is refused by SI-01.
    let (a, img) = image(4);
    assert!(clauses(&a, &img).contains(&StepInvClauseId::SI01));
}

#[test]
fn waiting_with_matching_await_is_admitted_only_under_profile_2() {
    let (a, mut img) = image(PROFILE_ID_MICROSTEP_2);
    suspend(&mut img, 1);
    assert_eq!(clauses(&a, &img), Vec::new());
    // The same suspended image under /1 rejects (SI-02 Waiting payload).
    let (a1, mut img1) = image(PROFILE_ID_MICROSTEP_1);
    suspend(&mut img1, 1);
    assert!(clauses(&a1, &img1).contains(&StepInvClauseId::SI02));
}

#[test]
fn waiting_and_await_must_agree() {
    // Waiting over a non-Await control rejects (SI-02, SI-27).
    let (a, mut img) = image(PROFILE_ID_MICROSTEP_2);
    img.outcome = OutcomeC::Waiting { site: 1 };
    assert!(clauses(&a, &img).contains(&StepInvClauseId::SI02));
    // Await control under a Running outcome rejects (SI-20 converse).
    let (a, mut img) = image(PROFILE_ID_MICROSTEP_2);
    img.control = ControlC::Await {
        site: 1,
        expr: img.control_origin,
    };
    assert!(clauses(&a, &img).contains(&StepInvClauseId::SI20));
    // Mismatched sites reject.
    let (a, mut img) = image(PROFILE_ID_MICROSTEP_2);
    suspend(&mut img, 1);
    img.outcome = OutcomeC::Waiting { site: 2 };
    let got = clauses(&a, &img);
    assert!(got.contains(&StepInvClauseId::SI02) && got.contains(&StepInvClauseId::SI20));
    // Site 0 is not a site (sites are 1-based).
    let (a, mut img) = image(PROFILE_ID_MICROSTEP_2);
    suspend(&mut img, 0);
    assert!(clauses(&a, &img).contains(&StepInvClauseId::SI02));
    // An Await whose expr differs from the control origin rejects (SI-20).
    let (a, mut img) = image(PROFILE_ID_MICROSTEP_2);
    suspend(&mut img, 1);
    img.control = ControlC::Await {
        site: 1,
        expr: img.control_origin + 1,
    };
    assert!(clauses(&a, &img).contains(&StepInvClauseId::SI20));
}

#[test]
fn logical_cap_exhaustion_is_profile_2_only() {
    let reason = ExhaustReason::LogicalCap;
    let (a, mut img) = image(PROFILE_ID_MICROSTEP_2);
    img.outcome = OutcomeC::Exhausted { reason };
    assert!(!clauses(&a, &img).contains(&StepInvClauseId::SI02));
    let (a, mut img) = image(PROFILE_ID_MICROSTEP_1);
    img.outcome = OutcomeC::Exhausted { reason };
    assert!(clauses(&a, &img).contains(&StepInvClauseId::SI02));
}

#[test]
fn waiting_image_is_not_steppable_and_unchanged() {
    let engine = super::owner_host::host_owner_engine();
    let (a, mut img) = image(PROFILE_ID_MICROSTEP_2);
    suspend(&mut img, 1);
    let before = (
        img.outcome,
        img.control,
        img.control_origin,
        img.remaining_work,
        img.consumed_work,
        img.reference_cursor,
    );
    match engine.step(&a, &mut img) {
        Ok(StepResult::Terminal) => {}
        other => panic!(
            "Waiting must be Terminal/no-next, got {:?}",
            other.map(|_| ())
        ),
    }
    let after = (
        img.outcome,
        img.control,
        img.control_origin,
        img.remaining_work,
        img.consumed_work,
        img.reference_cursor,
    );
    assert_eq!(before, after, "a Waiting step must change nothing");
    // And select refuses Await control rather than inventing a site.
    let pre = PreImage::bind(&a, &img).expect("bind");
    assert!(engine.select(&pre).is_err());
}
