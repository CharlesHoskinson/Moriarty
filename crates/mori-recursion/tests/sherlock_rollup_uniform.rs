#![cfg(feature = "sherlock-rollup-prefix")]
use midnight_proofs::{circuit::Value, dev::MockProver};
use midnight_zk_stdlib::{MidnightCircuit, Relation, cost_model};
use mori_recursion::{
    rollup::uniform::{EVENTS, UniformBatch, UniformWitness},
    util::F,
};
use std::path::PathBuf;

fn corpus() -> PathBuf {
    std::env::var_os("FRTF_V3_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../../../projectfenrir-main/recursion/rollup-test-format/v3")
        })
}

#[test]
fn all_six_batches_share_one_serialized_relation_and_match_b2() {
    let mut serialized = Vec::new();
    UniformBatch.write_relation(&mut serialized).unwrap();
    for batch in 1..=6 {
        let (r, w, expected) = UniformBatch::load(&corpus(), batch).unwrap();
        let mut bytes = Vec::new();
        r.write_relation(&mut bytes).unwrap();
        assert_eq!(bytes, serialized);
        assert_eq!(r.instance(&w).unwrap(), expected);
        assert_eq!(w.states.len(), EVENTS + 1);
        let mut indices = [w.tx_events, w.close_events].concat();
        indices.sort();
        assert_eq!(indices, (0..EVENTS as u32).collect::<Vec<_>>());
    }
}

#[test]
fn uniform_batch_fits_k20() {
    let model = cost_model(&UniformBatch, Some(20));
    eprintln!(
        "uniform batch layout {}",
        serde_json::to_string(&model).unwrap()
    );
    assert_eq!(model.k, 20, "uniform batch exceeds the production bound");
    assert!(model.rows + model.nb_unusable_rows < 1 << 20);
}

fn accepts(r: &UniformBatch, w: UniformWitness, instance: &[F]) -> bool {
    let c = MidnightCircuit::new(
        r,
        Value::known(instance.to_vec()),
        Value::known(w),
        Some(20),
    );
    MockProver::run(&c, vec![vec![], instance.to_vec()])
        .unwrap()
        .verify()
        .is_ok()
}

#[test]
#[ignore = "whole fixed-key batch; run through DoctorWatson heavy"]
fn fixed_key_batch_accepts_and_rejects_trace_mutations() {
    let (r, w, instance) = UniformBatch::load(&corpus(), 1).unwrap();
    assert!(accepts(&r, w.clone(), &instance));
    let mut changed = w.clone();
    changed.tx_events[1] = changed.tx_events[0];
    assert!(
        !accepts(&r, changed, &instance),
        "duplicate transaction edge"
    );
    let mut changed = w.clone();
    changed.close_events[1] = changed.close_events[0];
    assert!(!accepts(&r, changed, &instance), "missing header close");
    let mut changed = w.clone();
    changed.tx_events.swap(0, 1);
    assert!(!accepts(&r, changed, &instance), "reordered transactions");
    let mut changed = w.clone();
    changed.slots[1] += 1;
    assert!(!accepts(&r, changed, &instance), "unlinked slot counter");
    let mut changed = w.clone();
    changed.batch.headers[3].state_before.fee_total += 1;
    assert!(
        !accepts(&r, changed, &instance),
        "redundant header state changed"
    );
    let mut changed = w.clone();
    changed
        .batch
        .headers
        .iter_mut()
        .flat_map(|h| &mut h.txs)
        .next()
        .unwrap()
        .public
        .batch += 1;
    assert!(
        !accepts(&r, changed, &instance),
        "transaction batch label changed"
    );
    let mut changed = w;
    changed.states[1].fee_total += 1;
    assert!(
        !accepts(&r, changed, &instance),
        "unlinked intermediate state"
    );
}
