use std::process::ExitCode;

use crate::cli::ActionInput;
use crate::shell::Shell;

#[derive(Debug, clap::Args)]
pub struct Args {
    #[command(flatten)]
    pub input: ActionInput,
    /// Generate a complete public replay sidecar against a trusted Target
    #[arg(long)]
    pub trace: bool,
    /// Trusted Target JSON (required with --trace)
    #[arg(long, value_name = "FILE")]
    pub target: Option<std::path::PathBuf>,
    #[command(flatten)]
    pub authorization: super::replay::AuthorizationArgs,
}

pub fn run(args: Args, _shell: &mut Shell) -> super::Result<ExitCode> {
    if args.trace {
        let path = args
            .target
            .as_ref()
            .ok_or_else(|| super::core_io::operation("--trace requires --target"))?;
        let target = super::core_io::read_json(path)?;
        let authorization = args.authorization.load(&target)?;
        let source = super::core_io::read_source(&args.input.file)?;
        let scenario = super::core_io::read_json(&args.input.scenario)?;
        return match mori_backend::replay::generate(
            &source,
            &args.input.action,
            &scenario,
            &target,
            authorization.as_input(),
        ) {
            Ok(record) => {
                super::core_io::print(&record)?;
                Ok(ExitCode::SUCCESS)
            }
            Err(error) => {
                super::core_io::print(
                    &serde_json::json!({"status":"rejected","qualification":"local-reference-replay","error":error}),
                )?;
                Ok(ExitCode::FAILURE)
            }
        };
    }
    if args.target.is_some() || args.authorization.present() {
        return Err(super::core_io::operation(
            "--target and signed replay arguments require --trace",
        ));
    }
    let (_, program) = super::core_io::program(&args.input.file)?;
    let scenario = super::core_io::read_json(&args.input.scenario)?;
    match mori_core::simulate(&program, &args.input.action, &scenario) {
        Ok(execution) => {
            super::core_io::print(
                &serde_json::json!({"status":"accepted", "qualification":"local-reference-execution", "execution":execution}),
            )?;
            Ok(ExitCode::SUCCESS)
        }
        Err(error) => {
            super::core_io::print(&serde_json::json!({"status":"rejected", "error":error}))?;
            Ok(ExitCode::FAILURE)
        }
    }
}
