//! I6: the full wrap of one exported `L_Fr` proof. Cost model, MockProver (honest, a changed
//! public input, a changed proof value), keygen on the ceremony SRS, one blinded proof
//! (OsRng), library verify, and the files for `ledger-check`.
//! usage: wrap <fixture name or path> <out dir> <k> [mode: 0|1|cost] [key fixture]
//! E6: with a key fixture the relation (shape and the pinned `L` pre-hash) is built from the
//! key fixture and the witness from the first fixture; with `E6_EXPECT_VK=<path>` the run
//! asserts the key equals that file byte for byte (the key installed on the devnet).
use std::{fs, path::PathBuf, time::Instant};

use ff::Field;
use midnight_proofs::{circuit::Value, dev::MockProver, utils::SerdeFormat};
use midnight_zk_stdlib::{MidnightCircuit, cost_model, prove, setup_pk, setup_vk, verify};
use rand::rngs::OsRng;
use serde_json::json;
use sherlock_wrap::{
    native::{F, f_to_hex},
    util::{K21_GATE, srs},
    verifier::ProofData,
    wrap::WrapRelation,
    wrapin::{WrapIn, fixture},
};

type B2 = blake2b_simd::State;

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let src = if a[1].contains('/') { PathBuf::from(&a[1]) } else { fixture(&a[1]) };
    let out = PathBuf::from(&a[2]);
    let k: u32 = a[3].parse().unwrap();
    let mode = a.get(4).cloned().unwrap_or_else(|| "1".into());
    let mock = mode == "1";
    assert!(k <= 21, "never k = 22");
    fs::create_dir_all(&out).unwrap();
    let w = WrapIn::load(&src);
    let rel = match a.get(5) {
        Some(kf) => {
            let kp = if kf.contains('/') { PathBuf::from(kf) } else { fixture(kf) };
            WrapRelation::from_wrapin(&WrapIn::load(&kp), 1)
        }
        None => WrapRelation::from_wrapin(&w, 1),
    };
    let p = ProofData::from_wrapin(&w);
    let pi = rel.instance(&p);

    let model = cost_model(&rel, Some(k));
    println!("E5JSON {}", json!({"phase": "cost_model", "fixture": a[1], "k": k, "rows": model.rows,
        "k21_gate": K21_GATE, "within_gate": model.rows <= K21_GATE, "table_rows": model.table_rows,
        "advice_columns": model.advice_columns, "fixed_columns": model.fixed_columns, "lookups": model.lookups}));
    assert!(model.rows <= K21_GATE, "rows {} above the k = 21 gate {K21_GATE}", model.rows);
    if mode == "cost" {
        return;
    }

    if mock {
        let run = |pi: &Vec<F>, p: &ProofData| {
            let c = MidnightCircuit::new(&rel, Value::known(pi.clone()), Value::known(p.clone()), Some(k));
            MockProver::<F>::run(&c, vec![vec![], pi.clone()]).unwrap().verify().is_ok()
        };
        let t = Instant::now();
        let ok = run(&pi, &p);
        let mock_s = t.elapsed().as_secs_f64();
        let mut bad = pi.clone();
        bad[0] += F::ONE;
        let pi_refused = !run(&bad, &p);
        let mut bp = p.clone();
        bp.whir.initial_rows[0][1][2][0] += F::ONE;
        let val_refused = !run(&pi, &bp);
        println!("E5JSON {}", json!({"phase": "mock", "honest_ok": ok, "changed_pi_refused": pi_refused,
            "changed_proof_value_refused": val_refused, "mock_s": mock_s}));
        assert!(ok && pi_refused && val_refused);
    }

    let params = srs(k);
    let t = Instant::now();
    let vk = setup_vk(&params, &rel);
    let pk = setup_pk(&rel, &vk);
    let keygen_s = t.elapsed().as_secs_f64();
    let t = Instant::now();
    let proof = prove::<WrapRelation, B2>(&params, &pk, &rel, &pi, p, OsRng).unwrap();
    let prove_s = t.elapsed().as_secs_f64();
    let lib_ok = verify::<WrapRelation, B2>(&params.verifier_params(), &vk, &pi, None, &proof).is_ok();
    let mut key = vec![];
    vk.write(&mut key, SerdeFormat::Processed).unwrap();
    if let Ok(path) = std::env::var("E6_EXPECT_VK") {
        assert_eq!(key, fs::read(&path).unwrap(), "the wrap key differs from {path}");
    }
    fs::write(out.join("wrap.vk"), &key).unwrap();
    fs::write(out.join("wrap.proof"), &proof).unwrap();
    fs::write(out.join("wrap.public.json"), serde_json::to_vec(&pi.iter().map(f_to_hex).collect::<Vec<_>>()).unwrap()).unwrap();
    println!("E5JSON {}", json!({"phase": "prove", "k": k, "keygen_s": keygen_s, "prove_s": prove_s,
        "proof_bytes": proof.len(), "key_bytes_midnightvk": key.len(), "library_verify_ok": lib_ok, "public_inputs": pi.len()}));
    assert!(lib_ok);
}
