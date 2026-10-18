//! Witness proposal, separated from acceptance.
//!
//! [`propose`] may call the current Core reference (`simulate`, or `simulate_metered`
//! when a cap is bound) to obtain a candidate transition. [`from_records`] and
//! [`from_execution`] turn Core records (from the reference, or from a frozen
//! independent oracle) into [`TransitionValues`]. Neither decides acceptance:
//!
//! - a reference refusal yields `MappingError::ReferenceRefused`, i.e. no proposal,
//!   never a "false" witness that the circuit would read;
//! - a reference success is only a claim; the circuit recomputes every internal wire
//!   from the claimed inputs and pre-state and refuses (unsatisfied constraints) any
//!   claim whose post-state or effects differ.
//!
//! The mapping checks only shape: counts, kinds, constant fields (asset, accounts,
//! names, duty identities), value types and frame stability of unrepresented rows.
use crate::circuit::CompiledRelation;
use crate::error::MappingError;
use crate::plan::{ActionPlan, CapBinding, EffectShape};
use crate::statement::{BodyValues, DutyValue, EffectValues, TransitionValues};
use mori_core::{Effect, Execution, Scenario, Snapshot, UInt, Value};
use std::collections::{BTreeMap, BTreeSet};

fn body(plan: &ActionPlan, snapshot: &Snapshot, phase: &str) -> Result<BodyValues, MappingError> {
    let mut states = Vec::with_capacity(plan.states.len());
    for s in &plan.states {
        let v = snapshot.state.get(&s.name).ok_or_else(|| MappingError::Shape(format!("{phase} state {} missing", s.name)))?;
        if v.ty() != s.ty {
            return Err(MappingError::Shape(format!("{phase} state {} has the wrong type", s.name)));
        }
        states.push(v.clone());
    }
    if snapshot.state.len() != plan.states.len() {
        return Err(MappingError::Shape(format!("{phase} snapshot carries undeclared states")));
    }
    let mut cells = Vec::with_capacity(plan.cells.len());
    for c in &plan.cells {
        let row = snapshot
            .balances
            .iter()
            .find(|b| b.asset == c.asset && b.account == c.account)
            .ok_or_else(|| MappingError::Shape(format!("{phase} balance {}/{} missing", c.asset_name, c.account)))?;
        cells.push(row.amount);
    }
    let mut duties = Vec::with_capacity(plan.duties.len());
    for d in &plan.duties {
        match snapshot.liabilities.iter().find(|l| l.name == d.name) {
            Some(l) => {
                if l.asset != d.asset || l.debtor != d.debtor || l.creditor != d.creditor {
                    return Err(MappingError::Shape(format!("{phase} duty {} identity differs from the source registry", d.name)));
                }
                duties.push(DutyValue { present: true, amount: l.amount });
            }
            None => duties.push(DutyValue { present: false, amount: UInt(0) }),
        }
    }
    Ok(BodyValues { states, cells, duties })
}

/// Rows outside the represented cells and registry must be identical before and after
/// (Core never changes them). Representing them is a target obligation.
fn frame_unchanged(plan: &ActionPlan, pre: &Snapshot, post: &Snapshot) -> Result<(), MappingError> {
    let represented = |asset: &mori_core::AssetId, account: &str| plan.cells.iter().any(|c| &c.asset == asset && c.account == account);
    let outside = |s: &Snapshot| -> BTreeSet<(mori_core::AssetId, String, UInt)> {
        s.balances.iter().filter(|b| !represented(&b.asset, &b.account)).map(|b| (b.asset.clone(), b.account.clone(), b.amount)).collect()
    };
    if outside(pre) != outside(post) {
        return Err(MappingError::FrameChanged("an unrepresented balance row differs between pre and post".into()));
    }
    let registry: BTreeSet<&str> = plan.duties.iter().map(|d| d.name.as_str()).collect();
    let rows = |s: &Snapshot| -> Vec<mori_core::Liability> {
        s.liabilities.iter().filter(|l| !registry.contains(l.name.as_str())).cloned().collect()
    };
    if rows(pre) != rows(post) {
        return Err(MappingError::FrameChanged("an unrepresented liability row differs between pre and post".into()));
    }
    Ok(())
}

fn effect(plan: &ActionPlan, index: usize, shape: &EffectShape, e: &Effect) -> Result<EffectValues, MappingError> {
    let mismatch = || MappingError::Shape(format!("effect {index} differs from its source shape"));
    Ok(match (shape, e) {
        (EffectShape::State { name, ty, .. }, Effect::State { name: n, before, after }) => {
            if n != name || before.ty() != *ty || after.ty() != *ty {
                return Err(mismatch());
            }
            EffectValues::State { before: before.clone(), after: after.clone() }
        }
        (EffectShape::Transfer { asset, from, to, .. }, Effect::Transfer { asset: a, from: f, to: t, amount })
        | (EffectShape::Fee { asset, from, to, .. }, Effect::Fee { asset: a, from: f, to: t, amount }) => {
            if a != asset || f != from || t != to {
                return Err(mismatch());
            }
            EffectValues::Amount(*amount)
        }
        (EffectShape::Liability { duty, name, .. }, Effect::Liability { name: n, asset, debtor, creditor, before, after }) => {
            let d = &plan.duties[*duty];
            if n != name || *asset != d.asset || *debtor != d.debtor || *creditor != d.creditor {
                return Err(mismatch());
            }
            EffectValues::Duty { before: *before, after: *after }
        }
        (EffectShape::Emit { name, ty, .. }, Effect::Emit { name: n, value }) => {
            if n != name || value.ty() != *ty {
                return Err(mismatch());
            }
            EffectValues::Emit(value.clone())
        }
        _ => return Err(mismatch()),
    })
}

/// Maps Core records into a claimed transition for `plan`.
pub fn from_records(
    plan: &ActionPlan,
    inputs: &BTreeMap<String, Value>,
    pre: &Snapshot,
    post: &Snapshot,
    effects: &[Effect],
    cap: Option<UInt>,
) -> Result<TransitionValues, MappingError> {
    if inputs.len() != plan.inputs.len() {
        return Err(MappingError::Shape("scenario inputs differ from the action's inputs".into()));
    }
    let mut claimed_inputs = Vec::with_capacity(plan.inputs.len());
    for i in &plan.inputs {
        let v = inputs.get(&i.name).ok_or_else(|| MappingError::Shape(format!("input {} missing", i.name)))?;
        if v.ty() != i.ty {
            return Err(MappingError::Shape(format!("input {} has the wrong type", i.name)));
        }
        claimed_inputs.push(v.clone());
    }
    frame_unchanged(plan, pre, post)?;
    if effects.len() != plan.effects.len() {
        return Err(MappingError::Shape(format!("record has {} effects, the action has {}", effects.len(), plan.effects.len())));
    }
    let mut claimed_effects = Vec::with_capacity(effects.len());
    for (index, (shape, e)) in plan.effects.iter().zip(effects).enumerate() {
        claimed_effects.push(effect(plan, index, shape, e)?);
    }
    match (plan.cap, cap) {
        (CapBinding::PublicCap, None) => return Err(MappingError::Cap("relation binds a public cap; supply one".into())),
        (CapBinding::NotBound, Some(_)) => return Err(MappingError::Cap("relation binds no cap".into())),
        _ => {}
    }
    Ok(TransitionValues {
        inputs: claimed_inputs,
        pre: body(plan, pre, "pre")?,
        post: body(plan, post, "post")?,
        effects: claimed_effects,
        cap,
    })
}

/// Maps an `Execution` (reference output or frozen oracle record) for `relation`.
pub fn from_execution(relation: &CompiledRelation, scenario: &Scenario, execution: &Execution, cap: Option<UInt>) -> Result<TransitionValues, MappingError> {
    if execution.action != relation.plan().action || execution.semantics != mori_core::SEMANTICS {
        return Err(MappingError::Shape("execution names another action or semantics".into()));
    }
    from_records(relation.plan(), &scenario.inputs, &execution.pre, &execution.post, &execution.effects, cap)
}

/// Proposes a claimed transition by running the current Core reference. The result
/// is a proposal only; see the module documentation.
pub fn propose(relation: &CompiledRelation, scenario: &Scenario, cap: Option<UInt>) -> Result<TransitionValues, MappingError> {
    let program = relation.program();
    let action = relation.plan().action.as_str();
    let execution = match (relation.plan().cap, cap) {
        (CapBinding::NotBound, None) => mori_core::simulate(program, action, scenario).map_err(MappingError::ReferenceRefused)?,
        (CapBinding::PublicCap, Some(c)) => {
            mori_core::simulate_metered(program, action, scenario, c).map_err(|refused| MappingError::ReferenceRefused(refused.error))?.execution
        }
        (CapBinding::PublicCap, None) => return Err(MappingError::Cap("relation binds a public cap; supply one".into())),
        (CapBinding::NotBound, Some(_)) => return Err(MappingError::Cap("relation binds no cap".into())),
    };
    from_execution(relation, scenario, &execution, cap)
}
