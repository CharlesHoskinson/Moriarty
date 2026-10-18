//! The public starter command and local reference consumers; no native execution.
use serde_json::{Value, json};
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::atomic::{AtomicUsize, Ordering},
};
static NEXT: AtomicUsize = AtomicUsize::new(0);
struct Files(PathBuf);
impl Files {
    fn new() -> Self {
        let p = std::env::temp_dir().join(format!(
            "mori-init-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&p).unwrap();
        Self(p)
    }
    fn path(&self, p: &str) -> PathBuf {
        self.0.join(p)
    }
    fn init(&self, p: &Path, template: Option<&str>) -> Output {
        let mut c = command();
        c.current_dir(&self.0).arg("init").arg(p);
        if let Some(t) = template {
            c.args(["--template", t]);
        }
        invoke(c)
    }
}
impl Drop for Files {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn command() -> Command {
    let mut c = Command::new(env!("CARGO_BIN_EXE_mori"));
    c.args(["--color", "never"]);
    c
}
fn invoke(mut c: Command) -> Output {
    let o = c.output().unwrap();
    if let Some(root) = std::env::var_os("MORI_INIT_CAPTURE_DIR") {
        let d = PathBuf::from(root).join(format!(
            "cli-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&d).unwrap();
        let argv: Vec<_> = std::iter::once(c.get_program())
            .chain(c.get_args())
            .map(|s| s.to_string_lossy().into_owned())
            .collect();
        fs::write(d.join("stdout"), &o.stdout).unwrap();
        fs::write(d.join("stderr"), &o.stderr).unwrap();
        fs::write(
            d.join("receipt.json"),
            serde_json::to_vec_pretty(&json!({"argv":argv,"exit":o.status.code()})).unwrap(),
        )
        .unwrap();
    }
    o
}
fn success(o: &Output) {
    assert!(
        o.status.success(),
        "exit {:?}: {}",
        o.status.code(),
        String::from_utf8_lossy(&o.stderr)
    );
}
fn refusal(o: &Output) {
    assert!(!o.status.success());
    assert_ne!(o.status.code(), Some(101));
    assert!(!String::from_utf8_lossy(&o.stderr).contains("panicked"));
}
fn report(o: &Output) -> Value {
    serde_json::from_slice(&o.stdout).unwrap()
}
fn asset() -> Value {
    json!({"network":"local","issuer":"mori-init-example","reference":"TEST","scale":0})
}
fn expected(amount: &str, alice: &str, bob: &str, debt: Option<&str>) -> (Value, Value) {
    let mut liabilities = vec![];
    let mut effects =
        vec![json!({"kind":"Transfer","asset":asset(),"from":"alice","to":"bob","amount":amount})];
    if let Some(d) = debt {
        liabilities.push(
            json!({"name":"debt","asset":asset(),"debtor":"alice","creditor":"bob","amount":d}),
        );
        effects.push(json!({"kind":"Liability","name":"debt","asset":asset(),"debtor":"alice","creditor":"bob","before":"100","after":d}));
    }
    (
        json!({"state":{},"balances":[{"account":"alice","asset":asset(),"amount":alice},{"account":"bob","asset":asset(),"amount":bob}],"liabilities":liabilities}),
        json!(effects),
    )
}
fn simulate(dir: &Path, action: &str) -> Output {
    let mut c = command();
    c.arg("simulate")
        .arg(dir.join("source.mori"))
        .args(["--action", action])
        .arg("--scenario")
        .arg(dir.join("scenario.json"));
    invoke(c)
}
fn test_project(dir: &Path) -> Output {
    let mut c = command();
    c.arg("test").arg(dir);
    invoke(c)
}
fn project_bytes(dir: &Path) -> Vec<(String, Vec<u8>)> {
    let mut bytes: Vec<_> = fs::read_dir(dir)
        .unwrap()
        .map(|p| {
            let p = p.unwrap();
            (
                p.file_name().to_string_lossy().into_owned(),
                fs::read(p.path()).unwrap(),
            )
        })
        .collect();
    bytes.sort();
    bytes
}
#[test]
fn both_starters_check_simulate_and_test_with_complete_literal_expectations() {
    let f = Files::new();
    for (template, amount, alice, bob, debt) in [
        ("transfer", "25", "75", "25", None),
        ("repay", "30", "70", "30", Some("70")),
    ] {
        let dir = f.path(template);
        success(&f.init(&dir, Some(template)));
        assert_eq!(
            project_bytes(&dir)
                .iter()
                .map(|(n, _)| n.as_str())
                .collect::<Vec<_>>(),
            [
                "README.md",
                "mori-tests.json",
                "scenario.json",
                "source.mori"
            ]
        );
        let mut c = command();
        c.arg("check").arg(dir.join("source.mori")).arg("--json");
        let o = invoke(c);
        success(&o);
        assert_eq!(report(&o)["status"], "accepted");
        let (post, effects) = expected(amount, alice, bob, debt);
        let action = if template == "transfer" {
            "pay"
        } else {
            "repay"
        };
        let o = simulate(&dir, action);
        success(&o);
        let v = report(&o);
        assert_eq!(v["qualification"], "local-reference-execution");
        assert_eq!(v["execution"]["post"], post);
        assert_eq!(v["execution"]["effects"], effects);
        let manifest: Value =
            serde_json::from_slice(&fs::read(dir.join("mori-tests.json")).unwrap()).unwrap();
        assert_eq!(manifest["profile"], "moriarty-tests/1");
        assert_eq!(
            manifest["cases"][0]["expected"],
            json!({"status":"accepted","post":post,"effects":effects})
        );
        let o = test_project(&dir);
        success(&o);
        assert_eq!(report(&o)["status"], "passed");
        assert_eq!(report(&o)["cases"][0]["actual"]["Ok"]["post"], post);
        // Full comparisons in the actual manifest consumer must reject either mismatch.
        for field in ["post", "effects"] {
            let mut bad = manifest.clone();
            if field == "post" {
                bad["cases"][0]["expected"]["post"]["balances"][0]["amount"] = json!("999");
            } else {
                bad["cases"][0]["expected"]["effects"][0]["amount"] = json!("999");
            }
            fs::write(
                dir.join("mori-tests.json"),
                serde_json::to_vec(&bad).unwrap(),
            )
            .unwrap();
            let o = test_project(&dir);
            refusal(&o);
            assert_eq!(report(&o)["status"], "failed");
        }
    }
}
#[test]
fn default_matches_explicit_transfer_and_relative_unicode_names_are_only_paths() {
    let f = Files::new();
    let relative = Path::new("starter space 🧪 \"; injected");
    success(&f.init(relative, None));
    success(&f.init(&f.path("explicit"), Some("transfer")));
    assert_eq!(
        project_bytes(&f.0.join(relative)),
        project_bytes(&f.path("explicit"))
    );
}
#[test]
fn refusal_preserves_existing_destinations_and_does_not_create_parents() {
    let f = Files::new();
    let empty = f.path("empty");
    fs::create_dir(&empty).unwrap();
    refusal(&f.init(&empty, None));
    assert_eq!(fs::read_dir(&empty).unwrap().count(), 0);
    let full = f.path("full");
    fs::create_dir(&full).unwrap();
    fs::write(full.join("source.mori"), b"foreign").unwrap();
    let before = project_bytes(&full);
    refusal(&f.init(&full, None));
    assert_eq!(before, project_bytes(&full));
    let file = f.path("file");
    fs::write(&file, b"untouched").unwrap();
    refusal(&f.init(&file, None));
    assert_eq!(fs::read(&file).unwrap(), b"untouched");
    refusal(&f.init(&file.join("child"), None));
    refusal(&f.init(&f.path("missing/child"), None));
    assert!(!f.path("missing").exists());
    refusal(&f.init(&f.path("unknown"), Some("unknown")));
    assert!(!f.path("unknown").exists());
}
#[cfg(unix)]
#[test]
fn live_dangling_and_directory_symlinks_are_preserved() {
    use std::os::unix::fs::symlink;
    let f = Files::new();
    fs::write(f.path("file"), b"foreign").unwrap();
    fs::create_dir(f.path("dir")).unwrap();
    for (link, target) in [
        ("live", "file"),
        ("dangling", "missing"),
        ("directory", "dir"),
    ] {
        let p = f.path(link);
        symlink(f.path(target), &p).unwrap();
        refusal(&f.init(&p, None));
        assert_eq!(fs::read_link(&p).unwrap(), f.path(target));
    }
    assert_eq!(fs::read(f.path("file")).unwrap(), b"foreign");
    assert_eq!(fs::read_dir(f.path("dir")).unwrap().count(), 0);
    assert!(!f.path("missing").exists());
}
#[test]
fn competing_inits_have_one_complete_winner_without_overwrite() {
    let f = Files::new();
    let p = f.path("race");
    let mut a = command();
    a.arg("init").arg(&p);
    let mut b = command();
    b.arg("init").arg(&p).args(["--template", "repay"]);
    let a = a.spawn().unwrap();
    let b = b.spawn().unwrap();
    let a = a.wait_with_output().unwrap();
    let b = b.wait_with_output().unwrap();
    assert_ne!(a.status.success(), b.status.success());
    assert_ne!(a.status.code(), Some(101));
    assert_ne!(b.status.code(), Some(101));
    success(&test_project(&p));
    assert_eq!(project_bytes(&p).len(), 4);
}
#[test]
fn repayment_full_tombstone_and_overdischarge_preserve_atomic_reference_results() {
    let f = Files::new();
    let dir = f.path("repay");
    success(&f.init(&dir, Some("repay")));
    let path = dir.join("scenario.json");
    let mut s: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    s["inputs"]["amount"]["value"] = json!("100");
    fs::write(&path, serde_json::to_vec(&s).unwrap()).unwrap();
    let o = simulate(&dir, "repay");
    success(&o);
    let (post, effects) = expected("100", "0", "100", Some("0"));
    assert_eq!(report(&o)["execution"]["post"], post);
    assert_eq!(report(&o)["execution"]["effects"], effects);
    s["inputs"]["amount"]["value"] = json!("30");
    s["liabilities"][0]["amount"] = json!("20");
    fs::write(&path, serde_json::to_vec(&s).unwrap()).unwrap();
    let bytes = fs::read(&path).unwrap();
    let o = simulate(&dir, "repay");
    refusal(&o);
    let v = report(&o);
    assert_eq!(v["error"]["code"], "mori::core::liability");
    assert!(v.get("execution").is_none());
    assert_eq!(bytes, fs::read(&path).unwrap());
    let scenario: mori_core::Scenario = serde_json::from_value(s).unwrap();
    let before = scenario.clone();
    let source = fs::read_to_string(dir.join("source.mori")).unwrap();
    let program = mori_core::parse(&source).unwrap();
    assert_eq!(
        mori_core::simulate(&program, "repay", &scenario)
            .unwrap_err()
            .code,
        "mori::core::liability"
    );
    assert_eq!(scenario, before);
}
#[test]
fn transfer_zero_and_unfunded_are_exact_reference_refusals() {
    let f = Files::new();
    let dir = f.path("transfer");
    success(&f.init(&dir, None));
    let path = dir.join("scenario.json");
    let mut s: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    for (amount, code) in [
        ("0", "mori::core::requires"),
        ("101", "mori::core::balance"),
    ] {
        s["inputs"]["amount"]["value"] = json!(amount);
        fs::write(&path, serde_json::to_vec(&s).unwrap()).unwrap();
        let o = simulate(&dir, "pay");
        refusal(&o);
        assert_eq!(report(&o)["error"]["code"], code);
        assert!(report(&o).get("execution").is_none());
    }
}
