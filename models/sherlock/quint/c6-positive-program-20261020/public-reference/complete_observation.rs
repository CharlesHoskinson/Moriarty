// PUBLIC REFERENCE ONLY complete_observation.rs:177-204
/// Same field/variant payloads as actual ExpressionRowC; no AST evaluation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExpressionSnapshot {
    Literal { constant: usize },
    Var { binder: usize },
    Let { binder: usize, value: usize, body: usize },
    Binary { op: BinaryOp, left: usize, right: usize },
    Unary { op: UnaryOp, operand: usize },
    If { cond: usize, then_expr: usize, else_expr: usize },
    Call { function_name: usize, args: SliceC },
    Match { scrutinee: usize, arms: SliceC, plan: Option<usize> },
    Construct { type_id: TypeIdC, ctor_id: adt::CtorIdC, fields: SliceC, constant: Option<usize> },
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ConstFieldsSnapshot { pub start: usize, pub len: usize }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DataFieldsSnapshot { pub start: usize, pub len: usize }
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConstantSnapshot {
    U128 { marker_free: bool, n: u128 },
    Bool { marker_free: bool, b: bool },
    Nil { marker_free: bool },
    Cons { marker_free: bool, fields: ConstFieldsSnapshot },
    Leaf { marker_free: bool, fields: ConstFieldsSnapshot },
    Branch { marker_free: bool, fields: ConstFieldsSnapshot },
    Constructor { marker_free: bool, type_id: TypeIdC, ctor_id: adt::CtorIdC, fields: ConstFieldsSnapshot },
    U128OutOfRange { marker_free: bool },
}

// PUBLIC REFERENCE ONLY complete_observation.rs:216-241
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProgramSnapshot {
    pub entry: usize,
    pub expressions: Vec<ExpressionSnapshot>,
    pub occurrences: Vec<OccurrenceRowC>,
    pub occurrence_paths: Vec<usize>,
    pub children: Vec<usize>,
    pub functions: Vec<FunctionRowC>,
    pub parameters: Vec<ParameterRowC>,
    pub arms: Vec<ArmRowC>,
    pub patterns: Vec<PatternRowC>,
    pub pattern_children: Vec<usize>,
    pub types: Vec<adt::TypeRowC>,
    pub type_args: Vec<TypeIdC>,
    pub constructors: Vec<adt_storage::CtorRowExt>,
    pub ctor_fields: Vec<adt_storage::CtorFieldRowExt>,
    pub decl_param_counts: Vec<usize>,
    pub decl_ctor_counts: Vec<usize>,
    pub decl_templates: Vec<owner_ext_type_builder::TypeDeclSrc>,
    pub plan_nodes: Vec<PlanNodeC>,
    pub plan_cases: Vec<Option<usize>>,
    pub field_paths: Vec<usize>,
    pub constants: Vec<ConstantSnapshot>,
    pub constant_fields: Vec<usize>,
    pub names: Vec<String>,
}

