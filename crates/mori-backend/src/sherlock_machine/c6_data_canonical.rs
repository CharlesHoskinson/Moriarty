//! In-circuit canonicality for the padded `sherlock-c6-phys/1` DATA section.
//!
//! The accepted domain is exactly the supplied `c6_phys_data` codec domain
//! (`decode_data_tables`, `data_row_caps`, `cap_fields`, `cap_ref`,
//! `env_row_caps`) placed in the fixed padded partitions of
//! `C6PaddedCarrierGeometry`: header (7), data rows, data-field refs, env
//! rows, value-slot refs, each at its capacity base, with inactive slots
//! exactly zero. This is a structural encoding/cap predicate only: it is
//! not `StepInv`, cross-table membership, reachability, admission, or a
//! proof/native claim.
//!
//! Gate policy: `gate` is boolean-constrained and every lane is
//! 128-bit-range-constrained unconditionally. When `gate == 1` every
//! canonicality constraint below is enforced. When `gate == 0` no header,
//! count, row, or padding constraint is enforced. Host shape admission
//! (checked geometry, exact padded length, `usize::MAX` cap refusal)
//! happens before any constraint is emitted and is not gated.
//!
//! Slice-end arithmetic: both slice lanes are unconditional 128-bit values,
//! so their mathematical sum is in `[0, 2^129 - 2]`. The BLS12-381 scalar
//! modulus is
//! `0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001`,
//! which is greater than `2^129`; therefore field addition of the two lanes
//! is exactly integer addition and cannot wrap modulo the field. The derived
//! end is compared with a 129-bit `LessThan`, never a 128-bit `LessThan`.
//! All other unconditional `LessThan` operands are either a raw 128-bit lane,
//! a small slot immediate, a small cap immediate, or a sum of at most seven
//! boolean `TestEq` results constrained by equality rather than by `LessThan`.
//!
//! Temporary-namespace composition contract:
//! - All intermediates minted by this module have identifiers of the form
//!   `%{scope}.c6data.temp.{serial}`.
//! - That whole prefix is reserved. Caller-supplied gate or DATA operands
//!   using it are rejected with `ReservedOperand`.
//! - A scope containing `%` or the reserved segment `.c6data.temp.` is
//!   rejected with `InvalidScope`.
//! - `emit_c6_data_canonical_block` is a single-shot convenience function:
//!   for one circuit, a given scope may be used by that function at most
//!   once, unless the caller can prove no other block uses the same reserved
//!   prefix.
//! - To compose several DATA blocks under one scope in one circuit, create
//!   one `C6DataCanonicalEmitter` for the scope and call `emit` repeatedly;
//!   its serial never resets, so later blocks cannot rebind earlier temps.
//! - Distinct DATA scopes are disjoint. In the Push integration, the parent
//!   uses `%{scope}.local.temp.*`, while this module uses only
//!   `%{scope}.c6data.temp.*`; the actual parent call is unchanged and calls
//!   the single-shot function exactly once for its `enable`/`pre_data` block.

use midnight_transient_crypto::curve::{FR_BITS, Fr};
use midnight_zkir_v3::ir::Operand;
use midnight_zkir_v3::{Identifier, Instruction};
use mori_core::sherlock::helper_machine::c6_phys::{C6_PHYS_MAGIC, C6_PHYS_VERSION};
use mori_core::sherlock::helper_machine::c6_phys_data::{
    C6StateCaps, DATA_HEADER_LANES, DATA_ROW_LANES, DATA_SECTION_TAG, ENV_ROW_LANES, REF_LANES,
};
use thiserror::Error;

const LANE_BITS: u32 = 128;
const SLICE_END_BITS: u32 = 129;
const DATA_VARIANT_COUNT: u128 = 7;
const RESERVED_SEGMENT: &str = ".c6data.temp.";

#[derive(Debug, Error, PartialEq, Eq)]
pub enum C6DataCanonicalError {
    #[error("invalid scope identifier: must not be empty")]
    EmptyScope,
    #[error(
        "invalid scope identifier: scope must not contain '%' or the reserved segment '.c6data.temp.'"
    )]
    InvalidScope,
    #[error("lane bit width {bits} is invalid for the BLS12-381 scalar field")]
    InvalidBitWidth { bits: u32 },
    #[error("geometry arithmetic overflow in DATA section")]
    GeometryOverflow,
    #[error("padded DATA length mismatch: expected {expected}, got {actual}")]
    LengthMismatch { expected: usize, actual: usize },
    #[error("operand uses reserved DATA temporary identifier: {identifier}")]
    ReservedOperand { identifier: String },
}

/// Checked fixed-partition geometry for one DATA section, derived only
/// from `caps`. `total = 7 + max_data_nodes*5 + max_data_field_slots*2
/// + max_env_cells*5 + max_value_slots*2`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct C6DataGeometry {
    pub data_base: usize,
    pub fields_base: usize,
    pub env_base: usize,
    pub values_base: usize,
    pub total: usize,
}

pub fn c6_data_geometry(caps: &C6StateCaps) -> Result<C6DataGeometry, C6DataCanonicalError> {
    let ov = || C6DataCanonicalError::GeometryOverflow;
    let data_base = DATA_HEADER_LANES;
    let fields_base = caps
        .phys
        .max_data_nodes
        .checked_mul(DATA_ROW_LANES)
        .and_then(|n| data_base.checked_add(n))
        .ok_or_else(ov)?;
    let env_base = caps
        .max_data_field_slots
        .checked_mul(REF_LANES)
        .and_then(|n| fields_base.checked_add(n))
        .ok_or_else(ov)?;
    let values_base = caps
        .phys
        .max_env_cells
        .checked_mul(ENV_ROW_LANES)
        .and_then(|n| env_base.checked_add(n))
        .ok_or_else(ov)?;
    let total = caps
        .phys
        .max_value_slots
        .checked_mul(REF_LANES)
        .and_then(|n| values_base.checked_add(n))
        .ok_or_else(ov)?;
    Ok(C6DataGeometry {
        data_base,
        fields_base,
        env_base,
        values_base,
        total,
    })
}

#[derive(Clone, Debug)]
pub struct C6DataCanonicalInputs {
    pub gate: Operand,
    pub data: Vec<Operand>,
    pub caps: C6StateCaps,
}

#[derive(Clone, Debug)]
pub struct C6DataCanonicalBlock {
    pub instructions: Vec<Instruction>,
}

fn validate_scope(scope: &str) -> Result<(), C6DataCanonicalError> {
    if scope.trim().is_empty() {
        return Err(C6DataCanonicalError::EmptyScope);
    }
    if scope.contains('%') || scope.contains(RESERVED_SEGMENT) {
        return Err(C6DataCanonicalError::InvalidScope);
    }
    Ok(())
}

fn reserved_prefix(scope: &str) -> String {
    format!("%{}.c6data.temp.", scope)
}

fn reject_reserved_operand(scope: &str, operand: &Operand) -> Result<(), C6DataCanonicalError> {
    if let Operand::Variable(id) = operand {
        if id.0.starts_with(&reserved_prefix(scope)) {
            return Err(C6DataCanonicalError::ReservedOperand {
                identifier: id.0.clone(),
            });
        }
    }
    Ok(())
}

struct DataEmitter<'a> {
    scope: &'a str,
    next_temp: usize,
    instructions: Vec<Instruction>,
}

impl<'a> DataEmitter<'a> {
    fn new(scope: &'a str, next_temp: usize) -> Self {
        Self {
            scope,
            next_temp,
            instructions: Vec::new(),
        }
    }
    fn temp(&mut self) -> Identifier {
        let id = Identifier(format!("%{}.c6data.temp.{}", self.scope, self.next_temp));
        self.next_temp += 1;
        id
    }
    fn imm(val: u128) -> Operand {
        Operand::Immediate(Fr::from(val))
    }
    fn constrain_boolean(&mut self, val: Operand) {
        self.instructions
            .push(Instruction::ConstrainToBoolean { val });
    }
    fn constrain_bits(&mut self, val: Operand, bits: u32) {
        self.instructions
            .push(Instruction::ConstrainBits { val, bits });
    }
    fn constrain_eq(&mut self, a: Operand, b: Operand) {
        self.instructions.push(Instruction::ConstrainEq { a, b });
    }
    fn test_eq(&mut self, a: Operand, b: Operand) -> Operand {
        let out = self.temp();
        self.instructions.push(Instruction::TestEq {
            a,
            b,
            output: out.clone(),
        });
        Operand::Variable(out)
    }
    fn less_than(&mut self, a: Operand, b: Operand, bits: u32) -> Operand {
        let out = self.temp();
        self.instructions.push(Instruction::LessThan {
            a,
            b,
            bits,
            output: out.clone(),
        });
        Operand::Variable(out)
    }
    fn less_than_128(&mut self, a: Operand, b: Operand) -> Operand {
        self.less_than(a, b, LANE_BITS)
    }
    fn less_than_slice_end(&mut self, a: Operand, b: Operand) -> Operand {
        self.less_than(a, b, SLICE_END_BITS)
    }
    fn add(&mut self, a: Operand, b: Operand) -> Operand {
        let out = self.temp();
        self.instructions.push(Instruction::Add {
            a,
            b,
            output: out.clone(),
        });
        Operand::Variable(out)
    }
    fn mul(&mut self, a: Operand, b: Operand) -> Operand {
        let out = self.temp();
        self.instructions.push(Instruction::Mul {
            a,
            b,
            output: out.clone(),
        });
        Operand::Variable(out)
    }
    fn not(&mut self, a: Operand) -> Operand {
        let out = self.temp();
        self.instructions.push(Instruction::Not {
            a,
            output: out.clone(),
        });
        Operand::Variable(out)
    }
    fn cond_select(&mut self, bit: Operand, a: Operand, b: Operand) -> Operand {
        let out = self.temp();
        self.instructions.push(Instruction::CondSelect {
            bit,
            a,
            b,
            output: out.clone(),
        });
        Operand::Variable(out)
    }
    fn assert_true(&mut self, cond: Operand) {
        self.instructions.push(Instruction::Assert { cond });
    }
    fn gated_constrain_eq(&mut self, gate: Operand, a: Operand, b: Operand) {
        let selected = self.cond_select(gate, a, b.clone());
        self.constrain_eq(selected, b);
    }
    fn gated_assert(&mut self, gate: Operand, cond: Operand) {
        let effective = self.cond_select(gate, cond, Self::imm(1));
        self.assert_true(effective);
    }
    /// Strict index bound: `value < cap`. `value` is a raw 128-bit lane or a
    /// derived boolean product; `cap` is a host `usize`, hence below `2^64`
    /// on every supported target and also a valid 128-bit operand.
    fn gated_lt_index(&mut self, gate: Operand, val: Operand, cap: usize) {
        let lt = self.less_than_128(val, Self::imm(cap as u128));
        self.gated_assert(gate, lt);
    }
    /// Inclusive raw-lane count/length bound: `value < cap + 1`. The caller
    /// has already refused `cap == usize::MAX` on the host, so `cap + 1`
    /// still fits a 128-bit immediate.
    fn gated_lt_count(&mut self, gate: Operand, val: Operand, cap: usize) {
        let lt = self.less_than_128(val, Self::imm((cap as u128) + 1));
        self.gated_assert(gate, lt);
    }
    /// Inclusive slice-end bound: `end < table_cap + 1`, where `end` is the
    /// field sum of two 128-bit lanes and therefore lies in
    /// `[0, 2^129 - 2]`. This deliberately uses the 129-bit comparator.
    fn gated_lt_slice_end(&mut self, gate: Operand, end: Operand, table_cap: usize) {
        let lt = self.less_than_slice_end(end, Self::imm((table_cap as u128) + 1));
        self.gated_assert(gate, lt);
    }
    fn gated_boolean(&mut self, gate: Operand, val: Operand) {
        let checked = self.cond_select(gate, val, Self::imm(0));
        self.constrain_boolean(checked);
    }
    fn gated_zero(&mut self, gate: Operand, val: Operand) {
        self.gated_constrain_eq(gate, val, Self::imm(0));
    }
    /// `RefC` lanes at `kind`/`index`: kind boolean, index strictly below
    /// the selected table cap (`max_constants` for Const, `max_data_nodes`
    /// for Data). This is a cap check, not active-count membership.
    fn gated_validate_ref(
        &mut self,
        gate: Operand,
        kind: Operand,
        index: Operand,
        caps: &C6StateCaps,
    ) {
        self.gated_boolean(gate.clone(), kind.clone());
        let is_data = self.test_eq(kind, Self::imm(1));
        let cap = self.cond_select(
            is_data,
            Self::imm(caps.phys.max_data_nodes as u128),
            Self::imm(caps.phys.max_constants as u128),
        );
        let lt = self.less_than_128(index, cap);
        self.gated_assert(gate, lt);
    }
    /// Raw field slice: `len <= max_arity` and `start + len <= table_cap`.
    /// `len` is a raw 128-bit lane. The derived `end` is compared at 129
    /// bits, so gate 0 and nonmatching variants impose no accidental bound
    /// beyond the unconditional 128-bit bounds on `start` and `len`
    /// themselves. An empty slice keeps its `start`, as in the codec.
    fn gated_validate_slice(
        &mut self,
        gate: Operand,
        start: Operand,
        len: Operand,
        table_cap: usize,
        caps: &C6StateCaps,
    ) {
        self.gated_lt_count(gate.clone(), len.clone(), caps.phys.max_arity);
        let end = self.add(start, len);
        self.gated_lt_slice_end(gate, end, table_cap);
    }
    /// Env previous option: present boolean; absent => index 0;
    /// present => index strictly below `max_env_cells`.
    fn gated_validate_env_previous(
        &mut self,
        gate: Operand,
        present: Operand,
        index: Operand,
        caps: &C6StateCaps,
    ) {
        self.gated_boolean(gate.clone(), present.clone());
        let is_absent = self.test_eq(present.clone(), Self::imm(0));
        let absent_gate = self.mul(gate.clone(), is_absent);
        self.gated_constrain_eq(absent_gate, index.clone(), Self::imm(0));
        let is_present = self.test_eq(present, Self::imm(1));
        let present_gate = self.mul(gate, is_present);
        self.gated_lt_index(present_gate, index, caps.phys.max_env_cells);
    }
    /// Dynamic active predicate from the constrained count operand:
    /// `slot < count`. Returns `(row_gate, inactive_gate)` already
    /// multiplied by the section gate. `slot` is a capacity index and
    /// `count` is a raw 128-bit lane, so the 128-bit comparator is exact.
    fn slot_gates(&mut self, gate: &Operand, count: &Operand, slot: usize) -> (Operand, Operand) {
        let active = self.less_than_128(Self::imm(slot as u128), count.clone());
        let inactive = self.not(active.clone());
        let row_gate = self.mul(gate.clone(), active);
        let inactive_gate = self.mul(gate.clone(), inactive);
        (row_gate, inactive_gate)
    }
    fn gated_validate_data_row(&mut self, gate: Operand, row: &[Operand], caps: &C6StateCaps) {
        debug_assert_eq!(row.len(), DATA_ROW_LANES);
        let mut tag_bits = Vec::with_capacity(DATA_VARIANT_COUNT as usize);
        let mut tag_sum = Self::imm(0);
        for tag in 0..DATA_VARIANT_COUNT {
            let is_tag = self.test_eq(row[0].clone(), Self::imm(tag));
            tag_sum = self.add(tag_sum, is_tag.clone());
            tag_bits.push(is_tag);
        }
        self.gated_constrain_eq(gate.clone(), tag_sum, Self::imm(1));
        // 0 U128 [0, n, 0, 0, 0]: payload lane unrestricted (full u128).
        let vg = self.mul(gate.clone(), tag_bits[0].clone());
        self.gated_zero(vg.clone(), row[2].clone());
        self.gated_zero(vg.clone(), row[3].clone());
        self.gated_zero(vg, row[4].clone());
        // 1 Bool [1, b, 0, 0, 0].
        let vg = self.mul(gate.clone(), tag_bits[1].clone());
        self.gated_boolean(vg.clone(), row[1].clone());
        self.gated_zero(vg.clone(), row[2].clone());
        self.gated_zero(vg.clone(), row[3].clone());
        self.gated_zero(vg, row[4].clone());
        // 2 Nil [2, 0, 0, 0, 0].
        let vg = self.mul(gate.clone(), tag_bits[2].clone());
        for lane in row.iter().skip(1) {
            self.gated_zero(vg.clone(), lane.clone());
        }
        // 3|4|5 Cons/Leaf/Branch [t, 0, 0, start, len].
        for tag in 3..=5usize {
            let vg = self.mul(gate.clone(), tag_bits[tag].clone());
            self.gated_zero(vg.clone(), row[1].clone());
            self.gated_zero(vg.clone(), row[2].clone());
            self.gated_validate_slice(
                vg,
                row[3].clone(),
                row[4].clone(),
                caps.max_data_field_slots,
                caps,
            );
        }
        // 6 Constructor [6, type, ctor, start, len]: strict DATA caps, no
        // builtin `max(max_types, 4)` exception.
        let vg = self.mul(gate, tag_bits[6].clone());
        self.gated_lt_index(vg.clone(), row[1].clone(), caps.phys.max_types);
        self.gated_lt_index(vg.clone(), row[2].clone(), caps.phys.max_constructors);
        self.gated_validate_slice(
            vg,
            row[3].clone(),
            row[4].clone(),
            caps.max_data_field_slots,
            caps,
        );
    }
}

/// Stateful composer for several DATA blocks sharing one scope in one
/// circuit. The temporary serial is retained between `emit` calls.
pub struct C6DataCanonicalEmitter {
    scope: String,
    next_temp: usize,
}

impl C6DataCanonicalEmitter {
    pub fn new(scope: &str) -> Result<Self, C6DataCanonicalError> {
        validate_scope(scope)?;
        Ok(Self {
            scope: scope.to_string(),
            next_temp: 0,
        })
    }

    pub fn scope(&self) -> &str {
        &self.scope
    }

    pub fn next_temp(&self) -> usize {
        self.next_temp
    }

    pub fn emit(
        &mut self,
        inputs: &C6DataCanonicalInputs,
    ) -> Result<C6DataCanonicalBlock, C6DataCanonicalError> {
        if (LANE_BITS as usize) >= FR_BITS || (SLICE_END_BITS as usize) >= FR_BITS {
            return Err(C6DataCanonicalError::InvalidBitWidth {
                bits: SLICE_END_BITS,
            });
        }
        reject_reserved_operand(&self.scope, &inputs.gate)?;
        for lane in &inputs.data {
            reject_reserved_operand(&self.scope, lane)?;
        }
        let caps = &inputs.caps;
        for cap in [
            caps.phys.max_data_nodes,
            caps.max_data_field_slots,
            caps.phys.max_env_cells,
            caps.phys.max_value_slots,
            caps.phys.max_constants,
            caps.phys.max_types,
            caps.phys.max_constructors,
            caps.phys.max_names,
            caps.phys.max_arity,
        ] {
            if cap == usize::MAX {
                return Err(C6DataCanonicalError::GeometryOverflow);
            }
        }
        let geom = c6_data_geometry(caps)?;
        if inputs.data.len() != geom.total {
            return Err(C6DataCanonicalError::LengthMismatch {
                expected: geom.total,
                actual: inputs.data.len(),
            });
        }

        let mut em = DataEmitter::new(&self.scope, self.next_temp);
        let gate = inputs.gate.clone();
        let data = &inputs.data;
        em.constrain_boolean(gate.clone());
        for lane in data.iter() {
            em.constrain_bits(lane.clone(), LANE_BITS);
        }

        // Header and counts.
        em.gated_constrain_eq(
            gate.clone(),
            data[0].clone(),
            DataEmitter::imm(C6_PHYS_MAGIC),
        );
        em.gated_constrain_eq(
            gate.clone(),
            data[1].clone(),
            DataEmitter::imm(C6_PHYS_VERSION),
        );
        em.gated_constrain_eq(
            gate.clone(),
            data[2].clone(),
            DataEmitter::imm(DATA_SECTION_TAG),
        );
        let counts: [(usize, usize); 4] = [
            (3, caps.phys.max_data_nodes),
            (4, caps.max_data_field_slots),
            (5, caps.phys.max_env_cells),
            (6, caps.phys.max_value_slots),
        ];
        for (lane, cap) in counts {
            em.gated_lt_count(gate.clone(), data[lane].clone(), cap);
        }

        // Data rows.
        for slot in 0..caps.phys.max_data_nodes {
            let base = geom.data_base + slot * DATA_ROW_LANES;
            let row = &data[base..base + DATA_ROW_LANES];
            let (row_gate, inactive_gate) = em.slot_gates(&gate, &data[3], slot);
            for lane in row.iter() {
                em.gated_zero(inactive_gate.clone(), lane.clone());
            }
            em.gated_validate_data_row(row_gate, row, caps);
        }
        // Data-field refs.
        for slot in 0..caps.max_data_field_slots {
            let base = geom.fields_base + slot * REF_LANES;
            let (row_gate, inactive_gate) = em.slot_gates(&gate, &data[4], slot);
            for lane in &data[base..base + REF_LANES] {
                em.gated_zero(inactive_gate.clone(), lane.clone());
            }
            em.gated_validate_ref(row_gate, data[base].clone(), data[base + 1].clone(), caps);
        }
        // Env rows: binder, value ref, previous option.
        for slot in 0..caps.phys.max_env_cells {
            let base = geom.env_base + slot * ENV_ROW_LANES;
            let (row_gate, inactive_gate) = em.slot_gates(&gate, &data[5], slot);
            for lane in &data[base..base + ENV_ROW_LANES] {
                em.gated_zero(inactive_gate.clone(), lane.clone());
            }
            em.gated_lt_index(row_gate.clone(), data[base].clone(), caps.phys.max_names);
            em.gated_validate_ref(
                row_gate.clone(),
                data[base + 1].clone(),
                data[base + 2].clone(),
                caps,
            );
            em.gated_validate_env_previous(
                row_gate,
                data[base + 3].clone(),
                data[base + 4].clone(),
                caps,
            );
        }
        // Value-slot refs.
        for slot in 0..caps.phys.max_value_slots {
            let base = geom.values_base + slot * REF_LANES;
            let (row_gate, inactive_gate) = em.slot_gates(&gate, &data[6], slot);
            for lane in &data[base..base + REF_LANES] {
                em.gated_zero(inactive_gate.clone(), lane.clone());
            }
            em.gated_validate_ref(row_gate, data[base].clone(), data[base + 1].clone(), caps);
        }

        self.next_temp = em.next_temp;
        Ok(C6DataCanonicalBlock {
            instructions: em.instructions,
        })
    }
}

/// Emit one complete padded DATA canonicality block. This single-shot form
/// starts its reserved temporary serial at zero; see the module-level
/// composition contract for the caller obligation and for the stateful
/// `C6DataCanonicalEmitter` form.
pub fn emit_c6_data_canonical_block(
    inputs: &C6DataCanonicalInputs,
    scope: &str,
) -> Result<C6DataCanonicalBlock, C6DataCanonicalError> {
    let mut emitter = C6DataCanonicalEmitter::new(scope)?;
    emitter.emit(inputs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use midnight_proofs::circuit::Value;
    use midnight_proofs::dev::MockProver;
    use midnight_transient_crypto::curve::outer;
    use midnight_transient_crypto::proofs::{KeyLocation, ProofPreimage};
    use midnight_zk_stdlib::MidnightCircuit;
    use midnight_zkir_v3::ir::IrMinorVersion;
    use midnight_zkir_v3::ir_types::{IrType, IrValue};
    use midnight_zkir_v3::{IrSource, Preprocessed};
    use mori_core::sherlock::helper_machine::c6_phys::C6PhysCaps;
    use mori_core::sherlock::helper_machine::c6_phys_data::{
        encode_data_row, encode_env_row, encode_ref_row,
    };
    use mori_core::sherlock::helper_machine::complete_observation::{
        DataFieldsSnapshot, DataSnapshot,
    };
    use mori_core::sherlock::helper_machine::shared::adt::{CtorIdC, TypeIdC};
    use mori_core::sherlock::helper_machine::shared::{EnvCellC, RefC};
    use serde_json::json;
    use std::collections::{BTreeSet, HashMap};
    use std::sync::Arc;

    fn caps() -> C6StateCaps {
        let mut phys = C6PhysCaps::test_caps();
        phys.max_data_nodes = 2;
        phys.max_constants = 3;
        phys.max_types = 2;
        phys.max_constructors = 2;
        phys.max_names = 2;
        phys.max_arity = 2;
        phys.max_env_cells = 2;
        phys.max_value_slots = 2;
        C6StateCaps {
            phys,
            max_data_field_slots: 3,
            max_helper_tasks: 0,
        }
    }

    fn build(
        caps: &C6StateCaps,
        data: &[DataSnapshot],
        fields: &[RefC],
        env: &[EnvCellC],
        values: &[RefC],
    ) -> Vec<u128> {
        let g = c6_data_geometry(caps).unwrap();
        let mut out = vec![0u128; g.total];
        out[0] = C6_PHYS_MAGIC;
        out[1] = C6_PHYS_VERSION;
        out[2] = DATA_SECTION_TAG;
        out[3] = data.len() as u128;
        out[4] = fields.len() as u128;
        out[5] = env.len() as u128;
        out[6] = values.len() as u128;
        for (i, row) in data.iter().enumerate() {
            let b = g.data_base + i * DATA_ROW_LANES;
            out[b..b + DATA_ROW_LANES].copy_from_slice(&encode_data_row(row));
        }
        for (i, r) in fields.iter().enumerate() {
            let b = g.fields_base + i * REF_LANES;
            out[b..b + REF_LANES].copy_from_slice(&encode_ref_row(*r));
        }
        for (i, cell) in env.iter().enumerate() {
            let b = g.env_base + i * ENV_ROW_LANES;
            out[b..b + ENV_ROW_LANES].copy_from_slice(&encode_env_row(cell));
        }
        for (i, r) in values.iter().enumerate() {
            let b = g.values_base + i * REF_LANES;
            out[b..b + REF_LANES].copy_from_slice(&encode_ref_row(*r));
        }
        out
    }

    fn f(start: usize, len: usize) -> DataFieldsSnapshot {
        DataFieldsSnapshot { start, len }
    }

    fn to_frs(lanes: &[u128]) -> Vec<Fr> {
        lanes.iter().map(|&lane| Fr::from(lane)).collect()
    }

    fn two_pow_128() -> Fr {
        Fr::from(u128::MAX) + Fr::from(1u64)
    }

    fn two_pow_129_minus_2() -> Fr {
        two_pow_128() + two_pow_128() - Fr::from(2u64)
    }

    /// Build the same independent public-input relation used by both the
    /// preprocessing check and the actual MockProver synthesis test.
    fn build_ir(
        gate: Fr,
        lanes: &[Fr],
        caps: C6StateCaps,
    ) -> Result<(IrSource, Vec<Fr>, Vec<Identifier>), String> {
        let mut instructions: Vec<Instruction> = Vec::new();
        let mut transcript: Vec<Operand> = Vec::new();
        let mut public_ids: Vec<Identifier> = Vec::new();
        for i in 0..lanes.len() + 1 {
            let id = Identifier(format!("%c6data_test.public.{i}"));
            instructions.push(Instruction::PublicInput {
                guard: None,
                val_t: IrType::Native,
                output: id.clone(),
            });
            transcript.push(Operand::Variable(id.clone()));
            public_ids.push(id);
        }
        let block = emit_c6_data_canonical_block(
            &C6DataCanonicalInputs {
                gate: transcript[0].clone(),
                data: transcript[1..].to_vec(),
                caps,
            },
            "c6data_test",
        )
        .map_err(|e| e.to_string())?;
        instructions.extend(block.instructions);
        instructions.push(Instruction::Impact {
            guard: Operand::Immediate(Fr::from(1u64)),
            inputs: transcript,
        });
        let ir = IrSource {
            version: IrMinorVersion::V0,
            inputs: Vec::new(),
            outputs: Vec::new(),
            do_communications_commitment: false,
            instructions: Arc::new(instructions),
        };
        let mut disk = serde_json::to_value(&ir).map_err(|e| e.to_string())?;
        disk["version"] = json!({ "major": 3, "minor": 0 });
        let serialized = serde_json::to_string(&disk).map_err(|e| e.to_string())?;
        let loaded = IrSource::load(serialized.as_bytes()).map_err(|e| e.to_string())?;
        let mut statement = vec![gate];
        statement.extend(lanes.iter().copied());
        Ok((loaded, statement, public_ids))
    }

    /// Real pinned-SDK preprocessing check over independent raw lane
    /// variables. This is `ProofPreimage::check` only, UNRUN here.
    fn run_check_fr(gate: Fr, lanes: &[Fr], caps: C6StateCaps) -> Result<(), String> {
        let (loaded, statement, _) = build_ir(gate, lanes, caps)?;
        let preimage = ProofPreimage {
            inputs: Vec::new(),
            private_transcript: Vec::new(),
            public_transcript_inputs: statement.clone(),
            public_transcript_outputs: statement,
            binding_input: Fr::from(0u64),
            communications_commitment: None,
            key_location: KeyLocation("sherlock_c6_data_canonical_check".into()),
        };
        preimage
            .check(&loaded)
            .map(|_| ())
            .map_err(|e| e.to_string())
    }

    fn run_check(gate: u128, lanes: &[u128], caps: C6StateCaps) -> Result<(), String> {
        run_check_fr(Fr::from(gate), &to_frs(lanes), caps)
    }

    /// Test-only synthesis outcome category. `ConstraintFailure` is only the
    /// `MockProver::verify` unsatisfied-constraint outcome. `SynthesisFailure`
    /// covers IR construction and `MockProver::run` synthesis/setup errors.
    /// Preprocessing rejection via `run_check_fr` is neither variant; a panic
    /// is never converted into either variant.
    #[derive(Debug, PartialEq, Eq)]
    enum SynthesisRejection {
        ConstraintFailure(String),
        SynthesisFailure(String),
    }

    /// Actual in-circuit synthesis through `IrSource` as a
    /// `midnight_zk_stdlib::Relation`, synthesized by `MidnightCircuit` and
    /// checked by `MockProver`. The `Preprocessed` witness is populated only
    /// with the public-input memory and public-input vector that
    /// preprocessing defines for these public inputs; every intermediate is
    /// produced by the emitted `IrSource` circuit itself. A preprocessing
    /// rejection remains distinguishable through `run_check_fr`.
    fn run_synthesis_fr(
        gate: Fr,
        lanes: &[Fr],
        caps: C6StateCaps,
    ) -> Result<(), SynthesisRejection> {
        let (ir, statement, public_ids) =
            build_ir(gate, lanes, caps).map_err(SynthesisRejection::SynthesisFailure)?;
        let mut memory: HashMap<Identifier, IrValue> = HashMap::new();
        for (id, value) in public_ids.iter().zip(statement.iter()) {
            memory.insert(id.clone(), IrValue::Native(*value));
        }
        let instance: Vec<outer::Scalar> = statement.iter().map(|value| value.0).collect();
        let mut pis: Vec<outer::Scalar> = vec![Fr::from(0u64).0];
        pis.extend(instance.iter().copied());
        let preimage = Preprocessed {
            memory,
            pis: pis.clone(),
            pi_skips: vec![None],
            binding_input: Fr::from(0u64).0,
            comm_comm: None,
        };
        let circuit = MidnightCircuit::new(
            &ir,
            Value::known(instance.clone()),
            Value::known(preimage),
            Some(14),
        );
        match MockProver::<outer::Scalar>::run(&circuit, vec![Vec::new(), pis]) {
            Ok(prover) => prover
                .verify()
                .map_err(|errors| SynthesisRejection::ConstraintFailure(format!("{errors:?}"))),
            Err(error) => Err(SynthesisRejection::SynthesisFailure(format!("{error:?}"))),
        }
    }

    fn valid(caps: C6StateCaps) -> Vec<u128> {
        build(
            &caps,
            &[
                DataSnapshot::U128 { n: u128::MAX },
                DataSnapshot::Bool { b: true },
            ],
            &[RefC::Const(2), RefC::Data(1), RefC::Const(0)],
            &[
                EnvCellC {
                    binder: 1,
                    value: RefC::Data(1),
                    previous: Some(0),
                },
                EnvCellC {
                    binder: 0,
                    value: RefC::Const(0),
                    previous: None,
                },
            ],
            &[RefC::Data(1), RefC::Const(2)],
        )
    }

    fn bad(lanes: &[u128], caps: C6StateCaps, at: usize, val: u128) {
        let mut m = lanes.to_vec();
        assert_ne!(m[at], val, "mutation must change its intended lane");
        m[at] = val;
        assert!(run_check(1, &m, caps).is_err(), "lane {at}");
    }

    fn temp_ids(block: &C6DataCanonicalBlock, scope: &str) -> BTreeSet<String> {
        let debug = format!("{:?}", block.instructions);
        let prefix = format!("%{}.c6data.temp.", scope);
        let mut out = BTreeSet::new();
        let mut rest = debug.as_str();
        while let Some(pos) = rest.find(&prefix) {
            let tail = &rest[pos..];
            let end = tail
                .find(|c: char| !(c.is_ascii_alphanumeric() || c == '%' || c == '.' || c == '_'))
                .unwrap_or(tail.len());
            out.insert(tail[..end].to_string());
            rest = &tail[end..];
        }
        out
    }

    #[test]
    fn geometry_formula_and_host_refusals() {
        let c = caps();
        let g = c6_data_geometry(&c).unwrap();
        assert_eq!(
            (
                g.data_base,
                g.fields_base,
                g.env_base,
                g.values_base,
                g.total
            ),
            (7, 17, 23, 33, 37)
        );
        assert_eq!(g.total, 7 + 2 * 5 + 3 * 2 + 2 * 5 + 2 * 2);
        let lanes = valid(c);
        let wrong_len = emit_c6_data_canonical_block(
            &C6DataCanonicalInputs {
                gate: Operand::Immediate(Fr::from(1u64)),
                data: lanes[..lanes.len() - 1]
                    .iter()
                    .map(|&v| Operand::Immediate(Fr::from(v)))
                    .collect(),
                caps: c,
            },
            "x",
        );
        assert!(matches!(
            wrong_len,
            Err(C6DataCanonicalError::LengthMismatch { .. })
        ));
        let mut huge = c;
        huge.phys.max_data_nodes = usize::MAX;
        assert!(matches!(
            c6_data_geometry(&huge),
            Err(C6DataCanonicalError::GeometryOverflow)
        ));
        let scoped = emit_c6_data_canonical_block(
            &C6DataCanonicalInputs {
                gate: Operand::Immediate(Fr::from(1u64)),
                data: lanes
                    .iter()
                    .map(|&v| Operand::Immediate(Fr::from(v)))
                    .collect(),
                caps: c,
            },
            " ",
        );
        assert!(matches!(scoped, Err(C6DataCanonicalError::EmptyScope)));
    }

    #[test]
    fn all_seven_variants_accept_including_full_u128() {
        let c = caps();
        assert!(run_check(1, &valid(c), c).is_ok(), "full-u128 positive");
        for row in [
            DataSnapshot::U128 { n: 0 },
            DataSnapshot::Bool { b: false },
            DataSnapshot::Nil,
            DataSnapshot::Cons { fields: f(1, 2) },
            DataSnapshot::Leaf { fields: f(3, 0) },
            DataSnapshot::Branch { fields: f(0, 1) },
            DataSnapshot::Constructor {
                type_id: TypeIdC(1),
                ctor_id: CtorIdC(1),
                fields: f(2, 1),
            },
        ] {
            let lanes = build(&c, &[row.clone()], &[RefC::Const(0)], &[], &[]);
            assert!(run_check(1, &lanes, c).is_ok(), "{row:?}");
        }
    }

    #[test]
    fn partitions_empty_partial_full_and_inactive_padding() {
        let c = caps();
        let g = c6_data_geometry(&c).unwrap();
        for lanes in [
            build(&c, &[], &[], &[], &[]),
            build(&c, &[DataSnapshot::Nil], &[], &[], &[]),
            build(&c, &[], &[RefC::Data(0)], &[], &[]),
            build(
                &c,
                &[],
                &[],
                &[EnvCellC {
                    binder: 0,
                    value: RefC::Const(0),
                    previous: None,
                }],
                &[],
            ),
            build(&c, &[], &[], &[], &[RefC::Const(0)]),
            valid(c),
        ] {
            assert!(run_check(1, &lanes, c).is_ok());
        }
        // First inactive slot of every partition is independently nonzero.
        let lanes = build(
            &c,
            &[DataSnapshot::Nil],
            &[RefC::Const(0)],
            &[],
            &[RefC::Const(0)],
        );
        bad(&lanes, c, g.data_base + DATA_ROW_LANES, 1);
        bad(&lanes, c, g.fields_base + REF_LANES + 1, 1);
        let lanes = build(&c, &[], &[], &[], &[]);
        bad(&lanes, c, g.env_base + 2, 1);
        bad(&lanes, c, g.values_base + REF_LANES, 1);
        // Count above cap in each header lane, and zero-cap geometry.
        for lane in 3..7usize {
            let cap_lane = [2u128, 3, 2, 2][lane - 3];
            bad(&valid(c), c, lane, cap_lane + 1);
        }
        let mut zero = caps();
        zero.phys.max_data_nodes = 0;
        zero.max_data_field_slots = 0;
        zero.phys.max_env_cells = 0;
        zero.phys.max_value_slots = 0;
        let lanes = build(&zero, &[], &[], &[], &[]);
        assert_eq!(lanes.len(), DATA_HEADER_LANES);
        assert!(run_check(1, &lanes, zero).is_ok());
        bad(&lanes, zero, 3, 1);
    }

    #[test]
    fn header_tag_bool_padding_slice_and_constructor_boundaries() {
        let c = caps();
        let g = c6_data_geometry(&c).unwrap();
        let lanes = valid(c);
        for (at, val) in [(0usize, 42u128), (1, 2), (2, 0)] {
            bad(&lanes, c, at, val);
        }
        // Tag, Bool payload, Nil and Cons unused lanes.
        bad(&lanes, c, g.data_base, 7);
        bad(&lanes, c, g.data_base + DATA_ROW_LANES + 1, 2);
        let nil = build(&c, &[DataSnapshot::Nil], &[], &[], &[]);
        bad(&nil, c, g.data_base + 1, 1);
        let cons = build(&c, &[DataSnapshot::Cons { fields: f(0, 1) }], &[], &[], &[]);
        bad(&cons, c, g.data_base + 1, 1);
        bad(&cons, c, g.data_base + 2, 1);
        // Slice: len cap-1/cap pass, cap+1 fails; end at cap passes,
        // cap+1 fails; empty slice retains a start at the table cap.
        for (start, len, ok) in [
            (0usize, 1usize, true),
            (0, 2, true),
            (0, 3, false),
            (2, 1, true),
            (3, 1, false),
            (3, 0, true),
            (2, 2, false),
        ] {
            let lanes = build(
                &c,
                &[DataSnapshot::Leaf {
                    fields: f(start, len),
                }],
                &[],
                &[],
                &[],
            );
            assert_eq!(
                run_check(1, &lanes, c).is_ok(),
                ok,
                "start={start} len={len}"
            );
        }
        // Constructor strict DATA caps: cap-1 passes, cap and cap+1 fail.
        // No builtin max(max_types, 4) widening is applied.
        for (type_id, ctor_id, ok) in [
            (1usize, 1usize, true),
            (2, 0, false),
            (3, 0, false),
            (0, 2, false),
            (0, 3, false),
        ] {
            let lanes = build(
                &c,
                &[DataSnapshot::Constructor {
                    type_id: TypeIdC(type_id),
                    ctor_id: CtorIdC(ctor_id),
                    fields: f(0, 0),
                }],
                &[],
                &[],
                &[],
            );
            assert_eq!(run_check(1, &lanes, c).is_ok(), ok, "{type_id} {ctor_id}");
        }
    }

    #[test]
    fn ref_env_boundaries_are_strict_and_split_by_table() {
        let c = caps();
        let g = c6_data_geometry(&c).unwrap();
        // Const cap is 3, Data cap is 2: index 2 passes as Const, fails as Data.
        let ok = build(&c, &[], &[RefC::Const(2)], &[], &[RefC::Data(1)]);
        assert!(run_check(1, &ok, c).is_ok());
        bad(&ok, c, g.fields_base, 2);
        bad(&ok, c, g.fields_base + 1, 3);
        bad(&ok, c, g.values_base + 1, 2);
        // Env binder, value ref, previous present/absent.
        let env = build(
            &c,
            &[],
            &[],
            &[EnvCellC {
                binder: 1,
                value: RefC::Const(2),
                previous: Some(1),
            }],
            &[],
        );
        assert!(run_check(1, &env, c).is_ok());
        bad(&env, c, g.env_base, 2);
        bad(&env, c, g.env_base + 1, 2);
        bad(&env, c, g.env_base + 4, 2);
        let absent = build(
            &c,
            &[],
            &[],
            &[EnvCellC {
                binder: 0,
                value: RefC::Const(0),
                previous: None,
            }],
            &[],
        );
        bad(&absent, c, g.env_base + 4, 1);
        bad(&absent, c, g.env_base + 3, 2);
    }

    #[test]
    fn disabled_gate_namespace_and_lane_range_policy() {
        let c = caps();
        let mut lanes = valid(c);
        lanes[0] = 42;
        lanes[c6_data_geometry(&c).unwrap().data_base] = 7;
        assert!(run_check(1, &lanes, c).is_err());
        assert!(
            run_check(0, &lanes, c).is_ok(),
            "gate 0 enforces no canonicality"
        );
        // Distinct scopes produce distinct temp identifiers.
        let ops: Vec<Operand> = lanes
            .iter()
            .map(|&v| Operand::Immediate(Fr::from(v)))
            .collect();
        let a = emit_c6_data_canonical_block(
            &C6DataCanonicalInputs {
                gate: Operand::Immediate(Fr::from(1u64)),
                data: ops.clone(),
                caps: c,
            },
            "scope_a",
        )
        .unwrap();
        let b = emit_c6_data_canonical_block(
            &C6DataCanonicalInputs {
                gate: Operand::Immediate(Fr::from(1u64)),
                data: ops,
                caps: c,
            },
            "scope_b",
        )
        .unwrap();
        assert_ne!(
            format!("{:?}", a.instructions),
            format!("{:?}", b.instructions)
        );
    }

    #[test]
    fn fr_range_nonboolean_and_derived_slice_end_regressions() {
        let c = caps();
        let g = c6_data_geometry(&c).unwrap();
        let base = to_frs(&valid(c));

        // A lane at exactly 2^128 is outside the unconditional lane domain
        // at both gate values; gate 2 is outside the unconditional boolean
        // gate domain.
        let mut lane_at_pow2 = base.clone();
        lane_at_pow2[g.data_base + 1] = two_pow_128();
        assert!(run_check_fr(Fr::from(0u64), &lane_at_pow2, c).is_err());
        assert!(run_check_fr(Fr::from(1u64), &lane_at_pow2, c).is_err());
        assert!(run_check_fr(Fr::from(2u64), &base, c).is_err());

        // Matching Cons row: end = 2^128. Gate 0 accepts the complete raw
        // domain; gate 1 rejects because end exceeds the field-table cap.
        let mut end_pow2 = base.clone();
        end_pow2[g.data_base] = Fr::from(3u64);
        end_pow2[g.data_base + 1] = Fr::from(0u64);
        end_pow2[g.data_base + 2] = Fr::from(0u64);
        end_pow2[g.data_base + 3] = Fr::from(u128::MAX);
        end_pow2[g.data_base + 4] = Fr::from(1u64);
        assert_eq!(
            end_pow2[g.data_base + 3] + end_pow2[g.data_base + 4],
            two_pow_128()
        );
        assert!(run_check_fr(Fr::from(0u64), &end_pow2, c).is_ok());
        assert!(run_check_fr(Fr::from(1u64), &end_pow2, c).is_err());

        // Stronger derived end: 2^129 - 2, still no field wrap.
        let mut end_max = end_pow2.clone();
        end_max[g.data_base + 4] = Fr::from(u128::MAX);
        assert_eq!(
            end_max[g.data_base + 3] + end_max[g.data_base + 4],
            two_pow_129_minus_2()
        );
        assert!(run_check_fr(Fr::from(0u64), &end_max, c).is_ok());
        assert!(run_check_fr(Fr::from(1u64), &end_max, c).is_err());

        // Nonmatching U128 row: its unused slice lanes may be maximal at
        // gate 0, while gate 1 rejects those same unused lanes as nonzero.
        let mut nonmatching = base.clone();
        nonmatching[g.data_base] = Fr::from(0u64);
        nonmatching[g.data_base + 1] = Fr::from(123u64);
        nonmatching[g.data_base + 2] = Fr::from(0u64);
        nonmatching[g.data_base + 3] = Fr::from(u128::MAX);
        nonmatching[g.data_base + 4] = Fr::from(u128::MAX);
        assert!(run_check_fr(Fr::from(0u64), &nonmatching, c).is_ok());
        assert!(run_check_fr(Fr::from(1u64), &nonmatching, c).is_err());

        // The zero form of that valid enabled non-slice row is accepted.
        let mut nonmatching_zero = nonmatching.clone();
        nonmatching_zero[g.data_base + 3] = Fr::from(0u64);
        nonmatching_zero[g.data_base + 4] = Fr::from(0u64);
        assert!(run_check_fr(Fr::from(1u64), &nonmatching_zero, c).is_ok());
    }

    #[test]
    fn zero_capacity_each_partition_individually() {
        let base = caps();
        for partition in 0..4usize {
            let mut z = base;
            match partition {
                0 => z.phys.max_data_nodes = 0,
                1 => z.max_data_field_slots = 0,
                2 => z.phys.max_env_cells = 0,
                _ => z.phys.max_value_slots = 0,
            }
            let lanes = build(&z, &[], &[], &[], &[]);
            assert!(
                run_check(1, &lanes, z).is_ok(),
                "partition {partition} empty"
            );
            let mut count_one = lanes.clone();
            count_one[3 + partition] = 1;
            assert!(
                run_check(1, &count_one, z).is_err(),
                "partition {partition} count"
            );
        }
    }

    #[test]
    fn namespace_emitter_composition_contract() {
        let c = caps();
        let lanes = valid(c);
        let ops: Vec<Operand> = lanes
            .iter()
            .map(|&v| Operand::Immediate(Fr::from(v)))
            .collect();
        let inputs = C6DataCanonicalInputs {
            gate: Operand::Immediate(Fr::from(1u64)),
            data: ops,
            caps: c,
        };

        // Repeated calls through one owned emitter retain the serial and
        // therefore mint disjoint temporary identifiers under the same scope.
        let mut emitter = C6DataCanonicalEmitter::new("compose").unwrap();
        let first = emitter.emit(&inputs).unwrap();
        let first_next = emitter.next_temp();
        let first_ids = temp_ids(&first, "compose");
        assert_eq!(first_ids.len(), first_next);
        assert!(first_ids.contains("%compose.c6data.temp.0"));
        let second = emitter.emit(&inputs).unwrap();
        let second_ids = temp_ids(&second, "compose");
        assert!(first_ids.is_disjoint(&second_ids));
        assert!(second_ids.contains(&format!("%compose.c6data.temp.{first_next}")));

        // Distinct stateful scopes are also disjoint.
        let mut other = C6DataCanonicalEmitter::new("compose_other").unwrap();
        let other_block = other.emit(&inputs).unwrap();
        let other_ids = temp_ids(&other_block, "compose_other");
        assert!(first_ids.is_disjoint(&other_ids));
        assert!(
            other_ids
                .iter()
                .all(|id| id.starts_with("%compose_other.c6data.temp."))
        );

        // Caller operands may not occupy the reserved temporary namespace.
        let mut reserved_inputs = inputs.clone();
        reserved_inputs.data[0] =
            Operand::Variable(Identifier("%compose.c6data.temp.0".to_string()));
        let mut fresh = C6DataCanonicalEmitter::new("compose").unwrap();
        assert!(matches!(
            fresh.emit(&reserved_inputs),
            Err(C6DataCanonicalError::ReservedOperand { .. })
        ));
        assert!(matches!(
            C6DataCanonicalEmitter::new("bad.c6data.temp.9"),
            Err(C6DataCanonicalError::InvalidScope)
        ));
        assert!(matches!(
            C6DataCanonicalEmitter::new("bad%scope"),
            Err(C6DataCanonicalError::InvalidScope)
        ));
    }

    #[test]
    fn in_circuit_mockprover_synthesis_outcomes() {
        let c = caps();
        let g = c6_data_geometry(&c).unwrap();
        let lanes = to_frs(&valid(c));
        assert!(
            run_synthesis_fr(Fr::from(1u64), &lanes, c).is_ok(),
            "valid synthesis"
        );

        // Wrong raw lane through the actual in-circuit constraint path:
        // set the DATA tag to the noncanonical value 7. This representable
        // lane is the in-circuit raw-lane rejection case. Preprocessing
        // behaviour is established separately, then synthesis must fail by
        // unsatisfied constraint, not by any synthesis/setup error.
        let mut wrong_tag = lanes.clone();
        wrong_tag[g.data_base] = Fr::from(7u64);
        assert!(
            run_check_fr(Fr::from(1u64), &wrong_tag, c).is_err(),
            "wrong-tag preprocessing"
        );
        assert!(
            matches!(
                run_synthesis_fr(Fr::from(1u64), &wrong_tag, c),
                Err(SynthesisRejection::ConstraintFailure(_))
            ),
            "wrong tag must be an unsatisfied-constraint rejection"
        );

        // Raw lane at 2^128: preprocessing-only boundary. The clean
        // ProofPreimage check rejects it. No synthesis form is asserted:
        // ConstrainBits(128) decomposition of 2^128 may panic in the SDK
        // CPU decomposition debug assertion, and a panic is neither a
        // constraint rejection nor circuit evidence.
        let mut lane_at_pow2 = lanes.clone();
        lane_at_pow2[g.data_base + 1] = two_pow_128();
        assert!(
            run_check_fr(Fr::from(1u64), &lane_at_pow2, c).is_err(),
            "2^128 preprocessing"
        );

        // Disabled derived-end regression in circuit: maximal start/len on
        // a matching Cons row is accepted at gate 0 and rejected at gate 1.
        // Preprocessing behaviour is established separately at both gates,
        // then gate-1 synthesis must fail by unsatisfied constraint only.
        let mut derived_end = lanes.clone();
        derived_end[g.data_base] = Fr::from(3u64);
        derived_end[g.data_base + 1] = Fr::from(0u64);
        derived_end[g.data_base + 2] = Fr::from(0u64);
        derived_end[g.data_base + 3] = Fr::from(u128::MAX);
        derived_end[g.data_base + 4] = Fr::from(u128::MAX);
        assert!(
            run_check_fr(Fr::from(0u64), &derived_end, c).is_ok(),
            "derived-end preprocessing gate 0"
        );
        assert!(
            run_check_fr(Fr::from(1u64), &derived_end, c).is_err(),
            "derived-end preprocessing gate 1"
        );
        assert!(
            run_synthesis_fr(Fr::from(0u64), &derived_end, c).is_ok(),
            "derived-end synthesis gate 0"
        );
        assert!(
            matches!(
                run_synthesis_fr(Fr::from(1u64), &derived_end, c),
                Err(SynthesisRejection::ConstraintFailure(_))
            ),
            "derived end at gate 1 must be an unsatisfied-constraint rejection"
        );
    }
}
