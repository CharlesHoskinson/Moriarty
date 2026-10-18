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
