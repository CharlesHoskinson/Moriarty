//! Symbolic source-only normal tests for `fuel_exhaustion_step`.
//! Inspects emitted ZKIR instructions over normative `LiteralSchema` symbolic
//! operands and normal valid control/spine fixture shapes without SDK execution,
//! native proving, or host evaluator calls. Specimen capacities are finite test
//! fixture shapes only, never runtime admission bounds.

use super::fuel_exhaustion_step::{
    emit_fuel_exhaustion_step, is_reserved_fuel_exhaustion_variable, FuelExhaustionStepInputs,
    FUEL_EXHAUSTION_RULE_CHARGE, FUEL_EXHAUSTION_RULE_ID, FUEL_EXHAUSTION_RULE_START_LINE,
    FUEL_EXHAUSTION_RULE_TAG,
};
use super::*;
use std::collections::BTreeMap;

fn sample_capacities(control_capacity: usize) -> LiteralCapacities {
    LiteralCapacities {
        program: [2, 4, 2, 2, 3, 2, 1, 2, 2, 2, 4, 4, 5, 2, 2],
        state: [2, 2, 2, 2, 2, control_capacity],
        stack_limit: 4,
    }
}

fn symbolic_image(prefix: &str, root: &[LiteralLane], tables: &[LiteralTable]) -> LiteralImage {
    let root_ops = root.iter().map(|l| Operand::Variable(Identifier(format!("%{prefix}.root.{}", l.name)))).collect();
    let table_ops = tables.iter().map(|t| {
        (0..t.capacity).map(|r| {
            t.lanes.iter().map(|l| Operand::Variable(Identifier(format!("%{prefix}.{}.{r}.{}", t.name, l.name)))).collect()
        }).collect()
    }).collect();
    LiteralImage { root: root_ops, tables: table_ops }
}

fn symbolic_inputs(schema: &LiteralSchema, prefix: &str) -> FuelExhaustionStepInputs {
    FuelExhaustionStepInputs {
        enable: Operand::Variable(Identifier(format!("%{prefix}.enable"))),
        program: symbolic_image(&format!("{prefix}.program"), &schema.program_header, &schema.program_tables),
        pre: symbolic_image(&format!("{prefix}.pre"), &schema.state_root, &schema.state_tables),
        post: symbolic_image(&format!("{prefix}.post"), &schema.state_root, &schema.state_tables),
        envelope: LiteralEnvelope {
            rule_tag: Operand::Variable(Identifier(format!("%{prefix}.envelope.rule_tag"))),
            charge: Operand::Variable(Identifier(format!("%{prefix}.envelope.charge"))),
        },
    }
}

fn has_gated_eq(instructions: &[Instruction], enable: &Operand, a: &Operand, b: &Operand) -> bool {
    instructions.windows(2).any(|pair| match (&pair[0], &pair[1]) {
        (
            Instruction::CondSelect { bit, a: sel_a, b: sel_b, output },
            Instruction::ConstrainEq { a: eq_a, b: eq_b },
        ) => bit == enable && sel_a == a && sel_b == b && eq_a == &Operand::Variable(output.clone()) && eq_b == b,
        _ => false,
    })
}

fn root_op(lanes: &[LiteralLane], image: &LiteralImage, name: &str) -> Operand {
    image.root[lanes.iter().position(|l| l.name == name).expect("root lane")].clone()
}

fn table_op(tables: &[LiteralTable], image: &LiteralImage, table_idx: usize, row: usize, name: &str) -> Operand {
    let col = tables[table_idx].lanes.iter().position(|l| l.name == name).expect("table column");
    image.tables[table_idx][row][col].clone()
}

struct NormalWireAssignment {
    vars: BTreeMap<String, u128>,
}

impl NormalWireAssignment {
    fn new(schema: &LiteralSchema, inputs: &FuelExhaustionStepInputs, prefix: &str) -> Self {
        let mut vars = BTreeMap::new();
        for op in [&inputs.enable, &inputs.envelope.rule_tag, &inputs.envelope.charge]
            .into_iter()
            .chain(inputs.program.root.iter())
            .chain(inputs.program.tables.iter().flatten().flatten())
            .chain(inputs.pre.root.iter())
            .chain(inputs.pre.tables.iter().flatten().flatten())
            .chain(inputs.post.root.iter())
            .chain(inputs.post.tables.iter().flatten().flatten())
        {
            if let Operand::Variable(id) = op {
                vars.insert(id.0.clone(), 0);
            }
        }
        vars.insert(format!("%{prefix}.enable"), 1);
        vars.insert(format!("%{prefix}.envelope.rule_tag"), FUEL_EXHAUSTION_RULE_TAG);
        vars.insert(format!("%{prefix}.envelope.charge"), FUEL_EXHAUSTION_RULE_CHARGE);
        vars.insert(format!("%{prefix}.pre.root.stackLimit"), schema.capacities().stack_limit);
        vars.insert(format!("%{prefix}.post.root.stackLimit"), schema.capacities().stack_limit);
        vars.insert(format!("%{prefix}.pre.root.initialFuel.0"), 5);
        vars.insert(format!("%{prefix}.post.root.initialFuel.0"), 5);
        vars.insert(format!("%{prefix}.post.root.outcomeTag"), 3);
        vars.insert(format!("%{prefix}.post.root.controlLength"), 0);
        Self { vars }
    }
    fn set_pre_root(&mut self, prefix: &str, lane: &str, v: u128) {
        self.vars.insert(format!("%{prefix}.pre.root.{lane}"), v);
    }
    fn set_preserved_root(&mut self, prefix: &str, lane: &str, v: u128) {
        self.vars.insert(format!("%{prefix}.pre.root.{lane}"), v);
        self.vars.insert(format!("%{prefix}.post.root.{lane}"), v);
    }
    fn set_pre_control(&mut self, prefix: &str, row: usize, lane: &str, v: u128) {
        self.vars.insert(format!("%{prefix}.pre.ControlRows.{row}.{lane}"), v);
    }
    fn set_preserved_frame(&mut self, prefix: &str, row: usize, lane: &str, v: u128) {
        self.vars.insert(format!("%{prefix}.pre.FrameSlots.{row}.{lane}"), v);
        self.vars.insert(format!("%{prefix}.post.FrameSlots.{row}.{lane}"), v);
    }
    fn resolve(&self, op: &Operand) -> u128 {
        match op {
            Operand::Variable(id) => *self.vars.get(&id.0).unwrap_or_else(|| panic!("unbound {}", id.0)),
            Operand::Immediate(_) => (0..=260u128).find(|&c| op == &immediate(c)).expect("immediate in range"),
        }
    }
    fn assert_satisfies(&mut self, instructions: &[Instruction]) {
        for (idx, inst) in instructions.iter().enumerate() {
            match inst {
                Instruction::ConstrainToBoolean { val } => assert!(self.resolve(val) <= 1, "bool at {idx}"),
                Instruction::ConstrainBits { val, bits } => {
                    let v = self.resolve(val);
                    assert!(*bits >= 128 || (v >> bits) == 0, "bits({bits}) at {idx}: {v}");
                }
                Instruction::LessThan { a, b, bits: 128, output } => {
                    let r = u128::from(self.resolve(a) < self.resolve(b));
                    self.vars.insert(output.0.clone(), r);
                }
                Instruction::TestEq { a, b, output } => {
                    let r = u128::from(self.resolve(a) == self.resolve(b));
                    self.vars.insert(output.0.clone(), r);
                }
                Instruction::CondSelect { bit, a, b, output } => {
                    let cond = self.resolve(bit);
                    assert!(cond <= 1, "select bit at {idx}");
                    let r = if cond == 1 { self.resolve(a) } else { self.resolve(b) };
                    self.vars.insert(output.0.clone(), r);
                }
                Instruction::ConstrainEq { a, b } => {
                    assert_eq!(self.resolve(a), self.resolve(b), "eq at {idx}: {a:?} != {b:?}");
                }
                other => panic!("unexpected instruction at {idx}: {other:?}"),
            }
        }
    }
}

fn populate_valid_frame(assign: &mut NormalWireAssignment, prefix: &str, row: usize, pending: bool, tag: u128) {
    let set = |a: &mut NormalWireAssignment, lane: &str, v: u128| {
        if pending {
            a.set_pre_control(prefix, row, &format!("pendingFrame.{lane}"), v);
        } else {
            a.set_preserved_frame(prefix, row, lane, v);
        }
    };
    set(assign, "tag", tag);
    match tag {
        0 => { set(assign, "nameId", 1); set(assign, "expr0", 2); set(assign, "savedEnv.present", 1); set(assign, "savedEnv.index", 1); }
        1 => { set(assign, "binaryOp", 6); set(assign, "expr0", 1); set(assign, "savedEnv.present", 1); }
        2 => { set(assign, "binaryOp", 2); set(assign, "valueRef.space", 1); set(assign, "valueRef.index", 1); }
        3 => { set(assign, "unaryOp", 1); }
        4 => { set(assign, "expr0", 1); set(assign, "expr1", 2); }
        5 => { set(assign, "armSlice.start", 1); set(assign, "armSlice.len", 1); set(assign, "savedEnv.present", 1); set(assign, "savedEnv.index", 1); }
        6 => {
            set(assign, "nameId", 1); set(assign, "completedSlice.len", 2);
            set(assign, "remainingExprSlice.start", 1); set(assign, "remainingExprSlice.len", 1); set(assign, "savedEnv.present", 1);
        }
        7 => { set(assign, "typeId", 3); set(assign, "savedEnv.present", 1); set(assign, "savedEnv.index", 1); }
        8 => { set(assign, "savedEnv.present", 1); }
        _ => unreachable!(),
    }
}

#[test]
fn fuel_exhaustion_step_emits_exact_guards_and_preservations() {
    assert_eq!(FUEL_EXHAUSTION_RULE_ID, "zero fuel: publish Exhausted");
    assert_eq!(FUEL_EXHAUSTION_RULE_START_LINE, 84);
    assert_eq!(FUEL_EXHAUSTION_RULE_TAG, 2);
    assert_eq!(FUEL_EXHAUSTION_RULE_CHARGE, 0);

    let schema = literal_schema(&sample_capacities(2)).expect("valid schema");
    let inputs = symbolic_inputs(&schema, "s84");
    let block = emit_fuel_exhaustion_step(&schema, &inputs, "exhaust_step").expect("emission");

    assert_eq!(block.preserved, inputs);
    assert_eq!(FuelExhaustionStepInputs::from(LiteralStepInputs::from(inputs.clone())), inputs);
    assert!(block.instructions.iter().all(|inst| matches!(
        inst,
        Instruction::ConstrainToBoolean { .. }
            | Instruction::ConstrainBits { .. }
            | Instruction::LessThan { .. }
            | Instruction::TestEq { .. }
            | Instruction::CondSelect { .. }
            | Instruction::ConstrainEq { .. }
    )));

    let enable = &inputs.enable;
    let (zero, one, two, three, nine) = (immediate(0), immediate(1), immediate(2), immediate(3), immediate(9));

    assert!(has_gated_eq(&block.instructions, enable, &inputs.envelope.rule_tag, &two));
    assert!(has_gated_eq(&block.instructions, enable, &inputs.envelope.charge, &zero));

    for name in ["outcomeTag", "outcomeValue.space", "outcomeValue.index", "outcomeError"] {
        assert!(has_gated_eq(&block.instructions, enable, &root_op(&schema.state_root, &inputs.pre, name), &zero));
    }
    for byte in 0..16 {
        let rem_op = root_op(&schema.state_root, &inputs.pre, &format!("remainingFuel.{byte}"));
        assert!(has_gated_eq(&block.instructions, enable, &rem_op, &zero));
    }

    let pre_ctrl_len = root_op(&schema.state_root, &inputs.pre, "controlLength");
    let has_ctrl_out = block.instructions.iter().find_map(|i| match i {
        Instruction::LessThan { a, b, bits: 128, output } if a == &zero && b == &pre_ctrl_len => Some(Operand::Variable(output.clone())),
        _ => None,
    }).expect("0 < controlLength");
    assert!(has_gated_eq(&block.instructions, enable, &has_ctrl_out, &one));

    let pre_depth = root_op(&schema.state_root, &inputs.pre, "depth");
    let pre_stack_limit = root_op(&schema.state_root, &inputs.pre, "stackLimit");
    let depth_exceeds_out = block.instructions.iter().find_map(|i| match i {
        Instruction::LessThan { a, b, bits: 128, output } if a == &pre_stack_limit && b == &pre_depth => Some(Operand::Variable(output.clone())),
        _ => None,
    }).expect("stackLimit < depth");
    assert!(has_gated_eq(&block.instructions, enable, &depth_exceeds_out, &zero));

    let cr0_tag = table_op(&schema.state_tables, &inputs.pre, 5, 0, "tag");
    let cr0_err_out = block.instructions.iter().find_map(|i| match i {
        Instruction::TestEq { a, b, output } if a == &cr0_tag && b == &nine => Some(Operand::Variable(output.clone())),
        _ => None,
    }).expect("cr0.tag == 9");
    assert!(has_gated_eq(&block.instructions, enable, &cr0_err_out, &zero));

    let cr0_ret_out = block.instructions.iter().find_map(|i| match i {
        Instruction::TestEq { a, b, output } if a == &cr0_tag && b == &one => Some(Operand::Variable(output.clone())),
        _ => None,
    }).expect("cr0.tag == 1");
    let depth_zero_out = block.instructions.iter().find_map(|i| match i {
        Instruction::TestEq { a, b, output } if a == &pre_depth && b == &zero => Some(Operand::Variable(output.clone())),
        _ => None,
    }).expect("depth == 0");
    let empty_stack_ret_out = block.instructions.iter().find_map(|i| match i {
        Instruction::CondSelect { bit, a, b, output } if bit == &cr0_ret_out && a == &depth_zero_out && b == &zero => Some(Operand::Variable(output.clone())),
        _ => None,
    }).expect("cr0_is_return && depth_is_zero");
    assert!(has_gated_eq(&block.instructions, enable, &empty_stack_ret_out, &zero));

    for row_idx in 0..schema.state_tables[5].capacity {
        let row_tag = table_op(&schema.state_tables, &inputs.pre, 5, row_idx, "tag");
        let next_idx = immediate((row_idx + 1) as u128);
        let is_push_prefix = block.instructions.iter().find_map(|i| match i {
            Instruction::LessThan { a, b, bits: 128, output } if a == &next_idx && b == &pre_ctrl_len => Some(Operand::Variable(output.clone())),
            _ => None,
        }).expect("row + 1 < controlLength");
        let tag_is_push = block.instructions.iter().find_map(|i| match i {
            Instruction::TestEq { a, b, output } if a == &row_tag && b == &two => Some(Operand::Variable(output.clone())),
            _ => None,
        }).expect("row.tag == 2");
        assert!(has_gated_eq(&block.instructions, enable, &tag_is_push, &is_push_prefix));
    }

    assert!(has_gated_eq(&block.instructions, enable, &root_op(&schema.state_root, &inputs.post, "controlLength"), &zero));
    assert!(has_gated_eq(&block.instructions, enable, &root_op(&schema.state_root, &inputs.post, "outcomeTag"), &three));
    for name in ["outcomeValue.space", "outcomeValue.index", "outcomeError"] {
        assert!(has_gated_eq(&block.instructions, enable, &root_op(&schema.state_root, &inputs.post, name), &zero));
    }
    for (lane, (pre_op, post_op)) in schema.state_root.iter().zip(inputs.pre.root.iter().zip(&inputs.post.root)) {
        if matches!(lane.name.as_str(), "controlLength" | "outcomeTag" | "outcomeValue.space" | "outcomeValue.index" | "outcomeError") {
            continue;
        }
        assert!(has_gated_eq(&block.instructions, enable, post_op, pre_op));
    }
    for table in 0..5 {
        for (pre, post) in inputs.pre.tables[table].iter().flatten().zip(inputs.post.tables[table].iter().flatten()) {
            assert!(has_gated_eq(&block.instructions, enable, post, pre));
        }
    }
    for cell in inputs.post.tables[5].iter().flatten() {
        assert!(has_gated_eq(&block.instructions, enable, cell, &zero));
    }
}

#[test]
fn normal_eval_and_nonempty_return_satisfy_emitted_symbolic_constraints() {
    let schema = literal_schema(&sample_capacities(2)).expect("valid schema");
    let inputs = symbolic_inputs(&schema, "norm");
    let block = emit_fuel_exhaustion_step(&schema, &inputs, "exhaust_norm").expect("emission");

    let mut eval_assign = NormalWireAssignment::new(&schema, &inputs, "norm");
    eval_assign.set_pre_root("norm", "controlLength", 1);
    eval_assign.set_pre_control("norm", 0, "tag", 0);
    eval_assign.set_pre_control("norm", 0, "exprId", 2);
    eval_assign.assert_satisfies(&block.instructions);

    let mut ret_assign = NormalWireAssignment::new(&schema, &inputs, "norm");
    ret_assign.set_pre_root("norm", "controlLength", 1);
    ret_assign.set_preserved_root("norm", "depth", 2);
    ret_assign.set_preserved_root("norm", "environment.present", 1);
    ret_assign.set_preserved_root("norm", "environment.index", 1);
    ret_assign.set_pre_control("norm", 0, "tag", 1);
    ret_assign.set_pre_control("norm", 0, "valueRef.space", 1);
    ret_assign.set_pre_control("norm", 0, "valueRef.index", 1);
    populate_valid_frame(&mut ret_assign, "norm", 0, false, 0);
    populate_valid_frame(&mut ret_assign, "norm", 1, false, 7);
    ret_assign.assert_satisfies(&block.instructions);
}

#[test]
fn table_driven_valid_head_constructors_and_frame_variants_are_accepted() {
    let schema = literal_schema(&sample_capacities(2)).expect("valid schema");
    let inputs = symbolic_inputs(&schema, "tbl");
    let block = emit_fuel_exhaustion_step(&schema, &inputs, "exhaust_tbl").expect("emission");

    for frame_tag in 0..=8u128 {
        let mut assign = NormalWireAssignment::new(&schema, &inputs, "tbl");
        assign.set_pre_root("tbl", "controlLength", 1);
        assign.set_preserved_root("tbl", "depth", 1);
        assign.set_pre_control("tbl", 0, "tag", 1);
        assign.set_pre_control("tbl", 0, "valueRef.index", 1);
        populate_valid_frame(&mut assign, "tbl", 0, false, frame_tag);
        assign.assert_satisfies(&block.instructions);
    }

    for case in 0..17usize {
        let mut a = NormalWireAssignment::new(&schema, &inputs, "tbl");
        a.set_pre_root("tbl", "controlLength", 1);
        match case {
            0 => { a.set_pre_control("tbl", 0, "tag", 0); a.set_pre_control("tbl", 0, "exprId", 2); }
            1 => { a.set_preserved_root("tbl", "depth", 1); populate_valid_frame(&mut a, "tbl", 0, false, 8); a.set_pre_control("tbl", 0, "tag", 1); a.set_pre_control("tbl", 0, "valueRef.space", 1); a.set_pre_control("tbl", 0, "valueRef.index", 1); }
            2 => { a.set_pre_root("tbl", "controlLength", 2); a.set_pre_control("tbl", 0, "tag", 2); populate_valid_frame(&mut a, "tbl", 0, true, 1); a.set_pre_control("tbl", 0, "environment.present", 1); a.set_pre_control("tbl", 0, "environment.index", 1); a.set_pre_control("tbl", 1, "tag", 0); a.set_pre_control("tbl", 1, "exprId", 1); }
            3 => { a.set_pre_control("tbl", 0, "tag", 3); a.set_pre_control("tbl", 0, "nameId", 1); }
            4 => { a.set_pre_control("tbl", 0, "tag", 3); a.set_pre_control("tbl", 0, "nameId", 1); a.set_pre_control("tbl", 0, "valueSlice.start", 1); a.set_pre_control("tbl", 0, "valueSlice.len", 1); }
            5 => { a.set_pre_control("tbl", 0, "tag", 4); a.set_pre_control("tbl", 0, "valueSlice.len", 2); }
            6 => { a.set_pre_control("tbl", 0, "tag", 4); a.set_pre_control("tbl", 0, "functionPresent", 1); a.set_pre_control("tbl", 0, "valueSlice.start", 1); a.set_pre_control("tbl", 0, "valueSlice.len", 1); }
            7 => { a.set_pre_control("tbl", 0, "tag", 5); a.set_pre_control("tbl", 0, "typeId", 2); a.set_pre_control("tbl", 0, "exprId", 1); }
            8 => { a.set_pre_control("tbl", 0, "tag", 5); a.set_pre_control("tbl", 0, "bindingsPresent", 1); a.set_pre_control("tbl", 0, "typeId", 1); a.set_pre_control("tbl", 0, "exprId", 2); }
            9 => { a.set_pre_control("tbl", 0, "tag", 5); a.set_pre_control("tbl", 0, "bindingsPresent", 1); a.set_pre_control("tbl", 0, "bindings.present", 1); a.set_pre_control("tbl", 0, "bindings.index", 1); a.set_pre_control("tbl", 0, "typeId", 3); a.set_pre_control("tbl", 0, "exprId", 1); }
            10 => { a.set_pre_control("tbl", 0, "tag", 6); a.set_pre_control("tbl", 0, "valueRef.space", 1); }
            11 => { a.set_pre_control("tbl", 0, "tag", 6); a.set_pre_control("tbl", 0, "valueRef.space", 1); a.set_pre_control("tbl", 0, "valueRef.index", 1); a.set_pre_control("tbl", 0, "armSlice.start", 1); a.set_pre_control("tbl", 0, "armSlice.len", 1); a.set_pre_control("tbl", 0, "environment.present", 1); a.set_pre_control("tbl", 0, "environment.index", 1); }
            12 => { a.set_pre_control("tbl", 0, "tag", 7); a.set_pre_control("tbl", 0, "valueRef.space", 1); a.set_pre_control("tbl", 0, "valueRef.index", 1); a.set_pre_control("tbl", 0, "exprId", 2); a.set_pre_control("tbl", 0, "environment.present", 1); }
            13 => { a.set_pre_control("tbl", 0, "tag", 7); a.set_pre_control("tbl", 0, "bindingsPresent", 1); a.set_pre_control("tbl", 0, "valueRef.index", 1); a.set_pre_control("tbl", 0, "exprId", 1); a.set_pre_control("tbl", 0, "armSlice.start", 1); a.set_pre_control("tbl", 0, "armSlice.len", 1); }
            14 => { a.set_pre_control("tbl", 0, "tag", 7); a.set_pre_control("tbl", 0, "bindingsPresent", 1); a.set_pre_control("tbl", 0, "bindings.present", 1); a.set_pre_control("tbl", 0, "bindings.index", 1); a.set_pre_control("tbl", 0, "valueRef.space", 1); a.set_pre_control("tbl", 0, "valueRef.index", 1); a.set_pre_control("tbl", 0, "exprId", 2); a.set_pre_control("tbl", 0, "armSlice.len", 2); a.set_pre_control("tbl", 0, "environment.present", 1); a.set_pre_control("tbl", 0, "environment.index", 1); }
            15 => { a.set_pre_control("tbl", 0, "tag", 8); a.set_pre_control("tbl", 0, "primitiveIsOk", 1); a.set_pre_control("tbl", 0, "valueRef.space", 1); a.set_pre_control("tbl", 0, "valueRef.index", 1); }
            16 => { a.set_pre_control("tbl", 0, "tag", 8); a.set_pre_control("tbl", 0, "errorCode", 255); }
            _ => unreachable!(),
        }
        a.assert_satisfies(&block.instructions);
    }
}

#[test]
fn recursive_push_spines_across_explicit_capacities_and_terminal_tails() {
    for control_capacity in [2usize, 3, 4] {
        let schema = literal_schema(&sample_capacities(control_capacity)).expect("valid schema");
        let inputs = symbolic_inputs(&schema, "spine");
        let block = emit_fuel_exhaustion_step(&schema, &inputs, "exhaust_spine").expect("emission");

        for spine_len in 2..=control_capacity {
            let mut eval_tail = NormalWireAssignment::new(&schema, &inputs, "spine");
            eval_tail.set_pre_root("spine", "controlLength", spine_len as u128);
            eval_tail.set_preserved_root("spine", "depth", 1);
            populate_valid_frame(&mut eval_tail, "spine", 0, false, 4);
            for r in 0..(spine_len - 1) {
                eval_tail.set_pre_control("spine", r, "tag", 2);
                populate_valid_frame(&mut eval_tail, "spine", r, true, (r * 3) as u128 % 9);
                eval_tail.set_pre_control("spine", r, "environment.present", (r % 2) as u128);
                eval_tail.set_pre_control("spine", r, "environment.index", (r % 2) as u128);
            }
            eval_tail.set_pre_control("spine", spine_len - 1, "tag", 0);
            eval_tail.set_pre_control("spine", spine_len - 1, "exprId", 1);
            eval_tail.assert_satisfies(&block.instructions);

            // Push -> Return tail at depth == 0 is non-terminal because head is Push.
            let mut ret_tail = NormalWireAssignment::new(&schema, &inputs, "spine");
            ret_tail.set_pre_root("spine", "controlLength", spine_len as u128);
            for r in 0..(spine_len - 1) {
                ret_tail.set_pre_control("spine", r, "tag", 2);
                populate_valid_frame(&mut ret_tail, "spine", r, true, (r + 5) as u128 % 9);
            }
            ret_tail.set_pre_control("spine", spine_len - 1, "tag", 1);
            ret_tail.set_pre_control("spine", spine_len - 1, "valueRef.index", 1);
            ret_tail.assert_satisfies(&block.instructions);

            // Push -> Error(code) tail for arbitrary u8 is non-terminal because head is Push.
            for code in [0u128, 3, 4, 11, 255] {
                let mut err_tail = NormalWireAssignment::new(&schema, &inputs, "spine");
                err_tail.set_pre_root("spine", "controlLength", spine_len as u128);
                for r in 0..(spine_len - 1) {
                    err_tail.set_pre_control("spine", r, "tag", 2);
                    populate_valid_frame(&mut err_tail, "spine", r, true, (r + 2) as u128 % 9);
                }
                err_tail.set_pre_control("spine", spine_len - 1, "tag", 9);
                err_tail.set_pre_control("spine", spine_len - 1, "errorCode", code);
                err_tail.assert_satisfies(&block.instructions);
            }
        }
    }
}

#[test]
fn fuel_exhaustion_step_composes_after_a102_literal_step_and_supports_zero_arenas() {
    let caps = LiteralCapacities {
        program: [0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 4, 4, 5, 1, 0],
        state: [0, 0, 0, 0, 0, 1],
        stack_limit: 0,
    };
    let schema = literal_schema(&caps).expect("minimal zero-arena schema");
    let program = symbolic_image("trace.program", &schema.program_header, &schema.program_tables);
    let state0 = symbolic_image("trace.state0", &schema.state_root, &schema.state_tables);
    let state1 = symbolic_image("trace.state1", &schema.state_root, &schema.state_tables);
    let state2 = symbolic_image("trace.state2", &schema.state_root, &schema.state_tables);

    let step0_inputs = LiteralStepInputs {
        enable: Operand::Variable(Identifier("%trace.step0.enable".into())),
        program: program.clone(),
        pre: state0,
        post: state1.clone(),
        envelope: LiteralEnvelope {
            rule_tag: Operand::Variable(Identifier("%trace.step0.rule_tag".into())),
            charge: Operand::Variable(Identifier("%trace.step0.charge".into())),
        },
    };
    let step0 = emit_literal_step(&schema, &step0_inputs, "step0_a102").expect("A102 emission");

    let step1_inputs = FuelExhaustionStepInputs {
        enable: Operand::Variable(Identifier("%trace.step1.enable".into())),
        program: step0.preserved.program.clone(),
        pre: step0.preserved.post.clone(),
        post: state2,
        envelope: LiteralEnvelope {
            rule_tag: Operand::Variable(Identifier("%trace.step1.rule_tag".into())),
            charge: Operand::Variable(Identifier("%trace.step1.charge".into())),
        },
    };
    let step1 = emit_fuel_exhaustion_step(&schema, &step1_inputs, "step1_a84").expect("A84 emission");
    assert_eq!(step1.preserved.program, step0.preserved.program);
    assert_eq!(step1.preserved.pre, step0.preserved.post);
}

#[test]
fn fuel_exhaustion_step_rejects_invalid_scope_variables_and_shapes() {
    let schema = literal_schema(&sample_capacities(2)).expect("valid schema");
    let base = symbolic_inputs(&schema, "err");
    assert_eq!(emit_fuel_exhaustion_step(&schema, &base, ""), Err(ConstructionError::EmptyScope.into()));

    let mut unprefixed = base.clone();
    unprefixed.enable = Operand::Variable(Identifier("bad_enable".into()));
    assert_eq!(emit_fuel_exhaustion_step(&schema, &unprefixed, "exhaust_step"), Err(ConstructionError::UnprefixedVariable.into()));

    assert!(is_reserved_fuel_exhaustion_variable("%exhaust_step.local.temp.0", "exhaust_step"));
    let mut colliding = base.clone();
    colliding.envelope.charge = Operand::Variable(Identifier("%exhaust_step.local.temp.0".into()));
    assert_eq!(emit_fuel_exhaustion_step(&schema, &colliding, "exhaust_step"), Err(ConstructionError::TemporaryNameCollision.into()));

    let mut bad_schema = schema.clone();
    bad_schema.state_root.pop();
    assert_eq!(emit_fuel_exhaustion_step(&bad_schema, &base, "exhaust_step"), Err(LiteralConstructionError::SchemaMismatch));

    let mut bad_post = base;
    bad_post.post.tables[5][0].pop();
    assert_eq!(
        emit_fuel_exhaustion_step(&schema, &bad_post, "exhaust_step"),
        Err(LiteralConstructionError::ImageShape { block: "post", table: Some(5), row: Some(0) })
    );
}
