//! Explicit-run GP node-chain harness (task 2.7 of the Sherlock consensus change, RPP task 5.3).
//! Building this binary never starts a proof.
//!
//! The run reproduces `gp_fold_and_key` (`crates/mori-recursion/tests/sherlock_gp.rs`) through
//! the public APIs, with caller spans around SRS and key load, `setup_pk`, each node `prove` and
//! `verify`, decider keygen and prove, and the ledger verifier. Leaves are stand-ins
//! (`MORIARTY_GP_STANDIN_STEPS`, a rehearsal) or proved leaves from `MORIARTY_GP_LEAF_DIR`. After
//! the command span closes, the node row count and its per-gadget breakdown (modeling only) are
//! added to the sidecar. CPU and RSS per phase come from the harness counters (feature on).
use std::{error::Error, fs, io::Cursor, path::PathBuf};

use ff::{Field, PrimeField};
use group::Curve;
use midnight_curves::Bls12;
use midnight_proofs::{
    poly::kzg::params::ParamsKZG, profile_result, profile_span, utils::SerdeFormat,
};
use midnight_transient_crypto::{
    curve::Fr,
    proofs::{PARAMS_VERIFIER, Proof, VerifierKey},
};
use midnight_zk_stdlib::{MidnightVK, cost_model, setup_vk};
use mori_recursion::{
    ivc::{
        IvcState, IvcWitness, OUTCOME_HALTED, OUTCOME_RUNNING, SherlockIvcRelation, state_digest,
    },
    parent::poseidon,
    run_decider::{
        CC_STMT, RunDeciderRelation, RunDeciderWitness, V_FIN, V_INIT, V_PROG, V_RESULT, V_STEPS,
    },
    standin::{StandInLeaf, standin_body},
    template::Template,
    util::{F, f_from_hex, f_to_hex, sha256_hex, vk_bytes, vk_from_bytes},
};
use mori_recursion_profile::{
    expected_rejection, node_rows::node_row_breakdown, prove, setup_pk, sidecar::Sidecar, verify,
};
use rand::{SeedableRng, rngs::OsRng};
use rand_chacha::ChaCha20Rng;
use serde_json::{Value, json};

type Result<T> = std::result::Result<T, Box<dyn Error>>;
type Pos = midnight_circuits::hash::poseidon::PoseidonState<F>;
type B2 = blake2b_simd::State;

/// The official k20 ceremony SRS (`sherlock_gp.rs` `srs()`).
const SRS_SHA256: &str = "1cc62978558fdc1e445cd70cfd9a86ec3c2e2151b6d74811232d37faf9133ff1";
const CC_INPUTS: [u32; 6] = [CC_STMT, V_PROG, V_INIT, V_STEPS, V_FIN, V_RESULT];
const USAGE: &str = "usage: profile-gp --workload GP --cache-mode resident_pk --threads N --sidecar PATH [--srs PATH] [--node-k K]";

struct Options {
    srs: PathBuf,
    threads: usize,
    node_k: u32,
}

struct Leaves {
    vk: MidnightVK,
    steps: Vec<(Vec<F>, Vec<u8>)>,
    result: u128,
    lanes_d: F,
    standin: bool,
}

fn at_k(base: &ParamsKZG<Bls12>, k: u32) -> ParamsKZG<Bls12> {
    let mut p = base.clone();
    p.downsize(k);
    p
}

/// A small field element (a step count) as u64.
fn small(x: &F) -> Result<u64> {
    let r = x.to_repr();
    let b: &[u8] = r.as_ref();
    if b[8..].iter().any(|&z| z != 0) {
        return Err("step count out of range".into());
    }
    Ok(u64::from_le_bytes(b[..8].try_into()?))
}

fn load_srs(path: &PathBuf) -> Result<ParamsKZG<Bls12>> {
    let bytes = fs::read(path)?;
    if sha256_hex(&bytes) != SRS_SHA256 {
        return Err("SRS is not the official k20 ceremony file".into());
    }
    Ok(ParamsKZG::read_custom(
        &mut Cursor::new(bytes),
        SerdeFormat::RawBytesUnchecked,
    )?)
}

fn standin_leaves(base: &ParamsKZG<Bls12>, n: u64, result: u128) -> Result<Leaves> {
    let mut rng = ChaCha20Rng::from_entropy();
    let leaf = StandInLeaf;
    let p = at_k(base, cost_model(&leaf, None).k);
    let vk = profile_span!(KeyLoad, Exclusive, setup_vk(&p, &leaf));
    let pk = setup_pk(&leaf, &vk);
    let prog_d = poseidon(&[F::from(0x5052_4f47u64)]);
    let mut pre = poseidon(&[F::from(0x494e_4954u64)]);
    let (mut steps, mut lanes_d) = (Vec::new(), F::ZERO);
    const PER_LEAF: u64 = 2;
    for i in 0..n {
        let h = i * PER_LEAF;
        let rule = F::from(1000 + i);
        let body = standin_body(prog_d, pre, rule, F::from(h));
        let (tag, val) = if i + 1 == n {
            (OUTCOME_HALTED, result)
        } else {
            (OUTCOME_RUNNING, 0)
        };
        let post = state_digest(tag, val, body);
        let inst = vec![
            F::from(0xb1d0 + i),
            prog_d,
            pre,
            post,
            rule,
            F::from(h),
            F::from(PER_LEAF),
        ];
        let mut w = inst.clone();
        w.extend([F::from(tag), F::from_u128(val)]);
        let proof = prove::<StandInLeaf, Pos>(&p, &pk, &leaf, &inst, w, &mut rng)
            .map_err(|e| format!("stand-in leaf {i} prove: {e:?}"))?;
        steps.push((inst, proof));
        pre = post;
        lanes_d = body;
    }
    Ok(Leaves {
        vk,
        steps,
        result,
        lanes_d,
        standin: true,
    })
}

fn read_fs(path: PathBuf) -> Result<Vec<F>> {
    let v: Vec<String> = serde_json::from_slice(&fs::read(path)?)?;
    v.iter().map(|s| Ok(f_from_hex(s)?)).collect()
}

fn dir_leaves(dir: PathBuf) -> Result<Leaves> {
    let vk = profile_result!(
        KeyLoad,
        Exclusive,
        vk_from_bytes(&fs::read(dir.join("leaf.vk"))?)
    )?;
    let mut names = Vec::new();
    for entry in fs::read_dir(&dir)? {
        let name = entry?
            .file_name()
            .into_string()
            .map_err(|_| "bad leaf name")?;
        if name.starts_with("step-") {
            names.push(name);
        }
    }
    names.sort();
    let mut steps = Vec::new();
    for n in &names {
        let d = dir.join(n);
        steps.push((
            read_fs(d.join("leaf.instance.json"))?,
            fs::read(d.join("leaf.proof"))?,
        ));
    }
    let fin: Value = serde_json::from_slice(&fs::read(dir.join("final.json"))?)?;
    Ok(Leaves {
        vk,
        steps,
        result: fin["result"].as_str().ok_or("final.json result")?.parse()?,
        lanes_d: f_from_hex(fin["lanes_d"].as_str().ok_or("final.json lanes_d")?)?,
        standin: false,
    })
}

fn template() -> Result<Template> {
    Ok(Template::from_any_json(include_bytes!(
        "../../../../crates/mori-recursion/tests/fixtures/run-template.json"
    ))?)
}

fn run(options: &Options, evidence: &mut Value) -> Result<SherlockIvcRelation> {
    let base = profile_result!(KeyLoad, Exclusive, load_srs(&options.srs))?;
    let leaves = match std::env::var("MORIARTY_GP_STANDIN_STEPS") {
        Ok(n) => standin_leaves(&base, n.parse()?, 6)?,
        Err(_) => dir_leaves(PathBuf::from(
            std::env::var_os("MORIARTY_GP_LEAF_DIR")
                .ok_or("MORIARTY_GP_STANDIN_STEPS or MORIARTY_GP_LEAF_DIR is required")?,
        ))?,
    };
    let n = leaves.steps.len() as u64;
    if n == 0 {
        return Err("no leaves".into());
    }

    // Every leaf verifies on its own, and the leaf chain is contiguous.
    let (prog_d, init_d) = (leaves.steps[0].0[1], leaves.steps[0].0[2]);
    let leaf_params = at_k(&base, leaves.vk.k() as u32);
    let mut machine_steps = 0u64;
    for (h, (inst, proof)) in leaves.steps.iter().enumerate() {
        if inst.len() != 7 || inst[1] != prog_d || inst[5] != F::from(machine_steps) {
            return Err(format!("leaf {h}: instance shape, program or height").into());
        }
        let s = small(&inst[6])?;
        if s == 0 || (h > 0 && inst[2] != leaves.steps[h - 1].0[3]) {
            return Err(format!("leaf {h}: empty or not linked").into());
        }
        machine_steps += s;
        verify::<StandInLeaf, Pos>(
            &leaf_params.verifier_params(),
            &leaves.vk,
            inst,
            None,
            proof,
        )
        .map_err(|e| format!("leaf {h} verify: {e:?}"))?;
    }
    let fin_d = leaves.steps[n as usize - 1].0[3];
    if fin_d != state_digest(OUTCOME_HALTED, leaves.result, leaves.lanes_d) {
        return Err("final leaf is not a halted digest".into());
    }

    // Node key: the smallest k from options.node_k whose cost model fits.
    let mut k = options.node_k;
    let rel = loop {
        let r = SherlockIvcRelation::new(4, k, leaves.vk.clone())?;
        if cost_model(&r, Some(k)).k <= k {
            break r;
        }
        k += 1;
        if k > 20 {
            return Err("node exceeds k20".into());
        }
    };
    let np = at_k(&base, k);
    let node_vk = profile_span!(KeyLoad, Exclusive, setup_vk(&np, &rel));
    let node_pk = setup_pk(&rel, &node_vk);
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
    let mut proofs = Vec::new();
    for (h, (inst, lproof)) in leaves.steps.iter().enumerate() {
        let w = IvcWitness {
            self_repr: repr,
            prev: st.clone(),
            prev_proof: proof.clone(),
            prev_acc: acc.clone(),
            cur_d: inst[3],
            rule: inst[4],
            leaf_binding: inst[0],
            leaf_steps: small(&inst[6])?,
            leaf_proof: lproof.clone(),
        };
        let next_acc = rel.next_acc(Some(&node_vk), &w)?;
        let next = IvcState {
            prog_d,
            init_d,
            cur_d: inst[3],
            h: st.h + small(&inst[6])?,
        };
        let ni = SherlockIvcRelation::instance(repr, &next, &next_acc);
        let p = prove::<SherlockIvcRelation, Pos>(&np, &node_pk, &rel, &ni, w, &mut rng)
            .map_err(|e| format!("node step {h} prove: {e:?}"))?;
        verify::<SherlockIvcRelation, Pos>(&vp, &node_vk, &ni, None, &p)
            .map_err(|e| format!("node step {h} verify: {e:?}"))?;
        proofs.push(
            json!({"shape":"node","step":h,"k":k,"transcript":"poseidon",
            "vk_sha256":sha256_hex(&vk_bytes(&node_vk)),"proof_sha256":sha256_hex(&p),
            "proof_bytes":p.len(),"cache_mode":"resident_pk"}),
        );
        st = next;
        acc = next_acc;
        proof = p;
    }
    if !SherlockIvcRelation::check_acc(&vp, &node_vk, &acc) {
        return Err("carried accumulator does not pair".into());
    }
    let ni = SherlockIvcRelation::instance(repr, &st, &acc);
    // Negative control on the last node proof: one flipped byte must be rejected.
    let mut bad = proof.clone();
    *bad.last_mut().ok_or("empty node proof")? ^= 1;
    if !profile_span!(
        Verify,
        Exclusive,
        expected_rejection(|| {
            verify::<SherlockIvcRelation, Pos>(&vp, &node_vk, &ni, None, &bad).is_err()
        })
    ) {
        return Err("node verifier accepted a malformed proof".into());
    }
    drop(node_pk);

    // Run decider key and proof, checked by the ledger's verifier key.
    let drel = RunDeciderRelation {
        nr_cols: 4,
        node_vk: node_vk.clone(),
        s_g2: base.s_g2().to_affine(),
        template: template()?,
        cc_inputs: CC_INPUTS.to_vec(),
    };
    drel.validate()?;
    let dk = cost_model(&drel, None).k;
    if dk > 20 {
        return Err("decider exceeds k20".into());
    }
    let dp = at_k(&base, dk);
    let dvk = profile_span!(KeyLoad, Exclusive, setup_vk(&dp, &drel));
    let dpk = setup_pk(&drel, &dvk);
    let dw = RunDeciderWitness {
        w_instance: ni.clone(),
        w_proof: proof.clone(),
        w_acc: acc.clone(),
        result: leaves.result,
        lanes_d: leaves.lanes_d,
        comm_rand: F::random(OsRng),
    };
    let public = drel
        .public_inputs(F::random(OsRng), &dw)
        .ok_or("decider public inputs")?;
    let dproof = prove::<RunDeciderRelation, B2>(&dp, &dpk, &drel, &public, dw, &mut rng)
        .map_err(|e| format!("decider prove: {e:?}"))?;
    verify::<RunDeciderRelation, B2>(&dp.verifier_params(), &dvk, &public, None, &dproof)
        .map_err(|e| format!("decider verify: {e:?}"))?;
    proofs.push(json!({"shape":"run_decider","k":dk,"transcript":"blake2b",
        "vk_sha256":sha256_hex(&vk_bytes(&dvk)),"proof_sha256":sha256_hex(&dproof),
        "proof_bytes":dproof.len(),"cache_mode":"resident_pk"}));
    evidence["verification"]["stock_verifier"] = json!("pass");
    let key = VerifierKey::from(dvk.clone());
    profile_result!(
        Verify,
        Exclusive,
        key.verify(
            &PARAMS_VERIFIER,
            &Proof(dproof.clone()),
            public.iter().copied().map(Fr)
        )
    )
    .map_err(|e| format!("ledger verify: {e:?}"))?;
    evidence["verification"]["installed_ledger_verifier"] = json!("pass");
    for lane in 0..public.len() {
        let mut bad = public.clone();
        bad[lane] += F::ONE;
        let rejected = profile_span!(
            Verify,
            Exclusive,
            expected_rejection(|| {
                key.verify(
                    &PARAMS_VERIFIER,
                    &Proof(dproof.clone()),
                    bad.into_iter().map(Fr),
                )
                .is_err()
            })
        );
        if !rejected {
            return Err(format!("ledger verifier accepted a changed lane {lane}").into());
        }
    }
    evidence["verification"]["negative_controls"] = json!("pass");
    evidence["verification"]["official_srs_authenticated"] = json!(true);
    evidence["proofs"] = json!(proofs);
    evidence["chain"] = json!({
        "standin": leaves.standin, "leaves": n, "machine_steps": st.h, "leaf_k": leaves.vk.k(),
        "node_k": k, "decider_k": dk, "public_slots": public.len(),
        "leaf_vk_sha256": sha256_hex(&vk_bytes(&leaves.vk)), "fin_d": f_to_hex(&fin_d),
    });
    Ok(rel)
}

fn main() -> Result<()> {
    let mut sidecar = None;
    let mut threads = None;
    let mut srs = std::env::var_os("MORIARTY_PARENT_SRS")
        .map(PathBuf::from)
        .or_else(|| {
            std::env::var_os("HOME")
                .map(|h| PathBuf::from(h).join(".cache/midnight/zk-params/bls_midnight_2p20"))
        })
        .ok_or("MORIARTY_PARENT_SRS or HOME is required to locate the k20 SRS")?;
    let mut node_k = std::env::var("MORIARTY_GP_NODE_K").map_or(Ok(18), |s| s.parse())?;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        let value = args.next().ok_or(USAGE)?;
        match arg.as_str() {
            "--sidecar" => sidecar = Some(PathBuf::from(value)),
            "--threads" => threads = Some(value.parse::<usize>()?),
            "--srs" => srs = PathBuf::from(value),
            "--node-k" => node_k = value.parse()?,
            "--workload" if value == "GP" => {}
            "--cache-mode" if value == "resident_pk" => {}
            "--cache-mode" => {
                return Err("GP keeps the node PK resident; only resident_pk is supported".into());
            }
            _ => return Err(USAGE.into()),
        }
    }
    let path = sidecar.ok_or("--sidecar is required")?;
    let threads = threads
        .filter(|t| *t > 0)
        .ok_or("--threads N is required")?;
    // The prover's pool reads RAYON_NUM_THREADS; the recorded count must be the one in force.
    if std::env::var("RAYON_NUM_THREADS").ok().as_deref() != Some(threads.to_string().as_str()) {
        return Err("RAYON_NUM_THREADS must equal --threads".into());
    }
    if !(10..=20).contains(&node_k) {
        return Err("--node-k must be in 10..=20".into());
    }
    let options = Options {
        srs,
        threads,
        node_k,
    };
    let logical = std::thread::available_parallelism().map_or(0, |n| n.get());
    let mut evidence = json!({"id":"GP","cache_mode":"resident_pk",
        "host":{"rayon_threads":options.threads,"logical_cpus_available":logical},
        "verification":{"stock_verifier":"not_run","negative_controls":"not_run",
            "installed_ledger_verifier":"not_run","official_srs_authenticated":false},
        "proofs":[],"binary_sha256":sha256_hex(&fs::read(std::env::current_exe()?)?),
        "cargo_lock_sha256":sha256_hex(include_bytes!("../../Cargo.lock")),
        "patch_sha256":sha256_hex(include_bytes!("../../patches/midnight-proofs-0.8.3-profile.patch"))});
    let writer = Sidecar::start(&path)?;
    let result = profile_result!(Command, Diagnostic, run(&options, &mut evidence));
    // Modeling only, outside the command span so it never enters the phase partition.
    let rows = result.as_ref().ok().map(node_row_breakdown);
    match &rows {
        Some(Ok(rows)) => evidence["node_rows"] = rows.to_json(),
        Some(Err(error)) => evidence["node_rows"] = json!({"error": error.to_string()}),
        None => {}
    }
    let written = writer.finish(evidence, result.is_ok());
    if let Err(error) = &written {
        eprintln!("profile-gp: sidecar write failed: {error}");
    }
    result?;
    written?;
    if let Some(Err(error)) = rows {
        return Err(error.into());
    }
    println!(
        "GP node chain, decider and ledger verification passed; sidecar={}",
        path.display()
    );
    Ok(())
}
