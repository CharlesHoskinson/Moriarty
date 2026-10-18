//! Handwritten B-core `midnight_zkir_v3` instruction emitter for
//! `b.helper.Scalar.arithmetic` on branch `site = RetRight`, `mode = Add`,
//! `phase = Second -> Done` (`/helper_events/2` in `rows-r4.json`).
//!
//! Source only: emits pinned SDK instructions over caller-supplied operands;
//! performs no SDK load, check, evaluation, compilation, or proof.
//!
//! Local carriers are logical lanes (16 little-endian byte lanes for `UInt128`
//! payloads and counters, schema enum ordinals for discriminators, and explicit
//! union payload lanes for `ScalarStaged`), not a frozen physical B width table
//! or serialized profile. `AB`, `P0`..`P3`, `PS`, exactly-one dispatch, full
//! `StepInv` (`SI-01`..`SI-23`, including `SI-22 S2` on `pre.staged`),
//! untouched-image complement preservation (`C`), and statement/field binding
//! (`L04`) remain explicit caller obligations.

use super::*;

pub const HELPER_PRESENT: u128 = 1; // notation Opt<A>.present
pub const OPT_ABSENT: u128 = 0; // notation Opt<A>.present = 0
pub const OPT_PRESENT: u128 = 1; // notation Opt<A>.present = 1
pub const HELPER_KIND_SCALAR: u128 = 0; // enums.HelperKind[0] = "Scalar"
pub const SITE_RET_RIGHT: u128 = 11; // enums.SiteTag[11] = "RetRight"
pub const SCALAR_MODE_ADD: u128 = 0; // enums.ScalarMode[0] = "Add"
pub const SCALAR_PHASE_FIRST: u128 = 0; // enums.ScalarPhase[0] = "First"
pub const SCALAR_PHASE_SECOND: u128 = 1; // enums.ScalarPhase[1] = "Second"
pub const SCALAR_PHASE_DONE: u128 = 2; // enums.ScalarPhase[2] = "Done"

pub const SCALAR_STAGED_TAG_NONE: u128 = 0; // enums.ScalarStagedTag[0]
pub const SCALAR_STAGED_TAG_U128_VALUE: u128 = 1; // enums.ScalarStagedTag[1]
pub const SCALAR_STAGED_TAG_BOOL_VALUE: u128 = 2; // enums.ScalarStagedTag[2]
pub const SCALAR_STAGED_TAG_PENDING_NODE: u128 = 3; // enums.ScalarStagedTag[3]
pub const SCALAR_STAGED_TAG_PRIM_ERROR: u128 = 4; // enums.ScalarStagedTag[4]
pub const SCALAR_STAGED_TAG_SELECTOR: u128 = 5; // enums.ScalarStagedTag[5]
pub const SCALAR_STAGED_TAG_TYPE_TEST: u128 = 6; // enums.ScalarStagedTag[6]

pub const PRIM_ERROR_ARITH_OVERFLOW: u128 = 10; // ErrorCode 10: checked overflow/underflow
pub const PRIM_ERROR_SORT_MISMATCH: u128 = 11; // ErrorCode 11: sort mismatch

pub const CONST_TAG_U128: u128 = 0; // enums.ConstTag[0] = "U128"
pub const DATA_TAG_U128: u128 = 0; // enums.DataTag[0] = "U128"
pub const VALUE_TAG_U128: u128 = 0; // LogicalKind::UInt ordinal 0
pub const VALUE_TAG_BOOL: u128 = 1; // LogicalKind::Bool ordinal 1
pub const VALUE_TAG_NIL: u128 = 2; // LogicalKind::Nil ordinal 2
pub const VALUE_TAG_CONS: u128 = 3; // LogicalKind::Cons ordinal 3
pub const VALUE_TAG_LEAF: u128 = 4; // LogicalKind::Leaf ordinal 4
pub const VALUE_TAG_BRANCH: u128 = 5; // LogicalKind::Branch ordinal 5
pub const VALID_VALUE_TAG_COUNT: u128 = 6; // core value kinds {UInt,Bool,Nil,Cons,Leaf,Branch}

/// UInt128 as 16 little-endian byte lanes; each enabled lane lies in [0, 255].
pub type BU128Lanes = [Operand; 16];

/// Encode a host `u128` into 16 little-endian immediate byte operands.
pub fn u128_to_le_immediates(n: u128) -> BU128Lanes {
    let bytes = n.to_le_bytes();
    std::array::from_fn(|i| immediate(u128::from(bytes[i])))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BValueSpace {
    Const,
    Data,
}

/// `records.ValueRef`: space bit (`0` = Const, `1` = Data) and index lane.
#[derive(Clone, Debug, PartialEq)]
pub struct BValueRef {
    pub space: Operand,
    pub index: Operand,
}

/// `Opt<ValueRef>`: presence bit (`0` = absent, `1` = present) and `ValueRef`.
#[derive(Clone, Debug, PartialEq)]
pub struct BOptValueRef {
    pub present: Operand,
    pub value: BValueRef,
}

/// Projection of one value-table row: variant tag ordinal lane and `U128.n` byte lanes.
/// By `/notation/inactive`, non-`U128` rows have all 16 `U128.n` lanes equal to zero.
#[derive(Clone, Debug, PartialEq)]
pub struct BValueRowLanes {
    pub tag: Operand,
    pub n: BU128Lanes,
}

/// Every physical row of one value space, active or not; `capacity` may be 0.
#[derive(Clone, Debug, PartialEq)]
pub struct BValueTableRead {
    pub capacity: usize,
    pub active_rows: Operand,
    pub rows: Vec<BValueRowLanes>,
}

/// `records.HelperControl.ScalarStaged`: discriminator `tag` plus union payload
/// columns. By `/notation/inactive`, every inactive union column must be zero.
#[derive(Clone, Debug, PartialEq)]
pub struct BScalarStagedLanes {
    pub tag: Operand,
    pub u128_value: BU128Lanes,
    pub bool_value: Operand,
    pub pending_node_tag: Operand,
    pub prim_error_code: Operand,
    pub selector: Operand,
    pub type_test_ok: Operand,
}

/// Declarative carrier for the remaining `HelperControl::Scalar` fields from
/// `SAFE-INPUTS.json` (`originReferenceCursor`, `parentResume.{present,value}`,
/// `expected.{present,value}`) when mapped into `BScalarArithmeticRegister::rest`.
#[derive(Clone, Debug, PartialEq)]
pub struct BScalarHelperRestLanes {
    pub origin_reference_cursor: Operand,
    pub parent_resume_present: Operand,
    pub parent_resume_value: Operand,
    pub expected_present: Operand,
    pub expected_value: Operand,
}

impl BScalarHelperRestLanes {
    pub fn to_rest_vec(&self) -> Vec<Operand> {
        vec![
            self.origin_reference_cursor.clone(),
            self.parent_resume_present.clone(),
            self.parent_resume_value.clone(),
            self.expected_present.clone(),
            self.expected_value.clone(),
        ]
    }
}

/// `reg.helper` lanes in a `Scalar.arithmetic` view. Additive extension of the
/// archived `005` `BScalarRegister` carrier with explicit `operand_b` and `staged`
/// views. `rest` holds every remaining helper lane in identical pre and post order.
/// `pre.staged` is not constrained in-circuit (`SI-22 S2` is an external pre-StepInv
/// caller obligation); every `post.staged` lane is constrained when `enable = 1`.
#[derive(Clone, Debug, PartialEq)]
pub struct BScalarArithmeticRegister {
    pub present: Operand,
    pub kind: Operand,
    pub site: Operand,
    pub mode: Operand,
    pub phase: Operand,
    pub operand_a: BValueRef,
    pub operand_b: BOptValueRef,
    pub reg_a: BU128Lanes,
    pub staged: BScalarStagedLanes,
    pub rest: Vec<Operand>,
}

/// The four delta-vector entries changed by `T-HELPER` (`[-1, +1, +1, +1, 0..]`),
/// represented as local 16-byte little-endian `UInt128` carriers.
#[derive(Clone, Debug, PartialEq)]
pub struct BChargedCounters {
    pub remaining_work: BU128Lanes,
    pub consumed_work: BU128Lanes,
    pub helper_work: BU128Lanes,
    pub helper_visits: BU128Lanes,
}

/// `post_*` are independent supplied witnesses; every enabled post lane named
/// here is constrained in-circuit. Value tables are read-only aliases.
#[derive(Clone, Debug, PartialEq)]
pub struct BScalarArithmeticAddInputs {
    pub enable: Operand,
    pub constants: BValueTableRead,
    pub data: BValueTableRead,
    pub pre_helper: BScalarArithmeticRegister,
    pub post_helper: BScalarArithmeticRegister,
    pub pre_counters: BChargedCounters,
    pub post_counters: BChargedCounters,
}

/// Composable instruction fragment for `b.helper.Scalar.arithmetic` (`RetRight/Add/Second`).
/// Contains no `PublicInput` or `Output` instructions. `read_tag`, `read_n`, and
/// `b_is_uint` are enable-effective (zero when `enable = 0`).
#[derive(Clone, Debug, PartialEq)]
pub struct BScalarArithmeticAddBlock {
    pub instructions: Vec<Instruction>,
    pub read_tag: Operand,
    pub read_n: BU128Lanes,
    pub b_is_uint: Operand,
    pub overflow: Operand,
    pub carry_chain: [Operand; 16],
    pub preserved: BScalarArithmeticAddInputs,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BScalarArithmeticConstructionError {
    RowCount { space: BValueSpace },
    RestLaneCount,
    Fetch(ConstructionError),
}

impl From<ConstructionError> for BScalarArithmeticConstructionError {
    fn from(e: ConstructionError) -> Self {
        Self::Fetch(e)
    }
}

/// Emit the in-circuit relation for `b.helper.Scalar.arithmetic` on branch
/// `site = RetRight`, `mode = Add`, `phase = Second -> Done`.
pub fn emit_b_scalar_arithmetic_add(
    input: &BScalarArithmeticAddInputs,
    scope: &str,
) -> Result<BScalarArithmeticAddBlock, BScalarArithmeticConstructionError> {
    if scope.is_empty() {
        return Err(ConstructionError::EmptyScope.into());
    }
    if input.constants.rows.len() != input.constants.capacity {
        return Err(BScalarArithmeticConstructionError::RowCount {
            space: BValueSpace::Const,
        });
    }
    if input.data.rows.len() != input.data.capacity {
        return Err(BScalarArithmeticConstructionError::RowCount {
            space: BValueSpace::Data,
        });
    }
    if input.pre_helper.rest.len() != input.post_helper.rest.len() {
        return Err(BScalarArithmeticConstructionError::RestLaneCount);
    }

    let namespaces = [
        format!("%{scope}.local.temp."),
        format!("%{scope}.const_fetch.temp."),
        format!("%{scope}.data_fetch.temp."),
    ];
    for operand in [
        &input.enable,
        &input.constants.active_rows,
        &input.data.active_rows,
    ]
    .into_iter()
    .chain(table_operands(&input.constants))
    .chain(table_operands(&input.data))
    .chain(helper_operands(&input.pre_helper))
    .chain(helper_operands(&input.post_helper))
    .chain(counter_operands(&input.pre_counters))
    .chain(counter_operands(&input.post_counters))
    {
        if let Operand::Variable(id) = operand {
            if !id.0.starts_with('%') {
                return Err(ConstructionError::UnprefixedVariable.into());
            }
            if namespaces.iter().any(|prefix| id.0.starts_with(prefix)) {
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

    // A01-A03: boolean enable and effective operandB space selection (0 Const, 1 Data).
    b.boolean(enable.clone());
    let space = b.select(
        enable.clone(),
        input.pre_helper.operand_b.value.space.clone(),
        immediate(0),
    );
    b.boolean(space.clone());
    let en_const = b.select(space.clone(), immediate(0), enable.clone());
    b.boolean(en_const.clone());

    // A04-A06: enabled value-table fetches for operandB across Const and Data spaces.
    let (tag_const, n_const) = space_read_any_kind(
        &mut b,
        &input.constants,
        en_const,
        input.pre_helper.operand_b.value.index.clone(),
        format!("{scope}.const_fetch"),
    )?;
    let (tag_data, n_data) = space_read_any_kind(
        &mut b,
        &input.data,
        space.clone(),
        input.pre_helper.operand_b.value.index.clone(),
        format!("{scope}.data_fetch"),
    )?;

    // A07: selected operandB tag must be a valid core value kind in 0..=5
    // ({UInt=0, Bool=1, Nil=2, Cons=3, Leaf=4, Branch=5}); reserved/Marker tags rejected.
    let read_tag = b.select(space.clone(), tag_data, tag_const);
    b.bits(read_tag.clone(), 3);
    let valid_tag = b.less_than(read_tag.clone(), immediate(VALID_VALUE_TAG_COUNT));
    b.assert(valid_tag);

    let tag_is_u128 = b.test_eq(read_tag.clone(), immediate(VALUE_TAG_U128));
    let b_is_uint = b.select(enable.clone(), tag_is_u128, immediate(0));
    b.boolean(b_is_uint.clone());

    // A08: selected operandB U128.n bytes; ranged to 8 bits and forced to 0 on non-UInt rows.
    let read_n = std::array::from_fn(|i| {
        let value = b.select(space.clone(), n_data[i].clone(), n_const[i].clone());
        b.bits(value.clone(), 8);
        let active_n = b.select(b_is_uint.clone(), value.clone(), immediate(0));
        b.eq(value.clone(), active_n);
        value
    });

    // A09: pre-state branch guards for RetRight / Add / Second with operandB present.
    for (pre, expected) in [
        (&input.pre_helper.present, HELPER_PRESENT),
        (&input.pre_helper.kind, HELPER_KIND_SCALAR),
        (&input.pre_helper.site, SITE_RET_RIGHT),
        (&input.pre_helper.mode, SCALAR_MODE_ADD),
        (&input.pre_helper.phase, SCALAR_PHASE_SECOND),
        (&input.pre_helper.operand_b.present, OPT_PRESENT),
    ] {
        b.gated_eq(enable, pre.clone(), immediate(expected));
    }

    // A10: enable-effective 8-bit range on pre.reg_a and post.staged.u128_value.
    let eff_reg_a = eff_bytes(&mut b, enable, &input.pre_helper.reg_a);
    let eff_post_u128 = eff_bytes(&mut b, enable, &input.post_helper.staged.u128_value);

    // A11: 16-byte little-endian addition carry chain over eff_reg_a + read_n.
    let mut carry = immediate(0);
    let mut totals: [Operand; 16] = std::array::from_fn(|_| immediate(0));
    let mut carry_chain: [Operand; 16] = std::array::from_fn(|_| immediate(0));
    for i in 0..16 {
        let ab = b.add(eff_reg_a[i].clone(), read_n[i].clone());
        b.bits(ab.clone(), 9);
        let total = b.add(ab, carry);
        b.bits(total.clone(), 9);
        let next_carry = b.less_than(immediate(255), total.clone());
        totals[i] = total;
        carry_chain[i] = next_carry.clone();
        carry = next_carry;
    }
    let overflow = carry;

    // A12: outcome selectors under enable:
    // - is_val = 1 iff enable = 1, b_is_uint = 1, and overflow = 0 (stages U128Value)
    // - otherwise when enable = 1: stages PrimError(10) if b_is_uint = 1 else PrimError(11)
    let not_overflow = b.select(overflow.clone(), immediate(0), immediate(1));
    b.boolean(not_overflow.clone());
    let is_val = b.select(b_is_uint.clone(), not_overflow, immediate(0));
    b.boolean(is_val.clone());

    for i in 0..16 {
        let carry_term = b.mul(immediate(256), carry_chain[i].clone());
        b.bits(carry_term.clone(), 9);
        let reconstructed = b.add(eff_post_u128[i].clone(), carry_term);
        b.bits(reconstructed.clone(), 9);
        b.gated_eq(&is_val, reconstructed, totals[i].clone());
        // Inactive U128Value payload columns must be 0 when PrimError is staged (or disabled).
        let inactive_u128 = b.select(is_val.clone(), immediate(0), eff_post_u128[i].clone());
        b.eq(inactive_u128, immediate(0));
    }

    // A13: post.phase = Done and post.staged discriminator + error code + inactive columns.
    b.gated_eq(
        enable,
        input.post_helper.phase.clone(),
        immediate(SCALAR_PHASE_DONE),
    );
    let expected_staged_tag = b.select(
        is_val,
        immediate(SCALAR_STAGED_TAG_U128_VALUE),
        immediate(SCALAR_STAGED_TAG_PRIM_ERROR),
    );
    b.bits(expected_staged_tag.clone(), 3);
    b.gated_eq(
        enable,
        input.post_helper.staged.tag.clone(),
        expected_staged_tag,
    );

    let code_if_uint = b.select(
        overflow.clone(),
        immediate(PRIM_ERROR_ARITH_OVERFLOW),
        immediate(0),
    );
    b.bits(code_if_uint.clone(), 4);
    let expected_error_code = b.select(
        b_is_uint.clone(),
        code_if_uint,
        immediate(PRIM_ERROR_SORT_MISMATCH),
    );
    b.bits(expected_error_code.clone(), 4);
    b.gated_eq(
        enable,
        input.post_helper.staged.prim_error_code.clone(),
        expected_error_code,
    );

    for inactive_col in [
        &input.post_helper.staged.bool_value,
        &input.post_helper.staged.pending_node_tag,
        &input.post_helper.staged.selector,
        &input.post_helper.staged.type_test_ok,
    ] {
        b.gated_eq(enable, inactive_col.clone(), immediate(0));
    }

    // A14: complete named helper preservation (including reg_a and operand_b) + caller rest.
    for (pre, post) in [
        (&input.pre_helper.present, &input.post_helper.present),
        (&input.pre_helper.kind, &input.post_helper.kind),
        (&input.pre_helper.site, &input.post_helper.site),
        (&input.pre_helper.mode, &input.post_helper.mode),
        (
            &input.pre_helper.operand_a.space,
            &input.post_helper.operand_a.space,
        ),
        (
            &input.pre_helper.operand_a.index,
            &input.post_helper.operand_a.index,
        ),
        (
            &input.pre_helper.operand_b.present,
            &input.post_helper.operand_b.present,
        ),
        (
            &input.pre_helper.operand_b.value.space,
            &input.post_helper.operand_b.value.space,
        ),
        (
            &input.pre_helper.operand_b.value.index,
            &input.post_helper.operand_b.value.index,
        ),
    ]
    .into_iter()
    .chain(eff_reg_a.iter().zip(input.post_helper.reg_a.iter()))
    .chain(
        input
            .pre_helper
            .rest
            .iter()
            .zip(input.post_helper.rest.iter()),
    ) {
        b.gated_eq(enable, post.clone(), pre.clone());
    }

    // A15-A17: enable-effective bytewise counter ranges and exact [-1, +1, +1, +1] deltas.
    let pre_remaining = eff_bytes(&mut b, enable, &input.pre_counters.remaining_work);
    let pre_consumed = eff_bytes(&mut b, enable, &input.pre_counters.consumed_work);
    let pre_helper_work = eff_bytes(&mut b, enable, &input.pre_counters.helper_work);
    let pre_visits = eff_bytes(&mut b, enable, &input.pre_counters.helper_visits);
    let post_remaining = eff_bytes(&mut b, enable, &input.post_counters.remaining_work);
    let post_consumed = eff_bytes(&mut b, enable, &input.post_counters.consumed_work);
    let post_helper_work = eff_bytes(&mut b, enable, &input.post_counters.helper_work);
    let post_visits = eff_bytes(&mut b, enable, &input.post_counters.helper_visits);

    decrement(&mut b, enable, &pre_remaining, &post_remaining);
    increment(&mut b, enable, &pre_consumed, &post_consumed);
    increment(&mut b, enable, &pre_helper_work, &post_helper_work);
    increment(&mut b, enable, &pre_visits, &post_visits);

    Ok(BScalarArithmeticAddBlock {
        instructions: b.code,
        read_tag,
        read_n,
        b_is_uint,
        overflow,
        carry_chain,
        preserved: input.clone(),
    })
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BScalarArithmeticFieldRole {
    Enable,
    ConstActiveRows,
    ConstRowTag { row: usize },
    ConstRowByte { row: usize, byte: usize },
    DataActiveRows,
    DataRowTag { row: usize },
    DataRowByte { row: usize, byte: usize },
    PreHelperLane { index: usize },
    PostHelperLane { index: usize },
    PreCounterByte { counter: usize, byte: usize },
    PostCounterByte { counter: usize, byte: usize },
    ReadTag,
    ReadByte { byte: usize },
    BIsUInt,
    Overflow,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BScalarArithmeticField {
    pub role: BScalarArithmeticFieldRole,
    pub kind: Type,
}

/// Standalone `IrSource` wrapper around `emit_b_scalar_arithmetic_add` with
/// explicit `PublicInput` / `Impact` / `Output` ordering for inspection and
/// downstream bridge consumption. Makes no general ledger or full-B claim.
#[derive(Clone, Debug, PartialEq)]
pub struct EmittedBScalarArithmeticAdd {
    pub ir: IrSource,
    pub input_fields: Vec<BScalarArithmeticField>,
    pub output_fields: Vec<BScalarArithmeticField>,
    pub const_capacity: usize,
    pub data_capacity: usize,
    pub rest_lanes: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BScalarArithmeticWitness {
    pub enable: bool,
    pub const_active_rows: u128,
    pub const_rows: Vec<(u128, [u8; 16])>,
    pub data_active_rows: u128,
    pub data_rows: Vec<(u128, [u8; 16])>,
    pub pre_helper_lanes: Vec<u128>,
    pub post_helper_lanes: Vec<u128>,
    pub pre_counters: [u128; 4],
    pub post_counters: [u128; 4],
}

impl EmittedBScalarArithmeticAdd {
    pub fn pack_witness(
        &self,
        w: &BScalarArithmeticWitness,
    ) -> Result<Vec<Fr>, BScalarArithmeticConstructionError> {
        if w.const_rows.len() != self.const_capacity {
            return Err(BScalarArithmeticConstructionError::RowCount {
                space: BValueSpace::Const,
            });
        }
        if w.data_rows.len() != self.data_capacity {
            return Err(BScalarArithmeticConstructionError::RowCount {
                space: BValueSpace::Data,
            });
        }
        let expected_helper_lanes = 48 + self.rest_lanes;
        if w.pre_helper_lanes.len() != expected_helper_lanes
            || w.post_helper_lanes.len() != expected_helper_lanes
        {
            return Err(BScalarArithmeticConstructionError::RestLaneCount);
        }
        let mut out = Vec::with_capacity(self.input_fields.len());
        out.push(Fr::from(u128::from(w.enable)));
        out.push(Fr::from(w.const_active_rows));
        for (tag, bytes) in &w.const_rows {
            out.push(Fr::from(*tag));
            out.extend(bytes.iter().map(|b| Fr::from(u128::from(*b))));
        }
        out.push(Fr::from(w.data_active_rows));
        for (tag, bytes) in &w.data_rows {
            out.push(Fr::from(*tag));
            out.extend(bytes.iter().map(|b| Fr::from(u128::from(*b))));
        }
        out.extend(w.pre_helper_lanes.iter().copied().map(Fr::from));
        out.extend(w.post_helper_lanes.iter().copied().map(Fr::from));
        for c in w.pre_counters {
            out.extend(c.to_le_bytes().into_iter().map(|b| Fr::from(u128::from(b))));
        }
        for c in w.post_counters {
            out.extend(c.to_le_bytes().into_iter().map(|b| Fr::from(u128::from(b))));
        }
        Ok(out)
    }
}

pub fn emit_public_b_scalar_arithmetic_add(
    const_capacity: usize,
    data_capacity: usize,
    rest_lanes: usize,
) -> Result<EmittedBScalarArithmeticAdd, BScalarArithmeticConstructionError> {
    let mut input_fields = vec![
        BScalarArithmeticField {
            role: BScalarArithmeticFieldRole::Enable,
            kind: Type::Bool,
        },
        BScalarArithmeticField {
            role: BScalarArithmeticFieldRole::ConstActiveRows,
            kind: Type::UInt,
        },
    ];
    for row in 0..const_capacity {
        input_fields.push(BScalarArithmeticField {
            role: BScalarArithmeticFieldRole::ConstRowTag { row },
            kind: Type::UInt,
        });
        for byte in 0..16 {
            input_fields.push(BScalarArithmeticField {
                role: BScalarArithmeticFieldRole::ConstRowByte { row, byte },
                kind: Type::UInt,
            });
        }
    }
    input_fields.push(BScalarArithmeticField {
        role: BScalarArithmeticFieldRole::DataActiveRows,
        kind: Type::UInt,
    });
    for row in 0..data_capacity {
        input_fields.push(BScalarArithmeticField {
            role: BScalarArithmeticFieldRole::DataRowTag { row },
            kind: Type::UInt,
        });
        for byte in 0..16 {
            input_fields.push(BScalarArithmeticField {
                role: BScalarArithmeticFieldRole::DataRowByte { row, byte },
                kind: Type::UInt,
            });
        }
    }
    let helper_lane_count = 48 + rest_lanes;
    for index in 0..helper_lane_count {
        input_fields.push(BScalarArithmeticField {
            role: BScalarArithmeticFieldRole::PreHelperLane { index },
            kind: Type::UInt,
        });
    }
    for index in 0..helper_lane_count {
        input_fields.push(BScalarArithmeticField {
            role: BScalarArithmeticFieldRole::PostHelperLane { index },
            kind: Type::UInt,
        });
    }
    for counter in 0..4 {
        for byte in 0..16 {
            input_fields.push(BScalarArithmeticField {
                role: BScalarArithmeticFieldRole::PreCounterByte { counter, byte },
                kind: Type::UInt,
            });
        }
    }
    for counter in 0..4 {
        for byte in 0..16 {
            input_fields.push(BScalarArithmeticField {
                role: BScalarArithmeticFieldRole::PostCounterByte { counter, byte },
                kind: Type::UInt,
            });
        }
    }

    let mut instructions = Vec::new();
    let mut ops = Vec::with_capacity(input_fields.len());
    for (position, field) in input_fields.iter().enumerate() {
        let id = Identifier(format!("%b_scalar_arith.input.{position}"));
        instructions.push(Instruction::PublicInput {
            guard: None,
            val_t: IrType::Native,
            output: id.clone(),
        });
        let value = Operand::Variable(id);
        constrain(&mut instructions, value.clone(), field.kind);
        instructions.push(Instruction::Impact {
            guard: immediate(1),
            inputs: vec![value.clone()],
        });
        ops.push(value);
    }

    let mut cursor = 0usize;
    let enable = ops[cursor].clone();
    cursor += 1;
    let const_active_rows = ops[cursor].clone();
    cursor += 1;
    let mut const_rows = Vec::with_capacity(const_capacity);
    for _ in 0..const_capacity {
        let tag = ops[cursor].clone();
        cursor += 1;
        let n = std::array::from_fn(|i| ops[cursor + i].clone());
        cursor += 16;
        const_rows.push(BValueRowLanes { tag, n });
    }
    let data_active_rows = ops[cursor].clone();
    cursor += 1;
    let mut data_rows = Vec::with_capacity(data_capacity);
    for _ in 0..data_capacity {
        let tag = ops[cursor].clone();
        cursor += 1;
        let n = std::array::from_fn(|i| ops[cursor + i].clone());
        cursor += 16;
        data_rows.push(BValueRowLanes { tag, n });
    }
    let pre_helper = unpack_helper_lanes(&ops[cursor..cursor + helper_lane_count]);
    cursor += helper_lane_count;
    let post_helper = unpack_helper_lanes(&ops[cursor..cursor + helper_lane_count]);
    cursor += helper_lane_count;
    let pre_counters = unpack_counter_lanes(&ops[cursor..cursor + 64]);
    cursor += 64;
    let post_counters = unpack_counter_lanes(&ops[cursor..cursor + 64]);

    let inputs = BScalarArithmeticAddInputs {
        enable,
        constants: BValueTableRead {
            capacity: const_capacity,
            active_rows: const_active_rows,
            rows: const_rows,
        },
        data: BValueTableRead {
            capacity: data_capacity,
            active_rows: data_active_rows,
            rows: data_rows,
        },
        pre_helper,
        post_helper,
        pre_counters,
        post_counters,
    };
    let block = emit_b_scalar_arithmetic_add(&inputs, "b_scalar_arith")?;
    instructions.extend(block.instructions);

    let mut outputs = Vec::with_capacity(19 + ops.len());
    outputs.push(block.read_tag);
    outputs.extend(block.read_n);
    outputs.push(block.b_is_uint);
    outputs.push(block.overflow);
    outputs.extend(ops);

    let mut output_fields = vec![BScalarArithmeticField {
        role: BScalarArithmeticFieldRole::ReadTag,
        kind: Type::UInt,
    }];
    for byte in 0..16 {
        output_fields.push(BScalarArithmeticField {
            role: BScalarArithmeticFieldRole::ReadByte { byte },
            kind: Type::UInt,
        });
    }
    output_fields.push(BScalarArithmeticField {
        role: BScalarArithmeticFieldRole::BIsUInt,
        kind: Type::Bool,
    });
    output_fields.push(BScalarArithmeticField {
        role: BScalarArithmeticFieldRole::Overflow,
        kind: Type::Bool,
    });
    output_fields.extend(input_fields.iter().cloned());

    instructions.push(Instruction::Output { vals: outputs });
    let ir = IrSource {
        version: Default::default(),
        inputs: vec![],
        outputs: vec![IrType::Native; output_fields.len()],
        do_communications_commitment: false,
        instructions: Arc::new(instructions),
    };
    Ok(EmittedBScalarArithmeticAdd {
        ir,
        input_fields,
        output_fields,
        const_capacity,
        data_capacity,
        rest_lanes,
    })
}

fn unpack_helper_lanes(slice: &[Operand]) -> BScalarArithmeticRegister {
    BScalarArithmeticRegister {
        present: slice[0].clone(),
        kind: slice[1].clone(),
        site: slice[2].clone(),
        mode: slice[3].clone(),
        phase: slice[4].clone(),
        operand_a: BValueRef {
            space: slice[5].clone(),
            index: slice[6].clone(),
        },
        operand_b: BOptValueRef {
            present: slice[7].clone(),
            value: BValueRef {
                space: slice[8].clone(),
                index: slice[9].clone(),
            },
        },
        reg_a: std::array::from_fn(|i| slice[10 + i].clone()),
        staged: BScalarStagedLanes {
            tag: slice[26].clone(),
            u128_value: std::array::from_fn(|i| slice[27 + i].clone()),
            bool_value: slice[43].clone(),
            pending_node_tag: slice[44].clone(),
            prim_error_code: slice[45].clone(),
            selector: slice[46].clone(),
            type_test_ok: slice[47].clone(),
        },
        rest: slice[48..].to_vec(),
    }
}

fn unpack_counter_lanes(slice: &[Operand]) -> BChargedCounters {
    BChargedCounters {
        remaining_work: std::array::from_fn(|i| slice[i].clone()),
        consumed_work: std::array::from_fn(|i| slice[16 + i].clone()),
        helper_work: std::array::from_fn(|i| slice[32 + i].clone()),
        helper_visits: std::array::from_fn(|i| slice[48 + i].clone()),
    }
}

fn table_operands(table: &BValueTableRead) -> impl Iterator<Item = &Operand> {
    table
        .rows
        .iter()
        .flat_map(|row| std::iter::once(&row.tag).chain(row.n.iter()))
}

fn helper_operands(helper: &BScalarArithmeticRegister) -> impl Iterator<Item = &Operand> {
    [
        &helper.present,
        &helper.kind,
        &helper.site,
        &helper.mode,
        &helper.phase,
        &helper.operand_a.space,
        &helper.operand_a.index,
        &helper.operand_b.present,
        &helper.operand_b.value.space,
        &helper.operand_b.value.index,
    ]
    .into_iter()
    .chain(helper.reg_a.iter())
    .chain(std::iter::once(&helper.staged.tag))
    .chain(helper.staged.u128_value.iter())
    .chain([
        &helper.staged.bool_value,
        &helper.staged.pending_node_tag,
        &helper.staged.prim_error_code,
        &helper.staged.selector,
        &helper.staged.type_test_ok,
    ])
    .chain(helper.rest.iter())
}

fn counter_operands(counters: &BChargedCounters) -> impl Iterator<Item = &Operand> {
    [
        &counters.remaining_work,
        &counters.consumed_work,
        &counters.helper_work,
        &counters.helper_visits,
    ]
    .into_iter()
    .flatten()
}

struct Emitter {
    scope: String,
    serial: usize,
    code: Vec<Instruction>,
}

impl Emitter {
    fn fresh(&mut self) -> Identifier {
        let id = Identifier(format!("%{}.local.temp.{}", self.scope, self.serial));
        self.serial += 1;
        id
    }

    fn select(&mut self, bit: Operand, a: Operand, b: Operand) -> Operand {
        let output = self.fresh();
        self.code.push(Instruction::CondSelect {
            bit,
            a,
            b,
            output: output.clone(),
        });
        Operand::Variable(output)
    }

    fn eq(&mut self, a: Operand, b: Operand) {
        self.code.push(Instruction::ConstrainEq { a, b });
    }

    fn gated_eq(&mut self, enable: &Operand, a: Operand, b: Operand) {
        let guarded = self.select(enable.clone(), a, b.clone());
        self.eq(guarded, b);
    }

    fn boolean(&mut self, val: Operand) {
        self.code.push(Instruction::ConstrainToBoolean { val });
    }

    fn bits(&mut self, val: Operand, bits: u32) {
        self.code.push(Instruction::ConstrainBits { val, bits });
    }

    fn assert(&mut self, cond: Operand) {
        self.code.push(Instruction::Assert { cond });
    }

    fn test_eq(&mut self, a: Operand, other: Operand) -> Operand {
        let output = self.fresh();
        self.code.push(Instruction::TestEq {
            a,
            b: other,
            output: output.clone(),
        });
        let value = Operand::Variable(output);
        self.boolean(value.clone());
        value
    }

    fn less_than(&mut self, a: Operand, b: Operand) -> Operand {
        let output = self.fresh();
        self.code.push(Instruction::LessThan {
            a,
            b,
            bits: 128,
            output: output.clone(),
        });
        let value = Operand::Variable(output);
        self.boolean(value.clone());
        value
    }

    fn add(&mut self, a: Operand, b: Operand) -> Operand {
        let output = self.fresh();
        self.code.push(Instruction::Add {
            a,
            b,
            output: output.clone(),
        });
        Operand::Variable(output)
    }

    fn mul(&mut self, a: Operand, b: Operand) -> Operand {
        let output = self.fresh();
        self.code.push(Instruction::Mul {
            a,
            b,
            output: output.clone(),
        });
        Operand::Variable(output)
    }
}

fn space_read_any_kind(
    b: &mut Emitter,
    table: &BValueTableRead,
    enable: Operand,
    index: Operand,
    scope: String,
) -> Result<(Operand, BU128Lanes), BScalarArithmeticConstructionError> {
    if table.capacity == 0 {
        b.eq(enable, immediate(0));
        return Ok((immediate(0), std::array::from_fn(|_| immediate(0))));
    }
    let mut columns = vec![Column {
        name: "tag".into(),
        kind: Type::UInt,
    }];
    columns.extend((0..16).map(|i| Column {
        name: format!("n.{i}"),
        kind: Type::UInt,
    }));
    let shape = TableShape::new(table.capacity, columns)?;
    let rows = table
        .rows
        .iter()
        .map(|row| {
            std::iter::once(row.tag.clone())
                .chain(row.n.iter().cloned())
                .collect()
        })
        .collect();
    let fetch = emit_enabled_fetch_block(
        &shape,
        &EnabledFetchInputs {
            enable,
            active_rows: table.active_rows.clone(),
            index,
            rows,
        },
        &scope,
    )?;
    b.code.extend(fetch.instructions);
    Ok((
        fetch.selected[0].value.clone(),
        std::array::from_fn(|i| fetch.selected[i + 1].value.clone()),
    ))
}

fn eff_bytes(b: &mut Emitter, enable: &Operand, lanes: &BU128Lanes) -> BU128Lanes {
    std::array::from_fn(|i| {
        let value = b.select(enable.clone(), lanes[i].clone(), immediate(0));
        b.bits(value.clone(), 8);
        value
    })
}

fn decrement(b: &mut Emitter, enable: &Operand, pre: &BU128Lanes, post: &BU128Lanes) {
    let mut borrow = enable.clone();
    for i in 0..16 {
        let zero = b.test_eq(pre[i].clone(), immediate(0));
        let next = b.select(borrow.clone(), zero, immediate(0));
        b.boolean(next.clone());
        let carry = b.mul(immediate(256), next.clone());
        b.bits(carry.clone(), 9);
        let left = b.add(pre[i].clone(), carry);
        b.bits(left.clone(), 9);
        let right = b.add(post[i].clone(), borrow);
        b.bits(right.clone(), 9);
        b.eq(left, right);
        borrow = next;
    }
    b.eq(borrow, immediate(0));
}

fn increment(b: &mut Emitter, enable: &Operand, pre: &BU128Lanes, post: &BU128Lanes) {
    let mut carry = enable.clone();
    for i in 0..16 {
        let full = b.test_eq(pre[i].clone(), immediate(255));
        let next = b.select(carry.clone(), full, immediate(0));
        b.boolean(next.clone());
        let overflow = b.mul(immediate(256), next.clone());
        b.bits(overflow.clone(), 9);
        let left = b.add(post[i].clone(), overflow);
        b.bits(left.clone(), 9);
        let right = b.add(pre[i].clone(), carry);
        b.bits(right.clone(), 9);
        b.eq(left, right);
        carry = next;
    }
    b.eq(carry, immediate(0));
}
