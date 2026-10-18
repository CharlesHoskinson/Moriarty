//! Static analysis of one action of an admitted `/1` Program.
//!
//! Everything here is derived from the typed IR, never from a program name, hash or
//! fixture. The plan fixes the statement shape of the relation: which pre/post cells
//! exist, the ordered effect kinds with their constant fields, and the logical-work
//! constant `W`.
use crate::error::CompileError;
use crate::identity::CompilerBuild;
use mori_core::{AssetId, Expr, ExprId, MAX_TRACE_CELLS, Program, Statement, Type, UInt};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};

/// How the logical-work cap appears in the statement. No default exists: the caller
/// chooses explicitly. This does not select the split D-13 stored-cap convention for
/// `/2`; it only says whether a `/1` claim carries a cap at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum CapBinding {
    /// Statement has no cap slot; the relation matches unmetered `simulate`.
    NotBound,
    /// Statement carries a UInt cap and the circuit enforces `W <= cap`, matching
    /// `simulate_metered(cap)`.
    PublicCap,
}

/// Whether the statement carries an in-circuit accounting summary. No default: the
/// caller chooses. UNADOPTED: this implements the gross-debit and fee part of the
/// one-vote P3-3(d) proposal (claim-utxo-abi-v1 VOTES.md §4) and needs a second vote.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum AccountingBinding {
    NotBound,
    /// For every represented cell named as `from` by a Transfer or Fee, two public
    /// 144-bit sums: gross debit (Transfer and Fee amounts, self-transfers included,
    /// as `intent::check_effects` counts them) and fees paid (Fee amounts).
    GrossDebitAndFees,
}

/// A balance cell named by source asset name and account.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CellRef {
    pub asset: String,
    pub account: String,
}

/// Which balance cells the statement represents beyond those the action touches.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum FrameCells {
    /// Only cells named by the action's Transfer/Fee statements.
    TouchedOnly,
    /// Touched cells plus these extra preserved cells (for example a target layout's
    /// framed matrix). Extra cells are constrained `post == pre`.
    Include(Vec<CellRef>),
}

/// Every compile choice is explicit; there is no `Default`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompileOptions {
    pub action: String,
    pub cap: CapBinding,
    pub frame: FrameCells,
    pub accounting: AccountingBinding,
    pub compiler: CompilerBuild,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NamedType {
    pub name: String,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CellKey {
    pub asset_name: String,
    pub asset: AssetId,
    pub account: String,
    /// Named by a Transfer/Fee of this action (otherwise a preserved frame cell).
    pub touched: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DutyKey {
    pub name: String,
    pub asset_name: String,
    pub asset: AssetId,
    pub debtor: String,
    pub creditor: String,
    /// Created, discharged or read by this action (otherwise preserved).
    pub touched: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum DutyOrigin {
    Accrue,
    Discharge,
}

/// One ordered effect: its kind and every constant field. Variable payloads are
/// statement slots (see `statement.rs`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum EffectShape {
    State { statement: u16, name: String, ty: Type },
    Transfer { statement: u16, asset: AssetId, from: String, to: String, from_cell: usize, to_cell: usize },
    Fee { statement: u16, asset: AssetId, from: String, to: String, from_cell: usize, to_cell: usize },
    Liability { statement: u16, duty: usize, name: String, origin: DutyOrigin },
    Emit { statement: u16, name: String, ty: Type },
}

impl EffectShape {
    /// The debited cell of a Transfer or Fee.
    pub fn debit_cell(&self) -> Option<usize> {
        match self {
            Self::Transfer { from_cell, .. } | Self::Fee { from_cell, .. } => Some(*from_cell),
            Self::State { .. } | Self::Liability { .. } | Self::Emit { .. } => None,
        }
    }
    pub fn is_fee(&self) -> bool {
        matches!(self, Self::Fee { .. })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ActionPlan {
    pub action: String,
    pub action_index: u16,
    pub statement_count: u16,
    /// Declaration order.
    pub inputs: Vec<NamedType>,
    /// Sorted by name (Core `BTreeMap` order).
    pub states: Vec<NamedType>,
    /// Sorted by `(AssetId, account)` (Core balance order, `runtime.rs:628`).
    pub cells: Vec<CellKey>,
    /// Every source-introduced duty of the program, sorted by name.
    pub duties: Vec<DutyKey>,
    pub effects: Vec<EffectShape>,
    /// Represented cells debited by some Transfer/Fee of this action, ascending.
    pub debit_cells: Vec<usize>,
    /// `1 + |distinct nodes reachable from the root|` per statement.
    pub logical_schedule: Vec<UInt>,
    pub logical_work: UInt,
    pub cap: CapBinding,
    pub accounting: AccountingBinding,
    pub frame: FrameCells,
    /// Conservative Core trace-record product for the represented snapshot.
    pub represented_trace_cells: u64,
}

impl ActionPlan {
    pub fn cell_index(&self, asset: &AssetId, account: &str) -> Option<usize> {
        self.cells.iter().position(|c| &c.asset == asset && c.account == account)
    }
    pub fn duty_index(&self, name: &str) -> Option<usize> {
        self.duties.iter().position(|d| d.name == name)
    }
    pub fn state_index(&self, name: &str) -> Option<usize> {
        self.states.iter().position(|s| s.name == name)
    }
}

/// Root expression of a statement, as charged by the logical meter.
pub(crate) fn statement_root(statement: &Statement) -> ExprId {
    match statement {
        Statement::Await { guard, .. } => *guard,
        Statement::Let { value, .. } | Statement::Next { value, .. } | Statement::Emit { value, .. } => *value,
        Statement::Requires { predicate } | Statement::Ensures { predicate } => *predicate,
        Statement::Transfer { amount, .. }
        | Statement::Fee { amount, .. }
        | Statement::Liability { amount, .. }
        | Statement::Discharge { amount, .. } => *amount,
    }
}

/// Distinct nodes reachable from `root` (the per-statement cache charges each once).
pub(crate) fn reachable(program: &Program, root: ExprId) -> Result<BTreeSet<ExprId>, CompileError> {
    let mut seen = BTreeSet::new();
    let mut pending = vec![root];
    while let Some(id) = pending.pop() {
        let node = program.expressions.get(usize::from(id)).ok_or_else(|| CompileError::Bound(format!("expression {id} out of range")))?;
        if !seen.insert(id) {
            continue;
        }
        match node {
            Expr::Binary { lhs, rhs, .. } => {
                pending.push(*lhs);
                pending.push(*rhs);
            }
            Expr::Not { value } => pending.push(*value),
            Expr::LiabilityPresent { .. } | Expr::LiabilityAmount { .. } | Expr::Literal { .. } | Expr::Name { .. } => {}
        }
    }
    Ok(seen)
}

/// Successful logical cost of one statement: one dispatch plus its distinct nodes.
pub(crate) fn statement_cost(program: &Program, statement: &Statement) -> Result<u128, CompileError> {
    Ok(reachable(program, statement_root(statement))?.len() as u128 + 1)
}

/// Type of an expression in a validated program under `env` (Core already proved
/// well-typedness; this only recovers types for slot shapes).
pub(crate) fn infer(program: &Program, id: ExprId, env: &BTreeMap<String, Type>) -> Result<Type, CompileError> {
    let node = program.expressions.get(usize::from(id)).ok_or_else(|| CompileError::Bound(format!("expression {id} out of range")))?;
    Ok(match node {
        Expr::LiabilityPresent { .. } => Type::Bool,
        Expr::LiabilityAmount { .. } => Type::UInt,
        Expr::Literal { value } => value.ty(),
        Expr::Name { name } => *env.get(name).ok_or_else(|| CompileError::Bound(format!("unbound name {name:?} after validation")))?,
        Expr::Not { .. } => Type::Bool,
        Expr::Binary { op, .. } => match op {
            mori_core::BinaryOp::Add | mori_core::BinaryOp::Sub | mori_core::BinaryOp::Mul => Type::UInt,
            mori_core::BinaryOp::Eq
            | mori_core::BinaryOp::Ne
            | mori_core::BinaryOp::Lt
            | mori_core::BinaryOp::Le
            | mori_core::BinaryOp::Gt
            | mori_core::BinaryOp::Ge
            | mori_core::BinaryOp::And
            | mori_core::BinaryOp::Or => Type::Bool,
        },
    })
}

fn asset_by_name<'a>(program: &'a Program, name: &str) -> Result<&'a AssetId, CompileError> {
    program
        .assets
        .iter()
        .find(|a| a.name == name)
        .map(|a| &a.identity)
        .ok_or_else(|| CompileError::Option(format!("unknown asset {name:?}")))
}

struct Registry {
    by_name: BTreeMap<String, (String, AssetId, String, String)>,
}

/// Source-introduced duty identities over every action. Conflicting identities make
/// every Core `simulate` refuse in `scenario_snapshot` (`runtime.rs:500-527,629`).
fn registry(program: &Program) -> Result<Registry, CompileError> {
    let mut by_name: BTreeMap<String, (String, AssetId, String, String)> = BTreeMap::new();
    for statement in program.actions.iter().flat_map(|a| &a.statements) {
        if let Statement::Liability { name, asset, debtor, creditor, .. } = statement {
            let identity = (asset.clone(), asset_by_name(program, asset)?.clone(), debtor.clone(), creditor.clone());
            if let Some(previous) = by_name.insert(name.clone(), identity.clone()) {
                if previous.1 != identity.1 || previous.2 != identity.2 || previous.3 != identity.3 {
                    return Err(CompileError::StaticFailure {
                        scope: "program".into(),
                        reason: format!("conflicting source liability identity for {name:?}; every Core simulate refuses"),
                    });
                }
            }
        }
    }
    Ok(Registry { by_name })
}

/// Builds the plan for `options.action`.
pub fn plan_action(program: &Program, options: &CompileOptions) -> Result<ActionPlan, CompileError> {
    let action_index = program
        .actions
        .iter()
        .position(|a| a.name == options.action)
        .ok_or_else(|| CompileError::UnknownAction(options.action.clone()))?;
    let action = &program.actions[action_index];
    let action_index = u16::try_from(action_index).map_err(|_| CompileError::Bound("action index exceeds u16".into()))?;
    let statement_count =
        u16::try_from(action.statements.len()).map_err(|_| CompileError::Bound("statement count exceeds u16".into()))?;

    let inputs: Vec<NamedType> = action.inputs.iter().map(|i| NamedType { name: i.name.clone(), ty: i.ty }).collect();
    let mut states: Vec<NamedType> = program.states.iter().map(|s| NamedType { name: s.name.clone(), ty: s.ty }).collect();
    states.sort_by(|a, b| a.name.cmp(&b.name));

    let registry = registry(program)?;

    // Cells: touched by this action, then optional preserved frame cells.
    let mut cells: BTreeMap<(AssetId, String), (String, bool)> = BTreeMap::new();
    let mut liability_statements = 0usize;
    let mut touched_duties: BTreeSet<String> = BTreeSet::new();
    for statement in &action.statements {
        match statement {
            Statement::Transfer { asset, from, to, .. } | Statement::Fee { asset, from, to, .. } => {
                let id = asset_by_name(program, asset)?.clone();
                cells.insert((id.clone(), from.clone()), (asset.clone(), true));
                cells.insert((id, to.clone()), (asset.clone(), true));
            }
            Statement::Liability { name, .. } => {
                liability_statements += 1;
                touched_duties.insert(name.clone());
            }
            Statement::Discharge { name, .. } => {
                touched_duties.insert(name.clone());
            }
            Statement::Await { .. } => {
                return Err(CompileError::Unsupported {
                    construct: "Statement::Await",
                    reason: "Await is moriarty-core/2 only; /1 admission refuses it".into(),
                });
            }
            Statement::Let { .. }
            | Statement::Requires { .. }
            | Statement::Ensures { .. }
            | Statement::Next { .. }
            | Statement::Emit { .. } => {}
        }
        for id in reachable(program, statement_root(statement))? {
            if let Expr::LiabilityPresent { name } | Expr::LiabilityAmount { name } = &program.expressions[usize::from(id)] {
                touched_duties.insert(name.clone());
            }
        }
    }
    if let FrameCells::Include(extra) = &options.frame {
        for cell in extra {
            if cell.account.is_empty() || cell.account.len() > 1024 {
                return Err(CompileError::Option("frame cell account must contain 1..1024 bytes".into()));
            }
            let id = asset_by_name(program, &cell.asset)?.clone();
            cells.entry((id, cell.account.clone())).or_insert((cell.asset.clone(), false));
        }
    }
    let cells: Vec<CellKey> = cells
        .into_iter()
        .map(|((asset, account), (asset_name, touched))| CellKey { asset_name, asset, account, touched })
        .collect();

    // Duties: the whole source registry. A duty this action touches or reads without
    // a source introduction would need a dynamic identity cell: refused (first slice).
    for name in &touched_duties {
        if !registry.by_name.contains_key(name) {
            return Err(CompileError::Unsupported {
                construct: "external duty",
                reason: format!(
                    "duty {name:?} has no source Liability introduction; its identity would be prover data. \
                     First slice represents only source-fixed duty identities."
                ),
            });
        }
    }
    let duties: Vec<DutyKey> = registry
        .by_name
        .iter()
        .map(|(name, (asset_name, asset, debtor, creditor))| DutyKey {
            name: name.clone(),
            asset_name: asset_name.clone(),
            asset: asset.clone(),
            debtor: debtor.clone(),
            creditor: creditor.clone(),
            touched: touched_duties.contains(name),
        })
        .collect();

    // Effect shapes and logical schedule, statement by statement.
    let mut env: BTreeMap<String, Type> = program.states.iter().map(|s| (s.name.clone(), s.ty)).collect();
    for input in &action.inputs {
        env.insert(input.name.clone(), input.ty);
    }
    let cell_at = |asset: &AssetId, account: &str| -> Result<usize, CompileError> {
        cells
            .iter()
            .position(|c| &c.asset == asset && c.account == account)
            .ok_or_else(|| CompileError::Bound("touched cell missing from plan".into()))
    };
    let duty_at = |name: &str| -> Result<usize, CompileError> {
        duties.iter().position(|d| d.name == name).ok_or_else(|| CompileError::Bound("duty missing from plan".into()))
    };
    let mut effects = Vec::new();
    let mut schedule = Vec::with_capacity(action.statements.len());
    let mut total: u128 = 0;
    for (index, statement) in action.statements.iter().enumerate() {
        let ordinal = index as u16;
        let cost = statement_cost(program, statement)?;
        total = total.checked_add(cost).ok_or_else(|| CompileError::Bound("logical work exceeds u128".into()))?;
        schedule.push(UInt(cost));
        match statement {
            Statement::Let { name, value } => {
                let ty = infer(program, *value, &env)?;
                env.insert(name.clone(), ty);
            }
            Statement::Next { state, .. } => {
                let ty = *env.get(state).ok_or_else(|| CompileError::Bound(format!("unknown state {state:?}")))?;
                effects.push(EffectShape::State { statement: ordinal, name: state.clone(), ty });
            }
            Statement::Emit { name, value } => {
                let ty = infer(program, *value, &env)?;
                effects.push(EffectShape::Emit { statement: ordinal, name: name.clone(), ty });
            }
            Statement::Transfer { asset, from, to, .. } | Statement::Fee { asset, from, to, .. } => {
                let id = asset_by_name(program, asset)?.clone();
                let from_cell = cell_at(&id, from)?;
                let to_cell = cell_at(&id, to)?;
                effects.push(if matches!(statement, Statement::Fee { .. }) {
                    EffectShape::Fee { statement: ordinal, asset: id, from: from.clone(), to: to.clone(), from_cell, to_cell }
                } else {
                    EffectShape::Transfer { statement: ordinal, asset: id, from: from.clone(), to: to.clone(), from_cell, to_cell }
                });
            }
            Statement::Liability { name, .. } => {
                effects.push(EffectShape::Liability { statement: ordinal, duty: duty_at(name)?, name: name.clone(), origin: DutyOrigin::Accrue });
            }
            Statement::Discharge { name, .. } => {
                effects.push(EffectShape::Liability { statement: ordinal, duty: duty_at(name)?, name: name.clone(), origin: DutyOrigin::Discharge });
            }
            Statement::Requires { .. } | Statement::Ensures { .. } => {}
            Statement::Await { .. } => unreachable!("refused above"),
        }
    }
    let debit_cells: Vec<usize> =
        effects.iter().filter_map(EffectShape::debit_cell).collect::<BTreeSet<_>>().into_iter().collect();

    // Core refuses scenarios whose trace product exceeds MAX_TRACE_CELLS
    // (runtime.rs:540-547). Counting every represented duty as present is the
    // conservative bound used by the Cardano layout as well (layout.rs:47-57). Rows
    // outside the represented snapshot are a target premise (see adapter.rs).
    let records = program.states.len() + cells.len() + duties.len() + liability_statements + 1;
    let trace = records
        .checked_mul(action.statements.len() + 2)
        .ok_or_else(|| CompileError::Bound("trace product overflow".into()))?;
    if trace > MAX_TRACE_CELLS {
        return Err(CompileError::Bound(format!(
            "represented snapshot needs {trace} trace records, above Core's {MAX_TRACE_CELLS}"
        )));
    }
    if cells.len() > 4096 || duties.len() > 1024 {
        return Err(CompileError::Bound("represented cells exceed Core's 4,096 balances or 1,024 liabilities".into()));
    }

    Ok(ActionPlan {
        action: action.name.clone(),
        action_index,
        statement_count,
        inputs,
        states,
        cells,
        duties,
        effects,
        debit_cells,
        logical_schedule: schedule,
        logical_work: UInt(total),
        cap: options.cap,
        accounting: options.accounting,
        frame: options.frame.clone(),
        represented_trace_cells: trace as u64,
    })
}
