//! SOURCE_ONLY / UNRUN: bounded host consumer of the existing scalar Core/1 projection.
//!
//! This consumer adds no compiler, evaluator, profile or machine outcome. It
//! calls the real `crate::validate`, `prepare_scalar_action` and
//! `Machine::run_steps`, then retains their actual typed results. It invokes the
//! single generated function once with a fixed literal scalar entry.
//!
//! Every [`HostObservationRefusal`] is host metadata, not a Core compiler error,
//! a narrowing of the Core subset or a `sherlock-k-a/1` result. Resource checks
//! all run before preparation allocates occurrences and before any machine is
//! built. The caller states every bound; no numeric default or capacity
//! profile is supplied here.
//!
//! The Machine's H1 warning is unchanged. `Machine::run` is never called. The
//! bounds below are finite operation, slot and nesting counts derived from the
//! current adapter and machine source. They are not wall-clock, byte or native
//! stack measurements. Whether a caller's chosen counts satisfy H1 for its own
//! thread stack and deployment is a separate review decision.
use crate::sherlock::{
    BinaryOp, Expr, ExprRef, Machine, Outcome, Program, Snapshot, StepRecord, Type, UnaryOp, Value,
};
use crate::sherlock_source::{ProjectionDiagnostic, ScalarActionReport, prepare_scalar_action};
use std::sync::Arc;

pub const QUALIFICATION: &str = "bounded-host-observation; stateless scalar Core/1 projection; operation, slot and nesting counts only";

/// Caller-stated finite extents. There is deliberately no `Default`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HostObservationBounds {
    /// Borrowed source shape, checked before Core validation walks it.
    pub max_source_actions: usize,
    pub max_source_expressions: usize,
    /// Statements summed over every action; Core validation types each one.
    pub max_source_statements: usize,
    pub max_scalar_arguments: usize,
    /// Unshared occurrences, including the adapter's synthetic lowering nodes,
    /// checked before the adapter's own fixed budget.
    pub max_projection_occurrences: usize,
    /// Generated body nesting. This bounds derived Drop/Clone/PartialEq/Debug
    /// recursion over the generated AST; its native stack use is unmeasured.
    pub max_ast_nesting: usize,
    /// The A frame cap `L`, passed unchanged to `Machine::new`. Reaching it is
    /// the existing A `Rejected(4)` outcome, never a host refusal.
    pub stack_limit: usize,
    /// Retained step records. Admission requires `initial_fuel + 1 <= max_step_records`.
    pub max_step_records: usize,
    /// Ceiling on [`HostExtent::peak_slot_bound`].
    pub max_retained_slots: u128,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HostMeasure {
    SourceActions,
    SourceExpressions,
    SourceStatements,
    ScalarArguments,
    ProjectionOccurrences,
    AstNesting,
    StepRecords,
    RetainedSlots,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HostObservationRefusal {
    /// `required: None` means the checked count overflowed `u128`.
    Exceeded {
        measure: HostMeasure,
        required: Option<u128>,
        bound: u128,
    },
    /// The actual Core validation error, unchanged.
    CoreValidation(crate::Error),
    NoSuchAction {
        action_index: usize,
        action_count: usize,
    },
    /// Lists and trees are outside the scalar domain. Only the discriminant is read.
    NonScalarArgument { index: usize },
    /// The actual adapter diagnostic, unchanged; it is not a compiler refusal.
    Projection(ProjectionDiagnostic),
    /// Actual adapter output fell outside this consumer's admitted shape.
    PreparationContract { detail: &'static str },
    /// Defensive: admitted fuel guarantees publication within the admitted records.
    UnpublishedOutcome,
}

/// Admission counts for one observation. The slot unit is one scalar value,
/// environment entry, parameter or expression reference. Environment keys are
/// Core identifiers of at most 64 bytes; no byte total is claimed. The shared
/// generated program and report are bounded by occurrences and nesting instead.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HostExtent {
    pub bounds: HostObservationBounds,
    /// Lets plus unshared generated nodes over every let and emit value,
    /// including synthetic lowering nodes.
    pub projected_occurrences: u128,
    pub ast_nesting_bound: u128,
    /// Measured iteratively on the actual generated body.
    pub observed_ast_nesting: usize,
    /// `initial_fuel + 1`: one per charged step plus one free publication.
    pub step_record_bound: u128,
    /// `min(stack_limit, initial_fuel)`: a step pushes at most one frame.
    pub frame_bound: u128,
    /// Arity plus lets plus two synthetic binders per lowered Gt, Le, And or Or
    /// occurrence: the distinct names any machine environment can hold.
    pub environment_entry_bound: u128,
    /// `(2 * initial_fuel + 6) * (frame_bound + 3) * (entries + other)`, where
    /// `other` is `2 * arity + 1`, raised to 2 when a lowered If is generated.
    pub peak_slot_bound: u128,
    pub argument_types_match_inputs: bool,
}

/// The actual adapter report, generated program and machine records.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoundedSourceObservation {
    pub report: ScalarActionReport,
    /// The same allocation every retained snapshot references.
    pub program: Arc<Program>,
    pub initial: Snapshot,
    pub records: Vec<StepRecord>,
    pub terminal: Snapshot,
    /// Actual post-publication step probe; separate from the terminal snapshot.
    pub terminal_has_next: bool,
    pub extent: HostExtent,
    pub qualification: &'static str,
}

/// Observe one stateless scalar action under caller-stated finite bounds.
///
/// Order: bounded source shape and argument checks, Core validation, a forward
/// count pass, every resource bound, then preparation, shape guards and the
/// machine. No preparation or machine work happens for a refused bound.
///
/// Arguments are borrowed so a refused list or tree is never cloned, compared
/// or dropped here. Arity and scalar type mismatches are admitted and produce
/// the existing A outcome; [`HostExtent::argument_types_match_inputs`] records them.
pub fn observe_scalar_action(
    program: &crate::Program,
    action_index: usize,
    arguments: &[Value],
    initial_fuel: u128,
    bounds: HostObservationBounds,
) -> Result<BoundedSourceObservation, HostObservationRefusal> {
    within(
        HostMeasure::SourceActions,
        Some(count(program.actions.len())),
        count(bounds.max_source_actions),
    )?;
    within(
        HostMeasure::SourceExpressions,
        Some(count(program.expressions.len())),
        count(bounds.max_source_expressions),
    )?;
    let statements = program.actions.iter().try_fold(0u128, |sum, action| {
        sum.checked_add(count(action.statements.len()))
    });
    within(
        HostMeasure::SourceStatements,
        statements,
        count(bounds.max_source_statements),
    )?;
    within(
        HostMeasure::ScalarArguments,
        Some(count(arguments.len())),
        count(bounds.max_scalar_arguments),
    )?;
    if let Some(index) = arguments
        .iter()
        .position(|value| !matches!(value, Value::UInt(_) | Value::Bool(_)))
    {
        return Err(HostObservationRefusal::NonScalarArgument { index });
    }
    let action = program
        .actions
        .get(action_index)
        .ok_or(HostObservationRefusal::NoSuchAction {
            action_index,
            action_count: program.actions.len(),
        })?;
    crate::validate(program).map_err(HostObservationRefusal::CoreValidation)?;

    // Validation orders every child before its parent, so one forward pass gives
    // each expression's exact unshared generated extent under the adapter's
    // lowering table, without recursion: nodes, nesting, synthetic binders and
    // lowered conditionals. Saturated counts exceed every usize bound and are
    // refused below.
    let mut extents: Vec<Generated> = Vec::with_capacity(program.expressions.len());
    for expression in &program.expressions {
        let (children, fanout, form): ([crate::ExprId; 2], usize, Lowering) = match expression {
            crate::Expr::Binary { op, lhs, rhs } => ([*lhs, *rhs], 2, lowering(*op)),
            crate::Expr::Not { value } => ([*value, 0], 1, DIRECT),
            _ => ([0, 0], 0, DIRECT),
        };
        let (nodes, offsets, nesting, binders, conditionals) = form;
        let mut extent = Generated {
            nodes,
            nesting,
            binders,
            conditionals,
        };
        for (child, offset) in children[..fanout].iter().zip(offsets) {
            let Some(child) = extents.get(usize::from(*child)) else {
                return Err(contract("expression-order"));
            };
            extent.nodes = extent.nodes.saturating_add(child.nodes);
            extent.nesting = extent.nesting.max(offset.saturating_add(child.nesting));
            extent.binders = extent.binders.saturating_add(child.binders);
            extent.conditionals = extent.conditionals.saturating_add(child.conditionals);
        }
        extents.push(extent);
    }
    // Conservative over the adapter: unsupported statements stop preparation
    // early. A prepared action is exactly its lets and final emit, counted here.
    let (mut lets, mut occurrences, mut value_depth) = (0u128, 0u128, 0u128);
    let (mut binders, mut conditionals) = (0u128, 0u128);
    for statement in &action.statements {
        let (value, binder) = match statement {
            crate::Statement::Let { value, .. } => (*value, 1),
            crate::Statement::Emit { value, .. } => (*value, 0),
            _ => continue,
        };
        let Some(extent) = extents.get(usize::from(value)) else {
            return Err(contract("statement-expression"));
        };
        lets += binder;
        occurrences = occurrences
            .saturating_add(binder)
            .saturating_add(extent.nodes);
        value_depth = value_depth.max(extent.nesting);
        binders = binders.saturating_add(extent.binders);
        conditionals = conditionals.saturating_add(extent.conditionals);
    }
    within(
        HostMeasure::ProjectionOccurrences,
        Some(occurrences),
        count(bounds.max_projection_occurrences),
    )?;
    // Let i sits at depth i; its deepest value node at i + depth(value).
    let ast_nesting_bound = lets.saturating_add(value_depth);
    within(
        HostMeasure::AstNesting,
        Some(ast_nesting_bound),
        count(bounds.max_ast_nesting),
    )?;
    let step_record_bound = within(
        HostMeasure::StepRecords,
        initial_fuel.checked_add(1),
        count(bounds.max_step_records),
    )?;
    let events = usize::try_from(step_record_bound).map_err(|_| contract("step-record-width"))?;
    let arity = count(action.inputs.len()).max(count(arguments.len()));
    let frame_bound = count(bounds.stack_limit).min(initial_fuel);
    // Synthetic binders join the environment; a lowered If frame holds two branches.
    let environment_entry_bound = arity.saturating_add(lets).saturating_add(binders);
    let peak_slot_bound = within(
        HostMeasure::RetainedSlots,
        peak_slots(
            initial_fuel,
            frame_bound,
            environment_entry_bound,
            arity,
            conditionals > 0,
        ),
        bounds.max_retained_slots,
    )?;

    let mapped =
        prepare_scalar_action(program, action_index).map_err(HostObservationRefusal::Projection)?;
    if count(mapped.report.occurrences.len()) != occurrences {
        return Err(contract("occurrence-precount"));
    }
    let function = mapped.function;
    if function.parameters.len() != action.inputs.len()
        || !function
            .parameters
            .iter()
            .all(|parameter| scalar(parameter.ty))
        || !scalar(function.return_type)
    {
        return Err(contract("generated-signature"));
    }
    // Each occurrence is exactly one fresh generated body node. Every Let binder
    // and If frame the machine can hold was counted above for environment and slots.
    let (nodes, observed_ast_nesting, let_nodes, if_nodes) =
        admitted_body(&function.body, mapped.report.occurrences.len()).map_err(contract)?;
    if nodes != mapped.report.occurrences.len() || count(observed_ast_nesting) > ast_nesting_bound {
        return Err(contract("generated-body-extent"));
    }
    if count(let_nodes) != lets.saturating_add(binders) || count(if_nodes) != conditionals {
        return Err(contract("generated-binders"));
    }
    let argument_types_match_inputs = arguments.len() == action.inputs.len()
        && arguments.iter().zip(&action.inputs).all(|(value, input)| {
            matches!(
                (value, input.ty),
                (Value::UInt(_), crate::Type::UInt) | (Value::Bool(_), crate::Type::Bool)
            )
        });

    // The fixed diagnostic invocation of the returned real function.
    let entry = Arc::new(Expr::Call(
        function.name.clone(),
        arguments
            .iter()
            .map(|value| Arc::new(Expr::Literal(value.clone())))
            .collect(),
    ));
    let generated = Arc::new(Program {
        functions: vec![function],
        entry,
    });
    let mut machine = Machine::new(generated.clone(), initial_fuel, bounds.stack_limit);
    let initial = machine.snapshot();
    let records = machine.run_steps(events);
    let terminal = machine.snapshot();
    if matches!(terminal.outcome, Outcome::Pending) {
        return Err(HostObservationRefusal::UnpublishedOutcome);
    }
    let terminal_has_next = machine.step().is_some();
    Ok(BoundedSourceObservation {
        report: mapped.report,
        program: generated,
        initial,
        records,
        terminal,
        terminal_has_next,
        extent: HostExtent {
            bounds,
            projected_occurrences: occurrences,
            ast_nesting_bound,
            observed_ast_nesting,
            step_record_bound,
            frame_bound,
            environment_entry_bound,
            peak_slot_bound,
            argument_types_match_inputs,
        },
        qualification: QUALIFICATION,
    })
}

fn count(value: usize) -> u128 {
    value as u128
}

fn contract(detail: &'static str) -> HostObservationRefusal {
    HostObservationRefusal::PreparationContract { detail }
}

fn within(
    measure: HostMeasure,
    required: Option<u128>,
    bound: u128,
) -> Result<u128, HostObservationRefusal> {
    match required {
        Some(value) if value <= bound => Ok(value),
        required => Err(HostObservationRefusal::Exceeded {
            measure,
            required,
            bound,
        }),
    }
}

fn scalar(ty: Type) -> bool {
    matches!(ty, Type::UInt | Type::Bool)
}

/// One expression's unshared generated extent.
#[derive(Clone, Copy)]
struct Generated {
    nodes: u128,
    nesting: u128,
    binders: u128,
    conditionals: u128,
}

/// Own nodes, operand nesting offsets, own nesting, synthetic binders, Ifs.
type Lowering = (u128, [u128; 2], u128, u128, u128);

/// Literal, Name, Not and the five direct binary primitives: one node over its operands.
const DIRECT: Lowering = (1, [1, 1], 1, 0, 0);

/// The adapter's lowering table, with L and R the mapped operands at the given offsets.
fn lowering(op: crate::BinaryOp) -> Lowering {
    use crate::BinaryOp as Op;
    match op {
        Op::Add | Op::Sub | Op::Mul | Op::Eq | Op::Lt => DIRECT,
        // Unary(Not, Binary(Eq | Lt, L, R)).
        Op::Ne | Op::Ge => (2, [2, 2], 2, 0, 0),
        // Let(l, L, Let(r, R, Binary(Lt, Var(r), Var(l)))).
        Op::Gt => (5, [1, 2], 4, 2, 0),
        // Let(l, L, Let(r, R, Unary(Not, Binary(Lt, Var(r), Var(l))))).
        Op::Le => (6, [1, 2], 5, 2, 0),
        // Let(l, L, Let(r, R, If(Var(l), Var(r) | true, false | Var(r)))).
        Op::And | Op::Or => (6, [1, 2], 4, 2, 1),
    }
}

/// Snapshot units: initial, terminal, two per record, the live machine state
/// and one transient control/frame copy inside a step. Per snapshot: its
/// environment, a control holding at most a frame plus an environment, and
/// each frame holding at most one environment and either arity values, arity
/// pending arguments and one reference, or a lowered If's two branch
/// references. Lists, trees and match arms are unreachable here.
fn peak_slots(
    fuel: u128,
    frames: u128,
    entries: u128,
    arity: u128,
    branches: bool,
) -> Option<u128> {
    let mut other = arity.checked_mul(2)?.checked_add(1)?;
    if branches {
        other = other.max(2);
    }
    let per_unit = entries.checked_add(other)?;
    let per_snapshot = frames.checked_add(3)?.checked_mul(per_unit)?;
    fuel.checked_mul(2)?
        .checked_add(6)?
        .checked_mul(per_snapshot)
}

/// Explicit-stack walk admitting only the scalar forms the adapter emits,
/// including its Let, Not, Eq/Lt and If lowerings. Returns the node count,
/// nesting, Let count and If count, stopping once `node_limit` is passed.
fn admitted_body(
    body: &ExprRef,
    node_limit: usize,
) -> Result<(usize, usize, usize, usize), &'static str> {
    let mut pending = vec![(body, 1usize)];
    let (mut nodes, mut nesting, mut lets, mut conditionals) = (0usize, 0usize, 0usize, 0usize);
    while let Some((expr, depth)) = pending.pop() {
        nodes += 1;
        if nodes > node_limit {
            return Err("generated-node-count");
        }
        nesting = nesting.max(depth);
        match &**expr {
            Expr::Literal(Value::UInt(_) | Value::Bool(_)) | Expr::Var(_) => {}
            Expr::Let(_, value, body) => {
                lets += 1;
                pending.push((value, depth + 1));
                pending.push((body, depth + 1));
            }
            Expr::Binary(
                BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Eq | BinaryOp::Lt,
                left,
                right,
            ) => {
                pending.push((left, depth + 1));
                pending.push((right, depth + 1));
            }
            Expr::Unary(UnaryOp::Not, value) => pending.push((value, depth + 1)),
            Expr::If(condition, yes, no) => {
                conditionals += 1;
                pending.push((condition, depth + 1));
                pending.push((yes, depth + 1));
                pending.push((no, depth + 1));
            }
            _ => return Err("generated-node-form"),
        }
    }
    Ok((nodes, nesting, lets, conditionals))
}

#[cfg(test)]
#[path = "bounded_scalar_source_tests.rs"]
mod tests;
