//! The ordered walk over the agreement's items.

use std::collections::HashMap;

use mori_ast::{Ast, NodeIdx, NodeTag};
use mori_diagnostics::{Diagnostic, MoriDiagnostic, suggest};
use mori_lexer::{TokenIdx, TokenKind};
use mori_span::Span;
use soa_rs::Soa;

use crate::declarations::Identities;
use crate::names::{Names, name_token};
use crate::reserved::SOURCE6_RESERVED;
use crate::values::{ValueIdx, ValueTag, Values};
use crate::{Action, CheckResult, Checked, DeclIdx, Declaration, UNRESOLVED, diagnostics};

/// Checks `ast`, returning everything checked and every error found.
pub fn check<'a>(ast: &'a Ast<'a>) -> CheckResult<'a> {
    let items: Vec<NodeIdx> = ast.list(NodeIdx::ROOT).collect();
    let mut checker = Checker {
        ast,
        names: Names::default(),
        declarations: Soa::with_capacity(items.len()),
        actions: Soa::new(),
        values: Values::default(),
        resolutions: vec![UNRESOLVED; ast.nodes.len()],
        visible: HashMap::new(),
        identities: Identities::new(),
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
            values: checker.values,
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

pub(crate) struct Checker<'a> {
    pub(crate) ast: &'a Ast<'a>,
    names: Names<'a>,
    pub(crate) declarations: Soa<Declaration>,
    actions: Soa<Action>,
    pub(crate) values: Values,
    resolutions: Vec<u16>,
    /// Declarations checked so far, by name. Later items may use them.
    visible: HashMap<&'a str, DeclIdx>,
    /// Economic identities declared so far, which must be unique.
    pub(crate) identities: Identities,
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

    pub(crate) fn report(&mut self, diagnostic: MoriDiagnostic) {
        self.diagnostics.push(diagnostic);
    }

    fn declaration(&mut self, order: usize, node: NodeIdx) {
        let decl = DeclIdx::new(self.declarations.len());
        let value_node = self.operand(node, 1);
        let mut value = self.eval(order, value_node);

        let keyword = self.ast.tokens.kind(self.ast.main_token(node));
        if let Some(record) = value
            && keyword != TokenKind::KwConst
        {
            value = self.entity(decl, node, record, value_node);
        }
        if let Some(entity) = value
            && keyword != TokenKind::KwConst
        {
            let ok = match keyword {
                TokenKind::KwIntent => self.check_intent(entity, value_node),
                _ => self.check_declaration(keyword, entity, value_node),
            };
            if !ok {
                value = None;
            }
        }
        if let (Some(checked), Some(ty)) = (value, self.ast.lhs_node(node))
            && !self.check_annotation(order, ty, checked, value_node)
        {
            value = None;
        }

        let failed = value.is_none() || self.names.is_duplicate(order);
        let value = value.filter(|_| !failed);
        self.declarations.push(Declaration {
            node,
            failed,
            value,
        });
        if !self.names.is_duplicate(order) {
            let name = self.ast.token_text(name_token(self.ast, node));
            self.visible.insert(name, decl);
        }
    }

    /// Every declaration except `const` is a record of fields, and becomes an
    /// entity that other declarations can refer to.
    fn entity(
        &mut self,
        decl: DeclIdx,
        node: NodeIdx,
        record: ValueIdx,
        value_node: NodeIdx,
    ) -> Option<ValueIdx> {
        if self.values.tag(record) != ValueTag::Record {
            let keyword = self.ast.token_text(self.ast.main_token(node));
            let description = self.describe(record);
            self.report(diagnostics::expected_record(
                self.ast.span(value_node),
                keyword,
                &description,
            ));
            return None;
        }
        let row = record.index();
        let (b, c) = (self.values.rows.b()[row], self.values.rows.c()[row]);
        Some(self.values.push(ValueTag::Entity, node, decl.0, b, c))
    }

    fn action(&mut self, order: usize, node: NodeIdx) {
        self.check_source6_name(name_token(self.ast, node), "action");
        let intent_token = self.ast.token_after(self.ast.main_token(node), 3);
        let intent = self
            .resolve(order, intent_token, node)
            .filter(|&decl| self.check_is_intent(decl, intent_token));
        let support = intent.and_then(|intent| self.support(intent));
        self.actions.push(Action {
            node,
            intent,
            support,
        });
    }

    /// Resolves the name at `token`, used by item number `order`, recording
    /// it for `node`. `None` if it does not name a usable declaration; an
    /// error is reported unless that declaration already failed.
    pub(crate) fn resolve(
        &mut self,
        order: usize,
        token: TokenIdx,
        node: NodeIdx,
    ) -> Option<DeclIdx> {
        let name = self.ast.token_text(token);
        if let Some(&decl) = self.visible.get(name) {
            if self.declarations.failed()[decl.index()] {
                return None;
            }
            self.resolutions[node.index()] = decl.0;
            return Some(decl);
        }

        let span = self.token_span(token);
        let diagnostic = match self.names.get(name) {
            Some(item) if self.ast.tag(item.node) == NodeTag::Action => {
                diagnostics::action_is_not_a_value(span, name, self.name_span(item.node))
            }
            Some(item) if item.order == order => diagnostics::refers_to_itself(span, name),
            Some(item) if item.order > order => {
                diagnostics::used_before_declared(span, name, self.name_span(item.node))
            }
            _ => {
                let visible =
                    (0..self.declarations.len()).map(|index| self.name(DeclIdx::new(index)));
                let suggestion = suggest(name, visible);
                diagnostics::unknown_name(span, name, suggestion, self.ast.source)
            }
        };
        self.report(diagnostic);
        None
    }

    /// An action's `uses` must name an intent.
    fn check_is_intent(&mut self, decl: DeclIdx, token: TokenIdx) -> bool {
        if self.is_kind(decl, TokenKind::KwIntent) {
            return true;
        }
        let node = self.declarations.node()[decl.index()];
        let keyword = self.ast.token_text(self.ast.main_token(node));
        self.report(diagnostics::uses_non_intent(
            self.token_span(token),
            self.ast.token_text(token),
            keyword,
            self.name_span(node),
        ));
        false
    }

    /// Agreement and action names must be representable in Source/6.
    fn check_source6_name(&mut self, token: TokenIdx, role: &str) {
        let name = self.ast.token_text(token);
        if SOURCE6_RESERVED.contains(&name) {
            self.report(diagnostics::reserved_by_source6(
                self.token_span(token),
                name,
                role,
                self.ast.source,
            ));
        }
    }

    pub(crate) fn name(&self, decl: DeclIdx) -> &'a str {
        let node = self.declarations.node()[decl.index()];
        self.ast.token_text(name_token(self.ast, node))
    }

    pub(crate) fn token_span(&self, token: TokenIdx) -> Span {
        self.ast.tokens.span(token, self.ast.source)
    }

    fn name_span(&self, item: NodeIdx) -> Span {
        self.token_span(name_token(self.ast, item))
    }

    pub(crate) fn decl_name_span(&self, decl: DeclIdx) -> Span {
        self.name_span(self.declarations.node()[decl.index()])
    }
}
