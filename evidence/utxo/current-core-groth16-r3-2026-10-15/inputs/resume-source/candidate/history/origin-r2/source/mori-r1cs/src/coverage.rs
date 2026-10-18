//! Construct coverage and refusal semantics of this compiler, as data.
//!
//! The `*_support` functions match the actual `mori_core` enums exhaustively, with no
//! wildcard, so a new Core variant fails to compile here until its row is written.
//! COVERAGE.md is the prose rendering of these tables; this module is the source.
use mori_core::{BinaryOp, Expr, Statement, Value};
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Support {
    /// Constrained for admitted successful transitions.
    Lowered,
    /// Admitted by Core, refused by this compiler at compile time with a reason.
    RefusedAtCompile,
    /// Admitted and planned (`/2`), refused with a lowering plan.
    PlannedNotLowered,
    /// A Core admission fact the relation relies on; not a circuit constraint.
    AdmissionFact,
    /// A relation constant (bound in-circuit through the header or shape digest).
    Constant,
    /// Not in the public statement: constrained internally or outside the relation.
    Internal,
    /// Outside the transition relation; a named target obligation.
    TargetObligation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Row {
    pub item: &'static str,
    pub support: Support,
    pub lowering: &'static str,
    pub source: &'static str,
}

const fn row(item: &'static str, support: Support, lowering: &'static str, source: &'static str) -> Row {
    Row { item, support, lowering, source }
}

pub fn expr_support(e: &Expr) -> Row {
    match e {
        Expr::LiabilityPresent { .. } => row(
            "Expr::LiabilityPresent",
            Support::Lowered,
            "current duty cell `present` wire (Bool); present-zero reads true",
            "runtime.rs:430-441; lower.rs Ctx::expr",
        ),
        Expr::LiabilityAmount { .. } => row(
            "Expr::LiabilityAmount",
            Support::Lowered,
            "current duty cell `amount` wire; absent cells are canonical (false, 0)",
            "runtime.rs:430-441; circuit.rs entry canonicality",
        ),
        Expr::Literal { value } => match value {
            Value::UInt(_) => row("Expr::Literal(UInt)", Support::Lowered, "constant limbs; constant folding with checked u128", "runtime.rs:442"),
            Value::Bool(_) => row("Expr::Literal(Bool)", Support::Lowered, "constant 0/1", "runtime.rs:442"),
        },
        Expr::Name { .. } => row(
            "Expr::Name",
            Support::Lowered,
            "environment wire: pre states + inputs, Let bindings, Next rebinding",
            "runtime.rs:443-446, 716-719, 735-746",
        ),
        Expr::Binary { op, .. } => binary_support(*op),
        Expr::Not { .. } => row("Expr::Not", Support::Lowered, "1 - b", "runtime.rs:447-450"),
    }
}

pub fn binary_support(op: BinaryOp) -> Row {
    match op {
        BinaryOp::Add => row("BinaryOp::Add", Support::Lowered, "a + b = c, c 128-bit range-proven; unsatisfiable on overflow", "runtime.rs:455-459; gadgets::add"),
        BinaryOp::Sub => row("BinaryOp::Sub", Support::Lowered, "b + c = a, c range-proven; unsatisfiable on underflow", "runtime.rs:460-464; gadgets::sub"),
        BinaryOp::Mul => row(
            "BinaryOp::Mul",
            Support::Lowered,
            "64-bit limb products, a1*b1 = 0 and c = a0b0 + 2^64(a0b1 + a1b0) with c range-proven: the 256-bit intermediate is exact",
            "runtime.rs:465-469; gadgets::mul",
        ),
        BinaryOp::Eq => row("BinaryOp::Eq", Support::Lowered, "UInt: inverse/flag pair; Bool: 1 - a - b + 2ab", "runtime.rs:470; gadgets::eq_uint, eq_bool"),
        BinaryOp::Ne => row("BinaryOp::Ne", Support::Lowered, "not(Eq)", "runtime.rs:471"),
        BinaryOp::Lt => row("BinaryOp::Lt", Support::Lowered, "f(2(a-b)+1) = (a-b) - t, f Boolean, t range-proven", "runtime.rs:472; gadgets::lt"),
        BinaryOp::Le => row("BinaryOp::Le", Support::Lowered, "not(b < a)", "runtime.rs:473"),
        BinaryOp::Gt => row("BinaryOp::Gt", Support::Lowered, "b < a", "runtime.rs:474"),
        BinaryOp::Ge => row("BinaryOp::Ge", Support::Lowered, "not(a < b)", "runtime.rs:475"),
        BinaryOp::And => row("BinaryOp::And", Support::Lowered, "strict: both operands lowered first, then a*b; no short-circuit", "runtime.rs:451-453, 476"),
        BinaryOp::Or => row("BinaryOp::Or", Support::Lowered, "strict: both operands lowered first, then a + b - ab; no short-circuit", "runtime.rs:451-453, 477"),
    }
}

pub fn statement_support(s: &Statement) -> Row {
    match s {
        Statement::Await { .. } => row(
            "Statement::Await",
            Support::PlannedNotLowered,
            "moriarty-core/2 only: /1 admission refuses it; /2 sources are admitted by stage::load and refused with a StagedLoweringPlan",
            "stage/profile.rs:10-23; runtime.rs:237-239; staged.rs",
        ),
        Statement::Let { .. } => row("Statement::Let", Support::Lowered, "binds the root wire; no effect", "runtime.rs:716-720"),
        Statement::Requires { .. } => row("Statement::Requires", Support::Lowered, "root wire = 1 at its position; constant false refuses at compile", "runtime.rs:721-734"),
        Statement::Ensures { .. } => row("Statement::Ensures", Support::Lowered, "root wire = 1 at its position (sees values current at that point)", "runtime.rs:721-734"),
        Statement::Next { .. } => row("Statement::Next", Support::Lowered, "State effect (before, after); replaces state and name binding", "runtime.rs:735-746"),
        Statement::Emit { .. } => row(
            "Statement::Emit",
            Support::Lowered,
            "Emit effect value slot (UInt or Bool). The initial ledger target refuses whole-program Emit as a target policy",
            "runtime.rs:747-754; adapter::initial_ledger_target_admits",
        ),
        Statement::Transfer { .. } => row(
            "Statement::Transfer",
            Support::Lowered,
            "checked debit then checked credit; self-transfer keeps the debit check and changes nothing",
            "runtime.rs:668-700, 755-786",
        ),
        Statement::Fee { .. } => row("Statement::Fee", Support::Lowered, "balance semantics of Transfer; distinct effect kind", "runtime.rs:755-786"),
        Statement::Liability { .. } => row(
            "Statement::Liability",
            Support::Lowered,
            "checked accrual into the source-registry cell; sets present (present-zero allowed); source-fixed identity only",
            "runtime.rs:787-836",
        ),
        Statement::Discharge { .. } => row(
            "Statement::Discharge",
            Support::Lowered,
            "present = 1 and checked subtraction; keeps present-zero. A name with no source Liability anywhere is refused at compile (external duty)",
            "runtime.rs:837-860; plan.rs external duty",
        ),
    }
}

/// Program, scenario and execution fields and how the relation treats each.
pub const FIELDS: &[Row] = &[
    row("Program.language / semantics", Support::Constant, "admission requires moriarty/1, moriarty-core/1; part of profile digest", "runtime.rs:120-126"),
    row("Program.name", Support::Constant, "inside the canonical Program hash", "intent.rs:72-88"),
    row("Program.states (name, ty, initial)", Support::Constant, "slot types and order; initial values are scenario defaults, pre-state is claimed", "runtime.rs:548-571"),
    row("Program.assets (name, identity{network, issuer, reference, scale})", Support::Constant, "cell identities in the shape digest; any scale admitted by Core is admitted (a target may refuse nonzero scale)", "runtime.rs:153-181"),
    row("Program.expressions", Support::Lowered, "every node reachable from each statement root, per-statement memo", "runtime.rs:392-484"),
    row("Program.actions (name, inputs, statements)", Support::Constant, "one relation per action; action index in the header", "plan.rs"),
    row("Scenario.state", Support::Lowered, "pre state slots, complete and typed", "runtime.rs:548-571"),
    row("Scenario.inputs", Support::Lowered, "input slots in declaration order, typed", "runtime.rs:572-585"),
    row("Scenario.balances", Support::Lowered, "represented cells; unrepresented rows are a frame-completeness target obligation", "runtime.rs:586-628"),
    row("Scenario.liabilities", Support::Lowered, "registry duty cells (present, amount); unregistered rows are a target obligation", "runtime.rs:629-661"),
    row("Execution.pre / post", Support::Lowered, "pre and post body slots", "runtime.rs:904-911"),
    row("Execution.effects", Support::Lowered, "effect slots in statement order; kinds and constant fields in the shape digest", "runtime.rs:895"),
    row("Execution.steps (index, statement, value, snapshot, effects)", Support::Internal, "every step value is a constrained internal wire; per-step snapshots are not published (determined by pre + inputs)", "runtime.rs:886-903"),
    row("Execution.semantics / action", Support::Constant, "profile digest and action index", "runtime.rs:905-906"),
    row("CostReport (logical, cap, used)", Support::Lowered, "W is a header constant; with PublicCap the circuit enforces W <= cap", "logical_cost.rs:33-55; stage/cost.rs:1-8"),
    row("Error.code / step (refusals)", Support::Internal, "no success assignment exists for a refused input; no rejection relation in this slice", "see REFUSALS"),
    row("Owner intent (check_effects limits)", Support::TargetObligation, "gross debit and fees per debit cell exposed when AccountingBinding::GrossDebitAndFees; limits are checked outside", "intent.rs:252-350"),
    row("Source sponsorship / DUST / service cost planes", Support::TargetObligation, "not in moriarty-reference-logical-cost/1; unmeasured", "source_sponsor.rs; policy.rs"),
];

/// Each Core refusal on the `/1` success path and how the success relation excludes it.
pub const REFUSALS: &[Row] = &[
    row("mori::core::profile / name / program / duplicate / type / expression / asset / account / liability (validate)", Support::AdmissionFact, "program not admitted; no relation", "runtime.rs:120-322"),
    row("mori::core::action (unknown action)", Support::RefusedAtCompile, "CompileError::UnknownAction", "runtime.rs:875-879"),
    row("mori::core::bound (scenario sizes, trace product)", Support::Constant, "conservative compile-time bound over the represented snapshot; extra rows are a target obligation", "runtime.rs:529-547"),
    row("mori::core::state / input (scenario typing)", Support::Lowered, "slot types fix the shape; encode refuses mistyped claims", "runtime.rs:548-585"),
    row("mori::core::balance (missing explicit balance)", Support::Constant, "every touched cell is represented", "runtime.rs:609-627"),
    row("mori::core::liability (scenario identity differs)", Support::Constant, "duty identities are registry constants", "runtime.rs:652-659"),
    row("mori::core::liability (conflicting source identity)", Support::RefusedAtCompile, "StaticFailure: every simulate refuses", "runtime.rs:500-527"),
    row("mori::core::overflow (Add, Mul, credit, accrual)", Support::Lowered, "range-proven result; unsatisfiable", "runtime.rs:455-469, 691-695, 809-814"),
    row("mori::core::underflow (Sub)", Support::Lowered, "range-proven difference; unsatisfiable", "runtime.rs:460-464"),
    row("mori::core::balance (insufficient, self-transfer included)", Support::Lowered, "checked debit; unsatisfiable", "runtime.rs:685-689"),
    row("mori::core::requires / ensures (false predicate)", Support::Lowered, "root wire = 1; unsatisfiable", "runtime.rs:721-734"),
    row("mori::core::liability (unknown residual / discharge exceeds)", Support::Lowered, "present = 1 and checked subtraction; unsatisfiable", "runtime.rs:837-850"),
    row("mori::core::meter (cap exceeded)", Support::Lowered, "W <= cap with PublicCap; unbound cap means unmetered simulate", "policy.rs:37-53"),
    row("mori::core::name / type (runtime)", Support::AdmissionFact, "impossible after validate", "runtime.rs:443-478"),
];

/// Every construct row, with representative IR values for the data-carrying variants.
pub fn table() -> Vec<Row> {
    let mut rows = vec![
        expr_support(&Expr::LiabilityPresent { name: String::new() }),
        expr_support(&Expr::LiabilityAmount { name: String::new() }),
        expr_support(&Expr::Literal { value: Value::UInt(mori_core::UInt(0)) }),
        expr_support(&Expr::Literal { value: Value::Bool(false) }),
        expr_support(&Expr::Name { name: String::new() }),
        expr_support(&Expr::Not { value: 0 }),
    ];
    for op in [
        BinaryOp::Add,
        BinaryOp::Sub,
        BinaryOp::Mul,
        BinaryOp::Eq,
        BinaryOp::Ne,
        BinaryOp::Lt,
        BinaryOp::Le,
        BinaryOp::Gt,
        BinaryOp::Ge,
        BinaryOp::And,
        BinaryOp::Or,
    ] {
        rows.push(binary_support(op));
    }
    let e = String::new();
    for s in [
        Statement::Await { label: e.clone(), binders: Vec::new(), guard: 0 },
        Statement::Let { name: e.clone(), value: 0 },
        Statement::Requires { predicate: 0 },
        Statement::Ensures { predicate: 0 },
        Statement::Next { state: e.clone(), value: 0 },
        Statement::Emit { name: e.clone(), value: 0 },
        Statement::Transfer { asset: e.clone(), from: e.clone(), to: e.clone(), amount: 0 },
        Statement::Fee { asset: e.clone(), from: e.clone(), to: e.clone(), amount: 0 },
        Statement::Liability { name: e.clone(), asset: e.clone(), debtor: e.clone(), creditor: e.clone(), amount: 0 },
        Statement::Discharge { name: e.clone(), amount: 0 },
    ] {
        rows.push(statement_support(&s));
    }
    rows.extend(FIELDS.iter().cloned());
    rows.extend(REFUSALS.iter().cloned());
    rows
}
