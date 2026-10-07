use std::process::ExitCode;

use clap::Parser;

use crate::cli::Cli;
use crate::shell::Shell;

mod cli;
mod commands;
mod error;
mod shell;
mod styles;

fn main() -> ExitCode {
    let cli = Cli::parse();
    let mut shell = Shell::new(cli.global.verbosity(), cli.global.color.into());

    match commands::run(cli.command, &mut shell) {
        Ok(code) => code,
        Err(report) => {
            shell.report(&report);
            ExitCode::FAILURE
        }
    }
}
