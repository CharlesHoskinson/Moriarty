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
        states: p
            .states
            .iter()
            .map(|state| mori_ledger::layout::StateField {
                name: state.name.clone(),
                ty: state.ty,
                key_hex: mori_ledger::codec::native_hex(&mori_ledger::layout::name_key(
                    "state",
                    &state.name,
                ))
                .unwrap(),
            })
            .collect(),
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
        assert_eq!(m["format"], "moriarty-mapped-role-source-static-relation/2");
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

// Frozen nonauthor normal oracle: 33e37dc74dd01bfec329b38eee3adeea462cdb4514e24f0116b75a3081cc1c88.
// Amount7/base3 gives ordered Transfer2, Fee7, Transfer4, Fee2; gross15/fees9.
// This tests static emission provenance, not native financial execution.
const PROVENANCE_SOURCE: &str = r#"language "moriarty/1";
agreement SourceMapOracle {
  state base: UInt = 3;
  asset PAY = { network: "local", issuer: "independent-static", reference: "PAY", scale: 0 };
  action mapped(amount: UInt) {
    transfer(PAY, "alice", "treasury", 2);
    fee(PAY, "alice", "treasury", amount);
    let local = base + 1;
    transfer(PAY, "alice", "treasury", local);
    fee(PAY, "bob", "treasury", 2);
  }
}
"#;

#[test]
fn source_step_field_correspondence_survives_public_export_and_reopen() {
    let fixture = Fixture::new();
    std::fs::write(fixture.path("source.mori"), PROVENANCE_SOURCE).unwrap();
    fixture.write("bindings.json", &bindings(PROVENANCE_SOURCE));
    let output = accepted(invoke(fixture.command(MODE, false, "out")));
    assert!(output.contains(&format!("{QUALIFICATION} 1 actions into")));
    let marker = read(fixture.path("out/static-compile.json"));
    assert_eq!(marker["profile"], PROFILE);
    assert_eq!(marker["qualification"], QUALIFICATION);
    assert_eq!(marker["authorization"], json!({"mode":MODE}));
    assert_eq!(marker["actions"].as_array().unwrap().len(), 1);
    assert_eq!(
        std::fs::read_to_string(fixture.path("out/source.mori")).unwrap(),
        PROVENANCE_SOURCE
    );
    let manifest = read(fixture.path("out/action-000000.manifest.json"));
    assert_eq!(
        manifest["format"],
        "moriarty-mapped-role-source-static-relation/2"
    );
    assert_eq!(
        manifest["qualification"],
        "source-static-mapped-debits-unauthenticated"
    );
    assert_eq!(manifest["action"], "mapped");
    assert_eq!(manifest["input_count"], 1);
    assert_eq!(manifest["compiler_hash"], marker["build"]);
    let roles = read(fixture.path("bindings.json"))["roles"]
        .as_array()
        .unwrap()
        .clone();
    let debit_sources = ["alice", "bob"].map(|name| {
        roles
            .iter()
            .find(|role| role["role"] == name)
            .unwrap()
            .clone()
    });
    assert_eq!(manifest["debit_sources"], json!(debit_sources));
    // Literal source roots and kinds remain separate from emitted field positions.
    assert_eq!(
        manifest["abi"]["actions"][0]["statements"],
        json!([
            {"kind":"Transfer","asset":"PAY","from":"alice","to":"treasury","amount":0},
            {"kind":"Fee","asset":"PAY","from":"alice","to":"treasury","amount":1},
            {"kind":"Let","name":"local","value":4},
            {"kind":"Transfer","asset":"PAY","from":"alice","to":"treasury","amount":5},
            {"kind":"Fee","asset":"PAY","from":"bob","to":"treasury","amount":6}
        ])
    );
    let expected_emissions = json!([
        {
            "statement": 0,
            "expression": 0,
            "instruction_start": 7,
            "instruction_end": 41,
            "group_start": 3,
            "group_end": 16,
            "dynamic_fields": [
                {
                    "group": 8,
                    "offset": 3
                },
                {
                    "group": 11,
                    "offset": 3
                },
                {
                    "group": 13,
                    "offset": 4
                }
            ]
        },
        {
            "statement": 1,
            "expression": 1,
            "instruction_start": 41,
            "instruction_end": 75,
            "group_start": 16,
            "group_end": 29,
            "dynamic_fields": [
                {
                    "group": 21,
                    "offset": 3
                },
                {
                    "group": 24,
                    "offset": 3
                },
                {
                    "group": 26,
                    "offset": 4
                }
            ]
        },
        {
            "statement": 2,
            "expression": 4,
            "instruction_start": 75,
            "instruction_end": 86,
            "group_start": 29,
            "group_end": 32,
            "dynamic_fields": [
                {
                    "group": 31,
                    "offset": 3
                }
            ]
        },
        {
            "statement": 3,
            "expression": 5,
            "instruction_start": 86,
            "instruction_end": 120,
            "group_start": 32,
            "group_end": 45,
            "dynamic_fields": [
                {
                    "group": 37,
                    "offset": 3
                },
                {
                    "group": 40,
                    "offset": 3
                },
                {
                    "group": 42,
                    "offset": 4
                }
            ]
        },
        {
            "statement": 4,
            "expression": 6,
            "instruction_start": 120,
            "instruction_end": 154,
            "group_start": 45,
            "group_end": 58,
            "dynamic_fields": [
                {
                    "group": 50,
                    "offset": 3
                },
                {
                    "group": 53,
                    "offset": 3
                },
                {
                    "group": 55,
                    "offset": 4
                }
            ]
        }
    ]);
    assert_eq!(manifest["statement_emissions"], expected_emissions);
    // These complete origins/widths come from the independent public encoding derivation.
    let expected_groups = json!([
        {
            "field_count": 1,
            "origin": "abi-root"
        },
        {
            "field_count": 1,
            "origin": "abi-root"
        },
        {
            "field_count": 4,
            "origin": "abi-root"
        },
        {
            "field_count": 1,
            "origin": "source-claim"
        },
        {
            "field_count": 4,
            "origin": "source-claim"
        },
        {
            "field_count": 1,
            "origin": "source-claim"
        },
        {
            "field_count": 19,
            "origin": "source-claim"
        },
        {
            "field_count": 1,
            "origin": "source-claim"
        },
        {
            "field_count": 4,
            "origin": "source-claim"
        },
        {
            "field_count": 1,
            "origin": "source-claim"
        },
        {
            "field_count": 18,
            "origin": "source-claim"
        },
        {
            "field_count": 4,
            "origin": "source-claim"
        },
        {
            "field_count": 19,
            "origin": "source-claim"
        },
        {
            "field_count": 5,
            "origin": "source-claim"
        },
        {
            "field_count": 1,
            "origin": "source-claim"
        },
        {
            "field_count": 1,
            "origin": "source-claim"
        },
        {
            "field_count": 1,
            "origin": "source-claim"
        },
        {
            "field_count": 4,
            "origin": "source-claim"
        },
        {
            "field_count": 1,
            "origin": "source-claim"
        },
        {
            "field_count": 19,
            "origin": "source-claim"
        },
        {
            "field_count": 1,
            "origin": "source-claim"
        },
        {
            "field_count": 4,
            "origin": "source-claim"
        },
        {
            "field_count": 1,
            "origin": "source-claim"
        },
        {
            "field_count": 18,
            "origin": "source-claim"
        },
        {
            "field_count": 4,
            "origin": "source-claim"
        },
        {
            "field_count": 19,
            "origin": "source-claim"
        },
        {
            "field_count": 5,
            "origin": "source-claim"
        },
        {
            "field_count": 1,
            "origin": "source-claim"
        },
        {
            "field_count": 1,
            "origin": "source-claim"
        },
        {
            "field_count": 1,
            "origin": "state-read"
        },
        {
            "field_count": 8,
            "origin": "state-read"
        },
        {
            "field_count": 4,
            "origin": "state-opening"
        },
        {
            "field_count": 1,
            "origin": "source-claim"
        },
        {
            "field_count": 4,
            "origin": "source-claim"
        },
        {
            "field_count": 1,
            "origin": "source-claim"
        },
        {
            "field_count": 19,
            "origin": "source-claim"
        },
        {
            "field_count": 1,
            "origin": "source-claim"
        },
        {
            "field_count": 4,
            "origin": "source-claim"
        },
        {
            "field_count": 1,
            "origin": "source-claim"
        },
        {
            "field_count": 18,
            "origin": "source-claim"
        },
        {
            "field_count": 4,
            "origin": "source-claim"
        },
        {
            "field_count": 19,
            "origin": "source-claim"
        },
        {
            "field_count": 5,
            "origin": "source-claim"
        },
        {
            "field_count": 1,
            "origin": "source-claim"
        },
        {
            "field_count": 1,
            "origin": "source-claim"
        },
        {
            "field_count": 1,
            "origin": "source-claim"
        },
        {
            "field_count": 4,
            "origin": "source-claim"
        },
        {
            "field_count": 1,
            "origin": "source-claim"
        },
        {
            "field_count": 19,
            "origin": "source-claim"
        },
        {
            "field_count": 1,
            "origin": "source-claim"
        },
        {
            "field_count": 4,
            "origin": "source-claim"
        },
        {
            "field_count": 1,
            "origin": "source-claim"
        },
        {
            "field_count": 18,
            "origin": "source-claim"
        },
        {
            "field_count": 4,
            "origin": "source-claim"
        },
        {
            "field_count": 19,
            "origin": "source-claim"
        },
        {
            "field_count": 5,
            "origin": "source-claim"
        },
        {
            "field_count": 1,
            "origin": "source-claim"
        },
        {
            "field_count": 1,
            "origin": "source-claim"
        }
    ]);
    assert_eq!(manifest["groups"], expected_groups);
    // Root may provide a separately pinned baseline executable for this
    // optional byte comparison. Its export uses identical public inputs.
    if let Some(binary) = std::env::var_os("MORI_PROVENANCE_BASELINE_BINARY") {
        let template = fixture.command(MODE, false, "baseline-out");
        let mut baseline = Command::new(binary);
        baseline.args(template.get_args());
        accepted(invoke(baseline));
        assert_eq!(
            std::fs::read(fixture.path("out/action-000000.ir.bin")).unwrap(),
            std::fs::read(fixture.path("baseline-out/action-000000.ir.bin")).unwrap(),
            "candidate IR must equal the explicitly supplied baseline export"
        );
    }
    let inspect = accepted(invoke(fixture.command(MODE, true, "out")));
    assert!(inspect.contains("source-static-recompile-admitted 1 actions; identity"));
    assert!(inspect.contains(&format!("; qualification {QUALIFICATION}")));
}

// Opus security regressions. Frozen expectations: OPUS-PUBLIC-EXPECTATIONS.json
// 722e0fdf856f4469e98544011297d94151e463d8bdcbeb3d90f4b4fc29943db3.
// Static export metadata only: no runtime value, debit, authorization or native check.
const MANIFEST: &str = "action-000000.manifest.json";
const FORMAT_2: &str = "\"moriarty-mapped-role-source-static-relation/2\"";
const FORMAT_1: &str = "\"moriarty-mapped-role-source-static-relation/1\"";
struct Export {
    marker: Value,
    manifest: Value,
    ir: Vec<u8>,
    bindings: Value,
}
// Simultaneous edits; each pattern must occur exactly once in the original text.
fn edit(text: &str, edits: &[(&str, &str)]) -> String {
    let mut spans = edits
        .iter()
        .map(|(from, to)| {
            assert_eq!(text.matches(from).count(), 1, "edit pattern {from}");
            (text.find(from).unwrap(), from.len(), *to)
        })
        .collect::<Vec<_>>();
    spans.sort_by_key(|span| span.0);
    let mut out = String::new();
    let mut at = 0;
    for (start, len, to) in spans {
        assert!(start >= at, "overlapping edits");
        out.push_str(&text[at..start]);
        out.push_str(to);
        at = start + len;
    }
    out.push_str(&text[at..]);
    out
}
fn without(value: &Value, keys: &[&str]) -> Value {
    let mut value = value.clone();
    for key in keys {
        assert!(
            value.as_object_mut().unwrap().remove(*key).is_some(),
            "{key}"
        );
    }
    value
}
fn role_rows(f: &Fixture, names: &[&str]) -> Value {
    let roles = read(f.path("bindings.json"))["roles"].clone();
    let roles = roles.as_array().unwrap();
    Value::Array(
        names
            .iter()
            .map(|name| {
                roles
                    .iter()
                    .find(|role| role["role"] == *name)
                    .unwrap()
                    .clone()
            })
            .collect(),
    )
}
// Lawful identity: every variant gets freshly recomputed public bindings.
fn use_inputs(f: &Fixture, source: &str) {
    std::fs::write(f.path("source.mori"), source).unwrap();
    f.write("bindings.json", &bindings(source));
}
fn export(f: &Fixture, source: &str, out: &str) -> Export {
    use_inputs(f, source);
    let output = accepted(invoke(f.command(MODE, false, out)));
    assert!(output.contains(&format!("{QUALIFICATION} 1 actions into")));
    Export {
        marker: read(f.path(&format!("{out}/static-compile.json"))),
        manifest: read(f.path(&format!("{out}/{MANIFEST}"))),
        ir: std::fs::read(f.path(&format!("{out}/action-000000.ir.bin"))).unwrap(),
        bindings: read(f.path("bindings.json")),
    }
}
fn reopens(f: &Fixture, source: &str, out: &str) {
    use_inputs(f, source);
    let inspect = accepted(invoke(f.command(MODE, true, out)));
    assert!(inspect.contains("source-static-recompile-admitted 1 actions; identity"));
}

#[test]
fn provenance_security_h2_kind_swap_changes_abi_not_ir() {
    let f = Fixture::new();
    let swapped_source = edit(
        PROVENANCE_SOURCE,
        &[
            (
                "transfer(PAY, \"alice\", \"treasury\", 2);",
                "fee(PAY, \"alice\", \"treasury\", 2);",
            ),
            (
                "fee(PAY, \"alice\", \"treasury\", amount);",
                "transfer(PAY, \"alice\", \"treasury\", amount);",
            ),
        ],
    );
    let base = export(&f, PROVENANCE_SOURCE, "base");
    let swapped = export(&f, &swapped_source, "kind");
    let statements = &base.manifest["abi"]["actions"][0]["statements"];
    assert_eq!(
        (&statements[0]["kind"], &statements[1]["kind"]),
        (&json!("Transfer"), &json!("Fee"))
    );
    for key in ["source_hash", "program_hash", "abi_hash"] {
        assert_ne!(swapped.bindings[key], base.bindings[key], "{key}");
    }
    // Transfer/Fee kind never reaches the IR or the emission map.
    assert_eq!(swapped.ir, base.ir, "kind must not change IR bytes");
    assert_eq!(
        swapped.marker["actions"][0]["ir_hash"],
        base.marker["actions"][0]["ir_hash"]
    );
    let committed = ["source_hash", "program_hash", "abi"];
    assert_eq!(
        without(&swapped.manifest, &committed),
        without(&base.manifest, &committed)
    );
    for key in committed {
        assert_ne!(swapped.manifest[key], base.manifest[key], "{key}");
    }
    let mut abi = base.manifest["abi"].clone();
    abi["program_hash"] = swapped.bindings["program_hash"].clone();
    abi["actions"][0]["statements"][0]["kind"] = json!("Fee");
    abi["actions"][0]["statements"][1]["kind"] = json!("Transfer");
    assert_eq!(swapped.manifest["abi"], abi);
    assert_eq!(
        swapped.manifest["abi"]["actions"][0]["statements"],
        json!([
            {"kind":"Fee","asset":"PAY","from":"alice","to":"treasury","amount":0},
            {"kind":"Transfer","asset":"PAY","from":"alice","to":"treasury","amount":1},
            {"kind":"Let","name":"local","value":4},
            {"kind":"Transfer","asset":"PAY","from":"alice","to":"treasury","amount":5},
            {"kind":"Fee","asset":"PAY","from":"bob","to":"treasury","amount":6}
        ])
    );
    assert_eq!(
        swapped.manifest["debit_sources"],
        role_rows(&f, &["alice", "bob"])
    );
    assert_ne!(swapped.marker["identity"], base.marker["identity"]);
    assert_eq!(swapped.marker["build"], base.marker["build"]);
    reopens(&f, PROVENANCE_SOURCE, "base");
    reopens(&f, &swapped_source, "kind");
    // Kind is bound only through the committed source: the other kind's inputs refuse.
    let message = refused(invoke(f.command(MODE, true, "base")));
    assert!(
        message.contains("static-compile: marker-identity"),
        "{message}"
    );
}

#[test]
fn provenance_security_h3_payer_change_changes_debit_metadata_not_ir() {
    let f = Fixture::new();
    let narrowed_source = edit(
        PROVENANCE_SOURCE,
        &[(
            "fee(PAY, \"bob\", \"treasury\", 2);",
            "fee(PAY, \"alice\", \"treasury\", 2);",
        )],
    );
    let moved_source = edit(
        PROVENANCE_SOURCE,
        &[(
            "transfer(PAY, \"alice\", \"treasury\", 2);",
            "transfer(PAY, \"bob\", \"treasury\", 2);",
        )],
    );
    let base = export(&f, PROVENANCE_SOURCE, "base");
    let narrowed = export(&f, &narrowed_source, "narrowed");
    let moved = export(&f, &moved_source, "moved");
    assert_eq!(
        base.manifest["debit_sources"],
        role_rows(&f, &["alice", "bob"])
    );
    // P1 shrinks the payer set; P2 reattributes a step with the set unchanged.
    for (variant, step, payer, debits, changed) in [
        (
            &narrowed,
            4usize,
            "alice",
            &["alice"][..],
            &["source_hash", "program_hash", "abi", "debit_sources"][..],
        ),
        (
            &moved,
            0,
            "bob",
            &["alice", "bob"][..],
            &["source_hash", "program_hash", "abi"][..],
        ),
    ] {
        assert_ne!(
            base.manifest["abi"]["actions"][0]["statements"][step]["from"],
            json!(payer)
        );
        for key in ["source_hash", "program_hash", "abi_hash"] {
            assert_ne!(variant.bindings[key], base.bindings[key], "{key}");
        }
        assert_eq!(variant.ir, base.ir, "payer must not change IR bytes");
        assert_eq!(
            without(&variant.manifest, changed),
            without(&base.manifest, changed)
        );
        assert_eq!(
            variant.manifest["statement_emissions"],
            base.manifest["statement_emissions"]
        );
        let mut abi = base.manifest["abi"].clone();
        abi["program_hash"] = variant.bindings["program_hash"].clone();
        abi["actions"][0]["statements"][step]["from"] = json!(payer);
        assert_eq!(variant.manifest["abi"], abi);
        assert_eq!(variant.manifest["debit_sources"], role_rows(&f, debits));
        assert_ne!(variant.marker["identity"], base.marker["identity"]);
    }
    // debit_sources is an unordered set: the reattribution leaves it identical.
    assert_eq!(
        moved.manifest["debit_sources"],
        base.manifest["debit_sources"]
    );
    assert_ne!(
        narrowed.manifest["debit_sources"],
        base.manifest["debit_sources"]
    );
    assert_ne!(narrowed.marker["identity"], moved.marker["identity"]);
    reopens(&f, &narrowed_source, "narrowed");
    reopens(&f, &moved_source, "moved");
}

#[test]
fn provenance_security_h4_rehashed_map_tampering_refuses_fresh_source_mismatch() {
    let f = Fixture::new();
    let base = export(&f, PROVENANCE_SOURCE, "out");
    let marker_path = f.path("out/static-compile.json");
    let manifest_path = f.path(&format!("out/{MANIFEST}"));
    let marker_bytes = std::fs::read(&marker_path).unwrap();
    let manifest_bytes = std::fs::read(&manifest_path).unwrap();
    let source_bytes = std::fs::read(f.path("out/source.mori")).unwrap();
    assert_eq!(
        base.marker["leaves"][MANIFEST]["sha256"],
        json!(mori_ledger::codec::hash(&manifest_bytes))
    );
    // Attacker reseal with public sha256 and canonical-JSON identity only.
    let reseal = |bytes: &[u8]| -> Vec<u8> {
        let sha256 = mori_ledger::codec::hash(bytes);
        let mut marker = base.marker.clone();
        marker["leaves"][MANIFEST]["sha256"] = json!(sha256);
        marker["leaves"][MANIFEST]["bytes"] = json!(bytes.len());
        marker["actions"][0]["manifest_hash"] = json!(sha256);
        let mut unsigned = marker.clone();
        unsigned.as_object_mut().unwrap().remove("identity");
        marker["identity"] = json!(mori_ledger::codec::object_hash(&unsigned).unwrap());
        let resealed = serde_json::to_vec_pretty(&marker).unwrap();
        std::fs::write(&manifest_path, bytes).unwrap();
        std::fs::write(&marker_path, &resealed).unwrap();
        resealed
    };
    assert_eq!(
        reseal(manifest_bytes.as_slice()),
        marker_bytes,
        "reseal must reproduce the exported public identity"
    );
    let text = String::from_utf8(manifest_bytes.clone()).unwrap();
    let original: Value = serde_json::from_slice(&manifest_bytes).unwrap();
    let emissions = &original["statement_emissions"];
    for (entry, range) in [(0usize, [7, 41, 3, 16]), (1, [41, 75, 16, 29])] {
        let keys = [
            "instruction_start",
            "instruction_end",
            "group_start",
            "group_end",
        ];
        for (key, value) in keys.into_iter().zip(range) {
            assert_eq!(emissions[entry][key], json!(value), "{entry} {key}");
        }
    }
    assert_eq!(
        emissions[2]["dynamic_fields"],
        json!([{"group": 31, "offset": 3}])
    );

    let range_swap = edit(
        &text,
        &[
            ("\"instruction_start\": 7,", "\"instruction_start\": 41,"),
            ("\"instruction_start\": 41,", "\"instruction_start\": 7,"),
            ("\"instruction_end\": 41,", "\"instruction_end\": 75,"),
            ("\"instruction_end\": 75,", "\"instruction_end\": 41,"),
            ("\"group_start\": 3,", "\"group_start\": 16,"),
            ("\"group_start\": 16,", "\"group_start\": 3,"),
            ("\"group_end\": 16,", "\"group_end\": 29,"),
            ("\"group_end\": 29,", "\"group_end\": 16,"),
        ],
    );
    let mut swapped = original.clone();
    for key in [
        "instruction_start",
        "instruction_end",
        "group_start",
        "group_end",
    ] {
        let first = swapped["statement_emissions"][0][key].clone();
        swapped["statement_emissions"][0][key] = swapped["statement_emissions"][1][key].clone();
        swapped["statement_emissions"][1][key] = first;
    }
    // Text positions are anchored on the Let entry's only replacement.
    let anchor = "\"group\": 31,";
    assert_eq!(text.matches(anchor).count(), 1);
    let at = text.find(anchor).unwrap();
    let offset = at + text[at..].find("\"offset\": 3").unwrap();
    let offset_shift = format!(
        "{}\"offset\": 2{}",
        &text[..offset],
        &text[offset + "\"offset\": 3".len()..]
    );
    let mut shifted = original.clone();
    shifted["statement_emissions"][2]["dynamic_fields"][0]["offset"] = json!(2);
    let open = text[..at].rfind('[').unwrap();
    let close = at + text[at..].find(']').unwrap();
    let field_deleted = format!("{}[]{}", &text[..open], &text[close + 1..]);
    let mut deleted = original.clone();
    deleted["statement_emissions"][2]["dynamic_fields"] = json!([]);
    let member = "\"statement_emissions\"";
    assert_eq!(text.matches(member).count(), 1);
    let comma = text[..text.find(member).unwrap()].rfind(',').unwrap();
    let end = text[..text.rfind('}').unwrap()].trim_end().len();
    let member_missing = format!("{}{}", &text[..comma], &text[end..]);
    let missing = without(&original, &["statement_emissions"]);
    let relabel = edit(&text, &[(FORMAT_2, FORMAT_1)]);
    let mut relabelled = original.clone();
    relabelled["format"] = serde_json::from_str(FORMAT_1).unwrap();
    let downgrade = edit(&member_missing, &[(FORMAT_2, FORMAT_1)]);
    let mut downgraded = missing.clone();
    downgraded["format"] = serde_json::from_str(FORMAT_1).unwrap();

    for (id, tampered, expected) in [
        ("M1-range-swap", range_swap, swapped),
        ("M2-offset-shift", offset_shift, shifted),
        ("M3-dynamic-field-deleted", field_deleted, deleted),
        ("M4-map-member-missing", member_missing, missing),
        ("M5-format-relabel", relabel, relabelled),
        ("M6-downgrade", downgrade, downgraded),
    ] {
        assert_ne!(tampered.as_bytes(), manifest_bytes.as_slice(), "{id}");
        assert_eq!(
            serde_json::from_str::<Value>(&tampered).unwrap(),
            expected,
            "{id}"
        );
        let resealed = reseal(tampered.as_bytes());
        // Recomputed marker and leaf identities pass; only fresh recompilation refuses.
        let message = refused(invoke(f.command(MODE, true, "out")));
        assert!(
            message.contains("static-compile: fresh-source-mismatch"),
            "{id}: {message}"
        );
        assert_eq!(std::fs::read(&manifest_path).unwrap(), tampered.as_bytes());
        assert_eq!(std::fs::read(&marker_path).unwrap(), resealed);
        assert_eq!(
            std::fs::read(f.path("out/action-000000.ir.bin")).unwrap(),
            base.ir
        );
        assert_eq!(
            std::fs::read(f.path("out/source.mori")).unwrap(),
            source_bytes
        );
    }
    std::fs::write(&manifest_path, &manifest_bytes).unwrap();
    std::fs::write(&marker_path, &marker_bytes).unwrap();
    let inspect = accepted(invoke(f.command(MODE, true, "out")));
    assert!(inspect.contains("source-static-recompile-admitted 1 actions; identity"));
}
