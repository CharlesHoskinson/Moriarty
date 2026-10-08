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
    #[error("could not write `{}`", path.display())]
    Write {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("`{}` does not exist", path.display())]
    NotFound { path: PathBuf },
    #[error("could not list `{}`", path.display())]
    Walk {
        path: PathBuf,
        #[source]
        source: ignore::Error,
    },
}

impl Diagnostic for CliError {}
