//! Chip-level recording tests for `NativeChip` synthesis under the
//! public `run_test_native_gadget!` harness (`MockProver`).
//!
//! These tests assert only what the `MockProver` synthesis path
//! records: the `NativeChip::configure` binding and the region
//! bases seen by the floor planner. They are not production
//! keygen/proving capture: no keygen `Assembly` or `parse_advices`
//! hook runs, so `k`/`num_rows` stay `None` and no fixed, selector,
//! advice, or copy table is recorded. Sessions taken here must not
//! be joined.

use std::collections::BTreeSet;

use midnight_proofs::plonk::native_add_view::{
    begin_recording, take_last_keygen_recording, CapturePhase, RecordedSession,
};

fn distinct_region_bases(session: &RecordedSession, name: &str) -> BTreeSet<usize> {
    session
        .regions
        .iter()
        .filter(|region| region.name == name)
        .map(|region| region.region_start)
        .collect()
}

fn take_mock_prover_session() -> RecordedSession {
    let session = take_last_keygen_recording()
        .expect("MockProver synthesis on this thread published a session");
    // The declared label is only a declaration; the MockProver path
    // supplies no domain dimensions and no keygen/proving tables.
    assert_eq!(session.declared_phase, CapturePhase::Keygen);
    assert_eq!(session.k, None);
    assert_eq!(session.num_rows, None);
    assert!(session.fixed_values.is_empty());
    assert!(session.selectors.is_empty());
    assert!(session.advice_values.is_empty());
    assert!(session.copy_pairs.is_empty());
    assert_eq!(session.config_bindings.len(), 1);
    assert_eq!(session.config_bindings[0].value_cols.len(), 5);
    assert_eq!(session.config_bindings[0].coeff_cols.len(), 5);
    session
}

#[test]
fn chip_empty_terms_create_no_linear_combination_region() {
    let guard = begin_recording(CapturePhase::Keygen, 1);
    midnight_circuits::run_test_native_gadget!(chip, layouter, {
        // Empty term list: `linear_combination` serves the constant
        // from `assign_fixed` and never enters the multi-term path.
        let _constant = chip.linear_combination(&mut layouter, &[], F::from(0))?;
    });
    drop(guard);
    let session = take_mock_prover_session();
    assert!(distinct_region_bases(&session, "Linear combination").is_empty());
}

#[test]
fn chip_singleton_identity_creates_no_linear_combination_region() {
    let guard = begin_recording(CapturePhase::Keygen, 1);
    midnight_circuits::run_test_native_gadget!(chip, layouter, {
        let x = chip.assign(&mut layouter, Value::known(F::from(7)))?;
        // One term, coefficient 1, zero constant: the identity early
        // return hands `x` back without a "Linear combination" region.
        let _same = chip.linear_combination(
            &mut layouter,
            &[(F::from(1), x.clone())],
            F::from(0),
        )?;
    });
    drop(guard);
    let session = take_mock_prover_session();
    assert!(distinct_region_bases(&session, "Linear combination").is_empty());
}

#[test]
fn chip_multi_term_add_creates_one_linear_combination_region() {
    let guard = begin_recording(CapturePhase::Keygen, 1);
    midnight_circuits::run_test_native_gadget!(chip, layouter, {
        let x = chip.assign(&mut layouter, Value::known(F::from(2)))?;
        let y = chip.assign(&mut layouter, Value::known(F::from(3)))?;
        let res = chip.add(&mut layouter, &x, &y)?;
        chip.assert_equal_to_fixed(&mut layouter, &res, F::from(5))?;
    });
    drop(guard);
    let session = take_mock_prover_session();
    assert_eq!(
        distinct_region_bases(&session, "Linear combination").len(),
        1
    );
}
