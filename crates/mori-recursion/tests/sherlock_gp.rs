//! General-purpose recursion pipeline: fold proved leaves through the IVC node, key the
//! run decider for the acceptRun template, and prove one acceptRun call from an SDK opening.
//!
//! Leaves come from MORIARTY_GP_LEAF_DIR (`leaf.vk`, `step-NNNNNN/{leaf.instance.json,leaf.proof}` (instance = the complete proved
//! instance [binding, prog_d, pre_d, post_d, rule, h_pre]),
//! `final.json` {result, lanes_d}) or, with MORIARTY_GP_STANDIN_STEPS, from the stand-in
//! leaf (a rehearsal; never a qualification).
use std::{fs, io::Cursor, path::PathBuf, time::Instant};

use ff::{Field, PrimeField};
use group::Curve;
use midnight_curves::Bls12;
use midnight_proofs::{poly::kzg::params::ParamsKZG, utils::SerdeFormat};
use midnight_transient_crypto::{
    curve::Fr,
    proofs::{PARAMS_VERIFIER, Proof, ProofPreimage, VerifierKey},
};
use midnight_zk_stdlib::{MidnightVK, Relation, cost_model, prove, setup_pk, setup_vk, verify};
use mori_recursion::{
    ivc::{
        IvcState, IvcWitness, OUTCOME_HALTED, OUTCOME_RUNNING, SherlockIvcRelation, acc_from_lanes,
        state_digest,
    },
    parent::poseidon,
    run_decider::{
        CC_STMT, RunDeciderRelation, RunDeciderWitness, V_FIN, V_INIT, V_PROG, V_RESULT, V_STEPS,
    },
    standin::{StandInLeaf, standin_body},
    template::Template,
    util::{F, f_from_hex, f_to_hex, sha256_hex, vk_bytes, vk_from_bytes},
};
use rand::{SeedableRng, rngs::OsRng};
use rand_chacha::ChaCha20Rng;
use serde_json::{Value, json};

type Pos = midnight_circuits::hash::poseidon::PoseidonState<F>;
type B2 = blake2b_simd::State;

fn env(name: &str) -> PathBuf {
    PathBuf::from(std::env::var_os(name).unwrap_or_else(|| panic!("{name} is required")))
}

fn srs() -> ParamsKZG<Bls12> {
    let bytes = fs::read(env("MORIARTY_PARENT_SRS")).unwrap();
    assert_eq!(
        sha256_hex(&bytes),
        "1cc62978558fdc1e445cd70cfd9a86ec3c2e2151b6d74811232d37faf9133ff1",
        "official k20 ceremony SRS"
    );
    let mut c = Cursor::new(bytes);
    ParamsKZG::read_custom(&mut c, SerdeFormat::RawBytesUnchecked).unwrap()
}

fn at_k(base: &ParamsKZG<Bls12>, k: u32) -> ParamsKZG<Bls12> {
    let mut p = base.clone();
    p.downsize(k);
    p
}

fn hexes(v: &[F]) -> Value {
    json!(v.iter().map(f_to_hex).collect::<Vec<_>>())
}

fn read_fs(path: PathBuf) -> Vec<F> {
    let v: Vec<String> = serde_json::from_slice(&fs::read(path).unwrap()).unwrap();
    v.iter().map(|s| f_from_hex(s).unwrap()).collect()
}

struct Leaves {
    vk: MidnightVK,
    steps: Vec<(Vec<F>, Vec<u8>)>,
    result: u128,
    lanes_d: F,
}

fn standin_leaves(base: &ParamsKZG<Bls12>, n: u64, result: u128) -> Leaves {
    let mut rng = ChaCha20Rng::from_entropy();
    let leaf = StandInLeaf;
    let p = at_k(base, cost_model(&leaf, None).k);
    let vk = setup_vk(&p, &leaf);
    let pk = setup_pk(&leaf, &vk);
    let prog_d = poseidon(&[F::from(0x5052_4f47u64)]);
    let mut pre = poseidon(&[F::from(0x494e_4954u64)]);
    let (mut steps, mut lanes_d) = (Vec::new(), F::ZERO);
    for h in 0..n {
        let rule = F::from(1000 + h);
        let body = standin_body(prog_d, pre, rule, F::from(h));
        let (tag, val) = if h + 1 == n {
            (OUTCOME_HALTED, result)
        } else {
            (OUTCOME_RUNNING, 0)
        };
        let post = state_digest(tag, val, body);
        let inst = vec![F::from(0xb1d0 + h), prog_d, pre, post, rule, F::from(h)];
        let mut w = inst.clone();
        w.extend([F::from(tag), F::from_u128(val)]);
        let proof = prove::<StandInLeaf, Pos>(&p, &pk, &leaf, &inst, w, &mut rng).unwrap();
        steps.push((inst, proof));
        pre = post;
        lanes_d = body;
    }
    Leaves {
        vk,
        steps,
        result,
        lanes_d,
    }
}

fn dir_leaves(dir: PathBuf) -> Leaves {
    let vk = vk_from_bytes(&fs::read(dir.join("leaf.vk")).unwrap()).unwrap();
    let mut names: Vec<_> = fs::read_dir(&dir)
        .unwrap()
        .map(|e| e.unwrap().file_name().into_string().unwrap())
        .filter(|n| n.starts_with("step-"))
        .collect();
    names.sort();
    let steps = names
        .iter()
        .map(|n| {
            let d = dir.join(n);
            (
                read_fs(d.join("leaf.instance.json")),
                fs::read(d.join("leaf.proof")).unwrap(),
            )
        })
        .collect();
    let fin: Value = serde_json::from_slice(&fs::read(dir.join("final.json")).unwrap()).unwrap();
    Leaves {
        vk,
        steps,
        result: fin["result"].as_str().unwrap().parse().unwrap(),
        lanes_d: f_from_hex(fin["lanes_d"].as_str().unwrap()).unwrap(),
    }
}

fn template() -> Template {
    Template::from_any_json(include_bytes!("fixtures/run-template.json")).unwrap()
}

const CC_INPUTS: [u32; 6] = [CC_STMT, V_PROG, V_INIT, V_STEPS, V_FIN, V_RESULT];

#[test]
#[ignore = "real node and decider keygen and proofs; heavy.lock"]
fn gp_fold_and_key() {
    let out = env("MORIARTY_GP_OUT");
    fs::create_dir(&out).unwrap();
    let base = srs();
    let t = Instant::now();
    let leaves = match std::env::var("MORIARTY_GP_STANDIN_STEPS") {
        Ok(n) => standin_leaves(&base, n.parse().unwrap(), 6),
        Err(_) => dir_leaves(env("MORIARTY_GP_LEAF_DIR")),
    };
    let n = leaves.steps.len() as u64;
    assert!(n >= 1);
    println!(
        "GP leaves={n} leaf_k={} load_seconds={:.3}",
        leaves.vk.k(),
        t.elapsed().as_secs_f64()
    );
    // Every leaf verifies on its own, and the leaf chain is contiguous.
    let (prog_d, init_d) = (leaves.steps[0].0[1], leaves.steps[0].0[2]);
    for (h, (inst, proof)) in leaves.steps.iter().enumerate() {
        assert_eq!(
            inst.len(),
            6,
            "leaf instance [binding, prog_d, pre_d, post_d, rule, h_pre]"
        );
        assert_eq!(
            (inst[1], inst[5]),
            (prog_d, F::from(h as u64)),
            "leaf {h} program/height"
        );
        if h > 0 {
            assert_eq!(
                inst[2],
                leaves.steps[h - 1].0[3],
                "leaf {h} pre = previous post"
            );
        }
        let p = at_k(&base, leaves.vk.k() as u32);
        verify::<StandInLeaf, Pos>(&p.verifier_params(), &leaves.vk, inst, None, proof)
            .unwrap_or_else(|e| panic!("leaf {h} verify: {e:?}"));
    }
    let fin_d = leaves.steps.last().unwrap().0[3];
    assert_eq!(
        fin_d,
        state_digest(OUTCOME_HALTED, leaves.result, leaves.lanes_d),
        "halted final digest"
    );

    // Node key.
    let mut k: u32 = std::env::var("MORIARTY_GP_NODE_K").map_or(18, |s| s.parse().unwrap());
    let rel = loop {
        let r = SherlockIvcRelation::new(4, k, leaves.vk.clone()).unwrap();
        let m = cost_model(&r, Some(k));
        println!("GP node try k={k} model_k={} rows={}", m.k, m.rows);
        if m.k <= k {
            break r;
        }
        k += 1;
        assert!(k <= 20, "node exceeds k20");
    };
    let np = at_k(&base, k);
    let t = Instant::now();
    let node_vk = setup_vk(&np, &rel);
    let node_pk = setup_pk(&rel, &node_vk);
    println!(
        "GP node_k={k} keygen_seconds={:.3}",
        t.elapsed().as_secs_f64()
    );
    let vp = np.verifier_params();
    let repr = node_vk.vk().transcript_repr();
    let mut rng = ChaCha20Rng::from_entropy();
    let mut st = IvcState {
        prog_d,
        init_d,
        cur_d: init_d,
        h: 0,
    };
    let (mut acc, mut proof) = (rel.trivial_acc(), Vec::new());
    let t_fold = Instant::now();
    for (h, (inst, lproof)) in leaves.steps.iter().enumerate() {
        let w = IvcWitness {
            self_repr: repr,
            prev: st.clone(),
            prev_proof: proof.clone(),
            prev_acc: acc.clone(),
            cur_d: inst[3],
            rule: inst[4],
            leaf_binding: inst[0],
            leaf_proof: lproof.clone(),
        };
        let next_acc = rel.next_acc(Some(&node_vk), &w).unwrap();
        let next = IvcState {
            prog_d,
            init_d,
            cur_d: inst[3],
            h: st.h + 1,
        };
        let ni = SherlockIvcRelation::instance(repr, &next, &next_acc);
        let t = Instant::now();
        let p = prove::<SherlockIvcRelation, Pos>(&np, &node_pk, &rel, &ni, w, &mut rng).unwrap();
        verify::<SherlockIvcRelation, Pos>(&vp, &node_vk, &ni, None, &p).unwrap();
        println!(
            "GP fold step={h} prove_seconds={:.3}",
            t.elapsed().as_secs_f64()
        );
        st = next;
        acc = next_acc;
        proof = p;
    }
    assert!(
        SherlockIvcRelation::check_acc(&vp, &node_vk, &acc),
        "carried accumulator pairs"
    );
    let ni = SherlockIvcRelation::instance(repr, &st, &acc);
    println!(
        "GP folded steps={n} fold_seconds={:.3}",
        t_fold.elapsed().as_secs_f64()
    );
    drop(node_pk);
    fs::create_dir(out.join("node")).unwrap();
    fs::write(out.join("node/node.vk"), vk_bytes(&node_vk)).unwrap();
    fs::write(out.join("node/node.proof"), &proof).unwrap();
    fs::write(out.join("node/node.instance.json"), hexes(&ni).to_string()).unwrap();
    let mut rb = Vec::new();
    rel.write_relation(&mut rb).unwrap();
    fs::write(out.join("node/node.relation"), rb).unwrap();

    // Run decider key and an offline proof checked by the ledger reader.
    let drel = RunDeciderRelation {
        nr_cols: 4,
        node_vk: node_vk.clone(),
        s_g2: base.s_g2().to_affine(),
        template: template(),
        cc_inputs: CC_INPUTS.to_vec(),
    };
    drel.validate().unwrap();
    let dk = cost_model(&drel, None).k;
    println!("GP decider model_k={dk}");
    assert!(dk <= 20);
    let dp = at_k(&base, dk);
    let t = Instant::now();
    let dvk = setup_vk(&dp, &drel);
    let dpk = setup_pk(&drel, &dvk);
    println!("GP decider keygen_seconds={:.3}", t.elapsed().as_secs_f64());
    let dw = RunDeciderWitness {
        w_instance: ni.clone(),
        w_proof: proof.clone(),
        w_acc: acc.clone(),
        result: leaves.result,
        lanes_d: leaves.lanes_d,
        comm_rand: F::random(OsRng),
    };
    let public = drel.public_inputs(F::random(OsRng), &dw).unwrap();
    let t = Instant::now();
    let dproof = prove::<RunDeciderRelation, B2>(&dp, &dpk, &drel, &public, dw, &mut rng).unwrap();
    println!(
        "GP decider prove_seconds={:.3} proof_bytes={}",
        t.elapsed().as_secs_f64(),
        dproof.len()
    );
    verify::<RunDeciderRelation, B2>(&dp.verifier_params(), &dvk, &public, None, &dproof).unwrap();
    let key = VerifierKey::from(dvk.clone());
    key.verify(
        &PARAMS_VERIFIER,
        &Proof(dproof.clone()),
        public.iter().copied().map(Fr),
    )
    .unwrap();
    for lane in 0..public.len() {
        let mut bad = public.clone();
        bad[lane] += F::ONE;
        assert!(
            key.verify(
                &PARAMS_VERIFIER,
                &Proof(dproof.clone()),
                bad.into_iter().map(Fr)
            )
            .is_err(),
            "lane {lane}"
        );
    }
    let mut tagged = Vec::new();
    midnight_serialize::tagged_serialize(&key, &mut tagged).unwrap();
    fs::create_dir(out.join("key")).unwrap();
    fs::write(out.join("key/decider.vk"), vk_bytes(&dvk)).unwrap();
    let mut drb = Vec::new();
    drel.write_relation(&mut drb).unwrap();
    fs::write(out.join("key/decider.relation"), &drb).unwrap();
    fs::write(out.join("key/acceptRun.verifier"), &tagged).unwrap();
    let run_values = RunDeciderRelation::values(&ni, leaves.result);
    let stmt = RunDeciderRelation::stmt(&run_values);
    let summary = json!({
        "schema": "sherlock-gp-bundle/1",
        "standin": std::env::var("MORIARTY_GP_STANDIN_STEPS").is_ok(),
        "steps": n.to_string(), "result": leaves.result.to_string(),
        "prog_d": f_to_hex(&prog_d), "init_d": f_to_hex(&init_d), "fin_d": f_to_hex(&fin_d),
        "lanes_d": f_to_hex(&leaves.lanes_d),
        "stmt": f_to_hex(&stmt), "result_be16_hex": format!("{:032x}", leaves.result),
        "leaf_vk_sha256": sha256_hex(&vk_bytes(&leaves.vk)), "leaf_k": leaves.vk.k(),
        "node_vk_sha256": sha256_hex(&vk_bytes(&node_vk)), "node_k": k,
        "decider_vk_sha256": sha256_hex(&vk_bytes(&dvk)), "decider_k": dk,
        "decider_relation_sha256": sha256_hex(&drb),
        "acceptRun_verifier_sha256": sha256_hex(&tagged), "acceptRun_verifier_bytes": tagged.len(),
        "public_slots": public.len(),
    });
    fs::write(
        out.join("bundle.json"),
        serde_json::to_vec_pretty(&summary).unwrap(),
    )
    .unwrap();
    println!("GP accepted bundle={}", out.display());
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct CallOpening {
    binding_input: String,
    preimage: String,
}

#[test]
#[ignore = "acceptRun call proof from an SDK opening on stdin; heavy.lock"]
fn gp_call_prove() {
    let dir = env("MORIARTY_GP_DIR");
    let out = env("MORIARTY_GP_CALL_OUT");
    let call: CallOpening = serde_json::from_reader(std::io::stdin().lock()).unwrap();
    let bundle: Value =
        serde_json::from_slice(&fs::read(dir.join("bundle.json")).unwrap()).unwrap();
    let base = srs();
    let drb = fs::read(dir.join("key/decider.relation")).unwrap();
    let drel = RunDeciderRelation::read_relation(&mut &drb[..]).unwrap();
    assert_eq!(
        drel.template,
        template(),
        "decider relation carries the committed template"
    );
    assert_eq!(drel.cc_inputs, CC_INPUTS.to_vec());
    let dvk = vk_from_bytes(&fs::read(dir.join("key/decider.vk")).unwrap()).unwrap();
    assert_eq!(
        sha256_hex(&vk_bytes(&dvk)),
        bundle["decider_vk_sha256"].as_str().unwrap()
    );
    let ni = read_fs(dir.join("node/node.instance.json"));
    let w_acc = acc_from_lanes(&ni[5..], &drel.node_base_names()).expect("node accumulator lanes");
    let result: u128 = bundle["result"].as_str().unwrap().parse().unwrap();
    let lanes_d = f_from_hex(bundle["lanes_d"].as_str().unwrap()).unwrap();

    let raw = hex::decode(&call.preimage).unwrap();
    let mut reader = &raw[..];
    let p: ProofPreimage = midnight_serialize::tagged_deserialize(&mut reader).unwrap();
    assert!(reader.is_empty() && p.private_transcript.is_empty());
    let binding = f_from_hex(&call.binding_input).unwrap();
    let (comm, rand) = p.communications_commitment.expect("call commitment");
    assert!(midnight_transient_crypto::hash::transient_commit(&p.inputs, rand) == comm);
    let dw = RunDeciderWitness {
        w_instance: ni,
        w_proof: fs::read(dir.join("node/node.proof")).unwrap(),
        w_acc,
        result,
        lanes_d,
        comm_rand: rand.0,
    };
    let values = RunDeciderRelation::values(&dw.w_instance, result);
    let stmt = RunDeciderRelation::stmt(&values);
    let want_inputs: Vec<F> = CC_INPUTS
        .iter()
        .map(|&i| {
            if i == CC_STMT {
                stmt
            } else {
                values[i as usize]
            }
        })
        .collect();
    assert_eq!(
        p.inputs.iter().map(|f| f.0).collect::<Vec<_>>(),
        want_inputs,
        "acceptRun inputs"
    );
    let actual = [
        vec![binding, comm.0],
        p.public_transcript_inputs.iter().map(|f| f.0).collect(),
    ]
    .concat();
    let public = drel.public_inputs(binding, &dw).unwrap();
    assert_eq!(
        actual, public,
        "SDK call vector equals the decider template"
    );
    let dp = at_k(&base, dvk.k() as u32);
    let dpk = setup_pk(&drel, &dvk);
    let t = Instant::now();
    let proof = prove::<RunDeciderRelation, B2>(&dp, &dpk, &drel, &public, dw, &mut OsRng).unwrap();
    let key = VerifierKey::from(dvk);
    key.verify(
        &PARAMS_VERIFIER,
        &Proof(proof.clone()),
        public.iter().copied().map(Fr),
    )
    .unwrap();
    fs::create_dir(&out).unwrap();
    fs::write(out.join("call.proof"), &proof).unwrap();
    let mut transport = Vec::new();
    midnight_serialize::tagged_serialize(&Proof(proof.clone()), &mut transport).unwrap();
    fs::write(out.join("call.tagged-proof"), transport).unwrap();
    fs::write(out.join("call.public.json"), hexes(&public).to_string()).unwrap();
    println!(
        "GP call proof_bytes={} prove_seconds={:.3} stmt={} result={result}",
        proof.len(),
        t.elapsed().as_secs_f64(),
        f_to_hex(&stmt)
    );
}
