//! Verify an E5 wrap proof as ledger 9.1.0.1 does: the key bytes in the `MidnightVK`
//! framing go through `VerifierKey`'s own deserializer (u64 length prefix, then
//! `MidnightVK::read`), then `VerifierKey::verify(&PARAMS_VERIFIER, ..)`. Negative controls:
//! one public input changed (first, middle, last) and one proof byte flipped (3 places).
//! usage: sherlock-wrap-ledger-check <dir with wrap.vk, wrap.proof, wrap.public.json>
use std::{fs, path::PathBuf, time::Instant};

use midnight_serialize::{Deserializable, Serializable};
use midnight_transient_crypto::{
    curve::Fr,
    proofs::{PARAMS_VERIFIER, Proof, VerifierKey},
};

/// 64-digit big-endian hex (as written by sherlock-wrap).
fn fr(h: &str) -> Fr {
    let mut b = hex::decode(h).unwrap();
    b.reverse();
    Fr::from_le_bytes(&b).expect("canonical scalar")
}

fn main() {
    let dir = PathBuf::from(std::env::args().nth(1).expect("dir"));
    let framed = fs::read(dir.join("wrap.vk")).unwrap();
    let proof = Proof(fs::read(dir.join("wrap.proof")).unwrap());
    let public: Vec<String> =
        serde_json::from_slice(&fs::read(dir.join("wrap.public.json")).unwrap()).unwrap();
    let pi: Vec<Fr> = public.iter().map(|h| fr(h)).collect();
    let mut ser = vec![];
    Serializable::serialize(&framed, &mut ser).unwrap();
    let key = VerifierKey::deserialize(&mut &ser[..], 0).expect("ledger reads the key");
    let t = Instant::now();
    let ok = key.verify(&PARAMS_VERIFIER, &proof, pi.iter().copied());
    let secs = t.elapsed().as_secs_f64();
    let n = pi.len();
    let mut pi_refused = vec![];
    for lane in [0, n / 2, n - 1] {
        let mut bad = pi.clone();
        bad[lane] = Fr(bad[lane].0 + Fr::from(1u64).0);
        pi_refused.push(key.verify(&PARAMS_VERIFIER, &proof, bad.into_iter()).is_err());
    }
    let mut byte_refused = vec![];
    for pos in [0, proof.0.len() / 2, proof.0.len() - 1] {
        let mut pb = proof.0.clone();
        pb[pos] ^= 1;
        byte_refused.push(key.verify(&PARAMS_VERIFIER, &Proof(pb), pi.iter().copied()).is_err());
    }
    let v = serde_json::json!({
        "phase": "ledger_9_1_0_1_verify",
        "ledger": "midnight-ledger rev a2b30128 (tag ledger-9.1.0.1), stock crates",
        "key_bytes_midnightvk": framed.len(),
        "proof_bytes": proof.0.len(),
        "public_inputs": n,
        "result": format!("{ok:?}"),
        "verify_seconds": secs,
        "changed_public_input_refused": pi_refused,
        "flipped_proof_byte_refused": byte_refused,
    });
    println!("E5JSON {v}");
    assert!(ok.is_ok() && pi_refused.iter().all(|x| *x) && byte_refused.iter().all(|x| *x));
}
