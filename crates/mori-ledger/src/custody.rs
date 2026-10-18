//! Actual unsigned public offers and exact complete prospective custody.
use crate::{Error, Result, layout::AdmittedPredecessor};
use midnight_coin_structure::coin::{UnshieldedTokenType, UserAddress};
use midnight_ledger::structure::{UnshieldedOffer, Utxo, UtxoMeta, UtxoOutput};
use midnight_storage::db::InMemoryDB;
use mori_core::{Balance, Effect, Execution, UInt};
use std::collections::BTreeMap;
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct EffectOutput {
    pub effect: usize,
    pub output: Option<u32>,
    pub claim_amount: Option<UInt>,
}
pub struct PreparedOffer {
    pub offer: UnshieldedOffer<(), InMemoryDB>,
    pub groups: BTreeMap<(UnshieldedTokenType, UserAddress), u128>,
    pub effect_outputs: Vec<EffectOutput>,
    pub change_indices: Vec<u32>,
    pub retained: Vec<(Utxo, UtxoMeta)>,
    pub balances: Vec<Balance>,
}
fn refusal(code: &'static str) -> Error {
    Error::new("native-offer", code)
}
pub fn prepare_offer(pre: &AdmittedPredecessor, e: &Execution) -> Result<PreparedOffer> {
    let mut groups = BTreeMap::new();
    let mut budget = BTreeMap::new();
    for u in &pre.selected {
        let n = budget.entry(u.type_).or_insert(0u128);
        *n = n
            .checked_add(u.value)
            .ok_or(refusal("input-sum-overflow"))?;
    }
    for effect in &e.effects {
        if let Effect::Transfer {
            asset, to, amount, ..
        }
        | Effect::Fee {
            asset, to, amount, ..
        } = effect
            && amount.0 > 0
        {
            let k = (pre.assets[asset], pre.roles[to].address);
            let n = groups.entry(k).or_insert(0u128);
            *n = n.checked_add(amount.0).ok_or(refusal("claim-overflow"))?;
        }
    }
    let mut required = BTreeMap::new();
    for ((token, _), amount) in &groups {
        let n = required.entry(*token).or_insert(0u128);
        *n = n
            .checked_add(*amount)
            .ok_or(refusal("payout-sum-overflow"))?;
    }
    let payer = pre
        .selected
        .first()
        .map(|s| UserAddress::from(s.owner.clone()));
    let mut outputs: Vec<(UtxoOutput, bool)> = groups
        .iter()
        .map(|((type_, owner), value)| {
            (
                UtxoOutput {
                    type_: *type_,
                    owner: *owner,
                    value: *value,
                },
                false,
            )
        })
        .collect();
    for (token, amount) in &required {
        if budget.get(token).copied().unwrap_or(0) < *amount {
            return Err(refusal("insufficient-selected-inputs"));
        }
    }
    for (token, amount) in &budget {
        let change = amount
            .checked_sub(required.get(token).copied().unwrap_or(0))
            .ok_or(refusal("insufficient-selected-inputs"))?;
        if change > 0 {
            outputs.push((
                UtxoOutput {
                    type_: *token,
                    owner: payer.ok_or(refusal("payer-absent"))?,
                    value: change,
                },
                true,
            ));
        }
    }
    outputs.sort_by(|a, b| a.0.cmp(&b.0));
    let mut effect_outputs = Vec::new();
    for (index, effect) in e.effects.iter().enumerate() {
        if let Effect::Transfer {
            asset, to, amount, ..
        }
        | Effect::Fee {
            asset, to, amount, ..
        } = effect
        {
            let mapping = if amount.0 == 0 {
                None
            } else {
                let k = (pre.assets[asset], pre.roles[to].address);
                Some((
                    outputs
                        .iter()
                        .position(|(o, change)| {
                            !change && o.type_ == k.0 && o.owner == k.1 && o.value == groups[&k]
                        })
                        .ok_or(refusal("group-output"))? as u32,
                    UInt(groups[&k]),
                ))
            };
            effect_outputs.push(EffectOutput {
                effect: index,
                output: mapping.map(|m| m.0),
                claim_amount: mapping.map(|m| m.1),
            });
        }
    }
    let change_indices = outputs
        .iter()
        .enumerate()
        .filter_map(|(i, (_, c))| c.then_some(i as u32))
        .collect();
    let offer = UnshieldedOffer {
        inputs: pre.selected.clone().into(),
        outputs: outputs.into_iter().map(|(o, _)| o).collect(),
        signatures: Default::default(),
    };
    let retained: Vec<_> = pre
        .all_utxos
        .iter()
        .filter(|(u, _)| !pre.selected.iter().any(|s| u == &Utxo::from(s.clone())))
        .cloned()
        .collect();
    let mut balances = Vec::new();
    for b in &e.pre.balances {
        let token = pre.assets[&b.asset];
        let address = pre.roles[&b.account].address;
        let amount = retained
            .iter()
            .filter(|(u, _)| u.type_ == token && u.owner == address)
            .map(|(u, _)| u.value)
            .chain(
                offer
                    .outputs
                    .iter()
                    .filter(|u| u.type_ == token && u.owner == address)
                    .map(|u| u.value),
            )
            .try_fold(0u128, |n, v| {
                n.checked_add(v).ok_or(refusal("projection-overflow"))
            })?;
        balances.push(Balance {
            account: b.account.clone(),
            asset: b.asset.clone(),
            amount: UInt(amount),
        });
    }
    if balances != e.post.balances {
        return Err(Error::new("full-readback", "prospective-balances"));
    }
    Ok(PreparedOffer {
        offer,
        groups,
        effect_outputs,
        change_indices,
        retained,
        balances,
    })
}

/// Independent exact output-count comparison. Native subset and aggregate
/// balance equality both permit split/extra change; this profile does not.
pub fn validate_offer(
    pre: &AdmittedPredecessor,
    e: &Execution,
    candidate: &UnshieldedOffer<(), InMemoryDB>,
) -> Result<()> {
    let inputs = Vec::from(&candidate.inputs);
    let outputs = Vec::from(&candidate.outputs);
    if inputs != pre.selected || !candidate.signatures.is_empty() {
        return Err(refusal("exact-unsigned-inputs"));
    }
    if outputs.windows(2).any(|p| p[0] > p[1]) || outputs.iter().any(|o| o.value == 0) {
        return Err(refusal("canonical-output-shape"));
    }
    let mut payments = BTreeMap::new();
    let mut funding = BTreeMap::new();
    for input in &pre.selected {
        let n = funding.entry(input.type_).or_insert(0u128);
        *n = n
            .checked_add(input.value)
            .ok_or(refusal("input-sum-overflow"))?;
    }
    for effect in &e.effects {
        if let Effect::Transfer {
            asset, to, amount, ..
        }
        | Effect::Fee {
            asset, to, amount, ..
        } = effect
            && amount.0 != 0
        {
            let n = payments
                .entry((pre.assets[asset], pre.roles[to].address))
                .or_insert(0u128);
            *n = n
                .checked_add(amount.0)
                .ok_or(refusal("payout-sum-overflow"))?;
        }
    }
    let mut counts: BTreeMap<UtxoOutput, usize> = BTreeMap::new();
    let mut used = BTreeMap::new();
    for ((type_, owner), value) in payments {
        *counts
            .entry(UtxoOutput {
                type_,
                owner,
                value,
            })
            .or_default() += 1;
        let n = used.entry(type_).or_insert(0u128);
        *n = n.checked_add(value).ok_or(refusal("payout-sum-overflow"))?;
    }
    for (token, budget) in funding {
        let change = budget
            .checked_sub(used.get(&token).copied().unwrap_or(0))
            .ok_or(refusal("insufficient-selected-inputs"))?;
        if change > 0 {
            let payer = UserAddress::from(
                pre.selected
                    .first()
                    .ok_or(refusal("payer-absent"))?
                    .owner
                    .clone(),
            );
            *counts
                .entry(UtxoOutput {
                    type_: token,
                    owner: payer,
                    value: change,
                })
                .or_default() += 1;
        }
    }
    for output in outputs {
        let count = counts
            .get_mut(&output)
            .ok_or(Error::new("full-readback", "unexpected-output"))?;
        if *count == 0 {
            return Err(Error::new("full-readback", "output-multiplicity"));
        }
        *count -= 1;
    }
    if counts.values().any(|n| *n != 0) {
        return Err(Error::new("full-readback", "missing-output"));
    }
    Ok(())
}
