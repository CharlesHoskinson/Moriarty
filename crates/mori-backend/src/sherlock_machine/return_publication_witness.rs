//! Proposed SDK-free honest witness producer for literal-entry A102 -> A80.
//!
//! Source only: no statement layout, native request, context, exposure policy,
//! SDK instruction construction or execution. `LiteralValues` is reused solely
//! as the existing complete logical program/pre/post/envelope data carrier.
//! Its A80 values must not be passed to the A102-only `LiteralLayout::statement`.
//!
//! Implements the designated return-publication Step-only declaration:
//! incoming Constant-space A102 Step post is the exact physical A80 pre;
//! A80 aliases its Return ValueRef into Done, preserves all five arenas and
//! fuel bytes, and clears every ControlRows cell. No canonical re-encoding or
//! `decode_observed_a` is applied to either constructed Step-mode state.
//!
//! H1 trusted-host diagnostics only, with zero native authority: inherited
//! prepare/Machine/A102 producer operations include unbounded allocation and
//! recursive clone/equality/drop; deep inputs can abort rather than return Err.
//! Full StepInv, F4, G4, projectA, dispatch, independent expectations, circuit
//! acceptance, statement binding and all native/target evidence remain open.

use super::literal_host::{LiteralCapacities, LiteralSourceIdentity, LiteralValues};
use super::literal_witness::{
    core_program_capacities, produce_a102_literal_transition_from_step,
    state_block_to_image_values, validate_literal_schema, ProducedLiteralTransition,
};
use mori_core::sherlock::layout_a::{
    prepare_a, ControlRow, Locator, StateBlock, StateFailure, StateResult,
};
use mori_core::sherlock::{Control, Expr, Machine, Outcome, Program, StepRecord};
use std::sync::Arc;

/// Complete source-derived Step data and both real reference observations.
/// `literal_transition.prepared.block` is the complete shared ProgramBlock;
/// `pre_state()` borrows its unchanged A102 Step post, including Constant alias.
/// `values` contains all program/pre/post lanes and row80's tag0/charge0 envelope.
/// There is no native Return request or acceptance result in this structure.
#[derive(Clone, Debug)]
pub struct ProducedLiteralReturnPublication {
    pub literal_transition: ProducedLiteralTransition,
    pub step_record: StepRecord,
    pub post_state: StateBlock,
    pub values: LiteralValues,
}

impl ProducedLiteralReturnPublication {
    pub fn pre_state(&self) -> &StateBlock {
        &self.literal_transition.post_state
    }
}

fn outside(condition: &'static str, field: &'static str) -> StateFailure {
    StateFailure::OutsideObservationDomain {
        condition,
        locator: Locator::field(field),
    }
}

/// Prepare a real complete Program and observe two steps of one Machine:
/// A102 Eval(Literal) -> Return, then A80 Return -> None/Done.
///
/// Entry/stack/fuel checks retain A102's existing cheap-admission order and
/// error classes before preparation; this API admits only literal entries.
/// `initial_fuel == 1` is valid: A80 publication occurs at remaining fuel zero.
/// `identity` belongs only to the retained A102 request; it selects no A80 native
/// identity. Caller-selected capacities are neither defaults nor resource grants.
///
/// Tests and implementation are source proposals, uncompiled and unexecuted.
pub fn produce_a102_literal_return_publication(
    program: Arc<Program>,
    capacities: &LiteralCapacities,
    initial_fuel: u128,
    stack_limit: usize,
    identity: &LiteralSourceIdentity,
) -> StateResult<ProducedLiteralReturnPublication> {
    validate_literal_schema(capacities)?;
    let program_capacities = core_program_capacities(capacities)?;
    if !matches!(program.entry.as_ref(), Expr::Literal(_)) {
        return Err(StateFailure::OutsideObservationDomain {
            condition: "A102 entry transition requires Program.entry to be Expr::Literal",
            locator: Locator {
                expression: Some(0), function: None, field: "Program.entry", item: 0,
            },
        });
    }
    let stack_limit_u128 = u128::try_from(stack_limit).map_err(|_| {
        StateFailure::HostUnrepresentable {
            field: "Snapshot.stack_limit",
            locator: Locator {
                expression: None, function: None, field: "Snapshot.stack_limit", item: 0,
            },
        }
    })?;
    if stack_limit_u128 > capacities.stack_limit {
        return Err(StateFailure::Capacity {
            table: "Root.stackLimit", needed: stack_limit_u128,
            limit: capacities.stack_limit, locator: Locator::field("Snapshot.stack_limit"),
        });
    }
    if initial_fuel == 0 {
        return Err(outside(
            "A102 requires nonzero initial_fuel for charged rule 102",
            "Snapshot.remaining_fuel",
        ));
    }
    let prepared = prepare_a(program.clone(), program_capacities)?;
    let mut machine = Machine::new(program, initial_fuel, stack_limit);
    let literal_record = machine.step().ok_or_else(|| outside(
        "Machine::step returned None on initial state", "Machine.step",
    ))?;
    let (pre_state, post_state, selected_expr_id, selected_constant_ref, request) =
        produce_a102_literal_transition_from_step(
            &prepared, capacities, &literal_record, identity,
        )?;
    let literal_transition = ProducedLiteralTransition {
        prepared, step_record: literal_record, pre_state, post_state,
        selected_expr_id, selected_constant_ref, request,
    };
    let step_record = machine.step().ok_or_else(|| outside(
        "Machine::step returned None before Return publication", "Machine.step",
    ))?;
    // Observe the real semantic transition before constructing its physical Step
    // post under the owner's alias/preservation contract. These are consistency
    // checks on this producer's own observations, not a transition validator.
    let returned_value = match &step_record.before.control {
        Some(Control::Return(value)) => value,
        _ => return Err(outside("A80 requires Return before publication", "StepRecord.before.control")),
    };
    if step_record.rule_start_line != 80 || step_record.charge != 0
        || step_record.before.outcome != Outcome::Pending
        || !step_record.before.frames.is_empty()
        || step_record.after.control.is_some()
        || step_record.after.outcome != Outcome::Done(returned_value.clone())
        || step_record.before.remaining_fuel != step_record.after.remaining_fuel
        || step_record.before.initial_fuel != step_record.after.initial_fuel
    {
        return Err(outside(
            "Machine observation must be the uncharged no-frame A80 Done publication",
            "StepRecord.after",
        ));
    }
    let pre = &literal_transition.post_state;
    let mut post_state = pre.clone();
    post_state.root.control_length = 0;
    post_state.root.outcome_tag = 1;
    post_state.root.outcome_value = literal_transition.selected_constant_ref;
    post_state.root.outcome_error = 0;
    post_state.control_rows.fill(ControlRow::default());
    let values = LiteralValues {
        program: literal_transition.request.values.program.clone(),
        pre: literal_transition.request.values.post.clone(),
        post: state_block_to_image_values(&post_state, capacities)?,
        rule_tag: 0,
        charge: 0,
    };
    Ok(ProducedLiteralReturnPublication {
        literal_transition, step_record, post_state, values,
    })
}
