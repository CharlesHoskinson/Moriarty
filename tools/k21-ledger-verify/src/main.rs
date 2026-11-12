//! Verify a run-decider proof as the ledger 9 verifier does: `tagged_deserialize` the
//! `acceptRun.verifier` bytes into `midnight_transient_crypto::proofs::VerifierKey`,
//! then `VerifierKey::verify(&PARAMS_VERIFIER, ..)` (the static 2^14 verifier params).
//! usage: k21-ledger-verify <bundle out dir with key/>
use std::{fs, path::PathBuf, time::Instant};

use midnight_serialize::tagged_deserialize;
use midnight_transient_crypto::{
    curve::Fr,
    proofs::{PARAMS_VERIFIER, Proof, VERIFIER_MAX_DEGREE, VerifierKey},
};

fn fr(h: &str) -> Fr {
    let b = hex::decode(h.trim_start_matches("0x")).unwrap();
    Fr::from_le_bytes(&b).expect("canonical scalar")
}

fn main() {
    let dir = PathBuf::from(std::env::args().nth(1).expect("bundle dir"));
    let tagged = fs::read(dir.join("key/acceptRun.verifier")).unwrap();
    let proof = Proof(fs::read(dir.join("key/decider.proof")).unwrap());
    let public: Vec<String> =
        serde_json::from_slice(&fs::read(dir.join("key/decider.public.json")).unwrap()).unwrap();
    let pi: Vec<Fr> = public.iter().map(|h| fr(h)).collect();
    let key: VerifierKey = tagged_deserialize(&mut &tagged[..]).unwrap();
    let t = Instant::now();
    let ok = key.verify(&PARAMS_VERIFIER, &proof, pi.iter().copied());
    let secs = t.elapsed().as_secs_f64();
    let t2 = Instant::now();
    let ok2 = key.verify(&PARAMS_VERIFIER, &proof, pi.iter().copied());
    let secs2 = t2.elapsed().as_secs_f64();
    // Negative controls: every public lane changed by one; one proof byte flipped.
    let mut lanes_rejected = 0;
    for lane in 0..pi.len() {
        let mut bad = pi.clone();
        bad[lane] = Fr(bad[lane].0 + Fr::from(1u64).0);
        if key
            .verify(&PARAMS_VERIFIER, &proof, bad.into_iter())
            .is_err()
        {
            lanes_rejected += 1;
        }
    }
    let mut pb = proof.0.clone();
    let mid = pb.len() / 2;
    pb[mid] ^= 1;
    let flipped_rejected = key
        .verify(&PARAMS_VERIFIER, &Proof(pb), pi.iter().copied())
        .is_err();
    let v = serde_json::json!({
        "phase": "ledger9_verify_standalone",
        "function": "midnight_transient_crypto::proofs::VerifierKey::verify(&PARAMS_VERIFIER, &Proof, statement) after midnight_serialize::tagged_deserialize::<VerifierKey>",
        "ledger_checkout": "repos/midnightntwrk/midnight-ledger ledger-9 f15fb9d (path dependency)",
        "verifier_max_degree": VERIFIER_MAX_DEGREE,
        "proof_bytes": proof.0.len(),
        "public_inputs": pi.len(),
        "result": format!("{ok:?}"),
        "result_repeat": format!("{ok2:?}"),
        "verify_seconds": secs, "verify_seconds_repeat": secs2,
        "negative_lanes_rejected": lanes_rejected, "negative_lanes_total": pi.len(),
        "negative_flipped_byte_rejected": flipped_rejected,
    });
    println!("{v}");
    assert!(ok.is_ok() && lanes_rejected == pi.len() && flipped_rejected);
}
