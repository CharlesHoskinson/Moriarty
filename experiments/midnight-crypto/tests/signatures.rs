use midnight_base_crypto::{ecdsa, hash::persistent_hash, schnorr};
use midnight_serialize::Serializable;
use moriarty_midnight_crypto::{canonical_message, verify, wallet_message};
use rand::rngs::OsRng;
use serde_json::{Value, json};
fn bytes<T: Serializable>(x: &T) -> Vec<u8> {
    let mut b = Vec::new();
    x.serialize(&mut b).unwrap();
    b
}
fn fixtures() -> Value {
    serde_json::from_str(include_str!("../fixtures/moriarty.json")).unwrap()
}
fn sign(scheme: &str, message: &[u8]) -> (Vec<u8>, Vec<u8>) {
    if scheme == "schnorr_bip340" {
        let sk = schnorr::SigningKey::sample(OsRng);
        (
            bytes(&sk.verifying_key()),
            bytes(&sk.sign(&mut OsRng, message)),
        )
    } else {
        let sk = ecdsa::SigningKey::sample(OsRng);
        (bytes(&sk.verifying_key()), bytes(&sk.sign(message)))
    }
}
const SCHEMES: [&str; 2] = ["schnorr_bip340", "ecdsa_secp256k1_sha256"];
fn mutated(v: &Value) -> Vec<Value> {
    match v {
        Value::Object(m) if !m.is_empty() => m
            .iter()
            .flat_map(|(k, x)| {
                mutated(x).into_iter().map(move |y| {
                    let mut out = v.clone();
                    out[k] = y;
                    out
                })
            })
            .collect(),
        Value::Array(a) if !a.is_empty() => a
            .iter()
            .enumerate()
            .flat_map(|(i, x)| {
                mutated(x).into_iter().map(move |y| {
                    let mut out = v.clone();
                    out[i] = y;
                    out
                })
            })
            .collect(),
        Value::String(s) => vec![json!(format!("{s}#altered"))],
        Value::Bool(b) => vec![json!(!b)],
        Value::Null => vec![json!("altered-null")],
        Value::Array(_) => vec![json!(["altered-empty-array"])],
        Value::Object(_) => vec![json!({"altered":"empty-object"})],
        _ => panic!("fixtures require string exact integers"),
    }
}
#[test]
fn equivalent_object_order_has_identical_framing() {
    assert_eq!(
        canonical_message(r#"{"v":"1","amount":"100","account":"Buyer"}"#).unwrap(),
        canonical_message(r#"{"account":"Buyer","amount":"100","v":"1"}"#).unwrap()
    );
    assert_ne!(
        canonical_message(r#"{"a":"ab","b":"c"}"#).unwrap(),
        canonical_message(r#"{"a":"a","b":"bc"}"#).unwrap()
    );
    assert_ne!(
        canonical_message(r#"{"a":["a","b"]}"#).unwrap(),
        canonical_message(r#"{"a":["b","a"]}"#).unwrap()
    );
    assert_ne!(
        canonical_message(r#"{"a":null}"#).unwrap(),
        canonical_message("{}").unwrap()
    );
}
#[test]
fn malformed_or_ambiguous_input_is_not_a_statement() {
    for s in [
        r#"{"a":"1","\u0061":"2"}"#,
        r#"{"a":1}"#,
        r#"{"a":1.0}"#,
        r#"{"a":"\ud800"}"#,
        r#"{"a":"1"} {}"#,
        "[]",
        "{",
        "\u{feff}{}",
    ] {
        assert!(canonical_message(s).is_err(), "{s}");
    }
    assert!(canonical_message(&format!("{{\"x\":\"{}\"}}", "x".repeat(65536))).is_err());
    assert!(
        canonical_message(&format!(
            "{{\"x\":{}null{}}}",
            "[".repeat(33),
            "]".repeat(33)
        ))
        .is_err()
    );
    assert!(canonical_message(&format!("{{\"x\":[{}]}}", vec!["null"; 4096].join(","))).is_err());
}
#[test]
fn cross_language_frames_hashes_and_all_fixture_leaf_mutations() {
    let f = fixtures();
    let mut tested = 0;
    for vector in f["codec_vectors"].as_array().unwrap() {
        let b = canonical_message(&vector["statement"].to_string()).unwrap();
        assert_eq!(
            hex::encode(&b),
            vector["golden_frame_hex"].as_str().unwrap()
        );
        assert_eq!(
            hex::encode(persistent_hash(&b).0),
            vector["golden_frame_sha256"].as_str().unwrap()
        );
    }
    for fixture in f["fixtures"].as_array().unwrap() {
        let statement = &fixture["statement"];
        let message = canonical_message(&statement.to_string()).unwrap();
        assert_eq!(
            mutated(statement).len(),
            fixture["mutation_leaf_paths"].as_array().unwrap().len()
        );
        assert_eq!(
            statement["candidate"]["effects"],
            fixture["expected_effects"]
        );
        assert_eq!(
            statement["candidate"]["candidatePost"],
            fixture["expected_post"]
        );
        assert_eq!(
            hex::encode(&message),
            fixture["golden_frame_hex"].as_str().unwrap()
        );
        assert_eq!(
            hex::encode(persistent_hash(&message).0),
            fixture["golden_frame_sha256"].as_str().unwrap()
        );
        for scheme in SCHEMES {
            let (pk, sig) = sign(scheme, &message);
            assert!(verify(scheme, &message, &pk, &sig).unwrap());
            for m in mutated(statement) {
                let b = canonical_message(&m.to_string()).unwrap();
                assert_ne!(b, message);
                assert!(!verify(scheme, &b, &pk, &sig).unwrap());
                tested += 1;
            }
            let owner = canonical_message(&fixture["intent_statement"].to_string()).unwrap();
            let (k, s) = sign(scheme, &owner);
            assert!(verify(scheme, &owner, &k, &s).unwrap());
            assert!(!verify(scheme, &message, &k, &s).unwrap());
        }
    }
    assert!(tested > 1000);
    println!("actual bound fixture leaf mutations rejected: {tested}");
}
#[test]
fn exact_encodings_wrong_key_changed_signature_and_message_reject() {
    let m = canonical_message(r#"{"intent":"pay","amount":"1000"}"#).unwrap();
    for scheme in SCHEMES {
        let (pk, sig) = sign(scheme, &m);
        let (other, _) = sign(scheme, &m);
        assert!(!verify(scheme, &m, &other, &sig).unwrap());
        let mut changed = m.clone();
        changed.push(0);
        assert!(!verify(scheme, &changed, &pk, &sig).unwrap());
        for len in [0, 1, 31, 32, 63, 65, 128] {
            assert!(verify(scheme, &m, &pk, &vec![0; len]).is_err());
        }
        let mut altered = sig.clone();
        altered[40] ^= 1;
        assert!(!verify(scheme, &m, &pk, &altered).unwrap_or(false));
        assert!(!verify(scheme, &m, &pk, &[0; 64]).unwrap_or(false));
        assert!(verify(scheme, &m, &[], &sig).is_err());
        let mut long = pk.clone();
        long.push(0);
        assert!(verify(scheme, &m, &long, &sig).is_err());
        assert!(verify("unknown", &m, &pk, &sig).is_err());
        let swapped = if scheme == SCHEMES[0] {
            SCHEMES[1]
        } else {
            SCHEMES[0]
        };
        assert!(!verify(swapped, &m, &pk, &sig).unwrap_or(false));
    }
}
#[test]
fn wallet_prefix_separates_raw_signatures_and_counts_utf8_bytes() {
    let m = canonical_message(r#"{"recipient":"é😀"}"#).unwrap();
    let w = wallet_message(&m);
    assert!(w.starts_with(format!("midnight_signed_message:{}:", m.len()).as_bytes()));
    for scheme in SCHEMES {
        let (pk, s) = sign(scheme, &w);
        assert!(verify(scheme, &w, &pk, &s).unwrap());
        assert!(!verify(scheme, &m, &pk, &s).unwrap());
        let bad = [b"midnight_signed_message:1:".as_slice(), m.as_slice()].concat();
        assert!(!verify(scheme, &bad, &pk, &s).unwrap());
    }
}
#[test]
fn freshly_signed_financially_invalid_claims_still_have_rejected_core_observations() {
    let fs = fixtures();
    let mut tested = 0;
    for f in fs["fixtures"].as_array().unwrap() {
        for x in f["negative_candidates"].as_array().unwrap() {
            assert_eq!(x["core_result"]["status"], "Rejected");
            assert!(x["core_result"]["publishedPost"].is_null());
            assert!(x["core_result"]["publishedEffects"].is_null());
            let m = canonical_message(&x["statement"].to_string()).unwrap();
            for scheme in SCHEMES {
                let (k, s) = sign(scheme, &m);
                assert!(verify(scheme, &m, &k, &s).unwrap());
                tested += 1;
            }
        }
    }
    assert_eq!(tested, 50);
}
