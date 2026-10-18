//! Every item's name, collected before the ordered walk so that a reference
//! to a later declaration can say where it is declared.

use std::collections::HashMap;

use mori_ast::{Ast, NodeIdx};
use mori_diagnostics::MoriDiagnostic;

use crate::diagnostics;

/// An item, by its position among the agreement's items.
#[derive(Debug, Clone, Copy)]
pub struct Item {
    pub order: usize,
    pub node: NodeIdx,
}

#[derive(Default)]
pub struct Names<'a> {
    /// The first item with each name.
    first: HashMap<&'a str, Item>,
    /// By item order: it reuses an earlier item's name.
    duplicate: Vec<bool>,
}

impl<'a> Names<'a> {
    /// Collects the names of `items`, reporting each reused name.
    pub fn collect(
        ast: &'a Ast<'a>,
        items: &[NodeIdx],
        diagnostics: &mut Vec<MoriDiagnostic>,
    ) -> Self {
        let mut names = Self::default();
        for (order, &node) in items.iter().enumerate() {
            let token = name_token(ast, node);
            let name = ast.token_text(token);
            match names.first.get(name) {
                Some(earlier) => {
                    let earlier = name_token(ast, earlier.node);
                    diagnostics.push(diagnostics::duplicate_name(
                        ast.tokens.span(token, ast.source),
                        ast.tokens.span(earlier, ast.source),
                        name,
                    ));
                    names.duplicate.push(true);
                }
                None => {
                    names.first.insert(name, Item { order, node });
                    names.duplicate.push(false);
                }
            }
        }
        names
    }

    /// The first item named `name`, anywhere in the agreement.
    pub fn get(&self, name: &str) -> Option<Item> {
        self.first.get(name).copied()
    }

    pub fn is_duplicate(&self, order: usize) -> bool {
        self.duplicate[order]
    }
}

/// The name of a declaration or action: the token after its keyword.
pub fn name_token(ast: &Ast<'_>, item: NodeIdx) -> mori_lexer::TokenIdx {
    ast.token_after(ast.main_token(item), 1)
}
