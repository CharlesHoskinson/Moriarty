use mori_ast::NodeIdx;
use mori_checker::{Checked, Support, ValueIdx, ValueTag};
use mori_diagnostics::MoriDiagnostic;
use mori_lexer::{TokenIdx, TokenKind, decode_string};
use mori_span::Span;

use crate::diagnostics;

type Result<T> = std::result::Result<T, MoriDiagnostic>;

/// A balance cell supplied as an unverified local claim.
#[derive(Debug, Clone, Copy)]
pub struct Balance<'a> {
    pub account: &'a str,
    pub amount: u128,
}

/// Explicit opening counters; expansion never resets or consumes them.
#[derive(Debug, Clone, Copy)]
pub struct Allowance<'a> {
    pub owner: &'a str,
    pub remaining: u128,
    pub spent: u128,
}

#[derive(Debug, Clone, Copy)]
pub enum Replay {
    Unused,
    Consumed,
}

/// Typed local scenario for the default transfer proposal only.
///
/// Balance order is owner, recipient, fee recipient. All three cells are
/// supplied even for a zero fee. This is not a JSON wire format, authenticated
/// snapshot or candidate-effects override API.
#[derive(Debug, Clone, Copy)]
pub struct LocalTransferScenario<'a> {
    pub domain: &'a str,
    pub asset: &'a str,
    pub head: &'a str,
    pub predecessor: &'a str,
    pub round: u128,
    pub balances: [Balance<'a>; 3],
    pub allowance: Allowance<'a>,
    pub replay: Replay,
    pub work_remaining: u128,
    pub work_spent: u128,
    pub post_head: &'a str,
}

/// Generated source text, with no Core result or published post-state.
#[derive(Debug, PartialEq, Eq)]
pub struct TransferExpansion {
    pub source6: String,
    pub qualification: &'static str,
}

/// A source-selected transfer, produced only after the existing frontend checks.
/// The owned presentation terms permit scenario I/O to happen after selection.
pub struct SelectedTransfer {
    terms: Terms,
    span: Span,
}

impl SelectedTransfer {
    /// Shape explicit local cells and emit a proposal, without Core preparation.
    pub fn expand(
        &self,
        scenario: &LocalTransferScenario<'_>,
    ) -> std::result::Result<TransferExpansion, MoriDiagnostic> {
        validate_scenario(&self.terms, scenario, self.span)?;
        let source6 = emit(&self.terms, scenario, self.span)?;
        Ok(TransferExpansion {
            source6,
            qualification: "local-stipulation-only",
        })
    }
}

/// Parse, check and select a beta transfer before a caller reads its scenario.
/// Existing checker diagnostics and unsupported-action selection are retained.
pub fn select_transfer(
    source: &str,
    action: &str,
) -> std::result::Result<SelectedTransfer, Vec<MoriDiagnostic>> {
    let ast = mori_parser::parse(source).map_err(|error| vec![error])?;
    let result = mori_checker::check(&ast);
    if !result.is_accepted() {
        return Err(result.diagnostics);
    }
    let (terms, span) = extract(&result.checked, action).map_err(|error| vec![error])?;
    Ok(SelectedTransfer { terms, span })
}

/// Parse and check beta source, then emit its selected transfer proposal.
/// Quantities and claims come from the unchanged checked transfer mapping.
/// Call select_transfer first when scenario I/O must follow source selection.
pub fn expand_transfer(
    source: &str,
    action: &str,
    scenario: &LocalTransferScenario<'_>,
) -> std::result::Result<TransferExpansion, Vec<MoriDiagnostic>> {
    select_transfer(source, action)?
        .expand(scenario)
        .map_err(|error| vec![error])
}

struct Terms {
    agreement: String,
    domain: String,
    asset: String,
    scale: u128,
    signer: String,
    owner: String,
    recipient: String,
    fee_recipient: String,
    key: String,
    nonce: String,
    pre_head: String,
    valid_from: u128,
    valid_to: u128,
    gross_cap: u128,
    fee_cap: u128,
    net_floor: u128,
    value: u128,
    fee: u128,
    gross: u128,
    source_hash: String,
    policy_digest: String,
}

fn field(checked: &Checked<'_>, value: ValueIdx, name: &str) -> Result<ValueIdx> {
    checked
        .values
        .pairs(value)
        .find(|&(key, _)| checked.ast.token_text(TokenIdx::new(usize::from(key))) == name)
        .map(|(_, field)| field)
        .ok_or_else(|| {
            diagnostics::shape(
                checked.ast.span(checked.values.node(value)),
                &format!("the checked `{name}` field"),
            )
        })
}

fn string(checked: &Checked<'_>, value: ValueIdx) -> Result<String> {
    if checked.values.tag(value) != ValueTag::String {
        return Err(diagnostics::shape(
            checked.ast.span(checked.values.node(value)),
            "a checked string",
        ));
    }
    let token = TokenIdx::new(usize::from(checked.values.rows.a()[value.index()]));
    Ok(decode_string(checked.ast.token_text(token)))
}

fn number(checked: &Checked<'_>, value: ValueIdx, tag: ValueTag) -> Result<u128> {
    if checked.values.tag(value) != tag {
        return Err(diagnostics::shape(
            checked.ast.span(checked.values.node(value)),
            "a checked scalar or nominal quantity in its declared role",
        ));
    }
    Ok(checked.values.amount(value))
}

fn id(checked: &Checked<'_>, entity: ValueIdx) -> Result<String> {
    string(checked, field(checked, entity, "id")?)
}

fn call_name(checked: &Checked<'_>, value: ValueIdx) -> String {
    let mut token = checked.ast.main_token(checked.values.node(value));
    let mut name = checked.ast.token_text(token).to_owned();
    while checked.ast.tokens.kind(checked.ast.token_after(token, 1)) == TokenKind::Dot {
        token = checked.ast.token_after(token, 2);
        name.push('.');
        name.push_str(checked.ast.token_text(token));
    }
    name
}

fn extract(checked: &Checked<'_>, action: &str) -> Result<(Terms, Span)> {
    let ast = checked.ast;
    let index = checked
        .actions
        .node()
        .iter()
        .position(|&node| ast.token_text(ast.token_after(ast.main_token(node), 1)) == action);
    let index = index.ok_or_else(|| diagnostics::action(ast.span(NodeIdx::ROOT), action))?;
    let node = checked.actions.node()[index];
    let span = ast.span(node);
    if checked.actions.support()[index] != Some(Support::LocalS0) {
        return Err(diagnostics::unsupported(span));
    }
    let intent = checked.actions.intent()[index]
        .and_then(|decl| checked.value(decl))
        .ok_or_else(|| diagnostics::shape(span, "a resolved checked intent"))?;
    let operation = field(checked, intent, "operation")?;
    if call_name(checked, operation) != "transfer" {
        return Err(diagnostics::unsupported(span));
    }
    let asset = field(checked, intent, "asset")?;
    let validity = field(checked, intent, "valid")?;
    let quantity =
        |record, name| number(checked, field(checked, record, name)?, ValueTag::Quantity);
    let scalar = |record, name| number(checked, field(checked, record, name)?, ValueTag::Scalar);
    let text = |name| string(checked, field(checked, intent, name)?);
    let value = quantity(operation, "value")?;
    let fee = quantity(operation, "fee")?;
    let gross = value
        .checked_add(fee)
        .ok_or_else(|| diagnostics::shape(span, "a UInt128 transfer total"))?;
    Ok((
        Terms {
            agreement: ast.token_text(ast.main_token(NodeIdx::ROOT)).to_owned(),
            domain: id(checked, field(checked, intent, "domain")?)?,
            asset: id(checked, asset)?,
            scale: scalar(asset, "scale")?,
            signer: id(checked, field(checked, intent, "signer")?)?,
            owner: id(checked, field(checked, operation, "from")?)?,
            recipient: id(checked, field(checked, operation, "to")?)?,
            fee_recipient: id(checked, field(checked, operation, "fee_to")?)?,
            key: text("key")?,
            nonce: text("nonce")?,
            pre_head: text("pre_head")?,
            valid_from: scalar(validity, "from")?,
            valid_to: scalar(validity, "to")?,
            gross_cap: quantity(intent, "gross_cap")?,
            fee_cap: quantity(intent, "fee_cap")?,
            net_floor: quantity(intent, "net_floor")?,
            value,
            fee,
            gross,
            source_hash: text("source_hash")?,
            policy_digest: text("policy_digest")?,
        },
        span,
    ))
}

fn validate_scenario(terms: &Terms, s: &LocalTransferScenario<'_>, span: Span) -> Result<()> {
    for (field, actual, expected) in [
        ("domain", s.domain, terms.domain.as_str()),
        ("asset", s.asset, terms.asset.as_str()),
        ("allowance.owner", s.allowance.owner, terms.signer.as_str()),
    ] {
        if actual != expected {
            return Err(diagnostics::scenario(span, field, expected));
        }
    }
    for (index, (balance, expected)) in s
        .balances
        .iter()
        .zip([
            terms.owner.as_str(),
            terms.recipient.as_str(),
            terms.fee_recipient.as_str(),
        ])
        .enumerate()
    {
        if balance.account != expected {
            return Err(diagnostics::scenario(
                span,
                &format!("balances[{index}].account"),
                expected,
            ));
        }
    }
    for (field, text) in [
        ("head", s.head),
        ("predecessor", s.predecessor),
        ("post_head", s.post_head),
    ] {
        if text.is_empty() || text.len() > mori_lexer::MAX_STRING_BYTES {
            return Err(diagnostics::scenario(
                span,
                field,
                "nonempty text of at most 1024 UTF-8 bytes",
            ));
        }
    }
    if s.allowance
        .remaining
        .checked_add(s.allowance.spent)
        .is_none()
        || s.work_remaining.checked_add(s.work_spent).is_none()
    {
        return Err(diagnostics::scenario(
            span,
            "counters",
            "opening totals that fit UInt128",
        ));
    }
    Ok(())
}

fn quote(text: &str) -> String {
    serde_json::to_string(text).expect("serializing a UTF-8 string cannot fail")
}

fn append_line(out: &mut String, line: String, span: Span) -> Result<()> {
    if out.len() + line.len() + 1 > mori_lexer::MAX_SOURCE_BYTES {
        return Err(diagnostics::bound(span));
    }
    out.push_str(&line);
    out.push('\n');
    Ok(())
}

fn emit(t: &Terms, s: &LocalTransferScenario<'_>, span: Span) -> Result<String> {
    let mut out = String::new();
    macro_rules! line {
        ($($arg:tt)*) => {
            append_line(&mut out, format!($($arg)*), span)?
        };
    }
    let statement = format!(
        "transfer from {} to {} fee_to {} value {} fee {}",
        t.owner, t.recipient, t.fee_recipient, t.value, t.fee,
    );
    line!("profile \"moriarty-financial-agreement-source/6\";");
    line!("agreement {} {{", t.agreement);
    line!("domain {};", t.domain);
    line!("settlement {} scale {};", t.asset, t.scale);
    line!(
        "selected TransferLiteralFee source_hash {} digest {};",
        quote(&t.source_hash),
        quote(&t.policy_digest)
    );
    line!("intent {{");
    line!("signer {} key {};", t.signer, quote(&t.key));
    line!("nonce {};", quote(&t.nonce));
    line!("pre_head {};", quote(&t.pre_head));
    line!("valid {}..{};", t.valid_from, t.valid_to);
    line!("gross_cap {};", t.gross_cap);
    line!("fee_cap {};", t.fee_cap);
    line!("net_floor {};", t.net_floor);
    line!("failure success_only;");
    line!("signed_action {statement};");
    for field in [
        "observations",
        "disclosures",
        "retained_effects",
        "retained_duties",
    ] {
        line!("{field} empty;");
    }
    for field in ["delegation", "recovery"] {
        line!("{field} none;");
    }
    line!("}}");
    line!("authenticated {{");
    line!("head {};", quote(s.head));
    line!("predecessor {};", quote(s.predecessor));
    line!("round {};", s.round);
    for balance in &s.balances {
        line!("balance {} {};", balance.account, balance.amount);
    }
    line!(
        "allowance {} remaining {} spent {};",
        s.allowance.owner,
        s.allowance.remaining,
        s.allowance.spent
    );
    let replay = match s.replay {
        Replay::Unused => "unused",
        Replay::Consumed => "consumed",
    };
    line!("replay {replay};");
    line!("work_remaining {};", s.work_remaining);
    line!("work_spent {};", s.work_spent);
    line!("}}");
    line!("submit {statement};");
    line!("effects {{");
    line!("debit {} {};", t.owner, t.gross);
    line!("credit {} {};", t.recipient, t.value);
    if t.fee > 0 {
        line!("credit {} {};", t.fee_recipient, t.fee);
    }
    line!("use_allowance {} {};", t.signer, t.gross);
    line!("use_replay {};", quote(&t.nonce));
    line!(
        "advance_head {} {};",
        quote(&t.pre_head),
        quote(s.post_head)
    );
    line!("}}");
    line!("post_head {};", quote(s.post_head));
    line!("}}");
    Ok(out)
}
