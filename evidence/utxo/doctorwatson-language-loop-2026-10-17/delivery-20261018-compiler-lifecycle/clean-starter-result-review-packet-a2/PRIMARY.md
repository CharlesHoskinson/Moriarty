# Fresh finite actual-result review: literal source and observed receipts

Read the complete two current candidate files and the complete RED/GREEN receipts/logs below. Both captured shell streams follow unchanged. Inline JSON is compacted without dropping fields; original bytes remain physical. The subject binds identities. Cite file/function/receipt names and actual case facts in your prose; do not repeat full hashes or reconstruct historical identifiers from memory.

Mechanical literal facts for checking, not owner judgments:
- init::project returns source.mori, scenario.json, mori-tests.json and README.md. Asset identity is TEST with network local, issuer mori-init-example, reference TEST, scale 0.
- init::run uses current_exe(), to_str(), a local format/replace expression and std::fs::create_dir; source contains no quote_arg helper call or create_dir_all. Complete source below is authoritative.
- The Unix regression resolves and canonicalizes sh from host PATH before clearing child PATH. It copies the Rust CLI into a path with spaces, apostrophe and emoji and executes the README. Shell wrappers cli-574969-20 and cli-574969-22 each run three Rust commands.
- GREEN has eight passed, zero failed, zero filtered tests. RED is a historical a1 single-test failure (child shell127/Cargo101); a2 also changes shell resolution and runs the full target. Do not claim identical RED/GREEN test bytes. GREEN-CANDIDATE.json is only an earlier unrun historical preparation record; it is not GREEN's source identity.
- 34 means top-level capture directories, 21 means parsed top-level JSON reports; two shell wrappers each execute three nested Rust commands, and two race inits are direct-spawn children outside invoke captures. There is no cli-574969-13 capture directory. The race stderr is a destination-directory creation refusal; consult the literal stderr rather than inventing a filename error.
- Root's 13 independent matches cover complete post plus ordered effects only. The first freeze supplies finite Transfer25/Repay30 predictions; the extra freeze separately supplies full repayment100 and three ordinary refusals. Do not extend this comparison into independent full Execution/per-step/value/source_hash/error-code/CLI-rendering qualification. Candidate expected() is not independent. The projection below merely selects post/effects from the unchanged independent files. Full basis/limits and actual root comparison remain physical, pinned inputs.
- Existing a2 designated source owner explicitly classified control/backtick README presentation and non-UTF8 path coverage recommendations ADVISORY rather than mandatory. Full unspliced a2 WHOLE objects preserve that designated-owner judgment; transport agent supplies no security judgment.
- Observed runtime is Linux with dash only; no observed Windows/Git Bash qualification. Owned session empty and owned temporary root removed are bounded cleanup observations, not detached-process/syscall exclusion. No SDK/VM/proof/key/wallet/ledger qualification or original gate closure.

Integration remains guarded: this result covers the exact reviewed observed source. Main may advance through PR210 and separate Core2 tests/fixtures. Root alone compares the actual current base and selected source/dependencies and rebases/reviews changed bytes as needed; do not approve a future merged tree by assumption. Root declared result a1 reviews ineligible for source/scope/identity substitutions; originals and comparison tables remain under original-result-reviews. Supply a fresh judgment; do not reuse those votes. No product rerun or new execution authority.

===== source-review-packet/proposal/candidate/crates/mori/src/commands/init.rs (COMPLETE) =====
use std::io::Write;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::ValueEnum;

use crate::error::CliError;
use crate::shell::Shell;
use serde_json::json;

#[derive(Debug, clap::Args)]
pub struct Args {
    /// Directory to create; must not exist
    pub dir: PathBuf,

    /// Starter agreement to generate
    #[arg(long, default_value = "transfer")]
    pub template: Template,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Template {
    Transfer,
    Repay,
}

const TRANSFER: &str = r#"language "moriarty/1";
agreement StarterTransfer {
  asset TEST = { network: "local", issuer: "mori-init-example", reference: "TEST", scale: 0 };
  action pay(amount: UInt) {
    requires amount > 0;
    transfer(TEST, "alice", "bob", amount);
  }
}
"#;
const REPAY: &str = r#"language "moriarty/1";
agreement StarterRepay {
  asset TEST = { network: "local", issuer: "mori-init-example", reference: "TEST", scale: 0 };
  action seed(amount: UInt) { liability("debt", TEST, "alice", "bob", amount); }
  action repay(amount: UInt) {
    requires amount > 0;
    transfer(TEST, "alice", "bob", amount);
    discharge("debt", amount);
  }
}
"#;

/// Prepare all bytes before reserving the destination. Expectations are literal
/// financial examples, never outputs obtained from running the interpreter.
fn project(template: Template, executable: &str) -> super::Result<Vec<(&'static str, Vec<u8>)>> {
    use super::core_io::operation;
    let (source, action, amount, alice, bob) = match template {
        Template::Transfer => (TRANSFER, "pay", "25", "75", "25"),
        Template::Repay => (REPAY, "repay", "30", "70", "30"),
    };
    mori_core::parse(source).map_err(operation)?;
    let asset =
        json!({"network":"local","issuer":"mori-init-example","reference":"TEST","scale":0});
    let mut pre_liabilities = vec![];
    let mut post_liabilities = vec![];
    let mut effects =
        vec![json!({"kind":"Transfer","asset":asset,"from":"alice","to":"bob","amount":amount})];
    if matches!(template, Template::Repay) {
        pre_liabilities.push(
            json!({"name":"debt","asset":asset,"debtor":"alice","creditor":"bob","amount":"100"}),
        );
        post_liabilities.push(
            json!({"name":"debt","asset":asset,"debtor":"alice","creditor":"bob","amount":"70"}),
        );
        effects.push(json!({"kind":"Liability","name":"debt","asset":asset,"debtor":"alice","creditor":"bob","before":"100","after":"70"}));
    }
    let scenario: mori_core::Scenario = serde_json::from_value(json!({
        "state":{}, "inputs":{"amount":{"type":"UInt","value":amount}},
        "balances":[{"account":"alice","asset":asset,"amount":"100"},{"account":"bob","asset":asset,"amount":"0"}],
        "liabilities":pre_liabilities
    })).map_err(operation)?;
    let post: mori_core::Snapshot = serde_json::from_value(json!({
        "state":{},
        "balances":[{"account":"alice","asset":asset,"amount":alice},{"account":"bob","asset":asset,"amount":bob}],
        "liabilities":post_liabilities
    })).map_err(operation)?;
    let effects: Vec<mori_core::Effect> =
        serde_json::from_value(json!(effects)).map_err(operation)?;
    let manifest = json!({
        "profile":"moriarty-tests/1", "cases":[{
            "name":action, "source":"source.mori", "scenario":"scenario.json", "action":action,
            "expected":{"status":"accepted", "post":post, "effects":effects}
        }]
    });
    let scenario = mori_core::json::export(&scenario).map_err(operation)?;
    let manifest = mori_core::json::export(&manifest).map_err(operation)?;
    let readme = format!(
        "# Moriarty starter\n\nThis project uses `language \"moriarty/1\"` and `moriarty-core/1` local reference execution.\nThe TEST asset and accounts are examples. Simulation does not settle a ledger transaction or produce a proof.\n\nThe commands below use the Rust CLI that created this project. If you move that executable, update its absolute path below.\nRun these commands from this directory in a POSIX shell:\n\n```sh\n{executable} check source.mori --json\n{executable} simulate source.mori --action {action} --scenario scenario.json\n{executable} test .\n```\n\nThe manifest checks the complete expected post-state and ordered effects.\n\nInitialization requires a new destination and an existing parent directory.\nAn I/O failure can leave an incomplete project. Choose a new destination before retrying.\n"
    );
    Ok(vec![
        ("source.mori", source.as_bytes().to_vec()),
        ("scenario.json", format!("{scenario}\n").into_bytes()),
        ("mori-tests.json", format!("{manifest}\n").into_bytes()),
        ("README.md", readme.into_bytes()),
    ])
}

pub fn run(args: Args, shell: &mut Shell) -> super::Result<ExitCode> {
    // Keep generated commands on this Rust CLI even when another `mori` is on PATH.
    // Quote the complete absolute executable as one POSIX shell word.
    let executable = std::env::current_exe().map_err(super::core_io::operation)?;
    let executable = executable.to_str().ok_or_else(|| {
        super::core_io::operation("starter commands require a UTF-8 Rust executable path")
    })?;
    let executable = format!("'{}'", executable.replace('\'', "'\\''"));
    let files = project(args.template, &executable)?;
    // create_dir atomically refuses existing files, directories and symlinks.
    // Do not create ancestors or remove this directory after a later failure.
    std::fs::create_dir(&args.dir).map_err(|source| CliError::Write {
        path: args.dir.clone(),
        source,
    })?;
    for (name, bytes) in files {
        let path = args.dir.join(name);
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .map_err(|source| CliError::Write {
                path: path.clone(),
                source,
            })?;
        file.write_all(&bytes)
            .map_err(|source| CliError::Write { path, source })?;
    }
    shell.status("Created", args.dir.display());
    Ok(ExitCode::SUCCESS)
}

===== source-review-packet/proposal/candidate/crates/mori/tests/init.rs (COMPLETE) =====
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

#[cfg(unix)]
#[test]
fn generated_commands_use_the_creating_rust_cli_without_path_lookup() {
    use std::os::unix::fs::PermissionsExt;

    // Resolve the host tool before clearing PATH in the external project children.
    let shell = std::env::var_os("PATH")
        .and_then(|path| {
            std::env::split_paths(&path)
                .map(|directory| directory.join("sh"))
                .find(|candidate| {
                    fs::metadata(candidate).is_ok_and(|metadata| {
                        metadata.is_file() && metadata.permissions().mode() & 0o111 != 0
                    })
                })
        })
        .expect("the test host PATH must supply a POSIX sh");
    let shell = fs::canonicalize(shell).expect("resolve the host shell before changing directory");
    let f = Files::new();
    let tools = f.path("Rust CLI's tools 🧪");
    fs::create_dir(&tools).unwrap();
    let executable = tools.join("mori");
    fs::copy(env!("CARGO_BIN_EXE_mori"), &executable).unwrap();
    for template in ["transfer", "repay"] {
        let dir = f.path(template);
        let mut c = Command::new(&executable);
        c.current_dir(&f.0)
            .env("PATH", "")
            .arg("init")
            .arg(&dir)
            .args(["--template", template]);
        success(&invoke(c));
        let readme = fs::read_to_string(dir.join("README.md")).unwrap();
        let commands = readme
            .split_once("```sh\n")
            .unwrap()
            .1
            .split_once("```")
            .unwrap()
            .0;
        let mut c = Command::new(&shell);
        c.current_dir(&dir)
            .env("PATH", "")
            .args(["-c", &format!("set -e\n{commands}")]);
        let o = invoke(c);
        success(&o);
        let reports: Vec<Value> = serde_json::Deserializer::from_slice(&o.stdout)
            .into_iter()
            .collect::<Result<_, _>>()
            .unwrap();
        assert_eq!(reports.len(), 3);
        assert!(readme.contains("Rust CLI"));
        assert!(readme.contains("moriarty-core/1"));
        assert_eq!(reports[0]["status"], "accepted");
        assert_eq!(reports[0]["language"], "moriarty/1");
        assert_eq!(reports[1]["status"], "accepted");
        assert_eq!(reports[1]["qualification"], "local-reference-execution");
        assert_eq!(reports[1]["execution"]["semantics"], "moriarty-core/1");
        let (post, effects) = if template == "transfer" {
            expected("25", "75", "25", None)
        } else {
            expected("30", "70", "30", Some("70"))
        };
        assert_eq!(reports[1]["execution"]["post"], post);
        assert_eq!(reports[1]["execution"]["effects"], effects);
        assert_eq!(reports[2]["status"], "passed");
        assert_eq!(reports[2]["cases"][0]["actual"]["Ok"]["post"], post);
        assert_eq!(reports[2]["cases"][0]["actual"]["Ok"]["effects"], effects);
        assert_eq!(project_bytes(&dir).len(), 4);
    }
}

===== actual-cycle/red/START.json (COMPLETE) =====
{"startedAt":"2026-10-18T10:01:49.027448+00:00","sourceCommit":"928ee0c3a17395921b2709459ef43e3eca339999","sourceTree":"6aaff904b228f1b29f3c9a3cb6da21c0ea4fe29a","sourceStatus":"","argv":["/usr/bin/timeout","--signal=TERM","--kill-after=5s","90s","/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/cargo","test","--manifest-path","/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017/Cargo.toml","--frozen","-j1","-p","mori","--no-default-features","--test","init","generated_commands_use_the_creating_rust_cli_without_path_lookup","--","--exact","--test-threads=1"],"cwd":"/tmp/doctorwatson-clean-starter-red-c9xg_2km/cwd","childEnvironment":{"PATH":"/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin:/usr/bin:/bin","RUSTC":"/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustc","RUSTDOC":"/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustdoc","CARGO_HOME":"/tmp/doctorwatson-clean-starter-red-c9xg_2km/cargo-home","CARGO_TARGET_DIR":"/tmp/doctorwatson-clean-starter-red-c9xg_2km/target","CARGO_NET_OFFLINE":"true","CARGO_INCREMENTAL":"0","CARGO_CACHE_AUTO_CLEAN_FREQUENCY":"never","GIT_CONFIG_NOSYSTEM":"1","GIT_CONFIG_GLOBAL":"/dev/null","GIT_CONFIG_SYSTEM":"/dev/null","LANG":"C.UTF-8","TMPDIR":"/tmp/doctorwatson-clean-starter-red-c9xg_2km/tmp","MORI_INIT_CAPTURE_DIR":"/home/charl/moriarty-c2-opus55-restored-20261017/clean-starter-check-cycle/red/cli-captures"},"ownedTemporaryRoot":"/tmp/doctorwatson-clean-starter-red-c9xg_2km","sourcePinsVerified":438,"toolPins":[{"path":"/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/cargo","resolved":"/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/cargo","sha256":"e951141cc55a6cd7b9876d187bd30a3720e6086b99413af95e3b8de1cdd72f14"},{"path":"/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustc","resolved":"/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustc","sha256":"f3834d26669b03f6855fa54bd2381443123e860167bc0c7a8dd137e5cf6e5e4f"},{"path":"/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustdoc","resolved":"/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustdoc","sha256":"a1321a60578a6b2e3e0ff428201b8618bbd9a9fbcd475c0d0f151bbf17cb0a9e"},{"path":"/usr/bin/timeout","resolved":"/usr/lib/cargo/bin/coreutils/timeout","sha256":"f84e76ea4da1e0c4e92254bd0d7dccdabe2c8b3098e734f27997dae3ee5d8e02"},{"path":"/bin/sh","resolved":"/usr/bin/dash","sha256":"c626229526bb58ec2d0f585f3c3ae1412e6f973b4353385042d11c38d8426917"}],"absentAncestorConfigs":["/.cargo/config","/.cargo/config.toml","/home/.cargo/config","/home/.cargo/config.toml","/home/charl/.cargo/config","/home/charl/.cargo/config.toml","/home/charl/Moriarty/.cargo/config","/home/charl/Moriarty/.cargo/config.toml","/home/charl/Moriarty/.worktrees/.cargo/config","/home/charl/Moriarty/.worktrees/.cargo/config.toml","/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017/.cargo/config","/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017/.cargo/config.toml","/tmp/.cargo/config","/tmp/.cargo/config.toml","/tmp/doctorwatson-clean-starter-red-c9xg_2km/.cargo/config","/tmp/doctorwatson-clean-starter-red-c9xg_2km/.cargo/config.toml","/tmp/doctorwatson-clean-starter-red-c9xg_2km/cwd/.cargo/config","/tmp/doctorwatson-clean-starter-red-c9xg_2km/cwd/.cargo/config.toml"],"oracleFreezeSha256":"f18bd05450c003aabe8c13aa24094ab06d113b95640a011a256d5dfd9f416ab1","oracleInventorySha256":"4b892ec1bb6ae3d043423f79425b340567fa08f8d420d6521de707119efcb686","scope":"One ordinary SDK-free RED public generated-command regression; existing user source/tooling authorization, no native admission.","expectedFailure":"Public generated README uses bare mori; empty child PATH causes shell exit127 and testfailure.","RAMDiskAndAggregateProcessCaps":null}

===== actual-cycle/red/CHILD.json (COMPLETE) =====
{"pid":562357,"session":562357,"startedAt":"2026-10-18T10:01:49.031736+00:00"}

===== actual-cycle/red/RESULT.json (COMPLETE) =====
{"finishedAt":"2026-10-18T10:02:43.382610+00:00","exitCode":101,"wallSeconds":54.351252503998694,"sourceCommitAfter":"928ee0c3a17395921b2709459ef43e3eca339999","sourceTreeAfter":"6aaff904b228f1b29f3c9a3cb6da21c0ea4fe29a","sourceStatusAfter":"","stdoutSha256":"66bbeabb146ac47e8dea798e4d4c1df54ddf904f06a1fde3d4499280fe362f7c","stderrSha256":"03486ce0f9df979adf74a87451f66643eca579f13252e168c45ad19bc699bf34","scope":"RED expected failed regression; no pass or broader product claim.","attemptConsumed":true}

===== actual-cycle/red/STDOUT.log (COMPLETE) =====

running 1 test
test generated_commands_use_the_creating_rust_cli_without_path_lookup ... FAILED

failures:

---- generated_commands_use_the_creating_rust_cli_without_path_lookup stdout ----

thread 'generated_commands_use_the_creating_rust_cli_without_path_lookup' (564558) panicked at crates/mori/tests/init.rs:67:5:
exit Some(127): /bin/sh: 2: mori: not found

note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    generated_commands_use_the_creating_rust_cli_without_path_lookup

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.05s


===== actual-cycle/red/STDERR.log (COMPLETE) =====
   Compiling proc-macro2 v1.0.107
   Compiling quote v1.0.47
   Compiling unicode-ident v1.0.26
   Compiling syn v3.0.6
   Compiling cfg-if v1.0.5
   Compiling synstructure v0.14.0
   Compiling zerofrom-derive v0.1.8
   Compiling zerofrom v0.1.8
   Compiling yoke-derive v0.8.4
   Compiling typenum v1.20.1
   Compiling stable_deref_trait v1.2.1
   Compiling yoke v0.8.3
   Compiling zerovec-derive v0.11.6
   Compiling zerovec v0.11.8
   Compiling displaydoc v0.2.7
   Compiling zeroize v1.9.1
   Compiling version_check v0.9.5
   Compiling libc v0.2.190
   Compiling writeable v0.6.4
   Compiling generic-array v0.14.9
   Compiling tinystr v0.8.4
   Compiling litemap v0.8.3
   Compiling icu_locale_fallback_data v2.3.0
   Compiling rustix v1.1.5
   Compiling icu_locale_core v2.3.0
   Compiling potential_utf v0.1.6
   Compiling zerotrie v0.2.5
   Compiling linux-raw-sys v0.12.1
   Compiling const-oid v0.9.6
   Compiling icu_segmenter_data v2.3.0
   Compiling bitflags v2.13.2
   Compiling subtle v2.6.1
   Compiling icu_provider v2.3.1
   Compiling utf8_iter v1.0.4
   Compiling icu_collections v2.3.0
   Compiling icu_locale_fallback v2.3.0
   Compiling smallvec v1.16.2
   Compiling icu_segmenter v2.3.0
   Compiling unicode-width v0.2.2
   Compiling is_ci v1.2.0
   Compiling supports-color v3.0.2
   Compiling textwrap v0.16.4
   Compiling terminal_size v0.4.4
   Compiling crypto-common v0.1.6
   Compiling block-buffer v0.10.4
   Compiling getrandom v0.2.17
   Compiling supports-hyperlinks v3.2.0
   Compiling unicode-width v0.1.14
   Compiling owo-colors v4.4.0
   Compiling supports-unicode v3.0.0
   Compiling miette v7.6.0
   Compiling rand_core v0.6.4
   Compiling digest v0.10.7
   Compiling syn v2.0.119
   Compiling soa-rs-derive v1.0.1
   Compiling mori-span v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017/crates/mori-span)
   Compiling der v0.7.10
   Compiling crossbeam-utils v0.8.23
   Compiling memchr v2.8.3
   Compiling mori-diagnostics v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017/crates/mori-diagnostics)
   Compiling soa-rs v1.0.1
   Compiling crossbeam-epoch v0.9.21
   Compiling mori-lexer v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017/crates/mori-lexer)
   Compiling spki v0.7.3
   Compiling ff v0.13.1
   Compiling hybrid-array v0.4.15
   Compiling base16ct v0.2.0
   Compiling serde_core v1.0.229
   Compiling crossbeam-deque v0.8.8
   Compiling sec1 v0.7.3
   Compiling group v0.13.0
   Compiling pkcs8 v0.10.2
   Compiling hmac v0.12.1
   Compiling crypto-bigint v0.5.5
   Compiling once_cell v1.21.4
   Compiling utf8parse v0.2.2
   Compiling zmij v1.0.23
   Compiling anstyle-parse v1.0.0
   Compiling elliptic-curve v0.13.8
   Compiling rfc6979 v0.4.0
   Compiling block-buffer v0.12.1
   Compiling crypto-common v0.2.2
   Compiling mori-ast v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017/crates/mori-ast)
   Compiling aho-corasick v1.1.5
   Compiling signature v2.2.0
   Compiling thiserror v2.0.21
   Compiling getrandom v0.4.3
   Compiling cpufeatures v0.2.17
   Compiling colorchoice v1.0.5
   Compiling serde_json v1.0.151
   Compiling is_terminal_polyfill v1.70.2
   Compiling regex-syntax v0.8.11
   Compiling rayon-core v1.13.0
   Compiling anstyle-query v1.1.5
   Compiling serde v1.0.229
   Compiling anstyle v1.0.14
   Compiling const-oid v0.10.2
   Compiling digest v0.11.3
   Compiling anstream v1.0.0
   Compiling regex-automata v0.4.18
   Compiling sha2 v0.10.9
   Compiling ecdsa v0.16.9
   Compiling bstr v1.13.1
   Compiling serde_derive v1.0.229
   Compiling thiserror-impl v2.0.21
   Compiling log v0.4.34
   Compiling cpufeatures v0.3.1
   Compiling strsim v0.11.1
   Compiling similar v2.7.0
   Compiling itoa v1.0.18
   Compiling heck v0.5.0
   Compiling clap_lex v1.1.1
   Compiling same-file v1.0.6
   Compiling walkdir v2.5.0
   Compiling clap_builder v4.6.7
   Compiling clap_derive v4.6.7
   Compiling sha2 v0.11.0
   Compiling globset v0.4.20
   Compiling k256 v0.13.4
   Compiling mori-parser v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017/crates/mori-parser)
   Compiling fastrand v2.5.0
   Compiling either v1.19.0
   Compiling rayon v1.12.0
   Compiling tempfile v3.27.0
   Compiling mori-fmt v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017/crates/mori-fmt)
   Compiling mori-core v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017/crates/mori-core)
   Compiling ignore v0.4.33
   Compiling clap v4.6.7
   Compiling mori-checker v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017/crates/mori-checker)
   Compiling console v0.16.6
   Compiling insta v1.49.0
   Compiling indoc v2.0.7
   Compiling mori v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017/crates/mori)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 54.28s
     Running tests/init.rs (/tmp/doctorwatson-clean-starter-red-c9xg_2km/target/debug/deps/init-15d503f56e6dac7d)
error: test failed, to rerun pass `-p mori --test init`

===== actual-cycle/red/PROCESS-CLEANUP.json (COMPLETE) =====
{"checkedAt":"2026-10-18T10:02:43.473834+00:00","ownedSession":562357,"members":[],"scope":"Owned session only, not syscall/detacheddescendant exclusion."}

===== actual-cycle/red/CLEANUP.json (COMPLETE) =====
{"finishedAt":"2026-10-18T10:02:43.512182+00:00","path":"/tmp/doctorwatson-clean-starter-red-c9xg_2km","existsAfter":false}

===== actual-cycle/green/START.json (COMPLETE) =====
{"startedAt":"2026-10-18T10:09:39.265420+00:00","sourceCommit":"3827270579a68df23330a360b5c5b8372e63bab3","sourceTree":"44da41db47de27543b4f3e9f919febdd7ad0ec78","sourceStatus":"","argv":["/usr/bin/timeout","--signal=TERM","--kill-after=5s","90s","/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/cargo","test","--manifest-path","/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017/Cargo.toml","--frozen","-j1","-p","mori","--no-default-features","--test","init","--","--test-threads=1"],"cwd":"/tmp/doctorwatson-clean-starter-green-e62lo3t2/cwd","childEnvironment":{"PATH":"/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin:/usr/bin:/bin","RUSTC":"/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustc","RUSTDOC":"/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustdoc","CARGO_HOME":"/tmp/doctorwatson-clean-starter-green-e62lo3t2/cargo-home","CARGO_TARGET_DIR":"/tmp/doctorwatson-clean-starter-green-e62lo3t2/target","CARGO_NET_OFFLINE":"true","CARGO_INCREMENTAL":"0","CARGO_CACHE_AUTO_CLEAN_FREQUENCY":"never","GIT_CONFIG_NOSYSTEM":"1","GIT_CONFIG_GLOBAL":"/dev/null","GIT_CONFIG_SYSTEM":"/dev/null","LANG":"C.UTF-8","TMPDIR":"/tmp/doctorwatson-clean-starter-green-e62lo3t2/tmp","MORI_INIT_CAPTURE_DIR":"/home/charl/moriarty-c2-opus55-restored-20261017/clean-starter-check-cycle/green/cli-captures"},"ownedTemporaryRoot":"/tmp/doctorwatson-clean-starter-green-e62lo3t2","sourcePinsVerified":438,"toolPins":[{"path":"/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/cargo","resolved":"/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/cargo","sha256":"e951141cc55a6cd7b9876d187bd30a3720e6086b99413af95e3b8de1cdd72f14"},{"path":"/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustc","resolved":"/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustc","sha256":"f3834d26669b03f6855fa54bd2381443123e860167bc0c7a8dd137e5cf6e5e4f"},{"path":"/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustdoc","resolved":"/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustdoc","sha256":"a1321a60578a6b2e3e0ff428201b8618bbd9a9fbcd475c0d0f151bbf17cb0a9e"},{"path":"/usr/bin/timeout","resolved":"/usr/lib/cargo/bin/coreutils/timeout","sha256":"f84e76ea4da1e0c4e92254bd0d7dccdabe2c8b3098e734f27997dae3ee5d8e02"},{"path":"/usr/bin/dash","resolved":"/usr/bin/dash","sha256":"c626229526bb58ec2d0f585f3c3ae1412e6f973b4353385042d11c38d8426917"}],"absentAncestorConfigs":["/.cargo/config","/.cargo/config.toml","/home/.cargo/config","/home/.cargo/config.toml","/home/charl/.cargo/config","/home/charl/.cargo/config.toml","/home/charl/Moriarty/.cargo/config","/home/charl/Moriarty/.cargo/config.toml","/home/charl/Moriarty/.worktrees/.cargo/config","/home/charl/Moriarty/.worktrees/.cargo/config.toml","/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017/.cargo/config","/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017/.cargo/config.toml","/tmp/.cargo/config","/tmp/.cargo/config.toml","/tmp/doctorwatson-clean-starter-green-e62lo3t2/.cargo/config","/tmp/doctorwatson-clean-starter-green-e62lo3t2/.cargo/config.toml","/tmp/doctorwatson-clean-starter-green-e62lo3t2/cwd/.cargo/config","/tmp/doctorwatson-clean-starter-green-e62lo3t2/cwd/.cargo/config.toml"],"independentOracles":[{"path":"/home/charl/moriarty-c2-opus55-restored-20261017/clean-devex-fresh-independent-oracles","freezeSha256":"f18bd05450c003aabe8c13aa24094ab06d113b95640a011a256d5dfd9f416ab1","inventorySha256":"4b892ec1bb6ae3d043423f79425b340567fa08f8d420d6521de707119efcb686"},{"path":"/home/charl/moriarty-c2-opus55-restored-20261017/clean-devex-init-extra-independent-oracles","freezeSha256":"dbcc0b8256617d90daa7eb7ec53cef17dd1b6a266c8236e953b14344b9425fe1","inventorySha256":"70efa21ee377b9ef879533796d2d4cb4b87d4f13218cd7823566365e6cec641f"}],"scope":"One ordinary SDK-free full existing init target after reproduced failure and actual production repair; no native/SDK/proof/ledger execution.","priorRed":"928ee0c3/0passed1failed54.351252503998694seconds retained; newsource and broader8testscope justify this separate check.","RAMDiskAndAggregateProcessCaps":null}

===== actual-cycle/green/CHILD.json (COMPLETE) =====
{"pid":572783,"session":572783,"startedAt":"2026-10-18T10:09:39.270289+00:00"}

===== actual-cycle/green/RESULT.json (COMPLETE) =====
{"finishedAt":"2026-10-18T10:10:33.547499+00:00","exitCode":0,"wallSeconds":54.27750513299907,"sourceCommitAfter":"3827270579a68df23330a360b5c5b8372e63bab3","sourceTreeAfter":"44da41db47de27543b4f3e9f919febdd7ad0ec78","sourceStatusAfter":"","stdoutSha256":"72243059307e3eaf0c2e5ae75ec926878f9e3e8b50486ac030c7d288f2cf2a71","stderrSha256":"7d4659da5cecc59c6235c45d174cb2e0f3e0ded07e1db6f8ce73efafec003cfc","scope":"Actual complete8testinit target; stdout defines reached assertions. No broader language/proof/ledger claim.","attemptConsumed":true}

===== actual-cycle/green/STDOUT.log (COMPLETE) =====

running 8 tests
test both_starters_check_simulate_and_test_with_complete_literal_expectations ... ok
test competing_inits_have_one_complete_winner_without_overwrite ... ok
test default_matches_explicit_transfer_and_relative_unicode_names_are_only_paths ... ok
test generated_commands_use_the_creating_rust_cli_without_path_lookup ... ok
test live_dangling_and_directory_symlinks_are_preserved ... ok
test refusal_preserves_existing_destinations_and_does_not_create_parents ... ok
test repayment_full_tombstone_and_overdischarge_preserve_atomic_reference_results ... ok
test transfer_zero_and_unfunded_are_exact_reference_refusals ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.16s


===== actual-cycle/green/STDERR.log (COMPLETE) =====
   Compiling proc-macro2 v1.0.107
   Compiling quote v1.0.47
   Compiling unicode-ident v1.0.26
   Compiling syn v3.0.6
   Compiling cfg-if v1.0.5
   Compiling synstructure v0.14.0
   Compiling zerofrom-derive v0.1.8
   Compiling zerofrom v0.1.8
   Compiling yoke-derive v0.8.4
   Compiling typenum v1.20.1
   Compiling stable_deref_trait v1.2.1
   Compiling yoke v0.8.3
   Compiling zerovec-derive v0.11.6
   Compiling zerovec v0.11.8
   Compiling displaydoc v0.2.7
   Compiling zeroize v1.9.1
   Compiling writeable v0.6.4
   Compiling version_check v0.9.5
   Compiling libc v0.2.190
   Compiling generic-array v0.14.9
   Compiling tinystr v0.8.4
   Compiling litemap v0.8.3
   Compiling icu_locale_fallback_data v2.3.0
   Compiling rustix v1.1.5
   Compiling icu_locale_core v2.3.0
   Compiling potential_utf v0.1.6
   Compiling zerotrie v0.2.5
   Compiling subtle v2.6.1
   Compiling const-oid v0.9.6
   Compiling linux-raw-sys v0.12.1
   Compiling icu_segmenter_data v2.3.0
   Compiling bitflags v2.13.2
   Compiling icu_provider v2.3.1
   Compiling utf8_iter v1.0.4
   Compiling icu_collections v2.3.0
   Compiling icu_locale_fallback v2.3.0
   Compiling smallvec v1.16.2
   Compiling icu_segmenter v2.3.0
   Compiling unicode-width v0.2.2
   Compiling is_ci v1.2.0
   Compiling supports-color v3.0.2
   Compiling textwrap v0.16.4
   Compiling terminal_size v0.4.4
   Compiling block-buffer v0.10.4
   Compiling crypto-common v0.1.6
   Compiling getrandom v0.2.17
   Compiling unicode-width v0.1.14
   Compiling supports-hyperlinks v3.2.0
   Compiling owo-colors v4.4.0
   Compiling supports-unicode v3.0.0
   Compiling miette v7.6.0
   Compiling rand_core v0.6.4
   Compiling digest v0.10.7
   Compiling syn v2.0.119
   Compiling soa-rs-derive v1.0.1
   Compiling mori-span v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017/crates/mori-span)
   Compiling der v0.7.10
   Compiling memchr v2.8.3
   Compiling crossbeam-utils v0.8.23
   Compiling mori-diagnostics v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017/crates/mori-diagnostics)
   Compiling soa-rs v1.0.1
   Compiling crossbeam-epoch v0.9.21
   Compiling mori-lexer v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017/crates/mori-lexer)
   Compiling spki v0.7.3
   Compiling ff v0.13.1
   Compiling hybrid-array v0.4.15
   Compiling serde_core v1.0.229
   Compiling crossbeam-deque v0.8.8
   Compiling base16ct v0.2.0
   Compiling sec1 v0.7.3
   Compiling group v0.13.0
   Compiling pkcs8 v0.10.2
   Compiling hmac v0.12.1
   Compiling crypto-bigint v0.5.5
   Compiling once_cell v1.21.4
   Compiling utf8parse v0.2.2
   Compiling zmij v1.0.23
   Compiling anstyle-parse v1.0.0
   Compiling elliptic-curve v0.13.8
   Compiling rfc6979 v0.4.0
   Compiling crypto-common v0.2.2
   Compiling block-buffer v0.12.1
   Compiling mori-ast v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017/crates/mori-ast)
   Compiling aho-corasick v1.1.5
   Compiling signature v2.2.0
   Compiling cpufeatures v0.2.17
   Compiling rayon-core v1.13.0
   Compiling serde_json v1.0.151
   Compiling anstyle-query v1.1.5
   Compiling is_terminal_polyfill v1.70.2
   Compiling getrandom v0.4.3
   Compiling serde v1.0.229
   Compiling regex-syntax v0.8.11
   Compiling anstyle v1.0.14
   Compiling const-oid v0.10.2
   Compiling thiserror v2.0.21
   Compiling colorchoice v1.0.5
   Compiling anstream v1.0.0
   Compiling digest v0.11.3
   Compiling regex-automata v0.4.18
   Compiling sha2 v0.10.9
   Compiling ecdsa v0.16.9
   Compiling bstr v1.13.1
   Compiling serde_derive v1.0.229
   Compiling thiserror-impl v2.0.21
   Compiling same-file v1.0.6
   Compiling similar v2.7.0
   Compiling log v0.4.34
   Compiling itoa v1.0.18
   Compiling heck v0.5.0
   Compiling clap_lex v1.1.1
   Compiling cpufeatures v0.3.1
   Compiling strsim v0.11.1
   Compiling clap_builder v4.6.7
   Compiling sha2 v0.11.0
   Compiling clap_derive v4.6.7
   Compiling globset v0.4.20
   Compiling walkdir v2.5.0
   Compiling k256 v0.13.4
   Compiling mori-parser v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017/crates/mori-parser)
   Compiling either v1.19.0
   Compiling fastrand v2.5.0
   Compiling tempfile v3.27.0
   Compiling rayon v1.12.0
   Compiling mori-fmt v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017/crates/mori-fmt)
   Compiling mori-core v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017/crates/mori-core)
   Compiling ignore v0.4.33
   Compiling clap v4.6.7
   Compiling mori-checker v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017/crates/mori-checker)
   Compiling console v0.16.6
   Compiling insta v1.49.0
   Compiling indoc v2.0.7
   Compiling mori v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017/crates/mori)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 54.09s
     Running tests/init.rs (/tmp/doctorwatson-clean-starter-green-e62lo3t2/target/debug/deps/init-15d503f56e6dac7d)
     Created /tmp/doctorwatson-clean-starter-green-e62lo3t2/tmp/mori-init-574969-13/race
  × could not write `/tmp/doctorwatson-clean-starter-green-e62lo3t2/tmp/mori-
  │ init-574969-13/race`
  ╰─▶ File exists (os error 17)


===== actual-cycle/green/PROCESS-CLEANUP.json (COMPLETE) =====
{"checkedAt":"2026-10-18T10:10:33.623991+00:00","ownedSession":572783,"members":[],"scope":"Owned session only, not syscall/detached exclusion."}

===== actual-cycle/green/CLEANUP.json (COMPLETE) =====
{"finishedAt":"2026-10-18T10:10:33.670901+00:00","path":"/tmp/doctorwatson-clean-starter-green-e62lo3t2","existsAfter":false}

===== actual-cycle/green/cli-captures/cli-574969-20/stdout (COMPLETE) =====
{
  "actions": [
    {
      "inputs": [
        {
          "name": "amount",
          "ty": "UInt"
        }
      ],
      "name": "pay",
      "statements": [
        {
          "kind": "Requires",
          "predicate": 2
        },
        {
          "amount": 3,
          "asset": "TEST",
          "from": "alice",
          "kind": "Transfer",
          "to": "bob"
        }
      ]
    }
  ],
  "agreement": "StarterTransfer",
  "diagnostics": [],
  "file": "source.mori",
  "language": "moriarty/1",
  "source_hash": "56292f1ef606953462aca5d92cbbcac43bd01c66d537fc6dac84df7c7ce8b1cc",
  "status": "accepted"
}
{
  "execution": {
    "action": "pay",
    "effects": [
      {
        "amount": "25",
        "asset": {
          "issuer": "mori-init-example",
          "network": "local",
          "reference": "TEST",
          "scale": 0
        },
        "from": "alice",
        "kind": "Transfer",
        "to": "bob"
      }
    ],
    "post": {
      "balances": [
        {
          "account": "alice",
          "amount": "75",
          "asset": {
            "issuer": "mori-init-example",
            "network": "local",
            "reference": "TEST",
            "scale": 0
          }
        },
        {
          "account": "bob",
          "amount": "25",
          "asset": {
            "issuer": "mori-init-example",
            "network": "local",
            "reference": "TEST",
            "scale": 0
          }
        }
      ],
      "liabilities": [],
      "state": {}
    },
    "pre": {
      "balances": [
        {
          "account": "alice",
          "amount": "100",
          "asset": {
            "issuer": "mori-init-example",
            "network": "local",
            "reference": "TEST",
            "scale": 0
          }
        },
        {
          "account": "bob",
          "amount": "0",
          "asset": {
            "issuer": "mori-init-example",
            "network": "local",
            "reference": "TEST",
            "scale": 0
          }
        }
      ],
      "liabilities": [],
      "state": {}
    },
    "semantics": "moriarty-core/1",
    "steps": [
      {
        "effects": [],
        "index": 0,
        "snapshot": {
          "balances": [
            {
              "account": "alice",
              "amount": "100",
              "asset": {
                "issuer": "mori-init-example",
                "network": "local",
                "reference": "TEST",
                "scale": 0
              }
            },
            {
              "account": "bob",
              "amount": "0",
              "asset": {
                "issuer": "mori-init-example",
                "network": "local",
                "reference": "TEST",
                "scale": 0
              }
            }
          ],
          "liabilities": [],
          "state": {}
        },
        "statement": {
          "kind": "Requires",
          "predicate": 2
        },
        "value": {
          "type": "Bool",
          "value": true
        }
      },
      {
        "effects": [
          {
            "amount": "25",
            "asset": {
              "issuer": "mori-init-example",
              "network": "local",
              "reference": "TEST",
              "scale": 0
            },
            "from": "alice",
            "kind": "Transfer",
            "to": "bob"
          }
        ],
        "index": 1,
        "snapshot": {
          "balances": [
            {
              "account": "alice",
              "amount": "75",
              "asset": {
                "issuer": "mori-init-example",
                "network": "local",
                "reference": "TEST",
                "scale": 0
              }
            },
            {
              "account": "bob",
              "amount": "25",
              "asset": {
                "issuer": "mori-init-example",
                "network": "local",
                "reference": "TEST",
                "scale": 0
              }
            }
          ],
          "liabilities": [],
          "state": {}
        },
        "statement": {
          "amount": 3,
          "asset": "TEST",
          "from": "alice",
          "kind": "Transfer",
          "to": "bob"
        },
        "value": {
          "type": "UInt",
          "value": "25"
        }
      }
    ]
  },
  "qualification": "local-reference-execution",
  "status": "accepted"
}
{
  "cases": [
    {
      "actual": {
        "Ok": {
          "action": "pay",
          "effects": [
            {
              "amount": "25",
              "asset": {
                "issuer": "mori-init-example",
                "network": "local",
                "reference": "TEST",
                "scale": 0
              },
              "from": "alice",
              "kind": "Transfer",
              "to": "bob"
            }
          ],
          "post": {
            "balances": [
              {
                "account": "alice",
                "amount": "75",
                "asset": {
                  "issuer": "mori-init-example",
                  "network": "local",
                  "reference": "TEST",
                  "scale": 0
                }
              },
              {
                "account": "bob",
                "amount": "25",
                "asset": {
                  "issuer": "mori-init-example",
                  "network": "local",
                  "reference": "TEST",
                  "scale": 0
                }
              }
            ],
            "liabilities": [],
            "state": {}
          },
          "pre": {
            "balances": [
              {
                "account": "alice",
                "amount": "100",
                "asset": {
                  "issuer": "mori-init-example",
                  "network": "local",
                  "reference": "TEST",
                  "scale": 0
                }
              },
              {
                "account": "bob",
                "amount": "0",
                "asset": {
                  "issuer": "mori-init-example",
                  "network": "local",
                  "reference": "TEST",
                  "scale": 0
                }
              }
            ],
            "liabilities": [],
            "state": {}
          },
          "semantics": "moriarty-core/1",
          "steps": [
            {
              "effects": [],
              "index": 0,
              "snapshot": {
                "balances": [
                  {
                    "account": "alice",
                    "amount": "100",
                    "asset": {
                      "issuer": "mori-init-example",
                      "network": "local",
                      "reference": "TEST",
                      "scale": 0
                    }
                  },
                  {
                    "account": "bob",
                    "amount": "0",
                    "asset": {
                      "issuer": "mori-init-example",
                      "network": "local",
                      "reference": "TEST",
                      "scale": 0
                    }
                  }
                ],
                "liabilities": [],
                "state": {}
              },
              "statement": {
                "kind": "Requires",
                "predicate": 2
              },
              "value": {
                "type": "Bool",
                "value": true
              }
            },
            {
              "effects": [
                {
                  "amount": "25",
                  "asset": {
                    "issuer": "mori-init-example",
                    "network": "local",
                    "reference": "TEST",
                    "scale": 0
                  },
                  "from": "alice",
                  "kind": "Transfer",
                  "to": "bob"
                }
              ],
              "index": 1,
              "snapshot": {
                "balances": [
                  {
                    "account": "alice",
                    "amount": "75",
                    "asset": {
                      "issuer": "mori-init-example",
                      "network": "local",
                      "reference": "TEST",
                      "scale": 0
                    }
                  },
                  {
                    "account": "bob",
                    "amount": "25",
                    "asset": {
                      "issuer": "mori-init-example",
                      "network": "local",
                      "reference": "TEST",
                      "scale": 0
                    }
                  }
                ],
                "liabilities": [],
                "state": {}
              },
              "statement": {
                "amount": 3,
                "asset": "TEST",
                "from": "alice",
                "kind": "Transfer",
                "to": "bob"
              },
              "value": {
                "type": "UInt",
                "value": "25"
              }
            }
          ]
        }
      },
      "name": "pay",
      "passed": true
    }
  ],
  "status": "passed"
}

===== actual-cycle/green/cli-captures/cli-574969-22/stdout (COMPLETE) =====
{
  "actions": [
    {
      "inputs": [
        {
          "name": "amount",
          "ty": "UInt"
        }
      ],
      "name": "seed",
      "statements": [
        {
          "amount": 0,
          "asset": "TEST",
          "creditor": "bob",
          "debtor": "alice",
          "kind": "Liability",
          "name": "debt"
        }
      ]
    },
    {
      "inputs": [
        {
          "name": "amount",
          "ty": "UInt"
        }
      ],
      "name": "repay",
      "statements": [
        {
          "kind": "Requires",
          "predicate": 3
        },
        {
          "amount": 4,
          "asset": "TEST",
          "from": "alice",
          "kind": "Transfer",
          "to": "bob"
        },
        {
          "amount": 5,
          "kind": "Discharge",
          "name": "debt"
        }
      ]
    }
  ],
  "agreement": "StarterRepay",
  "diagnostics": [],
  "file": "source.mori",
  "language": "moriarty/1",
  "source_hash": "f619527f31a8d535e9a5708971345e4844752c1dbb5356e326f0a079f9fe37be",
  "status": "accepted"
}
{
  "execution": {
    "action": "repay",
    "effects": [
      {
        "amount": "30",
        "asset": {
          "issuer": "mori-init-example",
          "network": "local",
          "reference": "TEST",
          "scale": 0
        },
        "from": "alice",
        "kind": "Transfer",
        "to": "bob"
      },
      {
        "after": "70",
        "asset": {
          "issuer": "mori-init-example",
          "network": "local",
          "reference": "TEST",
          "scale": 0
        },
        "before": "100",
        "creditor": "bob",
        "debtor": "alice",
        "kind": "Liability",
        "name": "debt"
      }
    ],
    "post": {
      "balances": [
        {
          "account": "alice",
          "amount": "70",
          "asset": {
            "issuer": "mori-init-example",
            "network": "local",
            "reference": "TEST",
            "scale": 0
          }
        },
        {
          "account": "bob",
          "amount": "30",
          "asset": {
            "issuer": "mori-init-example",
            "network": "local",
            "reference": "TEST",
            "scale": 0
          }
        }
      ],
      "liabilities": [
        {
          "amount": "70",
          "asset": {
            "issuer": "mori-init-example",
            "network": "local",
            "reference": "TEST",
            "scale": 0
          },
          "creditor": "bob",
          "debtor": "alice",
          "name": "debt"
        }
      ],
      "state": {}
    },
    "pre": {
      "balances": [
        {
          "account": "alice",
          "amount": "100",
          "asset": {
            "issuer": "mori-init-example",
            "network": "local",
            "reference": "TEST",
            "scale": 0
          }
        },
        {
          "account": "bob",
          "amount": "0",
          "asset": {
            "issuer": "mori-init-example",
            "network": "local",
            "reference": "TEST",
            "scale": 0
          }
        }
      ],
      "liabilities": [
        {
          "amount": "100",
          "asset": {
            "issuer": "mori-init-example",
            "network": "local",
            "reference": "TEST",
            "scale": 0
          },
          "creditor": "bob",
          "debtor": "alice",
          "name": "debt"
        }
      ],
      "state": {}
    },
    "semantics": "moriarty-core/1",
    "steps": [
      {
        "effects": [],
        "index": 0,
        "snapshot": {
          "balances": [
            {
              "account": "alice",
              "amount": "100",
              "asset": {
                "issuer": "mori-init-example",
                "network": "local",
                "reference": "TEST",
                "scale": 0
              }
            },
            {
              "account": "bob",
              "amount": "0",
              "asset": {
                "issuer": "mori-init-example",
                "network": "local",
                "reference": "TEST",
                "scale": 0
              }
            }
          ],
          "liabilities": [
            {
              "amount": "100",
              "asset": {
                "issuer": "mori-init-example",
                "network": "local",
                "reference": "TEST",
                "scale": 0
              },
              "creditor": "bob",
              "debtor": "alice",
              "name": "debt"
            }
          ],
          "state": {}
        },
        "statement": {
          "kind": "Requires",
          "predicate": 3
        },
        "value": {
          "type": "Bool",
          "value": true
        }
      },
      {
        "effects": [
          {
            "amount": "30",
            "asset": {
              "issuer": "mori-init-example",
              "network": "local",
              "reference": "TEST",
              "scale": 0
            },
            "from": "alice",
            "kind": "Transfer",
            "to": "bob"
          }
        ],
        "index": 1,
        "snapshot": {
          "balances": [
            {
              "account": "alice",
              "amount": "70",
              "asset": {
                "issuer": "mori-init-example",
                "network": "local",
                "reference": "TEST",
                "scale": 0
              }
            },
            {
              "account": "bob",
              "amount": "30",
              "asset": {
                "issuer": "mori-init-example",
                "network": "local",
                "reference": "TEST",
                "scale": 0
              }
            }
          ],
          "liabilities": [
            {
              "amount": "100",
              "asset": {
                "issuer": "mori-init-example",
                "network": "local",
                "reference": "TEST",
                "scale": 0
              },
              "creditor": "bob",
              "debtor": "alice",
              "name": "debt"
            }
          ],
          "state": {}
        },
        "statement": {
          "amount": 4,
          "asset": "TEST",
          "from": "alice",
          "kind": "Transfer",
          "to": "bob"
        },
        "value": {
          "type": "UInt",
          "value": "30"
        }
      },
      {
        "effects": [
          {
            "after": "70",
            "asset": {
              "issuer": "mori-init-example",
              "network": "local",
              "reference": "TEST",
              "scale": 0
            },
            "before": "100",
            "creditor": "bob",
            "debtor": "alice",
            "kind": "Liability",
            "name": "debt"
          }
        ],
        "index": 2,
        "snapshot": {
          "balances": [
            {
              "account": "alice",
              "amount": "70",
              "asset": {
                "issuer": "mori-init-example",
                "network": "local",
                "reference": "TEST",
                "scale": 0
              }
            },
            {
              "account": "bob",
              "amount": "30",
              "asset": {
                "issuer": "mori-init-example",
                "network": "local",
                "reference": "TEST",
                "scale": 0
              }
            }
          ],
          "liabilities": [
            {
              "amount": "70",
              "asset": {
                "issuer": "mori-init-example",
                "network": "local",
                "reference": "TEST",
                "scale": 0
              },
              "creditor": "bob",
              "debtor": "alice",
              "name": "debt"
            }
          ],
          "state": {}
        },
        "statement": {
          "amount": 5,
          "kind": "Discharge",
          "name": "debt"
        },
        "value": {
          "type": "UInt",
          "value": "30"
        }
      }
    ]
  },
  "qualification": "local-reference-execution",
  "status": "accepted"
}
{
  "cases": [
    {
      "actual": {
        "Ok": {
          "action": "repay",
          "effects": [
            {
              "amount": "30",
              "asset": {
                "issuer": "mori-init-example",
                "network": "local",
                "reference": "TEST",
                "scale": 0
              },
              "from": "alice",
              "kind": "Transfer",
              "to": "bob"
            },
            {
              "after": "70",
              "asset": {
                "issuer": "mori-init-example",
                "network": "local",
                "reference": "TEST",
                "scale": 0
              },
              "before": "100",
              "creditor": "bob",
              "debtor": "alice",
              "kind": "Liability",
              "name": "debt"
            }
          ],
          "post": {
            "balances": [
              {
                "account": "alice",
                "amount": "70",
                "asset": {
                  "issuer": "mori-init-example",
                  "network": "local",
                  "reference": "TEST",
                  "scale": 0
                }
              },
              {
                "account": "bob",
                "amount": "30",
                "asset": {
                  "issuer": "mori-init-example",
                  "network": "local",
                  "reference": "TEST",
                  "scale": 0
                }
              }
            ],
            "liabilities": [
              {
                "amount": "70",
                "asset": {
                  "issuer": "mori-init-example",
                  "network": "local",
                  "reference": "TEST",
                  "scale": 0
                },
                "creditor": "bob",
                "debtor": "alice",
                "name": "debt"
              }
            ],
            "state": {}
          },
          "pre": {
            "balances": [
              {
                "account": "alice",
                "amount": "100",
                "asset": {
                  "issuer": "mori-init-example",
                  "network": "local",
                  "reference": "TEST",
                  "scale": 0
                }
              },
              {
                "account": "bob",
                "amount": "0",
                "asset": {
                  "issuer": "mori-init-example",
                  "network": "local",
                  "reference": "TEST",
                  "scale": 0
                }
              }
            ],
            "liabilities": [
              {
                "amount": "100",
                "asset": {
                  "issuer": "mori-init-example",
                  "network": "local",
                  "reference": "TEST",
                  "scale": 0
                },
                "creditor": "bob",
                "debtor": "alice",
                "name": "debt"
              }
            ],
            "state": {}
          },
          "semantics": "moriarty-core/1",
          "steps": [
            {
              "effects": [],
              "index": 0,
              "snapshot": {
                "balances": [
                  {
                    "account": "alice",
                    "amount": "100",
                    "asset": {
                      "issuer": "mori-init-example",
                      "network": "local",
                      "reference": "TEST",
                      "scale": 0
                    }
                  },
                  {
                    "account": "bob",
                    "amount": "0",
                    "asset": {
                      "issuer": "mori-init-example",
                      "network": "local",
                      "reference": "TEST",
                      "scale": 0
                    }
                  }
                ],
                "liabilities": [
                  {
                    "amount": "100",
                    "asset": {
                      "issuer": "mori-init-example",
                      "network": "local",
                      "reference": "TEST",
                      "scale": 0
                    },
                    "creditor": "bob",
                    "debtor": "alice",
                    "name": "debt"
                  }
                ],
                "state": {}
              },
              "statement": {
                "kind": "Requires",
                "predicate": 3
              },
              "value": {
                "type": "Bool",
                "value": true
              }
            },
            {
              "effects": [
                {
                  "amount": "30",
                  "asset": {
                    "issuer": "mori-init-example",
                    "network": "local",
                    "reference": "TEST",
                    "scale": 0
                  },
                  "from": "alice",
                  "kind": "Transfer",
                  "to": "bob"
                }
              ],
              "index": 1,
              "snapshot": {
                "balances": [
                  {
                    "account": "alice",
                    "amount": "70",
                    "asset": {
                      "issuer": "mori-init-example",
                      "network": "local",
                      "reference": "TEST",
                      "scale": 0
                    }
                  },
                  {
                    "account": "bob",
                    "amount": "30",
                    "asset": {
                      "issuer": "mori-init-example",
                      "network": "local",
                      "reference": "TEST",
                      "scale": 0
                    }
                  }
                ],
                "liabilities": [
                  {
                    "amount": "100",
                    "asset": {
                      "issuer": "mori-init-example",
                      "network": "local",
                      "reference": "TEST",
                      "scale": 0
                    },
                    "creditor": "bob",
                    "debtor": "alice",
                    "name": "debt"
                  }
                ],
                "state": {}
              },
              "statement": {
                "amount": 4,
                "asset": "TEST",
                "from": "alice",
                "kind": "Transfer",
                "to": "bob"
              },
              "value": {
                "type": "UInt",
                "value": "30"
              }
            },
            {
              "effects": [
                {
                  "after": "70",
                  "asset": {
                    "issuer": "mori-init-example",
                    "network": "local",
                    "reference": "TEST",
                    "scale": 0
                  },
                  "before": "100",
                  "creditor": "bob",
                  "debtor": "alice",
                  "kind": "Liability",
                  "name": "debt"
                }
              ],
              "index": 2,
              "snapshot": {
                "balances": [
                  {
                    "account": "alice",
                    "amount": "70",
                    "asset": {
                      "issuer": "mori-init-example",
                      "network": "local",
                      "reference": "TEST",
                      "scale": 0
                    }
                  },
                  {
                    "account": "bob",
                    "amount": "30",
                    "asset": {
                      "issuer": "mori-init-example",
                      "network": "local",
                      "reference": "TEST",
                      "scale": 0
                    }
                  }
                ],
                "liabilities": [
                  {
                    "amount": "70",
                    "asset": {
                      "issuer": "mori-init-example",
                      "network": "local",
                      "reference": "TEST",
                      "scale": 0
                    },
                    "creditor": "bob",
                    "debtor": "alice",
                    "name": "debt"
                  }
                ],
                "state": {}
              },
              "statement": {
                "amount": 5,
                "kind": "Discharge",
                "name": "debt"
              },
              "value": {
                "type": "UInt",
                "value": "30"
              }
            }
          ]
        }
      },
      "name": "repay",
      "passed": true
    }
  ],
  "status": "passed"
}

===== FROZEN-POST-EFFECTS-PROJECTION.json (COMPLETE) =====
{"status":"Mechanical field selection from independent expected files, not a new oracle and not producer-derived","predicates":{"transfer":{"source":"source-review-packet/finance-freezes/clean-devex-fresh-independent-oracles/transfer.expected.json","sourceSha256":"251272bbdaa23ce600dbac33ffa620755702a6d7092d1ad4ab15001e890dcb61","post":{"state":{},"balances":[{"account":"alice","asset":{"network":"local","issuer":"mori-init-example","reference":"TEST","scale":0},"amount":"75"},{"account":"bob","asset":{"network":"local","issuer":"mori-init-example","reference":"TEST","scale":0},"amount":"25"}],"liabilities":[]},"effects":[{"kind":"Transfer","asset":{"network":"local","issuer":"mori-init-example","reference":"TEST","scale":0},"from":"alice","to":"bob","amount":"25"}]},"repay":{"source":"source-review-packet/finance-freezes/clean-devex-fresh-independent-oracles/repay.expected.json","sourceSha256":"3db3e6035f1046036e05dc977fd92cf05488976eadddd72cb0cbe3de2c741148","post":{"state":{},"balances":[{"account":"alice","asset":{"network":"local","issuer":"mori-init-example","reference":"TEST","scale":0},"amount":"70"},{"account":"bob","asset":{"network":"local","issuer":"mori-init-example","reference":"TEST","scale":0},"amount":"30"}],"liabilities":[{"name":"debt","asset":{"network":"local","issuer":"mori-init-example","reference":"TEST","scale":0},"debtor":"alice","creditor":"bob","amount":"70"}]},"effects":[{"kind":"Transfer","asset":{"network":"local","issuer":"mori-init-example","reference":"TEST","scale":0},"from":"alice","to":"bob","amount":"30"},{"kind":"Liability","name":"debt","asset":{"network":"local","issuer":"mori-init-example","reference":"TEST","scale":0},"debtor":"alice","creditor":"bob","before":"100","after":"70"}]}}}
