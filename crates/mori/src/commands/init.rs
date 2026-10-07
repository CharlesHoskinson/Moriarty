use std::path::PathBuf;
use std::process::ExitCode;

use clap::ValueEnum;

use crate::shell::Shell;

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

pub fn run(_args: Args, _shell: &mut Shell) -> super::Result<ExitCode> {
    todo!("mori init")
}
