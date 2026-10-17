//! Whole-source, host-only table proposal for the existing strict K-A AST.
//!
//! This draft is deliberately unwired. Its Rust names and `usize` indices are
//! not a shared IR, tag ABI, native image, admission policy or B schema freeze.
//! No source/membership digest, field encoding, capacity or layout is selected.
//! Source identity must be supplied by the integrator's separately reviewed
//! binding: these local locators are meaningful only within this whole image.
//! Preparing tables evaluates no expression and never follows a named call.

use super::{BinaryOp, Expr, ExprRef, Name, Parameter, Pattern, Program, Tree,
            Type, UnaryOp, Value, PROFILE};

/// Host table positions only; these select no native width or tag convention.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExprId(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ConstantId(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TableSlice {
    pub start: usize,
    pub len: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SourceOwner {
    Entry,
    Function(usize),
}

/// Child-position paths: let value/body=0/1, binary left/right=0/1,
/// unary operand=0, if condition/then/else=0/1/2, call arguments=0..n,
/// match scrutinee=0 and arm bodies=1..n. Function roots have an empty path.
/// This records typed AST positions, not textual byte offsets or run steps.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceLocator {
    pub owner: SourceOwner,
    pub ast_path: Vec<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExprPayload {
    Literal(ConstantId),
    Var(Name),
    Let(Name),
    Binary(BinaryOp),
    Unary(UnaryOp),
    If,
    Call(Name),
    Match(TableSlice),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExprRow {
    pub payload: ExprPayload,
    pub children: TableSlice,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FunctionRow {
    pub source_table_index: usize,
    pub name: Name,
    pub parameters: TableSlice,
    pub return_type: Type,
    pub body: ExprId,
}

/// K-A patterns are flat. Names and source-arm priority are retained verbatim.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MatchArmRow {
    pub pattern: Pattern,
    pub body: ExprId,
}

/// All existing literal cases are flat here. List fields refer to UInt rows;
/// tree branches refer to two ordered tree rows. Tree leaves carry their u128
/// directly. These are the existing host types, not nominal ADT/native tags.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConstantPayload {
    UInt(u128),
    Bool(bool),
    List,
    TreeLeaf(u128),
    TreeBranch,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConstantLocator {
    pub expression: ExprId,
    pub field_path: Vec<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConstantRow {
    pub payload: ConstantPayload,
    pub fields: TableSlice,
    pub source: ConstantLocator,
}

/// Unbounded host source representation. No recursive Expr/Value/Tree is stored
/// in these tables. Strings, paths and growable vectors remain host metadata;
/// flatness alone gives no finite circuit capacity or native payload claim.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProgramImageDraft {
    pub profile_id: &'static str,
    pub expressions: Vec<ExprRow>,
    pub ordered_children: Vec<ExprId>,
    pub functions: Vec<FunctionRow>,
    pub ordered_parameters: Vec<Parameter>,
    pub match_arms: Vec<MatchArmRow>,
    pub constants: Vec<ConstantRow>,
    pub constant_fields: Vec<ConstantId>,
    /// Row i has source locator source_map[i], giving each occurrence its own ID.
    pub source_map: Vec<SourceLocator>,
    pub entry: ExprId,
}

/// Traverse entry first, then every function body in original table order.
/// Each root expands in source-child preorder, even when Arc storage is shared.
/// Each row reserves its complete ordered child slice before visiting children.
/// This is source preparation only: no unique-name/closed-body admission,
/// evaluation, source-call unrolling, fuel charge or new failure outcome occurs.
/// Host allocation and ordinary host address-space limits remain unmetered.
pub fn prepare_program_image(program: &Program) -> ProgramImageDraft {
    let mut image = ProgramImageDraft {
        profile_id: PROFILE,
        expressions: Vec::new(), ordered_children: Vec::new(),
        functions: Vec::new(), ordered_parameters: Vec::new(),
        match_arms: Vec::new(), constants: Vec::new(),
        constant_fields: Vec::new(), source_map: Vec::new(), entry: ExprId(0),
    };
    image.entry = image.append_expression(&program.entry, SourceOwner::Entry);
    for (source_table_index, function) in program.functions.iter().enumerate() {
        let parameters = TableSlice {
            start: image.ordered_parameters.len(), len: function.parameters.len(),
        };
        image.ordered_parameters.extend_from_slice(&function.parameters);
        let body = image.append_expression(&function.body, SourceOwner::Function(source_table_index));
        image.functions.push(FunctionRow {
            source_table_index, name: function.name.clone(), parameters,
            return_type: function.return_type, body,
        });
    }
    image
}

struct ExprTask<'a> {
    expression: &'a Expr,
    path: Vec<usize>,
    parent_child: Option<usize>,
    arm: Option<usize>,
}

#[derive(Clone, Copy)]
enum ConstantInput<'a> {
    Value(&'a Value),
    UInt(u128),
    Tree(&'a Tree),
}

struct ConstantTask<'a> {
    input: ConstantInput<'a>,
    path: Vec<usize>,
    parent_field: Option<usize>,
}

impl ProgramImageDraft {
    fn append_expression(&mut self, root: &ExprRef, owner: SourceOwner) -> ExprId {
        let root_id = ExprId(self.expressions.len());
        let mut pending = vec![ExprTask {
            expression: root.as_ref(), path: Vec::new(), parent_child: None, arm: None,
        }];
        while let Some(task) = pending.pop() {
            let id = ExprId(self.expressions.len());
            if let Some(slot) = task.parent_child { self.ordered_children[slot] = id; }
            if let Some(arm) = task.arm { self.match_arms[arm].body = id; }
            let mut children: Vec<(&Expr, Option<usize>)> = Vec::new();
            let payload = match task.expression {
                Expr::Literal(value) => ExprPayload::Literal(self.append_constant(value, id)),
                Expr::Var(name) => ExprPayload::Var(name.clone()),
                Expr::Let(name, value, body) => {
                    children.extend([(value.as_ref(), None), (body.as_ref(), None)]);
                    ExprPayload::Let(name.clone())
                }
                Expr::Binary(op, left, right) => {
                    children.extend([(left.as_ref(), None), (right.as_ref(), None)]);
                    ExprPayload::Binary(*op)
                }
                Expr::Unary(op, operand) => {
                    children.push((operand.as_ref(), None));
                    ExprPayload::Unary(*op)
                }
                Expr::If(condition, then_body, else_body) => {
                    children.extend([(condition.as_ref(), None), (then_body.as_ref(), None), (else_body.as_ref(), None)]);
                    ExprPayload::If
                }
                Expr::Call(name, arguments) => {
                    children.extend(arguments.iter().map(|argument| (argument.as_ref(), None)));
                    ExprPayload::Call(name.clone())
                }
                Expr::Match(scrutinee, arms) => {
                    children.push((scrutinee.as_ref(), None));
                    let slice = TableSlice { start: self.match_arms.len(), len: arms.len() };
                    for arm in arms {
                        let arm_id = self.match_arms.len();
                        self.match_arms.push(MatchArmRow { pattern: arm.pattern.clone(), body: ExprId(0) });
                        children.push((arm.body.as_ref(), Some(arm_id)));
                    }
                    ExprPayload::Match(slice)
                }
            };
            let child_slice = TableSlice { start: self.ordered_children.len(), len: children.len() };
            // Temporary slots are filled by the corresponding pending tasks.
            self.ordered_children.resize(child_slice.start + child_slice.len, ExprId(0));
            self.expressions.push(ExprRow { payload, children: child_slice });
            self.source_map.push(SourceLocator { owner, ast_path: task.path.clone() });
            for (position, (expression, arm)) in children.into_iter().enumerate().rev() {
                let mut path = task.path.clone();
                path.push(position);
                pending.push(ExprTask { expression, path, parent_child: Some(child_slice.start + position), arm });
            }
        }
        root_id
    }

    fn append_constant(&mut self, value: &Value, expression: ExprId) -> ConstantId {
        let root_id = ConstantId(self.constants.len());
        let mut pending = vec![ConstantTask { input: ConstantInput::Value(value), path: Vec::new(), parent_field: None }];
        while let Some(task) = pending.pop() {
            let id = ConstantId(self.constants.len());
            if let Some(slot) = task.parent_field { self.constant_fields[slot] = id; }
            let mut children = Vec::new();
            let payload = match task.input {
                ConstantInput::UInt(n) => ConstantPayload::UInt(n),
                ConstantInput::Value(Value::UInt(n)) => ConstantPayload::UInt(*n),
                ConstantInput::Value(Value::Bool(b)) => ConstantPayload::Bool(*b),
                ConstantInput::Value(Value::List(elements)) => {
                    children.extend(elements.iter().copied().map(ConstantInput::UInt));
                    ConstantPayload::List
                }
                ConstantInput::Value(Value::Tree(tree)) | ConstantInput::Tree(tree) => match tree {
                    Tree::Leaf(n) => ConstantPayload::TreeLeaf(*n),
                    Tree::Branch(left, right) => {
                        children.extend([ConstantInput::Tree(left.as_ref()), ConstantInput::Tree(right.as_ref())]);
                        ConstantPayload::TreeBranch
                    }
                },
            };
            let fields = TableSlice { start: self.constant_fields.len(), len: children.len() };
            self.constant_fields.resize(fields.start + fields.len, ConstantId(0));
            self.constants.push(ConstantRow { payload, fields, source: ConstantLocator {
                expression, field_path: task.path.clone(),
            }});
            for (position, input) in children.into_iter().enumerate().rev() {
                let mut path = task.path.clone();
                path.push(position);
                pending.push(ConstantTask { input, path, parent_field: Some(fields.start + position) });
            }
        }
        root_id
    }
}

#[cfg(test)]
#[path = "program_image_tests.rs"]
mod tests;
