use std::path::PathBuf;
use std::process::ExitCode;

use crate::shell::Shell;

#[derive(Debug, clap::Args)]
pub struct Args {
    /// Source file
    pub file: PathBuf,

    /// Write the formatted source back to the file instead of stdout
    #[arg(long)]
    pub write: bool,
}

pub fn run(_args: Args, _shell: &mut Shell) -> super::Result<ExitCode> {
    todo!("mori fmt")
}
