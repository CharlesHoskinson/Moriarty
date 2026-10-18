use std::process::ExitCode;

use crate::shell::Shell;

#[cfg(feature = "mcp")]
mod server;

#[derive(Debug, clap::Args)]
pub struct Args {}

pub fn run(_args: Args, _shell: &mut Shell) -> super::Result<ExitCode> {
    #[cfg(feature = "mcp")]
    {
        use rmcp::{ServiceExt, service::QuitReason};

        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(super::core_io::operation)?;
        runtime.block_on(async {
            let service = server::MoriServer
                .serve(rmcp::transport::stdio())
                .await
                .map_err(super::core_io::operation)?;
            match service.waiting().await.map_err(super::core_io::operation)? {
                QuitReason::Closed | QuitReason::Cancelled => Ok(ExitCode::SUCCESS),
                QuitReason::JoinError(error) => Err(super::core_io::operation(error)),
                _ => Err(super::core_io::operation("MCP service ended unexpectedly")),
            }
        })
    }
    #[cfg(not(feature = "mcp"))]
    {
        Err(crate::error::CliError::Operation {
            code: "mori::command::mcp-unavailable".into(),
            message: "This mori build has no MCP server. Rebuild with the mcp feature enabled."
                .into(),
        }
        .into())
    }
}

#[cfg(all(test, not(feature = "mcp")))]
mod tests {
    use super::{Args, run};
    use crate::shell::{Shell, Verbosity};

    #[test]
    fn unavailable_feature_returns_an_explicit_command_error() {
        let mut shell = Shell::new(Verbosity::Quiet, anstream::ColorChoice::Never);
        let error = run(Args {}, &mut shell).unwrap_err();
        assert_eq!(
            error.to_string(),
            "mori::command::mcp-unavailable: This mori build has no MCP server. Rebuild with the mcp feature enabled."
        );
    }
}
