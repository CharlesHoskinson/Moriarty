//! The node row probe (`src/node_rows.rs`) is a copy of `SherlockIvcRelation::circuit`.
//! `node_row_breakdown` refuses when the copy's rows differ from the node's, so this test
//! catches a node change that was not mirrored into the copy before a full profile run does.
//! Row counts do not depend on the SRS, so a throwaway setup gives the leaf key.
use midnight_curves::Bls12;
use midnight_proofs::poly::kzg::params::ParamsKZG;
use midnight_zk_stdlib::{cost_model, setup_vk};
use mori_recursion::{ivc::SherlockIvcRelation, standin::StandInLeaf};
use mori_recursion_profile::node_rows::node_row_breakdown;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

#[test]
#[ignore = "node cost_model twice (about 2 GB, under a minute in release); T1"]
fn node_row_probe_matches_node() {
    let leaf = StandInLeaf;
    let params =
        ParamsKZG::<Bls12>::unsafe_setup(cost_model(&leaf, None).k, ChaCha20Rng::seed_from_u64(1));
    let leaf_vk = setup_vk(&params, &leaf);
    let relation = SherlockIvcRelation::new(4, 20, leaf_vk).unwrap();
    let rows = node_row_breakdown(&relation).unwrap();
    println!("node row probe k={} rows={}", rows.k, rows.rows);
    println!("NODE_ROWS nr_cols=4 {}", rows.to_json());
    assert_eq!(rows.k, 20);
}

/// Task 2.3 per-gadget row report for the k=19 cut: the node at nr_pow2range_cols 4 and 3
/// (`prover.md` §2 candidate T2), each at k=19 and k=20, with the excess over 2^19.
#[test]
#[ignore = "node cost_model 4 x 9 times (a few minutes in release); T1"]
fn node_rows_k19_candidates() {
    let leaf = StandInLeaf;
    let params =
        ParamsKZG::<Bls12>::unsafe_setup(cost_model(&leaf, None).k, ChaCha20Rng::seed_from_u64(1));
    let leaf_vk = setup_vk(&params, &leaf);
    for nr_cols in [4u8, 3] {
        for k in [19u32, 20] {
            let relation = SherlockIvcRelation::new(nr_cols, k, leaf_vk.clone()).unwrap();
            let rows = node_row_breakdown(&relation).unwrap();
            println!(
                "NODE_ROWS nr_cols={nr_cols} k={k} model_k={} over_2^19={} {}",
                rows.model_k,
                rows.rows as i64 - (1 << 19),
                rows.to_json()
            );
        }
    }
}
