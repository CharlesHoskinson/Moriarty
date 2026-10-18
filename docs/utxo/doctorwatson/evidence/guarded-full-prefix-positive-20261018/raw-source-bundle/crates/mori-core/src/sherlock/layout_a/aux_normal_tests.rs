//! Normal literal packing examples, authored before the component. All UNRUN.
use super::auxiliary::*;
use super::rows::{Capacities, TableId, ValueRef};
use super::state::{EnvRoot, StateCapacities};

fn dimensions() -> (Capacities, StateCapacities, AuxDimensions) {
    let mut p = Capacities { tables: [0; 15], data: 1 };
    for id in [TableId::Names, TableId::Types, TableId::Constructors, TableId::Expressions] {
        p.tables[id as usize] = 4;
    }
    p.tables[TableId::Functions as usize] = 2;
    p.tables[TableId::Parameters as usize] = 3;
    p.tables[TableId::Constants as usize] = 1;
    p.tables[TableId::ConstantFields as usize] = 2;
    let s = StateCapacities { data: 1, data_fields: 2, env_cells: 1,
        value_slots: 4, frame_slots: 0, control_rows: 0, stack_limit: 0 };
    (p, s, AuxDimensions { q_eq: 1 })
}

// Zero supplied records, not a trace generator; fixture sizes are model examples.
fn zero_block() -> AuxBlock {
    AuxBlock {
        env_get: EnvGetScan { res_found: false, res_value: ValueRef::default(),
            rows: vec![EnvGetRow::default(); 1] },
        find: FindScan { out_present: false, out_id: 0, rows: vec![FindRow::default(); 2] },
        bind: BindPreflight { header: BindHeader::default(),
            rows: vec![BindRow::default(); 3], eq_name: vec![false; 3] },
        eq: EqScratch { rows: vec![EqRow { act: false, sp: 0,
            stack: vec![RefPair::default(); 3], left: AuxNode::default(),
            right: AuxNode::default(), hd: false, fields_left: [ValueRef::default(); 2],
            fields_right: [ValueRef::default(); 2], term: EqTerm::None }] },
    }
}

#[test]
fn literal_env_found_complete_block() {
    let (p,s,d) = dimensions();
    let mut b = zero_block();
    b.env_get.res_found = true;
    b.env_get.rows[0] = EnvGetRow { act: true, cur: EnvRoot { present: true, index: 0 },
        name_id: 3, value: ValueRef::constant(0), parent: EnvRoot::default(),
        matches: true, term: EnvTerm::Found };
    assert_eq!(b.ordered_bytes(&p,&s,&d).unwrap(), ENV_FOUND);
}

#[test]
fn literal_find_retains_seen_in_inactive_tail() {
    let (p,s,d) = dimensions();
    let mut b = zero_block();
    b.find.out_present = true;
    b.find.rows[0] = FindRow { act: true, name_id: 2, matches: true, seen: true, first: true };
    b.find.rows[1].seen = true;
    assert_eq!(b.ordered_bytes(&p,&s,&d).unwrap(), FIND_TAIL_SEEN);
}

#[test]
fn literal_bind_reverse_duplicate_and_upper_triangle() {
    let (p,s,d) = dimensions();
    let mut b = zero_block();
    b.bind.header = BindHeader { parameters_start: 0, k: 3, return_type: 0,
        body: 2, values_start: 0, values_len: 3, len_ok: true, ok: false };
    b.bind.rows = vec![
        BindRow { reached: true, name_id: 1, parameter_type: 0,
            argument: ValueRef::constant(0), argument_type: 0, type_ok: true,
            duplicate: true, pass: false },
        BindRow { reached: true, name_id: 2, parameter_type: 0,
            argument: ValueRef { space: true, index: 0 }, argument_type: 0,
            type_ok: true, duplicate: false, pass: true },
        BindRow { reached: true, name_id: 1, parameter_type: 0,
            argument: ValueRef::constant(0), argument_type: 0, type_ok: true,
            duplicate: false, pass: true },
    ];
    // (0,1), (0,2), (1,2), no diagonal/lower triangle/pass_P.
    b.bind.eq_name = vec![false, true, false];
    assert_eq!(b.ordered_bytes(&p,&s,&d).unwrap(), BIND_DUPLICATE);
}

#[test]
fn literal_eq_leaf_with_fixed_stack_and_complete_uint_bytes() {
    let (p,s,d) = dimensions();
    let mut b = zero_block();
    let r = &mut b.eq.rows[0];
    r.act = true; r.sp = 1;
    r.stack[0] = RefPair { left: ValueRef { space: true, index: 0 },
        right: ValueRef::constant(0) };
    r.left.uint_bytes = [52,18,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
    r.right.uint_bytes = [52,18,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
    r.hd = true; r.term = EqTerm::Empty;
    assert_eq!(b.ordered_bytes(&p,&s,&d).unwrap(), EQ_UINT);
}

#[test]
fn literal_multibyte_name_width_changes_all_fixed_rows() {
    let (mut p,s,d) = dimensions();
    p.tables[TableId::Names as usize] = 257;
    let mut b = zero_block();
    b.env_get.res_found = true;
    b.env_get.rows[0] = EnvGetRow { act: true, cur: EnvRoot { present: true, index: 0 },
        name_id: 256, value: ValueRef::constant(0), parent: EnvRoot::default(),
        matches: true, term: EnvTerm::Found };
    assert_eq!(b.ordered_bytes(&p,&s,&d).unwrap(), WIDE_NAME);
}

#[test]
fn literal_empty_program_dimensions_keep_minimum_env_and_eq_rows() {
    let p = Capacities { tables: [0;15], data: 0 };
    let s = StateCapacities { data: 0, data_fields: 0, env_cells: 0,
        value_slots: 0, frame_slots: 0, control_rows: 0, stack_limit: 0 };
    let d = AuxDimensions { q_eq: 1 };
    let mut b = zero_block();
    b.find.rows.clear(); b.bind.rows.clear(); b.bind.eq_name.clear();
    b.eq.rows[0].stack.truncate(1);
    assert_eq!(b.ordered_bytes(&p,&s,&d).unwrap(), ZERO_MINIMUM);
}

const ENV_FOUND: [u8; 131] = [
    1, 0, 0, 1, 1, 0, 3, 0, 0, 0, 0, 1, 1, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0,
];

const FIND_TAIL_SEEN: [u8; 131] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1,
    2, 1, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0,
];

const BIND_DUPLICATE: [u8; 131] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 2, 0, 3, 1,
    0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 1, 2, 0, 1, 0, 0,
    1, 0, 1, 1, 1, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0,
];

const EQ_UINT: [u8; 131] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    52, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 52, 18, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
    0, 0, 2,
];

const WIDE_NAME: [u8; 137] = [
    1, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0,
];

const ZERO_MINIMUM: [u8; 83] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0,
];
