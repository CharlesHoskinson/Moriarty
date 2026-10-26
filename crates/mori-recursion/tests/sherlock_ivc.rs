//! M3: the general-purpose recursion node (IVC over leaves) at its cost-model degree.
//!
//! The leaf here is a stand-in step relation with the frozen 5-lane leaf statement
//! [prog_d, pre_d, post_d, rule, h_pre] and post_d = state_digest(tag, value,
//! Poseidon(prog_d, pre_d, rule, h_pre)) with private (tag, value).
//! It measures the node's k, keygen and prove cost and checks three chained steps,
//! the final accumulator pairing and refusals; the C6 leaf replaces the stand-in.
use std::{fs, io, io::Cursor, path::PathBuf, time::Instant};

use group::Curve;
use midnight_circuits::{
    hash::poseidon::{PoseidonChip, PoseidonState},
    instructions::{
        AssertionInstructions, AssignmentInstructions, PublicInputInstructions, hash::HashCPU,
    },
    types::AssignedNative,
};
use midnight_proofs::{
    circuit::{Layouter, Value},
    plonk::Error,
    poly::kzg::params::ParamsKZG,
    utils::SerdeFormat,
};
use midnight_zk_stdlib::{
    MidnightVK, Relation, ZkStdLib, ZkStdLibArch, cost_model, prove, setup_pk, setup_vk, verify,
};
use mori_recursion::{
    inner::verify_offcircuit,
    ivc::{
        DOM_STATE_D, IvcState, IvcWitness, OUTCOME_HALTED, OUTCOME_RUNNING, SherlockIvcRelation,
        state_digest,
    },
    parent::dom,
    run_decider::{
        CC_STMT, RunDeciderRelation, RunDeciderWitness, V_FIN, V_INIT, V_PROG, V_RESULT, V_STEPS,
    },
    template::Template,
    util::F,
};
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

type Pos = PoseidonState<F>;
const RESULT: u128 = 6;

fn h(xs: &[F]) -> F {
    <PoseidonChip<F> as HashCPU<F, F>>::hash(xs)
}

#[derive(Clone, Debug)]
struct ToyLeaf;

impl Relation for ToyLeaf {
    type Instance = Vec<F>;
    type Witness = Vec<F>;
    type Error = Error;
    fn format_instance(i: &Vec<F>) -> Result<Vec<F>, Error> {
        Ok(i.clone())
    }
    fn circuit(
        &self,
        std: &ZkStdLib,
        l: &mut impl Layouter<F>,
        _i: Value<Vec<F>>,
        w: Value<Vec<F>>,
    ) -> Result<(), Error> {
        let mut v: Vec<AssignedNative<F>> = Vec::new();
        // Lane 0 is the binding (unconstrained here), then the 5 statement lanes.
        for i in 0..6 {
            let x: AssignedNative<F> = std.assign(l, w.as_ref().map(|w| w[i]))?;
            std.constrain_as_public_input(l, &x)?;
            v.push(x);
        }
        let body = std.poseidon(l, &[v[1].clone(), v[2].clone(), v[4].clone(), v[5].clone()])?;
        let d: AssignedNative<F> = std.assign_fixed(l, dom(DOM_STATE_D))?;
        let tag: AssignedNative<F> = std.assign(l, w.as_ref().map(|w| w[6]))?;
        let val: AssignedNative<F> = std.assign(l, w.as_ref().map(|w| w[7]))?;
        let post = std.poseidon(l, &[d, tag, val, body])?;
        std.assert_equal(l, &post, &v[3])
    }
    fn used_chips(&self) -> ZkStdLibArch {
        ZkStdLibArch {
            poseidon: true,
            ..ZkStdLibArch::default()
        }
    }
    fn write_relation<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_all(b"TOYLEAF1")
    }
    fn read_relation<R: io::Read>(r: &mut R) -> io::Result<Self> {
        let mut b = [0u8; 8];
        r.read_exact(&mut b)?;
        Ok(ToyLeaf)
    }
}

fn srs() -> ParamsKZG<midnight_curves::Bls12> {
    let p = PathBuf::from(std::env::var_os("MORIARTY_PARENT_SRS").expect("MORIARTY_PARENT_SRS"));
    let bytes = fs::read(p).unwrap();
    let mut c = Cursor::new(bytes);
    ParamsKZG::read_custom(&mut c, SerdeFormat::RawBytesUnchecked).unwrap()
}

fn at_k(base: &ParamsKZG<midnight_curves::Bls12>, k: u32) -> ParamsKZG<midnight_curves::Bls12> {
    let mut p = base.clone();
    p.downsize(k);
    p
}

#[test]
#[ignore = "real node keygen and proofs; heavy.lock"]
fn ivc_node_folds_three_leaves() {
    let base = srs();
    let mut rng = ChaCha20Rng::from_entropy();

    // Leaf key.
    let leaf = ToyLeaf;
    let lk = cost_model(&leaf, None).k;
    let lp = at_k(&base, lk);
    let leaf_vk: MidnightVK = setup_vk(&lp, &leaf);
    let leaf_pk = setup_pk(&leaf, &leaf_vk);
    println!("M3 leaf_k={lk}");

    // Node key at its cost-model degree (the relation needs k up front: try upward).
    let mut k = 18;
    let rel = loop {
        let r = SherlockIvcRelation::new(4, k, leaf_vk.clone()).unwrap();
        let m = cost_model(&r, Some(k));
        println!("M3 node try k={k} model_k={} rows={}", m.k, m.rows);
        if m.k <= k {
            break r;
        }
        k += 1;
        assert!(k <= 20, "node does not fit k20");
    };
    let np = at_k(&base, k);
    let t = Instant::now();
    let node_vk = setup_vk(&np, &rel);
    let node_pk = setup_pk(&rel, &node_vk);
    println!(
        "M3 node_k={k} keygen_seconds={:.3}",
        t.elapsed().as_secs_f64()
    );
    let vp = np.verifier_params();
    let self_repr = node_vk.vk().transcript_repr();

    let prog_d = h(&[F::from(7u64)]);
    let init_d = h(&[F::from(11u64)]);
    let mut st = IvcState {
        prog_d,
        init_d,
        cur_d: init_d,
        h: 0,
    };
    let mut acc = rel.trivial_acc();
    let mut proof: Vec<u8> = Vec::new();
    let mut last_lanes = F::from(0u64);
    for step in 0..3u64 {
        let rule = F::from(100 + step);
        let body = h(&[st.prog_d, st.cur_d, rule, F::from(st.h)]);
        let (tag, val) = if step == 2 {
            (OUTCOME_HALTED, RESULT)
        } else {
            (OUTCOME_RUNNING, 0)
        };
        let cur_d = state_digest(tag, val, body);
        last_lanes = body;
        let binding = F::from(0xb1d0 + step);
        let li = SherlockIvcRelation::leaf_instance(&st, binding, cur_d, rule);
        let mut lw = li.clone();
        lw.extend([F::from(tag), F::from(val as u64)]);
        let lproof = prove::<ToyLeaf, Pos>(&lp, &leaf_pk, &leaf, &li, lw, &mut rng).unwrap();
        let w = IvcWitness {
            self_repr,
            prev: st.clone(),
            prev_proof: proof.clone(),
            prev_acc: acc.clone(),
            cur_d,
            rule,
            leaf_binding: binding,
            leaf_proof: lproof,
        };
        let next_acc = rel.next_acc(Some(&node_vk), &w).unwrap();
        let next = IvcState {
            prog_d,
            init_d,
            cur_d,
            h: st.h + 1,
        };
        let inst = SherlockIvcRelation::instance(self_repr, &next, &next_acc);
        let t = Instant::now();
        let p = prove::<SherlockIvcRelation, Pos>(&np, &node_pk, &rel, &inst, w, &mut rng).unwrap();
        println!(
            "M3 step={step} prove_seconds={:.3} proof_bytes={} instance_lanes={}",
            t.elapsed().as_secs_f64(),
            p.len(),
            inst.len()
        );
        verify::<SherlockIvcRelation, Pos>(&vp, &node_vk, &inst, None, &p).unwrap();
        let mut bad = inst.clone();
        bad[3] += F::from(1u64);
        assert!(verify::<SherlockIvcRelation, Pos>(&vp, &node_vk, &bad, None, &p).is_err());
        st = next;
        acc = next_acc;
        proof = p;
    }
    // Terminal view: the last proof's own accumulator folded with its carried one.
    let inst = SherlockIvcRelation::instance(self_repr, &st, &acc);
    let own = verify_offcircuit(&vp, &node_vk, "m3_terminal", &inst, &proof, false);
    assert!(own.is_ok(), "last node proof verifies off circuit");
    assert!(
        SherlockIvcRelation::check_acc(&vp, &node_vk, &acc),
        "carried accumulator pairs"
    );
    let mut flipped = proof.clone();
    let mid = flipped.len() / 2;
    flipped[mid] ^= 1;
    assert!(verify::<SherlockIvcRelation, Pos>(&vp, &node_vk, &inst, None, &flipped).is_err());
    println!("M3 accepted steps=3 node_k={k} leaf_k={lk}");

    // M4: the run decider over the last node proof, for the captured acceptRun template.
    let template = Template::from_any_json(include_bytes!("fixtures/run-template.json")).unwrap();
    let s_g2 = base.s_g2().to_affine();
    let drel = RunDeciderRelation {
        nr_cols: 4,
        node_vk: node_vk.clone(),
        s_g2,
        template,
        cc_inputs: vec![CC_STMT, V_PROG, V_INIT, V_STEPS, V_FIN, V_RESULT],
    };
    drel.validate().unwrap();
    let dm = cost_model(&drel, None);
    println!("M4 decider model_k={} rows={}", dm.k, dm.rows);
    assert!(dm.k <= 20, "run decider must fit the ledger k20 SRS");
    let dp = at_k(&base, dm.k);
    let t = Instant::now();
    let dvk = setup_vk(&dp, &drel);
    let dpk = setup_pk(&drel, &dvk);
    println!("M4 decider keygen_seconds={:.3}", t.elapsed().as_secs_f64());
    let dw = RunDeciderWitness {
        w_instance: inst.clone(),
        w_proof: proof.clone(),
        w_acc: acc.clone(),
        result: RESULT,
        lanes_d: last_lanes,
        comm_rand: F::from(777u64),
    };
    let public = drel.public_inputs(F::from(5u64), &dw).unwrap();
    let t = Instant::now();
    let dproof = prove::<RunDeciderRelation, blake2b_simd::State>(
        &dp,
        &dpk,
        &drel,
        &public,
        dw.clone(),
        &mut rng,
    )
    .unwrap();
    println!(
        "M4 decider prove_seconds={:.3} proof_bytes={} public={}",
        t.elapsed().as_secs_f64(),
        dproof.len(),
        public.len()
    );
    let dvp = dp.verifier_params();
    verify::<RunDeciderRelation, blake2b_simd::State>(&dvp, &dvk, &public, None, &dproof).unwrap();
    let ri = drel
        .template
        .slots
        .iter()
        .position(|s| *s == mori_recursion::template::Slot::Value(V_RESULT))
        .unwrap();
    let mut wrong = public.clone();
    wrong[ri] = F::from(7u64);
    assert!(
        verify::<RunDeciderRelation, blake2b_simd::State>(&dvp, &dvk, &wrong, None, &dproof)
            .is_err()
    );
    let out = std::env::var_os("MORIARTY_M4_OUT").map(PathBuf::from);
    if let Some(dir) = out {
        fs::create_dir_all(&dir).unwrap();
        fs::write(
            dir.join("run-decider.vk"),
            mori_recursion::util::vk_bytes(&dvk),
        )
        .unwrap();
        let mut rb = Vec::new();
        drel.write_relation(&mut rb).unwrap();
        fs::write(dir.join("run-decider.relation"), rb).unwrap();
    }
    println!("M4 accepted decider_k={} result={RESULT}", dm.k);
}
