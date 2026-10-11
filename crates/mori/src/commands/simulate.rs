use std::process::ExitCode;

use crate::cli::ActionInput;
use crate::shell::Shell;

#[derive(Debug, clap::Args)]
pub struct Args {
    #[command(flatten)]
    pub input: ActionInput,
}

pub fn run(args: Args, _shell: &mut Shell) -> super::Result<ExitCode> {
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
