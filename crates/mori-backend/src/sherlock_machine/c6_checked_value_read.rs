//! Checked Const/DATA value read over the actual full C6 PROG and the
//! padded DATA carrier. This is an operand-denotation prerequisite leaf:
//! it binds one caller-supplied `RefC`-shaped reference (kind bit +
//! index) to the exact referenced row and exports the Core
//! `pure_leaves` projections of that row. It is not a ScalarTag,
//! ScalarArithmetic or ScalarConstruct transition, not `check_step_inv`,
//! and not a ScalarExit denotation proof.
//!
//! Denotation (from Core `pure_leaves::borrowed_kind/u128/bool` and the
//! Unit 02/03 codecs, not a substitute table):
//! Const codec tags 0..=7 map to kind codes 0..=7 used here:
//! 0 U128->UInt, 1 Bool->Bool, 2 Nil->Nil, 3 Cons, 4 Leaf, 5 Branch,
//! 6 Constructor{type_id,ctor_id,fields}, 7 U128OutOfRange->Marker.
//! DATA tags 0..=6 map to the same kind codes 0..=6; DATA has no Marker.
//! `marker_free` is a Const-only raw lane: `marker_free_present == 1`
//! exactly for Const references, and the exported `marker_free` is the
//! row's raw lane. SI17's held-value marker predicate is a separate,
//! retained premise; this leaf neither erases nor enforces it.
//!
//! Inactive convention (per projection, enforced by construction):
//! a projection is exported as its row value only when its present bit
//! is 1, and as exact 0 otherwise. `uint_value` is present only on
//! UInt, `bool_value` only on Bool, `type_id`/`ctor_id` only on
//! Constructor, and `slice_start`/`slice_len` only on
//! Cons/Leaf/Branch/Constructor. No UInt/Bool value is ever synthesised
//! from a wrong variant. When `gate == 0`, every exported semantic
//! operand below is exact 0 (the disabled convention); gate 0 imposes
//! no membership or canonicality beyond the unconditional 128-bit lane
//! ranges and the boolean gate/ref-kind domains.
//!
//! Membership: under `gate == 1`, a Const reference is accepted only
//! when `index < captured const_count`, where `const_count` is the
//! `PH_CONST_COUNT` lane reached by the full parser walk over all 22
//! earlier variable-length tables (never `<= max_constants` and never
//! a host offset); a DATA reference only when
//! `index < pre_data[3]`, the active DATA count of the padded carrier
//! whose canonicality (inactive slots exactly zero, active rows valid)
//! is emitted here through `C6DataCanonicalEmitter`. Equal-to-count and
//! capacity-padded indices therefore reject in-circuit.
//!
//! Retained premises (NOT discharged here): nominal constructor
//! validation and `checked_sort` (the pure `sort_of_kind` leaf is not
//! claimed to be a proved nominal bridge), const-field/data-field
//! table membership of an exported slice beyond the DATA canonical
//! slice-end bounds, SI08 site/top-frame coherence, SI21/SI22 staged
//! denotation, StepInv, program membership, and native issuance.
//! Constructor ids and slices are exported as the exact row lanes.

use midnight_transient_crypto::curve::{FR_BITS, Fr};
use midnight_zkir_v3::ir::Operand;
use midnight_zkir_v3::{Identifier, Instruction};
use mori_core::sherlock::helper_machine::c6_phys_data::{C6StateCaps, DATA_ROW_LANES};
use thiserror::Error;

use super::c6_data_canonical::{C6DataCanonicalEmitter, C6DataCanonicalInputs, c6_data_geometry};
use super::c6_program_context_canonical::{
    C6ProgramContextCanonicalEmitter, C6ProgramContextConstCaptureInputs,
};

const LANE_BITS: u32 = 128;
const RESERVED_SEGMENTS: [&str; 3] = [".local.temp.", ".c6progctx.temp.", ".c6data.temp."];

#[derive(Debug, Error)]
pub enum C6CheckedValueReadError {
    #[error("invalid scope identifier: must not be empty")]
    EmptyScope,
    #[error("invalid scope identifier: scope must not contain '%' or a reserved composed segment")]
    InvalidScope,
    #[error("operand uses reserved temporary identifier: {identifier}")]
    ReservedOperand { identifier: String },
    #[error("lane bit width {bits} is invalid for the BLS12-381 scalar field")]
    InvalidBitWidth { bits: u32 },
    #[error("geometry arithmetic overflow ({section})")]
    GeometryOverflow { section: &'static str },
    #[error("composed canonical emitter error: {0}")]
    Backend(String),
}

fn be<E: std::fmt::Display>(e: E) -> C6CheckedValueReadError {
    C6CheckedValueReadError::Backend(e.to_string())
}

fn validate_scope(scope: &str) -> Result<(), C6CheckedValueReadError> {
    if scope.trim().is_empty() {
        return Err(C6CheckedValueReadError::EmptyScope);
    }
    if scope.contains('%') || RESERVED_SEGMENTS.iter().any(|s| scope.contains(s)) {
        return Err(C6CheckedValueReadError::InvalidScope);
    }
    Ok(())
}

fn reject_reserved(scope: &str, op: &Operand) -> Result<(), C6CheckedValueReadError> {
    if let Operand::Variable(id) = op {
        for seg in RESERVED_SEGMENTS {
            if id.0.starts_with(&format!("%{}{}", scope, seg)) {
                return Err(C6CheckedValueReadError::ReservedOperand {
                    identifier: id.0.clone(),
                });
            }
        }
    }
    Ok(())
}

/// Caller reference and operand tables. `ref_kind_is_data` is the
/// `RefC` space bit (0 = Const, 1 = Data); `ref_index` is the raw
/// index lane in that space. Both are circuit operands, never host
/// hints: membership and row binding are emitted constraints.
#[derive(Clone, Debug)]
pub struct C6CheckedValueReadInputs {
    pub gate: Operand,
    pub ref_kind_is_data: Operand,
    pub ref_index: Operand,
    pub pre_prog: Vec<Operand>,
    pub pre_data: Vec<Operand>,
    pub caps: C6StateCaps,
}

/// Exported, gate-normalised projections of the one selected row.
/// Kind codes: 0 UInt, 1 Bool, 2 Nil, 3 Cons, 4 Leaf, 5 Branch,
/// 6 Constructor, 7 Marker (Marker is Const-only). See the module
/// docs for the present/zero inactive convention of every field.
#[derive(Clone, Debug)]
pub struct C6CheckedValueReadBlock {
    pub instructions: Vec<Instruction>,
    pub selected_found: Operand,
    pub is_data: Operand,
    pub kind_code: Operand,
    pub is_uint: Operand,
    pub is_bool: Operand,
    pub is_nil: Operand,
    pub is_cons: Operand,
    pub is_leaf: Operand,
    pub is_branch: Operand,
    pub is_constructor: Operand,
    pub is_marker: Operand,
    pub marker_free_present: Operand,
    pub marker_free: Operand,
    pub uint_present: Operand,
    pub uint_value: Operand,
    pub bool_present: Operand,
    pub bool_value: Operand,
    pub ctor_present: Operand,
    pub type_id: Operand,
    pub ctor_id: Operand,
    pub slice_present: Operand,
    pub slice_start: Operand,
    pub slice_len: Operand,
}

/// The exact view ScalarTag / ScalarArithmetic / ScalarConstruct
/// consume from Core `kind_of` / `u128_of` / `bool_of`: kind code,
/// the UInt payload (defined only when `uint_present == 1`), the Bool
/// payload (only when `bool_present == 1`), the Marker bit, and the
/// Constructor ids (only when `ctor_present == 1`).
#[derive(Clone, Debug)]
pub struct C6ScalarOperandView {
    pub kind_code: Operand,
    pub is_marker: Operand,
    pub uint_present: Operand,
    pub uint_value: Operand,
    pub bool_present: Operand,
    pub bool_value: Operand,
    pub ctor_present: Operand,
    pub type_id: Operand,
    pub ctor_id: Operand,
    pub slice_present: Operand,
    pub slice_start: Operand,
    pub slice_len: Operand,
}

pub fn scalar_operand_view(block: &C6CheckedValueReadBlock) -> C6ScalarOperandView {
    C6ScalarOperandView {
        kind_code: block.kind_code.clone(),
        is_marker: block.is_marker.clone(),
        uint_present: block.uint_present.clone(),
        uint_value: block.uint_value.clone(),
        bool_present: block.bool_present.clone(),
        bool_value: block.bool_value.clone(),
        ctor_present: block.ctor_present.clone(),
        type_id: block.type_id.clone(),
        ctor_id: block.ctor_id.clone(),
        slice_present: block.slice_present.clone(),
        slice_start: block.slice_start.clone(),
        slice_len: block.slice_len.clone(),
    }
}

struct Em<'a> {
    scope: &'a str,
    next: usize,
    ins: Vec<Instruction>,
}
impl<'a> Em<'a> {
    fn imm(v: u128) -> Operand {
        Operand::Immediate(Fr::from(v))
    }
    fn t(&mut self) -> Identifier {
        let id = Identifier(format!("%{}.local.temp.{}", self.scope, self.next));
        self.next += 1;
        id
    }
    fn cbool(&mut self, v: Operand) {
        self.ins.push(Instruction::ConstrainToBoolean { val: v });
    }
    fn cbits(&mut self, v: Operand) {
        self.ins.push(Instruction::ConstrainBits {
            val: v,
            bits: LANE_BITS,
        });
    }
    fn teq(&mut self, a: Operand, b: Operand) -> Operand {
        let o = self.t();
        self.ins.push(Instruction::TestEq {
            a,
            b,
            output: o.clone(),
        });
        Operand::Variable(o)
    }
    fn lt128(&mut self, a: Operand, b: Operand) -> Operand {
        let o = self.t();
        self.ins.push(Instruction::LessThan {
            a,
            b,
            bits: LANE_BITS,
            output: o.clone(),
        });
        Operand::Variable(o)
    }
    fn add(&mut self, a: Operand, b: Operand) -> Operand {
        let o = self.t();
        self.ins.push(Instruction::Add {
            a,
            b,
            output: o.clone(),
        });
        Operand::Variable(o)
    }
    fn mul(&mut self, a: Operand, b: Operand) -> Operand {
        let o = self.t();
        self.ins.push(Instruction::Mul {
            a,
            b,
            output: o.clone(),
        });
        Operand::Variable(o)
    }
    fn not(&mut self, a: Operand) -> Operand {
        let o = self.t();
        self.ins.push(Instruction::Not {
            a,
            output: o.clone(),
        });
        Operand::Variable(o)
    }
    fn sel(&mut self, bit: Operand, a: Operand, b: Operand) -> Operand {
        let o = self.t();
        self.ins.push(Instruction::CondSelect {
            bit,
            a,
            b,
            output: o.clone(),
        });
        Operand::Variable(o)
    }
    fn assert_t(&mut self, c: Operand) {
        self.ins.push(Instruction::Assert { cond: c });
    }
    fn gassert(&mut self, g: Operand, c: Operand) {
        let e = self.sel(g, c, Self::imm(1));
        self.assert_t(e);
    }
    fn and(&mut self, a: Operand, b: Operand) -> Operand {
        self.mul(a, b)
    }
    fn or(&mut self, a: Operand, b: Operand) -> Operand {
        let na = self.not(a);
        let nb = self.not(b);
        let both = self.mul(na, nb);
        self.not(both)
    }
}

/// Retained-serial composer: one emitter owns the parent
/// `%{scope}.local.temp.*` serial and one retained serial inside each
/// composed canonical emitter, so repeated `emit` calls under one
/// scope never rebind a temporary. The free function is single-shot
/// per scope per circuit.
pub struct C6CheckedValueReadEmitter {
    scope: String,
    next_temp: usize,
    progctx_em: C6ProgramContextCanonicalEmitter,
    data_em: C6DataCanonicalEmitter,
}

impl C6CheckedValueReadEmitter {
    pub fn new(scope: &str) -> Result<Self, C6CheckedValueReadError> {
        validate_scope(scope)?;
        Ok(Self {
            scope: scope.to_string(),
            next_temp: 0,
            progctx_em: C6ProgramContextCanonicalEmitter::new(scope).map_err(be)?,
            data_em: C6DataCanonicalEmitter::new(scope).map_err(be)?,
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
        inputs: &C6CheckedValueReadInputs,
    ) -> Result<C6CheckedValueReadBlock, C6CheckedValueReadError> {
        let scope: &str = &self.scope;
        validate_scope(scope)?;
        if (LANE_BITS as usize) >= FR_BITS {
            return Err(C6CheckedValueReadError::InvalidBitWidth { bits: LANE_BITS });
        }
        reject_reserved(scope, &inputs.gate)?;
        reject_reserved(scope, &inputs.ref_kind_is_data)?;
        reject_reserved(scope, &inputs.ref_index)?;
        for op in inputs.pre_prog.iter().chain(inputs.pre_data.iter()) {
            reject_reserved(scope, op)?;
        }
        let caps = inputs.caps;
        let p = &caps.phys;
        for cap in [
            p.max_constants,
            p.max_data_nodes,
            p.max_types,
            p.max_constructors,
            p.max_arity,
            p.max_expressions,
            caps.max_data_field_slots,
        ] {
            if cap == usize::MAX {
                return Err(C6CheckedValueReadError::GeometryOverflow { section: "caps" });
            }
        }
        let geom = c6_data_geometry(&caps).map_err(be)?;

        let mut em = Em {
            scope,
            next: self.next_temp,
            ins: Vec::new(),
        };
        let en = inputs.gate.clone();
        let is_data = inputs.ref_kind_is_data.clone();
        let index = inputs.ref_index.clone();
        em.cbool(en.clone());
        em.cbool(is_data.clone());
        em.cbits(index.clone());
        for lane in inputs.pre_prog.iter().chain(inputs.pre_data.iter()) {
            em.cbits(lane.clone());
        }
        let is_const = em.not(is_data.clone());

        // PROG: full parser walk with constant-row capture. This walk
        // is itself the codec canonicality for PROG (gated, ending at
        // PH_END); the captured count is the parsed constants count.
        let capture = self
            .progctx_em
            .emit_prog_const_capture(&C6ProgramContextConstCaptureInputs {
                gate: en.clone(),
                pre_prog: inputs.pre_prog.clone(),
                caps,
                selected_index: index.clone(),
            })
            .map_err(be)?;
        em.ins.extend(capture.instructions);

        // DATA: padded-carrier canonicality; binds pre_data[3] as the
        // active count against cap, rows and zero inactive padding.
        let data_block = self
            .data_em
            .emit(&C6DataCanonicalInputs {
                gate: en.clone(),
                data: inputs.pre_data.clone(),
                caps,
            })
            .map_err(be)?;
        em.ins.extend(data_block.instructions);

        // Membership, strict and gated per space.
        let lt_const = em.lt128(index.clone(), capture.const_count.clone());
        let const_gate = em.and(en.clone(), is_const.clone());
        em.gassert(const_gate, lt_const.clone());
        let active_data = inputs.pre_data[3].clone();
        let lt_data = em.lt128(index.clone(), active_data);
        let data_gate = em.and(en.clone(), is_data.clone());
        em.gassert(data_gate, lt_data.clone());
        let found_any = em.sel(is_data.clone(), lt_data, capture.selected_found.clone());

        // DATA row selection: exact slot mux over the padded geometry.
        let mut drow: [Operand; DATA_ROW_LANES] =
            [Em::imm(0), Em::imm(0), Em::imm(0), Em::imm(0), Em::imm(0)];
        for slot in 0..caps.phys.max_data_nodes {
            let base = geom.data_base + slot * DATA_ROW_LANES;
            let hit = em.teq(index.clone(), Em::imm(slot as u128));
            for off in 0..DATA_ROW_LANES {
                drow[off] = em.sel(
                    hit.clone(),
                    inputs.pre_data[base + off].clone(),
                    drow[off].clone(),
                );
            }
        }

        // Unified raw row views. Const row lanes:
        // [tag, marker_free, c2, c3, c4, c5]; DATA row lanes:
        // [tag, payloadA, payloadB, slice_start, slice_len], where
        // payloadA is n / b / type_id and payloadB is ctor_id.
        let crow = &capture.row;
        let raw_tag = em.sel(is_data.clone(), drow[0].clone(), crow[0].clone());
        let payload_a = em.sel(is_data.clone(), drow[1].clone(), crow[2].clone());
        let payload_b = em.sel(is_data.clone(), drow[2].clone(), crow[3].clone());
        let kind_code_raw = raw_tag;
        let is_uint = em.teq(kind_code_raw.clone(), Em::imm(0));
        let is_bool = em.teq(kind_code_raw.clone(), Em::imm(1));
        let is_nil = em.teq(kind_code_raw.clone(), Em::imm(2));
        let is_cons = em.teq(kind_code_raw.clone(), Em::imm(3));
        let is_leaf = em.teq(kind_code_raw.clone(), Em::imm(4));
        let is_branch = em.teq(kind_code_raw.clone(), Em::imm(5));
        let is_constructor = em.teq(kind_code_raw.clone(), Em::imm(6));
        let is_marker = em.teq(kind_code_raw.clone(), Em::imm(7));
        // Const constructor slice lives at c4/c5, other Const slices
        // at c2/c3; DATA slice always at lanes 3/4.
        let const_slice_start = em.sel(is_constructor.clone(), crow[4].clone(), crow[2].clone());
        let const_slice_len = em.sel(is_constructor.clone(), crow[5].clone(), crow[3].clone());
        let slice_start_raw = em.sel(is_data.clone(), drow[3].clone(), const_slice_start);
        let slice_len_raw = em.sel(is_data.clone(), drow[4].clone(), const_slice_len);
        let slice_present_raw = {
            let a = em.or(is_cons.clone(), is_leaf.clone());
            let b = em.or(a, is_branch.clone());
            em.or(b, is_constructor.clone())
        };

        // Present-zero inactive convention, then gate-zero convention.
        let proj = |em: &mut Em, present: Operand, raw: Operand| -> Operand {
            let v = em.sel(present, raw, Em::imm(0));
            em.sel(en.clone(), v, Em::imm(0))
        };
        let gate_bit =
            |em: &mut Em, raw: Operand| -> Operand { em.sel(en.clone(), raw, Em::imm(0)) };
        let uint_value = proj(&mut em, is_uint.clone(), payload_a.clone());
        let bool_value = proj(&mut em, is_bool.clone(), payload_a.clone());
        let type_id = proj(&mut em, is_constructor.clone(), payload_a);
        let ctor_id = proj(&mut em, is_constructor.clone(), payload_b);
        let slice_start = proj(&mut em, slice_present_raw.clone(), slice_start_raw);
        let slice_len = proj(&mut em, slice_present_raw.clone(), slice_len_raw);
        let marker_free = proj(&mut em, is_const.clone(), crow[1].clone());

        let block = C6CheckedValueReadBlock {
            instructions: Vec::new(),
            selected_found: gate_bit(&mut em, found_any),
            is_data: gate_bit(&mut em, is_data.clone()),
            kind_code: gate_bit(&mut em, kind_code_raw),
            is_uint: gate_bit(&mut em, is_uint.clone()),
            is_bool: gate_bit(&mut em, is_bool.clone()),
            is_nil: gate_bit(&mut em, is_nil.clone()),
            is_cons: gate_bit(&mut em, is_cons.clone()),
            is_leaf: gate_bit(&mut em, is_leaf.clone()),
            is_branch: gate_bit(&mut em, is_branch.clone()),
            is_constructor: gate_bit(&mut em, is_constructor.clone()),
            is_marker: gate_bit(&mut em, is_marker.clone()),
            marker_free_present: gate_bit(&mut em, is_const),
            marker_free,
            uint_present: gate_bit(&mut em, is_uint),
            uint_value,
            bool_present: gate_bit(&mut em, is_bool),
            bool_value,
            ctor_present: gate_bit(&mut em, is_constructor),
            type_id,
            ctor_id,
            slice_present: gate_bit(&mut em, slice_present_raw),
            slice_start,
            slice_len,
        };
        self.next_temp = em.next;
        Ok(C6CheckedValueReadBlock {
            instructions: em.ins,
            ..block
        })
    }
}

/// Single-shot convenience form at parent serial zero; see
/// `C6CheckedValueReadEmitter` for the composition contract.
pub fn emit_c6_checked_value_read_block(
    inputs: &C6CheckedValueReadInputs,
    scope: &str,
) -> Result<C6CheckedValueReadBlock, C6CheckedValueReadError> {
    C6CheckedValueReadEmitter::new(scope)?.emit(inputs)
}
