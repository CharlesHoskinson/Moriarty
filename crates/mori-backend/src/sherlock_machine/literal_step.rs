//! Unregistered A102 full-image instruction fragment; no SDK execution.
//! Native logical nonfuel carriers are NOT an alternate L4 serialization.
//! Byte reconstitution, full StepInv/F4, dispatch and binding remain caller work.
use super::*;

pub use super::literal_host::{
    literal_schema, LiteralCapacities, LiteralConstructionError, LiteralLane, LiteralSchema,
    LiteralTable,
};
use super::literal_host::{tag, unsigned};
#[derive(Clone, Debug, PartialEq)]
pub struct LiteralImage {
    pub root: Vec<Operand>,
    pub tables: Vec<Vec<Vec<Operand>>>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct LiteralEnvelope {
    /// Wire enum(39), NOT host RecordEnvelopeA.rule_start_line.
    pub rule_tag: Operand,
    pub charge: Operand,
}
#[derive(Clone, Debug, PartialEq)]
pub struct LiteralStepInputs {
    pub enable: Operand,
    /// One complete immutable ProgramBlock shared by both state images.
    pub program: LiteralImage,
    pub pre: LiteralImage,
    /// Independent witness operands; every enabled post cell is constrained.
    pub post: LiteralImage,
    pub envelope: LiteralEnvelope,
}
pub struct LiteralStepBlock {
    pub instructions: Vec<Instruction>,
    pub selected_expression: Vec<Operand>,
    /// Operand aliases only; this is not a transition/acceptance result.
    pub preserved: LiteralStepInputs,
}
type Result<T> = std::result::Result<T, LiteralConstructionError>;

fn check_image(image: &LiteralImage, root: &[LiteralLane], tables: &[LiteralTable], block: &'static str) -> Result<()> {
    if image.root.len() != root.len() || image.tables.len() != tables.len() {
        return Err(LiteralConstructionError::ImageShape { block, table: None, row: None });
    }
    for (i, table) in tables.iter().enumerate() {
        if image.tables[i].len() != table.capacity {
            return Err(LiteralConstructionError::ImageShape { block, table: Some(i), row: None });
        }
        for (row, cells) in image.tables[i].iter().enumerate() {
            if cells.len() != table.lanes.len() {
                return Err(LiteralConstructionError::ImageShape { block, table: Some(i), row: Some(row) });
            }
        }
    }
    Ok(())
}
fn operands(image: &LiteralImage) -> impl Iterator<Item = &Operand> {
    image.root.iter().chain(image.tables.iter().flatten().flatten())
}
fn field(lanes: &[LiteralLane], row: &[Operand], name: &str) -> Operand {
    // Only constructor-owned schema names reach here, after exact schema check.
    row[lanes.iter().position(|lane| lane.name == name).expect("fixed L4 column")].clone()
}
struct Emitter { scope: String, serial: usize, code: Vec<Instruction> }
impl Emitter {
    fn fresh(&mut self) -> Identifier {
        let id = Identifier(format!("%{}.local.temp.{}", self.scope, self.serial)); self.serial += 1; id
    }
    fn select(&mut self, bit: Operand, a: Operand, b: Operand) -> Operand {
        let output = self.fresh(); self.code.push(Instruction::CondSelect { bit, a, b, output: output.clone() }); Operand::Variable(output)
    }
    fn eq(&mut self, a: Operand, b: Operand) { self.code.push(Instruction::ConstrainEq { a, b }); }
    fn gated_eq(&mut self, enable: &Operand, a: Operand, b: Operand) {
        let guarded = self.select(enable.clone(), a, b.clone()); self.eq(guarded, b);
    }
    fn boolean(&mut self, val: Operand) { self.code.push(Instruction::ConstrainToBoolean { val }); }
    fn bits(&mut self, val: Operand, bits: u32) { self.code.push(Instruction::ConstrainBits { val, bits }); }
    fn less(&mut self, a: Operand, b: Operand) -> Operand {
        let output = self.fresh(); self.code.push(Instruction::LessThan { a, b, bits: 128, output: output.clone() });
        let value = Operand::Variable(output); self.boolean(value.clone()); value
    }
    fn test_zero(&mut self, a: Operand) -> Operand {
        let output = self.fresh(); self.code.push(Instruction::TestEq { a, b: immediate(0), output: output.clone() });
        let value = Operand::Variable(output); self.boolean(value.clone()); value
    }
    fn add(&mut self, a: Operand, b: Operand) -> Operand {
        let output = self.fresh(); self.code.push(Instruction::Add { a, b, output: output.clone() }); Operand::Variable(output)
    }
    fn mul(&mut self, a: Operand, b: Operand) -> Operand {
        let output = self.fresh(); self.code.push(Instruction::Mul { a, b, output: output.clone() }); Operand::Variable(output)
    }
    fn ranges(&mut self, lanes: &[LiteralLane], row: &[Operand]) {
        for (lane, value) in lanes.iter().zip(row) {
            if lane.boolean { self.boolean(value.clone()); } else { self.bits(value.clone(), lane.bits); }
            if let Some(maximum) = lane.maximum {
                let exceeds = self.less(immediate(maximum), value.clone()); self.eq(exceeds, immediate(0));
            }
        }
    }
    fn image_ranges(&mut self, image: &LiteralImage, root: &[LiteralLane], tables: &[LiteralTable]) {
        self.ranges(root, &image.root);
        for (table, rows) in tables.iter().zip(&image.tables) {
            for row in rows { self.ranges(&table.lanes, row); }
        }
    }
}

/// Emits real SDK instructions only. No PublicInput, Output, witness producer,
/// Machine::step, callbacks or host acceptance flags. Caller binds all operands
/// and the exact-one dispatch enable. All cells need unconditional scalar ranges
/// even when this branch is disabled, matching the G3 fetch contract.
pub fn emit_literal_step(schema: &LiteralSchema, input: &LiteralStepInputs, scope: &str) -> Result<LiteralStepBlock> {
    if &literal_schema(&schema.capacities)? != schema { return Err(LiteralConstructionError::SchemaMismatch); }
    check_image(&input.program, &schema.program_header, &schema.program_tables, "program")?;
    check_image(&input.pre, &schema.state_root, &schema.state_tables, "pre")?;
    check_image(&input.post, &schema.state_root, &schema.state_tables, "post")?;
    if scope.is_empty() { return Err(ConstructionError::EmptyScope.into()); }
    let local = format!("%{scope}.local.temp."); let fetch_namespace = format!("%{scope}.expression_fetch.temp.");
    for operand in [&input.enable, &input.envelope.rule_tag, &input.envelope.charge].into_iter()
        .chain(operands(&input.program)).chain(operands(&input.pre)).chain(operands(&input.post)) {
        if let Operand::Variable(name) = operand {
            if !name.0.starts_with('%') { return Err(ConstructionError::UnprefixedVariable.into()); }
            if name.0.starts_with(&local) || name.0.starts_with(&fetch_namespace) {
                return Err(ConstructionError::TemporaryNameCollision.into());
            }
        }
    }
    let mut b = Emitter { scope: scope.into(), serial: 0, code: Vec::new() };
    let enable = &input.enable;
    b.boolean(enable.clone());
    b.image_ranges(&input.program, &schema.program_header, &schema.program_tables);
    b.image_ranges(&input.pre, &schema.state_root, &schema.state_tables);
    b.image_ranges(&input.post, &schema.state_root, &schema.state_tables);
    b.ranges(&[tag("rule_tag", 39), unsigned("charge", 8, None)], &[input.envelope.rule_tag.clone(), input.envelope.charge.clone()]);
    b.gated_eq(enable, input.envelope.rule_tag.clone(), immediate(3));
    b.gated_eq(enable, input.envelope.charge.clone(), immediate(1));
    for name in ["outcomeTag", "outcomeValue.space", "outcomeValue.index", "outcomeError"] {
        b.gated_eq(enable, field(&schema.state_root, &input.pre.root, name), immediate(0));
    }
    b.gated_eq(enable, field(&schema.state_root, &input.pre.root, "controlLength"), immediate(1));
    let controls = &schema.state_tables[5].lanes;
    for (row, values) in input.pre.tables[5].iter().enumerate() {
        for (lane, value) in controls.iter().zip(values) {
            if row == 0 && lane.name == "exprId" { continue; }
            b.gated_eq(enable, value.clone(), immediate(0));
        }
    }
    let expression = &schema.program_tables[4];
    let fetch_shape = TableShape::new(expression.capacity, expression.lanes.iter().map(|lane|
        Column { name: lane.name.clone(), kind: if lane.boolean { Type::Bool } else { Type::UInt } }).collect())?;
    let expr_id = field(controls, &input.pre.tables[5][0], "exprId");
    let fetch = emit_enabled_fetch_block(&fetch_shape, &EnabledFetchInputs {
        enable: enable.clone(), active_rows: input.program.root[5].clone(), index: expr_id.clone(),
        rows: input.program.tables[4].clone(),
    }, &format!("{scope}.expression_fetch"))?;
    b.code.extend(fetch.instructions);
    let selected: Vec<_> = fetch.selected.into_iter().map(|cell| cell.value).collect();
    b.ranges(&expression.lanes, &selected);
    for name in ["tag", "valueRef.space", "nameId", "binaryOp", "unaryOp", "childSlice.start", "childSlice.len", "armSlice.start", "armSlice.len"] {
        b.gated_eq(enable, field(&expression.lanes, &selected, name), immediate(0));
    }
    b.gated_eq(enable, field(&expression.lanes, &selected, "occurrenceId"), expr_id);
    let constant_index = field(&expression.lanes, &selected, "valueRef.index");
    let in_constants = b.less(constant_index, input.program.root[14].clone());
    b.gated_eq(enable, in_constants, immediate(1));

    // Complete Root preservation, except precisely the two A102 write fields.
    for (lane, (pre, post)) in schema.state_root.iter().zip(input.pre.root.iter().zip(&input.post.root)) {
        if lane.name.starts_with("remainingFuel.") { continue; }
        if lane.name == "controlLength" { b.gated_eq(enable, post.clone(), immediate(1)); }
        else { b.gated_eq(enable, post.clone(), pre.clone()); }
    }
    for table in 0..5 {
        for (pre, post) in input.pre.tables[table].iter().flatten().zip(input.post.tables[table].iter().flatten()) {
            b.gated_eq(enable, post.clone(), pre.clone());
        }
    }
    for (row, values) in input.post.tables[5].iter().enumerate() {
        for (lane, post) in controls.iter().zip(values) {
            let expected = if row == 0 && lane.name == "tag" { immediate(1) }
                else if row == 0 && (lane.name == "valueRef.space" || lane.name == "valueRef.index") {
                    field(&expression.lanes, &selected, &lane.name)
                } else { immediate(0) };
            b.gated_eq(enable, post.clone(), expected);
        }
    }

    // Guarded bytewise decrement: no packed UInt128/native-fuel shortcut.
    let mut zero_bytes = Vec::with_capacity(16); let mut all_zero = immediate(1);
    for byte in 0..16 {
        let pre = field(&schema.state_root, &input.pre.root, &format!("remainingFuel.{byte}"));
        let zero = b.test_zero(pre);
        all_zero = b.select(all_zero, zero.clone(), immediate(0)); b.boolean(all_zero.clone());
        zero_bytes.push(zero);
    }
    b.gated_eq(enable, all_zero, immediate(0));
    let mut borrow = enable.clone();
    for (byte, zero) in zero_bytes.into_iter().enumerate() {
        let pre = field(&schema.state_root, &input.pre.root, &format!("remainingFuel.{byte}"));
        let post = field(&schema.state_root, &input.post.root, &format!("remainingFuel.{byte}"));
        let next = b.select(borrow.clone(), zero, immediate(0)); b.boolean(next.clone());
        let carry = b.mul(immediate(256), next.clone()); b.bits(carry.clone(), 9);
        let left = b.add(pre, carry); b.bits(left.clone(), 9);
        let right = b.add(post, borrow); b.bits(right.clone(), 9);
        b.gated_eq(enable, left, right); borrow = next;
    }
    b.eq(borrow, immediate(0));
    Ok(LiteralStepBlock { instructions: b.code, selected_expression: selected, preserved: input.clone() })
}
