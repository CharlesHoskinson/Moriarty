//! Parser for `moriarty-beta/1` source.
//!
//! [`parse`] lexes and parses a file into an [`Ast`], stopping at the first
//! error. It checks syntax and syntactic bounds only; names, types and values
//! are checked by later phases.

mod diagnostics;
mod parser;

#[cfg(test)]
mod tests;

use mori_ast::Ast;
use mori_diagnostics::MoriDiagnostic;

/// The only profile this parser accepts.
pub const PROFILE: &str = "moriarty-beta/1";
/// Deepest nesting of expressions and types.
pub const MAX_DEPTH: usize = 64;
/// Most fields in one record or call, and most type arguments.
pub const MAX_FIELDS: usize = 64;
/// Most declarations and actions in one agreement.
pub const MAX_ITEMS: usize = 256;

/// Lexes and parses `source`, or reports the first error.
pub fn parse(source: &str) -> Result<Ast<'_>, MoriDiagnostic> {
    let tokens = mori_lexer::lex(source)?;
    let (nodes, extra) = parser::Parser::new(source, &tokens).run()?;
    Ok(Ast {
        source,
        tokens,
        nodes,
        extra,
    })
}
