//! Internal typed L4 decoder source proposal. No evaluator or transition check.
//! Canonical validation calls the actual encoder; there is no success callback.
use super::{limits::Locator, rows::*, state::*};
use crate::sherlock::{
    Arm, BinaryOp, Control, Environment, ExprRef, Frame, Function, Outcome, PROFILE, Parameter,
    Pattern, Snapshot, Tree, Type, UnaryOp, Value,
};
use std::sync::Arc;

fn wrong(
    table: &'static str,
    row: usize,
    column: &'static str,
    definition: &'static str,
) -> StateFailure {
    StateFailure::WrongRepresentation {
        block: "StateBlock",
        table,
        row,
        column,
        definition,
    }
}
fn host(field: &'static str, row: usize) -> StateFailure {
    StateFailure::HostUnrepresentable {
        field,
        locator: Locator {
            item: row,
            ..Locator::field(field)
        },
    }
}
fn push<T>(values: &mut Vec<T>, value: T, field: &'static str) -> StateResult<()> {
    values
        .try_reserve(1)
        .map_err(|_| host(field, values.len()))?;
    values.push(value);
    Ok(())
}
fn limit(table: &'static str, needed: usize, bound: u128) -> StateResult<()> {
    if needed as u128 > bound {
        return Err(StateFailure::Capacity {
            table,
            needed: needed as u128,
            limit: bound,
            locator: Locator::field(table),
        });
    }
    Ok(())
}
fn fixed<T: Default + PartialEq>(
    table: &'static str,
    rows: &[T],
    active: usize,
    capacity: u128,
) -> StateResult<()> {
    limit(table, active, capacity)?;
    let capacity = usize::try_from(capacity).map_err(|_| host(table, 0))?;
    if rows.len() != capacity {
        return Err(wrong(
            table,
            0,
            "length",
            "fixed vector length must equal declared capacity",
        ));
    }
    // active <= capacity == len was checked before slicing or indexing.
    for (i, row) in rows.iter().enumerate().skip(active) {
        if row != &T::default() {
            return Err(wrong(table, i, "padding", "every inactive row is zero"));
        }
    }
    Ok(())
}
fn slice(
    slice: Slice,
    active: usize,
    table: &'static str,
    row: usize,
    field: &'static str,
) -> StateResult<std::ops::Range<usize>> {
    if slice.len == 0 && slice.start != 0 {
        return Err(wrong(table, row, field, "empty slice must be (0,0)"));
    }
    let end = slice
        .start
        .checked_add(slice.len)
        .ok_or_else(|| wrong(table, row, field, "slice end overflows host index"))?;
    if end > active {
        return Err(wrong(
            table,
            row,
            field,
            "slice extends beyond active length",
        ));
    }
    Ok(slice.start..end)
}
fn ty(tag: u8, table: &'static str, row: usize) -> StateResult<Type> {
    match tag {
        0 => Ok(Type::UInt),
        1 => Ok(Type::Bool),
        2 => Ok(Type::List),
        3 => Ok(Type::Tree),
        _ => Err(wrong(table, row, "typeId", "Type tag must be 0..3")),
    }
}
fn binary(tag: u8, table: &'static str, row: usize) -> StateResult<BinaryOp> {
    match tag {
        0 => Ok(BinaryOp::Add),
        1 => Ok(BinaryOp::Sub),
        2 => Ok(BinaryOp::Mul),
        3 => Ok(BinaryOp::Eq),
        4 => Ok(BinaryOp::Lt),
        5 => Ok(BinaryOp::Cons),
        6 => Ok(BinaryOp::Branch),
        _ => Err(wrong(table, row, "binaryOp", "BinaryOp tag must be 0..6")),
    }
}
fn unary(tag: u8, table: &'static str, row: usize) -> StateResult<UnaryOp> {
    match tag {
        0 => Ok(UnaryOp::Leaf),
        1 => Ok(UnaryOp::Not),
        _ => Err(wrong(table, row, "unaryOp", "UnaryOp tag must be 0..1")),
    }
}

/// A decoded tree cache retains the finite input DAG, not its exponential
/// canonical unfolding. Lists are materialized by a single forward chain walk.
/// The real canonical encoder performs the separate unfolded-capacity checks.
struct CheckedNode {
    kind: Type,
    tree: Option<Arc<Tree>>,
}
struct Reader<'a> {
    prepared: &'a PreparedA,
    state: &'a StateBlock,
    mode: DecodeMode,
    constants: Vec<CheckedNode>,
    data: Vec<CheckedNode>,
}
impl<'a> Reader<'a> {
    fn node(
        &self,
        r: ValueRef,
        table: &'static str,
        row: usize,
        field: &'static str,
    ) -> StateResult<&ConstantNode> {
        let (nodes, active) = if r.space {
            (&self.state.data, self.state.root.active_data)
        } else {
            (
                &self.prepared.block.constants.rows,
                self.prepared.block.constants.active,
            )
        };
        if r.index >= active {
            return Err(wrong(
                table,
                row,
                field,
                "ValueRef index must be below selected-space active length",
            ));
        }
        nodes
            .get(r.index)
            .ok_or_else(|| wrong(table, row, field, "selected value node is missing"))
    }
    fn checked(&self, r: ValueRef, table: &'static str, row: usize) -> StateResult<&CheckedNode> {
        let nodes = if r.space { &self.data } else { &self.constants };
        nodes.get(r.index).ok_or_else(|| {
            wrong(
                table,
                row,
                "fieldRef",
                "child must precede parent in its space",
            )
        })
    }
    fn fields(
        &self,
        r: ValueRef,
        n: &ConstantNode,
        table: &'static str,
        row: usize,
    ) -> StateResult<&[ValueRef]> {
        let (fields, active) = if r.space {
            (&self.state.data_fields, self.state.root.active_data_fields)
        } else {
            (
                &self.prepared.block.constant_fields.rows,
                self.prepared.block.constant_fields.active,
            )
        };
        let range = slice(n.fields, active, table, row, "fields")?;
        fields
            .get(range)
            .ok_or_else(|| wrong(table, row, "fields", "field rows are missing"))
    }
    fn check_node(&self, r: ValueRef) -> StateResult<CheckedNode> {
        let table = if r.space { "Data" } else { "Constants" };
        let row = r.index;
        let n = self.node(r, table, row, "node")?;
        let mut expected = ConstantNode {
            kind: n.kind,
            type_id: n.type_id,
            ..ConstantNode::default()
        };
        let kind;
        let mut tree = None;
        match n.kind {
            0 => {
                kind = Type::UInt;
                if n.type_id != 0 {
                    return Err(wrong(table, row, "typeId", "UInt has typeId=0"));
                }
                expected.uint_bytes = n.uint_bytes;
            }
            1 => {
                kind = Type::Bool;
                if n.type_id != 1 {
                    return Err(wrong(table, row, "typeId", "Bool has typeId=1"));
                }
                expected.bool_value = n.bool_value;
            }
            2 => {
                expected.ctor_id = n.ctor_id;
                expected.fields = n.fields;
                let (type_id, child_types): (u8, &[Type]) = match n.ctor_id {
                    0 => (2, &[]),
                    1 => (2, &[Type::UInt, Type::List]),
                    2 => (3, &[Type::UInt]),
                    3 => (3, &[Type::Tree, Type::Tree]),
                    _ => {
                        return Err(wrong(
                            table,
                            row,
                            "ctorId",
                            "constructor id must be Nil/Cons/Leaf/Branch",
                        ));
                    }
                };
                if n.type_id != type_id {
                    return Err(wrong(
                        table,
                        row,
                        "typeId",
                        "constructor owning type mismatch",
                    ));
                }
                kind = ty(type_id, table, row)?;
                let fields = self.fields(r, n, table, row)?;
                if fields.len() != child_types.len() {
                    return Err(wrong(
                        table,
                        row,
                        "fieldLength",
                        "constructor field count mismatch",
                    ));
                }
                for (child, want) in fields.iter().zip(child_types) {
                    self.node(*child, table, row, "fieldRef")?;
                    if (!r.space && child.space) || (r.space == child.space && child.index >= row) {
                        return Err(wrong(
                            table,
                            row,
                            "fieldRef",
                            "constant children are constant; same-space children precede parent",
                        ));
                    }
                    if self.checked(*child, table, row)?.kind != *want {
                        return Err(wrong(
                            table,
                            row,
                            "fieldRef.typeId",
                            "constructor child type mismatch",
                        ));
                    }
                }
                if n.ctor_id == 2 {
                    let child = self.node(fields[0], table, row, "Leaf.value")?;
                    tree = Some(Arc::new(Tree::Leaf(u128::from_le_bytes(child.uint_bytes))));
                } else if n.ctor_id == 3 {
                    let left = self
                        .checked(fields[0], table, row)?
                        .tree
                        .as_ref()
                        .ok_or_else(|| wrong(table, row, "left", "Tree child missing"))?;
                    let right = self
                        .checked(fields[1], table, row)?
                        .tree
                        .as_ref()
                        .ok_or_else(|| wrong(table, row, "right", "Tree child missing"))?;
                    tree = Some(Arc::new(Tree::Branch(left.clone(), right.clone())));
                }
            }
            _ => {
                return Err(wrong(
                    table,
                    row,
                    "kind",
                    "node kind must be UInt/Bool/Constructor",
                ));
            }
        }
        macro_rules! zero_column {
            ($field:ident, $name:literal) => {
                if expected.$field != n.$field {
                    return Err(wrong(table, row, $name, "unused node column must be zero"));
                }
            };
        }
        zero_column!(ctor_id, "ctorId");
        zero_column!(uint_bytes, "uintBytes");
        zero_column!(bool_value, "boolValue");
        zero_column!(fields, "fieldSlice");
        Ok(CheckedNode { kind, tree })
    }
    fn name(&self, id: usize, table: &'static str, row: usize) -> StateResult<String> {
        if id >= self.prepared.block.names.active {
            return Err(wrong(
                table,
                row,
                "nameId",
                "name id is outside ProgramBlock",
            ));
        }
        let name = self
            .prepared
            .names
            .get(id)
            .ok_or_else(|| wrong(table, row, "nameId", "prepared name metadata is missing"))?;
        let mut out = String::new();
        out.try_reserve_exact(name.len())
            .map_err(|_| host("name", row))?;
        out.push_str(name);
        Ok(out)
    }
    fn expr(&self, id: usize, table: &'static str, row: usize) -> StateResult<ExprRef> {
        if id >= self.prepared.block.expressions.active {
            return Err(wrong(
                table,
                row,
                "exprId",
                "expression id is outside ProgramBlock",
            ));
        }
        self.prepared
            .occurrences
            .get(id)
            .map(|m| m.source.clone())
            .ok_or_else(|| {
                wrong(
                    table,
                    row,
                    "exprId",
                    "prepared occurrence metadata is missing",
                )
            })
    }
    fn value(&self, r: ValueRef, table: &'static str, row: usize) -> StateResult<Value> {
        if self.mode == DecodeMode::Canonical && !r.space {
            return Err(wrong(
                table,
                row,
                "valueRef.space",
                "canonical state roots use Data space",
            ));
        }
        let n = self.node(r, table, row, "valueRef")?;
        let checked = self.checked(r, table, row)?;
        match checked.kind {
            Type::UInt => Ok(Value::UInt(u128::from_le_bytes(n.uint_bytes))),
            Type::Bool => Ok(Value::Bool(n.bool_value)),
            Type::Tree => Ok(Value::Tree(
                checked
                    .tree
                    .as_ref()
                    .ok_or_else(|| wrong(table, row, "tree", "checked Tree absent"))?
                    .as_ref()
                    .clone(),
            )),
            Type::List => {
                let mut items = Vec::new();
                let mut at = r;
                loop {
                    let n = self.node(at, table, row, "list")?;
                    if n.ctor_id == 0 {
                        break;
                    }
                    // All nodes and links were checked before any root is decoded.
                    let fields = self.fields(at, n, table, row)?;
                    let head = self.node(fields[0], table, row, "list.head")?;
                    push(
                        &mut items,
                        u128::from_le_bytes(head.uint_bytes),
                        "decoded list",
                    )?;
                    at = fields[1];
                }
                Ok(Value::List(items))
            }
        }
    }
    fn env_ref(
        &self,
        r: EnvRoot,
        table: &'static str,
        row: usize,
        field: &'static str,
    ) -> StateResult<()> {
        if !r.present {
            if r.index != 0 {
                return Err(wrong(
                    table,
                    row,
                    field,
                    "absent environment index must be zero",
                ));
            }
        } else if r.index >= self.state.root.active_env_cells {
            return Err(wrong(
                table,
                row,
                field,
                "environment index exceeds active length",
            ));
        }
        Ok(())
    }
    fn environment(
        &self,
        root: EnvRoot,
        table: &'static str,
        row: usize,
    ) -> StateResult<Environment> {
        self.env_ref(root, table, row, "environment")?;
        let mut env = Environment::new();
        let mut at = root;
        while at.present {
            let cell = &self.state.env_cells[at.index];
            let name = self.name(cell.name_id, "EnvCells", at.index)?;
            if !env.contains_key(&name) {
                let value = self.value(cell.value, "EnvCells", at.index)?;
                env.insert(name, value);
            }
            at = cell.parent;
        }
        Ok(env)
    }
    fn values(&self, s: Slice, table: &'static str, row: usize) -> StateResult<Vec<Value>> {
        let range = slice(
            s,
            self.state.root.active_value_slots,
            table,
            row,
            "valueSlice",
        )?;
        let mut out = Vec::new();
        for i in range {
            push(
                &mut out,
                self.value(self.state.value_slots[i], "ValueSlots", i)?,
                "decoded values",
            )?;
        }
        Ok(out)
    }
    fn suffix(
        &self,
        s: Slice,
        arms: bool,
        table: &'static str,
        row: usize,
    ) -> StateResult<std::ops::Range<usize>> {
        let active = if arms {
            self.prepared.block.arms.active
        } else {
            self.prepared.block.expr_children.active
        };
        let range = slice(
            s,
            active,
            table,
            row,
            if arms {
                "armSlice"
            } else {
                "remainingExprSlice"
            },
        )?;
        if s.len == 0 {
            return Ok(range);
        }
        let tag = if arms { 7 } else { 6 };
        let member = self
            .prepared
            .block
            .expressions
            .rows
            .iter()
            .take(self.prepared.block.expressions.active)
            .any(|e| {
                if e.tag != tag {
                    return false;
                }
                let source = if arms { e.arms } else { e.children };
                source.start <= s.start && source.start.checked_add(source.len) == Some(range.end)
            });
        if !member {
            return Err(wrong(
                table,
                row,
                if arms {
                    "armSlice"
                } else {
                    "remainingExprSlice"
                },
                "nonempty state slice must be a suffix of one source Match/Call slice in both modes",
            ));
        }
        Ok(range)
    }
    fn expressions(&self, s: Slice, table: &'static str, row: usize) -> StateResult<Vec<ExprRef>> {
        let mut out = Vec::new();
        for i in self.suffix(s, false, table, row)? {
            let id = self.prepared.block.expr_children.rows[i];
            push(
                &mut out,
                self.expr(id, table, row)?,
                "decoded expression slice",
            )?;
        }
        Ok(out)
    }
    fn arms(&self, s: Slice, table: &'static str, row: usize) -> StateResult<Vec<Arm>> {
        let mut out = Vec::new();
        for i in self.suffix(s, true, table, row)? {
            let arm = &self.prepared.block.arms.rows[i];
            let p = self
                .prepared
                .block
                .patterns
                .rows
                .get(arm.pattern_id)
                .filter(|_| arm.pattern_id < self.prepared.block.patterns.active)
                .ok_or_else(|| {
                    wrong(
                        table,
                        row,
                        "patternId",
                        "pattern id is outside ProgramBlock",
                    )
                })?;
            let pattern = match p.tag {
                0 => Pattern::Var(self.name(p.name0, "Patterns", arm.pattern_id)?),
                1 => Pattern::Nil,
                2 => Pattern::Cons(
                    self.name(p.name0, "Patterns", arm.pattern_id)?,
                    self.name(p.name1, "Patterns", arm.pattern_id)?,
                ),
                3 => Pattern::Leaf(self.name(p.name0, "Patterns", arm.pattern_id)?),
                4 => Pattern::Branch(
                    self.name(p.name0, "Patterns", arm.pattern_id)?,
                    self.name(p.name1, "Patterns", arm.pattern_id)?,
                ),
                _ => {
                    return Err(wrong(
                        "Patterns",
                        arm.pattern_id,
                        "tag",
                        "Pattern tag must be 0..4",
                    ));
                }
            };
            push(
                &mut out,
                Arm {
                    pattern,
                    body: self.expr(arm.body_expr_id, table, row)?,
                },
                "decoded arms",
            )?;
        }
        Ok(out)
    }
    fn function(&self, id: usize, row: usize) -> StateResult<Function> {
        let f = self
            .prepared
            .block
            .functions
            .rows
            .get(id)
            .filter(|_| id < self.prepared.block.functions.active)
            .ok_or_else(|| {
                wrong(
                    "ControlRows",
                    row,
                    "functionId",
                    "function id is outside ProgramBlock",
                )
            })?;
        let mut parameters = Vec::new();
        for i in slice(
            f.parameters,
            self.prepared.block.parameters.active,
            "Functions",
            id,
            "parameterSlice",
        )? {
            let p = &self.prepared.block.parameters.rows[i];
            push(
                &mut parameters,
                Parameter {
                    name: self.name(p.name_id, "Parameters", i)?,
                    ty: ty(p.type_id, "Parameters", i)?,
                },
                "decoded parameters",
            )?;
        }
        Ok(Function {
            name: self.name(f.name_id, "Functions", id)?,
            parameters,
            return_type: ty(f.return_type, "Functions", id)?,
            body: self.expr(f.body_expr_id, "Functions", id)?,
        })
    }
    fn frame(&self, f: &FrameRow, table: &'static str, row: usize) -> StateResult<Frame> {
        let mut used = FrameRow {
            tag: f.tag,
            ..FrameRow::default()
        };
        let value = match f.tag {
            0 => {
                used.name_id = f.name_id;
                used.expr0 = f.expr0;
                used.saved_env = f.saved_env;
                Frame::Let(
                    self.name(f.name_id, table, row)?,
                    self.expr(f.expr0, table, row)?,
                    self.environment(f.saved_env, table, row)?,
                )
            }
            1 => {
                used.binary_op = f.binary_op;
                used.expr0 = f.expr0;
                used.saved_env = f.saved_env;
                Frame::Left(
                    binary(f.binary_op, table, row)?,
                    self.expr(f.expr0, table, row)?,
                    self.environment(f.saved_env, table, row)?,
                )
            }
            2 => {
                used.binary_op = f.binary_op;
                used.value = f.value;
                Frame::Right(
                    binary(f.binary_op, table, row)?,
                    self.value(f.value, table, row)?,
                )
            }
            3 => {
                used.unary_op = f.unary_op;
                Frame::Unary(unary(f.unary_op, table, row)?)
            }
            4 => {
                used.expr0 = f.expr0;
                used.expr1 = f.expr1;
                used.saved_env = f.saved_env;
                Frame::If(
                    self.expr(f.expr0, table, row)?,
                    self.expr(f.expr1, table, row)?,
                    self.environment(f.saved_env, table, row)?,
                )
            }
            5 => {
                used.arms = f.arms;
                used.saved_env = f.saved_env;
                Frame::Match(
                    self.arms(f.arms, table, row)?,
                    self.environment(f.saved_env, table, row)?,
                )
            }
            6 => {
                used.name_id = f.name_id;
                used.completed = f.completed;
                used.remaining_exprs = f.remaining_exprs;
                used.saved_env = f.saved_env;
                Frame::Args(
                    self.name(f.name_id, table, row)?,
                    self.values(f.completed, table, row)?,
                    self.expressions(f.remaining_exprs, table, row)?,
                    self.environment(f.saved_env, table, row)?,
                )
            }
            7 => {
                used.type_id = f.type_id;
                used.saved_env = f.saved_env;
                Frame::Return(
                    ty(f.type_id, table, row)?,
                    self.environment(f.saved_env, table, row)?,
                )
            }
            8 => {
                used.saved_env = f.saved_env;
                Frame::Scope(self.environment(f.saved_env, table, row)?)
            }
            _ => return Err(wrong(table, row, "tag", "Frame tag must be 0..8")),
        };
        macro_rules! zero_column {
            ($field:ident, $name:literal) => {
                if used.$field != f.$field {
                    return Err(wrong(table, row, $name, "unused Frame column must be zero"));
                }
            };
        }
        zero_column!(name_id, "nameId");
        zero_column!(binary_op, "binaryOp");
        zero_column!(unary_op, "unaryOp");
        zero_column!(type_id, "typeId");
        zero_column!(expr0, "expr0");
        zero_column!(expr1, "expr1");
        zero_column!(saved_env, "savedEnv");
        zero_column!(value, "valueRef");
        zero_column!(completed, "completedSlice");
        zero_column!(remaining_exprs, "remainingExprSlice");
        zero_column!(arms, "armSlice");
        Ok(value)
    }
    fn control_row(
        &self,
        c: &ControlRow,
        row: usize,
        next: Option<Control>,
    ) -> StateResult<Control> {
        let table = "ControlRows";
        let mut used = ControlRow {
            tag: c.tag,
            ..ControlRow::default()
        };
        let value = match c.tag {
            0 => {
                used.expr_id = c.expr_id;
                Control::Eval(self.expr(c.expr_id, table, row)?)
            }
            1 => {
                used.value = c.value;
                Control::Return(self.value(c.value, table, row)?)
            }
            2 => {
                used.pending_frame = c.pending_frame.clone();
                used.environment = c.environment;
                let next = next.ok_or_else(|| {
                    wrong(
                        table,
                        row,
                        "next",
                        "Push requires a following active control row",
                    )
                })?;
                Control::Push(
                    self.frame(&c.pending_frame, table, row)?,
                    Box::new(next),
                    self.environment(c.environment, table, row)?,
                )
            }
            3 => {
                used.name_id = c.name_id;
                used.values = c.values;
                Control::Invoke(
                    self.name(c.name_id, table, row)?,
                    self.values(c.values, table, row)?,
                )
            }
            4 => {
                used.function_present = c.function_present;
                used.values = c.values;
                let function = if c.function_present {
                    used.function_id = c.function_id;
                    Some(self.function(c.function_id, row)?)
                } else {
                    None
                };
                Control::Enter(function, self.values(c.values, table, row)?)
            }
            5 => {
                used.bindings_present = c.bindings_present;
                used.type_id = c.type_id;
                used.expr_id = c.expr_id;
                let bindings = if c.bindings_present {
                    used.bindings = c.bindings;
                    Some(self.environment(c.bindings, table, row)?)
                } else {
                    None
                };
                Control::Bound(
                    bindings,
                    ty(c.type_id, table, row)?,
                    self.expr(c.expr_id, table, row)?,
                )
            }
            6 => {
                used.value = c.value;
                used.arms = c.arms;
                used.environment = c.environment;
                Control::Cases(
                    self.value(c.value, table, row)?,
                    self.arms(c.arms, table, row)?,
                    self.environment(c.environment, table, row)?,
                )
            }
            7 => {
                used.bindings_present = c.bindings_present;
                used.value = c.value;
                used.expr_id = c.expr_id;
                used.arms = c.arms;
                used.environment = c.environment;
                let bindings = if c.bindings_present {
                    used.bindings = c.bindings;
                    Some(self.environment(c.bindings, table, row)?)
                } else {
                    None
                };
                Control::CaseResult(
                    bindings,
                    self.value(c.value, table, row)?,
                    self.expr(c.expr_id, table, row)?,
                    self.arms(c.arms, table, row)?,
                    self.environment(c.environment, table, row)?,
                )
            }
            8 => {
                used.primitive_is_ok = c.primitive_is_ok;
                if c.primitive_is_ok {
                    used.value = c.value;
                    Control::PrimitiveResult(Ok(self.value(c.value, table, row)?))
                } else {
                    used.error_code = c.error_code;
                    Control::PrimitiveResult(Err(c.error_code))
                }
            }
            9 => {
                used.error_code = c.error_code;
                Control::Error(c.error_code)
            }
            _ => return Err(wrong(table, row, "tag", "Control tag must be 0..9")),
        };
        macro_rules! zero_column {
            ($field:ident, $name:literal) => {
                if used.$field != c.$field {
                    return Err(wrong(
                        table,
                        row,
                        $name,
                        "unused Control column or absent payload must be zero",
                    ));
                }
            };
        }
        zero_column!(expr_id, "exprId");
        zero_column!(value, "valueRef");
        zero_column!(pending_frame, "pendingFrame");
        zero_column!(environment, "environment");
        zero_column!(name_id, "nameId");
        zero_column!(values, "valueSlice");
        zero_column!(function_present, "functionPresent");
        zero_column!(function_id, "functionId");
        zero_column!(bindings_present, "bindingsPresent");
        zero_column!(bindings, "bindings");
        zero_column!(type_id, "typeId");
        zero_column!(arms, "armSlice");
        zero_column!(primitive_is_ok, "primitiveIsOk");
        zero_column!(error_code, "errorCode");
        Ok(value)
    }
    fn check_arenas(&mut self) -> StateResult<()> {
        // All nodes are visited in child-before-parent order, including rows
        // unreachable from current roots. No tree unfolding or recursive walk.
        for i in 0..self.prepared.block.constants.active {
            let node = self.check_node(ValueRef::constant(i))?;
            push(&mut self.constants, node, "constant validation cache")?;
        }
        for i in 0..self.state.root.active_data {
            let node = self.check_node(ValueRef {
                space: true,
                index: i,
            })?;
            push(&mut self.data, node, "data validation cache")?;
        }
        for (i, r) in self
            .state
            .data_fields
            .iter()
            .take(self.state.root.active_data_fields)
            .enumerate()
        {
            self.node(*r, "DataFields", i, "valueRef")?;
            if self.mode == DecodeMode::Canonical && !r.space {
                return Err(wrong(
                    "DataFields",
                    i,
                    "space",
                    "canonical state fields use Data space",
                ));
            }
        }
        for (i, r) in self
            .state
            .value_slots
            .iter()
            .take(self.state.root.active_value_slots)
            .enumerate()
        {
            self.node(*r, "ValueSlots", i, "valueRef")?;
            if self.mode == DecodeMode::Canonical && !r.space {
                return Err(wrong(
                    "ValueSlots",
                    i,
                    "space",
                    "canonical state values use Data space",
                ));
            }
        }
        for (i, c) in self
            .state
            .env_cells
            .iter()
            .take(self.state.root.active_env_cells)
            .enumerate()
        {
            self.name(c.name_id, "EnvCells", i)?;
            self.node(c.value, "EnvCells", i, "valueRef")?;
            self.env_ref(c.parent, "EnvCells", i, "parent")?;
            if c.parent.present && c.parent.index >= i {
                return Err(wrong(
                    "EnvCells",
                    i,
                    "parent",
                    "parent index must be lower than child",
                ));
            }
            if self.mode == DecodeMode::Canonical && !c.value.space {
                return Err(wrong(
                    "EnvCells",
                    i,
                    "valueRef.space",
                    "canonical bindings use Data space",
                ));
            }
        }
        Ok(())
    }
}

fn compare_program_block(expected: &ProgramBlock, supplied: &ProgramBlock) -> StateResult<()> {
    let failure = |table, row, column| StateFailure::WrongRepresentation {
        block: "ProgramBlock",
        table,
        row,
        column,
        definition: "supplied complete ProgramBlock must equal retained canonical preparation",
    };
    if expected.header.entry_expr_id != supplied.header.entry_expr_id {
        return Err(failure("ProgramHeader", 0, "entryExprId"));
    }
    for i in 0..15 {
        if expected.header.active_lengths[i] != supplied.header.active_lengths[i] {
            return Err(failure("ProgramHeader", i, "activeLengths"));
        }
    }
    macro_rules! rows { ($field:ident, $table:literal, $($column:ident => $name:literal),+) => {{
        let a=&expected.$field;let b=&supplied.$field;
        if a.active!=b.active{return Err(failure($table,0,"activeLength"));}
        if a.rows.len()!=b.rows.len(){return Err(failure($table,0,"fixedLength"));}
        for (i,(a,b)) in a.rows.iter().zip(&b.rows).enumerate() {$(if a.$column!=b.$column{return Err(failure($table,i,$name));})+}
    }}; }
    macro_rules! scalars {
        ($field:ident,$table:literal) => {{
            let a = &expected.$field;
            let b = &supplied.$field;
            if a.active != b.active {
                return Err(failure($table, 0, "activeLength"));
            }
            if a.rows.len() != b.rows.len() {
                return Err(failure($table, 0, "fixedLength"));
            }
            for (i, (a, b)) in a.rows.iter().zip(&b.rows).enumerate() {
                if a != b {
                    return Err(failure($table, i, "value"));
                }
            }
        }};
    }
    rows!(names,"Names",bytes=>"byteSlice");
    scalars!(name_bytes, "NameBytes");
    rows!(occurrences,"Occurrences",owner_tag=>"ownerTag",function_index=>"functionIndex",path=>"pathSlice");
    scalars!(occurrence_paths, "OccurrencePaths");
    rows!(expressions,"Expressions",tag=>"tag",occurrence_id=>"occurrenceId",value=>"valueRef",name_id=>"nameId",binary_op=>"binaryOp",unary_op=>"unaryOp",children=>"childSlice",arms=>"armSlice");
    scalars!(expr_children, "ExprChildren");
    rows!(functions,"Functions",source_table_index=>"sourceTableIndex",name_id=>"nameId",parameters=>"parameterSlice",return_type=>"returnType",body_expr_id=>"bodyExprId");
    rows!(parameters,"Parameters",name_id=>"nameId",type_id=>"typeId");
    rows!(patterns,"Patterns",tag=>"tag",name0=>"name0",name1=>"name1");
    rows!(arms,"Arms",pattern_id=>"patternId",body_expr_id=>"bodyExprId");
    rows!(types,"Types",kind=>"kind",constructors=>"constructorSlice");
    rows!(constructors,"Constructors",owning_type=>"owningType",source_order=>"sourceOrder",fields=>"fieldTypeSlice");
    scalars!(constructor_field_types, "ConstructorFieldTypes");
    rows!(constants,"Constants",kind=>"kind",type_id=>"typeId",ctor_id=>"ctorId",uint_bytes=>"uintBytes",bool_value=>"boolValue",fields=>"fieldSlice");
    rows!(constant_fields,"ConstantFields",space=>"space",index=>"index");
    Ok(())
}

/// Decode a complete internal typed image. PreparedA must be the retained
/// result of prepare_a; equality below binds the supplied complete block to
/// that canonical ProgramBlock, including inactive rows and all metadata.
/// No machine step, host truth flag, trace acceptance, or SDK call occurs.
///
/// Canonical mode has a real source dependency on state_encode. It refuses a
/// byte/logical-row mismatch against full re-encoding, including least-equal
/// program references, root visitation order, dead rows and sharing history.
/// The integrator must register both actual modules; no fallback is provided.
pub fn decode_observed_a(
    prepared: &PreparedA,
    capacities: &StateCapacities,
    image: &AImage<'_>,
    mode: DecodeMode,
) -> StateResult<Snapshot> {
    if image.profile != PROFILE {
        return Err(StateFailure::WrongProfile {
            locator: Locator::field("profile"),
        });
    }
    if !super::state_encode::programs_equal(prepared, image.full_program)? {
        return Err(StateFailure::ProgramMismatch {
            locator: Locator::field("fullProgram"),
        });
    }
    compare_program_block(&prepared.block, image.program)?;
    if capacities.data != prepared.capacities.data {
        return Err(wrong(
            "Root",
            0,
            "D",
            "state D must equal preparation's ValueRef-width D",
        ));
    }
    let s = image.state;
    let root = &s.root;
    fixed("Data", &s.data, root.active_data, capacities.data)?;
    fixed(
        "DataFields",
        &s.data_fields,
        root.active_data_fields,
        capacities.data_fields,
    )?;
    fixed(
        "EnvCells",
        &s.env_cells,
        root.active_env_cells,
        capacities.env_cells,
    )?;
    fixed(
        "ValueSlots",
        &s.value_slots,
        root.active_value_slots,
        capacities.value_slots,
    )?;
    fixed(
        "FrameSlots",
        &s.frame_slots,
        root.depth,
        capacities.frame_slots,
    )?;
    fixed(
        "ControlRows",
        &s.control_rows,
        root.control_length,
        capacities.control_rows,
    )?;
    if root.stack_limit > capacities.stack_limit {
        return Err(StateFailure::Capacity {
            table: "SL",
            needed: root.stack_limit,
            limit: capacities.stack_limit,
            locator: Locator::field("Root.stackLimit"),
        });
    }
    let stack_limit = usize::try_from(root.stack_limit).map_err(|_| host("Root.stackLimit", 0))?;
    let remaining_fuel = u128::from_le_bytes(root.remaining_fuel);
    let initial_fuel = u128::from_le_bytes(root.initial_fuel);
    if remaining_fuel > initial_fuel {
        return Err(StateFailure::OutsideObservationDomain {
            condition: "O1: remaining_fuel <= initial_fuel",
            locator: Locator::field("Root.remainingFuel"),
        });
    }
    if root.depth > stack_limit {
        return Err(StateFailure::OutsideObservationDomain {
            condition: "O2: depth <= stack_limit",
            locator: Locator::field("Root.depth"),
        });
    }
    if root.outcome_tag > 3 {
        return Err(wrong("Root", 0, "outcomeTag", "Outcome tag must be 0..3"));
    }
    if (root.outcome_tag == 0) != (root.control_length > 0) {
        return Err(StateFailure::OutsideObservationDomain {
            condition: "O3: Pending iff control is Some",
            locator: Locator::field("Root.controlLength"),
        });
    }
    if root.outcome_tag != 1 && root.outcome_value != ValueRef::default() {
        return Err(wrong(
            "Root",
            0,
            "outcomeValue",
            "non-Done outcome value must be zero",
        ));
    }
    if root.outcome_tag != 2 && root.outcome_error != 0 {
        return Err(wrong(
            "Root",
            0,
            "outcomeError",
            "non-Rejected outcome error must be zero",
        ));
    }
    for (i, c) in s.control_rows.iter().take(root.control_length).enumerate() {
        if (c.tag == 2) != (i + 1 < root.control_length) {
            return Err(wrong(
                "ControlRows",
                i,
                "tag",
                "all but final active row are Push; final row is non-Push",
            ));
        }
    }
    let mut reader = Reader {
        prepared,
        state: s,
        mode,
        constants: Vec::new(),
        data: Vec::new(),
    };
    reader.check_arenas()?;
    // Decode the sole control spine iteratively from tail to head. This changes
    // no canonical allocation ordering: only the real encoder allocates images.
    let mut control = None;
    for i in (0..root.control_length).rev() {
        control = Some(reader.control_row(&s.control_rows[i], i, control)?);
    }
    let environment = reader.environment(root.environment, "Root", 0)?;
    let mut frames = Vec::new();
    for (i, f) in s.frame_slots.iter().take(root.depth).enumerate() {
        push(
            &mut frames,
            reader.frame(f, "FrameSlots", i)?,
            "decoded frames",
        )?;
    }
    let outcome = match root.outcome_tag {
        0 => Outcome::Pending,
        1 => Outcome::Done(reader.value(root.outcome_value, "Root", 0)?),
        2 => Outcome::Rejected(root.outcome_error),
        3 => Outcome::Exhausted,
        _ => return Err(wrong("Root", 0, "outcomeTag", "Outcome tag must be 0..3")),
    };
    let snapshot = Snapshot {
        program: prepared.full_program.clone(),
        control,
        environment,
        frames,
        stack_limit,
        remaining_fuel,
        initial_fuel,
        outcome,
    };
    if mode == DecodeMode::Canonical {
        let canonical = super::state_encode::encode_observed_a(prepared, capacities, &snapshot)?;
        if &canonical != s {
            return Err(wrong(
                "StateBlock",
                0,
                "canonical image",
                "complete canonical re-encoding must equal every supplied row and root column",
            ));
        }
    }
    Ok(snapshot)
}
