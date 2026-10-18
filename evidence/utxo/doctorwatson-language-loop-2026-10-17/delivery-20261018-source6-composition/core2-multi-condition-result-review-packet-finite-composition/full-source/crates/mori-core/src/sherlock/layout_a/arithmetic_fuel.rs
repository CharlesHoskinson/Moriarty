//! Private decoded A program/state consumer for aux/2 numeric host columns.
//! No transition acceptance, serializer, circuit or change to the aux/1 prefix.
use crate::sherlock::arithmetic_witness::{
    materialize_arithmetic, materialize_fuel, ArithmeticOp, ArithmeticWitness, FuelWitness,
};
use super::rows::{ConstantNode, PreparedA, ValueRef};
use super::state::{AImage, DecodeMode, StateCapacities, StateFailure, StateResult};
use super::state_decode::decode_observed_a;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct ArithHeaderA {
    pub op_l: ValueRef,
    pub op_r: ValueRef,
    pub n_l: ConstantNode,
    pub n_r: ConstantNode,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct NumericAuxA {
    pub header: ArithHeaderA,
    pub arithmetic: ArithmeticWitness,
    pub fuel: FuelWitness,
}

fn fetch(image: &AImage<'_>, value: ValueRef) -> StateResult<ConstantNode> {
    let (rows, active, table) = if value.space {
        (&image.state.data, image.state.root.active_data, "Data")
    } else {
        (&image.program.constants.rows, image.program.constants.active, "Constants")
    };
    rows.get(value.index).filter(|_| value.index < active).cloned()
        .ok_or(StateFailure::WrongRepresentation {
            block: "AuxBlock2", table, row: value.index, column: "valueRef",
            definition: "ARH-2: operand index must be below its selected arena active length",
        })
}

/// Use the existing Step decoder to check the complete supplied program and
/// state domain, then compute honest aux/2 numeric columns from their real rows.
/// Decoding is not reachability, transition validation, or a proof relation.
/// No supplied guard, operation, operand bytes or charge can override these rows.
/// Header VR/NODE lanes follow ARH-1/2; this does not bind a native statement.
pub(super) fn materialize_numeric_aux_a(
    prepared: &PreparedA,
    capacities: &StateCapacities,
    image: &AImage<'_>,
) -> StateResult<NumericAuxA> {
    drop(decode_observed_a(prepared, capacities, image, DecodeMode::Step)?);
    let pre = image.state;
    let root = &pre.root;
    let en_evt = root.outcome_tag == 0 && root.control_length != 0;
    let control = pre.control_rows.first().filter(|_| root.control_length != 0);
    let is_return = control.is_some_and(|row| row.tag == 1);
    let term = (is_return && root.depth == 0) || control.is_some_and(|row| row.tag == 9);
    let fuel = materialize_fuel(en_evt, term, &root.remaining_fuel);
    let frame = pre.frame_slots.first().filter(|_| root.depth != 0);
    let en_bin = fuel.charged && is_return && frame.is_some_and(|row| row.tag == 2);
    let op = frame.filter(|_| en_bin).and_then(|row| match row.binary_op {
        0 => Some(ArithmeticOp::Add),
        1 => Some(ArithmeticOp::Sub),
        2 => Some(ArithmeticOp::Mul),
        4 => Some(ArithmeticOp::Lt),
        _ => None,
    });
    let mut header = ArithHeaderA {
        op_l: ValueRef::default(), op_r: ValueRef::default(),
        n_l: ConstantNode::default(), n_r: ConstantNode::default(),
    };
    if op.is_some() {
        // en_bin implies both actual active head rows are present.
        if let (Some(frame), Some(control)) = (frame, control) {
            header.op_l = frame.value;
            header.op_r = control.value;
            header.n_l = fetch(image, header.op_l)?;
            header.n_r = fetch(image, header.op_r)?;
        }
    }
    let left = (op.is_some() && header.n_l.type_id == 0).then_some(&header.n_l.uint_bytes);
    let right = (op.is_some() && header.n_r.type_id == 0).then_some(&header.n_r.uint_bytes);
    let arithmetic = materialize_arithmetic(op, left, right);
    Ok(NumericAuxA { header, arithmetic, fuel })
}
