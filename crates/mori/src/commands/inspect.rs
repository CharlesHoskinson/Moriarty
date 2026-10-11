use std::path::PathBuf;
use std::process::ExitCode;

use crate::shell::Shell;

#[derive(Debug, clap::Args)]
pub struct Args {
    /// Source file
    pub file: PathBuf,
}

pub fn run(args: Args, _shell: &mut Shell) -> super::Result<ExitCode> {
    let (_, program) = super::core_io::program(&args.file)?;
    super::core_io::print(
        &serde_json::json!({"language":program.language,"semantics":program.semantics,"agreement":program.name,"states":program.states,"assets":program.assets,"actions":program.actions,"open_premises":["ledger authentication and acceptance","native proof soundness and compiler correspondence","account and sponsor authority"]}),
    )?;
    Ok(ExitCode::SUCCESS)
}
