//! Sherlock C6P1 Chunk 3A public-API integration consumer (R-only proposed afterimage).
//!
//! External test crate compiled with `--test` against the ordinary production
//! `mori_core` rlib (Core `cfg(test)` false). Every codec, owner and observation
//! call goes to the actual public production item in the linked rlib; nothing is
//! included by path, proxied, stubbed or re-implemented here.
//!
//! The three tests are separately named external observations corresponding to
//! the three original `c6_phys_data::tests` unit tests. Those originals remain
//! UNRUN; any result here is its own record and never stands in for them.
//!
//! Locally owned inputs only: `caps`, `f`, `all_variants`, `every_lane_significant`
//! (verbatim from the 3A test module) and `fixtures` (the current Core
//! `c6_phys::fixtures` input blueprint, rebuilt through public `ExtProgram`
//! carriers, public `admit_ext_program` and the public fixed `host_owner_engine`).
//! `SECTION`/`DATA_S` are expected-only copies of private production strings.
//! Assertions marked `[ext+]` are external-only strengthenings, not 3A originals.

use mori_core::sherlock::helper_machine::c6_phys::{
    C6CodecBudgets, C6PhysCaps, C6PhysError, C6Result, C6_PHYS_MAGIC, C6_PHYS_VERSION,
};
use mori_core::sherlock::helper_machine::c6_phys_data::{
    C6DataTables, C6StateCaps, DATA_HEADER_LANES, DATA_ROW_LANES, DATA_SECTION_TAG, ENV_ROW_LANES,
    REF_LANES, data_lanes_from_host, decode_data_row, decode_data_tables, decode_env_row,
    decode_ref_row, encode_data_row, encode_data_tables, encode_env_row, encode_ref_row,
    roundtrip_data_tables_owned,
};
use mori_core::sherlock::helper_machine::c6_phys_program::PROGRAM_SECTION_TAG;
use mori_core::sherlock::helper_machine::complete_observation::{
    DataFieldsSnapshot, DataSnapshot, ObservedStep, capture, record_step,
};
use mori_core::sherlock::helper_machine::shared::adt::{CtorIdC, TypeIdC};
use mori_core::sherlock::helper_machine::shared::{
    BoundsC, EnvCellC, ExtCapacitiesC, HelperResultC, LayoutCapacitiesC, RefC,
};

/// EXPECTED-ONLY: value of the private production `c6_phys_data::SECTION`.
/// Used solely on the right-hand side of error expectations; never passed in.
const SECTION: &str = "state_data";
/// EXPECTED-ONLY: value of the private production `c6_phys_data::DATA_S`.
const DATA_S: &str = "state.data";

/// Local input fixture: the current Core `c6_phys::fixtures` blueprint (`#[cfg(test)]`
/// `pub(crate)`, unreachable from this crate) reproduced value-for-value through
/// public carriers. No `Admitted`/`MachineImage` internal is constructed here: both
/// come only from the owner's public admission and its public `admit_run`.
mod fixtures {
    use mori_core::sherlock::helper_machine::shared::i128_class::KIntC;
    use mori_core::sherlock::helper_machine::shared::owner_ext_source_program::{ExtDomain, ExtProgram};
    use mori_core::sherlock::helper_machine::shared::owner_ext_t3_uses::{
        ExtExpr, ExtFunction, ExtParamSrc, ExtValue,
    };
    use mori_core::sherlock::helper_machine::shared::owner_ext_type_builder::{
        CtorDeclSrc, CtorFieldSrc, TypeDeclSrc, TypeExprSrc,
    };
    use mori_core::sherlock::helper_machine::shared::owner_host::host_owner_engine;
    use mori_core::sherlock::helper_machine::shared::{
        Admitted, BoundsC, CoreValue, ExtCapacitiesC, FinanceInput, LayoutCapacitiesC,
        LayoutSchemaC, MachineImage, OwnerEngine, RunIntC, admit_ext_program,
    };

    /// Initial fuel; kept <= layout value_slots and max_reference_work.
    pub(crate) const FUEL: u128 = 4096;

    pub(crate) fn must<T, E>(result: Result<T, E>, message: &str) -> T {
        match result { Ok(value) => value, Err(_) => panic!("{message}") }
    }

    fn lit(n: u128) -> ExtExpr {
        ExtExpr::Literal { value: ExtValue::Core { value: CoreValue::UInt { n: KIntC::InRange(n) } } }
    }
    fn box_uint() -> TypeExprSrc {
        TypeExprSrc::Apply { name: "Box".into(), args: vec![TypeExprSrc::UInt] }
    }

    /// `type Box<A> = MkBox { item: A }`, `type Pair = MkPair { fst: UInt, snd: Box<UInt> }`,
    /// `fn mk(x: UInt) -> Pair { MkPair(x, MkBox(x)) }`, entry `let p = mk(7) in p`.
    /// The call gives genuine ArgsF and nominal ReturnF pushes. `KMachineBoundValid`
    /// schedules the ReturnF with the callee environment, so that push changes `env_root`.
    fn nominal_program() -> ExtProgram {
        let field = |name: &str, ty: TypeExprSrc| CtorFieldSrc { name: name.into(), ty };
        let var = |name: &str| ExtExpr::Var { name: name.into() };
        let pair = || TypeExprSrc::Named { name: "Pair".into() };
        let mk = ExtFunction {
            name: "mk".into(),
            parameters: vec![ExtParamSrc { name: "x".into(), ty: TypeExprSrc::UInt }],
            return_type: pair(),
            body: ExtExpr::Construct {
                ty: pair(), ctor: "MkPair".into(),
                fields: vec![var("x"), ExtExpr::Construct { ty: box_uint(), ctor: "MkBox".into(), fields: vec![var("x")] }],
            },
        };
        ExtProgram {
            types: vec![
                TypeDeclSrc { name: "Box".into(), parameters: vec!["A".into()], ctors: vec![CtorDeclSrc {
                    name: "MkBox".into(), fields: vec![field("item", TypeExprSrc::Param { name: "A".into() })] }] },
                TypeDeclSrc { name: "Pair".into(), parameters: Vec::new(), ctors: vec![CtorDeclSrc {
                    name: "MkPair".into(),
                    fields: vec![field("fst", TypeExprSrc::UInt), field("snd", box_uint())] }] },
            ],
            assets: Vec::new(), states: Vec::new(), inputs: Vec::new(), functions: vec![mk],
            entry: ExtExpr::Let {
                name: "p".into(),
                value: Box::new(ExtExpr::Call { name: "mk".into(), args: vec![lit(7)] }),
                body: Box::new(var("p")),
            },
        }
    }

    /// Real generic nominal admission and the owner's real `admit_run`.
    pub(crate) fn admitted_run(stack_limit: usize) -> (Admitted, MachineImage) {
        let bounds = BoundsC {
            code_rows: 128, function_rows: 16, code_child_slots: 128, parameter_slots: 32,
            pattern_rows: 32, match_plan_rows: 32, match_arm_rows: 32, type_rows: 16,
            constructor_rows: 16, constructor_field_slots: 32, max_arity: 8, constant_nodes: 128,
            constant_field_slots: 128, data_nodes: 128, data_field_slots: 128, environment_cells: 128,
            frames: 64, helper_tasks: 64, serialization_bytes: 1_000_000, max_reference_work: 1_000_000,
            max_helper_work: 1_000_000, step_cap: 10_000,
        };
        let layout = LayoutSchemaC {
            layout_version: ("c6-phys-test".into(), "1".into()), field_id: None, widths: None,
            scalar_layout_id: None, bounds,
            capacities: LayoutCapacitiesC { name_rows: 128, name_bytes: 4096, occurrence_path_slots: 4096,
                value_slots: 4096 },
            ext_capacities: Some(ExtCapacitiesC { plan_case_slots: 64, field_path_slots: 64,
                pattern_child_slots: 64, ..ExtCapacitiesC::all_zero() }),
            boundary_capacities_present: false,
        };
        let (admitted, _report) = must(admit_ext_program(nominal_program(), ExtDomain::GENERIC_ABSENT,
            &bounds, &layout, None), "generic nominal admission");
        let image = must(host_owner_engine().admit_run(&admitted, &RunIntC::Nat(FUEL),
            &RunIntC::Nat(stack_limit as u128), &FinanceInput::Absent), "admit run");
        (admitted, image)
    }
}

// ---------- local test helpers (verbatim from the 3A test module) ----------

fn caps() -> C6StateCaps { C6StateCaps::test_caps() }
fn f(start: usize, len: usize) -> DataFieldsSnapshot { DataFieldsSnapshot { start, len } }

fn all_variants() -> C6DataTables {
    C6DataTables {
        data: vec![
            DataSnapshot::U128 { n: u128::MAX }, DataSnapshot::U128 { n: 0 },
            DataSnapshot::Bool { b: true }, DataSnapshot::Bool { b: false }, DataSnapshot::Nil,
            DataSnapshot::Cons { fields: f(0, 2) }, DataSnapshot::Leaf { fields: f(2, 1) },
            DataSnapshot::Branch { fields: f(3, 2) },
            DataSnapshot::Constructor { type_id: TypeIdC(5), ctor_id: CtorIdC(6), fields: f(5, 3) },
            DataSnapshot::Constructor { type_id: TypeIdC(4), ctor_id: CtorIdC(4), fields: f(7, 0) },
        ],
        data_fields: vec![RefC::Const(3), RefC::Data(3), RefC::Data(3), RefC::Const(0),
            RefC::Data(0), RefC::Const(3), RefC::Data(9), RefC::Const(1)],
        env: vec![EnvCellC { binder: 2, value: RefC::Data(8), previous: None },
            EnvCellC { binder: 2, value: RefC::Const(8), previous: Some(0) },
            EnvCellC { binder: 0, value: RefC::Data(0), previous: Some(1) }],
        value_slots: vec![RefC::Data(9), RefC::Data(9), RefC::Const(9), RefC::Const(0)],
    }
}

/// Changing any single lane must refuse or decode to a different value.
fn every_lane_significant<const N: usize, T: PartialEq>(lanes: [u128; N], value: &T,
    decode: fn(&[u128; N]) -> C6Result<T>) {
    for at in 0..N {
        let mut changed = lanes;
        changed[at] ^= 0x55;
        assert!(decode(&changed).map_or(true, |v| &v != value), "lane {at} was ignored");
    }
}

// ---------- external observations ----------

#[test]
fn public3a_every_table_variant_and_raw_field_roundtrips_exactly() {
    // SYNTHETIC codec vectors over every variant; not admitted images.
    let t = all_variants();
    let lanes = encode_data_tables(&t, &caps()).unwrap();
    assert_eq!(lanes[..DATA_HEADER_LANES], [C6_PHYS_MAGIC, C6_PHYS_VERSION, DATA_SECTION_TAG, 10, 8, 3, 4]);
    assert_eq!(lanes.len(), 7 + 10 * DATA_ROW_LANES + 8 * REF_LANES + 3 * ENV_ROW_LANES + 4 * REF_LANES);
    assert_eq!(lanes[7..12], [0, u128::MAX, 0, 0, 0]);
    assert_eq!(lanes[32..37], [3, 0, 0, 0, 2]);
    assert_eq!(lanes[47..52], [6, 5, 6, 5, 3]);
    assert_eq!(lanes[52..57], [6, 4, 4, 7, 0], "nonzero start of an empty slice is kept");
    assert_eq!(lanes[57..61], [0, 3, 1, 3], "same-index Const and Data stay distinct");
    assert_eq!(lanes[73..83], [2, 1, 8, 0, 0, 2, 0, 8, 1, 0], "None vs Some(0) previous");
    let decoded = decode_data_tables(&lanes, &caps()).unwrap();
    assert_eq!(decoded, t);
    assert_eq!(encode_data_tables(&decoded, &caps()).unwrap(), lanes, "accepted lanes re-encode identically");
    assert_eq!(roundtrip_data_tables_owned(&t, &caps()).unwrap(), t);
    for row in &t.data { every_lane_significant(encode_data_row(row), row, decode_data_row); }
    for cell in &t.env { every_lane_significant(encode_env_row(cell), cell, decode_env_row); }
    for r in t.data_fields.iter().chain(&t.value_slots) {
        every_lane_significant(encode_ref_row(*r), r, |l| decode_ref_row("state.ref", l));
    }
    let differs = |m: C6DataTables| assert_ne!(encode_data_tables(&m, &caps()).unwrap(), lanes);
    let mut m = t.clone(); m.value_slots.swap(1, 2); differs(m);
    let mut m = t.clone(); m.data_fields[0] = RefC::Data(3); differs(m);
    let mut m = t.clone(); m.env[0].previous = Some(0); differs(m);
    let mut m = t.clone();
    m.data[9] = DataSnapshot::Constructor { type_id: TypeIdC(4), ctor_id: CtorIdC(4), fields: f(0, 0) };
    differs(m);
    let mut m = t; m.data[0] = DataSnapshot::U128 { n: u128::MAX - 1 }; differs(m);
}

#[test]
fn public3a_refusals_follow_the_declared_first_error_order() {
    let t = all_variants();
    let lanes = encode_data_tables(&t, &caps()).unwrap();
    let dec = |l: &[u128]| decode_data_tables(l, &caps());
    let set = |pairs: &[(usize, u128)]| { let mut m = lanes.clone(); for &(k, v) in pairs { m[k] = v; } m };
    fn cap<T>(section: &'static str, needed: usize, limit: usize) -> C6Result<T> {
        Err(C6PhysError::CapacityExceeded { section, needed, limit })
    }
    assert_eq!(dec(&lanes[..6]), Err(C6PhysError::Truncated { section: SECTION, needed: 7, actual: 6 }));
    assert_eq!(dec(&set(&[(0, 0), (2, 0)])), Err(C6PhysError::BadMagic { actual: 0 }));
    assert_eq!(dec(&set(&[(1, 2)])), Err(C6PhysError::BadVersion { actual: 2 }));
    assert_eq!(dec(&set(&[(2, PROGRAM_SECTION_TAG)])),
        Err(C6PhysError::BadTag { section: SECTION, tag: PROGRAM_SECTION_TAG }));
    // Structural counts precede count caps; count caps precede length.
    assert_eq!(dec(&set(&[(3, 999), (4, u128::MAX)])),
        Err(C6PhysError::UsizeOverflow { section: SECTION, value: u128::MAX }));
    assert_eq!(dec(&set(&[(3, 257)])), cap("state.data", 257, 256));
    assert_eq!(dec(&set(&[(3, 11)])), Err(C6PhysError::Truncated { section: SECTION, needed: 101, actual: 96 }));
    let mut trailing = lanes.clone();
    trailing.push(0);
    assert_eq!(dec(&trailing), Err(C6PhysError::TrailingLanes { section: SECTION, extra: 1 }));
    // Data rows at 7 + 5k: structural, row-relative lanes.
    assert_eq!(dec(&set(&[(7, 7)])), Err(C6PhysError::BadTag { section: DATA_S, tag: 7 }));
    assert_eq!(dec(&set(&[(18, 2)])), Err(C6PhysError::BadBool { section: DATA_S, value: 2 }));
    assert_eq!(dec(&set(&[(28, 1)])), Err(C6PhysError::NonZeroPadding { section: DATA_S, lane: 1, value: 1 }));
    assert_eq!(dec(&set(&[(34, 4)])), Err(C6PhysError::NonZeroPadding { section: DATA_S, lane: 2, value: 4 }));
    // Within a row, structure precedes caps; earlier tables precede later ones.
    assert_eq!(dec(&set(&[(48, 64), (49, u128::MAX)])),
        Err(C6PhysError::UsizeOverflow { section: DATA_S, value: u128::MAX }));
    assert_eq!(dec(&set(&[(48, 64), (88, 7)])), cap("state.data.type_id", 65, 64));
    assert_eq!(dec(&set(&[(49, 256)])), cap("state.data.ctor_id", 257, 256));
    assert_eq!(dec(&set(&[(51, 9)])), cap("state.data.fields", 9, 8));
    assert_eq!(dec(&set(&[(50, 1022)])), cap("state.data.fields", 1025, 1024));
    assert_eq!(dec(&set(&[(50, usize::MAX as u128)])),
        Err(C6PhysError::UsizeOverflow { section: "state.data.fields", value: usize::MAX as u128 }));
    // Ref, env and value-slot rows.
    assert_eq!(dec(&set(&[(57, 2)])), Err(C6PhysError::BadTag { section: "state.data_fields", tag: 2 }));
    assert_eq!(dec(&set(&[(77, 1)])), Err(C6PhysError::NonZeroPadding { section: "state.env", lane: 4, value: 1 }));
    assert_eq!(dec(&set(&[(76, 2)])), Err(C6PhysError::BadTag { section: "state.env", tag: 2 }));
    assert_eq!(dec(&set(&[(73, 256)])), cap("state.env.binder", 257, 256));
    assert_eq!(dec(&set(&[(82, 256)])), cap("state.env.previous", 257, 256));
    assert_eq!(dec(&set(&[(88, 7)])), Err(C6PhysError::BadTag { section: "state.value_slots", tag: 7 }));
    // Const and Data references use their own tables.
    let one = |r| C6DataTables { data: vec![], data_fields: vec![r], env: vec![], value_slots: vec![] };
    let mut c = caps();
    c.phys.max_data_nodes = 4;
    c.phys.max_constants = 9;
    assert!(encode_data_tables(&one(RefC::Const(8)), &c).is_ok());
    assert_eq!(encode_data_tables(&one(RefC::Data(4)), &c), cap("state.data_fields.ref", 5, 4));
    let mut l = encode_data_tables(&one(RefC::Data(3)), &c).unwrap();
    l[8] = 4;
    assert_eq!(decode_data_tables(&l, &c), cap("state.data_fields.ref", 5, 4));
    // The encoder applies the same caps; the data-field limit is its own.
    let mut tight = caps();
    tight.max_data_field_slots = 7;
    assert_eq!(encode_data_tables(&t, &tight), cap("state.data_fields", 8, 7));
    assert_eq!(decode_data_tables(&lanes, &tight), cap("state.data_fields", 8, 7));
    let mut wide = t;
    wide.data[8] = DataSnapshot::Constructor { type_id: TypeIdC(64), ctor_id: CtorIdC(6), fields: f(5, 3) };
    assert_eq!(encode_data_tables(&wide, &caps()), cap("state.data.type_id", 65, 64));
    // Exact admitted mapping: no borrowed or defaulted limit.
    let bounds = BoundsC { data_nodes: 5, data_field_slots: 11, helper_tasks: 13, ..BoundsC::default() };
    let budgets = C6CodecBudgets { type_expr_nodes: 1, template_bytes: 2 };
    let (lc, ext) = (LayoutCapacitiesC::default(), ExtCapacitiesC::all_zero());
    assert_eq!(C6StateCaps::from_parts(&bounds, &lc, &ext, budgets), C6StateCaps {
        phys: C6PhysCaps::from_parts(&bounds, &lc, &ext, budgets),
        max_data_field_slots: 11, max_helper_tasks: 13 });
    // [ext+] EXTERNAL-ONLY strengthening (not a 3A original): the linked rlib's
    // convenience caps are exactly those every expectation above was derived from.
    assert_eq!(caps(), C6StateCaps { phys: C6PhysCaps::test_caps(), max_data_field_slots: 1024,
        max_helper_tasks: 64 });
    assert_eq!((caps().phys.max_types, caps().phys.max_arity, caps().phys.max_data_nodes,
        caps().phys.max_names, caps().phys.max_env_cells), (64, 8, 256, 256, 256));
}

#[test]
fn public3a_captured_nominal_host_tables_roundtrip_and_preserve_host() {
    // OWNER-REACHABLE: real generic nominal admission, real fixed-owner steps.
    let (admitted, mut image) = fixtures::admitted_run(64);
    let mut terminal = false;
    for _ in 0..4096 {
        match record_step(&admitted, &mut image) {
            Ok(ObservedStep::Event(_)) => {}
            Ok(ObservedStep::Terminal { .. }) => { terminal = true; break; }
            Err(_) => panic!("owner step"),
        }
    }
    assert!(terminal, "the nominal run reaches its terminal boundary");
    let host = fixtures::must(capture(&admitted, &image), "capture");
    let img = &host.image;
    assert!(img.data.iter().any(|d| matches!(d, DataSnapshot::Constructor { .. })));
    assert!(img.data_fields.iter().any(|r| matches!(r, RefC::Const(_))));
    assert!(img.data_fields.iter().any(|r| matches!(r, RefC::Data(_))));
    assert!(!img.env.is_empty() && !img.value_slots.is_empty());
    // Admitted bounds/capacities of this host; the data section reads no ext limit.
    let caps = C6StateCaps::from_parts(&host.admission.bounds, &host.admission.layout.capacities,
        &ExtCapacitiesC::all_zero(), C6CodecBudgets { type_expr_nodes: 0, template_bytes: 0 });
    let lanes = data_lanes_from_host(&host, &caps).unwrap();
    let owned = C6DataTables::from_image(img);
    assert_eq!(lanes, encode_data_tables(&owned, &caps).unwrap());
    let decoded = decode_data_tables(&lanes, &caps).unwrap();
    assert_eq!(decoded, owned);
    assert_eq!(encode_data_tables(&decoded, &caps).unwrap(), lanes);
    // Replace ONLY the four tables of a clone: the whole owned host is equal.
    let mut rebuilt = host.clone();
    C6DataTables { data: vec![], data_fields: vec![], env: vec![], value_slots: vec![] }
        .replace_tables_of(&mut rebuilt.image);
    assert_ne!(rebuilt, host);
    decoded.replace_tables_of(&mut rebuilt.image);
    assert_eq!(rebuilt, host);
    // Root counters and other state are not in this section.
    let mut other = host.clone();
    other.image.alloc.data_nodes += 1;
    other.image.active.data_fields += 1;
    other.image.reference_cursor += 7;
    other.image.frames.clear();
    other.image.result = Some(HelperResultC::EnvMissing);
    assert_eq!(data_lanes_from_host(&other, &caps).unwrap(), lanes);
    // A table field change is visible.
    let mut changed = host.clone();
    changed.image.data_fields[0] = match changed.image.data_fields[0] {
        RefC::Const(i) => RefC::Data(i), RefC::Data(i) => RefC::Const(i) };
    assert_ne!(data_lanes_from_host(&changed, &caps).unwrap(), lanes);
    let n = img.data_fields.len();
    let mut tight = caps;
    tight.max_data_field_slots = n - 1;
    assert_eq!(data_lanes_from_host(&host, &tight),
        Err(C6PhysError::CapacityExceeded { section: "state.data_fields", needed: n, limit: n - 1 }));
    // [ext+] EXTERNAL-ONLY strengthening (not a 3A original): the public admission
    // route carried exactly the local fixture's limits, so the Const<->Data flip
    // above was refused or accepted under equal 128/128 index caps.
    assert_eq!(host.admission.layout.bounds, host.admission.bounds);
    assert_eq!(host.admission.layout.layout_version, ("c6-phys-test".to_string(), "1".to_string()));
    assert_eq!(host.admission.layout.capacities, LayoutCapacitiesC { name_rows: 128, name_bytes: 4096,
        occurrence_path_slots: 4096, value_slots: 4096 });
    assert_eq!((caps.phys.max_constants, caps.phys.max_data_nodes, caps.phys.max_env_cells,
        caps.max_data_field_slots), (128, 128, 128, 128));
}
