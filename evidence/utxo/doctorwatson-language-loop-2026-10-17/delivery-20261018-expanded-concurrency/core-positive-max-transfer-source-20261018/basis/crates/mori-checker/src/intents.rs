//! Intents: what an owner signs, and the operation it authorizes.
//!
//! A `transfer` or `repay` intent can run locally (S0), so every field is
//! required and the operation must match what is signed. Family intents, such
//! as `amm.swap_exact_input`, are specified but not run; their fields are
//! optional and only checked for consistency.

use mori_ast::NodeIdx;
use mori_lexer::{TokenIdx, TokenKind, decode_string};
use mori_span::Span;

use crate::checker::Checker;
use crate::declarations::{FieldRole as R, Rules, optional, required};
use crate::values::{ValueIdx, ValueTag};
use crate::{DeclIdx, diagnostics};

/// Largest amount an S0 operation may carry: `2^127 - 1`.
pub const S0_MAX: u128 = (1 << 127) - 1;

/// The fields of a `transfer` or `repay` intent. All are required.
const S0_INTENT: Rules = Rules {
    kind: TokenKind::KwIntent,
    fields: &[
        required("domain", R::Domain),
        required("asset", R::Asset),
        required("signer", R::Account),
        required("key", R::String),
        required("nonce", R::String),
        required("pre_head", R::String),
        required("valid", R::Any),
        required("gross_cap", R::Quantity),
        required("fee_cap", R::Quantity),
        required("net_floor", R::Quantity),
        required("operation", R::Any),
        required("source_hash", R::String),
        required("policy_digest", R::String),
        required("failure", R::Any),
        required("observations", R::Any),
        required("disclosures", R::Any),
        required("retained_effects", R::Any),
        required("retained_duties", R::Any),
        required("delegation", R::Any),
        required("recovery", R::Any),
    ],
};

/// The fields of a family intent. Only `operation` is required.
const FAMILY_INTENT: Rules = Rules {
    kind: TokenKind::KwIntent,
    fields: &[
        required("operation", R::Any),
        optional("authority", R::GrantOrString),
        optional("policy", R::Policy),
        optional("nonce", R::String),
        optional("valid", R::Any),
        optional("reads", R::StringList),
        optional("writes", R::StringList),
        optional("kernel", R::String),
        optional("completion", R::String),
        optional("rounding", R::String),
        optional("relation", R::String),
        optional("failure", R::Any),
        optional("status", R::String),
        optional("observation", R::String),
        optional("continuation", R::String),
        optional("loss", R::String),
        optional("fixing", R::String),
        optional("earliest_round", R::Scalar),
        optional("veto_before", R::Scalar),
        optional("domain", R::Domain),
        optional("asset", R::Asset),
        optional("signer", R::Account),
        optional("key", R::String),
        optional("pre_head", R::String),
        optional("gross_cap", R::Quantity),
        optional("fee_cap", R::Quantity),
        optional("net_floor", R::Quantity),
        optional("source_hash", R::String),
        optional("policy_digest", R::String),
        optional("observations", R::Any),
        optional("disclosures", R::Any),
        optional("retained_effects", R::Any),
        optional("retained_duties", R::StringOrStringList),
        optional("delegation", R::Any),
        optional("recovery", R::Any),
    ],
};

/// Claims an S0 intent binds into what the owner signs; none may be empty.
const S0_CLAIMS: [&str; 5] = ["key", "nonce", "pre_head", "source_hash", "policy_digest"];
const CAPS: [&str; 3] = ["gross_cap", "fee_cap", "net_floor"];

/// What an action's intent can do.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Support {
    /// A `transfer` or `repay`, prepared locally.
    LocalS0,
    /// A family operation: checked, but not run.
    SpecifiedOnly,
}

impl<'a> Checker<'a> {
    /// Checks an intent's fields and its operation. False on any error.
    pub(crate) fn check_intent(&mut self, intent: ValueIdx, value_node: NodeIdx) -> bool {
        let fields = self.field_nodes(value_node);
        let intent_span = self.ast.span(value_node);
        let Some(operation) = self.field(intent, "operation") else {
            self.report(diagnostics::intent_without_operation(intent_span));
            return false;
        };
        let operation_span = self.value_span(&fields, "operation", value_node);
        if self.values.tag(operation) != ValueTag::Call {
            let description = self.describe(operation);
            self.report(diagnostics::operation_not_a_call(
                operation_span,
                &description,
            ));
            return false;
        }
        let name = self.call_name(self.values.node(operation));
        if name.contains('.') {
            return self.check_fields(&FAMILY_INTENT, intent, value_node, &fields)
                && self.check_family_intent(intent, operation, &name, value_node, &fields);
        }
        if !matches!(name.as_str(), "transfer" | "repay") {
            self.report(diagnostics::unsupported_operation(operation_span, &name));
            return false;
        }
        self.check_fields(&S0_INTENT, intent, value_node, &fields)
            && self.check_s0_intent(intent, operation, &name, value_node, &fields)
    }

    fn check_s0_intent(
        &mut self,
        intent: ValueIdx,
        operation: ValueIdx,
        name: &str,
        value_node: NodeIdx,
        fields: &[NodeIdx],
    ) -> bool {
        let mut ok = true;
        let span = |checker: &Self, field: &str| checker.value_span(fields, field, value_node);
        let domain = self
            .values
            .entity(self.field(intent, "domain").expect("checked"));
        let domain_span = span(self, "domain");
        let asset_value = self.field(intent, "asset").expect("checked");
        let asset = self.values.entity(asset_value);
        let signer_value = self.field(intent, "signer").expect("checked");
        let signer = self.values.entity(signer_value);

        for role in ["asset", "signer"] {
            let value = self.field(intent, role).expect("checked");
            ok &= self.check_same_domain(
                value,
                span(self, role),
                role,
                domain,
                domain_span,
                "domain",
            );
        }
        for claim in S0_CLAIMS {
            let value = self.field(intent, claim).expect("checked");
            if self.string_text(value).is_empty() {
                self.report(diagnostics::empty_claim(span(self, claim), claim));
                ok = false;
            }
        }
        ok &= self.check_validity(
            intent,
            span(self, "valid"),
            Some((domain, domain_span)),
            true,
        );
        for cap in CAPS {
            ok &= self.check_amount(
                intent,
                cap,
                span(self, cap),
                asset,
                "it is a cap on the intent's `asset`",
                true,
            );
        }

        let policies: [(&str, &str, Expectation); 7] = [
            ("failure", "SuccessOnly", Expectation::Tag(1)),
            ("observations", "[]", Expectation::EmptyList),
            ("disclosures", "[]", Expectation::EmptyList),
            ("retained_effects", "[]", Expectation::EmptyList),
            ("retained_duties", "[]", Expectation::EmptyList),
            ("delegation", "None", Expectation::Tag(0)),
            ("recovery", "None", Expectation::Tag(0)),
        ];
        for (field, written, expected) in policies {
            let value = self.field(intent, field).expect("checked");
            if !self.matches(value, expected) {
                let description = self.describe(value);
                self.report(diagnostics::s0_policy(
                    span(self, field),
                    field,
                    written,
                    &description,
                ));
                ok = false;
            }
        }

        // The operation must be what the owner signs for.
        let operation_node = self.values.node(operation);
        let arg = |checker: &Self, role: &str| checker.call_arg_span(operation_node, role);
        if name == "transfer" {
            for role in ["from", "to", "fee_to"] {
                let account = self.field(operation, role).expect("checked");
                ok &= self.check_same_domain(
                    account,
                    arg(self, role),
                    role,
                    domain,
                    domain_span,
                    "domain",
                );
            }
            let from = self
                .values
                .entity(self.field(operation, "from").expect("checked"));
            if from != signer {
                self.report(diagnostics::wrong_signer(
                    span(self, "signer"),
                    self.name(signer),
                    "from",
                    self.name(from),
                    arg(self, "from"),
                ));
                ok = false;
            }
            for role in ["value", "fee"] {
                ok &= self.check_amount(
                    operation,
                    role,
                    arg(self, role),
                    asset,
                    "it must be in the intent's `asset`",
                    true,
                );
            }
        } else {
            let loan = self.field(operation, "obligation").expect("checked");
            let loan_span = arg(self, "obligation");
            ok &= self.check_same_domain(
                loan,
                loan_span,
                "obligation",
                domain,
                domain_span,
                "domain",
            );
            if let Some(owed) = self.entity_field(loan, "asset")
                && owed != asset
            {
                let because = format!("the intent's `asset` is `{}`", self.name(asset));
                self.report(diagnostics::wrong_obligation_asset(
                    loan_span,
                    self.name(owed),
                    &because,
                ));
                ok = false;
            }
            let payer = self
                .values
                .entity(self.field(operation, "payer").expect("checked"));
            if payer != signer {
                self.report(diagnostics::wrong_signer(
                    span(self, "signer"),
                    self.name(signer),
                    "payer",
                    self.name(payer),
                    arg(self, "payer"),
                ));
                ok = false;
            }
            ok &= self.check_amount(
                operation,
                "amount",
                arg(self, "amount"),
                asset,
                "it must be in the intent's `asset`",
                true,
            );
        }
        ok
    }

    fn check_family_intent(
        &mut self,
        intent: ValueIdx,
        operation: ValueIdx,
        name: &str,
        value_node: NodeIdx,
        fields: &[NodeIdx],
    ) -> bool {
        let mut ok = true;
        let span = |checker: &Self, field: &str| checker.value_span(fields, field, value_node);
        let operation_node = self.values.node(operation);
        let bridge = name.starts_with("bridge.");

        // The header and the operation's local arguments share one domain.
        let mut header: Vec<(String, ValueIdx, Span)> = Vec::new();
        for role in ["domain", "asset", "signer"] {
            if let Some(value) = self.field(intent, role) {
                header.push((role.to_owned(), value, span(self, role)));
            }
        }
        for (key, value) in self.values.pairs(operation).collect::<Vec<_>>() {
            let role = self.ast.token_text(TokenIdx::new(usize::from(key)));
            if bridge && matches!(role, "source" | "destination") {
                continue;
            }
            header.push((
                role.to_owned(),
                value,
                self.call_arg_span(operation_node, role),
            ));
        }
        let mut domain: Option<(DeclIdx, Span, String)> = None;
        for (role, value, value_span) in &header {
            let Some(this) = self.domain_of(*value) else {
                continue;
            };
            match &domain {
                None => domain = Some((this, *value_span, role.clone())),
                Some((first, first_span, first_role)) => {
                    let (first, first_span, first_role) = (*first, *first_span, first_role.clone());
                    ok &= self.check_same_domain(
                        *value,
                        *value_span,
                        role,
                        first,
                        first_span,
                        &first_role,
                    );
                }
            }
        }

        let bridge_asset = bridge
            .then(|| self.field(operation, "amount"))
            .flatten()
            .filter(|&amount| self.values.tag(amount) == ValueTag::Quantity)
            .map(|amount| self.values.asset(amount));
        let declared = self
            .field(intent, "asset")
            .map(|asset| self.values.entity(asset));
        if let (Some(moved), Some(declared)) = (bridge_asset, declared)
            && moved != declared
        {
            let because = format!(
                "the bridge moves its `amount`, which is in `{}`",
                self.name(moved)
            );
            self.report(diagnostics::wrong_intent_asset(
                span(self, "asset"),
                self.name(declared),
                &because,
            ));
            ok = false;
        }
        let bound = declared.or(bridge_asset);

        let rounds_domain = domain.as_ref().map(|(domain, span, _)| (*domain, *span));
        if self.field(intent, "valid").is_some() {
            ok &= self.check_validity(intent, span(self, "valid"), rounds_domain, false);
        }
        for cap in CAPS {
            let unit = match (cap, name) {
                ("net_floor", "amm.swap_exact_input") => self
                    .field(operation, "output_asset")
                    .map(|asset| self.values.entity(asset)),
                _ => bound,
            };
            if let Some(unit) = unit {
                let because = if cap == "net_floor" && name == "amm.swap_exact_input" {
                    "a swap's floor is measured in its `output_asset`"
                } else {
                    "it is a cap on the intent's asset"
                };
                ok &= self.check_amount(intent, cap, span(self, cap), unit, because, false);
            }
            if let (Some(value), Some((domain, first_span, first_role))) =
                (self.field(intent, cap), &domain)
            {
                let (domain, first_span, first_role) = (*domain, *first_span, first_role.clone());
                ok &= self.check_same_domain(
                    value,
                    span(self, cap),
                    cap,
                    domain,
                    first_span,
                    &first_role,
                );
            }
        }
        ok
    }

    /// `valid` is a `rounds(...)` window on the intent's domain; family
    /// intents may also give two round numbers.
    fn check_validity(
        &mut self,
        intent: ValueIdx,
        span: Span,
        domain: Option<(DeclIdx, Span)>,
        s0: bool,
    ) -> bool {
        let valid = self.field(intent, "valid").expect("validity is present");
        match self.values.tag(valid) {
            ValueTag::Call if self.call_name(self.values.node(valid)) == "rounds" => {
                let window = self.field(valid, "domain").expect("rounds has a domain");
                match domain {
                    Some((domain, domain_span)) => {
                        self.check_same_domain(window, span, "valid", domain, domain_span, "domain")
                    }
                    None => true,
                }
            }
            ValueTag::List if !s0 => {
                let items: Vec<ValueIdx> = self.values.items(valid).collect();
                let scalars = items
                    .iter()
                    .all(|&item| self.values.tag(item) == ValueTag::Scalar);
                if items.len() == 2 && scalars {
                    return true;
                }
                self.report(diagnostics::invalid_validity(span, s0));
                false
            }
            _ => {
                self.report(diagnostics::invalid_validity(span, s0));
                false
            }
        }
    }

    /// The field `role` of `value`, when present, is an amount of `asset`,
    /// within the S0 maximum when `s0` is set.
    fn check_amount(
        &mut self,
        value: ValueIdx,
        role: &str,
        span: Span,
        asset: DeclIdx,
        because: &str,
        s0: bool,
    ) -> bool {
        let Some(amount) = self.field(value, role) else {
            return true;
        };
        let actual = self.values.asset(amount);
        if actual != asset {
            let what = format!("`{role}`");
            self.report(diagnostics::wrong_asset(
                span,
                &what,
                self.name(asset),
                self.name(actual),
                because,
            ));
            return false;
        }
        if s0 && self.values.amount(amount) > S0_MAX {
            self.report(diagnostics::too_large_for_s0(span, role));
            return false;
        }
        true
    }

    fn matches(&self, value: ValueIdx, expected: Expectation) -> bool {
        match expected {
            Expectation::Tag(tag) => {
                self.values.tag(value) == ValueTag::Tag
                    && self.values.rows.a()[value.index()] == tag
            }
            Expectation::EmptyList => {
                self.values.tag(value) == ValueTag::List
                    && self.values.items(value).next().is_none()
            }
        }
    }

    fn string_text(&self, value: ValueIdx) -> String {
        let token = TokenIdx::new(usize::from(self.values.rows.a()[value.index()]));
        decode_string(self.ast.token_text(token))
    }

    /// The span of argument `role` of the call at `node`, or the whole value
    /// when the call is not written out there.
    fn call_arg_span(&self, node: NodeIdx, role: &str) -> Span {
        if self.ast.tag(node) == mori_ast::NodeTag::Call
            && let Some(field) = self
                .ast
                .list(node)
                .find(|&field| self.ast.token_text(self.ast.main_token(field)) == role)
        {
            return self.ast.span(self.operand(field, 0));
        }
        self.ast.span(node)
    }

    /// What the action's intent can do, from its operation.
    pub(crate) fn support(&self, intent: DeclIdx) -> Option<Support> {
        let value = self.declarations.value()[intent.index()]?;
        let operation = self.field(value, "operation")?;
        let name = self.call_name(self.values.node(operation));
        Some(if name.contains('.') {
            Support::SpecifiedOnly
        } else {
            Support::LocalS0
        })
    }
}

/// A value an S0 intent requires exactly.
#[derive(Debug, Clone, Copy)]
enum Expectation {
    /// The tag `None` (`0`) or `SuccessOnly` (`1`).
    Tag(u16),
    EmptyList,
}
