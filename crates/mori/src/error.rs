//! Command failures that do not point at source code.

use std::io;
use std::path::PathBuf;

use mori_diagnostics::Diagnostic;

#[derive(Debug, thiserror::Error)]
pub enum CliError {
    #[error("{code}: {message}")]
    Operation { code: String, message: String },
    #[error("could not read `{}`", crate::display::text(&path.display().to_string()))]
    Read {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("could not write `{}`", crate::display::text(&path.display().to_string()))]
    Write {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("`{}` does not exist", crate::display::text(&path.display().to_string()))]
    NotFound { path: PathBuf },
    #[error("could not list `{}`", crate::display::text(&path.display().to_string()))]
    Walk {
        path: PathBuf,
        #[source]
        source: ignore::Error,
    },
}

impl Diagnostic for CliError {}
