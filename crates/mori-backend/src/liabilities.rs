//! Source-derived, name-unique duty identities for the fixed compiler profile.
use crate::{Error, error};
use mori_core::{AssetId, Program, Statement};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Engineering admission ceiling, not a measured proving-resource bound.
pub const MAX_LIABILITY_REGISTRY_ENTRIES: usize = 16;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct LiabilityIdentity {
    pub name: String,
    pub asset: AssetId,
    pub debtor: String,
    pub creditor: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LiabilityDescriptor {
    pub name: String,
    pub asset: AssetId,
    pub debtor: String,
    pub creditor: String,
    pub pre_presence_position: u32,
    pub pre_amount_position: u32,
    pub post_presence_position: u32,
    pub post_amount_position: u32,
}

/// The source statement determines the operation; core effects are amount-only.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LiabilityOperation {
    CreateOrAccrue,
    Discharge,
}

/// Build at most sixteen entries before any circuit or frame allocation.
/// Identity is immutable by name, including exact asset scale and parties.
pub(crate) fn registry(program: &Program) -> Result<Vec<LiabilityIdentity>, Error> {
    let mut entries = BTreeMap::<String, LiabilityIdentity>::new();
    for statement in program.actions.iter().flat_map(|a| &a.statements) {
        if let Statement::Liability {
            name,
            asset,
            debtor,
            creditor,
            ..
        } = statement
        {
            let asset = program
                .assets
                .iter()
                .find(|a| &a.name == asset)
                .ok_or_else(|| error("program", "Liability asset is not declared."))?;
            if let Some(existing) = entries.get(name) {
                if existing.asset != asset.identity
                    || &existing.debtor != debtor
                    || &existing.creditor != creditor
                {
                    return Err(error(
                        "unsupported",
                        format!("Conflicting source liability identity for {name:?}."),
                    ));
                }
                continue;
            }
            if entries.len() == MAX_LIABILITY_REGISTRY_ENTRIES {
                return Err(error(
                    "limit",
                    format!(
                        "Liability registry exceeds the fixed {MAX_LIABILITY_REGISTRY_ENTRIES}-entry compiler profile."
                    ),
                ));
            }
            entries.insert(
                name.clone(),
                LiabilityIdentity {
                    name: name.clone(),
                    asset: asset.identity.clone(),
                    debtor: debtor.clone(),
                    creditor: creditor.clone(),
                },
            );
        }
    }
    for statement in program.actions.iter().flat_map(|a| &a.statements) {
        if let Statement::Discharge { name, .. } = statement {
            if !entries.contains_key(name) {
                return Err(error(
                    "unsupported",
                    format!("Discharge {name:?} has no source-derived liability identity."),
                ));
            }
        }
    }
    Ok(entries.into_values().collect())
}
