//! F7/N2 owner-authored malformed-view regression properties. Source only: the
//! author did not compile or run it. Each case prepares a real GenericNominal
//! view through the public typed preparation, mutates public ProgramView
//! fields, and requires a typed ProgramViewMalformed rejection (never a panic)
//! from check_program_view_ext and from the fixed OwnerEngine::check_program_view.
//! These are not independent whole-runtime semantic expectations.

use std::panic::{AssertUnwindSafe, catch_unwind};

use mori_core::sherlock::helper_machine::shared::adt::CtorIdC;
use mori_core::sherlock::helper_machine::shared::i128_class::KIntC;
use mori_core::sherlock::helper_machine::shared::owner_ext_admit::{
    ExtCapacitiesC, PreparationReportC, admit_ext_program, check_program_view_ext,
    prepare_ext_program,
};
use mori_core::sherlock::helper_machine::shared::owner_ext_source_program::{
    ExtDomain, ExtProgram,
};
use mori_core::sherlock::helper_machine::shared::owner_ext_t3_uses::{
    ExtArm, ExtExpr, ExtPattern, ExtValue,
};
use mori_core::sherlock::helper_machine::shared::owner_ext_type_builder::{
    CtorDeclSrc, CtorFieldSrc, ExtTypeBuildError, TypeDeclSrc, TypeExprSrc,
};
use mori_core::sherlock::helper_machine::shared::owner_host::host_owner_engine;
use mori_core::sherlock::helper_machine::shared::pure_leaves::{BorrowedValueRow, borrowed_kind};
use mori_core::sherlock::helper_machine::shared::{
    AdmissionReason, BoundsC, CorePattern, CoreValue, ExpressionRowC, LayoutCapacitiesC,
    LayoutSchemaC, LogicalKind, OwnerEngine, PatternRowC, PlanNodeC, ProgramView, Tooling, TypeIdC,
    ViewCheck,
};

fn bounds() -> BoundsC {
    BoundsC {
        code_rows: 64,
        function_rows: 8,
        code_child_slots: 64,
        parameter_slots: 16,
        pattern_rows: 32,
        match_plan_rows: 32,
        match_arm_rows: 16,
        type_rows: 16,
        constructor_rows: 16,
        constructor_field_slots: 32,
        max_arity: 4,
        constant_nodes: 64,
        constant_field_slots: 64,
        data_nodes: 256,
        data_field_slots: 256,
        environment_cells: 256,
        frames: 64,
        helper_tasks: 64,
        serialization_bytes: 1 << 16,
        max_reference_work: 1 << 20,
        max_helper_work: 1 << 20,
        step_cap: 1 << 20,
    }
}

fn layout() -> LayoutSchemaC {
    LayoutSchemaC {
        layout_version: (
            "moriarty".to_string(),
            "f7-view-bounds-regression".to_string(),
        ),
        field_id: None,
        widths: None,
        scalar_layout_id: None,
        bounds: bounds(),
        capacities: LayoutCapacitiesC {
            name_rows: 64,
            name_bytes: 1024,
            occurrence_path_slots: 256,
            value_slots: 256,
        },
        ext_capacities: Some(ExtCapacitiesC {
            plan_case_slots: 32,
            field_path_slots: 32,
            pattern_child_slots: 32,
            ..ExtCapacitiesC::all_zero()
        }),
        boundary_capacities_present: false,
    }
}

fn named(name: &str) -> TypeExprSrc {
    TypeExprSrc::Named {
        name: name.to_string(),
    }
}

fn uint(n: u128) -> ExtValue {
    ExtValue::Core {
        value: CoreValue::UInt {
            n: KIntC::InRange(n),
        },
    }
}

fn nothing() -> ExtExpr {
    ExtExpr::Construct {
        ty: named("Opt"),
        ctor: "Nothing".to_string(),
        fields: Vec::new(),
    }
}

/// type Opt = Nothing | Just(value: UInt); type Pair = MkPair(fst: UInt, snd: UInt);
/// let p = MkPair(1, 2) in match Just(7) { Just(x) => x, Nothing => if true then Nothing else Nothing }
fn baseline_program() -> ExtProgram {
    let field = |name: &str| CtorFieldSrc {
        name: name.to_string(),
        ty: TypeExprSrc::UInt,
    };
    let types = vec![
        TypeDeclSrc {
            name: "Opt".to_string(),
            parameters: Vec::new(),
            ctors: vec![
                CtorDeclSrc {
                    name: "Nothing".to_string(),
                    fields: Vec::new(),
                },
                CtorDeclSrc {
                    name: "Just".to_string(),
                    fields: vec![field("value")],
                },
            ],
        },
        TypeDeclSrc {
            name: "Pair".to_string(),
            parameters: Vec::new(),
            ctors: vec![CtorDeclSrc {
                name: "MkPair".to_string(),
                fields: vec![field("fst"), field("snd")],
            }],
        },
    ];
    let just_arm = ExtArm {
        pattern: ExtPattern::Ctor {
            ty: named("Opt"),
            ctor: "Just".to_string(),
            fields: vec![ExtPattern::Dedicated {
                pattern: CorePattern::PVar { x: "x".to_string() },
            }],
        },
        body: ExtExpr::Var {
            name: "x".to_string(),
        },
    };
    let nothing_arm = ExtArm {
        pattern: ExtPattern::Ctor {
            ty: named("Opt"),
            ctor: "Nothing".to_string(),
            fields: Vec::new(),
        },
        body: ExtExpr::If {
            cond: Box::new(ExtExpr::Literal {
                value: ExtValue::Core {
                    value: CoreValue::Bool { b: true },
                },
            }),
            then_expr: Box::new(nothing()),
            else_expr: Box::new(nothing()),
        },
    };
    let scrutinee = ExtExpr::Construct {
        ty: named("Opt"),
        ctor: "Just".to_string(),
        fields: vec![ExtExpr::Literal { value: uint(7) }],
    };
    let pair = ExtValue::Constructor {
        ty: named("Pair"),
        ctor: "MkPair".to_string(),
        fields: vec![uint(1), uint(2)],
    };
    let entry = ExtExpr::Let {
        name: "p".to_string(),
        value: Box::new(ExtExpr::Literal { value: pair }),
        body: Box::new(ExtExpr::Match {
            scrutinee: Box::new(scrutinee),
            arms: vec![just_arm, nothing_arm],
        }),
    };
    ExtProgram {
        types,
        assets: Vec::new(),
        states: Vec::new(),
        inputs: Vec::new(),
        functions: Vec::new(),
        entry,
    }
}

fn describe(e: &ExtTypeBuildError) -> String {
    match e {
        ExtTypeBuildError::CoreTooling(Tooling::AdmissionRejected { reason, detail, .. }) => {
            format!("AdmissionRejected {reason:?} {detail}")
        }
        ExtTypeBuildError::CoreTooling(Tooling::CapacityNotAdmitted { kind, .. }) => {
            format!("CapacityNotAdmitted {kind:?}")
        }
        ExtTypeBuildError::CoreTooling(Tooling::UnsupportedFeature { feature, .. }) => {
            format!("UnsupportedFeature {feature:?}")
        }
        ExtTypeBuildError::ExtCapacityNotAdmitted { kind, .. } => {
            format!("ExtCapacityNotAdmitted {kind:?}")
        }
        ExtTypeBuildError::ExtAdmissionRejected { reason, detail, .. } => {
            format!("ExtAdmissionRejected {reason:?} {detail}")
        }
        ExtTypeBuildError::ExtUnsupportedFeature { feature, .. } => {
            format!("ExtUnsupportedFeature {feature:?}")
        }
        ExtTypeBuildError::Host(h) => format!("Host {:?} {:?}", h.kind, h.locator),
    }
}

fn prepared() -> (ProgramView, PreparationReportC) {
    match prepare_ext_program(
        baseline_program(),
        ExtDomain::GENERIC_ABSENT,
        &bounds(),
        &layout(),
        None,
    ) {
        Ok(pair) => pair,
        Err(e) => panic!("baseline preparation failed: {}", describe(&e)),
    }
}

/// Both public entries must return the typed malformed-view rejection carrying
/// `expected` as the phase check id, and neither may panic.
fn assert_rejected_by_both_entries(v: &ProgramView, expected: &str) {
    let (b, l) = (bounds(), layout());
    let ext = catch_unwind(AssertUnwindSafe(|| {
        check_program_view_ext(v, None, ExtDomain::GENERIC_ABSENT, &b, &l)
    }));
    match ext {
        Err(_) => panic!("check_program_view_ext panicked on a malformed view"),
        Ok(Ok(())) => panic!("check_program_view_ext accepted a malformed view"),
        Ok(Err(ExtTypeBuildError::CoreTooling(Tooling::AdmissionRejected {
            reason: AdmissionReason::ProgramViewMalformed,
            detail,
            locator: None,
        }))) => assert_eq!(detail, expected, "check_program_view_ext check id"),
        Ok(Err(e)) => panic!(
            "unexpected check_program_view_ext error class: {}",
            describe(&e)
        ),
    }
    let engine = host_owner_engine();
    match catch_unwind(AssertUnwindSafe(|| engine.check_program_view(v, &b, &l))) {
        Err(_) => panic!("OwnerEngine::check_program_view panicked on a malformed view"),
        Ok(Ok(())) => panic!("OwnerEngine::check_program_view accepted a malformed view"),
        Ok(Err(pv)) => {
            assert_eq!(pv.check, ViewCheck::V8);
            assert_eq!(pv.table, "program");
            assert_eq!(pv.row, None);
            assert_eq!(
                pv.detail, expected,
                "OwnerEngine::check_program_view check id"
            );
        }
    }
}

fn plan_source_match(n: &PlanNodeC) -> usize {
    match *n {
        PlanNodeC::Switch { source_match, .. }
        | PlanNodeC::Bind { source_match, .. }
        | PlanNodeC::Select { source_match, .. }
        | PlanNodeC::NoMatch { source_match } => source_match,
    }
}

fn set_plan_source_match(n: &mut PlanNodeC, value: usize) {
    match n {
        PlanNodeC::Switch { source_match, .. }
        | PlanNodeC::Bind { source_match, .. }
        | PlanNodeC::Select { source_match, .. }
        | PlanNodeC::NoMatch { source_match } => *source_match = value,
    }
}

fn planned_match(v: &ProgramView) -> (usize, usize) {
    let planned: Vec<(usize, usize)> = v
        .expressions
        .iter()
        .enumerate()
        .filter_map(|(ei, e)| match e {
            ExpressionRowC::Match {
                plan: Some(root), ..
            } => Some((ei, *root)),
            _ => None,
        })
        .collect();
    assert_eq!(planned.len(), 1, "baseline has exactly one planned Match");
    planned[0]
}

fn plan_block_end(v: &ProgramView, ei: usize, root: usize) -> usize {
    let mut end = root;
    while end < v.plan_nodes.len() && plan_source_match(&v.plan_nodes[end]) == ei {
        end += 1;
    }
    end
}

fn nullary_constructs(v: &ProgramView) -> Vec<(usize, usize)> {
    v.expressions
        .iter()
        .enumerate()
        .filter_map(|(ei, e)| match e {
            ExpressionRowC::Construct {
                constant: Some(cid),
                ..
            } => Some((ei, *cid)),
            _ => None,
        })
        .collect()
}

fn set_construct_constant(v: &mut ProgramView, ei: usize, cid: usize) {
    match &mut v.expressions[ei] {
        ExpressionRowC::Construct { constant, .. } => *constant = Some(cid),
        _ => panic!("expected a Construct row"),
    }
}

fn first_ctor_pattern(v: &ProgramView) -> (usize, TypeIdC, CtorIdC) {
    v.patterns
        .iter()
        .enumerate()
        .find_map(|(pi, p)| match *p {
            PatternRowC::Constructor {
                type_id, ctor_id, ..
            } => Some((pi, type_id, ctor_id)),
            _ => None,
        })
        .unwrap_or_else(|| panic!("baseline has a Constructor pattern row"))
}

/// The single fielded ConstRow.Constructor (MkPair). No pattern row names its
/// type, so V16 never reads the type/constructor rows mutated for V11 cases.
fn v11_target(v: &ProgramView) -> (TypeIdC, CtorIdC) {
    let found: Vec<(TypeIdC, CtorIdC)> = v
        .constants
        .iter()
        .filter_map(|row| match borrowed_kind(BorrowedValueRow::Const(row)) {
            LogicalKind::Constructor { type_id, ctor_id }
                if v.constructors[ctor_id.0].field_len > 0 =>
            {
                Some((type_id, ctor_id))
            }
            _ => None,
        })
        .collect();
    assert_eq!(
        found.len(),
        1,
        "baseline has exactly one fielded constructor constant"
    );
    let (ty, ctor) = found[0];
    assert!(ty.0 >= 4);
    assert!(v.patterns.iter().all(|p| !matches!(p,
        PatternRowC::Constructor { type_id, .. } if *type_id == ty)));
    (ty, ctor)
}

#[test]
fn valid_prepared_baseline_is_accepted_by_both_public_entries() {
    let (v, report) = prepared();
    let (b, l) = (bounds(), layout());
    if let Err(e) = check_program_view_ext(&v, Some(&report), ExtDomain::GENERIC_ABSENT, &b, &l) {
        panic!("baseline rejected with report: {}", describe(&e));
    }
    if let Err(e) = check_program_view_ext(&v, None, ExtDomain::GENERIC_ABSENT, &b, &l) {
        panic!("baseline rejected without report: {}", describe(&e));
    }
    if let Err(pv) = host_owner_engine().check_program_view(&v, &b, &l) {
        panic!(
            "baseline rejected by OwnerEngine::check_program_view: {}",
            pv.detail
        );
    }
    if let Err(e) = admit_ext_program(baseline_program(), ExtDomain::GENERIC_ABSENT, &b, &l, None) {
        panic!("baseline admission failed: {}", describe(&e));
    }
    let (ei, root) = planned_match(&v);
    assert_eq!(plan_source_match(&v.plan_nodes[root]), ei);
    assert!(plan_block_end(&v, ei, root) - root >= 2);
    assert_eq!(nullary_constructs(&v).len(), 2);
    let (_, pat_ty, _) = first_ctor_pattern(&v);
    let (pair_ty, _) = v11_target(&v);
    assert_ne!(pat_ty, pair_ty);
}

#[test]
fn n2_plan_root_with_foreign_source_match_is_rejected() {
    let (mut v, _) = prepared();
    let (ei, root) = planned_match(&v);
    assert_eq!(plan_source_match(&v.plan_nodes[root]), ei);
    set_plan_source_match(&mut v.plan_nodes[root], ei + 1);
    // Root still in range, block now empty: pre-F7 V13 indexed has_parent[0].
    assert!(root < v.plan_nodes.len());
    assert_eq!(plan_block_end(&v, ei, root), root);
    assert_rejected_by_both_entries(&v, "V13");
}

#[test]
fn n2_neighbor_last_block_node_with_foreign_source_match_is_rejected() {
    let (mut v, _) = prepared();
    let (ei, root) = planned_match(&v);
    let end = plan_block_end(&v, ei, root);
    assert!(end - root >= 2);
    set_plan_source_match(&mut v.plan_nodes[end - 1], ei + 1);
    assert_eq!(plan_block_end(&v, ei, root), end - 1);
    // Its in-block parent now targets a node outside the block; the earlier V9
    // generic plan-target sub-check reports first (precedence preserved).
    assert_rejected_by_both_entries(&v, "V9");
}

#[test]
fn v16_pattern_type_index_out_of_range_is_rejected() {
    let (mut v, _) = prepared();
    let (pi, _, _) = first_ctor_pattern(&v);
    let bad = TypeIdC(v.types.len());
    match &mut v.patterns[pi] {
        PatternRowC::Constructor { type_id, .. } => *type_id = bad,
        _ => panic!("expected a Constructor pattern row"),
    }
    assert!(v.types.get(bad.0).is_none());
    assert_rejected_by_both_entries(&v, "V16");
}

#[test]
fn v16_pattern_ctor_range_overflow_is_rejected() {
    let (mut v, _) = prepared();
    let (_, ty, _) = first_ctor_pattern(&v);
    v.types[ty.0].ctor_len = usize::MAX;
    let t = v.types[ty.0];
    assert!(t.ctor_start.0.checked_add(t.ctor_len).is_none());
    assert_rejected_by_both_entries(&v, "V16");
}

#[test]
fn v16_pattern_ctor_index_beyond_constructor_table_is_rejected() {
    let (mut v, _) = prepared();
    let (pi, ty, _) = first_ctor_pattern(&v);
    let bad = CtorIdC(v.constructors.len());
    let start = v.types[ty.0].ctor_start.0;
    v.types[ty.0].ctor_len = bad.0 - start + 1;
    match &mut v.patterns[pi] {
        PatternRowC::Constructor { ctor_id, .. } => *ctor_id = bad,
        _ => panic!("expected a Constructor pattern row"),
    }
    // Inside the (mutated) type slice but past the end of prog.constructors.
    assert!(bad.0 >= start && bad.0 < start + v.types[ty.0].ctor_len);
    assert!(v.constructors.get(bad.0).is_none());
    assert_rejected_by_both_entries(&v, "V16");
}

#[test]
fn v11_constant_ctor_range_overflow_is_rejected() {
    let (mut v, _) = prepared();
    let (ty, _) = v11_target(&v);
    v.types[ty.0].ctor_len = usize::MAX;
    let t = v.types[ty.0];
    assert!(t.ctor_start.0.checked_add(t.ctor_len).is_none());
    assert_rejected_by_both_entries(&v, "V11");
}

#[test]
fn v11_constant_ctor_index_beyond_constructor_table_is_rejected() {
    let (mut v, _) = prepared();
    let (ty, ctor) = v11_target(&v);
    assert_eq!(
        ctor.0,
        v.constructors.len() - 1,
        "fielded literal constructor is the last row"
    );
    assert!(v.patterns.iter().all(|p| match p {
        PatternRowC::Constructor { ctor_id, .. } => ctor_id.0 < ctor.0,
        _ => true,
    }));
    v.constructors.truncate(ctor.0);
    let t = v.types[ty.0];
    assert!(ctor.0 >= t.ctor_start.0 && ctor.0 < t.ctor_start.0 + t.ctor_len);
    assert!(v.constructors.get(ctor.0).is_none() && v.constructors.len() >= 4);
    assert_rejected_by_both_entries(&v, "V11");
}

#[test]
fn v11_constant_ctor_field_slice_overflow_is_rejected() {
    let (mut v, _) = prepared();
    let (_, ctor) = v11_target(&v);
    v.constructors[ctor.0].field_start = usize::MAX;
    let c = v.constructors[ctor.0];
    assert!(c.field_len > 0 && c.field_start.checked_add(c.field_len).is_none());
    assert_rejected_by_both_entries(&v, "V11");
}

#[test]
fn v11_constant_ctor_field_slice_past_table_end_is_rejected() {
    let (mut v, _) = prepared();
    let (_, ctor) = v11_target(&v);
    v.constructors[ctor.0].field_start = v.ctor_fields.len() - 1;
    let c = v.constructors[ctor.0];
    // Starts inside prog.ctorFields, ends past it.
    assert!(c.field_len >= 2 && c.field_start < v.ctor_fields.len());
    assert!(c.field_start + c.field_len > v.ctor_fields.len());
    assert_rejected_by_both_entries(&v, "V11");
}

#[test]
fn v12_nullary_construct_bound_to_non_constructor_constant_is_rejected() {
    let (mut v, _) = prepared();
    let nullary = nullary_constructs(&v);
    assert_eq!(nullary.len(), 2);
    let scalar = v
        .constants
        .iter()
        .position(|row| borrowed_kind(BorrowedValueRow::Const(row)) == LogicalKind::UInt)
        .unwrap_or_else(|| panic!("baseline has a UInt constant"));
    set_construct_constant(&mut v, nullary[0].0, scalar);
    // The earlier V9 generic nullary-row sub-check reports first; V12 repeats it.
    assert_rejected_by_both_entries(&v, "V9");
}

#[test]
fn v12_two_nullary_constructs_sharing_one_owned_constant_are_rejected() {
    let (mut v, _) = prepared();
    let nullary = nullary_constructs(&v);
    assert_eq!(nullary.len(), 2);
    let (_, owned) = nullary[0];
    let (second, other) = nullary[1];
    assert_ne!(owned, other);
    set_construct_constant(&mut v, second, owned);
    assert_rejected_by_both_entries(&v, "V12");
}
