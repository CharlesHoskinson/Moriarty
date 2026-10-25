//! Shared instruction-emitter kit for C6 step fragments.
//!
//! The helpers are the exact gate conventions of the existing C6 fragments
//! (boolean gate, unconditional 128-bit lanes, `geq` gated equality, the
//! full Core `frame_caps` frame validator), lifted verbatim from
//! `c6_return_scope_step` so new fragments share one copy. Temporaries live in
//! the `<scope>.c6kit.temp.` namespace.
use midnight_transient_crypto::curve::Fr;
use midnight_zkir_v3::ir::Operand;
use midnight_zkir_v3::{Identifier, Instruction};
use mori_core::sherlock::helper_machine::c6_phys::{C6PhysCaps, FRAME_LANES};

pub(super) const LANE_BITS: u32 = 128;

pub(super) struct Em<'a> {
    pub(super) scope: &'a str,
    pub(super) next: usize,
    pub(super) ins: Vec<Instruction>,
}
impl<'a> Em<'a> {
    pub(super) fn imm(v: u128) -> Operand {
        Operand::Immediate(Fr::from(v))
    }
    pub(super) fn t(&mut self) -> Identifier {
        let id = Identifier(format!("%{}.c6kit.temp.{}", self.scope, self.next));
        self.next += 1;
        id
    }
    pub(super) fn cbool(&mut self, v: Operand) {
        self.ins.push(Instruction::ConstrainToBoolean { val: v });
    }
    pub(super) fn cbits(&mut self, v: Operand) {
        self.ins.push(Instruction::ConstrainBits {
            val: v,
            bits: LANE_BITS,
        });
    }
    pub(super) fn ceq(&mut self, a: Operand, b: Operand) {
        self.ins.push(Instruction::ConstrainEq { a, b });
    }
    pub(super) fn teq(&mut self, a: Operand, b: Operand) -> Operand {
        let o = self.t();
        self.ins.push(Instruction::TestEq {
            a,
            b,
            output: o.clone(),
        });
        Operand::Variable(o)
    }
    pub(super) fn lt128(&mut self, a: Operand, b: Operand) -> Operand {
        let o = self.t();
        self.ins.push(Instruction::LessThan {
            a,
            b,
            bits: LANE_BITS,
            output: o.clone(),
        });
        Operand::Variable(o)
    }
    pub(super) fn add(&mut self, a: Operand, b: Operand) -> Operand {
        let o = self.t();
        self.ins.push(Instruction::Add {
            a,
            b,
            output: o.clone(),
        });
        Operand::Variable(o)
    }
    pub(super) fn mul(&mut self, a: Operand, b: Operand) -> Operand {
        let o = self.t();
        self.ins.push(Instruction::Mul {
            a,
            b,
            output: o.clone(),
        });
        Operand::Variable(o)
    }
    pub(super) fn not(&mut self, a: Operand) -> Operand {
        let o = self.t();
        self.ins.push(Instruction::Not {
            a,
            output: o.clone(),
        });
        Operand::Variable(o)
    }
    pub(super) fn sel(&mut self, bit: Operand, a: Operand, b: Operand) -> Operand {
        let o = self.t();
        self.ins.push(Instruction::CondSelect {
            bit,
            a,
            b,
            output: o.clone(),
        });
        Operand::Variable(o)
    }
    pub(super) fn assert_t(&mut self, c: Operand) {
        self.ins.push(Instruction::Assert { cond: c });
    }
    pub(super) fn geq(&mut self, g: Operand, a: Operand, b: Operand) {
        let s = self.sel(g, a, b.clone());
        self.ceq(s, b);
    }
    pub(super) fn gassert(&mut self, g: Operand, c: Operand) {
        let e = self.sel(g, c, Self::imm(1));
        self.assert_t(e);
    }
    pub(super) fn gzero(&mut self, g: Operand, v: Operand) {
        self.geq(g, v, Self::imm(0));
    }
    pub(super) fn and(&mut self, a: Operand, b: Operand) -> Operand {
        self.mul(a, b)
    }
    /// Strict index bound: `value < cap`.
    pub(super) fn gated_lt_index(&mut self, gate: Operand, val: Operand, cap: usize) {
        let lt = self.lt128(val, Self::imm(cap as u128));
        self.gassert(gate, lt);
    }
    /// Inclusive count/length/end bound: `value < cap + 1`, checked host-side.
    pub(super) fn gated_lt_count(&mut self, gate: Operand, val: Operand, cap: usize) {
        debug_assert!(cap < usize::MAX);
        let lt = self.lt128(val, Self::imm((cap as u128) + 1));
        self.gassert(gate, lt);
    }
    pub(super) fn gated_boolean(&mut self, gate: Operand, val: Operand) {
        let checked = self.sel(gate, val, Self::imm(0));
        self.cbool(checked);
    }
    /// `EnvRootC` lanes: present is boolean; absent => index 0; present => index < env cap.
    pub(super) fn gated_validate_env(
        &mut self,
        gate: Operand,
        present: Operand,
        index: Operand,
        env_cap: usize,
    ) {
        self.gated_boolean(gate.clone(), present.clone());
        let is_absent = self.teq(present.clone(), Self::imm(0));
        let absent_gate = self.mul(gate.clone(), is_absent);
        self.geq(absent_gate, index.clone(), Self::imm(0));
        let is_present = self.teq(present, Self::imm(1));
        let present_gate = self.mul(gate, is_present);
        self.gated_lt_index(present_gate, index, env_cap);
    }
    pub(super) fn gated_validate_ref(
        &mut self,
        gate: Operand,
        lanes: &[Operand],
        at: usize,
        caps: &C6PhysCaps,
    ) {
        self.gated_boolean(gate.clone(), lanes[at].clone());
        let is_data = self.teq(lanes[at].clone(), Self::imm(1));
        let cap = self.sel(
            is_data,
            Self::imm(caps.max_data_nodes as u128),
            Self::imm(caps.max_constants as u128),
        );
        let lt = self.lt128(lanes[at + 1].clone(), cap);
        self.gassert(gate, lt);
    }
    pub(super) fn less_than_129(&mut self, a: Operand, b: Operand) -> Operand {
        let out = self.t();
        self.ins.push(Instruction::LessThan {
            a,
            b,
            bits: 129,
            output: out.clone(),
        });
        Operand::Variable(out)
    }
    /// Inclusive slice-end bound: `end < table_cap + 1`, where `end`
    /// is the field sum of two unconditional 128-bit lanes and
    /// therefore lies in `[0, 2^129 - 2]`, below the BLS12-381
    /// modulus. The 129-bit comparator is exact; a 128-bit
    /// comparator on this derived sum would be out of width on
    /// gate-0, rejecting, and non-matching paths.
    pub(super) fn gated_lt_end(&mut self, gate: Operand, end: Operand, table_cap: usize) {
        debug_assert!(table_cap < usize::MAX);
        let lt = self.less_than_129(end, Self::imm((table_cap as u128) + 1));
        self.gassert(gate, lt);
    }
    pub(super) fn gated_validate_slice(
        &mut self,
        gate: Operand,
        lanes: &[Operand],
        at: usize,
        table_cap: usize,
        max_len: Option<usize>,
    ) {
        if let Some(max_len) = max_len {
            self.gated_lt_count(gate.clone(), lanes[at + 1].clone(), max_len);
        }
        let end = self.add(lanes[at].clone(), lanes[at + 1].clone());
        self.gated_lt_end(gate, end, table_cap);
    }
    /// Full structural validation of one 13-lane `FrameRowC` under `gate`,
    /// mirroring Core `frame_caps` field by field, including the builtin
    /// TypeId cap `max(max_types, 4)` and per-variant zero padding.
    #[allow(clippy::needless_range_loop)]
    pub(super) fn gated_validate_frame(
        &mut self,
        gate: Operand,
        frame: &[Operand],
        caps: &C6PhysCaps,
    ) {
        debug_assert_eq!(frame.len(), FRAME_LANES);
        let builtin_types = caps.max_types.max(4);
        let mut tag_bits = Vec::with_capacity(10);
        let mut tag_sum = Self::imm(0);
        for tag in 0..10 {
            let is_tag = self.teq(frame[0].clone(), Self::imm(tag as u128));
            tag_sum = self.add(tag_sum, is_tag.clone());
            tag_bits.push(is_tag);
        }
        self.geq(gate.clone(), tag_sum, Self::imm(1));
        // 0 LetF
        let gt = self.mul(gate.clone(), tag_bits[0].clone());
        self.gated_lt_index(gt.clone(), frame[1].clone(), caps.max_names);
        self.gated_lt_index(gt.clone(), frame[2].clone(), caps.max_expressions);
        self.gated_validate_env(
            gt.clone(),
            frame[3].clone(),
            frame[4].clone(),
            caps.max_env_cells,
        );
        self.gated_lt_index(gt, frame[5].clone(), caps.max_expressions);
        // 1 LeftF
        let gt = self.mul(gate.clone(), tag_bits[1].clone());
        self.gated_lt_index(gt.clone(), frame[1].clone(), 7);
        self.gated_lt_index(gt.clone(), frame[2].clone(), caps.max_expressions);
        self.gated_validate_env(
            gt.clone(),
            frame[3].clone(),
            frame[4].clone(),
            caps.max_env_cells,
        );
        self.gated_lt_index(gt, frame[5].clone(), caps.max_expressions);
        // 2 RightF
        let gt = self.mul(gate.clone(), tag_bits[2].clone());
        self.gated_lt_index(gt.clone(), frame[1].clone(), 7);
        self.gated_validate_ref(gt.clone(), frame, 2, caps);
        self.gated_lt_index(gt, frame[4].clone(), caps.max_expressions);
        // 3 UnaryF
        let gt = self.mul(gate.clone(), tag_bits[3].clone());
        self.gated_lt_index(gt.clone(), frame[1].clone(), 2);
        self.gated_lt_index(gt, frame[2].clone(), caps.max_expressions);
        // 4 IfF
        let gt = self.mul(gate.clone(), tag_bits[4].clone());
        self.gated_lt_index(gt.clone(), frame[1].clone(), caps.max_expressions);
        self.gated_lt_index(gt.clone(), frame[2].clone(), caps.max_expressions);
        self.gated_validate_env(
            gt.clone(),
            frame[3].clone(),
            frame[4].clone(),
            caps.max_env_cells,
        );
        self.gated_lt_index(gt, frame[5].clone(), caps.max_expressions);
        // 5 MatchF
        let gt = self.mul(gate.clone(), tag_bits[5].clone());
        self.gated_validate_slice(gt.clone(), frame, 1, caps.max_arms, None);
        self.gated_validate_env(
            gt.clone(),
            frame[3].clone(),
            frame[4].clone(),
            caps.max_env_cells,
        );
        self.gated_lt_index(gt, frame[5].clone(), caps.max_expressions);
        // 6 ArgsF
        let gt = self.mul(gate.clone(), tag_bits[6].clone());
        self.gated_lt_index(gt.clone(), frame[1].clone(), caps.max_names);
        self.gated_validate_slice(
            gt.clone(),
            frame,
            2,
            caps.max_value_slots,
            Some(caps.max_arity),
        );
        self.gated_validate_slice(
            gt.clone(),
            frame,
            4,
            caps.max_children,
            Some(caps.max_arity),
        );
        self.gated_validate_env(
            gt.clone(),
            frame[6].clone(),
            frame[7].clone(),
            caps.max_env_cells,
        );
        self.gated_lt_index(gt, frame[8].clone(), caps.max_expressions);
        // 7 ReturnF (builtin TypeId cap)
        let gt = self.mul(gate.clone(), tag_bits[7].clone());
        self.gated_lt_index(gt.clone(), frame[1].clone(), builtin_types);
        self.gated_validate_env(
            gt.clone(),
            frame[2].clone(),
            frame[3].clone(),
            caps.max_env_cells,
        );
        self.gated_lt_index(gt, frame[4].clone(), caps.max_expressions);
        // 8 ScopeF
        let gt = self.mul(gate.clone(), tag_bits[8].clone());
        self.gated_validate_env(
            gt.clone(),
            frame[1].clone(),
            frame[2].clone(),
            caps.max_env_cells,
        );
        self.gated_lt_index(gt, frame[3].clone(), caps.max_expressions);
        // 9 ConstructorFieldsF (builtin TypeId caps)
        let gt = self.mul(gate.clone(), tag_bits[9].clone());
        self.gated_lt_index(gt.clone(), frame[1].clone(), builtin_types);
        self.gated_lt_index(gt.clone(), frame[2].clone(), caps.max_constructors);
        self.gated_lt_index(gt.clone(), frame[3].clone(), caps.max_arity);
        self.gated_lt_index(gt.clone(), frame[4].clone(), builtin_types);
        self.gated_boolean(gt.clone(), frame[5].clone());
        self.gated_validate_slice(
            gt.clone(),
            frame,
            6,
            caps.max_value_slots,
            Some(caps.max_arity),
        );
        self.gated_validate_slice(
            gt.clone(),
            frame,
            8,
            caps.max_children,
            Some(caps.max_arity),
        );
        self.gated_validate_env(
            gt.clone(),
            frame[10].clone(),
            frame[11].clone(),
            caps.max_env_cells,
        );
        self.gated_lt_index(gt, frame[12].clone(), caps.max_expressions);
        // Zero padding of unused lanes per variant.
        const USED: [usize; 10] = [6, 6, 5, 3, 6, 6, 9, 5, 4, 13];
        for lane in 1..FRAME_LANES {
            let mut must_zero = Self::imm(0);
            let mut any = false;
            for (tag, used) in USED.iter().enumerate() {
                if lane >= *used {
                    must_zero = self.add(must_zero, tag_bits[tag].clone());
                    any = true;
                }
            }
            if any {
                let pad_gate = self.mul(gate.clone(), must_zero);
                self.geq(pad_gate, frame[lane].clone(), Self::imm(0));
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Shared step frame: the conventions of `c6_return_left_step::emit`, factored
// so that a fragment adds only its selection (composed dispatch owner) and its
// rule effect.
// ---------------------------------------------------------------------------

use midnight_transient_crypto::curve::FR_BITS;
use mori_core::sherlock::helper_machine::c6_phys::{
    C6_PHYS_MAGIC, C6_PHYS_VERSION, CONTROL_LANES, IMAGE_SECTION_HEADER_LANES, L_ACTIVE, L_ALLOC,
    L_CONSUMED_WORK, L_CONTROL, L_CONTROL_ORIGIN, L_ENV_ROOT, L_FRAME_COUNT, L_MAGIC,
    L_REFERENCE_CURSOR, L_REMAINING_WORK, L_SEG_ACTIVE, L_SEG_ALLOC, L_STACK_LIMIT, L_VERSION,
};
use mori_core::sherlock::helper_machine::c6_phys_host::{C6_HOST_SCHEMA, CTX_LANES};

use super::c6_data_canonical::{C6DataCanonicalEmitter, C6DataCanonicalInputs};
use super::c6_dispatch_step::{C6DispatchEmitter, C6DispatchInputs};
use super::c6_helper_canonical::{C6HelperCanonicalEmitter, C6HelperCanonicalInputs};
use super::c6_meta_canonical::{C6MetaBoundCanonicalInputs, C6MetaCanonicalEmitter};
use super::c6_program_context_canonical::{
    C6ProgramContextCanonicalEmitter, C6ProgramContextCanonicalInputs,
    C6ProgramContextCtxBindingInputs,
};
use super::c6_push_step::{C6_PADDED_CARRIER_VERSION, C6PaddedCarrierGeometry, C6PushFamilyParams};
use mori_core::sherlock::helper_machine::shared::RuleIdC;

/// The common C6 step statement carriers (same fields as every C6 fragment).
#[derive(Clone, Debug)]
pub struct C6StepInputs {
    pub enable: Operand,
    pub host_schema: Operand,
    pub carrier_version: Operand,
    pub geometry: C6PaddedCarrierGeometry,
    pub meta_bounds_frames_offset: usize,
    pub source_params: C6PushFamilyParams,
    pub pre_meta: Vec<Operand>,
    pub pre_prog: Vec<Operand>,
    pub pre_ctx: [Operand; CTX_LANES],
    pub pre_image: Vec<Operand>,
    pub pre_data: Vec<Operand>,
    pub pre_helper: Vec<Operand>,
    pub post_meta: Vec<Operand>,
    pub post_prog: Vec<Operand>,
    pub post_ctx: [Operand; CTX_LANES],
    pub post_image: Vec<Operand>,
    pub post_data: Vec<Operand>,
    pub post_helper: Vec<Operand>,
}

/// The composed emitters one step fragment retains (one serial namespace each).
pub(super) struct Composed {
    pub(super) data_em: C6DataCanonicalEmitter,
    pub(super) meta_em: C6MetaCanonicalEmitter,
    pub(super) progctx_em: C6ProgramContextCanonicalEmitter,
    pub(super) helper_em: C6HelperCanonicalEmitter,
    pub(super) dispatch_em: C6DispatchEmitter,
}

fn bk<E: std::fmt::Display>(e: E) -> String {
    e.to_string()
}

impl Composed {
    pub(super) fn new(scope: &str) -> Result<Self, String> {
        Ok(Self {
            data_em: C6DataCanonicalEmitter::new(scope).map_err(bk)?,
            meta_em: C6MetaCanonicalEmitter::new(scope).map_err(bk)?,
            progctx_em: C6ProgramContextCanonicalEmitter::new(scope).map_err(bk)?,
            helper_em: C6HelperCanonicalEmitter::new(scope).map_err(bk)?,
            dispatch_em: C6DispatchEmitter::new(scope).map_err(bk)?,
        })
    }
}

pub(super) fn check_shapes(inputs: &C6StepInputs) -> Result<(), String> {
    if 129usize >= FR_BITS {
        return Err("129-bit comparator exceeds the field".into());
    }
    inputs
        .source_params
        .validate_geometry(inputs.geometry)
        .map_err(bk)?;
    let g = &inputs.geometry;
    for (s, a, e) in [
        ("pre image", inputs.pre_image.len(), g.image_total_lanes),
        ("post image", inputs.post_image.len(), g.image_total_lanes),
        ("pre data", inputs.pre_data.len(), g.data_total_lanes),
        ("post data", inputs.post_data.len(), g.data_total_lanes),
        ("pre helper", inputs.pre_helper.len(), g.helper_total_lanes),
        (
            "post helper",
            inputs.post_helper.len(),
            g.helper_total_lanes,
        ),
    ] {
        if a != e {
            return Err(format!("length mismatch in {s}: expected {e}, got {a}"));
        }
    }
    if inputs.pre_meta.len() < 3
        || inputs.pre_meta.len() != inputs.post_meta.len()
        || inputs.pre_prog.len() != inputs.post_prog.len()
        || inputs.meta_bounds_frames_offset >= inputs.pre_meta.len()
    {
        return Err("host prefix length/offset".into());
    }
    Ok(())
}

/// Fixed schema/carrier, unconditional prefix equality and lane ranges.
pub(super) fn prefix(em: &mut Em, inputs: &C6StepInputs) {
    let en = inputs.enable.clone();
    em.cbool(en.clone());
    em.ceq(inputs.host_schema.clone(), Em::imm(C6_HOST_SCHEMA));
    em.ceq(
        inputs.carrier_version.clone(),
        Em::imm(C6_PADDED_CARRIER_VERSION),
    );
    for (a, b) in inputs
        .pre_meta
        .iter()
        .zip(inputs.post_meta.iter())
        .chain(inputs.pre_prog.iter().zip(inputs.post_prog.iter()))
        .chain(inputs.pre_ctx.iter().zip(inputs.post_ctx.iter()))
    {
        em.cbits(a.clone());
        em.cbits(b.clone());
        em.ceq(b.clone(), a.clone());
    }
    em.geq(
        en.clone(),
        inputs.pre_meta[0].clone(),
        Em::imm(C6_PHYS_MAGIC),
    );
    em.geq(
        en.clone(),
        inputs.pre_meta[1].clone(),
        Em::imm(C6_PHYS_VERSION),
    );
    em.geq(
        en.clone(),
        inputs.pre_ctx[0].clone(),
        Em::imm(C6_PHYS_MAGIC),
    );
    em.geq(
        en.clone(),
        inputs.pre_ctx[1].clone(),
        Em::imm(C6_PHYS_VERSION),
    );
    for l in inputs
        .pre_image
        .iter()
        .chain(inputs.post_image.iter())
        .chain(inputs.pre_data.iter())
        .chain(inputs.post_data.iter())
        .chain(inputs.pre_helper.iter())
        .chain(inputs.post_helper.iter())
    {
        em.cbits(l.clone());
    }
}

/// Pre root representation checks (return_left conventions) and the full
/// validation of every active frame; returns the top frame lanes.
pub(super) fn validate_pre_image(em: &mut Em, inputs: &C6StepInputs) -> Vec<Operand> {
    let en = inputs.enable.clone();
    let caps = &inputs.geometry.caps;
    let p = &caps.phys;
    let pre_i = &inputs.pre_image;
    em.geq(en.clone(), pre_i[L_MAGIC].clone(), Em::imm(C6_PHYS_MAGIC));
    em.geq(
        en.clone(),
        pre_i[L_VERSION].clone(),
        Em::imm(C6_PHYS_VERSION),
    );
    em.gated_lt_index(
        en.clone(),
        pre_i[L_CONTROL_ORIGIN].clone(),
        p.max_expressions,
    );
    em.gated_validate_env(
        en.clone(),
        pre_i[L_ENV_ROOT].clone(),
        pre_i[L_ENV_ROOT + 1].clone(),
        p.max_env_cells,
    );
    em.gated_lt_count(en.clone(), pre_i[L_STACK_LIMIT].clone(), p.max_frames);
    for (at, cap) in [
        (L_ALLOC, p.max_data_nodes),
        (L_ALLOC + 1, caps.max_data_field_slots),
        (L_ALLOC + 2, p.max_env_cells),
        (L_SEG_ALLOC, p.max_data_nodes),
        (L_SEG_ALLOC + 1, caps.max_data_field_slots),
        (L_SEG_ALLOC + 2, p.max_env_cells),
        (L_SEG_ACTIVE, p.max_data_nodes),
        (L_SEG_ACTIVE + 1, caps.max_data_field_slots),
        (L_SEG_ACTIVE + 2, p.max_env_cells),
        (L_SEG_ACTIVE + 3, p.max_value_slots),
    ] {
        em.gated_lt_count(en.clone(), pre_i[at].clone(), cap);
    }
    for off in 0..4 {
        em.geq(
            en.clone(),
            pre_i[L_ACTIVE + off].clone(),
            inputs.pre_data[3 + off].clone(),
        );
    }
    let depth = pre_i[L_FRAME_COUNT].clone();
    em.gated_lt_count(en.clone(), depth.clone(), p.max_frames);
    let over = em.lt128(pre_i[L_STACK_LIMIT].clone(), depth.clone());
    let fits = em.not(over);
    em.gassert(en.clone(), fits);
    let mut top = vec![Em::imm(0); FRAME_LANES];
    for slot in 0..p.max_frames {
        let base = IMAGE_SECTION_HEADER_LANES + slot * FRAME_LANES;
        let active = em.lt128(Em::imm(slot as u128), depth.clone());
        let ga = em.and(en.clone(), active.clone());
        em.gated_validate_frame(ga, &pre_i[base..base + FRAME_LANES], p);
        let inactive = em.not(active);
        let gi = em.and(en.clone(), inactive);
        let hit = em.teq(Em::imm(slot as u128 + 1), depth.clone());
        for lane in 0..FRAME_LANES {
            em.gzero(gi.clone(), pre_i[base + lane].clone());
            top[lane] = em.sel(hit.clone(), pre_i[base + lane].clone(), top[lane].clone());
        }
    }
    top
}

/// Core selection of exactly `rule` on the pre carriers (dispatch owner).
pub(super) fn select(
    composed: &mut Composed,
    em: &mut Em,
    rule: RuleIdC,
    inputs: &C6StepInputs,
) -> Result<(), String> {
    let block = composed
        .dispatch_em
        .emit(
            rule,
            &C6DispatchInputs {
                enable: inputs.enable.clone(),
                caps: inputs.geometry.caps,
                pre_prog: inputs.pre_prog.clone(),
                pre_image: inputs.pre_image.clone(),
                pre_data: inputs.pre_data.clone(),
                pre_helper: inputs.pre_helper.clone(),
            },
        )
        .map_err(bk)?;
    em.ins.extend(block.instructions);
    Ok(())
}

/// LegacyComplete charge: work 1 and reference cursor 1, checked 128-bit.
pub(super) fn charge_legacy(em: &mut Em, inputs: &C6StepInputs) {
    let en = inputs.enable.clone();
    let pre_i = &inputs.pre_image;
    let post_i = &inputs.post_image;
    let dec = em.add(post_i[L_REMAINING_WORK].clone(), Em::imm(1));
    em.geq(en.clone(), dec, pre_i[L_REMAINING_WORK].clone());
    for lane in [L_CONSUMED_WORK, L_REFERENCE_CURSOR] {
        let no_overflow = em.lt128(pre_i[lane].clone(), Em::imm(u128::MAX));
        em.gassert(en.clone(), no_overflow);
        let inc = em.add(pre_i[lane].clone(), Em::imm(1));
        em.geq(en.clone(), post_i[lane].clone(), inc);
    }
}

/// Under enable, post control lanes equal `control` exactly.
pub(super) fn set_control(em: &mut Em, inputs: &C6StepInputs, control: &[Operand]) {
    debug_assert_eq!(control.len(), CONTROL_LANES);
    for (off, want) in control.iter().enumerate() {
        em.geq(
            inputs.enable.clone(),
            inputs.post_image[L_CONTROL + off].clone(),
            want.clone(),
        );
    }
}

/// Under enable, every image header lane not in `mutable` and every frame
/// slot lane is preserved.
pub(super) fn preserve_image(
    em: &mut Em,
    inputs: &C6StepInputs,
    mutable: &[usize],
    frames_mutable: bool,
) {
    let en = inputs.enable.clone();
    for lane in 0..IMAGE_SECTION_HEADER_LANES {
        if !mutable.contains(&lane) {
            em.geq(
                en.clone(),
                inputs.post_image[lane].clone(),
                inputs.pre_image[lane].clone(),
            );
        }
    }
    if !frames_mutable {
        for lane in IMAGE_SECTION_HEADER_LANES..inputs.pre_image.len() {
            em.geq(
                en.clone(),
                inputs.post_image[lane].clone(),
                inputs.pre_image[lane].clone(),
            );
        }
    }
}

/// The header lanes a LegacyComplete control rewrite may change.
pub(super) fn legacy_control_lanes() -> Vec<usize> {
    let mut v: Vec<usize> = (L_CONTROL..L_CONTROL + CONTROL_LANES).collect();
    v.extend([L_REMAINING_WORK, L_CONSUMED_WORK, L_REFERENCE_CURSOR]);
    v
}

/// Retained-serial composition of META/PROG/CTX/DATA/HELPER canonicality.
pub(super) fn compose_canonical(
    composed: &mut Composed,
    em: &mut Em,
    inputs: &C6StepInputs,
) -> Result<(), String> {
    let en = inputs.enable.clone();
    let caps = inputs.geometry.caps;
    let mb = composed
        .meta_em
        .emit_bound(&C6MetaBoundCanonicalInputs {
            gate: en.clone(),
            pre_meta: inputs.pre_meta.clone(),
            post_meta: inputs.post_meta.clone(),
            limits: inputs.source_params.limits(),
            meta_bounds_frames_offset: inputs.meta_bounds_frames_offset,
            binding: inputs.source_params.meta_binding(),
        })
        .map_err(bk)?;
    em.ins.extend(mb.instructions);
    let pc = composed
        .progctx_em
        .emit(&C6ProgramContextCanonicalInputs {
            gate: en.clone(),
            pre_prog: inputs.pre_prog.clone(),
            pre_ctx: inputs.pre_ctx.clone(),
            caps,
        })
        .map_err(bk)?;
    em.ins.extend(pc.instructions);
    let cx = composed
        .progctx_em
        .emit_ctx_binding(&C6ProgramContextCtxBindingInputs {
            gate: en.clone(),
            pre_ctx: inputs.pre_ctx.clone(),
            expected_ctx: inputs.source_params.expected_ctx(),
        })
        .map_err(bk)?;
    em.ins.extend(cx.instructions);
    for data in [&inputs.pre_data, &inputs.post_data] {
        let db = composed
            .data_em
            .emit(&C6DataCanonicalInputs {
                gate: en.clone(),
                data: data.clone(),
                caps,
            })
            .map_err(bk)?;
        em.ins.extend(db.instructions);
    }
    for helper in [&inputs.pre_helper, &inputs.post_helper] {
        let hb = composed
            .helper_em
            .emit(&C6HelperCanonicalInputs {
                gate: en.clone(),
                helper: helper.clone(),
                caps,
            })
            .map_err(bk)?;
        em.ins.extend(hb.instructions);
    }
    Ok(())
}

/// Disabled: the full post state equals the pre state.
pub(super) fn disabled_identity(em: &mut Em, inputs: &C6StepInputs) {
    let disabled = em.not(inputs.enable.clone());
    for (a, b) in inputs
        .pre_image
        .iter()
        .zip(inputs.post_image.iter())
        .chain(inputs.pre_data.iter().zip(inputs.post_data.iter()))
        .chain(inputs.pre_helper.iter().zip(inputs.post_helper.iter()))
    {
        em.geq(disabled.clone(), b.clone(), a.clone());
    }
}

/// Applied (non-P3) helper charge: remaining work - 1, consumed work + 1,
/// helper work + 1, helper visits + `visits`; outcome and reference cursor
/// stay. The P3 helper-work refusal (`pre.helper_work == max_helper_work`) is
/// a different disposition and is rejected here, not constrained.
pub(super) fn charge_helper(em: &mut Em, inputs: &C6StepInputs, visits: u128) {
    use mori_core::sherlock::helper_machine::c6_phys::{L_HELPER_VISITS, L_HELPER_WORK};
    let en = inputs.enable.clone();
    let pre_i = &inputs.pre_image;
    let post_i = &inputs.post_image;
    let max_hw = inputs.source_params.admission_bounds().max_helper_work;
    let is_p3 = em.teq(pre_i[L_HELPER_WORK].clone(), Em::imm(max_hw));
    let not_p3 = em.not(is_p3);
    em.gassert(en.clone(), not_p3);
    let dec = em.add(post_i[L_REMAINING_WORK].clone(), Em::imm(1));
    em.geq(en.clone(), dec, pre_i[L_REMAINING_WORK].clone());
    for (lane, by) in [
        (L_CONSUMED_WORK, 1),
        (L_HELPER_WORK, 1),
        (L_HELPER_VISITS, visits),
    ] {
        if by > 0 {
            let room = em.lt128(pre_i[lane].clone(), Em::imm(u128::MAX - by + 1));
            em.gassert(en.clone(), room);
        }
        let inc = em.add(pre_i[lane].clone(), Em::imm(by));
        em.geq(en.clone(), post_i[lane].clone(), inc);
    }
}

/// The header lanes an applied helper step may change.
pub(super) fn helper_charge_lanes() -> Vec<usize> {
    use mori_core::sherlock::helper_machine::c6_phys::{L_HELPER_VISITS, L_HELPER_WORK};
    vec![
        L_REMAINING_WORK,
        L_CONSUMED_WORK,
        L_HELPER_WORK,
        L_HELPER_VISITS,
    ]
}

/// Row `index` of a fixed-width padded table (`rows` rows of `width` lanes
/// starting at `base`), by one-hot mux; also returns `index < rows`.
pub(super) fn mux_row(
    em: &mut Em,
    lanes: &[Operand],
    base: usize,
    rows: usize,
    width: usize,
    index: Operand,
) -> (Operand, Vec<Operand>) {
    let mut out = vec![Em::imm(0); width];
    let mut hit_any = Em::imm(0);
    for r in 0..rows {
        let hit = em.teq(Em::imm(r as u128), index.clone());
        hit_any = em.add(hit_any, hit.clone());
        for (c, o) in out.iter_mut().enumerate() {
            *o = em.sel(hit.clone(), lanes[base + r * width + c].clone(), o.clone());
        }
    }
    (hit_any, out)
}
