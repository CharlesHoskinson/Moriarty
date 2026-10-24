//! Existing uniquely owned extended program carriers and Core embedding only.
//! Mechanical extraction from the historical source carrier leaf; all retained
//! declarations, source-value methods and embedding/ownership bodies are exact.
//! Pass-A/domain admission and financial shape policies are not installed here.
//! One ExtProgram AST is shared with the existing T3 and borrowed source walkers.
//! R-only source preparation, uncompiled/unrun; no generic admission/runtime claim.

use super::i128_class::KIntC;
use super::owner_ext_t3_uses::{ExtArm, ExtExpr, ExtFunction, ExtParamSrc, ExtPattern, ExtValue};
use super::owner_ext_type_builder::{TypeDeclSrc, TypeExprSrc};
use super::{
    BinaryOp, CoreExpr, CoreProgram, CoreValue, FinanceInput, GrammarDomain, RunIntC, SortTag,
    UnaryOp,
};
use std::mem;

// ---------- PROPOSED (PSC-1): raw AssetId.scale carrier vs 0..=38 validation ----------

/// Normative maximum legal scale in `AssetId` (`adt.source_input.AssetDeclSrc`, `G-12`).
pub const MAX_ASSET_SCALE: u8 = 38;

/// PROPOSED (PSC-1): Raw caller source representation for `AssetId.scale` (`Nat in 0..38`).
/// Preserves unvalidated source values (including naturals `> 38`, naturals above `u128`, or
/// negative source integers) so host `u8`/`usize` widths never silently truncate caller input
/// or preempt Pass F step (1) `AssetRows`-before-`G-12` refusal ordering.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AssetScaleSrc {
    Nat(u128),
    NatAboveU128,
    Negative,
}

/// Validated `AssetId.scale` guaranteed to lie in `0..=38` (`G-12`).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct ValidatedAssetScale(u8);

impl ValidatedAssetScale {
    pub const fn as_u8(self) -> u8 {
        self.0
    }
    pub const fn as_u128(self) -> u128 {
        self.0 as u128
    }
}

impl AssetScaleSrc {
    pub const fn from_u8(scale: u8) -> Self {
        Self::Nat(scale as u128)
    }
    pub const fn from_u128(scale: u128) -> Self {
        Self::Nat(scale)
    }
    pub const fn from_kint(k: KIntC) -> Self {
        match k {
            KIntC::InRange(n) => Self::Nat(n),
            KIntC::AboveU128 => Self::NatAboveU128,
            KIntC::Negative => Self::Negative,
        }
    }
    pub const fn from_run_int(r: RunIntC) -> Self {
        match r {
            RunIntC::Nat(n) => Self::Nat(n),
            RunIntC::NatAboveU128 => Self::NatAboveU128,
            RunIntC::Negative => Self::Negative,
        }
    }
    /// Pure `0..=38` check for Pass F (`G-12`). Never invoked by Pass A.
    pub const fn validate_0_to_38(self) -> Option<ValidatedAssetScale> {
        match self {
            Self::Nat(n) if n <= (MAX_ASSET_SCALE as u128) => Some(ValidatedAssetScale(n as u8)),
            _ => None,
        }
    }
}

// ---------- PROPOSED (PSC-2): top-level source carriers & explicit ExtDomain ----------

/// Source `AssetId` carrier (`adt.source_input.AssetDeclSrc`).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AssetIdSrc {
    pub network: String,
    pub issuer: String,
    pub reference: String,
    pub scale: AssetScaleSrc,
}

impl AssetIdSrc {
    /// Pure helper for Pass F `G-12` completeness (non-empty `network`, `issuer`, `reference`)
    /// and `scale in 0..=38`. Not a Pass-A check; Pass F checks `AssetRows` before `G-12`.
    pub fn validate_g12_complete_and_scale(&self) -> Option<ValidatedAssetScale> {
        if self.network.is_empty() || self.issuer.is_empty() || self.reference.is_empty() {
            return None;
        }
        self.scale.validate_0_to_38()
    }
}

/// `adt.source_input.AssetDeclSrc`: `{name: Text, identity: AssetId}`.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AssetDeclSrc {
    pub name: String,
    pub identity: AssetIdSrc,
}

/// `adt.source_input.StateDeclSrc`: `{name: Text, ty: TypeExprSrc, initial: ExtValue}`.
/// `ty` remains `TypeExprSrc` and `initial` remains `ExtValue` so Pass A catches `Param`
/// (`TypeVariableInSignature`) and `ExtValue::Constructor` (`GenericAdt` under `DedicatedSorts`
/// or `Param` in constructor `ty`), while Pass F checks `StateRows` before `G-12` scalar kinds.
/// No recursive `Clone`/`PartialEq`/`Eq`/`Debug`/`Hash` derive (D-36/C8).
pub struct StateDeclSrc {
    pub name: String,
    pub ty: TypeExprSrc,
    pub initial: ExtValue,
}

/// `adt.source_input.InputSrc`: `{name: Text, ty: TypeExprSrc}`.
/// `ty` remains `TypeExprSrc` so Pass A catches `Param` (`TypeVariableInSignature`) and
/// Pass F checks `InputRows` before `G-12` scalar type validation. No recursive derives (D-36/C8).
pub struct InputSrc {
    pub name: String,
    pub ty: TypeExprSrc,
}

/// `adt.source_input.ExtProgram`:
/// `{types: [TypeDeclSrc], assets: [AssetDeclSrc], states: [StateDeclSrc], inputs: [InputSrc],
///   functions: [ExtFunction], entry: ExtExpr}`.
/// Uniquely owned tree; recursive children (`TypeExprSrc`, `ExtValue`, `ExtPattern`, `ExtExpr`)
/// already supply iterative heap-worklist `Drop` in `owner_ext_type_builder` and `owner_ext_t3_uses`.
pub struct ExtProgram {
    pub types: Vec<TypeDeclSrc>,
    pub assets: Vec<AssetDeclSrc>,
    pub states: Vec<StateDeclSrc>,
    pub inputs: Vec<InputSrc>,
    pub functions: Vec<ExtFunction>,
    pub entry: ExtExpr,
}

/// Explicit caller domain `ExtDomain = {grammarDomain: GrammarDomain, finance: FinanceInput}`
/// (`adt.source_input.consumer`). Never inferred from program content.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ExtDomain {
    pub grammar_domain: GrammarDomain,
    pub finance: FinanceInput,
}

impl ExtDomain {
    pub const DEDICATED_ABSENT: Self = Self {
        grammar_domain: GrammarDomain::DedicatedSorts,
        finance: FinanceInput::Absent,
    };
    pub const DEDICATED_TYPED: Self = Self {
        grammar_domain: GrammarDomain::DedicatedSorts,
        finance: FinanceInput::TypedExtensionReserved,
    };
    pub const GENERIC_ABSENT: Self = Self {
        grammar_domain: GrammarDomain::GenericNominalExtensionReserved,
        finance: FinanceInput::Absent,
    };
    pub const GENERIC_TYPED: Self = Self {
        grammar_domain: GrammarDomain::GenericNominalExtensionReserved,
        finance: FinanceInput::TypedExtensionReserved,
    };
    pub const fn is_dedicated_sorts(self) -> bool {
        matches!(self.grammar_domain, GrammarDomain::DedicatedSorts)
    }
    pub const fn is_generic_nominal(self) -> bool {
        matches!(
            self.grammar_domain,
            GrammarDomain::GenericNominalExtensionReserved
        )
    }
    pub const fn is_finance_absent(self) -> bool {
        matches!(self.finance, FinanceInput::Absent)
    }
    pub const fn is_finance_typed(self) -> bool {
        matches!(self.finance, FinanceInput::TypedExtensionReserved)
    }
}

// ---------- Iterative injective CoreProgram -> ExtProgram embedding (D-36/C8) ----------

const fn sort_to_type_expr(sort: SortTag) -> TypeExprSrc {
    match sort {
        SortTag::UInt => TypeExprSrc::UInt,
        SortTag::Bool => TypeExprSrc::Bool,
        SortTag::List => TypeExprSrc::List,
        SortTag::Tree => TypeExprSrc::Tree,
    }
}

fn take_core_expr(slot: &mut Box<CoreExpr>) -> CoreExpr {
    mem::replace(
        &mut **slot,
        CoreExpr::Var {
            name: String::new(),
        },
    )
}

pub fn embed_core_expr(root: CoreExpr) -> ExtExpr {
    enum Frame {
        Visit(CoreExpr),
        Let(String),
        Binary(BinaryOp),
        Unary(UnaryOp),
        If,
        Call(String, usize),
        Match(Vec<super::CorePattern>, usize),
    }
    let mut work = vec![Frame::Visit(root)];
    let mut out: Vec<ExtExpr> = Vec::new();
    while let Some(frame) = work.pop() {
        match frame {
            Frame::Visit(mut e) => match &mut e {
                CoreExpr::Literal { value } => {
                    let v = mem::replace(value, CoreValue::Bool { b: false });
                    out.push(ExtExpr::Literal {
                        value: ExtValue::Core { value: v },
                    });
                }
                CoreExpr::Var { name } => {
                    out.push(ExtExpr::Var {
                        name: mem::take(name),
                    });
                }
                CoreExpr::Let { name, value, body } => {
                    let n = mem::take(name);
                    let v = take_core_expr(value);
                    let b = take_core_expr(body);
                    work.push(Frame::Let(n));
                    work.push(Frame::Visit(b));
                    work.push(Frame::Visit(v));
                }
                CoreExpr::Binary { op, left, right } => {
                    let l = take_core_expr(left);
                    let r = take_core_expr(right);
                    work.push(Frame::Binary(*op));
                    work.push(Frame::Visit(r));
                    work.push(Frame::Visit(l));
                }
                CoreExpr::Unary { op, operand } => {
                    let x = take_core_expr(operand);
                    work.push(Frame::Unary(*op));
                    work.push(Frame::Visit(x));
                }
                CoreExpr::If {
                    cond,
                    then_expr,
                    else_expr,
                } => {
                    let c = take_core_expr(cond);
                    let t = take_core_expr(then_expr);
                    let el = take_core_expr(else_expr);
                    work.push(Frame::If);
                    work.push(Frame::Visit(el));
                    work.push(Frame::Visit(t));
                    work.push(Frame::Visit(c));
                }
                CoreExpr::Call { name, args } => {
                    let n = mem::take(name);
                    let count = args.len();
                    work.push(Frame::Call(n, count));
                    for arg in args.drain(..).rev() {
                        work.push(Frame::Visit(arg));
                    }
                }
                CoreExpr::Match { scrutinee, arms } => {
                    let s = take_core_expr(scrutinee);
                    let count = arms.len();
                    let mut pats = Vec::with_capacity(count);
                    let mut bodies = Vec::with_capacity(count);
                    for super::CoreArm { pattern, body } in arms.drain(..) {
                        pats.push(pattern);
                        bodies.push(body);
                    }
                    work.push(Frame::Match(pats, count));
                    for b in bodies.into_iter().rev() {
                        work.push(Frame::Visit(b));
                    }
                    work.push(Frame::Visit(s));
                }
            },
            Frame::Let(name) => {
                let body = Box::new(out.pop().expect("Let body"));
                let value = Box::new(out.pop().expect("Let value"));
                out.push(ExtExpr::Let { name, value, body });
            }
            Frame::Binary(op) => {
                let right = Box::new(out.pop().expect("Binary right"));
                let left = Box::new(out.pop().expect("Binary left"));
                out.push(ExtExpr::Binary { op, left, right });
            }
            Frame::Unary(op) => {
                let operand = Box::new(out.pop().expect("Unary operand"));
                out.push(ExtExpr::Unary { op, operand });
            }
            Frame::If => {
                let else_expr = Box::new(out.pop().expect("If else"));
                let then_expr = Box::new(out.pop().expect("If then"));
                let cond = Box::new(out.pop().expect("If cond"));
                out.push(ExtExpr::If {
                    cond,
                    then_expr,
                    else_expr,
                });
            }
            Frame::Call(name, count) => {
                let split = out.len() - count;
                let args = out.split_off(split);
                out.push(ExtExpr::Call { name, args });
            }
            Frame::Match(pats, count) => {
                let split = out.len() - count;
                let bodies = out.split_off(split);
                let scrutinee = Box::new(out.pop().expect("Match scrutinee"));
                let arms = pats
                    .into_iter()
                    .zip(bodies)
                    .map(|(pattern, body)| ExtArm {
                        pattern: ExtPattern::Dedicated { pattern },
                        body,
                    })
                    .collect();
                out.push(ExtExpr::Match { scrutinee, arms });
            }
        }
    }
    out.pop().expect("embedded root expression")
}

/// Injective source embedding (`adt.source_input.consumer`): `types = assets = states = inputs = []`.
pub fn embed_core_program(mut core: CoreProgram) -> ExtProgram {
    let functions = mem::take(&mut core.functions)
        .into_iter()
        .map(|f| ExtFunction {
            name: f.name,
            parameters: f
                .parameters
                .into_iter()
                .map(|p| ExtParamSrc {
                    name: p.name,
                    ty: sort_to_type_expr(p.sort),
                })
                .collect(),
            return_type: sort_to_type_expr(f.return_type),
            body: embed_core_expr(f.body),
        })
        .collect();
    let entry = embed_core_expr(mem::replace(
        &mut core.entry,
        CoreExpr::Var {
            name: String::new(),
        },
    ));
    ExtProgram {
        types: Vec::new(),
        assets: Vec::new(),
        states: Vec::new(),
        inputs: Vec::new(),
        functions,
        entry,
    }
}
