//! The fields each kind of declaration takes, and the rules between them.
//!
//! Intents have their own rules and are checked separately.

use std::collections::HashMap;

use mori_ast::{NodeIdx, NodeTag};
use mori_diagnostics::suggest;
use mori_lexer::{TokenIdx, TokenKind, decode_string};
use mori_span::Span;

use crate::checker::Checker;
use crate::eval::MAX_SCALE;
use crate::reserved::SOURCE6_RESERVED;
use crate::values::{ValueIdx, ValueTag};
use crate::{DeclIdx, diagnostics};

/// What kind of value a field must hold.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldRole {
    String,
    Scalar,
    Quantity,
    Asset,
    Domain,
    Account,
    /// A grant, or a string describing the authority.
    GrantOrString,
    /// A quantity, or a string describing the floor.
    QuantityOrString,
    StringList,
    AccountList,
    StageList,
    /// At least one asset, without repeats.
    AssetList,
    /// Not checked.
    Any,
}

impl FieldRole {
    pub fn describe(self) -> &'static str {
        match self {
            Self::String => "a string",
            Self::Scalar => "a whole number",
            Self::Quantity => "an amount",
            Self::Asset => "an asset",
            Self::Domain => "a domain",
            Self::Account => "an account",
            Self::GrantOrString => "a grant or a string",
            Self::QuantityOrString => "an amount or a string",
            Self::StringList => "a list of strings",
            Self::AccountList => "a list of accounts",
            Self::StageList => "a list of stages",
            Self::AssetList => "a list of assets",
            Self::Any => "any value",
        }
    }

    fn signature(self) -> &'static str {
        match self {
            Self::String => "string",
            Self::Scalar => "number",
            Self::Quantity => "amount",
            Self::Asset => "asset",
            Self::Domain => "domain",
            Self::Account => "account",
            Self::GrantOrString => "grant or string",
            Self::QuantityOrString => "amount or string",
            Self::StringList => "[string]",
            Self::AccountList => "[account]",
            Self::StageList => "[stage]",
            Self::AssetList => "[asset]",
            Self::Any => "any",
        }
    }
}

pub struct Field {
    pub name: &'static str,
    pub required: bool,
    pub role: FieldRole,
}

pub struct Rules {
    pub kind: TokenKind,
    pub fields: &'static [Field],
}

impl Rules {
    /// The fields as a record, such as `{ domain: domain, id: string }`,
    /// with `?` marking optional fields.
    pub fn signature(&self) -> String {
        let fields: Vec<String> = self
            .fields
            .iter()
            .map(|field| {
                let optional = if field.required { "" } else { "?" };
                format!("{}{optional}: {}", field.name, field.role.signature())
            })
            .collect();
        diagnostics::layout("{ ", &fields, " }")
    }

    fn field(&self, name: &str) -> Option<&Field> {
        self.fields.iter().find(|field| field.name == name)
    }
}

const fn required(name: &'static str, role: FieldRole) -> Field {
    Field {
        name,
        required: true,
        role,
    }
}

const fn optional(name: &'static str, role: FieldRole) -> Field {
    Field {
        name,
        required: false,
        role,
    }
}

use FieldRole as R;

pub const RULES: &[Rules] = &[
    Rules {
        kind: TokenKind::KwDomain,
        fields: &[
            required("id", R::String),
            required("chain", R::String),
            required("network", R::String),
        ],
    },
    Rules {
        kind: TokenKind::KwAccount,
        fields: &[required("domain", R::Domain), required("id", R::String)],
    },
    Rules {
        kind: TokenKind::KwAsset,
        fields: &[
            required("domain", R::Domain),
            required("id", R::String),
            required("scale", R::Scalar),
            required("representation", R::String),
            optional("symbol", R::String),
        ],
    },
    Rules {
        kind: TokenKind::KwObligation,
        fields: &[
            required("domain", R::Domain),
            required("id", R::String),
            required("asset", R::Asset),
        ],
    },
    Rules {
        kind: TokenKind::KwPool,
        fields: &[
            required("domain", R::Domain),
            required("id", R::String),
            required("assets", R::AssetList),
        ],
    },
    Rules {
        kind: TokenKind::KwShareClass,
        fields: &[
            required("domain", R::Domain),
            required("id", R::String),
            required("backing", R::Asset),
        ],
    },
    Rules {
        kind: TokenKind::KwInstrument,
        fields: &[
            required("domain", R::Domain),
            required("id", R::String),
            optional("asset", R::Asset),
            optional("backing", R::Asset),
            optional("underlying", R::Asset),
            optional("settlement", R::Asset),
            optional("strike_units", R::String),
            optional("strike", R::String),
            optional("exercise_round", R::Scalar),
            optional("collateral", R::Quantity),
        ],
    },
    Rules {
        kind: TokenKind::KwObservation,
        fields: &[
            required("id", R::String),
            required("domain", R::Domain),
            optional("unit", R::String),
            optional("source", R::String),
            optional("observed_round", R::Scalar),
            optional("maximum_age", R::Scalar),
            optional("finality", R::String),
            optional("provenance", R::String),
        ],
    },
    Rules {
        kind: TokenKind::KwPolicy,
        fields: &[
            optional("id", R::String),
            optional("epoch", R::Scalar),
            optional("source_hash", R::String),
            optional("duty_preservation", R::String),
        ],
    },
    Rules {
        kind: TokenKind::KwGrant,
        fields: &[
            optional("issuer", R::Account),
            optional("scope", R::String),
            optional("expiry", R::Scalar),
            optional("revocation_epoch", R::Scalar),
            optional("gross_limit", R::Quantity),
            optional("work_limit", R::Scalar),
            optional("signers", R::AccountList),
            optional("evidence", R::String),
        ],
    },
    Rules {
        kind: TokenKind::KwParty,
        fields: &[required("id", R::String), required("account", R::Account)],
    },
    Rules {
        kind: TokenKind::KwStage,
        fields: &[
            required("domain", R::Domain),
            optional("id", R::String),
            optional("trigger", R::String),
            optional("paired_claim", R::String),
            optional("authority", R::GrantOrString),
            optional("reads", R::StringList),
            optional("writes", R::StringList),
            optional("duty", R::String),
            optional("status", R::String),
            optional("amount", R::Quantity),
            optional("evidence", R::String),
            optional("relation", R::String),
            optional("signed_floor", R::QuantityOrString),
            optional("failure", R::String),
            optional("operation", R::Any),
            optional("completion", R::String),
            optional("rounding", R::String),
            optional("retained_duties", R::String),
        ],
    },
    Rules {
        kind: TokenKind::KwEpisode,
        fields: &[
            required("id", R::String),
            required("stages", R::StageList),
            optional("pending", R::String),
            optional("timeout", R::String),
            optional("recovery", R::String),
            optional("failure", R::String),
            optional("duty", R::String),
            optional("authority", R::GrantOrString),
        ],
    },
];

/// Kinds whose `id` must be a valid Source/6 identifier.
const SOURCE6_IDS: [TokenKind; 4] = [
    TokenKind::KwDomain,
    TokenKind::KwAccount,
    TokenKind::KwAsset,
    TokenKind::KwObligation,
];

/// Kinds whose `id` must be unique: per domain, or overall for domains.
const UNIQUE_IDS: [TokenKind; 5] = [
    TokenKind::KwDomain,
    TokenKind::KwAccount,
    TokenKind::KwAsset,
    TokenKind::KwObligation,
    TokenKind::KwShareClass,
];

/// Fields naming an asset that must be on the declaration's own domain.
const SAME_DOMAIN_ASSETS: [&str; 5] = [
    "asset",
    "backing",
    "underlying",
    "settlement",
    "signed_floor",
];

/// Economic identities seen so far: kind, domain and id, with the id's span.
pub type Identities = HashMap<(TokenKind, Option<DeclIdx>, String), Span>;

impl<'a> Checker<'a> {
    /// Checks an entity's fields against its kind's rules. False on any error.
    pub(crate) fn check_declaration(
        &mut self,
        kind: TokenKind,
        entity: ValueIdx,
        value_node: NodeIdx,
    ) -> bool {
        let Some(rules) = RULES.iter().find(|rules| rules.kind == kind) else {
            return true;
        };
        let fields = self.field_nodes(value_node);
        let keyword = self
            .ast
            .token_text(self.ast.main_token(self.values.node(entity)));
        let mut ok = true;

        for (key, _) in self.values.pairs(entity).collect::<Vec<_>>() {
            let name = self.ast.token_text(TokenIdx::new(usize::from(key)));
            if rules.field(name).is_none() {
                let suggestion = suggest(name, rules.fields.iter().map(|field| field.name));
                let span = self.key_span(&fields, name, value_node);
                self.report(diagnostics::unknown_field(
                    span,
                    keyword,
                    name,
                    suggestion,
                    &rules.signature(),
                ));
                ok = false;
            }
        }
        for field in rules.fields {
            let Some(value) = self.field(entity, field.name) else {
                if field.required {
                    let span = self.ast.span(value_node);
                    self.report(diagnostics::missing_field(
                        span,
                        keyword,
                        field.name,
                        &rules.signature(),
                    ));
                    ok = false;
                }
                continue;
            };
            let span = self.value_span(&fields, field.name, value_node);
            if !self.has_field_role(value, field.role) {
                let diagnostic = match self.bad_item(value, field.role) {
                    // Point at the first item of the wrong kind.
                    Some((index, item)) => {
                        let count = self.values.items(value).count();
                        let item_span = self.item_spans(&fields, field.name, count, span)[index];
                        let description = self.describe(item);
                        diagnostics::wrong_list_item(
                            item_span,
                            keyword,
                            field.name,
                            field.role,
                            &description,
                        )
                    }
                    None => {
                        let description = self.describe(value);
                        diagnostics::wrong_field(
                            span,
                            keyword,
                            field.name,
                            field.role,
                            &description,
                        )
                    }
                };
                self.report(diagnostic);
                ok = false;
            }
        }
        // Relations only make sense once every field has the right kind.
        ok && self.check_relations(kind, entity, &fields, value_node)
    }

    fn check_relations(
        &mut self,
        kind: TokenKind,
        entity: ValueIdx,
        fields: &[NodeIdx],
        value_node: NodeIdx,
    ) -> bool {
        let keyword = self
            .ast
            .token_text(self.ast.main_token(self.values.node(entity)));
        let mut ok = true;
        let own_domain = self
            .field(entity, "domain")
            .map(|domain| self.values.entity(domain));

        if kind == TokenKind::KwAsset {
            let scale = self.field(entity, "scale").expect("assets have a scale");
            if self.values.amount(scale) > MAX_SCALE {
                let span = self.value_span(fields, "scale", value_node);
                self.report(diagnostics::invalid_scale(
                    span,
                    &format!("`{}`", self.values.amount(scale)),
                ));
                ok = false;
            }
        }

        if let Some(id) = self.field(entity, "id") {
            let text = decode_string(
                self.ast
                    .token_text(TokenIdx::new(usize::from(self.values.rows.a()[id.index()]))),
            );
            let span = self.value_span(fields, "id", value_node);
            if SOURCE6_IDS.contains(&kind) && !is_source6_id(&text) {
                self.report(diagnostics::invalid_id(span, &text));
                ok = false;
            }
            if UNIQUE_IDS.contains(&kind) {
                let domain = (kind != TokenKind::KwDomain)
                    .then_some(own_domain)
                    .flatten();
                let key = (kind, domain, text.clone());
                if let Some(&earlier) = self.identities.get(&key) {
                    let place = domain.map(|domain| self.name(domain));
                    self.report(diagnostics::duplicate_id(
                        span, earlier, keyword, &text, place,
                    ));
                    ok = false;
                } else {
                    self.identities.insert(key, span);
                }
            }
        }

        if let Some(domain) = own_domain {
            for name in SAME_DOMAIN_ASSETS {
                let Some(value) = self.field(entity, name) else {
                    continue;
                };
                let asset = match self.values.tag(value) {
                    ValueTag::Entity => self.values.entity(value),
                    ValueTag::Quantity => self.values.asset(value),
                    _ => continue,
                };
                let span = self.value_span(fields, name, value_node);
                ok &= self.check_asset_domain(asset, domain, span, keyword);
            }
        }

        if kind == TokenKind::KwPool {
            let assets = self.field(entity, "assets").expect("pools have assets");
            let span = self.value_span(fields, "assets", value_node);
            let items: Vec<ValueIdx> = self.values.items(assets).collect();
            if items.is_empty() {
                self.report(diagnostics::empty_pool(span));
                return false;
            }
            let item_spans = self.item_spans(fields, "assets", items.len(), span);
            let domain = own_domain.expect("pools have a domain");
            let mut seen: Vec<(DeclIdx, Span)> = Vec::new();
            for (item, item_span) in items.into_iter().zip(item_spans) {
                let asset = self.values.entity(item);
                ok &= self.check_asset_domain(asset, domain, item_span, keyword);
                match seen.iter().find(|&&(other, _)| other == asset) {
                    Some(&(_, earlier)) => {
                        self.report(diagnostics::duplicate_pool_asset(
                            item_span,
                            earlier,
                            self.name(asset),
                        ));
                        ok = false;
                    }
                    None => seen.push((asset, item_span)),
                }
            }
        }
        ok
    }

    /// The asset's own `domain` must be `domain`.
    fn check_asset_domain(
        &mut self,
        asset: DeclIdx,
        domain: DeclIdx,
        span: Span,
        keyword: &str,
    ) -> bool {
        let Some(asset_domain) = self.declarations.value()[asset.index()]
            .and_then(|value| self.field(value, "domain"))
            .map(|field| self.values.entity(field))
        else {
            return true;
        };
        if asset_domain == domain {
            return true;
        }
        self.report(diagnostics::wrong_asset_domain(
            span,
            self.name(asset),
            self.name(asset_domain),
            keyword,
            self.name(domain),
        ));
        false
    }

    /// For a list role given a list, the first item of the wrong kind.
    fn bad_item(&self, value: ValueIdx, role: FieldRole) -> Option<(usize, ValueIdx)> {
        let kind = match role {
            FieldRole::StringList => None,
            FieldRole::AccountList => Some(TokenKind::KwAccount),
            FieldRole::StageList => Some(TokenKind::KwStage),
            FieldRole::AssetList => Some(TokenKind::KwAsset),
            _ => return None,
        };
        if self.values.tag(value) != ValueTag::List {
            return None;
        }
        self.values
            .items(value)
            .enumerate()
            .find(|&(_, item)| match kind {
                Some(kind) => {
                    self.values.tag(item) != ValueTag::Entity
                        || !self.is_kind(self.values.entity(item), kind)
                }
                None => self.values.tag(item) != ValueTag::String,
            })
    }

    fn has_field_role(&self, value: ValueIdx, role: FieldRole) -> bool {
        let tag = self.values.tag(value);
        let is_entity = |value: ValueIdx, kind: TokenKind| {
            self.values.tag(value) == ValueTag::Entity
                && self.is_kind(self.values.entity(value), kind)
        };
        let all = |kind: Option<TokenKind>, item_tag: ValueTag| {
            tag == ValueTag::List
                && self.values.items(value).all(|item| match kind {
                    Some(kind) => is_entity(item, kind),
                    None => self.values.tag(item) == item_tag,
                })
        };
        match role {
            FieldRole::String => tag == ValueTag::String,
            FieldRole::Scalar => tag == ValueTag::Scalar,
            FieldRole::Quantity => tag == ValueTag::Quantity,
            FieldRole::Asset => is_entity(value, TokenKind::KwAsset),
            FieldRole::Domain => is_entity(value, TokenKind::KwDomain),
            FieldRole::Account => is_entity(value, TokenKind::KwAccount),
            FieldRole::GrantOrString => {
                tag == ValueTag::String || is_entity(value, TokenKind::KwGrant)
            }
            FieldRole::QuantityOrString => matches!(tag, ValueTag::String | ValueTag::Quantity),
            FieldRole::StringList => all(None, ValueTag::String),
            FieldRole::AccountList => all(Some(TokenKind::KwAccount), ValueTag::Entity),
            FieldRole::StageList => all(Some(TokenKind::KwStage), ValueTag::Entity),
            FieldRole::AssetList => all(Some(TokenKind::KwAsset), ValueTag::Entity),
            FieldRole::Any => true,
        }
    }

    /// The field nodes of a declaration's value, when it is written as a
    /// record literal.
    fn field_nodes(&self, value_node: NodeIdx) -> Vec<NodeIdx> {
        match self.ast.tag(value_node) {
            NodeTag::Record => self.ast.list(value_node).collect(),
            _ => Vec::new(),
        }
    }

    fn find_field(&self, fields: &[NodeIdx], name: &str) -> Option<NodeIdx> {
        fields
            .iter()
            .copied()
            .find(|&field| self.ast.token_text(self.ast.main_token(field)) == name)
    }

    /// The span of field `name`'s key, or the whole value when it is not a
    /// record literal.
    fn key_span(&self, fields: &[NodeIdx], name: &str, value_node: NodeIdx) -> Span {
        match self.find_field(fields, name) {
            Some(field) => self.token_span(self.ast.main_token(field)),
            None => self.ast.span(value_node),
        }
    }

    /// The span of field `name`'s value, or the whole value when it is not a
    /// record literal.
    fn value_span(&self, fields: &[NodeIdx], name: &str, value_node: NodeIdx) -> Span {
        match self.find_field(fields, name) {
            Some(field) => self.ast.span(self.operand(field, 0)),
            None => self.ast.span(value_node),
        }
    }

    /// The spans of a list field's items, or the whole field for each item
    /// when the list is not written out.
    fn item_spans(
        &self,
        fields: &[NodeIdx],
        name: &str,
        count: usize,
        fallback: Span,
    ) -> Vec<Span> {
        let list = self
            .find_field(fields, name)
            .map(|field| self.operand(field, 0))
            .filter(|&node| self.ast.tag(node) == NodeTag::Array);
        match list {
            Some(node) => self
                .ast
                .list(node)
                .map(|item| self.ast.span(item))
                .collect(),
            None => vec![fallback; count],
        }
    }
}

/// Representable in Source/6: an ASCII letter, then up to 63 letters, digits
/// or `_`, and not a reserved word.
fn is_source6_id(id: &str) -> bool {
    let mut chars = id.chars();
    chars
        .next()
        .is_some_and(|first| first.is_ascii_alphabetic())
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
        && id.len() <= 64
        && !SOURCE6_RESERVED.contains(&id)
}
