//! X14 (Sherlock freeze experiment): the verifier-side structure of two registered halo2
//! circuits on the core curve, from `cost_model` (no SRS for the leaf, no keygen of the node):
//! commitments, column queries, point sets and proof size. These are the inputs of the
//! operation inventory of the GP-3 general verifier (FV-PARSE, FV-TRANSCRIPT, FV-ARITH,
//! FV-FINAL). Modeling only.
use midnight_curves::Bls12;
use midnight_proofs::poly::kzg::params::ParamsKZG;
use midnight_zk_stdlib::{cost_model, setup_vk};
use mori_recursion::{ivc::SherlockIvcRelation, standin::StandInLeaf};
use rand::rngs::OsRng;

#[test]
#[ignore = "X14 inventory probe; light lane"]
fn x14_registered_circuit_models() {
    let leaf = StandInLeaf;
    let m = cost_model(&leaf, None);
    println!("X14-MODEL standin_leaf {m:?}");
    let p: ParamsKZG<Bls12> = ParamsKZG::unsafe_setup(m.k, OsRng);
    let vk = setup_vk(&p, &leaf);
    let rel = SherlockIvcRelation::new(4, 19, vk).unwrap();
    let n = cost_model(&rel, Some(19));
    println!("X14-MODEL ivc_node {n:?}");
}
