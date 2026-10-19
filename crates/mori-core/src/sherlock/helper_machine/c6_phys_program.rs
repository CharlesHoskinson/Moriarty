//! Program-section codec for sherlock-c6-phys/1 (C6P1), Chunk 2.
//!
//! Encodes and decodes the owned `ProgramSnapshot` only. It does not admit
//! programs, construct `Admitted` or `MachineImage`, execute steps, or claim
//! admission, execution, native binding, or proof qualification. The lane
//! contract is the owner-declared Chunk 2 contract: same C6P1 magic/version,
//! program section tag at lane 2, tables in ProgramSnapshot field order.
//!
//! Caps: table counts use the `C6PhysCaps` field named for that table.
//! Declaration parameters, fields per constructor and Apply arity are capped
//! by `max_arity`; constructors per declaration by `max_names`. The names
//! table's bytes count against `max_name_bytes`; declaration-template text
//! counts against the separate codec work limit `max_template_bytes`.
//! First error is in lane-stream order. At a count lane the order is
//! UsizeOverflow, CapacityExceeded, then Truncated when the count exceeds the
//! remaining lanes, refused before any allocation. At a string the order is
//! length, cumulative budget, remaining lanes, then bytes; invalid UTF-8 is
//! BadTag { tag: len }. TrailingLanes comes after the last table.

use super::c6_phys::{C6PhysCaps, C6PhysError, C6Result, C6_PHYS_MAGIC, C6_PHYS_VERSION};
use super::complete_observation::{
    ConstFieldsSnapshot, ConstantSnapshot, ExpressionSnapshot, HostSnapshot, ProgramSnapshot,
};
use super::shared::adt::{CtorIdC, TypeIdC, TypeRowC};
use super::shared::adt_storage::{CtorFieldRowExt, CtorRowExt};
use super::shared::owner_ext_type_builder::{CtorDeclSrc, CtorFieldSrc, TypeDeclSrc, TypeExprSrc};
use super::shared::{
    ArmRowC, BinaryOp, FunctionRowC, OccurrenceRowC, ParameterRowC, PatternRowC, PlanNodeC,
    SliceC, UnaryOp,
};

pub const PROGRAM_SECTION_TAG: u128 = 0x5052_4F47;
pub const PROGRAM_HEADER_LANES: usize = 4;

fn check_count(section: &'static str, count: usize, limit: usize) -> C6Result<()> {
    if count > limit {
        return Err(C6PhysError::CapacityExceeded { section, needed: count, limit });
    }
    Ok(())
}

fn binary_tag(op: BinaryOp) -> u128 {
    match op {
        BinaryOp::Add => 0,
        BinaryOp::Sub => 1,
        BinaryOp::Mul => 2,
        BinaryOp::Eq => 3,
        BinaryOp::Lt => 4,
        BinaryOp::Cons => 5,
        BinaryOp::Branch => 6,
    }
}

fn binary_from_tag(tag: u128) -> C6Result<BinaryOp> {
    Ok(match tag {
        0 => BinaryOp::Add,
        1 => BinaryOp::Sub,
        2 => BinaryOp::Mul,
        3 => BinaryOp::Eq,
        4 => BinaryOp::Lt,
        5 => BinaryOp::Cons,
        6 => BinaryOp::Branch,
        _ => return Err(C6PhysError::BadTag { section: "program.binary_op", tag }),
    })
}

fn unary_tag(op: UnaryOp) -> u128 {
    match op {
        UnaryOp::Leaf => 0,
        UnaryOp::Not => 1,
    }
}

fn unary_from_tag(tag: u128) -> C6Result<UnaryOp> {
    Ok(match tag {
        0 => UnaryOp::Leaf,
        1 => UnaryOp::Not,
        _ => return Err(C6PhysError::BadTag { section: "program.unary_op", tag }),
    })
}

#[derive(Clone, Copy)]
enum TextBudget {
    Names,
    Template,
}

impl TextBudget {
    fn section(self) -> &'static str {
        match self {
            TextBudget::Names => "program.name_bytes",
            TextBudget::Template => "program.template_bytes",
        }
    }
    fn limit(self, caps: &C6PhysCaps) -> usize {
        match self {
            TextBudget::Names => caps.max_name_bytes,
            TextBudget::Template => caps.max_template_bytes,
        }
    }
}

struct Writer<'a> {
    out: Vec<u128>,
    caps: &'a C6PhysCaps,
    name_bytes: usize,
    template_bytes: usize,
    type_nodes: usize,
}

impl<'a> Writer<'a> {
    fn new(caps: &'a C6PhysCaps) -> Self {
        Self { out: Vec::new(), caps, name_bytes: 0, template_bytes: 0, type_nodes: 0 }
    }
    fn usize(&mut self, value: usize) {
        self.out.push(value as u128);
    }
    fn raw(&mut self, value: u128) {
        self.out.push(value);
    }
    fn bool(&mut self, value: bool) {
        self.out.push(u128::from(value));
    }
    fn slice(&mut self, value: SliceC) {
        self.usize(value.start);
        self.usize(value.len);
    }
    fn opt_usize(&mut self, value: Option<usize>) {
        match value {
            None => self.out.extend([0, 0]),
            Some(index) => {
                self.raw(1);
                self.usize(index);
            }
        }
    }
    fn count(&mut self, section: &'static str, count: usize, limit: usize) -> C6Result<()> {
        check_count(section, count, limit)?;
        self.usize(count);
        Ok(())
    }
    fn string(&mut self, section: &'static str, value: &str, budget: TextBudget) -> C6Result<()> {
        let next = match budget {
            TextBudget::Names => self.name_bytes.checked_add(value.len()),
            TextBudget::Template => self.template_bytes.checked_add(value.len()),
        }
        .ok_or(C6PhysError::UsizeOverflow { section, value: value.len() as u128 })?;
        check_count(budget.section(), next, budget.limit(self.caps))?;
        match budget {
            TextBudget::Names => self.name_bytes = next,
            TextBudget::Template => self.template_bytes = next,
        }
        self.usize(value.len());
        for byte in value.bytes() {
            self.out.push(u128::from(byte));
        }
        Ok(())
    }
    fn type_expr(&mut self, root: &TypeExprSrc) -> C6Result<()> {
        // Iterative preorder: an Apply node is written before its children.
        let mut work: Vec<&TypeExprSrc> = vec![root];
        while let Some(node) = work.pop() {
            self.type_nodes = self
                .type_nodes
                .checked_add(1)
                .ok_or(C6PhysError::UsizeOverflow { section: "program.type_expr", value: 0 })?;
            check_count("program.type_expr_nodes", self.type_nodes, self.caps.max_type_expr_nodes)?;
            match node {
                TypeExprSrc::UInt => self.raw(0),
                TypeExprSrc::Bool => self.raw(1),
                TypeExprSrc::List => self.raw(2),
                TypeExprSrc::Tree => self.raw(3),
                TypeExprSrc::Named { name } => {
                    self.raw(4);
                    self.string("program.type_expr", name, TextBudget::Template)?;
                }
                TypeExprSrc::Param { name } => {
                    self.raw(5);
                    self.string("program.type_expr", name, TextBudget::Template)?;
                }
                TypeExprSrc::Apply { name, args } => {
                    self.raw(6);
                    self.string("program.type_expr", name, TextBudget::Template)?;
                    check_count("program.type_expr_arity", args.len(), self.caps.max_arity)?;
                    self.usize(args.len());
                    for arg in args.iter().rev() {
                        work.push(arg);
                    }
                }
            }
        }
        Ok(())
    }
}

struct Reader<'a> {
    lanes: &'a [u128],
    pos: usize,
    caps: &'a C6PhysCaps,
    name_bytes: usize,
    template_bytes: usize,
    type_nodes: usize,
}

impl<'a> Reader<'a> {
    fn new(lanes: &'a [u128], caps: &'a C6PhysCaps) -> Self {
        Self { lanes, pos: 0, caps, name_bytes: 0, template_bytes: 0, type_nodes: 0 }
    }
    fn remaining(&self) -> usize {
        self.lanes.len() - self.pos
    }
    fn raw(&mut self, section: &'static str) -> C6Result<u128> {
        let value = self.lanes.get(self.pos).copied().ok_or(C6PhysError::Truncated {
            section,
            needed: self.pos + 1,
            actual: self.lanes.len(),
        })?;
        self.pos += 1;
        Ok(value)
    }
    fn usize(&mut self, section: &'static str) -> C6Result<usize> {
        let value = self.raw(section)?;
        usize::try_from(value).map_err(|_| C6PhysError::UsizeOverflow { section, value })
    }
    fn bool(&mut self, section: &'static str) -> C6Result<bool> {
        match self.raw(section)? {
            0 => Ok(false),
            1 => Ok(true),
            value => Err(C6PhysError::BadBool { section, value }),
        }
    }
    fn slice(&mut self, section: &'static str) -> C6Result<SliceC> {
        Ok(SliceC { start: self.usize(section)?, len: self.usize(section)? })
    }
    fn opt_usize(&mut self, section: &'static str) -> C6Result<Option<usize>> {
        match self.raw(section)? {
            0 => {
                let padding = self.raw(section)?;
                if padding != 0 {
                    return Err(C6PhysError::NonZeroPadding { section, lane: self.pos - 1, value: padding });
                }
                Ok(None)
            }
            1 => Ok(Some(self.usize(section)?)),
            tag => Err(C6PhysError::BadTag { section, tag }),
        }
    }
    fn count(&mut self, section: &'static str, limit: usize) -> C6Result<usize> {
        let count = self.usize(section)?;
        check_count(section, count, limit)?;
        // Every row occupies at least one lane, so a longer count is truncated
        // input. Refusing here bounds the following allocation by the input.
        if count > self.remaining() {
            return Err(C6PhysError::Truncated {
                section,
                needed: self.pos.saturating_add(count),
                actual: self.lanes.len(),
            });
        }
        Ok(count)
    }
    fn string(&mut self, section: &'static str, budget: TextBudget) -> C6Result<String> {
        let len = self.usize(section)?;
        let next = match budget {
            TextBudget::Names => self.name_bytes.checked_add(len),
            TextBudget::Template => self.template_bytes.checked_add(len),
        }
        .ok_or(C6PhysError::UsizeOverflow { section, value: len as u128 })?;
        check_count(budget.section(), next, budget.limit(self.caps))?;
        match budget {
            TextBudget::Names => self.name_bytes = next,
            TextBudget::Template => self.template_bytes = next,
        }
        if len > self.remaining() {
            return Err(C6PhysError::Truncated {
                section,
                needed: self.pos.saturating_add(len),
                actual: self.lanes.len(),
            });
        }
        let mut bytes = Vec::with_capacity(len);
        for _ in 0..len {
            let lane = self.raw(section)?;
            let byte = u8::try_from(lane).map_err(|_| C6PhysError::BadTag { section, tag: lane })?;
            bytes.push(byte);
        }
        String::from_utf8(bytes).map_err(|_| C6PhysError::BadTag { section, tag: len as u128 })
    }
    fn type_expr(&mut self) -> C6Result<TypeExprSrc> {
        struct ApplyBuilder {
            name: String,
            expected: usize,
            args: Vec<TypeExprSrc>,
        }
        // Iterative: read one node, push incomplete Apply builders, and fold
        // each completed value into its parent until the root completes.
        let section = "program.type_expr";
        let mut builders: Vec<ApplyBuilder> = Vec::new();
        loop {
            self.type_nodes = self
                .type_nodes
                .checked_add(1)
                .ok_or(C6PhysError::UsizeOverflow { section, value: 0 })?;
            check_count("program.type_expr_nodes", self.type_nodes, self.caps.max_type_expr_nodes)?;
            let tag = self.raw(section)?;
            let completed: Option<TypeExprSrc> = match tag {
                0 => Some(TypeExprSrc::UInt),
                1 => Some(TypeExprSrc::Bool),
                2 => Some(TypeExprSrc::List),
                3 => Some(TypeExprSrc::Tree),
                4 => Some(TypeExprSrc::Named { name: self.string(section, TextBudget::Template)? }),
                5 => Some(TypeExprSrc::Param { name: self.string(section, TextBudget::Template)? }),
                6 => {
                    let name = self.string(section, TextBudget::Template)?;
                    let arity = self.usize(section)?;
                    check_count("program.type_expr_arity", arity, self.caps.max_arity)?;
                    if arity == 0 {
                        Some(TypeExprSrc::Apply { name, args: Vec::new() })
                    } else {
                        builders.push(ApplyBuilder { name, expected: arity, args: Vec::new() });
                        None
                    }
                }
                tag => return Err(C6PhysError::BadTag { section, tag }),
            };
            if let Some(mut value) = completed {
                loop {
                    match builders.pop() {
                        None => return Ok(value),
                        Some(mut builder) => {
                            builder.args.push(value);
                            if builder.args.len() == builder.expected {
                                value = TypeExprSrc::Apply { name: builder.name, args: builder.args };
                            } else {
                                builders.push(builder);
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn encode_program(program: &ProgramSnapshot, caps: &C6PhysCaps) -> C6Result<Vec<u128>> {
    let mut w = Writer::new(caps);
    w.raw(C6_PHYS_MAGIC);
    w.raw(C6_PHYS_VERSION);
    w.raw(PROGRAM_SECTION_TAG);
    w.usize(program.entry);

    w.count("program.expressions", program.expressions.len(), caps.max_expressions)?;
    for expr in &program.expressions {
        match expr {
            ExpressionSnapshot::Literal { constant } => {
                w.raw(0);
                w.usize(*constant);
            }
            ExpressionSnapshot::Var { binder } => {
                w.raw(1);
                w.usize(*binder);
            }
            ExpressionSnapshot::Let { binder, value, body } => {
                w.raw(2);
                w.usize(*binder);
                w.usize(*value);
                w.usize(*body);
            }
            ExpressionSnapshot::Binary { op, left, right } => {
                w.raw(3);
                w.raw(binary_tag(*op));
                w.usize(*left);
                w.usize(*right);
            }
            ExpressionSnapshot::Unary { op, operand } => {
                w.raw(4);
                w.raw(unary_tag(*op));
                w.usize(*operand);
            }
            ExpressionSnapshot::If { cond, then_expr, else_expr } => {
                w.raw(5);
                w.usize(*cond);
                w.usize(*then_expr);
                w.usize(*else_expr);
            }
            ExpressionSnapshot::Call { function_name, args } => {
                w.raw(6);
                w.usize(*function_name);
                w.slice(*args);
            }
            ExpressionSnapshot::Match { scrutinee, arms, plan } => {
                w.raw(7);
                w.usize(*scrutinee);
                w.slice(*arms);
                w.opt_usize(*plan);
            }
            ExpressionSnapshot::Construct { type_id, ctor_id, fields, constant } => {
                w.raw(8);
                w.usize(type_id.0);
                w.usize(ctor_id.0);
                w.slice(*fields);
                w.opt_usize(*constant);
            }
        }
    }

    w.count("program.occurrences", program.occurrences.len(), caps.max_occurrences)?;
    for row in &program.occurrences {
        w.bool(row.owner_is_function);
        w.usize(row.function);
        w.slice(row.path);
    }

    w.count("program.occurrence_paths", program.occurrence_paths.len(), caps.max_occurrence_paths)?;
    for value in &program.occurrence_paths {
        w.usize(*value);
    }

    w.count("program.children", program.children.len(), caps.max_children)?;
    for value in &program.children {
        w.usize(*value);
    }

    w.count("program.functions", program.functions.len(), caps.max_functions)?;
    for row in &program.functions {
        w.usize(row.function_name);
        w.slice(row.params);
        w.usize(row.return_type.0);
        w.usize(row.body);
    }

    w.count("program.parameters", program.parameters.len(), caps.max_parameters)?;
    for row in &program.parameters {
        w.usize(row.binder);
        w.usize(row.sort.0);
    }

    w.count("program.arms", program.arms.len(), caps.max_arms)?;
    for row in &program.arms {
        w.usize(row.pattern);
        w.usize(row.body);
    }

    w.count("program.patterns", program.patterns.len(), caps.max_patterns)?;
    for row in &program.patterns {
        match row {
            PatternRowC::PVar { x } => {
                w.raw(0);
                w.usize(*x);
            }
            PatternRowC::PNil => w.raw(1),
            PatternRowC::PCons { x, y } => {
                w.raw(2);
                w.usize(*x);
                w.usize(*y);
            }
            PatternRowC::PLeaf { x } => {
                w.raw(3);
                w.usize(*x);
            }
            PatternRowC::PBranch { x, y } => {
                w.raw(4);
                w.usize(*x);
                w.usize(*y);
            }
            PatternRowC::Wildcard => w.raw(5),
            PatternRowC::Constructor { type_id, ctor_id, children } => {
                w.raw(6);
                w.usize(type_id.0);
                w.usize(ctor_id.0);
                w.slice(*children);
            }
        }
    }

    w.count("program.pattern_children", program.pattern_children.len(), caps.max_pattern_children)?;
    for value in &program.pattern_children {
        w.usize(*value);
    }

    w.count("program.types", program.types.len(), caps.max_types)?;
    for row in &program.types {
        w.usize(row.name_id);
        w.opt_usize(row.decl);
        w.slice(row.args);
        w.usize(row.ctor_start.0);
        w.usize(row.ctor_len);
    }

    w.count("program.type_args", program.type_args.len(), caps.max_types)?;
    for value in &program.type_args {
        w.usize(value.0);
    }

    w.count("program.constructors", program.constructors.len(), caps.max_constructors)?;
    for row in &program.constructors {
        w.usize(row.owning_type.0);
        w.usize(row.name_id);
        w.usize(row.source_order);
        w.usize(row.field_start);
        w.usize(row.field_len);
    }

    w.count("program.ctor_fields", program.ctor_fields.len(), caps.max_ctor_fields)?;
    for row in &program.ctor_fields {
        w.usize(row.name_id);
        w.usize(row.type_id.0);
    }

    w.count("program.decl_param_counts", program.decl_param_counts.len(), caps.max_decl_templates)?;
    for value in &program.decl_param_counts {
        w.usize(*value);
    }

    w.count("program.decl_ctor_counts", program.decl_ctor_counts.len(), caps.max_decl_templates)?;
    for value in &program.decl_ctor_counts {
        w.usize(*value);
    }

    w.count("program.decl_templates", program.decl_templates.len(), caps.max_decl_templates)?;
    for decl in &program.decl_templates {
        w.string("program.decl_templates", &decl.name, TextBudget::Template)?;
        w.count("program.decl_parameters", decl.parameters.len(), caps.max_arity)?;
        for parameter in &decl.parameters {
            w.string("program.decl_templates", parameter, TextBudget::Template)?;
        }
        w.count("program.decl_ctors", decl.ctors.len(), caps.max_names)?;
        for ctor in &decl.ctors {
            w.string("program.decl_templates", &ctor.name, TextBudget::Template)?;
            w.count("program.decl_fields", ctor.fields.len(), caps.max_arity)?;
            for field in &ctor.fields {
                w.string("program.decl_templates", &field.name, TextBudget::Template)?;
                w.type_expr(&field.ty)?;
            }
        }
    }

    w.count("program.plan_nodes", program.plan_nodes.len(), caps.max_plan_nodes)?;
    for node in &program.plan_nodes {
        match node {
            PlanNodeC::Switch { source_match, path, type_id, cases, default } => {
                w.raw(0);
                w.usize(*source_match);
                w.slice(*path);
                w.usize(type_id.0);
                w.slice(*cases);
                w.usize(*default);
            }
            PlanNodeC::Bind { source_match, path, binder, next } => {
                w.raw(1);
                w.usize(*source_match);
                w.slice(*path);
                w.usize(*binder);
                w.usize(*next);
            }
            PlanNodeC::Select { source_match, arm } => {
                w.raw(2);
                w.usize(*source_match);
                w.usize(*arm);
            }
            PlanNodeC::NoMatch { source_match } => {
                w.raw(3);
                w.usize(*source_match);
            }
        }
    }

    w.count("program.plan_cases", program.plan_cases.len(), caps.max_plan_cases)?;
    for case in &program.plan_cases {
        w.opt_usize(*case);
    }

    w.count("program.field_paths", program.field_paths.len(), caps.max_field_paths)?;
    for value in &program.field_paths {
        w.usize(*value);
    }

    w.count("program.constants", program.constants.len(), caps.max_constants)?;
    for constant in &program.constants {
        match constant {
            ConstantSnapshot::U128 { marker_free, n } => {
                w.raw(0);
                w.bool(*marker_free);
                w.raw(*n);
            }
            ConstantSnapshot::Bool { marker_free, b } => {
                w.raw(1);
                w.bool(*marker_free);
                w.bool(*b);
            }
            ConstantSnapshot::Nil { marker_free } => {
                w.raw(2);
                w.bool(*marker_free);
            }
            ConstantSnapshot::Cons { marker_free, fields } => {
                w.raw(3);
                w.bool(*marker_free);
                w.usize(fields.start);
                w.usize(fields.len);
            }
            ConstantSnapshot::Leaf { marker_free, fields } => {
                w.raw(4);
                w.bool(*marker_free);
                w.usize(fields.start);
                w.usize(fields.len);
            }
            ConstantSnapshot::Branch { marker_free, fields } => {
                w.raw(5);
                w.bool(*marker_free);
                w.usize(fields.start);
                w.usize(fields.len);
            }
            ConstantSnapshot::Constructor { marker_free, type_id, ctor_id, fields } => {
                w.raw(6);
                w.bool(*marker_free);
                w.usize(type_id.0);
                w.usize(ctor_id.0);
                w.usize(fields.start);
                w.usize(fields.len);
            }
            ConstantSnapshot::U128OutOfRange { marker_free } => {
                w.raw(7);
                w.bool(*marker_free);
            }
        }
    }

    w.count("program.constant_fields", program.constant_fields.len(), caps.max_constant_fields)?;
    for value in &program.constant_fields {
        w.usize(*value);
    }

    w.count("program.names", program.names.len(), caps.max_names)?;
    for name in &program.names {
        w.string("program.names", name, TextBudget::Names)?;
    }

    Ok(w.out)
}

pub fn decode_program(lanes: &[u128], caps: &C6PhysCaps) -> C6Result<ProgramSnapshot> {
    let section = "program";
    if lanes.len() < PROGRAM_HEADER_LANES {
        return Err(C6PhysError::Truncated {
            section,
            needed: PROGRAM_HEADER_LANES,
            actual: lanes.len(),
        });
    }
    if lanes[0] != C6_PHYS_MAGIC {
        return Err(C6PhysError::BadMagic { actual: lanes[0] });
    }
    if lanes[1] != C6_PHYS_VERSION {
        return Err(C6PhysError::BadVersion { actual: lanes[1] });
    }
    if lanes[2] != PROGRAM_SECTION_TAG {
        return Err(C6PhysError::BadTag { section, tag: lanes[2] });
    }
    let mut r = Reader::new(lanes, caps);
    r.pos = 3;
    let entry = r.usize(section)?;

    let expression_count = r.count("program.expressions", caps.max_expressions)?;
    let mut expressions = Vec::with_capacity(expression_count);
    for _ in 0..expression_count {
        let s = "program.expressions";
        expressions.push(match r.raw(s)? {
            0 => ExpressionSnapshot::Literal { constant: r.usize(s)? },
            1 => ExpressionSnapshot::Var { binder: r.usize(s)? },
            2 => ExpressionSnapshot::Let {
                binder: r.usize(s)?,
                value: r.usize(s)?,
                body: r.usize(s)?,
            },
            3 => ExpressionSnapshot::Binary {
                op: binary_from_tag(r.raw(s)?)?,
                left: r.usize(s)?,
                right: r.usize(s)?,
            },
            4 => ExpressionSnapshot::Unary {
                op: unary_from_tag(r.raw(s)?)?,
                operand: r.usize(s)?,
            },
            5 => ExpressionSnapshot::If {
                cond: r.usize(s)?,
                then_expr: r.usize(s)?,
                else_expr: r.usize(s)?,
            },
            6 => ExpressionSnapshot::Call {
                function_name: r.usize(s)?,
                args: r.slice(s)?,
            },
            7 => ExpressionSnapshot::Match {
                scrutinee: r.usize(s)?,
                arms: r.slice(s)?,
                plan: r.opt_usize(s)?,
            },
            8 => ExpressionSnapshot::Construct {
                type_id: TypeIdC(r.usize(s)?),
                ctor_id: CtorIdC(r.usize(s)?),
                fields: r.slice(s)?,
                constant: r.opt_usize(s)?,
            },
            tag => return Err(C6PhysError::BadTag { section: s, tag }),
        });
    }

    let occurrence_count = r.count("program.occurrences", caps.max_occurrences)?;
    let mut occurrences = Vec::with_capacity(occurrence_count);
    for _ in 0..occurrence_count {
        let s = "program.occurrences";
        occurrences.push(OccurrenceRowC {
            owner_is_function: r.bool(s)?,
            function: r.usize(s)?,
            path: r.slice(s)?,
        });
    }

    let path_count = r.count("program.occurrence_paths", caps.max_occurrence_paths)?;
    let mut occurrence_paths = Vec::with_capacity(path_count);
    for _ in 0..path_count {
        occurrence_paths.push(r.usize("program.occurrence_paths")?);
    }

    let children_count = r.count("program.children", caps.max_children)?;
    let mut children = Vec::with_capacity(children_count);
    for _ in 0..children_count {
        children.push(r.usize("program.children")?);
    }

    let function_count = r.count("program.functions", caps.max_functions)?;
    let mut functions = Vec::with_capacity(function_count);
    for _ in 0..function_count {
        let s = "program.functions";
        functions.push(FunctionRowC {
            function_name: r.usize(s)?,
            params: r.slice(s)?,
            return_type: TypeIdC(r.usize(s)?),
            body: r.usize(s)?,
        });
    }

    let parameter_count = r.count("program.parameters", caps.max_parameters)?;
    let mut parameters = Vec::with_capacity(parameter_count);
    for _ in 0..parameter_count {
        let s = "program.parameters";
        parameters.push(ParameterRowC { binder: r.usize(s)?, sort: TypeIdC(r.usize(s)?) });
    }

    let arm_count = r.count("program.arms", caps.max_arms)?;
    let mut arms = Vec::with_capacity(arm_count);
    for _ in 0..arm_count {
        let s = "program.arms";
        arms.push(ArmRowC { pattern: r.usize(s)?, body: r.usize(s)? });
    }

    let pattern_count = r.count("program.patterns", caps.max_patterns)?;
    let mut patterns = Vec::with_capacity(pattern_count);
    for _ in 0..pattern_count {
        let s = "program.patterns";
        patterns.push(match r.raw(s)? {
            0 => PatternRowC::PVar { x: r.usize(s)? },
            1 => PatternRowC::PNil,
            2 => PatternRowC::PCons { x: r.usize(s)?, y: r.usize(s)? },
            3 => PatternRowC::PLeaf { x: r.usize(s)? },
            4 => PatternRowC::PBranch { x: r.usize(s)?, y: r.usize(s)? },
            5 => PatternRowC::Wildcard,
            6 => PatternRowC::Constructor {
                type_id: TypeIdC(r.usize(s)?),
                ctor_id: CtorIdC(r.usize(s)?),
                children: r.slice(s)?,
            },
            tag => return Err(C6PhysError::BadTag { section: s, tag }),
        });
    }

    let pattern_children_count = r.count("program.pattern_children", caps.max_pattern_children)?;
    let mut pattern_children = Vec::with_capacity(pattern_children_count);
    for _ in 0..pattern_children_count {
        pattern_children.push(r.usize("program.pattern_children")?);
    }

    let type_count = r.count("program.types", caps.max_types)?;
    let mut types = Vec::with_capacity(type_count);
    for _ in 0..type_count {
        let s = "program.types";
        types.push(TypeRowC {
            name_id: r.usize(s)?,
            decl: r.opt_usize(s)?,
            args: r.slice(s)?,
            ctor_start: CtorIdC(r.usize(s)?),
            ctor_len: r.usize(s)?,
        });
    }

    let type_arg_count = r.count("program.type_args", caps.max_types)?;
    let mut type_args = Vec::with_capacity(type_arg_count);
    for _ in 0..type_arg_count {
        type_args.push(TypeIdC(r.usize("program.type_args")?));
    }

    let constructor_count = r.count("program.constructors", caps.max_constructors)?;
    let mut constructors = Vec::with_capacity(constructor_count);
    for _ in 0..constructor_count {
        let s = "program.constructors";
        constructors.push(CtorRowExt {
            owning_type: TypeIdC(r.usize(s)?),
            name_id: r.usize(s)?,
            source_order: r.usize(s)?,
            field_start: r.usize(s)?,
            field_len: r.usize(s)?,
        });
    }

    let ctor_field_count = r.count("program.ctor_fields", caps.max_ctor_fields)?;
    let mut ctor_fields = Vec::with_capacity(ctor_field_count);
    for _ in 0..ctor_field_count {
        let s = "program.ctor_fields";
        ctor_fields.push(CtorFieldRowExt {
            name_id: r.usize(s)?,
            type_id: TypeIdC(r.usize(s)?),
        });
    }

    let decl_param_count_count = r.count("program.decl_param_counts", caps.max_decl_templates)?;
    let mut decl_param_counts = Vec::with_capacity(decl_param_count_count);
    for _ in 0..decl_param_count_count {
        decl_param_counts.push(r.usize("program.decl_param_counts")?);
    }

    let decl_ctor_count_count = r.count("program.decl_ctor_counts", caps.max_decl_templates)?;
    let mut decl_ctor_counts = Vec::with_capacity(decl_ctor_count_count);
    for _ in 0..decl_ctor_count_count {
        decl_ctor_counts.push(r.usize("program.decl_ctor_counts")?);
    }

    let decl_count = r.count("program.decl_templates", caps.max_decl_templates)?;
    let mut decl_templates = Vec::with_capacity(decl_count);
    for _ in 0..decl_count {
        let s = "program.decl_templates";
        let name = r.string(s, TextBudget::Template)?;
        let parameter_count = r.count("program.decl_parameters", caps.max_arity)?;
        let mut parameters = Vec::with_capacity(parameter_count);
        for _ in 0..parameter_count {
            parameters.push(r.string(s, TextBudget::Template)?);
        }
        let ctor_count = r.count("program.decl_ctors", caps.max_names)?;
        let mut ctors = Vec::with_capacity(ctor_count);
        for _ in 0..ctor_count {
            let ctor_name = r.string(s, TextBudget::Template)?;
            let field_count = r.count("program.decl_fields", caps.max_arity)?;
            let mut fields = Vec::with_capacity(field_count);
            for _ in 0..field_count {
                let field_name = r.string(s, TextBudget::Template)?;
                let ty = r.type_expr()?;
                fields.push(CtorFieldSrc { name: field_name, ty });
            }
            ctors.push(CtorDeclSrc { name: ctor_name, fields });
        }
        decl_templates.push(TypeDeclSrc { name, parameters, ctors });
    }

    let plan_node_count = r.count("program.plan_nodes", caps.max_plan_nodes)?;
    let mut plan_nodes = Vec::with_capacity(plan_node_count);
    for _ in 0..plan_node_count {
        let s = "program.plan_nodes";
        plan_nodes.push(match r.raw(s)? {
            0 => PlanNodeC::Switch {
                source_match: r.usize(s)?,
                path: r.slice(s)?,
                type_id: TypeIdC(r.usize(s)?),
                cases: r.slice(s)?,
                default: r.usize(s)?,
            },
            1 => PlanNodeC::Bind {
                source_match: r.usize(s)?,
                path: r.slice(s)?,
                binder: r.usize(s)?,
                next: r.usize(s)?,
            },
            2 => PlanNodeC::Select { source_match: r.usize(s)?, arm: r.usize(s)? },
            3 => PlanNodeC::NoMatch { source_match: r.usize(s)? },
            tag => return Err(C6PhysError::BadTag { section: s, tag }),
        });
    }

    let plan_case_count = r.count("program.plan_cases", caps.max_plan_cases)?;
    let mut plan_cases = Vec::with_capacity(plan_case_count);
    for _ in 0..plan_case_count {
        plan_cases.push(r.opt_usize("program.plan_cases")?);
    }

    let field_path_count = r.count("program.field_paths", caps.max_field_paths)?;
    let mut field_paths = Vec::with_capacity(field_path_count);
    for _ in 0..field_path_count {
        field_paths.push(r.usize("program.field_paths")?);
    }

    let constant_count = r.count("program.constants", caps.max_constants)?;
    let mut constants = Vec::with_capacity(constant_count);
    for _ in 0..constant_count {
        let s = "program.constants";
        constants.push(match r.raw(s)? {
            0 => ConstantSnapshot::U128 { marker_free: r.bool(s)?, n: r.raw(s)? },
            1 => ConstantSnapshot::Bool { marker_free: r.bool(s)?, b: r.bool(s)? },
            2 => ConstantSnapshot::Nil { marker_free: r.bool(s)? },
            3 => ConstantSnapshot::Cons {
                marker_free: r.bool(s)?,
                fields: ConstFieldsSnapshot { start: r.usize(s)?, len: r.usize(s)? },
            },
            4 => ConstantSnapshot::Leaf {
                marker_free: r.bool(s)?,
                fields: ConstFieldsSnapshot { start: r.usize(s)?, len: r.usize(s)? },
            },
            5 => ConstantSnapshot::Branch {
                marker_free: r.bool(s)?,
                fields: ConstFieldsSnapshot { start: r.usize(s)?, len: r.usize(s)? },
            },
            6 => ConstantSnapshot::Constructor {
                marker_free: r.bool(s)?,
                type_id: TypeIdC(r.usize(s)?),
                ctor_id: CtorIdC(r.usize(s)?),
                fields: ConstFieldsSnapshot { start: r.usize(s)?, len: r.usize(s)? },
            },
            7 => ConstantSnapshot::U128OutOfRange { marker_free: r.bool(s)? },
            tag => return Err(C6PhysError::BadTag { section: s, tag }),
        });
    }

    let constant_field_count = r.count("program.constant_fields", caps.max_constant_fields)?;
    let mut constant_fields = Vec::with_capacity(constant_field_count);
    for _ in 0..constant_field_count {
        constant_fields.push(r.usize("program.constant_fields")?);
    }

    let name_count = r.count("program.names", caps.max_names)?;
    let mut names = Vec::with_capacity(name_count);
    for _ in 0..name_count {
        names.push(r.string("program.names", TextBudget::Names)?);
    }

    if r.pos != lanes.len() {
        return Err(C6PhysError::TrailingLanes { section, extra: lanes.len() - r.pos });
    }

    Ok(ProgramSnapshot {
        entry,
        expressions,
        occurrences,
        occurrence_paths,
        children,
        functions,
        parameters,
        arms,
        patterns,
        pattern_children,
        types,
        type_args,
        constructors,
        ctor_fields,
        decl_param_counts,
        decl_ctor_counts,
        decl_templates,
        plan_nodes,
        plan_cases,
        field_paths,
        constants,
        constant_fields,
        names,
    })
}

/// Normal owned-snapshot caller path: encode the program view already owned
/// inside a captured `HostSnapshot`. This never constructs `Admitted` or
/// `MachineImage` and claims no execution or admission. Chunk 4 calls
/// `encode_program(&host.admission.view, caps)` (equivalently this helper)
/// when assembling a host encoding, and `decode_program` for the inverse;
/// no parallel runtime is introduced.
pub fn program_lanes_from_host(host: &HostSnapshot, caps: &C6PhysCaps) -> C6Result<Vec<u128>> {
    encode_program(&host.admission.view, caps)
}

/// Bounded owned-program roundtrip over an already-owned snapshot.
pub fn roundtrip_program_owned(
    program: &ProgramSnapshot,
    caps: &C6PhysCaps,
) -> C6Result<ProgramSnapshot> {
    decode_program(&encode_program(program, caps)?, caps)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sherlock::helper_machine::c6_phys::fixtures;
    use crate::sherlock::helper_machine::complete_observation::capture;

    fn caps() -> C6PhysCaps {
        C6PhysCaps {
            max_frames: 64,
            max_expressions: 1024,
            max_names: 1024,
            max_arms: 256,
            max_children: 4096,
            max_value_slots: 256,
            max_constants: 1024,
            max_data_nodes: 1024,
            max_env_cells: 1024,
            max_functions: 256,
            max_arity: 8,
            max_types: 1024,
            max_occurrences: 1024,
            max_occurrence_paths: 8192,
            max_parameters: 1024,
            max_patterns: 1024,
            max_pattern_children: 4096,
            max_constructors: 1024,
            max_ctor_fields: 4096,
            max_decl_templates: 256,
            max_plan_nodes: 1024,
            max_plan_cases: 1024,
            max_field_paths: 8192,
            max_constant_fields: 4096,
            max_name_bytes: 65536,
            max_type_expr_nodes: 65536,
            max_template_bytes: 65536,
        }
    }

    fn empty_program() -> ProgramSnapshot {
        ProgramSnapshot {
            entry: 0,
            expressions: Vec::new(),
            occurrences: Vec::new(),
            occurrence_paths: Vec::new(),
            children: Vec::new(),
            functions: Vec::new(),
            parameters: Vec::new(),
            arms: Vec::new(),
            patterns: Vec::new(),
            pattern_children: Vec::new(),
            types: Vec::new(),
            type_args: Vec::new(),
            constructors: Vec::new(),
            ctor_fields: Vec::new(),
            decl_param_counts: Vec::new(),
            decl_ctor_counts: Vec::new(),
            decl_templates: Vec::new(),
            plan_nodes: Vec::new(),
            plan_cases: Vec::new(),
            field_paths: Vec::new(),
            constants: Vec::new(),
            constant_fields: Vec::new(),
            names: Vec::new(),
        }
    }

    fn nested_apply(depth: usize) -> TypeExprSrc {
        let mut ty = TypeExprSrc::UInt;
        for _ in 0..depth {
            ty = TypeExprSrc::Apply {
                name: "F".into(),
                args: vec![std::mem::replace(&mut ty, TypeExprSrc::UInt)],
            };
        }
        ty
    }

    fn full_program() -> ProgramSnapshot {
        ProgramSnapshot {
            entry: 8,
            expressions: vec![
                ExpressionSnapshot::Literal { constant: 0 },
                ExpressionSnapshot::Var { binder: 1 },
                ExpressionSnapshot::Let { binder: 1, value: 0, body: 1 },
                ExpressionSnapshot::Binary { op: BinaryOp::Cons, left: 0, right: 1 },
                ExpressionSnapshot::Unary { op: UnaryOp::Not, operand: 1 },
                ExpressionSnapshot::If { cond: 1, then_expr: 0, else_expr: 1 },
                ExpressionSnapshot::Call {
                    function_name: 2,
                    args: SliceC { start: 7, len: 0 },
                },
                ExpressionSnapshot::Match {
                    scrutinee: 0,
                    arms: SliceC { start: 0, len: 2 },
                    plan: Some(0),
                },
                ExpressionSnapshot::Construct {
                    type_id: TypeIdC(1_000_000),
                    ctor_id: CtorIdC(999_999),
                    fields: SliceC { start: 3, len: 0 },
                    constant: None,
                },
                ExpressionSnapshot::Match {
                    scrutinee: 1,
                    arms: SliceC { start: 2, len: 0 },
                    plan: None,
                },
                ExpressionSnapshot::Construct {
                    type_id: TypeIdC(4),
                    ctor_id: CtorIdC(4),
                    fields: SliceC { start: 0, len: 2 },
                    constant: Some(0),
                },
            ],
            occurrences: vec![
                OccurrenceRowC { owner_is_function: false, function: 0, path: SliceC { start: 0, len: 0 } },
                OccurrenceRowC { owner_is_function: true, function: 3, path: SliceC { start: 0, len: 2 } },
            ],
            occurrence_paths: vec![0, 1, 1],
            children: vec![5, 5, 9],
            functions: vec![FunctionRowC {
                function_name: 2,
                params: SliceC { start: 0, len: 1 },
                return_type: TypeIdC(4),
                body: 8,
            }],
            parameters: vec![ParameterRowC { binder: 1, sort: TypeIdC(1_000_000) }],
            arms: vec![
                ArmRowC { pattern: 0, body: 0 },
                ArmRowC { pattern: 6, body: 1 },
            ],
            patterns: vec![
                PatternRowC::PVar { x: 1 },
                PatternRowC::PNil,
                PatternRowC::PCons { x: 1, y: 2 },
                PatternRowC::PLeaf { x: 1 },
                PatternRowC::PBranch { x: 1, y: 2 },
                PatternRowC::Wildcard,
                PatternRowC::Constructor {
                    type_id: TypeIdC(4),
                    ctor_id: CtorIdC(4),
                    children: SliceC { start: 4, len: 0 },
                },
            ],
            pattern_children: vec![0, 5, 5],
            types: vec![
                TypeRowC {
                    name_id: 0,
                    decl: None,
                    args: SliceC { start: 9, len: 0 },
                    ctor_start: CtorIdC(0),
                    ctor_len: 0,
                },
                TypeRowC {
                    name_id: 3,
                    decl: Some(0),
                    args: SliceC { start: 0, len: 1 },
                    ctor_start: CtorIdC(4),
                    ctor_len: 1,
                },
            ],
            type_args: vec![TypeIdC(0), TypeIdC(1_000_000)],
            constructors: vec![CtorRowExt {
                owning_type: TypeIdC(4),
                name_id: 4,
                source_order: 0,
                field_start: 0,
                field_len: 2,
            }],
            ctor_fields: vec![
                CtorFieldRowExt { name_id: 5, type_id: TypeIdC(0) },
                CtorFieldRowExt { name_id: 6, type_id: TypeIdC(4) },
            ],
            decl_param_counts: vec![1, 0],
            decl_ctor_counts: vec![1, 1],
            decl_templates: vec![
                TypeDeclSrc {
                    name: "Box".into(),
                    parameters: vec!["A".into()],
                    ctors: vec![CtorDeclSrc {
                        name: "MkBox".into(),
                        fields: vec![
                            CtorFieldSrc { name: "head".into(), ty: TypeExprSrc::Param { name: "A".into() } },
                            CtorFieldSrc {
                                name: "rest".into(),
                                ty: TypeExprSrc::Apply {
                                    name: "Box".into(),
                                    args: vec![
                                        TypeExprSrc::Named { name: "Box".into() },
                                        TypeExprSrc::Apply { name: "Box".into(), args: Vec::new() },
                                        nested_apply(48),
                                    ],
                                },
                            },
                        ],
                    }],
                },
                TypeDeclSrc {
                    name: "Unit2".into(),
                    parameters: Vec::new(),
                    ctors: vec![CtorDeclSrc {
                        name: "MkUnit2".into(),
                        fields: vec![
                            CtorFieldSrc { name: "u".into(), ty: TypeExprSrc::UInt },
                            CtorFieldSrc { name: "b".into(), ty: TypeExprSrc::Bool },
                            CtorFieldSrc { name: "l".into(), ty: TypeExprSrc::List },
                            CtorFieldSrc { name: "t".into(), ty: TypeExprSrc::Tree },
                        ],
                    }],
                },
            ],
            plan_nodes: vec![
                PlanNodeC::Switch {
                    source_match: 7,
                    path: SliceC { start: 1, len: 0 },
                    type_id: TypeIdC(4),
                    cases: SliceC { start: 0, len: 1 },
                    default: 3,
                },
                PlanNodeC::Bind { source_match: 7, path: SliceC { start: 0, len: 1 }, binder: 1, next: 2 },
                PlanNodeC::Select { source_match: 7, arm: 1 },
                PlanNodeC::NoMatch { source_match: 7 },
            ],
            plan_cases: vec![None, Some(0), Some(2)],
            field_paths: vec![0, 1, 0],
            constants: vec![
                ConstantSnapshot::U128 { marker_free: true, n: u128::MAX },
                ConstantSnapshot::Bool { marker_free: false, b: true },
                ConstantSnapshot::Nil { marker_free: true },
                ConstantSnapshot::Cons {
                    marker_free: false,
                    fields: ConstFieldsSnapshot { start: 5, len: 0 },
                },
                ConstantSnapshot::Leaf {
                    marker_free: true,
                    fields: ConstFieldsSnapshot { start: 0, len: 1 },
                },
                ConstantSnapshot::Branch {
                    marker_free: false,
                    fields: ConstFieldsSnapshot { start: 1, len: 2 },
                },
                ConstantSnapshot::Constructor {
                    marker_free: true,
                    type_id: TypeIdC(1_000_000),
                    ctor_id: CtorIdC(999_999),
                    fields: ConstFieldsSnapshot { start: 0, len: 2 },
                },
                ConstantSnapshot::U128OutOfRange { marker_free: false },
            ],
            constant_fields: vec![7, 7, 0],
            names: vec!["héllo".into(), "héllo".into(), "".into(), "x".into()],
        }
    }

    #[test]
    fn empty_tables_roundtrip() {
        let program = empty_program();
        assert_eq!(roundtrip_program_owned(&program, &caps()).unwrap(), program);
    }

    #[test]
    fn full_program_roundtrip_all_variants_and_fields() {
        let program = full_program();
        let lanes = encode_program(&program, &caps()).unwrap();
        assert_eq!(lanes[0], C6_PHYS_MAGIC);
        assert_eq!(lanes[1], C6_PHYS_VERSION);
        assert_eq!(lanes[2], PROGRAM_SECTION_TAG);
        let decoded = decode_program(&lanes, &caps()).unwrap();
        assert_eq!(decoded, program);
        assert_eq!(encode_program(&decoded, &caps()).unwrap(), lanes, "accepted lanes re-encode identically");
    }

    #[test]
    fn owned_program_roundtrip_direct() {
        // Caller path only reads an already-owned ProgramSnapshot value; it
        // constructs no Admitted/MachineImage and claims no execution.
        let program = full_program();
        assert_eq!(
            decode_program(&encode_program(&program, &caps()).unwrap(), &caps()).unwrap(),
            program
        );
    }

    #[test]
    fn truncated_trailing_bad_magic_version_and_tag_are_refused() {
        let lanes = encode_program(&full_program(), &caps()).unwrap();
        let mut truncated = lanes.clone();
        truncated.pop();
        assert!(matches!(
            decode_program(&truncated, &caps()),
            Err(C6PhysError::Truncated { .. })
        ));
        let mut trailing = lanes.clone();
        trailing.push(0);
        assert!(matches!(
            decode_program(&trailing, &caps()),
            Err(C6PhysError::TrailingLanes { .. })
        ));
        let mut bad_magic = lanes.clone();
        bad_magic[0] = 0;
        assert!(matches!(
            decode_program(&bad_magic, &caps()),
            Err(C6PhysError::BadMagic { .. })
        ));
        let mut bad_version = lanes.clone();
        bad_version[1] = 99;
        assert!(matches!(
            decode_program(&bad_version, &caps()),
            Err(C6PhysError::BadVersion { .. })
        ));
        let mut bad_tag = lanes;
        bad_tag[2] = 99;
        assert!(matches!(
            decode_program(&bad_tag, &caps()),
            Err(C6PhysError::BadTag { .. })
        ));
    }

    #[test]
    fn noncanonical_none_padding_and_bad_bool_are_refused() {
        let mut program = empty_program();
        program.expressions.push(ExpressionSnapshot::Construct {
            type_id: TypeIdC(4),
            ctor_id: CtorIdC(4),
            fields: SliceC { start: 0, len: 0 },
            constant: None,
        });
        let mut lanes = encode_program(&program, &caps()).unwrap();
        // Header (4) + expression count (1): tag at 5, type 6, ctor 7,
        // fields 8..9, constant present tag at 10, padding lane at 11.
        assert_eq!(lanes[10], 0);
        lanes[11] = 9;
        assert!(matches!(
            decode_program(&lanes, &caps()),
            Err(C6PhysError::NonZeroPadding { .. })
        ));

        let mut program = empty_program();
        program.constants.push(ConstantSnapshot::U128OutOfRange { marker_free: true });
        let mut lanes = encode_program(&program, &caps()).unwrap();
        // Header (4) + 19 empty-table count lanes: constant tag at 24,
        // marker_free at 25.
        assert_eq!(lanes[24], 7);
        lanes[25] = 7;
        assert!(matches!(
            decode_program(&lanes, &caps()),
            Err(C6PhysError::BadBool { .. })
        ));
    }

    #[test]
    fn capacity_is_refused_on_encode_and_decode() {
        let program = full_program();
        let mut small = caps();
        small.max_expressions = 2;
        assert!(matches!(
            encode_program(&program, &small),
            Err(C6PhysError::CapacityExceeded { .. })
        ));
        let lanes = encode_program(&program, &caps()).unwrap();
        assert!(matches!(
            decode_program(&lanes, &small),
            Err(C6PhysError::CapacityExceeded { .. })
        ));
        let mut small_nodes = caps();
        small_nodes.max_type_expr_nodes = 4;
        assert!(matches!(
            encode_program(&program, &small_nodes),
            Err(C6PhysError::CapacityExceeded { .. })
        ));
    }
    #[test]
    fn count_beyond_remaining_lanes_is_truncated_before_allocation() {
        let mut lanes = encode_program(&empty_program(), &caps()).unwrap();
        assert_eq!(lanes.len(), PROGRAM_HEADER_LANES + 22);
        lanes[4] = 1000;
        assert_eq!(
            decode_program(&lanes, &caps()),
            Err(C6PhysError::Truncated { section: "program.expressions", needed: 1005, actual: 26 })
        );
    }

    #[test]
    fn template_and_name_budgets_are_separate() {
        let mut program = empty_program();
        program.names = vec!["abc".into()];
        program.decl_templates =
            vec![TypeDeclSrc { name: "Templ".into(), parameters: Vec::new(), ctors: Vec::new() }];
        let mut fit = caps();
        fit.max_name_bytes = 3;
        fit.max_template_bytes = 5;
        let lanes = encode_program(&program, &fit).unwrap();
        assert_eq!(decode_program(&lanes, &fit).unwrap(), program);
        let mut template = fit;
        template.max_template_bytes = 4;
        let refusal = Err(C6PhysError::CapacityExceeded { section: "program.template_bytes", needed: 5, limit: 4 });
        assert_eq!(encode_program(&program, &template), refusal);
        assert_eq!(decode_program(&lanes, &template).map(|_| ()), refusal.map(|_: Vec<u128>| ()));
        let mut names = fit;
        names.max_name_bytes = 2;
        assert_eq!(
            encode_program(&program, &names),
            Err(C6PhysError::CapacityExceeded { section: "program.name_bytes", needed: 3, limit: 2 })
        );
    }

    #[test]
    fn type_expr_apply_arity_is_capped_on_encode_and_decode() {
        let mut program = empty_program();
        program.decl_templates = vec![TypeDeclSrc {
            name: "T".into(),
            parameters: Vec::new(),
            ctors: vec![CtorDeclSrc {
                name: "C".into(),
                fields: vec![CtorFieldSrc {
                    name: "f".into(),
                    ty: TypeExprSrc::Apply {
                        name: "F".into(),
                        args: vec![TypeExprSrc::UInt, TypeExprSrc::Bool, TypeExprSrc::List],
                    },
                }],
            }],
        }];
        let lanes = encode_program(&program, &caps()).unwrap();
        let mut narrow = caps();
        narrow.max_arity = 2;
        let refusal = C6PhysError::CapacityExceeded { section: "program.type_expr_arity", needed: 3, limit: 2 };
        assert_eq!(encode_program(&program, &narrow), Err(refusal.clone()));
        assert_eq!(decode_program(&lanes, &narrow), Err(refusal));
    }

    #[test]
    fn program_lanes_from_host_on_captured_admitted_snapshot() {
        // OWNER-REACHABLE: HostSnapshot from the actual `capture` of an admitted
        // generic nominal program with a parameterized Box<A> template.
        let caps = fixtures::caps();
        let (admitted, image) = fixtures::admitted_run(64);
        let host = fixtures::must(capture(&admitted, &image), "capture");
        let view = &host.admission.view;
        assert!(view.decl_templates.iter().any(|d| !d.parameters.is_empty()));
        assert!(view.decl_templates.iter().flat_map(|d| &d.ctors).flat_map(|c| &c.fields)
            .any(|f| matches!(f.ty, TypeExprSrc::Apply { .. })));
        let lanes = program_lanes_from_host(&host, &caps).unwrap();
        assert_eq!(lanes, encode_program(view, &caps).unwrap());
        let decoded = decode_program(&lanes, &caps).unwrap();
        assert_eq!(&decoded, view);
        assert_eq!(encode_program(&decoded, &caps).unwrap(), lanes);
        assert_eq!(&roundtrip_program_owned(view, &caps).unwrap(), view);
        let mut tight = caps;
        tight.max_template_bytes = 0;
        assert!(matches!(
            program_lanes_from_host(&host, &tight),
            Err(C6PhysError::CapacityExceeded { section: "program.template_bytes", .. })
        ));
    }
}
