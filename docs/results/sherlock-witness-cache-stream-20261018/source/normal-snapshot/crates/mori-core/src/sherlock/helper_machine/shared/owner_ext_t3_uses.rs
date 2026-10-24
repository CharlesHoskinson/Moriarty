//! Existing extended source carriers, exact iterative Drop and borrowed T3 walk.
//! Mechanical extraction of the historical owner-authored input portion only.
//! Retained hv/wrong/bump, ownership/Drop and walker bodies are byte-exact; the
//! walker still carries its historical PD-1 type-use order, not runtime order.
//! No checked builder session, type mutation, domain gate, admission or engine.
//! The current owner will bind these same source uses to its checked Pass T.
//! R-only source preparation, uncompiled/unrun; no policy adoption or runtime claim.

use std::mem;
use super::adt::{CtorIdC, TypeIdC};
use super::i128_class::KIntC;
use super::owner_ext_type_builder::{ExtTypeBuildError, TypeExprSrc};
use super::{BinaryOp, CorePattern, CoreTree, CoreValue, GrammarDomain, HostViolation,
    HostViolationKind, LocatorC, UnaryOp};

fn hv(kind: HostViolationKind, loc: &str) -> ExtTypeBuildError {
    ExtTypeBuildError::Host(HostViolation { kind, clause: None, rule: None, locator: Some(loc.to_string()) })
}
fn wrong(loc: &str) -> ExtTypeBuildError { hv(HostViolationKind::WrongVariant, loc) }
fn bump(c: &mut usize, loc: &str) -> Result<usize, ExtTypeBuildError> {
    let id = *c;
    *c = id.checked_add(1).ok_or_else(|| hv(HostViolationKind::ArithmeticUndefined, loc))?;
    Ok(id)
}

// ---------- source carriers (adt.source_input) ----------
pub struct ExtParamSrc { pub name: String, pub ty: TypeExprSrc }
pub struct ExtFunction {
    pub name: String,
    pub parameters: Vec<ExtParamSrc>,
    pub return_type: TypeExprSrc,
    pub body: ExtExpr,
}
/// Await binder. `ty` stays a TypeExprSrc so pass A (I-1 Param) and pass C (G-13
/// AwaitBinderType) can refuse non-UInt/Bool forms. Not a T3 use (I-2).
pub struct AwaitBinderSrc { pub name: String, pub ty: TypeExprSrc }
/// Fin source op: exactly the twelve names of ExtExpr_additions. No schema FinOp ordinal claimed.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FinOpSrc { Transfer, Fee, Request, Liability, Discharge, Next, Emit, Requires, Ensures,
    OpDelivered, OpHeld, OpTerminal }
pub struct ExtArm { pub pattern: ExtPattern, pub body: ExtExpr }
pub enum ExtExpr {
    Literal { value: ExtValue },
    Var { name: String },
    Let { name: String, value: Box<ExtExpr>, body: Box<ExtExpr> },
    Binary { op: BinaryOp, left: Box<ExtExpr>, right: Box<ExtExpr> },
    Unary { op: UnaryOp, operand: Box<ExtExpr> },
    If { cond: Box<ExtExpr>, then_expr: Box<ExtExpr>, else_expr: Box<ExtExpr> },
    Call { name: String, args: Vec<ExtExpr> },
    Match { scrutinee: Box<ExtExpr>, arms: Vec<ExtArm> },
    Construct { ty: TypeExprSrc, ctor: String, fields: Vec<ExtExpr> },
    StateRead { state: String },
    LiabilityPresent { name: String },
    LiabilityAmount { name: String },
    /// Per-op field presence (ExtExpr_additions) is carried here, not validated here.
    Fin { op: FinOpSrc, asset: Option<String>, from: Option<String>, to: Option<String>,
        liability: Option<String>, state: Option<String>, name: Option<String>,
        operand: Box<ExtExpr> },
    Await { label: String, binders: Vec<AwaitBinderSrc>, guard: Box<ExtExpr>, body: Box<ExtExpr> },
}
/// Core literal forms reused exactly via CoreValue (UInt(KInt), Bool, List([KInt]), Tree).
pub enum ExtValue {
    Core { value: CoreValue },
    Constructor { ty: TypeExprSrc, ctor: String, fields: Vec<ExtValue> },
}
/// Dedicated forms reused exactly via CorePattern (PVar, PNil, PCons, PLeaf, PBranch).
pub enum ExtPattern {
    Dedicated { pattern: CorePattern },
    Wildcard,
    Ctor { ty: TypeExprSrc, ctor: String, fields: Vec<ExtPattern> },
}

// ---------- explicit iterative Drop: heap worklist, native depth <= 2 ----------
enum Owned { E(ExtExpr), V(ExtValue), P(ExtPattern), T(CoreTree) }
fn hole_e() -> ExtExpr { ExtExpr::Var { name: String::new() } }
fn hole_t() -> CoreTree { CoreTree::Leaf { n: KIntC::Negative } }
fn take_e(b: &mut Box<ExtExpr>) -> Owned { Owned::E(mem::replace(&mut **b, hole_e())) }
fn take_t(b: &mut Box<CoreTree>) -> Owned { Owned::T(mem::replace(&mut **b, hole_t())) }
fn detach_e(e: &mut ExtExpr, out: &mut Vec<Owned>) {
    match e {
        ExtExpr::Literal { value } => out.push(Owned::V(mem::replace(value,
            ExtValue::Core { value: CoreValue::Bool { b: false } }))),
        ExtExpr::Var { .. } | ExtExpr::StateRead { .. } | ExtExpr::LiabilityPresent { .. }
        | ExtExpr::LiabilityAmount { .. } => {}
        ExtExpr::Let { value: a, body: b, .. } | ExtExpr::Binary { left: a, right: b, .. }
        | ExtExpr::Await { guard: a, body: b, .. } => { out.push(take_e(a)); out.push(take_e(b)); }
        ExtExpr::Unary { operand, .. } | ExtExpr::Fin { operand, .. } => out.push(take_e(operand)),
        ExtExpr::If { cond, then_expr, else_expr } => {
            out.push(take_e(cond)); out.push(take_e(then_expr)); out.push(take_e(else_expr));
        }
        ExtExpr::Call { args: v, .. } | ExtExpr::Construct { fields: v, .. } =>
            out.extend(v.drain(..).map(Owned::E)),
        ExtExpr::Match { scrutinee, arms } => {
            out.push(take_e(scrutinee));
            for ExtArm { pattern, body } in arms.drain(..) {
                out.push(Owned::P(pattern));
                out.push(Owned::E(body));
            }
        }
    }
}
fn detach_v(v: &mut ExtValue, out: &mut Vec<Owned>) {
    match v {
        ExtValue::Core { value: CoreValue::Tree { tree } } => out.push(Owned::T(mem::replace(tree, hole_t()))),
        ExtValue::Core { .. } => {}
        ExtValue::Constructor { fields, .. } => out.extend(fields.drain(..).map(Owned::V)),
    }
}
fn detach_p(p: &mut ExtPattern, out: &mut Vec<Owned>) {
    if let ExtPattern::Ctor { fields, .. } = p { out.extend(fields.drain(..).map(Owned::P)); }
}
fn dismantle(mut stack: Vec<Owned>) {
    while let Some(mut n) = stack.pop() {
        match &mut n {
            Owned::E(e) => detach_e(e, &mut stack),
            Owned::V(v) => detach_v(v, &mut stack),
            Owned::P(p) => detach_p(p, &mut stack),
            Owned::T(CoreTree::Branch { left, right }) => { stack.push(take_t(left)); stack.push(take_t(right)); }
            Owned::T(CoreTree::Leaf { .. }) => {}
        }
    }
}
impl Drop for ExtExpr { fn drop(&mut self) { let mut s = Vec::new(); detach_e(self, &mut s); dismantle(s); } }
impl Drop for ExtValue { fn drop(&mut self) { let mut s = Vec::new(); detach_v(self, &mut s); dismantle(s); } }
impl Drop for ExtPattern { fn drop(&mut self) { let mut s = Vec::new(); detach_p(self, &mut s); dismantle(s); } }

// ---------- T3 use-site identity and pass C record ----------
/// Body owner; maps 1:1 to OccurrenceOwner {Entry, Function{index}} in function-table order.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BodyOwner { Entry, Function(usize) }
/// `expr`: 0-based ExtExpr ordinal in this walker's occurrence preorder of the owner body (root 0).
/// `value`: 0-based ExtValue ordinal in value preorder of that Literal (root 0). `pattern`: 0-based
/// ExtPattern ordinal in preorder of arm `arm`'s root pattern (root 0). These are NOT ExprId or
/// SourceOccurrenceId; pass C must materialize that map.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum T3UseSite {
    ParamType { function: usize, param: usize },
    ReturnType { function: usize },
    Construct { owner: BodyOwner, expr: usize },
    LiteralCtor { owner: BodyOwner, expr: usize, value: usize },
    CtorPattern { owner: BodyOwner, expr: usize, arm: usize, pattern: usize },
}
#[derive(Clone, Copy)]
pub enum T3UseKind<'s> {
    Signature { ty: &'s TypeExprSrc },
    /// Construct or ExtValue Constructor (G-3 refusals).
    Constructor { ty: &'s TypeExprSrc, ctor: &'s str, arity: usize },
    /// Ctor pattern (ExtPattern; G-4 resolution part).
    PatternCtor { ty: &'s TypeExprSrc, ctor: &'s str, arity: usize },
}
#[derive(Clone, Copy)]
pub struct T3Use<'s> { pub site: T3UseSite, pub kind: T3UseKind<'s> }
/// ctor_id is Some iff site is Construct, LiteralCtor or CtorPattern.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct T3Record { pub site: T3UseSite, pub type_id: TypeIdC, pub ctor_id: Option<CtorIdC> }

// ---------- iterative I-2 T3 walker (read-only; never resolves) ----------
#[derive(Clone, Copy)]
enum Act<'s> {
    Expr(&'s ExtExpr),
    Value { expr: usize, root: bool, v: &'s ExtValue },
    Pat { expr: usize, arm: usize, root: bool, p: &'s ExtPattern },
}
pub struct T3UseWalker<'s> {
    functions: &'s [ExtFunction], entry: &'s ExtExpr, sig_fn: usize, sig_param: usize,
    next_owner: usize, owner: BodyOwner, stack: Vec<Act<'s>>,
    next_expr: usize, next_value: usize, next_pat: usize, finished: bool,
}
impl<'s> T3UseWalker<'s> {
    pub fn new(functions: &'s [ExtFunction], entry: &'s ExtExpr) -> Self {
        Self { functions, entry, sig_fn: 0, sig_param: 0, next_owner: 0, owner: BodyOwner::Entry,
            stack: Vec::new(), next_expr: 0, next_value: 0, next_pat: 0, finished: false }
    }
    pub fn is_finished(&self) -> bool { self.finished }
    /// Signatures in table order (params in order, then return type); then entry and each
    /// function body in occurrence preorder. A node's own uses precede its children: Construct
    /// ty before its fields; Literal values in value preorder; at a Match, every arm's Ctor
    /// patterns in arm order and pattern preorder, then the scrutinee, then arm bodies (PD-1).
    pub fn next_use(&mut self) -> Result<Option<T3Use<'s>>, ExtTypeBuildError> {
        if self.finished { return Ok(None); }
        let fns: &'s [ExtFunction] = self.functions;
        if self.sig_fn < fns.len() {
            let fi = self.sig_fn;
            let f = &fns[fi];
            if self.sig_param < f.parameters.len() {
                let p = bump(&mut self.sig_param, "T3 param cursor")?;
                return Ok(Some(T3Use { site: T3UseSite::ParamType { function: fi, param: p },
                    kind: T3UseKind::Signature { ty: &f.parameters[p].ty } }));
            }
            self.sig_param = 0;
            bump(&mut self.sig_fn, "T3 function cursor")?;
            return Ok(Some(T3Use { site: T3UseSite::ReturnType { function: fi },
                kind: T3UseKind::Signature { ty: &f.return_type } }));
        }
        loop {
            let act = match self.stack.pop() {
                Some(a) => a,
                None => {
                    if self.next_owner > fns.len() { self.finished = true; return Ok(None); }
                    let k = bump(&mut self.next_owner, "T3 owner cursor")?;
                    let entry: &'s ExtExpr = self.entry;
                    let (owner, root) = if k == 0 { (BodyOwner::Entry, entry) }
                        else { (BodyOwner::Function(k - 1), &fns[k - 1].body) };
                    self.owner = owner;
                    self.next_expr = 0;
                    self.stack.push(Act::Expr(root));
                    continue;
                }
            };
            let owner = self.owner;
            match act {
                Act::Expr(e) => {
                    let id = bump(&mut self.next_expr, "T3 expr ordinal")?;
                    let st = &mut self.stack;
                    match e {
                        ExtExpr::Literal { value } => st.push(Act::Value { expr: id, root: true, v: value }),
                        ExtExpr::Var { .. } | ExtExpr::StateRead { .. } | ExtExpr::LiabilityPresent { .. }
                        | ExtExpr::LiabilityAmount { .. } => {}
                        ExtExpr::Let { value: a, body: b, .. } | ExtExpr::Binary { left: a, right: b, .. }
                        | ExtExpr::Await { guard: a, body: b, .. } => {
                            st.push(Act::Expr(&**b)); st.push(Act::Expr(&**a));
                        }
                        ExtExpr::Unary { operand, .. } | ExtExpr::Fin { operand, .. } => st.push(Act::Expr(&**operand)),
                        ExtExpr::If { cond, then_expr, else_expr } => {
                            st.push(Act::Expr(&**else_expr)); st.push(Act::Expr(&**then_expr));
                            st.push(Act::Expr(&**cond));
                        }
                        ExtExpr::Call { args, .. } => { for a in args.iter().rev() { st.push(Act::Expr(a)); } }
                        ExtExpr::Match { scrutinee, arms } => {
                            for arm in arms.iter().rev() { st.push(Act::Expr(&arm.body)); }
                            st.push(Act::Expr(&**scrutinee));
                            for (a, arm) in arms.iter().enumerate().rev() {
                                st.push(Act::Pat { expr: id, arm: a, root: true, p: &arm.pattern });
                            }
                        }
                        ExtExpr::Construct { ty, ctor, fields } => {
                            for f in fields.iter().rev() { st.push(Act::Expr(f)); }
                            return Ok(Some(T3Use { site: T3UseSite::Construct { owner, expr: id },
                                kind: T3UseKind::Constructor { ty, ctor: ctor.as_str(), arity: fields.len() } }));
                        }
                    }
                }
                Act::Value { expr, root, v } => {
                    if root { self.next_value = 0; }
                    let id = bump(&mut self.next_value, "T3 value ordinal")?;
                    if let ExtValue::Constructor { ty, ctor, fields } = v {
                        for f in fields.iter().rev() { self.stack.push(Act::Value { expr, root: false, v: f }); }
                        return Ok(Some(T3Use { site: T3UseSite::LiteralCtor { owner, expr, value: id },
                            kind: T3UseKind::Constructor { ty, ctor: ctor.as_str(), arity: fields.len() } }));
                    }
                }
                Act::Pat { expr, arm, root, p } => {
                    if root { self.next_pat = 0; }
                    let id = bump(&mut self.next_pat, "T3 pattern ordinal")?;
                    if let ExtPattern::Ctor { ty, ctor, fields } = p {
                        for f in fields.iter().rev() { self.stack.push(Act::Pat { expr, arm, root: false, p: f }); }
                        return Ok(Some(T3Use { site: T3UseSite::CtorPattern { owner, expr, arm, pattern: id },
                            kind: T3UseKind::PatternCtor { ty, ctor: ctor.as_str(), arity: fields.len() } }));
                    }
                }
            }
        }
    }
}

