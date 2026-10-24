//! C6 `BHelperScalarFailure`: active Scalar helper at Done/PrimError.
//! Applied: P4 charges work/consumed/helper-work exactly once, helper=None,
//! result=ScalarError{exact staged code}, outcome stays Running.
//! P3: pre.helper_work == retained admission BoundsC.max_helper_work:
//! outcome=Exhausted{HelperWork} only, no charge, no other change.
//! Pure zero-fuel P2 is not this relation (distinct earlier driver branch).
//! Absolute statement order: enable, disposition_is_refused_helper_work,
//! host_schema, carrier_version, pre(META,PROG,CTX,image,data,helper),
//! post(META,PROG,CTX,image,data,helper). At enable=0 post == pre.
//! Padded lanes follow `sherlock-c6-phys/1` exactly.

use std::sync::Arc;

use midnight_transient_crypto::curve::{FR_BITS, Fr};
use midnight_transient_crypto::hash::transient_commit;
use midnight_transient_crypto::proofs::{KeyLocation, ProofPreimage, Zkir};
use midnight_zkir_v3::ir::{IrMinorVersion, Operand, TypedIdentifier};
use midnight_zkir_v3::ir_types::IrType;
use midnight_zkir_v3::{Identifier, Instruction, IrSource};
use mori_core::sherlock::helper_machine::c6_phys::{
    C6_PHYS_MAGIC, C6_PHYS_VERSION, C6PhysCaps, L_CONSUMED_WORK, L_CONTROL, L_CONTROL_ORIGIN,
    L_ENV_ROOT, L_FRAME_COUNT, L_HELPER_VISITS, L_HELPER_WORK, L_INITIAL_FUEL, L_OUTCOME,
    L_REFERENCE_CURSOR, L_REMAINING_WORK, L_STACK_LIMIT,
};
use mori_core::sherlock::helper_machine::c6_phys_data::C6StateCaps;
use mori_core::sherlock::helper_machine::c6_phys_helper::{
    HELPER_FIXED_LANES, L_HELPER_RECORD, L_RESULT_RECORD, L_TASKS,
};
use mori_core::sherlock::helper_machine::c6_phys_host::{C6HostConfig, C6HostLimits, CTX_LANES};
use mori_core::sherlock::helper_machine::c6_phys_state::{C6OwnedState, C6StateLimits};
use mori_core::sherlock::helper_machine::complete_observation::{
    CaptureStepError, CompleteStepRecord, HostSnapshot, ObservedStep, record_step,
};
use mori_core::sherlock::helper_machine::shared::{
    Admitted, BoundaryKind, EventKind, ExhaustReason, HelperControlC, HelperKind,
    HelperPhaseSummaryC, HelperResultC, HelperResultTagC, HostViolation, MachineImage, OutcomeC,
    OutcomeTagC, RuleIdC, ScalarPhase, ScalarStaged, StepDisposition,
};
use serde_json::json;
use thiserror::Error;

use super::c6_data_canonical::{C6DataCanonicalEmitter, C6DataCanonicalInputs};
use super::c6_helper_canonical::{C6HelperCanonicalEmitter, C6HelperCanonicalInputs};
use super::c6_meta_canonical::{C6MetaBoundCanonicalInputs, C6MetaCanonicalEmitter};
use super::c6_program_context_canonical::{
    C6ProgramContextCanonicalEmitter, C6ProgramContextCanonicalInputs,
    C6ProgramContextCtxBindingInputs,
};
use super::c6_push_step::{
    C6_PADDED_CARRIER_VERSION, C6PaddedCarrierGeometry, C6PushFamilyParams, C6PushHostPrefixLanes,
    C6PushStatementBindingMode, C6PushStepError,
};
use mori_core::sherlock::helper_machine::c6_phys_host::C6_HOST_SCHEMA;

const LANE_BITS: u32 = 128;
const CHECK_KEY_LOCATION: &str = "sherlock_c6_scalar_failure_step_check";

#[derive(Debug, Error)]
pub enum C6ScalarFailureStepError {
    #[error("invalid scope identifier: must not be empty")]
    EmptyScope,
    #[error("invalid scope identifier: scope must not contain '%' or a reserved composed segment")]
    InvalidScope,
    #[error("operand uses reserved parent temporary identifier: {identifier}")]
    ReservedOperand { identifier: String },
    #[error("lane bit width {bits} is invalid for the BLS12-381 scalar field")]
    InvalidBitWidth { bits: u32 },
    #[error("geometry arithmetic overflow ({section})")]
    GeometryOverflow { section: &'static str },
    #[error("length mismatch in {section}: expected {expected}, got {actual}")]
    LengthMismatch {
        section: &'static str,
        expected: usize,
        actual: usize,
    },
    #[error("host prefix section {section} mismatch between pre and post state")]
    HostPrefixMismatch { section: &'static str },
    #[error("owner step is not the selected Scalar failure step (terminal or wrong rule)")]
    UnexpectedOwnerStep,
    #[error("owner event mismatch in {field}")]
    EventMismatch { field: &'static str },
    #[error("core C6 physical/push error: {0}")]
    Push(#[from] C6PushStepError),
    #[error("composed canonical emitter or SDK backend error: {0}")]
    Backend(String),
}

fn be<E: std::fmt::Display>(e: E) -> C6ScalarFailureStepError {
    C6ScalarFailureStepError::Backend(e.to_string())
}

fn violation_label(v: &HostViolation) -> String {
    format!(
        "kind={:?}, clause={:?}, rule={:?}, locator={}",
        v.kind,
        v.clause,
        v.rule,
        v.locator.as_deref().unwrap_or("<none>")
    )
}

fn capture_step_error_label(e: &CaptureStepError) -> String {
    match e {
        CaptureStepError::BeforeObservation(v) => {
            format!("before observation: {}", violation_label(v))
        }
        CaptureStepError::Step {
            violation, after, ..
        } => match after {
            Ok(_) => format!(
                "owner step: {}; after snapshot captured",
                violation_label(violation)
            ),
            Err(after_violation) => format!(
                "owner step: {}; after observation: {}",
                violation_label(violation),
                violation_label(after_violation)
            ),
        },
        CaptureStepError::AfterObservation {
            violation, event, ..
        } => format!(
            "after observation: {}; event_present={}",
            violation_label(violation),
            event.is_some()
        ),
    }
}
const LOCAL_RESERVED_SEGMENT: &str = ".local.temp.";
const COMPOSED_RESERVED_SEGMENTS: [&str; 5] = [
    LOCAL_RESERVED_SEGMENT,
    ".c6data.temp.",
    ".c6helper.temp.",
    ".c6meta.temp.",
    ".c6progctx.temp.",
];

fn validate_parent_scope(scope: &str) -> Result<(), C6ScalarFailureStepError> {
    if scope.trim().is_empty() {
        return Err(C6ScalarFailureStepError::EmptyScope);
    }
    if scope.contains('%') || COMPOSED_RESERVED_SEGMENTS.iter().any(|s| scope.contains(s)) {
        return Err(C6ScalarFailureStepError::InvalidScope);
    }
    Ok(())
}

fn reject_reserved_operand(scope: &str, op: &Operand) -> Result<(), C6ScalarFailureStepError> {
    if let Operand::Variable(id) = op {
        for seg in COMPOSED_RESERVED_SEGMENTS {
            if id.0.starts_with(&format!("%{}{}", scope, seg)) {
                return Err(C6ScalarFailureStepError::ReservedOperand {
                    identifier: id.0.clone(),
                });
            }
        }
    }
    Ok(())
}

fn reject_all_reserved(
    scope: &str,
    inputs: &C6ScalarFailureStepInputs,
) -> Result<(), C6ScalarFailureStepError> {
    for op in [
        &inputs.enable,
        &inputs.disposition_is_refused_helper_work,
        &inputs.host_schema,
        &inputs.carrier_version,
    ] {
        reject_reserved_operand(scope, op)?;
    }
    for op in inputs
        .pre_meta
        .iter()
        .chain(inputs.post_meta.iter())
        .chain(inputs.pre_prog.iter())
        .chain(inputs.post_prog.iter())
        .chain(inputs.pre_ctx.iter())
        .chain(inputs.post_ctx.iter())
        .chain(inputs.pre_image.iter())
        .chain(inputs.post_image.iter())
        .chain(inputs.pre_data.iter())
        .chain(inputs.post_data.iter())
        .chain(inputs.pre_helper.iter())
        .chain(inputs.post_helper.iter())
    {
        reject_reserved_operand(scope, op)?;
    }
    Ok(())
}

/// Retained-serial composition API for several ScalarFailure blocks under
/// one scope in one circuit. One emitter owns the parent
/// `%{scope}.local.temp.*` serial and one retained serial for each composed
/// canonical block (DATA, META, PROGCTX, HELPER), so repeated `emit` calls
/// never rebind a temporary minted by an earlier call. The free function
/// `emit_c6_scalar_failure_step_block` is single-shot: it constructs one
/// emitter and calls it exactly once, so a given scope may be used by that
/// function at most once per circuit unless the caller can prove no other
/// block uses the same reserved prefixes. Standalone (non-composed) use
/// follows the same contract: one block per scope per circuit, or this
/// retained emitter for composition.
pub struct C6ScalarFailureStepEmitter {
    scope: String,
    next_temp: usize,
    data_em: C6DataCanonicalEmitter,
    meta_em: C6MetaCanonicalEmitter,
    progctx_em: C6ProgramContextCanonicalEmitter,
    helper_em: C6HelperCanonicalEmitter,
}

impl C6ScalarFailureStepEmitter {
    pub fn new(scope: &str) -> Result<Self, C6ScalarFailureStepError> {
        validate_parent_scope(scope)?;
        Ok(Self {
            scope: scope.to_string(),
            next_temp: 0,
            data_em: C6DataCanonicalEmitter::new(scope).map_err(be)?,
            meta_em: C6MetaCanonicalEmitter::new(scope).map_err(be)?,
            progctx_em: C6ProgramContextCanonicalEmitter::new(scope).map_err(be)?,
            helper_em: C6HelperCanonicalEmitter::new(scope).map_err(be)?,
        })
    }
    pub fn scope(&self) -> &str {
        &self.scope
    }
    pub fn next_temp(&self) -> usize {
        self.next_temp
    }
}

#[derive(Clone, Debug)]
pub struct C6ScalarFailureStepInputs {
    pub enable: Operand,
    pub disposition_is_refused_helper_work: Operand,
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

#[derive(Clone, Debug)]
pub struct C6ScalarFailureStepBlock {
    pub is_helper_work_max: Operand,
    pub instructions: Vec<Instruction>,
}

struct Em<'a> {
    scope: &'a str,
    next: usize,
    ins: Vec<Instruction>,
}
impl<'a> Em<'a> {
    fn new(scope: &'a str) -> Self {
        Self {
            scope,
            next: 0,
            ins: Vec::new(),
        }
    }
    fn t(&mut self) -> Identifier {
        let id = Identifier(format!("%{}.local.temp.{}", self.scope, self.next));
        self.next += 1;
        id
    }
    fn imm(v: u128) -> Operand {
        Operand::Immediate(Fr::from(v))
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
    fn ceq(&mut self, a: Operand, b: Operand) {
        self.ins.push(Instruction::ConstrainEq { a, b });
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
    fn lt(&mut self, a: Operand, b: Operand, bits: u32) -> Operand {
        let o = self.t();
        self.ins.push(Instruction::LessThan {
            a,
            b,
            bits,
            output: o.clone(),
        });
        Operand::Variable(o)
    }
    fn lt128(&mut self, a: Operand, b: Operand) -> Operand {
        self.lt(a, b, LANE_BITS)
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
    fn geq(&mut self, g: Operand, a: Operand, b: Operand) {
        let s = self.sel(g, a, b.clone());
        self.ceq(s, b);
    }
    fn gassert(&mut self, g: Operand, c: Operand) {
        let e = self.sel(g, c, Self::imm(1));
        self.assert_t(e);
    }
    fn gzero(&mut self, g: Operand, v: Operand) {
        self.geq(g, v, Self::imm(0));
    }
    fn gbool(&mut self, g: Operand, v: Operand) {
        let c = self.sel(g, v, Self::imm(0));
        self.cbool(c);
    }
    fn glt_i(&mut self, g: Operand, v: Operand, cap: usize) {
        let l = self.lt128(v, Self::imm(cap as u128));
        self.gassert(g, l);
    }
    fn glt_c(&mut self, g: Operand, v: Operand, cap: usize) {
        let l = self.lt128(v, Self::imm((cap as u128) + 1));
        self.gassert(g, l);
    }
    fn glt_end(&mut self, g: Operand, end: Operand, cap: usize) {
        let l = self.lt(end, Self::imm((cap as u128) + 1), 129);
        self.gassert(g, l);
    }
    fn tag_bits(&mut self, g: &Operand, lane: Operand, n: u128) -> Vec<Operand> {
        let mut bits = Vec::new();
        let mut sum = Self::imm(0);
        for t in 0..n {
            let b = self.teq(lane.clone(), Self::imm(t));
            sum = self.add(sum, b.clone());
            bits.push(b);
        }
        self.geq(g.clone(), sum, Self::imm(1));
        bits
    }
    fn vref(&mut self, g: Operand, lanes: &[Operand], at: usize, c: &C6PhysCaps) {
        self.gbool(g.clone(), lanes[at].clone());
        let d = self.teq(lanes[at].clone(), Self::imm(1));
        let cap = self.sel(
            d,
            Self::imm(c.max_data_nodes as u128),
            Self::imm(c.max_constants as u128),
        );
        let l = self.lt128(lanes[at + 1].clone(), cap);
        self.gassert(g, l);
    }
    fn venv(&mut self, g: Operand, present: Operand, idx: Operand, cap: usize) {
        self.gbool(g.clone(), present.clone());
        let a = self.teq(present.clone(), Self::imm(0));
        let ag = self.mul(g.clone(), a);
        self.gzero(ag, idx.clone());
        let p = self.teq(present, Self::imm(1));
        let pg = self.mul(g, p);
        self.glt_i(pg, idx, cap);
    }
    fn vslice(
        &mut self,
        g: Operand,
        lanes: &[Operand],
        at: usize,
        table: usize,
        max_len: Option<usize>,
    ) {
        if let Some(m) = max_len {
            self.glt_c(g.clone(), lanes[at + 1].clone(), m);
        }
        let end = self.add(lanes[at].clone(), lanes[at + 1].clone());
        self.glt_end(g, end, table);
    }
    fn vframe(&mut self, gate: Operand, f: &[Operand], c: &C6PhysCaps) {
        debug_assert_eq!(f.len(), 13);
        let bt = c.max_types.max(4);
        let bits = self.tag_bits(&gate, f[0].clone(), 10);
        let vg = |em: &mut Self, i: usize| em.mul(gate.clone(), bits[i].clone());
        {
            let g = vg(self, 0);
            self.glt_i(g.clone(), f[1].clone(), c.max_names);
            self.glt_i(g.clone(), f[2].clone(), c.max_expressions);
            self.venv(g.clone(), f[3].clone(), f[4].clone(), c.max_env_cells);
            self.glt_i(g, f[5].clone(), c.max_expressions);
        }
        {
            let g = vg(self, 1);
            self.glt_i(g.clone(), f[1].clone(), 7);
            self.glt_i(g.clone(), f[2].clone(), c.max_expressions);
            self.venv(g.clone(), f[3].clone(), f[4].clone(), c.max_env_cells);
            self.glt_i(g, f[5].clone(), c.max_expressions);
        }
        {
            let g = vg(self, 2);
            self.glt_i(g.clone(), f[1].clone(), 7);
            self.vref(g.clone(), f, 2, c);
            self.glt_i(g, f[4].clone(), c.max_expressions);
        }
        {
            let g = vg(self, 3);
            self.glt_i(g.clone(), f[1].clone(), 2);
            self.glt_i(g, f[2].clone(), c.max_expressions);
        }
        {
            let g = vg(self, 4);
            self.glt_i(g.clone(), f[1].clone(), c.max_expressions);
            self.glt_i(g.clone(), f[2].clone(), c.max_expressions);
            self.venv(g.clone(), f[3].clone(), f[4].clone(), c.max_env_cells);
            self.glt_i(g, f[5].clone(), c.max_expressions);
        }
        {
            let g = vg(self, 5);
            self.vslice(g.clone(), f, 1, c.max_arms, None);
            self.venv(g.clone(), f[3].clone(), f[4].clone(), c.max_env_cells);
            self.glt_i(g, f[5].clone(), c.max_expressions);
        }
        {
            let g = vg(self, 6);
            self.glt_i(g.clone(), f[1].clone(), c.max_names);
            self.vslice(g.clone(), f, 2, c.max_value_slots, Some(c.max_arity));
            self.vslice(g.clone(), f, 4, c.max_children, Some(c.max_arity));
            self.venv(g.clone(), f[6].clone(), f[7].clone(), c.max_env_cells);
            self.glt_i(g, f[8].clone(), c.max_expressions);
        }
        {
            let g = vg(self, 7);
            self.glt_i(g.clone(), f[1].clone(), bt);
            self.venv(g.clone(), f[2].clone(), f[3].clone(), c.max_env_cells);
            self.glt_i(g, f[4].clone(), c.max_expressions);
        }
        {
            let g = vg(self, 8);
            self.venv(g.clone(), f[1].clone(), f[2].clone(), c.max_env_cells);
            self.glt_i(g, f[3].clone(), c.max_expressions);
        }
        {
            let g = vg(self, 9);
            self.glt_i(g.clone(), f[1].clone(), bt);
            self.glt_i(g.clone(), f[2].clone(), c.max_constructors);
            self.glt_i(g.clone(), f[3].clone(), c.max_arity);
            self.glt_i(g.clone(), f[4].clone(), bt);
            self.gbool(g.clone(), f[5].clone());
            self.vslice(g.clone(), f, 6, c.max_value_slots, Some(c.max_arity));
            self.vslice(g.clone(), f, 8, c.max_children, Some(c.max_arity));
            self.venv(g.clone(), f[10].clone(), f[11].clone(), c.max_env_cells);
            self.glt_i(g, f[12].clone(), c.max_expressions);
        }
        const USED: [usize; 10] = [6, 6, 5, 3, 6, 6, 9, 5, 4, 13];
        for lane in 1..13usize {
            let mut mz = Self::imm(0);
            let mut any = false;
            for (t, u) in USED.iter().enumerate() {
                if lane >= *u {
                    mz = self.add(mz, bits[t].clone());
                    any = true;
                }
            }
            if any {
                let pg = self.mul(gate.clone(), mz);
                self.gzero(pg, f[lane].clone());
            }
        }
    }
    fn vcontrol(&mut self, gate: Operand, k: &[Operand], c: &C6PhysCaps) {
        debug_assert_eq!(k.len(), 17);
        let bt = c.max_types.max(4);
        let bits = self.tag_bits(&gate, k[0].clone(), 11);
        let vg = |em: &mut Self, i: usize| em.mul(gate.clone(), bits[i].clone());
        {
            let g = vg(self, 0);
            self.glt_i(g, k[1].clone(), c.max_expressions);
        }
        {
            let g = vg(self, 1);
            self.vref(g, k, 1, c);
        }
        {
            let g = vg(self, 2);
            let fr: Vec<Operand> = k[1..14].to_vec();
            self.vframe(g.clone(), &fr, c);
            self.glt_i(g.clone(), k[14].clone(), c.max_expressions);
            self.venv(g, k[15].clone(), k[16].clone(), c.max_env_cells);
        }
        {
            let g = vg(self, 3);
            self.glt_i(g.clone(), k[1].clone(), c.max_names);
            self.vslice(g, k, 2, c.max_value_slots, Some(c.max_arity));
        }
        {
            let g = vg(self, 4);
            self.gbool(g.clone(), k[1].clone());
            let a = self.teq(k[1].clone(), Self::imm(0));
            let ag = self.mul(g.clone(), a);
            self.gzero(ag, k[2].clone());
            let p = self.teq(k[1].clone(), Self::imm(1));
            let pg = self.mul(g.clone(), p);
            self.glt_i(pg, k[2].clone(), c.max_functions);
            self.vslice(g, k, 3, c.max_value_slots, Some(c.max_arity));
        }
        {
            let g = vg(self, 5);
            self.gbool(g.clone(), k[1].clone());
            let a = self.teq(k[1].clone(), Self::imm(0));
            let ag = self.mul(g.clone(), a);
            self.gzero(ag.clone(), k[2].clone());
            self.gzero(ag, k[3].clone());
            self.venv(g.clone(), k[2].clone(), k[3].clone(), c.max_env_cells);
            self.glt_i(g.clone(), k[4].clone(), bt);
            self.glt_i(g, k[5].clone(), c.max_expressions);
        }
        {
            let g = vg(self, 6);
            self.vref(g.clone(), k, 1, c);
            self.vslice(g.clone(), k, 3, c.max_arms, None);
            self.venv(g, k[5].clone(), k[6].clone(), c.max_env_cells);
        }
        {
            let g = vg(self, 7);
            self.gbool(g.clone(), k[1].clone());
            let a = self.teq(k[1].clone(), Self::imm(0));
            let ag = self.mul(g.clone(), a);
            self.gzero(ag.clone(), k[2].clone());
            self.gzero(ag, k[3].clone());
            self.venv(g.clone(), k[2].clone(), k[3].clone(), c.max_env_cells);
            self.vref(g.clone(), k, 4, c);
            self.glt_i(g.clone(), k[6].clone(), c.max_expressions);
            self.vslice(g.clone(), k, 7, c.max_arms, None);
            self.venv(g, k[9].clone(), k[10].clone(), c.max_env_cells);
        }
        {
            let g = vg(self, 8);
            self.gbool(g.clone(), k[1].clone());
            let e = self.teq(k[1].clone(), Self::imm(1));
            let eg = self.mul(g.clone(), e.clone());
            self.glt_i(eg.clone(), k[2].clone(), 256);
            self.gzero(eg, k[3].clone());
            let v = self.not(e);
            let vlg = self.mul(g, v);
            self.vref(vlg, k, 2, c);
        }
        {
            let g = vg(self, 9);
            self.glt_i(g, k[1].clone(), 256);
        }
        {
            let g = vg(self, 10);
            self.glt_i(g.clone(), k[1].clone(), bt);
            self.glt_i(g.clone(), k[2].clone(), c.max_constructors);
            self.vslice(g, k, 3, c.max_value_slots, Some(c.max_arity));
        }
        const USED: [usize; 11] = [2, 3, 17, 4, 5, 6, 7, 11, 4, 2, 5];
        for lane in 1..17usize {
            // Whole-record padding against Core decode_control/encode_control:
            // USED[t] is the used-lane count for tag t. Tag 8 (PrimResult)
            // is NOT exempt: USED[8] = 4 covers the shared 4..16 tail of
            // both sub-tags, the Value ref occupies lanes 2..4 (so lanes
            // 1..3 are untouched by this rule), and the Error sub-tag's
            // lane 3 is separately gated zero by `gzero(eg, k[3])` above.
            let mut mz = Self::imm(0);
            let mut any = false;
            for (t, u) in USED.iter().enumerate() {
                if lane >= *u {
                    mz = self.add(mz, bits[t].clone());
                    any = true;
                }
            }
            if any {
                let pg = self.mul(gate.clone(), mz);
                self.gzero(pg, k[lane].clone());
            }
        }
    }
}

/// Single-shot block at parent serial zero; see
/// `C6ScalarFailureStepEmitter` for the composition contract. Repeated
/// blocks under one scope in one circuit must go through one retained
/// `C6ScalarFailureStepEmitter`.
pub fn emit_c6_scalar_failure_step_block(
    inputs: &C6ScalarFailureStepInputs,
    scope: &str,
) -> Result<C6ScalarFailureStepBlock, C6ScalarFailureStepError> {
    C6ScalarFailureStepEmitter::new(scope)?.emit(inputs)
}

impl C6ScalarFailureStepEmitter {
    pub fn emit(
        &mut self,
        inputs: &C6ScalarFailureStepInputs,
    ) -> Result<C6ScalarFailureStepBlock, C6ScalarFailureStepError> {
        let scope: &str = &self.scope;
        validate_parent_scope(scope)?;
        reject_all_reserved(scope, inputs)?;
        if (LANE_BITS as usize) >= FR_BITS {
            return Err(C6ScalarFailureStepError::InvalidBitWidth { bits: LANE_BITS });
        }
        inputs.source_params.validate_geometry(inputs.geometry)?;
        let g = &inputs.geometry;
        let caps = &g.caps;
        let p = &caps.phys;
        for cap in [
            p.max_frames,
            p.max_expressions,
            p.max_names,
            p.max_arms,
            p.max_children,
            p.max_value_slots,
            p.max_constants,
            p.max_data_nodes,
            p.max_env_cells,
            p.max_functions,
            p.max_arity,
            p.max_types,
            p.max_constructors,
        ] {
            if cap == usize::MAX {
                return Err(C6ScalarFailureStepError::GeometryOverflow { section: "caps" });
            }
        }
        for (s, a, e) in [
            ("image", inputs.pre_image.len(), g.image_total_lanes),
            ("image", inputs.post_image.len(), g.image_total_lanes),
            ("data", inputs.pre_data.len(), g.data_total_lanes),
            ("data", inputs.post_data.len(), g.data_total_lanes),
            ("helper", inputs.pre_helper.len(), g.helper_total_lanes),
            ("helper", inputs.post_helper.len(), g.helper_total_lanes),
        ] {
            if a != e {
                return Err(C6ScalarFailureStepError::LengthMismatch {
                    section: s,
                    expected: e,
                    actual: a,
                });
            }
        }
        if inputs.pre_meta.len() < 3
            || inputs.pre_meta.len() != inputs.post_meta.len()
            || inputs.pre_prog.len() != inputs.post_prog.len()
            || inputs.meta_bounds_frames_offset >= inputs.pre_meta.len()
        {
            return Err(C6ScalarFailureStepError::Backend(
                "host prefix length/offset".into(),
            ));
        }

        let max_hw = inputs.source_params.admission_bounds().max_helper_work;
        let mut em = Em {
            scope,
            next: self.next_temp,
            ins: Vec::new(),
        };
        let en = inputs.enable.clone();
        let dp = inputs.disposition_is_refused_helper_work.clone();
        em.cbool(en.clone());
        em.cbool(dp.clone());
        let disabled = em.not(en.clone());
        em.ceq(inputs.host_schema.clone(), Em::imm(C6_HOST_SCHEMA));
        em.ceq(
            inputs.carrier_version.clone(),
            Em::imm(C6_PADDED_CARRIER_VERSION),
        );

        // Host prefix: unconditional ranges + pre == post; gated headers.
        for (a, b) in inputs
            .pre_meta
            .iter()
            .zip(inputs.post_meta.iter())
            .chain(inputs.pre_prog.iter().zip(inputs.post_prog.iter()))
        {
            em.cbits(a.clone());
            em.cbits(b.clone());
            em.ceq(b.clone(), a.clone());
        }
        for (a, b) in inputs.pre_ctx.iter().zip(inputs.post_ctx.iter()) {
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
        // State lanes: unconditional ranges; DATA/HELPER equality is branch-specific below.
        for l in inputs
            .pre_image
            .iter()
            .chain(inputs.post_image.iter())
            .chain(inputs.pre_data.iter())
            .chain(inputs.post_data.iter())
        {
            em.cbits(l.clone());
        }
        for (a, b) in inputs.pre_data.iter().zip(inputs.post_data.iter()) {
            em.ceq(b.clone(), a.clone());
        }

        let pre_h = &inputs.pre_helper;
        let post_h = &inputs.post_helper;
        let pre_i = &inputs.pre_image;
        let post_i = &inputs.post_image;

        // Pre image root/control/frames structural canonicality (all 11 controls, 10 frames).
        em.geq(en.clone(), pre_i[0].clone(), Em::imm(C6_PHYS_MAGIC));
        em.geq(en.clone(), pre_i[1].clone(), Em::imm(C6_PHYS_VERSION));
        {
            let ctl: Vec<Operand> = pre_i[L_CONTROL..L_CONTROL + 17].to_vec();
            em.vcontrol(en.clone(), &ctl, p);
        }
        em.glt_i(
            en.clone(),
            pre_i[L_CONTROL_ORIGIN].clone(),
            p.max_expressions,
        );
        em.venv(
            en.clone(),
            pre_i[L_ENV_ROOT].clone(),
            pre_i[L_ENV_ROOT + 1].clone(),
            p.max_env_cells,
        );
        em.glt_c(en.clone(), pre_i[L_STACK_LIMIT].clone(), p.max_frames);
        for off in 0..3 {
            em.gzero(en.clone(), pre_i[L_OUTCOME + off].clone());
        }
        let has_fuel = em.lt128(Em::imm(0), pre_i[L_REMAINING_WORK].clone());
        em.gassert(en.clone(), has_fuel);
        for (lane, cap) in [
            (32usize, p.max_data_nodes),
            (34, p.max_env_cells),
            (35, p.max_data_nodes),
            (37, p.max_env_cells),
            (38, p.max_value_slots),
            (39, p.max_data_nodes),
            (41, p.max_env_cells),
            (42, p.max_value_slots),
            (43, p.max_data_nodes),
            (45, p.max_env_cells),
        ] {
            em.glt_c(en.clone(), pre_i[lane].clone(), cap);
        }
        em.glt_c(en.clone(), pre_i[L_FRAME_COUNT].clone(), p.max_frames);
        for slot in 0..p.max_frames {
            let base = g.frame_slot_offset(slot);
            let active = em.lt128(Em::imm(slot as u128), pre_i[L_FRAME_COUNT].clone());
            let inact = em.not(active.clone());
            let pg = em.mul(en.clone(), inact);
            for off in 0..13 {
                em.gzero(pg.clone(), pre_i[base + off].clone());
            }
            let rg_gate = em.mul(en.clone(), active);
            let fr: Vec<Operand> = pre_i[base..base + 13].to_vec();
            em.vframe(rg_gate, &fr, p);
        }

        // Selected pre-state (partial SI08/SI22 predicates emitted; see module notes).
        em.geq(en.clone(), pre_h[L_HELPER_RECORD].clone(), Em::imm(1)); // Scalar
        em.geq(en.clone(), pre_h[L_HELPER_RECORD + 13].clone(), Em::imm(2)); // phase Done
        em.geq(en.clone(), pre_h[L_HELPER_RECORD + 15].clone(), Em::imm(4)); // staged PrimError
        em.gzero(en.clone(), pre_h[L_RESULT_RECORD].clone()); // pre result None
        em.geq(en.clone(), pre_h[3].clone(), Em::imm(0)); // task count 0
        em.gzero(en.clone(), pre_h[L_HELPER_RECORD + 3].clone()); // parent_resume absent
        em.geq(
            en.clone(),
            pre_h[L_HELPER_RECORD + 2].clone(),
            pre_i[L_REFERENCE_CURSOR].clone(),
        );
        {
            // site in {RetRight 11, RetUnary 12, RetIf 13, RetCall 16, RetCtorFields 25}
            let mut s = Em::imm(0);
            for v in [11u128, 12, 13, 16, 25] {
                let b = em.teq(pre_h[L_HELPER_RECORD + 1].clone(), Em::imm(v));
                s = em.add(s, b);
            }
            em.geq(en.clone(), s, Em::imm(1));
        }
        {
            // operand_b present iff mode in 0..=5; expected present iff mode == 9
            let mut bin = Em::imm(0);
            for v in 0..6u128 {
                let b = em.teq(pre_h[L_HELPER_RECORD + 5].clone(), Em::imm(v));
                bin = em.add(bin, b);
            }
            em.geq(en.clone(), pre_h[L_HELPER_RECORD + 8].clone(), bin);
            let tt = em.teq(pre_h[L_HELPER_RECORD + 5].clone(), Em::imm(9));
            em.geq(en.clone(), pre_h[L_HELPER_RECORD + 11].clone(), tt);
        }
        {
            // PrimError code 10|11; 10 only for Add/Sub/Mul (modes 0,1,2)
            let code = pre_h[L_HELPER_RECORD + 16].clone();
            let e10 = em.teq(code.clone(), Em::imm(10));
            let e11 = em.teq(code.clone(), Em::imm(11));
            let mut ok = em.add(e10, e11);
            em.geq(en.clone(), ok.clone(), Em::imm(1));
            let mut arith = Em::imm(0);
            for v in 0..3u128 {
                let b = em.teq(pre_h[L_HELPER_RECORD + 5].clone(), Em::imm(v));
                arith = em.add(arith, b);
            }
            let non_arith = em.not(ok.clone());
            let _ = &mut ok;
            let na_gate_src = em.not(arith);
            let nag = em.mul(en.clone(), na_gate_src);
            em.geq(nag, code, Em::imm(11));
            let _ = non_arith;
        }

        // Disposition: P3 iff helper_work == retained admission max_helper_work.
        let is_p3 = em.teq(pre_i[L_HELPER_WORK].clone(), Em::imm(max_hw));
        em.geq(en.clone(), dp.clone(), is_p3.clone());
        let not_p3 = em.not(is_p3.clone());
        let applied_gate = em.mul(en.clone(), not_p3.clone());

        // Post outcome: Applied Running [0,0,0]; P3 Exhausted{HelperWork} [3,5,0].
        let exp_o0 = em.sel(is_p3.clone(), Em::imm(3), Em::imm(0));
        em.geq(en.clone(), post_i[L_OUTCOME].clone(), exp_o0);
        let exp_o1 = em.sel(is_p3.clone(), Em::imm(5), Em::imm(0));
        em.geq(en.clone(), post_i[L_OUTCOME + 1].clone(), exp_o1);
        em.gzero(en.clone(), post_i[L_OUTCOME + 2].clone());
        // Counters.
        {
            let s = em.add(post_i[L_REMAINING_WORK].clone(), Em::imm(1));
            em.geq(applied_gate.clone(), s, pre_i[L_REMAINING_WORK].clone());
            let pg = em.mul(en.clone(), is_p3.clone());
            em.geq(
                pg,
                post_i[L_REMAINING_WORK].clone(),
                pre_i[L_REMAINING_WORK].clone(),
            );
        }
        for lane in [L_CONSUMED_WORK, L_HELPER_WORK] {
            let noov = em.lt128(pre_i[lane].clone(), Em::imm(u128::MAX));
            em.gassert(applied_gate.clone(), noov);
            let exp = em.add(pre_i[lane].clone(), Em::imm(1));
            let exp2 = em.sel(is_p3.clone(), pre_i[lane].clone(), exp);
            em.geq(en.clone(), post_i[lane].clone(), exp2);
        }
        // All other image lanes preserved (incl. frames, control, origin, env root,
        // initial fuel, helper visits, reference cursor, alloc/active/seg bases).
        let mutable = [
            L_OUTCOME,
            L_OUTCOME + 1,
            L_OUTCOME + 2,
            L_REMAINING_WORK,
            L_CONSUMED_WORK,
            L_HELPER_WORK,
        ];
        for lane in 0..g.image_total_lanes {
            if !mutable.contains(&lane) {
                em.geq(en.clone(), post_i[lane].clone(), pre_i[lane].clone());
            }
        }
        // HELPER post lanes: Applied helper=None/result=ScalarError{pre code}; P3 = pre.
        let pre_code = pre_h[L_HELPER_RECORD + 16].clone();
        for lane in 0..g.helper_total_lanes {
            let expected = if (L_HELPER_RECORD..L_RESULT_RECORD).contains(&lane) {
                em.sel(is_p3.clone(), pre_h[lane].clone(), Em::imm(0))
            } else if lane == L_RESULT_RECORD {
                em.sel(is_p3.clone(), pre_h[lane].clone(), Em::imm(11))
            } else if lane == L_RESULT_RECORD + 1 {
                em.sel(is_p3.clone(), pre_h[lane].clone(), pre_code.clone())
            } else if lane == L_RESULT_RECORD + 2 || lane == L_RESULT_RECORD + 3 {
                em.sel(is_p3.clone(), pre_h[lane].clone(), Em::imm(0))
            } else {
                pre_h[lane].clone()
            };
            em.geq(en.clone(), post_h[lane].clone(), expected);
        }
        let _ = HELPER_FIXED_LANES;
        let _ = L_TASKS;

        // Composed canonical blocks (pre-only parsers bound post by unconditional equality).
        // Each composed emitter retains its serial inside `self`, so a second
        // block under this scope continues every composed namespace.
        let data_block = self
            .data_em
            .emit(&C6DataCanonicalInputs {
                gate: en.clone(),
                data: inputs.pre_data.clone(),
                caps: *caps,
            })
            .map_err(be)?;
        em.ins.extend(data_block.instructions);
        let meta_block = self
            .meta_em
            .emit_bound(&C6MetaBoundCanonicalInputs {
                gate: en.clone(),
                pre_meta: inputs.pre_meta.clone(),
                post_meta: inputs.post_meta.clone(),
                limits: inputs.source_params.limits(),
                meta_bounds_frames_offset: inputs.meta_bounds_frames_offset,
                binding: inputs.source_params.meta_binding(),
            })
            .map_err(be)?;
        em.ins.extend(meta_block.instructions);
        let pc_block = self
            .progctx_em
            .emit(&C6ProgramContextCanonicalInputs {
                gate: en.clone(),
                pre_prog: inputs.pre_prog.clone(),
                pre_ctx: inputs.pre_ctx.clone(),
                caps: *caps,
            })
            .map_err(be)?;
        em.ins.extend(pc_block.instructions);
        let ctx_block = self
            .progctx_em
            .emit_ctx_binding(&C6ProgramContextCtxBindingInputs {
                gate: en.clone(),
                pre_ctx: inputs.pre_ctx.clone(),
                expected_ctx: inputs.source_params.expected_ctx(),
            })
            .map_err(be)?;
        em.ins.extend(ctx_block.instructions);
        // HELPER canonicality on BOTH pre and post under one retained serial.
        let hb1 = self
            .helper_em
            .emit(&C6HelperCanonicalInputs {
                gate: en.clone(),
                helper: inputs.pre_helper.clone(),
                caps: *caps,
            })
            .map_err(be)?;
        em.ins.extend(hb1.instructions);
        let hb2 = self
            .helper_em
            .emit(&C6HelperCanonicalInputs {
                gate: en.clone(),
                helper: inputs.post_helper.clone(),
                caps: *caps,
            })
            .map_err(be)?;
        em.ins.extend(hb2.instructions);

        // Disabled: full equality in every section.
        for (a, b) in inputs
            .pre_image
            .iter()
            .zip(inputs.post_image.iter())
            .chain(inputs.pre_helper.iter().zip(inputs.post_helper.iter()))
        {
            em.geq(disabled.clone(), b.clone(), a.clone());
        }
        self.next_temp = em.next;
        Ok(C6ScalarFailureStepBlock {
            is_helper_work_max: is_p3,
            instructions: em.ins,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct C6ScalarFailureStepWitness {
    enable: bool,
    disposition_is_refused_helper_work: bool,
    source_params: C6PushFamilyParams,
    host_prefix: C6PushHostPrefixLanes,
    pre_image: Vec<u128>,
    pre_data: Vec<u128>,
    pre_helper: Vec<u128>,
    post_image: Vec<u128>,
    post_data: Vec<u128>,
    post_helper: Vec<u128>,
}
impl C6ScalarFailureStepWitness {
    pub fn source_params(&self) -> &C6PushFamilyParams {
        &self.source_params
    }
    pub fn to_statement_frs(&self) -> Vec<Fr> {
        let mut out = Vec::new();
        out.push(Fr::from(self.enable));
        out.push(Fr::from(self.disposition_is_refused_helper_work));
        out.push(Fr::from(C6_HOST_SCHEMA));
        out.push(Fr::from(C6_PADDED_CARRIER_VERSION));
        for l in self
            .host_prefix
            .meta
            .iter()
            .chain(&self.host_prefix.prog)
            .chain(self.host_prefix.ctx.iter())
            .chain(&self.pre_image)
            .chain(&self.pre_data)
            .chain(&self.pre_helper)
        {
            out.push(Fr::from(*l));
        }
        for l in self
            .host_prefix
            .meta
            .iter()
            .chain(&self.host_prefix.prog)
            .chain(self.host_prefix.ctx.iter())
            .chain(&self.post_image)
            .chain(&self.post_data)
            .chain(&self.post_helper)
        {
            out.push(Fr::from(*l));
        }
        out
    }
    fn post_flat(&self) -> Vec<u128> {
        let mut v = self.post_image.clone();
        v.extend(&self.post_data);
        v.extend(&self.post_helper);
        v
    }
}

fn split_state(
    state: &mori_core::sherlock::helper_machine::complete_observation::ImageSnapshot,
    caps: C6StateCaps,
    limits: C6HostLimits,
    geom: C6PaddedCarrierGeometry,
) -> Result<(Vec<u128>, Vec<u128>, Vec<u128>), C6ScalarFailureStepError> {
    let padded = super::c6_push_step::encode_owned_state_to_padded(
        &C6OwnedState::from_image(state),
        caps,
        C6StateLimits {
            max_total_lanes: limits.max_state_lanes,
        },
    )?;
    let flat = padded.to_flat_lanes();
    let i = geom.image_total_lanes;
    let d = i + geom.data_total_lanes;
    Ok((flat[..i].to_vec(), flat[i..d].to_vec(), flat[d..].to_vec()))
}

/// Independent-post route: no recorder runs; wrong post reaches the circuit.
pub fn witness_from_host_pair(
    enable: bool,
    pre: &HostSnapshot,
    post: &HostSnapshot,
    cfg: &C6HostConfig,
    limits: &C6HostLimits,
) -> Result<C6ScalarFailureStepWitness, C6ScalarFailureStepError> {
    let params = C6PushFamilyParams::new(cfg.clone(), pre.admission.bounds, *limits)?;
    let geom = params.geometry();
    let caps = params.caps();
    let pp = C6PushHostPrefixLanes::from_host_snapshot(pre, cfg, limits)?;
    let qp = C6PushHostPrefixLanes::from_host_snapshot(post, cfg, limits)?;
    if pp.meta != qp.meta {
        return Err(C6ScalarFailureStepError::HostPrefixMismatch { section: "meta" });
    }
    if pp.prog != qp.prog {
        return Err(C6ScalarFailureStepError::HostPrefixMismatch { section: "prog" });
    }
    if pp.ctx != qp.ctx {
        return Err(C6ScalarFailureStepError::HostPrefixMismatch { section: "ctx" });
    }
    let (pre_image, pre_data, pre_helper) = split_state(&pre.image, caps, *limits, geom)?;
    let (post_image, post_data, post_helper) = split_state(&post.image, caps, *limits, geom)?;
    Ok(C6ScalarFailureStepWitness {
        enable,
        disposition_is_refused_helper_work: pre.image.helper_work
            == params.admission_bounds().max_helper_work,
        source_params: params,
        host_prefix: pp,
        pre_image,
        pre_data,
        pre_helper,
        post_image,
        post_data,
        post_helper,
    })
}

/// Host record boundary only: checks internal consistency of the provided
/// event AND its agreement with the provided before/after snapshots. A
/// public record remains provided data, not authenticated owner
/// provenance; owner provenance exists only through
/// `record_scalar_failure_step`, whose `Err` may follow one advanced step.
fn validate_owner_record(r: &CompleteStepRecord) -> Result<(), C6ScalarFailureStepError> {
    let e = &r.event;
    let applied = e.disposition == StepDisposition::Applied;
    let p3 = e.disposition == StepDisposition::RefusedHelperWorkUncharged;
    if e.rule_id != RuleIdC::BHelperScalarFailure {
        return Err(C6ScalarFailureStepError::UnexpectedOwnerStep);
    }
    if e.event_kind != EventKind::HelperFail {
        return Err(C6ScalarFailureStepError::EventMismatch {
            field: "event_kind",
        });
    }
    if e.helper_kind != Some(HelperKind::Scalar) {
        return Err(C6ScalarFailureStepError::EventMismatch {
            field: "helper_kind",
        });
    }
    if e.helper_visits_delta != 0 {
        return Err(C6ScalarFailureStepError::EventMismatch {
            field: "helper_visits_delta",
        });
    }
    if e.reference_cursor_delta != 0 {
        return Err(C6ScalarFailureStepError::EventMismatch {
            field: "reference_cursor_delta",
        });
    }
    if e.alloc_deltas
        != (mori_core::sherlock::helper_machine::shared::AllocDeltasC {
            data_nodes: 0,
            data_fields: 0,
            environment_cells: 0,
        })
        || e.value_slot_writes != 0
    {
        return Err(C6ScalarFailureStepError::EventMismatch {
            field: "alloc/value_slot deltas",
        });
    }
    if e.logical_depth_before != e.logical_depth_after
        || e.task_cursor_before != e.task_cursor_after
    {
        return Err(C6ScalarFailureStepError::EventMismatch {
            field: "depth/task cursors",
        });
    }
    if e.helper_phase_before != Some(HelperPhaseSummaryC::Scalar(ScalarPhase::Done))
        || e.helper_cursor_before != Some(2)
    {
        return Err(C6ScalarFailureStepError::EventMismatch {
            field: "helper phase/cursor before",
        });
    }
    if applied {
        if e.work_delta != 1
            || e.helper_work_delta != 1
            || e.result_tag != Some(HelperResultTagC::ScalarError)
        {
            return Err(C6ScalarFailureStepError::EventMismatch {
                field: "applied deltas/result",
            });
        }
        if e.exhausted_kind.is_some()
            || e.outcome_tag != OutcomeTagC::Running
            || e.outcome_payload != OutcomeC::Running
            || e.boundary != BoundaryKind::None
        {
            return Err(C6ScalarFailureStepError::EventMismatch {
                field: "applied outcome",
            });
        }
        if e.helper_phase_after.is_some() || e.helper_cursor_after.is_some() {
            return Err(C6ScalarFailureStepError::EventMismatch {
                field: "applied helper after",
            });
        }
    } else if p3 {
        if e.work_delta != 0 || e.helper_work_delta != 0 {
            return Err(C6ScalarFailureStepError::EventMismatch { field: "p3 deltas" });
        }
        if e.result_tag.is_some() {
            return Err(C6ScalarFailureStepError::EventMismatch {
                field: "p3 result_tag",
            });
        }
        if e.exhausted_kind != Some(ExhaustReason::HelperWork) {
            return Err(C6ScalarFailureStepError::EventMismatch {
                field: "p3 exhausted_kind",
            });
        }
        if e.outcome_tag != OutcomeTagC::Exhausted
            || e.outcome_payload
                != (OutcomeC::Exhausted {
                    reason: ExhaustReason::HelperWork,
                })
            || e.boundary != BoundaryKind::Terminal
        {
            return Err(C6ScalarFailureStepError::EventMismatch {
                field: "p3 outcome",
            });
        }
        if e.helper_phase_after != Some(HelperPhaseSummaryC::Scalar(ScalarPhase::Done))
            || e.helper_cursor_after != Some(2)
        {
            return Err(C6ScalarFailureStepError::EventMismatch {
                field: "p3 helper after",
            });
        }
    } else {
        return Err(C6ScalarFailureStepError::EventMismatch {
            field: "disposition",
        });
    }

    // Snapshot binding (checked arithmetic only; no panics).
    let before = &r.before.image;
    let after = &r.after.image;
    if before.remaining_work == 0 {
        return Err(C6ScalarFailureStepError::EventMismatch {
            field: "snapshot remaining_work",
        });
    }
    let at_max = before.helper_work == r.before.admission.bounds.max_helper_work;
    if p3 != at_max {
        return Err(C6ScalarFailureStepError::EventMismatch {
            field: "snapshot disposition",
        });
    }
    if e.step_id.ordinal != before.consumed_work || *e.step_id.run_id != before.run_id {
        return Err(C6ScalarFailureStepError::EventMismatch {
            field: "snapshot step_id",
        });
    }
    if e.logical_depth_before != before.frames.len() || e.logical_depth_after != after.frames.len()
    {
        return Err(C6ScalarFailureStepError::EventMismatch {
            field: "snapshot depth",
        });
    }
    if e.task_cursor_before != before.tasks.len() || e.task_cursor_after != after.tasks.len() {
        return Err(C6ScalarFailureStepError::EventMismatch {
            field: "snapshot task cursor",
        });
    }
    if after.helper_visits != before.helper_visits {
        return Err(C6ScalarFailureStepError::EventMismatch {
            field: "snapshot helper_visits",
        });
    }
    if after.reference_cursor != before.reference_cursor {
        return Err(C6ScalarFailureStepError::EventMismatch {
            field: "snapshot reference_cursor",
        });
    }
    if after.alloc != before.alloc {
        return Err(C6ScalarFailureStepError::EventMismatch {
            field: "snapshot alloc",
        });
    }
    let (site, origin_cursor, staged_code) = match before.helper {
        Some(HelperControlC::Scalar {
            common,
            staged: ScalarStaged::PrimError { code },
            phase: ScalarPhase::Done,
            ..
        }) => (common.site, common.origin_reference_cursor, code),
        _ => {
            return Err(C6ScalarFailureStepError::EventMismatch {
                field: "snapshot helper before",
            });
        }
    };
    if e.site != Some(site) {
        return Err(C6ScalarFailureStepError::EventMismatch {
            field: "snapshot site",
        });
    }
    if e.origin_reference_cursor != Some(origin_cursor) {
        return Err(C6ScalarFailureStepError::EventMismatch {
            field: "snapshot origin_reference_cursor",
        });
    }
    if e.source_expr_id != before.control_origin {
        return Err(C6ScalarFailureStepError::EventMismatch {
            field: "snapshot source_expr_id",
        });
    }
    if applied {
        let expected = before.remaining_work.checked_sub(1).ok_or(
            C6ScalarFailureStepError::EventMismatch {
                field: "snapshot remaining_work",
            },
        )?;
        if after.remaining_work != expected {
            return Err(C6ScalarFailureStepError::EventMismatch {
                field: "snapshot remaining_work",
            });
        }
        let expected =
            before
                .consumed_work
                .checked_add(1)
                .ok_or(C6ScalarFailureStepError::EventMismatch {
                    field: "snapshot consumed_work",
                })?;
        if after.consumed_work != expected {
            return Err(C6ScalarFailureStepError::EventMismatch {
                field: "snapshot consumed_work",
            });
        }
        let expected =
            before
                .helper_work
                .checked_add(1)
                .ok_or(C6ScalarFailureStepError::EventMismatch {
                    field: "snapshot helper_work",
                })?;
        if after.helper_work != expected {
            return Err(C6ScalarFailureStepError::EventMismatch {
                field: "snapshot helper_work",
            });
        }
        if after.helper.is_some() {
            return Err(C6ScalarFailureStepError::EventMismatch {
                field: "snapshot helper",
            });
        }
        if after.result != Some(HelperResultC::ScalarError { code: staged_code }) {
            return Err(C6ScalarFailureStepError::EventMismatch {
                field: "snapshot result",
            });
        }
        if after.outcome != OutcomeC::Running || e.outcome_payload != after.outcome {
            return Err(C6ScalarFailureStepError::EventMismatch {
                field: "snapshot outcome",
            });
        }
    } else {
        if after.remaining_work != before.remaining_work {
            return Err(C6ScalarFailureStepError::EventMismatch {
                field: "snapshot remaining_work",
            });
        }
        if after.consumed_work != before.consumed_work {
            return Err(C6ScalarFailureStepError::EventMismatch {
                field: "snapshot consumed_work",
            });
        }
        if after.helper_work != before.helper_work {
            return Err(C6ScalarFailureStepError::EventMismatch {
                field: "snapshot helper_work",
            });
        }
        if after.helper != before.helper {
            return Err(C6ScalarFailureStepError::EventMismatch {
                field: "snapshot helper",
            });
        }
        if after.result != before.result {
            return Err(C6ScalarFailureStepError::EventMismatch {
                field: "snapshot result",
            });
        }
        if after.outcome
            != (OutcomeC::Exhausted {
                reason: ExhaustReason::HelperWork,
            })
            || e.outcome_payload != after.outcome
        {
            return Err(C6ScalarFailureStepError::EventMismatch {
                field: "snapshot outcome",
            });
        }
    }
    Ok(())
}

/// Build a witness from one provided complete record. No step is taken.
/// `CompleteStepRecord` is publicly constructible, so this validates only
/// internal consistency of the provided event against the selected
/// ScalarFailure relation (rule, kind, disposition charges, result,
/// exhausted kind, outcome, and helper phase/cursors). It does not
/// authenticate that a real owner produced the record; actual owner
/// provenance exists only through `record_scalar_failure_step`, which
/// calls the fixed owner once. The independent host-pair route remains
/// able to supply a wrong post that reaches the circuit constraints.
pub fn witness_from_owner_record(
    record: &CompleteStepRecord,
    cfg: &C6HostConfig,
    limits: &C6HostLimits,
) -> Result<C6ScalarFailureStepWitness, C6ScalarFailureStepError> {
    validate_owner_record(record)?;
    witness_from_host_pair(true, &record.before, &record.after, cfg, limits)
}

/// Wrapper: calls fixed owner `record_step` exactly once on this image.
/// Refusal means the stepped step was not the selected Scalar failure.
///
/// Non-rollback contract: the one owner step is taken before this wrapper
/// can refuse. An `Err` return can therefore leave `image` already
/// advanced and mutated by that one step (for example a wrong selected
/// rule, or a capture failure after the step). No rollback is performed
/// or guaranteed, and callers must not interpret `Err` as "no step
/// happened". Only success returns a witness for the selected step.
pub fn record_scalar_failure_step(
    admitted: &Admitted,
    image: &mut MachineImage,
    cfg: &C6HostConfig,
    limits: &C6HostLimits,
) -> Result<C6ScalarFailureStepWitness, C6ScalarFailureStepError> {
    match record_step(admitted, image) {
        Ok(ObservedStep::Event(record)) => witness_from_owner_record(&record, cfg, limits),
        Ok(ObservedStep::Terminal { .. }) => Err(C6ScalarFailureStepError::UnexpectedOwnerStep),
        Err(e) => Err(C6ScalarFailureStepError::Backend(format!(
            "owner recorder failed: {}",
            capture_step_error_label(&e)
        ))),
    }
}

#[derive(Clone, Debug)]
pub struct C6ScalarFailureStepCircuit {
    mode: C6PushStatementBindingMode,
    geometry: C6PaddedCarrierGeometry,
    meta_len: usize,
    prog_len: usize,
    meta_bounds_frames_offset: usize,
    source_params: C6PushFamilyParams,
    statement_lane_count: usize,
    ir: IrSource,
    serialized_zkir: String,
}
impl C6ScalarFailureStepCircuit {
    pub fn source_params(&self) -> &C6PushFamilyParams {
        &self.source_params
    }
    pub fn geometry(&self) -> C6PaddedCarrierGeometry {
        self.geometry
    }
}

fn make_typed(id: Identifier) -> Result<TypedIdentifier, C6ScalarFailureStepError> {
    serde_json::from_value(json!({ "name": id, "type": IrType::Native })).map_err(be)
}

#[allow(clippy::too_many_arguments)]
fn alloc_lane(
    mode: C6PushStatementBindingMode,
    scope: &str,
    label: &str,
    idx: usize,
    inputs: &mut Vec<TypedIdentifier>,
    ins: &mut Vec<Instruction>,
    tr: &mut Vec<Operand>,
) -> Result<Operand, C6ScalarFailureStepError> {
    let prefix = match mode {
        C6PushStatementBindingMode::LocalPublic => "public",
        C6PushStatementBindingMode::LedgerCall => "input",
    };
    let id = Identifier(format!("%{scope}.{prefix}.{label}.{idx}"));
    match mode {
        C6PushStatementBindingMode::LocalPublic => ins.push(Instruction::PublicInput {
            guard: None,
            val_t: IrType::Native,
            output: id.clone(),
        }),
        C6PushStatementBindingMode::LedgerCall => inputs.push(make_typed(id.clone())?),
    }
    let op = Operand::Variable(id);
    tr.push(op.clone());
    Ok(op)
}

pub fn build_c6_scalar_failure_step_circuit(
    source_params: C6PushFamilyParams,
    meta_len: usize,
    prog_len: usize,
    meta_bounds_frames_offset: usize,
    mode: C6PushStatementBindingMode,
    scope: &str,
) -> Result<C6ScalarFailureStepCircuit, C6ScalarFailureStepError> {
    if scope.trim().is_empty() {
        return Err(C6ScalarFailureStepError::EmptyScope);
    }
    let geometry = source_params.geometry();
    let mut call_inputs = Vec::new();
    let mut ins = Vec::new();
    let mut tr = Vec::new();
    let mut alloc =
        |label: &str,
         idx: usize,
         ci: &mut Vec<TypedIdentifier>,
         ii: &mut Vec<Instruction>,
         tt: &mut Vec<Operand>| alloc_lane(mode, scope, label, idx, ci, ii, tt);
    let enable = alloc("enable", 0, &mut call_inputs, &mut ins, &mut tr)?;
    let disp = alloc(
        "disposition_is_refused",
        0,
        &mut call_inputs,
        &mut ins,
        &mut tr,
    )?;
    let schema = alloc("host_schema", 0, &mut call_inputs, &mut ins, &mut tr)?;
    let carrier = alloc("carrier_version", 0, &mut call_inputs, &mut ins, &mut tr)?;
    let mut vec_of = |label: &str,
                      n: usize,
                      ci: &mut Vec<TypedIdentifier>,
                      ii: &mut Vec<Instruction>,
                      tt: &mut Vec<Operand>|
     -> Result<Vec<Operand>, C6ScalarFailureStepError> {
        let mut o = Vec::with_capacity(n);
        for i in 0..n {
            o.push(alloc(label, i, ci, ii, tt)?);
        }
        Ok(o)
    };
    let pre_meta = vec_of("pre_meta", meta_len, &mut call_inputs, &mut ins, &mut tr)?;
    let pre_prog = vec_of("pre_prog", prog_len, &mut call_inputs, &mut ins, &mut tr)?;
    let pre_ctx_v = vec_of("pre_ctx", CTX_LANES, &mut call_inputs, &mut ins, &mut tr)?;
    let pre_ctx: [Operand; CTX_LANES] = pre_ctx_v
        .try_into()
        .map_err(|_| C6ScalarFailureStepError::Backend("pre_ctx".into()))?;
    let pre_image = vec_of(
        "pre_image",
        geometry.image_total_lanes,
        &mut call_inputs,
        &mut ins,
        &mut tr,
    )?;
    let pre_data = vec_of(
        "pre_data",
        geometry.data_total_lanes,
        &mut call_inputs,
        &mut ins,
        &mut tr,
    )?;
    let pre_helper = vec_of(
        "pre_helper",
        geometry.helper_total_lanes,
        &mut call_inputs,
        &mut ins,
        &mut tr,
    )?;
    let post_meta = vec_of("post_meta", meta_len, &mut call_inputs, &mut ins, &mut tr)?;
    let post_prog = vec_of("post_prog", prog_len, &mut call_inputs, &mut ins, &mut tr)?;
    let post_ctx_v = vec_of("post_ctx", CTX_LANES, &mut call_inputs, &mut ins, &mut tr)?;
    let post_ctx: [Operand; CTX_LANES] = post_ctx_v
        .try_into()
        .map_err(|_| C6ScalarFailureStepError::Backend("post_ctx".into()))?;
    let post_image = vec_of(
        "post_image",
        geometry.image_total_lanes,
        &mut call_inputs,
        &mut ins,
        &mut tr,
    )?;
    let post_data = vec_of(
        "post_data",
        geometry.data_total_lanes,
        &mut call_inputs,
        &mut ins,
        &mut tr,
    )?;
    let post_helper = vec_of(
        "post_helper",
        geometry.helper_total_lanes,
        &mut call_inputs,
        &mut ins,
        &mut tr,
    )?;
    let block = emit_c6_scalar_failure_step_block(
        &C6ScalarFailureStepInputs {
            enable,
            disposition_is_refused_helper_work: disp,
            host_schema: schema,
            carrier_version: carrier,
            geometry,
            meta_bounds_frames_offset,
            source_params: source_params.clone(),
            pre_meta,
            pre_prog,
            pre_ctx,
            pre_image,
            pre_data,
            pre_helper,
            post_meta,
            post_prog,
            post_ctx,
            post_image: post_image.clone(),
            post_data: post_data.clone(),
            post_helper: post_helper.clone(),
        },
        scope,
    )?;
    ins.extend(block.instructions);
    let statement_lane_count = tr.len();
    ins.push(Instruction::Impact {
        guard: Operand::Immediate(Fr::from(1u64)),
        inputs: tr,
    });
    let outputs = match mode {
        C6PushStatementBindingMode::LocalPublic => Vec::new(),
        C6PushStatementBindingMode::LedgerCall => {
            let mut vals = Vec::with_capacity(geometry.state_total_lanes);
            vals.extend(post_image);
            vals.extend(post_data);
            vals.extend(post_helper);
            ins.push(Instruction::Output { vals });
            vec![IrType::Native; geometry.state_total_lanes]
        }
    };
    let ir0 = IrSource {
        version: IrMinorVersion::V0,
        inputs: call_inputs,
        outputs,
        do_communications_commitment: matches!(mode, C6PushStatementBindingMode::LedgerCall),
        instructions: Arc::new(ins),
    };
    let mut disk = serde_json::to_value(&ir0).map_err(be)?;
    disk["version"] = json!({ "major": 3, "minor": 0 });
    let serialized_zkir = serde_json::to_string(&disk).map_err(be)?;
    let ir = IrSource::load(serialized_zkir.as_bytes()).map_err(be)?;
    // No instruction-count guard is emitted here. An instruction count is
    // not the circuit degree `k`: the pinned SDK derives `k` from its
    // cost model (`IrSource::k` / `optimal_k`), and `VERIFIER_MAX_DEGREE`
    // is a standard-verifier-key parameter, not a bound this builder may
    // present as a capacity proof. This builder therefore refuses nothing
    // on size grounds; any future coarse host resource limit must be a
    // distinctly named, separately reviewed policy, not a degree claim.
    // No size, `k`, or performance qualification is made by this module.
    Ok(C6ScalarFailureStepCircuit {
        mode,
        geometry,
        meta_len,
        prog_len,
        meta_bounds_frames_offset,
        source_params,
        statement_lane_count,
        ir,
        serialized_zkir,
    })
}

/// `ProofPreimage::check` preprocessing only; not synthesis/proving/native.
pub fn check_c6_scalar_failure_step_circuit(
    circuit: &C6ScalarFailureStepCircuit,
    witness: &C6ScalarFailureStepWitness,
) -> Result<Vec<Option<usize>>, C6ScalarFailureStepError> {
    if witness.source_params.geometry() != circuit.geometry {
        return Err(C6ScalarFailureStepError::Backend(
            "witness geometry does not match circuit".into(),
        ));
    }
    if witness.host_prefix.meta.len() != circuit.meta_len {
        return Err(C6ScalarFailureStepError::LengthMismatch {
            section: "meta",
            expected: circuit.meta_len,
            actual: witness.host_prefix.meta.len(),
        });
    }
    if witness.host_prefix.prog.len() != circuit.prog_len {
        return Err(C6ScalarFailureStepError::LengthMismatch {
            section: "prog",
            expected: circuit.prog_len,
            actual: witness.host_prefix.prog.len(),
        });
    }
    if witness.host_prefix.meta_bounds_frames_offset != circuit.meta_bounds_frames_offset {
        return Err(C6ScalarFailureStepError::Backend(
            "meta bounds frames offset mismatch".into(),
        ));
    }
    let statement = witness.to_statement_frs();
    if statement.len() != circuit.statement_lane_count {
        return Err(C6ScalarFailureStepError::LengthMismatch {
            section: "statement",
            expected: circuit.statement_lane_count,
            actual: statement.len(),
        });
    }
    let preimage = match circuit.mode {
        C6PushStatementBindingMode::LocalPublic => ProofPreimage {
            inputs: Vec::new(),
            private_transcript: Vec::new(),
            public_transcript_inputs: statement.clone(),
            public_transcript_outputs: statement,
            binding_input: Fr::from(0u64),
            communications_commitment: None,
            key_location: KeyLocation(CHECK_KEY_LOCATION.into()),
        },
        C6PushStatementBindingMode::LedgerCall => {
            let randomness = Fr::from(0u64);
            let mut comm = statement.clone();
            comm.extend(witness.post_flat().iter().map(|&l| Fr::from(l)));
            ProofPreimage {
                inputs: statement.clone(),
                private_transcript: Vec::new(),
                public_transcript_inputs: statement,
                public_transcript_outputs: Vec::new(),
                binding_input: Fr::from(0u64),
                communications_commitment: Some((transient_commit(&comm, randomness), randomness)),
                key_location: KeyLocation(CHECK_KEY_LOCATION.into()),
            }
        }
    };
    preimage.check(&circuit.ir).map_err(|e| {
        C6ScalarFailureStepError::Backend(format!("SDK ProofPreimage::check failed: {e}"))
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    use midnight_proofs::circuit::Value;
    use midnight_proofs::dev::MockProver;
    use midnight_transient_crypto::curve::outer;
    use midnight_zk_stdlib::MidnightCircuit;
    use midnight_zkir_v3::Preprocessed;
    use midnight_zkir_v3::ir_types::IrValue;
    use mori_core::sherlock::helper_machine::c6_phys::{C6_PHYS_MAGIC, L_FRAME_COUNT};
    use mori_core::sherlock::helper_machine::c6_phys_host::{C6HostMetadata, encode_metadata};
    use mori_core::sherlock::helper_machine::complete_observation::{
        AdmissionSnapshot, ConstantSnapshot, DataSnapshot, ExpressionSnapshot, ImageSnapshot,
        LayoutSnapshot, ProgramSnapshot,
    };
    use mori_core::sherlock::helper_machine::shared::host_implied_prefix::SegBase;
    use mori_core::sherlock::helper_machine::shared::{
        ActiveLengths, AllocCounters, BinaryOp, BoundsC, ControlC, DigestC, EnvCellC, FinanceInput,
        FrameRowC, GrammarDomain, HelperCommon, HelperControlC, HelperResultC, IdentityC,
        LayoutCapacitiesC, OutcomeC, RefC, RunIdC, RunIntC, ScalarMode, ScalarPhase, ScalarStaged,
        SiteTag, SliceC,
    };
    use std::collections::HashMap;

    fn tooling_label(e: &mori_core::sherlock::helper_machine::shared::Tooling) -> String {
        use mori_core::sherlock::helper_machine::shared::Tooling;
        match e {
            Tooling::AdmissionRejected {
                reason,
                detail,
                locator,
            } => format!(
                "admission rejected: reason={reason:?}, detail={detail}, locator={locator:?}"
            ),
            Tooling::CapacityNotAdmitted {
                kind,
                needed,
                limit,
                locator,
            } => format!(
                "capacity not admitted: kind={kind:?}, needed={needed:?}, limit={limit}, locator={locator:?}"
            ),
            Tooling::UnsupportedFeature { feature, locator } => {
                format!("unsupported feature: feature={feature:?}, locator={locator:?}")
            }
        }
    }

    fn limits() -> C6HostLimits {
        C6HostLimits {
            max_total_lanes: 1 << 20,
            max_state_lanes: 1 << 20,
            max_metadata_bytes: 8192,
            max_string_bytes: 512,
            max_width_entries: 64,
        }
    }
    fn bounds(frames: usize, max_hw: u128) -> BoundsC {
        let mut b = BoundsC {
            frames,
            ..BoundsC::default()
        };
        b.code_rows = 16;
        b.function_rows = 8;
        b.code_child_slots = 16;
        b.parameter_slots = 16;
        b.pattern_rows = 16;
        b.match_plan_rows = 16;
        b.match_arm_rows = 8;
        b.type_rows = 16;
        b.constructor_rows = 16;
        b.constructor_field_slots = 16;
        b.max_arity = 4;
        b.constant_nodes = 16;
        b.constant_field_slots = 16;
        b.data_nodes = 2;
        b.data_field_slots = 2;
        b.environment_cells = 2;
        b.helper_tasks = 2;
        b.serialization_bytes = 1_000_000;
        b.max_helper_work = max_hw;
        b.step_cap = 1_000_000;
        b.max_reference_work = 1_000_000;
        b
    }
    fn identity() -> IdentityC {
        IdentityC {
            profile_id: 2,
            grammar_domain: GrammarDomain::GenericNominalExtensionReserved,
            finance: FinanceInput::Absent,
            layout_version: ("c6".into(), "1".into()),
            layout_digest: DigestC::Unselected,
            bounds_digest: DigestC::Selected {
                format: ("f".into(), "1".into()),
                bytes: vec![1, 2],
            },
            field_id: Some(("fid".into(), "".into())),
            program_membership_digest: DigestC::Unselected,
        }
    }
    fn config() -> C6HostConfig {
        C6HostConfig {
            layout: LayoutSnapshot {
                layout_version: ("lv".into(), "1".into()),
                field_id: None,
                widths: None,
                scalar_layout_id: None,
                bounds: bounds(3, 0),
                capacities: LayoutCapacitiesC {
                    name_rows: 16,
                    name_bytes: 256,
                    occurrence_path_slots: 32,
                    value_slots: 2,
                },
                boundary_capacities_present: false,
            },
            ext: None,
            budgets: mori_core::sherlock::helper_machine::c6_phys::C6CodecBudgets {
                type_expr_nodes: 0,
                template_bytes: 0,
            },
        }
    }
    fn sample(helper_work: u128, max_hw: u128) -> HostSnapshot {
        let cfg = config();
        let id = identity();
        HostSnapshot {
            admission: AdmissionSnapshot {
                identity: id.clone(),
                bounds: bounds(3, max_hw),
                view: ProgramSnapshot {
                    entry: 0,
                    expressions: vec![
                        ExpressionSnapshot::Literal { constant: 0 },
                        ExpressionSnapshot::Binary {
                            op: BinaryOp::Add,
                            left: 0,
                            right: 0,
                        },
                    ],
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
                    constants: vec![ConstantSnapshot::U128 {
                        marker_free: true,
                        n: u128::MAX,
                    }],
                    constant_fields: Vec::new(),
                    names: vec!["x".into()],
                },
                layout: cfg.layout.clone(),
            },
            image: ImageSnapshot {
                run_id: RunIdC {
                    identity: id,
                    fuel_input: RunIntC::Nat(64),
                    stack_limit_input: RunIntC::Nat(2),
                },
                control: ControlC::Return {
                    value: RefC::Data(0),
                },
                control_origin: 1,
                env_root: Some(0),
                stack_limit: 2,
                outcome: OutcomeC::Running,
                initial_fuel: 64,
                remaining_work: 10,
                consumed_work: 5,
                helper_work,
                helper_visits: 1,
                reference_cursor: 7,
                alloc: AllocCounters {
                    data_nodes: 1,
                    data_fields: 1,
                    environment_cells: 2,
                },
                active: ActiveLengths {
                    data: 1,
                    data_fields: 1,
                    env: 2,
                    value_slots: 1,
                },
                helper: Some(HelperControlC::Scalar {
                    common: HelperCommon {
                        site: SiteTag::RetRight,
                        origin_reference_cursor: 7,
                        parent_resume: None,
                    },
                    mode: ScalarMode::Add,
                    operand_a: RefC::Const(0),
                    operand_b: Some(RefC::Data(0)),
                    expected: None,
                    phase: ScalarPhase::Done,
                    reg_a: u128::MAX,
                    staged: ScalarStaged::PrimError { code: 10 },
                }),
                result: None,
                seg_base: SegBase::genesis_uncut(),
                data: vec![DataSnapshot::U128 { n: 1 }],
                data_fields: vec![RefC::Const(0)],
                env: vec![
                    EnvCellC {
                        binder: 0,
                        value: RefC::Const(0),
                        previous: None,
                    },
                    EnvCellC {
                        binder: 0,
                        value: RefC::Data(0),
                        previous: Some(0),
                    },
                ],
                frames: vec![FrameRowC::RightF {
                    op: BinaryOp::Add,
                    left: RefC::Const(0),
                    origin: 1,
                }],
                tasks: Vec::new(),
                value_slots: vec![RefC::Data(0)],
            },
        }
    }
    fn applied_post(pre: &HostSnapshot) -> HostSnapshot {
        let mut post = pre.clone();
        post.image.helper = None;
        post.image.result = Some(HelperResultC::ScalarError { code: 10 });
        post.image.remaining_work -= 1;
        post.image.consumed_work += 1;
        post.image.helper_work += 1;
        post
    }
    fn p3_post(pre: &HostSnapshot) -> HostSnapshot {
        let mut post = pre.clone();
        post.image.outcome = OutcomeC::Exhausted {
            reason: mori_core::sherlock::helper_machine::shared::ExhaustReason::HelperWork,
        };
        post
    }
    fn circuit(w: &C6ScalarFailureStepWitness) -> C6ScalarFailureStepCircuit {
        build_c6_scalar_failure_step_circuit(
            w.source_params().clone(),
            w.host_prefix.meta.len(),
            w.host_prefix.prog.len(),
            w.host_prefix.meta_bounds_frames_offset,
            C6PushStatementBindingMode::LocalPublic,
            "c6_sf_test",
        )
        .unwrap()
    }
    fn assert_pre_rejects(w: &C6ScalarFailureStepWitness, label: &str) {
        let c = circuit_for(w);
        match check_c6_scalar_failure_step_circuit(&c, w) {
            Err(C6ScalarFailureStepError::Backend(m)) => assert!(
                m.starts_with("SDK ProofPreimage::check failed:"),
                "{label}: {m}"
            ),
            other => panic!(
                "{label}: expected preprocessing rejection, got ok={}",
                other.is_ok()
            ),
        }
    }
    fn circuit_for(w: &C6ScalarFailureStepWitness) -> C6ScalarFailureStepCircuit {
        circuit(w)
    }

    #[test]
    fn positive_applied_p3_disabled_preprocessing() {
        let cfg = config();
        let pre = sample(2, 100);
        let post = applied_post(&pre);
        let w = witness_from_host_pair(true, &pre, &post, &cfg, &limits()).unwrap();
        assert!(!w.disposition_is_refused_helper_work);
        assert!(
            check_c6_scalar_failure_step_circuit(&circuit(&w), &w).is_ok(),
            "applied"
        );
        let pre3 = sample(100, 100);
        let post3 = p3_post(&pre3);
        let w3 = witness_from_host_pair(true, &pre3, &post3, &cfg, &limits()).unwrap();
        assert!(w3.disposition_is_refused_helper_work);
        assert!(
            check_c6_scalar_failure_step_circuit(&circuit(&w3), &w3).is_ok(),
            "p3 boundary"
        );
        let wd = witness_from_host_pair(false, &pre, &pre, &cfg, &limits()).unwrap();
        assert!(
            check_c6_scalar_failure_step_circuit(&circuit(&wd), &wd).is_ok(),
            "disabled"
        );
        // helperWork just below max is Applied, not P3.
        let pre_b = sample(99, 100);
        let post_b = applied_post(&pre_b);
        let wb = witness_from_host_pair(true, &pre_b, &post_b, &cfg, &limits()).unwrap();
        assert!(
            check_c6_scalar_failure_step_circuit(&circuit(&wb), &wb).is_ok(),
            "below max"
        );
    }

    #[test]
    fn wrong_code_helper_result_and_selected_shape_reject_preprocessing() {
        let cfg = config();
        let pre = sample(2, 100);
        let post = applied_post(&pre);
        let w = witness_from_host_pair(true, &pre, &post, &cfg, &limits()).unwrap();
        assert!(
            check_c6_scalar_failure_step_circuit(&circuit(&w), &w).is_ok(),
            "control"
        );
        let mut bad = w.clone();
        bad.post_helper[L_RESULT_RECORD + 1] = 11;
        assert_pre_rejects(&bad, "wrong post code");
        let mut bad = w.clone();
        bad.post_helper[L_RESULT_RECORD] = 10;
        assert_pre_rejects(&bad, "wrong result tag ScalarValue");
        let mut bad = w.clone();
        bad.post_helper[L_HELPER_RECORD] = 1;
        assert_pre_rejects(&bad, "helper not cleared");
        let mut bad = w.clone();
        bad.post_helper[L_RESULT_RECORD + 2] = 1;
        assert_pre_rejects(&bad, "result padding");
        let mut bad = w.clone();
        bad.pre_helper[L_HELPER_RECORD] = 2;
        assert_pre_rejects(&bad, "wrong helper kind EnvGet");
        let mut bad = w.clone();
        bad.pre_helper[L_HELPER_RECORD + 13] = 1;
        assert_pre_rejects(&bad, "wrong phase Second");
        let mut bad = w.clone();
        bad.pre_helper[L_HELPER_RECORD + 15] = 1;
        assert_pre_rejects(&bad, "wrong staged tag U128Value");
        let mut bad = w.clone();
        bad.pre_helper[L_HELPER_RECORD + 16] = 12;
        assert_pre_rejects(&bad, "staged code outside 10/11");
    }

    #[test]
    fn charge_and_preservation_mutations_reject_preprocessing() {
        let cfg = config();
        let pre = sample(2, 100);
        let post = applied_post(&pre);
        let w = witness_from_host_pair(true, &pre, &post, &cfg, &limits()).unwrap();
        assert!(
            check_c6_scalar_failure_step_circuit(&circuit(&w), &w).is_ok(),
            "positive control"
        );
        let mut bad = w.clone();
        bad.post_image[L_HELPER_WORK] = bad.pre_image[L_HELPER_WORK];
        assert_pre_rejects(&bad, "missing helper-work charge");
        let mut bad = w.clone();
        bad.post_image[L_REFERENCE_CURSOR] += 1;
        assert_pre_rejects(&bad, "reference cursor changed");
        let mut bad = w.clone();
        bad.post_image[L_HELPER_VISITS] += 1;
        assert_pre_rejects(&bad, "helper visits changed");
        let mut bad = w.clone();
        bad.post_image[L_CONTROL] = 1;
        assert_pre_rejects(&bad, "control changed");
        let mut bad = w.clone();
        bad.post_image[L_CONTROL_ORIGIN] = 0;
        assert_pre_rejects(&bad, "origin changed");
        let mut bad = w.clone();
        bad.post_image[bad.source_params.geometry().frame_slot_offset(0) + 1] += 1;
        assert_pre_rejects(&bad, "frame changed");
        let mut bad = w.clone();
        bad.post_data[8] += 1;
        assert_pre_rejects(&bad, "data/value-slot lane changed");
        let mut bad = w.clone();
        bad.post_helper[31] = 1;
        assert_pre_rejects(&bad, "task lane changed");
        // P3 charged refusal is rejected: consumed changed on P3 post.
        let pre3 = sample(100, 100);
        let mut post3 = p3_post(&pre3);
        post3.image.consumed_work += 1;
        let w3 = witness_from_host_pair(true, &pre3, &post3, &cfg, &limits()).unwrap();
        assert_pre_rejects(&w3, "charged P3 refusal");
        // Counter extremes: consumed at u128::MAX on Applied must not wrap.
        // The wrong post is supplied directly as raw state; it is not computed
        // by applying `+= 1` to `u128::MAX` in the test host.
        let mut pre_m = sample(2, 100);
        pre_m.image.remaining_work = 10;
        pre_m.image.consumed_work = u128::MAX;
        let mut post_m = pre_m.clone();
        post_m.image.helper = None;
        post_m.image.result = Some(HelperResultC::ScalarError { code: 10 });
        post_m.image.remaining_work -= 1;
        post_m.image.helper_work += 1;
        post_m.image.consumed_work = 0;
        let wm = witness_from_host_pair(true, &pre_m, &post_m, &cfg, &limits()).unwrap();
        assert_pre_rejects(&wm, "consumed overflow wrap");
    }

    #[test]
    fn meta_ctx_mutations_reject_preprocessing() {
        let cfg = config();
        let pre = sample(2, 100);
        let post = applied_post(&pre);
        let w = witness_from_host_pair(true, &pre, &post, &cfg, &limits()).unwrap();
        assert!(
            check_c6_scalar_failure_step_circuit(&circuit(&w), &w).is_ok(),
            "control"
        );
        let mut bad = w.clone();
        bad.host_prefix.ctx[20] += 1;
        assert_pre_rejects(&bad, "ctx budget");
        let mut bad = w.clone();
        bad.host_prefix.ctx[3] = 1;
        assert_pre_rejects(&bad, "ctx ext presence");
        let mut md = C6HostMetadata::from_host(&pre);
        md.bounds.code_rows += 1;
        let mut bad = w.clone();
        bad.host_prefix.meta = encode_metadata(&md, &limits()).unwrap();
        assert_pre_rejects(&bad, "admission META field");
    }

    #[derive(Debug, PartialEq, Eq)]
    enum Synth {
        Constraint(String),
        Synthesis(String),
    }
    fn run_synthesis(
        w: &C6ScalarFailureStepWitness,
        c: &C6ScalarFailureStepCircuit,
    ) -> Result<(), Synth> {
        // Reconstruct LocalPublic identifiers in builder allocation order.
        let g = c.geometry();
        let scope = "c6_sf_test";
        let mut ids: Vec<Identifier> = Vec::new();
        let mut push = |label: &str, n: usize, ids: &mut Vec<Identifier>| {
            for i in 0..n {
                ids.push(Identifier(format!("%{scope}.public.{label}.{i}")));
            }
        };
        push("enable", 1, &mut ids);
        push("disposition_is_refused", 1, &mut ids);
        push("host_schema", 1, &mut ids);
        push("carrier_version", 1, &mut ids);
        push("pre_meta", c.meta_len, &mut ids);
        push("pre_prog", c.prog_len, &mut ids);
        push("pre_ctx", CTX_LANES, &mut ids);
        push("pre_image", g.image_total_lanes, &mut ids);
        push("pre_data", g.data_total_lanes, &mut ids);
        push("pre_helper", g.helper_total_lanes, &mut ids);
        push("post_meta", c.meta_len, &mut ids);
        push("post_prog", c.prog_len, &mut ids);
        push("post_ctx", CTX_LANES, &mut ids);
        push("post_image", g.image_total_lanes, &mut ids);
        push("post_data", g.data_total_lanes, &mut ids);
        push("post_helper", g.helper_total_lanes, &mut ids);
        let statement = w.to_statement_frs();
        assert_eq!(ids.len(), statement.len());
        let mut memory: HashMap<Identifier, IrValue> = HashMap::new();
        for (id, v) in ids.iter().zip(statement.iter()) {
            memory.insert(id.clone(), IrValue::Native(*v));
        }
        let instance: Vec<outer::Scalar> = statement.iter().map(|v| v.0).collect();
        let mut pis: Vec<outer::Scalar> = vec![Fr::from(0u64).0];
        pis.extend(instance.iter().copied());
        let pre = Preprocessed {
            memory,
            pis: pis.clone(),
            pi_skips: vec![None],
            binding_input: Fr::from(0u64).0,
            comm_comm: None,
        };
        let circ = MidnightCircuit::new(&c.ir, Value::known(instance), Value::known(pre), Some(18));
        match MockProver::<outer::Scalar>::run(&circ, vec![Vec::new(), pis]) {
            Ok(p) => p.verify().map_err(|e| Synth::Constraint(format!("{e:?}"))),
            Err(e) => Err(Synth::Synthesis(format!("{e:?}"))),
        }
    }

    #[test]
    fn synthesis_distinguishes_constraint_failure() {
        let cfg = config();
        let pre = sample(2, 100);
        let post = applied_post(&pre);
        let w = witness_from_host_pair(true, &pre, &post, &cfg, &limits()).unwrap();
        let c = circuit(&w);
        assert!(
            check_c6_scalar_failure_step_circuit(&c, &w).is_ok(),
            "preprocessing control"
        );
        assert!(run_synthesis(&w, &c).is_ok(), "valid synthesis");
        let mut bad = w.clone();
        bad.post_helper[L_RESULT_RECORD + 1] = 11;
        assert_pre_rejects(&bad, "wrong code preprocessing");
        assert!(
            matches!(run_synthesis(&bad, &c), Err(Synth::Constraint(_))),
            "wrong raw post code must fail only as unsatisfied constraint"
        );
    }

    #[test]
    fn real_owner_recorder_reaches_scalar_failure() {
        use mori_core::sherlock::helper_machine::complete_observation::record_step as owner_record;
        use mori_core::sherlock::helper_machine::shared::i128_class::KIntC;
        use mori_core::sherlock::helper_machine::shared::owner_host::host_owner_engine;
        use mori_core::sherlock::helper_machine::shared::{
            CoreExpr, CoreProgram, CoreValue, HelperControlC as HC, LayoutSchemaC, OwnerEngine,
            ScalarPhase as SP, ScalarStaged as SS,
        };
        let lit = |n: u128| {
            Box::new(CoreExpr::Literal {
                value: CoreValue::UInt {
                    n: KIntC::InRange(n),
                },
            })
        };
        // Direct CoreProgram AST admitted through the real public owner
        // `admit` path (`prepare_core`, `check_program_view`, `admit_run`).
        // This is not a nominal Mori source-text compiler test.
        let program = CoreProgram {
            functions: Vec::new(),
            entry: CoreExpr::Binary {
                op: BinaryOp::Add,
                left: lit(u128::MAX),
                right: lit(1),
            },
        };
        let b = bounds(3, 100);
        let layout = LayoutSchemaC {
            layout_version: ("lv".into(), "1".into()),
            field_id: None,
            widths: None,
            scalar_layout_id: None,
            bounds: b,
            capacities: LayoutCapacitiesC {
                name_rows: 16,
                name_bytes: 256,
                occurrence_path_slots: 32,
                value_slots: 64,
            },
            ext_capacities: None,
            boundary_capacities_present: false,
        };
        let engine = host_owner_engine();
        let admitted = match engine.admit(&program, &b, &layout) {
            Ok(admitted) => admitted,
            Err(e) => panic!("admit overflow program: {}", tooling_label(&e)),
        };
        let mut image = match engine.admit_run(
            &admitted,
            &RunIntC::Nat(64),
            &RunIntC::Nat(3),
            &FinanceInput::Absent,
        ) {
            Ok(image) => image,
            Err(e) => panic!("genesis: {}", tooling_label(&e)),
        };
        // Source-test-only bounded search for the selected pre-state; the
        // wrapper below is the one and only step taken on that selected image.
        let mut selected = false;
        for _ in 0..64 {
            let step = match owner_record(&admitted, &mut image) {
                Ok(step) => step,
                Err(e) => panic!("search step: {}", capture_step_error_label(&e)),
            };
            let after = match &step {
                ObservedStep::Event(r) => &r.after,
                ObservedStep::Terminal { after, .. } => after,
            };
            if matches!(
                after.image.helper,
                Some(HC::Scalar {
                    phase: SP::Done,
                    staged: SS::PrimError { .. },
                    ..
                })
            ) {
                selected = true;
                break;
            }
        }
        assert!(selected, "owner must reach Scalar Done/PrimError");
        let cfg2 = C6HostConfig {
            layout: LayoutSnapshot {
                layout_version: ("lv".into(), "1".into()),
                field_id: None,
                widths: None,
                scalar_layout_id: None,
                bounds: b,
                capacities: layout.capacities,
                boundary_capacities_present: false,
            },
            ext: None,
            budgets: mori_core::sherlock::helper_machine::c6_phys::C6CodecBudgets {
                type_expr_nodes: 256,
                template_bytes: 4096,
            },
        };
        let w = record_scalar_failure_step(&admitted, &mut image, &cfg2, &limits())
            .expect("selected wrapper step");
        assert!(!w.disposition_is_refused_helper_work);
        assert!(
            check_c6_scalar_failure_step_circuit(&circuit(&w), &w).is_ok(),
            "owner-produced witness preprocessing"
        );
    }

    #[test]
    fn zero_value_slots_unreachable_active_scalar_failure_boundary() {
        // U-D5 boundary/obstruction only. This is NOT an active admitted
        // zero-cap ScalarFailure witness and does not close U-D5: with
        // value_slots = 0, R5 refuses every positive fuel before genesis,
        // and the zero-fuel genesis is consumed by owner_step P2
        // (Exhausted{Fuel}) before any Scalar helper can be selected.
        use mori_core::sherlock::helper_machine::complete_observation::record_step as owner_record;
        use mori_core::sherlock::helper_machine::shared::i128_class::KIntC;
        use mori_core::sherlock::helper_machine::shared::owner_host::host_owner_engine;
        use mori_core::sherlock::helper_machine::shared::{
            CapacityKind, CoreExpr, CoreProgram, CoreValue, LayoutSchemaC, NeededNatC, OwnerEngine,
            Tooling,
        };
        let program = CoreProgram {
            functions: Vec::new(),
            entry: CoreExpr::Literal {
                value: CoreValue::UInt {
                    n: KIntC::InRange(0),
                },
            },
        };
        let b = bounds(3, 100);
        let layout = LayoutSchemaC {
            layout_version: ("lv".into(), "1".into()),
            field_id: None,
            widths: None,
            scalar_layout_id: None,
            bounds: b,
            capacities: LayoutCapacitiesC {
                name_rows: 16,
                name_bytes: 256,
                occurrence_path_slots: 32,
                value_slots: 0,
            },
            ext_capacities: None,
            boundary_capacities_present: false,
        };
        let engine = host_owner_engine();
        let admitted = match engine.admit(&program, &b, &layout) {
            Ok(admitted) => admitted,
            Err(e) => panic!("admit zero-cap program: {}", tooling_label(&e)),
        };
        // R5 exact refusal: positive fuel under a zero value_slots cap is
        // CapacityNotAdmitted{ValueSlots, Exact(1), limit 0, no locator},
        // returned before any genesis exists.
        match engine.admit_run(
            &admitted,
            &RunIntC::Nat(1),
            &RunIntC::Nat(3),
            &FinanceInput::Absent,
        ) {
            Err(Tooling::CapacityNotAdmitted {
                kind: CapacityKind::ValueSlots,
                needed: NeededNatC::Exact(1),
                limit: 0,
                locator: None,
            }) => {}
            Err(e) => panic!(
                "zero-cap positive fuel: unexpected refusal: {}",
                tooling_label(&e)
            ),
            Ok(_) => panic!("zero-cap positive fuel must be refused at R5"),
        }
        // Zero fuel is the only admitted zero-cap genesis. Its first real
        // owner step is the P2 fuel publication, not a Scalar selection:
        // before has remaining_work 0 and no helper; after is Exhausted{Fuel}.
        let mut image0 = match engine.admit_run(
            &admitted,
            &RunIntC::Nat(0),
            &RunIntC::Nat(3),
            &FinanceInput::Absent,
        ) {
            Ok(image) => image,
            Err(e) => panic!("zero-cap zero-fuel genesis: {}", tooling_label(&e)),
        };
        match owner_record(&admitted, &mut image0) {
            Ok(ObservedStep::Event(r)) => {
                assert_eq!(
                    r.before.image.initial_fuel, 0,
                    "zero-fuel genesis initial fuel"
                );
                assert_eq!(
                    r.before.image.remaining_work, 0,
                    "zero-fuel genesis remaining work"
                );
                assert!(
                    r.before.image.helper.is_none(),
                    "zero-fuel genesis has no Scalar helper"
                );
                assert_eq!(
                    r.event.rule_id,
                    RuleIdC::KAdminPublishExhausted,
                    "P2 publishes exhausted fuel"
                );
                assert_eq!(
                    r.event.disposition,
                    StepDisposition::ExhaustedFuelUncharged,
                    "P2 is uncharged"
                );
                assert_eq!(
                    r.event.exhausted_kind,
                    Some(ExhaustReason::Fuel),
                    "P2 exhausted kind is Fuel"
                );
                assert_eq!(
                    r.after.image.outcome,
                    OutcomeC::Exhausted {
                        reason: ExhaustReason::Fuel
                    },
                    "zero-fuel genesis terminates as Exhausted{{Fuel}}, never ScalarFailure"
                );
            }
            Ok(ObservedStep::Terminal { .. }) => {
                panic!("zero-fuel genesis must publish P2, not Terminal")
            }
            Err(e) => panic!("zero-fuel genesis step: {}", capture_step_error_label(&e)),
        }
        // The selected-step wrapper on a fresh zero-fuel genesis therefore
        // refuses as UnexpectedOwnerStep; no active zero-cap ScalarFailure
        // witness exists on this public path.
        let mut image0b = match engine.admit_run(
            &admitted,
            &RunIntC::Nat(0),
            &RunIntC::Nat(3),
            &FinanceInput::Absent,
        ) {
            Ok(image) => image,
            Err(e) => panic!(
                "zero-cap zero-fuel genesis (wrapper): {}",
                tooling_label(&e)
            ),
        };
        let cfg0 = C6HostConfig {
            layout: LayoutSnapshot {
                layout_version: ("lv".into(), "1".into()),
                field_id: None,
                widths: None,
                scalar_layout_id: None,
                bounds: b,
                capacities: layout.capacities,
                boundary_capacities_present: false,
            },
            ext: None,
            budgets: mori_core::sherlock::helper_machine::c6_phys::C6CodecBudgets {
                type_expr_nodes: 256,
                template_bytes: 4096,
            },
        };
        assert!(
            matches!(
                record_scalar_failure_step(&admitted, &mut image0b, &cfg0, &limits()),
                Err(C6ScalarFailureStepError::UnexpectedOwnerStep)
            ),
            "zero-fuel genesis cannot be a selected ScalarFailure step"
        );
    }

    #[test]
    fn real_owner_sort_mismatch_code11_preprocessing() {
        // U-D6 genuine SI21 sort denotation: Add(UInt, Bool) admitted and
        // stepped only through the real public owner. The Scalar arithmetic
        // row stages PrimError{11} because operand_b is Bool, not UInt.
        // This is selected real-code-11 coverage only; it does not prove
        // full SI21/StepInv, and the code-10 overflow fixture is unchanged.
        use mori_core::sherlock::helper_machine::complete_observation::record_step as owner_record;
        use mori_core::sherlock::helper_machine::shared::i128_class::KIntC;
        use mori_core::sherlock::helper_machine::shared::owner_host::host_owner_engine;
        use mori_core::sherlock::helper_machine::shared::{
            CoreExpr, CoreProgram, CoreValue, HelperControlC as HC, LayoutSchemaC, OwnerEngine,
            ScalarPhase as SP, ScalarStaged as SS,
        };
        let uint = |n: u128| {
            Box::new(CoreExpr::Literal {
                value: CoreValue::UInt {
                    n: KIntC::InRange(n),
                },
            })
        };
        let boolean = |v: bool| {
            Box::new(CoreExpr::Literal {
                value: CoreValue::Bool { b: v },
            })
        };
        let program = CoreProgram {
            functions: Vec::new(),
            entry: CoreExpr::Binary {
                op: BinaryOp::Add,
                left: uint(1),
                right: boolean(true),
            },
        };
        let b = bounds(3, 100);
        let layout = LayoutSchemaC {
            layout_version: ("lv".into(), "1".into()),
            field_id: None,
            widths: None,
            scalar_layout_id: None,
            bounds: b,
            capacities: LayoutCapacitiesC {
                name_rows: 16,
                name_bytes: 256,
                occurrence_path_slots: 32,
                value_slots: 64,
            },
            ext_capacities: None,
            boundary_capacities_present: false,
        };
        let engine = host_owner_engine();
        let admitted = match engine.admit(&program, &b, &layout) {
            Ok(admitted) => admitted,
            Err(e) => panic!("admit sort-mismatch program: {}", tooling_label(&e)),
        };
        let mut image = match engine.admit_run(
            &admitted,
            &RunIntC::Nat(64),
            &RunIntC::Nat(3),
            &FinanceInput::Absent,
        ) {
            Ok(image) => image,
            Err(e) => panic!("genesis sort-mismatch: {}", tooling_label(&e)),
        };
        // Bounded search for the real selected pre-state; the wrapper below
        // is the one and only step taken on that selected image.
        let mut selected = false;
        for _ in 0..64 {
            let step = match owner_record(&admitted, &mut image) {
                Ok(step) => step,
                Err(e) => panic!("search step: {}", capture_step_error_label(&e)),
            };
            let after = match &step {
                ObservedStep::Event(r) => &r.after,
                ObservedStep::Terminal { after, .. } => panic!(
                    "terminal before Scalar Done/PrimError 11: outcome {:?}",
                    after.image.outcome
                ),
            };
            if matches!(
                after.image.helper,
                Some(HC::Scalar {
                    mode: ScalarMode::Add,
                    phase: SP::Done,
                    staged: SS::PrimError { code: 11 },
                    ..
                })
            ) {
                selected = true;
                break;
            }
        }
        assert!(
            selected,
            "owner must reach Scalar Add Done/PrimError code 11 from UInt+Bool"
        );
        let cfg2 = C6HostConfig {
            layout: LayoutSnapshot {
                layout_version: ("lv".into(), "1".into()),
                field_id: None,
                widths: None,
                scalar_layout_id: None,
                bounds: b,
                capacities: layout.capacities,
                boundary_capacities_present: false,
            },
            ext: None,
            budgets: mori_core::sherlock::helper_machine::c6_phys::C6CodecBudgets {
                type_expr_nodes: 256,
                template_bytes: 4096,
            },
        };
        let w = record_scalar_failure_step(&admitted, &mut image, &cfg2, &limits())
            .expect("selected code-11 wrapper step");
        assert!(!w.disposition_is_refused_helper_work);
        assert_eq!(
            w.pre_helper[L_HELPER_RECORD + 16],
            11,
            "staged code is the real 11"
        );
        assert_eq!(
            w.post_helper[L_RESULT_RECORD + 1],
            11,
            "result code is the selected real 11, not mutated 10"
        );
        assert!(
            check_c6_scalar_failure_step_circuit(&circuit(&w), &w).is_ok(),
            "owner-produced code-11 witness preprocessing"
        );
    }
    // ---- C8 helpers: raw-Fr statement harness and mode-aware circuits ----

    fn two_pow_128() -> Fr {
        Fr::from(u128::MAX) + Fr::from(1u64)
    }

    /// Direct raw-statement preprocessing for values the u128 witness API
    /// cannot express (a lane at exactly 2^128, gate value 2). This is the
    /// preprocessing boundary only: rejection here is an SDK preprocessing
    /// refusal, not a synthesis constraint result and not a panic. The
    /// LocalPublic preimage shape matches `check_c6_scalar_failure_step_circuit`.
    fn check_raw_statement(
        c: &C6ScalarFailureStepCircuit,
        statement: Vec<Fr>,
    ) -> Result<Vec<Option<usize>>, C6ScalarFailureStepError> {
        let preimage = ProofPreimage {
            inputs: Vec::new(),
            private_transcript: Vec::new(),
            public_transcript_inputs: statement.clone(),
            public_transcript_outputs: statement,
            binding_input: Fr::from(0u64),
            communications_commitment: None,
            key_location: KeyLocation(CHECK_KEY_LOCATION.into()),
        };
        preimage.check(&c.ir).map_err(|e| {
            C6ScalarFailureStepError::Backend(format!("SDK ProofPreimage::check failed: {e}"))
        })
    }

    fn circuit_mode(
        w: &C6ScalarFailureStepWitness,
        mode: C6PushStatementBindingMode,
    ) -> C6ScalarFailureStepCircuit {
        build_c6_scalar_failure_step_circuit(
            w.source_params().clone(),
            w.host_prefix.meta.len(),
            w.host_prefix.prog.len(),
            w.host_prefix.meta_bounds_frames_offset,
            mode,
            "c6_sf_test",
        )
        .unwrap()
    }

    fn assert_raw_rejects(c: &C6ScalarFailureStepCircuit, statement: Vec<Fr>, label: &str) {
        match check_raw_statement(c, statement) {
            Err(C6ScalarFailureStepError::Backend(m)) => assert!(
                m.starts_with("SDK ProofPreimage::check failed:"),
                "{label}: {m}"
            ),
            other => panic!(
                "{label}: expected preprocessing rejection, got ok={}",
                other.is_ok()
            ),
        }
    }

    fn set_both(w: &mut C6ScalarFailureStepWitness, section: u8, lane: usize, value: u128) {
        // section: 0 image, 1 data, 2 helper. Pre and post are set together
        // where the relation requires preservation, so the mutation isolates
        // the validator/cap predicate rather than the preservation equality.
        match section {
            0 => {
                assert_ne!(w.pre_image[lane], value, "image mutation no-op");
                w.pre_image[lane] = value;
                w.post_image[lane] = value;
            }
            1 => {
                assert_ne!(w.pre_data[lane], value, "data mutation no-op");
                w.pre_data[lane] = value;
                w.post_data[lane] = value;
            }
            _ => {
                assert_ne!(w.pre_helper[lane], value, "helper mutation no-op");
                w.pre_helper[lane] = value;
                w.post_helper[lane] = value;
            }
        }
    }

    fn base_witness() -> (C6ScalarFailureStepWitness, C6ScalarFailureStepCircuit) {
        let cfg = config();
        let pre = sample(2, 100);
        let post = applied_post(&pre);
        let w = witness_from_host_pair(true, &pre, &post, &cfg, &limits()).unwrap();
        let c = circuit(&w);
        assert!(
            check_c6_scalar_failure_step_circuit(&c, &w).is_ok(),
            "positive control"
        );
        (w, c)
    }

    #[test]
    fn ledger_call_positive_and_negative() {
        let cfg = config();
        let pre = sample(2, 100);
        let post = applied_post(&pre);
        let w = witness_from_host_pair(true, &pre, &post, &cfg, &limits()).unwrap();
        let lc = circuit_mode(&w, C6PushStatementBindingMode::LedgerCall);
        // Positive: real Output, Impact ordering, communications commitment,
        // and PI-skip semantics all run through ProofPreimage::check.
        assert!(
            check_c6_scalar_failure_step_circuit(&lc, &w).is_ok(),
            "LedgerCall positive"
        );
        // Negative: a wrong post lane reaches LedgerCall checking and is
        // rejected at the preprocessing stage, not filtered by a producer.
        let mut bad = w.clone();
        bad.post_helper[L_RESULT_RECORD + 1] = 11;
        match check_c6_scalar_failure_step_circuit(&lc, &bad) {
            Err(C6ScalarFailureStepError::Backend(m)) => assert!(
                m.starts_with("SDK ProofPreimage::check failed:"),
                "LedgerCall negative: {m}"
            ),
            other => panic!(
                "LedgerCall negative: expected preprocessing rejection, got ok={}",
                other.is_ok()
            ),
        }
        // Disabled LedgerCall equality also checks.
        let wd = witness_from_host_pair(false, &pre, &pre, &cfg, &limits()).unwrap();
        let lcd = circuit_mode(&wd, C6PushStatementBindingMode::LedgerCall);
        assert!(
            check_c6_scalar_failure_step_circuit(&lcd, &wd).is_ok(),
            "LedgerCall disabled"
        );
    }

    #[test]
    fn raw_lane_at_two_pow_128_and_gate_two_reject_at_preprocessing() {
        let (w, c) = base_witness();
        let statement = w.to_statement_frs();
        assert!(
            check_raw_statement(&c, statement.clone()).is_ok(),
            "raw positive control"
        );
        // Isolated range negative: the initial-fuel lane (no cap, not a
        // mutable lane) set to 2^128 on BOTH statement copies, so gated
        // preservation and every canonical predicate hold and only the
        // unconditional ConstrainBits(128) range can reject.
        let prefix_len = w.host_prefix.meta.len() + w.host_prefix.prog.len() + CTX_LANES;
        let half_len = prefix_len + w.pre_image.len() + w.pre_data.len() + w.pre_helper.len();
        let fuel_pre = 4 + prefix_len + L_INITIAL_FUEL;
        let fuel_post = fuel_pre + half_len;
        assert_eq!(
            statement[fuel_pre], statement[fuel_post],
            "fuel lane copies agree"
        );
        let mut lane_pow2 = statement.clone();
        lane_pow2[fuel_pre] = two_pow_128();
        lane_pow2[fuel_post] = two_pow_128();
        assert_raw_rejects(&c, lane_pow2, "lane at 2^128 gate 1 statement");
        // Gate 0: same two-sided construction on a disabled statement.
        let mut disabled_raw = w.clone();
        disabled_raw.enable = false;
        disabled_raw.post_image = disabled_raw.pre_image.clone();
        disabled_raw.post_helper = disabled_raw.pre_helper.clone();
        disabled_raw.post_data = disabled_raw.pre_data.clone();
        let mut lane_pow2_g0 = disabled_raw.to_statement_frs();
        assert_eq!(
            lane_pow2_g0[0],
            Fr::from(0u64),
            "disabled statement gate lane is 0"
        );
        lane_pow2_g0[fuel_pre] = two_pow_128();
        lane_pow2_g0[fuel_post] = two_pow_128();
        assert_raw_rejects(&c, lane_pow2_g0, "lane at 2^128 gate 0 statement");
        // Gate value 2 is outside the unconditional boolean gate domain.
        let mut gate_two = statement.clone();
        gate_two[0] = Fr::from(2u64);
        assert_raw_rejects(&c, gate_two, "gate value 2");
        // Gate 0 with a canonical-violating lane still accepts.
        let mut disabled_bad = w.clone();
        disabled_bad.enable = false;
        disabled_bad.post_image = disabled_bad.pre_image.clone();
        disabled_bad.post_helper = disabled_bad.pre_helper.clone();
        disabled_bad.post_data = disabled_bad.pre_data.clone();
        disabled_bad.pre_image[L_CONTROL + 3] = 1;
        disabled_bad.post_image[L_CONTROL + 3] = 1;
        assert!(
            check_c6_scalar_failure_step_circuit(&circuit(&disabled_bad), &disabled_bad).is_ok(),
            "gate 0 imposes no canonicality beyond ranges"
        );
    }

    #[test]
    fn control_lane3_padding_regressions_all_variants() {
        let (w, c) = base_witness();
        // Reviewer counterexample: valid Applied Return control with lane 3
        // set on both copies must reject at preprocessing (Core decode_control
        // requires Return's lanes after its 3-lane record to be zero).
        let mut bad = w.clone();
        set_both(&mut bad, 0, L_CONTROL + 3, 1);
        assert_pre_rejects(&bad, "Return lane 3 padding");
        // Eval tag 0 with a valid expression index and the same padding.
        let mut eval_bad = w.clone();
        set_both(&mut eval_bad, 0, L_CONTROL, 0);
        set_both(&mut eval_bad, 0, L_CONTROL + 1, 0);
        set_both(&mut eval_bad, 0, L_CONTROL + 3, 1);
        // Zero the remaining Return ref lane so only lane 3 is noncanonical.
        eval_bad.pre_image[L_CONTROL + 2] = 0;
        eval_bad.post_image[L_CONTROL + 2] = 0;
        assert_pre_rejects(&eval_bad, "Eval lane 3 padding");
        // Error tag 9, used 2: lane 3 is padding.
        let mut err_bad = w.clone();
        set_both(&mut err_bad, 0, L_CONTROL, 9);
        set_both(&mut err_bad, 0, L_CONTROL + 1, 4);
        err_bad.pre_image[L_CONTROL + 2] = 0;
        err_bad.post_image[L_CONTROL + 2] = 0;
        set_both(&mut err_bad, 0, L_CONTROL + 3, 1);
        assert_pre_rejects(&err_bad, "Error lane 3 padding");
        // Positive canonical controls for all 11 tags through this module's
        // own control validator. These are validator-domain cases, outside
        // the full selected SI site/control coherence premise.
        let cases: [(u128, [u128; 16]); 11] = [
            (0, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), // Eval expr 0
            (1, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), // Return Const 0
            (2, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), // ScheduleNext LetF frame
            (3, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), // Invoke name 0, empty args
            (4, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), // Enter absent, empty args
            (5, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), // Bound Bad, type 0, body 0
            (6, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), // Cases Const 0, empty arms
            (7, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), // CaseResult NoMatch
            (8, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), // PrimResult Value Const 0
            (9, [4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), // Error code 4
            (10, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), // Materialize type/ctor 0
        ];
        for (tag, rest) in cases {
            let mut v = w.clone();
            v.pre_image[L_CONTROL] = tag;
            v.post_image[L_CONTROL] = tag;
            for (i, value) in rest.iter().enumerate() {
                v.pre_image[L_CONTROL + 1 + i] = *value;
                v.post_image[L_CONTROL + 1 + i] = *value;
            }
            assert!(
                check_c6_scalar_failure_step_circuit(&c, &v).is_ok(),
                "canonical control tag {tag}"
            );
        }
        // PrimResult Error: lane 2 is the code (< 256); lane 3 must be zero.
        // The base fixture control is Return Data(0): raw lanes
        // [tag=1, ref tag=1, ref index=0, lane 3=0]. The Error sub-tag
        // lane (+1) is therefore already 1, so it is asserted, not passed
        // through the changed-lane mutation helper.
        let mut prim_err = w.clone();
        assert_eq!(
            prim_err.pre_image[L_CONTROL + 1],
            1,
            "base Return ref tag is Data"
        );
        assert_eq!(
            prim_err.post_image[L_CONTROL + 1],
            1,
            "base Return ref tag is Data"
        );
        set_both(&mut prim_err, 0, L_CONTROL, 8);
        set_both(&mut prim_err, 0, L_CONTROL + 2, 10);
        assert_eq!(
            prim_err.pre_image[L_CONTROL + 3],
            0,
            "Error lane 3 padding already zero"
        );
        assert_eq!(
            prim_err.post_image[L_CONTROL + 3],
            0,
            "Error lane 3 padding already zero"
        );
        assert!(
            check_c6_scalar_failure_step_circuit(&c, &prim_err).is_ok(),
            "PrimResult Error canonical"
        );
        let mut prim_err_bad = prim_err.clone();
        set_both(&mut prim_err_bad, 0, L_CONTROL + 3, 1);
        assert_pre_rejects(&prim_err_bad, "PrimResult Error lane 3 padding");
        // PrimResult Value: lane +2 is the ref tag and lane +3 is the ref
        // index, and the index may be nonzero. Data(1) is a nonzero index
        // strictly inside the fixture data cap (data_nodes = 2).
        assert!(
            w.source_params().caps().phys.max_data_nodes > 1,
            "Data(1) inside fixture data cap"
        );
        let mut prim_val = w.clone();
        set_both(&mut prim_val, 0, L_CONTROL, 8);
        set_both(&mut prim_val, 0, L_CONTROL + 1, 0);
        set_both(&mut prim_val, 0, L_CONTROL + 2, 1);
        set_both(&mut prim_val, 0, L_CONTROL + 3, 1);
        assert!(
            check_c6_scalar_failure_step_circuit(&c, &prim_val).is_ok(),
            "PrimResult Value Data(1) canonical"
        );
    }

    #[test]
    fn control_all_variant_all_unused_padding_regressions() {
        let (w, c) = base_witness();
        // (tag, payload lanes 1..17, used-lane count incl. tag lane)
        let cases: [(u128, [u128; 16], usize); 11] = [
            (0, [0; 16], 2),
            (1, [0; 16], 3),
            (2, [0; 16], 17),
            (3, [0; 16], 4),
            (4, [0; 16], 5),
            (5, [0; 16], 6),
            (6, [0; 16], 7),
            (7, [0; 16], 11),
            (8, [0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 4), // PrimResult Value Data(1)
            (9, [4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 2), // Error code 4
            (10, [0; 16], 5),
        ];
        for (tag, rest, used) in cases {
            let mut v = w.clone();
            v.pre_image[L_CONTROL] = tag;
            v.post_image[L_CONTROL] = tag;
            for (i, value) in rest.iter().enumerate() {
                v.pre_image[L_CONTROL + 1 + i] = *value;
                v.post_image[L_CONTROL + 1 + i] = *value;
            }
            assert!(
                check_c6_scalar_failure_step_circuit(&c, &v).is_ok(),
                "canonical control tag {tag}"
            );
            for lane in used..17usize {
                let mut bad = v.clone();
                bad.pre_image[L_CONTROL + lane] = 1;
                bad.post_image[L_CONTROL + lane] = 1;
                assert_pre_rejects(&bad, "control tag padding lane");
            }
        }
        // PrimResult Error (sub-tag 1, code 10): used 3, tail 3..17 zero.
        let mut prim_err = w.clone();
        set_both(&mut prim_err, 0, L_CONTROL, 8);
        set_both(&mut prim_err, 0, L_CONTROL + 2, 10);
        assert_eq!(prim_err.pre_image[L_CONTROL + 1], 1, "Error sub-tag lane");
        assert!(
            check_c6_scalar_failure_step_circuit(&c, &prim_err).is_ok(),
            "PrimResult Error canonical"
        );
        for lane in 3..17usize {
            let mut bad = prim_err.clone();
            bad.pre_image[L_CONTROL + lane] = 1;
            bad.post_image[L_CONTROL + lane] = 1;
            assert_pre_rejects(&bad, "PrimResult Error padding lane");
        }
    }
    #[test]
    fn frame_variants_and_maximal_unused_lanes() {
        let (w, c) = base_witness();
        let slot = w.source_params.geometry().frame_slot_offset(0);
        // All 10 frame tags as canonical validator-domain cases on the one
        // active frame slot (validator domain only; outside SI08/SI15
        // selected-premise claims).
        let cases: [(u128, [u128; 12]); 10] = [
            (0, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), // LetF
            (1, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), // LeftF Add
            (2, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), // RightF Add Const 0
            (3, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), // UnaryF Leaf
            (4, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), // IfF
            (5, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), // MatchF empty arms
            (6, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), // ArgsF
            (7, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), // ReturnF type 0
            (8, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), // ScopeF
            (9, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), // ConstructorFieldsF
        ];
        for (tag, rest) in cases {
            let mut v = w.clone();
            v.pre_image[slot] = tag;
            v.post_image[slot] = tag;
            for (i, value) in rest.iter().enumerate() {
                v.pre_image[slot + 1 + i] = *value;
                v.post_image[slot + 1 + i] = *value;
            }
            assert!(
                check_c6_scalar_failure_step_circuit(&c, &v).is_ok(),
                "canonical frame tag {tag}"
            );
            // A nonzero maximal unused lane beyond the variant's used count
            // is rejected. Lane 12 is unused for every tag except
            // ConstructorFieldsF (used 13), exercised separately below.
            if tag != 9 {
                let mut badf = v.clone();
                badf.pre_image[slot + 12] = u128::MAX;
                badf.post_image[slot + 12] = u128::MAX;
                assert_pre_rejects(&badf, "frame maximal unused lane");
            }
        }
        // ConstructorFieldsF with a nonzero origin and a nonzero payload
        // (the all-zero tag-9 case above is not this regression): type,
        // ctor, index, and field type 1; checked; a completed slice of
        // length 1; an empty rest slice; no saved env; origin 1. Every
        // value is inside the actual frame caps. Validator domain only.
        let mut ctor = w.clone();
        let ctor_rest: [u128; 12] = [1, 1, 1, 1, 1, 0, 1, 0, 0, 0, 0, 1];
        ctor.pre_image[slot] = 9;
        ctor.post_image[slot] = 9;
        for (i, value) in ctor_rest.iter().enumerate() {
            ctor.pre_image[slot + 1 + i] = *value;
            ctor.post_image[slot + 1 + i] = *value;
        }
        assert!(
            check_c6_scalar_failure_step_circuit(&c, &ctor).is_ok(),
            "ConstructorFieldsF nonzero origin/payload"
        );
        // Origin at the expression cap is outside the frame cap and
        // rejects at preprocessing (a genuine changed-lane mutation).
        let mut ctor_bad = ctor.clone();
        let origin_cap = w.source_params().caps().phys.max_expressions as u128;
        ctor_bad.pre_image[slot + 12] = origin_cap;
        ctor_bad.post_image[slot + 12] = origin_cap;
        assert_pre_rejects(&ctor_bad, "ConstructorFieldsF origin at expression cap");
        // Nonmatching control variant with maximal unused slice lanes at
        // gate 1 rejects; the same lanes are the DATA-derived-end domain in
        // the disabled test below.
        let mut bad = w.clone();
        set_both(&mut bad, 0, L_CONTROL + 16, u128::MAX);
        assert_pre_rejects(&bad, "control maximal unused lane");
    }

    #[test]
    fn disabled_derived_slice_ends_at_field_boundaries() {
        let (w, c) = base_witness();
        // Build a raw statement whose active frame is a MatchF whose arms
        // slice is (u128::MAX, 1): the derived end is exactly 2^128. At
        // gate 1 the 129-bit end check rejects; this is exercised through
        // preprocessing on u128-expressible lanes (start/len are u128; the
        // end is derived in-circuit, as in the DATA regression pattern).
        // Positive canonical MatchF control: arms (0, 1), saved env absent
        // with index 0, origin 0. Only the slice lanes change below.
        let slot = w.source_params.geometry().frame_slot_offset(0);
        let mut matchf_ok = w.clone();
        matchf_ok.pre_image[slot] = 5;
        matchf_ok.post_image[slot] = 5;
        matchf_ok.pre_image[slot + 1] = 0;
        matchf_ok.post_image[slot + 1] = 0;
        matchf_ok.pre_image[slot + 2] = 1;
        matchf_ok.post_image[slot + 2] = 1;
        matchf_ok.pre_image[slot + 3] = 0;
        matchf_ok.post_image[slot + 3] = 0;
        matchf_ok.pre_image[slot + 4] = 0;
        matchf_ok.post_image[slot + 4] = 0;
        matchf_ok.pre_image[slot + 5] = 0;
        matchf_ok.post_image[slot + 5] = 0;
        assert!(
            check_c6_scalar_failure_step_circuit(&c, &matchf_ok).is_ok(),
            "canonical MatchF control"
        );
        // Derived end 2^128: arms (MAX, 1). The old RightF origin at
        // slot+4 is explicitly zeroed on both copies so the absent-env
        // index is canonical and the 129-bit end is the only failure.
        let mut end_pow2 = matchf_ok.clone();
        end_pow2.pre_image[slot + 1] = u128::MAX;
        end_pow2.post_image[slot + 1] = u128::MAX;
        assert_eq!(end_pow2.pre_image[slot + 3], 0, "MatchF env absent");
        assert_eq!(
            end_pow2.pre_image[slot + 4],
            0,
            "MatchF absent-env index zero"
        );
        assert_eq!(
            end_pow2.post_image[slot + 4],
            0,
            "MatchF absent-env index zero (post)"
        );
        assert_pre_rejects(&end_pow2, "frame slice end at 2^128");
        // Stronger: (MAX, MAX) gives end 2^129 - 2, inheriting the setup.
        let mut end_max = end_pow2.clone();
        end_max.pre_image[slot + 2] = u128::MAX;
        end_max.post_image[slot + 2] = u128::MAX;
        assert_pre_rejects(&end_max, "frame slice end at 2^129-2");
        // At gate 0 the validator imposes nothing beyond unconditional
        // ranges, so maximal nonmatching slice lanes on a disabled step
        // check. The pre/post are equal, preserving the disabled relation.
        let mut disabled = w.clone();
        disabled.enable = false;
        disabled.post_image = disabled.pre_image.clone();
        disabled.post_data = disabled.pre_data.clone();
        disabled.post_helper = disabled.pre_helper.clone();
        disabled.pre_image[slot] = 5;
        disabled.post_image[slot] = 5;
        disabled.pre_image[slot + 1] = u128::MAX;
        disabled.post_image[slot + 1] = u128::MAX;
        disabled.pre_image[slot + 2] = u128::MAX;
        disabled.post_image[slot + 2] = u128::MAX;
        assert!(
            check_c6_scalar_failure_step_circuit(&c, &disabled).is_ok(),
            "disabled maximal slice lanes impose no end bound"
        );
        // The exact-boundary disabled case: (u128::MAX, 1) derives end
        // exactly 2^128. Gate 1 rejects this end (end_pow2 above); at
        // gate 0 the validator imposes no end bound, so it checks.
        let mut disabled_pow2 = disabled.clone();
        disabled_pow2.pre_image[slot + 2] = 1;
        disabled_pow2.post_image[slot + 2] = 1;
        assert!(
            check_c6_scalar_failure_step_circuit(&c, &disabled_pow2).is_ok(),
            "disabled frame slice end at 2^128 imposes no end bound"
        );
    }

    #[test]
    fn root_value_slots_caps_at_and_above() {
        let (w, c) = base_witness();
        let cap = w.source_params().caps().phys.max_value_slots as u128;
        for lane in [38usize, 42usize] {
            let mut at_cap = w.clone();
            set_both(&mut at_cap, 0, lane, cap);
            assert!(
                check_c6_scalar_failure_step_circuit(&c, &at_cap).is_ok(),
                "root lane {lane} at cap"
            );
            let mut above = w.clone();
            set_both(&mut above, 0, lane, cap + 1);
            assert_pre_rejects(&above, "root value_slots above cap");
        }
        // Zero-capacity geometry through the actual checked API. A caps
        // copy with max_value_slots = 0 is geometrically representable:
        // checked_new succeeds and the derived value-slot lane total is
        // exactly zero. This is the empty/disabled-domain geometry; it
        // is not, by itself, an active ScalarFailure witness.
        let mut zero_caps = w.source_params().caps();
        zero_caps.phys.max_value_slots = 0;
        let zero_geom = C6PaddedCarrierGeometry::checked_new(
            zero_caps,
            C6StateLimits {
                max_total_lanes: usize::MAX,
            },
        )
        .expect("zero value_slots geometry must be representable");
        assert_eq!(zero_geom.caps.phys.max_value_slots, 0, "zero cap retained");
        assert_eq!(
            zero_geom.value_slots_lanes, 0,
            "zero cap derives zero value-slot lanes"
        );
        // The current family's recomputed geometry is the admitted
        // fixture geometry (cap 2), so supplying the zero-cap geometry
        // to this family is refused as an independent geometry, never
        // silently adopted for an active witness.
        assert!(
            matches!(
                w.source_params().validate_geometry(zero_geom),
                Err(C6PushStepError::InvalidGeometry {
                    section: "geometry"
                })
            ),
            "zero-cap geometry refused for the cap-2 family"
        );
        // Actual checked usize::MAX overflow refusals, with no massive
        // allocation: checked_new performs checked_mul/checked_add on
        // lane counts only and refuses before any carrier exists.
        // max_value_slots * REF_LANES (2) overflows first in data.values.
        let mut max_value_caps = w.source_params().caps();
        max_value_caps.phys.max_value_slots = usize::MAX;
        assert!(
            matches!(
                C6PaddedCarrierGeometry::checked_new(
                    max_value_caps,
                    C6StateLimits {
                        max_total_lanes: usize::MAX
                    }
                ),
                Err(C6PushStepError::GeometryOverflow {
                    section: "data.values"
                })
            ),
            "usize::MAX value_slots cap must overflow in data.values"
        );
        // max_frames * FRAME_LANES (13) overflows first in image.
        let mut max_frame_caps = w.source_params().caps();
        max_frame_caps.phys.max_frames = usize::MAX;
        assert!(
            matches!(
                C6PaddedCarrierGeometry::checked_new(
                    max_frame_caps,
                    C6StateLimits {
                        max_total_lanes: usize::MAX
                    }
                ),
                Err(C6PushStepError::GeometryOverflow { section: "image" })
            ),
            "usize::MAX frames cap must overflow in image"
        );
    }

    #[test]
    fn helper_work_boundaries_and_charge_overflow() {
        let cfg = config();
        // max - 1 is Applied; max is the uncharged P3 refusal.
        let pre = sample(99, 100);
        let post = applied_post(&pre);
        let w = witness_from_host_pair(true, &pre, &post, &cfg, &limits()).unwrap();
        assert!(!w.disposition_is_refused_helper_work);
        assert!(
            check_c6_scalar_failure_step_circuit(&circuit(&w), &w).is_ok(),
            "helper work max-1"
        );
        let pre3 = sample(100, 100);
        let post3 = p3_post(&pre3);
        let w3 = witness_from_host_pair(true, &pre3, &post3, &cfg, &limits()).unwrap();
        assert!(w3.disposition_is_refused_helper_work);
        assert!(
            check_c6_scalar_failure_step_circuit(&circuit(&w3), &w3).is_ok(),
            "helper work max"
        );
        // helper_work at u128::MAX under a P3 bound equal to it charges
        // nothing and checks.
        let pre_max = sample(u128::MAX, u128::MAX);
        let post_max = p3_post(&pre_max);
        let wm = witness_from_host_pair(true, &pre_max, &post_max, &cfg, &limits()).unwrap();
        assert!(
            check_c6_scalar_failure_step_circuit(&circuit(&wm), &wm).is_ok(),
            "helper work u128::MAX P3"
        );
        // Would-be helper-work charge overflow on an Applied step: raw post
        // wraps to 0 and reaches the no-wrap assertion.
        let mut pre_o = sample(2, 100);
        pre_o.image.helper_work = u128::MAX;
        let mut post_o = pre_o.clone();
        post_o.image.helper = None;
        post_o.image.result = Some(HelperResultC::ScalarError { code: 10 });
        post_o.image.remaining_work -= 1;
        post_o.image.consumed_work += 1;
        post_o.image.helper_work = 0;
        let wo = witness_from_host_pair(true, &pre_o, &post_o, &cfg, &limits()).unwrap();
        assert_pre_rejects(&wo, "helper-work charge overflow wrap");
        // Raw disposition-bit mismatch: the host-pair route derives the bit,
        // so mutate the witness field directly. It must reach the circuit.
        let (bw, _c) = base_witness();
        let mut flipped = bw.clone();
        flipped.disposition_is_refused_helper_work = true;
        assert_pre_rejects(&flipped, "disposition bit mismatch");
        // Zero remaining fuel on the selected pre-state is the distinct
        // driver P2 branch, refused by the circuit's positive-fuel gate.
        let mut pre_z = sample(2, 100);
        pre_z.image.remaining_work = 0;
        let mut post_z = pre_z.clone();
        post_z.image.helper = None;
        post_z.image.result = Some(HelperResultC::ScalarError { code: 10 });
        post_z.image.consumed_work += 1;
        post_z.image.helper_work += 1;
        let wz = witness_from_host_pair(true, &pre_z, &post_z, &cfg, &limits()).unwrap();
        assert_pre_rejects(&wz, "zero remaining fuel selected pre-state");
    }

    #[test]
    fn scalar_code_mode_and_site_predicate_domain() {
        // Emitted-predicate domain only: this exercises the circuit's code
        // 10|11 and mode gating, not SI21 denotation for manufactured
        // states. Denotationally consistent SI-positive cases are the
        // overflow Add fixture and the real-owner test.
        let (w, c) = base_witness();
        let mode_lane = L_HELPER_RECORD + 5;
        let code_lane = L_HELPER_RECORD + 16;
        // Code 10 is accepted only for Add/Sub/Mul modes 0..2. The base
        // fixture is Add/overflow (code 10). Mutating the mode to Sub keeps
        // the emitted arithmetic gate; to Lt (mode 3) it must reject.
        for (mode, ok) in [(0u128, true), (1, true), (2, true), (3, false), (4, false)] {
            let mut v = w.clone();
            v.pre_helper[mode_lane] = mode;
            // Keep the post result code equal to the staged code (10).
            assert_eq!(v.post_helper[L_RESULT_RECORD + 1], 10);
            assert_eq!(
                check_c6_scalar_failure_step_circuit(&c, &v).is_ok(),
                ok,
                "mode {mode} code 10"
            );
        }
        // Code 11 is accepted for non-arithmetic modes through the emitted
        // gate. Lt with code 11 checks; Lt with code 10 (above) rejects.
        let mut lt11 = w.clone();
        lt11.pre_helper[mode_lane] = 3;
        lt11.pre_helper[code_lane] = 11;
        lt11.post_helper[L_RESULT_RECORD + 1] = 11;
        assert!(
            check_c6_scalar_failure_step_circuit(&c, &lt11).is_ok(),
            "Lt code 11 emitted gate"
        );
        // Cons (mode 4) and Branch (mode 5) are binary non-arithmetic
        // shapes: code 11 checks with the base fixture's present
        // operand_b (Data(0)); code 10 is outside the arithmetic gate
        // and rejects. Validator domain only, not an SI21 denotation.
        for mode in [4u128, 5u128] {
            let mut v = w.clone();
            v.pre_helper[mode_lane] = mode;
            v.pre_helper[code_lane] = 11;
            v.post_helper[L_RESULT_RECORD + 1] = 11;
            assert!(
                check_c6_scalar_failure_step_circuit(&c, &v).is_ok(),
                "mode {mode} code 11 emitted shape"
            );
            let mut bad = v.clone();
            bad.pre_helper[code_lane] = 10;
            bad.post_helper[L_RESULT_RECORD + 1] = 10;
            assert_pre_rejects(&bad, "binary non-arithmetic mode code 10");
        }
        // Leaf (mode 6) is a unary shape: operand_b must be absent in
        // the emitted shape, by the same lane pattern as Not below.
        let mut leaf11 = w.clone();
        leaf11.pre_helper[mode_lane] = 6;
        leaf11.pre_helper[code_lane] = 11;
        leaf11.post_helper[L_RESULT_RECORD + 1] = 11;
        leaf11.pre_helper[L_HELPER_RECORD + 8] = 0;
        leaf11.pre_helper[L_HELPER_RECORD + 9] = 0;
        leaf11.pre_helper[L_HELPER_RECORD + 10] = 0;
        assert!(
            check_c6_scalar_failure_step_circuit(&c, &leaf11).is_ok(),
            "Leaf code 11 emitted shape"
        );
        // Not (mode 7): operand_b must be absent in the emitted shape.
        let mut not11 = w.clone();
        not11.pre_helper[mode_lane] = 7;
        not11.pre_helper[code_lane] = 11;
        not11.post_helper[L_RESULT_RECORD + 1] = 11;
        not11.pre_helper[L_HELPER_RECORD + 8] = 0;
        not11.pre_helper[L_HELPER_RECORD + 9] = 0;
        not11.pre_helper[L_HELPER_RECORD + 10] = 0;
        assert!(
            check_c6_scalar_failure_step_circuit(&c, &not11).is_ok(),
            "Not code 11 emitted shape"
        );
        let mut not10 = not11.clone();
        not10.pre_helper[code_lane] = 10;
        not10.post_helper[L_RESULT_RECORD + 1] = 10;
        assert_pre_rejects(&not10, "Not code 10 outside arithmetic");
        // Site beyond RetRight through the emitted site set, including
        // ConstructorFields (25): a validator-domain case.
        let mut ctor_site = w.clone();
        ctor_site.pre_helper[L_HELPER_RECORD + 1] = 25;
        assert!(
            check_c6_scalar_failure_step_circuit(&c, &ctor_site).is_ok(),
            "RetCtorFields site emitted set"
        );
        let mut bad_site = w.clone();
        bad_site.pre_helper[L_HELPER_RECORD + 1] = 14;
        assert_pre_rejects(&bad_site, "site outside emitted set");
        let _ = c;
    }

    #[test]
    fn namespace_reserved_scope_and_retained_serial_composition() {
        let (w, _c) = base_witness();
        let geom = w.source_params.geometry();
        let operand = |i: usize| Operand::Variable(Identifier(format!("%in.{i}")));
        let inputs = |enable: Operand| C6ScalarFailureStepInputs {
            enable,
            disposition_is_refused_helper_work: operand(900),
            host_schema: operand(901),
            carrier_version: operand(902),
            geometry: geom,
            meta_bounds_frames_offset: w.host_prefix.meta_bounds_frames_offset,
            source_params: w.source_params().clone(),
            pre_meta: w
                .host_prefix
                .meta
                .iter()
                .enumerate()
                .map(|(i, _)| operand(i))
                .collect(),
            pre_prog: w
                .host_prefix
                .prog
                .iter()
                .enumerate()
                .map(|(i, _)| operand(100 + i))
                .collect(),
            pre_ctx: core::array::from_fn(|i| operand(200 + i)),
            pre_image: (0..geom.image_total_lanes)
                .map(|i| operand(300 + i))
                .collect(),
            pre_data: (0..geom.data_total_lanes)
                .map(|i| operand(400 + i))
                .collect(),
            pre_helper: (0..geom.helper_total_lanes)
                .map(|i| operand(500 + i))
                .collect(),
            post_meta: w
                .host_prefix
                .meta
                .iter()
                .enumerate()
                .map(|(i, _)| operand(600 + i))
                .collect(),
            post_prog: w
                .host_prefix
                .prog
                .iter()
                .enumerate()
                .map(|(i, _)| operand(700 + i))
                .collect(),
            post_ctx: core::array::from_fn(|i| operand(800 + i)),
            post_image: (0..geom.image_total_lanes)
                .map(|i| operand(1000 + i))
                .collect(),
            post_data: (0..geom.data_total_lanes)
                .map(|i| operand(1100 + i))
                .collect(),
            post_helper: (0..geom.helper_total_lanes)
                .map(|i| operand(1200 + i))
                .collect(),
        };
        // The exact reviewer counterexample: enable already occupies the
        // first parent temporary under scope "s".
        let colliding = inputs(Operand::Variable(Identifier("%s.local.temp.0".to_string())));
        assert!(
            matches!(
                C6ScalarFailureStepEmitter::new("s")
                    .unwrap()
                    .emit(&colliding),
                Err(C6ScalarFailureStepError::ReservedOperand { .. })
            ),
            "reserved enable operand"
        );
        // Cross-composed regression: a non-DATA (pre-image) operand named
        // with the DATA emitter prefix of this same scope is rejected
        // before the DATA emitter can mint the identical identifier.
        let mut cross = inputs(operand(42));
        cross.pre_image[0] = Operand::Variable(Identifier("%s.c6data.temp.0".to_string()));
        assert!(
            matches!(C6ScalarFailureStepEmitter::new("s").unwrap().emit(&cross),
            Err(C6ScalarFailureStepError::ReservedOperand { identifier })
                if identifier == "%s.c6data.temp.0"),
            "cross-composed pre-image operand"
        );
        // Every composed prefix against every operand group.
        for seg in [
            ".local.temp.",
            ".c6data.temp.",
            ".c6helper.temp.",
            ".c6meta.temp.",
            ".c6progctx.temp.",
        ] {
            for group in 0..16usize {
                let mut inp = inputs(operand(42));
                let bad = Operand::Variable(Identifier(format!("%s{seg}0")));
                match group {
                    0 => inp.enable = bad,
                    1 => inp.disposition_is_refused_helper_work = bad,
                    2 => inp.host_schema = bad,
                    3 => inp.carrier_version = bad,
                    4 => inp.pre_meta[0] = bad,
                    5 => inp.pre_prog[0] = bad,
                    6 => inp.pre_ctx[0] = bad,
                    7 => inp.pre_image[0] = bad,
                    8 => inp.pre_data[0] = bad,
                    9 => inp.pre_helper[0] = bad,
                    10 => inp.post_meta[0] = bad,
                    11 => inp.post_prog[0] = bad,
                    12 => inp.post_ctx[0] = bad,
                    13 => inp.post_image[0] = bad,
                    14 => inp.post_data[0] = bad,
                    _ => inp.post_helper[0] = bad,
                }
                assert!(
                    matches!(
                        C6ScalarFailureStepEmitter::new("s").unwrap().emit(&inp),
                        Err(C6ScalarFailureStepError::ReservedOperand { .. })
                    ),
                    "reserved {seg} group {group}"
                );
            }
        }
        // Scope injection: '%' and every composed reserved segment refuse.
        for scope in [
            "bad%scope",
            "bad.local.temp.1",
            "x.c6data.temp.2",
            "x.c6helper.temp.3",
            "x.c6meta.temp.4",
            "x.c6progctx.temp.5",
        ] {
            assert!(
                matches!(
                    C6ScalarFailureStepEmitter::new(scope),
                    Err(C6ScalarFailureStepError::InvalidScope)
                ),
                "scope {scope}"
            );
        }
        assert!(
            matches!(
                C6ScalarFailureStepEmitter::new("  "),
                Err(C6ScalarFailureStepError::EmptyScope)
            ),
            "empty scope"
        );
        // Repeated emission through one retained emitter never rebinds a
        // parent temporary; the standalone function under a fresh scope is
        // the single-shot form.
        fn temps(
            block: &C6ScalarFailureStepBlock,
            scope: &str,
        ) -> std::collections::BTreeSet<String> {
            let debug = format!("{:?}", block.instructions);
            let prefix = format!("%{}.local.temp.", scope);
            let mut out = std::collections::BTreeSet::new();
            let mut rest = debug.as_str();
            while let Some(pos) = rest.find(&prefix) {
                let tail = &rest[pos..];
                let end = tail
                    .find(|ch: char| {
                        !(ch.is_ascii_alphanumeric() || ch == '%' || ch == '.' || ch == '_')
                    })
                    .unwrap_or(tail.len());
                out.insert(tail[..end].to_string());
                rest = &tail[end..];
            }
            out
        }
        let good = inputs(operand(42));
        let mut retained = C6ScalarFailureStepEmitter::new("compose_sf").unwrap();
        let first = retained.emit(&good).unwrap();
        let second = retained.emit(&good).unwrap();
        assert!(
            temps(&first, "compose_sf").is_disjoint(&temps(&second, "compose_sf")),
            "retained serial composition must not alias parent temps"
        );
        assert!(
            temps(&second, "compose_sf")
                .iter()
                .any(|t| t != "%compose_sf.local.temp.0"),
            "second block continues the serial"
        );
        let single = emit_c6_scalar_failure_step_block(&good, "single_sf").unwrap();
        assert!(
            temps(&single, "single_sf").contains("%single_sf.local.temp.0"),
            "single-shot serial zero"
        );
    }

    #[test]
    fn meta_bounds_frames_offset_shift_rejects() {
        let (w, c) = base_witness();
        assert!(
            check_c6_scalar_failure_step_circuit(&c, &w).is_ok(),
            "offset positive control"
        );
        // A circuit built at a shifted offset ties the frames lane of a
        // different META lane. The two refusal stages are distinct and
        // are asserted separately below; neither is relabelled as the
        // other, and neither is a catch-all Err.
        let shifted_offset = w.host_prefix.meta_bounds_frames_offset + 1;
        let shifted = build_c6_scalar_failure_step_circuit(
            w.source_params().clone(),
            w.host_prefix.meta.len(),
            w.host_prefix.prog.len(),
            shifted_offset,
            C6PushStatementBindingMode::LocalPublic,
            "c6_sf_test",
        )
        .unwrap();
        // Stage 1, host preflight: the unchanged witness carries the
        // unshifted offset, so the checker refuses before constructing
        // the preimage or calling the SDK. The exact host message is
        // asserted; this is not an SDK failure.
        match check_c6_scalar_failure_step_circuit(&shifted, &w) {
            Err(C6ScalarFailureStepError::Backend(m)) => assert_eq!(
                m, "meta bounds frames offset mismatch",
                "host offset stage: {m}"
            ),
            Err(e) => panic!("host offset stage: unexpected refusal category {e:?}"),
            Ok(_) => {
                panic!("host offset stage: shifted circuit must not check the unchanged witness")
            }
        }
        // Stage 2, SDK preprocessing: a witness tied to the same shifted
        // offset passes the host offset equality, so checking reaches
        // ProofPreimage::check. The shifted circuit's META binding ties
        // the frames bound to a different META lane than the witness's
        // frames lane, so preprocessing rejects with the SDK prefix.
        // The offset mutation itself is asserted non-no-op.
        let mut tied = w.clone();
        tied.host_prefix.meta_bounds_frames_offset = shifted_offset;
        assert_ne!(
            tied.host_prefix.meta_bounds_frames_offset, w.host_prefix.meta_bounds_frames_offset,
            "offset tie mutation no-op"
        );
        match check_c6_scalar_failure_step_circuit(&shifted, &tied) {
            Err(C6ScalarFailureStepError::Backend(m)) => assert!(
                m.starts_with("SDK ProofPreimage::check failed:"),
                "shifted offset SDK stage: {m}"
            ),
            Err(e) => panic!("shifted offset SDK stage: unexpected refusal category {e:?}"),
            Ok(_) => panic!("shifted offset SDK stage: tied witness must not check"),
        }
        // META and CTX lane mutations reach this composition's checking.
        let mut bad = w.clone();
        bad.host_prefix.meta[3] += 1;
        assert_pre_rejects(&bad, "META lane through ScalarFailure composition");
        let mut bad = w.clone();
        bad.host_prefix.ctx[21] += 1;
        assert_pre_rejects(&bad, "CTX budget lane through ScalarFailure composition");
    }

    #[test]
    fn fabricated_owner_record_event_validation() {
        // Host record boundary only: a fabricated public record is
        // provided data, not authenticated owner provenance and not a
        // circuit proof. validate_owner_record binds the event to the
        // provided snapshots; it does not authenticate them.
        let cfg = config();
        let pre = sample(100, 100);
        let post = p3_post(&pre);
        let mk_record = |before: &HostSnapshot, after: &HostSnapshot| -> CompleteStepRecord {
            CompleteStepRecord {
                before: before.clone(),
                after: after.clone(),
                event: mori_core::sherlock::helper_machine::shared::StepEventC {
                    step_id: mori_core::sherlock::helper_machine::shared::ExpandedStepIdC {
                        run_id: std::sync::Arc::new(before.image.run_id.clone()),
                        ordinal: before.image.consumed_work,
                    },
                    rule_id: RuleIdC::BHelperScalarFailure,
                    event_kind: EventKind::HelperFail,
                    site: Some(mori_core::sherlock::helper_machine::shared::SiteTag::RetRight),
                    helper_kind: Some(HelperKind::Scalar),
                    disposition: StepDisposition::RefusedHelperWorkUncharged,
                    work_delta: 0,
                    helper_work_delta: 0,
                    helper_visits_delta: 0,
                    reference_cursor_delta: 0,
                    alloc_deltas: mori_core::sherlock::helper_machine::shared::AllocDeltasC {
                        data_nodes: 0,
                        data_fields: 0,
                        environment_cells: 0,
                    },
                    value_slot_writes: 0,
                    logical_depth_before: 1,
                    logical_depth_after: 1,
                    task_cursor_before: 0,
                    task_cursor_after: 0,
                    helper_phase_before: Some(
                        mori_core::sherlock::helper_machine::shared::HelperPhaseSummaryC::Scalar(
                            ScalarPhase::Done,
                        ),
                    ),
                    helper_phase_after: Some(
                        mori_core::sherlock::helper_machine::shared::HelperPhaseSummaryC::Scalar(
                            ScalarPhase::Done,
                        ),
                    ),
                    helper_cursor_before: Some(2),
                    helper_cursor_after: Some(2),
                    result_tag: None,
                    origin_reference_cursor: Some(7),
                    source_expr_id: 1,
                    source_occurrence:
                        mori_core::sherlock::helper_machine::shared::LocatorC::Occurrence {
                            owner_is_function: false,
                            function: 0,
                            ast_path: Vec::new(),
                        },
                    exhausted_kind: Some(
                        mori_core::sherlock::helper_machine::shared::ExhaustReason::HelperWork,
                    ),
                    outcome_tag:
                        mori_core::sherlock::helper_machine::shared::OutcomeTagC::Exhausted,
                    outcome_payload: OutcomeC::Exhausted {
                        reason:
                            mori_core::sherlock::helper_machine::shared::ExhaustReason::HelperWork,
                    },
                    boundary: mori_core::sherlock::helper_machine::shared::BoundaryKind::Terminal,
                },
            }
        };
        let good = mk_record(&pre, &post);
        assert!(
            witness_from_owner_record(&good, &cfg, &limits()).is_ok(),
            "fabricated-consistent P3 control"
        );
        let mut bad = good.clone();
        bad.event.result_tag = Some(HelperResultTagC::ScalarError);
        assert!(
            matches!(
                witness_from_owner_record(&bad, &cfg, &limits()),
                Err(C6ScalarFailureStepError::EventMismatch {
                    field: "p3 result_tag"
                })
            ),
            "P3 wrong result_tag"
        );
        let mut bad = good.clone();
        bad.event.exhausted_kind =
            Some(mori_core::sherlock::helper_machine::shared::ExhaustReason::Fuel);
        assert!(
            matches!(
                witness_from_owner_record(&bad, &cfg, &limits()),
                Err(C6ScalarFailureStepError::EventMismatch {
                    field: "p3 exhausted_kind"
                })
            ),
            "P3 wrong exhausted kind"
        );
        let mut bad = good.clone();
        bad.event.outcome_payload = OutcomeC::Running;
        assert!(
            matches!(
                witness_from_owner_record(&bad, &cfg, &limits()),
                Err(C6ScalarFailureStepError::EventMismatch {
                    field: "p3 outcome"
                })
            ),
            "P3 wrong outcome payload"
        );
        let mut bad = good.clone();
        bad.event.work_delta = 1;
        assert!(
            matches!(
                witness_from_owner_record(&bad, &cfg, &limits()),
                Err(C6ScalarFailureStepError::EventMismatch { field: "p3 deltas" })
            ),
            "P3 charged event"
        );
        // Genuine below-max Applied control: sample(2, 100) + applied_post.
        let pre_a = sample(2, 100);
        let post_a = applied_post(&pre_a);
        let mut applied = mk_record(&pre_a, &post_a);
        applied.event.disposition = StepDisposition::Applied;
        applied.event.work_delta = 1;
        applied.event.helper_work_delta = 1;
        applied.event.result_tag = Some(HelperResultTagC::ScalarError);
        applied.event.exhausted_kind = None;
        applied.event.outcome_tag =
            mori_core::sherlock::helper_machine::shared::OutcomeTagC::Running;
        applied.event.outcome_payload = OutcomeC::Running;
        applied.event.boundary = mori_core::sherlock::helper_machine::shared::BoundaryKind::None;
        applied.event.helper_phase_after = None;
        applied.event.helper_cursor_after = None;
        assert!(
            witness_from_owner_record(&applied, &cfg, &limits()).is_ok(),
            "Applied event control"
        );
        let mut bad = applied.clone();
        bad.event.result_tag = None;
        assert!(
            matches!(
                witness_from_owner_record(&bad, &cfg, &limits()),
                Err(C6ScalarFailureStepError::EventMismatch {
                    field: "applied deltas/result"
                })
            ),
            "Applied missing result"
        );
        // Crossed records: Applied event on the P3 pair, P3 event on the
        // Applied pair. Both fail the snapshot disposition binding.
        let mut crossed = good.clone();
        crossed.event.disposition = StepDisposition::Applied;
        crossed.event.work_delta = 1;
        crossed.event.helper_work_delta = 1;
        crossed.event.result_tag = Some(HelperResultTagC::ScalarError);
        crossed.event.exhausted_kind = None;
        crossed.event.outcome_tag =
            mori_core::sherlock::helper_machine::shared::OutcomeTagC::Running;
        crossed.event.outcome_payload = OutcomeC::Running;
        crossed.event.boundary = mori_core::sherlock::helper_machine::shared::BoundaryKind::None;
        crossed.event.helper_phase_after = None;
        crossed.event.helper_cursor_after = None;
        assert!(
            matches!(
                witness_from_owner_record(&crossed, &cfg, &limits()),
                Err(C6ScalarFailureStepError::EventMismatch {
                    field: "snapshot disposition"
                })
            ),
            "Applied event on P3 snapshots"
        );
        let crossed_p3 = {
            let mut r = applied.clone();
            r.event.disposition = StepDisposition::RefusedHelperWorkUncharged;
            r.event.work_delta = 0;
            r.event.helper_work_delta = 0;
            r.event.result_tag = None;
            r.event.exhausted_kind =
                Some(mori_core::sherlock::helper_machine::shared::ExhaustReason::HelperWork);
            r.event.outcome_tag =
                mori_core::sherlock::helper_machine::shared::OutcomeTagC::Exhausted;
            r.event.outcome_payload = OutcomeC::Exhausted {
                reason: mori_core::sherlock::helper_machine::shared::ExhaustReason::HelperWork,
            };
            r.event.boundary = mori_core::sherlock::helper_machine::shared::BoundaryKind::Terminal;
            r.event.helper_phase_after = Some(
                mori_core::sherlock::helper_machine::shared::HelperPhaseSummaryC::Scalar(
                    ScalarPhase::Done,
                ),
            );
            r.event.helper_cursor_after = Some(2);
            r
        };
        assert!(
            matches!(
                witness_from_owner_record(&crossed_p3, &cfg, &limits()),
                Err(C6ScalarFailureStepError::EventMismatch {
                    field: "snapshot disposition"
                })
            ),
            "P3 event on Applied snapshots"
        );
        // Snapshot-field tampers on the genuine Applied pair.
        let mut bad = applied.clone();
        bad.after.image.consumed_work = bad.before.image.consumed_work + 2;
        assert!(
            matches!(
                witness_from_owner_record(&bad, &cfg, &limits()),
                Err(C6ScalarFailureStepError::EventMismatch {
                    field: "snapshot consumed_work"
                })
            ),
            "Applied counter"
        );
        let mut bad = applied.clone();
        bad.after.image.helper = bad.before.image.helper;
        assert!(
            matches!(
                witness_from_owner_record(&bad, &cfg, &limits()),
                Err(C6ScalarFailureStepError::EventMismatch {
                    field: "snapshot helper"
                })
            ),
            "Applied helper retained"
        );
        let mut bad = applied.clone();
        bad.after.image.result = Some(HelperResultC::ScalarError { code: 11 });
        assert!(
            matches!(
                witness_from_owner_record(&bad, &cfg, &limits()),
                Err(C6ScalarFailureStepError::EventMismatch {
                    field: "snapshot result"
                })
            ),
            "Applied wrong code"
        );
        let mut bad = applied.clone();
        bad.after.image.outcome = OutcomeC::Exhausted {
            reason: mori_core::sherlock::helper_machine::shared::ExhaustReason::HelperWork,
        };
        assert!(
            matches!(
                witness_from_owner_record(&bad, &cfg, &limits()),
                Err(C6ScalarFailureStepError::EventMismatch {
                    field: "snapshot outcome"
                })
            ),
            "Applied wrong outcome"
        );
        // Checked helper-work overflow is a typed mismatch, never a wrap.
        let mut bad = applied.clone();
        bad.before.image.helper_work = u128::MAX;
        bad.after.image.helper_work = 0;
        assert!(
            matches!(
                witness_from_owner_record(&bad, &cfg, &limits()),
                Err(C6ScalarFailureStepError::EventMismatch {
                    field: "snapshot helper_work"
                })
            ),
            "Applied helper_work overflow"
        );
    }
}
