//! Command failures that do not point at source code.

use std::io;
use std::path::PathBuf;

use mori_diagnostics::Diagnostic;

#[derive(Debug, thiserror::Error)]
pub enum CliError {
    #[error("could not read `{}`", path.display())]
    Read {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
}

impl Diagnostic for CliError {}
