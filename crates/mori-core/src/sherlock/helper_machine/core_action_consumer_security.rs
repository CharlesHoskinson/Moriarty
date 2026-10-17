// Security-owned guard, configuration and staged error types for the generic
// Core scalar source -> Sherlock B consumer.
//
// Placement (Root only): sibling of the Sol-owned consumer, proposed as
//   crate::sherlock::helper_machine::core_action_consumer_security
// Root registers this module and the consumer module. This file contains no
// consumer body: it does not build the canonical program, call the bridge,
// admit, run, evaluate, simulate, or mutate any owner image. Sol wires the
// existing calls in the order fixed by SECURITY-CONTRACT.md, calling the
// guards below at the stated points.
//
// Trust scope: this is a trusted-host guard for an in-process caller that can
// already construct a parsed crate::Program, a BTreeMap of inputs and the
// operational Bounds/Layout/fuel/stack values. It bounds host preprocessing
// allocation before and around the existing projection and bridge. It is not
// a remote or untrusted-input sandbox, does not close general H1, imposes no
// maintainer or human approval on permissionless compilation, and creates no
// new B Outcome, compiler refusal, budget, counter or native authority. Every
// owner Tooling and HostViolation payload is carried unchanged.

use std::collections::BTreeMap;

use crate::sherlock::helper_machine::shared::owner_host::{observe_bound_image, HostObservation};
use crate::sherlock::helper_machine::shared::{
    Admitted, BoundsC, FinanceInput, GrammarDomain, HostViolation, LayoutSchemaC, MachineImage,
    RunIntC, RunSummary, Tooling,
};
use crate::sherlock::{Expr, Type as SherlockType};
use crate::sherlock_source::{ProjectionDiagnostic, ScalarActionMap};

/// Profile named by the caller. Only the B helper-microstep profile is
/// admitted by this consumer contract; the other two frozen profile names are
/// representable only so that naming one is an explicit, early rejection
/// rather than a silent substitution.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConsumerProfile {
    SherlockKA1,
    SherlockQuintA1,
    SherlockHelperMicrostep1,
}

/// Caller-stated finite host extents for preprocessing. There is deliberately
/// no Default and no profile that supplies these numbers: the caller states
/// every extent, and a normal envelope is a caller-specific operational
/// choice, not a policy default.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SourceHostBounds {
    pub max_source_actions: usize,
    pub max_source_expressions: usize,
    /// Statements summed over every action, because Core validation walks all.
    pub max_source_statements: usize,
    pub max_action_inputs: usize,
    pub max_supplied_inputs: usize,
    /// Unshared (per-edge) projection occurrences of the selected action,
    /// including synthetic scalar-lowering nodes, counted before
    /// prepare_scalar_action allocates them.
    pub max_projection_occurrences: usize,
    /// Generated A AST nodes (function body plus entry call and its literals),
    /// counted iteratively before source_input_bridge allocates the B tree.
    pub max_generated_ast_nodes: usize,
    /// Generated A AST nesting. The B CoreProgram drop is owner-iterative; the
    /// A Arc tree drop is not, so nesting is bounded explicitly. Native stack
    /// bytes are not measured or claimed by this count.
    pub max_ast_nesting: usize,
    pub max_input_name_bytes: usize,
    pub max_total_input_name_bytes: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HostMeasure {
    SourceActions,
    SourceExpressions,
    SourceStatements,
    ActionInputs,
    SuppliedInputs,
    ProjectionOccurrences,
    GeneratedAstNodes,
    AstNesting,
    InputNameBytes,
    TotalInputNameBytes,
}

/// New guard errors are HOST metadata only. They are never a B Outcome, never
/// a Core compiler refusal, and never replace an owner Tooling/HostViolation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HostGuardError {
    InvalidHostBounds { field: &'static str },
    ProfileRejected { supplied: ConsumerProfile },
    GrammarRejected { supplied: GrammarDomain },
    FinanceRejected { supplied: FinanceInput },
    Exceeded { measure: HostMeasure, required: Option<u128>, bound: u128 },
    CountOverflow { measure: HostMeasure },
    NoSuchAction { action_index: usize, action_count: usize },
    MissingInput { name: String },
    TypeMismatch { name: String, declared: crate::Type, supplied: crate::Type },
    ExtraInput { name: String },
    PreparationContract { detail: &'static str },
}

/// Explicit operational configuration. fuel and stack are the exact RunIntC
/// values handed to admit_run; Negative and NatAboveU128 keep their existing
/// owner dispositions and are not guard-rejected or clamped.
#[derive(Clone)]
pub struct ConsumerConfig {
    pub bounds: BoundsC,
    pub layout: LayoutSchemaC,
    pub fuel: RunIntC,
    pub stack: RunIntC,
    pub profile: ConsumerProfile,
    pub grammar: GrammarDomain,
    pub finance: FinanceInput,
    pub host: SourceHostBounds,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeclaredInput {
    pub name: String,
    pub ty: crate::Type,
}

/// What a passed preflight hands to Sol: the declared inputs in declared
/// order. Sol builds entry literals by looking each name up in the supplied
/// map in exactly this order; map iteration order is never used.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourcePreflight {
    pub declared_inputs: Vec<DeclaredInput>,
    pub projected_occurrences: u128,
}

/// Staged consumer error. Tooling and HostViolation payloads are the actual
/// owner values, unchanged. The stage tells the caller how far the real call
/// sequence got and, for run/observation failures, what the sink has seen.
pub enum ConsumerError {
    HostGuard(HostGuardError),
    /// The actual crate validation error, unchanged. This is the existing
    /// compiler's own refusal surface, not a new guard refusal.
    CoreValidation(crate::Error),
    /// The actual prepare_scalar_action diagnostic, unchanged. Per the
    /// adapter contract this is projection metadata, not a compiler refusal.
    SourceProjection(ProjectionDiagnostic),
    Admit(Tooling),
    AdmitRun(Tooling),
    /// Owner run returned a HostViolation after earlier events were already
    /// forwarded to the caller's sink. Those events are a partial prefix:
    /// no RunSummary and no observation exist for this call.
    Run {
        violation: HostViolation,
        events_forwarded: u128,
    },
    /// Run completed and the summary is real, but the bound observation was
    /// refused. The summary and the already-forwarded events remain valid.
    Observation {
        violation: HostViolation,
        summary: RunSummary,
        events_forwarded: u128,
    },
}

/// A completed real owner run. The admitted bindings and the image are owned
/// side by side (never a stored self-referential observation); observe()
/// borrows both for the duration of one owner-bound read-only observation.
pub struct CompletedRun {
    pub admitted: Admitted,
    pub image: MachineImage,
    pub summary: RunSummary,
    pub events_forwarded: u128,
}

impl CompletedRun {
    pub fn observe(&self) -> Result<HostObservation<'_>, ConsumerError> {
        observe_bound_image(&self.admitted, &self.image).map_err(|violation| {
            ConsumerError::Observation {
                violation,
                summary: RunSummary {
                    outcome: self.summary.outcome,
                    events: self.summary.events,
                    consumed_work: self.summary.consumed_work,
                    helper_work: self.summary.helper_work,
                    helper_visits: self.summary.helper_visits,
                    reference_cursor: self.summary.reference_cursor,
                },
                events_forwarded: self.events_forwarded,
            }
        })
    }
}

/// Checked event counter for Sol's forwarding sink. The consumer retains no
/// events itself; it forwards each real StepEventC to the caller's sink
/// exactly once, in run order. Retention is therefore the caller's own
/// choice and allocation, outside this module. One fresh counter is used
/// for one owner run. The fixed owner `run` performs `checked_add(1)` on
/// its own event count before invoking the sink, so it returns its
/// unchanged HostViolation before forwarding an event that would overflow
/// either count; sink invocations in one run therefore cannot exceed
/// u128::MAX and `overflowed` cannot be set on that path. `overflowed`
/// remains a defensive record only if this counter were reused outside
/// that contract; it is not surfaced as a lower-bound count.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct EventForwarding {
    pub forwarded: u128,
    pub overflowed: bool,
}

impl EventForwarding {
    pub fn note(&mut self) {
        match self.forwarded.checked_add(1) {
            Some(next) => self.forwarded = next,
            None => self.overflowed = true,
        }
    }
}

fn exceeded(measure: HostMeasure, required: Option<u128>, bound: usize) -> HostGuardError {
    HostGuardError::Exceeded { measure, required, bound: bound as u128 }
}

fn within(measure: HostMeasure, required: u128, bound: usize) -> Result<u128, HostGuardError> {
    if required <= bound as u128 {
        Ok(required)
    } else {
        Err(exceeded(measure, Some(required), bound))
    }
}

fn count(value: usize) -> u128 {
    value as u128
}

/// Guard step 0: configuration shape, before the source is touched.
/// Precedence inside this step: host-bound fields in declaration order, then
/// profile, then grammar, then finance. Bounds/Layout/fuel/stack values are
/// not judged here; the owner admit/admit_run lanes keep that authority,
/// including LayoutBoundsMismatch, CapacityNotAdmitted, the genesis-invalid
/// Negative disposition and the NatAboveU128 refusals.
pub fn validate_config(config: &ConsumerConfig) -> Result<(), HostGuardError> {
    let host = &config.host;
    let fields: [(&'static str, usize); 10] = [
        ("max_source_actions", host.max_source_actions),
        ("max_source_expressions", host.max_source_expressions),
        ("max_source_statements", host.max_source_statements),
        ("max_action_inputs", host.max_action_inputs),
        ("max_supplied_inputs", host.max_supplied_inputs),
        ("max_projection_occurrences", host.max_projection_occurrences),
        ("max_generated_ast_nodes", host.max_generated_ast_nodes),
        ("max_ast_nesting", host.max_ast_nesting),
        ("max_input_name_bytes", host.max_input_name_bytes),
        ("max_total_input_name_bytes", host.max_total_input_name_bytes),
    ];
    for (field, value) in fields {
        if value == 0 {
            return Err(HostGuardError::InvalidHostBounds { field });
        }
    }
    if config.profile != ConsumerProfile::SherlockHelperMicrostep1 {
        return Err(HostGuardError::ProfileRejected { supplied: config.profile });
    }
    if config.grammar != GrammarDomain::DedicatedSorts {
        return Err(HostGuardError::GrammarRejected { supplied: config.grammar });
    }
    if config.finance != FinanceInput::Absent {
        return Err(HostGuardError::FinanceRejected { supplied: config.finance });
    }
    Ok(())
}

/// Guard steps 1-4, all before prepare_scalar_action allocates anything:
/// 1. O(1) source-shape counts against SourceHostBounds (actions, flat
///    expressions, summed statements), then action existence.
/// 2. Exact input policy against the action's DECLARED inputs, in this
///    precedence: supplied/declared count extents, then MissingInput in
///    declared order, then TypeMismatch in declared order, then ExtraInput
///    in the supplied map's ascending key order. A scalar literal's type is
///    read from its discriminant only; nothing is cloned or evaluated.
/// 3. The existing crate::validate, its error carried unchanged.
/// 4. An iterative forward pass over the flat expression DAG (validation
///    guarantees children precede parents) computing unshared occurrence
///    sizes with checked arithmetic; the selected action's lets-plus-emit
///    total must fit max_projection_occurrences before the adapter expands
///    the same DAG per edge. Overflow of the count itself is CountOverflow,
///    never saturation-as-success.
/// The sizes workspace is one u128 per flat expression; its length was
/// already bounded in step 1 by max_source_expressions.
pub fn preflight_source_global(
    program: &crate::Program,
    config: &ConsumerConfig,
) -> Result<(), ConsumerError> {
    validate_config(config).map_err(ConsumerError::HostGuard)?;
    let host = &config.host;
    within(HostMeasure::SourceActions, count(program.actions.len()), host.max_source_actions)
        .map_err(ConsumerError::HostGuard)?;
    within(
        HostMeasure::SourceExpressions,
        count(program.expressions.len()),
        host.max_source_expressions,
    )
    .map_err(ConsumerError::HostGuard)?;
    let mut statements: u128 = 0;
    for action in &program.actions {
        statements = statements
            .checked_add(count(action.statements.len()))
            .ok_or(ConsumerError::HostGuard(HostGuardError::CountOverflow {
                measure: HostMeasure::SourceStatements,
            }))?;
    }
    within(HostMeasure::SourceStatements, statements, host.max_source_statements)
        .map_err(ConsumerError::HostGuard)?;
    Ok(())
}

pub fn preflight_action_inputs<'a>(
    program: &'a crate::Program,
    action_index: usize,
    supplied: &BTreeMap<String, crate::Value>,
    config: &ConsumerConfig,
) -> Result<&'a crate::Action, ConsumerError> {
    validate_config(config).map_err(ConsumerError::HostGuard)?;
    let host = &config.host;
    let action = program.actions.get(action_index).ok_or(ConsumerError::HostGuard(
        HostGuardError::NoSuchAction { action_index, action_count: program.actions.len() },
    ))?;
    within(HostMeasure::ActionInputs, count(action.inputs.len()), host.max_action_inputs)
        .map_err(ConsumerError::HostGuard)?;
    within(HostMeasure::SuppliedInputs, count(supplied.len()), host.max_supplied_inputs)
        .map_err(ConsumerError::HostGuard)?;
    // Name-byte extents over declared and supplied names, allocation-free.
    let mut total_name_bytes: u128 = 0;
    for input in &action.inputs {
        let bytes = count(input.name.len());
        within(HostMeasure::InputNameBytes, bytes, host.max_input_name_bytes)
            .map_err(ConsumerError::HostGuard)?;
        total_name_bytes = total_name_bytes.checked_add(bytes).ok_or(ConsumerError::HostGuard(
            HostGuardError::CountOverflow { measure: HostMeasure::TotalInputNameBytes },
        ))?;
    }
    for name in supplied.keys() {
        let bytes = count(name.len());
        within(HostMeasure::InputNameBytes, bytes, host.max_input_name_bytes)
            .map_err(ConsumerError::HostGuard)?;
        total_name_bytes = total_name_bytes.checked_add(bytes).ok_or(ConsumerError::HostGuard(
            HostGuardError::CountOverflow { measure: HostMeasure::TotalInputNameBytes },
        ))?;
    }
    within(HostMeasure::TotalInputNameBytes, total_name_bytes, host.max_total_input_name_bytes)
        .map_err(ConsumerError::HostGuard)?;
    Ok(action)
}

pub fn preflight_source(
    program: &crate::Program,
    action_index: usize,
    supplied: &BTreeMap<String, crate::Value>,
    config: &ConsumerConfig,
) -> Result<SourcePreflight, ConsumerError> {
    preflight_source_global(program, config)?;
    let action = preflight_action_inputs(program, action_index, supplied, config)?;
    let host = &config.host;
    // Exact input policy: no missing, no type mismatch, no extra. A complete
    // MissingInput pass in declared order precedes a complete TypeMismatch
    // pass in declared order; nothing is defaulted or coerced.
    for input in &action.inputs {
        if !supplied.contains_key(&input.name) {
            return Err(ConsumerError::HostGuard(HostGuardError::MissingInput {
                name: input.name.clone(),
            }));
        }
    }
    for input in &action.inputs {
        if let Some(value) = supplied.get(&input.name) {
            if value.ty() != input.ty {
                return Err(ConsumerError::HostGuard(HostGuardError::TypeMismatch {
                    name: input.name.clone(),
                    declared: input.ty,
                    supplied: value.ty(),
                }));
            }
        }
    }
    for name in supplied.keys() {
        if !action.inputs.iter().any(|input| &input.name == name) {
            return Err(ConsumerError::HostGuard(HostGuardError::ExtraInput {
                name: name.clone(),
            }));
        }
    }
    crate::validate(program).map_err(ConsumerError::CoreValidation)?;
    // Unshared occurrence precount over the validated flat DAG, including
    // synthetic scalar-lowering occurrences per operator.
    let mut sizes: Vec<u128> = Vec::with_capacity(program.expressions.len());
    for expression in &program.expressions {
        match expression {
            crate::Expr::Binary { op, lhs, rhs } => {
                let pair = [*lhs, *rhs];
                sizes.push(dag_size(binary_occurrence_overhead(*op), &sizes, &pair)?);
            }
            crate::Expr::Not { value } => {
                let single = [*value];
                sizes.push(dag_size(1, &sizes, &single)?);
            }
            crate::Expr::Literal { .. }
            | crate::Expr::Name { .. }
            | crate::Expr::LiabilityPresent { .. }
            | crate::Expr::LiabilityAmount { .. } => {
                sizes.push(1);
            }
        }
    }
    let mut occurrences: u128 = 0;
    for statement in &action.statements {
        let (value, binder): (crate::ExprId, u128) = match statement {
            crate::Statement::Let { value, .. } => (*value, 1),
            crate::Statement::Emit { value, .. } => (*value, 0),
            _ => continue,
        };
        let size = sizes.get(usize::from(value)).copied().ok_or(ConsumerError::HostGuard(
            HostGuardError::PreparationContract { detail: "statement-expression-order" },
        ))?;
        occurrences = occurrences
            .checked_add(binder)
            .and_then(|sum| sum.checked_add(size))
            .ok_or(ConsumerError::HostGuard(HostGuardError::CountOverflow {
                measure: HostMeasure::ProjectionOccurrences,
            }))?;
    }
    within(
        HostMeasure::ProjectionOccurrences,
        occurrences,
        host.max_projection_occurrences,
    )
    .map_err(ConsumerError::HostGuard)?;
    Ok(SourcePreflight {
        declared_inputs: action
            .inputs
            .iter()
            .map(|input| DeclaredInput { name: input.name.clone(), ty: input.ty })
            .collect(),
        projected_occurrences: occurrences,
    })
}

fn binary_occurrence_overhead(op: crate::BinaryOp) -> u128 {
    match op {
        crate::BinaryOp::Add
        | crate::BinaryOp::Sub
        | crate::BinaryOp::Mul
        | crate::BinaryOp::Eq
        | crate::BinaryOp::Lt => 1,
        crate::BinaryOp::Ne | crate::BinaryOp::Ge => 2,
        crate::BinaryOp::Gt => 5,
        crate::BinaryOp::Le | crate::BinaryOp::And | crate::BinaryOp::Or => 6,
    }
}

fn dag_size(own: u128, sizes: &[u128], children: &[crate::ExprId]) -> Result<u128, ConsumerError> {
    let mut size: u128 = own;
    for child in children {
        let child_size = sizes.get(usize::from(*child)).copied().ok_or(ConsumerError::HostGuard(
            HostGuardError::PreparationContract { detail: "expression-order" },
        ))?;
        size = size.checked_add(child_size).ok_or(ConsumerError::HostGuard(
            HostGuardError::CountOverflow { measure: HostMeasure::ProjectionOccurrences },
        ))?;
    }
    Ok(size)
}

fn scalar_sherlock(ty: crate::Type) -> Option<SherlockType> {
    match ty {
        crate::Type::UInt => Some(SherlockType::UInt),
        crate::Type::Bool => Some(SherlockType::Bool),
    }
}

/// Guard step 5, after prepare_scalar_action and before Sol builds the
/// canonical A program or calls source_input_bridge: the adapter's actual
/// output must match the preflight's declared shape, and the generated body
/// must fit the node/nesting extents. Checks, in order: generated function
/// name equals the report name; parameter count, order, names and scalar
/// types equal report.inputs (which preflight already bound to the action's
/// declared order); return type is the report's scalar return type; the
/// emit role is exactly the fixed value-projection role; the report's
/// materialized occurrence count does not exceed the precount; then an
/// explicit-stack count of body nodes and nesting against SourceHostBounds.
/// The traversal stack holds at most one entry per counted node, so its size
/// is itself bounded by max_generated_ast_nodes before the bridge runs.
pub fn check_generated(
    mapped: &ScalarActionMap,
    preflight: &SourcePreflight,
    host: &SourceHostBounds,
) -> Result<(usize, usize), ConsumerError> {
    let contract = |detail: &'static str| {
        ConsumerError::HostGuard(HostGuardError::PreparationContract { detail })
    };
    let function = &mapped.function;
    let report = &mapped.report;
    if function.name != report.generated_function_name || report.action.is_empty() {
        return Err(contract("generated-function-name"));
    }
    if report.emit.role != "value-projection; ordered emit effect remains in Core" {
        return Err(contract("emit-role"));
    }
    if count(report.occurrences.len()) > preflight.projected_occurrences {
        return Err(contract("occurrence-precount"));
    }
    if function.parameters.len() != preflight.declared_inputs.len()
        || report.inputs.len() != preflight.declared_inputs.len()
    {
        return Err(contract("generated-signature"));
    }
    for (parameter, declared) in function.parameters.iter().zip(&preflight.declared_inputs) {
        if parameter.name != declared.name || scalar_sherlock(declared.ty) != Some(parameter.ty) {
            return Err(contract("generated-parameter"));
        }
    }
    for (site, declared) in report.inputs.iter().zip(&preflight.declared_inputs) {
        if site.name != declared.name || site.ty != declared.ty {
            return Err(contract("report-input"));
        }
    }
    if scalar_sherlock(report.return_type) != Some(function.return_type) {
        return Err(contract("generated-return-type"));
    }
    count_ast(&function.body, host)
}

/// Iterative node/nesting count of one generated A expression tree. Sol
/// does not add body and entry counts itself: it passes the entry Call it
/// constructs (one Call node plus one Literal per declared input, in
/// declared order) to `check_combined_generated_ast_nodes`, which calls
/// this function on the entry and performs the checked combined-total
/// enforcement before the bridge. A Literal list/tree value cannot occur
/// here: entry literals are built only from the scalar supplied inputs.
pub fn count_ast(root: &crate::sherlock::ExprRef, host: &SourceHostBounds) -> Result<(usize, usize), ConsumerError> {
    let mut pending: Vec<(&Expr, usize)> = vec![(root.as_ref(), 1)];
    let mut nodes: usize = 0;
    let mut nesting: usize = 0;
    while let Some((expr, depth)) = pending.pop() {
        nodes = nodes.checked_add(1).ok_or(ConsumerError::HostGuard(
            HostGuardError::CountOverflow { measure: HostMeasure::GeneratedAstNodes },
        ))?;
        if nodes > host.max_generated_ast_nodes {
            return Err(ConsumerError::HostGuard(exceeded(
                HostMeasure::GeneratedAstNodes,
                Some(count(nodes)),
                host.max_generated_ast_nodes,
            )));
        }
        if depth > nesting {
            nesting = depth;
        }
        if depth > host.max_ast_nesting {
            return Err(ConsumerError::HostGuard(exceeded(
                HostMeasure::AstNesting,
                Some(count(depth)),
                host.max_ast_nesting,
            )));
        }
        match expr {
            Expr::Literal(_) | Expr::Var(_) => {}
            Expr::Let(_, value, body) => {
                pending.push((body.as_ref(), depth + 1));
                pending.push((value.as_ref(), depth + 1));
            }
            Expr::Binary(_, left, right) => {
                pending.push((right.as_ref(), depth + 1));
                pending.push((left.as_ref(), depth + 1));
            }
            Expr::Unary(_, operand) => pending.push((operand.as_ref(), depth + 1)),
            Expr::If(cond, then_expr, else_expr) => {
                pending.push((else_expr.as_ref(), depth + 1));
                pending.push((then_expr.as_ref(), depth + 1));
                pending.push((cond.as_ref(), depth + 1));
            }
            Expr::Call(_, args) => {
                for arg in args.iter().rev() {
                    pending.push((arg.as_ref(), depth + 1));
                }
            }
            Expr::Match(scrutinee, arms) => {
                for arm in arms.iter().rev() {
                    pending.push((arm.body.as_ref(), depth + 1));
                }
                pending.push((scrutinee.as_ref(), depth + 1));
            }
        }
    }
    Ok((nodes, nesting))
}

/// Guard step 6 (combined generated-AST enforcement), security-owned: given
/// the body node count returned by `check_generated`, count the constructed
/// entry expression with `count_ast`, checked-add body and entry node
/// counts, and enforce `host.max_generated_ast_nodes` on the combined total
/// before `source_input_bridge` allocates the B tree. The entry nesting
/// returned by `count_ast` is enforced against `host.max_ast_nesting`
/// inside `count_ast` for the entry tree itself; nesting is not added
/// across the separate body and entry trees. On success returns the
/// combined generated node total. Sol calls this helper only; it performs
/// no checked-add or bound comparison for this total itself.
pub fn check_combined_generated_ast_nodes(
    body_nodes: usize,
    entry: &crate::sherlock::ExprRef,
    host: &SourceHostBounds,
) -> Result<usize, ConsumerError> {
    let (entry_nodes, _) = count_ast(entry, host)?;
    let total = body_nodes.checked_add(entry_nodes).ok_or(ConsumerError::HostGuard(
        HostGuardError::CountOverflow { measure: HostMeasure::GeneratedAstNodes },
    ))?;
    if total > host.max_generated_ast_nodes {
        return Err(ConsumerError::HostGuard(exceeded(
            HostMeasure::GeneratedAstNodes,
            Some(count(total)),
            host.max_generated_ast_nodes,
        )));
    }
    Ok(total)
}
