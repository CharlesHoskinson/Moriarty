//! C6 Scalar construct step: the `b.helper.scalar.construct` row
//! factorial/tree-fold-style programs fire while building `leaf(..)`,
//! `cons(..)` and `branch(..)` values, as one per-rule relation over the
//! full C6 carriers.
//!
//! The circuit fixes its rule at build time, selects it with the composed
//! dispatch owner and writes the post state exactly as
//! `fragments/scalar.proposed.rs` (row `b.helper.Scalar.construct`) plans
//! it. The Scalar helper (kind 1) carries mode (lane 5), operand_a
//! (6-7), operand_b (8-10), phase (13) and staged (15-16):
//! - Leaf at phase First: `kind_of(operand_a) == UInt` stages
//!   `PendingNode { Leaf }`, anything else stages `PrimError { 11 }`;
//! - Cons at phase Second: `kind_of(operand_b)` in {Nil, Cons} stages
//!   `PendingNode { Cons }`, anything else `PrimError { 11 }`;
//! - Branch at phase Second: `kind_of(operand_b)` in {Leaf, Branch}
//!   stages `PendingNode { Branch }`, anything else `PrimError { 11 }`.
//! Every construct step ends at phase Done. Kinds come from the checked
//! Const/DATA value read of the operand itself, never from a host lane.
//!
//! Charge is the applied helper charge (visits 1). Not constrained here,
//! so rejected: the P3 helper-work refusal. Every unnamed lane is
//! preserved; canonicality is composed; enable = 0 is identity.
use midnight_zkir_v3::Instruction;
use mori_core::sherlock::helper_machine::c6_phys_helper::{HELPER_LANES, L_HELPER_RECORD};
use mori_core::sherlock::helper_machine::shared::RuleIdC;
use thiserror::Error;

use super::c6_checked_value_read::{C6CheckedValueReadEmitter, C6CheckedValueReadInputs};
use super::c6_step_kit::{
    C6StepInputs, Composed, Em, charge_helper, check_shapes, compose_canonical, disabled_identity,
    helper_charge_lanes, prefix, preserve_image, select, validate_pre_image,
};

#[derive(Debug, Error)]
pub enum C6ScalarConstructError {
    #[error("rule {0:?} is not a Scalar construct rule of this fragment")]
    Unsupported(RuleIdC),
    #[error("invalid scope")]
    InvalidScope,
    #[error("C6 Scalar construct emission: {0}")]
    Emit(String),
}

pub const SCALAR_CONSTRUCT_RULES: [RuleIdC; 1] = [RuleIdC::BHelperScalarConstruct];

pub fn scalar_construct_supported(rule: RuleIdC) -> bool {
    SCALAR_CONSTRUCT_RULES.contains(&rule)
}

const K_SCALAR: u128 = 1;
// Scalar modes (c6_phys_helper mode tags) and phases (sphase tags).
const MODE_CONS: u128 = 4;
const MODE_BRANCH: u128 = 5;
const MODE_LEAF: u128 = 6;
const PH_FIRST: u128 = 0;
const PH_SECOND: u128 = 1;
const PH_DONE: u128 = 2;
// ScalarStaged tags (Writer::staged) and PendingNode payloads (node tags).
const STAGED_PENDING_NODE: u128 = 3;
const STAGED_PRIM_ERROR: u128 = 4;
const NODE_CONS: u128 = 0;
const NODE_LEAF: u128 = 1;
const NODE_BRANCH: u128 = 2;
const PRIM_ERROR_KIND: u128 = 11;
// Scalar record offsets after the 5-lane head.
const S_MODE: usize = 5;
const S_A: usize = 6;
const S_B: usize = 8;
const S_PHASE: usize = 13;
const S_STAGED: usize = 15;

#[derive(Clone, Debug)]
pub struct C6ScalarConstructBlock {
    pub instructions: Vec<Instruction>,
}

pub struct C6ScalarConstructEmitter {
    scope: String,
    next_temp: usize,
    composed: Composed,
    reader: C6CheckedValueReadEmitter,
}

impl C6ScalarConstructEmitter {
    pub fn new(scope: &str) -> Result<Self, C6ScalarConstructError> {
        if scope.trim().is_empty() || scope.contains('%') || scope.contains(".c6kit.") {
            return Err(C6ScalarConstructError::InvalidScope);
        }
        Ok(Self {
            scope: scope.to_string(),
            next_temp: 0,
            composed: Composed::new(scope).map_err(C6ScalarConstructError::Emit)?,
            reader: C6CheckedValueReadEmitter::new(&format!("{scope}.constructread"))
                .map_err(|e| C6ScalarConstructError::Emit(e.to_string()))?,
        })
    }

    pub fn emit(
        &mut self,
        rule: RuleIdC,
        inputs: &C6StepInputs,
    ) -> Result<C6ScalarConstructBlock, C6ScalarConstructError> {
        if !scalar_construct_supported(rule) {
            return Err(C6ScalarConstructError::Unsupported(rule));
        }
        check_shapes(inputs).map_err(C6ScalarConstructError::Emit)?;
        let scope = self.scope.clone();
        let mut em = Em {
            scope: &scope,
            next: self.next_temp,
            ins: Vec::new(),
        };
        let r = self.body(&mut em, rule, inputs);
        self.next_temp = em.next;
        r.map_err(C6ScalarConstructError::Emit)?;
        Ok(C6ScalarConstructBlock {
            instructions: em.ins,
        })
    }

    fn body(&mut self, em: &mut Em, rule: RuleIdC, inputs: &C6StepInputs) -> Result<(), String> {
        let en = inputs.enable.clone();
        let caps = inputs.geometry.caps;
        prefix(em, inputs);
        let _top = validate_pre_image(em, inputs);
        select(&mut self.composed, em, rule, inputs)?;

        let pre_h = &inputs.pre_helper;
        let rec = |o: usize| pre_h[L_HELPER_RECORD + o].clone();
        em.geq(en.clone(), rec(0), Em::imm(K_SCALAR));
        let mode = rec(S_MODE);
        let is_leaf = em.teq(mode.clone(), Em::imm(MODE_LEAF));
        let is_cons = em.teq(mode.clone(), Em::imm(MODE_CONS));
        let is_branch = em.teq(mode.clone(), Em::imm(MODE_BRANCH));
        // Dispatch selects Leaf only at First, Cons/Branch only at Second.
        let at_first = em.teq(rec(S_PHASE), Em::imm(PH_FIRST));
        let at_second = em.teq(rec(S_PHASE), Em::imm(PH_SECOND));
        let leaf_first = em.mul(is_leaf.clone(), at_first);
        let cons_second = em.mul(is_cons.clone(), at_second.clone());
        let branch_second = em.mul(is_branch.clone(), at_second);
        let shape_a = em.add(leaf_first, cons_second);
        let shape = em.add(shape_a, branch_second);
        em.geq(en.clone(), shape, Em::imm(1));

        // Leaf: the checked kind of operand_a must be UInt.
        let read_a = self
            .reader
            .emit(&C6CheckedValueReadInputs {
                gate: en.clone(),
                ref_kind_is_data: rec(S_A),
                ref_index: rec(S_A + 1),
                pre_prog: inputs.pre_prog.clone(),
                pre_data: inputs.pre_data.clone(),
                caps,
            })
            .map_err(|e| e.to_string())?;
        em.ins.extend(read_a.instructions);
        em.gassert(en.clone(), read_a.selected_found.clone());
        // Cons/Branch: operand_b must be present and read.
        let binary = em.add(is_cons.clone(), is_branch.clone());
        em.geq(en.clone(), binary.clone(), rec(S_B));
        let read_b = self
            .reader
            .emit(&C6CheckedValueReadInputs {
                gate: en.clone(),
                ref_kind_is_data: rec(S_B + 1),
                ref_index: rec(S_B + 2),
                pre_prog: inputs.pre_prog.clone(),
                pre_data: inputs.pre_data.clone(),
                caps,
            })
            .map_err(|e| e.to_string())?;
        em.ins.extend(read_b.instructions);
        let b_gate = em.and(en.clone(), binary);
        em.gassert(b_gate, read_b.selected_found.clone());

        let cons_ok_inner = em.add(read_b.is_nil.clone(), read_b.is_cons.clone());
        let branch_ok_inner = em.add(read_b.is_leaf.clone(), read_b.is_branch.clone());
        let ok_leaf = em.mul(is_leaf.clone(), read_a.is_uint.clone());
        let ok_cons = em.mul(is_cons.clone(), cons_ok_inner);
        let ok_branch = em.mul(is_branch.clone(), branch_ok_inner);
        let ok_a = em.add(ok_leaf, ok_cons);
        let ok = em.add(ok_a, ok_branch);
        let node_cons = em.mul(is_cons.clone(), Em::imm(NODE_CONS));
        let node_leaf = em.mul(is_leaf.clone(), Em::imm(NODE_LEAF));
        let node_branch = em.mul(is_branch, Em::imm(NODE_BRANCH));
        let node_a = em.add(node_cons, node_leaf);
        let node = em.add(node_a, node_branch);
        let staged_tag = em.sel(
            ok.clone(),
            Em::imm(STAGED_PENDING_NODE),
            Em::imm(STAGED_PRIM_ERROR),
        );
        let staged_payload = em.sel(ok, node, Em::imm(PRIM_ERROR_KIND));

        let post_h = &inputs.post_helper;
        for (lane, (a, b)) in pre_h.iter().zip(post_h.iter()).enumerate() {
            let in_record = (L_HELPER_RECORD..L_HELPER_RECORD + HELPER_LANES).contains(&lane);
            let want = if in_record && lane - L_HELPER_RECORD == S_PHASE {
                Some(Em::imm(PH_DONE))
            } else if in_record && lane - L_HELPER_RECORD == S_STAGED {
                Some(staged_tag.clone())
            } else if in_record && lane - L_HELPER_RECORD == S_STAGED + 1 {
                Some(staged_payload.clone())
            } else {
                None
            };
            em.geq(en.clone(), b.clone(), want.unwrap_or_else(|| a.clone()));
        }
        preserve_image(em, inputs, &helper_charge_lanes(), false);
        charge_helper(em, inputs, 1);
        for (a, b) in inputs.pre_data.iter().zip(inputs.post_data.iter()) {
            em.geq(en.clone(), b.clone(), a.clone());
        }
        compose_canonical(&mut self.composed, em, inputs)?;
        disabled_identity(em, inputs);
        Ok(())
    }
}

/// One-shot emission of the Scalar construct relation for `rule`.
pub fn emit_c6_scalar_construct_block(
    rule: RuleIdC,
    inputs: &C6StepInputs,
    scope: &str,
) -> Result<C6ScalarConstructBlock, C6ScalarConstructError> {
    C6ScalarConstructEmitter::new(scope)?.emit(rule, inputs)
}
