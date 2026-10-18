//! shared::owner_ext_type_builder -- Reconciled owner type/declaration carrier and
//! bounded T1/T2 FIFO type/constructor/field table builder.
//! Does not claim ProgramView, Admitted, or full EXT-01 completion.

use std::collections::VecDeque;
use super::adt::{CtorIdC, TypeIdC, TypeRowC};
use super::adt_storage::{CtorFieldRowExt, CtorRowExt};
use super::nominal_declaration_order::DeclarationFieldOccurrence;
use super::{
    BoundsC, CapacityKind, FeatureKind, GrammarDomain, HostViolation, HostViolationKind, LocatorC,
    NeededNatC, SliceC, Tooling,
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct DeclIdxC(pub usize);

pub enum TypeExprSrc {
    UInt,
    Bool,
    List,
    Tree,
    Named { name: String },
    Apply { name: String, args: Vec<TypeExprSrc> },
    Param { name: String },
}
impl Drop for TypeExprSrc {
    fn drop(&mut self) {
        let mut stack = Vec::new();
        if let TypeExprSrc::Apply { args, .. } = self {
            stack.append(args);
        }
        while let Some(mut node) = stack.pop() {
            if let TypeExprSrc::Apply { args, .. } = &mut node {
                stack.append(args);
            }
        }
    }
}
pub struct CtorFieldSrc {
    pub name: String,
    pub ty: TypeExprSrc,
}
pub struct CtorDeclSrc {
    pub name: String,
    pub fields: Vec<CtorFieldSrc>,
}
pub struct TypeDeclSrc {
    pub name: String,
    pub parameters: Vec<String>,
    pub ctors: Vec<CtorDeclSrc>,
}

/// Owner-authenticated Pass B declaration NameId bindings.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DeclNameIds {
    pub type_name_id: usize,
    pub ctor_name_ids: Vec<usize>,
    pub field_name_ids: Vec<Vec<usize>>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PassBTypeNameIds {
    pub name_count: usize,
    pub decls: Vec<DeclNameIds>,
    owner_verified: bool,
}

impl PassBTypeNameIds {
    pub(super) fn new_owner_verified(name_count: usize, decls: Vec<DeclNameIds>) -> Self {
        Self {
            name_count,
            decls,
            owner_verified: true,
        }
    }

    pub fn is_owner_verified(&self) -> bool {
        self.owner_verified
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ExtTypeCapKind {
    TypeRows,
    ConstructorRows,
    ConstructorFieldSlots,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ExtTypeRejectReason {
    DuplicateTypeName,
    ReservedTypeName,
    UnknownType,
    TypeArity,
    ConstructorArity,
    DuplicateCtorName,
    DuplicateFieldName,
    UnknownCtor,
    BuiltinConstruct,
    PatternArity,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ExtTypeFeatureKind {
    TypeVariableInSignature,
}
pub enum ExtTypeBuildError {
    CoreTooling(Tooling),
    ExtCapacityNotAdmitted {
        kind: ExtTypeCapKind,
        needed: NeededNatC,
        limit: u128,
        locator: Option<LocatorC>,
    },
    ExtAdmissionRejected {
        reason: ExtTypeRejectReason,
        detail: String,
        locator: Option<LocatorC>,
    },
    ExtUnsupportedFeature {
        feature: ExtTypeFeatureKind,
        locator: Option<LocatorC>,
    },
    Host(HostViolation),
}

fn hv(kind: HostViolationKind, loc: &str) -> ExtTypeBuildError {
    ExtTypeBuildError::Host(HostViolation {
        kind,
        clause: None,
        rule: None,
        locator: Some(loc.to_string()),
    })
}
fn oob(loc: &str) -> ExtTypeBuildError {
    hv(HostViolationKind::ReadOutOfBounds, loc)
}
fn wrong(loc: &str) -> ExtTypeBuildError {
    hv(HostViolationKind::WrongVariant, loc)
}
fn arith(loc: &str) -> ExtTypeBuildError {
    hv(HostViolationKind::ArithmeticUndefined, loc)
}
fn ext_cap(
    kind: ExtTypeCapKind,
    needed: usize,
    limit: usize,
    locator: Option<LocatorC>,
) -> ExtTypeBuildError {
    ExtTypeBuildError::ExtCapacityNotAdmitted {
        kind,
        needed: NeededNatC::Exact(needed as u128),
        limit: limit as u128,
        locator,
    }
}
fn max_arity_err(needed: usize, limit: usize, locator: Option<LocatorC>) -> ExtTypeBuildError {
    ExtTypeBuildError::CoreTooling(Tooling::CapacityNotAdmitted {
        kind: CapacityKind::MaxArity,
        needed: NeededNatC::Exact(needed as u128),
        limit: limit as u128,
        locator,
    })
}
fn reject(
    reason: ExtTypeRejectReason,
    detail: &str,
    locator: Option<LocatorC>,
) -> ExtTypeBuildError {
    ExtTypeBuildError::ExtAdmissionRejected {
        reason,
        detail: detail.to_string(),
        locator,
    }
}
fn is_reserved_type_name(s: &str) -> bool {
    matches!(s, "UInt" | "Bool" | "List" | "Tree")
}

pub struct CheckedTypeDeclView<'a> {
    decls: &'a [TypeDeclSrc],
    names: &'a PassBTypeNameIds,
    bounds: BoundsC,
    non_param_decls: Vec<DeclIdxC>,
}
impl<'a> CheckedTypeDeclView<'a> {
    pub fn new_checked(
        domain: GrammarDomain,
        decls: &'a [TypeDeclSrc],
        names: &'a PassBTypeNameIds,
        bounds: &BoundsC,
    ) -> Result<Self, ExtTypeBuildError> {
        if !names.is_owner_verified() {
            return Err(wrong("unverified PassBTypeNameIds"));
        }
        if domain == GrammarDomain::DedicatedSorts && !decls.is_empty() {
            return Err(ExtTypeBuildError::CoreTooling(
                Tooling::UnsupportedFeature {
                    feature: FeatureKind::GenericAdt,
                    locator: Some(LocatorC::Table {
                        table: "types".to_string(),
                        row: 0,
                    }),
                },
            ));
        }
        let needed_types = 4usize
            .checked_add(decls.len())
            .ok_or_else(|| arith("I-4 4 + decls.len"))?;
        if needed_types > bounds.type_rows {
            return Err(ext_cap(
                ExtTypeCapKind::TypeRows,
                needed_types,
                bounds.type_rows,
                None,
            ));
        }
        if names.decls.len() != decls.len() {
            return Err(wrong("PassBTypeNameIds decl len"));
        }
        let mut non_param_decls = Vec::new();
        for (d, decl) in decls.iter().enumerate() {
            let loc = Some(LocatorC::Table {
                table: "types".to_string(),
                row: d,
            });
            if decls[..d].iter().any(|prev| prev.name == decl.name) {
                return Err(reject(
                    ExtTypeRejectReason::DuplicateTypeName,
                    &decl.name,
                    loc,
                ));
            }
            if is_reserved_type_name(&decl.name) {
                return Err(reject(
                    ExtTypeRejectReason::ReservedTypeName,
                    &decl.name,
                    loc,
                ));
            }
            if decl.parameters.len() > bounds.max_arity {
                return Err(max_arity_err(decl.parameters.len(), bounds.max_arity, loc));
            }
            for (p_idx, p_name) in decl.parameters.iter().enumerate() {
                if decl.parameters[..p_idx].iter().any(|prev| prev == p_name) {
                    return Err(reject(ExtTypeRejectReason::DuplicateTypeName, p_name, loc));
                }
            }
            if decl.ctors.is_empty() {
                return Err(reject(
                    ExtTypeRejectReason::ConstructorArity,
                    "declaration has no constructors",
                    loc,
                ));
            }
            for (c_idx, ctor) in decl.ctors.iter().enumerate() {
                if decl.ctors[..c_idx].iter().any(|prev| prev.name == ctor.name) {
                    return Err(reject(
                        ExtTypeRejectReason::DuplicateCtorName,
                        &ctor.name,
                        loc,
                    ));
                }
            }
            for ctor in &decl.ctors {
                if ctor.fields.len() > bounds.max_arity {
                    return Err(max_arity_err(ctor.fields.len(), bounds.max_arity, loc));
                }
                for (f_idx, field) in ctor.fields.iter().enumerate() {
                    if ctor.fields[..f_idx]
                        .iter()
                        .any(|prev| prev.name == field.name)
                    {
                        return Err(reject(
                            ExtTypeRejectReason::DuplicateFieldName,
                            &field.name,
                            loc,
                        ));
                    }
                }
            }
            let dn = &names.decls[d];
            if dn.type_name_id >= names.name_count
                || dn.ctor_name_ids.len() != decl.ctors.len()
                || dn.field_name_ids.len() != decl.ctors.len()
            {
                return Err(oob("PassBTypeNameIds decl/ctor shape"));
            }
            for (c_idx, ctor) in decl.ctors.iter().enumerate() {
                if dn.ctor_name_ids[c_idx] >= names.name_count
                    || dn.field_name_ids[c_idx].len() != ctor.fields.len()
                    || dn.field_name_ids[c_idx]
                        .iter()
                        .any(|&fid| fid >= names.name_count)
                {
                    return Err(oob("PassBTypeNameIds ctor/field NameId"));
                }
            }
            if decl.parameters.is_empty() {
                non_param_decls.push(DeclIdxC(d));
            }
        }
        Ok(Self {
            decls,
            names,
            bounds: *bounds,
            non_param_decls,
        })
    }

    pub fn validate_declaration_type_occurrence(
        &self,
        occ: DeclarationFieldOccurrence,
        expr: &TypeExprSrc,
    ) -> Result<(), ExtTypeBuildError> {
        let decl = self
            .decls
            .get(occ.declaration.0)
            .ok_or_else(|| oob("occ decl idx"))?;
        let loc = Some(LocatorC::Table {
            table: "types".to_string(),
            row: occ.declaration.0,
        });
        match expr {
            TypeExprSrc::UInt | TypeExprSrc::Bool | TypeExprSrc::List | TypeExprSrc::Tree => Ok(()),
            TypeExprSrc::Param { name } => {
                if decl.parameters.iter().any(|p| p == name) {
                    Ok(())
                } else {
                    Err(reject(ExtTypeRejectReason::UnknownType, name, loc))
                }
            }
            TypeExprSrc::Named { name } => {
                let target = self
                    .decls
                    .iter()
                    .find(|d| &d.name == name)
                    .ok_or_else(|| reject(ExtTypeRejectReason::UnknownType, name, loc.clone()))?;
                if !target.parameters.is_empty() {
                    return Err(reject(ExtTypeRejectReason::TypeArity, name, loc));
                }
                Ok(())
            }
            TypeExprSrc::Apply { name, args } => {
                if args.len() > self.bounds.max_arity {
                    return Err(max_arity_err(args.len(), self.bounds.max_arity, loc));
                }
                let target = self
                    .decls
                    .iter()
                    .find(|d| &d.name == name)
                    .ok_or_else(|| reject(ExtTypeRejectReason::UnknownType, name, loc.clone()))?;
                if target.parameters.is_empty() || args.len() != target.parameters.len() {
                    return Err(reject(ExtTypeRejectReason::TypeArity, name, loc));
                }
                Ok(())
            }
        }
    }

    pub fn decl_count(&self) -> usize {
        self.decls.len()
    }
    pub fn non_parameterized_decls(&self) -> &[DeclIdxC] {
        &self.non_param_decls
    }
    pub fn param_count(&self, d: DeclIdxC) -> Result<usize, ExtTypeBuildError> {
        Ok(self
            .decls
            .get(d.0)
            .ok_or_else(|| oob("decl idx"))?
            .parameters
            .len())
    }
    pub fn ctor_count(&self, d: DeclIdxC) -> Result<usize, ExtTypeBuildError> {
        Ok(self
            .decls
            .get(d.0)
            .ok_or_else(|| oob("decl idx"))?
            .ctors
            .len())
    }
    pub fn field_count(&self, d: DeclIdxC, ctor_ord: usize) -> Result<usize, ExtTypeBuildError> {
        Ok(self
            .decls
            .get(d.0)
            .ok_or_else(|| oob("decl idx"))?
            .ctors
            .get(ctor_ord)
            .ok_or_else(|| oob("ctor ord"))?
            .fields
            .len())
    }
    pub fn field_type_expr(
        &self,
        d: DeclIdxC,
        ctor_ord: usize,
        field_ord: usize,
    ) -> Result<&'a TypeExprSrc, ExtTypeBuildError> {
        Ok(&self
            .decls
            .get(d.0)
            .ok_or_else(|| oob("decl idx"))?
            .ctors
            .get(ctor_ord)
            .ok_or_else(|| oob("ctor ord"))?
            .fields
            .get(field_ord)
            .ok_or_else(|| oob("field ord"))?
            .ty)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct DequeuedTypeKey {
    pub type_id: TypeIdC,
    pub decl: DeclIdxC,
    pub ctor_count: usize,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ResolvedCtorUse {
    pub type_id: TypeIdC,
    pub ctor_ordinal: usize,
}
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BuiltAdtTypeTables {
    pub types: Vec<TypeRowC>,
    pub type_args: Vec<TypeIdC>,
    pub constructors: Vec<CtorRowExt>,
    pub ctor_fields: Vec<CtorFieldRowExt>,
    pub decl_param_counts: Vec<usize>,
    pub decl_ctor_counts: Vec<usize>,
    pub max_arity: usize,
    pub name_count: usize,
}

struct ActiveDequeue {
    type_id: TypeIdC,
    decl: DeclIdxC,
    next_ctor: usize,
    active_ctor: Option<(CtorIdC, usize, usize)>,
}
pub struct OwnerExtTypeBuilder<'a> {
    view: &'a CheckedTypeDeclView<'a>,
    types: Vec<TypeRowC>,
    type_args: Vec<TypeIdC>,
    constructors: Vec<CtorRowExt>,
    ctor_fields: Vec<CtorFieldRowExt>,
    type_done: Vec<bool>,
    fifo: VecDeque<TypeIdC>,
    t1_cursor: usize,
    t1_finished: bool,
    active: Option<ActiveDequeue>,
}
impl<'a> OwnerExtTypeBuilder<'a> {
    pub fn new_checked(view: &'a CheckedTypeDeclView<'a>) -> Result<Self, ExtTypeBuildError> {
        if view.bounds.type_rows < 4 {
            return Err(ext_cap(
                ExtTypeCapKind::TypeRows,
                4,
                view.bounds.type_rows,
                None,
            ));
        }
        if view.bounds.constructor_rows < 4 {
            return Err(ext_cap(
                ExtTypeCapKind::ConstructorRows,
                4,
                view.bounds.constructor_rows,
                None,
            ));
        }
        if view.bounds.constructor_field_slots < 5 {
            return Err(ext_cap(
                ExtTypeCapKind::ConstructorFieldSlots,
                5,
                view.bounds.constructor_field_slots,
                None,
            ));
        }
        let types = vec![
            TypeRowC {
                name_id: 0,
                decl: None,
                args: SliceC::empty(),
                ctor_start: CtorIdC(0),
                ctor_len: 0,
            },
            TypeRowC {
                name_id: 0,
                decl: None,
                args: SliceC::empty(),
                ctor_start: CtorIdC(0),
                ctor_len: 0,
            },
            TypeRowC {
                name_id: 0,
                decl: None,
                args: SliceC::empty(),
                ctor_start: CtorIdC(0),
                ctor_len: 2,
            },
            TypeRowC {
                name_id: 0,
                decl: None,
                args: SliceC::empty(),
                ctor_start: CtorIdC(2),
                ctor_len: 2,
            },
        ];
        let constructors = vec![
            CtorRowExt {
                owning_type: TypeIdC(2),
                name_id: 0,
                source_order: 0,
                field_start: 0,
                field_len: 0,
            },
            CtorRowExt {
                owning_type: TypeIdC(2),
                name_id: 0,
                source_order: 1,
                field_start: 0,
                field_len: 2,
            },
            CtorRowExt {
                owning_type: TypeIdC(3),
                name_id: 0,
                source_order: 0,
                field_start: 2,
                field_len: 1,
            },
            CtorRowExt {
                owning_type: TypeIdC(3),
                name_id: 0,
                source_order: 1,
                field_start: 3,
                field_len: 2,
            },
        ];
        let ctor_fields = vec![
            CtorFieldRowExt {
                name_id: 0,
                type_id: TypeIdC(0),
            },
            CtorFieldRowExt {
                name_id: 0,
                type_id: TypeIdC(2),
            },
            CtorFieldRowExt {
                name_id: 0,
                type_id: TypeIdC(0),
            },
            CtorFieldRowExt {
                name_id: 0,
                type_id: TypeIdC(3),
            },
            CtorFieldRowExt {
                name_id: 0,
                type_id: TypeIdC(3),
            },
        ];
        Ok(Self {
            view,
            types,
            type_args: Vec::new(),
            constructors,
            ctor_fields,
            type_done: vec![true, true, true, true],
            fifo: VecDeque::new(),
            t1_cursor: 0,
            t1_finished: view.non_param_decls.is_empty(),
            active: None,
        })
    }
    pub fn register_non_parameterized_decl(
        &mut self,
        d: DeclIdxC,
    ) -> Result<TypeIdC, ExtTypeBuildError> {
        if self.t1_finished {
            return Err(wrong("T1 already finished"));
        }
        let expected = self
            .view
            .non_param_decls
            .get(self.t1_cursor)
            .copied()
            .ok_or_else(|| wrong("T1 cursor"))?;
        if d != expected {
            return Err(wrong("T1 non-parameterized source order"));
        }
        let next_len = self
            .types
            .len()
            .checked_add(1)
            .ok_or_else(|| arith("types len"))?;
        if next_len > self.view.bounds.type_rows {
            return Err(ext_cap(
                ExtTypeCapKind::TypeRows,
                next_len,
                self.view.bounds.type_rows,
                None,
            ));
        }
        let tid = TypeIdC(self.types.len());
        self.types.push(TypeRowC {
            name_id: self.view.names.decls[d.0].type_name_id,
            decl: Some(d.0),
            args: SliceC::empty(),
            ctor_start: CtorIdC(0),
            ctor_len: 0,
        });
        self.type_done.push(false);
        self.fifo.push_back(tid);
        self.t1_cursor += 1;
        if self.t1_cursor == self.view.non_param_decls.len() {
            self.t1_finished = true;
        }
        Ok(tid)
    }
    pub fn discover_or_intern_key(
        &mut self,
        d: DeclIdxC,
        args: &[TypeIdC],
        locator: Option<LocatorC>,
    ) -> Result<TypeIdC, ExtTypeBuildError> {
        if !self.t1_finished {
            return Err(wrong("T1 non-parameterized registration incomplete"));
        }
        let decl = self
            .view
            .decls
            .get(d.0)
            .ok_or_else(|| oob("discover decl idx"))?;
        if args.len() != decl.parameters.len() {
            return Err(reject(ExtTypeRejectReason::TypeArity, &decl.name, locator));
        }
        for a in args {
            if a.0 >= self.types.len() {
                return Err(oob("instance arg TypeId"));
            }
        }
        for (i, row) in self.types.iter().enumerate().skip(4) {
            if row.decl == Some(d.0)
                && row.args.len == args.len()
                && &self.type_args[row.args.start..row.args.start + row.args.len] == args
            {
                return Ok(TypeIdC(i));
            }
        }
        let next_types = self
            .types
            .len()
            .checked_add(1)
            .ok_or_else(|| arith("types len"))?;
        if next_types > self.view.bounds.type_rows {
            return Err(ext_cap(
                ExtTypeCapKind::TypeRows,
                next_types,
                self.view.bounds.type_rows,
                locator,
            ));
        }
        let args_start = self.type_args.len();
        for &arg_tid in args {
            let next_args = self
                .type_args
                .len()
                .checked_add(1)
                .ok_or_else(|| arith("typeArgs len"))?;
            if next_args > self.view.bounds.type_rows {
                return Err(ext_cap(
                    ExtTypeCapKind::TypeRows,
                    next_args,
                    self.view.bounds.type_rows,
                    locator,
                ));
            }
            self.type_args.push(arg_tid);
        }
        let tid = TypeIdC(self.types.len());
        self.types.push(TypeRowC {
            name_id: self.view.names.decls[d.0].type_name_id,
            decl: Some(d.0),
            args: SliceC::normalized(args_start, args.len()),
            ctor_start: CtorIdC(0),
            ctor_len: 0,
        });
        self.type_done.push(false);
        self.fifo.push_back(tid);
        Ok(tid)
    }
    pub fn resolve_type_expr(
        &mut self,
        expr: &TypeExprSrc,
        enclosing: Option<TypeIdC>,
        locator: Option<LocatorC>,
    ) -> Result<TypeIdC, ExtTypeBuildError> {
        if !self.t1_finished {
            return Err(wrong("T1 incomplete"));
        }
        let enc_info = match enclosing {
            Some(tid) => {
                let row = self
                    .types
                    .get(tid.0)
                    .ok_or_else(|| oob("enclosing TypeId"))?;
                let d = row.decl.ok_or_else(|| wrong("enclosing builtin"))?;
                Some((d, row.args))
            }
            None => None,
        };
        struct Frame<'b> {
            node: &'b TypeExprSrc,
            next_idx: usize,
            args: Vec<TypeIdC>,
        }
        let mut stack = vec![Frame {
            node: expr,
            next_idx: 0,
            args: Vec::new(),
        }];
        while let Some(top) = stack.last_mut() {
            if let TypeExprSrc::Apply { args, .. } = top.node {
                if top.next_idx < args.len() {
                    let child = &args[top.next_idx];
                    top.next_idx += 1;
                    stack.push(Frame {
                        node: child,
                        next_idx: 0,
                        args: Vec::new(),
                    });
                    continue;
                }
            }
            let frame = stack.pop().ok_or_else(|| wrong("empty frame stack"))?;
            let resolved = match frame.node {
                TypeExprSrc::UInt => TypeIdC(0),
                TypeExprSrc::Bool => TypeIdC(1),
                TypeExprSrc::List => TypeIdC(2),
                TypeExprSrc::Tree => TypeIdC(3),
                TypeExprSrc::Param { name } => match enc_info {
                    None => {
                        return Err(ExtTypeBuildError::ExtUnsupportedFeature {
                            feature: ExtTypeFeatureKind::TypeVariableInSignature,
                            locator,
                        })
                    }
                    Some((d, slice)) => {
                        let pos = self.view.decls[d]
                            .parameters
                            .iter()
                            .position(|p| p == name)
                            .ok_or_else(|| {
                                reject(ExtTypeRejectReason::UnknownType, name, locator.clone())
                            })?;
                        self.type_args[slice.start + pos]
                    }
                },
                TypeExprSrc::Named { name } => {
                    let d = self
                        .view
                        .decls
                        .iter()
                        .position(|decl| &decl.name == name)
                        .ok_or_else(|| {
                            reject(ExtTypeRejectReason::UnknownType, name, locator.clone())
                        })?;
                    if !self.view.decls[d].parameters.is_empty() {
                        return Err(reject(ExtTypeRejectReason::TypeArity, name, locator));
                    }
                    self.discover_or_intern_key(DeclIdxC(d), &[], locator.clone())?
                }
                TypeExprSrc::Apply { name, .. } => {
                    let d = self
                        .view
                        .decls
                        .iter()
                        .position(|decl| &decl.name == name)
                        .ok_or_else(|| {
                            reject(ExtTypeRejectReason::UnknownType, name, locator.clone())
                        })?;
                    if self.view.decls[d].parameters.is_empty()
                        || frame.args.len() != self.view.decls[d].parameters.len()
                    {
                        return Err(reject(ExtTypeRejectReason::TypeArity, name, locator));
                    }
                    self.discover_or_intern_key(DeclIdxC(d), &frame.args, locator.clone())?
                }
            };
            if let Some(parent) = stack.last_mut() {
                parent.args.push(resolved);
            } else {
                return Ok(resolved);
            }
        }
        Err(wrong("unreachable type resolution"))
    }
    pub fn resolve_construct_use(
        &mut self,
        ty: &TypeExprSrc,
        ctor_name: &str,
        field_len: usize,
        locator: Option<LocatorC>,
    ) -> Result<ResolvedCtorUse, ExtTypeBuildError> {
        let tid = self.resolve_type_expr(ty, None, locator.clone())?;
        if tid.0 <= 1 {
            return Err(reject(ExtTypeRejectReason::UnknownCtor, ctor_name, locator));
        }
        if tid.0 <= 3 {
            return Err(reject(
                ExtTypeRejectReason::BuiltinConstruct,
                ctor_name,
                locator,
            ));
        }
        let d = self.types[tid.0]
            .decl
            .ok_or_else(|| wrong("user type decl"))?;
        let c_ord = self.view.decls[d]
            .ctors
            .iter()
            .position(|c| c.name == ctor_name)
            .ok_or_else(|| {
                reject(ExtTypeRejectReason::UnknownCtor, ctor_name, locator.clone())
            })?;
        if self.view.decls[d].ctors[c_ord].fields.len() != field_len {
            return Err(reject(
                ExtTypeRejectReason::ConstructorArity,
                ctor_name,
                locator,
            ));
        }
        Ok(ResolvedCtorUse {
            type_id: tid,
            ctor_ordinal: c_ord,
        })
    }
    pub fn resolve_pattern_ctor_use(
        &mut self,
        ty: &TypeExprSrc,
        ctor_name: &str,
        subpat_len: usize,
        locator: Option<LocatorC>,
    ) -> Result<ResolvedCtorUse, ExtTypeBuildError> {
        let tid = self.resolve_type_expr(ty, None, locator.clone())?;
        let (c_ord, expected_arity) = match tid.0 {
            0 | 1 => return Err(reject(ExtTypeRejectReason::UnknownCtor, ctor_name, locator)),
            2 => match ctor_name {
                "Nil" => (0, 0),
                "Cons" => (1, 2),
                _ => return Err(reject(ExtTypeRejectReason::UnknownCtor, ctor_name, locator)),
            },
            3 => match ctor_name {
                "Leaf" => (0, 1),
                "Branch" => (1, 2),
                _ => return Err(reject(ExtTypeRejectReason::UnknownCtor, ctor_name, locator)),
            },
            _ => {
                let d = self.types[tid.0]
                    .decl
                    .ok_or_else(|| wrong("user type decl"))?;
                let ord = self.view.decls[d]
                    .ctors
                    .iter()
                    .position(|c| c.name == ctor_name)
                    .ok_or_else(|| {
                        reject(ExtTypeRejectReason::UnknownCtor, ctor_name, locator.clone())
                    })?;
                (ord, self.view.decls[d].ctors[ord].fields.len())
            }
        };
        if subpat_len != expected_arity {
            return Err(reject(
                ExtTypeRejectReason::PatternArity,
                ctor_name,
                locator,
            ));
        }
        Ok(ResolvedCtorUse {
            type_id: tid,
            ctor_ordinal: c_ord,
        })
    }
    pub fn resolved_ctor_id(&self, u: ResolvedCtorUse) -> Result<CtorIdC, ExtTypeBuildError> {
        if !self.type_done.get(u.type_id.0).copied().unwrap_or(false) {
            return Err(wrong("type key not yet dequeued"));
        }
        let row = &self.types[u.type_id.0];
        if u.ctor_ordinal >= row.ctor_len {
            return Err(oob("ctor ordinal"));
        }
        Ok(CtorIdC(
            row.ctor_start
                .0
                .checked_add(u.ctor_ordinal)
                .ok_or_else(|| arith("ctor id"))?,
        ))
    }
    pub fn has_pending_keys(&self) -> bool {
        !self.fifo.is_empty()
    }
    pub fn begin_dequeue_next_key(&mut self) -> Result<DequeuedTypeKey, ExtTypeBuildError> {
        if !self.t1_finished || self.active.is_some() {
            return Err(wrong("cannot dequeue key now"));
        }
        let tid = self
            .fifo
            .pop_front()
            .ok_or_else(|| wrong("FIFO queue empty"))?;
        let d = DeclIdxC(
            self.types[tid.0]
                .decl
                .ok_or_else(|| wrong("builtin in FIFO"))?,
        );
        let ctor_count = self.view.decls[d.0].ctors.len();
        self.types[tid.0].ctor_start = CtorIdC(self.constructors.len());
        self.types[tid.0].ctor_len = ctor_count;
        self.active = Some(ActiveDequeue {
            type_id: tid,
            decl: d,
            next_ctor: 0,
            active_ctor: None,
        });
        Ok(DequeuedTypeKey {
            type_id: tid,
            decl: d,
            ctor_count,
        })
    }
    pub fn begin_constructor(
        &mut self,
        key: &DequeuedTypeKey,
        ctor_ord: usize,
        locator: Option<LocatorC>,
    ) -> Result<CtorIdC, ExtTypeBuildError> {
        let act = self
            .active
            .as_mut()
            .ok_or_else(|| wrong("no active dequeued key"))?;
        if act.type_id != key.type_id || act.active_ctor.is_some() || ctor_ord != act.next_ctor {
            return Err(wrong("constructor order violation"));
        }
        let field_len = self.view.decls[act.decl.0]
            .ctors
            .get(ctor_ord)
            .ok_or_else(|| oob("ctor ord"))?
            .fields
            .len();
        let next_ctors = self
            .constructors
            .len()
            .checked_add(1)
            .ok_or_else(|| arith("ctors len"))?;
        if next_ctors > self.view.bounds.constructor_rows {
            return Err(ext_cap(
                ExtTypeCapKind::ConstructorRows,
                next_ctors,
                self.view.bounds.constructor_rows,
                locator,
            ));
        }
        let cid = CtorIdC(self.constructors.len());
        self.constructors.push(CtorRowExt {
            owning_type: key.type_id,
            name_id: self.view.names.decls[act.decl.0].ctor_name_ids[ctor_ord],
            source_order: ctor_ord,
            field_start: self.ctor_fields.len(),
            field_len,
        });
        act.next_ctor += 1;
        if field_len > 0 {
            act.active_ctor = Some((cid, ctor_ord, 0));
        }
        Ok(cid)
    }
    pub fn append_ctor_field(
        &mut self,
        key: &DequeuedTypeKey,
        ctor_id: CtorIdC,
        ctor_ord: usize,
        field_ord: usize,
        field_type: TypeIdC,
        locator: Option<LocatorC>,
    ) -> Result<(), ExtTypeBuildError> {
        if field_type.0 >= self.types.len() {
            return Err(oob("ctorField type_id"));
        }
        let act = self
            .active
            .as_mut()
            .ok_or_else(|| wrong("no active dequeued key"))?;
        let (ac_id, ac_ord, next_f) = act.active_ctor.ok_or_else(|| wrong("no active ctor"))?;
        if act.type_id != key.type_id
            || ac_id != ctor_id
            || ac_ord != ctor_ord
            || field_ord != next_f
        {
            return Err(wrong("ctor field order violation"));
        }
        let next_slots = self
            .ctor_fields
            .len()
            .checked_add(1)
            .ok_or_else(|| arith("ctorFields len"))?;
        if next_slots > self.view.bounds.constructor_field_slots {
            return Err(ext_cap(
                ExtTypeCapKind::ConstructorFieldSlots,
                next_slots,
                self.view.bounds.constructor_field_slots,
                locator,
            ));
        }
        let name_id = self.view.names.decls[act.decl.0].field_name_ids[ctor_ord][field_ord];
        self.ctor_fields.push(CtorFieldRowExt {
            name_id,
            type_id: field_type,
        });
        let expected_len = self.view.decls[act.decl.0].ctors[ctor_ord].fields.len();
        if field_ord + 1 == expected_len {
            act.active_ctor = None;
        } else {
            act.active_ctor = Some((ac_id, ac_ord, field_ord + 1));
        }
        Ok(())
    }
    pub fn complete_dequeued_key(&mut self, key: DequeuedTypeKey) -> Result<(), ExtTypeBuildError> {
        let act = self
            .active
            .take()
            .ok_or_else(|| wrong("no active dequeued key"))?;
        if act.type_id != key.type_id
            || act.active_ctor.is_some()
            || act.next_ctor != self.view.decls[act.decl.0].ctors.len()
        {
            return Err(wrong("incomplete constructors/fields for key"));
        }
        self.type_done[key.type_id.0] = true;
        Ok(())
    }
    pub fn finish(self) -> Result<BuiltAdtTypeTables, ExtTypeBuildError> {
        if !self.t1_finished
            || self.active.is_some()
            || !self.fifo.is_empty()
            || self.type_done.iter().any(|&done| !done)
        {
            return Err(wrong("unfinished FIFO type instantiation"));
        }
        Ok(BuiltAdtTypeTables {
            types: self.types,
            type_args: self.type_args,
            constructors: self.constructors,
            ctor_fields: self.ctor_fields,
            decl_param_counts: self.view.decls.iter().map(|d| d.parameters.len()).collect(),
            decl_ctor_counts: self.view.decls.iter().map(|d| d.ctors.len()).collect(),
            max_arity: self.view.bounds.max_arity,
            name_count: self.view.names.name_count,
        })
    }
}
