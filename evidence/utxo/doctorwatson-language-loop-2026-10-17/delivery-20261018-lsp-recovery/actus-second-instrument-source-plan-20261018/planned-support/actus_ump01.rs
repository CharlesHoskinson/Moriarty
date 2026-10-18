//! Proposed normal UMP01 observation adapter, not an expected-value producer.
//! No parser/runtime call was executed during source preparation.
use mori_core::{AssetId, Effect, Execution, Scenario, UInt, Value as CoreValue};

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Ump01Bindings {
    pub asset: AssetId,
    pub lender: String,
    pub borrower: String,
    pub principal_duty: String,
}

// Exact selected-fixture JSON integer types. Other decimal/signed rate or
// amount domains are deliberately outside this finite adapter's admission.
#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Ump01Fields {
    pub event_date: String,
    pub event_type: String,
    pub payoff: i64,
    pub currency: String,
    pub notional_principal: u64,
    pub nominal_interest_rate: u64,
    pub accrued_interest: u64,
}

fn uint_state(execution: &Execution, name: &str) -> Result<UInt, String> {
    match execution.post.state.get(name) {
        Some(CoreValue::UInt(value)) => Ok(*value),
        _ => Err(format!("missing or non-UInt source state {name}")),
    }
}

fn integral_result(value: UInt, name: &str) -> Result<u64, String> {
    u64::try_from(value.0).map_err(|_| format!("{name} exceeds selected integer adapter"))
}

/// Project actual public effects/duties/state into the seven present fields.
/// This function reads no fixture results or expected oracle. It preserves no
/// claim that projection alone validates the complete execution: `observe`
/// returns the complete Execution for a separate independent trace comparison.
pub fn project(
    execution: &Execution,
    bindings: &Ump01Bindings,
) -> Result<Ump01Fields, String> {
    if execution.semantics != "moriarty-core/1" || execution.action != "initialExchange" {
        return Err("wrong selected profile/action".into());
    }
    let mut transfer = None;
    let mut event = None;
    let mut principal_effects = 0usize;
    for effect in &execution.effects {
        match effect {
            Effect::Transfer { asset, from, to, amount } => {
                if transfer.is_some() || asset != &bindings.asset
                    || from != &bindings.lender || to != &bindings.borrower
                {
                    return Err("wrong or additional selected initial-exchange cashflow".into());
                }
                transfer = Some((asset.reference.clone(), *amount));
            }
            Effect::Liability { name, asset, debtor, creditor, .. } => {
                if name != &bindings.principal_duty || asset != &bindings.asset
                    || debtor != &bindings.borrower || creditor != &bindings.lender
                {
                    return Err("unexpected duty identity".into());
                }
                principal_effects += 1;
            }
            Effect::Emit { name, value } => {
                if event.is_some() || name != "IED" {
                    return Err("wrong or additional selected event".into());
                }
                let CoreValue::UInt(time) = value else {
                    return Err("event timestamp is not UInt".into());
                };
                event = Some((name.clone(), *time));
            }
            Effect::State { .. } => {}
            _ => return Err("effect outside selected initial-exchange source".into()),
        }
    }
    if principal_effects != 1 {
        return Err("selected issuance requires one actual principal-duty effect".into());
    }
    let (currency, amount) = transfer.ok_or("missing actual cashflow")?;
    let (event_type, time) = event.ok_or("missing actual IED emit")?;
    if uint_state(execution, "statusTime")? != time {
        return Err("source status time differs from emitted time".into());
    }
    let duty = execution.post.liabilities.iter()
        .find(|row| row.name == bindings.principal_duty)
        .ok_or("missing actual outstanding principal duty")?;
    if duty.asset != bindings.asset || duty.debtor != bindings.borrower
        || duty.creditor != bindings.lender
    {
        return Err("closing duty does not retain exact selected identity".into());
    }
    let magnitude = i64::try_from(amount.0)
        .map_err(|_| "payoff exceeds selected signed integer adapter")?;
    let payoff = magnitude.checked_neg().ok_or("signed payoff overflow")?;
    let digits = format!("{:014}", time.0);
    if digits.len() != 14 {
        return Err("timestamp outside supplied YYYYMMDDhhmmss encoding".into());
    }
    let event_date = format!("{}-{}-{}T{}:{}:{}",
        &digits[0..4], &digits[4..6], &digits[6..8],
        &digits[8..10], &digits[10..12], &digits[12..14]);
    Ok(Ump01Fields {
        event_date,
        event_type,
        payoff,
        currency,
        notional_principal: integral_result(duty.amount, "notionalPrincipal")?,
        nominal_interest_rate: integral_result(uint_state(execution, "nominalInterestRate")?, "nominalInterestRate")?,
        accrued_interest: integral_result(uint_state(execution, "accruedInterest")?, "accruedInterest")?,
    })
}

/// Actual existing public consumer path; no fixture lookup or shadow evaluator.
/// Future financial_pair integration must compare the returned complete trace
/// against a separately adopted, independently frozen expected oracle.
pub fn observe(
    source: &str,
    scenario: &Scenario,
    bindings: &Ump01Bindings,
) -> Result<(Execution, Ump01Fields), String> {
    let program = mori_core::parse(source).map_err(|error| error.to_string())?;
    mori_core::validate(&program).map_err(|error| error.to_string())?;
    let execution = mori_core::simulate(&program, "initialExchange", scenario)
        .map_err(|error| error.to_string())?;
    let fields = project(&execution, bindings)?;
    Ok((execution, fields))
}
