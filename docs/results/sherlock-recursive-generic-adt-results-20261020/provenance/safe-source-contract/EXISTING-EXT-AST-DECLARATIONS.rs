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

