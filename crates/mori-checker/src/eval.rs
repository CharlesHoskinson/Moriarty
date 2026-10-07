//! Computing each expression's value.
//!
//! Every value is a constant, computed in exact unsigned 128-bit arithmetic.
//! An expression that fails yields `None`; its parent then fails without a
//! further error, while independent siblings are still checked.

use mori_ast::{NodeIdx, NodeTag};
use mori_diagnostics::suggest;
use mori_lexer::{TokenIdx, TokenKind};
use mori_span::Span;

use crate::calls::{self, CALLS, Call, Role};
use crate::checker::Checker;
use crate::values::{ValueIdx, ValueTag};
use crate::{DeclIdx, diagnostics};

/// Largest number of decimal places an asset may have.
pub const MAX_SCALE: u128 = 18;

impl<'a> Checker<'a> {
    /// The value of the expression `node`, used by item number `order`.
    pub(crate) fn eval(&mut self, order: usize, node: NodeIdx) -> Option<ValueIdx> {
        let ast = self.ast;
        let main = ast.main_token(node);
        match ast.tag(node) {
            NodeTag::String => Some(self.values.push(ValueTag::String, node, raw(main), 0, 0)),
            NodeTag::Bool => {
                let truth = u16::from(ast.tokens.kind(main) == TokenKind::KwTrue);
                Some(self.values.push(ValueTag::Bool, node, truth, 0, 0))
            }
            NodeTag::Tag => {
                let tag = u16::from(ast.tokens.kind(main) == TokenKind::KwSuccessOnly);
                Some(self.values.push(ValueTag::Tag, node, tag, 0, 0))
            }
            NodeTag::Number => {
                let Some(amount) = parse_whole(ast.token_text(main)) else {
                    self.report(diagnostics::too_large(ast.span(node)));
                    return None;
                };
                Some(self.values.push_amount(ValueTag::Scalar, node, amount, 0))
            }
            NodeTag::Quantity => self.quantity(order, node),
            NodeTag::Reference => {
                let decl = self.resolve(order, main, node)?;
                self.declarations.value()[decl.index()]
            }
            NodeTag::Paren => self.eval(order, self.operand(node, 0)),
            NodeTag::Add | NodeTag::Sub | NodeTag::Mul => self.arithmetic(order, node),
            NodeTag::Array => {
                let items: Vec<NodeIdx> = ast.list(node).collect();
                let mut values = Vec::with_capacity(items.len());
                let mut ok = true;
                for item in items {
                    match self.eval(order, item) {
                        Some(value) => values.push(value.0),
                        None => ok = false,
                    }
                }
                ok.then(|| {
                    let (b, c) = self.values.push_extra(&values);
                    self.values.push(ValueTag::List, node, 0, b, c)
                })
            }
            NodeTag::Record => {
                let (b, c) = self.fields(order, node)?;
                Some(self.values.push(ValueTag::Record, node, 0, b, c))
            }
            NodeTag::Call => self.call(order, node),
            tag => unreachable!("{tag:?} is not an expression"),
        }
    }

    /// Evaluates the fields of a record or call into `(key, value)` pairs.
    fn fields(&mut self, order: usize, node: NodeIdx) -> Option<(u16, u16)> {
        let fields: Vec<NodeIdx> = self.ast.list(node).collect();
        let mut pairs = Vec::with_capacity(fields.len() * 2);
        let mut ok = true;
        for field in fields {
            match self.eval(order, self.operand(field, 0)) {
                Some(value) => pairs.extend([raw(self.ast.main_token(field)), value.0]),
                None => ok = false,
            }
        }
        ok.then(|| self.values.push_extra(&pairs))
    }

    /// `NUMBER ASSET`, scaled to the asset's smallest unit.
    fn quantity(&mut self, order: usize, node: NodeIdx) -> Option<ValueIdx> {
        let number = self.ast.main_token(node);
        let asset_token = self.ast.token_after(number, 1);
        let asset = self.resolve(order, asset_token, node)?;
        let asset_span = self.token_span(asset_token);
        if !self.is_kind(asset, TokenKind::KwAsset) {
            self.report(diagnostics::not_an_asset(
                asset_span,
                self.describe_decl(asset),
                self.decl_name_span(asset),
            ));
            return None;
        }
        let scale = self.asset_scale(asset)?;

        let text = self.ast.token_text(number).replace('_', "");
        let (whole, fraction) = text.split_once('.').unwrap_or((&text, ""));
        let span = self.ast.span(node);
        if fraction.len() > scale as usize {
            let fixed = format!(
                "{whole}{}{} {}",
                if scale > 0 { "." } else { "" },
                &fraction[..scale as usize],
                self.ast.token_text(asset_token)
            );
            self.report(diagnostics::too_precise(
                span,
                self.ast.token_text(asset_token),
                scale,
                fraction.len(),
                &fixed,
                self.ast.source,
            ));
            return None;
        }
        let padded = format!("{whole}{fraction:0<width$}", width = scale as usize);
        let Some(amount) = parse_whole(&padded) else {
            self.report(diagnostics::too_large(span));
            return None;
        };
        Some(
            self.values
                .push_amount(ValueTag::Quantity, node, amount, asset.0),
        )
    }

    fn arithmetic(&mut self, order: usize, node: NodeIdx) -> Option<ValueIdx> {
        let (left_node, right_node) = (self.operand(node, 0), self.operand(node, 1));
        let left = self.eval(order, left_node);
        let right = self.eval(order, right_node);
        let (left, right) = (left?, right?);
        let tag = self.ast.tag(node);

        for (value, operand) in [(left, left_node), (right, right_node)] {
            if !matches!(
                self.values.tag(value),
                ValueTag::Scalar | ValueTag::Quantity
            ) {
                self.report(diagnostics::not_a_number(
                    self.ast.span(operand),
                    &self.describe(value),
                    verb(tag),
                ));
                return None;
            }
        }
        let (left_tag, right_tag) = (self.values.tag(left), self.values.tag(right));
        let asset = match tag {
            NodeTag::Mul => {
                if left_tag == ValueTag::Quantity && right_tag == ValueTag::Quantity {
                    self.report(diagnostics::amount_times_amount(
                        self.ast.span(left_node),
                        self.ast.span(right_node),
                    ));
                    return None;
                }
                [left, right]
                    .into_iter()
                    .find(|&value| self.values.tag(value) == ValueTag::Quantity)
                    .map(|value| self.values.asset(value))
            }
            _ => {
                if !self.same_numeric_kind(left, right) {
                    let diagnostic = self.mixed(left, left_node, right, right_node, verb(tag));
                    self.report(diagnostic);
                    return None;
                }
                (left_tag == ValueTag::Quantity).then(|| self.values.asset(left))
            }
        };

        let (a, b) = (self.values.amount(left), self.values.amount(right));
        let span = self.ast.span(node);
        let result = match tag {
            NodeTag::Add => a.checked_add(b),
            NodeTag::Sub => match a.checked_sub(b) {
                Some(result) => Some(result),
                None => {
                    let shown = format!("{} - {}", self.show(left), self.show(right));
                    self.report(diagnostics::below_zero(span, &shown));
                    return None;
                }
            },
            _ => a.checked_mul(b),
        };
        let Some(result) = result else {
            self.report(diagnostics::too_large(span));
            return None;
        };
        Some(match asset {
            Some(asset) => self
                .values
                .push_amount(ValueTag::Quantity, node, result, asset.0),
            None => self.values.push_amount(ValueTag::Scalar, node, result, 0),
        })
    }

    fn call(&mut self, order: usize, node: NodeIdx) -> Option<ValueIdx> {
        let name = self.call_name(node);
        let name_span = self.call_name_span(node);
        let Some(call) = calls::find(&name) else {
            let suggestion = suggest(&name, CALLS.iter().map(|call| call.name));
            self.report(diagnostics::unknown_call(
                name_span,
                &name,
                suggestion,
                self.ast.source,
            ));
            // Still check the arguments, so their own errors are reported.
            self.fields(order, node);
            return None;
        };
        let (b, c) = self.fields(order, node)?;
        let value = self
            .values
            .push(ValueTag::Call, node, raw(self.ast.main_token(node)), b, c);
        if !self.check_arguments(call, node, value) {
            return None;
        }
        match call.name {
            "atoms" => {
                let asset = self.argument(value, "asset")?;
                let amount = self.argument(value, "value")?;
                let atoms = self.values.amount(amount);
                Some(self.values.push_amount(
                    ValueTag::Quantity,
                    node,
                    atoms,
                    self.values.entity(asset).0,
                ))
            }
            "min" | "max" => {
                let (a, b) = (self.argument(value, "a")?, self.argument(value, "b")?);
                if !self.same_numeric_kind(a, b) {
                    let fields: Vec<NodeIdx> = self.ast.list(node).collect();
                    let (a_node, b_node) = (
                        self.field_value_node(&fields, "a")?,
                        self.field_value_node(&fields, "b")?,
                    );
                    let diagnostic =
                        self.mixed(a, a_node, b, b_node, &format!("take the {} of", call.name));
                    self.report(diagnostic);
                    return None;
                }
                let pick_a = if call.name == "min" {
                    self.values.amount(a) <= self.values.amount(b)
                } else {
                    self.values.amount(a) >= self.values.amount(b)
                };
                Some(if pick_a { a } else { b })
            }
            _ => Some(value),
        }
    }

    /// Every argument is present, none is unknown, and each has its role.
    fn check_arguments(&mut self, call: &Call, node: NodeIdx, value: ValueIdx) -> bool {
        let mut ok = true;
        let fields: Vec<NodeIdx> = self.ast.list(node).collect();
        let pairs: Vec<(u16, ValueIdx)> = self.values.pairs(value).collect();
        for &field in &fields {
            let key = self.ast.token_text(self.ast.main_token(field));
            if call.role(key).is_none() {
                let suggestion = suggest(key, call.args.iter().map(|(name, _)| *name));
                self.report(diagnostics::unknown_argument(
                    self.token_span(self.ast.main_token(field)),
                    call,
                    key,
                    suggestion,
                ));
                ok = false;
            }
        }
        for &(arg, role) in call.args {
            let Some(&(_, arg_value)) = pairs
                .iter()
                .find(|(key, _)| self.ast.token_text(TokenIdx::new(usize::from(*key))) == arg)
            else {
                self.report(diagnostics::missing_argument(
                    self.call_name_span(node),
                    call,
                    arg,
                ));
                ok = false;
                continue;
            };
            if !self.has_role(arg_value, role) {
                let value_node = self
                    .field_value_node(&fields, arg)
                    .expect("present arguments have fields");
                self.report(diagnostics::wrong_argument(
                    self.ast.span(value_node),
                    call.name,
                    arg,
                    role,
                    &self.describe(arg_value),
                ));
                ok = false;
            }
        }
        ok
    }

    fn has_role(&self, value: ValueIdx, role: Role) -> bool {
        let tag = self.values.tag(value);
        match role {
            Role::Scalar => tag == ValueTag::Scalar,
            Role::Numeric => matches!(tag, ValueTag::Scalar | ValueTag::Quantity),
            Role::Quantity => tag == ValueTag::Quantity,
            Role::QuantityList => {
                tag == ValueTag::List
                    && self
                        .values
                        .items(value)
                        .all(|item| self.values.tag(item) == ValueTag::Quantity)
            }
            Role::String => tag == ValueTag::String,
            _ => {
                let kind = role.entity_kind().expect("remaining roles are entities");
                tag == ValueTag::Entity && self.is_kind(self.values.entity(value), kind)
            }
        }
    }

    /// Both scalars, or both quantities of the same asset.
    fn same_numeric_kind(&self, a: ValueIdx, b: ValueIdx) -> bool {
        let (a_tag, b_tag) = (self.values.tag(a), self.values.tag(b));
        a_tag == b_tag
            && (a_tag == ValueTag::Scalar || self.values.asset(a) == self.values.asset(b))
    }

    fn mixed(
        &self,
        left: ValueIdx,
        left_node: NodeIdx,
        right: ValueIdx,
        right_node: NodeIdx,
        verb: &str,
    ) -> mori_diagnostics::MoriDiagnostic {
        let (left_desc, right_desc) = (self.describe(left), self.describe(right));
        let (left_span, right_span) = (self.ast.span(left_node), self.ast.span(right_node));
        let number_side = [(left, left_node), (right, right_node)]
            .into_iter()
            .find(|&(value, _)| self.values.tag(value) == ValueTag::Scalar);
        let other_side = [left, right]
            .into_iter()
            .find(|&value| self.values.tag(value) == ValueTag::Quantity);
        match (number_side, other_side) {
            // A bare number next to an amount: suggest giving the number the asset.
            (Some((_, number_node)), Some(amount))
                if self.ast.tag(number_node) == NodeTag::Number =>
            {
                let asset = self.name(self.values.asset(amount));
                diagnostics::number_and_amount(
                    left_span,
                    &left_desc,
                    right_span,
                    &right_desc,
                    verb,
                    Some((self.ast.span(number_node), asset, self.ast.source)),
                )
            }
            (Some(_), Some(_)) => diagnostics::number_and_amount(
                left_span,
                &left_desc,
                right_span,
                &right_desc,
                verb,
                None,
            ),
            _ => diagnostics::mixed_assets(left_span, &left_desc, right_span, &right_desc, verb),
        }
    }

    /// The value of a call's argument `name`, when it is present.
    fn argument(&self, call: ValueIdx, name: &str) -> Option<ValueIdx> {
        self.values
            .pairs(call)
            .find(|&(key, _)| self.ast.token_text(TokenIdx::new(usize::from(key))) == name)
            .map(|(_, value)| value)
    }

    /// The expression node of the field `name` among `fields`.
    fn field_value_node(&self, fields: &[NodeIdx], name: &str) -> Option<NodeIdx> {
        fields
            .iter()
            .find(|&&field| self.ast.token_text(self.ast.main_token(field)) == name)
            .map(|&field| self.operand(field, 0))
    }

    /// The scale of an asset, when its declaration gave a valid one.
    pub(crate) fn asset_scale(&self, asset: DeclIdx) -> Option<u32> {
        let value = self.declarations.value()[asset.index()]?;
        let scale = self.field(value, "scale")?;
        (self.values.tag(scale) == ValueTag::Scalar && self.values.amount(scale) <= MAX_SCALE)
            .then(|| self.values.amount(scale) as u32)
    }

    /// The field `name` of a record, call or entity.
    pub(crate) fn field(&self, value: ValueIdx, name: &str) -> Option<ValueIdx> {
        self.values
            .pairs(value)
            .find(|&(key, _)| self.ast.token_text(TokenIdx::new(usize::from(key))) == name)
            .map(|(_, field)| field)
    }

    /// A value's kind, for messages: "a `USD` amount", "the account `alice`".
    pub(crate) fn describe(&self, value: ValueIdx) -> String {
        match self.values.tag(value) {
            ValueTag::String => "a string".to_owned(),
            ValueTag::Bool => "a true or false value".to_owned(),
            ValueTag::Scalar => "a plain number".to_owned(),
            ValueTag::Quantity => format!("a `{}` amount", self.name(self.values.asset(value))),
            ValueTag::Tag => "a tag".to_owned(),
            ValueTag::List => "a list".to_owned(),
            ValueTag::Record => "a record".to_owned(),
            ValueTag::Call => format!("a `{}` call", self.call_name(self.values.node(value))),
            ValueTag::Entity => self.describe_decl(self.values.entity(value)),
        }
    }

    /// A declaration for messages: "the account `alice`".
    pub(crate) fn describe_decl(&self, decl: DeclIdx) -> String {
        let node = self.declarations.node()[decl.index()];
        let keyword = self.ast.token_text(self.ast.main_token(node));
        format!("the {} `{}`", keyword.replace('_', " "), self.name(decl))
    }

    /// A scalar or quantity as written, such as `10.50 USD`.
    fn show(&self, value: ValueIdx) -> String {
        let amount = self.values.amount(value);
        match self.values.tag(value) {
            ValueTag::Quantity => {
                let asset = self.values.asset(value);
                let scale = self.asset_scale(asset).unwrap_or(0) as usize;
                let digits = format!("{amount:0>width$}", width = scale + 1);
                let (whole, fraction) = digits.split_at(digits.len() - scale);
                let point = if scale > 0 { "." } else { "" };
                format!("{whole}{point}{fraction} {}", self.name(asset))
            }
            _ => amount.to_string(),
        }
    }

    fn call_name(&self, node: NodeIdx) -> String {
        let mut token = self.ast.main_token(node);
        let mut name = self.ast.token_text(token).to_owned();
        while self.ast.tokens.kind(self.ast.token_after(token, 1)) == TokenKind::Dot {
            token = self.ast.token_after(token, 2);
            name.push('.');
            name.push_str(self.ast.token_text(token));
        }
        name
    }

    fn call_name_span(&self, node: NodeIdx) -> Span {
        let first = self.ast.main_token(node);
        let mut last = first;
        while self.ast.tokens.kind(self.ast.token_after(last, 1)) == TokenKind::Dot {
            last = self.ast.token_after(last, 2);
        }
        Span::new(self.token_span(first).start, self.token_span(last).end)
    }

    pub(crate) fn operand(&self, node: NodeIdx, which: usize) -> NodeIdx {
        let raw = if which == 0 {
            self.ast.lhs(node)
        } else {
            self.ast.rhs(node)
        };
        NodeIdx::from_raw(raw)
    }
}

/// How an operator reads in "I cannot add ...".
fn verb(tag: NodeTag) -> &'static str {
    match tag {
        NodeTag::Add => "add",
        NodeTag::Sub => "subtract",
        _ => "multiply",
    }
}

/// A whole number spelled with digits and `_`, if it fits in `u128`.
fn parse_whole(text: &str) -> Option<u128> {
    text.chars()
        .filter(|&c| c != '_')
        .try_fold(0u128, |total, digit| {
            total
                .checked_mul(10)?
                .checked_add(u128::from(digit.to_digit(10)?))
        })
}

fn raw(token: TokenIdx) -> u16 {
    u16::try_from(token.index()).expect("token indices fit in u16")
}
