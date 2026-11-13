//! I0 smoke: cost model, MockProver (honest and a changed public input), keygen on the
//! ceremony SRS, one blinded proof (OsRng), library verify, then the key in the
//! `MidnightVK` framing, the proof and the public inputs written for `ledger-check`.
//! usage: smoke <out dir> <k> [nr_pow2range_cols] [pow_bits]
use std::{fs, path::PathBuf, time::Instant};

use ff::Field;
use midnight_proofs::{circuit::Value, dev::MockProver, utils::SerdeFormat};
use midnight_zk_stdlib::{MidnightCircuit, cost_model, prove, setup_pk, setup_vk, verify};
use rand::rngs::OsRng;
use serde_json::json;
use sherlock_wrap::{
    native::{F, f_to_hex},
    smoke::{NPI, SmokeRelation, SmokeWitness},
    util::srs,
};

type B2 = blake2b_simd::State;

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let out = PathBuf::from(&a[1]);
    let k: u32 = a[2].parse().unwrap();
    let nr_cols: u8 = a.get(3).map(|s| s.parse().unwrap()).unwrap_or(1);
    let pow_bits: usize = a.get(4).map(|s| s.parse().unwrap()).unwrap_or(0);
    fs::create_dir_all(&out).unwrap();
    let rel = SmokeRelation { nr_cols, pow_bits };
    let w: Vec<F> = (0..NPI - 1).map(|_| F::random(OsRng)).collect();
    let pow = if pow_bits > 0 { rel.grind(&w) } else { F::ZERO };
    let wit = SmokeWitness { w, pow };
    let pi = rel.instance(&wit);
    assert_eq!(pi.len(), NPI);

    let model = cost_model(&rel, Some(k));
    println!("E5JSON {}", json!({"phase": "cost_model", "k": k, "rows": model.rows, "table_rows": model.table_rows,
        "advice_columns": model.advice_columns, "fixed_columns": model.fixed_columns, "lookups": model.lookups}));

    let circ = MidnightCircuit::new(&rel, Value::known(pi.clone()), Value::known(wit.clone()), Some(k));
    let mock_ok = MockProver::<F>::run(&circ, vec![vec![], pi.clone()]).unwrap().verify().is_ok();
    let mut bad = pi.clone();
    bad[7] += F::ONE;
    let mock_bad = MockProver::<F>::run(&circ, vec![vec![], bad]).unwrap().verify().is_err();
    let mut badw = wit.clone();
    badw.pow += F::ONE;
    let circ_bad = MidnightCircuit::new(&rel, Value::known(pi.clone()), Value::known(badw), Some(k));
    let mock_badpow = pow_bits > 0
        && MockProver::<F>::run(&circ_bad, vec![vec![], pi.clone()]).unwrap().verify().is_err();
    println!("E5JSON {}", json!({"phase": "mock", "honest_ok": mock_ok, "changed_pi_refused": mock_bad,
        "changed_pow_refused": mock_badpow}));
    assert!(mock_ok && mock_bad && (pow_bits == 0 || mock_badpow));

    let params = srs(k);
    let t = Instant::now();
    let vk = setup_vk(&params, &rel);
    let pk = setup_pk(&rel, &vk);
    let keygen_s = t.elapsed().as_secs_f64();
    let t = Instant::now();
    let proof = prove::<SmokeRelation, B2>(&params, &pk, &rel, &pi, wit, OsRng).unwrap();
    let prove_s = t.elapsed().as_secs_f64();
    let lib_ok = verify::<SmokeRelation, B2>(&params.verifier_params(), &vk, &pi, None, &proof).is_ok();
    let mut key = vec![];
    vk.write(&mut key, SerdeFormat::Processed).unwrap();
    fs::write(out.join("wrap.vk"), &key).unwrap();
    fs::write(out.join("wrap.proof"), &proof).unwrap();
    fs::write(out.join("wrap.public.json"), serde_json::to_vec(&pi.iter().map(f_to_hex).collect::<Vec<_>>()).unwrap()).unwrap();
    println!("E5JSON {}", json!({"phase": "prove", "k": k, "keygen_s": keygen_s, "prove_s": prove_s,
        "proof_bytes": proof.len(), "key_bytes_midnightvk": key.len(), "library_verify_ok": lib_ok, "public_inputs": pi.len()}));
    assert!(lib_ok);
}
