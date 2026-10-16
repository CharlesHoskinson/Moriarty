//! Review finding F-12: a hand-built `Assignment` whose vectors do not have the
//! relation's lengths is refused by `CompiledRelation::circuit` before synthesis,
//! instead of panicking on an out-of-bounds index. Rows R12-* are [REPAIR-AUTHOR]
//! expectations frozen in REPAIR-EXPECTATIONS/F12-ASSIGNMENT-SHAPE.json before this
//! file was written: an extra derivation by the repair author, NOT an independent
//! oracle. Explicit imports only (review F-01).
mod common;

use ark_bls12_381::Fr;
use ark_relations::r1cs::ConstraintSystem;
use common::{SELF_MOVE, compile_ok, options, u};
use mori_core::UInt;
use mori_r1cs::{Assignment, BodyValues, CapBinding, CompileError, CompiledRelation, DutyValue, EffectValues, Tamper, TransitionValues};

/// S1 shape (duties_frame.rs `move_claim(5, 5)`): balance 5, self-transfer of 5.
fn honest() -> TransitionValues {
    TransitionValues {
        inputs: vec![u(5)],
        pre: BodyValues { states: vec![], cells: vec![UInt(5)], duties: vec![] },
        post: BodyValues { states: vec![], cells: vec![UInt(5)], duties: vec![] },
        effects: vec![EffectValues::Amount(UInt(5))],
        cap: None,
    }
}

fn relation() -> CompiledRelation {
    compile_ok(SELF_MOVE, options("move", CapBinding::NotBound))
}

#[test]
fn well_shaped_assignments_still_reach_the_constraint_system() {
    let rel = relation();
    // R12-0: the honest claim synthesizes and satisfies, as before the repair.
    let a = rel.assignment::<Fr>(honest()).expect("claim encodes");
    let circuit = rel.circuit(Some(a), Tamper::none()).expect("well-shaped assignment");
    let cs = ConstraintSystem::<Fr>::new_ref();
    circuit.synthesize_into(cs.clone()).expect("synthesis");
    assert!(cs.is_satisfied().expect("prove-mode system"));
    // R12-0s: setup mode carries no assignment and is unaffected.
    assert!(rel.circuit::<Fr>(None, Tamper::none()).is_ok());
}

#[test]
fn malformed_assignment_lengths_are_refused_before_synthesis() {
    let rel = relation();
    let cases: Vec<(&str, fn(&mut Assignment<Fr>))> = vec![
        ("R12-1 inputs short", |a: &mut Assignment<Fr>| a.values.inputs.clear()),
        ("R12-2 inputs long", |a: &mut Assignment<Fr>| a.values.inputs.push(u(0))),
        ("R12-3 pre states long", |a: &mut Assignment<Fr>| a.values.pre.states.push(u(0))),
        ("R12-4 pre cells short", |a: &mut Assignment<Fr>| a.values.pre.cells.clear()),
        ("R12-5 pre duties long", |a: &mut Assignment<Fr>| a.values.pre.duties.push(DutyValue { present: false, amount: UInt(0) })),
        ("R12-6 post cells long", |a: &mut Assignment<Fr>| a.values.post.cells.push(UInt(0))),
        ("R12-7 effects short", |a: &mut Assignment<Fr>| a.values.effects.clear()),
        ("R12-8 public short", |a: &mut Assignment<Fr>| {
            a.public.pop();
        }),
        ("R12-9 public long", |a: &mut Assignment<Fr>| a.public.push(Fr::from(0u64))),
    ];
    for (label, mutate) in cases {
        let mut a = rel.assignment::<Fr>(honest()).expect("claim encodes");
        mutate(&mut a);
        assert!(matches!(rel.circuit(Some(a), Tamper::none()), Err(CompileError::AssignmentShape(_))), "{label}");
    }
}
