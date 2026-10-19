//! `/2` static validation and stage plan invariants.
//!
//! `validate_v2` applies every legacy `/1` program check with the legacy codes
//! and messages, plus the Await rules: top-level placement (structural in IR),
//! unique labels, non-shadowing binders, Bool guards typed over the prefix and
//! the Await's own binders, the per-action bounds, and `await`/`when` reserved.
//! Legacy `validate` is unchanged and remains `/1`-only.
use super::{
    AwaitSite, CaptureSlot, LANGUAGE_2, LANGUAGE_3, MAX_AWAITS_PER_ACTION, MAX_CAPTURE_SLOTS,
    MAX_EVENT_BINDERS, Origin, SEMANTICS_2, SEMANTICS_3, STAGE_PLAN_FORMAT, StageBinder, StagePlan,
};
use crate::runtime::{
    expr_type, identifier, require_uint, validate_asset, validate_liability_reads,
};
use crate::{
    AssetId, Error, Expr, MAX_DECLARATIONS, MAX_EXPRESSIONS, MAX_STATEMENTS, Program, Statement,
    Type, UInt,
};
use std::collections::{BTreeMap, BTreeSet};

#[cfg(test)]
#[path = "validate_v3_tests.rs"]
mod validate_v3_tests;

fn failure(code: &str, message: impl Into<String>) -> Error {
    Error::new(code, message)
}

fn invariant(what: &str) -> Error {
    failure("program", format!("Stage plan invariant violated: {what}."))
}

/// Legacy identifier rule with `await` and `when` also reserved (C6).
fn identifier_v2(name: &str) -> bool {
    identifier(name) && !matches!(name, "await" | "when")
}

fn names_v2<'a>(values: impl Iterator<Item = &'a str>, kind: &str) -> Result<(), Error> {
    let mut seen = BTreeSet::new();
    for name in values {
        if !identifier_v2(name) || !seen.insert(name) {
            return Err(failure(
                "duplicate",
                format!("Invalid or duplicate {kind} name {name:?}."),
            ));
        }
    }
    Ok(())
}

/// Full finite validation of a `moriarty-core/2` program.
pub(crate) fn validate_v2(program: &Program) -> Result<(), Error> {
    if program.language != LANGUAGE_2 || program.semantics != SEMANTICS_2 {
        return Err(failure(
            "profile",
            "Unsupported language or semantics identity.",
        ));
    }
    // F2 defence in depth, mirroring `require_core2`: duties are `/3`-only
    // and a manually constructed `/2` value carrying them is refused even
    // if a caller reaches validation without the profile helper first.
    if !program.duties.is_empty() {
        return Err(failure(
            "profile",
            "Unsupported language or semantics identity.",
        ));
    }
    if !identifier_v2(&program.name) {
        return Err(failure(
            "name",
            "Agreement name must be a non-reserved ASCII identifier of at most 64 bytes.",
        ));
    }
    if program.actions.is_empty() {
        return Err(failure("program", "Program needs at least one action."));
    }
    if program.states.len() + program.assets.len() + program.actions.len() > MAX_DECLARATIONS
        || program.expressions.len() > MAX_EXPRESSIONS
    {
        return Err(failure(
            "bound",
            "Program exceeds finite declaration/expression bounds.",
        ));
    }
    names_v2(
        program
            .states
            .iter()
            .map(|s| s.name.as_str())
            .chain(program.assets.iter().map(|a| a.name.as_str())),
        "declaration",
    )?;
    names_v2(program.actions.iter().map(|a| a.name.as_str()), "action")?;
    let mut identities = BTreeSet::new();
    for asset in &program.assets {
        let id = &asset.identity;
        if id.network.is_empty()
            || id.issuer.is_empty()
            || id.reference.is_empty()
            || id.network.len() > 1024
            || id.issuer.len() > 1024
            || id.reference.len() > 1024
            || id.scale > 38
            || !identities.insert(id)
        {
            return Err(failure(
                "asset",
                "Asset identities must be complete, unique and use scale 0..38.",
            ));
        }
        if program.assets.iter().any(|other| {
            other.identity.network == id.network
                && other.identity.issuer == id.issuer
                && other.identity.reference == id.reference
                && other.identity.scale != id.scale
        }) {
            return Err(failure(
                "asset",
                "Same asset identity cannot have conflicting scales.",
            ));
        }
    }
    let mut depths = Vec::with_capacity(program.expressions.len());
    for (i, e) in program.expressions.iter().enumerate() {
        let refs = match e {
            Expr::Binary { lhs, rhs, .. } => vec![*lhs, *rhs],
            Expr::Not { value } => vec![*value],
            _ => vec![],
        };
        let mut depth = 0;
        for r in refs {
            if usize::from(r) >= i {
                return Err(failure(
                    "expression",
                    "Expression references must point to earlier nodes; cycles are forbidden.",
                ));
            }
            depth = depth.max(depths[usize::from(r)] + 1);
        }
        if depth > 64 {
            return Err(failure("bound", "Expression depth exceeds 64."));
        }
        depths.push(depth);
    }
    let state_types: BTreeMap<_, _> = program
        .states
        .iter()
        .map(|s| (s.name.clone(), s.ty))
        .collect();
    for state in &program.states {
        if state.initial.ty() != state.ty {
            return Err(failure(
                "type",
                format!("Initial state {:?} has wrong type.", state.name),
            ));
        }
    }
    for action in &program.actions {
        if action.inputs.len() > MAX_DECLARATIONS || action.statements.len() > MAX_STATEMENTS {
            return Err(failure(
                "bound",
                "Action exceeds input or statement bounds.",
            ));
        }
        names_v2(action.inputs.iter().map(|i| i.name.as_str()), "input")?;
        let mut env = state_types.clone();
        for input in &action.inputs {
            if env.insert(input.name.clone(), input.ty).is_some()
                || program.assets.iter().any(|a| a.name == input.name)
            {
                return Err(failure("duplicate", "Input cannot shadow a declaration."));
            }
        }
        let mut labels = BTreeSet::new();
        let mut awaits = 0_usize;
        // Inputs, lets and earlier binders: the captures of the next site.
        let mut captured = action.inputs.len();
        for statement in &action.statements {
            match statement {
                Statement::Await {
                    label,
                    binders,
                    guard,
                } => {
                    awaits += 1;
                    if awaits > MAX_AWAITS_PER_ACTION {
                        return Err(failure("bound", "Action exceeds 16 awaits."));
                    }
                    if binders.len() > MAX_EVENT_BINDERS {
                        return Err(failure("bound", "Await exceeds 16 event binders."));
                    }
                    if captured > MAX_CAPTURE_SLOTS {
                        return Err(failure("bound", "Await site exceeds 256 capture slots."));
                    }
                    if !identifier_v2(label) || !labels.insert(label.as_str()) {
                        return Err(failure(
                            "duplicate",
                            format!("Invalid or duplicate await label {label:?}."),
                        ));
                    }
                    for binder in binders {
                        if !identifier_v2(&binder.name)
                            || env.insert(binder.name.clone(), binder.ty).is_some()
                            || program.assets.iter().any(|a| a.name == binder.name)
                        {
                            return Err(failure(
                                "duplicate",
                                "Await binder cannot shadow an existing name.",
                            ));
                        }
                    }
                    if expr_type(program, *guard, &env, 0)? != Type::Bool {
                        return Err(failure("type", "Await guard must have type Bool."));
                    }
                    captured += binders.len();
                }
                Statement::Let { name, value } => {
                    let ty = expr_type(program, *value, &env, 0)?;
                    if !identifier_v2(name)
                        || env.insert(name.clone(), ty).is_some()
                        || program.assets.iter().any(|a| a.name == *name)
                    {
                        return Err(failure("duplicate", "let cannot shadow an existing name."));
                    }
                    captured += 1;
                }
                Statement::Requires { predicate } | Statement::Ensures { predicate } => {
                    if expr_type(program, *predicate, &env, 0)? != Type::Bool {
                        return Err(failure("type", "Predicate must have type Bool."));
                    }
                }
                Statement::Next { state, value } => {
                    let expected = state_types
                        .get(state)
                        .ok_or_else(|| failure("name", format!("Unknown state {state:?}.")))?;
                    if expr_type(program, *value, &env, 0)? != *expected {
                        return Err(failure("type", "State update has wrong type."));
                    }
                }
                Statement::Emit { name, value } => {
                    if name.is_empty() || name.len() > 1024 {
                        return Err(failure("name", "Event name must contain 1..1024 bytes."));
                    }
                    expr_type(program, *value, &env, 0)?;
                }
                Statement::Transfer {
                    asset,
                    from,
                    to,
                    amount,
                }
                | Statement::Fee {
                    asset,
                    from,
                    to,
                    amount,
                } => {
                    validate_asset(program, asset)?;
                    if from.is_empty() || to.is_empty() || from.len() > 1024 || to.len() > 1024 {
                        return Err(failure("account", "Account identities cannot be empty."));
                    }
                    require_uint(program, *amount, &env)?;
                }
                Statement::Liability {
                    name,
                    asset,
                    debtor,
                    creditor,
                    amount,
                } => {
                    validate_asset(program, asset)?;
                    if name.is_empty()
                        || debtor.is_empty()
                        || creditor.is_empty()
                        || name.len() > 1024
                        || debtor.len() > 1024
                        || creditor.len() > 1024
                    {
                        return Err(failure(
                            "liability",
                            "Liability identities cannot be empty.",
                        ));
                    }
                    require_uint(program, *amount, &env)?;
                }
                Statement::Discharge { name, amount } => {
                    if name.is_empty() || name.len() > 1024 {
                        return Err(failure(
                            "liability",
                            "Liability name must contain 1..1024 bytes.",
                        ));
                    }
                    require_uint(program, *amount, &env)?;
                }
            }
        }
    }
    validate_liability_reads(program)?;
    Ok(())
}

fn duty_failure() -> Error {
    failure(
        "sequence_duty_declaration",
        "External duty declaration is not admissible in this program.",
    )
}

/// First duty pass for `/3`, in declaration order. A duty name is a decoded
/// string (1..=1024 bytes), not an identifier. Every refusal in this pass
/// uses `sequence_duty_declaration`, including an identical duplicate.
fn validate_duties_v3(program: &Program) -> Result<(), Error> {
    let mut seen = BTreeSet::new();
    for decl in &program.duties {
        if decl.name.is_empty()
            || decl.name.len() > 1024
            || decl.debtor.is_empty()
            || decl.debtor.len() > 1024
            || decl.creditor.is_empty()
            || decl.creditor.len() > 1024
        {
            return Err(duty_failure());
        }
        if !seen.insert(decl.name.as_str()) {
            return Err(duty_failure());
        }
        let Some(asset_decl) = program.assets.iter().find(|a| a.name == decl.asset) else {
            return Err(duty_failure());
        };
        let duty_identity = (
            &asset_decl.identity,
            decl.debtor.as_str(),
            decl.creditor.as_str(),
        );
        for statement in program.actions.iter().flat_map(|a| &a.statements) {
            if let Statement::Liability {
                name,
                asset,
                debtor,
                creditor,
                ..
            } = statement
                && name == &decl.name
                && let Ok(statement_asset) = validate_asset(program, asset)
                && (statement_asset, debtor.as_str(), creditor.as_str()) != duty_identity
            {
                return Err(duty_failure());
            }
        }
    }
    Ok(())
}

/// `/3` liability-read validation over REG3: `Statement::Liability`
/// identities over all actions union `Program.duties`, each duty asset name
/// resolved to its declared exact `AssetId`. Statement-only conflicts and
/// undeclared reads keep the legacy `liability` errors; duty-caused
/// conflicts use `sequence_duty_declaration`.
fn validate_liability_reads_v3(program: &Program) -> Result<(), Error> {
    let mut reads = BTreeSet::new();
    for expr in &program.expressions {
        if let Expr::LiabilityPresent { name } | Expr::LiabilityAmount { name } = expr {
            if name.is_empty() || name.len() > 1024 {
                return Err(failure(
                    "liability",
                    "Liability read name must contain 1..1024 bytes.",
                ));
            }
            reads.insert(name.as_str());
        }
    }
    let mut registry: BTreeMap<&str, (&AssetId, &str, &str)> = BTreeMap::new();
    for statement in program.actions.iter().flat_map(|a| &a.statements) {
        if let Statement::Liability {
            name,
            asset,
            debtor,
            creditor,
            ..
        } = statement
        {
            let identity = (
                validate_asset(program, asset)?,
                debtor.as_str(),
                creditor.as_str(),
            );
            if registry
                .insert(name.as_str(), identity)
                .is_some_and(|existing| existing != identity)
            {
                return Err(failure(
                    "liability",
                    format!("Conflicting source liability identity for {name:?}."),
                ));
            }
        }
    }
    for decl in &program.duties {
        let Some(asset_decl) = program.assets.iter().find(|a| a.name == decl.asset) else {
            return Err(duty_failure());
        };
        let identity = (
            &asset_decl.identity,
            decl.debtor.as_str(),
            decl.creditor.as_str(),
        );
        if registry
            .insert(decl.name.as_str(), identity)
            .is_some_and(|existing| existing != identity)
        {
            return Err(duty_failure());
        }
    }
    for name in reads {
        if !registry.contains_key(name) {
            return Err(failure(
                "liability",
                format!("Liability read {name:?} has no source-derived identity."),
            ));
        }
    }
    Ok(())
}

/// Full finite validation of a `moriarty-core/3` program. Applies every
/// `validate_v2` check with the same codes and messages, accepting only the
/// `/3` profile, counting duties toward the declaration bound, running the
/// duty first pass, and closing with the REG3 liability-read check.
pub(crate) fn validate_v3(program: &Program) -> Result<(), Error> {
    if program.language != LANGUAGE_3 || program.semantics != SEMANTICS_3 {
        return Err(failure(
            "profile",
            "Unsupported language or semantics identity.",
        ));
    }
    validate_duties_v3(program)?;
    if !identifier_v2(&program.name) {
        return Err(failure(
            "name",
            "Agreement name must be a non-reserved ASCII identifier of at most 64 bytes.",
        ));
    }
    if program.actions.is_empty() {
        return Err(failure("program", "Program needs at least one action."));
    }
    if program.states.len() + program.assets.len() + program.actions.len() + program.duties.len()
        > MAX_DECLARATIONS
        || program.expressions.len() > MAX_EXPRESSIONS
    {
        return Err(failure(
            "bound",
            "Program exceeds finite declaration/expression bounds.",
        ));
    }
    names_v2(
        program
            .states
            .iter()
            .map(|s| s.name.as_str())
            .chain(program.assets.iter().map(|a| a.name.as_str())),
        "declaration",
    )?;
    names_v2(program.actions.iter().map(|a| a.name.as_str()), "action")?;
    let mut identities = BTreeSet::new();
    for asset in &program.assets {
        let id = &asset.identity;
        if id.network.is_empty()
            || id.issuer.is_empty()
            || id.reference.is_empty()
            || id.network.len() > 1024
            || id.issuer.len() > 1024
            || id.reference.len() > 1024
            || id.scale > 38
            || !identities.insert(id)
        {
            return Err(failure(
                "asset",
                "Asset identities must be complete, unique and use scale 0..38.",
            ));
        }
        if program.assets.iter().any(|other| {
            other.identity.network == id.network
                && other.identity.issuer == id.issuer
                && other.identity.reference == id.reference
                && other.identity.scale != id.scale
        }) {
            return Err(failure(
                "asset",
                "Same asset identity cannot have conflicting scales.",
            ));
        }
    }
    let mut depths = Vec::with_capacity(program.expressions.len());
    for (i, e) in program.expressions.iter().enumerate() {
        let refs = match e {
            Expr::Binary { lhs, rhs, .. } => vec![*lhs, *rhs],
            Expr::Not { value } => vec![*value],
            _ => vec![],
        };
        let mut depth = 0;
        for r in refs {
            if usize::from(r) >= i {
                return Err(failure(
                    "expression",
                    "Expression references must point to earlier nodes; cycles are forbidden.",
                ));
            }
            depth = depth.max(depths[usize::from(r)] + 1);
        }
        if depth > 64 {
            return Err(failure("bound", "Expression depth exceeds 64."));
        }
        depths.push(depth);
    }
    let state_types: BTreeMap<_, _> = program
        .states
        .iter()
        .map(|s| (s.name.clone(), s.ty))
        .collect();
    for state in &program.states {
        if state.initial.ty() != state.ty {
            return Err(failure(
                "type",
                format!("Initial state {:?} has wrong type.", state.name),
            ));
        }
    }
    for action in &program.actions {
        if action.inputs.len() > MAX_DECLARATIONS || action.statements.len() > MAX_STATEMENTS {
            return Err(failure(
                "bound",
                "Action exceeds input or statement bounds.",
            ));
        }
        names_v2(action.inputs.iter().map(|i| i.name.as_str()), "input")?;
        let mut env = state_types.clone();
        for input in &action.inputs {
            if env.insert(input.name.clone(), input.ty).is_some()
                || program.assets.iter().any(|a| a.name == input.name)
            {
                return Err(failure("duplicate", "Input cannot shadow a declaration."));
            }
        }
        let mut labels = BTreeSet::new();
        let mut awaits = 0_usize;
        let mut captured = action.inputs.len();
        for statement in &action.statements {
            match statement {
                Statement::Await {
                    label,
                    binders,
                    guard,
                } => {
                    awaits += 1;
                    if awaits > MAX_AWAITS_PER_ACTION {
                        return Err(failure("bound", "Action exceeds 16 awaits."));
                    }
                    if binders.len() > MAX_EVENT_BINDERS {
                        return Err(failure("bound", "Await exceeds 16 event binders."));
                    }
                    if captured > MAX_CAPTURE_SLOTS {
                        return Err(failure("bound", "Await site exceeds 256 capture slots."));
                    }
                    if !identifier_v2(label) || !labels.insert(label.as_str()) {
                        return Err(failure(
                            "duplicate",
                            format!("Invalid or duplicate await label {label:?}."),
                        ));
                    }
                    for binder in binders {
                        if !identifier_v2(&binder.name)
                            || env.insert(binder.name.clone(), binder.ty).is_some()
                            || program.assets.iter().any(|a| a.name == binder.name)
                        {
                            return Err(failure(
                                "duplicate",
                                "Await binder cannot shadow an existing name.",
                            ));
                        }
                    }
                    if expr_type(program, *guard, &env, 0)? != Type::Bool {
                        return Err(failure("type", "Await guard must have type Bool."));
                    }
                    captured += binders.len();
                }
                Statement::Let { name, value } => {
                    let ty = expr_type(program, *value, &env, 0)?;
                    if !identifier_v2(name)
                        || env.insert(name.clone(), ty).is_some()
                        || program.assets.iter().any(|a| a.name == *name)
                    {
                        return Err(failure("duplicate", "let cannot shadow an existing name."));
                    }
                    captured += 1;
                }
                Statement::Requires { predicate } | Statement::Ensures { predicate } => {
                    if expr_type(program, *predicate, &env, 0)? != Type::Bool {
                        return Err(failure("type", "Predicate must have type Bool."));
                    }
                }
                Statement::Next { state, value } => {
                    let expected = state_types
                        .get(state)
                        .ok_or_else(|| failure("name", format!("Unknown state {state:?}.")))?;
                    if expr_type(program, *value, &env, 0)? != *expected {
                        return Err(failure("type", "State update has wrong type."));
                    }
                }
                Statement::Emit { name, value } => {
                    if name.is_empty() || name.len() > 1024 {
                        return Err(failure("name", "Event name must contain 1..1024 bytes."));
                    }
                    expr_type(program, *value, &env, 0)?;
                }
                Statement::Transfer {
                    asset,
                    from,
                    to,
                    amount,
                }
                | Statement::Fee {
                    asset,
                    from,
                    to,
                    amount,
                } => {
                    validate_asset(program, asset)?;
                    if from.is_empty() || to.is_empty() || from.len() > 1024 || to.len() > 1024 {
                        return Err(failure("account", "Account identities cannot be empty."));
                    }
                    require_uint(program, *amount, &env)?;
                }
                Statement::Liability {
                    name,
                    asset,
                    debtor,
                    creditor,
                    amount,
                } => {
                    validate_asset(program, asset)?;
                    if name.is_empty()
                        || debtor.is_empty()
                        || creditor.is_empty()
                        || name.len() > 1024
                        || debtor.len() > 1024
                        || creditor.len() > 1024
                    {
                        return Err(failure(
                            "liability",
                            "Liability identities cannot be empty.",
                        ));
                    }
                    require_uint(program, *amount, &env)?;
                }
                Statement::Discharge { name, amount } => {
                    if name.is_empty() || name.len() > 1024 {
                        return Err(failure(
                            "liability",
                            "Liability name must contain 1..1024 bytes.",
                        ));
                    }
                    require_uint(program, *amount, &env)?;
                }
            }
        }
    }
    validate_liability_reads_v3(program)?;
    Ok(())
}

/// Independently re-derives each action's sites and capture layouts and
/// requires the Sol-built plan to equal them, then checks the A3 predicates
/// explicitly. Runs before costing, so cost vectors and hash must be empty.
pub(crate) fn plan_invariants(program: &Program, plan: &StagePlan) -> Result<(), Error> {
    if plan.format != STAGE_PLAN_FORMAT {
        return Err(invariant("format"));
    }
    if !plan.program_hash.is_empty() {
        return Err(invariant("program hash filled before costing"));
    }
    if plan.actions.len() != program.actions.len() {
        return Err(invariant("action count"));
    }
    let state_names: BTreeSet<_> = program.states.iter().map(|s| s.name.as_str()).collect();
    for (action, ap) in program.actions.iter().zip(&plan.actions) {
        if ap.action != action.name || usize::from(ap.statement_count) != action.statements.len() {
            return Err(invariant("action identity"));
        }
        if !ap.segment_cost.is_empty()
            || !ap.committed_before.is_empty()
            || ap.plan_total != UInt(0)
        {
            return Err(invariant("costs filled before costing"));
        }
        // Independent derivation in declaration and statement order.
        let mut types: BTreeMap<_, _> = program
            .states
            .iter()
            .map(|s| (s.name.clone(), s.ty))
            .collect();
        let mut slots = Vec::with_capacity(action.inputs.len());
        for (index, input) in action.inputs.iter().enumerate() {
            types.insert(input.name.clone(), input.ty);
            slots.push(CaptureSlot {
                name: input.name.clone(),
                ty: input.ty,
                origin: Origin::Input,
                ordinal: u16::try_from(index).map_err(|_| invariant("input ordinal"))?,
            });
        }
        let mut expected = Vec::new();
        for (index, statement) in action.statements.iter().enumerate() {
            let ordinal = u16::try_from(index).map_err(|_| invariant("statement ordinal"))?;
            match statement {
                Statement::Let { name, value } => {
                    let ty = expr_type(program, *value, &types, 0)?;
                    types.insert(name.clone(), ty);
                    slots.push(CaptureSlot {
                        name: name.clone(),
                        ty,
                        origin: Origin::Let,
                        ordinal,
                    });
                }
                Statement::Await { label, binders, .. } => {
                    expected.push(AwaitSite {
                        site: u16::try_from(expected.len() + 1).map_err(|_| invariant("site"))?,
                        ordinal,
                        label: label.clone(),
                        binders: binders.iter().map(StageBinder::from).collect(),
                        captures: slots.clone(),
                    });
                    for binder in binders {
                        types.insert(binder.name.clone(), binder.ty);
                        slots.push(CaptureSlot {
                            name: binder.name.clone(),
                            ty: binder.ty,
                            origin: Origin::Event,
                            ordinal,
                        });
                    }
                }
                _ => {}
            }
        }
        if ap.sites != expected {
            return Err(invariant("sites differ from source"));
        }
        // Explicit A3 predicates, stated separately from the derivation.
        if ap.sites.len() > MAX_AWAITS_PER_ACTION {
            return Err(failure("bound", "Action exceeds 16 awaits."));
        }
        let mut previous: Option<&AwaitSite> = None;
        for (index, site) in ap.sites.iter().enumerate() {
            if usize::from(site.site) != index + 1 {
                return Err(invariant("sites are not numbered 1..N"));
            }
            if previous.is_some_and(|p| p.ordinal >= site.ordinal) {
                return Err(invariant("ordinals are not strictly increasing"));
            }
            if site.binders.len() > MAX_EVENT_BINDERS || site.captures.len() > MAX_CAPTURE_SLOTS {
                return Err(failure("bound", "Await site exceeds stage layout bounds."));
            }
            let inputs = action.inputs.len();
            if site.captures.len() < inputs
                || site.captures[..inputs]
                    .iter()
                    .zip(&action.inputs)
                    .enumerate()
                    .any(|(i, (slot, input))| {
                        slot.origin != Origin::Input
                            || slot.name != input.name
                            || slot.ty != input.ty
                            || usize::from(slot.ordinal) != i
                    })
            {
                return Err(invariant("captures do not begin with all inputs"));
            }
            let mut seen = BTreeSet::new();
            for slot in &site.captures {
                if state_names.contains(slot.name.as_str())
                    || !seen.insert(slot.name.as_str())
                    || (slot.origin != Origin::Input && slot.ordinal >= site.ordinal)
                {
                    return Err(invariant(
                        "capture names a state, repeats or follows its site",
                    ));
                }
            }
            if let Some(p) = previous
                && !site.captures.starts_with(&p.captures)
            {
                return Err(invariant("captures are not prefix-monotone"));
            }
            previous = Some(site);
        }
    }
    Ok(())
}

/// Checks the filled cost vectors: N+1 entries, prefix sums, and a total that
/// also equals the per-statement sum plus one entry charge per await.
pub(crate) fn cost_invariants(program: &Program, plan: &StagePlan) -> Result<(), Error> {
    let overflow = || failure("bound", "Stage logical plan exceeds u128.");
    for (action, ap) in program.actions.iter().zip(&plan.actions) {
        let segments = ap.sites.len() + 1;
        if ap.segment_cost.len() != segments || ap.committed_before.len() != segments {
            return Err(invariant("cost vector lengths"));
        }
        let mut running = 0_u128;
        for (cost, before) in ap.segment_cost.iter().zip(&ap.committed_before) {
            if before.0 != running {
                return Err(invariant("committed_before is not a prefix sum"));
            }
            running = running.checked_add(cost.0).ok_or_else(overflow)?;
        }
        if ap.plan_total.0 != running {
            return Err(invariant("plan total"));
        }
        let mut independent = u128::try_from(ap.sites.len()).map_err(|_| overflow())?;
        for statement in &action.statements {
            let cost = super::cost::statement_cost(program, statement)?;
            independent = independent.checked_add(cost.0).ok_or_else(overflow)?;
        }
        if independent != running {
            return Err(invariant("plan total differs from statement costs"));
        }
    }
    Ok(())
}
