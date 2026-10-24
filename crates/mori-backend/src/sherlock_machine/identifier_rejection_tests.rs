//! UNRUN construction tests implementing Opus's G-D1(b,c) prescriptions from
//! machine-layout-gadget-security-opus-r1. Test requirements/security design
//! belong to that review's author; this module adds no new rejection policy.
//! No SDK load, check, evaluation, compilation or proving is called.

use super::*;

fn fixture() -> (TableShape, FetchInputs) {
    let shape = TableShape::new(
        1,
        vec![Column {
            name: "amount".into(),
            kind: Type::UInt,
        }],
    )
    .unwrap();
    let variable = |name: &str| Operand::Variable(Identifier(name.into()));
    let inputs = FetchInputs {
        active_rows: variable("%caller.count"),
        index: variable("%caller.index"),
        rows: vec![vec![variable("%caller.amount")]],
    };
    (shape, inputs)
}

#[test]
fn opus_g_d1_rejects_unprefixed_caller_variables() {
    let (shape, supplied) = fixture();
    for position in 0..3 {
        let mut inputs = supplied.clone();
        let operand = match position {
            0 => &mut inputs.active_rows,
            1 => &mut inputs.index,
            _ => &mut inputs.rows[0][0],
        };
        *operand = Operand::Variable(Identifier("caller.unprefixed".into()));
        assert!(matches!(
            emit_fetch_block(&shape, &inputs, "fetch"),
            Err(ConstructionError::UnprefixedVariable)
        ));
    }
}

#[test]
fn opus_g_d1_rejects_inputs_in_the_final_temporary_namespace() {
    let (shape, supplied) = fixture();
    for position in 0..3 {
        let mut inputs = supplied.clone();
        let operand = match position {
            0 => &mut inputs.active_rows,
            1 => &mut inputs.index,
            _ => &mut inputs.rows[0][0],
        };
        *operand = Operand::Variable(Identifier("%fetch.temp.0".into()));
        assert!(matches!(
            emit_fetch_block(&shape, &inputs, "fetch"),
            Err(ConstructionError::TemporaryNameCollision)
        ));
    }
}
