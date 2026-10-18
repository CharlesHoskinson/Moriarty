//! Exact tagged scalar event bytes are compressed inside the generated circuit.
use super::{
    arithmetic::{Builder, field, imm},
    refusal,
};
use crate::{Result, ops};
use midnight_base_crypto::fab::AlignedValue;
use midnight_onchain_state::state::StateValue;
use midnight_onchain_vm::ops::{LogEventType, Op};
use midnight_transient_crypto::{curve::Fr, repr::FieldRepr};
use midnight_zkir_v3::{Instruction, ir::Operand};
use mori_core::{BinaryOp, Type, UInt, Value};

fn native_event_admitted(payload: &[u8]) -> bool {
    midnight_storage::arena::Sp::new(ops::cell(AlignedValue::from(payload.to_vec())))
        .serialize_to_node_list_bounded(midnight_onchain_vm::vm::MAX_LOG_EMITTED)
        .is_some()
}

fn shift(bits: usize) -> Operand {
    // Packing factors are powers at most2^240; each packed chunk is <2^248.
    let mut value = Fr::from(1);
    for _ in 0..bits {
        value = value * Fr::from(2);
    }
    field(value)
}

fn branch(
    b: &mut Builder,
    name: &str,
    action: &str,
    val: Operand,
    ty: Type,
    guard: Operand,
    length: Option<usize>,
) -> Result<()> {
    let selected = b.select(guard.clone(), val, imm(0));
    // These constraints execute even for inactive branches: select-zero FIRST.
    b.bits(
        selected.clone(),
        length.map(|n| (n * 8) as u32).unwrap_or(5),
    );
    let (sample, dynamic) = if let Some(n) = length {
        (
            Value::UInt(UInt(if n == 1 { 32 } else { 1u128 << (8 * (n - 1)) })),
            selected,
        )
    } else {
        let zero = b.binary(selected.clone(), imm(0), BinaryOp::Eq);
        let atom = b.select(zero, imm(0x40), selected);
        b.bits(atom.clone(), 8);
        (
            match ty {
                Type::UInt => Value::UInt(UInt(0)),
                Type::Bool => Value::Bool(false),
            },
            atom,
        )
    };
    let payload = ops::event_payload(name, action, &sample)?;
    // Payload width is source-static per FAB normalization branch. Preserve
    // shorter valid instances, but forbid activating a native-dropped event.
    if !native_event_admitted(&payload) {
        b.equal(guard, imm(0));
        return Ok(());
    }
    let dynamic_len = length.unwrap_or(1);
    // The last byte is the fixed UInt Bytes16 or Bool Bytes1 alignment tail.
    let dynamic_start = payload
        .len()
        .checked_sub(dynamic_len + 1)
        .ok_or(refusal("event-template"))?;
    let mut constants = payload.clone();
    constants[dynamic_start..dynamic_start + dynamic_len].fill(0);
    let mut chunks = vec![];
    for start in (0..constants.len()).step_by(31) {
        let end = (start + 31).min(constants.len());
        let mut packed =
            field(Fr::from_le_bytes(&constants[start..end]).ok_or(refusal("event-chunk"))?);
        let overlap_start = start.max(dynamic_start);
        let overlap_end = end.min(dynamic_start + dynamic_len);
        if overlap_start < overlap_end {
            let skip = overlap_start - dynamic_start;
            let width = overlap_end - overlap_start;
            let remainder = if skip == 0 {
                dynamic.clone()
            } else {
                b.split(dynamic.clone(), (skip * 8) as u32).0
            };
            let piece = b.split(remainder, (width * 8) as u32).1;
            b.bits(piece.clone(), (width * 8) as u32);
            let shifted = b.binary(piece, shift((overlap_start - start) * 8), BinaryOp::Mul);
            b.bits(shifted.clone(), ((end - start) * 8) as u32);
            packed = b.binary(packed, shifted, BinaryOp::Add);
        }
        b.bits(packed.clone(), ((end - start) * 8) as u32);
        chunks.push(packed);
    }
    chunks.reverse();
    let mut hash_inputs = vec![imm(payload.len() as u128)];
    hash_inputs.extend(chunks);
    let output = b.fresh();
    b.instructions.push(Instruction::TransientHash {
        inputs: hash_inputs,
        output: output.clone(),
    });
    // Hash output is native Fr, NOT UInt128. Vec<u8> uses Compress alignment.
    let hash = Operand::Variable(output);
    let wrapper = StateValue::Array(
        vec![
            ops::cell(1u32.into()),
            ops::cell((LogEventType::Misc as u8).into()),
            ops::cell(AlignedValue::from(payload)),
        ]
        .into(),
    );
    let push = Op::Push {
        storage: false,
        value: wrapper,
    };
    if push.field_size() != 14 {
        return Err(refusal("event-field-width"));
    }
    b.op(push, guard.clone(), &[(13, hash)], "source-event-push")?;
    b.op(Op::Log, guard, &[], "source-event-log")?;
    Ok(())
}

pub fn emit(b: &mut Builder, name: &str, action: &str, val: Operand, ty: Type) -> Result<()> {
    let minimum = match ty {
        Type::UInt => Value::UInt(UInt(0)),
        Type::Bool => Value::Bool(false),
    };
    if !native_event_admitted(&ops::event_payload(name, action, &minimum)?) {
        return Err(refusal("event-size-domain"));
    }
    if ty == Type::Bool {
        return branch(b, name, action, val, ty, imm(1), None);
    }
    let small = b.binary(val.clone(), imm(32), BinaryOp::Lt);
    b.range(small.clone(), Type::Bool);
    let mut guards = vec![small];
    for n in 1..=16 {
        let lower = if n == 1 { 32 } else { 1u128 << (8 * (n - 1)) };
        let below = b.binary(val.clone(), imm(lower), BinaryOp::Lt);
        let at_least = b.unary(below, true);
        let upper = if n == 16 {
            imm(1)
        } else {
            b.binary(val.clone(), imm(1u128 << (8 * n)), BinaryOp::Lt)
        };
        let guard = b.binary(at_least, upper, BinaryOp::Mul);
        b.range(guard.clone(), Type::Bool);
        guards.push(guard);
    }
    let mut sum = imm(0);
    for guard in &guards {
        sum = b.binary(sum, guard.clone(), BinaryOp::Add);
        b.bits(sum.clone(), 5);
    }
    b.equal(sum, imm(1));
    for (i, guard) in guards.into_iter().enumerate() {
        branch(
            b,
            name,
            action,
            val.clone(),
            ty,
            guard,
            if i == 0 { None } else { Some(i) },
        )?;
    }
    Ok(())
}
