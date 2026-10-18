//! Rules between a call's arguments, once each argument has the right kind.

use mori_ast::NodeIdx;
use mori_span::Span;

use crate::calls::Call;
use crate::checker::Checker;
use crate::values::{ValueIdx, ValueTag};
use crate::{DeclIdx, diagnostics};

/// A call's arguments, with the source span of each value.
struct Args {
    pairs: Vec<(String, ValueIdx, Span)>,
}

impl Args {
    fn get(&self, name: &str) -> Option<(ValueIdx, Span)> {
        self.pairs
            .iter()
            .find(|(key, _, _)| key == name)
            .map(|&(_, value, span)| (value, span))
    }

    fn value(&self, name: &str) -> ValueIdx {
        self.get(name).expect("checked calls have every argument").0
    }
}

impl<'a> Checker<'a> {
    /// Checks the rules between `call`'s arguments. False on any error.
    pub(crate) fn check_relations_of_call(
        &mut self,
        call: &Call,
        node: NodeIdx,
        value: ValueIdx,
    ) -> bool {
        let fields: Vec<NodeIdx> = self.ast.list(node).collect();
        let pairs = fields
            .iter()
            .map(|&field| {
                let key = self.ast.token_text(self.ast.main_token(field)).to_owned();
                let field_value = self.field(value, &key).expect("fields were evaluated");
                (key, field_value, self.ast.span(self.operand(field, 0)))
            })
            .collect();
        let args = Args { pairs };
        let name = call.name;

        match name {
            "rounds" => self.check_rounds(&args),
            "transfer" => self.check_transfer(&args),
            "repay" => self.check_repay(&args),
            _ if name.contains('.') => {
                let family = self.check_family(name, &args, node);
                // Separate checks: report both kinds of mistake.
                let domains = self.check_one_domain(name, &args);
                family && domains
            }
            _ => true,
        }
    }

    fn check_rounds(&mut self, args: &Args) -> bool {
        let (from, from_span) = args.get("from").expect("rounds has from");
        let (to, to_span) = args.get("to").expect("rounds has to");
        let (start, end) = (self.values.amount(from), self.values.amount(to));
        if start <= end {
            return true;
        }
        self.report(diagnostics::inverted_rounds(from_span, start, to_span, end));
        false
    }

    fn check_transfer(&mut self, args: &Args) -> bool {
        let mut ok = true;
        let roles = ["from", "to", "fee_to"];
        let accounts: Vec<(&str, DeclIdx, Span)> = roles
            .iter()
            .map(|&role| {
                let (value, span) = args.get(role).expect("transfer has its accounts");
                (role, self.values.entity(value), span)
            })
            .collect();
        for (index, &(role, account, span)) in accounts.iter().enumerate() {
            if let Some(&(earlier_role, _, earlier_span)) = accounts[..index]
                .iter()
                .find(|&&(_, other, _)| other == account)
            {
                self.report(diagnostics::same_account(
                    earlier_span,
                    earlier_role,
                    span,
                    role,
                    self.name(account),
                ));
                ok = false;
            }
        }

        let (first_role, _, first_span) = accounts[0];
        let Some(domain) = self.domain_of(args.value(first_role)) else {
            return ok;
        };
        for &(role, _, span) in &accounts[1..] {
            ok &= self.check_same_domain(
                args.value(role),
                span,
                role,
                domain,
                first_span,
                first_role,
            );
        }

        let (value, value_span) = args.get("value").expect("transfer has a value");
        let (fee, fee_span) = args.get("fee").expect("transfer has a fee");
        if self.values.asset(fee) != self.values.asset(value) {
            let because = "it must match `value`";
            self.report(diagnostics::wrong_asset(
                fee_span,
                "`fee`",
                self.name(self.values.asset(value)),
                self.name(self.values.asset(fee)),
                because,
            ));
            ok = false;
        }
        ok & self.check_same_domain(value, value_span, "value", domain, first_span, first_role)
    }

    fn check_repay(&mut self, args: &Args) -> bool {
        let mut ok = true;
        let loan = args.value("obligation");
        let (amount, amount_span) = args.get("amount").expect("repay has an amount");
        if let Some(asset) = self.entity_field(loan, "asset") {
            let paid = self.values.asset(amount);
            if paid != asset {
                let because = format!("the obligation is owed in `{}`", self.name(asset));
                self.report(diagnostics::wrong_asset(
                    amount_span,
                    "`amount`",
                    self.name(asset),
                    self.name(paid),
                    &because,
                ));
                ok = false;
            }
        }
        let (_, loan_span) = args.get("obligation").expect("repay has an obligation");
        let (payer, payer_span) = args.get("payer").expect("repay has a payer");
        if let Some(domain) = self.domain_of(loan) {
            ok &=
                self.check_same_domain(payer, payer_span, "payer", domain, loan_span, "obligation");
        }
        ok
    }

    /// Asset rules specific to each family of calls.
    fn check_family(&mut self, name: &str, args: &Args, node: NodeIdx) -> bool {
        let mut ok = true;
        let family = name.split('.').next().expect("family calls are dotted");
        match family {
            "amm" => {
                let pool = args.value("pool");
                let pool_name = self.name(self.values.entity(pool));
                let members: Vec<DeclIdx> = self
                    .field(pool, "assets")
                    .map(|assets| {
                        self.values
                            .items(assets)
                            .map(|item| self.values.entity(item))
                            .collect()
                    })
                    .unwrap_or_default();
                if name == "amm.swap_exact_input" {
                    let output = self.values.entity(args.value("output_asset"));
                    let input = self.values.asset(args.value("input"));
                    ok &= self.check_asset(
                        args,
                        "net_floor",
                        output,
                        "it is measured in `output_asset`",
                    );
                    ok &= self.check_asset(
                        args,
                        "fee_cap",
                        input,
                        "fees are paid in the `input` asset",
                    );
                }
                let mut used: Vec<(DeclIdx, Span)> = Vec::new();
                for role in ["input", "output_asset"] {
                    if let Some((value, span)) = args.get(role) {
                        used.push((self.asset_of(value), span));
                    }
                }
                if let Some((amounts, span)) = args.get("amounts") {
                    let items: Vec<ValueIdx> = self.values.items(amounts).collect();
                    let spans = self.list_item_spans(node, "amounts", items.len(), span);
                    for (item, item_span) in items.into_iter().zip(spans) {
                        used.push((self.values.asset(item), item_span));
                    }
                }
                for (asset, span) in used {
                    if !members.contains(&asset) {
                        self.report(diagnostics::not_in_pool(span, self.name(asset), pool_name));
                        ok = false;
                    }
                }
            }
            "lending" if name == "lending.originate" => {
                if let Some(asset) = self.entity_field(args.value("obligation"), "asset") {
                    ok &= self.check_asset(
                        args,
                        "principal",
                        asset,
                        "the obligation is owed in that asset",
                    );
                }
            }
            "stablecoin" | "option" => {
                let instrument = args.value("instrument");
                let needed: &[(&str, &[&str])] = if family == "stablecoin" {
                    &[
                        ("asset", &["supply", "burn", "claim"]),
                        ("backing", &["backing", "minimum_backing"]),
                    ]
                } else {
                    &[("underlying", &[]), ("settlement", &["payoff"])]
                };
                let call_span = self.ast.span(node);
                for &(field, roles) in needed {
                    let Some(asset) = self.entity_field(instrument, field) else {
                        let instrument_name = self.name(self.values.entity(instrument));
                        self.report(diagnostics::instrument_missing_field(
                            call_span,
                            name,
                            instrument_name,
                            field,
                        ));
                        ok = false;
                        continue;
                    };
                    for role in roles {
                        let because = format!("it is the instrument's `{field}`");
                        ok &= self.check_asset(args, role, asset, &because);
                    }
                }
            }
            "staking" => {
                if let Some(backing) = self.entity_field(args.value("shares"), "backing") {
                    for role in ["backing", "amount", "minimum_backing"] {
                        ok &= self.check_asset(
                            args,
                            role,
                            backing,
                            "it is the share class's `backing`",
                        );
                    }
                }
            }
            _ => {}
        }
        ok
    }

    /// Every argument of a family call is on one domain, except a bridge's
    /// `source` and `destination`; amounts must be in assets on that domain.
    fn check_one_domain(&mut self, name: &str, args: &Args) -> bool {
        let bridge = name.starts_with("bridge.");
        let mut expected: Option<(DeclIdx, Span, &str)> = None;
        let mut ok = true;
        for (role, value, span) in &args.pairs {
            if bridge && matches!(role.as_str(), "source" | "destination") {
                continue;
            }
            if self.values.tag(*value) != ValueTag::Entity {
                continue;
            }
            let Some(domain) = self.domain_of(*value) else {
                continue;
            };
            match expected {
                None => expected = Some((domain, *span, role)),
                Some((first, first_span, first_role)) => {
                    ok &=
                        self.check_same_domain(*value, *span, role, first, first_span, first_role);
                }
            }
        }
        let Some((domain, first_span, first_role)) = expected else {
            return ok;
        };
        for (role, value, span) in &args.pairs {
            let amounts: Vec<ValueIdx> = match self.values.tag(*value) {
                ValueTag::Quantity => vec![*value],
                ValueTag::List => self.values.items(*value).collect(),
                _ => continue,
            };
            for amount in amounts {
                if self.values.tag(amount) == ValueTag::Quantity {
                    ok &=
                        self.check_same_domain(amount, *span, role, domain, first_span, first_role);
                }
            }
        }
        ok
    }

    /// The argument `role`, when present, is an amount of `asset`.
    fn check_asset(&mut self, args: &Args, role: &str, asset: DeclIdx, because: &str) -> bool {
        let Some((value, span)) = args.get(role) else {
            return true;
        };
        let actual = self.values.asset(value);
        if actual == asset {
            return true;
        }
        let what = format!("`{role}`");
        self.report(diagnostics::wrong_asset(
            span,
            &what,
            self.name(asset),
            self.name(actual),
            because,
        ));
        false
    }

    /// `value` is on `domain`, the domain of the argument `first_role`.
    pub(crate) fn check_same_domain(
        &mut self,
        value: ValueIdx,
        span: Span,
        role: &str,
        domain: DeclIdx,
        first_span: Span,
        first_role: &str,
    ) -> bool {
        let Some(actual) = self.domain_of(value) else {
            return true;
        };
        if actual == domain {
            return true;
        }
        self.report(diagnostics::different_domains(
            span,
            role,
            self.name(actual),
            first_span,
            first_role,
            self.name(domain),
        ));
        false
    }

    /// The domain a value lives on: a domain itself, an entity's `domain`
    /// field, or an amount's asset's domain.
    pub(crate) fn domain_of(&self, value: ValueIdx) -> Option<DeclIdx> {
        match self.values.tag(value) {
            ValueTag::Entity => {
                let decl = self.values.entity(value);
                if self.is_kind(decl, mori_lexer::TokenKind::KwDomain) {
                    Some(decl)
                } else {
                    self.entity_field(value, "domain")
                }
            }
            ValueTag::Quantity => {
                let asset = self.declarations.value()[self.values.asset(value).index()]?;
                self.entity_field(asset, "domain")
            }
            _ => None,
        }
    }

    /// The asset an amount is in, or the asset itself.
    fn asset_of(&self, value: ValueIdx) -> DeclIdx {
        match self.values.tag(value) {
            ValueTag::Quantity => self.values.asset(value),
            _ => self.values.entity(value),
        }
    }

    /// The declaration an entity field names, such as an obligation's `asset`.
    pub(crate) fn entity_field(&self, value: ValueIdx, name: &str) -> Option<DeclIdx> {
        let field = self.field(value, name)?;
        (self.values.tag(field) == ValueTag::Entity).then(|| self.values.entity(field))
    }

    /// The spans of a list argument's items, or the whole argument for each.
    fn list_item_spans(
        &self,
        node: NodeIdx,
        name: &str,
        count: usize,
        fallback: Span,
    ) -> Vec<Span> {
        let list = self
            .ast
            .list(node)
            .find(|&field| self.ast.token_text(self.ast.main_token(field)) == name)
            .map(|field| self.operand(field, 0))
            .filter(|&value| self.ast.tag(value) == mori_ast::NodeTag::Array);
        match list {
            Some(list) => self
                .ast
                .list(list)
                .map(|item| self.ast.span(item))
                .collect(),
            None => vec![fallback; count],
        }
    }
}
