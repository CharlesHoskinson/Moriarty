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
    for k in [19u32] {
        let rel = SherlockIvcRelation::new(4, k, vk.clone()).unwrap();
        let m = cost_model(&rel, Some(k));
        println!(
            "IVCCOST k={k} model_k={} rows={} over_2^19={}",
            m.k,
            m.rows,
            m.rows as i64 - (1 << 19)
        );
    }
}
