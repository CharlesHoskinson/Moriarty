//! Production CLI signed owner-intent callers. Ordinary host reconstruction,
//! compiled-relation checker and empty/nonexistent parameter diagnostics only:
//! no SRS, key generation, native proving or native verification.
use serde_json::{Value, json};
use std::{
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::atomic::{AtomicUsize, Ordering},
};

const PROFILE: &str = "moriarty-jubjub-owner-intent/1";
const ALTERNATIVE: &str = "fec0b07c971073c462c88d722fb5fe8df764311114dbcc79e46263b74a863850e92947318a0258f88a6869c98a7e96733662780d4936401c0e732a4c87fb651a98b9a4ecb5630fefa96bd5f061388d910c95dabfa7f2c089c2d3651ca422e900";
static NEXT: AtomicUsize = AtomicUsize::new(0);

fn examples() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/core/native-owner-intent")
}
fn golden() -> Value {
    serde_json::from_slice(&std::fs::read(examples().join("golden-vector.json")).unwrap()).unwrap()
}
fn sha256(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}
fn canonical_hash(value: &Value) -> String {
    sha256(&mori_core::intent::canonical(value).unwrap())
}

struct Files(PathBuf);
impl Drop for Files {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
impl Files {
    fn new() -> Self {
        let dir = std::env::temp_dir().join(format!(
            "mori-signed-cli-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&dir).unwrap();
        Self(dir)
    }
    fn path(&self, name: &str) -> PathBuf {
        self.0.join(name)
    }
    fn write(&self, name: &str, bytes: impl AsRef<[u8]>) -> PathBuf {
        let path = self.path(name);
        std::fs::write(&path, bytes).unwrap();
        path
    }
    fn json(&self, name: &str, value: &Value) -> PathBuf {
        self.write(name, value.to_string())
    }
    /// Golden settlement source, scenario, signed Target, expected authority and wrapper.
    fn golden(self) -> Self {
        let g = golden();
        self.write("source.mori", g["source_utf8"].as_str().unwrap());
        self.json("scenario.json", &g["scenario"]);
        self.write("target.json", g["target_canonical_utf8"].as_str().unwrap());
        let mut authority = serde_json::Map::new();
        for k in [
            "profile",
            "network",
            "anchor",
            "program_address",
            "target_hash",
            "owner",
            "beneficiary",
            "public_key_hex",
            "nonce",
            "predecessor",
            "not_before",
            "not_after",
            "limits",
        ] {
            authority.insert(k.into(), g["payload"][k].clone());
        }
        self.json("authority.json", &Value::Object(authority));
        self.json("signed.json", &g["signed_wrapper"]);
        self
    }
}
fn mori(args: &[&dyn AsRef<std::ffi::OsStr>]) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_mori"));
    command.args(["--color", "never"]);
    for arg in args {
        command.arg(arg);
    }
    command.output().unwrap()
}
fn stderr(o: &Output) -> String {
    String::from_utf8_lossy(&o.stderr).into_owned()
}
fn refused(o: &Output) -> String {
    assert!(
        !o.status.success(),
        "{}",
        String::from_utf8_lossy(&o.stdout)
    );
    assert!(
        o.stdout.is_empty(),
        "{}",
        String::from_utf8_lossy(&o.stdout)
    );
    stderr(o)
}
fn signed_intent(f: &Files, extra: &[&str]) -> Output {
    let (s, sc, a, t) = (
        f.path("source.mori"),
        f.path("scenario.json"),
        f.path("authority.json"),
        f.path("target.json"),
    );
    let mut args: Vec<&dyn AsRef<std::ffi::OsStr>> = vec![
        &"intent",
        &s,
        &"--action",
        &"repay",
        &"--scenario",
        &sc,
        &"--authority",
        &a,
        &"--profile",
        &PROFILE,
        &"--target",
        &t,
        &"--at",
        &"15",
    ];
    for e in extra {
        args.push(e);
    }
    mori(&args)
}
fn proof_args<'a>(
    f: &'a Files,
    op: &'a str,
    params: &'a Path,
) -> Vec<Box<dyn AsRef<std::ffi::OsStr> + 'a>> {
    vec![
        Box::new(op),
        Box::new(f.path("source.mori")),
        Box::new("--action"),
        Box::new("repay"),
        Box::new("--scenario"),
        Box::new(f.path("scenario.json")),
        Box::new("--target"),
        Box::new(f.path("target.json")),
        Box::new("--params-dir"),
        Box::new(params),
        Box::new("--max-k"),
        Box::new("19"),
    ]
}
fn run(
    args: Vec<Box<dyn AsRef<std::ffi::OsStr> + '_>>,
    extra: &[&dyn AsRef<std::ffi::OsStr>],
) -> Output {
    let mut all: Vec<&dyn AsRef<std::ffi::OsStr>> = args.iter().map(|b| b.as_ref() as _).collect();
    all.extend_from_slice(extra);
    mori(&all)
}
fn signed_group(f: &Files) -> [PathBuf; 2] {
    [f.path("signed.json"), f.path("authority.json")]
}

#[test]
fn compile_dispatches_on_trusted_target_mode_and_refuses_signed_admission() {
    let f = Files::new().golden();
    let out = f.path("signed-artifact");
    let o = mori(&[
        &"compile",
        &f.path("source.mori"),
        &"--target",
        &f.path("target.json"),
        &"--out",
        &out,
    ]);
    assert!(o.status.success(), "{}", stderr(&o));
    let text = std::fs::read_to_string(out.join("artifact.json")).unwrap();
    let artifact: mori_backend::Artifact = serde_json::from_str(&text).unwrap();
    assert_eq!(artifact.format, "moriarty-zkir-artifact/3");
    assert_eq!(artifact.authorization_mode, PROFILE);
    let repay = artifact
        .circuits
        .iter()
        .find(|c| c.action == "repay")
        .unwrap();
    assert_eq!(repay.context_hash, golden()["context_hash"]);
    assert_eq!(repay.authorization_bindings.len(), 15);
    assert_eq!(repay.private_inputs.len(), 3);
    let unsigned = f.json(
        "unsigned-target.json",
        &serde_json::to_value(mori_backend::Target::ledger9(
            "local",
            "native-liability-qa",
        ))
        .unwrap(),
    );
    let out = f.path("unsigned-artifact");
    let o = mori(&[
        &"compile",
        &f.path("source.mori"),
        &"--target",
        &unsigned,
        &"--out",
        &out,
    ]);
    assert!(o.status.success(), "{}", stderr(&o));
    let artifact: Value =
        serde_json::from_slice(&std::fs::read(out.join("artifact.json")).unwrap()).unwrap();
    assert_eq!(artifact["authorization_mode"], "unsigned");
    // A second payer in an unexecuted action refuses before any output exists.
    let second = golden()["source_utf8"].as_str().unwrap().replace(
        "action observe() {",
        "action other() { transfer(QA, \"carol\", \"bob\", 1); }\n  action observe() {",
    );
    let src = f.write("second.mori", second);
    let out = f.path("second-artifact");
    let err = refused(&mori(&[
        &"compile",
        &src,
        &"--target",
        &f.path("target.json"),
        &"--out",
        &out,
    ]));
    assert!(err.contains("mori::backend::signed"), "{err}");
    assert!(!out.exists());
    // Explicit null policy refuses rather than becoming unsigned.
    let mut target: Value =
        serde_json::from_str(golden()["target_canonical_utf8"].as_str().unwrap()).unwrap();
    target["signed_policy"] = Value::Null;
    let null = f.json("null-target.json", &target);
    let out = f.path("null-artifact");
    refused(&mori(&[
        &"compile",
        &f.path("source.mori"),
        &"--target",
        &null,
        &"--out",
        &out,
    ]));
    assert!(!out.exists());
}

#[test]
fn signed_preparation_reproduces_golden_and_review_names_exact_scope() {
    let f = Files::new().golden();
    let g = golden();
    let o = signed_intent(&f, &[]);
    assert!(o.status.success(), "{}", stderr(&o));
    let plain: Value = serde_json::from_slice(&o.stdout).unwrap();
    assert_eq!(plain["profile"], "moriarty-jubjub-prepared-intent/1");
    assert_eq!(
        mori_core::intent::canonical(&plain["payload"]).unwrap(),
        g["payload_canonical_utf8"].as_str().unwrap().as_bytes()
    );
    assert_eq!(plain["payload_hash"], g["payload_hash"]);
    assert_eq!(plain["context_hash"], g["context_hash"]);
    assert_eq!(plain["message_hex"], g["message_hex_le32"]);
    let o = signed_intent(&f, &["--review"]);
    assert!(o.status.success(), "{}", stderr(&o));
    let review = String::from_utf8(o.stdout).unwrap();
    let (header, statement) = review.split_once("Exact prepared statement:\n").unwrap();
    assert_eq!(serde_json::from_str::<Value>(statement).unwrap(), plain);
    for needle in [
        PROFILE,
        g["message_hex_le32"].as_str().unwrap(),
        g["context_hash"].as_str().unwrap(),
        g["payload_hash"].as_str().unwrap(),
        "gross <= 9, fee <= 2, net >= 5, closing owner liability <= 3",
        "existential",
        "not signed",
        "not ledger acceptance",
    ] {
        assert!(header.contains(needle), "{needle}\n{header}");
    }
    let lower = header.to_lowercase();
    for forbidden in ["secret", "private key", "wallet", "approve", "approval"] {
        assert!(!lower.contains(forbidden), "{forbidden}\n{header}");
    }
}

#[test]
fn review_escapes_unicode_labels_without_changing_the_statement() {
    let f = Files::new().golden();
    // Legal non-control Unicode labels (bidi override is a label character,
    // not a C0/C1 control) must be escaped in human review only.
    let hostile = "native\u{202e}λ";
    let mut target: Value =
        serde_json::from_str(golden()["target_canonical_utf8"].as_str().unwrap()).unwrap();
    target["anchor"] = hostile.into();
    f.json("target.json", &target);
    let mut authority: Value =
        serde_json::from_slice(&std::fs::read(f.path("authority.json")).unwrap()).unwrap();
    authority["anchor"] = hostile.into();
    authority["target_hash"] = canonical_hash(&target).into();
    f.json("authority.json", &authority);
    let plain = signed_intent(&f, &[]);
    assert!(plain.status.success(), "{}", stderr(&plain));
    let review = signed_intent(&f, &["--review"]);
    assert!(review.status.success(), "{}", stderr(&review));
    let review = String::from_utf8(review.stdout).unwrap();
    let (header, statement) = review.split_once("Exact prepared statement:\n").unwrap();
    assert!(header.is_ascii(), "{header}");
    assert!(header.contains("native\\u{202e}\\u{3bb}"), "{header}");
    let plain: Value = serde_json::from_slice(&plain.stdout).unwrap();
    assert_eq!(serde_json::from_str::<Value>(statement).unwrap(), plain);
    assert_eq!(plain["payload"]["anchor"], hostile);
    // A control label in the trusted Target refuses before any output.
    target["anchor"] = "native\u{85}".into();
    f.json("target.json", &target);
    refused(&signed_intent(&f, &[]));
}

#[test]
fn signed_and_legacy_argument_groups_refuse_partial_or_mixed_use() {
    let f = Files::new().golden();
    let (s, sc, a, t) = (
        f.path("source.mori"),
        f.path("scenario.json"),
        f.path("authority.json"),
        f.path("target.json"),
    );
    let base: Vec<&dyn AsRef<std::ffi::OsStr>> = vec![
        &"intent",
        &s,
        &"--action",
        &"repay",
        &"--scenario",
        &sc,
        &"--authority",
        &a,
    ];
    let cases: [Vec<&dyn AsRef<std::ffi::OsStr>>; 6] = [
        vec![&"--profile", &PROFILE, &"--at", &"15"],
        vec![&"--profile", &PROFILE, &"--target", &t],
        vec![
            &"--profile",
            &PROFILE,
            &"--target",
            &t,
            &"--at",
            &"15",
            &"--scheme",
            &"schnorr_bip340",
        ],
        vec![
            &"--profile",
            &PROFILE,
            &"--target",
            &t,
            &"--at",
            &"15",
            &"--framing",
            &"raw",
        ],
        vec![&"--target", &t, &"--at", &"15"],
        vec![&"--scheme", &"schnorr_bip340", &"--public-key", &"00"],
    ];
    for extra in cases {
        let mut args = base.clone();
        args.extend(extra);
        let err = refused(&mori(&args));
        assert!(err.contains("mori::command"), "{err}");
        assert!(err.contains("--profile") || err.contains("legacy"), "{err}");
    }
    // verify-intent: signed Target or profile pieces never enter legacy dispatch.
    let sig = f.path("signed.json");
    for extra in [
        vec![&"--target" as &dyn AsRef<std::ffi::OsStr>, &t],
        vec![&"--profile", &PROFILE],
    ] {
        let mut args: Vec<&dyn AsRef<std::ffi::OsStr>> = vec![
            &"verify-intent",
            &s,
            &"--action",
            &"repay",
            &"--scenario",
            &sc,
            &"--signature",
            &sig,
            &"--authority",
            &a,
            &"--at",
            &"15",
        ];
        args.extend(extra);
        let err = refused(&mori(&args));
        assert!(err.contains("mori::command") && err.contains("--"), "{err}");
    }
}

#[test]
fn independent_payment_caps_prepare_exactly_and_each_tighter_cap_refuses() {
    let f = Files::new();
    let expected: Value =
        serde_json::from_slice(&std::fs::read(examples().join("payment.expected.json")).unwrap())
            .unwrap();
    let authority: Value =
        serde_json::from_slice(&std::fs::read(examples().join("payment.authority.json")).unwrap())
            .unwrap();
    let prepare = |caps: &Value| {
        let mut a = authority.clone();
        for k in ["gross_cap", "fee_cap", "net_floor", "liability_cap"] {
            a["limits"][0][k] = caps[k].clone();
        }
        let path = f.json("authority.json", &a);
        mori(&[
            &"intent",
            &examples().join("payment.mori"),
            &"--action",
            &"settle",
            &"--scenario",
            &examples().join("payment.scenario.json"),
            &"--authority",
            &path,
            &"--profile",
            &PROFILE,
            &"--target",
            &examples().join("payment.target.json"),
            &"--at",
            &"15",
        ])
    };
    let o = prepare(&expected["caps_accept"]);
    assert!(o.status.success(), "{}", stderr(&o));
    let prepared: Value = serde_json::from_slice(&o.stdout).unwrap();
    let post = &prepared["review"]["post"];
    assert_eq!(post["state"]["paid"], json!({"type":"UInt","value":"70"}));
    for (account, amount) in expected["post_balances"].as_object().unwrap() {
        let b = post["balances"]
            .as_array()
            .unwrap()
            .iter()
            .find(|b| &b["account"] == account)
            .unwrap();
        assert_eq!(&b["amount"], amount);
    }
    for (caps, word) in expected["individual_cap_refusals"]
        .as_array()
        .unwrap()
        .iter()
        .zip(["gross", "fee", "net", "liability"])
    {
        let err = refused(&prepare(caps));
        assert!(
            err.contains(word) && err.contains("mori::backend::authorization"),
            "{word}: {err}"
        );
    }
}

#[test]
fn signed_verify_intent_accepts_exact_and_alternative_attachment_and_rejects_others() {
    let f = Files::new().golden();
    let verify = |at: &str| {
        mori(&[
            &"verify-intent",
            &f.path("source.mori"),
            &"--action",
            &"repay",
            &"--scenario",
            &f.path("scenario.json"),
            &"--signature",
            &f.path("signed.json"),
            &"--authority",
            &f.path("authority.json"),
            &"--at",
            &at,
            &"--profile",
            &PROFILE,
            &"--target",
            &f.path("target.json"),
        ])
    };
    for at in ["10", "15", "20"] {
        let o = verify(at);
        assert!(
            o.status.success(),
            "{at}: {}",
            String::from_utf8_lossy(&o.stdout)
        );
        let v: Value = serde_json::from_slice(&o.stdout).unwrap();
        assert_eq!(v["status"], "accepted");
        assert_eq!(v["profile"], PROFILE);
        assert_eq!(v["message_hex"], golden()["message_hex_le32"]);
        assert_eq!(v["execution"]["action"], "repay");
    }
    for at in ["9", "21"] {
        let o = verify(at);
        assert!(!o.status.success());
        let v: Value = serde_json::from_slice(&o.stdout).unwrap();
        assert_eq!(v["status"], "rejected");
    }
    let mut wrapper = golden()["signed_wrapper"].clone();
    wrapper["signature_hex"] = ALTERNATIVE.into();
    f.json("signed.json", &wrapper);
    assert!(verify("15").status.success());
    let mut flipped = ALTERNATIVE.to_owned();
    flipped.replace_range(130..132, "00");
    wrapper["signature_hex"] = flipped.into();
    f.json("signed.json", &wrapper);
    let o = verify("15");
    assert!(!o.status.success());
    let v: Value = serde_json::from_slice(&o.stdout).unwrap();
    assert_eq!(v["error"]["code"], "mori::backend::signature");
    // Envelope files are capped at 1 MiB before parsing.
    f.write("signed.json", " ".repeat(1_048_577));
    let err = refused(&verify("15"));
    assert!(err.contains("1 MiB"), "{err}");
}

#[test]
fn signed_prove_reaches_missing_parameters_and_refuses_bad_groups_earlier() {
    let f = Files::new().golden();
    let missing = f.path("missing-params");
    let out = f.path("proof.json");
    let [sig, auth] = signed_group(&f);
    let o = run(
        proof_args(&f, "prove", &missing),
        &[
            &"--signature",
            &sig,
            &"--authority",
            &auth,
            &"--at",
            &"15",
            &"--out",
            &out,
        ],
    );
    let err = refused(&o);
    assert!(err.contains("Cannot open explicit local SRS"), "{err}");
    assert!(!out.exists());
    // Signed Target without its signed group: no unsigned fallback.
    for extra in [
        vec![],
        vec![&"--signature" as &dyn AsRef<std::ffi::OsStr>, &sig],
        vec![&"--signature", &sig, &"--authority", &auth],
    ] {
        let mut all = extra.clone();
        all.extend([&"--out" as &dyn AsRef<std::ffi::OsStr>, &out]);
        let err = refused(&run(proof_args(&f, "prove", &missing), &all));
        assert!(err.contains("--signature, --authority and --at"), "{err}");
        assert!(!err.contains("SRS"), "{err}");
        assert!(!out.exists());
    }
    // Signed group with an unsigned Target refuses rather than ignoring it.
    let unsigned = f.json(
        "unsigned.json",
        &serde_json::to_value(mori_backend::Target::ledger9(
            "local",
            "native-liability-qa",
        ))
        .unwrap(),
    );
    let mut args = proof_args(&f, "prove", &missing);
    args[7] = Box::new(unsigned);
    let err = refused(&run(
        args,
        &[
            &"--signature",
            &sig,
            &"--authority",
            &auth,
            &"--at",
            &"15",
            &"--out",
            &out,
        ],
    ));
    assert!(err.contains("signed Target"), "{err}");
    // Invalid attachment, time and authority refuse before parameter access.
    let mut wrapper = golden()["signed_wrapper"].clone();
    let mut flipped = wrapper["signature_hex"].as_str().unwrap().to_owned();
    flipped.replace_range(130..132, "00");
    wrapper["signature_hex"] = flipped.into();
    let bad = f.json("bad.json", &wrapper);
    for (s, at, word) in [(&bad, "15", "signature"), (&sig, "21", "authorization")] {
        let err = refused(&run(
            proof_args(&f, "prove", &missing),
            &[
                &"--signature",
                s,
                &"--authority",
                &auth,
                &"--at",
                &at,
                &"--out",
                &out,
            ],
        ));
        assert!(err.contains(word) && !err.contains("SRS"), "{err}");
        assert!(!out.exists());
    }
    // Existing destinations are preserved.
    std::fs::write(&out, b"original user content").unwrap();
    let err = refused(&run(
        proof_args(&f, "prove", &missing),
        &[
            &"--signature",
            &sig,
            &"--authority",
            &auth,
            &"--at",
            &"15",
            &"--out",
            &out,
        ],
    ));
    assert!(err.contains("exists"), "{err}");
    assert_eq!(std::fs::read(&out).unwrap(), b"original user content");
}

#[cfg(target_os = "linux")]
#[test]
fn signed_prove_named_pipe_parameters_refuse_without_waiting() {
    use std::{
        process::Stdio,
        time::{Duration, Instant},
    };
    let f = Files::new().golden();
    let params = f.path("pipes");
    std::fs::create_dir(&params).unwrap();
    for k in 1..=19 {
        assert!(
            Command::new("mkfifo")
                .arg(params.join(format!("bls_midnight_2p{k}")))
                .status()
                .unwrap()
                .success()
        );
    }
    let out = f.path("pipe-proof.json");
    let [sig, auth] = signed_group(&f);
    let mut command = Command::new(env!("CARGO_BIN_EXE_mori"));
    command.args(["--color", "never"]);
    for a in proof_args(&f, "prove", &params) {
        command.arg(a.as_ref());
    }
    command
        .arg("--signature")
        .arg(&sig)
        .arg("--authority")
        .arg(&auth)
        .args(["--at", "15", "--out"])
        .arg(&out)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = command.spawn().unwrap();
    let deadline = Instant::now() + Duration::from_secs(20);
    while child.try_wait().unwrap().is_none() {
        if Instant::now() >= deadline {
            child.kill().unwrap();
            let _ = child.wait();
            panic!("named SRS pipe blocked waiting for a writer");
        }
        std::thread::sleep(Duration::from_millis(20));
    }
    let o = child.wait_with_output().unwrap();
    let err = refused(&o);
    assert!(err.contains("regular file"), "{err}");
    assert!(!out.exists());
}

#[test]
fn signed_verify_requires_its_group_and_refuses_stale_bundles() {
    let f = Files::new().golden();
    let missing = f.path("missing-params");
    let [sig, auth] = signed_group(&f);
    let stale = f.json(
        "stale.json",
        &json!({"format":"moriarty-native-transition-proof/2","abi":"moriarty-public-abi/2",
            "relation":"moriarty-zkir-relation/2","binding":"moriarty-native-transition-binding/2",
            "profile":"ledger-9","registry":[],"source_hash":"x","program_hash":"x","target_hash":"x",
            "scenario_hash":"x","execution_hash":"x","action":"repay","circuit_k":9,"proof_hex":"00"}),
    );
    let err = refused(&run(
        proof_args(&f, "verify", &missing),
        &[
            &"--proof",
            &stale,
            &"--signature",
            &sig,
            &"--authority",
            &auth,
            &"--at",
            &"15",
        ],
    ));
    assert!(err.contains("json") || err.contains("recompile"), "{err}");
    assert!(!err.contains("SRS"), "{err}");
    let err = refused(&run(
        proof_args(&f, "verify", &missing),
        &[&"--proof", &stale],
    ));
    assert!(err.contains("--signature, --authority and --at"), "{err}");
    assert!(!err.contains("SRS"), "{err}");
}
