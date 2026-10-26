//! `mori simulate --execution-profile sherlock-helper-microstep/2` (suspension
//! design note §3.5). The /1 argument definitions and their frozen conflicts
//! are untouched: an invocation selecting profile /2 is recognized before the
//! ordinary CLI parse and handled by this separate argument set, which admits
//! `--action/--scenario` (financial start), `--resume/--observation` and
//! `--checkpoint-out`.
use std::ffi::OsString;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;
use mori_core::sherlock::helper_machine::shared::PROFILE_MICROSTEP_2;
use mori_core::sherlock_compile_artifact::suspension2::{self as s2, Report};
use serde_json::json;

#[derive(Debug, Parser)]
#[command(
    name = "mori simulate",
    about = "Profile /2 suspension and finance simulation"
)]
struct Args2 {
    /// Source file
    file: PathBuf,
    /// Must be sherlock-helper-microstep/2
    #[arg(long)]
    execution_profile: String,
    /// moriarty-sherlock-draft/2 (pure) or moriarty/2 (financial)
    #[arg(long)]
    source_profile: String,
    /// The draft host config with profile sherlock-helper-microstep/2
    #[arg(long)]
    execution_config: PathBuf,
    /// Explicit target descriptor
    #[arg(long)]
    target: PathBuf,
    /// Pure start: the program's closed entry
    #[arg(long, conflicts_with_all = ["action", "scenario", "resume"])]
    entry: bool,
    /// Financial start: the action to run
    #[arg(long, requires = "scenario", conflicts_with = "resume")]
    action: Option<String>,
    /// Financial start: scenario JSON
    #[arg(long, requires = "action", conflicts_with = "resume")]
    scenario: Option<PathBuf>,
    /// Resume this Waiting checkpoint
    #[arg(long, requires = "observation")]
    resume: Option<PathBuf>,
    /// sherlock-observation/2 JSON (at most 64 KiB)
    #[arg(long, requires = "resume")]
    observation: Option<PathBuf>,
    /// Write the resulting checkpoint (Waiting, Halted, not_ready input bytes)
    #[arg(long)]
    checkpoint_out: Option<PathBuf>,
    /// Financial source phase policy: all-or-nothing/1 (default) or
    /// retain-guaranteed-fees/1; bound at admission
    #[arg(long, value_name = "POLICY")]
    phase_policy: Option<String>,
}

/// True when argv selects the /2 profile on `simulate`.
fn selects_profile2(argv: &[OsString]) -> bool {
    if argv.get(1).and_then(|a| a.to_str()) != Some("simulate") {
        return false;
    }
    let want = format!("--execution-profile={PROFILE_MICROSTEP_2}");
    argv.windows(2).any(|w| {
        w[0].to_str() == Some("--execution-profile") && w[1].to_str() == Some(PROFILE_MICROSTEP_2)
    }) || argv.iter().any(|a| a.to_str() == Some(want.as_str()))
}

/// Handle a /2 `simulate` invocation, or return `None` for every other argv.
pub fn try_run_from_env() -> Option<ExitCode> {
    let argv: Vec<OsString> = std::env::args_os().collect();
    if !selects_profile2(&argv) {
        return None;
    }
    let mut rest = vec![OsString::from("mori simulate")];
    rest.extend(argv.into_iter().skip(2));
    let args = match Args2::try_parse_from(rest) {
        Ok(args) => args,
        Err(e) => e.exit(),
    };
    Some(run(&args))
}

fn input_failure(message: impl Into<String>) -> Report {
    Report {
        json: json!({"stage": "input", "actual_error": {"message": message.into()},
                     "evaluation_performed": false, "events_forwarded": "0", "completed": false}),
        exit: 1,
        checkpoint: None,
    }
}

fn read(path: &PathBuf) -> Result<String, String> {
    std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))
}

fn dispatch(args: &Args2) -> Report {
    if args.execution_profile != PROFILE_MICROSTEP_2 {
        return input_failure("unknown execution profile");
    }
    let fin = match args.source_profile.as_str() {
        s2::SOURCE_PROFILE_DRAFT_2 => false,
        s2::SOURCE_PROFILE_FIN_2 => true,
        _ => return input_failure("the /2 route admits moriarty-sherlock-draft/2 or moriarty/2"),
    };
    let (source, config, target) = match (
        read(&args.file),
        read(&args.execution_config),
        read(&args.target),
    ) {
        (Ok(a), Ok(b), Ok(c)) => (a, b, c),
        (Err(e), ..) | (_, Err(e), _) | (.., Err(e)) => return input_failure(e),
    };
    if let Some(resume) = &args.resume {
        let checkpoint = match std::fs::read(resume) {
            Ok(bytes) => bytes,
            Err(e) => {
                return s2::Report {
                    json: s2_refused("R0", format!("{}: {e}", resume.display())),
                    exit: 4,
                    checkpoint: None,
                };
            }
        };
        let observation = match args.observation.as_ref().map(read) {
            Some(Ok(text)) => text,
            Some(Err(e)) => return input_failure(e),
            None => return input_failure("--resume requires --observation"),
        };
        return if fin {
            s2::fin::resume_fin(
                &source,
                &config,
                &target,
                args.phase_policy.as_deref(),
                &checkpoint,
                &observation,
            )
        } else if args.phase_policy.is_some() {
            input_failure("--phase-policy applies to the financial route only")
        } else {
            s2::resume_pure(&source, &config, &target, &checkpoint, &observation)
        };
    }
    if fin {
        let (Some(action), Some(scenario)) = (&args.action, &args.scenario) else {
            return input_failure("the financial /2 start requires --action and --scenario");
        };
        let scenario = match read(scenario) {
            Ok(text) => text,
            Err(e) => return input_failure(e),
        };
        s2::fin::start_fin(
            &source,
            &config,
            &target,
            args.phase_policy.as_deref(),
            action,
            &scenario,
        )
    } else {
        if !args.entry {
            return input_failure("the pure /2 start requires --entry");
        }
        if args.phase_policy.is_some() {
            return input_failure("--phase-policy applies to the financial route only");
        }
        s2::start_pure(&source, &config, &target)
    }
}

fn s2_refused(stage: &str, error: String) -> serde_json::Value {
    json!({"status": "refused", "stage": stage, "error": error,
           "profile": PROFILE_MICROSTEP_2, "qualification": s2::QUALIFICATION,
           "authentication": "none", "consumption": "untracked"})
}

fn run(args: &Args2) -> ExitCode {
    let mut report = dispatch(args);
    if let (Some(path), Some(bytes)) = (&args.checkpoint_out, &report.checkpoint) {
        if let Err(e) = std::fs::write(path, bytes) {
            report = input_failure(format!("{}: {e}", path.display()));
        } else if let Some(o) = report.json.as_object_mut() {
            o.insert("checkpoint_file".into(), json!(path.display().to_string()));
        }
    }
    match mori_core::json::export(&report.json) {
        Ok(text) => println!("{text}"),
        Err(_) => return ExitCode::FAILURE,
    }
    ExitCode::from(report.exit)
}
