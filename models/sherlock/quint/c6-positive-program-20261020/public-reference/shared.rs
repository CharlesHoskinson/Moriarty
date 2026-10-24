// PUBLIC REFERENCE ONLY shared.rs:32-36
pub use adt::TypeIdC;
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BinaryOp { Add, Sub, Mul, Eq, Lt, Cons, Branch }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum UnaryOp { Leaf, Not }

// PUBLIC REFERENCE ONLY shared.rs:168-177
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RefC { Const(usize), Data(usize) } // shared owned ref; owner maps to raw
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SliceC { pub start: usize, pub len: usize }
impl SliceC {
    pub fn normalized(start: usize, len: usize) -> Self {
        if len == 0 { Self { start: 0, len: 0 } } else { Self { start, len } }
    }
    pub fn empty() -> Self { Self { start: 0, len: 0 } }
}

// PUBLIC REFERENCE ONLY shared.rs:321-340
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct FunctionRowC { pub function_name: usize, pub params: SliceC, pub return_type: TypeIdC, pub body: usize }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ParameterRowC { pub binder: usize, pub sort: TypeIdC }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ArmRowC { pub pattern: usize, pub body: usize }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PatternRowC { PVar { x: usize }, PNil, PCons { x: usize, y: usize },
    PLeaf { x: usize }, PBranch { x: usize, y: usize }, Wildcard,
    Constructor { type_id: TypeIdC, ctor_id: adt::CtorIdC, children: SliceC } }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PlanNodeC {
    Switch { source_match: usize, path: SliceC, type_id: TypeIdC,
        cases: SliceC, default: usize },
    Bind { source_match: usize, path: SliceC, binder: usize, next: usize },
    Select { source_match: usize, arm: usize },
    NoMatch { source_match: usize },
}
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OccurrenceRowC { pub owner_is_function: bool, pub function: usize, pub path: SliceC }

