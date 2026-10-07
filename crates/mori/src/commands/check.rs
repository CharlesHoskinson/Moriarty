use std::path::PathBuf;
use std::process::ExitCode;

use crate::shell::Shell;

#[derive(Debug, clap::Args)]
pub struct Args {
    /// Source file
    pub file: PathBuf,

    /// Print the result as JSON
    #[arg(long)]
    pub json: bool,
}

pub fn run(_args: Args, _shell: &mut Shell) -> super::Result<ExitCode> {
    todo!("mori check")
}
