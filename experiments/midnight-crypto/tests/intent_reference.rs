//! Independent reference conformance for signed intent /1.
//! Expected bytes come from `reference-intent.py` and hand-computed fixture inputs, never from the Rust
//! encoder. Ephemeral keys exist only in memory. Signature validity is not account authority.
use midnight_base_crypto::{ecdsa, schnorr};
use midnight_serialize::Serializable;
use moriarty_midnight_crypto::{canonical_message, intent, wallet_message};
use rand::rngs::OsRng;
use serde_json::{Value, json};
use std::{
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
};

const SCHEMES: [&str; 2] = ["schnorr_bip340", "ecdsa_secp256k1_sha256"];
const FRAMINGS: [&str; 2] = ["raw", "midnight-sign-data"];
const ORDER_N: &str = "fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141";

fn vectors() -> &'static Value {
    static V: std::sync::OnceLock<Value> = std::sync::OnceLock::new();
    V.get_or_init(|| serde_json::from_str(include_str!("../fixtures/intent-vectors.json")).unwrap())
}
fn bytes<T: Serializable>(v: &T) -> Vec<u8> {
    let mut b = vec![];
    v.serialize(&mut b).unwrap();
    b
}
fn sha(b: &[u8]) -> String {
    hex::encode(midnight_base_crypto::hash::persistent_hash(b).0)
}
fn run_cli(cmd: &str, text: &str) -> (i32, Value) {
    let mut c = Command::new(env!("CARGO_BIN_EXE_moriarty-midnight-crypto"))
        .arg(cmd)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut stdin = c.stdin.take().unwrap();
    let _ = stdin.write_all(text.as_bytes());
    drop(stdin);
    let o = c.wait_with_output().unwrap();
    let v: Value = serde_json::from_slice(&o.stdout).unwrap_or_else(|e| panic!("stdout {e}: {:?}", o.stdout));
    (o.status.code().unwrap(), v)
}
/// Library call normalized to (exit, value) with the documented CLI exit mapping.
fn lib(cmd: &str, text: &str) -> (i32, Value) {
    match intent::run(cmd, text) {
        Ok(v) => (if v.get("signature_valid") == Some(&Value::Bool(false)) { 1 } else { 0 }, v),
        Err(e) => (2, json!({"status":"IntentRejected","code":e.code,"error":e.error,"signature_valid":null,"ledger_accepted":false})),
    }
}
fn s(v: &Value) -> &str {
    v.as_str().unwrap()
}
fn golden() -> Vec<Value> {
    let g = vectors()["golden"].as_array().cloned().unwrap_or_default();
    assert_eq!(g.len(), 20, "reference-intent.py write must supply 5 fixtures x 2 schemes x 2 framings");
    g
}
fn fixture_draft(name: &str, scheme: &str, framing: &str) -> Value {
    let v = vectors();
    let f = v["fixtures"].as_array().unwrap().iter().find(|f| f["name"] == name).unwrap();
    let mut d = f["draft"].clone();
    d["sourceSha256"] = golden().iter().find(|g| g["fixture"] == name).unwrap()["draft"]["sourceSha256"].clone();
    d["signature"]["scheme"] = json!(scheme);
    d["signature"]["framing"] = json!(framing);
    d["signature"]["publicKeyHex"] = v["keys"][scheme].clone();
    d
}
fn names() -> Vec<String> {
    vectors()["fixtures"].as_array().unwrap().iter().map(|f| s(&f["name"]).to_string()).collect()
}

enum Key {
    S(schnorr::SigningKey),
    E(ecdsa::SigningKey),
}
fn fresh(scheme: &str) -> (Key, String) {
    if scheme == "schnorr_bip340" {
        let k = schnorr::SigningKey::sample(OsRng);
        let p = hex::encode(bytes(&k.verifying_key()));
        (Key::S(k), p)
    } else {
        let k = ecdsa::SigningKey::sample(OsRng);
        let p = hex::encode(bytes(&k.verifying_key()));
        (Key::E(k), p)
    }
}
fn sign(k: &Key, msg: &[u8]) -> String {
    hex::encode(match k {
        Key::S(k) => bytes(&k.sign(&mut OsRng, msg)),
        Key::E(k) => bytes(&k.sign(msg)),
    })
}
/// Build a draft with a fresh in-memory key, sign the returned message, return (statement, signature, frame bytes, key).
fn signed(name: &str, scheme: &str, framing: &str) -> (Value, String, Vec<u8>, Key) {
    let (key, pubhex) = fresh(scheme);
    let mut d = fixture_draft(name, scheme, framing);
    d["signature"]["publicKeyHex"] = json!(pubhex);
    let (c, b) = lib("intent-build", &d.to_string());
    assert_eq!(c, 0, "{b}");
    let sig = sign(&key, &hex::decode(s(&b["signing_message_hex"])).unwrap());
    (b["statement"].clone(), sig, hex::decode(s(&b["frame_hex"])).unwrap(), key)
}
fn verify(statement: &Value, sig: &str) -> (i32, Value) {
    lib("intent-verify", &json!({"statement":statement,"signatureHex":sig}).to_string())
}
fn assert_unknown_not_false(r: &Value) {
    assert_eq!(r["signature_valid"], Value::Null, "rejection must keep signature validity unknown: {r}");
    assert_eq!(r["ledger_accepted"], false);
}
fn n_minus(s_hex: &str) -> String {
    // 256-bit big-endian n - s without a bignum dependency.
    let n = hex::decode(ORDER_N).unwrap();
    let sb = hex::decode(s_hex).unwrap();
    let (mut out, mut borrow) = (vec![0u8; 32], 0i32);
    for i in (0..32).rev() {
        let mut d = n[i] as i32 - sb[i] as i32 - borrow;
        borrow = if d < 0 { d += 256; 1 } else { 0 };
        out[i] = d as u8;
    }
    hex::encode(out)
}
fn leaves(v: &Value, p: String, out: &mut Vec<String>) {
    match v {
        Value::Object(m) if !m.is_empty() => {
            for (k, x) in m {
                leaves(x, format!("{p}/{}", k.replace('~', "~0").replace('/', "~1")), out);
            }
        }
        Value::Array(a) if !a.is_empty() => {
            for (i, x) in a.iter().enumerate() {
                leaves(x, format!("{p}/{i}"), out);
            }
        }
        _ => out.push(p),
    }
}
fn decimal_step(v: &str, up: bool) -> Option<String> {
    let n: u128 = v.parse().ok()?;
    if up { n.checked_add(1) } else { n.checked_sub(1) }.map(|x| x.to_string())
}
/// Mutation change-sets for one leaf; several candidates because some leaves are tied by typed rules.
fn candidates(path: &str, st: &Value, other_key: &dyn Fn(&str) -> String) -> Vec<Vec<(String, Value)>> {
    let cur = st.pointer(path).unwrap();
    let scheme = s(&st["signature"]["scheme"]).to_string();
    let other_scheme = if scheme == SCHEMES[0] { SCHEMES[1] } else { SCHEMES[0] };
    let kind = s(&st["intent"]["operation"]["kind"]).to_string();
    let amount = s(&st["intent"]["operation"]["amount"]).to_string();
    let swap = || -> Vec<(String, Value)> {
        let signer = st["intent"]["signer"].clone();
        if kind == "Transfer" {
            vec![
                ("/selectedActionId".into(), json!("RepayAccrualFirst")),
                ("/intent/feeCap".into(), json!("0")),
                ("/intent/netFloor".into(), json!("0")),
                ("/intent/operation".into(), json!({"kind":"Repay","payer":signer,"obligationId":"Loan","amount":amount,"conversion":"identity"})),
            ]
        } else {
            vec![
                ("/selectedActionId".into(), json!("TransferLiteralFee")),
                ("/intent/operation".into(), json!({"kind":"Transfer","from":signer,"recipient":"Recipient","feeRecipient":"Fee","amount":amount,"fee":"0"})),
            ]
        }
    };
    let one = |v: Value| vec![vec![(path.to_string(), v)]];
    match path {
        "/signature/scheme" => vec![vec![(path.into(), json!(other_scheme)), ("/signature/publicKeyHex".into(), json!(other_key(other_scheme)))]],
        "/signature/publicKeyHex" => one(json!(other_key(&scheme))),
        "/signature/framing" => one(json!(if cur == "raw" { "midnight-sign-data" } else { "raw" })),
        "/selectedActionId" | "/intent/operation/kind" => vec![swap()],
        "/intent/signer" | "/intent/operation/from" | "/intent/operation/payer" => {
            let alias = if kind == "Transfer" { "/intent/operation/from" } else { "/intent/operation/payer" };
            vec![vec![("/intent/signer".into(), json!("Other")), (alias.into(), json!("Other"))]]
        }
        "/intent/operation/amount" => vec![vec![(path.into(), json!(decimal_step(s(cur), true).unwrap())), ("/intent/grossCap".into(), json!(decimal_step(s(&st["intent"]["grossCap"]), true).unwrap()))]],
        "/intent/operation/fee" => vec![vec![
            (path.into(), json!(decimal_step(s(cur), true).unwrap())),
            ("/intent/grossCap".into(), json!(decimal_step(s(&st["intent"]["grossCap"]), true).unwrap())),
            ("/intent/feeCap".into(), json!(decimal_step(s(&st["intent"]["feeCap"]), true).unwrap())),
        ]],
        "/asset/symbol" => one(if cur.is_null() { json!("USD") } else { Value::Null }),
        _ => match cur {
            Value::String(t) if !t.is_empty() && t.bytes().all(|b| b.is_ascii_digit()) => {
                let mut c = vec![vec![(path.to_string(), json!(decimal_step(t, true).unwrap()))]];
                if let Some(d) = decimal_step(t, false) {
                    c.push(vec![(path.to_string(), json!(d))]);
                }
                c
            }
            Value::String(t) if t.len() == 64 && t.bytes().all(|b| b.is_ascii_hexdigit()) => {
                let flipped = if t.starts_with('0') { "1" } else { "0" };
                one(json!(format!("{flipped}{}", &t[1..])))
            }
            Value::String(t) => one(json!(format!("{t}x"))),
            _ => one(json!("unmutatable")),
        },
    }
}
fn expected_class(path: &str, kind: &str) -> &'static str {
    const CONST: [&str; 12] = [
        "/profile", "/core", "/sourceProfile", "/authoringProfile", "/intent/version", "/intent/failure",
        "/intent/delegation", "/intent/recovery", "/intent/observations", "/intent/disclosures",
        "/intent/retainedEffects", "/intent/retainedDuties",
    ];
    if path == "/ownerProgramSha256" {
        "derived-hash"
    } else if CONST.contains(&path)
        || (kind == "Repay" && ["/intent/feeCap", "/intent/netFloor", "/intent/operation/conversion"].contains(&path))
    {
        "constrained-const"
    } else {
        "bound"
    }
}

#[test]
fn golden_frames_match_independent_python_reference() {
    let g = golden();
    for v in &g {
        let draft = v["draft"].to_string();
        let (c, b) = lib("intent-build", &draft);
        let id = format!("{} {} {}", v["fixture"], v["scheme"], v["framing"]);
        assert_eq!(c, 0, "{id}: {b}");
        assert_eq!(b["statement"], v["statement"], "{id}");
        assert_eq!(b["statement"]["ownerProgramSha256"], v["owner_program_sha256"], "{id}");
        assert_eq!(b["frame_hex"], v["frame_hex"], "{id}");
        assert_eq!(b["frame_sha256"], v["frame_sha256"], "{id}");
        assert_eq!(b["signing_message_hex"], v["signing_message_hex"], "{id}");
        assert_eq!(b["qualification"], "signature-protocol-only");
        let (c, f) = lib("intent-frame", &v["statement"].to_string());
        assert_eq!((c, &f["frame_hex"], &f["frame_sha256"]), (0, &v["frame_hex"], &v["frame_sha256"]), "{id}");
        // Frame-shape facts are checked here from the Python golden bytes, not from Rust output.
        let frame = hex::decode(s(&v["frame_hex"])).unwrap();
        let msg = hex::decode(s(&v["signing_message_hex"])).unwrap();
        if v["framing"] == "raw" {
            assert_eq!(msg, frame);
        } else {
            let prefix = format!("midnight_signed_message:{}:", frame.len());
            assert_eq!(&msg[..prefix.len()], prefix.as_bytes());
            assert_eq!(&msg[prefix.len()..], &frame[..]);
        }
        assert_eq!(sha(&frame), s(&v["frame_sha256"]));
    }
    // The real binary yields the same bytes for one vector per scheme/framing.
    for v in g.iter().filter(|v| v["fixture"] == "transfer-fee" || v["fixture"] == "repay-full") {
        let (c, b) = run_cli("intent-build", &v["draft"].to_string());
        assert_eq!((c, &b["frame_hex"], &b["signing_message_hex"]), (0, &v["frame_hex"], &v["signing_message_hex"]));
        let (c, f) = run_cli("intent-frame", &serde_json::to_string_pretty(&v["statement"]).unwrap());
        assert_eq!((c, &f["frame_sha256"]), (0, &v["frame_sha256"]));
    }
}

#[test]
fn hand_computed_fixture_scope_is_what_the_vectors_sign() {
    for f in vectors()["fixtures"].as_array().unwrap() {
        let (e, i) = (&f["expected_scope"], &f["draft"]["intent"]);
        assert_eq!(e["amount"], i["operation"]["amount"], "{}", f["name"]);
        for k in ["grossCap", "feeCap", "netFloor"] {
            assert_eq!(e[k], i[k], "{} {k}", f["name"]);
        }
        assert_eq!(e["signer"], i["signer"]);
        assert_eq!(e["kind"], i["operation"]["kind"]);
        if e["kind"] == "Transfer" {
            assert_eq!(e["fee"], i["operation"]["fee"]);
        }
    }
}

#[test]
fn corpus_matches_oracle_and_native_rejections_stay_closed() {
    let v = vectors();
    let corpus = v["corpus"].as_array().expect("reference-intent.py write must supply the corpus");
    let mut families = std::collections::BTreeMap::<String, usize>::new();
    assert!(corpus.len() >= 60, "corpus unexpectedly small: {}", corpus.len());
    for e in corpus {
        let mut text = s(&e["input_text"]).to_string();
        if let Some(n) = e["pad_spaces"].as_u64() {
            text.push_str(&" ".repeat(n as usize));
        }
        *families.entry(s(&e["family"]).into()).or_default() += 1;
        let want = &e["expect"];
        let name = s(&e["name"]);
        for (c, r) in [lib(s(&e["command"]), &text), run_cli(s(&e["command"]), &text)] {
            if want["outcome"] == "accept" {
                assert_eq!(c, 0, "{name}: {r}");
                for k in ["frame_hex", "frame_sha256", "signing_message_hex"] {
                    assert_eq!(r[k], want[k], "{name} {k}");
                }
                assert_eq!(r["statement"], want["statement"], "{name}");
            } else {
                assert_eq!(c, 2, "{name}: {r}");
                let keys: Vec<_> = r.as_object().unwrap().keys().cloned().collect();
                let mut sorted = keys.clone();
                sorted.sort();
                assert_eq!(sorted, ["code", "error", "ledger_accepted", "signature_valid", "status"], "{name}");
                assert_eq!(r["status"], "IntentRejected");
                assert_unknown_not_false(&r);
                let ok = match &want["code"] {
                    Value::Array(a) => a.contains(&r["code"]),
                    c => *c == r["code"],
                };
                assert!(ok, "{name}: expected {} got {}", want["code"], r["code"]);
            }
        }
    }
    for f in ["unicode", "order", "source-label", "hash", "closed-empty", "numeric-key", "closed-unknown", "overflow", "input-bound", "scalar-text"] {
        assert!(families.get(f).copied().unwrap_or(0) > 0, "missing corpus family {f}");
    }
    println!("S1_CORPUS {}", json!(families));
}

#[test]
fn numeric_object_key_counterexample_is_preserved_and_new_protocol_rejects_unknown_keys() {
    // Prior experimental codec: ordinal order puts "10" before "2"; a JS-style integer-index object enumerates 2 first.
    let fixtures: Value = serde_json::from_str(include_str!("../fixtures/moriarty.json")).unwrap();
    let old = &fixtures["codec_vectors"][0];
    let text = old["statement"].to_string();
    assert!(text.contains("\"10\"") && text.contains("\"2\""));
    let frame = canonical_message(&text).unwrap();
    assert_eq!(hex::encode(&frame), s(&old["golden_frame_hex"]));
    let payload = std::str::from_utf8(&frame[frame.len() - u32::from_be_bytes(frame[36..40].try_into().unwrap()) as usize..]).unwrap();
    assert!(payload.find("\"10\"").unwrap() < payload.find("\"2\"").unwrap(), "ordinal key order must put 10 before 2: {payload}");
    // New closed protocol: any extra key, including numeric, is rejected at every object level.
    for scheme in SCHEMES {
        let d = fixture_draft("transfer-fee", scheme, "raw");
        assert_eq!(lib("intent-build", &d.to_string()).0, 0);
        for level in ["", "/domain", "/asset", "/signature", "/intent", "/intent/operation"] {
            for key in ["2", "10", "0", "extra"] {
                let mut bad = d.clone();
                let target = if level.is_empty() { &mut bad } else { bad.pointer_mut(level).unwrap() };
                target.as_object_mut().unwrap().insert(key.into(), json!("x"));
                for (c, r) in [lib("intent-build", &bad.to_string()), run_cli("intent-build", &bad.to_string())] {
                    assert_eq!((c, &r["code"]), (2, &json!("schema")), "{level}/{key}: {r}");
                    assert_unknown_not_false(&r);
                }
            }
        }
    }
}

fn helper_path() -> PathBuf {
    std::env::var_os("S1_INTENT_FIXTURE").map(PathBuf::from).unwrap_or_else(|| {
        let exe = std::env::current_exe().unwrap();
        exe.parent().unwrap().parent().unwrap().join("examples").join("intent-fixture")
    })
}
fn helper(args: &[&str], stdin: &[u8], cwd: &std::path::Path) -> std::process::Output {
    let mut c = Command::new(helper_path())
        .args(args)
        .current_dir(cwd)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| panic!("build the helper first: cargo build --example intent-fixture ({e})"));
    let mut si = c.stdin.take().unwrap();
    let _ = si.write_all(stdin);
    drop(si);
    c.wait_with_output().unwrap()
}
fn python(args: &[&str], stdin: &str) -> Value {
    let mut c = Command::new("python3")
        .arg(concat!(env!("CARGO_MANIFEST_DIR"), "/reference-intent.py"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("python3 is required for the independent reference");
    let mut si = c.stdin.take().unwrap();
    si.write_all(stdin.as_bytes()).unwrap();
    drop(si);
    let o = c.wait_with_output().unwrap();
    assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
    serde_json::from_slice(&o.stdout).unwrap()
}

#[test]
fn test_only_helper_public_receipts_verify_natively_and_in_python() {
    let temp = std::env::temp_dir().join(format!("s1-helper-{}", std::process::id()));
    std::fs::create_dir_all(&temp).unwrap();
    let mut receipts = vec![];
    for name in names() {
        for scheme in SCHEMES {
            for framing in FRAMINGS {
                let mut draft = fixture_draft(&name, scheme, framing);
                // The helper owns key, scheme and framing metadata; stale caller values must be replaced.
                draft["signature"] = json!({"scheme":"ecdsa_secp256k1_sha256","publicKeyHex":"00","framing":"raw","keyRef":draft["signature"]["keyRef"]});
                let o = helper(&[scheme, framing], draft.to_string().as_bytes(), &temp);
                assert!(o.status.success(), "{name} {scheme} {framing}: {}", String::from_utf8_lossy(&o.stderr));
                assert!(o.stderr.is_empty(), "helper must be quiet on stderr");
                let r: Value = serde_json::from_slice(&o.stdout).unwrap();
                let mut keys: Vec<_> = r.as_object().unwrap().keys().cloned().collect();
                keys.sort();
                assert_eq!(keys, ["signatureHex", "statement"]);
                assert_eq!(r["statement"]["signature"]["scheme"], scheme);
                assert_eq!(r["statement"]["signature"]["framing"], framing);
                let g = golden().into_iter().find(|g| g["fixture"] == name.as_str() && g["scheme"] == scheme && g["framing"] == framing).unwrap();
                let mut a = r["statement"].clone();
                let mut b = g["statement"].clone();
                let key = a["signature"]["publicKeyHex"].clone();
                a["signature"]["publicKeyHex"] = json!("-");
                b["signature"]["publicKeyHex"] = json!("-");
                // Everything but the ephemeral key and the hash-independent frame must equal the independent statement.
                assert_eq!(a["ownerProgramSha256"], b["ownerProgramSha256"]);
                assert_eq!(a, b);
                assert_ne!(key, g["statement"]["signature"]["publicKeyHex"], "helper must not reuse the public golden key");
                let (c, v) = run_cli("intent-verify", std::str::from_utf8(&o.stdout).unwrap());
                assert_eq!((c, &v["signature_valid"]), (0, &json!(true)), "{v}");
                assert_eq!(v["authority_valid"], Value::Null);
                assert_eq!(v["ledger_accepted"], false);
                receipts.push(r);
            }
        }
    }
    assert_eq!(std::fs::read_dir(&temp).unwrap().count(), 0, "helper must write no files");
    std::fs::remove_dir_all(&temp).unwrap();
    let keys: std::collections::BTreeSet<_> = receipts.iter().map(|r| s(&r["statement"]["signature"]["publicKeyHex"]).to_string()).collect();
    assert_eq!(keys.len(), receipts.len(), "every helper run uses a new ephemeral key");
    let checked = python(&["verify"], &Value::Array(receipts.clone()).to_string());
    for (r, p) in receipts.iter().zip(checked.as_array().unwrap()) {
        assert_eq!(p["valid"], true, "independent Python verifier disagrees: {p}");
        let (_, f) = lib("intent-frame", &r["statement"].to_string());
        assert_eq!(p["frame_sha256"], f["frame_sha256"]);
    }
    println!("S1_HELPER_RECEIPTS {}", receipts.len());
}

#[test]
fn test_only_helper_refuses_bad_use_without_stdout_or_key_material() {
    let temp = std::env::temp_dir().join(format!("s1-helper-bad-{}", std::process::id()));
    std::fs::create_dir_all(&temp).unwrap();
    let draft = fixture_draft("transfer-fee", "schnorr_bip340", "raw").to_string();
    let mut built = lib("intent-build", &draft).1["statement"].clone();
    built["intent"]["nonce"] = json!("n1");
    let cases: Vec<(Vec<&str>, Vec<u8>)> = vec![
        (vec![], draft.clone().into_bytes()),
        (vec!["schnorr_bip340"], draft.clone().into_bytes()),
        (vec!["schnorr_bip340", "raw", "extra"], draft.clone().into_bytes()),
        (vec!["rsa", "raw"], draft.clone().into_bytes()),
        (vec!["schnorr_bip340", "Raw"], draft.clone().into_bytes()),
        (vec!["schnorr_bip340", "raw"], vec![b' '; 65537]),
        (vec!["schnorr_bip340", "raw"], b"{".to_vec()),
        (vec!["schnorr_bip340", "raw"], vec![0xff, 0xfe]),
        (vec!["schnorr_bip340", "raw"], built.to_string().into_bytes()),
        (vec!["schnorr_bip340", "raw"], draft.replacen("{", "{\"profile\":\"x\",", 1).into_bytes()),
    ];
    for (args, input) in cases {
        let o = helper(&args, &input, &temp);
        assert!(!o.status.success(), "{args:?} must fail");
        assert!(o.stdout.is_empty(), "{args:?} must print nothing on stdout");
        let err = String::from_utf8_lossy(&o.stderr).to_lowercase();
        assert!(!err.contains("secret") || err.contains("test-only"), "{err}");
    }
    assert_eq!(std::fs::read_dir(&temp).unwrap().count(), 0);
    std::fs::remove_dir_all(&temp).unwrap();
}

#[test]
fn malformed_signatures_high_s_wrong_key_and_wrong_context_keep_unknown_and_false_distinct() {
    for scheme in SCHEMES {
        for framing in FRAMINGS {
            let (st, sig, frame, key) = signed("transfer-fee", scheme, framing);
            assert_eq!(verify(&st, &sig).0, 0);
            // Malformed spellings and lengths: unknown (null), never false.
            let mut malformed = vec![String::new(), "00".into(), "aa".repeat(31), "aa".repeat(32), "aa".repeat(63), "aa".repeat(65), "aa".repeat(128), sig.to_uppercase(), format!("0x{}", &sig[2..]), format!("{}g", &sig[..127]), format!("{} ", &sig[..127])];
            malformed.push("00".repeat(64));
            malformed.push("ff".repeat(64));
            malformed.push(format!("{}{}", "ff".repeat(32), &sig[64..]));
            malformed.push(format!("{}{}", &sig[..64], "ff".repeat(32)));
            malformed.push(format!("{}{}", &sig[..64], ORDER_N));
            malformed.push(format!("{}{}", &sig[..64], "00".repeat(32)));
            for m in malformed {
                for (c, r) in [verify(&st, &m), run_cli("intent-verify", &json!({"statement":st,"signatureHex":m}).to_string())] {
                    assert_eq!(c, 2, "{scheme} {m}: {r}");
                    assert_eq!(r["status"], "IntentRejected");
                    assert_eq!(r["code"], "signature");
                    assert_unknown_not_false(&r);
                }
            }
            // Well formed but wrong: native false (exit 1), different from malformed.
            let mut flipped = hex::decode(&sig).unwrap();
            flipped[63] ^= 1;
            let mut reversed = hex::decode(&sig).unwrap();
            reversed.reverse();
            for wrong in [hex::encode(&flipped), hex::encode(&reversed)] {
                let (c, r) = verify(&st, &wrong);
                assert!(matches!((c, &r["signature_valid"]), (1, Value::Bool(false)) | (2, Value::Null)), "{r}");
                assert_ne!(r["signature_valid"], true);
            }
            if scheme == "ecdsa_secp256k1_sha256" {
                let twin = format!("{}{}", &sig[..64], n_minus(&sig[64..]));
                let (c, r) = verify(&st, &twin);
                assert_eq!((c, &r["signature_valid"]), (1, &json!(false)), "high-S twin must be native false: {r}");
                assert_eq!(run_cli("intent-verify", &json!({"statement":st,"signatureHex":twin}).to_string()).0, 1);
            }
            // Keys: malformed key is unknown; another valid key is native false; scheme confusion is rejected by key length.
            let mut bad = st.clone();
            let badkeys: Vec<String> = if scheme == "schnorr_bip340" {
                vec!["ff".repeat(32), "00".repeat(32), "00".repeat(33), "aa".repeat(31)]
            } else {
                vec!["04".to_string() + &"aa".repeat(32), "00".repeat(33), "ff".repeat(33), "02".to_string() + &"ff".repeat(32), "02".to_string() + &"aa".repeat(31)]
            };
            for k in badkeys {
                bad["signature"]["publicKeyHex"] = json!(k);
                let (c, r) = verify(&bad, &sig);
                assert_eq!((c, &r["code"]), (2, &json!("key")), "{k}: {r}");
                assert_unknown_not_false(&r);
            }
            bad["signature"]["publicKeyHex"] = json!(fresh(scheme).1);
            let (c, r) = verify(&bad, &sig);
            assert_eq!((c, &r["signature_valid"]), (1, &json!(false)), "another valid key: {r}");
            let mut confused = st.clone();
            confused["signature"]["scheme"] = json!(if scheme == SCHEMES[0] { SCHEMES[1] } else { SCHEMES[0] });
            let (c, r) = verify(&confused, &sig);
            assert_eq!((c, &r["code"]), (2, &json!("key")), "{r}");
            // Context: signatures over other messages never verify; the right message under the other framing never does.
            let right = if framing == "raw" { frame.clone() } else { wallet_message(&frame) };
            let other = if framing == "raw" { wallet_message(&frame) } else { frame.clone() };
            let digest = hex::decode(sha(&frame)).unwrap();
            let mut variants: Vec<(String, Vec<u8>)> = vec![
                ("other framing".into(), other),
                ("sha256 of frame".into(), digest.clone()),
                ("hex text of frame".into(), hex::encode(&frame).into_bytes()),
                ("hex text under prefix".into(), wallet_message(hex::encode(&frame).as_bytes())),
                ("digest under prefix".into(), wallet_message(&digest)),
                ("length off by one".into(), [format!("midnight_signed_message:{}:", frame.len() + 1).into_bytes(), frame.clone()].concat()),
                ("length off by minus one".into(), [format!("midnight_signed_message:{}:", frame.len() - 1).into_bytes(), frame.clone()].concat()),
                ("leading zero length".into(), [format!("midnight_signed_message:0{}:", frame.len()).into_bytes(), frame.clone()].concat()),
                ("uppercase prefix".into(), [format!("MIDNIGHT_SIGNED_MESSAGE:{}:", frame.len()).into_bytes(), frame.clone()].concat()),
                ("double prefix".into(), wallet_message(&wallet_message(&frame))),
                ("utf16-ish length".into(), [format!("midnight_signed_message:{}:", frame.len() * 2).into_bytes(), frame.clone()].concat()),
                ("empty".into(), vec![]),
                ("frame minus last byte".into(), frame[..frame.len() - 1].to_vec()),
                ("frame plus NUL".into(), [frame.clone(), vec![0]].concat()),
            ];
            let old = canonical_message(&st.to_string()).unwrap();
            variants.push(("old experimental frame".into(), old.clone()));
            variants.push(("old experimental wallet frame".into(), wallet_message(&old)));
            for (label, msg) in variants {
                assert_ne!(msg, right, "{label}");
                let (c, r) = verify(&st, &sign(&key, &msg));
                assert_eq!((c, &r["signature_valid"]), (1, &json!(false)), "{scheme} {framing} {label}: {r}");
            }
        }
    }
}

#[test]
fn signatures_do_not_transfer_between_fixtures_or_old_experimental_protocol() {
    for scheme in SCHEMES {
        let (key, pubhex) = fresh(scheme);
        let mut all = vec![];
        for name in names() {
            let mut d = fixture_draft(&name, scheme, "raw");
            d["signature"]["publicKeyHex"] = json!(pubhex);
            let (_, b) = lib("intent-build", &d.to_string());
            let sig = sign(&key, &hex::decode(s(&b["signing_message_hex"])).unwrap());
            assert_eq!(verify(&b["statement"], &sig).0, 0);
            all.push((name, b["statement"].clone(), sig));
        }
        for (i, (ni, _, sig)) in all.iter().enumerate() {
            for (j, (nj, st, _)) in all.iter().enumerate() {
                if i != j {
                    assert_eq!(verify(st, sig).0, 1, "{scheme} signature of {ni} accepted for {nj}");
                }
            }
        }
        // Old experimental signatures over the same statement object are cross-protocol negatives in both directions.
        let st = &all[0].1;
        let old_frame = canonical_message(&st.to_string()).unwrap();
        let new_frame = hex::decode(s(&lib("intent-frame", &st.to_string()).1["frame_hex"])).unwrap();
        assert!(old_frame.starts_with(b"moriarty-midnight-auth-experiment/1\0"));
        assert!(new_frame.starts_with(b"moriarty-signed-intent/1\0"));
        assert!(!new_frame.starts_with(b"moriarty-midnight-auth-experiment/1\0"));
        assert_ne!(old_frame, new_frame);
        let old_sig = sign(&key, &old_frame);
        assert_eq!(verify(st, &old_sig).0, 1);
        for wallet in [false, true] {
            let req = |sig: &str| json!({"scheme":scheme,"statement":st,"public_key_hex":pubhex,"signature_hex":sig,"wallet_prefix":wallet});
            let (c, r) = run_cli("verify", &req(&sign(&key, &if wallet { wallet_message(&old_frame) } else { old_frame.clone() })).to_string());
            assert_eq!((c, &r["signature_valid"]), (0, &json!(true)), "old verify control must keep working: {r}");
            let (c, r) = run_cli("verify", &req(&sign(&key, &if wallet { wallet_message(&new_frame) } else { new_frame.clone() })).to_string());
            assert_eq!((c, &r["signature_valid"]), (1, &json!(false)), "new frame signature must not verify as old experimental: {r}");
        }
        // The owner-program digest domain is also separate: signing its preimage proves nothing.
        let (_, built) = lib("intent-build", &fixture_draft("transfer-fee", scheme, "raw").to_string());
        assert_ne!(built["statement"]["ownerProgramSha256"], built["frame_sha256"]);
    }
}

#[test]
fn every_leaf_is_bound_constrained_or_derived_and_none_verify() {
    let mut classes = std::collections::BTreeMap::<&str, usize>::new();
    let (mut runs, mut leaf_total, mut raw_hash, mut raw_false, mut raw_rejected) = (0usize, 0usize, 0usize, 0usize, 0usize);
    let other_key = |scheme: &str| vectors()["keys"][scheme].as_str().unwrap().to_string();
    let mut per_fixture = vec![];
    for name in names() {
        for scheme in SCHEMES {
            for framing in FRAMINGS {
                let (st, sig, frame, _key) = signed(&name, scheme, framing);
                // Alternate valid key: the public golden generator differs from the fresh ephemeral key.
                let mut paths = vec![];
                leaves(&st, String::new(), &mut paths);
                let kind = s(&st["intent"]["operation"]["kind"]).to_string();
                leaf_total += paths.len();
                let mut counts = std::collections::BTreeMap::<&str, usize>::new();
                for path in &paths {
                    let class = expected_class(path, &kind);
                    let mut outcome = "constrained-const";
                    // Raw mutation keeps the stale digest: projection leaves hit the hash check, others stay native false/reject.
                    for set in candidates(path, &st, &other_key).into_iter().take(1) {
                        let mut raw = st.clone();
                        for (p, v) in &set {
                            *raw.pointer_mut(p).unwrap() = v.clone();
                        }
                        assert_ne!(raw, st, "{path}");
                        let (c, r) = verify(&raw, &sig);
                        runs += 1;
                        assert_ne!(r["signature_valid"], true, "{name} {path} verified after mutation");
                        match c {
                            2 => { assert_unknown_not_false(&r); if r["code"] == "hash" { raw_hash += 1 } else { raw_rejected += 1 } }
                            1 => raw_false += 1,
                            _ => panic!("{path}: {r}"),
                        }
                    }
                    if path == "/ownerProgramSha256" {
                        outcome = "derived-hash";
                    } else {
                        // Re-hashed mutation: a different valid statement carrying the original signature must be native false.
                        for set in candidates(path, &st, &other_key) {
                            let mut m = st.clone();
                            for (p, v) in &set {
                                *m.pointer_mut(p).unwrap() = v.clone();
                            }
                            m.as_object_mut().unwrap().remove("ownerProgramSha256");
                            runs += 1;
                            let (c, b) = lib("intent-build", &m.to_string());
                            if c != 0 {
                                assert_eq!(c, 2);
                                continue;
                            }
                            assert_ne!(hex::decode(s(&b["frame_hex"])).unwrap(), frame, "{path}");
                            let (c, r) = verify(&b["statement"], &sig);
                            assert_eq!((c, &r["signature_valid"]), (1, &json!(false)), "{name} {scheme} {framing} {path}: {r}");
                            outcome = "bound";
                            break;
                        }
                    }
                    assert_eq!(outcome, class, "{name} {scheme} {framing} {path}");
                    *classes.entry(class).or_default() += 1;
                    *counts.entry(class).or_default() += 1;
                }
                per_fixture.push(json!({"fixture":name,"scheme":scheme,"framing":framing,"leaves":paths.len(),"classes":counts}));
            }
        }
    }
    let total: usize = classes.values().sum();
    assert_eq!(total, leaf_total, "every walker leaf must be classified exactly once");
    let coverage = json!({
        "profile": "moriarty-s1-intent-rust-coverage/1",
        "leafCountFromWalker": leaf_total,
        "classifiedLeaves": total,
        "unclassifiedLeaves": leaf_total - total,
        "byClass": classes,
        "nativeCalls": runs,
        "rawMutationOutcomes": {"hashRejected": raw_hash, "otherRejected": raw_rejected, "nativeFalse": raw_false},
        "goldenVectors": golden().len(),
        "corpusEntries": vectors()["corpus"].as_array().map(|c| c.len()),
        "perFixture": per_fixture,
        "claims": {"allAuthNativeClosed": false, "reason": "local protocol and verifier tests only: no key authority, wallet, snapshot, ledger or proof evidence"}
    });
    if let Some(path) = std::env::var_os("S1_RUST_COVERAGE_OUT") {
        std::fs::write(path, serde_json::to_string_pretty(&coverage).unwrap() + "\n").unwrap();
    }
    println!("S1_RUST_COVERAGE {coverage}");
}
