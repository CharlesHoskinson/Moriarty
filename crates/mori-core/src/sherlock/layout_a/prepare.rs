use super::{
    constants,
    limits::{self, Limits, Locator, Result},
    rows::*,
};
use crate::sherlock::{BinaryOp, Expr, ExprRef, Pattern, Program, Type, UnaryOp};
use std::sync::Arc;

fn ty(t: Type) -> u8 {
    match t {
        Type::UInt => 0,
        Type::Bool => 1,
        Type::List => 2,
        Type::Tree => 3,
    }
}
fn slice(start: usize, len: usize) -> Slice {
    if len == 0 {
        Slice::ZERO
    } else {
        Slice { start, len }
    }
}
fn zeros(n: usize, loc: Locator) -> Result<Vec<usize>> {
    let mut v = Vec::new();
    v.try_reserve_exact(n).map_err(|_| limits::host(loc))?;
    v.resize(n, 0);
    Ok(v)
}
struct Task {
    source: ExprRef,
    owner: u8,
    function: usize,
    path: Vec<usize>,
    parent: Option<(usize, bool, usize)>,
}
struct Draft {
    limits: Limits,
    metadata: Vec<OccurrenceMetadata>,
    children: Vec<Vec<usize>>,
    arm_bodies: Vec<Vec<usize>>,
    names: Vec<String>,
}
impl Draft {
    fn name(&mut self, name: &str, loc: Locator) -> Result<()> {
        if self.names.iter().any(|s| s == name) {
            return Ok(());
        }
        self.limits.reserve(TableId::Names, 1, loc)?;
        self.limits.reserve(TableId::NameBytes, name.len(), loc)?;
        let mut owned = String::new();
        owned
            .try_reserve_exact(name.len())
            .map_err(|_| limits::host(loc))?;
        owned.push_str(name);
        limits::push(&mut self.names, owned, loc)
    }
    fn pattern_names(&mut self, pattern: &Pattern, loc: Locator) -> Result<()> {
        match pattern {
            Pattern::Nil => Ok(()),
            Pattern::Var(a) | Pattern::Leaf(a) => self.name(a, loc),
            Pattern::Cons(a, b) | Pattern::Branch(a, b) => {
                self.name(a, loc)?;
                self.name(b, loc)
            }
        }
    }
    fn enqueue(
        &mut self,
        tasks: &mut Vec<Task>,
        source: &ExprRef,
        owner: u8,
        function: usize,
        path: &[usize],
        position: Option<usize>,
        parent: Option<(usize, bool, usize)>,
    ) -> Result<()> {
        let loc = Locator {
            expression: parent.map(|p| p.0),
            function: if owner == 1 { Some(function) } else { None },
            field: "expression occurrence",
            item: position.unwrap_or(0),
        };
        let length = path
            .len()
            .checked_add(usize::from(position.is_some()))
            .ok_or_else(|| limits::host(loc))?;
        // Reserve output and path sizes before cloning a path or expanding a node.
        self.limits.reserve(TableId::Expressions, 1, loc)?;
        self.limits.reserve(TableId::Occurrences, 1, loc)?;
        self.limits.reserve(TableId::OccurrencePaths, length, loc)?;
        let mut child_path = limits::copy_slice(path, loc)?;
        if let Some(p) = position {
            limits::push(&mut child_path, p, loc)?;
        }
        limits::push(
            tasks,
            Task {
                source: source.clone(),
                owner,
                function,
                path: child_path,
                parent,
            },
            loc,
        )
    }
    fn root(&mut self, source: &ExprRef, owner: u8, function: usize) -> Result<usize> {
        let root = self.metadata.len();
        let mut tasks = Vec::new();
        self.enqueue(&mut tasks, source, owner, function, &[], None, None)?;
        while let Some(task) = tasks.pop() {
            let id = self.metadata.len();
            let loc = Locator {
                expression: Some(id),
                function: if task.owner == 1 {
                    Some(task.function)
                } else {
                    None
                },
                field: "expression",
                item: 0,
            };
            if let Some((parent, arm, position)) = task.parent {
                if arm {
                    self.arm_bodies[parent][position] = id;
                } else {
                    self.children[parent][position] = id;
                }
            }
            let (child_count, arm_count) = match task.source.as_ref() {
                Expr::Literal(_) | Expr::Var(_) => (0, 0),
                Expr::Let(_, _, _) | Expr::Binary(_, _, _) => (2, 0),
                Expr::Unary(_, _) => (1, 0),
                Expr::If(_, _, _) => (3, 0),
                Expr::Call(_, args) => (args.len(), 0),
                Expr::Match(_, arms) => (1, arms.len()),
            };
            self.limits
                .reserve(TableId::ExprChildren, child_count, loc)?;
            self.limits.reserve(TableId::Arms, arm_count, loc)?;
            self.limits.reserve(TableId::Patterns, arm_count, loc)?;
            limits::push(&mut self.children, zeros(child_count, loc)?, loc)?;
            limits::push(&mut self.arm_bodies, zeros(arm_count, loc)?, loc)?;
            // Reverse push order yields source preorder. No named Call is followed.
            match task.source.as_ref() {
                Expr::Literal(_) => {}
                Expr::Var(name) => self.name(name, loc)?,
                Expr::Let(name, a, b) => {
                    self.name(name, loc)?;
                    self.enqueue(
                        &mut tasks,
                        b,
                        owner,
                        function,
                        &task.path,
                        Some(1),
                        Some((id, false, 1)),
                    )?;
                    self.enqueue(
                        &mut tasks,
                        a,
                        owner,
                        function,
                        &task.path,
                        Some(0),
                        Some((id, false, 0)),
                    )?;
                }
                Expr::Binary(_, a, b) => {
                    self.enqueue(
                        &mut tasks,
                        b,
                        owner,
                        function,
                        &task.path,
                        Some(1),
                        Some((id, false, 1)),
                    )?;
                    self.enqueue(
                        &mut tasks,
                        a,
                        owner,
                        function,
                        &task.path,
                        Some(0),
                        Some((id, false, 0)),
                    )?;
                }
                Expr::Unary(_, a) => self.enqueue(
                    &mut tasks,
                    a,
                    owner,
                    function,
                    &task.path,
                    Some(0),
                    Some((id, false, 0)),
                )?,
                Expr::If(a, b, c) => {
                    self.enqueue(
                        &mut tasks,
                        c,
                        owner,
                        function,
                        &task.path,
                        Some(2),
                        Some((id, false, 2)),
                    )?;
                    self.enqueue(
                        &mut tasks,
                        b,
                        owner,
                        function,
                        &task.path,
                        Some(1),
                        Some((id, false, 1)),
                    )?;
                    self.enqueue(
                        &mut tasks,
                        a,
                        owner,
                        function,
                        &task.path,
                        Some(0),
                        Some((id, false, 0)),
                    )?;
                }
                Expr::Call(name, args) => {
                    self.name(name, loc)?;
                    for (i, arg) in args.iter().enumerate().rev() {
                        self.enqueue(
                            &mut tasks,
                            arg,
                            owner,
                            function,
                            &task.path,
                            Some(i),
                            Some((id, false, i)),
                        )?;
                    }
                }
                Expr::Match(scrutinee, arms) => {
                    for arm in arms {
                        self.pattern_names(&arm.pattern, loc)?;
                    }
                    for (k, arm) in arms.iter().enumerate().rev() {
                        let position = k.checked_add(1).ok_or_else(|| limits::host(loc))?;
                        self.enqueue(
                            &mut tasks,
                            &arm.body,
                            owner,
                            function,
                            &task.path,
                            Some(position),
                            Some((id, true, k)),
                        )?;
                    }
                    self.enqueue(
                        &mut tasks,
                        scrutinee,
                        owner,
                        function,
                        &task.path,
                        Some(0),
                        Some((id, false, 0)),
                    )?;
                }
            }
            limits::push(
                &mut self.metadata,
                OccurrenceMetadata {
                    source: task.source,
                    owner_tag: task.owner,
                    function_index: task.function,
                    path: task.path,
                },
                loc,
            )?;
        }
        Ok(root)
    }
    fn name_id(&self, name: &str) -> usize {
        // Every use was collected from this same source before sorting.
        self.names
            .binary_search_by(|s| s.as_bytes().cmp(name.as_bytes()))
            .expect("collected syntax name")
    }
    fn pattern(&self, p: &Pattern) -> PatternRow {
        match p {
            Pattern::Var(a) => PatternRow {
                tag: 0,
                name0: self.name_id(a),
                name1: 0,
            },
            Pattern::Nil => PatternRow {
                tag: 1,
                ..PatternRow::default()
            },
            Pattern::Cons(a, b) => PatternRow {
                tag: 2,
                name0: self.name_id(a),
                name1: self.name_id(b),
            },
            Pattern::Leaf(a) => PatternRow {
                tag: 3,
                name0: self.name_id(a),
                name1: 0,
            },
            Pattern::Branch(a, b) => PatternRow {
                tag: 4,
                name0: self.name_id(a),
                name1: self.name_id(b),
            },
        }
    }
}

fn builtins(block: &mut ProgramBlock, limits: &mut Limits) -> Result<()> {
    let loc = Locator::field("builtin declarations");
    for row in [
        TypeRow {
            kind: 0,
            constructors: Slice::ZERO,
        },
        TypeRow {
            kind: 1,
            constructors: Slice::ZERO,
        },
        TypeRow {
            kind: 2,
            constructors: Slice { start: 0, len: 2 },
        },
        TypeRow {
            kind: 3,
            constructors: Slice { start: 2, len: 2 },
        },
    ] {
        limits.reserve(TableId::Types, 1, loc)?;
        limits::push(&mut block.types.rows, row, loc)?;
    }
    for row in [
        ConstructorRow {
            owning_type: 2,
            source_order: 0,
            fields: Slice::ZERO,
        },
        ConstructorRow {
            owning_type: 2,
            source_order: 1,
            fields: Slice { start: 0, len: 2 },
        },
        ConstructorRow {
            owning_type: 3,
            source_order: 0,
            fields: Slice { start: 2, len: 1 },
        },
        ConstructorRow {
            owning_type: 3,
            source_order: 1,
            fields: Slice { start: 3, len: 2 },
        },
    ] {
        limits.reserve(TableId::Constructors, 1, loc)?;
        limits::push(&mut block.constructors.rows, row, loc)?;
    }
    for t in [0, 2, 0, 3, 3] {
        limits.reserve(TableId::ConstructorFieldTypes, 1, loc)?;
        limits::push(&mut block.constructor_field_types.rows, t, loc)?;
    }
    Ok(())
}

/// Complete Program syntax preparation only. Caller supplies every dimension.
/// Retaining Arc<Program> avoids deep cloning recursive input. No evaluation,
/// static type admission, call unrolling, digest, decoder or native consumer.
pub fn prepare_a(program: Arc<Program>, capacities: Capacities) -> Result<PreparedA> {
    let mut d = Draft {
        limits: Limits::new(capacities)?,
        metadata: Vec::new(),
        children: Vec::new(),
        arm_bodies: Vec::new(),
        names: Vec::new(),
    };
    let mut b = ProgramBlock::default();
    builtins(&mut b, &mut d.limits)?;
    d.root(&program.entry, 0, 0)?;
    let mut function_bodies = Vec::new();
    for (i, f) in program.functions.iter().enumerate() {
        let loc = Locator {
            function: Some(i),
            ..Locator::field("function")
        };
        d.limits.reserve(TableId::Functions, 1, loc)?;
        d.limits
            .reserve(TableId::Parameters, f.parameters.len(), loc)?;
        d.name(&f.name, loc)?;
        for (j, p) in f.parameters.iter().enumerate() {
            d.name(
                &p.name,
                Locator {
                    item: j,
                    field: "parameter",
                    ..loc
                },
            )?;
        }
        let root = d.root(&f.body, 1, i)?;
        limits::push(&mut function_bodies, root, loc)?;
    }
    d.names
        .sort_unstable_by(|a, b| a.as_bytes().cmp(b.as_bytes()));
    for name in &d.names {
        let loc = Locator::field("name bytes");
        let start = b.name_bytes.rows.len();
        for byte in name.bytes() {
            limits::push(&mut b.name_bytes.rows, byte, loc)?;
        }
        limits::push(
            &mut b.names.rows,
            NameRow {
                bytes: slice(start, name.len()),
            },
            loc,
        )?;
    }
    for id in 0..d.metadata.len() {
        let meta = &d.metadata[id];
        let loc = Locator {
            expression: Some(id),
            function: if meta.owner_tag == 1 {
                Some(meta.function_index)
            } else {
                None
            },
            ..Locator::field("expression row")
        };
        let path_start = b.occurrence_paths.rows.len();
        for &p in &meta.path {
            limits::push(&mut b.occurrence_paths.rows, p, loc)?;
        }
        limits::push(
            &mut b.occurrences.rows,
            OccurrenceRow {
                owner_tag: meta.owner_tag,
                function_index: meta.function_index,
                path: slice(path_start, meta.path.len()),
            },
            loc,
        )?;
        let start = b.expr_children.rows.len();
        for &child in &d.children[id] {
            limits::push(&mut b.expr_children.rows, child, loc)?;
        }
        let mut row = ExpressionRow {
            occurrence_id: id,
            children: slice(start, d.children[id].len()),
            ..ExpressionRow::default()
        };
        match meta.source.as_ref() {
            Expr::Literal(v) => {
                row.tag = 0;
                row.value = constants::append(v, &mut b, &mut d.limits, loc)?;
            }
            Expr::Var(name) => {
                row.tag = 1;
                row.name_id = d.name_id(name);
            }
            Expr::Let(name, _, _) => {
                row.tag = 2;
                row.name_id = d.name_id(name);
            }
            Expr::Binary(op, _, _) => {
                row.tag = 3;
                row.binary_op = match op {
                    BinaryOp::Add => 0,
                    BinaryOp::Sub => 1,
                    BinaryOp::Mul => 2,
                    BinaryOp::Eq => 3,
                    BinaryOp::Lt => 4,
                    BinaryOp::Cons => 5,
                    BinaryOp::Branch => 6,
                };
            }
            Expr::Unary(op, _) => {
                row.tag = 4;
                row.unary_op = match op {
                    UnaryOp::Leaf => 0,
                    UnaryOp::Not => 1,
                };
            }
            Expr::If(_, _, _) => row.tag = 5,
            Expr::Call(name, _) => {
                row.tag = 6;
                row.name_id = d.name_id(name);
            }
            Expr::Match(_, arms) => {
                row.tag = 7;
                row.arms = slice(b.arms.rows.len(), arms.len());
                for (i, arm) in arms.iter().enumerate() {
                    let pattern_id = b.patterns.rows.len();
                    limits::push(&mut b.patterns.rows, d.pattern(&arm.pattern), loc)?;
                    limits::push(
                        &mut b.arms.rows,
                        ArmRow {
                            pattern_id,
                            body_expr_id: d.arm_bodies[id][i],
                        },
                        loc,
                    )?;
                }
            }
        }
        limits::push(&mut b.expressions.rows, row, loc)?;
    }
    for (i, f) in program.functions.iter().enumerate() {
        let loc = Locator {
            function: Some(i),
            ..Locator::field("function row")
        };
        let start = b.parameters.rows.len();
        for p in &f.parameters {
            limits::push(
                &mut b.parameters.rows,
                ParameterRow {
                    name_id: d.name_id(&p.name),
                    type_id: ty(p.ty),
                },
                loc,
            )?;
        }
        limits::push(
            &mut b.functions.rows,
            FunctionRow {
                source_table_index: i,
                name_id: d.name_id(&f.name),
                parameters: slice(start, f.parameters.len()),
                return_type: ty(f.return_type),
                body_expr_id: function_bodies[i],
            },
            loc,
        )?;
    }
    macro_rules! pad {
        ($field:ident,$table:ident) => {
            d.limits.pad(TableId::$table, &mut b.$field)?;
            b.header.active_lengths[TableId::$table as usize] = b.$field.active;
        };
    }
    pad!(names, Names);
    pad!(name_bytes, NameBytes);
    pad!(occurrences, Occurrences);
    pad!(occurrence_paths, OccurrencePaths);
    pad!(expressions, Expressions);
    pad!(expr_children, ExprChildren);
    pad!(functions, Functions);
    pad!(parameters, Parameters);
    pad!(patterns, Patterns);
    pad!(arms, Arms);
    pad!(types, Types);
    pad!(constructors, Constructors);
    pad!(constructor_field_types, ConstructorFieldTypes);
    pad!(constants, Constants);
    pad!(constant_fields, ConstantFields);
    Ok(PreparedA {
        block: b,
        capacities: d.limits.capacities,
        full_program: program,
        occurrences: d.metadata,
        names: d.names,
    })
}
