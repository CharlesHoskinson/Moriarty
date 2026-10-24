//! Scope return source tests: UNCOMPILED_UNRUN, preprocessing is not synthesis.
//! Genuine pairs are produced only by real Core admission/capture/record_step.
//! The oracle is public ScopeF row denotation, not emitter receipts or blind goldens.
use super::super::c6_return_scope_step::*;
use super::*;
use mori_core::sherlock::helper_machine::c6_phys::*;
use mori_core::sherlock::helper_machine::c6_phys_helper::{L_HELPER_RECORD, L_RESULT_RECORD};
use mori_core::sherlock::helper_machine::complete_observation::{
    CompleteStepRecord, ObservedStep, capture, record_step,
};
use mori_core::sherlock::helper_machine::shared::owner_host::host_owner_engine;
use mori_core::sherlock::helper_machine::shared::*;
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
        layout_version: ("return-scope".into(), "1".into()),
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
fn program(data: bool) -> CoreProgram {
    let body = if data {
        CoreExpr::Binary {
            op: BinaryOp::Add,
            left: Box::new(uint(3)),
            right: Box::new(uint(4)),
        }
    } else {
        uint(7)
    };
    CoreProgram {
        functions: vec![CoreFunction {
            name: "f".into(),
            parameters: vec![],
            return_type: SortTag::UInt,
            body,
        }],
        entry: CoreExpr::Let {
            name: "caller".into(),
            value: Box::new(uint(99)),
            body: Box::new(CoreExpr::Let {
                name: "inner".into(),
                value: Box::new(uint(42)),
                body: Box::new(CoreExpr::Call {
                    name: "f".into(),
                    args: vec![],
                }),
            }),
        },
    }
}
fn selected(h: &HostSnapshot) -> bool {
    matches!(h.image.control, ControlC::Return { .. })
        && h.image.helper.is_none()
        && h.image.result.is_none()
        && matches!(h.image.frames.last(), Some(FrameRowC::ScopeF { .. }))
        && h.image.outcome == OutcomeC::Running
        && h.image.remaining_work > 0
}
fn next_event(a: &Admitted, i: &mut MachineImage) -> CompleteStepRecord {
    match record_step(a, i).unwrap_or_else(|_| panic!("actual owner recorder refused")) {
        ObservedStep::Event(r) => r,
        _ => panic!("unexpected terminal before selected scope"),
    }
}
fn genuine(data: bool) -> [CompleteStepRecord; 2] {
    let e = host_owner_engine();
    let a = e
        .admit(&program(data), &bounds(), &layout())
        .unwrap_or_else(|_| panic!("actual admission refused"));
    // R3 128<=10000, R4 8<=8, R5 128<=128: source-grounded admission inputs.
    let mut i = e
        .admit_run(
            &a,
            &RunIntC::Nat(128),
            &RunIntC::Nat(8),
            &FinanceInput::Absent,
        )
        .unwrap_or_else(|_| panic!("actual genesis refused"));
    let mut previous = None;
    for _ in 0..512 {
        let h = capture(&a, &i).unwrap_or_else(|_| panic!("actual capture refused"));
        if selected(&h) {
            assert_eq!(previous, Some(RuleIdC::KMachineReturnCallValid));
            assert!(h.image.frames.len() > 1);
            let inner = next_event(&a, &mut i);
            let outer = next_event(&a, &mut i);
            assert_eq!(inner.event.rule_id, RuleIdC::KMachineReturnScope);
            assert_eq!(outer.event.rule_id, RuleIdC::KMachineReturnScope);
            assert_eq!(inner.after, outer.before);
            return [inner, outer];
        }
        previous = Some(next_event(&a, &mut i).event.rule_id);
    }
    panic!("scope after actual ReturnCallValid not reached within 512 steps")
}
fn expected(pre: &HostSnapshot) -> HostSnapshot {
    let mut post = pre.clone();
    let f = post.image.frames.pop().unwrap();
    match f {
        FrameRowC::ScopeF { saved_env, origin } => {
            post.image.env_root = saved_env;
            post.image.control_origin = origin;
        }
        _ => panic!("ScopeF expected"),
    }
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
fn pair(pre: &HostSnapshot, post: &HostSnapshot, en: bool) -> C6ReturnScopeStepWitness {
    witness_from_host_pair(en, pre, post, &config(), &limits()).unwrap()
}
fn circuit(
    w: &C6ReturnScopeStepWitness,
    m: C6PushStatementBindingMode,
) -> C6ReturnScopeStepCircuit {
    build_c6_return_scope_step_circuit(
        w.source_params.clone(),
        w.host_prefix.meta.len(),
        w.host_prefix.prog.len(),
        w.host_prefix.meta_bounds_frames_offset,
        m,
        "return_scope_test",
    )
    .unwrap()
}
fn reject(c: &C6ReturnScopeStepCircuit, w: &C6ReturnScopeStepWitness) {
    match check_c6_return_scope_step_circuit(c, w) {
        Err(C6ReturnScopeConsumerError::Backend(s)) => {
            assert!(s.starts_with("SDK ProofPreimage::check failed:"))
        }
        _ => panic!("expected circuit preprocessing rejection, not an early host refusal"),
    }
}
fn raw(c: &C6ReturnScopeStepCircuit, s: Vec<Fr>) -> Result<Vec<Option<usize>>, String> {
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
            key_location: KeyLocation("c6_scope_raw_test".into()),
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
                key_location: KeyLocation("c6_scope_raw_test".into()),
            }
        }
    };
    p.check(&c.ir).map_err(|e| e.to_string())
}
#[test]
fn actual_call_then_two_scope_returns_preserve_whole_const_and_data_pairs_both_modes() {
    for data in [false, true] {
        let records = genuine(data);
        for (which, r) in records.iter().enumerate() {
            assert_eq!(expected(&r.before), r.after);
            assert_eq!(r.event.event_kind, EventKind::LegacyComplete);
            assert_eq!(r.event.disposition, StepDisposition::Applied);
            assert_eq!(
                (
                    r.event.work_delta,
                    r.event.helper_work_delta,
                    r.event.helper_visits_delta,
                    r.event.reference_cursor_delta
                ),
                (1, 0, 0, 1)
            );
            assert_eq!(r.event.logical_depth_before, r.before.image.frames.len());
            assert_eq!(r.event.logical_depth_after, r.after.image.frames.len());
            assert!(r.before.image.helper.is_none() && r.before.image.result.is_none());
            assert_eq!(r.before.image.control, r.after.image.control);
            assert_eq!(
                r.after.image.env_root,
                if which == 0 { Some(0) } else { None }
            );
            match r.before.image.control {
                ControlC::Return { value } => assert_eq!(matches!(value, RefC::Data(_)), data),
                _ => panic!("Return expected"),
            }
            for m in modes() {
                let w = pair(&r.before, &r.after, true);
                assert!(check_c6_return_scope_step_circuit(&circuit(&w, m), &w).is_ok());
            }
        }
    }
}
#[test]
fn genuine_none_and_some_zero_restorations_reject_the_opposite_option() {
    for r in genuine(false) {
        for m in modes() {
            let w = pair(&r.before, &r.after, true);
            let c = circuit(&w, m);
            assert!(check_c6_return_scope_step_circuit(&c, &w).is_ok());
            let mut wrong = r.after.clone();
            wrong.image.env_root = if r.after.image.env_root.is_some() {
                None
            } else {
                Some(0)
            };
            reject(&c, &pair(&r.before, &wrong, true));
        }
    }
}
#[test]
fn wrong_pop_origin_lower_frame_and_return_ref_reach_constraints() {
    let [r, _] = genuine(false);
    for m in modes() {
        let w = pair(&r.before, &r.after, true);
        let c = circuit(&w, m);
        assert!(check_c6_return_scope_step_circuit(&c, &w).is_ok());
        let mut q = r.after.clone();
        q.image.frames = r.before.image.frames.clone();
        reject(&c, &pair(&r.before, &q, true));
        let mut q = r.after.clone();
        q.image.control_origin = if q.image.control_origin == 0 { 1 } else { 0 };
        reject(&c, &pair(&r.before, &q, true));
        let mut q = r.after.clone();
        q.image.control = ControlC::Return {
            value: RefC::Const(0),
        };
        if q.image.control == r.after.image.control {
            q.image.control = ControlC::Return {
                value: RefC::Const(1),
            };
        }
        reject(&c, &pair(&r.before, &q, true));
        let mut q = r.after.clone();
        match &mut q.image.frames[0] {
            FrameRowC::ScopeF { origin, .. } => *origin = if *origin == 0 { 1 } else { 0 },
            _ => panic!("outer ScopeF required"),
        }
        reject(&c, &pair(&r.before, &q, true));
    }
}
#[test]
fn enabled_site_absence_tags_padding_and_popped_row_are_constrained() {
    let [r, _] = genuine(false);
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
            (top, 7),
            (top + 4, 1),
        ] {
            let mut z = s.clone();
            z[at] = Fr::from(v);
            assert!(raw(&c, z).is_err(), "selected-site negative at {at}");
        }
        let post_image = image + c.geometry.state_total_lanes + c.meta_len + c.prog_len + CTX_LANES;
        let mut z = s.clone();
        z[post_image
            + IMAGE_SECTION_HEADER_LANES
            + (r.before.image.frames.len() - 1) * FRAME_LANES] = Fr::from(8u64);
        assert!(raw(&c, z).is_err(), "retired padded ScopeF must be zero");
    }
}
#[test]
fn exact_clocks_preserve_helper_at_max_and_reject_wrap_and_zero_fuel() {
    let [r, _] = genuine(false);
    let mut pre = r.before.clone();
    pre.image.helper_work = bounds().max_helper_work;
    // Diagnostic clock carrier, not claimed admitted SI03 state.
    let post = expected(&pre);
    for m in modes() {
        let w = pair(&pre, &post, true);
        let c = circuit(&w, m);
        assert!(check_c6_return_scope_step_circuit(&c, &w).is_ok());
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
fn disabled_is_identity_with_raw128_and_129bit_derived_slice_domain() {
    let [r, _] = genuine(false);
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
        // Nonmatching variant slice sums are emitted even with gate0: MAX+MAX
        // is 2^129-2, representable in the129 comparator and below Fr modulus.
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
            "gate0 must not impose a false128bit range on derived ends"
        );
    }
}
#[test]
fn immutable_data_program_context_and_schema_mutations_use_current_commitment() {
    let [r, _] = genuine(true);
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
        ] {
            let mut z = s.clone();
            z[at] = z[at] + Fr::from(1u64);
            assert!(raw(&c, z).is_err());
        }
    }
}
fn inputs(w: &C6ReturnScopeStepWitness) -> C6ReturnScopeStepInputs {
    let cv = |xs: &[u128]| {
        xs.iter()
            .map(|&v| Operand::Immediate(Fr::from(v)))
            .collect::<Vec<_>>()
    };
    C6ReturnScopeStepInputs {
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
fn all_fifteen_operand_groups_refuse_five_composed_reserved_prefixes() {
    let [r, _] = genuine(false);
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
                C6ReturnScopeStepEmitter::new("reserved").unwrap().emit(&i),
                Err(C6ReturnScopeStepError::ReservedOperand { .. })
            ));
        }
    }
}
#[test]
fn retained_scope_serial_outputs_are_disjoint_across_two_full_emits() {
    let [r, _] = genuine(false);
    let w = pair(&r.before, &r.after, true);
    let i = inputs(&w);
    let mut em = C6ReturnScopeStepEmitter::new("serial").unwrap();
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
