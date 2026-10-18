Fresh independent nonauthor HIGH Gemini4Argon a2 SOURCE review. Ordinary NONSECURITY source: review currentfullcandidate publicpath/portableinstructions/source/testscope, actualconsumer/manifestfeatures and literal Rustguideline compliance. Leave securitypath/shell/inputjudgment todesignatedfallback.

Packet/home/charl/moriarty-c2-opus55-restored-20261017/clean-starter-source-review-packet-a2, MANIFESTSHA256 33bbc8e53707fe72ff293e599f851a67c014d0f0f3395c89125741b28f3a8046, combinedcandidatepatchSHA256 53e6f8ebaeff7828b2fe9843b16d54af5cc44a3a5a9995fec6c904564b2ae76f. Read bothfullcurrentchangedfiles inline, exacta1-to-a2delta, fullbaselines/source and actualCoreconsumers/manifests/RUSTGUIDELINES. Fulla1judgments remain physical a1-reviews/, including2preservedsecurityWHOLE objects/helper1complete1boundedpreviewexcluded; do not splice/canonicalize objects ortransfera1approvals. Do not read the other currenta2review.

Root rejected an impliedexception to literal Noabsolutepaths and chose testdelta resolvingPOSIXsh from hostPATHbeforeclearingchildrenPATH; productionbytesunchanged. Freshreviewmust assessactuala2code, notstale/bin/sh. Frozenfinancepackets f18bd054... and dbcc0b825.../allbasis/limitations physicalandhashverified; candidateexpected() is not independent. These are SOURCEreviews, no actualresult supplied;root handles SDK-freeRED/GREEN separately and lateractualresultreview. No executionapproval imposed onordinarySDKfreeconsumer and no tests run byreviewer.

Securitycontext must explicitly say which remaining a1securityrecommendations (presentation controls/backticks/nonUTF8coverage) are mandatoryconditions if any, versus advisoryorunqualifiedscope; a2testdelta does notchangeproduction. If mandatorysourcechangesneeded returnchanges-required, notapprovewithhiddenrequiredrepair. Ordinarycontext defers that judgment. Both contexts assessactualsourcefidelity/portability/RUSTGUIDELINES and finite localmoriarty/1/moriarty-core/1 publicpath. PreserveUnix-onlyregression,hostPATHshprerequisite,POSIXinstruction,independent-oracle/actualresult and broaderU0-U7/M/MC/SP/G/financial/native/targetlimits.

Return ONE complete conventional sixfieldJSON with tokendoctorwatson-clean-starter-source-ordinary-20261018-a2-whole-review, verdictapprove-source/changes-required/abstain, summary<=1200,<=5findings<=750each,<=5limitations<=400each. No speculativeAPIclaims/testpasses/securitywaivers. No Cargo/rustc/rustfmt/Mori/SDK/runtime/model/VM/proof/key/wallet/network/private/config/provider operations or edits; readonlysuppliedpublicsource allowed. Existinghelper420s cap+5sgrace, oneattempt,no automaticrestart/retry. Root aloneintegrates/runs/publishes;Solonlytransport.

# Fresh a2 source review

Exact a2 includes full current2candidatefiles plus a1-to-a2testdelta; production init.rs bytes e7640559... unchanged, test26a616f9.... Root chose literal compliance with RUSTGUIDELINES Noabsolutepaths: test resolves/canonicalizes host sh from testprocessPATH before clearing childPATH, replacing hardcoded /bin/sh. This is not a waiver. No a1approval transfers to thesechangedbytes. Both a1WHOLE exports/PROCESS are preserved physically under a1-reviews/, including allsecurity recommendations.

Financefreezes now physically preserved under finance-freezes/: firstf18bd05450c003aabe8c13aa24094ab06d113b95640a011a256d5dfd9f416ab1;extradbcc0b8256617d90daa7eb7ec53cef17dd1b6a266c8236e953b14344b9425fe1. Hashinventories verified;custodianBASIS/limitations unchanged. Candidate expected() remains candidate-authored, not the independentbasis. No actualRED/GREEN results are included; root handles them separately.

Fresh review must inspect bothfullcurrentfiles, exactdelta, actualpubliccheck/simulate/projecttest and feature/guideline source. Designatedsecurity owns whether remaining a1control/backtick/nonUTF8 recommendations are mandatorychanges or advisorygaps; unchangedproduction does not silently resolve them. Sol only transports thosejudgments. Ordinary source review covers publicpath/portability/testscope withoutsecurityjudgment. No SDK/native/model/VM/proof/ledger or oldgateclosure from this localsource slice.


===== FULL CURRENT CANDIDATE crates/mori/src/commands/init.rs =====
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


===== FULL CURRENT CANDIDATE crates/mori/tests/init.rs =====
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


===== EXACT A1-TO-A2 DELTA =====
--- a/crates/mori/tests/init.rs
+++ b/crates/mori/tests/init.rs
@@ -325,6 +325,21 @@
 #[cfg(unix)]
 #[test]
 fn generated_commands_use_the_creating_rust_cli_without_path_lookup() {
+    use std::os::unix::fs::PermissionsExt;
+
+    // Resolve the host tool before clearing PATH in the external project children.
+    let shell = std::env::var_os("PATH")
+        .and_then(|path| {
+            std::env::split_paths(&path)
+                .map(|directory| directory.join("sh"))
+                .find(|candidate| {
+                    fs::metadata(candidate).is_ok_and(|metadata| {
+                        metadata.is_file() && metadata.permissions().mode() & 0o111 != 0
+                    })
+                })
+        })
+        .expect("the test host PATH must supply a POSIX sh");
+    let shell = fs::canonicalize(shell).expect("resolve the host shell before changing directory");
     let f = Files::new();
     let tools = f.path("Rust CLI's tools 🧪");
     fs::create_dir(&tools).unwrap();
@@ -347,7 +362,7 @@
             .split_once("```")
             .unwrap()
             .0;
-        let mut c = Command::new("/bin/sh");
+        let mut c = Command::new(&shell);
         c.current_dir(&dir)
             .env("PATH", "")
             .args(["-c", &format!("set -e\n{commands}")]);


Source public-main/crates/mori/src/commands/check.rs 21-60; fullfilephysical.
```
21: pub fn run(args: Args, shell: &mut Shell) -> super::Result<ExitCode> {
22:     let name = args.file.display().to_string();
23:     let source = super::core_io::read_source(&args.file)?;
24: 
25:     if !args.json {
26:         shell.status("Checking", &name);
27:     }
28:     let started = Instant::now();
29:     if mori_lexer::first_token_text(&source).is_ok_and(|token| token == "language") {
30:         let staged = mori_core::language(&source).is_ok_and(|language| language == "moriarty/2");
31:         let result = if staged {
32:             mori_core::stage::load(&source).map(|program| program.program().clone())
33:         } else {
34:             mori_core::parse(&source)
35:         };
36:         if args.json {
37:             let report = match &result {
38:                 Ok(program) => {
39:                     serde_json::json!({"status":"accepted", "file":name,"source_hash":mori_core::intent::hash_bytes(source.as_bytes()),"language":program.language,"agreement":program.name,"actions":program.actions,"diagnostics":[]})
40:                 }
41:                 Err(error) => {
42:                     serde_json::json!({"status":"rejected","file":name,"diagnostics":[error]})
43:                 }
44:             };
45:             if staged {
46:                 super::simulate::print_stage_report(report)?;
47:             } else {
48:                 super::core_io::print(&report)?;
49:             }
50:         } else {
51:             match &result {
52:                 Ok(_) => shell.status("Finished", "bounded core check"),
53:                 Err(error) => shell.error(error),
54:             }
55:         }
56:         return Ok(if result.is_ok() {
57:             ExitCode::SUCCESS
58:         } else {
59:             ExitCode::FAILURE
60:         });
```

Source public-main/crates/mori/src/commands/simulate.rs 109-115; fullfilephysical.
```
109:     let source = super::core_io::read_source(&args.file)?;
110:     // Hook: /2 routing. A /2 start keeps every core failure labelled.
111:     if mori_core::language(&source) == Ok(mori_core::stage::LANGUAGE_2) {
112:         return run_stage_start(&args, &source, action, scenario_path);
113:     }
114:     let program = mori_core::parse(&source).map_err(super::core_io::operation)?;
115:     let scenario = super::core_io::read_json(scenario_path)?;
```

Source public-main/crates/mori/src/commands/simulate.rs 136-147; fullfilephysical.
```
136:     match mori_core::simulate(&program, action, &scenario) {
137:         Ok(execution) => {
138:             super::core_io::print(
139:                 &serde_json::json!({"status":"accepted", "qualification":"local-reference-execution", "execution":execution}),
140:             )?;
141:             Ok(ExitCode::SUCCESS)
142:         }
143:         Err(error) => {
144:             super::core_io::print(&serde_json::json!({"status":"rejected", "error":error}))?;
145:             Ok(ExitCode::FAILURE)
146:         }
147:     }
```

Source public-main/crates/mori/src/commands/test.rs 62-89; fullfilephysical.
```
62:         let source = core_io::read_source(&args.dir.join(&case.source))?;
63:         let scenario: mori_core::Scenario = core_io::read_json(&args.dir.join(&case.scenario))?;
64:         let result = mori_core::parse(&source)
65:             .and_then(|p| mori_core::simulate(&p, &case.action, &scenario));
66:         let passed = match (&case.expected, &result) {
67:             (Expected::Accepted { post, effects }, Ok(actual)) => {
68:                 post == &actual.post && effects == &actual.effects
69:             }
70:             (Expected::Rejected { code }, Err(error)) => code == &error.code,
71:             _ => false,
72:         };
73:         if !passed {
74:             failed += 1;
75:         }
76:         reports.push(serde_json::json!({"name":case.name,"passed":passed,"actual":result}));
77:     }
78:     shell.status(
79:         "Tested",
80:         format!("{} passed; {} failed", reports.len() - failed, failed),
81:     );
82:     let report = aggregate_report(failed == 0, reports)?;
83:     println!("{report}");
84:     Ok(if failed == 0 {
85:         ExitCode::SUCCESS
86:     } else {
87:         ExitCode::FAILURE
88:     })
89: }
```

Source public-main/crates/mori/Cargo.toml 10-25; fullfilephysical.
```
10: [features]
11: default = ["native"]
12: native = [
13:     "dep:mori-backend",
14:     "dep:mori-ledger",
15:     "dep:futures",
16:     "dep:rand",
17:     "dep:midnight-ledger",
18:     "dep:midnight-base-crypto",
19:     "dep:midnight-coin-structure",
20:     "dep:midnight-onchain-state",
21:     "dep:midnight-onchain-runtime",
22:     "dep:midnight-storage",
23:     "dep:midnight-transient-crypto",
24:     "dep:midnight-serialize",
25: ]
```

Source public-main/RUST_GUIDELINES.md 55-60; fullfilephysical.
```
55: - Code only tests use lives in test modules. Unadopted prototypes go in
56:   `experiments/` or behind a feature, not in `src/` gated by `#[cfg(test)]`.
57: - Never hash source text to identify an ABI, deployment, key or build. Any
58:   reformat changes it. Hash the produced artifact or a declared version.
59: - No absolute paths. Locate files from `env!("CARGO_MANIFEST_DIR")`.
60: 
```

Source public-main/RUST_GUIDELINES.md 253-258; fullfilephysical.
```
253:   The CLI feature-boundary work has a narrow exception for real binary tests
254:   of public dispatch, feature availability, frozen complete output, process
255:   exits and I/O. This exception does not make a shared evaluator an independent
256:   oracle. Never snapshot `--help`; use clap schema assertions for that contract.
257: - Existing external fixture paths and test-support mounts are retained during
258:   the feature-boundary cut; they are not precedents for new paths or mounts.
```
