//! Verify an E5 wrap proof as ledger 9.1.0.1 does: the key bytes in the `MidnightVK`
//! framing go through `VerifierKey`'s own deserializer (u64 length prefix, then
//! `MidnightVK::read`), then `VerifierKey::verify(&PARAMS_VERIFIER, ..)`. Negative controls:
//! one public input changed (first, middle, last; T10 also lane 10 the anchor, 20 and 34
//! `claim_pub`, 39 `outcome_pub`) and one proof byte flipped (3 places).
//! usage: sherlock-wrap-ledger-check <dir with wrap.vk, wrap.proof, wrap.public.json>
//!          [--against LABEL PUBLIC.json]...
//! Each `--against` verifies the proof of <dir> against another public-input vector (for
//! example the proof of program B against the call vector of program A's anchor) and requires
//! the ledger verifier to refuse it.
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
    let args: Vec<String> = std::env::args().collect();
    let dir = PathBuf::from(args.get(1).expect("dir"));
    let mut against = vec![];
    let mut i = 2;
    while i < args.len() {
        assert_eq!(args[i], "--against", "usage: DIR [--against LABEL PUBLIC.json]...");
        let v: Vec<String> = serde_json::from_slice(&fs::read(&args[i + 2]).unwrap()).unwrap();
        against.push((args[i + 1].clone(), v.iter().map(|h| fr(h)).collect::<Vec<Fr>>()));
        i += 3;
    }
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
    let mut named = serde_json::Map::new();
    for (lane, label) in [(0, "lane 0"), (10, "anchor (lane 10)"), (20, "claim_pub (lane 20)"), (34, "claim_pub (lane 34)"),
        (39, "outcome_pub (lane 39)"), (n / 2, "middle lane"), (n - 1, "last lane")] {
        let mut bad = pi.clone();
        bad[lane] = Fr(bad[lane].0 + Fr::from(1u64).0);
        let refused = key.verify(&PARAMS_VERIFIER, &proof, bad.into_iter()).is_err();
        pi_refused.push(refused);
        named.insert(format!("{label} + 1"), serde_json::json!(refused));
    }
    let mut cross = serde_json::Map::new();
    for (label, v) in &against {
        assert_eq!(v.len(), n, "{label}: public input count");
        let differs: Vec<usize> = (0..n).filter(|&j| v[j] != pi[j]).collect();
        let refused = key.verify(&PARAMS_VERIFIER, &proof, v.iter().copied()).is_err();
        pi_refused.push(refused);
        cross.insert(label.clone(), serde_json::json!({"refused": refused, "lanes_that_differ": differs}));
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
        "changed_lane_refused": named,
        "against": cross,
        "flipped_proof_byte_refused": byte_refused,
    });
    println!("E5JSON {v}");
    assert!(ok.is_ok() && pi_refused.iter().all(|x| *x) && byte_refused.iter().all(|x| *x));
}
