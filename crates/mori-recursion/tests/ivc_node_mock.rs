//! The IVC node's genesis step under MockProver: names any unsatisfied gate, copy or
//! lookup that a real proof would only surface as `Error::Opening` at verify.
//!
//! Regression (SHKIVC03/04): the circuit folded the zero-scaled, uncollapsed previous-proof
//! accumulator while `next_acc` folded `trivial_acc`; the fold challenge hashes both, so the
//! published accumulator lanes (instance rows 5..15) disagreed with the native statement.
//! No node key is needed: at genesis the self key's repr is a free public lane.
use ff::Field;
use midnight_proofs::{dev::MockProver, poly::kzg::params::ParamsKZG};
use midnight_zk_stdlib::{MidnightCircuit, cost_model, prove, setup_pk, setup_vk};
use mori_recursion::{
    ivc::{IvcState, IvcWitness, OUTCOME_RUNNING, SherlockIvcRelation, state_digest},
    parent::poseidon,
    standin::{StandInLeaf, standin_body},
    util::F,
};
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

type Pos = midnight_circuits::hash::poseidon::PoseidonState<F>;

#[test]
#[ignore = "node MockProver at k20; heavy.lock"]
fn ivc_node_genesis_mock() {
    let mut rng = ChaCha20Rng::seed_from_u64(7);
    let leaf = StandInLeaf;
    let lk = cost_model(&leaf, None).k;
    let lp = ParamsKZG::<midnight_curves::Bls12>::unsafe_setup(lk, &mut rng);
    let leaf_vk = setup_vk(&lp, &leaf);
    let leaf_pk = setup_pk(&leaf, &leaf_vk);

    let k: u32 = std::env::var("MORIARTY_IVC_MOCK_K").map_or(20, |s| s.parse().unwrap());
    let rel = SherlockIvcRelation::new(4, k, leaf_vk).unwrap();
    let prog_d = poseidon(&[F::from(0x5052_4f47u64)]);
    let init_d = poseidon(&[F::from(0x494e_4954u64)]);
    let st = IvcState {
        prog_d,
        init_d,
        cur_d: init_d,
        h: 0,
    };
    let rule = F::from(1000u64);
    let body = standin_body(prog_d, init_d, rule, F::ZERO);
    let cur_d = state_digest(OUTCOME_RUNNING, 0, body);
    let li = SherlockIvcRelation::leaf_instance(&st, F::from(0xb1d0u64), cur_d, rule, 2);
    let mut lw = li.clone();
    lw.extend([F::from(OUTCOME_RUNNING), F::ZERO]);
    let lproof = prove::<StandInLeaf, Pos>(&lp, &leaf_pk, &leaf, &li, lw, &mut rng).unwrap();

    let repr = F::from(0x5e1fu64);
    let w = IvcWitness {
        self_repr: repr,
        prev: st.clone(),
        prev_proof: Vec::new(),
        prev_acc: rel.trivial_acc(),
        cur_d,
        rule,
        leaf_binding: li[0],
        leaf_steps: 2,
        leaf_proof: lproof,
    };
    let next_acc = rel.next_acc(None, &w).unwrap();
    let next = IvcState { cur_d, h: 2, ..st };
    let ni = SherlockIvcRelation::instance(repr, &next, &next_acc);
    let circuit = MidnightCircuit::new(
        &rel,
        midnight_proofs::circuit::Value::known(ni.clone()),
        midnight_proofs::circuit::Value::known(w),
        Some(k),
    );
    let mock = MockProver::run(&circuit, vec![vec![], ni]).unwrap();
    if let Err(fails) = mock.verify() {
        for f in fails.iter().take(20) {
            println!("MOCK FAIL {f:?}");
        }
        panic!("{} failures", fails.len());
    }
    println!("MOCK node genesis satisfied k={k}");
}
