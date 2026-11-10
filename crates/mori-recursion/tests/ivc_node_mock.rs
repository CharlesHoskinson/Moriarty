//! The IVC node under MockProver: names any unsatisfied gate, copy or lookup that a real
//! proof would only surface as `Error::Opening` at verify, and refuses each SRC-004/SRC-005
//! boundary from a valid baseline (task 2.3).
//!
//! Regression (SHKIVC03/04): the circuit folded the zero-scaled, uncollapsed previous-proof
//! accumulator while `next_acc` folded `trivial_acc`; the fold challenge hashes both, so the
//! published accumulator lanes (instance rows 5..15) disagreed with the native statement.
//! No node key is needed at genesis: the self key's repr is then a free public lane.
//!
//! Every negative builds a statement that agrees with the circuit's own fold (`next_acc_raw`),
//! so the only failing constraint is the one under test; its baseline passes the same path.
//! Genesis tests need no SRS. `ivc_node_step1_negatives` keys the node (MORIARTY_PARENT_SRS)
//! to get a real previous proof, because a non-genesis step parses one.
use std::{collections::BTreeMap, fs, io::Cursor, path::PathBuf};

use ff::Field;
use midnight_circuits::verifier::{Accumulator, Msm};
use midnight_curves::Bls12;
use midnight_proofs::{
    dev::{MockProver, VerifyFailure},
    poly::kzg::params::ParamsKZG,
    utils::SerdeFormat,
};
use midnight_zk_stdlib::{
    MidnightCircuit, MidnightPK, MidnightVK, cost_model, prove, setup_pk, setup_vk, verify,
};
use mori_recursion::{
    inner::{C, S},
    ivc::{
        IVC_STATE, IvcState, IvcWitness, OUTCOME_RUNNING, S_MAX, SherlockIvcRelation, state_digest,
    },
    parent::poseidon,
    standin::{StandInLeaf, standin_body},
    util::F,
};
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

type Pos = midnight_circuits::hash::poseidon::PoseidonState<F>;

const REPR: u64 = 0x5e1f;

struct Fixture {
    rng: ChaCha20Rng,
    lp: ParamsKZG<Bls12>,
    leaf_pk: MidnightPK<StandInLeaf>,
    leaf_vk: MidnightVK,
    prog_d: F,
    init_d: F,
}

impl Fixture {
    /// Leaf key on `base` (downsized) or on throwaway parameters.
    fn new(base: Option<&ParamsKZG<Bls12>>) -> Self {
        let mut rng = ChaCha20Rng::seed_from_u64(7);
        let leaf = StandInLeaf;
        let lk = cost_model(&leaf, None).k;
        let lp = match base {
            Some(b) => {
                let mut p = b.clone();
                p.downsize(lk);
                p
            }
            None => ParamsKZG::<Bls12>::unsafe_setup(lk, &mut rng),
        };
        let leaf_vk = setup_vk(&lp, &leaf);
        let leaf_pk = setup_pk(&leaf, &leaf_vk);
        Self {
            rng,
            lp,
            leaf_pk,
            leaf_vk,
            prog_d: poseidon(&[F::from(0x5052_4f47u64)]),
            init_d: poseidon(&[F::from(0x494e_4954u64)]),
        }
    }

    fn rel(&self) -> SherlockIvcRelation {
        let k: u32 = std::env::var("MORIARTY_IVC_MOCK_K").map_or(20, |s| s.parse().unwrap());
        SherlockIvcRelation::new(4, k, self.leaf_vk.clone()).unwrap()
    }

    /// A stand-in leaf proof of `[binding, prog_d, pre_d, post_d, rule, h_pre, steps]` over
    /// raw field values; returns the instance and the proof.
    fn leaf(&mut self, pre_d: F, h_pre: F, steps: F, rule: F) -> (Vec<F>, Vec<u8>) {
        let body = standin_body(self.prog_d, pre_d, rule, h_pre);
        let post = state_digest(OUTCOME_RUNNING, 0, body);
        let inst = vec![
            F::from(0xb1d0u64),
            self.prog_d,
            pre_d,
            post,
            rule,
            h_pre,
            steps,
        ];
        let mut w = inst.clone();
        w.extend([F::from(OUTCOME_RUNNING), F::ZERO]);
        let proof = prove::<StandInLeaf, Pos>(
            &self.lp,
            &self.leaf_pk,
            &StandInLeaf,
            &inst,
            w,
            &mut self.rng,
        )
        .unwrap();
        (inst, proof)
    }

    fn genesis(&self) -> IvcState {
        IvcState {
            prog_d: self.prog_d,
            init_d: self.init_d,
            cur_d: self.init_d,
            h: 0,
        }
    }
}

/// The witness for a leaf instance `li` on top of `prev`. `leaf_steps` is the `u64` the
/// witness carries; tests that need a field value outside `u64` use `with_raw_heights`.
fn witness(
    repr: F,
    prev: &IvcState,
    prev_proof: Vec<u8>,
    prev_acc: Accumulator<S>,
    li: &[F],
    leaf_steps: u64,
    leaf_proof: Vec<u8>,
) -> IvcWitness {
    IvcWitness {
        self_repr: repr,
        prev: prev.clone(),
        prev_proof,
        prev_acc,
        cur_d: li[3],
        rule: li[4],
        leaf_binding: li[0],
        leaf_steps,
        leaf_proof,
    }
}

/// The node statement that matches the circuit's own fold for these raw values.
#[allow(clippy::too_many_arguments)]
fn statement(
    rel: &SherlockIvcRelation,
    node_vk: Option<&MidnightVK>,
    repr: F,
    prev: [F; IVC_STATE],
    w: &IvcWitness,
    li: &[F],
    h: F,
) -> (Vec<F>, Accumulator<S>) {
    let acc = rel
        .next_acc_raw(
            node_vk,
            repr,
            &prev,
            &w.prev_proof,
            &w.prev_acc,
            li,
            &w.leaf_proof,
        )
        .unwrap();
    let fields = [prev[0], prev[1], li[3], h];
    (
        SherlockIvcRelation::instance_from_fields(repr, &fields, &acc),
        acc,
    )
}

fn mock(rel: &SherlockIvcRelation, ni: &[F], w: IvcWitness) -> Result<(), Vec<VerifyFailure>> {
    let k = rel.self_k;
    let circuit = MidnightCircuit::new(
        rel,
        midnight_proofs::circuit::Value::known(ni.to_vec()),
        midnight_proofs::circuit::Value::known(w),
        Some(k),
    );
    MockProver::run(&circuit, vec![vec![], ni.to_vec()])
        .unwrap()
        .verify()
}

fn expect_pass(name: &str, r: Result<(), Vec<VerifyFailure>>) {
    if let Err(fails) = r {
        for f in fails.iter().take(20) {
            println!("MOCK FAIL {name}: {f:?}");
        }
        panic!("{name}: baseline must hold, {} failures", fails.len());
    }
    println!("MOCK PASS {name}");
}

fn expect_refused(name: &str, r: Result<(), Vec<VerifyFailure>>) {
    match r {
        Ok(()) => panic!("{name}: the node accepted a witness it must refuse"),
        Err(fails) => {
            let lookups = fails
                .iter()
                .filter(|f| matches!(f, VerifyFailure::Lookup { .. }))
                .count();
            let perms = fails
                .iter()
                .filter(|f| matches!(f, VerifyFailure::Permutation { .. }))
                .count();
            println!(
                "MOCK REFUSED {name}: {} failures (lookup {lookups}, permutation {perms}); first: {:?}",
                fails.len(),
                fails.first()
            );
        }
    }
}

/// Genesis leaf of `steps` steps: the honest witness and its matching statement.
fn genesis_case(fx: &mut Fixture, rel: &SherlockIvcRelation, steps: u64) -> (Vec<F>, IvcWitness) {
    let st = fx.genesis();
    let (li, lp) = fx.leaf(st.cur_d, F::ZERO, F::from(steps), F::from(1000u64));
    let repr = F::from(REPR);
    let w = witness(repr, &st, Vec::new(), rel.trivial_acc(), &li, steps, lp);
    let (ni, _) = statement(rel, None, repr, st.fields(), &w, &li, F::from(steps));
    (ni, w)
}

#[test]
#[ignore = "node MockProver at k20; heavy.lock"]
fn ivc_node_genesis_mock() {
    let mut fx = Fixture::new(None);
    let rel = fx.rel();
    let (ni, w) = genesis_case(&mut fx, &rel, 2);
    // `next_acc` (the production host path) and the raw statement agree.
    let next = IvcState {
        cur_d: w.cur_d,
        h: 2,
        ..fx.genesis()
    };
    let acc = rel.next_acc(None, &w).unwrap();
    assert_eq!(
        SherlockIvcRelation::instance(F::from(REPR), &next, &acc),
        ni
    );
    expect_pass("genesis steps=2", mock(&rel, &ni, w));
}

/// SRC-004 at genesis: steps = S_MAX holds, S_MAX + 1 and 0 are refused.
#[test]
#[ignore = "node MockProver at k20 (three runs); heavy.lock"]
fn ivc_node_genesis_steps_bound() {
    let mut fx = Fixture::new(None);
    let rel = fx.rel();
    let (ni, w) = genesis_case(&mut fx, &rel, S_MAX);
    expect_pass("genesis steps=S_MAX", mock(&rel, &ni, w));
    let (ni, w) = genesis_case(&mut fx, &rel, S_MAX + 1);
    expect_refused("genesis steps=S_MAX+1", mock(&rel, &ni, w));
    let (ni, w) = genesis_case(&mut fx, &rel, 0);
    expect_refused("genesis steps=0", mock(&rel, &ni, w));
}

/// SRC-005 realization (a): a well-formed, non-neutral carried accumulator at genesis is
/// refused (CEX-3/CEX-4 of `qa/gemini-quint/ivc/REPORT.md`), and the host refuses it too.
#[test]
#[ignore = "node MockProver at k20; heavy.lock"]
fn ivc_node_genesis_non_neutral_carry() {
    let mut fx = Fixture::new(None);
    let rel = fx.rel();
    let st = fx.genesis();
    let (li, lp) = fx.leaf(st.cur_d, F::ZERO, F::from(2u64), F::from(1000u64));
    let repr = F::from(REPR);
    // Same shape as trivial_acc (one base per side, every self fixed-base name), but the
    // generator for a base and a non-zero fixed scalar: a valid encoding, not the neutral one.
    let names = rel.self_base_names();
    let mut fixed: BTreeMap<String, F> = names.iter().map(|n| (n.clone(), F::ZERO)).collect();
    *fixed.get_mut(&names[0]).unwrap() = F::from(5u64);
    let g = <C as group::Group>::generator();
    let carry = Accumulator::<S>::new(
        Msm::new(&[g], &[F::ONE], &BTreeMap::new()),
        Msm::new(&[g], &[F::ONE], &fixed),
    );
    assert!(!rel.is_neutral(&carry));
    let w = witness(repr, &st, Vec::new(), carry, &li, 2, lp);
    assert!(
        rel.next_acc(None, &w).is_err(),
        "host refuses a non-neutral genesis carry"
    );
    let (ni, _) = statement(&rel, None, repr, st.fields(), &w, &li, F::from(2u64));
    expect_refused("genesis non-neutral carry", mock(&rel, &ni, w));
}

/// SRC-005, both realizations: an invalid leaf at genesis satisfies every node constraint
/// (the leaf's opening is deferred) and is refused by the final pairing of the folded
/// accumulator. At genesis the self part is trivial (all self fixed-base scalars zero), so the
/// pairing needs no node key. Baseline: the honest leaf's folded accumulator pairs.
#[test]
#[ignore = "node MockProver at k20 (two runs); heavy.lock"]
fn ivc_node_genesis_invalid_leaf_pairing() {
    let mut fx = Fixture::new(None);
    let rel = fx.rel();
    let bases: BTreeMap<String, C> = rel
        .self_base_names()
        .into_iter()
        .map(|n| (n, <C as group::Group>::identity()))
        .collect();
    let vp = fx.lp.verifier_params();
    let st = fx.genesis();
    let repr = F::from(REPR);

    let (li, lp) = fx.leaf(st.cur_d, F::ZERO, F::from(2u64), F::from(1000u64));
    let w = witness(repr, &st, Vec::new(), rel.trivial_acc(), &li, 2, lp.clone());
    let (ni, acc) = statement(&rel, None, repr, st.fields(), &w, &li, F::from(2u64));
    expect_pass("genesis honest leaf", mock(&rel, &ni, w));
    assert!(acc.check(&vp, &bases), "honest genesis accumulator pairs");
    println!("PAIRING PASS genesis honest leaf");

    // The same proof presented for another post state: not a proof of this statement.
    let mut bad = li.clone();
    bad[3] = state_digest(OUTCOME_RUNNING, 1, F::from(0xbadu64));
    let w = witness(repr, &st, Vec::new(), rel.trivial_acc(), &bad, 2, lp);
    let (ni, acc) = statement(&rel, None, repr, st.fields(), &w, &bad, F::from(2u64));
    expect_pass("genesis invalid leaf (deferred)", mock(&rel, &ni, w));
    assert!(
        !acc.check(&vp, &bases),
        "invalid genesis leaf must fail the final pairing"
    );
    println!("PAIRING REFUSED genesis invalid leaf");
}

fn srs() -> ParamsKZG<Bls12> {
    let p = PathBuf::from(std::env::var_os("MORIARTY_PARENT_SRS").expect("MORIARTY_PARENT_SRS"));
    let mut c = Cursor::new(fs::read(p).unwrap());
    ParamsKZG::read_custom(&mut c, SerdeFormat::RawBytesUnchecked).unwrap()
}

/// SRC-004 off genesis, plus the invalid-leaf refusal through the final pairing under the real
/// node key. Keys the node (k20), proves an honest genesis step and one with an invalid leaf,
/// then runs step 1 under MockProver: the honest step holds; `h = 2^64` (prev_h = 2^64 - 1)
/// and the wrap `prev_h = -2, steps = 2` (h = 0 in the field, CEX-1) are refused. A wrong
/// `prev_h` only defers a pairing defect for the previous proof, so each refusal comes from the
/// range checks alone. A carried accumulator of the same value but not in collapsed form
/// (scalar 2) is refused by the collapsed-carry constraint (Lean `nodeFold`).
#[test]
#[ignore = "real node keygen and two proofs at k20, then MockProver; heavy.lock"]
fn ivc_node_step1_negatives() {
    let base = srs();
    let mut fx = Fixture::new(Some(&base));
    let rel = fx.rel();
    let mut np = base.clone();
    np.downsize(rel.self_k);
    let node_vk = setup_vk(&np, &rel);
    let node_pk = setup_pk(&rel, &node_vk);
    let vp = np.verifier_params();
    let repr = node_vk.vk().transcript_repr();
    let st = fx.genesis();

    // Real genesis proofs: honest, and with an invalid leaf (the proof for another post).
    let (li, lp) = fx.leaf(st.cur_d, F::ZERO, F::from(2u64), F::from(1000u64));
    let mut bad = li.clone();
    bad[3] = state_digest(OUTCOME_RUNNING, 1, F::from(0xbadu64));
    let mut real = Vec::new();
    for (name, inst) in [("honest", &li), ("invalid leaf", &bad)] {
        let w = witness(
            repr,
            &st,
            Vec::new(),
            rel.trivial_acc(),
            inst,
            2,
            lp.clone(),
        );
        let (ni, acc) = statement(
            &rel,
            Some(&node_vk),
            repr,
            st.fields(),
            &w,
            inst,
            F::from(2u64),
        );
        let p =
            prove::<SherlockIvcRelation, Pos>(&np, &node_pk, &rel, &ni, w, &mut fx.rng).unwrap();
        verify::<SherlockIvcRelation, Pos>(&vp, &node_vk, &ni, None, &p).unwrap();
        let pairs = SherlockIvcRelation::check_acc(&vp, &node_vk, &acc);
        println!("NODE genesis {name}: proof verifies, final pairing {pairs}");
        real.push((inst[3], p, acc, pairs));
    }
    assert!(
        real[0].3,
        "honest genesis accumulator pairs under the node key"
    );
    assert!(
        !real[1].3,
        "invalid genesis leaf refused by the final pairing"
    );
    drop(node_pk);

    let (post0, p0, acc0, _) = real.swap_remove(0);
    let s1 = IvcState {
        cur_d: post0,
        h: 2,
        ..st.clone()
    };
    let rule = F::from(1001u64);

    // Baseline: honest step 1.
    let (li1, lp1) = fx.leaf(s1.cur_d, F::from(2u64), F::from(3u64), rule);
    let w = witness(repr, &s1, p0.clone(), acc0.clone(), &li1, 3, lp1);
    let (ni, _) = statement(
        &rel,
        Some(&node_vk),
        repr,
        s1.fields(),
        &w,
        &li1,
        F::from(5u64),
    );
    expect_pass("step1 honest", mock(&rel, &ni, w));

    // Non-collapsed carry: the same accumulator value re-encoded as (B / 2, scalar 2) on each
    // side. `collapse` would map it back to (B, 1), so `nodeFold` needs the carry collapsed;
    // the node refuses the re-encoding (and the host refuses it before folding).
    let two = F::from(2u64);
    let half = two.invert().unwrap();
    let reenc = |m: Msm<S>| Msm::<S>::new(&[m.bases()[0] * half], &[two], &m.fixed_base_scalars());
    let carry = Accumulator::<S>::new(reenc(acc0.lhs()), reenc(acc0.rhs()));
    assert!(!rel.is_collapsed_carry(&carry));
    let (li1, lp1) = fx.leaf(s1.cur_d, F::from(2u64), F::from(3u64), rule);
    let w = witness(repr, &s1, p0.clone(), carry, &li1, 3, lp1);
    assert!(
        rel.next_acc(Some(&node_vk), &w).is_err(),
        "host refuses a non-collapsed carry"
    );
    let (ni, _) = statement(
        &rel,
        Some(&node_vk),
        repr,
        s1.fields(),
        &w,
        &li1,
        F::from(5u64),
    );
    expect_refused("step1 non-collapsed carry", mock(&rel, &ni, w));

    // h = 2^64: prev_h = 2^64 - 1 is in range, the sum is not.
    let top = IvcState {
        h: u64::MAX,
        ..s1.clone()
    };
    let h_top = F::from(u64::MAX) + F::ONE;
    let (li2, lp2) = fx.leaf(top.cur_d, F::from(u64::MAX), F::ONE, rule);
    let w = witness(repr, &top, p0.clone(), acc0.clone(), &li2, 1, lp2);
    let (ni, _) = statement(&rel, Some(&node_vk), repr, top.fields(), &w, &li2, h_top);
    expect_refused("step1 h=2^64", mock(&rel, &ni, w));

    // CEX-1 wrap: prev_h = -2 (not zero, so not genesis), steps = 2, h = 0 in the field; the
    // next step would read h = 0 as genesis and drop this chain's previous proof.
    let minus2 = -F::from(2u64);
    let (li3, lp3) = fx.leaf(s1.cur_d, minus2, F::from(2u64), rule);
    let wrap_rel = rel.clone().with_raw_heights(minus2, F::from(2u64));
    let w = witness(repr, &s1, p0, acc0, &li3, 2, lp3);
    let prev = [s1.prog_d, s1.init_d, s1.cur_d, minus2];
    let (ni, _) = statement(&wrap_rel, Some(&node_vk), repr, prev, &w, &li3, F::ZERO);
    expect_refused("step1 wrap h=0", mock(&wrap_rel, &ni, w));
}
