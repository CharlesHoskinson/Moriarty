use crate::*;
use std::collections::{BTreeMap, BTreeSet};

fn failure(code: &str, message: impl Into<String>) -> Error {
    Error::new(code, message)
}
fn identifier(name: &str) -> bool {
    let mut bytes = name.bytes();
    matches!(bytes.next(), Some(b'A'..=b'Z' | b'a'..=b'z'))
        && name.len() <= 64
        && bytes.all(|b| b.is_ascii_alphanumeric() || b == b'_')
        && !matches!(
            name,
            "language"
                | "agreement"
                | "state"
                | "asset"
                | "action"
                | "input"
                | "let"
                | "next"
                | "requires"
                | "ensures"
                | "emit"
                | "transfer"
                | "fee"
                | "liability"
                | "discharge"
                | "true"
                | "false"
                | "and"
                | "or"
                | "not"
                | "UInt"
                | "Bool"
        )
}
fn names<'a>(values: impl Iterator<Item = &'a str>, kind: &str) -> Result<(), Error> {
    let mut seen = BTreeSet::new();
    for name in values {
        if !identifier(name) || !seen.insert(name) {
            return Err(failure(
                "duplicate",
                format!("Invalid or duplicate {kind} name {name:?}."),
            ));
        }
    }
    Ok(())
}
fn expr_type(
    program: &Program,
    id: ExprId,
    env: &BTreeMap<String, Type>,
    depth: usize,
) -> Result<Type, Error> {
    type_cached(
        program,
        id,
        env,
        depth,
        &mut vec![None; program.expressions.len()],
    )
}
fn type_cached(
    program: &Program,
    id: ExprId,
    env: &BTreeMap<String, Type>,
    depth: usize,
    cache: &mut [Option<Type>],
) -> Result<Type, Error> {
    if depth > 64 {
        return Err(failure("bound", "Expression depth exceeds 64."));
    }
    let index = usize::from(id);
    let node = program
        .expressions
        .get(index)
        .ok_or_else(|| failure("expression", "Expression index out of bounds."))?;
    if let Some(ty) = cache[index] {
        return Ok(ty);
    }
    let result = match node {
        Expr::LiabilityPresent { .. } => Ok(Type::Bool),
        Expr::LiabilityAmount { .. } => Ok(Type::UInt),
        Expr::Literal { value } => Ok(value.ty()),
        Expr::Name { name } => env
            .get(name)
            .copied()
            .ok_or_else(|| failure("name", format!("Unknown or forward name {name:?}."))),
        Expr::Not { value } => {
            if type_cached(program, *value, env, depth + 1, cache)? != Type::Bool {
                return Err(failure("type", "not requires Bool."));
            }
            Ok(Type::Bool)
        }
        Expr::Binary { op, lhs, rhs } => {
            let a = type_cached(program, *lhs, env, depth + 1, cache)?;
            let b = type_cached(program, *rhs, env, depth + 1, cache)?;
            let expected = match op {
                BinaryOp::And | BinaryOp::Or => Type::Bool,
                _ => Type::UInt,
            };
            if a != b || (!matches!(op, BinaryOp::Eq | BinaryOp::Ne) && a != expected) {
                return Err(failure("type", "Binary operands have incompatible types."));
            }
            Ok(
                if matches!(op, BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul) {
                    Type::UInt
                } else {
                    Type::Bool
                },
            )
        }
    }?;
    cache[index] = Some(result);
    Ok(result)
}
/// Validates even manually constructed/deserialized IR before interpretation.
pub fn validate(program: &Program) -> Result<(), Error> {
    if program.language != LANGUAGE || program.semantics != SEMANTICS {
        return Err(failure(
            "profile",
            "Unsupported language or semantics identity.",
        ));
    }
    if !identifier(&program.name) {
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
    names(
        program
            .states
            .iter()
            .map(|s| s.name.as_str())
            .chain(program.assets.iter().map(|a| a.name.as_str())),
        "declaration",
    )?;
    names(program.actions.iter().map(|a| a.name.as_str()), "action")?;
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
        names(action.inputs.iter().map(|i| i.name.as_str()), "input")?;
        let mut env = state_types.clone();
        for input in &action.inputs {
            if env.insert(input.name.clone(), input.ty).is_some()
                || program.assets.iter().any(|a| a.name == input.name)
            {
                return Err(failure("duplicate", "Input cannot shadow a declaration."));
            }
        }
        for statement in &action.statements {
            match statement {
                Statement::Let { name, value } => {
                    let ty = expr_type(program, *value, &env, 0)?;
                    if !identifier(name)
                        || env.insert(name.clone(), ty).is_some()
                        || program.assets.iter().any(|a| a.name == *name)
                    {
                        return Err(failure("duplicate", "let cannot shadow an existing name."));
                    }
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
// New reads require a source-fixed identity. Legacy Core external duties and
// discharge-only programs keep their existing validation rules.
fn validate_liability_reads(p: &Program) -> Result<(), Error> {
    let mut reads = BTreeMap::new();
    for expr in &p.expressions {
        if let Expr::LiabilityPresent { name } | Expr::LiabilityAmount { name } = expr {
            if name.is_empty() || name.len() > 1024 {
                return Err(failure(
                    "liability",
                    "Liability read name must contain 1..1024 bytes.",
                ));
            }
            reads.insert(name.as_str(), None);
        }
    }
    for stmt in p.actions.iter().flat_map(|a| &a.statements) {
        if let Statement::Liability {
            name,
            asset,
            debtor,
            creditor,
            ..
        } = stmt
        {
            if let Some(found) = reads.get_mut(name.as_str()) {
                let identity = (
                    validate_asset(p, asset)?,
                    debtor.as_str(),
                    creditor.as_str(),
                );
                if found.is_some_and(|previous| previous != identity) {
                    return Err(failure(
                        "liability",
                        format!("Conflicting source liability identity for {name:?}."),
                    ));
                }
                *found = Some(identity);
            }
        }
    }
    for (name, identity) in reads {
        if identity.is_none() {
            return Err(failure(
                "liability",
                format!("Liability read {name:?} has no source-derived identity."),
            ));
        }
    }
    Ok(())
}

fn require_uint(p: &Program, id: ExprId, env: &BTreeMap<String, Type>) -> Result<(), Error> {
    if expr_type(p, id, env, 0)? != Type::UInt {
        Err(failure(
            "type",
            "Financial amount requires UInt atomic units.",
        ))
    } else {
        Ok(())
    }
}
fn validate_asset<'a>(p: &'a Program, name: &str) -> Result<&'a AssetId, Error> {
    p.assets
        .iter()
        .find(|a| a.name == name)
        .map(|a| &a.identity)
        .ok_or_else(|| failure("asset", format!("Unknown asset {name:?}.")))
}

fn evaluate(
    p: &Program,
    id: ExprId,
    env: &BTreeMap<String, Value>,
    snapshot: &Snapshot,
) -> Result<Value, Error> {
    eval_cached(p, id, env, snapshot, &mut vec![None; p.expressions.len()])
}
fn eval_cached(
    p: &Program,
    id: ExprId,
    env: &BTreeMap<String, Value>,
    snapshot: &Snapshot,
    cache: &mut [Option<Value>],
) -> Result<Value, Error> {
    let i = usize::from(id);
    if let Some(value) = &cache[i] {
        return Ok(value.clone());
    }
    let result = match &p.expressions[i] {
        Expr::LiabilityPresent { name } | Expr::LiabilityAmount { name } => {
            let row = snapshot
                .liabilities
                .binary_search_by(|l| l.name.as_str().cmp(name.as_str()))
                .ok()
                .map(|index| &snapshot.liabilities[index]);
            if matches!(&p.expressions[i], Expr::LiabilityPresent { .. }) {
                Value::Bool(row.is_some())
            } else {
                Value::UInt(row.map_or(UInt(0), |l| l.amount))
            }
        }
        Expr::Literal { value } => value.clone(),
        Expr::Name { name } => env
            .get(name)
            .cloned()
            .ok_or_else(|| failure("name", format!("Unknown name {name:?}.")))?,
        Expr::Not { value } => match eval_cached(p, *value, env, snapshot, cache)? {
            Value::Bool(b) => Value::Bool(!b),
            _ => return Err(failure("type", "not expects Bool.")),
        },
        Expr::Binary { op, lhs, rhs } => {
            let a = eval_cached(p, *lhs, env, snapshot, cache)?;
            let b = eval_cached(p, *rhs, env, snapshot, cache)?;
            match (op, a, b) {
                (BinaryOp::Add, Value::UInt(UInt(a)), Value::UInt(UInt(b))) => {
                    Value::UInt(UInt(a.checked_add(b).ok_or_else(|| {
                        failure("overflow", "UInt addition exceeds u128.")
                    })?))
                }
                (BinaryOp::Sub, Value::UInt(UInt(a)), Value::UInt(UInt(b))) => {
                    Value::UInt(UInt(a.checked_sub(b).ok_or_else(|| {
                        failure("underflow", "UInt subtraction is negative.")
                    })?))
                }
                (BinaryOp::Mul, Value::UInt(UInt(a)), Value::UInt(UInt(b))) => {
                    Value::UInt(UInt(a.checked_mul(b).ok_or_else(|| {
                        failure("overflow", "UInt multiplication exceeds u128.")
                    })?))
                }
                (BinaryOp::Eq, a, b) => Value::Bool(a == b),
                (BinaryOp::Ne, a, b) => Value::Bool(a != b),
                (BinaryOp::Lt, Value::UInt(a), Value::UInt(b)) => Value::Bool(a < b),
                (BinaryOp::Le, Value::UInt(a), Value::UInt(b)) => Value::Bool(a <= b),
                (BinaryOp::Gt, Value::UInt(a), Value::UInt(b)) => Value::Bool(a > b),
                (BinaryOp::Ge, Value::UInt(a), Value::UInt(b)) => Value::Bool(a >= b),
                (BinaryOp::And, Value::Bool(a), Value::Bool(b)) => Value::Bool(a && b),
                (BinaryOp::Or, Value::Bool(a), Value::Bool(b)) => Value::Bool(a || b),
                _ => return Err(failure("type", "Invalid operand types.")),
            }
        }
    };
    cache[i] = Some(result.clone());
    Ok(result)
}
fn amount(
    p: &Program,
    id: ExprId,
    env: &BTreeMap<String, Value>,
    snapshot: &Snapshot,
) -> Result<UInt, Error> {
    match evaluate(p, id, env, snapshot)? {
        Value::UInt(v) => Ok(v),
        _ => Err(failure("type", "Expected UInt amount.")),
    }
}
// Core also supports incoming duties without a source declaration. Only names
// declared by Liability statements have a source-fixed identity; do not impose
// the backend's closed, sixteen-entry compiler profile on general Core input.
fn liability_identities(p: &Program) -> Result<BTreeMap<&str, (&AssetId, &str, &str)>, Error> {
    let mut identities = BTreeMap::new();
    for stmt in p.actions.iter().flat_map(|action| &action.statements) {
        if let Statement::Liability {
            name,
            asset,
            debtor,
            creditor,
            ..
        } = stmt
        {
            let identity = (
                validate_asset(p, asset)?,
                debtor.as_str(),
                creditor.as_str(),
            );
            if let Some(existing) = identities.insert(name.as_str(), identity) {
                if existing != identity {
                    return Err(failure(
                        "liability",
                        format!("Conflicting source liability identity for {name:?}."),
                    ));
                }
            }
        }
    }
    Ok(identities)
}
fn scenario_snapshot(p: &Program, a: &Action, s: &Scenario) -> Result<Snapshot, Error> {
    if s.balances.len() > 4096 || s.liabilities.len() > 1024 {
        return Err(failure(
            "bound",
            "Scenario exceeds 4,096 balances or 1,024 liabilities.",
        ));
    }
    let added_liabilities = a
        .statements
        .iter()
        .filter(|stmt| matches!(stmt, Statement::Liability { .. }))
        .count();
    let trace_records =
        p.states.len() + s.balances.len() + s.liabilities.len() + added_liabilities + 1;
    if trace_records.saturating_mul(a.statements.len() + 2) > MAX_TRACE_CELLS {
        return Err(failure(
            "bound",
            "Scenario and action exceed 65,536 trace snapshot records.",
        ));
    }
    if !s.state.is_empty() && s.state.len() != p.states.len() {
        return Err(failure(
            "state",
            "Provide either initial state {} or every declared state value.",
        ));
    }
    let mut state = BTreeMap::new();
    for d in &p.states {
        let value = if s.state.is_empty() {
            d.initial.clone()
        } else {
            s.state
                .get(&d.name)
                .ok_or_else(|| failure("state", format!("Missing state {:?}.", d.name)))?
                .clone()
        };
        if value.ty() != d.ty {
            return Err(failure(
                "state",
                format!("Wrong type for state {:?}.", d.name),
            ));
        }
        state.insert(d.name.clone(), value);
    }
    if s.inputs.len() != a.inputs.len() {
        return Err(failure(
            "input",
            "Scenario must provide exactly the selected action's inputs.",
        ));
    }
    for input in &a.inputs {
        if s.inputs.get(&input.name).map(Value::ty) != Some(input.ty) {
            return Err(failure(
                "input",
                format!("Missing or wrong type for input {:?}.", input.name),
            ));
        }
    }
    let ids: BTreeSet<_> = p.assets.iter().map(|a| a.identity.clone()).collect();
    let mut keys = BTreeSet::new();
    let mut balances = s.balances.clone();
    for b in &balances {
        if !ids.contains(&b.asset) {
            return Err(failure(
                "asset",
                format!(
                    "Undeclared exact asset identity for account {:?}.",
                    b.account
                ),
            ));
        }
        if b.account.is_empty()
            || b.account.len() > 1024
            || !keys.insert((b.asset.clone(), b.account.clone()))
        {
            return Err(failure(
                "balance",
                "Duplicate or invalid balance account/asset.",
            ));
        }
    }
    for stmt in &a.statements {
        if let Statement::Transfer {
            asset, from, to, ..
        }
        | Statement::Fee {
            asset, from, to, ..
        } = stmt
        {
            let id = validate_asset(p, asset)?;
            for account in [from, to] {
                if !keys.contains(&(id.clone(), account.clone())) {
                    return Err(failure(
                        "balance",
                        format!("Missing explicit balance for {account:?} and asset {asset:?}."),
                    ));
                }
            }
        }
    }
    balances.sort_by(|a, b| (&a.asset, &a.account).cmp(&(&b.asset, &b.account)));
    let identities = liability_identities(p)?;
    let mut liabilities = s.liabilities.clone();
    let mut names = BTreeSet::new();
    for l in &liabilities {
        if !ids.contains(&l.asset) {
            return Err(failure(
                "asset",
                "Liability uses undeclared exact asset identity.",
            ));
        }
        if l.name.is_empty()
            || l.name.len() > 1024
            || l.debtor.is_empty()
            || l.creditor.is_empty()
            || l.debtor.len() > 1024
            || l.creditor.len() > 1024
            || !names.insert(l.name.clone())
        {
            return Err(failure(
                "liability",
                "Invalid or duplicate liability identity.",
            ));
        }
        if let Some(identity) = identities.get(l.name.as_str()) {
            if *identity != (&l.asset, l.debtor.as_str(), l.creditor.as_str()) {
                return Err(failure(
                    "liability",
                    format!("Existing liability identity differs for {:?}.", l.name),
                ));
            }
        }
    }
    liabilities.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(Snapshot {
        state,
        balances,
        liabilities,
    })
}
fn move_funds(
    snapshot: &mut Snapshot,
    id: &AssetId,
    from: &str,
    to: &str,
    amount: UInt,
) -> Result<(), Error> {
    let f = snapshot
        .balances
        .iter()
        .position(|b| b.asset == *id && b.account == from)
        .ok_or_else(|| failure("balance", "Missing source balance."))?;
    let t = snapshot
        .balances
        .iter()
        .position(|b| b.asset == *id && b.account == to)
        .ok_or_else(|| failure("balance", "Missing recipient balance."))?;
    let debit = snapshot.balances[f]
        .amount
        .0
        .checked_sub(amount.0)
        .ok_or_else(|| failure("balance", format!("Insufficient balance for {from:?}.")))?;
    if f != t {
        let credit = snapshot.balances[t]
            .amount
            .0
            .checked_add(amount.0)
            .ok_or_else(|| failure("overflow", "Recipient balance exceeds u128."))?;
        snapshot.balances[f].amount = UInt(debit);
        snapshot.balances[t].amount = UInt(credit);
    }
    Ok(())
}
/// Executes one finite action on a clone. A refusal exposes no committed effects.
pub fn simulate(program: &Program, action: &str, scenario: &Scenario) -> Result<Execution, Error> {
    validate(program)?;
    let selected = program
        .actions
        .iter()
        .find(|a| a.name == action)
        .ok_or_else(|| failure("action", format!("Unknown action {action:?}.")))?;
    let pre = scenario_snapshot(program, selected, scenario)?;
    let mut current = pre.clone();
    let mut env = current.state.clone();
    env.extend(scenario.inputs.clone());
    let mut steps = vec![];
    let mut all_effects = vec![];
    for (i, stmt) in selected.statements.iter().enumerate() {
        let mut effects = vec![];
        let result = (|| -> Result<Option<Value>, Error> {
            match stmt {
                Statement::Let { name, value } => {
                    let v = evaluate(program, *value, &env, &current)?;
                    env.insert(name.clone(), v.clone());
                    Ok(Some(v))
                }
                Statement::Requires { predicate } | Statement::Ensures { predicate } => {
                    let v = evaluate(program, *predicate, &env, &current)?;
                    if v != Value::Bool(true) {
                        return Err(failure(
                            if matches!(stmt, Statement::Requires { .. }) {
                                "requires"
                            } else {
                                "ensures"
                            },
                            "Action predicate evaluated to false.",
                        ));
                    }
                    Ok(Some(v))
                }
                Statement::Next { state, value } => {
                    let after = evaluate(program, *value, &env, &current)?;
                    let before = current.state[state].clone();
                    current.state.insert(state.clone(), after.clone());
                    env.insert(state.clone(), after.clone());
                    effects.push(Effect::State {
                        name: state.clone(),
                        before,
                        after: after.clone(),
                    });
                    Ok(Some(after))
                }
                Statement::Emit { name, value } => {
                    let v = evaluate(program, *value, &env, &current)?;
                    effects.push(Effect::Emit {
                        name: name.clone(),
                        value: v.clone(),
                    });
                    Ok(Some(v))
                }
                Statement::Transfer {
                    asset,
                    from,
                    to,
                    amount: expr,
                }
                | Statement::Fee {
                    asset,
                    from,
                    to,
                    amount: expr,
                } => {
                    let amount = amount(program, *expr, &env, &current)?;
                    let id = validate_asset(program, asset)?.clone();
                    move_funds(&mut current, &id, from, to, amount)?;
                    effects.push(if matches!(stmt, Statement::Fee { .. }) {
                        Effect::Fee {
                            asset: id,
                            from: from.clone(),
                            to: to.clone(),
                            amount,
                        }
                    } else {
                        Effect::Transfer {
                            asset: id,
                            from: from.clone(),
                            to: to.clone(),
                            amount,
                        }
                    });
                    Ok(Some(Value::UInt(amount)))
                }
                Statement::Liability {
                    name,
                    asset,
                    debtor,
                    creditor,
                    amount: expr,
                } => {
                    let amount = amount(program, *expr, &env, &current)?;
                    let id = validate_asset(program, asset)?.clone();
                    let found = current.liabilities.iter().position(|l| l.name == *name);
                    let before = if let Some(index) = found {
                        let l = &current.liabilities[index];
                        if l.asset != id || l.debtor != *debtor || l.creditor != *creditor {
                            return Err(failure(
                                "liability",
                                "Existing liability identity differs.",
                            ));
                        }
                        l.amount
                    } else {
                        UInt(0)
                    };
                    let after = UInt(
                        before
                            .0
                            .checked_add(amount.0)
                            .ok_or_else(|| failure("overflow", "Liability exceeds u128."))?,
                    );
                    if let Some(index) = found {
                        current.liabilities[index].amount = after;
                    } else {
                        current.liabilities.push(Liability {
                            name: name.clone(),
                            asset: id.clone(),
                            debtor: debtor.clone(),
                            creditor: creditor.clone(),
                            amount: after,
                        });
                        current.liabilities.sort_by(|a, b| a.name.cmp(&b.name));
                    }
                    effects.push(Effect::Liability {
                        name: name.clone(),
                        asset: id,
                        debtor: debtor.clone(),
                        creditor: creditor.clone(),
                        before,
                        after,
                    });
                    Ok(Some(Value::UInt(amount)))
                }
                Statement::Discharge { name, amount: expr } => {
                    let amount = amount(program, *expr, &env, &current)?;
                    let l = current
                        .liabilities
                        .iter_mut()
                        .find(|l| l.name == *name)
                        .ok_or_else(|| {
                            failure("liability", format!("Unknown residual liability {name:?}."))
                        })?;
                    let before = l.amount;
                    let after = UInt(before.0.checked_sub(amount.0).ok_or_else(|| {
                        failure("liability", "Discharge exceeds residual liability.")
                    })?);
                    l.amount = after;
                    effects.push(Effect::Liability {
                        name: name.clone(),
                        asset: l.asset.clone(),
                        debtor: l.debtor.clone(),
                        creditor: l.creditor.clone(),
                        before,
                        after,
                    });
                    Ok(Some(Value::UInt(amount)))
                }
            }
        })();
        let value = result.map_err(|mut e| {
            e.step = Some(i as u16);
            e
        })?;
        all_effects.extend(effects.clone());
        steps.push(Step {
            index: i as u16,
            statement: stmt.clone(),
            value,
            snapshot: current.clone(),
            effects,
        });
    }
    Ok(Execution {
        semantics: SEMANTICS.into(),
        action: action.into(),
        pre,
        post: current,
        steps,
        effects: all_effects,
    })
}
/// Recomputes all steps and rejects at the first deterministic divergence.
pub fn replay(
    program: &Program,
    action: &str,
    scenario: &Scenario,
    expected: &Execution,
) -> Result<(), Error> {
    let actual = simulate(program, action, scenario)?;
    if actual.semantics != expected.semantics
        || actual.action != expected.action
        || actual.pre != expected.pre
    {
        return Err(failure(
            "replay",
            "Execution identity or predecessor differs.",
        ));
    }
    for i in 0..actual.steps.len().max(expected.steps.len()) {
        if actual.steps.get(i) != expected.steps.get(i) {
            let mut e = failure("replay", format!("First divergence at step {i}."));
            e.step = Some(i as u16);
            return Err(e);
        }
    }
    if actual.effects != expected.effects || actual.post != expected.post {
        return Err(failure(
            "replay",
            "Complete effect sequence or poststate differs.",
        ));
    }
    Ok(())
}
