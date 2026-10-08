//! Checks a parsed `moriarty-beta/1` file.
//!
//! [`check`] walks the agreement's items in order, and each declaration sees
//! only the declarations above it. A declaration with an error is marked
//! failed, and anything that depends on it is skipped without further errors,
//! so each mistake is reported once.

mod calls;
mod checker;
mod declarations;
mod diagnostics;
mod eval;
mod intents;
mod names;
mod relations;
mod reserved;
mod types;
mod values;

#[cfg(test)]
mod proptests;
#[cfg(test)]
mod tests;

use mori_ast::{Ast, NodeIdx};
use mori_diagnostics::MoriDiagnostic;
use mori_lexer::TokenKind;
use soa_rs::{Soa, Soars};

pub use checker::check;
pub use intents::Support;
pub use values::{Value, ValueIdx, ValueTag, Values};

/// Everything [`check`] could check, and every error it found. The program
/// is accepted only when there are no diagnostics.
pub struct CheckResult<'a> {
    pub checked: Checked<'a>,
    pub diagnostics: Vec<MoriDiagnostic>,
}

impl CheckResult<'_> {
    pub fn is_accepted(&self) -> bool {
        self.diagnostics.is_empty()
    }
}

/// Index of a declaration in [`Checked`]. An agreement has at most 256 items.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeclIdx(u16);

impl DeclIdx {
    pub fn new(index: usize) -> Self {
        Self(u16::try_from(index).expect("declaration index exceeds u16"))
    }

    pub fn index(self) -> usize {
        usize::from(self.0)
    }
}

/// A declaration, in source order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Soars)]
#[soa_derive(Debug, PartialEq)]
pub struct Declaration {
    pub node: NodeIdx,
    /// It has an error, or depends on a declaration that does.
    pub failed: bool,
    /// Its value, unless it failed.
    pub value: Option<ValueIdx>,
}

/// An action, in source order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Soars)]
#[soa_derive(Debug, PartialEq)]
pub struct Action {
    pub node: NodeIdx,
    /// The intent it uses, when that resolved to an intent declaration.
    pub intent: Option<DeclIdx>,
    /// What that intent can do, when it was checked.
    pub support: Option<Support>,
}

/// The checked program, as side tables over the [`Ast`].
pub struct Checked<'a> {
    pub ast: &'a Ast<'a>,
    pub declarations: Soa<Declaration>,
    pub actions: Soa<Action>,
    pub values: Values,
    /// For each node, the declaration it names: a reference, a quantity's
    /// asset or an action's intent. [`UNRESOLVED`] otherwise.
    resolutions: Vec<u16>,
}

const UNRESOLVED: u16 = u16::MAX;

impl<'a> Checked<'a> {
    /// The keyword that declared it, such as [`TokenKind::KwConst`].
    pub fn kind(&self, decl: DeclIdx) -> TokenKind {
        let node = self.declarations.node()[decl.index()];
        self.ast.tokens.kind(self.ast.main_token(node))
    }

    /// The value of a declaration that did not fail.
    pub fn value(&self, decl: DeclIdx) -> Option<ValueIdx> {
        self.declarations.value()[decl.index()]
    }

    pub fn name(&self, decl: DeclIdx) -> &'a str {
        let node = self.declarations.node()[decl.index()];
        self.ast
            .token_text(self.ast.token_after(self.ast.main_token(node), 1))
    }

    /// The declaration `node` names, if it is a reference, quantity or action
    /// that resolved.
    pub fn resolution(&self, node: NodeIdx) -> Option<DeclIdx> {
        let raw = self.resolutions[node.index()];
        (raw != UNRESOLVED).then_some(DeclIdx(raw))
    }
}
