//! Public CLI checks for opt-in mapped-role source-static export. No native work.
use serde_json::{Value, json};
use std::{
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::atomic::{AtomicUsize, Ordering},
};
static NEXT: AtomicUsize = AtomicUsize::new(0);
const MODE: &str = "unsigned-mapped-roles";
const PROFILE: &str = "moriarty-mapped-role-source-static-bundle/1";
const QUALIFICATION: &str = "source-static-mapped-roles-unproven-uninstalled-unauthenticated";
const SOURCE: &str = r#"language "moriarty/1";
agreement PublicMappedDraft {
 asset PAY={network:"local",issuer:"independent-static",reference:"PAY",scale:0};
 action first(amount:UInt){requires amount>0;transfer(PAY,"alice","bob",amount);}
 action mixed(amount:UInt){transfer(PAY,"bob","treasury",amount);fee(PAY,"alice","treasury",1);transfer(PAY,"bob","alice",1);}
 action idle(){emit "observation"(true);}
}"#;
fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&std::fs::read(p).unwrap()).unwrap()
}
fn cli() -> Command {
    let mut c = Command::new(env!("CARGO_BIN_EXE_mori"));
    c.args(["--color", "never", "compile-call"]);
    c
}
fn invoke(mut c: Command) -> Output {
    let argv = c
        .get_args()
        .map(|a| a.to_string_lossy().into_owned())
        .collect::<Vec<_>>();
    let o = c.output().unwrap();
    if let Some(d) = std::env::var_os("MORI_FEE_POOL_CAPTURE_DIR") {
        let p = PathBuf::from(d).join(format!(
            "mapped-cli-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir_all(&p).unwrap();
        std::fs::write(
            p.join("argv.json"),
            serde_json::to_vec_pretty(&argv).unwrap(),
        )
        .unwrap();
        std::fs::write(p.join("stdout"), &o.stdout).unwrap();
        std::fs::write(p.join("stderr"), &o.stderr).unwrap();
        std::fs::write(p.join("exit"), o.status.code().unwrap_or(-1).to_string()).unwrap();
    }
    o
}
fn accepted(o: Output) -> String {
    assert!(
        o.status.success(),
        "stdout={} stderr={}",
        String::from_utf8_lossy(&o.stdout),
        String::from_utf8_lossy(&o.stderr)
    );
    format!(
        "{}{}",
        String::from_utf8_lossy(&o.stdout),
        String::from_utf8_lossy(&o.stderr)
    )
}
fn refused(o: Output) -> String {
    assert!(!o.status.success(), "unexpected success");
    format!(
        "{}{}",
        String::from_utf8_lossy(&o.stdout),
        String::from_utf8_lossy(&o.stderr)
    )
}
struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        let p = std::env::temp_dir().join(format!(
            "mori-mapped-cli-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&p).unwrap();
        let f = Self(p);
        std::fs::write(f.path("source.mori"), SOURCE).unwrap();
        f.write("bindings.json", &bindings(SOURCE));
        std::fs::copy(
            root().join("examples/core/static-compile/work-limits.json"),
            f.path("limits.json"),
        )
        .unwrap();
        f
    }
    fn path(&self, n: &str) -> PathBuf {
        self.0.join(n)
    }
    fn write(&self, n: &str, v: &Value) {
        std::fs::write(self.path(n), serde_json::to_vec_pretty(v).unwrap()).unwrap();
    }
    fn command(&self, mode: &str, inspect: bool, destination: &str) -> Command {
        let mut c = cli();
        c.arg(self.path("source.mori"))
            .args(["--all-actions", "--static-bindings"])
            .arg(self.path("bindings.json"))
            .args(["--authorization", mode, "--work-limits"])
            .arg(self.path("limits.json"))
            .arg(if inspect { "--inspect-static" } else { "--out" })
            .arg(self.path(destination));
        c
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
// Recompute public source/ABI commitments; fixed public points are existing fixtures.
fn bindings(source: &str) -> Value {
    let p = mori_core::parse(source).unwrap();
    let mut b: mori_ledger::profile::StaticBindings = serde_json::from_value(read(
        root().join("examples/core/static-compile/financial-duty.bindings.json"),
    ))
    .unwrap();
    b.duties.clear();
    b.source_hash = mori_ledger::codec::hash(source.as_bytes());
    b.program_hash = mori_ledger::codec::object_hash(&p).unwrap();
    b.templates_hash = mori_ledger::layout::templates_hash();
    let abi = mori_ledger::layout::LedgerAbi {
        profile: mori_ledger::profile::ABI.into(),
        language: p.language.clone(),
        semantics: p.semantics.clone(),
        program_hash: b.program_hash.clone(),
        states: vec![],
        actions: p.actions,
        assets: b.assets.clone(),
        roles: b.roles.clone(),
        duties: b.duties.clone(),
        templates_hash: b.templates_hash.clone(),
    };
    b.abi_hash = mori_ledger::codec::object_hash(&abi).unwrap();
    let mut v = serde_json::to_value(b).unwrap();
    v.as_object_mut().unwrap().remove("economic_bounds");
    v
}
#[test]
fn mapped_mode_flag_shape_refuses_before_any_file_read() {
    let base = vec![
        "--all-actions",
        "--static-bindings",
        "missing-bindings",
        "--authorization",
        MODE,
        "--work-limits",
        "missing-limits",
        "--out",
        "missing-out",
    ];
    let mut variants = vec![
        vec!["--authorization", MODE],
        vec![
            "--authorization",
            MODE,
            "--action",
            "first",
            "--deployment",
            "missing-deployment",
            "--out",
            "missing-out",
        ],
    ];
    for flag in [
        "--all-actions",
        "--static-bindings",
        "--authorization",
        "--work-limits",
        "--out",
    ] {
        let mut args = base.clone();
        let i = args.iter().position(|s| *s == flag).unwrap();
        args.remove(i);
        if flag != "--all-actions" {
            args.remove(i);
        }
        variants.push(args);
    }
    for tail in [
        vec!["--signer-policy", "missing-policy"],
        vec!["--action", "first"],
        vec!["--deployment", "missing-deployment"],
        vec!["--inspect-static", "missing-bundle"],
    ] {
        let mut args = base.clone();
        args.extend(tail);
        variants.push(args);
    }
    let mut unknown = base.clone();
    *unknown.iter_mut().find(|s| **s == MODE).unwrap() = "unsigned-mapped-roles-unknown";
    variants.push(unknown);
    for args in variants {
        let mut c = cli();
        c.arg("missing-source").args(args);
        let message = refused(invoke(c));
        assert!(
            !message.contains("file-metadata") && !message.contains("could not read"),
            "shape must precede reads: {message}"
        );
    }
}
#[test]
fn authorization_decoder_accepts_exact_mode_object_only() {
    let exact = r#"{"mode":"unsigned-mapped-roles"}"#;
    let a = mori_ledger::static_compile::parse_authorization(exact).unwrap();
    assert_eq!(serde_json::to_value(a).unwrap(), json!({"mode":MODE}));
    for text in [
        r#"{"mode":"unsigned-mapped-roles","signer_policy":{}}"#,
        r#"{"mode":"unsigned-mapped-roles","extra":0}"#,
        r#"{"mode":"unsigned-mapped-roles","mode":"unsigned"}"#,
        r#"{"mode":"unsigned","mode":"unsigned-mapped-roles"}"#,
        r#"{"mode":null}"#,
        r#"null"#,
        r#"{"mode":"unsigned-mapped-roles","signer_policy":null}"#,
        r#"{"mode":"unsigned-mapped-roles/1"}"#,
    ] {
        assert!(
            mori_ledger::static_compile::parse_authorization(text).is_err(),
            "{text}"
        );
        assert!(
            serde_json::from_str::<mori_ledger::static_compile::StaticAuthorization>(text).is_err(),
            "direct serde {text}"
        );
    }
}
#[test]
fn generic_cli_commits_sorted_from_only_inventory_and_requires_same_explicit_mode() {
    let f = Fixture::new();
    let output = accepted(invoke(f.command(MODE, false, "out")));
    assert!(output.contains(&format!("{QUALIFICATION} 3 actions into")));
    assert!(!output.contains("source-static-unproven-uninstalled"));
    let marker = read(f.path("out/static-compile.json"));
    assert_eq!(marker["profile"], PROFILE);
    assert_eq!(marker["qualification"], QUALIFICATION);
    assert_eq!(marker["authorization"], json!({"mode":MODE}));
    let roles = read(f.path("bindings.json"))["roles"]
        .as_array()
        .unwrap()
        .clone();
    for (i, expected) in [(0, vec!["alice"]), (1, vec!["alice", "bob"]), (2, vec![])] {
        let m = read(f.path(&format!("out/action-{i:06}.manifest.json")));
        assert_eq!(m["format"], "moriarty-mapped-role-source-static-relation/1");
        assert_eq!(
            m["qualification"],
            "source-static-mapped-debits-unauthenticated"
        );
        let rows = expected
            .into_iter()
            .map(|role| roles.iter().find(|r| r["role"] == role).unwrap().clone())
            .collect::<Vec<_>>();
        assert_eq!(m["debit_sources"], json!(rows));
        assert_eq!(m["compiler_hash"], marker["build"]);
        assert!(m.get("signed").is_none());
    }
    let inspect = accepted(invoke(f.command(MODE, true, "out")));
    assert!(inspect.contains("source-static-recompile-admitted 3 actions; identity"));
    assert!(inspect.contains(&format!("; qualification {QUALIFICATION}")));
    let before = std::fs::read(f.path("out/static-compile.json")).unwrap();
    for mode in ["unsigned", "signed-v2", "unsigned-mapped-roles-unknown"] {
        refused(invoke(f.command(mode, true, "out")));
        assert_eq!(
            std::fs::read(f.path("out/static-compile.json")).unwrap(),
            before
        );
    }
    let old = refused(invoke(f.command("unsigned", false, "legacy")));
    assert!(old.contains("source-profile: one-public-payer"));
    assert!(!f.path("legacy").exists());
    // Attacker-recomputed marker identity does not authorize relabelling to the old profile.
    let mut marker = marker;
    marker["profile"] = json!("moriarty-native-call-static-bundle/1");
    marker["qualification"] = json!("source-static-unproven-uninstalled");
    marker.as_object_mut().unwrap().remove("identity");
    let identity = mori_ledger::codec::object_hash(&marker).unwrap();
    marker["identity"] = json!(identity);
    f.write("out/static-compile.json", &marker);
    refused(invoke(f.command(MODE, true, "out")));
}
#[test]
fn cli_debit_role_mutation_needs_fresh_abi_and_changes_public_identity() {
    let f = Fixture::new();
    accepted(invoke(f.command(MODE, false, "original")));
    let original = read(f.path("original/static-compile.json"));
    let changed = SOURCE.replacen(
        "transfer(PAY,\"alice\",\"bob\",amount)",
        "transfer(PAY,\"treasury\",\"bob\",amount)",
        1,
    );
    std::fs::write(f.path("source.mori"), &changed).unwrap();
    let mut stale = read(f.path("bindings.json"));
    stale["source_hash"] = json!(mori_ledger::codec::hash(changed.as_bytes()));
    stale["program_hash"] =
        json!(mori_ledger::codec::object_hash(&mori_core::parse(&changed).unwrap()).unwrap());
    f.write("bindings.json", &stale);
    let message = refused(invoke(f.command(MODE, false, "stale")));
    assert!(message.contains("static-abi-identity"), "{message}");
    assert!(!f.path("stale").exists());
    f.write("bindings.json", &bindings(&changed));
    accepted(invoke(f.command(MODE, false, "changed")));
    let changed_marker = read(f.path("changed/static-compile.json"));
    assert_ne!(changed_marker["identity"], original["identity"]);
    let m = read(f.path("changed/action-000000.manifest.json"));
    assert_eq!(
        m["debit_sources"].as_array().unwrap()[0]["role"],
        "treasury"
    );
    refused(invoke(f.command(MODE, true, "original")));
    accepted(invoke(f.command(MODE, true, "changed")));
}
#[test]
fn supplied_economic_bounds_stay_inert_committed_metadata() {
    let f = Fixture::new();
    accepted(invoke(f.command(MODE, false, "unbounded")));
    let unbounded_identity = read(f.path("unbounded/static-compile.json"))["identity"].clone();
    let mut b = read(f.path("bindings.json"));
    b["economic_bounds"] = json!({"owner":"alice","limits":[{"asset":b["assets"][0]["identity"].clone(),"beneficiary":"bob","gross_cap":"0","fee_cap":"0","net_floor":"18446744073709551615","liability_cap":"0"}]});
    f.write("bindings.json", &b);
    accepted(invoke(f.command(MODE, false, "bounded")));
    let marker = read(f.path("bounded/static-compile.json"));
    assert_eq!(marker["bindings"]["economic_bounds"], b["economic_bounds"]);
    assert_eq!(marker["qualification"], QUALIFICATION);
    assert_ne!(marker["identity"], unbounded_identity);
    for i in 0..3 {
        assert_eq!(
            std::fs::read(f.path(&format!("unbounded/action-{i:06}.ir.bin"))).unwrap(),
            std::fs::read(f.path(&format!("bounded/action-{i:06}.ir.bin"))).unwrap(),
            "economic metadata must not add constraints or authority"
        );
    }
    b["economic_bounds"]["limits"][0]["gross_cap"] = json!("1");
    f.write("bindings.json", &b);
    refused(invoke(f.command(MODE, true, "bounded")));
}
#[test]
fn cli_requires_mapped_debit_and_recipient_roles_without_fallback() {
    for (from, to, code) in [
        ("missing-debit", "bob", "source-debit-role"),
        ("alice", "missing-recipient", "source-recipient-role"),
        ("missing-debit", "missing-recipient", "source-debit-role"),
    ] {
        let f = Fixture::new();
        let source = SOURCE.replacen(
            "transfer(PAY,\"alice\",\"bob\",amount)",
            &format!("transfer(PAY,\"{from}\",\"{to}\",amount)"),
            1,
        );
        std::fs::write(f.path("source.mori"), &source).unwrap();
        f.write("bindings.json", &bindings(&source));
        let message = refused(invoke(f.command(MODE, false, "out")));
        assert!(
            message.contains(&format!("source-profile: {code}")),
            "{message}"
        );
        assert!(!f.path("out").exists());
    }
}
