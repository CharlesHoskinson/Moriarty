//! Type annotations, such as `const fee: Qty<USD> = 0.10 USD;`.

use mori_ast::NodeIdx;
use mori_diagnostics::suggest;
use mori_lexer::TokenKind;

use crate::checker::Checker;
use crate::values::{ValueIdx, ValueTag};
use crate::{DeclIdx, diagnostics};

/// Annotations naming a declared kind, with the keyword they require.
const ENTITY_TYPES: [(&str, TokenKind); 13] = [
    ("Domain", TokenKind::KwDomain),
    ("Account", TokenKind::KwAccount),
    ("Asset", TokenKind::KwAsset),
    ("Obligation", TokenKind::KwObligation),
    ("Pool", TokenKind::KwPool),
    ("Instrument", TokenKind::KwInstrument),
    ("Observation", TokenKind::KwObservation),
    ("Policy", TokenKind::KwPolicy),
    ("Grant", TokenKind::KwGrant),
    ("Stage", TokenKind::KwStage),
    ("Episode", TokenKind::KwEpisode),
    ("Party", TokenKind::KwParty),
    ("ShareClass", TokenKind::KwShareClass),
];

/// Annotations for plain values, with the value tag they require.
const PLAIN_TYPES: [(&str, ValueTag); 4] = [
    ("Scalar", ValueTag::Scalar),
    ("UInt128", ValueTag::Scalar),
    ("String", ValueTag::String),
    ("Bool", ValueTag::Bool),
];

/// Entity annotations that may name the entity's domain, like `Account<Preview>`.
const DOMAIN_TYPES: [&str; 3] = ["Account", "Asset", "Obligation"];

impl<'a> Checker<'a> {
    /// Checks `value` against the annotation `ty`. False on a mismatch.
    pub(crate) fn check_annotation(
        &mut self,
        order: usize,
        ty: NodeIdx,
        value: ValueIdx,
        value_node: NodeIdx,
    ) -> bool {
        let ast = self.ast;
        let name = ast.token_text(ast.main_token(ty));
        let args: Vec<NodeIdx> = ast.list(ty).collect();
        let simple_arg = match args.as_slice() {
            [arg] if ast.list(*arg).next().is_none() => Some(*arg),
            _ => None,
        };
        let type_span = ast.span(ty);
        let value_span = ast.span(value_node);

        if name == "Qty" {
            let Some(arg) = simple_arg else {
                self.report(diagnostics::wrong_type_arguments(
                    type_span,
                    "`Qty` needs one asset, like `Qty<USD>`.".to_owned(),
                ));
                return false;
            };
            let Some(asset) = self.resolve(order, ast.main_token(arg), arg) else {
                return false;
            };
            if !self.is_kind(asset, TokenKind::KwAsset) {
                let span = ast.span(arg);
                self.report(diagnostics::not_an_asset(
                    span,
                    self.describe_decl(asset),
                    self.decl_name_span(asset),
                ));
                return false;
            }
            let matches =
                self.values.tag(value) == ValueTag::Quantity && self.values.asset(value) == asset;
            return matches
                || self.mismatch(
                    type_span,
                    &format!("a `{}` amount", self.name(asset)),
                    value_span,
                    value,
                );
        }

        if let Some(&(_, tag)) = PLAIN_TYPES.iter().find(|(plain, _)| *plain == name) {
            if !args.is_empty() {
                self.report(diagnostics::wrong_type_arguments(
                    type_span,
                    format!("`{name}` takes no type arguments."),
                ));
                return false;
            }
            let expected = match tag {
                ValueTag::Scalar => "a plain number",
                ValueTag::String => "a string",
                _ => "a true or false value",
            };
            return self.values.tag(value) == tag
                || self.mismatch(type_span, expected, value_span, value);
        }

        if let Some(&(_, kind)) = ENTITY_TYPES.iter().find(|(entity, _)| *entity == name) {
            let domain = match (args.as_slice(), simple_arg) {
                ([], _) => None,
                (_, Some(arg)) if DOMAIN_TYPES.contains(&name) => Some(arg),
                _ => {
                    let message = if DOMAIN_TYPES.contains(&name) {
                        format!("`{name}` takes at most one domain, like `{name}<Preview>`.")
                    } else {
                        format!("`{name}` takes no type arguments.")
                    };
                    self.report(diagnostics::wrong_type_arguments(type_span, message));
                    return false;
                }
            };
            let is_entity = self.values.tag(value) == ValueTag::Entity
                && self.is_kind(self.values.entity(value), kind);
            if !is_entity {
                let expected = format!("{} {}", article(name), kind_word(kind));
                return self.mismatch(type_span, &expected, value_span, value);
            }
            return match domain {
                Some(arg) => self.check_domain_annotation(order, arg, value, value_span),
                None => true,
            };
        }

        let known = PLAIN_TYPES
            .iter()
            .map(|(name, _)| *name)
            .chain(ENTITY_TYPES.iter().map(|(name, _)| *name))
            .chain(["Qty"]);
        let suggestion = suggest(name, known);
        self.report(diagnostics::unknown_type(
            ast.tokens.span(ast.main_token(ty), ast.source),
            name,
            suggestion,
            ast.source,
        ));
        false
    }

    /// `Account<Preview>`: the entity's `domain` field must be that domain.
    fn check_domain_annotation(
        &mut self,
        order: usize,
        arg: NodeIdx,
        value: ValueIdx,
        value_span: mori_span::Span,
    ) -> bool {
        let Some(domain) = self.resolve(order, self.ast.main_token(arg), arg) else {
            return false;
        };
        let arg_span = self.ast.span(arg);
        if !self.is_kind(domain, TokenKind::KwDomain) {
            self.report(diagnostics::not_a_domain(
                arg_span,
                self.describe_decl(domain),
                self.decl_name_span(domain),
            ));
            return false;
        }
        // A missing or invalid `domain` field is reported with the entity's own rules.
        let Some(field) = self.field(value, "domain") else {
            return true;
        };
        if self.values.tag(field) != ValueTag::Entity {
            return true;
        }
        let actual = self.values.entity(field);
        if actual == domain {
            return true;
        }
        self.report(diagnostics::wrong_domain(
            arg_span,
            self.name(domain),
            value_span,
            &self.describe(value),
            self.name(actual),
        ));
        false
    }

    fn mismatch(
        &mut self,
        type_span: mori_span::Span,
        expected: &str,
        value_span: mori_span::Span,
        value: ValueIdx,
    ) -> bool {
        self.report(diagnostics::type_mismatch(
            type_span,
            expected,
            value_span,
            &self.describe(value),
        ));
        false
    }

    pub(crate) fn is_kind(&self, decl: DeclIdx, kind: TokenKind) -> bool {
        let node = self.declarations.node()[decl.index()];
        self.ast.tokens.kind(self.ast.main_token(node)) == kind
    }
}

fn kind_word(kind: TokenKind) -> &'static str {
    match kind {
        TokenKind::KwDomain => "domain",
        TokenKind::KwAccount => "account",
        TokenKind::KwAsset => "asset",
        TokenKind::KwObligation => "obligation",
        TokenKind::KwPool => "pool",
        TokenKind::KwInstrument => "instrument",
        TokenKind::KwObservation => "observation",
        TokenKind::KwPolicy => "policy",
        TokenKind::KwGrant => "grant",
        TokenKind::KwStage => "stage",
        TokenKind::KwEpisode => "episode",
        TokenKind::KwParty => "party",
        _ => "share class",
    }
}

fn article(name: &str) -> &'static str {
    if name.starts_with(['A', 'E', 'I', 'O', 'U']) {
        "an"
    } else {
        "a"
    }
}
