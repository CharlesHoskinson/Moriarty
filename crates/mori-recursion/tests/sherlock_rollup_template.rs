#![cfg(feature = "sherlock-rollup")]
//! v3 acceptValidity's replay check exposes the same statement twice.
use mori_recursion::{
    template::{Slot, Template},
    util::{F, f_to_hex},
};

#[test]
fn repeated_statement_slots_are_equal_in_both_directions() {
    let template = Template {
        slots: vec![
            Slot::BindingInput,
            Slot::CommunicationCommitment,
            Slot::Const(F::from(7)),
            Slot::Stmt,
            Slot::Const(F::from(9)),
            Slot::Stmt,
        ],
    };
    template.validate().unwrap();
    let fields = template.public_inputs(F::from(1), F::from(2), F::from(3));
    assert_eq!(fields[3], fields[5]);
    assert_eq!(template.match_public_inputs(&fields), Some(F::from(3)));
    let mut changed = fields;
    changed[5] += F::from(1);
    assert_eq!(template.match_public_inputs(&changed), None);
}

#[test]
fn v3_capture_requires_exactly_the_declared_repeated_slots() {
    let fixture = serde_json::json!({"n":6,"free":[0,1,3,5],"stmt_index":3,"stmt_indices":[3,5],
        "constants":{"2":f_to_hex(&F::from(7)),"4":f_to_hex(&F::from(9))}});
    let parse = |v: &serde_json::Value| Template::from_any_json(&serde_json::to_vec(v).unwrap());
    let template = parse(&fixture).unwrap();
    assert_eq!(
        template.slots.iter().filter(|s| **s == Slot::Stmt).count(),
        2
    );
    for indices in [vec![3, 3], vec![3, 6], vec![5], vec![0, 3, 5]] {
        let mut bad = fixture.clone();
        bad["stmt_indices"] = serde_json::json!(indices);
        assert!(parse(&bad).is_err());
    }
}
