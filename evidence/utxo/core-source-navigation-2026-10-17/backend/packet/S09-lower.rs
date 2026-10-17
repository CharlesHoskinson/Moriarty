//! Constraint lowering of one `/1` action, statement by statement.
//!
//! Follows `runtime.rs:702-912` exactly:
//! - statements run in order; there is no branch, so no path is inactive and no gate
//!   is needed;
//! - each statement lowers every node reachable from its root, `lhs` before `rhs`,
//!   with a fresh per-statement memo (strict evaluation; no short-circuit);
//! - `Next` replaces both the state value and the environment binding;
//! - duty reads see the current duty cell, including present-zero;
//! - Transfer/Fee check the debit before the credit and keep the debit check for a
//!   self-transfer;
//! - Liability accrues into the current cell and marks it present; Discharge requires
//!   presence and keeps a present-zero cell.
use crate::gadgets::{self, BoolW, CVal, GadgetError, UIntW};
use crate::plan::ActionPlan;
use crate::r1cs::Cs;
use ark_ff::PrimeField;
use mori_core::{BinaryOp, Expr, ExprId, Program, Statement, UInt, Value};
use std::collections::BTreeMap;

#[derive(Debug)]
pub(crate) enum LowerError {
    Gadget { scope: String, error: GadgetError },
    Shape(String),
}

#[derive(Clone, Debug)]
pub(crate) struct DutyW<F: PrimeField> {
    pub present: BoolW<F>,
    pub amount: UIntW<F>,
}

#[derive(Clone, Debug)]
pub(crate) enum EffectW<F: PrimeField> {
    State { before: CVal<F>, after: CVal<F> },
    Amount(UIntW<F>),
    Duty { before: UIntW<F>, after: UIntW<F> },
    Emit(CVal<F>),
}

/// Pre-state and input wires, already bound to their public slots.
pub(crate) struct Entry<F: PrimeField> {
    pub inputs: Vec<CVal<F>>,
    pub states: Vec<CVal<F>>,
    pub cells: Vec<UIntW<F>>,
    pub duties: Vec<DutyW<F>>,
}

/// Post-state and effect wires, to be bound to their public slots.
pub(crate) struct Lowered<F: PrimeField> {
    pub states: Vec<CVal<F>>,
    pub cells: Vec<UIntW<F>>,
    pub duties: Vec<DutyW<F>>,
    pub effects: Vec<EffectW<F>>,
}

struct Ctx<'p, F: PrimeField> {
    program: &'p Program,
    env: BTreeMap<String, CVal<F>>,
    duties: Vec<DutyW<F>>,
    duty_index: BTreeMap<String, usize>,
}

/// Attaches the statement/expression scope to a gadget error. Takes no `Cs` so the
/// gadget call itself can hold the only mutable borrow.
fn gadget<T>(statement: usize, expr: Option<ExprId>, r: Result<T, GadgetError>) -> Result<T, LowerError> {
    r.map_err(|error| LowerError::Gadget {
        scope: match expr {
            Some(e) => format!("s{statement}/e{e}"),
            None => format!("s{statement}"),
        },
        error,
    })
}

fn as_uint<F: PrimeField>(v: CVal<F>) -> Result<UIntW<F>, LowerError> {
    match v {
        CVal::U(u) => Ok(u),
        CVal::B(_) => Err(LowerError::Shape("expected UInt after validation".into())),
    }
}

fn as_bool<F: PrimeField>(v: CVal<F>) -> Result<BoolW<F>, LowerError> {
    match v {
        CVal::B(b) => Ok(b),
        CVal::U(_) => Err(LowerError::Shape("expected Bool after validation".into())),
    }
}

impl<F: PrimeField> Ctx<'_, F> {
    fn duty(&self, name: &str) -> Result<usize, LowerError> {
        self.duty_index.get(name).copied().ok_or_else(|| LowerError::Shape(format!("duty {name:?} absent from plan")))
    }

    /// Lowers node `id` once per statement (memo), operands first, `lhs` before `rhs`.
    fn expr(&self, cs: &mut Cs<F>, statement: usize, id: ExprId, memo: &mut BTreeMap<ExprId, CVal<F>>) -> Result<CVal<F>, LowerError> {
        if let Some(v) = memo.get(&id) {
            return Ok(v.clone());
        }
        let node = self
            .program
            .expressions
            .get(usize::from(id))
            .ok_or_else(|| LowerError::Shape(format!("expression {id} out of range")))?;
        let value = match node {
            Expr::Literal { value } => match value {
                Value::UInt(UInt(n)) => CVal::U(UIntW::constant(*n)),
                Value::Bool(b) => CVal::B(BoolW::constant(*b)),
            },
            Expr::Name { name } => self.env.get(name).cloned().ok_or_else(|| LowerError::Shape(format!("unbound name {name:?}")))?,
            Expr::LiabilityPresent { name } => CVal::B(self.duties[self.duty(name)?].present.clone()),
            Expr::LiabilityAmount { name } => CVal::U(self.duties[self.duty(name)?].amount.clone()),
            Expr::Not { value } => {
                let v = as_bool(self.expr(cs, statement, *value, memo)?)?;
                CVal::B(gadgets::not(&v))
            }
            Expr::Binary { op, lhs, rhs } => {
                let a = self.expr(cs, statement, *lhs, memo)?;
                let b = self.expr(cs, statement, *rhs, memo)?;
                // Statement scope `s{statement}` is already pushed by `lower_action`;
                // witness labels read `s{statement}/e{id}/{role}#n`.
                cs.push(format!("e{id}"));
                let out = self.binary(cs, *op, a, b);
                cs.pop();
                gadget(statement, Some(id), out)?
            }
        };
        memo.insert(id, value.clone());
        Ok(value)
    }

    fn binary(&self, cs: &mut Cs<F>, op: BinaryOp, a: CVal<F>, b: CVal<F>) -> Result<CVal<F>, GadgetError> {
        let shape = |m: &str| GadgetError::Static(format!("operand types differ from validated IR: {m}"));
        Ok(match (op, a, b) {
            (BinaryOp::Add, CVal::U(x), CVal::U(y)) => CVal::U(gadgets::add(cs, "add", &x, &y)?),
            (BinaryOp::Sub, CVal::U(x), CVal::U(y)) => CVal::U(gadgets::sub(cs, "sub", &x, &y)?),
            (BinaryOp::Mul, CVal::U(x), CVal::U(y)) => CVal::U(gadgets::mul(cs, "mul", &x, &y)?),
            (BinaryOp::Lt, CVal::U(x), CVal::U(y)) => CVal::B(gadgets::lt(cs, "lt", &x, &y)?),
            (BinaryOp::Gt, CVal::U(x), CVal::U(y)) => CVal::B(gadgets::lt(cs, "lt", &y, &x)?),
            (BinaryOp::Le, CVal::U(x), CVal::U(y)) => CVal::B(gadgets::not(&gadgets::lt(cs, "lt", &y, &x)?)),
            (BinaryOp::Ge, CVal::U(x), CVal::U(y)) => CVal::B(gadgets::not(&gadgets::lt(cs, "lt", &x, &y)?)),
            (BinaryOp::Eq, CVal::U(x), CVal::U(y)) => CVal::B(gadgets::eq_uint(cs, "eq", &x, &y)?),
            (BinaryOp::Ne, CVal::U(x), CVal::U(y)) => CVal::B(gadgets::not(&gadgets::eq_uint(cs, "eq", &x, &y)?)),
            (BinaryOp::Eq, CVal::B(x), CVal::B(y)) => CVal::B(gadgets::eq_bool(cs, "beq", &x, &y)?),
            (BinaryOp::Ne, CVal::B(x), CVal::B(y)) => CVal::B(gadgets::not(&gadgets::eq_bool(cs, "beq", &x, &y)?)),
            (BinaryOp::And, CVal::B(x), CVal::B(y)) => CVal::B(gadgets::and(cs, "and", &x, &y)?),
            (BinaryOp::Or, CVal::B(x), CVal::B(y)) => CVal::B(gadgets::or(cs, "or", &x, &y)?),
            (op, _, _) => return Err(shape(&format!("{op:?}"))),
        })
    }

    fn root(&self, cs: &mut Cs<F>, statement: usize, id: ExprId) -> Result<CVal<F>, LowerError> {
        let mut memo = BTreeMap::new();
        self.expr(cs, statement, id, &mut memo)
    }
}

/// Lowers the selected action. `entry` wires are the bound pre-state and inputs.
pub(crate) fn lower_action<F: PrimeField>(cs: &mut Cs<F>, program: &Program, plan: &ActionPlan, entry: Entry<F>) -> Result<Lowered<F>, LowerError> {
    let action = program
        .actions
        .get(usize::from(plan.action_index))
        .ok_or_else(|| LowerError::Shape("action index out of range".into()))?;
    if entry.inputs.len() != plan.inputs.len()
        || entry.states.len() != plan.states.len()
        || entry.cells.len() != plan.cells.len()
        || entry.duties.len() != plan.duties.len()
    {
        return Err(LowerError::Shape("entry wires differ from plan".into()));
    }
    let mut env: BTreeMap<String, CVal<F>> = BTreeMap::new();
    for (slot, wire) in plan.states.iter().zip(&entry.states) {
        env.insert(slot.name.clone(), wire.clone());
    }
    for (slot, wire) in plan.inputs.iter().zip(&entry.inputs) {
        env.insert(slot.name.clone(), wire.clone());
    }
    let duty_index: BTreeMap<String, usize> = plan.duties.iter().enumerate().map(|(i, d)| (d.name.clone(), i)).collect();
    let mut ctx = Ctx { program, env, duties: entry.duties, duty_index };
    let mut states = entry.states;
    let mut cells = entry.cells;
    let mut effects = Vec::with_capacity(plan.effects.len());

    for (i, statement) in action.statements.iter().enumerate() {
        let start = cs.num_constraints();
        cs.push(format!("s{i}"));
        let step = lower_statement(cs, &mut ctx, plan, i, statement, &mut states, &mut cells, &mut effects);
        cs.pop();
        step?;
        cs.record(format!("statement {i}"), start);
    }
    if effects.len() != plan.effects.len() {
        return Err(LowerError::Shape("lowered effect count differs from plan".into()));
    }
    Ok(Lowered { states, cells, duties: ctx.duties, effects })
}

#[allow(clippy::too_many_arguments)]
fn lower_statement<F: PrimeField>(
    cs: &mut Cs<F>,
    ctx: &mut Ctx<'_, F>,
    plan: &ActionPlan,
    i: usize,
    statement: &Statement,
    states: &mut [CVal<F>],
    cells: &mut [UIntW<F>],
    effects: &mut Vec<EffectW<F>>,
) -> Result<(), LowerError> {
    match statement {
        Statement::Let { name, value } => {
            let v = ctx.root(cs, i, *value)?;
            ctx.env.insert(name.clone(), v);
        }
        Statement::Requires { predicate } | Statement::Ensures { predicate } => {
            let v = as_bool(ctx.root(cs, i, *predicate)?)?;
            let what = if matches!(statement, Statement::Requires { .. }) { "requires" } else { "ensures" };
            gadget(i, None, gadgets::assert_true(cs, &v, what))?;
        }
        Statement::Next { state, value } => {
            let after = ctx.root(cs, i, *value)?;
            let index = plan.state_index(state).ok_or_else(|| LowerError::Shape(format!("state {state:?} absent")))?;
            let before = std::mem::replace(&mut states[index], after.clone());
            ctx.env.insert(state.clone(), after.clone());
            effects.push(EffectW::State { before, after });
        }
        Statement::Emit { value, .. } => {
            let v = ctx.root(cs, i, *value)?;
            effects.push(EffectW::Emit(v));
        }
        Statement::Transfer { asset, from, to, amount } | Statement::Fee { asset, from, to, amount } => {
            let amt = as_uint(ctx.root(cs, i, *amount)?)?;
            let id = &ctx
                .program
                .assets
                .iter()
                .find(|a| a.name == *asset)
                .ok_or_else(|| LowerError::Shape(format!("asset {asset:?} absent")))?
                .identity;
            let f = plan.cell_index(id, from).ok_or_else(|| LowerError::Shape("debit cell absent".into()))?;
            let t = plan.cell_index(id, to).ok_or_else(|| LowerError::Shape("credit cell absent".into()))?;
            // Debit check first (runtime.rs:685-689), also for a self-transfer.
            let debit = gadget(i, None, gadgets::sub(cs, "debit", &cells[f], &amt))?;
            if f != t {
                let credit = gadget(i, None, gadgets::add(cs, "credit", &cells[t], &amt))?;
                cells[f] = debit;
                cells[t] = credit;
            }
            effects.push(EffectW::Amount(amt));
        }
        Statement::Liability { name, amount, .. } => {
            let amt = as_uint(ctx.root(cs, i, *amount)?)?;
            let d = ctx.duty(name)?;
            // Absent cells carry amount 0 (canonicality enforced at entry), so `before`
            // is the Core `before` in both the present and absent cases.
            let before = ctx.duties[d].amount.clone();
            let after = gadget(i, None, gadgets::add(cs, "accrue", &before, &amt))?;
            ctx.duties[d] = DutyW { present: BoolW::constant(true), amount: after.clone() };
            effects.push(EffectW::Duty { before, after });
        }
        Statement::Discharge { name, amount } => {
            let amt = as_uint(ctx.root(cs, i, *amount)?)?;
            let d = ctx.duty(name)?;
            let present = ctx.duties[d].present.clone();
            gadget(i, None, gadgets::assert_true(cs, &present, "discharged duty presence"))?;
            let before = ctx.duties[d].amount.clone();
            let after = gadget(i, None, gadgets::sub(cs, "discharge", &before, &amt))?;
            ctx.duties[d].amount = after.clone();
            effects.push(EffectW::Duty { before, after });
        }
        Statement::Await { .. } => {
            return Err(LowerError::Shape("Await reached /1 lowering".into()));
        }
    }
    Ok(())
}
