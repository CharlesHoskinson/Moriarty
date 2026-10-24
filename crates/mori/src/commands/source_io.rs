//! Production SDK-free source reading and public JSON printing.
//! Extracted without changing existing byte/JSON/escaping/newline rules.
use mori_diagnostics::{Diagnostic, Report};
use serde::Serialize;
use std::path::Path;

type Result<T> = std::result::Result<T, Report>;

// These display templates and source chaining match the existing CliError
// Read/Operation variants. Backend/ledger and directory-walk errors stay there.
#[derive(Debug, thiserror::Error)]
enum SourceIoError {
    #[error("{code}: {message}")]
    Operation { code: String, message: String },
    #[error("could not read `{}`", crate::display::text(&path.display().to_string()))]
    Read {
        path: std::path::PathBuf,
        #[source]
        source: std::io::Error,
    },
}
impl Diagnostic for SourceIoError {}

pub fn read_source(path: &Path) -> Result<String> {
    use std::io::Read;
    let file = std::fs::File::open(path).map_err(|source| SourceIoError::Read {
        path: path.into(),
        source,
    })?;
    let mut source = String::new();
    file.take(1_048_577)
        .read_to_string(&mut source)
        .map_err(|source| SourceIoError::Read {
            path: path.into(),
            source,
        })?;
    if source.len() > 1_048_576 {
        return Err(operation("input file exceeds 1 MiB"));
    }
    Ok(source)
}

pub fn operation(error: impl std::fmt::Display) -> mori_diagnostics::Report {
    SourceIoError::Operation {
        code: "mori::command".into(),
        message: crate::display::text(&error.to_string()),
    }
    .into()
}

pub fn print<T: Serialize>(value: &T) -> Result<()> {
    let display = mori_core::json::export(value).map_err(operation)?;
    println!("{display}");
    Ok(())
}
