use std::collections::BTreeSet;
use std::sync::Arc;

use super::adt::{CtorIdC, TypeIdC, TypeRowC};
use super::adt_storage::{CtorFieldRowExt, CtorRowExt};
use super::owner_ext_c_expression::prepare_ext_c1_to_p;
use super::owner_ext_source_program::{ExtDomain, ExtProgram};
use super::owner_ext_t3_uses::{ExtArm, ExtExpr, ExtFunction, ExtParamSrc, ExtPattern, ExtValue};
use super::owner_ext_type_builder::{
    CtorDeclSrc, CtorFieldSrc, ExtTypeBuildError, TypeDeclSrc, TypeExprSrc,
};
use super::owner_runtime::check_generic_program_view;
use super::super::raw::{
    ConstFieldPos as RawConstFieldPos, ConstFieldSlice as RawConstFieldSlice,
    ConstRow as RawConstRow, CtorId as RawCtorId, TypeId as RawTypeId,
};
use super::{
    AdmissionReason, Admitted, AdmittedBody, ArmRowC, BoundsC, CapacityKind, CorePattern,
    CoreTree, CoreValue, CounterWidthField, DigestC, ExpressionRowC, FeatureKind, FinanceInput,
    FunctionRowC, GrammarDomain, IdentityC, LayoutSchemaC, LocatorC, NeededNatC, OccurrenceRowC,
    ParameterRowC, PatternRowC, PlanNodeC, ProgramView, RuleIdC, SliceC, Tooling,
    i128_class::KIntC, prepare_ext_program_types,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct ExtCapacitiesC {
    pub plan_case_slots: usize,
    pub field_path_slots: usize,
    pub pattern_child_slots: usize,
    pub input_rows: usize,
    pub asset_rows: usize,
    pub state_rows: usize,
    pub account_rows: usize,
    pub liability_rows: usize,
    pub await_site_rows: usize,
    pub await_binder_slots: usize,
    pub balance_cells: usize,
    pub journal_rows: usize,
    pub pending_op_rows: usize,
    pub observations_per_invocation: usize,
    pub max_lifetime_work: u128,
    pub resume_rows: usize,
}

impl ExtCapacitiesC {
    pub const fn all_zero() -> Self {
        Self {
            plan_case_slots: 0,
            field_path_slots: 0,
            pattern_child_slots: 0,
            input_rows: 0,
            asset_rows: 0,
            state_rows: 0,
            account_rows: 0,
            liability_rows: 0,
            await_site_rows: 0,
            await_binder_slots: 0,
            balance_cells: 0,
            journal_rows: 0,
            pending_op_rows: 0,
            observations_per_invocation: 0,
            max_lifetime_work: 0,
            resume_rows: 0,
        }
    }

    pub fn effective_for_extended_domain(opt: Option<Self>) -> Self {
        opt.unwrap_or_else(Self::all_zero)
    }
}

pub struct WorkMapEntryC {
    pub source_match: usize,
    pub live_arms: Vec<usize>,
    pub field_path: Vec<usize>,
    pub events_per_visit: Vec<RuleIdC>,
}

impl PartialEq for WorkMapEntryC {
    fn eq(&self, other: &Self) -> bool {
        self.source_match == other.source_match
            && self.live_arms == other.live_arms
            && self.field_path == other.field_path
            && self.events_per_visit.len() == other.events_per_visit.len()
            && self
                .events_per_visit
                .iter()
                .zip(other.events_per_visit.iter())
                .all(|(a, b)| std::mem::discriminant(a) == std::mem::discriminant(b))
    }
}

impl Eq for WorkMapEntryC {}

pub struct PreparationReportC {
    pub work_map: Vec<WorkMapEntryC>,
}

impl PartialEq for PreparationReportC {
    fn eq(&self, other: &Self) -> bool {
        self.work_map == other.work_map
    }
}

impl Eq for PreparationReportC {}

enum CloneTypeTask<'a> {
    Visit(&'a TypeExprSrc),
    BuildApply {
        name: String,
        arity: usize,
    },
}

pub(crate) fn clone_type_expr_iterative(ty: &TypeExprSrc) -> TypeExprSrc {
    let mut stack = vec![CloneTypeTask::Visit(ty)];
    let mut out: Vec<TypeExprSrc> = Vec::new();
    while let Some(task) = stack.pop() {
        match task {
            CloneTypeTask::Visit(node) => match node {
                TypeExprSrc::UInt => out.push(TypeExprSrc::UInt),
                TypeExprSrc::Bool => out.push(TypeExprSrc::Bool),
                TypeExprSrc::List => out.push(TypeExprSrc::List),
                TypeExprSrc::Tree => out.push(TypeExprSrc::Tree),
                TypeExprSrc::Named { name } => out.push(TypeExprSrc::Named {
                    name: name.clone(),
                }),
                TypeExprSrc::Param { name } => out.push(TypeExprSrc::Param {
                    name: name.clone(),
                }),
                TypeExprSrc::Apply { name, args } => {
                    stack.push(CloneTypeTask::BuildApply {
                        name: name.clone(),
                        arity: args.len(),
                    });
                    for arg in args.iter().rev() {
                        stack.push(CloneTypeTask::Visit(arg));
                    }
                }
            },
            CloneTypeTask::BuildApply { name, arity } => {
                let split_at = out.len().saturating_sub(arity);
                let args = out.split_off(split_at);
                out.push(TypeExprSrc::Apply { name, args });
            }
        }
    }
    out.pop().unwrap_or(TypeExprSrc::UInt)
}

pub(crate) fn clone_type_decls_iterative(decls: &[TypeDeclSrc]) -> Vec<TypeDeclSrc> {
    let mut out = Vec::with_capacity(decls.len());
    for decl in decls {
        let mut ctors = Vec::with_capacity(decl.ctors.len());
        for ctor in &decl.ctors {
            let mut fields = Vec::with_capacity(ctor.fields.len());
            for field in &ctor.fields {
                fields.push(CtorFieldSrc {
                    name: field.name.clone(),
                    ty: clone_type_expr_iterative(&field.ty),
                });
            }
            ctors.push(CtorDeclSrc {
                name: ctor.name.clone(),
                fields,
            });
        }
        out.push(TypeDeclSrc {
            name: decl.name.clone(),
            parameters: decl.parameters.clone(),
            ctors,
        });
    }
    out
}

fn field_loc(name: &str) -> Option<LocatorC> {
    Some(LocatorC::Field {
        name: name.to_string(),
    })
}

fn bitlen_count(n: u128) -> u128 {
    if n == 0 {
        1
    } else {
        (128 - n.leading_zeros()) as u128
    }
}

fn bitlen_id(z: u128) -> u128 {
    if z <= 1 {
        1
    } else {
        (128 - (z - 1).leading_zeros()) as u128
    }
}

fn pv_malformed(check_id: &str) -> ExtTypeBuildError {
    ExtTypeBuildError::CoreTooling(Tooling::AdmissionRejected {
        reason: AdmissionReason::ProgramViewMalformed,
        detail: check_id.to_string(),
        locator: None,
    })
}

// Checked constructor lookup for phases that run before V9 validates the type
// and constructor tables: the type row exists, its constructor slice end is
// computed without overflow, ctor_id lies in that slice and its row exists.
fn checked_type_ctor(
    v: &ProgramView,
    type_id: TypeIdC,
    ctor_id: CtorIdC,
) -> Option<&CtorRowExt> {
    let trow = v.types.get(type_id.0)?;
    let c_end = trow.ctor_start.0.checked_add(trow.ctor_len)?;
    if ctor_id.0 < trow.ctor_start.0 || ctor_id.0 >= c_end {
        return None;
    }
    v.constructors.get(ctor_id.0)
}

fn const_row_marker_free(row: &RawConstRow) -> bool {
    match row {
        RawConstRow::U128 { marker_free, .. }
        | RawConstRow::Bool { marker_free, .. }
        | RawConstRow::Nil { marker_free }
        | RawConstRow::Cons { marker_free, .. }
        | RawConstRow::Leaf { marker_free, .. }
        | RawConstRow::Branch { marker_free, .. }
        | RawConstRow::Constructor { marker_free, .. }
        | RawConstRow::U128OutOfRange { marker_free } => *marker_free,
    }
}

fn const_row_fields_slice(row: &RawConstRow) -> SliceC {
    match row {
        RawConstRow::Cons { fields, .. }
        | RawConstRow::Leaf { fields, .. }
        | RawConstRow::Branch { fields, .. }
        | RawConstRow::Constructor { fields, .. } => SliceC {
            start: fields.start.0,
            len: fields.len,
        },
        _ => SliceC { start: 0, len: 0 },
    }
}

fn const_row_type_id(row: &RawConstRow) -> TypeIdC {
    match row {
        RawConstRow::U128 { .. } | RawConstRow::U128OutOfRange { .. } => TypeIdC(0),
        RawConstRow::Bool { .. } => TypeIdC(1),
        RawConstRow::Nil { .. } | RawConstRow::Cons { .. } => TypeIdC(2),
        RawConstRow::Leaf { .. } | RawConstRow::Branch { .. } => TypeIdC(3),
        RawConstRow::Constructor { type_id, .. } => TypeIdC(type_id.0),
    }
}

fn resolve_template_type_in_view(
    ty: &TypeExprSrc,
    param_names: &[String],
    inst_args: &[TypeIdC],
    v: &ProgramView,
) -> Option<TypeIdC> {
    enum ResolveTask<'a> {
        Visit(&'a TypeExprSrc),
        FinishApply {
            decl_idx: usize,
            arity: usize,
        },
    }
    let mut stack = vec![ResolveTask::Visit(ty)];
    let mut out: Vec<TypeIdC> = Vec::new();
    while let Some(task) = stack.pop() {
        match task {
            ResolveTask::Visit(node) => match node {
                TypeExprSrc::UInt => out.push(TypeIdC(0)),
                TypeExprSrc::Bool => out.push(TypeIdC(1)),
                TypeExprSrc::List => out.push(TypeIdC(2)),
                TypeExprSrc::Tree => out.push(TypeIdC(3)),
                TypeExprSrc::Param { name } => {
                    let pos = param_names.iter().position(|p| p == name)?;
                    out.push(*inst_args.get(pos)?);
                }
                TypeExprSrc::Named { name } => {
                    let decl_idx = v
                        .decl_templates
                        .iter()
                        .position(|d| d.name == *name && d.parameters.is_empty())?;
                    let tid = v
                        .types
                        .iter()
                        .enumerate()
                        .skip(4)
                        .find(|(_, r)| r.decl == Some(decl_idx) && r.args.len == 0)
                        .map(|(i, _)| TypeIdC(i))?;
                    out.push(tid);
                }
                TypeExprSrc::Apply { name, args } => {
                    let decl_idx = v
                        .decl_templates
                        .iter()
                        .position(|d| d.name == *name && d.parameters.len() == args.len())?;
                    stack.push(ResolveTask::FinishApply {
                        decl_idx,
                        arity: args.len(),
                    });
                    for arg in args.iter().rev() {
                        stack.push(ResolveTask::Visit(arg));
                    }
                }
            },
            ResolveTask::FinishApply { decl_idx, arity } => {
                if out.len() < arity {
                    return None;
                }
                let split_at = out.len() - arity;
                let resolved_args = out.split_off(split_at);
                let tid = v
                    .types
                    .iter()
                    .enumerate()
                    .skip(4)
                    .find(|(_, r)| {
                        r.decl == Some(decl_idx)
                            && r.args.len == arity
                            && v.type_args
                                .get(r.args.start..r.args.start.saturating_add(r.args.len))
                                == Some(resolved_args.as_slice())
                    })
                    .map(|(i, _)| TypeIdC(i))?;
                out.push(tid);
            }
        }
    }
    if out.len() == 1 {
        out.pop()
    } else {
        None
    }
}

fn decode_const_tree_iterative(
    root_cid: usize,
    constants: &[RawConstRow],
    constant_fields: &[usize],
    decoded_types: &[TypeExprSrc],
    constructors: &[CtorRowExt],
    names: &[String],
) -> Option<ExtValue> {
    enum DecodeConstStep {
        Visit(usize),
        BuildTreeLeaf,
        BuildTreeBranch,
        BuildCtor {
            type_id: usize,
            ctor_id: usize,
            arity: usize,
        },
    }
    let mut stack = vec![DecodeConstStep::Visit(root_cid)];
    let mut val_stack: Vec<ExtValue> = Vec::new();
    let mut tree_stack: Vec<CoreTree> = Vec::new();
    while let Some(step) = stack.pop() {
        match step {
            DecodeConstStep::Visit(cid) => match constants.get(cid)? {
                RawConstRow::U128 { n, .. } => {
                    val_stack.push(ExtValue::Core {
                        value: CoreValue::UInt {
                            n: KIntC::InRange(*n),
                        },
                    });
                }
                RawConstRow::U128OutOfRange { .. } => {
                    val_stack.push(ExtValue::Core {
                        value: CoreValue::UInt {
                            n: KIntC::AboveU128,
                        },
                    });
                }
                RawConstRow::Bool { b, .. } => {
                    val_stack.push(ExtValue::Core {
                        value: CoreValue::Bool { b: *b },
                    });
                }
                RawConstRow::Nil { .. } | RawConstRow::Cons { .. } => {
                    let mut elems: Vec<KIntC> = Vec::new();
                    let mut cur = cid;
                    let mut steps = 0usize;
                    loop {
                        if steps > constants.len() {
                            return None;
                        }
                        steps += 1;
                        match constants.get(cur)? {
                            RawConstRow::Nil { .. } => break,
                            RawConstRow::Cons { fields, .. } => {
                                if fields.len != 2 {
                                    return None;
                                }
                                let head_cid = *constant_fields.get(fields.start.0)?;
                                let tail_cid = *constant_fields.get(fields.start.0.checked_add(1)?)?;
                                if head_cid >= cur || tail_cid >= cur {
                                    return None;
                                }
                                let k = match constants.get(head_cid)? {
                                    RawConstRow::U128 { n, .. } => KIntC::InRange(*n),
                                    RawConstRow::U128OutOfRange { .. } => KIntC::AboveU128,
                                    _ => return None,
                                };
                                elems.push(k);
                                cur = tail_cid;
                            }
                            _ => return None,
                        }
                    }
                    val_stack.push(ExtValue::Core {
                        value: CoreValue::List { elements: elems },
                    });
                }
                RawConstRow::Leaf { .. } | RawConstRow::Branch { .. } => {
                    enum TreeStep {
                        Enter(usize),
                        FinishLeaf,
                        FinishBranch,
                    }
                    let mut tstack = vec![TreeStep::Enter(cid)];
                    while let Some(ts) = tstack.pop() {
                        match ts {
                            TreeStep::Enter(tcid) => match constants.get(tcid)? {
                                RawConstRow::Leaf { fields, .. } => {
                                    if fields.len != 1 {
                                        return None;
                                    }
                                    let scid = *constant_fields.get(fields.start.0)?;
                                    if scid >= tcid {
                                        return None;
                                    }
                                    let k = match constants.get(scid)? {
                                        RawConstRow::U128 { n, .. } => KIntC::InRange(*n),
                                        RawConstRow::U128OutOfRange { .. } => KIntC::AboveU128,
                                        _ => return None,
                                    };
                                    tree_stack.push(CoreTree::Leaf { n: k });
                                }
                                RawConstRow::Branch { fields, .. } => {
                                    if fields.len != 2 {
                                        return None;
                                    }
                                    let lcid = *constant_fields.get(fields.start.0)?;
                                    let rcid = *constant_fields.get(fields.start.0.checked_add(1)?)?;
                                    if lcid >= tcid || rcid >= tcid {
                                        return None;
                                    }
                                    tstack.push(TreeStep::FinishBranch);
                                    tstack.push(TreeStep::Enter(rcid));
                                    tstack.push(TreeStep::Enter(lcid));
                                }
                                _ => return None,
                            },
                            TreeStep::FinishLeaf => {}
                            TreeStep::FinishBranch => {
                                let right = tree_stack.pop()?;
                                let left = tree_stack.pop()?;
                                tree_stack.push(CoreTree::Branch {
                                    left: Box::new(left),
                                    right: Box::new(right),
                                });
                            }
                        }
                    }
                    let tree = tree_stack.pop()?;
                    val_stack.push(ExtValue::Core {
                        value: CoreValue::Tree { tree },
                    });
                }
                RawConstRow::Constructor {
                    type_id,
                    ctor_id,
                    fields,
                    ..
                } => {
                    stack.push(DecodeConstStep::BuildCtor {
                        type_id: type_id.0,
                        ctor_id: ctor_id.0,
                        arity: fields.len,
                    });
                    let f_end = fields.start.0.checked_add(fields.len)?;
                    let f_slice = constant_fields.get(fields.start.0..f_end)?;
                    for &fcid in f_slice.iter().rev() {
                        if fcid >= cid {
                            return None;
                        }
                        stack.push(DecodeConstStep::Visit(fcid));
                    }
                }
            },
            DecodeConstStep::BuildTreeLeaf | DecodeConstStep::BuildTreeBranch => return None,
            DecodeConstStep::BuildCtor {
                type_id,
                ctor_id,
                arity,
            } => {
                if val_stack.len() < arity {
                    return None;
                }
                let split_at = val_stack.len() - arity;
                let fields = val_stack.split_off(split_at);
                let ty = clone_type_expr_iterative(decoded_types.get(type_id)?);
                let crow = constructors.get(ctor_id)?;
                let ctor = names.get(crow.name_id)?.clone();
                val_stack.push(ExtValue::Constructor { ty, ctor, fields });
            }
        }
    }
    if val_stack.len() == 1 {
        val_stack.pop()
    } else {
        None
    }
}

fn decode_ext_program_from_view(v: &ProgramView) -> Option<ExtProgram> {
    let mut decoded_types: Vec<TypeExprSrc> = Vec::with_capacity(v.types.len());
    for (ti, trow) in v.types.iter().enumerate() {
        match ti {
            0 => decoded_types.push(TypeExprSrc::UInt),
            1 => decoded_types.push(TypeExprSrc::Bool),
            2 => decoded_types.push(TypeExprSrc::List),
            3 => decoded_types.push(TypeExprSrc::Tree),
            _ => {
                let name = v.names.get(trow.name_id)?.clone();
                if trow.args.len == 0 {
                    decoded_types.push(TypeExprSrc::Named { name });
                } else {
                    let a_end = trow.args.start.checked_add(trow.args.len)?;
                    let arg_ids = v.type_args.get(trow.args.start..a_end)?;
                    let mut args = Vec::with_capacity(arg_ids.len());
                    for arg_tid in arg_ids {
                        if arg_tid.0 >= ti {
                            return None;
                        }
                        args.push(clone_type_expr_iterative(decoded_types.get(arg_tid.0)?));
                    }
                    decoded_types.push(TypeExprSrc::Apply { name, args });
                }
            }
        }
    }

    let mut decoded_patterns: Vec<Option<ExtPattern>> = Vec::with_capacity(v.patterns.len());
    for (pi, pat) in v.patterns.iter().enumerate() {
        let ext_pat = match pat {
            PatternRowC::PVar { x } => ExtPattern::Dedicated {
                pattern: CorePattern::PVar {
                    x: v.names.get(*x)?.clone(),
                },
            },
            PatternRowC::PNil => ExtPattern::Dedicated {
                pattern: CorePattern::PNil,
            },
            PatternRowC::PCons { x, y } => ExtPattern::Dedicated {
                pattern: CorePattern::PCons {
                    x: v.names.get(*x)?.clone(),
                    y: v.names.get(*y)?.clone(),
                },
            },
            PatternRowC::PLeaf { x } => ExtPattern::Dedicated {
                pattern: CorePattern::PLeaf {
                    x: v.names.get(*x)?.clone(),
                },
            },
            PatternRowC::PBranch { x, y } => ExtPattern::Dedicated {
                pattern: CorePattern::PBranch {
                    x: v.names.get(*x)?.clone(),
                    y: v.names.get(*y)?.clone(),
                },
            },
            PatternRowC::Wildcard => ExtPattern::Wildcard,
            PatternRowC::Constructor {
                type_id,
                ctor_id,
                children,
            } => {
                let ty = clone_type_expr_iterative(decoded_types.get(type_id.0)?);
                let ctor = match (type_id.0, ctor_id.0) {
                    (2, 0) => "Nil".to_string(),
                    (2, 1) => "Cons".to_string(),
                    (3, 2) => "Leaf".to_string(),
                    (3, 3) => "Branch".to_string(),
                    _ => {
                        let crow = v.constructors.get(ctor_id.0)?;
                        v.names.get(crow.name_id)?.clone()
                    }
                };
                let c_end = children.start.checked_add(children.len)?;
                let c_ids = v.pattern_children.get(children.start..c_end)?;
                let mut fields = Vec::with_capacity(c_ids.len());
                for &cid in c_ids {
                    if cid >= pi {
                        return None;
                    }
                    fields.push(decoded_patterns.get_mut(cid)?.take()?);
                }
                ExtPattern::Ctor { ty, ctor, fields }
            }
        };
        decoded_patterns.push(Some(ext_pat));
    }

    let mut decoded_exprs: Vec<Option<ExtExpr>> = (0..v.expressions.len()).map(|_| None).collect();
    for ei in (0..v.expressions.len()).rev() {
        let expr = match &v.expressions[ei] {
            ExpressionRowC::Literal { constant } => {
                let value = decode_const_tree_iterative(
                    *constant,
                    &v.constants,
                    &v.constant_fields,
                    &decoded_types,
                    &v.constructors,
                    &v.names,
                )?;
                ExtExpr::Literal { value }
            }
            ExpressionRowC::Var { binder } => ExtExpr::Var {
                name: v.names.get(*binder)?.clone(),
            },
            ExpressionRowC::Let {
                binder,
                value,
                body,
            } => {
                if *value <= ei || *body <= ei {
                    return None;
                }
                let val_expr = decoded_exprs.get_mut(*value)?.take()?;
                let body_expr = decoded_exprs.get_mut(*body)?.take()?;
                ExtExpr::Let {
                    name: v.names.get(*binder)?.clone(),
                    value: Box::new(val_expr),
                    body: Box::new(body_expr),
                }
            }
            ExpressionRowC::Binary { op, left, right } => {
                if *left <= ei || *right <= ei {
                    return None;
                }
                let l_expr = decoded_exprs.get_mut(*left)?.take()?;
                let r_expr = decoded_exprs.get_mut(*right)?.take()?;
                ExtExpr::Binary {
                    op: *op,
                    left: Box::new(l_expr),
                    right: Box::new(r_expr),
                }
            }
            ExpressionRowC::Unary { op, operand } => {
                if *operand <= ei {
                    return None;
                }
                let op_expr = decoded_exprs.get_mut(*operand)?.take()?;
                ExtExpr::Unary {
                    op: *op,
                    operand: Box::new(op_expr),
                }
            }
            ExpressionRowC::If {
                cond,
                then_expr,
                else_expr,
            } => {
                if *cond <= ei || *then_expr <= ei || *else_expr <= ei {
                    return None;
                }
                let c_expr = decoded_exprs.get_mut(*cond)?.take()?;
                let t_expr = decoded_exprs.get_mut(*then_expr)?.take()?;
                let e_expr = decoded_exprs.get_mut(*else_expr)?.take()?;
                ExtExpr::If {
                    cond: Box::new(c_expr),
                    then_expr: Box::new(t_expr),
                    else_expr: Box::new(e_expr),
                }
            }
            ExpressionRowC::Call {
                function_name,
                args,
            } => {
                let a_end = args.start.checked_add(args.len)?;
                let arg_ids = v.children.get(args.start..a_end)?;
                let mut call_args = Vec::with_capacity(arg_ids.len());
                for &cid in arg_ids {
                    if cid <= ei {
                        return None;
                    }
                    call_args.push(decoded_exprs.get_mut(cid)?.take()?);
                }
                ExtExpr::Call {
                    name: v.names.get(*function_name)?.clone(),
                    args: call_args,
                }
            }
            ExpressionRowC::Construct {
                type_id,
                ctor_id,
                fields,
                ..
            } => {
                let ty = clone_type_expr_iterative(decoded_types.get(type_id.0)?);
                let crow = v.constructors.get(ctor_id.0)?;
                let ctor = v.names.get(crow.name_id)?.clone();
                let f_end = fields.start.checked_add(fields.len)?;
                let f_ids = v.children.get(fields.start..f_end)?;
                let mut field_exprs = Vec::with_capacity(f_ids.len());
                for &cid in f_ids {
                    if cid <= ei {
                        return None;
                    }
                    field_exprs.push(decoded_exprs.get_mut(cid)?.take()?);
                }
                ExtExpr::Construct {
                    ty,
                    ctor,
                    fields: field_exprs,
                }
            }
            ExpressionRowC::Match {
                scrutinee,
                arms,
                ..
            } => {
                if *scrutinee <= ei {
                    return None;
                }
                let scrut_expr = decoded_exprs.get_mut(*scrutinee)?.take()?;
                let a_end = arms.start.checked_add(arms.len)?;
                let arm_rows = v.arms.get(arms.start..a_end)?;
                let mut ext_arms = Vec::with_capacity(arm_rows.len());
                for arm_row in arm_rows {
                    if arm_row.body <= ei {
                        return None;
                    }
                    let pattern = decoded_patterns.get_mut(arm_row.pattern)?.take()?;
                    let body = decoded_exprs.get_mut(arm_row.body)?.take()?;
                    ext_arms.push(ExtArm { pattern, body });
                }
                ExtExpr::Match {
                    scrutinee: Box::new(scrut_expr),
                    arms: ext_arms,
                }
            }
        };
        decoded_exprs[ei] = Some(expr);
    }

    let entry = decoded_exprs.get_mut(v.entry)?.take()?;
    let mut functions = Vec::with_capacity(v.functions.len());
    for fn_row in &v.functions {
        let name = v.names.get(fn_row.function_name)?.clone();
        let p_end = fn_row.params.start.checked_add(fn_row.params.len)?;
        let param_rows = v.parameters.get(fn_row.params.start..p_end)?;
        let mut parameters = Vec::with_capacity(param_rows.len());
        for prow in param_rows {
            parameters.push(ExtParamSrc {
                name: v.names.get(prow.binder)?.clone(),
                ty: clone_type_expr_iterative(decoded_types.get(prow.sort.0)?),
            });
        }
        let return_type = clone_type_expr_iterative(decoded_types.get(fn_row.return_type.0)?);
        let body = decoded_exprs.get_mut(fn_row.body)?.take()?;
        functions.push(ExtFunction {
            name,
            parameters,
            return_type,
            body,
        });
    }

    if decoded_exprs.iter().any(Option::is_some) || decoded_patterns.iter().any(Option::is_some) {
        return None;
    }

    Some(ExtProgram {
        types: clone_type_decls_iterative(&v.decl_templates),
        assets: Vec::new(),
        states: Vec::new(),
        inputs: Vec::new(),
        functions,
        entry,
    })
}

fn const_row_eq(a: &RawConstRow, b: &RawConstRow) -> bool {
    match (a, b) {
        (
            RawConstRow::U128 {
                marker_free: mf1,
                n: n1,
            },
            RawConstRow::U128 {
                marker_free: mf2,
                n: n2,
            },
        ) => mf1 == mf2 && n1 == n2,
        (
            RawConstRow::Bool {
                marker_free: mf1,
                b: b1,
            },
            RawConstRow::Bool {
                marker_free: mf2,
                b: b2,
            },
        ) => mf1 == mf2 && b1 == b2,
        (RawConstRow::Nil { marker_free: mf1 }, RawConstRow::Nil { marker_free: mf2 }) => {
            mf1 == mf2
        }
        (
            RawConstRow::Cons {
                marker_free: mf1,
                fields: f1,
            },
            RawConstRow::Cons {
                marker_free: mf2,
                fields: f2,
            },
        )
        | (
            RawConstRow::Leaf {
                marker_free: mf1,
                fields: f1,
            },
            RawConstRow::Leaf {
                marker_free: mf2,
                fields: f2,
            },
        )
        | (
            RawConstRow::Branch {
                marker_free: mf1,
                fields: f1,
            },
            RawConstRow::Branch {
                marker_free: mf2,
                fields: f2,
            },
        ) => mf1 == mf2 && f1.start.0 == f2.start.0 && f1.len == f2.len,
        (
            RawConstRow::Constructor {
                marker_free: mf1,
                type_id: t1,
                ctor_id: c1,
                fields: f1,
            },
            RawConstRow::Constructor {
                marker_free: mf2,
                type_id: t2,
                ctor_id: c2,
                fields: f2,
            },
        ) => {
            mf1 == mf2
                && t1.0 == t2.0
                && c1.0 == c2.0
                && f1.start.0 == f2.start.0
                && f1.len == f2.len
        }
        (
            RawConstRow::U128OutOfRange { marker_free: mf1 },
            RawConstRow::U128OutOfRange { marker_free: mf2 },
        ) => mf1 == mf2,
        _ => false,
    }
}

fn expr_row_eq(a: &ExpressionRowC, b: &ExpressionRowC) -> bool {
    match (a, b) {
        (
            ExpressionRowC::Literal { constant: c1 },
            ExpressionRowC::Literal { constant: c2 },
        ) => c1 == c2,
        (ExpressionRowC::Var { binder: b1 }, ExpressionRowC::Var { binder: b2 }) => b1 == b2,
        (
            ExpressionRowC::Let {
                binder: b1,
                value: v1,
                body: bd1,
            },
            ExpressionRowC::Let {
                binder: b2,
                value: v2,
                body: bd2,
            },
        ) => b1 == b2 && v1 == v2 && bd1 == bd2,
        (
            ExpressionRowC::Binary {
                op: op1,
                left: l1,
                right: r1,
            },
            ExpressionRowC::Binary {
                op: op2,
                left: l2,
                right: r2,
            },
        ) => std::mem::discriminant(op1) == std::mem::discriminant(op2) && l1 == l2 && r1 == r2,
        (
            ExpressionRowC::Unary {
                op: op1,
                operand: o1,
            },
            ExpressionRowC::Unary {
                op: op2,
                operand: o2,
            },
        ) => std::mem::discriminant(op1) == std::mem::discriminant(op2) && o1 == o2,
        (
            ExpressionRowC::If {
                cond: c1,
                then_expr: t1,
                else_expr: e1,
            },
            ExpressionRowC::If {
                cond: c2,
                then_expr: t2,
                else_expr: e2,
            },
        ) => c1 == c2 && t1 == t2 && e1 == e2,
        (
            ExpressionRowC::Call {
                function_name: f1,
                args: a1,
            },
            ExpressionRowC::Call {
                function_name: f2,
                args: a2,
            },
        ) => f1 == f2 && a1 == a2,
        (
            ExpressionRowC::Match {
                scrutinee: s1,
                arms: a1,
                plan: p1,
            },
            ExpressionRowC::Match {
                scrutinee: s2,
                arms: a2,
                plan: p2,
            },
        ) => s1 == s2 && a1 == a2 && p1 == p2,
        (
            ExpressionRowC::Construct {
                type_id: t1,
                ctor_id: c1,
                fields: f1,
                constant: k1,
            },
            ExpressionRowC::Construct {
                type_id: t2,
                ctor_id: c2,
                fields: f2,
                constant: k2,
            },
        ) => t1 == t2 && c1 == c2 && f1 == f2 && k1 == k2,
        _ => false,
    }
}

fn type_expr_eq_iterative(a: &TypeExprSrc, b: &TypeExprSrc) -> bool {
    let mut stack = vec![(a, b)];
    while let Some((x, y)) = stack.pop() {
        match (x, y) {
            (TypeExprSrc::UInt, TypeExprSrc::UInt)
            | (TypeExprSrc::Bool, TypeExprSrc::Bool)
            | (TypeExprSrc::List, TypeExprSrc::List)
            | (TypeExprSrc::Tree, TypeExprSrc::Tree) => {}
            (TypeExprSrc::Named { name: n1 }, TypeExprSrc::Named { name: n2 })
            | (TypeExprSrc::Param { name: n1 }, TypeExprSrc::Param { name: n2 }) => {
                if n1 != n2 {
                    return false;
                }
            }
            (
                TypeExprSrc::Apply {
                    name: n1,
                    args: a1,
                },
                TypeExprSrc::Apply {
                    name: n2,
                    args: a2,
                },
            ) => {
                if n1 != n2 || a1.len() != a2.len() {
                    return false;
                }
                for (u, v) in a1.iter().zip(a2.iter()) {
                    stack.push((u, v));
                }
            }
            _ => return false,
        }
    }
    true
}

fn decl_templates_eq(a: &[TypeDeclSrc], b: &[TypeDeclSrc]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    for (d1, d2) in a.iter().zip(b.iter()) {
        if d1.name != d2.name
            || d1.parameters != d2.parameters
            || d1.ctors.len() != d2.ctors.len()
        {
            return false;
        }
        for (c1, c2) in d1.ctors.iter().zip(d2.ctors.iter()) {
            if c1.name != c2.name || c1.fields.len() != c2.fields.len() {
                return false;
            }
            for (f1, f2) in c1.fields.iter().zip(c2.fields.iter()) {
                if f1.name != f2.name || !type_expr_eq_iterative(&f1.ty, &f2.ty) {
                    return false;
                }
            }
        }
    }
    true
}

fn program_view_non_plan_eq(a: &ProgramView, b: &ProgramView) -> bool {
    a.entry == b.entry
        && a.expressions.len() == b.expressions.len()
        && a.expressions
            .iter()
            .zip(b.expressions.iter())
            .all(|(x, y)| expr_row_eq(x, y))
        && a.occurrences == b.occurrences
        && a.occurrence_paths == b.occurrence_paths
        && a.children == b.children
        && a.functions == b.functions
        && a.parameters == b.parameters
        && a.arms == b.arms
        && a.patterns == b.patterns
        && a.pattern_children == b.pattern_children
        && a.types.len() == b.types.len()
        && a.types.iter().zip(b.types.iter()).all(|(x, y)| {
            x.name_id == y.name_id
                && x.decl == y.decl
                && x.args == y.args
                && x.ctor_start == y.ctor_start
                && x.ctor_len == y.ctor_len
        })
        && a.type_args == b.type_args
        && a.constructors.len() == b.constructors.len()
        && a.constructors
            .iter()
            .zip(b.constructors.iter())
            .all(|(x, y)| {
                x.owning_type == y.owning_type
                    && x.name_id == y.name_id
                    && x.source_order == y.source_order
                    && x.field_start == y.field_start
                    && x.field_len == y.field_len
            })
        && a.ctor_fields.len() == b.ctor_fields.len()
        && a.ctor_fields
            .iter()
            .zip(b.ctor_fields.iter())
            .all(|(x, y)| x.name_id == y.name_id && x.type_id == y.type_id)
        && a.decl_param_counts == b.decl_param_counts
        && a.decl_ctor_counts == b.decl_ctor_counts
        && decl_templates_eq(&a.decl_templates, &b.decl_templates)
        && a.constants.len() == b.constants.len()
        && a.constants
            .iter()
            .zip(b.constants.iter())
            .all(|(x, y)| const_row_eq(x, y))
        && a.constant_fields == b.constant_fields
        && a.names == b.names
}

pub fn check_program_view_ext(
    v: &ProgramView,
    report: Option<&PreparationReportC>,
    domain: ExtDomain,
    bounds: &BoundsC,
    layout: &LayoutSchemaC,
) -> Result<(), ExtTypeBuildError> {
    let ext_caps = ExtCapacitiesC::effective_for_extended_domain(layout.ext_capacities);

    // V1: table bounds, capacities, and sorted unique non-empty names.
    if v.expressions.is_empty()
        || v.expressions.len() > bounds.code_rows
        || v.occurrences.len() != v.expressions.len()
        || v.occurrence_paths.len() > layout.capacities.occurrence_path_slots
        || v.children.len() > bounds.code_child_slots
        || v.functions.len() > bounds.function_rows
        || v.parameters.len() > bounds.parameter_slots
        || v.arms.len() > bounds.match_arm_rows
        || v.patterns.len() > bounds.pattern_rows
        || v.pattern_children.len() > ext_caps.pattern_child_slots
        || v.types.len() < 4
        || v.types.len() > bounds.type_rows
        || v.type_args.len() > bounds.type_rows
        || v.constructors.len() < 4
        || v.constructors.len() > bounds.constructor_rows
        || v.ctor_fields.len() < 5
        || v.ctor_fields.len() > bounds.constructor_field_slots
        || v.plan_nodes.len() > bounds.match_plan_rows
        || v.plan_cases.len() > ext_caps.plan_case_slots
        || v.field_paths.len() > ext_caps.field_path_slots
        || v.constants.len() > bounds.constant_nodes
        || v.constant_fields.len() > bounds.constant_field_slots
        || v.names.len() > layout.capacities.name_rows
    {
        return Err(pv_malformed("V1"));
    }
    let mut total_name_bytes = 0usize;
    for (ni, nm) in v.names.iter().enumerate() {
        if nm.is_empty() {
            return Err(pv_malformed("V1"));
        }
        if ni > 0 && v.names[ni - 1].as_str() >= nm.as_str() {
            return Err(pv_malformed("V1"));
        }
        total_name_bytes = total_name_bytes
            .checked_add(nm.len())
            .ok_or_else(|| pv_malformed("V1"))?;
    }
    if total_name_bytes > layout.capacities.name_bytes {
        return Err(pv_malformed("V1"));
    }

    // V2: expression forest, contiguous child/arm/occurrence slices.
    if v.entry != 0 {
        return Err(pv_malformed("V2"));
    }
    let n_expr = v.expressions.len();
    let mut parent: Vec<Option<(usize, usize)>> = vec![None; n_expr];
    let mut expected_children_cursor = 0usize;
    let mut expected_arms_cursor = 0usize;
    for (ei, erow) in v.expressions.iter().enumerate() {
        let mut kids: Vec<(usize, usize)> = Vec::new();
        match erow {
            ExpressionRowC::Literal { constant } => {
                if *constant >= v.constants.len() {
                    return Err(pv_malformed("V2"));
                }
            }
            ExpressionRowC::Var { binder } => {
                if *binder >= v.names.len() {
                    return Err(pv_malformed("V2"));
                }
            }
            ExpressionRowC::Let {
                binder,
                value,
                body,
            } => {
                if *binder >= v.names.len() {
                    return Err(pv_malformed("V2"));
                }
                kids.push((*value, 0));
                kids.push((*body, 1));
            }
            ExpressionRowC::Binary { left, right, .. } => {
                kids.push((*left, 0));
                kids.push((*right, 1));
            }
            ExpressionRowC::Unary { operand, .. } => {
                kids.push((*operand, 0));
            }
            ExpressionRowC::If {
                cond,
                then_expr,
                else_expr,
            } => {
                kids.push((*cond, 0));
                kids.push((*then_expr, 1));
                kids.push((*else_expr, 2));
            }
            ExpressionRowC::Call {
                function_name,
                args,
            } => {
                if *function_name >= v.names.len() || args.len > bounds.max_arity {
                    return Err(pv_malformed("V2"));
                }
                if args.len == 0 {
                    if args.start != 0 {
                        return Err(pv_malformed("V2"));
                    }
                } else {
                    if args.start != expected_children_cursor {
                        return Err(pv_malformed("V2"));
                    }
                    let a_end = args
                        .start
                        .checked_add(args.len)
                        .ok_or_else(|| pv_malformed("V2"))?;
                    if a_end > v.children.len() {
                        return Err(pv_malformed("V2"));
                    }
                    expected_children_cursor = a_end;
                    for (k, pos) in (args.start..a_end).enumerate() {
                        kids.push((v.children[pos], k));
                    }
                }
            }
            ExpressionRowC::Construct { fields, .. } => {
                if fields.len > bounds.max_arity {
                    return Err(pv_malformed("V2"));
                }
                if fields.len == 0 {
                    if fields.start != 0 {
                        return Err(pv_malformed("V2"));
                    }
                } else {
                    if fields.start != expected_children_cursor {
                        return Err(pv_malformed("V2"));
                    }
                    let f_end = fields
                        .start
                        .checked_add(fields.len)
                        .ok_or_else(|| pv_malformed("V2"))?;
                    if f_end > v.children.len() {
                        return Err(pv_malformed("V2"));
                    }
                    expected_children_cursor = f_end;
                    for (k, pos) in (fields.start..f_end).enumerate() {
                        kids.push((v.children[pos], k));
                    }
                }
            }
            ExpressionRowC::Match {
                scrutinee,
                arms,
                ..
            } => {
                if arms.len == 0
                    || arms.len > bounds.match_arm_rows
                    || arms.start != expected_arms_cursor
                {
                    return Err(pv_malformed("V2"));
                }
                let a_end = arms
                    .start
                    .checked_add(arms.len)
                    .ok_or_else(|| pv_malformed("V2"))?;
                if a_end > v.arms.len() {
                    return Err(pv_malformed("V2"));
                }
                expected_arms_cursor = a_end;
                kids.push((*scrutinee, 0));
                for (k, aid) in (arms.start..a_end).enumerate() {
                    kids.push((v.arms[aid].body, k + 1));
                }
            }
        }
        for (c, ord) in kids {
            if c <= ei || c >= n_expr || parent[c].is_some() {
                return Err(pv_malformed("V2"));
            }
            parent[c] = Some((ei, ord));
        }
    }
    if expected_children_cursor != v.children.len() || expected_arms_cursor != v.arms.len() {
        return Err(pv_malformed("V2"));
    }

    // V3 & V4: functions and parameters.
    let mut expected_param_cursor = 0usize;
    let mut seen_fn_names = BTreeSet::new();
    for (fi, frow) in v.functions.iter().enumerate() {
        if frow.function_name >= v.names.len()
            || !seen_fn_names.insert(frow.function_name)
            || frow.return_type.0 >= v.types.len()
            || frow.body >= n_expr
            || parent[frow.body].is_some()
            || frow.params.len > bounds.max_arity
        {
            return Err(pv_malformed("V3"));
        }
        if fi > 0 && frow.body <= v.functions[fi - 1].body {
            return Err(pv_malformed("V3"));
        }
        if frow.params.len == 0 {
            if frow.params.start != 0 {
                return Err(pv_malformed("V3"));
            }
        } else {
            if frow.params.start != expected_param_cursor {
                return Err(pv_malformed("V3"));
            }
            let p_end = frow
                .params
                .start
                .checked_add(frow.params.len)
                .ok_or_else(|| pv_malformed("V3"))?;
            if p_end > v.parameters.len() {
                return Err(pv_malformed("V3"));
            }
            expected_param_cursor = p_end;
            let mut seen_params = BTreeSet::new();
            for pos in frow.params.start..p_end {
                let prow = &v.parameters[pos];
                if prow.binder >= v.names.len()
                    || prow.sort.0 >= v.types.len()
                    || !seen_params.insert(prow.binder)
                {
                    return Err(pv_malformed("V4"));
                }
            }
        }
    }
    if expected_param_cursor != v.parameters.len() {
        return Err(pv_malformed("V3"));
    }

    // Verify occurrence rows and paths against parent chain.
    let mut expected_occ_path_cursor = 0usize;
    for ei in 0..n_expr {
        let mut rev_path = Vec::new();
        let mut cur = ei;
        while let Some((p, ord)) = parent[cur] {
            rev_path.push(ord);
            cur = p;
        }
        rev_path.reverse();
        let occ = &v.occurrences[ei];
        if cur == 0 {
            if occ.owner_is_function || occ.function != 0 {
                return Err(pv_malformed("V2"));
            }
        } else {
            let Some(fn_idx) = v.functions.iter().position(|f| f.body == cur) else {
                return Err(pv_malformed("V2"));
            };
            if !occ.owner_is_function || occ.function != fn_idx {
                return Err(pv_malformed("V2"));
            }
        }
        if rev_path.is_empty() {
            if occ.path.start != 0 || occ.path.len != 0 {
                return Err(pv_malformed("V2"));
            }
        } else {
            if occ.path.start != expected_occ_path_cursor || occ.path.len != rev_path.len() {
                return Err(pv_malformed("V2"));
            }
            let o_end = occ
                .path
                .start
                .checked_add(occ.path.len)
                .ok_or_else(|| pv_malformed("V2"))?;
            if v.occurrence_paths.get(occ.path.start..o_end) != Some(rev_path.as_slice()) {
                return Err(pv_malformed("V2"));
            }
            expected_occ_path_cursor = o_end;
        }
    }
    if expected_occ_path_cursor != v.occurrence_paths.len() {
        return Err(pv_malformed("V2"));
    }

    // V16: patterns (post-order blocks per arm, dedicated vs planned).
    let mut expected_pat_cursor = 0usize;
    let mut expected_pat_child_cursor = 0usize;
    for erow in &v.expressions {
        let ExpressionRowC::Match { arms, .. } = erow else {
            continue;
        };
        let a_end = arms
            .start
            .checked_add(arms.len)
            .ok_or_else(|| pv_malformed("V16"))?;
        let arm_rows = v
            .arms
            .get(arms.start..a_end)
            .ok_or_else(|| pv_malformed("V16"))?;
        let mut is_planned = false;
        for arm_row in arm_rows {
            let root_pat = v
                .patterns
                .get(arm_row.pattern)
                .ok_or_else(|| pv_malformed("V16"))?;
            if matches!(
                root_pat,
                PatternRowC::Wildcard | PatternRowC::Constructor { .. }
            ) {
                is_planned = true;
            }
        }
        for arm_row in arm_rows {
            let root_id = arm_row.pattern;
            if root_id < expected_pat_cursor || root_id >= v.patterns.len() {
                return Err(pv_malformed("V16"));
            }
            if !is_planned && root_id != expected_pat_cursor {
                return Err(pv_malformed("V16"));
            }
            let mut has_pat_parent = vec![false; root_id - expected_pat_cursor + 1];
            let mut planned_binders = BTreeSet::new();
            for pi in expected_pat_cursor..=root_id {
                match &v.patterns[pi] {
                    PatternRowC::PVar { x } | PatternRowC::PLeaf { x } => {
                        if *x >= v.names.len() || (is_planned && !planned_binders.insert(*x)) {
                            return Err(pv_malformed("V16"));
                        }
                    }
                    PatternRowC::PCons { x, y } | PatternRowC::PBranch { x, y } => {
                        if *x >= v.names.len()
                            || *y >= v.names.len()
                            || (is_planned
                                && (!planned_binders.insert(*x) || !planned_binders.insert(*y)))
                        {
                            return Err(pv_malformed("V16"));
                        }
                    }
                    PatternRowC::PNil => {}
                    PatternRowC::Wildcard => {
                        if !is_planned {
                            return Err(pv_malformed("V16"));
                        }
                    }
                    PatternRowC::Constructor {
                        type_id,
                        ctor_id,
                        children,
                    } => {
                        if !is_planned || type_id.0 >= v.types.len() {
                            return Err(pv_malformed("V16"));
                        }
                        let c_row = checked_type_ctor(v, *type_id, *ctor_id)
                            .ok_or_else(|| pv_malformed("V16"))?;
                        if children.len != c_row.field_len {
                            return Err(pv_malformed("V16"));
                        }
                        if children.len == 0 {
                            if children.start != 0 {
                                return Err(pv_malformed("V16"));
                            }
                        } else {
                            if children.start != expected_pat_child_cursor {
                                return Err(pv_malformed("V16"));
                            }
                            let c_end = children
                                .start
                                .checked_add(children.len)
                                .ok_or_else(|| pv_malformed("V16"))?;
                            if c_end > v.pattern_children.len() {
                                return Err(pv_malformed("V16"));
                            }
                            expected_pat_child_cursor = c_end;
                            for pos in children.start..c_end {
                                let child_pi = v.pattern_children[pos];
                                if child_pi < expected_pat_cursor
                                    || child_pi >= pi
                                    || has_pat_parent[child_pi - expected_pat_cursor]
                                {
                                    return Err(pv_malformed("V16"));
                                }
                                has_pat_parent[child_pi - expected_pat_cursor] = true;
                            }
                        }
                    }
                }
            }
            if has_pat_parent[root_id - expected_pat_cursor]
                || has_pat_parent[..root_id - expected_pat_cursor]
                    .iter()
                    .any(|&p| !p)
            {
                return Err(pv_malformed("V16"));
            }
            expected_pat_cursor = root_id + 1;
        }
    }
    if expected_pat_cursor != v.patterns.len()
        || expected_pat_child_cursor != v.pattern_children.len()
    {
        return Err(pv_malformed("V16"));
    }

    // V6 & V11: constants and ConstRow.Constructor.
    let mut expected_const_field_cursor = 0usize;
    for (ci, crow) in v.constants.iter().enumerate() {
        let fslice = const_row_fields_slice(crow);
        let mut children_mf = true;
        if fslice.len == 0 {
            if fslice.start != 0 {
                return Err(pv_malformed("V6"));
            }
        } else {
            if fslice.start != expected_const_field_cursor {
                return Err(pv_malformed("V6"));
            }
            let f_end = fslice
                .start
                .checked_add(fslice.len)
                .ok_or_else(|| pv_malformed("V6"))?;
            if f_end > v.constant_fields.len() {
                return Err(pv_malformed("V6"));
            }
            expected_const_field_cursor = f_end;
            for pos in fslice.start..f_end {
                let child_cid = v.constant_fields[pos];
                if child_cid >= ci {
                    return Err(pv_malformed("V6"));
                }
                children_mf &= const_row_marker_free(&v.constants[child_cid]);
            }
        }
        let expected_mf = !matches!(crow, RawConstRow::U128OutOfRange { .. }) && children_mf;
        if const_row_marker_free(crow) != expected_mf {
            return Err(pv_malformed("V6"));
        }
        if let RawConstRow::Constructor {
            type_id,
            ctor_id,
            fields,
            ..
        } = crow
        {
            if type_id.0 < 4 || type_id.0 >= v.types.len() {
                return Err(pv_malformed("V11"));
            }
            let c_row = checked_type_ctor(v, TypeIdC(type_id.0), CtorIdC(ctor_id.0))
                .ok_or_else(|| pv_malformed("V11"))?;
            if fields.len != c_row.field_len {
                return Err(pv_malformed("V11"));
            }
            if fields.len > 0 {
                let k_end = fields
                    .start
                    .0
                    .checked_add(fields.len)
                    .ok_or_else(|| pv_malformed("V11"))?;
                let child_cids = v
                    .constant_fields
                    .get(fields.start.0..k_end)
                    .ok_or_else(|| pv_malformed("V11"))?;
                let cf_end = c_row
                    .field_start
                    .checked_add(c_row.field_len)
                    .ok_or_else(|| pv_malformed("V11"))?;
                let field_rows = v
                    .ctor_fields
                    .get(c_row.field_start..cf_end)
                    .ok_or_else(|| pv_malformed("V11"))?;
                for (&child_cid, field_row) in child_cids.iter().zip(field_rows) {
                    let child_row = v
                        .constants
                        .get(child_cid)
                        .ok_or_else(|| pv_malformed("V11"))?;
                    if const_row_type_id(child_row) != field_row.type_id {
                        return Err(pv_malformed("V11"));
                    }
                }
            }
        }
    }
    if expected_const_field_cursor != v.constant_fields.len() {
        return Err(pv_malformed("V6"));
    }

    // V9 & V10: ADT tables.
    if check_generic_program_view(v).is_err() {
        return Err(pv_malformed("V9"));
    }
    for ti in 0..4 {
        if v.types[ti].name_id != 0 {
            return Err(pv_malformed("V9"));
        }
    }
    let expected_builtin_ctors = [
        (2usize, 0usize, 0usize, 0usize),
        (2usize, 1usize, 0usize, 2usize),
        (3usize, 0usize, 2usize, 1usize),
        (3usize, 1usize, 3usize, 2usize),
    ];
    for (ci, (ot, so, fs, fl)) in expected_builtin_ctors.into_iter().enumerate() {
        let c = &v.constructors[ci];
        if c.owning_type != TypeIdC(ot)
            || c.name_id != 0
            || c.source_order != so
            || c.field_start != fs
            || c.field_len != fl
        {
            return Err(pv_malformed("V9"));
        }
    }
    let expected_builtin_fields = [0usize, 2usize, 0usize, 3usize, 3usize];
    for (fi, fty) in expected_builtin_fields.into_iter().enumerate() {
        if v.ctor_fields[fi].name_id != 0 || v.ctor_fields[fi].type_id != TypeIdC(fty) {
            return Err(pv_malformed("V9"));
        }
    }
    let non_param_decls: Vec<usize> = v
        .decl_templates
        .iter()
        .enumerate()
        .filter(|(_, d)| d.parameters.is_empty())
        .map(|(i, _)| i)
        .collect();
    if v.types.len() < 4 + non_param_decls.len() {
        return Err(pv_malformed("V9"));
    }
    for (offset, &d_idx) in non_param_decls.iter().enumerate() {
        let trow = &v.types[4 + offset];
        if trow.decl != Some(d_idx) || trow.args.start != 0 || trow.args.len != 0 {
            return Err(pv_malformed("V9"));
        }
    }
    let mut expected_type_arg_cursor = 0usize;
    let mut expected_ctor_cursor = 4usize;
    let mut expected_ctor_field_cursor = 5usize;
    let mut seen_instances: BTreeSet<(usize, Vec<usize>)> = BTreeSet::new();
    for (ti, trow) in v.types.iter().enumerate().skip(4) {
        let Some(d_idx) = trow.decl else {
            return Err(pv_malformed("V9"));
        };
        let decl = v.decl_templates.get(d_idx).ok_or_else(|| pv_malformed("V9"))?;
        if v.names.get(trow.name_id) != Some(&decl.name)
            || trow.args.len != decl.parameters.len()
            || trow.ctor_start.0 != expected_ctor_cursor
            || trow.ctor_len != decl.ctors.len()
        {
            return Err(pv_malformed("V9"));
        }
        expected_ctor_cursor = expected_ctor_cursor
            .checked_add(trow.ctor_len)
            .ok_or_else(|| pv_malformed("V9"))?;
        let inst_args: &[TypeIdC] = if ti < 4 + non_param_decls.len() {
            &[]
        } else {
            if trow.args.len == 0 || trow.args.start != expected_type_arg_cursor {
                return Err(pv_malformed("V9"));
            }
            let a_end = trow
                .args
                .start
                .checked_add(trow.args.len)
                .ok_or_else(|| pv_malformed("V9"))?;
            expected_type_arg_cursor = a_end;
            let slice = v
                .type_args
                .get(trow.args.start..a_end)
                .ok_or_else(|| pv_malformed("V9"))?;
            if slice.iter().any(|a| a.0 >= ti)
                || !seen_instances.insert((d_idx, slice.iter().map(|a| a.0).collect()))
            {
                return Err(pv_malformed("V9"));
            }
            slice
        };
        for (ord, c_decl) in decl.ctors.iter().enumerate() {
            let cid = trow
                .ctor_start
                .0
                .checked_add(ord)
                .ok_or_else(|| pv_malformed("V9"))?;
            let crow = v.constructors.get(cid).ok_or_else(|| pv_malformed("V9"))?;
            if v.names.get(crow.name_id) != Some(&c_decl.name)
                || crow.field_len != c_decl.fields.len()
                || crow.field_len > bounds.max_arity
                || (crow.field_len > 0 && crow.field_start != expected_ctor_field_cursor)
            {
                return Err(pv_malformed("V9"));
            }
            expected_ctor_field_cursor = expected_ctor_field_cursor
                .checked_add(crow.field_len)
                .ok_or_else(|| pv_malformed("V9"))?;
            for (f_idx, f_decl) in c_decl.fields.iter().enumerate() {
                let cf_pos = crow
                    .field_start
                    .checked_add(f_idx)
                    .ok_or_else(|| pv_malformed("V9"))?;
                let cf = v.ctor_fields.get(cf_pos).ok_or_else(|| pv_malformed("V9"))?;
                if v.names.get(cf.name_id) != Some(&f_decl.name) {
                    return Err(pv_malformed("V9"));
                }
                let resolved = resolve_template_type_in_view(
                    &f_decl.ty,
                    &decl.parameters,
                    inst_args,
                    v,
                )
                .ok_or_else(|| pv_malformed("V10"))?;
                if cf.type_id != resolved {
                    return Err(pv_malformed("V10"));
                }
            }
        }
    }
    if expected_type_arg_cursor != v.type_args.len()
        || expected_ctor_cursor != v.constructors.len()
        || expected_ctor_field_cursor != v.ctor_fields.len()
    {
        return Err(pv_malformed("V9"));
    }

    // V12: Construct rows and unique nullary constant ownership.
    let mut referenced_consts = BTreeSet::new();
    for erow in &v.expressions {
        match erow {
            ExpressionRowC::Literal { constant } => {
                if !referenced_consts.insert(*constant) {
                    return Err(pv_malformed("V12"));
                }
            }
            ExpressionRowC::Construct {
                type_id,
                ctor_id,
                fields,
                constant,
            } => {
                if type_id.0 < 4 || type_id.0 >= v.types.len() {
                    return Err(pv_malformed("V12"));
                }
                let c_row = checked_type_ctor(v, *type_id, *ctor_id)
                    .ok_or_else(|| pv_malformed("V12"))?;
                if fields.len != c_row.field_len || (fields.len == 0) != constant.is_some() {
                    return Err(pv_malformed("V12"));
                }
                if let Some(cid) = constant {
                    // The owned nullary row must be exactly ConstRow.Constructor with
                    // this row's typeId and ctorId and fields (0, 0), owned only here.
                    let owned_row_matches = matches!(
                        v.constants.get(*cid),
                        Some(RawConstRow::Constructor {
                            type_id: owned_type,
                            ctor_id: owned_ctor,
                            fields: owned_fields,
                            ..
                        }) if owned_type.0 == type_id.0
                            && owned_ctor.0 == ctor_id.0
                            && owned_fields.start.0 == 0
                            && owned_fields.len == 0
                    );
                    if !owned_row_matches || !referenced_consts.insert(*cid) {
                        return Err(pv_malformed("V12"));
                    }
                }
            }
            _ => {}
        }
    }

    // V13: plans.
    let mut expected_plan_node_cursor = 0usize;
    let mut expected_plan_case_cursor = 0usize;
    let mut expected_field_path_cursor = 0usize;
    for (ei, erow) in v.expressions.iter().enumerate() {
        let ExpressionRowC::Match { arms, plan, .. } = erow else {
            continue;
        };
        let a_end = arms
            .start
            .checked_add(arms.len)
            .ok_or_else(|| pv_malformed("V13"))?;
        let arm_rows = v
            .arms
            .get(arms.start..a_end)
            .ok_or_else(|| pv_malformed("V13"))?;
        let mut is_planned = false;
        for arm_row in arm_rows {
            let root_pat = v
                .patterns
                .get(arm_row.pattern)
                .ok_or_else(|| pv_malformed("V13"))?;
            if matches!(
                root_pat,
                PatternRowC::Wildcard | PatternRowC::Constructor { .. }
            ) {
                is_planned = true;
            }
        }
        if !is_planned {
            if plan.is_some() {
                return Err(pv_malformed("V13"));
            }
            continue;
        }
        let Some(root) = *plan else {
            return Err(pv_malformed("V13"));
        };
        if root != expected_plan_node_cursor || root >= v.plan_nodes.len() {
            return Err(pv_malformed("V13"));
        }
        let mut block_end = root;
        while block_end < v.plan_nodes.len() {
            let sm = match v.plan_nodes[block_end] {
                PlanNodeC::Switch { source_match, .. }
                | PlanNodeC::Bind { source_match, .. }
                | PlanNodeC::Select { source_match, .. }
                | PlanNodeC::NoMatch { source_match } => source_match,
            };
            if sm != ei {
                break;
            }
            block_end += 1;
        }
        // Match.plan names the first node of this Match's block, so the root's
        // source_match must be this Match: an empty block is malformed (N2).
        if block_end == root {
            return Err(pv_malformed("V13"));
        }
        let mut has_parent = vec![false; block_end - root];
        for ni in root..block_end {
            match v.plan_nodes[ni] {
                PlanNodeC::Switch {
                    path,
                    type_id,
                    cases,
                    default,
                    ..
                } => {
                    if path.len == 0 {
                        if path.start != 0 {
                            return Err(pv_malformed("V13"));
                        }
                    } else {
                        if path.start != expected_field_path_cursor {
                            return Err(pv_malformed("V13"));
                        }
                        let p_end = path
                            .start
                            .checked_add(path.len)
                            .ok_or_else(|| pv_malformed("V13"))?;
                        if p_end > v.field_paths.len()
                            || v.field_paths[path.start..p_end]
                                .iter()
                                .any(|&fp| fp >= bounds.max_arity)
                        {
                            return Err(pv_malformed("V13"));
                        }
                        expected_field_path_cursor = p_end;
                    }
                    if type_id.0 >= v.types.len()
                        || cases.len != v.types[type_id.0].ctor_len
                        || cases.len == 0
                        || cases.start != expected_plan_case_cursor
                    {
                        return Err(pv_malformed("V13"));
                    }
                    let c_end = cases
                        .start
                        .checked_add(cases.len)
                        .ok_or_else(|| pv_malformed("V13"))?;
                    if c_end > v.plan_cases.len() {
                        return Err(pv_malformed("V13"));
                    }
                    expected_plan_case_cursor = c_end;
                    for cpos in cases.start..c_end {
                        if let Some(target) = v.plan_cases[cpos] {
                            if target <= ni || target >= block_end || has_parent[target - root] {
                                return Err(pv_malformed("V13"));
                            }
                            has_parent[target - root] = true;
                        }
                    }
                    if default <= ni || default >= block_end || has_parent[default - root] {
                        return Err(pv_malformed("V13"));
                    }
                    has_parent[default - root] = true;
                }
                PlanNodeC::Bind {
                    path, binder, next, ..
                } => {
                    if path.len == 0 {
                        if path.start != 0 {
                            return Err(pv_malformed("V13"));
                        }
                    } else {
                        if path.start != expected_field_path_cursor {
                            return Err(pv_malformed("V13"));
                        }
                        let p_end = path
                            .start
                            .checked_add(path.len)
                            .ok_or_else(|| pv_malformed("V13"))?;
                        if p_end > v.field_paths.len()
                            || v.field_paths[path.start..p_end]
                                .iter()
                                .any(|&fp| fp >= bounds.max_arity)
                        {
                            return Err(pv_malformed("V13"));
                        }
                        expected_field_path_cursor = p_end;
                    }
                    if binder >= v.names.len()
                        || next <= ni
                        || next >= block_end
                        || has_parent[next - root]
                    {
                        return Err(pv_malformed("V13"));
                    }
                    has_parent[next - root] = true;
                }
                PlanNodeC::Select { arm, .. } => {
                    if arm < arms.start || arm >= a_end {
                        return Err(pv_malformed("V13"));
                    }
                }
                PlanNodeC::NoMatch { .. } => {}
            }
        }
        let Some((&root_has_parent, non_root)) = has_parent.split_first() else {
            return Err(pv_malformed("V13"));
        };
        if root_has_parent || non_root.iter().any(|&p| !p) {
            return Err(pv_malformed("V13"));
        }
        expected_plan_node_cursor = block_end;
    }
    if expected_plan_node_cursor != v.plan_nodes.len()
        || expected_plan_case_cursor != v.plan_cases.len()
        || expected_field_path_cursor != v.field_paths.len()
    {
        return Err(pv_malformed("V13"));
    }

    // V14: decode ProgramView to ExtProgram and re-run prepare_ext_program.
    let decoded = decode_ext_program_from_view(v).ok_or_else(|| pv_malformed("V14"))?;
    let (reprepared_view, reprepared_report) =
        prepare_ext_program(decoded, domain, bounds, layout, None)
            .map_err(|_| pv_malformed("V14"))?;
    if !program_view_non_plan_eq(v, &reprepared_view) {
        return Err(pv_malformed("V14"));
    }
    if v.plan_nodes != reprepared_view.plan_nodes
        || v.plan_cases != reprepared_view.plan_cases
        || v.field_paths != reprepared_view.field_paths
    {
        return Err(pv_malformed("V13"));
    }
    if let Some(rep) = report {
        if rep != &reprepared_report {
            return Err(pv_malformed("V14"));
        }
    }
    Ok(())
}

pub fn prepare_ext_program(
    program: ExtProgram,
    domain: ExtDomain,
    bounds: &BoundsC,
    layout: &LayoutSchemaC,
    locator: Option<LocatorC>,
) -> Result<(ProgramView, PreparationReportC), ExtTypeBuildError> {
    let mut effective_layout = layout.clone();
    effective_layout.bounds = *bounds;
    let ext_caps = ExtCapacitiesC::effective_for_extended_domain(layout.ext_capacities);
    let prepared = prepare_ext_program_types(program, &effective_layout, domain, locator)?;
    prepare_ext_c1_to_p(&prepared, bounds, &effective_layout, ext_caps)
}

pub fn admit_ext_program(
    program: ExtProgram,
    domain: ExtDomain,
    bounds: &BoundsC,
    layout: &LayoutSchemaC,
    locator: Option<LocatorC>,
) -> Result<(Admitted, PreparationReportC), ExtTypeBuildError> {
    // AD-0a: layout.bounds == bounds.
    if layout.bounds != *bounds {
        return Err(ExtTypeBuildError::CoreTooling(Tooling::AdmissionRejected {
            reason: AdmissionReason::LayoutBoundsMismatch,
            detail: "layout.bounds != bounds".to_string(),
            locator: field_loc("layout.bounds"),
        }));
    }
    // AD-1: stepCap >= 1.
    if bounds.step_cap < 1 {
        return Err(ExtTypeBuildError::CoreTooling(
            Tooling::CapacityNotAdmitted {
                kind: CapacityKind::StepCap,
                needed: NeededNatC::Exact(1),
                limit: bounds.step_cap,
                locator: field_loc("bounds.stepCap"),
            },
        ));
    }
    let ext_caps = ExtCapacitiesC::effective_for_extended_domain(layout.ext_capacities);
    // AD-2a & AD-2b: widths.
    if let Some(w) = &layout.widths {
        let checks: [(CounterWidthField, u128, &str); 3] = [
            (
                CounterWidthField::WorkCounter,
                bitlen_count(bounds.max_reference_work),
                "WorkCounter",
            ),
            (
                CounterWidthField::HelperCounter,
                bitlen_count(bounds.max_helper_work),
                "HelperCounter",
            ),
            (
                CounterWidthField::ReferenceCursor,
                bitlen_count(bounds.max_reference_work),
                "ReferenceCursor",
            ),
        ];
        for (field, needed, nm) in checks {
            let limit = w.counter_width(&field) as u128;
            if limit < needed {
                return Err(ExtTypeBuildError::CoreTooling(
                    Tooling::CapacityNotAdmitted {
                        kind: CapacityKind::CounterWidth { field },
                        needed: NeededNatC::Exact(needed),
                        limit,
                        locator: field_loc(&format!("layout.widths.counterWidths.{nm}")),
                    },
                ));
            }
        }
        let idx: [(&str, u128); 29] = [
            ("ExprId", bitlen_id(bounds.code_rows as u128)),
            ("ChildPos", bitlen_count(bounds.code_child_slots as u128)),
            ("FunctionRowId", bitlen_id(bounds.function_rows as u128)),
            ("FunctionCursor", bitlen_count(bounds.function_rows as u128)),
            ("ParamPos", bitlen_count(bounds.parameter_slots as u128)),
            ("ArmId", bitlen_id(bounds.match_arm_rows as u128)),
            ("ArmPos", bitlen_count(bounds.match_arm_rows as u128)),
            ("PatternRowId", bitlen_id(bounds.pattern_rows as u128)),
            ("ConstId", bitlen_id(bounds.constant_nodes as u128)),
            ("ConstFieldPos", bitlen_count(bounds.constant_field_slots as u128)),
            ("DataId", bitlen_id(bounds.data_nodes as u128)),
            ("FieldPos", bitlen_count(bounds.data_field_slots as u128)),
            ("EnvId", bitlen_id(bounds.environment_cells as u128)),
            ("NameId", bitlen_id(layout.capacities.name_rows as u128)),
            (
                "OccPathPos",
                bitlen_count(layout.capacities.occurrence_path_slots as u128),
            ),
            (
                "ChildPosition",
                bitlen_count(
                    (bounds.max_arity as u128).max(bounds.match_arm_rows as u128 + 1),
                ),
            ),
            ("ValueSlotPos", bitlen_count(layout.capacities.value_slots as u128)),
            ("ArityCount", bitlen_count(bounds.max_arity as u128)),
            ("Depth", bitlen_count(bounds.frames as u128)),
            ("TaskCount", bitlen_count(bounds.helper_tasks as u128)),
            ("TypeId", bitlen_id(bounds.type_rows as u128)),
            ("CtorId", bitlen_id(bounds.constructor_rows as u128)),
            (
                "CtorFieldPos",
                bitlen_count(bounds.constructor_field_slots as u128),
            ),
            ("PlanNodeId", bitlen_id(bounds.match_plan_rows as u128)),
            ("PlanCasePos", bitlen_count(ext_caps.plan_case_slots as u128)),
            ("FieldPathPos", bitlen_count(ext_caps.field_path_slots as u128)),
            (
                "PatternChildPos",
                bitlen_count(ext_caps.pattern_child_slots as u128),
            ),
            ("DeclIdx", bitlen_id(bounds.type_rows as u128)),
            ("TypeArgPos", bitlen_count(bounds.type_rows as u128)),
        ];
        for (nm, needed) in idx {
            let limit = w.index_width(nm) as u128;
            if limit < needed {
                return Err(ExtTypeBuildError::CoreTooling(Tooling::AdmissionRejected {
                    reason: AdmissionReason::LayoutWidthTooSmall,
                    detail: nm.to_string(),
                    locator: field_loc(&format!("layout.widths.indexWidths.{nm}")),
                }));
            }
        }
        return Err(ExtTypeBuildError::CoreTooling(Tooling::AdmissionRejected {
            reason: AdmissionReason::LayoutWidthTooSmall,
            detail: "recordWidths".to_string(),
            locator: field_loc("layout.widths.recordWidths"),
        }));
    }
    // AD-3: maxArity >= 2.
    if bounds.max_arity < 2 {
        return Err(ExtTypeBuildError::CoreTooling(
            Tooling::CapacityNotAdmitted {
                kind: CapacityKind::MaxArity,
                needed: NeededNatC::Exact(2),
                limit: bounds.max_arity as u128,
                locator: field_loc("bounds.maxArity"),
            },
        ));
    }
    // prepareExt: Pass B -> Pass D -> Pass T -> Pass C1..C6 -> Pass P.
    let (view, report) = prepare_ext_program(program, domain, bounds, layout, locator)?;
    // AD-5 (G-11): check_program_view_ext (V1..V17).
    check_program_view_ext(&view, Some(&report), domain, bounds, layout)?;
    // AD-6: boundary capacities must be absent.
    if layout.boundary_capacities_present {
        return Err(ExtTypeBuildError::CoreTooling(Tooling::UnsupportedFeature {
            feature: FeatureKind::BoundarySection,
            locator: field_loc("layout.boundaryCapacities"),
        }));
    }
    let identity = IdentityC {
        profile_id: 2,
        grammar_domain: domain.grammar_domain,
        finance: domain.finance,
        layout_version: layout.layout_version.clone(),
        layout_digest: DigestC::Unselected,
        bounds_digest: DigestC::Unselected,
        field_id: layout.field_id.clone(),
        program_membership_digest: DigestC::Unselected,
    };
    let admitted = Admitted {
        body: Arc::new(AdmittedBody {
            identity,
            bounds: *bounds,
            view,
            layout: layout.clone(),
        }),
    };
    Ok((admitted, report))
}

pub fn admit_ext(
    program: ExtProgram,
    domain: ExtDomain,
    bounds: &BoundsC,
    layout: &LayoutSchemaC,
    locator: Option<LocatorC>,
) -> Result<(Admitted, PreparationReportC), ExtTypeBuildError> {
    admit_ext_program(program, domain, bounds, layout, locator)
}
