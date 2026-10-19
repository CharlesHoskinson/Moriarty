//! Ordinary ExtProgram C1-C5 source/table emission for the existing owner engine.
//! Source-only proposal: checked writer/schema integration is owner-owned and pending.
//! No admission, validation, type resolution, execution, or alternative runtime IR.

use super::*;
use super::adt::{CtorIdC, TypeIdC};
use super::owner_ext_t3_uses::{BodyOwner, ExtArm, ExtExpr, ExtPattern, ExtValue, T3UseSite};
use super::owner_ext_type_builder::ExtTypeBuildError;

/// Owner-only checked writes into the ONE ProgramView being prepared. Implement
/// beside owner_prepare, against the frozen canonical source, before registering
/// an extended preparation/admission entry point. All methods are declarations:
/// none author a guard or provide a fallback implementation.
pub(super) trait ExtCProgramWriter {
    /// Reserve the next actual expression/occurrence pair; return its ExprId and
    /// owner-local expression ordinal. Checks precede mutation/child traversal.
    fn reserve_expression(&mut self, source: &ExtExpr, owner: BodyOwner, parent: Option<(usize, usize)>)
        -> Result<(usize, usize), ExtTypeBuildError>;
    /// Read only the prepared source's final global Pass-B name table.
    fn name_id(&self, name: &str) -> Result<usize, ExtTypeBuildError>;
    /// Read the existing final T3 record; do not resolve/instantiate types again.
    fn signature_type(&self, site: T3UseSite) -> Result<TypeIdC, ExtTypeBuildError>;
    fn constructor_use(&self, site: T3UseSite)
        -> Result<(TypeIdC, CtorIdC), ExtTypeBuildError>;
    fn write_expression(&mut self, id: usize, row: ExpressionRowC)
        -> Result<(), ExtTypeBuildError>;
    fn append_code_children(&mut self, children: &[usize]) -> Result<SliceC, ExtTypeBuildError>;
    fn set_call_args(&mut self, id: usize, args: SliceC) -> Result<(), ExtTypeBuildError>;
    fn set_construct_fields(&mut self, id: usize, fields: SliceC) -> Result<(), ExtTypeBuildError>;
    /// The owner retains the current finance source-domain disposition. This
    /// leaf does not convert finance expressions into a scalar/Core surrogate.
    fn write_finance_expression(&mut self, id: usize, source: &ExtExpr, children: &[usize])
        -> Result<(), ExtTypeBuildError>;
    /// Check the next arm before its pattern traversal; reserve its actual row.
    fn reserve_arm(&mut self) -> Result<usize, ExtTypeBuildError>;
    /// Bound source traversal before extending its heap worklist. `ordinal` is
    /// the pattern's source preorder ordinal, not its postorder runtime row ID.
    fn visit_pattern_source(&mut self, owner: BodyOwner, expr: usize, arm: usize, ordinal: usize)
        -> Result<(), ExtTypeBuildError>;
    fn pattern_traversal_invariant(&self) -> ExtTypeBuildError;
    /// Append an actual PatternRowC, with child rows already emitted. For a
    /// Constructor the owner fills its child slice from these ordered row IDs,
    /// checking patternChildSlots/patternRows in the designated owner's order.
    fn append_pattern(&mut self, row: PatternRowC, children: &[usize])
        -> Result<usize, ExtTypeBuildError>;
    fn write_arm(&mut self, id: usize, row: ArmRowC) -> Result<(), ExtTypeBuildError>;
    fn set_match_arms(&mut self, id: usize, arms: &[usize]) -> Result<SliceC, ExtTypeBuildError>;
    fn append_occurrence_path(&mut self, path: &[usize]) -> Result<SliceC, ExtTypeBuildError>;
    fn write_occurrence(&mut self, id: usize, row: OccurrenceRowC)
        -> Result<(), ExtTypeBuildError>;
    /// Preserve the C5 function-row/max-arity checks before parameter appends.
    /// Return the reserved actual function row and current parameter position.
    fn begin_function(&mut self, function: usize, arity: usize)
        -> Result<(usize, usize), ExtTypeBuildError>;
    fn append_parameter(&mut self, row: ParameterRowC) -> Result<(), ExtTypeBuildError>;
    fn write_function(&mut self, id: usize, row: FunctionRowC) -> Result<(), ExtTypeBuildError>;
}

/// Source-use mapping retained for the owner C6/G-5 and P/G-4 continuation.
/// These are borrowed pending source occurrences, not another program/signature
/// table, evaluated values, runtime state, or precomputed match decisions.
pub(super) struct LiteralOccurrence<'s> {
    pub expr_id: usize,
    pub owner: BodyOwner,
    pub local_expr: usize,
    pub value: &'s ExtValue,
}
pub(super) struct NullaryConstructOccurrence {
    pub expr_id: usize,
    pub type_id: TypeIdC,
    pub ctor_id: CtorIdC,
}
pub(super) struct MatchOccurrence<'s> {
    pub expr_id: usize,
    pub owner: BodyOwner,
    pub local_expr: usize,
    pub arms: &'s [ExtArm],
    pub arm_rows: SliceC,
}
pub(super) struct ExtCContinuation<'s> {
    pub literals: Vec<LiteralOccurrence<'s>>,
    pub nullary_constructs: Vec<NullaryConstructOccurrence>,
    pub matches: Vec<MatchOccurrence<'s>>,
}

struct TempExpr<'s> {
    id: usize,
    owner: BodyOwner,
    local_expr: usize,
    source: &'s ExtExpr,
    parent: Option<usize>, // private TempExpr index, never a runtime ExprId
    child_position: usize,
    children: Vec<usize>, // actual child ExprIds, appended in source order
}
struct ExprVisit<'s> {
    source: &'s ExtExpr,
    owner: BodyOwner,
    parent: Option<usize>,
    child_position: usize,
}

/// Materialize the ordinary source traversal/assignments directly through the
/// owner's checked writer. C6 and P continue on these actual rows; this function
/// does not finish a ProgramView, admit it, or make an incomplete view runnable.
pub(super) fn lower_ext_c1_to_c5<'s>(
    prepared: &'s PreparedExtProgramSource,
    writer: &mut impl ExtCProgramWriter,
) -> Result<ExtCContinuation<'s>, ExtTypeBuildError> {
    let program = &prepared.program;
    let mut temps: Vec<TempExpr<'s>> = Vec::new();
    let mut roots: Vec<usize> = Vec::new();
    let mut stack = Vec::new();
    for (function, f) in program.functions.iter().enumerate().rev() {
        stack.push(ExprVisit { source: &f.body, owner: BodyOwner::Function(function),
            parent: None, child_position: 0 });
    }
    stack.push(ExprVisit { source: &program.entry, owner: BodyOwner::Entry,
        parent: None, child_position: 0 });

    // C1: reserve actual ExprIds in entry-then-functions source preorder.
    while let Some(visit) = stack.pop() {
        let parent = visit.parent.map(|p| (temps[p].id, visit.child_position));
        let (id, local_expr) = writer.reserve_expression(visit.source, visit.owner, parent)?;
        if let Some(parent) = visit.parent { temps[parent].children.push(id); }
        else if let BodyOwner::Function(_) = visit.owner { roots.push(id); }
        let index = temps.len();
        temps.push(TempExpr { id, owner: visit.owner, local_expr,
            source: visit.source, parent: visit.parent,
            child_position: visit.child_position, children: Vec::new() });
        let mut push = |source: &'s ExtExpr, child_position| {
            stack.push(ExprVisit { source, owner: visit.owner, parent: Some(index), child_position });
        };
        match visit.source {
            ExtExpr::Let { value, body, .. } => { push(body, 1); push(value, 0); }
            ExtExpr::Binary { left, right, .. } => { push(right, 1); push(left, 0); }
            ExtExpr::Unary { operand, .. } | ExtExpr::Fin { operand, .. } => push(operand, 0),
            ExtExpr::If { cond, then_expr, else_expr } => {
                push(else_expr, 2); push(then_expr, 1); push(cond, 0);
            }
            ExtExpr::Call { args, .. } => {
                for (position, arg) in args.iter().enumerate().rev() { push(arg, position); }
            }
            ExtExpr::Construct { fields, .. } => {
                for (position, field) in fields.iter().enumerate().rev() { push(field, position); }
            }
            ExtExpr::Match { scrutinee, arms } => {
                for (arm, source) in arms.iter().enumerate().rev() { push(&source.body, arm + 1); }
                push(scrutinee, 0);
            }
            ExtExpr::Await { guard, body, .. } => { push(body, 1); push(guard, 0); }
            ExtExpr::Literal { .. } | ExtExpr::Var { .. } | ExtExpr::StateRead { .. }
            | ExtExpr::LiabilityPresent { .. } | ExtExpr::LiabilityAmount { .. } => {}
        }
    }

    let mut continuation = ExtCContinuation {
        literals: Vec::new(), nullary_constructs: Vec::new(), matches: Vec::new(),
    };
    for t in &temps {
        let row = match t.source {
            ExtExpr::Literal { value } => {
                continuation.literals.push(LiteralOccurrence { expr_id: t.id,
                    owner: t.owner, local_expr: t.local_expr, value });
                // Owner C6 owns allocation/typing and supplies the real ConstId.
                // No fake constant ID or incomplete Literal row is emitted.
                continue;
            }
            ExtExpr::Var { name } => ExpressionRowC::Var { binder: writer.name_id(name)? },
            ExtExpr::Let { name, .. } => ExpressionRowC::Let {
                binder: writer.name_id(name)?, value: t.children[0], body: t.children[1] },
            ExtExpr::Binary { op, .. } => ExpressionRowC::Binary {
                op: *op, left: t.children[0], right: t.children[1] },
            ExtExpr::Unary { op, .. } => ExpressionRowC::Unary { op: *op, operand: t.children[0] },
            ExtExpr::If { .. } => ExpressionRowC::If {
                cond: t.children[0], then_expr: t.children[1], else_expr: t.children[2] },
            ExtExpr::Call { name, .. } => ExpressionRowC::Call {
                function_name: writer.name_id(name)?, args: SliceC::empty() },
            ExtExpr::Match { .. } => ExpressionRowC::Match {
                scrutinee: t.children[0], arms: SliceC::empty(), plan: None },
            ExtExpr::Construct { fields, .. } => {
                let (type_id, ctor_id) = writer.constructor_use(
                    T3UseSite::Construct { owner: t.owner, expr: t.local_expr })?;
                if fields.is_empty() {
                    continuation.nullary_constructs.push(NullaryConstructOccurrence {
                        expr_id: t.id, type_id, ctor_id });
                }
                ExpressionRowC::Construct { type_id, ctor_id, fields: SliceC::empty(), constant: None }
            }
            ExtExpr::StateRead { .. } | ExtExpr::LiabilityPresent { .. }
            | ExtExpr::LiabilityAmount { .. } | ExtExpr::Fin { .. } | ExtExpr::Await { .. } => {
                writer.write_finance_expression(t.id, t.source, &t.children)?;
                continue;
            }
        };
        writer.write_expression(t.id, row)?;
    }

    // C2: calls and constructor fields share the actual code-child table.
    for t in &temps {
        match t.source {
            ExtExpr::Call { .. } => {
                let slice = writer.append_code_children(&t.children)?;
                writer.set_call_args(t.id, slice)?;
            }
            ExtExpr::Construct { .. } => {
                let slice = writer.append_code_children(&t.children)?;
                writer.set_construct_fields(t.id, slice)?;
            }
            _ => {}
        }
    }

    // C3: arm source order; generic pattern rows use field-first postorder,
    // while T3 lookups retain each pattern's independent source preorder.
    for t in &temps {
        if let ExtExpr::Match { arms, .. } = t.source {
            let mut arm_rows = Vec::new();
            for (arm, source) in arms.iter().enumerate() {
                let id = writer.reserve_arm()?;
                let pattern = emit_pattern(writer, t.owner, t.local_expr, arm, &source.pattern)?;
                writer.write_arm(id, ArmRowC { pattern, body: t.children[arm + 1] })?;
                arm_rows.push(id);
            }
            let arm_rows = writer.set_match_arms(t.id, &arm_rows)?;
            continuation.matches.push(MatchOccurrence { expr_id: t.id,
                owner: t.owner, local_expr: t.local_expr, arms, arm_rows });
        }
    }

    // C4: root-first source occurrence paths, including constructor fields.
    for t in &temps {
        let mut path = Vec::new();
        let mut cursor = t;
        while let Some(parent) = cursor.parent {
            path.push(cursor.child_position);
            cursor = &temps[parent];
        }
        path.reverse();
        let path = writer.append_occurrence_path(&path)?;
        let (owner_is_function, function) = match t.owner {
            BodyOwner::Entry => (false, 0), BodyOwner::Function(function) => (true, function),
        };
        writer.write_occurrence(t.id, OccurrenceRowC { owner_is_function, function, path })?;
    }

    // C5: actual final runtime signature rows; IDs come from the one completed
    // Pass T, and body IDs come from the actual C1 reservations above.
    for (function, source) in program.functions.iter().enumerate() {
        let (id, parameter_start) = writer.begin_function(function, source.parameters.len())?;
        for (param, source) in source.parameters.iter().enumerate() {
            let sort = writer.signature_type(T3UseSite::ParamType { function, param })?;
            let binder = writer.name_id(&source.name)?;
            writer.append_parameter(ParameterRowC { binder, sort })?;
        }
        let return_type = writer.signature_type(T3UseSite::ReturnType { function })?;
        let function_name = writer.name_id(&source.name)?;
        writer.write_function(id, FunctionRowC { function_name,
            params: SliceC::normalized(parameter_start, source.parameters.len()),
            return_type, body: roots[function] })?;
    }
    Ok(continuation)
}

struct PatternTemp<'s> {
    source: &'s ExtPattern,
    parent: Option<usize>,
    children: Vec<usize>,
}
enum PatternVisit<'s> {
    Enter { source: &'s ExtPattern, parent: Option<usize> },
    Exit { index: usize },
}
fn emit_pattern(
    writer: &mut impl ExtCProgramWriter,
    owner: BodyOwner,
    expr: usize,
    arm: usize,
    source: &ExtPattern,
) -> Result<usize, ExtTypeBuildError> {
    let mut temps: Vec<PatternTemp<'_>> = Vec::new();
    let mut stack = vec![PatternVisit::Enter { source, parent: None }];
    // The root result is returned at its real postorder append, so no sentinel
    // pattern ID or separate root table is ever manufactured.
    while let Some(visit) = stack.pop() {
        match visit {
            PatternVisit::Enter { source, parent } => {
                let index = temps.len();
                writer.visit_pattern_source(owner, expr, arm, index)?;
                temps.push(PatternTemp { source, parent, children: Vec::new() });
                stack.push(PatternVisit::Exit { index });
                if let ExtPattern::Ctor { fields, .. } = source {
                    for field in fields.iter().rev() {
                        stack.push(PatternVisit::Enter { source: field, parent: Some(index) });
                    }
                }
            }
            PatternVisit::Exit { index } => {
                let t = &temps[index];
                let row = match t.source {
                    ExtPattern::Dedicated { pattern } => match pattern {
                        CorePattern::PVar { x } => PatternRowC::PVar { x: writer.name_id(x)? },
                        CorePattern::PNil => PatternRowC::PNil,
                        CorePattern::PCons { x, y } => PatternRowC::PCons {
                            x: writer.name_id(x)?, y: writer.name_id(y)? },
                        CorePattern::PLeaf { x } => PatternRowC::PLeaf { x: writer.name_id(x)? },
                        CorePattern::PBranch { x, y } => PatternRowC::PBranch {
                            x: writer.name_id(x)?, y: writer.name_id(y)? },
                    },
                    ExtPattern::Wildcard => PatternRowC::Wildcard,
                    ExtPattern::Ctor { .. } => {
                        let (type_id, ctor_id) = writer.constructor_use(
                            T3UseSite::CtorPattern { owner, expr, arm, pattern: index })?;
                        PatternRowC::Constructor { type_id, ctor_id, children: SliceC::empty() }
                    }
                };
                let id = writer.append_pattern(row, &t.children)?;
                if let Some(parent) = t.parent { temps[parent].children.push(id); }
                else { return Ok(id); }
            }
        }
    }
    // An entered root always emits exactly once. The owner owns the diagnostic
    // for an inconsistent checked source/writer integration; this leaf invents
    // neither a guard nor a fallback row.
    Err(writer.pattern_traversal_invariant())
}
