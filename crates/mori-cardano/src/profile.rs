use crate::types::*;
use mori_core::{Expr, Program, Statement};
use std::collections::{BTreeMap, BTreeSet};

pub(crate) fn describe_duties(program: &Program) -> Result<Vec<DutyMetadata>, InspectionError> {
    let mut universe = BTreeSet::new();
    let mut identities = BTreeMap::new();
    for action in &program.actions {
        for statement in &action.statements {
            match statement {
                Statement::Emit { .. } => {
                    return Err(InspectionError::UnsupportedDomain("Emit".into()));
                }
                Statement::Liability {
                    name,
                    asset,
                    debtor,
                    creditor,
                    ..
                } => {
                    universe.insert(name.clone());
                    let identity = DutyMetadata {
                        name: name.clone(),
                        asset: program
                            .assets
                            .iter()
                            .find(|a| a.name == *asset)
                            .expect("Core validated the asset reference")
                            .identity
                            .clone(),
                        debtor: debtor.clone(),
                        creditor: creditor.clone(),
                    };
                    if let Some(previous) = identities.insert(name.clone(), identity.clone()) {
                        if previous != identity {
                            return Err(InspectionError::UnsupportedDomain(format!(
                                "Conflicting duty {name}"
                            )));
                        }
                    }
                }
                Statement::Discharge { name, .. } => {
                    universe.insert(name.clone());
                }
                _ => {}
            }
        }
    }
    for expression in &program.expressions {
        if let Expr::LiabilityPresent { name } | Expr::LiabilityAmount { name } = expression {
            universe.insert(name.clone());
        }
    }
    universe
        .into_iter()
        .map(|name| {
            identities.remove(&name).ok_or_else(|| {
                InspectionError::UnsupportedDomain(format!(
                    "Duty has no source introduction: {name}"
                ))
            })
        })
        .collect()
}

pub(crate) fn describe_accounts(
    program: &Program,
    bindings: &[AccountBinding],
) -> Result<Vec<AccountBinding>, InspectionError> {
    describe_accounts_with_rule(program, bindings, true)
}

pub(crate) fn describe_accounts_v2(
    program: &Program,
    bindings: &[AccountBinding],
) -> Result<Vec<AccountBinding>, InspectionError> {
    describe_accounts_with_rule(program, bindings, false)
}

fn describe_accounts_with_rule(
    program: &Program,
    bindings: &[AccountBinding],
    bind_duty_parties: bool,
) -> Result<Vec<AccountBinding>, InspectionError> {
    let mut accounts = BTreeMap::new();
    for binding in bindings {
        if binding.name.is_empty()
            || binding.name.len() > 1024
            || accounts
                .insert(binding.name.clone(), binding.clone())
                .is_some()
        {
            return Err(InspectionError::BindingMismatch(
                "Empty, oversized or duplicate account name".into(),
            ));
        }
    }
    for statement in program.actions.iter().flat_map(|a| &a.statements) {
        let pair = match statement {
            Statement::Transfer { from, to, .. } | Statement::Fee { from, to, .. } => {
                Some((from, to))
            }
            Statement::Liability {
                debtor, creditor, ..
            } if bind_duty_parties => Some((debtor, creditor)),
            _ => None,
        };
        if let Some((a, b)) = pair {
            for name in [a, b] {
                if !accounts.contains_key(name) {
                    return Err(InspectionError::BindingMismatch(format!(
                        "Missing account {name}"
                    )));
                }
            }
        }
    }
    Ok(accounts.into_values().collect())
}
