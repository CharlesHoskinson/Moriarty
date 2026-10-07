use std::path::PathBuf;
use std::process::ExitCode;

use crate::shell::Shell;

#[derive(Debug, clap::Args)]
pub struct Args {
    /// Project directory
    pub dir: PathBuf,
}

pub fn run(_args: Args, _shell: &mut Shell) -> super::Result<ExitCode> {
    todo!("mori test")
}
