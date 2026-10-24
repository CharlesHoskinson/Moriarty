//! Unregistered A84 (`zero fuel: publish Exhausted`) full-image instruction fragment;
//! no SDK execution, witness producer, `Machine::step`, or host acceptance flag.
//!
//! Enforces `<k> C:UIControl => .K </k> <stack> S </stack> <fuel> 0 </fuel> <out> uiPending() => uiExhausted() </out>`
//! with `requires notBool uiTerminal(C, S)` (K line 84, wire `rule_tag = 2`, `charge = 0`).
//! Excludes `Control::Error(_)` (`ControlRows[0].tag == 9`) always and `Control::Return(_)`
//! (`ControlRows[0].tag == 1`) when `depth == 0`. Admits `Return` with `depth >= 1` and all
//! other valid `Control` heads (`0, 2..=8`), including recursive `Push` spines (`controlLength >= 2`)
//! whose terminal row `controlLength - 1` may be any non-`Push` constructor (including `Return`
//! at `depth == 0` or `Error(code)` for any `u8`). Full `StepInv` (`O1`..`O4` cross-table
//! references, `prepareA`, `uiValid`, acyclicity, `F2` suffix slices), `F4`, `OBL.G4` byte
//! binding, `projectA`, and 39-way dispatch remain open caller obligations.

use super::literal_host::{tag, unsigned};
use super::*;

pub const FUEL_EXHAUSTION_RULE_ID: &str = "zero fuel: publish Exhausted";
pub const FUEL_EXHAUSTION_RULE_START_LINE: u128 = 84;
pub const FUEL_EXHAUSTION_RULE_TAG: u128 = 2;
pub const FUEL_EXHAUSTION_RULE_CHARGE: u128 = 0;

#[derive(Clone, Debug, PartialEq)]
pub struct FuelExhaustionStepInputs {
    pub enable: Operand,
    /// One complete immutable ProgramBlock shared by both state images.
    pub program: LiteralImage,
    pub pre: LiteralImage,
    /// Independent witness operands; every enabled post cell is constrained.
    pub post: LiteralImage,
    pub envelope: LiteralEnvelope,
}

impl From<LiteralStepInputs> for FuelExhaustionStepInputs {
    fn from(v: LiteralStepInputs) -> Self {
        Self {
            enable: v.enable,
            program: v.program,
            pre: v.pre,
            post: v.post,
            envelope: v.envelope,
        }
    }
}

impl From<FuelExhaustionStepInputs> for LiteralStepInputs {
    fn from(v: FuelExhaustionStepInputs) -> Self {
        Self {
            enable: v.enable,
            program: v.program,
            pre: v.pre,
            post: v.post,
            envelope: v.envelope,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FuelExhaustionStepBlock {
    pub instructions: Vec<Instruction>,
    /// Operand aliases only; this is not a transition/acceptance result.
    pub preserved: FuelExhaustionStepInputs,
}

type Result<T> = std::result::Result<T, LiteralConstructionError>;

pub fn is_reserved_fuel_exhaustion_variable(name: &str, scope: &str) -> bool {
    name.starts_with(&format!("%{scope}.local.temp."))
}

fn check_image(
    image: &LiteralImage,
    root: &[LiteralLane],
    tables: &[LiteralTable],
    block: &'static str,
) -> Result<()> {
    if image.root.len() != root.len() || image.tables.len() != tables.len() {
        return Err(LiteralConstructionError::ImageShape {
            block,
            table: None,
            row: None,
        });
    }
    for (i, table) in tables.iter().enumerate() {
        if image.tables[i].len() != table.capacity {
            return Err(LiteralConstructionError::ImageShape {
                block,
                table: Some(i),
                row: None,
            });
        }
        for (row, cells) in image.tables[i].iter().enumerate() {
            if cells.len() != table.lanes.len() {
                return Err(LiteralConstructionError::ImageShape {
                    block,
                    table: Some(i),
                    row: Some(row),
                });
            }
        }
    }
    Ok(())
}

fn operands(image: &LiteralImage) -> impl Iterator<Item = &Operand> {
    image
        .root
        .iter()
        .chain(image.tables.iter().flatten().flatten())
}

fn field(lanes: &[LiteralLane], row: &[Operand], name: &str) -> Operand {
    row[lanes
        .iter()
        .position(|lane| lane.name == name)
        .expect("fixed L4 column")]
    .clone()
}

fn pfield(lanes: &[LiteralLane], row: &[Operand], prefix: &str, name: &str) -> Operand {
    if prefix.is_empty() {
        field(lanes, row, name)
    } else {
        field(lanes, row, &format!("{prefix}{name}"))
    }
}

struct Emitter {
    scope: String,
    serial: usize,
    code: Vec<Instruction>,
}

impl Emitter {
    fn fresh(&mut self) -> Result<Identifier> {
        let current = self.serial;
        self.serial = self
            .serial
            .checked_add(1)
            .ok_or(LiteralConstructionError::SizeOverflow)?;
        Ok(Identifier(format!("%{}.local.temp.{current}", self.scope)))
    }
    fn select(&mut self, bit: Operand, a: Operand, b: Operand) -> Result<Operand> {
        let output = self.fresh()?;
        self.code.push(Instruction::CondSelect {
            bit,
            a,
            b,
            output: output.clone(),
        });
        Ok(Operand::Variable(output))
    }
    fn select_bool(&mut self, bit: Operand, a: Operand, b: Operand) -> Result<Operand> {
        let v = self.select(bit, a, b)?;
        self.boolean(v.clone());
        Ok(v)
    }
    fn and(&mut self, a: Operand, b: Operand) -> Result<Operand> {
        self.select_bool(a, b, immediate(0))
    }
    fn or(&mut self, a: Operand, b: Operand) -> Result<Operand> {
        self.select_bool(a, immediate(1), b)
    }
    fn and_not(&mut self, a: Operand, b: Operand) -> Result<Operand> {
        self.select_bool(b, immediate(0), a)
    }
    fn eq(&mut self, a: Operand, b: Operand) {
        self.code.push(Instruction::ConstrainEq { a, b });
    }
    fn gated_eq(&mut self, enable: &Operand, a: Operand, b: Operand) -> Result<()> {
        let guarded = self.select(enable.clone(), a, b.clone())?;
        self.eq(guarded, b);
        Ok(())
    }
    fn gated_zero_when_inactive(
        &mut self,
        enable: &Operand,
        active: Operand,
        value: Operand,
    ) -> Result<()> {
        let inactive_val = self.select(active, immediate(0), value)?;
        self.gated_eq(enable, inactive_val, immediate(0))
    }
    fn boolean(&mut self, val: Operand) {
        self.code.push(Instruction::ConstrainToBoolean { val });
    }
    fn bits(&mut self, val: Operand, bits: u32) {
        self.code.push(Instruction::ConstrainBits { val, bits });
    }
    fn less(&mut self, a: Operand, b: Operand) -> Result<Operand> {
        let output = self.fresh()?;
        self.code.push(Instruction::LessThan {
            a,
            b,
            bits: 128,
            output: output.clone(),
        });
        let value = Operand::Variable(output);
        self.boolean(value.clone());
        Ok(value)
    }
    fn test_eq(&mut self, a: Operand, b: Operand) -> Result<Operand> {
        let output = self.fresh()?;
        self.code.push(Instruction::TestEq {
            a,
            b,
            output: output.clone(),
        });
        let value = Operand::Variable(output);
        self.boolean(value.clone());
        Ok(value)
    }
    fn ranges(&mut self, lanes: &[LiteralLane], row: &[Operand]) -> Result<()> {
        for (lane, value) in lanes.iter().zip(row) {
            if lane.boolean {
                self.boolean(value.clone());
            } else {
                self.bits(value.clone(), lane.bits);
            }
            if let Some(maximum) = lane.maximum {
                let exceeds = self.less(immediate(maximum), value.clone())?;
                self.eq(exceeds, immediate(0));
            }
        }
        Ok(())
    }
    fn image_ranges(
        &mut self,
        image: &LiteralImage,
        root: &[LiteralLane],
        tables: &[LiteralTable],
    ) -> Result<()> {
        self.ranges(root, &image.root)?;
        for (table, rows) in tables.iter().zip(&image.tables) {
            for row in rows {
                self.ranges(&table.lanes, row)?;
            }
        }
        Ok(())
    }

    /// Enforces L4 `FrameRow` active/inactive and variant-column zero-padding
    /// on the 17 columns at `prefix` (`""` for `FrameSlots`, `"pendingFrame."` for `Push`).
    fn constrain_frame_row_padding(
        &mut self,
        enable: &Operand,
        lanes: &[LiteralLane],
        row: &[Operand],
        prefix: &str,
        frame_active: Operand,
    ) -> Result<()> {
        let f_tag = pfield(lanes, row, prefix, "tag");
        self.gated_zero_when_inactive(enable, frame_active.clone(), f_tag.clone())?;
        let mut is_f = Vec::with_capacity(9);
        for k in 0..9u128 {
            let eq_k = self.test_eq(f_tag.clone(), immediate(k))?;
            is_f.push(self.and(frame_active.clone(), eq_k)?);
        }
        let name_active = self.or(is_f[0].clone(), is_f[6].clone())?;
        self.gated_zero_when_inactive(enable, name_active, pfield(lanes, row, prefix, "nameId"))?;
        let binop_active = self.or(is_f[1].clone(), is_f[2].clone())?;
        self.gated_zero_when_inactive(
            enable,
            binop_active,
            pfield(lanes, row, prefix, "binaryOp"),
        )?;
        self.gated_zero_when_inactive(
            enable,
            is_f[3].clone(),
            pfield(lanes, row, prefix, "unaryOp"),
        )?;
        self.gated_zero_when_inactive(
            enable,
            is_f[7].clone(),
            pfield(lanes, row, prefix, "typeId"),
        )?;
        let expr0_01 = self.or(is_f[0].clone(), is_f[1].clone())?;
        let expr0_active = self.or(expr0_01, is_f[4].clone())?;
        self.gated_zero_when_inactive(enable, expr0_active, pfield(lanes, row, prefix, "expr0"))?;
        self.gated_zero_when_inactive(
            enable,
            is_f[4].clone(),
            pfield(lanes, row, prefix, "expr1"),
        )?;

        let right_or_unary = self.or(is_f[2].clone(), is_f[3].clone())?;
        let saved_env_active = self.and_not(frame_active, right_or_unary)?;
        let saved_env_present = pfield(lanes, row, prefix, "savedEnv.present");
        self.gated_zero_when_inactive(enable, saved_env_active.clone(), saved_env_present.clone())?;
        let saved_env_idx_active = self.and(saved_env_active, saved_env_present)?;
        self.gated_zero_when_inactive(
            enable,
            saved_env_idx_active,
            pfield(lanes, row, prefix, "savedEnv.index"),
        )?;

        self.gated_zero_when_inactive(
            enable,
            is_f[2].clone(),
            pfield(lanes, row, prefix, "valueRef.space"),
        )?;
        self.gated_zero_when_inactive(
            enable,
            is_f[2].clone(),
            pfield(lanes, row, prefix, "valueRef.index"),
        )?;

        for (slice_name, owner_active) in [
            ("completedSlice", is_f[6].clone()),
            ("remainingExprSlice", is_f[6].clone()),
            ("armSlice", is_f[5].clone()),
        ] {
            let len_op = pfield(lanes, row, prefix, &format!("{slice_name}.len"));
            self.gated_zero_when_inactive(enable, owner_active.clone(), len_op.clone())?;
            let is_empty = self.test_eq(len_op, immediate(0))?;
            let start_active = self.and_not(owner_active, is_empty)?;
            self.gated_zero_when_inactive(
                enable,
                start_active,
                pfield(lanes, row, prefix, &format!("{slice_name}.start")),
            )?;
        }
        Ok(())
    }
}

/// Emits real ZKIR instructions for K source line 84 (`zero fuel: publish Exhausted`,
/// wire `rule_tag = 2`, `charge = 0`).
pub fn emit_fuel_exhaustion_step(
    schema: &LiteralSchema,
    input: &FuelExhaustionStepInputs,
    scope: &str,
) -> Result<FuelExhaustionStepBlock> {
    if &literal_schema(schema.capacities())? != schema {
        return Err(LiteralConstructionError::SchemaMismatch);
    }
    check_image(
        &input.program,
        &schema.program_header,
        &schema.program_tables,
        "program",
    )?;
    check_image(&input.pre, &schema.state_root, &schema.state_tables, "pre")?;
    check_image(
        &input.post,
        &schema.state_root,
        &schema.state_tables,
        "post",
    )?;
    if scope.is_empty() {
        return Err(ConstructionError::EmptyScope.into());
    }
    for operand in [
        &input.enable,
        &input.envelope.rule_tag,
        &input.envelope.charge,
    ]
    .into_iter()
    .chain(operands(&input.program))
    .chain(operands(&input.pre))
    .chain(operands(&input.post))
    {
        if let Operand::Variable(name) = operand {
            if !name.0.starts_with('%') {
                return Err(ConstructionError::UnprefixedVariable.into());
            }
            if is_reserved_fuel_exhaustion_variable(&name.0, scope) {
                return Err(ConstructionError::TemporaryNameCollision.into());
            }
        }
    }

    let mut b = Emitter {
        scope: scope.into(),
        serial: 0,
        code: Vec::new(),
    };
    let enable = &input.enable;
    b.boolean(enable.clone());
    b.image_ranges(
        &input.program,
        &schema.program_header,
        &schema.program_tables,
    )?;
    b.image_ranges(&input.pre, &schema.state_root, &schema.state_tables)?;
    b.image_ranges(&input.post, &schema.state_root, &schema.state_tables)?;
    b.ranges(
        &[tag("rule_tag", 39), unsigned("charge", 8, None)],
        &[
            input.envelope.rule_tag.clone(),
            input.envelope.charge.clone(),
        ],
    )?;

    // Envelope: wire tag 2 (line 84) and administrative charge 0.
    b.gated_eq(
        enable,
        input.envelope.rule_tag.clone(),
        immediate(FUEL_EXHAUSTION_RULE_TAG),
    )?;
    b.gated_eq(
        enable,
        input.envelope.charge.clone(),
        immediate(FUEL_EXHAUSTION_RULE_CHARGE),
    )?;

    // Pre-state Root outcome Pending (0) with zeroed inactive outcome lanes.
    for name in [
        "outcomeTag",
        "outcomeValue.space",
        "outcomeValue.index",
        "outcomeError",
    ] {
        b.gated_eq(
            enable,
            field(&schema.state_root, &input.pre.root, name),
            immediate(0),
        )?;
    }

    // Pre-state remainingFuel must be 0 across all 16 little-endian UInt128 bytes.
    for byte in 0..16 {
        b.gated_eq(
            enable,
            field(
                &schema.state_root,
                &input.pre.root,
                &format!("remainingFuel.{byte}"),
            ),
            immediate(0),
        )?;
    }

    // Pre-state Root environment absent-option padding and O2 stack bound (depth <= stackLimit).
    let pre_env_present = field(&schema.state_root, &input.pre.root, "environment.present");
    let pre_env_index = field(&schema.state_root, &input.pre.root, "environment.index");
    b.gated_zero_when_inactive(enable, pre_env_present, pre_env_index)?;

    let pre_depth = field(&schema.state_root, &input.pre.root, "depth");
    let pre_stack_limit = field(&schema.state_root, &input.pre.root, "stackLimit");
    let depth_exceeds_stack_limit = b.less(pre_stack_limit, pre_depth.clone())?;
    b.gated_eq(enable, depth_exceeds_stack_limit, immediate(0))?;

    // Pre-state Control must be present (O3: controlLength >= 1).
    let pre_control_length = field(&schema.state_root, &input.pre.root, "controlLength");
    let has_control = b.less(immediate(0), pre_control_length.clone())?;
    b.gated_eq(enable, has_control, immediate(1))?;

    // Pre-state FrameSlots (table 4): active rows 0..depth padded per tag, rows depth..S zeroed.
    let frame_lanes = &schema.state_tables[4].lanes;
    for (row_idx, row_values) in input.pre.tables[4].iter().enumerate() {
        let frame_active = b.less(immediate(row_idx as u128), pre_depth.clone())?;
        b.constrain_frame_row_padding(enable, frame_lanes, row_values, "", frame_active)?;
    }

    // Pre-state ControlRows (table 5): terminal head exclusions, Push spine, and variant padding.
    let controls = &schema.state_tables[5].lanes;
    let depth_is_zero = b.test_eq(pre_depth, immediate(0))?;
    for (row_idx, row_values) in input.pre.tables[5].iter().enumerate() {
        let row_active = b.less(immediate(row_idx as u128), pre_control_length.clone())?;
        let next_idx = (row_idx as u128)
            .checked_add(1)
            .ok_or(LiteralConstructionError::SizeOverflow)?;
        let is_push_prefix = b.less(immediate(next_idx), pre_control_length.clone())?;
        let c_tag = field(controls, row_values, "tag");
        b.gated_zero_when_inactive(enable, row_active.clone(), c_tag.clone())?;

        let mut is_c = Vec::with_capacity(10);
        for k in 0..10u128 {
            let eq_k = b.test_eq(c_tag.clone(), immediate(k))?;
            if row_idx == 0 && k == 9 {
                b.gated_eq(enable, eq_k.clone(), immediate(0))?;
            }
            if row_idx == 0 && k == 1 {
                let empty_stack_return = b.and(eq_k.clone(), depth_is_zero.clone())?;
                b.gated_eq(enable, empty_stack_return, immediate(0))?;
            }
            if k == 2 {
                b.gated_eq(enable, eq_k.clone(), is_push_prefix.clone())?;
            }
            is_c.push(b.and(row_active.clone(), eq_k)?);
        }

        let expr_05 = b.or(is_c[0].clone(), is_c[5].clone())?;
        let expr_active = b.or(expr_05, is_c[7].clone())?;
        b.gated_zero_when_inactive(enable, expr_active, field(controls, row_values, "exprId"))?;

        let prim_ok = field(controls, row_values, "primitiveIsOk");
        b.gated_zero_when_inactive(enable, is_c[8].clone(), prim_ok.clone())?;
        let prim_is_ok = b.and(is_c[8].clone(), prim_ok.clone())?;
        let prim_is_err = b.and_not(is_c[8].clone(), prim_ok)?;

        let val_16 = b.or(is_c[1].clone(), is_c[6].clone())?;
        let val_167 = b.or(val_16, is_c[7].clone())?;
        let val_active = b.or(val_167, prim_is_ok)?;
        b.gated_zero_when_inactive(
            enable,
            val_active.clone(),
            field(controls, row_values, "valueRef.space"),
        )?;
        b.gated_zero_when_inactive(
            enable,
            val_active,
            field(controls, row_values, "valueRef.index"),
        )?;

        b.constrain_frame_row_padding(
            enable,
            controls,
            row_values,
            "pendingFrame.",
            is_c[2].clone(),
        )?;

        let env_26 = b.or(is_c[2].clone(), is_c[6].clone())?;
        let env_active = b.or(env_26, is_c[7].clone())?;
        let env_present = field(controls, row_values, "environment.present");
        b.gated_zero_when_inactive(enable, env_active.clone(), env_present.clone())?;
        let env_idx_active = b.and(env_active, env_present)?;
        b.gated_zero_when_inactive(
            enable,
            env_idx_active,
            field(controls, row_values, "environment.index"),
        )?;

        b.gated_zero_when_inactive(
            enable,
            is_c[3].clone(),
            field(controls, row_values, "nameId"),
        )?;

        let val_slice_active = b.or(is_c[3].clone(), is_c[4].clone())?;
        let val_slice_len = field(controls, row_values, "valueSlice.len");
        b.gated_zero_when_inactive(enable, val_slice_active.clone(), val_slice_len.clone())?;
        let val_slice_empty = b.test_eq(val_slice_len, immediate(0))?;
        let val_slice_start_active = b.and_not(val_slice_active, val_slice_empty)?;
        b.gated_zero_when_inactive(
            enable,
            val_slice_start_active,
            field(controls, row_values, "valueSlice.start"),
        )?;

        let fn_present = field(controls, row_values, "functionPresent");
        b.gated_zero_when_inactive(enable, is_c[4].clone(), fn_present.clone())?;
        let fn_id_active = b.and(is_c[4].clone(), fn_present)?;
        b.gated_zero_when_inactive(
            enable,
            fn_id_active,
            field(controls, row_values, "functionId"),
        )?;

        let bind_active = b.or(is_c[5].clone(), is_c[7].clone())?;
        let bind_present = field(controls, row_values, "bindingsPresent");
        b.gated_zero_when_inactive(enable, bind_active.clone(), bind_present.clone())?;
        let bind_root_present_active = b.and(bind_active, bind_present)?;
        let bind_root_present = field(controls, row_values, "bindings.present");
        b.gated_zero_when_inactive(
            enable,
            bind_root_present_active.clone(),
            bind_root_present.clone(),
        )?;
        let bind_root_idx_active = b.and(bind_root_present_active, bind_root_present)?;
        b.gated_zero_when_inactive(
            enable,
            bind_root_idx_active,
            field(controls, row_values, "bindings.index"),
        )?;

        b.gated_zero_when_inactive(
            enable,
            is_c[5].clone(),
            field(controls, row_values, "typeId"),
        )?;

        let arm_slice_active = b.or(is_c[6].clone(), is_c[7].clone())?;
        let arm_slice_len = field(controls, row_values, "armSlice.len");
        b.gated_zero_when_inactive(enable, arm_slice_active.clone(), arm_slice_len.clone())?;
        let arm_slice_empty = b.test_eq(arm_slice_len, immediate(0))?;
        let arm_slice_start_active = b.and_not(arm_slice_active, arm_slice_empty)?;
        b.gated_zero_when_inactive(
            enable,
            arm_slice_start_active,
            field(controls, row_values, "armSlice.start"),
        )?;

        let err_active = b.or(is_c[9].clone(), prim_is_err)?;
        b.gated_zero_when_inactive(enable, err_active, field(controls, row_values, "errorCode"))?;
    }

    // Post-state Root: controlLength = 0, outcome = Exhausted(3) with zeroed inactive fields,
    // and preservation of all other Root lanes (including all 16 LE remainingFuel & initialFuel bytes).
    for (lane, (pre, post)) in schema
        .state_root
        .iter()
        .zip(input.pre.root.iter().zip(&input.post.root))
    {
        let expected = match lane.name.as_str() {
            "controlLength" => immediate(0),
            "outcomeTag" => immediate(3),
            "outcomeValue.space" | "outcomeValue.index" | "outcomeError" => immediate(0),
            _ => pre.clone(),
        };
        b.gated_eq(enable, post.clone(), expected)?;
    }

    // Preserve all five state arenas: Data, DataFields, EnvCells, ValueSlots, FrameSlots.
    for table in 0..5 {
        for (pre, post) in input.pre.tables[table]
            .iter()
            .flatten()
            .zip(input.post.tables[table].iter().flatten())
        {
            b.gated_eq(enable, post.clone(), pre.clone())?;
        }
    }

    // Release zeroing of all post ControlRows[0..C].
    for row in &input.post.tables[5] {
        for post in row {
            b.gated_eq(enable, post.clone(), immediate(0))?;
        }
    }

    Ok(FuelExhaustionStepBlock {
        instructions: b.code,
        preserved: input.clone(),
    })
}
