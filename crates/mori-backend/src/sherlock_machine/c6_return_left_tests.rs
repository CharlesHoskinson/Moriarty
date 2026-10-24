//! UNCOMPILED_UNRUN source. No SDK preprocessing/synthesis/proof was executed.
//! Real owner admission/capture/record_step supplies positive states. Public row
//! denotation supplies independent expected full state/event effects, not receipts.
use super::super::c6_return_left_step::*;
use super::*;
use mori_core::sherlock::helper_machine::c6_phys::*;
use mori_core::sherlock::helper_machine::c6_phys_helper::{L_HELPER_RECORD, L_RESULT_RECORD};
use mori_core::sherlock::helper_machine::complete_observation::{
    CompleteStepRecord, DataSnapshot, ObservedStep, capture, record_step,
};
use mori_core::sherlock::helper_machine::shared::owner_host::host_owner_engine;
use mori_core::sherlock::helper_machine::shared::*;
#[derive(Clone, Copy, PartialEq, Eq)]
enum Left {
    Constant,
    UIntData,
    TreeData,
    ZeroData,
}
#[derive(Clone, Copy)]
struct Case {
    op: BinaryOp,
    left: Left,
    wrapped: bool,
}
fn bounds() -> BoundsC {
    BoundsC {
        code_rows: 32,
        function_rows: 4,
        code_child_slots: 32,
        parameter_slots: 8,
        pattern_rows: 4,
        match_plan_rows: 4,
        match_arm_rows: 4,
        type_rows: 4,
        constructor_rows: 4,
        constructor_field_slots: 8,
        max_arity: 4,
        constant_nodes: 16,
        constant_field_slots: 16,
        data_nodes: 4,
        data_field_slots: 8,
        environment_cells: 8,
        frames: 8,
        helper_tasks: 4,
        serialization_bytes: 100000,
        max_reference_work: 10000,
        max_helper_work: 10000,
        step_cap: 10000,
    }
}
fn layout() -> LayoutSchemaC {
    LayoutSchemaC {
        layout_version: ("return-left".into(), "1".into()),
        field_id: None,
        widths: None,
        scalar_layout_id: None,
        bounds: bounds(),
        capacities: LayoutCapacitiesC {
            name_rows: 16,
            name_bytes: 256,
            occurrence_path_slots: 64,
            value_slots: 128,
        },
        ext_capacities: None,
        boundary_capacities_present: false,
    }
}
fn config() -> C6HostConfig {
    C6HostConfig::from_retained_layout(
        &layout(),
        C6CodecBudgets {
            type_expr_nodes: 128,
            template_bytes: 4096,
        },
    )
}
fn limits() -> C6HostLimits {
    C6HostLimits {
        max_total_lanes: 1 << 20,
        max_state_lanes: 1 << 20,
        max_metadata_bytes: 8192,
        max_string_bytes: 512,
        max_width_entries: 64,
    }
}
fn uint(n: u128) -> CoreExpr {
    CoreExpr::Literal {
        value: CoreValue::UInt {
            n: i128_class::KIntC::InRange(n),
        },
    }
}
fn tree(n: u128) -> CoreExpr {
    CoreExpr::Literal {
        value: CoreValue::Tree {
            tree: CoreTree::Leaf {
                n: i128_class::KIntC::InRange(n),
            },
        },
    }
}
fn binary(op: BinaryOp, left: CoreExpr, right: CoreExpr) -> CoreExpr {
    CoreExpr::Binary {
        op,
        left: Box::new(left),
        right: Box::new(right),
    }
}
fn program(c: Case) -> CoreProgram {
    let left = match c.left {
        Left::Constant => {
            if c.op == BinaryOp::Branch {
                tree(7)
            } else {
                uint(7)
            }
        }
        Left::UIntData => binary(BinaryOp::Add, uint(3), uint(4)),
        Left::TreeData => CoreExpr::Unary {
            op: UnaryOp::Leaf,
            operand: Box::new(uint(7)),
        },
        Left::ZeroData => binary(BinaryOp::Sub, uint(7), uint(7)),
    };
    let right = match c.op {
        BinaryOp::Branch => tree(5),
        BinaryOp::Cons => CoreExpr::Literal {
            value: CoreValue::List { elements: vec![] },
        },
        _ => uint(5),
    };
    let expr = binary(c.op, left, right);
    CoreProgram {
        functions: vec![],
        entry: if c.wrapped {
            CoreExpr::Let {
                name: "caller".into(),
                value: Box::new(uint(99)),
                body: Box::new(expr),
            }
        } else {
            expr
        },
    }
}
fn selected(h: &HostSnapshot, c: Case) -> bool {
    let ControlC::Return { value } = h.image.control else {
        return false;
    };
    let Some(FrameRowC::LeftF { op, .. }) = h.image.frames.last() else {
        return false;
    };
    *op == c.op
        && matches!(value, RefC::Data(_)) == (c.left != Left::Constant)
        && h.image.helper.is_none()
        && h.image.result.is_none()
        && h.image.outcome == OutcomeC::Running
        && h.image.remaining_work > 0
}
fn genuine(c: Case) -> CompleteStepRecord {
    let e = host_owner_engine();
    let a = e
        .admit(&program(c), &bounds(), &layout())
        .unwrap_or_else(|_| panic!("actual Core admission refused"));
    // Actual R3/R4/R5:128<=10000,8<=8,128<=128; no relaxed admission.
    let mut i = e
        .admit_run(
            &a,
            &RunIntC::Nat(128),
            &RunIntC::Nat(8),
            &FinanceInput::Absent,
        )
        .unwrap_or_else(|_| panic!("actual genesis refused"));
    for _ in 0..512 {
        let h = capture(&a, &i).unwrap_or_else(|_| panic!("actual capture refused"));
        if selected(&h, c) {
            return match record_step(&a, &mut i)
                .unwrap_or_else(|_| panic!("selected actual owner step refused"))
            {
                ObservedStep::Event(r) => r,
                _ => panic!("selected LeftF unexpectedly terminal"),
            };
        }
        match record_step(&a, &mut i).unwrap_or_else(|_| panic!("actual prefix step refused")) {
            ObservedStep::Event(_) => {}
            _ => panic!("terminal before selected ReturnLeft"),
        }
    }
    panic!("selected ReturnLeft not reached within512 real steps")
}
fn base() -> CompleteStepRecord {
    genuine(Case {
        op: BinaryOp::Add,
        left: Left::Constant,
        wrapped: true,
    })
}
fn expected(pre: &HostSnapshot) -> HostSnapshot {
    let mut post = pre.clone();
    let ControlC::Return { value } = pre.image.control else {
        panic!("Return expected");
    };
    let f = post.image.frames.last_mut().unwrap();
    let FrameRowC::LeftF {
        op,
        right,
        saved_env,
        origin,
    } = *f
    else {
        panic!("LeftF expected");
    };
    *f = FrameRowC::RightF {
        op,
        left: value,
        origin,
    };
    post.image.control = ControlC::Eval { expr: right };
    post.image.control_origin = right;
    post.image.env_root = saved_env;
    post.image.remaining_work = pre.image.remaining_work.checked_sub(1).unwrap();
    post.image.consumed_work = pre.image.consumed_work.checked_add(1).unwrap();
    post.image.reference_cursor = pre.image.reference_cursor.checked_add(1).unwrap();
    post
}
fn modes() -> [C6PushStatementBindingMode; 2] {
    [
        C6PushStatementBindingMode::LocalPublic,
        C6PushStatementBindingMode::LedgerCall,
    ]
}
fn pair(pre: &HostSnapshot, post: &HostSnapshot, en: bool) -> C6ReturnLeftStepWitness {
    witness_from_host_pair(en, pre, post, &config(), &limits()).unwrap()
}
fn circuit(w: &C6ReturnLeftStepWitness, m: C6PushStatementBindingMode) -> C6ReturnLeftStepCircuit {
    build_c6_return_left_step_circuit(
        w.source_params.clone(),
        w.host_prefix.meta.len(),
        w.host_prefix.prog.len(),
        w.host_prefix.meta_bounds_frames_offset,
        m,
        "return_left_test",
    )
    .unwrap()
}
fn reject(c: &C6ReturnLeftStepCircuit, w: &C6ReturnLeftStepWitness) {
    match check_c6_return_left_step_circuit(c, w) {
        Err(C6ReturnLeftConsumerError::Backend(s)) => {
            assert!(s.starts_with("SDK ProofPreimage::check failed:"))
        }
        _ => panic!("expected circuit preprocessing rejection, not producer refusal"),
    }
}
fn raw(c: &C6ReturnLeftStepCircuit, s: Vec<Fr>) -> Result<Vec<Option<usize>>, String> {
    let prefix = c.meta_len + c.prog_len + CTX_LANES;
    let post_start = 3 + 2 * prefix + c.geometry.state_total_lanes;
    let post = &s[post_start..];
    let p = match c.mode {
        C6PushStatementBindingMode::LocalPublic => ProofPreimage {
            inputs: vec![],
            private_transcript: vec![],
            public_transcript_inputs: s.clone(),
            public_transcript_outputs: s,
            binding_input: Fr::from(0u64),
            communications_commitment: None,
            key_location: KeyLocation("c6_left_raw_test".into()),
        },
        C6PushStatementBindingMode::LedgerCall => {
            let mut comm = s.clone();
            comm.extend_from_slice(post);
            let z = Fr::from(0u64);
            ProofPreimage {
                inputs: s.clone(),
                private_transcript: vec![],
                public_transcript_inputs: s,
                public_transcript_outputs: vec![],
                binding_input: z,
                communications_commitment: Some((transient_commit(&comm, z), z)),
                key_location: KeyLocation("c6_left_raw_test".into()),
            }
        }
    };
    p.check(&c.ir).map_err(|e| e.to_string())
}
#[test]
fn all_seven_binary_operators_rewrite_real_left_to_right_with_whole_state_events_both_modes() {
    for op in [
        BinaryOp::Add,
        BinaryOp::Sub,
        BinaryOp::Mul,
        BinaryOp::Eq,
        BinaryOp::Lt,
        BinaryOp::Cons,
        BinaryOp::Branch,
    ] {
        let r = genuine(Case {
            op,
            left: Left::Constant,
            wrapped: true,
        });
        assert_eq!(r.event.rule_id, RuleIdC::KMachineReturnLeft);
        assert_eq!(r.event.event_kind, EventKind::LegacyComplete);
        assert_eq!(r.event.site, Some(SiteTag::RetLeft));
        assert_eq!(r.event.disposition, StepDisposition::Applied);
        assert_eq!(r.event.result_tag, None);
        assert_eq!(
            (
                r.event.work_delta,
                r.event.helper_work_delta,
                r.event.helper_visits_delta,
                r.event.reference_cursor_delta
            ),
            (1, 0, 0, 1)
        );
        assert_eq!(expected(&r.before), r.after);
        assert_eq!(r.before.image.frames.len(), r.after.image.frames.len());
        assert!(r.before.image.frames.len() > 1);
        assert_eq!(r.after.image.env_root, Some(0));
        for m in modes() {
            let w = pair(&r.before, &r.after, true);
            assert!(check_c6_return_left_step_circuit(&circuit(&w, m), &w).is_ok());
        }
    }
}
#[test]
fn genuine_data_zero_tree_and_const_index_zero_are_full_refs_not_absence() {
    let cases = [
        Case {
            op: BinaryOp::Mul,
            left: Left::UIntData,
            wrapped: true,
        },
        Case {
            op: BinaryOp::Branch,
            left: Left::TreeData,
            wrapped: true,
        },
        Case {
            op: BinaryOp::Add,
            left: Left::ZeroData,
            wrapped: true,
        },
        Case {
            op: BinaryOp::Add,
            left: Left::Constant,
            wrapped: false,
        },
    ];
    for c in cases {
        let r = genuine(c);
        assert_eq!(expected(&r.before), r.after);
        let ControlC::Return { value } = r.before.image.control else {
            panic!("Return expected");
        };
        let Some(FrameRowC::RightF { left, .. }) = r.after.image.frames.last() else {
            panic!("RightF expected");
        };
        assert_eq!(*left, value);
        if c.left != Left::Constant {
            assert_eq!(value, RefC::Data(0));
        }
        if c.left == Left::ZeroData {
            assert_eq!(r.before.image.data[0], DataSnapshot::U128 { n: 0 });
        }
        if !c.wrapped {
            assert_eq!(value, RefC::Const(0));
            assert_eq!(r.after.image.env_root, None);
        }
        for m in modes() {
            let w = pair(&r.before, &r.after, true);
            assert!(check_c6_return_left_step_circuit(&circuit(&w, m), &w).is_ok());
        }
    }
}
#[test]
fn wrong_operator_left_ref_origin_top_padding_depth_and_lowerframe_reject() {
    let r = base();
    for m in modes() {
        let w = pair(&r.before, &r.after, true);
        let c = circuit(&w, m);
        assert!(check_c6_return_left_step_circuit(&c, &w).is_ok());
        let mut q = r.after.clone();
        match q.image.frames.last_mut().unwrap() {
            FrameRowC::RightF { op, .. } => *op = BinaryOp::Sub,
            _ => panic!("RightF expected"),
        };
        reject(&c, &pair(&r.before, &q, true));
        let mut q = r.after.clone();
        match q.image.frames.last_mut().unwrap() {
            FrameRowC::RightF { left, .. } => *left = RefC::Const(0),
            _ => panic!("RightF expected"),
        };
        assert_ne!(q.image.frames, r.after.image.frames);
        reject(&c, &pair(&r.before, &q, true));
        let mut q = r.after.clone();
        match q.image.frames.last_mut().unwrap() {
            FrameRowC::RightF { origin, .. } => *origin = if *origin == 0 { 1 } else { 0 },
            _ => panic!("RightF expected"),
        };
        reject(&c, &pair(&r.before, &q, true));
        let mut q = r.after.clone();
        q.image.frames.pop();
        reject(&c, &pair(&r.before, &q, true));
        let mut q = r.after.clone();
        match &mut q.image.frames[0] {
            FrameRowC::ScopeF { origin, .. } => *origin = if *origin == 0 { 1 } else { 0 },
            _ => panic!("lower ScopeF expected"),
        };
        reject(&c, &pair(&r.before, &q, true));
        let mut z = w.to_statement_frs();
        let image = 3 + c.meta_len + c.prog_len + CTX_LANES;
        let post_image = image + c.geometry.state_total_lanes + c.meta_len + c.prog_len + CTX_LANES;
        let top = post_image
            + IMAGE_SECTION_HEADER_LANES
            + (r.after.image.frames.len() - 1) * FRAME_LANES;
        z[top + 5] = Fr::from(1u64);
        assert!(raw(&c, z).is_err());
    }
}
#[test]
fn wrong_eval_target_control_padding_env_option_and_origin_reject() {
    let r = base();
    assert!(!r.before.image.env.is_empty());
    for m in modes() {
        let w = pair(&r.before, &r.after, true);
        let c = circuit(&w, m);
        assert!(check_c6_return_left_step_circuit(&c, &w).is_ok());
        let mut q = r.after.clone();
        q.image.env_root = None;
        reject(&c, &pair(&r.before, &q, true));
        let mut q = r.after.clone();
        let ControlC::Eval { expr } = q.image.control else {
            panic!("Eval expected");
        };
        q.image.control = ControlC::Eval {
            expr: if expr == 0 { 1 } else { 0 },
        };
        reject(&c, &pair(&r.before, &q, true));
        let mut q = r.after.clone();
        q.image.control = r.before.image.control;
        reject(&c, &pair(&r.before, &q, true));
        let mut q = r.after.clone();
        q.image.control_origin = if q.image.control_origin == 0 { 1 } else { 0 };
        reject(&c, &pair(&r.before, &q, true));
        let mut z = w.to_statement_frs();
        let image = 3 + c.meta_len + c.prog_len + CTX_LANES;
        let post_image = image + c.geometry.state_total_lanes + c.meta_len + c.prog_len + CTX_LANES;
        z[post_image + L_CONTROL + 2] = Fr::from(1u64);
        assert!(raw(&c, z).is_err());
    }
}
#[test]
fn raw_wrong_pre_site_stale_result_helper_tag_and_op_range_reject() {
    let r = base();
    let w = pair(&r.before, &r.after, true);
    for m in modes() {
        let c = circuit(&w, m);
        let s = w.to_statement_frs();
        assert!(raw(&c, s.clone()).is_ok());
        let image = 3 + c.meta_len + c.prog_len + CTX_LANES;
        let helper = image + c.geometry.image_total_lanes + c.geometry.data_total_lanes;
        let top =
            image + IMAGE_SECTION_HEADER_LANES + (r.before.image.frames.len() - 1) * FRAME_LANES;
        for (at, v) in [
            (helper + L_RESULT_RECORD, 13u128),
            (helper + L_HELPER_RECORD, 1),
            (image + L_CONTROL, 0),
            (image + L_CONTROL + 3, 1),
            (image + L_FRAME_COUNT, 0),
            (top, 2),
            (top + 1, 7),
            (top + 6, 1),
        ] {
            let mut z = s.clone();
            z[at] = Fr::from(v);
            assert!(raw(&c, z).is_err(), "selected-site mutation at {at}");
        }
    }
}
#[test]
fn clocks_preserve_helper_at_max_and_reject_fuel_zero_and_checked_wrap() {
    let r = base();
    let mut pre = r.before.clone();
    pre.image.helper_work = bounds().max_helper_work;
    // Bare typed clock carrier, not an admitted SI03 trace.
    let post = expected(&pre);
    for m in modes() {
        let w = pair(&pre, &post, true);
        let c = circuit(&w, m);
        assert!(check_c6_return_left_step_circuit(&c, &w).is_ok());
        for lane in [L_CONSUMED_WORK, L_REFERENCE_CURSOR] {
            let mut z = w.to_statement_frs();
            let image = 3 + c.meta_len + c.prog_len + CTX_LANES;
            let post_image =
                image + c.geometry.state_total_lanes + c.meta_len + c.prog_len + CTX_LANES;
            z[image + lane] = Fr::from(u128::MAX);
            z[post_image + lane] = Fr::from(0u64);
            assert!(raw(&c, z).is_err());
        }
        let mut q = post.clone();
        q.image.helper_work += 1;
        reject(&c, &pair(&pre, &q, true));
        let mut q = post.clone();
        q.image.helper_visits += 1;
        reject(&c, &pair(&pre, &q, true));
        let mut z = w.to_statement_frs();
        let image = 3 + c.meta_len + c.prog_len + CTX_LANES;
        z[image + L_REMAINING_WORK] = Fr::from(0u64);
        assert!(raw(&c, z).is_err());
    }
}
#[test]
fn disabled_identity_raw128_gate2_and_129bit_nonmatching_slice_ends() {
    let r = base();
    let w = pair(&r.before, &r.before, false);
    for m in modes() {
        let c = circuit(&w, m);
        let s = w.to_statement_frs();
        assert!(raw(&c, s.clone()).is_ok());
        let image = 3 + c.meta_len + c.prog_len + CTX_LANES;
        let post_image = image + c.geometry.state_total_lanes + c.meta_len + c.prog_len + CTX_LANES;
        let mut z = s.clone();
        z[0] = Fr::from(2u64);
        assert!(raw(&c, z).is_err());
        let mut z = s.clone();
        z[post_image + L_CONSUMED_WORK] = z[post_image + L_CONSUMED_WORK] + Fr::from(1u64);
        assert!(raw(&c, z).is_err());
        let high = Fr::from(u128::MAX) + Fr::from(1u64);
        let mut z = s.clone();
        z[image + L_CONSUMED_WORK] = high;
        z[post_image + L_CONSUMED_WORK] = high;
        assert!(raw(&c, z).is_err());
        let mut z = s.clone();
        for at in [
            IMAGE_SECTION_HEADER_LANES + 1,
            IMAGE_SECTION_HEADER_LANES + 2,
        ] {
            z[image + at] = Fr::from(u128::MAX);
            z[post_image + at] = Fr::from(u128::MAX);
        }
        assert!(
            raw(&c, z).is_ok(),
            "gate0 derivedslice2^129-2 must not be constrained to128bits"
        );
    }
}
#[test]
fn invariant_data_program_context_helper_and_schema_use_current_commitment() {
    let r = genuine(Case {
        op: BinaryOp::Mul,
        left: Left::UIntData,
        wrapped: true,
    });
    let w = pair(&r.before, &r.after, true);
    for m in modes() {
        let c = circuit(&w, m);
        let s = w.to_statement_frs();
        assert!(raw(&c, s.clone()).is_ok());
        let image = 3 + c.meta_len + c.prog_len + CTX_LANES;
        let post_base = image + c.geometry.state_total_lanes;
        for at in [
            1,
            2,
            3 + c.meta_len,
            3 + c.meta_len + c.prog_len,
            post_base + c.meta_len,
            post_base + c.meta_len + c.prog_len,
            post_base + c.meta_len + c.prog_len + CTX_LANES + c.geometry.image_total_lanes + 7,
            post_base
                + c.meta_len
                + c.prog_len
                + CTX_LANES
                + c.geometry.image_total_lanes
                + c.geometry.data_total_lanes
                + L_RESULT_RECORD,
        ] {
            let mut z = s.clone();
            z[at] = z[at] + Fr::from(1u64);
            assert!(raw(&c, z).is_err());
        }
    }
}
fn inputs(w: &C6ReturnLeftStepWitness) -> C6ReturnLeftStepInputs {
    let cv = |xs: &[u128]| {
        xs.iter()
            .map(|&v| Operand::Immediate(Fr::from(v)))
            .collect::<Vec<_>>()
    };
    C6ReturnLeftStepInputs {
        enable: Operand::Immediate(Fr::from(w.enable)),
        host_schema: Operand::Immediate(Fr::from(C6_HOST_SCHEMA)),
        carrier_version: Operand::Immediate(Fr::from(C6_PADDED_CARRIER_VERSION)),
        geometry: w.source_params.geometry(),
        source_params: w.source_params.clone(),
        meta_bounds_frames_offset: w.host_prefix.meta_bounds_frames_offset,
        pre_meta: cv(&w.host_prefix.meta),
        post_meta: cv(&w.host_prefix.meta),
        pre_prog: cv(&w.host_prefix.prog),
        post_prog: cv(&w.host_prefix.prog),
        pre_ctx: std::array::from_fn(|i| Operand::Immediate(Fr::from(w.host_prefix.ctx[i]))),
        post_ctx: std::array::from_fn(|i| Operand::Immediate(Fr::from(w.host_prefix.ctx[i]))),
        pre_image: cv(&w.pre_image),
        post_image: cv(&w.post_image),
        pre_data: cv(&w.pre_data),
        post_data: cv(&w.post_data),
        pre_helper: cv(&w.pre_helper),
        post_helper: cv(&w.post_helper),
    }
}
#[test]
fn fifteen_groups_all_five_prefixes_and_retained_serial_namespace_contract() {
    let r = base();
    let w = pair(&r.before, &r.after, true);
    for seg in ["local", "c6data", "c6helper", "c6meta", "c6progctx"] {
        for group in 0..15 {
            let mut i = inputs(&w);
            let op = Operand::Variable(Identifier(format!("%reserved.{seg}.temp.0")));
            match group {
                0 => i.enable = op,
                1 => i.host_schema = op,
                2 => i.carrier_version = op,
                3 => i.pre_meta[0] = op,
                4 => i.post_meta[0] = op,
                5 => i.pre_prog[0] = op,
                6 => i.post_prog[0] = op,
                7 => i.pre_ctx[0] = op,
                8 => i.post_ctx[0] = op,
                9 => i.pre_image[0] = op,
                10 => i.post_image[0] = op,
                11 => i.pre_data[0] = op,
                12 => i.post_data[0] = op,
                13 => i.pre_helper[0] = op,
                _ => i.post_helper[0] = op,
            }
            assert!(matches!(
                C6ReturnLeftStepEmitter::new("reserved").unwrap().emit(&i),
                Err(C6ReturnLeftStepError::ReservedOperand { .. })
            ));
        }
    }
    let i = inputs(&w);
    let mut em = C6ReturnLeftStepEmitter::new("serial").unwrap();
    let a = em.emit(&i).unwrap();
    let b = em.emit(&i).unwrap();
    let ids = |v: Vec<Instruction>| {
        v.into_iter()
            .filter_map(|ins| {
                serde_json::to_value(ins)
                    .unwrap()
                    .get("output")
                    .and_then(|x| x.as_str())
                    .map(str::to_owned)
            })
            .collect::<std::collections::BTreeSet<_>>()
    };
    let aa = ids(a.instructions);
    let bb = ids(b.instructions);
    assert!(!aa.is_empty() && !bb.is_empty());
    assert!(aa.is_disjoint(&bb));
}
