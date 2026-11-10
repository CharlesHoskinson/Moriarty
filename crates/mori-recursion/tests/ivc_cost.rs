//! Row-count probe for the IVC node (no SRS, no keygen of the node): models the node
//! against a stand-in leaf keyed on throwaway parameters. Modeling only.
use midnight_curves::Bls12;
use midnight_proofs::poly::kzg::params::ParamsKZG;
use midnight_zk_stdlib::{cost_model, setup_vk};
use mori_recursion::{ivc::SherlockIvcRelation, standin::StandInLeaf};
use rand::rngs::OsRng;

#[test]
#[ignore = "row-count probe; light lane"]
fn ivc_node_rows() {
    let leaf = StandInLeaf;
    let lk = cost_model(&leaf, None).k;
    let p: ParamsKZG<Bls12> = ParamsKZG::unsafe_setup(lk, OsRng);
    let vk = setup_vk(&p, &leaf);
    // Task 2.3 T1 check: `SherlockIvcRelation::new(4, 19, leaf_vk)` and
    // `cost_model(&r, Some(19)).k == 19`; candidate T2 of `prover.md` §2 is nr_pow2range_cols 3.
    for nr_cols in [4u8, 3] {
        for k in [19u32, 20] {
            let rel = SherlockIvcRelation::new(nr_cols, k, vk.clone()).unwrap();
            let m = cost_model(&rel, Some(k));
            println!(
                "IVCCOST nr_cols={nr_cols} k={k} model_k={} rows={} over_2^19={} k19_fits={}",
                m.k,
                m.rows,
                m.rows as i64 - (1 << 19),
                m.k <= 19
            );
        }
    }
}
