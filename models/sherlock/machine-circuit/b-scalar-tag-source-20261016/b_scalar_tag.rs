//! Unregistered standalone B-core positive leaf: b.helper.Scalar.tag on
//! site RetRight, mode Add, phase First, kind(operandA) = UInt. Source only:
//! emits SDK instructions; no load, check, evaluation, compilation or proof.
//! Carriers are local logical lanes, NOT a B serialized profile or width table.
//! AB, P0, P3, exactly-one dispatch, StepInv, untouched-image preservation and
//! all program/image/statement binding remain caller obligations.
use super::*;

pub const HELPER_PRESENT: u128 = 1;      // notation Opt<A>.present
pub const HELPER_KIND_SCALAR: u128 = 0;  // enums.HelperKind[0]
pub const SITE_RET_RIGHT: u128 = 11;     // enums.SiteTag[11]
pub const SCALAR_MODE_ADD: u128 = 0;     // enums.ScalarMode[0]
pub const SCALAR_PHASE_FIRST: u128 = 0;  // enums.ScalarPhase[0]
pub const SCALAR_PHASE_SECOND: u128 = 1; // enums.ScalarPhase[1]
pub const CONST_TAG_U128: u128 = 0;      // enums.ConstTag[0]
pub const DATA_TAG_U128: u128 = 0;       // enums.DataTag[0]

/// UInt128 as 16 little-endian byte lanes; each enabled lane lies in [0, 255].
pub type BU128Lanes = [Operand; 16];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BValueSpace { Const, Data }

/// records.ValueRef: space bit (0 Const, 1 Data) and index lane.
#[derive(Clone, Debug, PartialEq)]
pub struct BValueRef { pub space: Operand, pub index: Operand }

/// Projection of one value-table row: variant tag ordinal lane and U128.n lanes.
#[derive(Clone, Debug, PartialEq)]
pub struct BValueRowLanes { pub tag: Operand, pub n: BU128Lanes }

/// Every physical row of one value space, active or not; capacity may be 0.
#[derive(Clone, Debug, PartialEq)]
pub struct BValueTableRead {
    pub capacity: usize,
    pub active_rows: Operand,
    pub rows: Vec<BValueRowLanes>,
}

/// reg.helper lanes in a Scalar view. rest holds every other register lane,
/// in the same order in pre and post. pre.reg_a is never referenced.
#[derive(Clone, Debug, PartialEq)]
pub struct BScalarRegister {
    pub present: Operand,
    pub kind: Operand,
    pub site: Operand,
    pub mode: Operand,
    pub phase: Operand,
    pub operand_a: BValueRef,
    pub reg_a: BU128Lanes,
    pub rest: Vec<Operand>,
}

/// The four delta-vector entries this row changes, as local u128 carriers.
#[derive(Clone, Debug, PartialEq)]
pub struct BChargedCounters {
    pub remaining_work: BU128Lanes,
    pub consumed_work: BU128Lanes,
    pub helper_work: BU128Lanes,
    pub helper_visits: BU128Lanes,
}

/// post_* are independent supplied witnesses; every enabled post lane named
/// here is constrained. Value tables are read-only aliases.
#[derive(Clone, Debug, PartialEq)]
pub struct BScalarTagAddInputs {
    pub enable: Operand,
    pub constants: BValueTableRead,
    pub data: BValueTableRead,
    pub pre_helper: BScalarRegister,
    pub post_helper: BScalarRegister,
    pub pre_counters: BChargedCounters,
    pub post_counters: BChargedCounters,
}

/// Fragment only: no PublicInput, Output, success flag or after-state.
/// read_n is enable-effective (zero when disabled) and is not row data then.
pub struct BScalarTagAddBlock {
    pub instructions: Vec<Instruction>,
    pub read_n: BU128Lanes,
    pub preserved: BScalarTagAddInputs,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BScalarTagConstructionError {
    RowCount { space: BValueSpace },
    RestLaneCount,
    Fetch(ConstructionError),
}
impl From<ConstructionError> for BScalarTagConstructionError {
    fn from(e: ConstructionError) -> Self { Self::Fetch(e) }
}

pub fn emit_b_scalar_tag_add(
    input: &BScalarTagAddInputs,
    scope: &str,
) -> Result<BScalarTagAddBlock, BScalarTagConstructionError> {
    if scope.is_empty() {
        return Err(ConstructionError::EmptyScope.into());
    }
    if input.constants.rows.len() != input.constants.capacity {
        return Err(BScalarTagConstructionError::RowCount { space: BValueSpace::Const });
    }
    if input.data.rows.len() != input.data.capacity {
        return Err(BScalarTagConstructionError::RowCount { space: BValueSpace::Data });
    }
    if input.pre_helper.rest.len() != input.post_helper.rest.len() {
        return Err(BScalarTagConstructionError::RestLaneCount);
    }
    let namespaces = [
        format!("%{scope}.local.temp."),
        format!("%{scope}.const_fetch.temp."),
        format!("%{scope}.data_fetch.temp."),
    ];
    for operand in [&input.enable, &input.constants.active_rows, &input.data.active_rows]
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

    let mut b = Emitter { scope: scope.into(), serial: 0, code: Vec::new() };
    let enable = &input.enable;
    // Owner K01-K03: effective space and exactly one enabled value-space read.
    b.boolean(enable.clone());
    let space = b.select(enable.clone(), input.pre_helper.operand_a.space.clone(), immediate(0));
    b.boolean(space.clone());
    let en_const = b.select(space.clone(), immediate(0), enable.clone());
    b.boolean(en_const.clone());

    // K04-K06: real enabled fetch constraints, including source active bounds.
    let n_const = space_read(&mut b, &input.constants, en_const,
        input.pre_helper.operand_a.index.clone(), CONST_TAG_U128,
        format!("{scope}.const_fetch"))?;
    let n_data = space_read(&mut b, &input.data, space.clone(),
        input.pre_helper.operand_a.index.clone(), DATA_TAG_U128,
        format!("{scope}.data_fetch"))?;

    // K07: selected UInt payload; disabled-zero is not a fetched row.
    let read_n = std::array::from_fn(|i| {
        let value = b.select(space.clone(), n_data[i].clone(), n_const[i].clone());
        b.bits(value.clone(), 8);
        value
    });
    // K08-K09: positive branch identity and exactly its two helper writes.
    for (pre, expected) in [
        (&input.pre_helper.present, HELPER_PRESENT),
        (&input.pre_helper.kind, HELPER_KIND_SCALAR),
        (&input.pre_helper.site, SITE_RET_RIGHT),
        (&input.pre_helper.mode, SCALAR_MODE_ADD),
        (&input.pre_helper.phase, SCALAR_PHASE_FIRST),
    ] {
        b.gated_eq(enable, pre.clone(), immediate(expected));
    }
    b.gated_eq(enable, input.post_helper.phase.clone(), immediate(SCALAR_PHASE_SECOND));
    for (post, value) in input.post_helper.reg_a.iter().zip(&read_n) {
        b.gated_eq(enable, post.clone(), value.clone());
    }
    // K10: complete named helper complement plus the caller's exact rest partition.
    for (pre, post) in [
        (&input.pre_helper.present, &input.post_helper.present),
        (&input.pre_helper.kind, &input.post_helper.kind),
        (&input.pre_helper.site, &input.post_helper.site),
        (&input.pre_helper.mode, &input.post_helper.mode),
        (&input.pre_helper.operand_a.space, &input.post_helper.operand_a.space),
        (&input.pre_helper.operand_a.index, &input.post_helper.operand_a.index),
    ].into_iter().chain(input.pre_helper.rest.iter().zip(&input.post_helper.rest)) {
        b.gated_eq(enable, post.clone(), pre.clone());
    }
    // K11: only enable-effective local counter lanes are ranged.
    let pre_remaining = eff_bytes(&mut b, enable, &input.pre_counters.remaining_work);
    let pre_consumed = eff_bytes(&mut b, enable, &input.pre_counters.consumed_work);
    let pre_helper_work = eff_bytes(&mut b, enable, &input.pre_counters.helper_work);
    let pre_visits = eff_bytes(&mut b, enable, &input.pre_counters.helper_visits);
    let post_remaining = eff_bytes(&mut b, enable, &input.post_counters.remaining_work);
    let post_consumed = eff_bytes(&mut b, enable, &input.post_counters.consumed_work);
    let post_helper_work = eff_bytes(&mut b, enable, &input.post_counters.helper_work);
    let post_visits = eff_bytes(&mut b, enable, &input.post_counters.helper_visits);
    // K12-K13: exact bytewise -1/+1, with no packed UInt128 shortcut or wrap.
    decrement(&mut b, enable, &pre_remaining, &post_remaining);
    increment(&mut b, enable, &pre_consumed, &post_consumed);
    increment(&mut b, enable, &pre_helper_work, &post_helper_work);
    increment(&mut b, enable, &pre_visits, &post_visits);
    Ok(BScalarTagAddBlock { instructions: b.code, read_n, preserved: input.clone() })
}

fn table_operands(table: &BValueTableRead) -> impl Iterator<Item = &Operand> {
    table.rows.iter().flat_map(|row| std::iter::once(&row.tag).chain(row.n.iter()))
}

fn helper_operands(helper: &BScalarRegister) -> impl Iterator<Item = &Operand> {
    [&helper.present, &helper.kind, &helper.site, &helper.mode, &helper.phase,
        &helper.operand_a.space, &helper.operand_a.index]
        .into_iter().chain(helper.reg_a.iter()).chain(helper.rest.iter())
}

fn counter_operands(counters: &BChargedCounters) -> impl Iterator<Item = &Operand> {
    [&counters.remaining_work, &counters.consumed_work,
        &counters.helper_work, &counters.helper_visits].into_iter().flatten()
}

struct Emitter { scope: String, serial: usize, code: Vec<Instruction> }

impl Emitter {
    fn fresh(&mut self) -> Identifier {
        let id = Identifier(format!("%{}.local.temp.{}", self.scope, self.serial));
        self.serial += 1;
        id
    }
    fn select(&mut self, bit: Operand, a: Operand, b: Operand) -> Operand {
        let output = self.fresh();
        self.code.push(Instruction::CondSelect { bit, a, b, output: output.clone() });
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
    fn test_eq(&mut self, a: Operand, other: Operand) -> Operand {
        let output = self.fresh();
        self.code.push(Instruction::TestEq { a, b: other, output: output.clone() });
        let value = Operand::Variable(output);
        self.boolean(value.clone());
        value
    }
    fn add(&mut self, a: Operand, b: Operand) -> Operand {
        let output = self.fresh();
        self.code.push(Instruction::Add { a, b, output: output.clone() });
        Operand::Variable(output)
    }
    fn mul(&mut self, a: Operand, b: Operand) -> Operand {
        let output = self.fresh();
        self.code.push(Instruction::Mul { a, b, output: output.clone() });
        Operand::Variable(output)
    }
}

fn space_read(
    b: &mut Emitter, table: &BValueTableRead, enable: Operand,
    index: Operand, tag_u128: u128, scope: String,
) -> Result<BU128Lanes, BScalarTagConstructionError> {
    if table.capacity == 0 {
        b.eq(enable, immediate(0));
        return Ok(std::array::from_fn(|_| immediate(0)));
    }
    let mut columns = vec![Column { name: "tag".into(), kind: Type::UInt }];
    columns.extend((0..16).map(|i| Column { name: format!("n.{i}"), kind: Type::UInt }));
    let shape = TableShape::new(table.capacity, columns)?;
    let rows = table.rows.iter().map(|row|
        std::iter::once(row.tag.clone()).chain(row.n.iter().cloned()).collect()).collect();
    let fetch = emit_enabled_fetch_block(&shape, &EnabledFetchInputs {
        enable: enable.clone(), active_rows: table.active_rows.clone(), index, rows,
    }, &scope)?;
    b.code.extend(fetch.instructions);
    // Constructor-owned TableShape has exactly tag + 16 payload columns.
    b.gated_eq(&enable, fetch.selected[0].value.clone(), immediate(tag_u128));
    Ok(std::array::from_fn(|i| fetch.selected[i + 1].value.clone()))
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
