//! One module per subcommand. Each defines its `Args` and a `run` function.

use std::process::ExitCode;

use mori_diagnostics::Report;

use crate::cli::Command;
use crate::shell::Shell;

mod call_io;
pub mod capabilities;
pub mod check;
mod check_source;
pub mod compile;
pub mod compile_call;
mod core_io;
pub mod expand;
pub mod format;
pub mod init;
pub mod inspect;
pub mod intent;
pub mod keygen_call;
pub mod keygen_signed_call;
pub mod lsp;
pub mod mcp;
pub mod prepare_call;
pub mod prepare_signed_call;
mod proof_io;
pub mod prove;
pub mod prove_call;
pub mod prove_signed_call;
pub mod replay;
mod signed_call_io;
pub mod simulate;
pub mod test;
pub mod verify;
pub mod verify_call;
pub mod verify_intent;
pub mod verify_signed_call;

pub type Result<T> = std::result::Result<T, Report>;

pub fn run(command: Command, shell: &mut Shell) -> Result<ExitCode> {
    match command {
        Command::Capabilities(args) => capabilities::run(args, shell),
        Command::Init(args) => init::run(args, shell),
        Command::Check(args) => check::run(args, shell),
        Command::Compile(args) => compile::run(args, shell),
        Command::KeygenSignedCall(args) => keygen_signed_call::run(args, shell),
        Command::PrepareSignedCall(args) => prepare_signed_call::run(args, shell),
        Command::ProveSignedCall(args) => prove_signed_call::run(args, shell),
        Command::VerifySignedCall(args) => verify_signed_call::run(args, shell),
        Command::KeygenCall(args) => keygen_call::run(args, shell),
        Command::ProveCall(args) => prove_call::run(args, shell),
        Command::VerifyCall(args) => verify_call::run(args, shell),
        Command::CompileCall(args) => compile_call::run(args, shell),
        Command::PrepareCall(args) => prepare_call::run(args, shell),
        Command::Prove(args) => prove::run(args, shell),
        Command::Verify(args) => verify::run(args, shell),
        Command::Format(args) => format::run(args, shell),
        Command::Inspect(args) => inspect::run(args, shell),
        Command::Expand(args) => expand::run(args, shell),
        Command::Simulate(args) => simulate::run(args, shell),
        Command::Replay(args) => replay::run(args, shell),
        Command::Test(args) => test::run(args, shell),
        Command::Intent(args) => intent::run(args, shell),
        Command::VerifyIntent(args) => verify_intent::run(args, shell),
        Command::Lsp(args) => lsp::run(args, shell),
        Command::Mcp(args) => mcp::run(args, shell),
    }
}

#[cfg(not(feature = "native"))]
fn native_unavailable<T>() -> Result<T> {
    Err(crate::error::CliError::Operation {
        code: "mori::command::native-unavailable".into(),
        message: "This mori build has no Midnight support. Rebuild with the native feature enabled."
            .into(),
    }
    .into())
}
