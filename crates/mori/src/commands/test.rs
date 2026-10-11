use super::core_io;
use crate::shell::Shell;
use serde::Deserialize;
use std::{path::PathBuf, process::ExitCode};

#[derive(Debug, clap::Args)]
pub struct Args {
    pub dir: PathBuf,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Manifest {
    profile: String,
    cases: Vec<Case>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Case {
    name: String,
    source: PathBuf,
    scenario: PathBuf,
    action: String,
    expected: Expected,
}
#[derive(Deserialize)]
#[serde(tag = "status", rename_all = "snake_case", deny_unknown_fields)]
enum Expected {
    Accepted {
        post: mori_core::Snapshot,
        effects: Vec<mori_core::Effect>,
    },
    Rejected {
        code: String,
    },
}

pub fn run(args: Args, shell: &mut Shell) -> super::Result<ExitCode> {
    let manifest: Manifest = core_io::read_json(&args.dir.join("mori-tests.json"))?;
    if manifest.profile != "moriarty-tests/1"
        || manifest.cases.is_empty()
        || manifest.cases.len() > 256
    {
        return Err(core_io::operation(
            "expected a nonempty bounded moriarty-tests/1 manifest",
        ));
    }
    let mut names = std::collections::BTreeSet::new();
    let mut reports = Vec::new();
    let mut failed = 0;
    for case in manifest.cases {
        if case.name.is_empty() || !names.insert(case.name.clone()) {
            return Err(core_io::operation(
                "test case names must be nonempty and unique",
            ));
        }
        let source = core_io::read_source(&args.dir.join(&case.source))?;
        let scenario: mori_core::Scenario = core_io::read_json(&args.dir.join(&case.scenario))?;
        let result = mori_core::parse(&source)
            .and_then(|p| mori_core::simulate(&p, &case.action, &scenario));
        let passed = match (&case.expected, &result) {
            (Expected::Accepted { post, effects }, Ok(actual)) => {
                post == &actual.post && effects == &actual.effects
            }
            (Expected::Rejected { code }, Err(error)) => code == &error.code,
            _ => false,
        };
        if !passed {
            failed += 1;
        }
        reports.push(serde_json::json!({"name":case.name,"passed":passed,"actual":result}));
    }
    shell.status(
        "Tested",
        format!("{} passed; {} failed", reports.len() - failed, failed),
    );
    core_io::print(
        &serde_json::json!({"status":if failed==0{"passed"}else{"failed"},"cases":reports}),
    )?;
    Ok(if failed == 0 {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    })
}
