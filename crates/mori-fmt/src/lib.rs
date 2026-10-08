//! Formatter for `moriarty-beta/1` source.
//!
//! [`format`] lays a parsed file out canonically within [`WIDTH`] columns,
//! keeping comments, with exactly one blank line between agreement items.
//! [`diff`] finds the regions a formatting run would change, for `--check`.

mod diagnostics;
mod diff;
mod doc;
mod format;

#[cfg(test)]
mod proptests;
#[cfg(test)]
mod tests;

use mori_ast::Ast;
use mori_diagnostics::MoriDiagnostic;
use mori_lexer::MAX_SOURCE_BYTES;

pub use diagnostics::not_formatted;
pub use diff::{Change, Hunk, diff};

/// Formatted lines fit in this many columns where they can.
pub const WIDTH: usize = 80;

/// Formats `source`, or reports why it cannot be formatted.
pub fn format(source: &str) -> Result<String, MoriDiagnostic> {
    let ast = mori_parser::parse(source)?;
    format_ast(&ast)
}

/// Formats an already parsed file.
pub fn format_ast(ast: &Ast<'_>) -> Result<String, MoriDiagnostic> {
    let output = format::Formatter::new(ast).format(WIDTH);
    if output.len() > MAX_SOURCE_BYTES {
        return Err(diagnostics::too_large(output.len()));
    }
    Ok(output)
}
