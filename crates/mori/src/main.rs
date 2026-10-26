use std::process::ExitCode;

use clap::Parser;

use crate::cli::Cli;
use crate::shell::Shell;

mod cli;
mod cli_args;
mod commands;
mod display;
mod error;
mod json;
mod shell;
mod status_output;
mod styles;

fn main() -> ExitCode {
    // Profile /2 suspension/finance simulation has its own argument set.
    if let Some(code) = commands::simulate_suspend::try_run_from_env() {
        return code;
    }
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
