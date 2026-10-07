//! The ordered walk over the agreement's items.

use std::collections::HashMap;

use mori_ast::{Ast, NodeIdx, NodeTag};
use mori_diagnostics::{Diagnostic, MoriDiagnostic, suggest};
use mori_lexer::{TokenIdx, TokenKind};
use soa_rs::Soa;

use crate::names::{Names, name_token};
use crate::reserved::SOURCE6_RESERVED;
use crate::{Action, CheckResult, Checked, DeclIdx, Declaration, UNRESOLVED, diagnostics};

/// Checks `ast`, returning everything checked and every error found.
pub fn check<'a>(ast: &'a Ast<'a>) -> CheckResult<'a> {
    let items: Vec<NodeIdx> = ast.list(NodeIdx::ROOT).collect();
    let mut checker = Checker {
        ast,
        names: Names::default(),
        declarations: Soa::with_capacity(items.len()),
        actions: Soa::new(),
        resolutions: vec![UNRESOLVED; ast.nodes.len()],
        visible: HashMap::new(),
        diagnostics: Vec::new(),
    };
    checker.run(&items);
    // Name collection finds duplicates before the walk; report in source order.
    checker.diagnostics.sort_by_key(position);
    CheckResult {
        checked: Checked {
            ast,
            declarations: checker.declarations,
            actions: checker.actions,
            resolutions: checker.resolutions,
        },
        diagnostics: checker.diagnostics,
    }
}

/// Where a diagnostic points: its primary label, or else its first label.
fn position(diagnostic: &MoriDiagnostic) -> usize {
    let Some(labels) = diagnostic.labels() else {
        return 0;
    };
    let labels: Vec<_> = labels.collect();
    labels
        .iter()
        .find(|label| label.primary())
        .or(labels.first())
        .map_or(0, |label| label.offset())
}

struct Checker<'a> {
    ast: &'a Ast<'a>,
    names: Names<'a>,
    declarations: Soa<Declaration>,
    actions: Soa<Action>,
    resolutions: Vec<u16>,
    /// Declarations checked so far, by name. Later items may use them.
    visible: HashMap<&'a str, DeclIdx>,
    diagnostics: Vec<MoriDiagnostic>,
}

impl<'a> Checker<'a> {
    fn run(&mut self, items: &[NodeIdx]) {
        self.check_source6_name(self.ast.main_token(NodeIdx::ROOT), "agreement");
        self.names = Names::collect(self.ast, items, &mut self.diagnostics);
        for (order, &item) in items.iter().enumerate() {
            match self.ast.tag(item) {
                NodeTag::Declaration => self.declaration(order, item),
                NodeTag::Action => self.action(order, item),
                tag => unreachable!("an agreement holds only items, not {tag:?}"),
            }
        }
    }

    fn declaration(&mut self, order: usize, node: NodeIdx) {
        let mut failed = self.names.is_duplicate(order);
        let value = self.ast.rhs_node(node).expect("a declaration has a value");
        failed |= !self.resolve_all(order, value);

        let decl = DeclIdx::new(self.declarations.len());
        self.declarations.push(Declaration { node, failed });
        if !self.names.is_duplicate(order) {
            let name = self.ast.token_text(name_token(self.ast, node));
            self.visible.insert(name, decl);
        }
    }

    fn action(&mut self, order: usize, node: NodeIdx) {
        self.check_source6_name(name_token(self.ast, node), "action");
        let intent_token = self.ast.token_after(self.ast.main_token(node), 3);
        let intent = self
            .resolve(order, intent_token, node)
            .filter(|&decl| self.check_is_intent(decl, intent_token));
        self.actions.push(Action { node, intent });
    }

    /// Resolves every name used inside `node`. False if any failed.
    fn resolve_all(&mut self, order: usize, node: NodeIdx) -> bool {
        let mut ok = match self.ast.tag(node) {
            NodeTag::Reference => {
                let token = self.ast.main_token(node);
                self.resolve(order, token, node).is_some()
            }
            NodeTag::Quantity => {
                let asset = self.ast.token_after(self.ast.main_token(node), 1);
                self.resolve(order, asset, node).is_some()
            }
            _ => true,
        };
        let children: Vec<NodeIdx> = self.ast.children(node).collect();
        for child in children {
            ok &= self.resolve_all(order, child);
        }
        ok
    }

    /// Resolves the name at `token`, used by item number `order`, recording
    /// it for `node`. `None` if it does not name a usable declaration; an
    /// error is reported unless that declaration already failed.
    fn resolve(&mut self, order: usize, token: TokenIdx, node: NodeIdx) -> Option<DeclIdx> {
        let name = self.ast.token_text(token);
        if let Some(&decl) = self.visible.get(name) {
            if self.declarations.failed()[decl.index()] {
                return None;
            }
            self.resolutions[node.index()] = decl.0;
            return Some(decl);
        }

        let span = self.ast.tokens.span(token, self.ast.source);
        let diagnostic = match self.names.get(name) {
            Some(item) if self.ast.tag(item.node) == NodeTag::Action => {
                let action = self.name_span(item.node);
                diagnostics::action_is_not_a_value(span, name, action)
            }
            Some(item) if item.order == order => diagnostics::refers_to_itself(span, name),
            Some(item) if item.order > order => {
                diagnostics::used_before_declared(span, name, self.name_span(item.node))
            }
            _ => {
                let visible = (0..self.declarations.len()).map(|index| {
                    self.ast
                        .token_text(name_token(self.ast, self.declarations.node()[index]))
                });
                let suggestion = suggest(name, visible);
                diagnostics::unknown_name(span, name, suggestion, self.ast.source)
            }
        };
        self.diagnostics.push(diagnostic);
        None
    }

    /// An action's `uses` must name an intent.
    fn check_is_intent(&mut self, decl: DeclIdx, token: TokenIdx) -> bool {
        let node = self.declarations.node()[decl.index()];
        let keyword = self.ast.main_token(node);
        if self.ast.tokens.kind(keyword) == TokenKind::KwIntent {
            return true;
        }
        self.diagnostics.push(diagnostics::uses_non_intent(
            self.ast.tokens.span(token, self.ast.source),
            self.ast.token_text(token),
            self.ast.token_text(keyword),
            self.name_span(node),
        ));
        false
    }

    /// Agreement and action names must be representable in Source/6.
    fn check_source6_name(&mut self, token: TokenIdx, role: &str) {
        let name = self.ast.token_text(token);
        if SOURCE6_RESERVED.contains(&name) {
            let span = self.ast.tokens.span(token, self.ast.source);
            self.diagnostics.push(diagnostics::reserved_by_source6(
                span,
                name,
                role,
                self.ast.source,
            ));
        }
    }

    fn name_span(&self, item: NodeIdx) -> mori_span::Span {
        self.ast
            .tokens
            .span(name_token(self.ast, item), self.ast.source)
    }
}
