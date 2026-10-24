//! Composable in-circuit recursive `uiValid` fragment for A102 Literal and
//! immutable `Constants` / `ConstantFields` / fixed-A type tables.
//! Source-only instruction emitter: no SDK execution, witness packing, host
//! validity shortcut, resource admission, or statement binding.

use super::*;

type Result<T> = std::result::Result<T, LiteralConstructionError>;

const FIXED_ACTIVE_TYPES: u128 = 4;
const FIXED_ACTIVE_CONSTRUCTORS: u128 = 4;
const FIXED_ACTIVE_CONSTRUCTOR_FIELD_TYPES: u128 = 5;

const FIXED_TYPES_ROWS: [[u128; 3]; 4] = [[0, 0, 0], [1, 0, 0], [2, 0, 2], [3, 2, 2]];

const FIXED_CONSTRUCTORS_ROWS: [[u128; 4]; 4] =
    [[2, 0, 0, 0], [2, 1, 0, 2], [3, 0, 2, 1], [3, 1, 3, 2]];

const FIXED_CONSTRUCTOR_FIELD_TYPES_ROWS: [u128; 5] = [0, 2, 0, 3, 3];

/// Caller-bound Native operands for recursive constant and literal validity.
/// All cells of `program` must be supplied and range-valid in every branch.
#[derive(Clone, Debug, PartialEq)]
pub struct LiteralUiValidInputs {
    pub enable: Operand,
    pub program: LiteralImage,
    pub constant_index: Operand,
}

/// Emitted instruction fragment and selected root constant row operands.
#[derive(Clone, Debug, PartialEq)]
pub struct LiteralUiValidBlock {
    pub instructions: Vec<Instruction>,
    pub selected_constant: Vec<Operand>,
    pub preserved: LiteralUiValidInputs,
}

/// Recover and verify `LiteralCapacities` from the public fields of `LiteralSchema`
/// without accessing the sibling-private `LiteralSchema.capacities` field.
pub fn schema_capacities(schema: &LiteralSchema) -> Result<LiteralCapacities> {
    let program: [usize; 15] = schema
        .program_tables
        .iter()
        .map(|table| table.capacity)
        .collect::<Vec<_>>()
        .try_into()
        .map_err(|_| LiteralConstructionError::SchemaMismatch)?;
    let state: [usize; 6] = schema
        .state_tables
        .iter()
        .map(|table| table.capacity)
        .collect::<Vec<_>>()
        .try_into()
        .map_err(|_| LiteralConstructionError::SchemaMismatch)?;
    let stack_limit = schema
        .state_root
        .iter()
        .find(|lane| lane.name == "stackLimit")
        .and_then(|lane| lane.maximum)
        .ok_or(LiteralConstructionError::SchemaMismatch)?;
    let capacities = LiteralCapacities {
        program,
        state,
        stack_limit,
    };
    if &literal_schema(&capacities)? != schema {
        return Err(LiteralConstructionError::SchemaMismatch);
    }
    Ok(capacities)
}

/// Check whether `name` collides with any temporary namespace reserved by
/// `emit_literal_ui_valid` / `emit_literal_ui_valid_with_capacities` under `scope`.
pub fn is_reserved_ui_valid_variable(name: &str, scope: &str, k: usize, kf: usize) -> bool {
    let direct = [
        format!("%{scope}.temp."),
        format!("%{scope}.local.temp."),
        format!("%{scope}.root_constant_fetch.temp."),
    ];
    if direct.iter().any(|prefix| name.starts_with(prefix)) {
        return true;
    }
    for i in 0..k {
        for sub in ["type_fetch", "ctor_fetch"] {
            let prefix = format!("%{scope}.const_{i}.{sub}.temp.");
            if name.starts_with(&prefix) {
                return true;
            }
        }
        if kf > 0 {
            for sub in [
                "field0_cft_fetch",
                "field0_ref_fetch",
                "field0_node_fetch",
                "field1_cft_fetch",
                "field1_ref_fetch",
                "field1_node_fetch",
            ] {
                let prefix = format!("%{scope}.const_{i}.{sub}.temp.");
                if name.starts_with(&prefix) {
                    return true;
                }
            }
        }
    }
    false
}

fn check_program_image(
    image: &LiteralImage,
    header: &[LiteralLane],
    tables: &[LiteralTable],
) -> Result<()> {
    if image.root.len() != header.len() || image.tables.len() != tables.len() {
        return Err(LiteralConstructionError::ImageShape {
            block: "program",
            table: None,
            row: None,
        });
    }
    for (i, table) in tables.iter().enumerate() {
        if image.tables[i].len() != table.capacity {
            return Err(LiteralConstructionError::ImageShape {
                block: "program",
                table: Some(i),
                row: None,
            });
        }
        for (row, cells) in image.tables[i].iter().enumerate() {
            if cells.len() != table.lanes.len() {
                return Err(LiteralConstructionError::ImageShape {
                    block: "program",
                    table: Some(i),
                    row: Some(row),
                });
            }
        }
    }
    Ok(())
}

fn program_operands(image: &LiteralImage) -> impl Iterator<Item = &Operand> {
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

fn table_shape(table: &LiteralTable) -> Result<TableShape> {
    Ok(TableShape::new(
        table.capacity,
        table
            .lanes
            .iter()
            .map(|lane| Column {
                name: lane.name.clone(),
                kind: if lane.boolean { Type::Bool } else { Type::UInt },
            })
            .collect(),
    )?)
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

    fn eq(&mut self, a: Operand, b: Operand) {
        self.code.push(Instruction::ConstrainEq { a, b });
    }

    fn gated_eq(&mut self, enable: &Operand, a: Operand, b: Operand) -> Result<()> {
        let guarded = self.select(enable.clone(), a, b.clone())?;
        self.eq(guarded, b);
        Ok(())
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

    fn add(&mut self, a: Operand, b: Operand) -> Result<Operand> {
        let output = self.fresh()?;
        self.code.push(Instruction::Add {
            a,
            b,
            output: output.clone(),
        });
        Ok(Operand::Variable(output))
    }

    fn and(&mut self, bit: Operand, a: Operand) -> Result<Operand> {
        let value = self.select(bit, a, immediate(0))?;
        self.boolean(value.clone());
        Ok(value)
    }

    fn not_bool(&mut self, bit: Operand) -> Result<Operand> {
        let value = self.select(bit, immediate(0), immediate(1))?;
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

    fn fetch_row(
        &mut self,
        shape: &TableShape,
        table: &LiteralTable,
        enable: Operand,
        active_rows: Operand,
        index: Operand,
        rows: &[Vec<Operand>],
        sub_scope: &str,
    ) -> Result<Vec<Operand>> {
        let block = emit_enabled_fetch_block(
            shape,
            &EnabledFetchInputs {
                enable,
                active_rows,
                index,
                rows: rows.to_vec(),
            },
            sub_scope,
        )?;
        self.code.extend(block.instructions);
        let selected: Vec<_> = block.selected.into_iter().map(|cell| cell.value).collect();
        self.ranges(&table.lanes, &selected)?;
        Ok(selected)
    }
}

/// Emit recursive in-circuit `uiValid` constraints for `Constants`, `ConstantFields`,
/// and fixed-A `Types`/`Constructors`/`ConstructorFieldTypes`, plus enabled root
/// constant selection at `input.constant_index`, using explicit `LiteralCapacities`.
pub fn emit_literal_ui_valid_with_capacities(
    capacities: &LiteralCapacities,
    schema: &LiteralSchema,
    input: &LiteralUiValidInputs,
    scope: &str,
) -> Result<LiteralUiValidBlock> {
    if &literal_schema(capacities)? != schema {
        return Err(LiteralConstructionError::SchemaMismatch);
    }
    check_program_image(
        &input.program,
        &schema.program_header,
        &schema.program_tables,
    )?;
    if scope.is_empty() {
        return Err(ConstructionError::EmptyScope.into());
    }
    let t_cap = capacities.program[10];
    let q_cap = capacities.program[11];
    let qf_cap = capacities.program[12];
    let k_cap = capacities.program[13];
    let kf_cap = capacities.program[14];
    u128::try_from(k_cap).map_err(|_| LiteralConstructionError::SizeOverflow)?;
    u128::try_from(kf_cap).map_err(|_| LiteralConstructionError::SizeOverflow)?;

    for operand in [&input.enable, &input.constant_index]
        .into_iter()
        .chain(program_operands(&input.program))
    {
        if let Operand::Variable(name) = operand {
            if !name.0.starts_with('%') {
                return Err(ConstructionError::UnprefixedVariable.into());
            }
            if is_reserved_ui_valid_variable(&name.0, scope, k_cap, kf_cap) {
                return Err(ConstructionError::TemporaryNameCollision.into());
            }
        }
    }

    let types_table = &schema.program_tables[10];
    let ctors_table = &schema.program_tables[11];
    let cft_table = &schema.program_tables[12];
    let constants_table = &schema.program_tables[13];
    let cf_table = &schema.program_tables[14];

    let types_shape = table_shape(types_table)?;
    let ctors_shape = table_shape(ctors_table)?;
    let cft_shape = table_shape(cft_table)?;
    let constants_shape = table_shape(constants_table)?;
    let cf_shape = if kf_cap > 0 {
        Some(table_shape(cf_table)?)
    } else {
        None
    };

    let mut b = Emitter {
        scope: scope.into(),
        serial: 0,
        code: Vec::new(),
    };
    let enable = &input.enable;
    b.boolean(enable.clone());
    b.bits(input.constant_index.clone(), 128);
    b.image_ranges(
        &input.program,
        &schema.program_header,
        &schema.program_tables,
    )?;

    let active_types = field(&schema.program_header, &input.program.root, "activeTypes");
    let active_ctors = field(
        &schema.program_header,
        &input.program.root,
        "activeConstructors",
    );
    let active_cft = field(
        &schema.program_header,
        &input.program.root,
        "activeConstructorFieldTypes",
    );
    let active_constants = field(
        &schema.program_header,
        &input.program.root,
        "activeConstants",
    );
    let active_cf = field(
        &schema.program_header,
        &input.program.root,
        "activeConstantFields",
    );

    // 1. Fixed-A builtin header active counts and table entries (Types, Constructors, ConstructorFieldTypes).
    b.gated_eq(enable, active_types.clone(), immediate(FIXED_ACTIVE_TYPES))?;
    b.gated_eq(
        enable,
        active_ctors.clone(),
        immediate(FIXED_ACTIVE_CONSTRUCTORS),
    )?;
    b.gated_eq(
        enable,
        active_cft.clone(),
        immediate(FIXED_ACTIVE_CONSTRUCTOR_FIELD_TYPES),
    )?;

    for row in 0..t_cap {
        let expected = if row < 4 {
            FIXED_TYPES_ROWS[row]
        } else {
            [0, 0, 0]
        };
        for (col, val) in input.program.tables[10][row].iter().enumerate() {
            b.gated_eq(enable, val.clone(), immediate(expected[col]))?;
        }
    }
    for row in 0..q_cap {
        let expected = if row < 4 {
            FIXED_CONSTRUCTORS_ROWS[row]
        } else {
            [0, 0, 0, 0]
        };
        for (col, val) in input.program.tables[11][row].iter().enumerate() {
            b.gated_eq(enable, val.clone(), immediate(expected[col]))?;
        }
    }
    for row in 0..qf_cap {
        let expected = if row < 5 {
            FIXED_CONSTRUCTOR_FIELD_TYPES_ROWS[row]
        } else {
            0
        };
        b.gated_eq(
            enable,
            input.program.tables[12][row][0].clone(),
            immediate(expected),
        )?;
    }

    // 2. ConstantFields table validity (including legal zero-capacity KF == 0).
    if kf_cap == 0 {
        b.gated_eq(enable, active_cf.clone(), immediate(0))?;
    } else {
        for j in 0..kf_cap {
            let row_j = &input.program.tables[14][j];
            let cf_space = field(&cf_table.lanes, row_j, "value.space");
            let cf_index = field(&cf_table.lanes, row_j, "value.index");
            let in_active_cf = b.less(immediate(j as u128), active_cf.clone())?;
            let en_cf_active = b.and(enable.clone(), in_active_cf.clone())?;
            let not_in_active_cf = b.not_bool(in_active_cf)?;
            let en_cf_inactive = b.and(enable.clone(), not_in_active_cf)?;

            b.gated_eq(enable, cf_space, immediate(0))?;
            let idx_in_constants = b.less(cf_index.clone(), active_constants.clone())?;
            b.gated_eq(&en_cf_active, idx_in_constants, immediate(1))?;
            b.gated_eq(&en_cf_inactive, cf_index, immediate(0))?;
        }
    }

    // 3. Per-row Constants table typing, unused-column zeroing, padding, and strict DAG validity.
    for i in 0..k_cap {
        let row_i = &input.program.tables[13][i];
        let in_active_k = b.less(immediate(i as u128), active_constants.clone())?;
        let en_row = b.and(enable.clone(), in_active_k.clone())?;
        let not_in_active_k = b.not_bool(in_active_k)?;
        let en_inactive = b.and(enable.clone(), not_in_active_k)?;

        for cell in row_i {
            b.gated_eq(&en_inactive, cell.clone(), immediate(0))?;
        }

        let kind = field(&constants_table.lanes, row_i, "kind");
        let type_id = field(&constants_table.lanes, row_i, "typeId");
        let ctor_id = field(&constants_table.lanes, row_i, "ctorId");
        let bool_val = field(&constants_table.lanes, row_i, "boolValue");
        let field_start = field(&constants_table.lanes, row_i, "fieldStart");
        let field_len = field(&constants_table.lanes, row_i, "fieldLength");
        let uint_bytes: Vec<_> = (0..16)
            .map(|byte| field(&constants_table.lanes, row_i, &format!("uintBytes.{byte}")))
            .collect();

        let is_uint = b.test_eq(kind.clone(), immediate(0))?;
        let is_bool = b.test_eq(kind.clone(), immediate(1))?;
        let is_ctor = b.test_eq(kind.clone(), immediate(2))?;
        let kind_01 = b.add(is_uint.clone(), is_bool.clone())?;
        b.bits(kind_01.clone(), 2);
        let kind_sum = b.add(kind_01, is_ctor.clone())?;
        b.bits(kind_sum.clone(), 2);
        b.gated_eq(&en_row, kind_sum, immediate(1))?;

        let en_uint = b.and(en_row.clone(), is_uint)?;
        let en_bool = b.and(en_row.clone(), is_bool)?;
        let en_ctor = b.and(en_row.clone(), is_ctor)?;

        let selected_type = b.fetch_row(
            &types_shape,
            types_table,
            en_row.clone(),
            active_types.clone(),
            type_id.clone(),
            &input.program.tables[10],
            &format!("{scope}.const_{i}.type_fetch"),
        )?;
        let fetched_type_kind = field(&types_table.lanes, &selected_type, "kind");
        let fetched_type_ctor_len = field(&types_table.lanes, &selected_type, "ctorLength");
        b.gated_eq(&en_row, fetched_type_kind, type_id.clone())?;

        // Kind 0: UInt128 (all 16 uintBytes constrained to 8-bit lanes).
        b.gated_eq(&en_uint, type_id.clone(), immediate(0))?;
        b.gated_eq(&en_uint, ctor_id.clone(), immediate(0))?;
        b.gated_eq(&en_uint, bool_val.clone(), immediate(0))?;
        b.gated_eq(&en_uint, field_start.clone(), immediate(0))?;
        b.gated_eq(&en_uint, field_len.clone(), immediate(0))?;
        b.gated_eq(&en_uint, fetched_type_ctor_len.clone(), immediate(0))?;

        // Kind 1: Bool.
        b.gated_eq(&en_bool, type_id.clone(), immediate(1))?;
        b.gated_eq(&en_bool, ctor_id.clone(), immediate(0))?;
        b.gated_eq(&en_bool, field_start.clone(), immediate(0))?;
        b.gated_eq(&en_bool, field_len.clone(), immediate(0))?;
        b.gated_eq(&en_bool, fetched_type_ctor_len.clone(), immediate(0))?;
        for byte_op in &uint_bytes {
            b.gated_eq(&en_bool, byte_op.clone(), immediate(0))?;
        }

        // Kind 2: Constructor (Nil=0, Cons=1, Leaf=2, Branch=3).
        for byte_op in &uint_bytes {
            b.gated_eq(&en_ctor, byte_op.clone(), immediate(0))?;
        }
        b.gated_eq(&en_ctor, bool_val, immediate(0))?;

        let selected_ctor = b.fetch_row(
            &ctors_shape,
            ctors_table,
            en_ctor.clone(),
            active_ctors.clone(),
            ctor_id.clone(),
            &input.program.tables[11],
            &format!("{scope}.const_{i}.ctor_fetch"),
        )?;
        let ctor_owning_type = field(&ctors_table.lanes, &selected_ctor, "owningType");
        let ctor_source_order = field(&ctors_table.lanes, &selected_ctor, "sourceOrder");
        let ctor_ft_start = field(&ctors_table.lanes, &selected_ctor, "fieldTypeSlice.start");
        let ctor_ft_len = field(&ctors_table.lanes, &selected_ctor, "fieldTypeSlice.len");

        b.gated_eq(&en_ctor, type_id, ctor_owning_type)?;
        b.gated_eq(&en_ctor, field_len.clone(), ctor_ft_len)?;
        let order_ok = b.less(ctor_source_order, fetched_type_ctor_len)?;
        b.gated_eq(&en_ctor, order_ok, immediate(1))?;

        let is_nil = b.test_eq(ctor_id.clone(), immediate(0))?;
        let is_cons = b.test_eq(ctor_id.clone(), immediate(1))?;
        let is_leaf = b.test_eq(ctor_id.clone(), immediate(2))?;
        let is_branch = b.test_eq(ctor_id, immediate(3))?;
        let ctor_01 = b.add(is_nil.clone(), is_cons.clone())?;
        b.bits(ctor_01.clone(), 2);
        let ctor_23 = b.add(is_leaf.clone(), is_branch.clone())?;
        b.bits(ctor_23.clone(), 2);
        let ctor_sum = b.add(ctor_01, ctor_23)?;
        b.bits(ctor_sum.clone(), 3);
        b.gated_eq(&en_ctor, ctor_sum, immediate(1))?;

        let en_nil = b.and(en_ctor.clone(), is_nil)?;
        b.gated_eq(&en_nil, field_start.clone(), immediate(0))?;
        b.gated_eq(&en_nil, field_len.clone(), immediate(0))?;

        let field_end = b.add(field_start.clone(), field_len)?;
        b.bits(field_end.clone(), 128);
        let slice_exceeds = b.less(active_cf.clone(), field_end)?;
        b.gated_eq(&en_ctor, slice_exceeds, immediate(0))?;

        let has_f1 = b.add(is_cons, is_branch.clone())?;
        b.boolean(has_f1.clone());
        let has_f0 = b.add(has_f1.clone(), is_leaf)?;
        b.boolean(has_f0.clone());
        let en_f0 = b.and(en_ctor.clone(), has_f0)?;
        let en_f1 = b.and(en_ctor, has_f1)?;

        if let Some(ref cf_shape) = cf_shape {
            let selected_cft0 = b.fetch_row(
                &cft_shape,
                cft_table,
                en_f0.clone(),
                active_cft.clone(),
                ctor_ft_start.clone(),
                &input.program.tables[12],
                &format!("{scope}.const_{i}.field0_cft_fetch"),
            )?;
            let decl_f0_type = field(&cft_table.lanes, &selected_cft0, "typeId");

            let selected_cf0 = b.fetch_row(
                cf_shape,
                cf_table,
                en_f0.clone(),
                active_cf.clone(),
                field_start.clone(),
                &input.program.tables[14],
                &format!("{scope}.const_{i}.field0_ref_fetch"),
            )?;
            let f0_space = field(&cf_table.lanes, &selected_cf0, "value.space");
            let f0_index = field(&cf_table.lanes, &selected_cf0, "value.index");
            b.gated_eq(&en_f0, f0_space, immediate(0))?;
            let f0_earlier = b.less(f0_index.clone(), immediate(i as u128))?;
            b.gated_eq(&en_f0, f0_earlier, immediate(1))?;

            let selected_child0 = b.fetch_row(
                &constants_shape,
                constants_table,
                en_f0.clone(),
                active_constants.clone(),
                f0_index,
                &input.program.tables[13],
                &format!("{scope}.const_{i}.field0_node_fetch"),
            )?;
            let child0_type = field(&constants_table.lanes, &selected_child0, "typeId");
            b.gated_eq(&en_f0, child0_type.clone(), decl_f0_type)?;
            let exp_f0_type = b.select(is_branch.clone(), immediate(3), immediate(0))?;
            b.gated_eq(&en_f0, child0_type, exp_f0_type)?;

            let cft1_idx = b.add(ctor_ft_start, immediate(1))?;
            b.bits(cft1_idx.clone(), 128);
            let selected_cft1 = b.fetch_row(
                &cft_shape,
                cft_table,
                en_f1.clone(),
                active_cft.clone(),
                cft1_idx,
                &input.program.tables[12],
                &format!("{scope}.const_{i}.field1_cft_fetch"),
            )?;
            let decl_f1_type = field(&cft_table.lanes, &selected_cft1, "typeId");

            let cf1_idx = b.add(field_start, immediate(1))?;
            b.bits(cf1_idx.clone(), 128);
            let selected_cf1 = b.fetch_row(
                cf_shape,
                cf_table,
                en_f1.clone(),
                active_cf.clone(),
                cf1_idx,
                &input.program.tables[14],
                &format!("{scope}.const_{i}.field1_ref_fetch"),
            )?;
            let f1_space = field(&cf_table.lanes, &selected_cf1, "value.space");
            let f1_index = field(&cf_table.lanes, &selected_cf1, "value.index");
            b.gated_eq(&en_f1, f1_space, immediate(0))?;
            let f1_earlier = b.less(f1_index.clone(), immediate(i as u128))?;
            b.gated_eq(&en_f1, f1_earlier, immediate(1))?;

            let selected_child1 = b.fetch_row(
                &constants_shape,
                constants_table,
                en_f1.clone(),
                active_constants.clone(),
                f1_index,
                &input.program.tables[13],
                &format!("{scope}.const_{i}.field1_node_fetch"),
            )?;
            let child1_type = field(&constants_table.lanes, &selected_child1, "typeId");
            b.gated_eq(&en_f1, child1_type.clone(), decl_f1_type)?;
            let exp_f1_type = b.select(is_branch, immediate(3), immediate(2))?;
            b.gated_eq(&en_f1, child1_type, exp_f1_type)?;
        } else {
            b.gated_eq(&en_f0, immediate(1), immediate(0))?;
            b.gated_eq(&en_f1, immediate(1), immediate(0))?;
        }
    }

    // 4. Enabled root Constant fetch at `input.constant_index`.
    let selected_constant = b.fetch_row(
        &constants_shape,
        constants_table,
        enable.clone(),
        active_constants,
        input.constant_index.clone(),
        &input.program.tables[13],
        &format!("{scope}.root_constant_fetch"),
    )?;

    Ok(LiteralUiValidBlock {
        instructions: b.code,
        selected_constant,
        preserved: input.clone(),
    })
}

/// Emit recursive in-circuit `uiValid` constraints using `LiteralSchema` directly.
pub fn emit_literal_ui_valid(
    schema: &LiteralSchema,
    input: &LiteralUiValidInputs,
    scope: &str,
) -> Result<LiteralUiValidBlock> {
    let capacities = schema_capacities(schema)?;
    emit_literal_ui_valid_with_capacities(&capacities, schema, input, scope)
}
