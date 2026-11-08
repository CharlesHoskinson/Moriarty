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
    assert_eq!(rows.k, 20);
}
