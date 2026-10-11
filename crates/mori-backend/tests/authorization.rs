//! Ordinary host/reference checks only; no native proof qualification.
use midnight_transient_crypto::curve::Fr;
use mori_backend::{authorization::UnsignedPayload, schnorr};

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!(
        "../../../examples/core/native-owner-intent/golden-vector.json"
    ))
    .unwrap()
}
fn unhex(s: &str) -> Vec<u8> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
        .collect()
}
#[test]
fn independent_payload_bytes_and_hash() {
    let f = fixture();
    let payload =
        UnsignedPayload::from_json(f["payload_canonical_utf8"].as_str().unwrap()).unwrap();
    assert_eq!(
        mori_core::intent::canonical(&payload).unwrap(),
        f["payload_canonical_utf8"].as_str().unwrap().as_bytes()
    );
    assert_eq!(
        mori_core::intent::hex(&payload.hash().unwrap()),
        f["payload_hash"]
    );
    assert!(UnsignedPayload::from_json("{\"profile\":null}").is_err());
}
#[test]
fn independent_signature_accepts_and_one_bit_mutation_refuses() {
    let f = fixture();
    let message = Fr::from_le_bytes(&unhex(f["message_hex_le32"].as_str().unwrap())).unwrap();
    let key = f["payload"]["public_key_hex"].as_str().unwrap();
    let signature = f["signed_wrapper"]["signature_hex"].as_str().unwrap();
    schnorr::verify_attachment(key, signature, message).unwrap();
    let mut changed = unhex(signature);
    changed[64] ^= 1;
    assert!(schnorr::verify_attachment(key, &mori_core::intent::hex(&changed), message).is_err());
}

use midnight_curves::pairing::group::{Group, ff::Field};
use midnight_curves::{Fq, Fr as Scalar, JubjubAffine, JubjubExtended, JubjubSubgroup};
use mori_backend::authorization::{ExpectedAuthority, SignedIntent};

// Independent affine Edwards reproduction by root, public QA scalars 13/19;
// SHA256 of original runtime fixture cf6cc127fc4d744a7ed5af16fa08045a264e10aa11f8383071cc656c9d5e64bd.
const SECOND_KEY: &str = "0677506737143823730512c5525e0b30acb75906fa7d53f494ba689b78620133d4cdab997110c2f1e02bb16ebef816c9d0a6025386825581a688b9bfa326360a";
const SECOND_SIG: &str = "2593c2eb72c63e02a77a6b75a5c163aea6587489208d8071a1da7c7b21c52023c05aa50c2dd954d876ad230a843aea2331df5befe0b9ef40f2954ea4a223ed02029d3e336ad33ba4360415ad36fca3a2a68f15ea0ab5d91812abe52c716e8a04";
fn encode_xy(p: JubjubExtended) -> String {
    let a = JubjubAffine::from(p);
    let mut bytes = Fr(a.get_u()).as_le_bytes();
    bytes.resize(32, 0);
    let mut y = Fr(a.get_v()).as_le_bytes();
    y.resize(32, 0);
    bytes.extend(y);
    mori_core::intent::hex(&bytes)
}
fn point_hex(x: Fq, y: Fq) -> String {
    encode_xy(JubjubExtended::from(JubjubAffine::from_raw_unchecked(x, y)))
}
fn payload_mutation(field: &str, value: serde_json::Value) -> bool {
    let mut p = fixture()["payload"].clone();
    p[field] = value;
    UnsignedPayload::from_json(&p.to_string()).is_err()
}
#[test]
fn second_independent_primitive_vector_and_binding_mutations() {
    schnorr::verify_attachment(SECOND_KEY, SECOND_SIG, Fr::from(1)).unwrap();
    assert!(schnorr::verify_attachment(SECOND_KEY, SECOND_SIG, Fr::from(2)).is_err());
    let f = fixture();
    assert!(
        schnorr::verify_attachment(
            f["payload"]["public_key_hex"].as_str().unwrap(),
            SECOND_SIG,
            Fr::from(1)
        )
        .is_err()
    );
    let msg = Fr::from_le_bytes(&unhex(f["message_hex_le32"].as_str().unwrap())).unwrap();
    assert!(schnorr::verify_attachment(SECOND_KEY, SECOND_SIG, msg).is_err());
}
#[test]
fn malformed_points_are_never_cleared_into_subgroup() {
    let identity = point_hex(Fq::ZERO, Fq::ONE);
    let torsion = JubjubExtended::from(JubjubAffine::from_raw_unchecked(Fq::ZERO, -Fq::ONE));
    let mixed = JubjubExtended::from(JubjubSubgroup::generator()) + torsion;
    let off_curve = "0".repeat(128);
    for key in [identity, encode_xy(torsion), encode_xy(mixed), off_curve] {
        assert!(schnorr::decode_key(&key).is_err(), "key {key}");
        let sig = format!("{}{}", key, &SECOND_SIG[128..]);
        assert!(schnorr::decode_signature(&sig).is_err(), "R {key}");
    }
}
#[test]
fn canonical_scalar_range_is_distinct_from_signature_acceptance() {
    let r = [
        183, 44, 247, 214, 94, 14, 151, 208, 130, 16, 200, 204, 147, 32, 104, 166, 0, 59, 52, 1, 1,
        59, 103, 6, 169, 175, 51, 101, 234, 180, 125, 14,
    ];
    let mut rm1 = r;
    rm1[0] -= 1;
    for bytes in [
        [0; 32],
        {
            let mut b = [0; 32];
            b[0] = 1;
            b
        },
        rm1,
    ] {
        let sig = format!("{}{}", &SECOND_SIG[..128], mori_core::intent::hex(&bytes));
        assert!(schnorr::decode_signature(&sig).is_ok());
        assert!(schnorr::verify_attachment(SECOND_KEY, &sig, Fr::from(1)).is_err());
    }
    let mut rp1 = r;
    rp1[0] += 1;
    let mut sr = r;
    let s = unhex(&SECOND_SIG[128..]);
    let mut carry = 0u16;
    for i in 0..32 {
        let sum = sr[i] as u16 + s[i] as u16 + carry;
        sr[i] = sum as u8;
        carry = sum >> 8;
    }
    for bytes in [r, rp1, sr] {
        let sig = format!("{}{}", &SECOND_SIG[..128], mori_core::intent::hex(&bytes));
        assert!(schnorr::decode_signature(&sig).is_err());
    }
}
#[test]
fn canonical_native_and_hex_boundaries() {
    // BLS12-381 scalar modulus, exact little-endian p.
    let p = unhex("01000000fffffffffe5bfeff02a4bd5305d8a10908d83933487d9d2953a7ed73");
    assert!(schnorr::decode_native_hex(&mori_core::intent::hex(&p)).is_err());
    let mut pm1 = p.clone();
    pm1[0] -= 1;
    assert!(schnorr::decode_native_hex(&mori_core::intent::hex(&pm1)).is_ok());
    assert!(schnorr::decode_native_hex(&"0".repeat(64)).is_ok());
    for text in [
        "".to_owned(),
        "0".repeat(63),
        "0".repeat(65),
        "G".repeat(64),
        format!("0x{}", "0".repeat(64)),
        SECOND_KEY.to_uppercase(),
    ] {
        assert!(schnorr::decode_key(&text).is_err());
        assert!(schnorr::decode_signature(&text).is_err());
        assert!(schnorr::decode_native_hex(&text).is_err());
    }
    let malformed_key = format!("{}{}", mori_core::intent::hex(&p), &SECOND_KEY[64..]);
    assert!(schnorr::decode_key(&malformed_key).is_err());
    let malformed_r = format!("{}{}", mori_core::intent::hex(&p), &SECOND_SIG[64..]);
    assert!(schnorr::decode_signature(&malformed_r).is_err());
}
#[test]
fn wrong_challenge_tag_and_order_refuse_real_equations() {
    // Fixed public QA key/nonce only. These adversarial signatures satisfy
    // their deliberately wrong challenge equations, so decoding alone is insufficient.
    let pk = JubjubSubgroup::generator() * Scalar::from(13);
    let r = JubjubSubgroup::generator() * Scalar::from(19);
    let pka = JubjubAffine::from(JubjubExtended::from(pk));
    let ra = JubjubAffine::from(JubjubExtended::from(r));
    let pre = [
        Fr::from(0x46454e53),
        Fr(ra.get_u()),
        Fr(ra.get_v()),
        Fr(pka.get_u()),
        Fr(pka.get_v()),
        Fr::from(1),
    ];
    let mut tag = pre;
    tag[0] = Fr::from(0x46454e52);
    let mut order = pre;
    order.swap(1, 3);
    order.swap(2, 4);
    for wrong in [tag, order] {
        let h = midnight_transient_crypto::hash::transient_hash(&wrong);
        let mut wide = [0; 64];
        let bytes = h.as_le_bytes();
        wide[..bytes.len()].copy_from_slice(&bytes);
        let e = Scalar::from_bytes_wide(&wide);
        let response = Scalar::from(19) + e * Scalar::from(13);
        let sig = format!(
            "{}{}",
            encode_xy(JubjubExtended::from(r)),
            mori_core::intent::hex(&response.to_bytes())
        );
        assert!(schnorr::decode_signature(&sig).is_ok());
        assert!(schnorr::verify_attachment(SECOND_KEY, &sig, Fr::from(1)).is_err());
    }
}
#[test]
fn required_fields_null_unknown_duplicate_trailing_and_work_bounds() {
    let f = fixture();
    let original = f["payload"].as_object().unwrap();
    for field in original.keys() {
        let mut missing = original.clone();
        missing.remove(field);
        assert!(
            UnsignedPayload::from_json(&serde_json::Value::Object(missing).to_string()).is_err(),
            "missing {field}"
        );
        assert!(
            payload_mutation(field, serde_json::Value::Null),
            "null {field}"
        );
    }
    assert!(payload_mutation("extra", serde_json::json!(1)));
    let text = f["payload_canonical_utf8"].as_str().unwrap();
    assert!(UnsignedPayload::from_json(&format!("{text} {{}}")).is_err());
    assert!(
        UnsignedPayload::from_json(&text.replacen(
            "{",
            "{\"profile\":\"moriarty-jubjub-owner-intent/1\",",
            1
        ))
        .is_err()
    );
    assert!(
        UnsignedPayload::from_json(&text.replacen(
            "{",
            "{\"\\u0070rofile\":\"moriarty-jubjub-owner-intent/1\",",
            1
        ))
        .is_err()
    );
    assert!(
        UnsignedPayload::from_json(&text.replacen("\"scale\":0", "\"scale\":0,\"scale\":0", 1))
            .is_err()
    );
    assert!(UnsignedPayload::from_json(&format!("{}0{}", "[".repeat(34), "]".repeat(34))).is_err());
    assert!(UnsignedPayload::from_json(&format!("[{}0]", "0,".repeat(65536))).is_err());
    assert!(UnsignedPayload::from_json(&" ".repeat(1_048_577)).is_err());
    assert!(UnsignedPayload::from_json(&text.replace("\"alice\"", "\"\\ud800\"")).is_err());
}
#[test]
fn exact_profiles_digests_labels_and_decimal_types() {
    for (field, value) in [
        ("profile", "moriarty-owner-intent/2"),
        ("semantics", "moriarty-core/2"),
        ("relation", "moriarty-zkir-relation/2"),
        ("abi", "moriarty-public-abi/2"),
        ("source_hash", "00"),
        (
            "target_hash",
            "A000000000000000000000000000000000000000000000000000000000000000",
        ),
    ] {
        assert!(payload_mutation(field, serde_json::json!(value)), "{field}");
    }
    for field in [
        "network",
        "anchor",
        "program_address",
        "action",
        "owner",
        "beneficiary",
        "predecessor",
    ] {
        for value in [
            "".to_owned(),
            "a".repeat(1025),
            "é".repeat(513),
            "a\0".to_owned(),
            "a\u{1f}".to_owned(),
            "a\u{7f}".to_owned(),
            "a\u{85}".to_owned(),
            "a\u{9f}".to_owned(),
        ] {
            assert!(
                payload_mutation(field, serde_json::json!(value)),
                "label {field}"
            );
        }
    }
    assert!(!payload_mutation(
        "predecessor",
        serde_json::json!("é".repeat(512))
    ));
    for field in ["nonce", "not_before", "not_after"] {
        for value in [
            serde_json::json!(1),
            serde_json::json!(1.0),
            serde_json::json!("01"),
            serde_json::json!("-1"),
            serde_json::json!("+1"),
            serde_json::json!("1e1"),
            serde_json::json!(" 1"),
            serde_json::json!("340282366920938463463374607431768211456"),
            serde_json::json!("1".repeat(40)),
        ] {
            assert!(payload_mutation(field, value), "decimal {field}");
        }
    }
    assert!(payload_mutation("not_before", serde_json::json!("21")));
    let mut p = fixture()["payload"].clone();
    p["limits"][0]["asset"]["scale"] = serde_json::json!("0");
    assert!(UnsignedPayload::from_json(&p.to_string()).is_err());
    p["limits"][0]["asset"]["scale"] = serde_json::json!(0.0);
    assert!(UnsignedPayload::from_json(&p.to_string()).is_err());
    p["limits"][0]["asset"]["scale"] = serde_json::json!(0);
    p["limits"][0]["gross_cap"] = serde_json::json!("01");
    assert!(UnsignedPayload::from_json(&p.to_string()).is_err());
}
#[test]
fn full_asset_identity_sorted_exact_set_and_ceiling() {
    let mut p = fixture()["payload"].clone();
    let a = p["limits"][0].clone();
    let mut b = a.clone();
    b["asset"]["scale"] = serde_json::json!(1);
    p["limits"] = serde_json::json!([a, b]);
    let parsed = UnsignedPayload::from_json(&p.to_string()).unwrap();
    let assets = parsed
        .limits
        .iter()
        .map(|l| l.asset.clone())
        .collect::<Vec<_>>();
    parsed.validate_assets(&assets).unwrap();
    assert!(parsed.validate_assets(&assets[..1]).is_err());
    let mut extra = assets.clone();
    let mut new = assets[0].clone();
    new.scale = 2;
    extra.push(new);
    assert!(parsed.validate_assets(&extra).is_err());
    assert!(
        parsed
            .validate_assets(&[assets[0].clone(), assets[0].clone()])
            .is_err()
    );
    let mut reversed = p.clone();
    reversed["limits"].as_array_mut().unwrap().reverse();
    assert!(UnsignedPayload::from_json(&reversed.to_string()).is_err());
    p["limits"][1] = p["limits"][0].clone();
    assert!(UnsignedPayload::from_json(&p.to_string()).is_err());
    p["limits"] = serde_json::json!([]);
    assert!(
        UnsignedPayload::from_json(&p.to_string())
            .unwrap()
            .validate_assets(&assets)
            .is_err()
    );
    let mut limits = Vec::new();
    for i in 0..17 {
        let mut l = fixture()["payload"]["limits"][0].clone();
        l["asset"]["scale"] = serde_json::json!(i);
        limits.push(l);
    }
    p["limits"] = serde_json::json!(&limits[..16]);
    assert!(UnsignedPayload::from_json(&p.to_string()).is_ok());
    p["limits"] = serde_json::json!(limits);
    assert!(UnsignedPayload::from_json(&p.to_string()).is_err());
    p = fixture()["payload"].clone();
    p["limits"][0]["beneficiary"] = serde_json::json!("mallory");
    assert!(UnsignedPayload::from_json(&p.to_string()).is_err());
    p = fixture()["payload"].clone();
    p["limits"][0]["asset"]["network"] = serde_json::json!("other");
    assert!(UnsignedPayload::from_json(&p.to_string()).is_ok());
    let changed = UnsignedPayload::from_json(&p.to_string()).unwrap();
    let original =
        UnsignedPayload::from_json(fixture()["payload_canonical_utf8"].as_str().unwrap()).unwrap();
    assert!(
        changed
            .validate_assets(&[original.limits[0].asset.clone()])
            .is_err()
    );
}
fn authority_value() -> serde_json::Value {
    let mut p = fixture()["payload"].clone();
    let m = p.as_object_mut().unwrap();
    for field in [
        "source_hash",
        "program_hash",
        "semantics",
        "relation",
        "relation_hash",
        "abi",
        "action",
        "scenario_hash",
    ] {
        m.remove(field);
    }
    p
}
#[test]
fn expected_authority_and_signed_wrapper_have_exact_schema() {
    let a = authority_value();
    ExpectedAuthority::from_json(&a.to_string()).unwrap();
    for field in a.as_object().unwrap().keys() {
        let mut v = a.clone();
        v.as_object_mut().unwrap().remove(field);
        assert!(ExpectedAuthority::from_json(&v.to_string()).is_err());
        v = a.clone();
        v[field] = serde_json::Value::Null;
        assert!(ExpectedAuthority::from_json(&v.to_string()).is_err());
    }
    let mut v = a.clone();
    v["signature_hex"] = serde_json::json!(SECOND_SIG);
    assert!(ExpectedAuthority::from_json(&v.to_string()).is_err());
    let wrapper = fixture()["signed_wrapper"].clone();
    SignedIntent::from_json(&wrapper.to_string()).unwrap();
    for field in ["payload", "signature_hex"] {
        let mut v = wrapper.clone();
        v.as_object_mut().unwrap().remove(field);
        assert!(SignedIntent::from_json(&v.to_string()).is_err());
        v = wrapper.clone();
        v[field] = serde_json::Value::Null;
        assert!(SignedIntent::from_json(&v.to_string()).is_err());
    }
    let mut v = wrapper.clone();
    v["message_hex"] = serde_json::json!("0".repeat(64));
    assert!(SignedIntent::from_json(&v.to_string()).is_err());
    assert!(SignedIntent::from_json(&format!("{} {{}}", wrapper)).is_err());
}
#[test]
fn predecessor_hash_keeps_domain_array_framing() {
    let p =
        UnsignedPayload::from_json(fixture()["payload_canonical_utf8"].as_str().unwrap()).unwrap();
    assert_eq!(
        mori_core::intent::hex(&p.predecessor_hash().unwrap()),
        fixture()["predecessor_hash"]
    );
}
#[test]
fn canonical_wrapper_limit_applies_separately_from_payload() {
    // Legal labels in 16 assets allow a near-limit valid payload. A quote
    // remains legal but increases its escaped canonical size by one.
    let mut p = fixture()["payload"].clone();
    for field in [
        "action",
        "anchor",
        "network",
        "program_address",
        "owner",
        "beneficiary",
        "predecessor",
    ] {
        p[field] = serde_json::json!("x".repeat(1024));
    }
    let mut limits = Vec::new();
    for i in 0..16 {
        let mut l = fixture()["payload"]["limits"][0].clone();
        l["asset"]["network"] = p["network"].clone();
        l["asset"]["issuer"] = serde_json::json!("\"".repeat(1024));
        l["asset"]["reference"] = serde_json::json!("\"".repeat(1024));
        l["asset"]["scale"] = serde_json::json!(i);
        l["beneficiary"] = p["beneficiary"].clone();
        limits.push(l);
    }
    p["limits"] = serde_json::json!(limits);
    // Tune legal label lengths to exact canonical payload 65,536-223. The
    // signature/wrapper framing adds 223 bytes, so wrapper is exactly 65,536.
    for i in 0..16 {
        p["limits"][i]["asset"]["issuer"] = serde_json::json!("i");
        p["limits"][i]["asset"]["reference"] = serde_json::json!("r");
    }
    let target = 65_536 - 223;
    for i in 0..16 {
        for field in ["issuer", "reference"] {
            let len = mori_core::intent::canonical(&p).unwrap().len();
            if len < target {
                let addition = (target - len).min(1023);
                p["limits"][i]["asset"][field] = serde_json::json!("x".repeat(addition + 1));
            }
        }
    }
    // Full identities must remain sorted after tuning issuer/reference lengths.
    p["limits"].as_array_mut().unwrap().sort_by_key(|v| {
        (
            v["asset"]["issuer"].as_str().unwrap().to_owned(),
            v["asset"]["reference"].as_str().unwrap().to_owned(),
            v["asset"]["scale"].as_u64().unwrap(),
        )
    });
    assert_eq!(mori_core::intent::canonical(&p).unwrap().len(), target);
    let mut wrapper = serde_json::json!({"payload":p,"signature_hex":SECOND_SIG});
    assert_eq!(
        mori_core::intent::canonical(&wrapper).unwrap().len(),
        65_536
    );
    assert!(UnsignedPayload::from_json(&wrapper["payload"].to_string()).is_ok());
    assert!(SignedIntent::from_json(&wrapper.to_string()).is_ok());
    // Replace one ASCII byte with a quote, increasing canonical size by one
    // while staying below the label byte bound and payload size ceiling.
    let limits = wrapper["payload"]["limits"].as_array_mut().unwrap();
    let l = limits
        .iter_mut()
        .find(|l| l["asset"]["reference"].as_str().unwrap().len() > 1)
        .unwrap();
    let old = l["asset"]["reference"].as_str().unwrap();
    l["asset"]["reference"] = serde_json::json!(format!("{}\"", &old[..old.len() - 1]));
    limits.sort_by_key(|v| {
        (
            v["asset"]["issuer"].as_str().unwrap().to_owned(),
            v["asset"]["reference"].as_str().unwrap().to_owned(),
            v["asset"]["scale"].as_u64().unwrap(),
        )
    });
    assert_eq!(
        mori_core::intent::canonical(&wrapper).unwrap().len(),
        65_537
    );
    assert!(UnsignedPayload::from_json(&wrapper["payload"].to_string()).is_ok());
    assert!(SignedIntent::from_json(&wrapper.to_string()).is_err());
}

#[test]
fn another_independent_valid_attachment_keeps_payload_commitment() {
    // Root's independent affine reproduction with public QA nonce23;
    // immutable fixture SHA fefeaab64704a73e25d73160ae637282dc3eebfcb313ddfecb93da06394fcb00.
    const ALT: &str = "fec0b07c971073c462c88d722fb5fe8df764311114dbcc79e46263b74a863850e92947318a0258f88a6869c98a7e96733662780d4936401c0e732a4c87fb651a98b9a4ecb5630fefa96bd5f061388d910c95dabfa7f2c089c2d3651ca422e900";
    let f = fixture();
    let key = f["payload"]["public_key_hex"].as_str().unwrap();
    let msg = Fr::from_le_bytes(&unhex(f["message_hex_le32"].as_str().unwrap())).unwrap();
    schnorr::verify_attachment(key, ALT, msg).unwrap();
    let mut wrapper = f["signed_wrapper"].clone();
    wrapper["signature_hex"] = serde_json::json!(ALT);
    let parsed = SignedIntent::from_json(&wrapper.to_string()).unwrap();
    assert_eq!(
        mori_core::intent::hex(&parsed.payload.hash().unwrap()),
        f["payload_hash"]
    );
}
#[test]
fn descriptor_types_required_nulls_and_strict_policy() {
    use mori_backend::authorization::{AuthorizationBinding, AuthorizationType, SignedPolicy};
    let binding = serde_json::json!({"position":30,"role":"authorization","name":"public_key.x","ty":"Native","constant":null,"asset":null});
    let decoded: AuthorizationBinding = serde_json::from_value(binding.clone()).unwrap();
    assert_eq!(decoded.ty, AuthorizationType::Native);
    for field in binding.as_object().unwrap().keys() {
        let mut b = binding.clone();
        b.as_object_mut().unwrap().remove(field);
        assert!(
            serde_json::from_value::<AuthorizationBinding>(b).is_err(),
            "missing {field}"
        );
    }
    for (field, value) in [
        ("position", serde_json::json!("30")),
        ("position", serde_json::json!(30.0)),
        ("position", serde_json::json!(4294967296u64)),
        ("role", serde_json::json!("public")),
        ("ty", serde_json::json!("Bool")),
        ("constant", serde_json::json!("0")),
        ("extra", serde_json::json!(1)),
    ] {
        let mut b = binding.clone();
        b[field] = value;
        assert!(
            serde_json::from_value::<AuthorizationBinding>(b).is_err(),
            "invalid {field}"
        );
    }
    let policy =
        serde_json::json!({"owner":"alice","beneficiary":"bob","program_address":"local/contract"});
    serde_json::from_value::<SignedPolicy>(policy.clone()).unwrap();
    for field in policy.as_object().unwrap().keys() {
        let mut p = policy.clone();
        p.as_object_mut().unwrap().remove(field);
        assert!(serde_json::from_value::<SignedPolicy>(p).is_err());
        p = policy.clone();
        p[field] = serde_json::Value::Null;
        assert!(serde_json::from_value::<SignedPolicy>(p).is_err());
        p = policy.clone();
        p[field] = serde_json::json!("\u{85}");
        assert!(serde_json::from_value::<SignedPolicy>(p).is_err());
    }
    let mut extra = policy;
    extra["public_key_hex"] = serde_json::json!(SECOND_KEY);
    assert!(serde_json::from_value::<SignedPolicy>(extra).is_err());
}
/// Canonical decimal (no sign, no leading zero) to a field integer below p.
/// Exact 256-bit accumulation; values at or above p refuse, never reduce.
fn canonical_decimal_field(text: &str) -> Fr {
    assert!(!text.is_empty() && text.bytes().all(|b| b.is_ascii_digit()));
    assert!(
        text == "0" || !text.starts_with('0'),
        "non-canonical {text}"
    );
    let mut le = [0u8; 32];
    for digit in text.bytes() {
        let mut carry = u16::from(digit - b'0');
        for byte in &mut le {
            let product = u16::from(*byte) * 10 + carry;
            *byte = product as u8;
            carry = product >> 8;
        }
        assert_eq!(carry, 0, "decimal exceeds 256 bits");
    }
    Fr::from_le_bytes(&le).expect("decimal below p")
}
#[test]
fn golden_decimal_metadata_matches_canonical_field_bytes() {
    let f = fixture();
    let message = canonical_decimal_field(f["message_decimal"].as_str().unwrap());
    assert_eq!(schnorr::native_hex(message), f["message_hex_le32"]);
    let key = f["payload"]["public_key_hex"].as_str().unwrap();
    let bindings = f["authorization_bindings"].as_array().unwrap();
    let value = |name: &str| {
        let entry = bindings.iter().find(|b| b["name"] == name).unwrap();
        schnorr::native_hex(canonical_decimal_field(entry["value"].as_str().unwrap()))
    };
    assert_eq!(value("public_key.x"), key[..64]);
    assert_eq!(value("public_key.y"), key[64..]);
    assert_eq!(value("message"), f["message_hex_le32"]);
    // p itself is a canonical decimal but not a field element.
    let p = "52435875175126190479447740508185965837690552500527637822603658699938581184513";
    assert!(std::panic::catch_unwind(|| canonical_decimal_field(p)).is_err());
}
fn prepared_value() -> serde_json::Value {
    let f = fixture();
    let program: mori_core::Program = serde_json::from_value(f["program"].clone()).unwrap();
    let scenario: mori_core::Scenario = serde_json::from_value(f["scenario"].clone()).unwrap();
    let review = mori_core::simulate(&program, "repay", &scenario).unwrap();
    let financial = f["financial_bindings"]
        .as_array()
        .unwrap()
        .iter()
        .map(|entry| {
            let mut binding = entry.clone();
            let position = binding.as_object_mut().unwrap().remove("position").unwrap();
            let value = binding.as_object_mut().unwrap().remove("value").unwrap();
            let core_value = if binding["ty"] == "Bool" {
                serde_json::json!({"type":"Bool","value":value=="1"})
            } else {
                serde_json::json!({"type":"UInt","value":value})
            };
            serde_json::json!({"position":position,"binding":binding,"value":core_value})
        })
        .collect::<Vec<_>>();
    let authorization = f["authorization_bindings"]
        .as_array()
        .unwrap()
        .iter()
        .map(|entry| {
            let mut binding = entry.clone();
            let position = binding["position"].clone();
            let value = binding.as_object_mut().unwrap().remove("value").unwrap();
            // Decimal QA metadata converted only for this report-format test.
            let native = schnorr::native_hex(canonical_decimal_field(value.as_str().unwrap()));
            serde_json::json!({"position":position,"binding":binding,"value":native})
        })
        .collect::<Vec<_>>();
    serde_json::json!({"profile":"moriarty-jubjub-prepared-intent/1","payload":f["payload"],"payload_hash":f["payload_hash"],"context_hash":f["context_hash"],"message_hex":f["message_hex_le32"],"financial_projection":financial,"authorization_projection":authorization,"review":review})
}
#[test]
fn prepared_report_exact_schema_preserves_full_reference_review() {
    use mori_backend::authorization::PreparedIntent;
    let v = prepared_value();
    let parsed = PreparedIntent::from_json(&v.to_string()).unwrap();
    assert_eq!(serde_json::to_value(&parsed).unwrap(), v);
    for field in v.as_object().unwrap().keys() {
        let mut changed = v.clone();
        changed.as_object_mut().unwrap().remove(field);
        assert!(
            PreparedIntent::from_json(&changed.to_string()).is_err(),
            "missing {field}"
        );
        changed = v.clone();
        changed[field] = serde_json::Value::Null;
        assert!(
            PreparedIntent::from_json(&changed.to_string()).is_err(),
            "null {field}"
        );
    }
    for (field, value) in [
        ("profile", serde_json::json!("moriarty-owner-intent/2")),
        ("payload_hash", serde_json::json!("0".repeat(64))),
        ("context_hash", serde_json::json!("0".repeat(64))),
        ("message_hex", serde_json::json!("0")),
        ("signature_hex", serde_json::json!(SECOND_SIG)),
    ] {
        let mut changed = v.clone();
        changed[field] = value;
        assert!(PreparedIntent::from_json(&changed.to_string()).is_err());
    }
    let mut changed = v.clone();
    changed["authorization_projection"][0]["value"] = serde_json::json!("0");
    assert!(PreparedIntent::from_json(&changed.to_string()).is_err());
    for field in ["constant", "asset"] {
        changed = v.clone();
        changed["financial_projection"][0]["binding"]
            .as_object_mut()
            .unwrap()
            .remove(field);
        assert!(PreparedIntent::from_json(&changed.to_string()).is_err());
    }
    changed = v.clone();
    changed["financial_projection"][0]["extra"] = serde_json::json!(1);
    assert!(PreparedIntent::from_json(&changed.to_string()).is_err());
    let mut huge = parsed;
    huge.review.action = "x".repeat(1_048_576);
    assert!(huge.validate().is_err());
}
#[test]
fn direct_schema_decoding_and_manual_values_keep_semantic_validation() {
    let mut p = fixture()["payload"].clone();
    p["not_before"] = serde_json::json!("21");
    assert!(serde_json::from_value::<UnsignedPayload>(p).is_err());
    let mut p =
        UnsignedPayload::from_json(fixture()["payload_canonical_utf8"].as_str().unwrap()).unwrap();
    p.limits.reverse();
    p.owner = "\u{7f}".to_owned();
    assert!(p.validate().is_err());
    assert!(p.hash().is_err());
    let digest =
        mori_backend::authorization::decode_digest(fixture()["payload_hash"].as_str().unwrap())
            .unwrap();
    let limbs = mori_backend::authorization::digest_limbs(digest);
    for i in 0..2 {
        assert_eq!(limbs[i].0.to_string(), fixture()["payload_limbs"][i]);
    }
    p.predecessor = "\u{85}".to_owned();
    assert!(p.predecessor_hash().is_err());
}
