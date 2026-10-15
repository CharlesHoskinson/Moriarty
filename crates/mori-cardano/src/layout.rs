use crate::{profile, types::*};
use mori_core::{Program, Statement};

pub(crate) fn describe(
    program: &Program,
    bindings: &[AccountBinding],
) -> Result<LayoutMetadataV1, InspectionError> {
    // Unit symbols preserve full source identity; no physical asset is derived.
    if program.assets.iter().any(|a| a.identity.scale != 0) {
        return Err(InspectionError::UnsupportedDomain(
            "Nonzero asset scale".into(),
        ));
    }
    let accounts = profile::describe_accounts(program, bindings)?;
    let duties = profile::describe_duties(program)?;
    let balance_cells = accounts
        .len()
        .checked_mul(program.assets.len())
        .ok_or_else(|| InspectionError::UnsupportedDomain("Balance matrix size".into()))?;
    if balance_cells > 4096 || duties.len() > 1024 {
        return Err(InspectionError::UnsupportedDomain(
            "Scenario metadata bounds".into(),
        ));
    }
    let mut action_bounds = Vec::new();
    for action in &program.actions {
        let added = action
            .statements
            .iter()
            .filter(|s| matches!(s, Statement::Liability { .. }))
            .count();
        // Conservative full-matrix/all-present-registry bound, never a claim
        // about current or measured ledger resources.
        let records = program.states.len() + balance_cells + duties.len() + added + 1;
        let cells = records
            .checked_mul(action.statements.len() + 2)
            .ok_or_else(|| InspectionError::UnsupportedDomain("Trace matrix size".into()))?;
        if cells > mori_core::MAX_TRACE_CELLS {
            return Err(InspectionError::UnsupportedDomain(
                "Trace metadata bounds".into(),
            ));
        }
        action_bounds.push(ActionBoundsMetadata {
            action: action.name.clone(),
            statements: action.statements.len(),
            liability_statements: added,
            trace_cells_upper_bound: cells,
        });
    }
    let mut states = program.states.clone();
    states.sort_by(|a, b| a.name.cmp(&b.name));
    let mut framed_cells: Vec<_> = accounts
        .iter()
        .filter(|a| a.class == AccountClass::ScriptCustodied)
        .flat_map(|a| {
            program.assets.iter().map(move |asset| FramedCellMetadata {
                account: a.name.clone(),
                asset: asset.identity.clone(),
            })
        })
        .collect();
    framed_cells.sort_by(|a, b| a.asset.cmp(&b.asset).then(a.account.cmp(&b.account)));
    Ok(LayoutMetadataV1 {
        format: "moriarty-cardano-layout-metadata/1".into(),
        states,
        assets: program.assets.clone(),
        accounts,
        duties,
        actions: program.actions.clone(),
        framed_cells,
        projected_balance_cells: balance_cells,
        action_bounds,
    })
}
