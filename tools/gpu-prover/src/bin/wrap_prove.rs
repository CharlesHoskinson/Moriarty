// SPDX-License-Identifier: Apache-2.0
//! The run decider (the k = 21 wrap proxy) proved on the CPU path and on GPU paths in
//! one process with one key, each proof checked by the library verifier and by the
//! ledger 9 verifier (`midnight_transient_crypto::proofs::VerifierKey::verify`), and
//! the proof bytes compared under a fixed test seed (test only; production blinding
//! stays `OsRng`).
//!
//! Environment:
//!   GW_SRS      ceremony SRS file (bls_midnight_2p20 or 2p21)
//!   GW_BUNDLE   a `gp_fold_and_key` output directory (node/, key/decider.relation, bundle.json)
//!   GW_K        decider k (20 or 21)
//!   GW_CONFIGS  ';'-separated phase lists, first is the reference (default "cpu;all")
//!   GW_REPS     proofs per config (default 1)
//!   GW_SEED     u64 test seed (default 20261111)
//!   GW_DET      1: fixed lookup leftover order (needed for byte identity across runs)
//!   GW_OUT      output directory (measure.jsonl, proofs)
use std::{fs, io::Cursor, io::Write, path::PathBuf, time::Instant};

use ff::Field;
use midnight_curves::Bls12;
use midnight_proofs::{
    poly::{commitment::Params, kzg::params::ParamsKZG},
    utils::SerdeFormat,
};
use midnight_transient_crypto::{
    curve::Fr,
    proofs::{PARAMS_VERIFIER, Proof, VerifierKey},
};
use midnight_zk_stdlib::{Relation, prove, setup_pk, setup_vk, verify};
use mori_gpu_prover::proxy::PoseidonChain;
use mori_recursion::{
    ivc::acc_from_lanes,
    run_decider::{RunDeciderRelation, RunDeciderWitness},
    util::{F, f_from_hex, sha256_hex},
};
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use serde_json::{Value, json};
use sha2::Digest;

type B2 = blake2b_simd::State;

fn env(n: &str) -> String {
    std::env::var(n).unwrap_or_else(|_| panic!("{n} is required"))
}

fn hwm() -> u64 {
    let s = fs::read_to_string("/proc/self/status").unwrap();
    let l = s.lines().find(|l| l.starts_with("VmHWM:")).unwrap();
    l.split_whitespace().nth(1).unwrap().parse::<u64>().unwrap() * 1024
}

fn rss() -> u64 {
    let s = fs::read_to_string("/proc/self/status").unwrap();
    let l = s.lines().find(|l| l.starts_with("VmRSS:")).unwrap();
    l.split_whitespace().nth(1).unwrap().parse::<u64>().unwrap() * 1024
}

fn hwm_reset() {
    fs::write("/proc/self/clear_refs", "5").unwrap();
}

fn read_fs(p: PathBuf) -> Vec<F> {
    let v: Vec<String> = serde_json::from_slice(&fs::read(p).unwrap()).unwrap();
    v.iter().map(|s| f_from_hex(s).unwrap()).collect()
}

fn main() {
    let out = PathBuf::from(env("GW_OUT"));
    fs::create_dir_all(&out).unwrap();
    let mut log = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(out.join("measure.jsonl"))
        .unwrap();
    let mut rec = |v: Value| {
        println!("GW {v}");
        writeln!(log, "{v}").unwrap();
    };
    let k: u32 = env("GW_K").parse().unwrap();
    let seed: u64 = std::env::var("GW_SEED").map_or(20261111, |s| s.parse().unwrap());
    let reps: usize = std::env::var("GW_REPS").map_or(1, |s| s.parse().unwrap());
    let configs = std::env::var("GW_CONFIGS").unwrap_or_else(|_| "cpu;all".into());
    let det = std::env::var("GW_DET").as_deref() == Ok("1");
    midnight_proofs::hooks::DETERMINISTIC_LOOKUP.store(det, std::sync::atomic::Ordering::Relaxed);
    let qcheck = std::env::var("GW_QCHECK").as_deref() == Ok("1");
    midnight_proofs::hooks::QUOTIENT_CHECK.store(qcheck, std::sync::atomic::Ordering::Relaxed);

    let info = mori_gpu_prover::install(0);
    rec(
        json!({"phase": "start", "k": k, "seed": seed, "deterministic_lookup": det,
               "rayon_threads": rayon_threads(), "gpu": info.map(|i| format!("{i:?}")),
               "configs": configs}),
    );

    // SRS and the saved node bundle.
    let bytes = fs::read(env("GW_SRS")).unwrap();
    let srs_sha = sha256_hex(&bytes);
    assert!(
        [
            "1cc62978558fdc1e445cd70cfd9a86ec3c2e2151b6d74811232d37faf9133ff1",
            "9cf1644a87f0f027ae5fc6278f91d823a6334ff3e338a29e2f2ef57d071ed64d",
        ]
        .contains(&srs_sha.as_str()),
        "official k20 or k21 ceremony SRS"
    );
    let base =
        ParamsKZG::<Bls12>::read_custom(&mut Cursor::new(bytes), SerdeFormat::RawBytesUnchecked)
            .unwrap();
    assert!(k <= base.max_k());
    let mut dp = base.clone();
    dp.downsize(k);
    drop(base);
    let mut ctx = Ctx {
        k,
        seed,
        reps,
        configs,
        info,
        out: out.clone(),
        srs_sha,
        bundle_vk_sha: None,
    };
    if let Ok(iters) = std::env::var("GW_PROXY_ITERS") {
        // k = 19 stand-in with the decider's constraint system (proxy.rs).
        let rel = PoseidonChain {
            iters: iters.parse().unwrap(),
        };
        let m = midnight_zk_stdlib::cost_model(&rel, Some(k));
        rec(
            json!({"phase": "proxy_model", "k": k, "iters": rel.iters, "rows": m.rows, "model_k": m.k}),
        );
        let mut wr = ChaCha20Rng::seed_from_u64(seed ^ 0x5eed);
        let x0 = F::random(&mut wr);
        let public = vec![rel.output(x0)];
        run(&ctx, &mut rec, &dp, &rel, &public, x0);
    } else {
        let dir = PathBuf::from(env("GW_BUNDLE"));
        let bundle: Value =
            serde_json::from_slice(&fs::read(dir.join("bundle.json")).unwrap()).unwrap();
        let drb = fs::read(dir.join("key/decider.relation")).unwrap();
        let drel = RunDeciderRelation::read_relation(&mut &drb[..]).unwrap();
        let ni = read_fs(dir.join("node/node.instance.json"));
        let w_acc =
            acc_from_lanes(&ni[5..], &drel.node_base_names()).expect("node accumulator lanes");
        let result: u128 = bundle["result"].as_str().unwrap().parse().unwrap();
        let lanes_d = f_from_hex(bundle["lanes_d"].as_str().unwrap()).unwrap();
        let mut wr = ChaCha20Rng::seed_from_u64(seed ^ 0x5eed);
        let dw = RunDeciderWitness {
            w_instance: ni,
            w_proof: fs::read(dir.join("node/node.proof")).unwrap(),
            w_acc,
            result,
            lanes_d,
            comm_rand: F::random(&mut wr),
        };
        let public = drel.public_inputs(F::random(&mut wr), &dw).unwrap();

        if bundle["decider_k"].as_u64() == Some(k as u64) {
            ctx.bundle_vk_sha = bundle["decider_vk_sha256"].as_str().map(String::from);
        }
        run(&ctx, &mut rec, &dp, &drel, &public, dw);
    }
    rec(json!({"phase": "done"}));
}

struct Ctx {
    k: u32,
    seed: u64,
    reps: usize,
    configs: String,
    info: Option<&'static mori_gpu::GpuInfo>,
    out: PathBuf,
    srs_sha: String,
    bundle_vk_sha: Option<String>,
}

fn run<R: Relation<Instance = Vec<F>>>(
    c: &Ctx,
    rec: &mut impl FnMut(Value),
    dp: &ParamsKZG<Bls12>,
    rel: &R,
    public: &Vec<F>,
    witness: R::Witness,
) where
    R::Witness: Clone,
{
    let (k, seed, reps, info, out) = (c.k, c.seed, c.reps, c.info, &c.out);
    let t = Instant::now();
    let dvk = setup_vk(dp, rel);
    let dpk = setup_pk(rel, &dvk);
    let vk_sha = sha256_hex(&mori_recursion::util::vk_bytes(&dvk));
    let dom = dvk.vk().get_domain();
    rec(
        json!({"phase": "keygen", "k": k, "vk_k": dvk.k(), "seconds": t.elapsed().as_secs_f64(),
               "srs_sha256": c.srs_sha, "rss_bytes": rss(), "vk_sha256": vk_sha,
               "bundle_decider_vk_sha256": c.bundle_vk_sha, "vk_equals_bundle": c.bundle_vk_sha.as_deref() == Some(vk_sha.as_str()),
               "cs_degree": dvk.vk().cs().degree(), "extended_k": dom.extended_k(),
               "extended_factor": 1u64 << (dom.extended_k() - dom.k())}),
    );
    let key = VerifierKey::from(dvk.clone());
    let vp = dp.verifier_params();

    let mut reference: Option<Vec<u8>> = None;
    for cfg in c.configs.split(';') {
        let mask = mori_gpu_prover::parse_phases(cfg);
        if mask != 0 && info.is_none() {
            panic!("config {cfg} needs a GPU");
        }
        mori_gpu_prover::set_phases(mask);
        for rep in 0..reps {
            mori_gpu::reset_stats();
            let mut rng = ChaCha20Rng::seed_from_u64(seed);
            let rss0 = rss();
            hwm_reset();
            let t = Instant::now();
            let proof = prove::<R, B2>(dp, &dpk, rel, public, witness.clone(), &mut rng).unwrap();
            let prove_s = t.elapsed().as_secs_f64();
            let peak = hwm();
            let st = mori_gpu::stats();
            mori_gpu::release_scratch();
            // Verify on the stock CPU path (hooks declining), independent of the GPU.
            mori_gpu_prover::set_phases(0);
            let need = |bit: u32, n: u64| mask & bit == 0 || n > 0;
            let valid = st.fallbacks == 0
                && need(mori_gpu_prover::MSM, st.msm_calls)
                && need(mori_gpu_prover::NTT, st.ntt_calls)
                && need(mori_gpu_prover::COSET, st.coset_calls)
                && need(mori_gpu_prover::QUOTIENT, st.quotient_calls);
            let lib_ok = verify::<R, B2>(&vp, &dvk, public, None, &proof).is_ok();
            let t = Instant::now();
            let ledger_ok = key
                .verify(
                    &PARAMS_VERIFIER,
                    &Proof(proof.clone()),
                    public.iter().copied().map(Fr),
                )
                .is_ok();
            let ledger_s = t.elapsed().as_secs_f64();
            // negative control: one public lane off by one must be rejected
            let mut bad = public.clone();
            let lane = 3.min(bad.len() - 1);
            bad[lane] += F::ONE;
            let neg_rejected = key
                .verify(
                    &PARAMS_VERIFIER,
                    &Proof(proof.clone()),
                    bad.into_iter().map(Fr),
                )
                .is_err();
            let same = match &reference {
                None => {
                    reference = Some(proof.clone());
                    true
                }
                Some(r) => *r == proof,
            };
            let sha = hex::encode(sha2::Sha256::digest(&proof));
            fs::write(
                out.join(format!("proof-k{k}-{}-{rep}.bin", cfg.replace(',', "+"))),
                &proof,
            )
            .unwrap();
            rec(
                json!({"phase": "prove", "k": k, "config": cfg, "mask": mask, "rep": rep,
                       "prove_seconds": prove_s, "proof_bytes": proof.len(), "proof_sha256": sha,
                       "byte_identical_to_reference": same, "gpu_path_valid": valid,
                       "msm_table_builds_total": mori_gpu::table_builds(),
                       "quotient_shape_calcs_slots_lookups_sets_resident": *midnight_proofs::hooks::LAST_QUOTIENT_SHAPE.lock().unwrap(),
                       "quotient_check_diffs_total": midnight_proofs::hooks::quotient_check_diffs(), "library_verify": lib_ok,
                       "ledger9_verify": ledger_ok, "ledger9_verify_seconds": ledger_s,
                       "negative_control_rejected": neg_rejected,
                       "peak_rss_bytes": peak, "rss_before_bytes": rss0,
                       "loadavg": fs::read_to_string("/proc/loadavg").unwrap_or_default().trim(),
                       "gpu_device_bytes_peak": mori_gpu::device_bytes_peak(),
                       "gpu": {"msm_calls": st.msm_calls, "msm_s": st.msm_ns as f64 * 1e-9,
                               "ntt_calls": st.ntt_calls, "ntt_s": st.ntt_ns as f64 * 1e-9,
                               "coset_calls": st.coset_calls, "quotient_calls": st.quotient_calls,
                               "quotient_s": st.quotient_ns as f64 * 1e-9, "fallbacks": st.fallbacks}}),
            );
            mori_gpu::reset_device_bytes_peak();
            assert!(lib_ok && ledger_ok, "proof must verify ({cfg})");
            mori_gpu_prover::set_phases(mask);
        }
    }
}

fn rayon_threads() -> usize {
    std::env::var("RAYON_NUM_THREADS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| std::thread::available_parallelism().map_or(1, |n| n.get()))
}
