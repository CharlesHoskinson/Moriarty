//! Ordinary Pass P matrix construction over the SAME C1-C6 canonical tables.
//! Checked storage, G-4, report interpretation and admission remain owner-only.
use super::adt::{CtorIdC, TypeIdC};
use super::ext_program_c::{ExtCProgramWriter, MatchOccurrence};
use super::ext_program_c6::{ExtC6ProgramWriter, lower_ext_c1_to_c6};
use super::owner_ext_type_builder::ExtTypeBuildError;
use super::{PatternRowC, PreparedExtProgramSource, SliceC};
use std::collections::BTreeSet;

/// Actual C3 arm/pattern IDs, returned only after the owner's binding of the
/// retained MatchOccurrence to the same private canonical assembly.
pub(super) struct PlanArmSourceC {
    pub arm: usize,
    pub pattern: usize,
}
pub(super) struct PatternBindingC {
    pub binder: usize,
    pub path: Vec<usize>,
}
#[derive(Clone, Copy)]
pub(super) enum PlanVisitKindC {
    Switch,
    Bind,
    Select,
    NoMatch,
}

/// Complete private owner interface, not a public caller-supplied preparation
/// API. All IDs are real reservations or reads in the existing arenas. A
/// reserved node remains Option::None until its forward links are filled;
/// no dummy next/default/root is put in a canonical row or exported view.
pub(super) trait ExtPProgramWriter {
    /// Validate same-assembly source/ExprId/order/C6 completion. Return None
    /// for a dedicated-only match, retaining its absent plan and old semantics;
    /// otherwise return its actual arm rows in source order. No plan writes.
    fn begin_match(
        &mut self,
        source: &MatchOccurrence<'_>,
    ) -> Result<Option<Vec<PlanArmSourceC>>, ExtTypeBuildError>;
    fn pattern(&self, id: usize) -> Result<PatternRowC, ExtTypeBuildError>;
    fn pattern_children(&self, slice: SliceC) -> Result<Vec<usize>, ExtTypeBuildError>;
    /// G-4 in arm order, with original-pattern preorder binders. Owner authors
    /// duplicate-name judgment, diagnostic/locator and exact failure ordering.
    fn check_arm_binders(
        &mut self,
        source: &MatchOccurrence<'_>,
        arm: usize,
        binders: &[PatternBindingC],
    ) -> Result<(), ExtTypeBuildError>;
    fn constructor_arity(&self, ctor: CtorIdC) -> Result<usize, ExtTypeBuildError>;
    /// Reserve/check node FIRST, then its complete absent case block and its
    /// own unshared path cells, in the owner's prescribed capacity order.
    /// The case block spans the real type row's complete constructor slice.
    fn reserve_switch(
        &mut self,
        source_match: usize,
        path: &[usize],
        type_id: TypeIdC,
    ) -> Result<usize, ExtTypeBuildError>;
    fn set_switch_case(
        &mut self,
        node: usize,
        ctor: CtorIdC,
        target: usize,
    ) -> Result<(), ExtTypeBuildError>;
    /// Fill the actual reserved Switch row with its real default root, after
    /// all present case roots have been linked. Unselected cases stay absent.
    fn finish_switch(&mut self, node: usize, default: usize) -> Result<(), ExtTypeBuildError>;
    fn reserve_bind(
        &mut self,
        source_match: usize,
        path: &[usize],
        binder: usize,
    ) -> Result<usize, ExtTypeBuildError>;
    fn finish_bind(&mut self, node: usize, next: usize) -> Result<(), ExtTypeBuildError>;
    fn append_select(
        &mut self,
        source_match: usize,
        arm: usize,
    ) -> Result<usize, ExtTypeBuildError>;
    fn append_no_match(&mut self, source_match: usize) -> Result<usize, ExtTypeBuildError>;
    /// Report-only provenance for each actual emitted node. candidate_arms are
    /// source arm ordinals of its current matrix, in source order; winner is
    /// present only after step 2 fixes that arm. The owner must derive the
    /// public workMap.liveArms interpretation from these explicit facts, and
    /// emit path.len Walk events then kind's one event without merging visits.
    /// No metadata is stored as a second runtime plan or used for evaluation.
    fn record_plan_work(
        &mut self,
        node: usize,
        source_match: usize,
        candidate_arms: &[usize],
        winner: Option<usize>,
        path: &[usize],
        kind: PlanVisitKindC,
    ) -> Result<(), ExtTypeBuildError>;
    /// Only after every node/link is filled: update this existing Match.plan.
    /// This does not finalize ProgramView, run V14 or admit a program.
    fn finish_match(
        &mut self,
        source: &MatchOccurrence<'_>,
        root: usize,
    ) -> Result<(), ExtTypeBuildError>;
    fn plan_traversal_invariant(&self) -> ExtTypeBuildError;
}

// Matrix cells reference canonical C3 rows. Synthetic cells below are only the
// compile-time expansion of dedicated binders or irrefutable specializations;
// no normalized pattern tree or parallel runtime IR is stored or exported.
#[derive(Clone, Copy)]
enum Cell {
    Row(usize),
    Wildcard,
    Binder(usize),
}
enum Head {
    Irrefutable,
    Constructor {
        type_id: TypeIdC,
        ctor_id: CtorIdC,
        fields: Vec<Cell>,
    },
}
struct MatrixRow {
    source_arm: usize,
    cells: Vec<Cell>,
}
struct Matrix {
    rows: Vec<MatrixRow>,
    occurrences: Vec<Vec<usize>>,
}

fn head(cell: Cell, writer: &impl ExtPProgramWriter) -> Result<Head, ExtTypeBuildError> {
    let row = match cell {
        Cell::Wildcard | Cell::Binder(_) => return Ok(Head::Irrefutable),
        Cell::Row(id) => writer.pattern(id)?,
    };
    // These are the fixed existing builtin TypeIds/CtorIds, never fabricated
    // user identities or surrogate values. Dedicated rows remain stored once.
    Ok(match row {
        PatternRowC::PVar { .. } | PatternRowC::Wildcard => Head::Irrefutable,
        PatternRowC::PNil => Head::Constructor {
            type_id: TypeIdC::List,
            ctor_id: CtorIdC(0),
            fields: vec![],
        },
        PatternRowC::PCons { x, y } => Head::Constructor {
            type_id: TypeIdC::List,
            ctor_id: CtorIdC(1),
            fields: vec![Cell::Binder(x), Cell::Binder(y)],
        },
        PatternRowC::PLeaf { x } => Head::Constructor {
            type_id: TypeIdC::Tree,
            ctor_id: CtorIdC(2),
            fields: vec![Cell::Binder(x)],
        },
        PatternRowC::PBranch { x, y } => Head::Constructor {
            type_id: TypeIdC::Tree,
            ctor_id: CtorIdC(3),
            fields: vec![Cell::Binder(x), Cell::Binder(y)],
        },
        PatternRowC::Constructor {
            type_id,
            ctor_id,
            children,
        } => Head::Constructor {
            type_id,
            ctor_id,
            fields: writer
                .pattern_children(children)?
                .into_iter()
                .map(Cell::Row)
                .collect(),
        },
    })
}

/// Original pattern preorder and static field paths, independent of matrix
/// rewrites. G-4 is called before any node of this match can be emitted.
fn bindings(
    root: usize,
    writer: &impl ExtPProgramWriter,
) -> Result<Vec<PatternBindingC>, ExtTypeBuildError> {
    let mut out = Vec::new();
    let mut stack = vec![(Cell::Row(root), Vec::new())];
    while let Some((cell, path)) = stack.pop() {
        let binder = match cell {
            Cell::Binder(binder) => Some(binder),
            Cell::Row(id) => match writer.pattern(id)? {
                PatternRowC::PVar { x } => Some(x),
                _ => None,
            },
            Cell::Wildcard => None,
        };
        if let Some(binder) = binder {
            out.push(PatternBindingC { binder, path });
        } else if let Head::Constructor { fields, .. } = head(cell, writer)? {
            for (position, field) in fields.into_iter().enumerate().rev() {
                let mut child_path = path.clone();
                child_path.push(position);
                stack.push((field, child_path));
            }
        }
    }
    Ok(out)
}

// Forward links are represented only as private worklist destinations, never
// predicted IDs. A child attaches only after its actual reservation/append.
enum Target {
    Root,
    BindNext(usize),
    Case { switch: usize, ctor: CtorIdC },
    Default(usize),
}
enum Work {
    Compile {
        matrix: Matrix,
        target: Target,
    },
    SwitchChildren {
        node: usize,
        matrix: Matrix,
        column: usize,
        type_id: TypeIdC,
        cases: Vec<CtorIdC>,
        cursor: usize,
    },
}
fn attach(
    writer: &mut impl ExtPProgramWriter,
    root: &mut Option<usize>,
    target: Target,
    id: usize,
) -> Result<(), ExtTypeBuildError> {
    match target {
        Target::Root => *root = Some(id),
        Target::BindNext(node) => writer.finish_bind(node, id)?,
        Target::Case { switch, ctor } => writer.set_switch_case(switch, ctor, id)?,
        Target::Default(node) => writer.finish_switch(node, id)?,
    }
    Ok(())
}

fn replace_cells(row: &MatrixRow, column: usize, replacement: &[Cell]) -> MatrixRow {
    let cells = row.cells[..column]
        .iter()
        .copied()
        .chain(replacement.iter().copied())
        .chain(row.cells[column + 1..].iter().copied())
        .collect();
    MatrixRow {
        source_arm: row.source_arm,
        cells,
    }
}
fn specialized(
    matrix: &Matrix,
    column: usize,
    type_id: TypeIdC,
    ctor: CtorIdC,
    writer: &impl ExtPProgramWriter,
) -> Result<Matrix, ExtTypeBuildError> {
    let arity = writer.constructor_arity(ctor)?;
    let mut rows = Vec::new();
    for row in &matrix.rows {
        match head(row.cells[column], writer)? {
            Head::Irrefutable => {
                rows.push(replace_cells(row, column, &vec![Cell::Wildcard; arity]))
            }
            Head::Constructor {
                type_id: t,
                ctor_id: k,
                fields,
            } if t == type_id && k == ctor => rows.push(replace_cells(row, column, &fields)),
            Head::Constructor { .. } => {}
        }
    }
    let mut occurrences = matrix.occurrences[..column].to_vec();
    for field in 0..arity {
        let mut path = matrix.occurrences[column].clone();
        path.push(field);
        occurrences.push(path);
    }
    occurrences.extend_from_slice(&matrix.occurrences[column + 1..]);
    Ok(Matrix { rows, occurrences })
}
fn defaulted(
    matrix: &Matrix,
    column: usize,
    type_id: TypeIdC,
    writer: &impl ExtPProgramWriter,
) -> Result<Matrix, ExtTypeBuildError> {
    let mut rows = Vec::new();
    let mut keep_column = false;
    for row in &matrix.rows {
        match head(row.cells[column], writer)? {
            Head::Irrefutable => rows.push(replace_cells(row, column, &[Cell::Wildcard])),
            Head::Constructor { type_id: t, .. } if t != type_id => {
                keep_column = true;
                rows.push(replace_cells(row, column, &[row.cells[column]]));
            }
            Head::Constructor { .. } => {}
        }
    }
    let mut occurrences = matrix.occurrences.clone();
    if !keep_column {
        for row in &mut rows {
            row.cells.remove(column);
        }
        occurrences.remove(column);
    }
    Ok(Matrix { rows, occurrences })
}

fn compile_match(
    source: &MatchOccurrence<'_>,
    arms: &[PlanArmSourceC],
    binders: &[Vec<PatternBindingC>],
    writer: &mut impl ExtPProgramWriter,
) -> Result<usize, ExtTypeBuildError> {
    let matrix = Matrix {
        rows: arms
            .iter()
            .enumerate()
            .map(|(source_arm, arm)| MatrixRow {
                source_arm,
                cells: vec![Cell::Row(arm.pattern)],
            })
            .collect(),
        occurrences: vec![vec![]],
    };
    let mut root = None;
    let mut stack = vec![Work::Compile {
        matrix,
        target: Target::Root,
    }];
    while let Some(work) = stack.pop() {
        match work {
            Work::Compile { matrix, mut target } => {
                let live: Vec<usize> = matrix.rows.iter().map(|r| r.source_arm).collect();
                let first = match matrix.rows.first() {
                    None => {
                        let node = writer.append_no_match(source.expr_id)?;
                        writer.record_plan_work(
                            node,
                            source.expr_id,
                            &live,
                            None,
                            &[],
                            PlanVisitKindC::NoMatch,
                        )?;
                        attach(writer, &mut root, target, node)?;
                        continue;
                    }
                    Some(first) => first,
                };
                let mut split = None;
                // Least constructor-class column of the FIRST surviving arm.
                for (column, cell) in first.cells.iter().enumerate() {
                    if let Head::Constructor { type_id, .. } = head(*cell, writer)? {
                        split = Some((column, type_id));
                        break;
                    }
                }
                match split {
                    None => {
                        // The winner is fixed before any Bind is emitted. Its
                        // ORIGINAL pattern supplies binder order/static paths.
                        let winner = first.source_arm;
                        for binding in &binders[winner] {
                            let node = writer.reserve_bind(
                                source.expr_id,
                                &binding.path,
                                binding.binder,
                            )?;
                            writer.record_plan_work(
                                node,
                                source.expr_id,
                                &live,
                                Some(winner),
                                &binding.path,
                                PlanVisitKindC::Bind,
                            )?;
                            attach(writer, &mut root, target, node)?;
                            target = Target::BindNext(node);
                        }
                        let node = writer.append_select(source.expr_id, arms[winner].arm)?;
                        writer.record_plan_work(
                            node,
                            source.expr_id,
                            &live,
                            Some(winner),
                            &[],
                            PlanVisitKindC::Select,
                        )?;
                        attach(writer, &mut root, target, node)?;
                    }
                    Some((column, type_id)) => {
                        // Reserve this node BEFORE constructing ANY child
                        // matrix. The owner checks bounded nodes/cases/paths.
                        let node = writer.reserve_switch(
                            source.expr_id,
                            &matrix.occurrences[column],
                            type_id,
                        )?;
                        writer.record_plan_work(
                            node,
                            source.expr_id,
                            &live,
                            None,
                            &matrix.occurrences[column],
                            PlanVisitKindC::Switch,
                        )?;
                        attach(writer, &mut root, target, node)?;
                        let mut cases = BTreeSet::new();
                        for row in &matrix.rows {
                            if let Head::Constructor {
                                type_id: t,
                                ctor_id,
                                ..
                            } = head(row.cells[column], writer)?
                            {
                                if t == type_id {
                                    cases.insert(ctor_id);
                                }
                            }
                        }
                        stack.push(Work::SwitchChildren {
                            node,
                            matrix,
                            column,
                            type_id,
                            cases: cases.into_iter().collect(),
                            cursor: 0,
                        });
                    }
                }
            }
            Work::SwitchChildren {
                node,
                matrix,
                column,
                type_id,
                cases,
                cursor,
            } => {
                if let Some(ctor) = cases.get(cursor).copied() {
                    let child = specialized(&matrix, column, type_id, ctor, writer)?;
                    stack.push(Work::SwitchChildren {
                        node,
                        matrix,
                        column,
                        type_id,
                        cases,
                        cursor: cursor + 1,
                    });
                    stack.push(Work::Compile {
                        matrix: child,
                        target: Target::Case { switch: node, ctor },
                    });
                } else {
                    let child = defaulted(&matrix, column, type_id, writer)?;
                    stack.push(Work::Compile {
                        matrix: child,
                        target: Target::Default(node),
                    });
                }
            }
        }
    }
    root.ok_or_else(|| writer.plan_traversal_invariant())
}

/// Callable ordinary product predicate: consume the real C6 continuation in
/// Match ExprId order; only planned matches acquire real complete plan roots.
pub(super) fn lower_ext_p(
    matches: Vec<MatchOccurrence<'_>>,
    writer: &mut impl ExtPProgramWriter,
) -> Result<(), ExtTypeBuildError> {
    for source in &matches {
        let arms = match writer.begin_match(source)? {
            None => continue,
            Some(arms) => arms,
        };
        let mut arm_binders = Vec::new();
        for (arm, row) in arms.iter().enumerate() {
            let binders = bindings(row.pattern, writer)?;
            writer.check_arm_binders(source, arm, &binders)?;
            arm_binders.push(binders);
        }
        let root = compile_match(source, &arms, &arm_binders, writer)?;
        writer.finish_match(source, root)?;
    }
    Ok(())
}

/// Same private writer and prepared source; no complete/public view exists
/// until the owner finishes its remaining checks and actual final assembly.
pub(super) fn lower_ext_c1_to_p<W>(
    prepared: &PreparedExtProgramSource,
    writer: &mut W,
) -> Result<(), ExtTypeBuildError>
where
    W: ExtCProgramWriter + ExtC6ProgramWriter + ExtPProgramWriter,
{
    let matches = lower_ext_c1_to_c6(prepared, writer)?;
    lower_ext_p(matches, writer)
}
