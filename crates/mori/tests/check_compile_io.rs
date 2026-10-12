//! Production check/compile consumers only. No native/proof/parameter APIs.
use mori_backend::{Artifact, Target};
use serde_json::{Value, json};
use std::{
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::atomic::{AtomicUsize, Ordering},
};
static NEXT: AtomicUsize = AtomicUsize::new(0);
const CORE: &str = "language \"moriarty/1\"; agreement CheckCore { state count: UInt = 0; action run() { next count = count + 1; } }";
const PAYMENT: &str = include_str!(
    "../../../deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.mori"
);
struct Files(PathBuf);
impl Files {
    fn new() -> Self {
        let dir = std::env::temp_dir().join(format!(
            "mori-check-compile-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&dir).unwrap();
        Self(dir)
    }
    fn path(&self, n: &str) -> PathBuf {
        self.0.join(n)
    }
    fn write(&self, n: &str, b: impl AsRef<[u8]>) {
        std::fs::write(self.path(n), b).unwrap();
    }
    fn check(&self, n: &str, flags: &[&str]) -> Output {
        let mut c = Command::new(env!("CARGO_BIN_EXE_mori"));
        c.args(["--color", "never", "check"])
            .arg(self.path(n))
            .args(flags);
        invoke(c)
    }
    fn compile_command(&self, out: &Path) -> Command {
        let mut c = Command::new(env!("CARGO_BIN_EXE_mori"));
        c.args(["--color", "never", "compile"])
            .arg(self.path("source.mori"))
            .arg("--target")
            .arg(self.path("target.json"))
            .arg("--out")
            .arg(out);
        c
    }
    fn compile(&self, out: &Path) -> Output {
        invoke(self.compile_command(out))
    }
    fn setup(&self, source: &str, target: &Target) {
        self.write("source.mori", source);
        self.write("target.json", serde_json::to_vec_pretty(target).unwrap());
    }
}
impl Drop for Files {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
// Optional immutable test evidence; never a production option or fault hook.
fn invoke(mut cmd: Command) -> Output {
    let o = cmd.output().unwrap();
    if let Some(root) = std::env::var_os("MORI_IO_CAPTURE_DIR") {
        let dir = PathBuf::from(root).join(format!(
            "cli-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&dir).unwrap();
        let argv: Vec<_> = std::iter::once(cmd.get_program())
            .chain(cmd.get_args())
            .map(|s| s.to_string_lossy().into_owned())
            .collect();
        let mut inputs = vec![];
        for (i, arg) in cmd.get_args().enumerate() {
            let path = Path::new(arg);
            if path.is_file() {
                match std::fs::read(path) {
                    Ok(b) => {
                        let n = format!("input-{i}");
                        std::fs::write(dir.join(&n), &b).unwrap();
                        inputs.push(json!({"path":path,"capture":n,"bytes":b.len(),"sha256":mori_core::intent::hash_bytes(&b)}));
                    }
                    Err(e) => inputs.push(json!({"path":path,"read_error":e.to_string()})),
                }
            }
        }
        std::fs::write(dir.join("stdout"), &o.stdout).unwrap();
        std::fs::write(dir.join("stderr"), &o.stderr).unwrap();
        std::fs::write(dir.join("receipt.json"),serde_json::to_vec_pretty(&json!({"argv":argv,"exit":o.status.code(),"inputs":inputs,"stdout_sha256":mori_core::intent::hash_bytes(&o.stdout),"stderr_sha256":mori_core::intent::hash_bytes(&o.stderr)})).unwrap()).unwrap();
    }
    o
}
fn success(o: &Output) {
    assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
}
fn refused(o: &Output) {
    assert!(!o.status.success());
    assert!(o.stdout.is_empty());
    assert!(!String::from_utf8_lossy(&o.stderr).contains("Compiled"));
}
fn report(o: &Output) -> Value {
    serde_json::from_slice(&o.stdout).unwrap()
}
fn snapshot(s: &str) -> (String, Value) {
    let (header, body) = s.split_once("\n---\n").unwrap();
    let description = header
        .lines()
        .find_map(|l| l.strip_prefix("description: "))
        .unwrap();
    let d: String = serde_json::from_str(description).unwrap();
    (
        d.strip_prefix("Code:\n\n").unwrap().to_owned(),
        serde_json::from_str(body).unwrap(),
    )
}
#[test]
fn check_retains_exact_core_beta_reports_and_profile_dispatch() {
    let f = Files::new();
    f.write("core.mori", CORE);
    let o = f.check("core.mori", &["--json"]);
    success(&o);
    let program = mori_core::parse(CORE).unwrap();
    assert_eq!(
        report(&o),
        json!({"status":"accepted","file":f.path("core.mori"),"source_hash":mori_core::intent::hash_bytes(CORE.as_bytes()),"language":program.language,"agreement":program.name,"actions":program.actions,"diagnostics":[]})
    );
    for snap in [
        include_str!("../src/snapshots/mori__json__tests__accepted_report.snap"),
        include_str!("../src/snapshots/mori__json__tests__rejected_report.snap"),
        include_str!("../src/snapshots/mori__json__tests__unparsable_report.snap"),
    ] {
        let (source, mut expected) = snapshot(snap);
        f.write("beta.mori", source);
        expected["file"] = f.path("beta.mori").to_string_lossy().into_owned().into();
        let o = f.check("beta.mori", &["--json"]);
        assert_eq!(report(&o), expected);
        assert_eq!(o.status.success(), expected["status"] == "accepted");
        assert!(o.stderr.is_empty());
    }
    for source in [
        format!(
            "// comments\n{}",
            CORE.replace("next count = count + 1;", "emit \"bad\\q\"(1);")
        ),
        CORE.replace("moriarty/1", "moriarty/99"),
    ] {
        f.write("core.mori", source);
        let o = f.check("core.mori", &["--json"]);
        assert!(!o.status.success());
        assert_eq!(report(&o)["status"], "rejected");
        assert!(
            report(&o)["diagnostics"][0]["code"]
                .as_str()
                .unwrap()
                .starts_with("mori::core::")
        );
    }
    f.write("beta.mori", "profile \"moriarty-beta/99\"; agreement X {}");
    let o = f.check("beta.mori", &["--json"]);
    assert!(!o.status.success());
    assert!(
        report(&o)["diagnostics"][0]["code"]
            .as_str()
            .unwrap()
            .starts_with("mori::")
    );
}
#[test]
fn check_preserves_grammar_byte_bounds_inside_the_reader_boundary() {
    let f = Files::new();
    let beta = snapshot(include_str!(
        "../src/snapshots/mori__json__tests__accepted_report.snap"
    ))
    .0;
    for source in [CORE, &beta] {
        for n in [65536, 65537, 80000, 1048576] {
            let mut b = source.as_bytes().to_vec();
            b.resize(n, b' ');
            f.write("bound.mori", &b);
            let o = f.check("bound.mori", &["--json"]);
            let v = report(&o);
            assert_eq!(o.status.success(), n == 65536);
            if n == 65536 {
                assert_eq!(v["source_hash"], mori_core::intent::hash_bytes(&b));
            } else {
                assert_eq!(
                    v["diagnostics"][0]["code"],
                    if source == CORE {
                        "mori::core::lex"
                    } else {
                        "mori::lex::source_too_large"
                    }
                );
            }
        }
    }
    let mut b = format!("{CORE}\n// ").into_bytes();
    b.resize(65534, b' ');
    b.extend_from_slice("é".as_bytes());
    f.write("unicode.mori", &b);
    let o = f.check("unicode.mori", &["--json"]);
    success(&o);
    assert_eq!(report(&o)["source_hash"], mori_core::intent::hash_bytes(&b));
    b.extend_from_slice("é".as_bytes());
    f.write("unicode.mori", b);
    assert_eq!(
        report(&f.check("unicode.mori", &["--json"]))["diagnostics"][0]["code"],
        "mori::core::lex"
    );
}
#[test]
fn check_refuses_oversize_before_parsing_and_keeps_read_errors() {
    let f = Files::new();
    f.write("large.mori", vec![b' '; 1048577]);
    let o = f.check("large.mori", &["--json"]);
    refused(&o);
    assert!(String::from_utf8_lossy(&o.stderr).contains("input file exceeds 1 MiB"));
    for (name, bytes) in [
        ("invalid.mori", vec![0xff]),
        ("cut.mori", {
            let mut b = vec![b' '; 1048576];
            b.extend_from_slice("é".as_bytes());
            b
        }),
    ] {
        f.write(name, bytes);
        let o = f.check(name, &["--json"]);
        refused(&o);
        assert!(String::from_utf8_lossy(&o.stderr).contains("could not read"));
    }
    for name in ["missing.mori", "."] {
        let o = f.check(name, &["--json"]);
        refused(&o);
        assert!(String::from_utf8_lossy(&o.stderr).contains("could not read"));
    }
}
#[test]
fn check_keeps_display_verbosity_and_small_resource_refusals() {
    let f = Files::new();
    let name = "odd é\u{202e}\n.mori";
    f.write(name, CORE);
    for flags in [
        vec![],
        vec!["--quiet"],
        vec!["--verbose"],
        vec!["--json"],
        vec!["--quiet", "--json"],
        vec!["--verbose", "--json"],
        vec!["--color", "always", "--json"],
    ] {
        let o = f.check(name, &flags);
        success(&o);
        if flags.contains(&"--json") {
            assert_eq!(report(&o)["file"], f.path(name).to_string_lossy().as_ref());
            assert!(o.stderr.is_empty());
            assert!(o.stdout.is_ascii());
        } else if flags.contains(&"--quiet") {
            assert!(o.stderr.is_empty());
        } else {
            let s = String::from_utf8_lossy(&o.stderr);
            assert!(s.contains("Finished bounded core check"));
            assert!(!s.contains('\u{202e}'));
            assert!(s.contains("\\u{202e}"));
        }
    }
    for source in [
        format!("language \"moriarty/1\"; {}", "+ ".repeat(8193)),
        format!(
            "language \"moriarty/1\"; agreement X {{ action run() {{ emit \"{}\"(1); }} }}",
            "x".repeat(1025)
        ),
        format!(
            "language \"moriarty/1\"; agreement X {{ action run() {{ requires {}true{}; }} }}",
            "(".repeat(70),
            ")".repeat(70)
        ),
        format!(
            "language \"moriarty/1\"; agreement X {{ {} }}",
            (0..257)
                .map(|n| format!("state s{n}: UInt = 0;"))
                .collect::<String>()
        ),
    ] {
        f.write("invalid.mori", source);
        let o = f.check("invalid.mori", &["--json"]);
        assert!(!o.status.success());
        assert_eq!(report(&o)["status"], "rejected");
        assert!(!report(&o)["diagnostics"].as_array().unwrap().is_empty());
    }
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(f.path(name), f.path("link.mori")).unwrap();
        success(&f.check("link.mori", &["--json"]));
    }
}
fn expected(source: &str, target: &Target) -> Artifact {
    let p = mori_core::parse(source).unwrap();
    if target.signed_policy.is_some() {
        mori_backend::compile_signed(&p, target).unwrap()
    } else {
        mori_backend::compile(&p, target).unwrap()
    }
}
fn capture_outputs(out: &Path) {
    if let Some(root) = std::env::var_os("MORI_IO_CAPTURE_DIR") {
        let dest = PathBuf::from(root).join(format!(
            "bundle-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&dest).unwrap();
        for e in std::fs::read_dir(out).unwrap() {
            let e = e.unwrap();
            if e.file_type().unwrap().is_file() {
                std::fs::copy(e.path(), dest.join(e.file_name())).unwrap();
            }
        }
    }
}
fn exact_bundle(out: &Path, artifact: &Artifact) {
    capture_outputs(out);
    let bytes = std::fs::read(out.join("artifact.json")).unwrap();
    assert_eq!(bytes, serde_json::to_vec_pretty(artifact).unwrap());
    assert_eq!(bytes.len(), artifact.resources.artifact_bytes as usize);
    assert_ne!(bytes.last(), Some(&b'\n'));
    let typed: Artifact = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(bytes, serde_json::to_vec_pretty(&typed).unwrap());
    assert_eq!(
        std::fs::read_dir(out).unwrap().count(),
        artifact.circuits.len() + 1
    );
    for (i, c) in artifact.circuits.iter().enumerate() {
        assert_eq!(
            std::fs::read(out.join(format!("{i:03}-{}.zkir", c.action))).unwrap(),
            c.zkir.as_bytes()
        );
    }
}
#[test]
fn compile_roundtrips_contrasting_programs_and_target_selected_signed_bytes() {
    let f = Files::new();
    let contrast: Value = serde_json::from_str(include_str!(
        "../../mori-core/tests/fixtures/bound-replay-oracle.json"
    ))
    .unwrap();
    let signed:Target=serde_json::from_str(include_str!("../../../deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.target.json")).unwrap();
    let unsigned = Target::ledger9("local", "compile-é-anchor");
    for (n, source, target) in [
        ("unsigned", PAYMENT, &unsigned),
        ("signed", PAYMENT, &signed),
        (
            "contrast",
            contrast["contrasting"]["source"].as_str().unwrap(),
            &unsigned,
        ),
    ] {
        f.setup(source, target);
        let a = expected(source, target);
        for suffix in [" first é", " repeat"] {
            let out = f.path(&format!("{n}{suffix}"));
            let o = f.compile(&out);
            success(&o);
            assert!(o.stdout.is_empty());
            assert!(
                String::from_utf8_lossy(&o.stderr)
                    .contains(&format!("Compiled {} circuits", a.circuits.len()))
            );
            exact_bundle(&out, &a);
        }
    }
}
#[test]
fn compile_preflight_keeps_destinations_absent_and_input_error_priority() {
    let f = Files::new();
    let base = Target::ledger9("local", "preflight");
    for edit in [0, 1, 2, 3, 4] {
        let mut t = base.clone();
        match edit {
            0 => t.max_instructions = 1,
            1 => t.max_artifact_bytes = 1,
            2 => t.ledger_revision = "stale".into(),
            3 => t.zkir_version = "stale".into(),
            _ => t.network = "foreign".into(),
        };
        f.setup(PAYMENT, &t);
        let out = f.path(&format!("bad-{edit}"));
        refused(&f.compile(&out));
        assert!(!out.exists());
    }
    f.setup(PAYMENT, &base);
    let original = std::fs::read_to_string(f.path("target.json")).unwrap();
    for target in [
        original.replacen("{", "{\"signed_policy\":null,", 1),
        original.replacen("{", "{\"unknown\":1,", 1),
        original.replacen(
            "{",
            "{\"network\":\"local\",\"\\u006eetwork\":\"local\",",
            1,
        ),
    ] {
        f.write("target.json", target);
        let out = f.path("invalid-target");
        refused(&f.compile(&out));
        assert!(!out.exists());
    }
    f.setup("not a program", &base);
    let out = f.path("existing");
    std::fs::create_dir(&out).unwrap();
    std::fs::write(out.join("sentinel"), b"user").unwrap();
    let o = f.compile(&out);
    refused(&o);
    assert!(!String::from_utf8_lossy(&o.stderr).contains("could not write"));
    assert_eq!(std::fs::read(out.join("sentinel")).unwrap(), b"user");
    let mut signed:Target=serde_json::from_str(include_str!("../../../deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.target.json")).unwrap();
    signed.anchor = "preflight".into();
    f.setup(
        &PAYMENT.replace("\"alice\", \"auditor\", 5", "\"mallory\", \"auditor\", 5"),
        &signed,
    );
    let out = f.path("invalid-owner");
    refused(&f.compile(&out));
    assert!(!out.exists());
}
#[test]
fn compile_target_limits_are_checked_at_exact_envelope_and_instruction_boundaries() {
    let f = Files::new();
    let mut t = Target::ledger9("local", "limits");
    let p = mori_core::parse(CORE).unwrap();
    let a = expected(CORE, &t);
    t.max_instructions = a.resources.instructions;
    // Target limits participate in context hashes, which change decimal ZKIR
    // lengths. Find a deterministic exact-cap fixture and its refusing neighbor;
    // merely assigning a previous artifact's byte count is not a fixed point.
    let mut boundary = None;
    'anchors: for salt in 0..16 {
        t.anchor = format!("limits-{salt}");
        t.max_artifact_bytes = 100000;
        let estimate = mori_backend::compile(&p, &t)
            .unwrap()
            .resources
            .artifact_bytes;
        for cap in estimate.saturating_sub(32)..=estimate + 32 {
            t.max_artifact_bytes = cap;
            if let Ok(a) = mori_backend::compile(&p, &t) {
                if a.resources.artifact_bytes == cap {
                    let mut below = t.clone();
                    below.max_artifact_bytes -= 1;
                    if mori_backend::compile(&p, &below).is_err() {
                        boundary = Some((t.clone(), a));
                        break 'anchors;
                    }
                }
            }
        }
    }
    let (mut t, admitted) =
        boundary.expect("an exact manifest-cap fixture with a refusing neighbor");
    assert_eq!(t.max_artifact_bytes, admitted.resources.artifact_bytes);
    assert_eq!(t.max_instructions, admitted.resources.instructions);
    f.setup(CORE, &t);
    let out = f.path("at-limit");
    success(&f.compile(&out));
    exact_bundle(&out, &admitted);
    t.max_artifact_bytes = admitted.resources.artifact_bytes - 1;
    assert_eq!(
        mori_backend::compile(&p, &t).unwrap_err().message,
        "Artifact envelope exceeds target byte limit."
    );
    f.setup(CORE, &t);
    let out = f.path("below-limit");
    let o = f.compile(&out);
    refused(&o);
    assert!(String::from_utf8_lossy(&o.stderr).contains("Artifact envelope exceeds target byte"));
    assert!(!out.exists());
    // Changing the cap also changes context constants and ZKIR byte lengths.
    // This cap exercises the summed-ZKIR refusal rather than its exact threshold.
    t.max_artifact_bytes = admitted.resources.zkir_bytes / 2;
    f.setup(CORE, &t);
    let out = f.path("zkir-limit");
    let o = f.compile(&out);
    refused(&o);
    assert!(!out.exists());
    assert!(String::from_utf8_lossy(&o.stderr).contains("ZKIR byte limit."));
    t.max_artifact_bytes = 16777216;
    t.max_instructions = admitted.resources.instructions - 1;
    f.setup(CORE, &t);
    let out = f.path("instruction-limit");
    refused(&f.compile(&out));
    assert!(!out.exists());
}
#[test]
fn compile_never_reuses_existing_destination_or_creates_missing_parents() {
    let f = Files::new();
    f.setup(CORE, &Target::ledger9("local", "destinations"));
    for n in ["empty", "populated"] {
        let out = f.path(n);
        std::fs::create_dir(&out).unwrap();
        if n == "populated" {
            std::fs::write(out.join("sentinel"), b"user").unwrap();
        }
        refused(&f.compile(&out));
        assert_eq!(
            std::fs::read_dir(&out).unwrap().count(),
            usize::from(n == "populated")
        );
        if n == "populated" {
            assert_eq!(std::fs::read(out.join("sentinel")).unwrap(), b"user");
        }
    }
    for n in ["source.mori", "target.json"] {
        let path = f.path(n);
        let old = std::fs::read(&path).unwrap();
        refused(&f.compile(&path));
        assert_eq!(old, std::fs::read(path).unwrap());
    }
    refused(&f.compile(&f.0));
    refused(&f.compile(&f.path("missing-parent/out")));
    assert!(!f.path("missing-parent").exists());
    refused(&f.compile(&f.path("source.mori/out")));
    #[cfg(unix)]
    for (n, target) in [
        ("directory-link", f.path("empty")),
        ("dangling-link", f.path("absent")),
    ] {
        std::os::unix::fs::symlink(&target, f.path(n)).unwrap();
        refused(&f.compile(&f.path(n)));
        assert_eq!(std::fs::read_link(f.path(n)).unwrap(), target);
    }
}
#[test]
fn compile_concurrent_reservations_have_one_unmixed_complete_winner() {
    for different in [false, true] {
        let a = Files::new();
        let b = Files::new();
        let target = Target::ledger9("local", "race");
        a.setup(CORE, &target);
        let other = if different {
            CORE.replace("CheckCore", "Other")
        } else {
            CORE.into()
        };
        b.setup(&other, &target);
        let out = a.path("shared");
        let barrier = std::sync::Arc::new(std::sync::Barrier::new(2));
        let cmds = [a.compile_command(&out), b.compile_command(&out)];
        let h: Vec<_> = cmds
            .into_iter()
            .map(|cmd| {
                let barrier = barrier.clone();
                std::thread::spawn(move || {
                    barrier.wait();
                    invoke(cmd)
                })
            })
            .collect();
        let o: Vec<_> = h.into_iter().map(|h| h.join().unwrap()).collect();
        assert_eq!(o.iter().filter(|o| o.status.success()).count(), 1);
        let winner = if o[0].status.success() { CORE } else { &other };
        exact_bundle(&out, &expected(winner, &target));
        for o in o.iter().filter(|o| !o.status.success()) {
            refused(o);
        }
    }
}
#[cfg(unix)]
fn compile_file_size_fault(first_complete: bool) {
    let f = Files::new();
    let target = Target::ledger9("local", "write-fault");
    f.setup(CORE, &target);
    let a = expected(CORE, &target);
    let (name, limit) = if first_complete {
        ("manifest-fault", a.circuits[0].zkir.len())
    } else {
        ("circuit-fault", 1)
    };
    {
        let out = f.path(name);
        let binary = f.compile_command(&out);
        let mut c = Command::new("python3");
        c.args(["-c","import resource,signal,os,sys; resource.setrlimit(resource.RLIMIT_FSIZE,(int(sys.argv[1]),int(sys.argv[1]))); signal.signal(signal.SIGXFSZ,signal.SIG_IGN); os.execv(sys.argv[2],sys.argv[2:])"]).arg(limit.to_string()).arg(binary.get_program()).args(binary.get_args());
        let o = invoke(c);
        refused(&o);
        assert!(String::from_utf8_lossy(&o.stderr).contains("could not write"));
        assert!(out.is_dir());
        capture_outputs(&out);
        assert!(
            !out.join("artifact.json").exists(),
            "failed manifest must never be visible as a partial marker"
        );
        let leaf = out.join("000-run.zkir");
        if first_complete {
            assert_eq!(std::fs::read(leaf).unwrap(), a.circuits[0].zkir.as_bytes());
        } else {
            assert!(
                !leaf.exists(),
                "failed circuit must never be a partial final leaf"
            );
        }
        assert_eq!(
            std::fs::read_dir(out).unwrap().count(),
            usize::from(first_complete),
            "normal-owned staging must be cleaned"
        );
    }
}
#[test]
fn compile_keeps_utf8_and_target_admitted_manifests_above_one_mib() {
    let f = Files::new();
    let source = format!(
        "language \"moriarty/1\"; agreement Large {{ state first: UInt = 0; state second: UInt = 0; state third: UInt = 0; {} }}",
        (0..200)
            .map(|i| format!("action a{i}() {{ next first = first + 1; }}"))
            .collect::<String>()
    );
    let target = Target::ledger9("local", "large é artifact");
    let a = expected(&source, &target);
    assert!(
        a.resources.artifact_bytes > 1048576,
        "fixture must cross the unrelated Core exporter bound"
    );
    assert!(a.resources.artifact_bytes < target.max_artifact_bytes);
    let bytes = serde_json::to_vec_pretty(&a).unwrap();
    assert!(!bytes.is_ascii(), "manifest keeps raw UTF-8");
    f.setup(&source, &target);
    let out = f.path("large");
    success(&f.compile(&out));
    exact_bundle(&out, &a);
}
#[cfg(unix)]
#[test]
fn check_and_compile_preserve_unreadable_and_unwritable_path_errors() {
    use std::os::unix::fs::PermissionsExt;
    if Command::new("id").arg("-u").output().unwrap().stdout == b"0\n" {
        return;
    }
    let f = Files::new();
    f.setup(CORE, &Target::ledger9("local", "permissions"));
    let source = f.path("source.mori");
    std::fs::set_permissions(&source, std::fs::Permissions::from_mode(0)).unwrap();
    let o = f.check("source.mori", &["--json"]);
    std::fs::set_permissions(&source, std::fs::Permissions::from_mode(0o600)).unwrap();
    refused(&o);
    assert!(String::from_utf8_lossy(&o.stderr).contains("could not read"));
    let parent = f.path("parent");
    std::fs::create_dir(&parent).unwrap();
    std::fs::set_permissions(&parent, std::fs::Permissions::from_mode(0o500)).unwrap();
    let out = parent.join("out");
    let o = f.compile(&out);
    std::fs::set_permissions(&parent, std::fs::Permissions::from_mode(0o700)).unwrap();
    refused(&o);
    assert!(!out.exists());
    assert!(String::from_utf8_lossy(&o.stderr).contains("could not write"));
}
#[test]
fn check_invalid_hostile_names_keep_safe_json_and_status_conventions() {
    let f = Files::new();
    let name = "bad\u{1b}[2J é\u{202e}\n.mori";
    let (source, mut v) = snapshot(include_str!(
        "../src/snapshots/mori__json__tests__unparsable_report.snap"
    ));
    f.write(name, source);
    v["file"] = f.path(name).to_string_lossy().into_owned().into();
    for color in ["never", "always"] {
        for verbosity in ["--quiet", "--verbose"] {
            let o = f.check(name, &["--color", color, verbosity, "--json"]);
            assert!(!o.status.success());
            assert_eq!(report(&o), v);
            assert!(o.stdout.is_ascii());
            assert!(o.stderr.is_empty());
            let o = f.check(name, &["--color", color, verbosity]);
            assert!(!o.status.success());
            assert!(o.stdout.is_empty());
            let s = String::from_utf8_lossy(&o.stderr);
            assert!(!s.contains('\u{202e}'));
            assert!(!s.contains("\u{1b}[2J"));
        }
    }
}

#[cfg(unix)]
#[test]
fn compile_circuit_write_fault_leaves_no_partial_final_file() {
    compile_file_size_fault(false);
}
#[cfg(unix)]
#[test]
fn compile_manifest_write_fault_leaves_no_partial_completion_marker() {
    compile_file_size_fault(true);
}
