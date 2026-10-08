//! Diagnostics reported while formatting.

use mori_diagnostics::MoriDiagnostic;
use mori_lexer::MAX_SOURCE_BYTES;

/// A file `mori format --check` would change. Its diff is printed below it.
pub fn not_formatted(file: &str) -> MoriDiagnostic {
    MoriDiagnostic::error(format!("`{file}` is not formatted."))
        .with_code("mori::fmt::not_formatted")
}

pub fn too_large(len: usize) -> MoriDiagnostic {
    MoriDiagnostic::error(format!(
        "Formatting this file would make it {len} bytes, but a source file can be at most \
         {MAX_SOURCE_BYTES} bytes."
    ))
    .with_code("mori::fmt::too_large")
}
