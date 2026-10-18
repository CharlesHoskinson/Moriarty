//! Visibility/module-hierarchy adapter for the complete real production handler.
pub use crate::cli_args::simulate::Args;
#[path = "../../src/commands/simulate_sherlock.rs"]
mod sherlock;

pub fn run(args: &Args) -> crate::Result<std::process::ExitCode> {
    sherlock::run(args)
}
