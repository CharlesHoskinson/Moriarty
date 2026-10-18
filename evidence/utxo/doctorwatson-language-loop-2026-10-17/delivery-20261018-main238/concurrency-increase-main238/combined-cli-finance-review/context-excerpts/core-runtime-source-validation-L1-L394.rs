use crate::policy::{CostPlane, Meter};
use crate::*;
use std::collections::{BTreeMap, BTreeSet};

fn failure(code: &str, message: impl Into<String>) -> Error {
    Error::new(code, message)
}
pub(crate) fn identifier(name: &str) -> bool {
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
pub(crate) fn expr_type(
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
                // Same identity as `stage::require_core1`; only an in-memory
                // program can carry Await here, since `/1` JSON cannot.
                Statement::Await { .. } => {
                    return Err(failure("profile", "Await requires moriarty-core/2."));
                }
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
pub(crate) fn validate_liability_reads(p: &Program) -> Result<(), Error> {
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
            && let Some(found) = reads.get_mut(name.as_str())
        {
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

pub(crate) fn require_uint(
    p: &Program,
    id: ExprId,
    env: &BTreeMap<String, Type>,
) -> Result<(), Error> {
    if expr_type(p, id, env, 0)? != Type::UInt {
        Err(failure(
            "type",
            "Financial amount requires UInt atomic units.",
        ))
    } else {
        Ok(())
    }
}
pub(crate) fn validate_asset<'a>(p: &'a Program, name: &str) -> Result<&'a AssetId, Error> {
    p.assets
        .iter()
        .find(|a| a.name == name)
        .map(|a| &a.identity)
        .ok_or_else(|| failure("asset", format!("Unknown asset {name:?}.")))
}

