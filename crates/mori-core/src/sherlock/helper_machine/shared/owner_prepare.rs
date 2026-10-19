// owner_prepare.proposed.rs — shared::owner_prepare (child of `shared`)
// Security-owned prepare/admit/drop lane ONLY. Plain std, no unsafe.
// Root places this as a child of `shared` so private Admitted/MachineImage/
// AdmittedBody/WidthTable fields are visible; Root's sole OwnerEngine impl
// delegates prepare_core/check_program_view/admit/admit_run to these fns.
// No selector, guard, StepInv, validate_plan, cap, apply, step, run or
// event-emission body is supplied here (runtime-driver lane).
use super::*;
use std::sync::Arc;

// ---------------------------------------------------------------- helpers
// NeededNatC is the HOST diagnostic carrier for schema Nat `needed`; it adds no cap.
// Exact(n) only for an exact mathematical n (widened usize count, bitlen, AD-1/AD-3
// minimum, or RunIntC::Nat). AboveU128 only for RunIntC::NatAboveU128 (R3/R4), whose
// magnitude the three-class input never carried; it is not a recovered or saturated Nat.
fn cap_err(kind: CapacityKind, needed: u128, limit: u128, locator: Option<LocatorC>) -> Tooling {
    Tooling::CapacityNotAdmitted { kind, needed: NeededNatC::Exact(needed), limit, locator }
}
fn cap_err_above_u128(kind: CapacityKind, limit: u128, locator: Option<LocatorC>) -> Tooling {
    Tooling::CapacityNotAdmitted { kind, needed: NeededNatC::AboveU128, limit, locator }
}
// emit_literal post-order result-stack underflow: an internal invariant breach,
// not a capacity event, so no CapacityNotAdmitted with a fabricated needed/limit.
fn const_stack_underflow() -> Tooling {
    Tooling::AdmissionRejected { reason: AdmissionReason::ProgramViewMalformed,
        detail: "constant post-order stack underflow".to_string(), locator: None }
}
fn field_loc(name: &str) -> Option<LocatorC> {
    Some(LocatorC::Field { name: name.to_string() })
}
fn pv_err(check: ViewCheck, table: &str, row: Option<usize>, detail: &str) -> ProgramViewViolation {
    ProgramViewViolation { check, table: table.to_string(), row, detail: detail.to_string() }
}
// bitlen(n) = max(1, ceil(log2(n+1))) per schema notation.
fn bitlen_count(n: u128) -> u128 {
    if n == 0 { 1 } else { (128 - n.leading_zeros()) as u128 }
}
// id(Z) width: max(1, ceil(log2(max(1,Z)))).
fn bitlen_id(z: u128) -> u128 {
    if z <= 1 { 1 } else { (128 - (z - 1).leading_zeros()) as u128 }
}
fn slice_inside(s: &SliceC, table_len: usize) -> bool {
    if s.len == 0 { s.start == 0 } else {
        match s.start.checked_add(s.len) { Some(end) => end <= table_len, None => false }
    }
}
fn marker_of(row: &RawConstRow) -> bool {
    match row {
        RawConstRow::U128 { marker_free, .. } | RawConstRow::Bool { marker_free, .. }
        | RawConstRow::Nil { marker_free } | RawConstRow::Cons { marker_free, .. }
        | RawConstRow::Leaf { marker_free, .. } | RawConstRow::Branch { marker_free, .. }
        | RawConstRow::Constructor { marker_free, .. }
        | RawConstRow::U128OutOfRange { marker_free } => *marker_free,
    }
}
fn const_fields_of(row: &RawConstRow) -> SliceC {
    match row {
        RawConstRow::Cons { fields, .. } | RawConstRow::Leaf { fields, .. }
        | RawConstRow::Branch { fields, .. }
        | RawConstRow::Constructor { fields, .. } => SliceC::normalized(fields.start.0, fields.len),
        _ => SliceC::empty(),
    }
}
fn const_kind_of(row: &RawConstRow) -> LogicalKind {
    match row {
        RawConstRow::U128 { .. } => LogicalKind::UInt,
        RawConstRow::Bool { .. } => LogicalKind::Bool,
        RawConstRow::Nil { .. } => LogicalKind::Nil,
        RawConstRow::Cons { .. } => LogicalKind::Cons,
        RawConstRow::Leaf { .. } => LogicalKind::Leaf,
        RawConstRow::Branch { .. } => LogicalKind::Branch,
        RawConstRow::Constructor { type_id, ctor_id, .. } => LogicalKind::Constructor {
            type_id: TypeIdC(type_id.0),
            ctor_id: adt::CtorIdC(ctor_id.0),
        },
        RawConstRow::U128OutOfRange { .. } => LogicalKind::Marker,
    }
}

// Clone for the two non-Clone layout carriers. Same-crate impls; WidthTable
// private fields are visible to this descendant of `shared`. This closes the
// admit() construction gap without any contract amendment.
impl Clone for WidthTable {
    fn clone(&self) -> Self {
        WidthTable { index_widths: self.index_widths.clone(),
            counter_widths: self.counter_widths.clone(),
            record_widths: self.record_widths.clone() }
    }
}
impl Clone for LayoutSchemaC {
    fn clone(&self) -> Self {
        LayoutSchemaC { layout_version: self.layout_version.clone(),
            field_id: self.field_id.clone(), widths: self.widths.clone(),
            scalar_layout_id: self.scalar_layout_id.clone(), bounds: self.bounds,
            capacities: self.capacities,
            ext_capacities: self.ext_capacities,
            boundary_capacities_present: self.boundary_capacities_present }
    }
}

// ------------------------------------------------------- iterative Drop C8
fn dummy_expr() -> CoreExpr { CoreExpr::Literal { value: CoreValue::Bool { b: false } } }
fn dummy_tree() -> CoreTree { CoreTree::Leaf { n: i128_class::KIntC::InRange(0) } }

fn take_value_children(v: &mut CoreValue, tw: &mut Vec<Box<CoreTree>>) {
    if let CoreValue::Tree { tree } = v {
        tw.push(Box::new(std::mem::replace(tree, dummy_tree())));
    }
}
fn take_expr_children(e: &mut CoreExpr, ew: &mut Vec<Box<CoreExpr>>, tw: &mut Vec<Box<CoreTree>>) {
    match e {
        CoreExpr::Literal { value } => take_value_children(value, tw),
        CoreExpr::Var { .. } => {}
        CoreExpr::Let { value, body, .. } => {
            ew.push(std::mem::replace(value, Box::new(dummy_expr())));
            ew.push(std::mem::replace(body, Box::new(dummy_expr())));
        }
        CoreExpr::Binary { left, right, .. } => {
            ew.push(std::mem::replace(left, Box::new(dummy_expr())));
            ew.push(std::mem::replace(right, Box::new(dummy_expr())));
        }
        CoreExpr::Unary { operand, .. } => {
            ew.push(std::mem::replace(operand, Box::new(dummy_expr())));
        }
        CoreExpr::If { cond, then_expr, else_expr } => {
            ew.push(std::mem::replace(cond, Box::new(dummy_expr())));
            ew.push(std::mem::replace(then_expr, Box::new(dummy_expr())));
            ew.push(std::mem::replace(else_expr, Box::new(dummy_expr())));
        }
        CoreExpr::Call { args, .. } => {
            for a in std::mem::take(args) { ew.push(Box::new(a)); }
        }
        CoreExpr::Match { scrutinee, arms } => {
            ew.push(std::mem::replace(scrutinee, Box::new(dummy_expr())));
            for arm in std::mem::take(arms) { ew.push(Box::new(arm.body)); }
        }
    }
}
fn take_tree_children(t: &mut CoreTree, tw: &mut Vec<Box<CoreTree>>) {
    if let CoreTree::Branch { left, right } = t {
        tw.push(std::mem::replace(left, Box::new(dummy_tree())));
        tw.push(std::mem::replace(right, Box::new(dummy_tree())));
    }
}
fn drain_worklists(ew: &mut Vec<Box<CoreExpr>>, tw: &mut Vec<Box<CoreTree>>) {
    loop {
        if let Some(mut b) = ew.pop() { take_expr_children(&mut b, ew, tw); continue; }
        if let Some(mut b) = tw.pop() { take_tree_children(&mut b, tw); continue; }
        break;
    }
}
impl Drop for CoreExpr {
    fn drop(&mut self) {
        let mut ew: Vec<Box<CoreExpr>> = Vec::new();
        let mut tw: Vec<Box<CoreTree>> = Vec::new();
        take_expr_children(self, &mut ew, &mut tw);
        drain_worklists(&mut ew, &mut tw);
    }
}
impl Drop for CoreTree {
    fn drop(&mut self) {
        let mut tw: Vec<Box<CoreTree>> = Vec::new();
        take_tree_children(self, &mut tw);
        let mut ew: Vec<Box<CoreExpr>> = Vec::new();
        drain_worklists(&mut ew, &mut tw);
    }
}
impl Drop for CoreProgram {
    fn drop(&mut self) {
        let mut ew: Vec<Box<CoreExpr>> = Vec::new();
        let mut tw: Vec<Box<CoreTree>> = Vec::new();
        ew.push(Box::new(std::mem::replace(&mut self.entry, dummy_expr())));
        for f in std::mem::take(&mut self.functions) { ew.push(Box::new(f.body)); }
        drain_worklists(&mut ew, &mut tw);
    }
}

// ------------------------------------------------------------- prepare_core
enum NameAct<'a> { Name(&'a str), Expr(&'a CoreExpr), Pat(&'a CorePattern) }

fn collect_names<'a>(p: &'a CoreProgram, l: &LayoutSchemaC)
    -> Result<Vec<String>, Tooling> {
    // Pass B: traversal order entry, then per function name/params/body;
    // node-own names before children. Distinct count/bytes checked before push.
    let mut distinct: std::collections::BTreeSet<&'a str> = std::collections::BTreeSet::new();
    let mut bytes_total: u128 = 0;
    let mut stack: Vec<NameAct<'a>> = Vec::new();
    for f in p.functions.iter().rev() {
        stack.push(NameAct::Expr(&f.body));
        for prm in f.parameters.iter().rev() { stack.push(NameAct::Name(&prm.name)); }
        stack.push(NameAct::Name(&f.name));
    }
    stack.push(NameAct::Expr(&p.entry));
    while let Some(act) = stack.pop() {
        match act {
            NameAct::Name(n) => {
                if !distinct.contains(n) {
                    let nb = bytes_total + n.len() as u128;
                    if distinct.len() as u128 >= l.capacities.name_rows as u128 {
                        return Err(cap_err(CapacityKind::NameRows,
                            distinct.len() as u128 + 1, l.capacities.name_rows as u128, None));
                    }
                    if nb > l.capacities.name_bytes as u128 {
                        return Err(cap_err(CapacityKind::NameBytes, nb,
                            l.capacities.name_bytes as u128, None));
                    }
                    bytes_total = nb;
                    distinct.insert(n);
                }
            }
            NameAct::Pat(pat) => match pat {
                CorePattern::PVar { x } | CorePattern::PLeaf { x } => stack.push(NameAct::Name(x)),
                CorePattern::PCons { x, y } | CorePattern::PBranch { x, y } => {
                    stack.push(NameAct::Name(y)); stack.push(NameAct::Name(x));
                }
                CorePattern::PNil => {}
            },
            NameAct::Expr(e) => match e {
                CoreExpr::Literal { .. } => {}
                CoreExpr::Var { name } => stack.push(NameAct::Name(name)),
                CoreExpr::Let { name, value, body } => {
                    stack.push(NameAct::Expr(body)); stack.push(NameAct::Expr(value));
                    stack.push(NameAct::Name(name));
                }
                CoreExpr::Binary { left, right, .. } => {
                    stack.push(NameAct::Expr(right)); stack.push(NameAct::Expr(left));
                }
                CoreExpr::Unary { operand, .. } => stack.push(NameAct::Expr(operand)),
                CoreExpr::If { cond, then_expr, else_expr } => {
                    stack.push(NameAct::Expr(else_expr)); stack.push(NameAct::Expr(then_expr));
                    stack.push(NameAct::Expr(cond));
                }
                CoreExpr::Call { name, args } => {
                    for a in args.iter().rev() { stack.push(NameAct::Expr(a)); }
                    stack.push(NameAct::Name(name));
                }
                CoreExpr::Match { scrutinee, arms } => {
                    for arm in arms.iter().rev() { stack.push(NameAct::Expr(&arm.body)); }
                    stack.push(NameAct::Expr(scrutinee));
                    for arm in arms.iter().rev() { stack.push(NameAct::Pat(&arm.pattern)); }
                }
            },
        }
    }
    Ok(distinct.into_iter().map(|n| n.to_string()).collect())
}

fn name_id(names: &[String], n: &str) -> Result<usize, Tooling> {
    names.binary_search_by(|x| x.as_str().cmp(n)).map_err(|_| Tooling::AdmissionRejected {
        reason: AdmissionReason::ProgramViewMalformed,
        detail: "name not collected in pass B".to_string(), locator: None })
}


/// Identifies whether `PreparedNominalDeclarations::interned_names` covers only `&[TypeDeclSrc]`
/// (`DeclarationOnly`) or the unified Pass-B name pool of `&CoreProgram` + `&[TypeDeclSrc]`
/// (`ProgramAndDeclarations`).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PreparedDeclarationScope {
    DeclarationOnly,
    ProgramAndDeclarations,
}

/// Checked nominal declaration-phase output: authenticated Pass-B name table, bound declaration
/// NameIds, and T1/T2 instantiated type/constructor/field tables.
/// Not an `Admitted` program, runtime `MachineImage`, full `ProgramView`, or proof.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PreparedNominalDeclarations {
    pub scope: PreparedDeclarationScope,
    pub domain: GrammarDomain,
    pub interned_names: Vec<String>,
    pub decl_name_ids: super::owner_ext_type_builder::PassBTypeNameIds,
    pub tables: super::owner_ext_type_builder::BuiltAdtTypeTables,
}

fn push_distinct_name<'a>(
    distinct: &mut std::collections::BTreeSet<&'a str>,
    bytes_total: &mut u128,
    n: &'a str,
    l: &LayoutSchemaC,
) -> Result<(), Tooling> {
    if !distinct.contains(n) {
        let nb = *bytes_total + n.len() as u128;
        if distinct.len() as u128 >= l.capacities.name_rows as u128 {
            return Err(cap_err(
                CapacityKind::NameRows,
                distinct.len() as u128 + 1,
                l.capacities.name_rows as u128,
                None,
            ));
        }
        if nb > l.capacities.name_bytes as u128 {
            return Err(cap_err(
                CapacityKind::NameBytes,
                nb,
                l.capacities.name_bytes as u128,
                None,
            ));
        }
        *bytes_total = nb;
        distinct.insert(n);
    }
    Ok(())
}

fn collect_decl_group_names<'a>(
    decls: &'a [super::owner_ext_type_builder::TypeDeclSrc],
    distinct: &mut std::collections::BTreeSet<&'a str>,
    bytes_total: &mut u128,
    l: &LayoutSchemaC,
) -> Result<(), Tooling> {
    for decl in decls {
        push_distinct_name(distinct, bytes_total, &decl.name, l)?;
        for ctor in &decl.ctors {
            push_distinct_name(distinct, bytes_total, &ctor.name, l)?;
            for field in &ctor.fields {
                push_distinct_name(distinct, bytes_total, &field.name, l)?;
            }
        }
    }
    Ok(())
}

fn bind_owner_decl_name_ids(
    decls: &[super::owner_ext_type_builder::TypeDeclSrc],
    interned_names: &[String],
) -> Result<super::owner_ext_type_builder::PassBTypeNameIds, Tooling> {
    use super::owner_ext_type_builder::{DeclNameIds, PassBTypeNameIds};
    let mut bound_decls = Vec::with_capacity(decls.len());
    for decl in decls {
        let type_name_id = name_id(interned_names, &decl.name)?;
        let mut ctor_name_ids = Vec::with_capacity(decl.ctors.len());
        let mut field_name_ids = Vec::with_capacity(decl.ctors.len());
        for ctor in &decl.ctors {
            ctor_name_ids.push(name_id(interned_names, &ctor.name)?);
            let mut f_ids = Vec::with_capacity(ctor.fields.len());
            for field in &ctor.fields {
                f_ids.push(name_id(interned_names, &field.name)?);
            }
            field_name_ids.push(f_ids);
        }
        bound_decls.push(DeclNameIds {
            type_name_id,
            ctor_name_ids,
            field_name_ids,
        });
    }
    Ok(PassBTypeNameIds::new_owner_verified(
        interned_names.len(),
        bound_decls,
    ))
}

fn collect_declaration_only_names(
    decls: &[super::owner_ext_type_builder::TypeDeclSrc],
    l: &LayoutSchemaC,
) -> Result<
    (Vec<String>, super::owner_ext_type_builder::PassBTypeNameIds),
    super::owner_ext_type_builder::ExtTypeBuildError,
> {
    use super::owner_ext_type_builder::ExtTypeBuildError;
    let mut distinct: std::collections::BTreeSet<&str> = std::collections::BTreeSet::new();
    let mut bytes_total: u128 = 0;
    collect_decl_group_names(decls, &mut distinct, &mut bytes_total, l)
        .map_err(ExtTypeBuildError::CoreTooling)?;
    let interned_names: Vec<String> = distinct.into_iter().map(|s| s.to_string()).collect();
    let decl_ids = bind_owner_decl_name_ids(decls, &interned_names)
        .map_err(ExtTypeBuildError::CoreTooling)?;
    Ok((interned_names, decl_ids))
}

fn collect_program_and_declaration_names(
    p: &CoreProgram,
    decls: &[super::owner_ext_type_builder::TypeDeclSrc],
    l: &LayoutSchemaC,
) -> Result<
    (Vec<String>, super::owner_ext_type_builder::PassBTypeNameIds),
    super::owner_ext_type_builder::ExtTypeBuildError,
> {
    use super::owner_ext_type_builder::ExtTypeBuildError;
    let mut distinct: std::collections::BTreeSet<&str> = std::collections::BTreeSet::new();
    let mut bytes_total: u128 = 0;
    collect_decl_group_names(decls, &mut distinct, &mut bytes_total, l)
        .map_err(ExtTypeBuildError::CoreTooling)?;
    let mut stack: Vec<NameAct<'_>> = Vec::new();
    for f in p.functions.iter().rev() {
        stack.push(NameAct::Expr(&f.body));
        for prm in f.parameters.iter().rev() {
            stack.push(NameAct::Name(&prm.name));
        }
        stack.push(NameAct::Name(&f.name));
    }
    stack.push(NameAct::Expr(&p.entry));
    while let Some(act) = stack.pop() {
        match act {
            NameAct::Name(n) => {
                push_distinct_name(&mut distinct, &mut bytes_total, n, l)
                    .map_err(ExtTypeBuildError::CoreTooling)?;
            }
            NameAct::Pat(pat) => match pat {
                CorePattern::PVar { x } | CorePattern::PLeaf { x } => stack.push(NameAct::Name(x)),
                CorePattern::PCons { x, y } | CorePattern::PBranch { x, y } => {
                    stack.push(NameAct::Name(y));
                    stack.push(NameAct::Name(x));
                }
                CorePattern::PNil => {}
            },
            NameAct::Expr(e) => match e {
                CoreExpr::Literal { .. } => {}
                CoreExpr::Var { name } => stack.push(NameAct::Name(name)),
                CoreExpr::Let { name, value, body } => {
                    stack.push(NameAct::Expr(body));
                    stack.push(NameAct::Expr(value));
                    stack.push(NameAct::Name(name));
                }
                CoreExpr::Binary { left, right, .. } => {
                    stack.push(NameAct::Expr(right));
                    stack.push(NameAct::Expr(left));
                }
                CoreExpr::Unary { operand, .. } => stack.push(NameAct::Expr(operand)),
                CoreExpr::If {
                    cond,
                    then_expr,
                    else_expr,
                } => {
                    stack.push(NameAct::Expr(else_expr));
                    stack.push(NameAct::Expr(then_expr));
                    stack.push(NameAct::Expr(cond));
                }
                CoreExpr::Call { name, args } => {
                    for a in args.iter().rev() {
                        stack.push(NameAct::Expr(a));
                    }
                    stack.push(NameAct::Name(name));
                }
                CoreExpr::Match { scrutinee, arms } => {
                    for arm in arms.iter().rev() {
                        stack.push(NameAct::Expr(&arm.body));
                    }
                    stack.push(NameAct::Expr(scrutinee));
                    for arm in arms.iter().rev() {
                        stack.push(NameAct::Pat(&arm.pattern));
                    }
                }
            },
        }
    }
    let interned_names: Vec<String> = distinct.into_iter().map(|s| s.to_string()).collect();
    let decl_ids = bind_owner_decl_name_ids(decls, &interned_names)
        .map_err(ExtTypeBuildError::CoreTooling)?;
    Ok((interned_names, decl_ids))
}

/// Checked nominal declaration table producer over raw declarations and explicit layout/bounds.
pub fn prepare_nominal_declarations(
    decls: &[super::owner_ext_type_builder::TypeDeclSrc],
    layout: &LayoutSchemaC,
    domain: GrammarDomain,
    locator: Option<LocatorC>,
) -> Result<PreparedNominalDeclarations, super::owner_ext_type_builder::ExtTypeBuildError> {
    use super::nominal_declaration_order::walk_declaration_type_occurrences;
    use super::owner_ext_prepare_types::instantiate_declared_types;
    use super::owner_ext_type_builder::{CheckedTypeDeclView, ExtTypeBuildError};

    if layout.boundary_capacities_present {
        return Err(ExtTypeBuildError::CoreTooling(
            Tooling::UnsupportedFeature {
                feature: FeatureKind::BoundarySection,
                locator: None,
            },
        ));
    }
    let (interned_names, decl_name_ids) = collect_declaration_only_names(decls, layout)?;
    let view = CheckedTypeDeclView::new_checked(domain, decls, &decl_name_ids, &layout.bounds)?;
    walk_declaration_type_occurrences(&view, |occ, _path, ty_expr| {
        view.validate_declaration_type_occurrence(occ, ty_expr)
    })?;
    let tables = instantiate_declared_types(&view, locator)?;
    Ok(PreparedNominalDeclarations {
        scope: PreparedDeclarationScope::DeclarationOnly,
        domain,
        interned_names,
        decl_name_ids,
        tables,
    })
}

fn pass_t_wrong(loc: &str) -> super::owner_ext_type_builder::ExtTypeBuildError {
    super::owner_ext_type_builder::ExtTypeBuildError::Host( super::HostViolation {
        kind: super::HostViolationKind::WrongVariant,
        clause: None,
        rule: None,
        locator: Some(loc.to_string()),
    })
}

fn reject_absent_fin_expr(
    expr: &super::owner_ext_t3_uses::ExtExpr,
) -> Result<Option<&str>, Tooling> {
    use super::owner_ext_t3_uses::ExtExpr;
    match expr {
        ExtExpr::StateRead { .. }
        | ExtExpr::LiabilityPresent { .. }
        | ExtExpr::LiabilityAmount { .. }
        | ExtExpr::Fin { .. }
        | ExtExpr::Await { .. } => Err(Tooling::UnsupportedFeature {
            feature: FeatureKind::FinancialSyntax,
            locator: None,
        }),
        _ => Ok(None),
    }
}

fn ensure_absent_finance_source(
    program: &super::owner_ext_source_program::ExtProgram,
) -> Result<(), Tooling> {
    use super::ext_source_traversal::walk_ext_expr_preorder;
    if !program.assets.is_empty() || !program.states.is_empty() || !program.inputs.is_empty() {
        return Err(Tooling::UnsupportedFeature {
            feature: FeatureKind::FinancialSyntax,
            locator: None,
        });
    }
    walk_ext_expr_preorder(&program.entry, |e| reject_absent_fin_expr(e).map(|_| ()))?;
    for function in &program.functions {
        walk_ext_expr_preorder(&function.body, |e| reject_absent_fin_expr(e).map(|_| ()))?;
    }
    Ok(())
}

fn collect_ext_program_names(
    program: &super::owner_ext_source_program::ExtProgram,
    l: &LayoutSchemaC,
) -> Result<
    (Vec<String>, super::owner_ext_type_builder::PassBTypeNameIds),
    super::owner_ext_type_builder::ExtTypeBuildError,
> {
    use super::ext_pass_b_walk::for_each_ext_program_name;
    use super::owner_ext_type_builder::ExtTypeBuildError;
    ensure_absent_finance_source(program).map_err(ExtTypeBuildError::CoreTooling)?;
    let mut distinct: std::collections::BTreeSet<&str> = std::collections::BTreeSet::new();
    let mut bytes_total: u128 = 0;
    for_each_ext_program_name(
        program,
        &mut |n| push_distinct_name(&mut distinct, &mut bytes_total, n, l),
        &mut reject_absent_fin_expr,
    )
    .map_err(ExtTypeBuildError::CoreTooling)?;
    let interned_names: Vec<String> = distinct.into_iter().map(|s| s.to_string()).collect();
    let decl_ids = bind_owner_decl_name_ids(&program.types, &interned_names)
        .map_err(ExtTypeBuildError::CoreTooling)?;
    Ok((interned_names, decl_ids))
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PassTOutput {
    pub tables: super::owner_ext_type_builder::BuiltAdtTypeTables,
    pub records: Vec<super::owner_ext_t3_uses::T3Record>,
}

pub struct PassTSession<'v, 's> {
    view: &'v super::owner_ext_type_builder::CheckedTypeDeclView<'v>,
    builder: super::owner_ext_type_builder::OwnerExtTypeBuilder<'v>,
    walker: super::owner_ext_t3_uses::T3UseWalker<'s>,
    locator: Option<LocatorC>,
    records: Vec<super::owner_ext_t3_uses::T3Record>,
    poisoned: bool,
}

impl<'v, 's> PassTSession<'v, 's> {
    pub fn begin(
        domain: GrammarDomain,
        view: &'v super::owner_ext_type_builder::CheckedTypeDeclView<'v>,
        functions: &'s [super::owner_ext_t3_uses::ExtFunction],
        entry: &'s super::owner_ext_t3_uses::ExtExpr,
        locator: Option<LocatorC>,
    ) -> Result<Self, super::owner_ext_type_builder::ExtTypeBuildError> {
        use super::owner_ext_prepare_types::drain_pending_keys;
        use super::owner_ext_t3_uses::T3UseWalker;
        use super::owner_ext_type_builder::OwnerExtTypeBuilder;
        if domain == GrammarDomain::DedicatedSorts {
            return Err(pass_t_wrong("pass T under DedicatedSorts (G-10)"));
        }
        let mut builder = OwnerExtTypeBuilder::new_checked(view)?;
        for &d in view.non_parameterized_decls() {
            builder.register_non_parameterized_decl(d)?;
        }
        drain_pending_keys(view, &mut builder, locator.clone())?;
        Ok(Self {
            view,
            builder,
            walker: T3UseWalker::new(functions, entry),
            locator,
            records: Vec::new(),
            poisoned: false,
        })
    }

    pub fn process_next_use(
        &mut self,
    ) -> Result<Option<super::owner_ext_t3_uses::T3Record>, super::owner_ext_type_builder::ExtTypeBuildError> {
        if self.poisoned {
            return Err(pass_t_wrong("pass T session poisoned"));
        }
        let r = self.step();
        if r.is_err() {
            self.poisoned = true;
        }
        r
    }

    fn step(
        &mut self,
    ) -> Result<Option<super::owner_ext_t3_uses::T3Record>, super::owner_ext_type_builder::ExtTypeBuildError> {
        use super::owner_ext_prepare_types::drain_pending_keys;
        use super::owner_ext_t3_uses::{T3Record, T3UseKind};
        let u = match self.walker.next_use()? {
            Some(u) => u,
            None => return Ok(None),
        };
        if self.builder.has_pending_keys() {
            return Err(pass_t_wrong("T3 use with undrained FIFO"));
        }
        let b = &mut self.builder;
        let (type_id, ctor_use) = match u.kind {
            T3UseKind::Signature { ty } => (b.resolve_type_expr(ty, None, self.locator.clone())?, None),
            T3UseKind::Constructor { ty, ctor, arity } => {
                let r = b.resolve_construct_use(ty, ctor, arity, self.locator.clone())?;
                (r.type_id, Some(r))
            }
            T3UseKind::PatternCtor { ty, ctor, arity } => {
                let r = b.resolve_pattern_ctor_use(ty, ctor, arity, self.locator.clone())?;
                (r.type_id, Some(r))
            }
        };
        drain_pending_keys(self.view, &mut self.builder, self.locator.clone())?;
        let ctor_id = match ctor_use {
            Some(r) => Some(self.builder.resolved_ctor_id(r)?),
            None => None,
        };
        let rec = T3Record { site: u.site, type_id, ctor_id };
        self.records.push(rec);
        Ok(Some(rec))
    }

    pub fn records(&self) -> &[super::owner_ext_t3_uses::T3Record] {
        &self.records
    }

    pub fn finish(self) -> Result<PassTOutput, super::owner_ext_type_builder::ExtTypeBuildError> {
        if self.poisoned || !self.walker.is_finished() {
            return Err(pass_t_wrong("pass T finish before every T3 use processed"));
        }
        let tables = self.builder.finish()?;
        Ok(PassTOutput { tables, records: self.records })
    }
}

pub fn run_pass_t<'v, 's>(
    domain: GrammarDomain,
    view: &'v super::owner_ext_type_builder::CheckedTypeDeclView<'v>,
    functions: &'s [super::owner_ext_t3_uses::ExtFunction],
    entry: &'s super::owner_ext_t3_uses::ExtExpr,
    locator: Option<LocatorC>,
) -> Result<PassTOutput, super::owner_ext_type_builder::ExtTypeBuildError> {
    let mut s = PassTSession::begin(domain, view, functions, entry, locator)?;
    while s.process_next_use()?.is_some() {}
    s.finish()
}

pub struct PreparedExtProgramSource {
    pub program: super::owner_ext_source_program::ExtProgram,
    pub domain: super::owner_ext_source_program::ExtDomain,
    pub interned_names: Vec<String>,
    pub decl_name_ids: super::owner_ext_type_builder::PassBTypeNameIds,
    pub tables: super::owner_ext_type_builder::BuiltAdtTypeTables,
    pub t3_records: Vec<super::owner_ext_t3_uses::T3Record>,
}

pub fn prepare_ext_program_types(
    program: super::owner_ext_source_program::ExtProgram,
    layout: &LayoutSchemaC,
    domain: super::owner_ext_source_program::ExtDomain,
    locator: Option<LocatorC>,
) -> Result<PreparedExtProgramSource, super::owner_ext_type_builder::ExtTypeBuildError> {
    use super::nominal_declaration_order::walk_declaration_type_occurrences;
    use super::owner_ext_type_builder::{CheckedTypeDeclView, ExtTypeBuildError};
    if layout.boundary_capacities_present {
        return Err(ExtTypeBuildError::CoreTooling(Tooling::UnsupportedFeature {
            feature: FeatureKind::BoundarySection,
            locator: None,
        }));
    }
    if !domain.is_finance_absent() {
        return Err(ExtTypeBuildError::CoreTooling(Tooling::UnsupportedFeature {
            feature: FeatureKind::FinancialInput,
            locator: None,
        }));
    }
    let (interned_names, decl_name_ids) = collect_ext_program_names(&program, layout)?;
    let pass_t = {
        let view = CheckedTypeDeclView::new_checked(
            domain.grammar_domain,
            &program.types,
            &decl_name_ids,
            &layout.bounds,
        )?;
        walk_declaration_type_occurrences(&view, |occ, _path, ty_expr| {
            view.validate_declaration_type_occurrence(occ, ty_expr)
        })?;
        run_pass_t(domain.grammar_domain, &view, &program.functions, &program.entry, locator)?
    };
    Ok(PreparedExtProgramSource {
        program,
        domain,
        interned_names,
        decl_name_ids,
        tables: pass_t.tables,
        t3_records: pass_t.records,
    })
}

/// Checked nominal declaration table producer binding declaration names into the unified
/// Pass-B name pool of `program` + `decls`.
pub fn prepare_program_nominal_declarations(
    program: &CoreProgram,
    decls: &[super::owner_ext_type_builder::TypeDeclSrc],
    layout: &LayoutSchemaC,
    domain: GrammarDomain,
    locator: Option<LocatorC>,
) -> Result<PreparedNominalDeclarations, super::owner_ext_type_builder::ExtTypeBuildError> {
    use super::nominal_declaration_order::walk_declaration_type_occurrences;
    use super::owner_ext_prepare_types::instantiate_declared_types;
    use super::owner_ext_type_builder::{CheckedTypeDeclView, ExtTypeBuildError};

    if layout.boundary_capacities_present {
        return Err(ExtTypeBuildError::CoreTooling(
            Tooling::UnsupportedFeature {
                feature: FeatureKind::BoundarySection,
                locator: None,
            },
        ));
    }
    let (interned_names, decl_name_ids) =
        collect_program_and_declaration_names(program, decls, layout)?;
    let view = CheckedTypeDeclView::new_checked(domain, decls, &decl_name_ids, &layout.bounds)?;
    walk_declaration_type_occurrences(&view, |occ, _path, ty_expr| {
        view.validate_declaration_type_occurrence(occ, ty_expr)
    })?;
    let tables = instantiate_declared_types(&view, locator)?;
    Ok(PreparedNominalDeclarations {
        scope: PreparedDeclarationScope::ProgramAndDeclarations,
        domain,
        interned_names,
        decl_name_ids,
        tables,
    })
}


struct TempExpr<'a> {
    expr: &'a CoreExpr, owner_is_function: bool, function: usize,
    parent: Option<usize>, child_pos: usize, child_ids: Vec<usize>,
}

fn emit_const_node(constants: &mut Vec<RawConstRow>, fields_out: &mut Vec<usize>,
    b: &BoundsC, kind: LogicalKind, payload: ConstPayload, field_ids: &[usize])
    -> Result<usize, Tooling> {
    if constants.len() >= b.constant_nodes {
        return Err(cap_err(CapacityKind::ConstantNodes, constants.len() as u128 + 1,
            b.constant_nodes as u128, None));
    }
    let start = fields_out.len();
    for f in field_ids {
        if fields_out.len() >= b.constant_field_slots {
            return Err(cap_err(CapacityKind::ConstantFieldSlots,
                fields_out.len() as u128 + 1, b.constant_field_slots as u128, None));
        }
        fields_out.push(*f);
    }
    let mf = match kind {
        LogicalKind::Marker => false,
        _ => field_ids.iter().all(|f| marker_of(&constants[*f])),
    };
    let slice = crate::sherlock::helper_machine::raw::ConstFieldSlice {
        start: crate::sherlock::helper_machine::raw::ConstFieldPos(if field_ids.is_empty() { 0 } else { start }),
        len: field_ids.len() };
    let row = match payload {
        ConstPayload::U128(n) => RawConstRow::U128 { marker_free: mf, n },
        ConstPayload::Bool(v) => RawConstRow::Bool { marker_free: mf, b: v },
        ConstPayload::Nil => RawConstRow::Nil { marker_free: mf },
        ConstPayload::Marker => RawConstRow::U128OutOfRange { marker_free: false },
        ConstPayload::Cons => RawConstRow::Cons { marker_free: mf, fields: slice },
        ConstPayload::Leaf => RawConstRow::Leaf { marker_free: mf, fields: slice },
        ConstPayload::Branch => RawConstRow::Branch { marker_free: mf, fields: slice },
    };
    constants.push(row);
    Ok(constants.len() - 1)
}
enum ConstPayload { U128(u128), Bool(bool), Nil, Marker, Cons, Leaf, Branch }

enum ConstTask<'a> { Value(&'a CoreValue), Tree(&'a CoreTree),
    Scalar(&'a i128_class::KIntC), BuildLeaf, BuildBranch }
enum DecodedConst { Scalar(i128_class::KIntC), BoolVal(bool),
    List(Vec<i128_class::KIntC>), Tree(CoreTree) }

pub(super) fn emit_literal(value: &CoreValue, constants: &mut Vec<RawConstRow>,
    fields_out: &mut Vec<usize>, b: &BoundsC) -> Result<usize, Tooling> {
    // Iterative post-order over the uniquely-owned literal tree (no recursion).
    let mut results: Vec<usize> = Vec::new();
    let mut stack: Vec<ConstTask<'_>> = vec![ConstTask::Value(value)];
    while let Some(t) = stack.pop() {
        match t {
            ConstTask::Scalar(k) => {
                let id = match k {
                    i128_class::KIntC::InRange(n) => emit_const_node(constants, fields_out, b,
                        LogicalKind::UInt, ConstPayload::U128(*n), &[])?,
                    _ => emit_const_node(constants, fields_out, b,
                        LogicalKind::Marker, ConstPayload::Marker, &[])?,
                };
                results.push(id);
            }
            ConstTask::Value(v) => match v {
                CoreValue::UInt { n } => stack.push(ConstTask::Scalar(n)),
                CoreValue::Bool { b: v } => {
                    results.push(emit_const_node(constants, fields_out, b,
                        LogicalKind::Bool, ConstPayload::Bool(*v), &[])?);
                }
                CoreValue::List { elements } => {
                    // Post-order a,b,Nil,Cons(b),Cons(a): scalars in order, Nil,
                    // then Cons nodes in reverse element order.
                    let mut elem_ids: Vec<usize> = Vec::new();
                    for el in elements {
                        elem_ids.push(match el {
                            i128_class::KIntC::InRange(n) => emit_const_node(constants,
                                fields_out, b, LogicalKind::UInt, ConstPayload::U128(*n), &[])?,
                            _ => emit_const_node(constants, fields_out, b,
                                LogicalKind::Marker, ConstPayload::Marker, &[])?,
                        });
                    }
                    let mut tail = emit_const_node(constants, fields_out, b,
                        LogicalKind::Nil, ConstPayload::Nil, &[])?;
                    for eid in elem_ids.iter().rev() {
                        tail = emit_const_node(constants, fields_out, b, LogicalKind::Cons,
                            ConstPayload::Cons, &[*eid, tail])?;
                    }
                    results.push(tail);
                }
                CoreValue::Tree { tree } => stack.push(ConstTask::Tree(tree)),
            },
            ConstTask::Tree(t) => match t {
                CoreTree::Leaf { n } => {
                    stack.push(ConstTask::BuildLeaf); stack.push(ConstTask::Scalar(n));
                }
                CoreTree::Branch { left, right } => {
                    stack.push(ConstTask::BuildBranch);
                    stack.push(ConstTask::Tree(right)); stack.push(ConstTask::Tree(left));
                }
            },
            ConstTask::BuildLeaf => {
                let f = results.pop().ok_or_else(const_stack_underflow)?;
                results.push(emit_const_node(constants, fields_out, b, LogicalKind::Leaf,
                    ConstPayload::Leaf, &[f])?);
            }
            ConstTask::BuildBranch => {
                let r = results.pop().ok_or_else(const_stack_underflow)?;
                let lf = results.pop().ok_or_else(const_stack_underflow)?;
                results.push(emit_const_node(constants, fields_out, b, LogicalKind::Branch,
                    ConstPayload::Branch, &[lf, r])?);
            }
        }
    }
    results.pop().ok_or_else(const_stack_underflow)
}

pub fn prepare_core(p: &CoreProgram, b: &BoundsC, l: &LayoutSchemaC)
    -> Result<ProgramView, Tooling> {
    // Pass A vacuous: CoreProgram cannot express GenericAdt/FinancialSyntax.
    let names = collect_names(p, l)?; // Pass B
    // ---- C1: expressions + occurrences, preorder, check before each push.
    let mut temps: Vec<TempExpr<'_>> = Vec::new();
    struct Item<'a> { e: &'a CoreExpr, owner_fn: bool, function: usize,
        parent: Option<usize>, child_pos: usize }
    let mut stack: Vec<Item<'_>> = Vec::new();
    for (fi, f) in p.functions.iter().enumerate().rev() {
        stack.push(Item { e: &f.body, owner_fn: true, function: fi, parent: None, child_pos: 0 });
    }
    stack.push(Item { e: &p.entry, owner_fn: false, function: 0, parent: None, child_pos: 0 });
    let mut fn_body_ids: Vec<usize> = vec![0; p.functions.len()];
    while let Some(it) = stack.pop() {
        if temps.len() >= b.code_rows {
            return Err(cap_err(CapacityKind::CodeRows, temps.len() as u128 + 1,
                b.code_rows as u128, None));
        }
        if let CoreExpr::Call { args, .. } = it.e {
            if args.len() > b.max_arity {
                return Err(cap_err(CapacityKind::MaxArity, args.len() as u128,
                    b.max_arity as u128, None));
            }
        }
        let id = temps.len();
        if it.parent.is_none() && it.owner_fn { fn_body_ids[it.function] = id; }
        let n_children = match it.e {
            CoreExpr::Let { .. } | CoreExpr::Binary { .. } => 2,
            CoreExpr::Unary { .. } => 1,
            CoreExpr::If { .. } => 3,
            CoreExpr::Call { args, .. } => args.len(),
            CoreExpr::Match { arms, .. } => 1 + arms.len(),
            _ => 0,
        };
        temps.push(TempExpr { expr: it.e, owner_is_function: it.owner_fn,
            function: it.function, parent: it.parent, child_pos: it.child_pos,
            child_ids: vec![usize::MAX; n_children] });
        if let Some(par) = it.parent { temps[par].child_ids[it.child_pos] = id; }
        // Push children reversed so IDs are preorder in semantic order.
        match it.e {
            CoreExpr::Let { value, body, .. } => {
                stack.push(Item { e: body, owner_fn: it.owner_fn, function: it.function, parent: Some(id), child_pos: 1 });
                stack.push(Item { e: value, owner_fn: it.owner_fn, function: it.function, parent: Some(id), child_pos: 0 });
            }
            CoreExpr::Binary { left, right, .. } => {
                stack.push(Item { e: right, owner_fn: it.owner_fn, function: it.function, parent: Some(id), child_pos: 1 });
                stack.push(Item { e: left, owner_fn: it.owner_fn, function: it.function, parent: Some(id), child_pos: 0 });
            }
            CoreExpr::Unary { operand, .. } => stack.push(Item { e: operand,
                owner_fn: it.owner_fn, function: it.function, parent: Some(id), child_pos: 0 }),
            CoreExpr::If { cond, then_expr, else_expr } => {
                stack.push(Item { e: else_expr, owner_fn: it.owner_fn, function: it.function, parent: Some(id), child_pos: 2 });
                stack.push(Item { e: then_expr, owner_fn: it.owner_fn, function: it.function, parent: Some(id), child_pos: 1 });
                stack.push(Item { e: cond, owner_fn: it.owner_fn, function: it.function, parent: Some(id), child_pos: 0 });
            }
            CoreExpr::Call { args, .. } => {
                for (i, a) in args.iter().enumerate().rev() {
                    stack.push(Item { e: a, owner_fn: it.owner_fn, function: it.function, parent: Some(id), child_pos: i });
                }
            }
            CoreExpr::Match { scrutinee, arms } => {
                for (k, arm) in arms.iter().enumerate().rev() {
                    stack.push(Item { e: &arm.body, owner_fn: it.owner_fn, function: it.function, parent: Some(id), child_pos: k + 1 });
                }
                stack.push(Item { e: scrutinee, owner_fn: it.owner_fn, function: it.function, parent: Some(id), child_pos: 0 });
            }
            _ => {}
        }
    }
    // Expression rows with placeholder Call/Match slices, fixed in C2/C3.
    let mut expressions: Vec<ExpressionRowC> = Vec::new();
    let mut occurrences: Vec<OccurrenceRowC> = Vec::new();
    for t in &temps {
        occurrences.push(OccurrenceRowC { owner_is_function: t.owner_is_function,
            function: if t.owner_is_function { t.function } else { 0 }, path: SliceC::empty() });
        expressions.push(match t.expr {
            CoreExpr::Literal { .. } => ExpressionRowC::Literal { constant: usize::MAX },
            CoreExpr::Var { name } => ExpressionRowC::Var { binder: name_id(&names, name)? },
            CoreExpr::Let { name, .. } => ExpressionRowC::Let { binder: name_id(&names, name)?,
                value: t.child_ids[0], body: t.child_ids[1] },
            CoreExpr::Binary { op, .. } => ExpressionRowC::Binary { op: *op,
                left: t.child_ids[0], right: t.child_ids[1] },
            CoreExpr::Unary { op, .. } => ExpressionRowC::Unary { op: *op, operand: t.child_ids[0] },
            CoreExpr::If { .. } => ExpressionRowC::If { cond: t.child_ids[0],
                then_expr: t.child_ids[1], else_expr: t.child_ids[2] },
            CoreExpr::Call { name, .. } => ExpressionRowC::Call {
                function_name: name_id(&names, name)?, args: SliceC::empty() },
            CoreExpr::Match { .. } => ExpressionRowC::Match {
                scrutinee: t.child_ids[0], arms: SliceC::empty(), plan: None },
        });
    }
    // ---- C2: children slots in ExprId order.
    let mut children: Vec<usize> = Vec::new();
    for (i, t) in temps.iter().enumerate() {
        if let CoreExpr::Call { args, .. } = t.expr {
            let start = children.len();
            for cid in &t.child_ids {
                if children.len() >= b.code_child_slots {
                    return Err(cap_err(CapacityKind::CodeChildSlots,
                        children.len() as u128 + 1, b.code_child_slots as u128, None));
                }
                children.push(*cid);
            }
            let _ = args;
            if let ExpressionRowC::Call { args: a, .. } = &mut expressions[i] {
                *a = SliceC::normalized(start, t.child_ids.len());
            }
        }
    }
    // ---- C3: arms, then pattern rows per arm, in ExprId order.
    let mut arms: Vec<ArmRowC> = Vec::new();
    let mut patterns: Vec<PatternRowC> = Vec::new();
    for (i, t) in temps.iter().enumerate() {
        if let CoreExpr::Match { arms: m_arms, .. } = t.expr {
            let start = arms.len();
            for (k, arm) in m_arms.iter().enumerate() {
                if arms.len() >= b.match_arm_rows {
                    return Err(cap_err(CapacityKind::MatchArmRows, arms.len() as u128 + 1,
                        b.match_arm_rows as u128, None));
                }
                if patterns.len() >= b.pattern_rows {
                    return Err(cap_err(CapacityKind::PatternRows, patterns.len() as u128 + 1,
                        b.pattern_rows as u128, None));
                }
                let prow = match &arm.pattern {
                    CorePattern::PVar { x } => PatternRowC::PVar { x: name_id(&names, x)? },
                    CorePattern::PNil => PatternRowC::PNil,
                    CorePattern::PCons { x, y } => PatternRowC::PCons {
                        x: name_id(&names, x)?, y: name_id(&names, y)? },
                    CorePattern::PLeaf { x } => PatternRowC::PLeaf { x: name_id(&names, x)? },
                    CorePattern::PBranch { x, y } => PatternRowC::PBranch {
                        x: name_id(&names, x)?, y: name_id(&names, y)? },
                };
                patterns.push(prow);
                arms.push(ArmRowC { pattern: patterns.len() - 1, body: t.child_ids[k + 1] });
            }
            if let ExpressionRowC::Match { arms: a, .. } = &mut expressions[i] {
                *a = SliceC::normalized(start, m_arms.len());
            }
        }
    }
    // ---- C4: occurrence paths in occurrence order, from parent links.
    let mut occurrence_paths: Vec<usize> = Vec::new();
    for i in 0..temps.len() {
        let mut path: Vec<usize> = Vec::new();
        let mut cur = i;
        while let Some(par) = temps[cur].parent { path.push(temps[cur].child_pos); cur = par; }
        path.reverse();
        let start = occurrence_paths.len();
        for c in &path {
            if occurrence_paths.len() >= l.capacities.occurrence_path_slots {
                return Err(cap_err(CapacityKind::OccurrencePathSlots,
                    occurrence_paths.len() as u128 + 1,
                    l.capacities.occurrence_path_slots as u128, None));
            }
            occurrence_paths.push(*c);
        }
        occurrences[i].path = SliceC::normalized(start, path.len());
    }
    // ---- C5: functions, then parameters, in table order.
    let mut functions: Vec<FunctionRowC> = Vec::new();
    let mut parameters: Vec<ParameterRowC> = Vec::new();
    for (fi, f) in p.functions.iter().enumerate() {
        if functions.len() >= b.function_rows {
            return Err(cap_err(CapacityKind::FunctionRows, functions.len() as u128 + 1,
                b.function_rows as u128, None));
        }
        if f.parameters.len() > b.max_arity {
            return Err(cap_err(CapacityKind::MaxArity, f.parameters.len() as u128,
                b.max_arity as u128, None));
        }
        let start = parameters.len();
        for prm in &f.parameters {
            if parameters.len() >= b.parameter_slots {
                return Err(cap_err(CapacityKind::ParameterSlots,
                    parameters.len() as u128 + 1, b.parameter_slots as u128, None));
            }
            parameters.push(ParameterRowC { binder: name_id(&names, &prm.name)?, sort: TypeIdC::from_source_sort(prm.sort) });
        }
        functions.push(FunctionRowC { function_name: name_id(&names, &f.name)?,
            params: SliceC::normalized(start, f.parameters.len()),
            return_type: TypeIdC::from_source_sort(f.return_type), body: fn_body_ids[fi] });
    }
    // ---- C6: constants in literal ExprId order, post-order per literal.
    let mut constants: Vec<RawConstRow> = Vec::new();
    let mut constant_fields: Vec<usize> = Vec::new();
    for (i, t) in temps.iter().enumerate() {
        if let CoreExpr::Literal { value } = t.expr {
            let cid = emit_literal(value, &mut constants, &mut constant_fields, b)?;
            if let ExpressionRowC::Literal { constant } = &mut expressions[i] { *constant = cid; }
        }
    }
    Ok(ProgramView { entry: 0, expressions, occurrences, occurrence_paths, children,
        functions, parameters, arms, patterns, pattern_children: Vec::new(),
        types: Vec::new(), type_args: Vec::new(), constructors: Vec::new(),
        ctor_fields: Vec::new(), decl_param_counts: Vec::new(),
        decl_ctor_counts: Vec::new(), decl_templates: Vec::new(),
        plan_nodes: Vec::new(), plan_cases: Vec::new(), field_paths: Vec::new(),
        constants, constant_fields, names })
}

// --------------------------------------------------------- check_program_view
fn decode_const_value(root: usize, v: &ProgramView, max_const_nodes: usize)
    -> Result<CoreValue, ProgramViewViolation> {
    enum Frame { Enter(usize), Cons, Leaf, Branch }
    let mut results: Vec<DecodedConst> = Vec::new();
    let mut stack: Vec<Frame> = vec![Frame::Enter(root)];
    let mut visited: usize = 0;
    while let Some(fr) = stack.pop() {
        match fr {
            Frame::Enter(id) => {
                visited += 1;
                if visited > max_const_nodes {
                    return Err(pv_err(ViewCheck::V8, "constants", Some(id), "constant expansion exceeds bounds.constantNodes"));
                }
                let row = &v.constants[id];
                let fs = const_fields_of(row);
                let fids: Vec<usize> = if fs.len == 0 { Vec::new() } else {
                    v.constant_fields[fs.start..fs.start + fs.len].to_vec() };
                match row {
                    RawConstRow::U128 { n, .. } => results.push(DecodedConst::Scalar(
                        i128_class::KIntC::InRange(*n))),
                    RawConstRow::U128OutOfRange { .. } => results.push(DecodedConst::Scalar(
                        i128_class::KIntC::AboveU128)),
                    RawConstRow::Bool { b, .. } => results.push(DecodedConst::BoolVal(*b)),
                    RawConstRow::Nil { .. } => results.push(DecodedConst::List(Vec::new())),
                    RawConstRow::Cons { .. } => { stack.push(Frame::Cons);
                        stack.push(Frame::Enter(fids[1])); stack.push(Frame::Enter(fids[0])); }
                    RawConstRow::Leaf { .. } => { stack.push(Frame::Leaf);
                        stack.push(Frame::Enter(fids[0])); }
                    RawConstRow::Branch { .. } => { stack.push(Frame::Branch);
                        stack.push(Frame::Enter(fids[1])); stack.push(Frame::Enter(fids[0])); }
                    RawConstRow::Constructor { .. } => {
                        return Err(pv_err(
                            ViewCheck::V8,
                            "constants",
                            Some(id),
                            "constructor constant in dedicated Core decode",
                        ));
                    }
                }
            }
            Frame::Cons => {
                let tail = results.pop().ok_or_else(|| pv_err(ViewCheck::V8, "constants", None, "cons tail"))?;
                let head = results.pop().ok_or_else(|| pv_err(ViewCheck::V8, "constants", None, "cons head"))?;
                match (head, tail) {
                    (DecodedConst::Scalar(h), DecodedConst::List(mut t)) => {
                        let mut lst = vec![h]; lst.append(&mut t);
                        results.push(DecodedConst::List(lst));
                    }
                    _ => return Err(pv_err(ViewCheck::V8, "constants", None, "cons shape")),
                }
            }
            Frame::Leaf => match results.pop() {
                Some(DecodedConst::Scalar(n)) => results.push(DecodedConst::Tree(
                    CoreTree::Leaf { n })),
                _ => return Err(pv_err(ViewCheck::V8, "constants", None, "leaf shape")),
            },
            Frame::Branch => {
                let r = results.pop().ok_or_else(|| pv_err(ViewCheck::V8, "constants", None, "branch"))?;
                let lf = results.pop().ok_or_else(|| pv_err(ViewCheck::V8, "constants", None, "branch"))?;
                match (lf, r) {
                    (DecodedConst::Tree(a), DecodedConst::Tree(b2)) => results.push(
                        DecodedConst::Tree(CoreTree::Branch {
                            left: Box::new(a), right: Box::new(b2) })),
                    _ => return Err(pv_err(ViewCheck::V8, "constants", None, "branch shape")),
                }
            }
        }
    }
    match results.pop() {
        Some(DecodedConst::Scalar(n)) => Ok(CoreValue::UInt { n }),
        Some(DecodedConst::BoolVal(b)) => Ok(CoreValue::Bool { b }),
        Some(DecodedConst::List(xs)) => Ok(CoreValue::List { elements: xs }),
        Some(DecodedConst::Tree(t)) => Ok(CoreValue::Tree { tree: t }),
        None => Err(pv_err(ViewCheck::V8, "constants", Some(root), "empty decode")),
    }
}

fn const_row_eq(a: &RawConstRow, b: &RawConstRow) -> bool {
    marker_of(a) == marker_of(b) && const_kind_of(a) == const_kind_of(b)
        && const_fields_of(a) == const_fields_of(b)
        && match (a, b) {
            (RawConstRow::U128 { n: x, .. }, RawConstRow::U128 { n: y, .. }) => x == y,
            (RawConstRow::Bool { b: x, .. }, RawConstRow::Bool { b: y, .. }) => x == y,
            _ => true,
        }
}
fn views_equal(a: &ProgramView, b: &ProgramView) -> bool {
    a.entry == b.entry && a.expressions.len() == b.expressions.len()
        && a.occurrences == b.occurrences && a.occurrence_paths == b.occurrence_paths
        && a.children == b.children && a.functions == b.functions
        && a.parameters == b.parameters && a.arms == b.arms && a.patterns == b.patterns
        && a.pattern_children == b.pattern_children && a.types == b.types
        && a.type_args == b.type_args && a.constructors == b.constructors
        && a.ctor_fields == b.ctor_fields && a.decl_param_counts == b.decl_param_counts
        && a.decl_ctor_counts == b.decl_ctor_counts && a.decl_templates == b.decl_templates
        && a.plan_nodes == b.plan_nodes && a.plan_cases == b.plan_cases
        && a.field_paths == b.field_paths
        && a.names == b.names && a.constant_fields == b.constant_fields
        && a.constants.len() == b.constants.len()
        && a.constants.iter().zip(b.constants.iter()).all(|(x, y)| const_row_eq(x, y))
        && a.expressions.iter().zip(b.expressions.iter()).all(|(x, y)| expr_eq(x, y))
}
fn expr_eq(a: &ExpressionRowC, b: &ExpressionRowC) -> bool {
    use ExpressionRowC as E;
    match (a, b) {
        (E::Literal { constant: x }, E::Literal { constant: y }) => x == y,
        (E::Var { binder: x }, E::Var { binder: y }) => x == y,
        (E::Let { binder: p, value: q, body: r }, E::Let { binder: s, value: t, body: u })
            => p == s && q == t && r == u,
        (E::Binary { op: p, left: q, right: r }, E::Binary { op: s, left: t, right: u })
            => p == s && q == t && r == u,
        (E::Unary { op: p, operand: q }, E::Unary { op: s, operand: t }) => p == s && q == t,
        (E::If { cond: p, then_expr: q, else_expr: r }, E::If { cond: s, then_expr: t, else_expr: u })
            => p == s && q == t && r == u,
        (E::Call { function_name: p, args: q }, E::Call { function_name: s, args: t })
            => p == s && q == t,
        (E::Match { scrutinee: p, arms: q, plan: r }, E::Match { scrutinee: s, arms: t, plan: u })
            => p == s && q == t && r == u,
        (E::Construct { type_id: t1, ctor_id: c1, fields: f1, constant: k1 },
         E::Construct { type_id: t2, ctor_id: c2, fields: f2, constant: k2 })
            => t1 == t2 && c1 == c2 && f1 == f2 && k1 == k2,
        _ => false,
    }
}

pub fn check_program_view(v: &ProgramView, b: &BoundsC, l: &LayoutSchemaC)
    -> Result<(), ProgramViewViolation> {
    // V1 header + capacities (Vec lengths are the sole active lengths).
    if v.entry != 0 { return Err(pv_err(ViewCheck::V1, "header", None, "entry != 0")); }
    if v.expressions.is_empty() { return Err(pv_err(ViewCheck::V1, "expressions", None, "empty")); }
    let is_generic = !v.pattern_children.is_empty() || !v.types.is_empty() || !v.type_args.is_empty()
        || !v.constructors.is_empty() || !v.ctor_fields.is_empty() || !v.decl_param_counts.is_empty()
        || !v.decl_ctor_counts.is_empty() || !v.decl_templates.is_empty()
        || !v.plan_nodes.is_empty() || !v.plan_cases.is_empty() || !v.field_paths.is_empty()
        || v.expressions.iter().any(|e| matches!(e, ExpressionRowC::Construct { .. } | ExpressionRowC::Match { plan: Some(_), .. }))
        || v.patterns.iter().any(|p| matches!(p, PatternRowC::Wildcard | PatternRowC::Constructor { .. }))
        || v.constants.iter().any(|c| matches!(c, RawConstRow::Constructor { .. }))
        || v.parameters.iter().any(|p| p.sort.0 >= 4)
        || v.functions.iter().any(|f| f.return_type.0 >= 4);
    let caps: [(&str, usize, u128); 15] = [
        ("expressions", v.expressions.len(), b.code_rows as u128),
        ("occurrences", v.occurrences.len(), b.code_rows as u128),
        ("occurrence_paths", v.occurrence_paths.len(), l.capacities.occurrence_path_slots as u128),
        ("children", v.children.len(), b.code_child_slots as u128),
        ("functions", v.functions.len(), b.function_rows as u128),
        ("parameters", v.parameters.len(), b.parameter_slots as u128),
        ("arms", v.arms.len(), b.match_arm_rows as u128),
        ("patterns", v.patterns.len(), b.pattern_rows as u128),
        ("constants", v.constants.len(), b.constant_nodes as u128),
        ("constant_fields", v.constant_fields.len(), b.constant_field_slots as u128),
        ("names", v.names.len(), l.capacities.name_rows as u128),
        ("types", v.types.len(), if is_generic { b.type_rows as u128 } else { 0 }),
        ("constructors", v.constructors.len(), if is_generic { b.constructor_rows as u128 } else { 0 }),
        ("ctor_fields", v.ctor_fields.len(), if is_generic { b.constructor_field_slots as u128 } else { 0 }),
        ("plan_nodes", v.plan_nodes.len(), if is_generic { b.match_plan_rows as u128 } else { 0 }),
    ];
    for (t, len, cap) in caps { if len as u128 > cap {
        return Err(pv_err(ViewCheck::V1, t, None, "active length exceeds capacity")); } }
    if is_generic {
        // Generic views take the single V1..V17 route (V14 decode/reprepare). A
        // ProgramView carries no finance tables and finance is refused throughout
        // preparation, so GenericNominal/Absent is the only preparing domain.
        return super::owner_ext_admit::check_program_view_ext(
            v, None, super::owner_ext_source_program::ExtDomain::GENERIC_ABSENT, b, l)
            .map_err(|e| match e {
                super::owner_ext_type_builder::ExtTypeBuildError::CoreTooling(
                    Tooling::AdmissionRejected { detail, .. }) =>
                    pv_err(ViewCheck::V8, "program", None, &detail),
                _ => pv_err(ViewCheck::V8, "program", None, "check_program_view_ext failed"),
            });
    }
    let n_expr = v.expressions.len();
    let mut parent: Vec<Option<usize>> = vec![None; n_expr];
    let mut child_pos: Vec<usize> = vec![0; n_expr];
    // V2 expressions.
    for (i, e) in v.expressions.iter().enumerate() {
        let mut kids: Vec<(usize, usize)> = Vec::new();
        match e {
            ExpressionRowC::Literal { constant } => if *constant >= v.constants.len() {
                return Err(pv_err(ViewCheck::V2, "expressions", Some(i), "constant out of range")); },
            ExpressionRowC::Var { binder } => if *binder >= v.names.len() {
                return Err(pv_err(ViewCheck::V2, "expressions", Some(i), "binder out of range")); },
            ExpressionRowC::Let { binder, value, body } => {
                if *binder >= v.names.len() { return Err(pv_err(ViewCheck::V2, "expressions", Some(i), "let binder")); }
                kids = vec![(*value, 0), (*body, 1)];
            }
            ExpressionRowC::Binary { left, right, .. } => kids = vec![(*left, 0), (*right, 1)],
            ExpressionRowC::Unary { operand, .. } => kids = vec![(*operand, 0)],
            ExpressionRowC::If { cond, then_expr, else_expr } =>
                kids = vec![(*cond, 0), (*then_expr, 1), (*else_expr, 2)],
            ExpressionRowC::Call { function_name, args } => {
                if *function_name >= v.names.len() { return Err(pv_err(ViewCheck::V2, "expressions", Some(i), "call name")); }
                if !slice_inside(args, v.children.len()) { return Err(pv_err(ViewCheck::V2, "expressions", Some(i), "args slice")); }
                if args.len > b.max_arity { return Err(pv_err(ViewCheck::V2, "expressions", Some(i), "args arity")); }
                for (k, cid) in v.children[if args.len == 0 { 0 } else { args.start }..
                    if args.len == 0 { 0 } else { args.start + args.len }].iter().enumerate() {
                    kids.push((*cid, k));
                }
            }
            ExpressionRowC::Match { scrutinee, arms, plan } => {
                if !is_generic && plan.is_some() { return Err(pv_err(ViewCheck::V2, "expressions", Some(i), "match plan in dedicated view")); }
                if !slice_inside(arms, v.arms.len()) { return Err(pv_err(ViewCheck::V2, "expressions", Some(i), "arms slice")); }
                kids.push((*scrutinee, 0));
                if arms.len > 0 { for (k, aid) in (arms.start..arms.start + arms.len).enumerate() {
                    kids.push((v.arms[aid].body, k + 1));
                } }
            }
            ExpressionRowC::Construct { fields, .. } => {
                if !is_generic {
                    return Err(pv_err(ViewCheck::V2, "expressions", Some(i), "construct in dedicated view"));
                }
                if !slice_inside(fields, v.children.len()) { return Err(pv_err(ViewCheck::V2, "expressions", Some(i), "construct fields slice")); }
                if fields.len > b.max_arity { return Err(pv_err(ViewCheck::V2, "expressions", Some(i), "construct arity")); }
                for (k, cid) in v.children[if fields.len == 0 { 0 } else { fields.start }..
                    if fields.len == 0 { 0 } else { fields.start + fields.len }].iter().enumerate() {
                    kids.push((*cid, k));
                }
            }
        }
        for (c, pos) in kids {
            if c <= i || c >= n_expr { return Err(pv_err(ViewCheck::V2, "expressions", Some(i), "child not greater/in range")); }
            if parent[c].is_some() { return Err(pv_err(ViewCheck::V2, "expressions", Some(c), "duplicate parent")); }
            parent[c] = Some(i); child_pos[c] = pos;
        }
    }
    // V3 occurrences: owner + path equals parent path plus ChildPosition.
    if v.occurrences.len() != n_expr { return Err(pv_err(ViewCheck::V3, "occurrences", None, "length")); }
    let mut fn_of_root: Vec<Option<usize>> = vec![None; n_expr];
    for (fi, f) in v.functions.iter().enumerate() {
        if f.body < n_expr { if fn_of_root[f.body].is_some() {
            return Err(pv_err(ViewCheck::V3, "occurrences", Some(f.body), "two function roots")); }
            fn_of_root[f.body] = Some(fi); }
    }
    let mut expected_owner: Vec<(bool, usize)> = vec![(false, 0); n_expr];
    for i in 0..n_expr {
        let (is_fn, fi) = match parent[i] {
            None => if i == v.entry { (false, 0) } else {
                match fn_of_root[i] { Some(f) => (true, f),
                    None => return Err(pv_err(ViewCheck::V3, "occurrences", Some(i), "orphan root")) } },
            Some(par) => expected_owner[par],
        };
        expected_owner[i] = (is_fn, fi);
        let occ = &v.occurrences[i];
        if occ.owner_is_function != is_fn || (is_fn && occ.function != fi)
            || (!is_fn && occ.function != 0) {
            return Err(pv_err(ViewCheck::V3, "occurrences", Some(i), "owner mismatch")); }
        if !slice_inside(&occ.path, v.occurrence_paths.len()) {
            return Err(pv_err(ViewCheck::V3, "occurrences", Some(i), "path slice")); }
        let mut exp_path: Vec<usize> = Vec::new();
        let mut cur = i;
        while let Some(par) = parent[cur] { exp_path.push(child_pos[cur]); cur = par; }
        exp_path.reverse();
        let actual: &[usize] = if occ.path.len == 0 { &[] } else {
            &v.occurrence_paths[occ.path.start..occ.path.start + occ.path.len] };
        if actual != exp_path.as_slice() {
            return Err(pv_err(ViewCheck::V3, "occurrences", Some(i), "path mismatch")); }
    }
    // V4 functions: contiguous params, body root, names in range.
    let mut expect_start: usize = 0;
    let n_types = if is_generic { v.types.len() } else { 4 };
    for (fi, f) in v.functions.iter().enumerate() {
        if f.function_name >= v.names.len() { return Err(pv_err(ViewCheck::V4, "functions", Some(fi), "name")); }
        if !slice_inside(&f.params, v.parameters.len()) || (f.params.len > 0 && f.params.start != expect_start) {
            return Err(pv_err(ViewCheck::V4, "functions", Some(fi), "params not contiguous")); }
        if f.params.len > b.max_arity { return Err(pv_err(ViewCheck::V4, "functions", Some(fi), "params max_arity")); }
        expect_start = expect_start.checked_add(f.params.len).ok_or_else(|| pv_err(ViewCheck::V4, "functions", Some(fi), "parameter offset overflow"))?;
        if f.body >= n_expr || parent[f.body].is_some() || fn_of_root[f.body] != Some(fi) {
            return Err(pv_err(ViewCheck::V4, "functions", Some(fi), "body not function root")); }
        if f.return_type.0 >= n_types { return Err(pv_err(ViewCheck::V4, "functions", Some(fi), "return_type")); }
        if f.params.len > 0 { for pid in f.params.start..f.params.start + f.params.len {
            if v.parameters[pid].binder >= v.names.len() || v.parameters[pid].sort.0 >= n_types {
                return Err(pv_err(ViewCheck::V4, "parameters", Some(pid), "binder/sort")); } } }
    }
    if expect_start != v.parameters.len() { return Err(pv_err(ViewCheck::V4, "parameters", None, "trailing parameters")); }
    // V5 patterns.
    if !is_generic && v.patterns.len() != v.arms.len() { return Err(pv_err(ViewCheck::V5, "patterns", None, "count != arms")); }
    let mut seen_pat = vec![false; v.patterns.len()];
    for (ai, a) in v.arms.iter().enumerate() {
        if a.pattern >= v.patterns.len() || seen_pat[a.pattern] {
            return Err(pv_err(ViewCheck::V5, "arms", Some(ai), "pattern not bijective")); }
        seen_pat[a.pattern] = true;
        if a.body >= n_expr { return Err(pv_err(ViewCheck::V5, "arms", Some(ai), "body")); }
    }
    for (pi, pat) in v.patterns.iter().enumerate() {
        match pat {
            PatternRowC::PVar { x } | PatternRowC::PLeaf { x } => if *x >= v.names.len() {
                return Err(pv_err(ViewCheck::V5, "patterns", Some(pi), "binder")); },
            PatternRowC::PCons { x, y } | PatternRowC::PBranch { x, y } => if *x >= v.names.len() || *y >= v.names.len() {
                return Err(pv_err(ViewCheck::V5, "patterns", Some(pi), "binder")); },
            PatternRowC::PNil => {}
            PatternRowC::Wildcard => if !is_generic {
                return Err(pv_err(ViewCheck::V5, "patterns", Some(pi), "extended pattern in dedicated view"));
            },
            PatternRowC::Constructor { children, .. } => {
                if !is_generic {
                    return Err(pv_err(ViewCheck::V5, "patterns", Some(pi), "extended pattern in dedicated view"));
                }
                if !slice_inside(children, v.pattern_children.len()) {
                    return Err(pv_err(ViewCheck::V5, "patterns", Some(pi), "pattern_children slice"));
                }
                if children.len > 0 {
                    for pos in children.start..children.start + children.len {
                        let cp = v.pattern_children[pos];
                        if cp >= pi || seen_pat[cp] {
                            return Err(pv_err(ViewCheck::V5, "patterns", Some(pi), "pattern child tree"));
                        }
                        seen_pat[cp] = true;
                    }
                }
            }
        }
    }
    if seen_pat.iter().any(|s| !*s) { return Err(pv_err(ViewCheck::V5, "patterns", None, "orphan pattern row")); }
    // V6 constants: fields < j, kinds, markerFree recomputed ascending.
    let mut mf: Vec<bool> = Vec::new();
    for (j, row) in v.constants.iter().enumerate() {
        let fs = const_fields_of(row);
        if !slice_inside(&fs, v.constant_fields.len()) {
            return Err(pv_err(ViewCheck::V6, "constants", Some(j), "field slice")); }
        let arity = match const_kind_of(row) {
            LogicalKind::UInt | LogicalKind::Bool | LogicalKind::Nil | LogicalKind::Marker => 0,
            LogicalKind::Leaf => 1,
            LogicalKind::Cons | LogicalKind::Branch => 2,
            LogicalKind::Constructor { type_id, ctor_id } => {
                if !is_generic || type_id.0 < 4 || type_id.0 >= v.types.len() {
                    return Err(pv_err(ViewCheck::V6, "constants", Some(j), "constructor constant type"));
                }
                let trow = &v.types[type_id.0];
                if ctor_id.0 < trow.ctor_start.0 || ctor_id.0 - trow.ctor_start.0 >= trow.ctor_len {
                    return Err(pv_err(ViewCheck::V6, "constants", Some(j), "constructor constant ctor"));
                }
                v.constructors[ctor_id.0].field_len
            }
            LogicalKind::ConstructorExtensionReserved => return Err(pv_err(ViewCheck::V6, "constants", Some(j), "reserved kind")),
        };
        if fs.len != arity { return Err(pv_err(ViewCheck::V6, "constants", Some(j), "arity")); }
        let fids: Vec<usize> = if fs.len == 0 { Vec::new() } else {
            v.constant_fields[fs.start..fs.start + fs.len].to_vec() };
        for f in &fids { if *f >= j {
            return Err(pv_err(ViewCheck::V6, "constants", Some(j), "forward field")); } }
        if fs.len > 0 {
            let kinds: Vec<LogicalKind> = fids.iter().map(|f| const_kind_of(&v.constants[*f])).collect();
            let ok = match const_kind_of(row) {
                LogicalKind::Cons => matches!(kinds[0], LogicalKind::UInt | LogicalKind::Marker)
                    && matches!(kinds[1], LogicalKind::Nil | LogicalKind::Cons),
                LogicalKind::Leaf => matches!(kinds[0], LogicalKind::UInt | LogicalKind::Marker),
                LogicalKind::Branch => kinds.iter().all(|k| matches!(k, LogicalKind::Leaf | LogicalKind::Branch)),
                LogicalKind::Constructor { ctor_id, .. } => {
                    let crow = &v.constructors[ctor_id.0];
                    (0..fs.len).all(|k| {
                        let exp_ty = v.ctor_fields[crow.field_start + k].type_id;
                        pure_leaves::sort_of_kind(kinds[k]) == Some(exp_ty)
                            || (exp_ty == TypeIdC::UInt && kinds[k] == LogicalKind::Marker)
                    })
                }
                _ => true,
            };
            if !ok { return Err(pv_err(ViewCheck::V6, "constants", Some(j), "field kind")); }
        }
        let expect = const_kind_of(row) != LogicalKind::Marker && fids.iter().all(|f| mf[*f]);
        if marker_of(row) != expect { return Err(pv_err(ViewCheck::V6, "constants", Some(j), "markerFree")); }
        mf.push(expect);
    }
    // V7 names: strictly ascending, byte budget, every id used.
    let mut total_bytes: u128 = 0;
    for (i, n) in v.names.iter().enumerate() {
        total_bytes += n.len() as u128;
        if i > 0 && v.names[i - 1] >= *n { return Err(pv_err(ViewCheck::V7, "names", Some(i), "not ascending")); }
    }
    if total_bytes > l.capacities.name_bytes as u128 {
        return Err(pv_err(ViewCheck::V7, "names", None, "name bytes")); }
    let mut used = vec![false; v.names.len()];
    for e in &v.expressions { match e {
        ExpressionRowC::Var { binder } | ExpressionRowC::Let { binder, .. } => used[*binder] = true,
        ExpressionRowC::Call { function_name, .. } => used[*function_name] = true, _ => {} } }
    for f in &v.functions { used[f.function_name] = true; }
    for prm in &v.parameters { used[prm.binder] = true; }
    for pat in &v.patterns { match pat {
        PatternRowC::PVar { x } | PatternRowC::PLeaf { x } => used[*x] = true,
        PatternRowC::PCons { x, y } | PatternRowC::PBranch { x, y } => { used[*x] = true; used[*y] = true; }
        PatternRowC::PNil | PatternRowC::Wildcard | PatternRowC::Constructor { .. } => {} } }
    if is_generic {
        for t in &v.types { if t.name_id < used.len() { used[t.name_id] = true; } }
        for c in &v.constructors { if c.name_id < used.len() { used[c.name_id] = true; } }
        for cf in &v.ctor_fields { if cf.name_id < used.len() { used[cf.name_id] = true; } }
        for pn in &v.plan_nodes { if let PlanNodeC::Bind { binder, .. } = pn { if *binder < used.len() { used[*binder] = true; } } }
    }
    for (i, u) in used.iter().enumerate() { if !u && !is_generic {
        return Err(pv_err(ViewCheck::V7, "names", Some(i), "unused name")); } }
    if is_generic {
        return Ok(());
    }
    // V8 dedicated Core round-trip.
    let mut decoded: Vec<Option<CoreExpr>> = (0..n_expr).map(|_| None).collect();
    for i in (0..n_expr).rev() {
        let e = match &v.expressions[i] {
            ExpressionRowC::Literal { constant } => CoreExpr::Literal {
                value: decode_const_value(*constant, v, b.constant_nodes)? },
            ExpressionRowC::Var { binder } => CoreExpr::Var { name: v.names[*binder].clone() },
            ExpressionRowC::Let { binder, value, body } => {
                let val = decoded[*value].take().ok_or_else(|| pv_err(ViewCheck::V8, "expressions", Some(i), "let value taken"))?;
                let bod = decoded[*body].take().ok_or_else(|| pv_err(ViewCheck::V8, "expressions", Some(i), "let body taken"))?;
                CoreExpr::Let { name: v.names[*binder].clone(),
                    value: Box::new(val), body: Box::new(bod) } }
            ExpressionRowC::Binary { op, left, right } => {
                let lf = decoded[*left].take().ok_or_else(|| pv_err(ViewCheck::V8, "expressions", Some(i), "binary left"))?;
                let rt = decoded[*right].take().ok_or_else(|| pv_err(ViewCheck::V8, "expressions", Some(i), "binary right"))?;
                CoreExpr::Binary { op: *op, left: Box::new(lf), right: Box::new(rt) } }
            ExpressionRowC::Unary { op, operand } => {
                let o = decoded[*operand].take().ok_or_else(|| pv_err(ViewCheck::V8, "expressions", Some(i), "unary"))?;
                CoreExpr::Unary { op: *op, operand: Box::new(o) } }
            ExpressionRowC::If { cond, then_expr, else_expr } => {
                let c = decoded[*cond].take().ok_or_else(|| pv_err(ViewCheck::V8, "expressions", Some(i), "if cond"))?;
                let t = decoded[*then_expr].take().ok_or_else(|| pv_err(ViewCheck::V8, "expressions", Some(i), "if then"))?;
                let el = decoded[*else_expr].take().ok_or_else(|| pv_err(ViewCheck::V8, "expressions", Some(i), "if else"))?;
                CoreExpr::If { cond: Box::new(c), then_expr: Box::new(t), else_expr: Box::new(el) } }
            ExpressionRowC::Call { function_name, args } => {
                let mut argv: Vec<CoreExpr> = Vec::new();
                if args.len > 0 { for cid in &v.children[args.start..args.start + args.len] {
                    argv.push(decoded[*cid].take().ok_or_else(|| pv_err(ViewCheck::V8, "expressions", Some(i), "call arg"))?); } }
                CoreExpr::Call { name: v.names[*function_name].clone(), args: argv } }
            ExpressionRowC::Match { scrutinee, arms, plan } => {
                if plan.is_some() { return Err(pv_err(ViewCheck::V8, "expressions", Some(i), "match plan")); }
                let scr = decoded[*scrutinee].take().ok_or_else(|| pv_err(ViewCheck::V8, "expressions", Some(i), "scrutinee"))?;
                let mut out: Vec<CoreArm> = Vec::new();
                if arms.len > 0 { for aid in arms.start..arms.start + arms.len {
                    let arow = &v.arms[aid];
                    let pat = match &v.patterns[arow.pattern] {
                        PatternRowC::PVar { x } => CorePattern::PVar { x: v.names[*x].clone() },
                        PatternRowC::PNil => CorePattern::PNil,
                        PatternRowC::PCons { x, y } => CorePattern::PCons {
                            x: v.names[*x].clone(), y: v.names[*y].clone() },
                        PatternRowC::PLeaf { x } => CorePattern::PLeaf { x: v.names[*x].clone() },
                        PatternRowC::PBranch { x, y } => CorePattern::PBranch {
                            x: v.names[*x].clone(), y: v.names[*y].clone() },
                        PatternRowC::Wildcard | PatternRowC::Constructor { .. } => {
                            return Err(pv_err(ViewCheck::V8, "patterns", Some(arow.pattern), "extended pattern"));
                        }
                    };
                    let bod = decoded[arow.body].take().ok_or_else(|| pv_err(ViewCheck::V8, "expressions", Some(i), "arm body"))?;
                    out.push(CoreArm { pattern: pat, body: bod }); } }
                CoreExpr::Match { scrutinee: Box::new(scr), arms: out } }
            ExpressionRowC::Construct { .. } => {
                return Err(pv_err(ViewCheck::V8, "expressions", Some(i), "construct"));
            }
        };
        decoded[i] = Some(e);
    }
    let entry = decoded[v.entry].take().ok_or_else(|| pv_err(ViewCheck::V8, "expressions", None, "entry taken"))?;
    fn decode_source_sort(sort: TypeIdC, table: &str, row: usize, detail: &str)
        -> Result<SortTag, ProgramViewViolation> {
        match sort {
            TypeIdC::UInt => Ok(SortTag::UInt),
            TypeIdC::Bool => Ok(SortTag::Bool),
            TypeIdC::List => Ok(SortTag::List),
            TypeIdC::Tree => Ok(SortTag::Tree),
            _ => Err(pv_err(ViewCheck::V8, table, Some(row), detail)),
        }
    }
    let mut funcs: Vec<CoreFunction> = Vec::new();
    for (fi, f) in v.functions.iter().enumerate() {
        let bod = decoded[f.body].take().ok_or_else(|| pv_err(ViewCheck::V8, "functions", None, "function body taken"))?;
        let mut params: Vec<CoreParameter> = Vec::new();
        if f.params.len > 0 { for pid in f.params.start..f.params.start + f.params.len {
            let sort = decode_source_sort(
                v.parameters[pid].sort, "parameters", pid, "non-builtin parameter sort")?;
            params.push(CoreParameter { name: v.names[v.parameters[pid].binder].clone(),
                sort }); } }
        let return_type = decode_source_sort(
            f.return_type, "functions", fi, "non-builtin return type")?;
        funcs.push(CoreFunction { name: v.names[f.function_name].clone(),
            parameters: params, return_type, body: bod });
    }
    let prog = CoreProgram { functions: funcs, entry };
    let reprepared = prepare_core(&prog, b, l)
        .map_err(|_| pv_err(ViewCheck::V8, "program", None, "re-prepare failed"))?;
    if !views_equal(v, &reprepared) {
        return Err(pv_err(ViewCheck::V8, "program", None, "view != prepare_core(decode(view))"));
    }
    Ok(())
}

// ------------------------------------------------------------------- admit
pub fn admit(p: &CoreProgram, b: &BoundsC, l: &LayoutSchemaC)
    -> Result<Admitted, Tooling> {
    // AD-0a layout.bounds == bounds, authoritative copy is the `b` argument.
    if l.bounds != *b {
        return Err(Tooling::AdmissionRejected { reason: AdmissionReason::LayoutBoundsMismatch,
            detail: "layout.bounds != bounds".to_string(), locator: field_loc("layout.bounds") });
    }
    // AD-1 stepCap.
    if b.step_cap < 1 {
        return Err(cap_err(CapacityKind::StepCap, 1, b.step_cap, field_loc("bounds.stepCap")));
    }
    // AD-2a counter widths in WorkCounter, HelperCounter, ReferenceCursor order.
    if let Some(w) = &l.widths {
        let checks: [(CounterWidthField, u128, &str); 3] = [
            (CounterWidthField::WorkCounter, bitlen_count(b.max_reference_work), "WorkCounter"),
            (CounterWidthField::HelperCounter, bitlen_count(b.max_helper_work), "HelperCounter"),
            (CounterWidthField::ReferenceCursor, bitlen_count(b.max_reference_work), "ReferenceCursor"),
        ];
        for (field, needed, nm) in checks {
            let limit = w.counter_width(&field) as u128;
            if limit < needed {
                return Err(cap_err(CapacityKind::CounterWidth { field }, needed, limit,
                    field_loc(&format!("layout.widths.counterWidths.{nm}"))));
            }
        }
        // AD-2b index widths in derived_widths order. Absent entry reads 0.
        let idx: [(&str, u128); 20] = [
            ("ExprId", bitlen_id(b.code_rows as u128)),
            ("ChildPos", bitlen_count(b.code_child_slots as u128)),
            ("FunctionRowId", bitlen_id(b.function_rows as u128)),
            ("FunctionCursor", bitlen_count(b.function_rows as u128)),
            ("ParamPos", bitlen_count(b.parameter_slots as u128)),
            ("ArmId", bitlen_id(b.match_arm_rows as u128)),
            ("ArmPos", bitlen_count(b.match_arm_rows as u128)),
            ("PatternRowId", bitlen_id(b.pattern_rows as u128)),
            ("ConstId", bitlen_id(b.constant_nodes as u128)),
            ("ConstFieldPos", bitlen_count(b.constant_field_slots as u128)),
            ("DataId", bitlen_id(b.data_nodes as u128)),
            ("FieldPos", bitlen_count(b.data_field_slots as u128)),
            ("EnvId", bitlen_id(b.environment_cells as u128)),
            ("NameId", bitlen_id(l.capacities.name_rows as u128)),
            ("OccPathPos", bitlen_count(l.capacities.occurrence_path_slots as u128)),
            ("ChildPosition", bitlen_count((b.max_arity as u128)
                .max(b.match_arm_rows as u128 + 1))),
            ("ValueSlotPos", bitlen_count(l.capacities.value_slots as u128)),
            ("ArityCount", bitlen_count(b.max_arity as u128)),
            ("Depth", bitlen_count(b.frames as u128)),
            ("TaskCount", bitlen_count(b.helper_tasks as u128)),
        ];
        for (nm, needed) in idx {
            let limit = w.index_width(nm) as u128;
            if limit < needed {
                return Err(Tooling::AdmissionRejected {
                    reason: AdmissionReason::LayoutWidthTooSmall,
                    detail: nm.to_string(), locator: field_loc(&format!("layout.widths.indexWidths.{nm}")) });
            }
        }
        // AD-2b record widths: R4 derived_widths supplies index and
        // counter formulas, but only a recordWidths map/ordering pointer
        // with no per-record formula in the supplied schema. This
        // HostOnly lane therefore refuses Bounded admission after the
        // AD-2a and AD-2b index checks above, instead of waiving it.
        return Err(Tooling::AdmissionRejected {
            reason: AdmissionReason::LayoutWidthTooSmall,
            detail: "recordWidths".to_string(),
            locator: field_loc("layout.widths.recordWidths") });
    }
    // AD-3 maxArity.
    if b.max_arity < 2 {
        return Err(cap_err(CapacityKind::MaxArity, 2, b.max_arity as u128,
            field_loc("bounds.maxArity")));
    }
    // AD-4: statement only under DedicatedSorts; produces nothing.
    // AD-5 prepare then view check.
    let view = prepare_core(p, b, l)?;
    if let Err(viol) = check_program_view(&view, b, l) {
        return Err(Tooling::AdmissionRejected {
            reason: AdmissionReason::ProgramViewMalformed,
            detail: format!("{:?}:{}", viol.check, viol.detail), locator: None });
    }
    // AD-6 boundary capacities must be absent.
    if l.boundary_capacities_present {
        return Err(Tooling::UnsupportedFeature { feature: FeatureKind::BoundarySection,
            locator: field_loc("layout.boundaryCapacities") });
    }
    let identity = IdentityC { profile_id: 2, grammar_domain: GrammarDomain::DedicatedSorts,
        finance: FinanceInput::Absent, layout_version: l.layout_version.clone(),
        layout_digest: DigestC::Unselected, bounds_digest: DigestC::Unselected,
        field_id: l.field_id.clone(), program_membership_digest: DigestC::Unselected };
    Ok(Admitted { body: Arc::new(AdmittedBody { identity, bounds: *b, view,
        layout: l.clone() }) })
}

// ---------------------------------------------------------------- admit_run
// Both genesis forms store the one RunIdC that admit_run built from the
// original fuel/stack RunIntC inputs; registers below carry converted values.
pub(super) fn genesis(a: &Admitted, run_id: Arc<RunIdC>, valid: bool, fuel: u128, stack: usize) -> MachineImage {
    MachineImage {
        admission: Admitted { body: Arc::clone(&a.body) }, run_id,
        control: if valid { ControlC::Eval { expr: 0 } } else { ControlC::Error { code: 1 } },
        control_origin: 0, env_root: None,
        stack_limit: if valid { stack } else { 0 }, outcome: OutcomeC::Running,
        initial_fuel: if valid { fuel } else { 0 },
        remaining_work: if valid { fuel } else { 0 },
        consumed_work: 0, helper_work: 0, helper_visits: 0, reference_cursor: 0,
        alloc: AllocCounters::default(), active: ActiveLengths::default(),
        seg_base: host_implied_prefix::SegBase::genesis_uncut(),
        helper: None, result: None,
        data: Vec::new(), data_fields: Vec::new(), env: Vec::new(),
        frames: Vec::new(), tasks: Vec::new(), value_slots: Vec::new(),
    }
}
pub fn admit_run(a: &Admitted, fuel: &RunIntC, stack_limit: &RunIntC,
    finance: &FinanceInput) -> Result<MachineImage, Tooling> {
    let bounds = a.body.bounds;
    let caps = a.body.layout.capacities;
    // R1 finance.
    if *finance != FinanceInput::Absent {
        return Err(Tooling::UnsupportedFeature {
            feature: FeatureKind::FinancialInput, locator: None });
    }
    // RunId (schema identity.RunId) from the original three-class inputs,
    // built before R2 returns and before R3/R4 convert fuel and stack_limit.
    let run_id = Arc::new(RunIdC { identity: a.body.identity.clone(),
        fuel_input: *fuel, stack_limit_input: *stack_limit });
    // R2 negative -> genesis_invalid, not a refusal.
    if matches!(fuel, RunIntC::Negative) || matches!(stack_limit, RunIntC::Negative) {
        return Ok(genesis(a, run_id, false, 0, 0));
    }
    // R3 fuel bound.
    let fuel_v: u128 = match fuel {
        RunIntC::Nat(v) => *v,
        RunIntC::NatAboveU128 => return Err(cap_err_above_u128(
            CapacityKind::MaxReferenceWork, bounds.max_reference_work, None)),
        // Already returned by R2 above; same genesis disposition, no panic.
        RunIntC::Negative => return Ok(genesis(a, run_id, false, 0, 0)),
    };
    if fuel_v > bounds.max_reference_work {
        return Err(cap_err(CapacityKind::MaxReferenceWork, fuel_v,
            bounds.max_reference_work, None));
    }
    // R4 stack bound.
    let stack_v: u128 = match stack_limit {
        RunIntC::Nat(v) => *v,
        RunIntC::NatAboveU128 => return Err(cap_err_above_u128(
            CapacityKind::Frames, bounds.frames as u128, None)),
        // Already returned by R2 above; same genesis disposition, no panic.
        RunIntC::Negative => return Ok(genesis(a, run_id, false, 0, 0)),
    };
    if stack_v > bounds.frames as u128 {
        return Err(cap_err(CapacityKind::Frames, stack_v, bounds.frames as u128, None));
    }
    // R5 value slots.
    if fuel_v > caps.value_slots as u128 {
        return Err(cap_err(CapacityKind::ValueSlots, fuel_v, caps.value_slots as u128, None));
    }
    // R6 genesis_valid. stack_v <= frames (usize) so conversion cannot fail;
    // fail closed instead of unwrapping.
    let stack_usize = usize::try_from(stack_v).map_err(|_| cap_err(
        CapacityKind::Frames, stack_v, bounds.frames as u128, None))?;
    Ok(genesis(a, run_id, true, fuel_v, stack_usize))
}

#[cfg(test)]
mod v4_empty_params_tests;
#[cfg(test)]
mod nominal_decl_producer_tests;
