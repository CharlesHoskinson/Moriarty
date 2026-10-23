//! Source-only consumer regressions. ALL UNRUN; SDK preprocessing is not a proof.
//! Genuine positives are reached by one real admitted Core Eq execution. Full
//! expected snapshots are derived from the pinned Core family/driver contract,
//! never from the emitted instructions. Wrong posts/raw lanes reach the actual
//! SDK checker; no semantic-success host filter discards them. Raw mutations do
//! not claim owner provenance or whole StepInv validity.
use super::*;
use std::collections::BTreeSet;
use mori_core::sherlock::helper_machine::c6_phys::{
    C6CodecBudgets, L_HELPER_VISITS, L_REFERENCE_CURSOR, L_INITIAL_FUEL,
};
use mori_core::sherlock::helper_machine::complete_observation::{
    CompleteStepRecord, ExpressionSnapshot, ObservedStep, record_step,
};
use mori_core::sherlock::helper_machine::shared::owner_host::host_owner_engine;
use mori_core::sherlock::helper_machine::shared::*;

fn bounds(max_helper_work: u128) -> BoundsC {
    BoundsC {
        code_rows: 16, function_rows: 8, code_child_slots: 16, parameter_slots: 16,
        pattern_rows: 16, match_plan_rows: 16, match_arm_rows: 8, type_rows: 16,
        constructor_rows: 16, constructor_field_slots: 16, max_arity: 4,
        constant_nodes: 16, constant_field_slots: 16, data_nodes: 16,
        data_field_slots: 16, environment_cells: 16, frames: 16, helper_tasks: 4,
        serialization_bytes: 1_000_000, max_reference_work: 1_000_000,
        max_helper_work, step_cap: 1_000_000,
    }
}
fn layout(b: BoundsC) -> LayoutSchemaC {
    LayoutSchemaC {
        layout_version: ("lv".into(), "1".into()), field_id: None, widths: None,
        scalar_layout_id: None, bounds: b,
        capacities: LayoutCapacitiesC {
            name_rows: 32, name_bytes: 512, occurrence_path_slots: 64, value_slots: 64,
        },
        ext_capacities: None, boundary_capacities_present: false,
    }
}
fn cfg(b: BoundsC) -> C6HostConfig {
    C6HostConfig::from_retained_layout(&layout(b),
        C6CodecBudgets { type_expr_nodes: 256, template_bytes: 4096 })
}
fn limits() -> C6HostLimits {
    C6HostLimits { max_total_lanes: 1 << 20, max_state_lanes: 1 << 20,
        max_metadata_bytes: 8192, max_string_bytes: 512, max_width_entries: 64 }
}
fn genuine(max_hw: u128) -> (CompleteStepRecord, C6HostConfig) { genuine_run(max_hw, 64) }
fn genuine_run(max_hw: u128, fuel: u128) -> (CompleteStepRecord, C6HostConfig) {
    let b = bounds(max_hw);
    let literal = || CoreExpr::Literal {
        value: CoreValue::UInt { n: i128_class::KIntC::InRange(7) },
    };
    // Compute both sides so the actual Equal refs name live DATA rows, not
    // only constants; full DATA framing is therefore tested on real payloads.
    let computed = || CoreExpr::Binary {
        op: BinaryOp::Add, left: Box::new(literal()), right: Box::new(literal()),
    };
    let p = CoreProgram { functions: Vec::new(), entry: CoreExpr::Binary {
        op: BinaryOp::Eq, left: Box::new(computed()), right: Box::new(computed()),
    }};
    let engine = host_owner_engine();
    let a = engine.admit(&p, &b, &layout(b)).unwrap_or_else(|_| panic!("admit Core Eq"));
    let mut image = engine.admit_run(&a, &RunIntC::Nat(fuel), &RunIntC::Nat(16),
        &FinanceInput::Absent).unwrap_or_else(|error| panic!("admit Eq run: {error:?}"));
    for _ in 0..1024 {
        let step = record_step(&a, &mut image).unwrap_or_else(|_| panic!("owner Eq step"));
        match step {
            ObservedStep::Event(r) if r.event.rule_id == RuleIdC::BHelperEqualEmpty =>
                return (r, cfg(b)),
            ObservedStep::Event(r) => assert_eq!(r.after.image.outcome, OutcomeC::Running,
                "must reach Empty before a terminal or exhausted result"),
            ObservedStep::Terminal { .. } => panic!("terminal before EqualEmpty"),
        }
    }
    panic!("finite genuine Eq execution did not reach EqualEmpty")
}
fn expected_full_record(r: &CompleteStepRecord) {
    let pre = &r.before.image;
    let p3 = pre.helper_work == r.before.admission.bounds.max_helper_work;
    let mut expected = pre.clone();
    if p3 {
        expected.outcome = OutcomeC::Exhausted { reason: ExhaustReason::HelperWork };
    } else {
        expected.remaining_work = pre.remaining_work.checked_sub(1).expect("positive fuel");
        expected.consumed_work = pre.consumed_work.checked_add(1).expect("work");
        expected.helper_work = pre.helper_work.checked_add(1).expect("helper work");
        match &mut expected.helper {
            Some(HelperControlC::Equal { status, .. }) => {
                assert_eq!(*status, TriStatus::Pending); *status = TriStatus::Positive;
            },
            _ => panic!("selected Equal helper"),
        }
    }
    assert!(r.after.image == expected, "complete image including every nested table/frame/counter");
    assert!(r.after.admission == r.before.admission, "complete admission unchanged");
    assert_eq!(r.event.helper_kind, Some(HelperKind::Equal));
    assert_eq!(r.event.event_kind, EventKind::HelperVisit);
    assert_eq!((r.event.work_delta, r.event.helper_work_delta, r.event.helper_visits_delta,
        r.event.reference_cursor_delta), if p3 { (0, 0, 0, 0) } else { (1, 1, 0, 0) });
    assert_eq!(r.event.disposition, if p3 { StepDisposition::RefusedHelperWorkUncharged }
        else { StepDisposition::Applied });
    assert_eq!(r.event.helper_cursor_before, Some(0));
    assert_eq!(r.event.helper_cursor_after, Some(0));
    assert_eq!(r.event.helper_phase_before, Some(HelperPhaseSummaryC::Tri(TriStatus::Pending)));
    assert_eq!(r.event.helper_phase_after, Some(HelperPhaseSummaryC::Tri(if p3 {
        TriStatus::Pending } else { TriStatus::Positive })));
    assert!(r.event.result_tag.is_none());
    let common = match pre.helper {
        Some(HelperControlC::Equal { common, .. }) => common,
        _ => panic!("Equal common"),
    };
    assert_eq!(r.event.site, Some(common.site));
    assert_eq!(r.event.origin_reference_cursor, Some(common.origin_reference_cursor));
    assert_eq!(r.event.source_expr_id, pre.control_origin);
    let view = &r.before.admission.view;
    let root = view.entry;
    let right = match &view.expressions[root] {
        ExpressionSnapshot::Binary { op: BinaryOp::Eq, right, .. } => *right,
        _ => panic!("fixture entry must be the selected Eq"),
    };
    assert!(matches!(pre.frames.last(), Some(FrameRowC::RightF {
        op: BinaryOp::Eq, origin, ..
    }) if *origin == root), "Equal helper belongs to the main-root Eq frame");
    assert_eq!(pre.control_origin, right,
        "helper interval preserves the completed right expression origin");
    let occurrence = &view.occurrences[right];
    let path = &view.occurrence_paths[occurrence.path.start
        ..occurrence.path.start + occurrence.path.len];
    assert_eq!(r.event.source_occurrence, LocatorC::Occurrence {
        owner_is_function: occurrence.owner_is_function,
        function: occurrence.function, ast_path: path.to_vec(),
    }, "event occurrence is the admitted pre-control origin, not the enclosing frame");
    assert_eq!(r.event.step_id.ordinal, pre.consumed_work);
    assert_eq!(*r.event.step_id.run_id, pre.run_id);
    assert_eq!(r.event.alloc_deltas, AllocDeltasC {
        data_nodes: 0, data_fields: 0, environment_cells: 0,
    });
    assert_eq!(r.event.value_slot_writes, 0);
    assert_eq!(r.event.logical_depth_before, pre.frames.len());
    assert_eq!(r.event.logical_depth_after, pre.frames.len());
    assert_eq!(r.event.task_cursor_before, 0); assert_eq!(r.event.task_cursor_after, 0);
    assert_eq!(r.event.exhausted_kind, if p3 { Some(ExhaustReason::HelperWork) } else { None });
    assert_eq!(r.event.outcome_tag, if p3 { OutcomeTagC::Exhausted } else { OutcomeTagC::Running });
    assert_eq!(r.event.outcome_payload, expected.outcome);
    assert_eq!(r.event.boundary, if p3 { BoundaryKind::Terminal } else { BoundaryKind::None });

}
fn witness(r: &CompleteStepRecord, c: &C6HostConfig) -> C6EqualEmptyStepWitness {
    witness_from_host_pair(true, &r.before, &r.after, c, &limits()).expect("encode complete pair")
}
fn circuit(w: &C6EqualEmptyStepWitness, mode: C6PushStatementBindingMode) -> C6EqualEmptyStepCircuit {
    build_c6_equal_empty_step_circuit(w.source_params.clone(), w.host_prefix.meta.len(),
        w.host_prefix.prog.len(), w.host_prefix.meta_bounds_frames_offset, mode, "equal_empty_test")
        .expect("build actual IR consumer")
}
fn raw_check(c: &C6EqualEmptyStepCircuit, values: Vec<Fr>) -> bool {
    let prefix = c.meta_len + c.prog_len + CTX_LANES;
    let post_flat_start = 4 + prefix + c.geometry.state_total_lanes + prefix;
    assert_eq!(values.len(), c.statement_lane_count);
    let preimage = match c.mode {
        C6PushStatementBindingMode::LocalPublic => ProofPreimage {
            inputs: Vec::new(), private_transcript: Vec::new(),
            public_transcript_inputs: values.clone(), public_transcript_outputs: values,
            binding_input: Fr::from(0u64), communications_commitment: None,
            key_location: KeyLocation(CHECK_KEY_LOCATION.into()),
        },
        C6PushStatementBindingMode::LedgerCall => {
            let randomness = Fr::from(0u64);
            let mut comm = values.clone(); comm.extend_from_slice(&values[post_flat_start..]);
            ProofPreimage { inputs: values.clone(), private_transcript: Vec::new(),
                public_transcript_inputs: values, public_transcript_outputs: Vec::new(),
                binding_input: Fr::from(0u64),
                communications_commitment: Some((transient_commit(&comm, randomness), randomness)),
                key_location: KeyLocation(CHECK_KEY_LOCATION.into()),
            }
        },
    };
    // Fresh communication commitment from THIS mutant; no stale-commitment proxy.
    preimage.check(&c.ir).is_ok()
}
fn modes() -> [C6PushStatementBindingMode; 2] {
    [C6PushStatementBindingMode::LocalPublic, C6PushStatementBindingMode::LedgerCall]
}
#[test]
fn genuine_owner_applied_and_p3_preserve_whole_state_and_zero_visit_delta() {
    // Diagnostic-only telemetry. The original records, modes and assertions below
    // remain in order; no marker supplies a semantic expected value.
    let diagnostic_start = std::time::Instant::now();
    let diagnostic = |stage: &str, record: &str, mode: &str| {
        let memory = match std::fs::read_to_string("/proc/self/status") {
            Ok(status) => status.lines().filter(|line| {
                ["VmPeak:", "VmSize:", "VmHWM:", "VmRSS:", "VmData:", "Threads:"]
                    .iter().any(|key| line.starts_with(key))
            }).collect::<Vec<_>>().join(" | "),
            Err(error) => format!("proc_status_unavailable={error}"),
        };
        eprintln!("equal_empty_diag stage={stage} record={record} mode={mode} elapsed_us={} memory={memory}",
            diagnostic_start.elapsed().as_micros());
        let _ = std::io::Write::flush(&mut std::io::stderr());
    };
    diagnostic("test_begin", "none", "none");
    diagnostic("owner_before", "applied", "none");
    let (r, c) = genuine(1000);
    diagnostic("owner_after", "applied", "none");
    diagnostic("oracle_before", "applied", "none");
    expected_full_record(&r);
    diagnostic("oracle_after", "applied", "none");
    let cap = r.before.image.helper_work;
    diagnostic("owner_before", "p3", "none");
    let (p3, c3) = genuine(cap);
    diagnostic("owner_after", "p3", "none");
    diagnostic("oracle_before", "p3", "none");
    expected_full_record(&p3);
    assert_eq!(p3.event.disposition, StepDisposition::RefusedHelperWorkUncharged);
    diagnostic("oracle_after", "p3", "none");
    for (record_label, record, config) in [("applied", &r, &c), ("p3", &p3, &c3)] {
        diagnostic("witness_before", record_label, "none");
        let w = witness(record, config);
        diagnostic("witness_after", record_label, "none");
        eprintln!("equal_empty_diag_shape record={record_label} meta={} prog={} ctx={} pre_image={} pre_data={} pre_helper={} post_image={} post_data={} post_helper={}",
            w.host_prefix.meta.len(), w.host_prefix.prog.len(), CTX_LANES,
            w.pre_image.len(), w.pre_data.len(), w.pre_helper.len(),
            w.post_image.len(), w.post_data.len(), w.post_helper.len());
        let _ = std::io::Write::flush(&mut std::io::stderr());
        for mode in modes() {
            let mode_label = match mode {
                C6PushStatementBindingMode::LocalPublic => "local_public",
                C6PushStatementBindingMode::LedgerCall => "ledger_call",
            };
            diagnostic("build_before", record_label, mode_label);
            let ir = circuit(&w, mode);
            diagnostic("build_after", record_label, mode_label);
            diagnostic("typed_check_before", record_label, mode_label);
            assert!(check_c6_equal_empty_step_circuit(&ir, &w).is_ok());
            diagnostic("typed_check_after", record_label, mode_label);
            diagnostic("raw_check_before", record_label, mode_label);
            assert!(raw_check(&ir, w.to_statement_frs()));
            diagnostic("raw_check_after", record_label, mode_label);
        }
    }
    diagnostic("test_end", "none", "none");
}

#[test]
fn genuine_last_fuel_is_applied_running_not_earlier_p2() {
    let (base, _) = genuine(1000);
    let (r, c) = genuine_run(1000, base.before.image.consumed_work + 1);
    assert_eq!(r.before.image.remaining_work, 1);
    expected_full_record(&r); assert_eq!(r.after.image.remaining_work, 0);
    assert_eq!(r.after.image.outcome, OutcomeC::Running);
    let w = witness(&r, &c);
    for mode in modes() { assert!(raw_check(&circuit(&w, mode), w.to_statement_frs())); }
}
#[test]
fn disabled_identity_and_below_cap_boundary_are_constrained() {
    let (base, _) = genuine(1000);
    let (r, c) = genuine(base.before.image.helper_work + 1);
    expected_full_record(&r);
    assert_eq!(r.after.image.helper_work, r.before.admission.bounds.max_helper_work);
    let wd = witness_from_host_pair(false, &r.before, &r.before, &c, &limits()).unwrap();
    for mode in modes() {
        let ir = circuit(&wd, mode);
        assert!(raw_check(&ir, wd.to_statement_frs()));
        let mut bad = wd.to_statement_frs();
        let post = 4 + 2 * (ir.meta_len + ir.prog_len + CTX_LANES) + ir.geometry.state_total_lanes;
        bad[post + L_INITIAL_FUEL] = bad[post + L_INITIAL_FUEL] + Fr::from(1u64);
        assert!(!raw_check(&ir, bad), "disabled cannot change preserved state");
    }
}
#[test]
fn independent_wrong_post_status_charge_or_framing_reaches_sdk_rejection() {
    let (r, c) = genuine(1000); let w = witness(&r, &c);
    for mode in modes() {
        let ir = circuit(&w, mode);
        let prefix = ir.meta_len + ir.prog_len + CTX_LANES;
        let pre = 4 + prefix;
        let post_prefix = pre + ir.geometry.state_total_lanes;
        let post = post_prefix + prefix;
        let helper = post + ir.geometry.image_total_lanes + ir.geometry.data_total_lanes;
        let indices = [post + L_REMAINING_WORK, post + L_CONSUMED_WORK,
            post + L_HELPER_WORK, post + L_HELPER_VISITS, post + L_REFERENCE_CURSOR,
            post + L_CONTROL, post + L_CONTROL_ORIGIN, post + L_ENV_ROOT,
            post + L_ACTIVE, post + L_STACK_LIMIT, post + ir.geometry.frame_slot_offset(0),
            post + ir.geometry.image_total_lanes, // complete DATA header
            post + ir.geometry.image_total_lanes + 8, // live first U128 payload
            helper + L_HELPER_RECORD + 9, helper + L_HELPER_RECORD + 5,
            helper + L_RESULT_RECORD, helper + 3,
            post_prefix, post_prefix + ir.meta_len, post_prefix + ir.meta_len + ir.prog_len];
        assert!(raw_check(&ir, w.to_statement_frs()), "unmutated control");
        for at in indices {
            let mut bad = w.to_statement_frs(); bad[at] = bad[at] + Fr::from(1u64);
            assert!(!raw_check(&ir, bad), "each independent wrong post/prefix must reject");
        }
        let mut wrong = r.after.clone();
        if let Some(HelperControlC::Equal { status, .. }) = &mut wrong.image.helper {
            *status = TriStatus::Negative;
        }
        let bad = witness_from_host_pair(true, &r.before, &wrong, &c, &limits()).unwrap();
        assert!(check_c6_equal_empty_step_circuit(&ir, &bad).is_err(),
            "typed independent post was encoded, not rejected by semantic host filter");
    }
}
#[test]
fn p3_cannot_charge_change_status_or_lie_about_disposition() {
    let (base, _) = genuine(1000);
    let (r, c) = genuine(base.before.image.helper_work); let w = witness(&r, &c);
    for mode in modes() {
        let ir = circuit(&w, mode);
        let prefix = ir.meta_len + ir.prog_len + CTX_LANES;
        let post = 4 + 2 * prefix + ir.geometry.state_total_lanes;
        let helper = post + ir.geometry.image_total_lanes + ir.geometry.data_total_lanes;
        for at in [1, post + L_CONSUMED_WORK, post + L_HELPER_WORK,
            post + L_HELPER_VISITS, helper + L_HELPER_RECORD + 9] {
            let mut bad = w.to_statement_frs();
            if at == 1 { bad[at] = Fr::from(0u64); } else { bad[at] = bad[at] + Fr::from(1u64); }
            assert!(!raw_check(&ir, bad));
        }
    }
}
#[test]
fn selected_domain_ps_active_binding_and_raw_width_refusals() {
    let (r, c) = genuine(1000); let w = witness(&r, &c);
    for mode in modes() {
        let ir = circuit(&w, mode); let prefix = ir.meta_len + ir.prog_len + CTX_LANES;
        let pre = 4 + prefix; let post = 4 + 2 * prefix + ir.geometry.state_total_lanes;
        let ph = pre + ir.geometry.image_total_lanes + ir.geometry.data_total_lanes;
        let qh = post + ir.geometry.image_total_lanes + ir.geometry.data_total_lanes;
        for at in [ph + L_HELPER_RECORD, ph + L_HELPER_RECORD + 9,
            ph + 3, ph + L_RESULT_RECORD, pre + L_REMAINING_WORK] {
            let mut bad = w.to_statement_frs();
            bad[at] = if at == pre + L_REMAINING_WORK { Fr::from(0u64) }
                else { bad[at] + Fr::from(1u64) };
            assert!(!raw_check(&ir, bad));
        }
        // Some(parent_resume) is structurally encodable, but PS rejects it even at P3.
        let mut bad = w.to_statement_frs(); bad[ph + L_HELPER_RECORD + 3] = Fr::from(1u64);
        bad[qh + L_HELPER_RECORD + 3] = Fr::from(1u64);
        assert!(!raw_check(&ir, bad));
        // Preserve a forged active counter in BOTH images: still fails DATA-count binding.
        let mut bad = w.to_statement_frs(); bad[pre + L_ACTIVE] = bad[pre + L_ACTIVE] + Fr::from(1u64);
        bad[post + L_ACTIVE] = bad[post + L_ACTIVE] + Fr::from(1u64); assert!(!raw_check(&ir, bad));
        for at in [0, 1] {
            let mut bad = w.to_statement_frs(); bad[at] = Fr::from(2u64);
            assert!(!raw_check(&ir, bad));
        }
        let mut bad = w.to_statement_frs();
        bad[pre + L_INITIAL_FUEL] = Fr::from(u128::MAX) + Fr::from(1u64);
        bad[post + L_INITIAL_FUEL] = bad[pre + L_INITIAL_FUEL];
        assert!(!raw_check(&ir, bad), "raw field2^128 is not a UInt128 lane");
        let mut bad = w.to_statement_frs(); bad[pre + L_CONSUMED_WORK] = Fr::from(u128::MAX);
        bad[post + L_CONSUMED_WORK] = Fr::from(0u64);
        assert!(!raw_check(&ir, bad), "applied consumed overflow cannot wrap");
    }
}
fn inputs(w: &C6EqualEmptyStepWitness) -> C6EqualEmptyStepInputs {
    let ops = |v: &[u128]| v.iter().map(|&n| Operand::Immediate(Fr::from(n))).collect::<Vec<_>>();
    let ctx: [Operand; CTX_LANES] = ops(&w.host_prefix.ctx).try_into().unwrap();
    C6EqualEmptyStepInputs {
        enable: Operand::Immediate(Fr::from(w.enable)),
        disposition_is_refused_helper_work: Operand::Immediate(Fr::from(w.disposition_is_refused_helper_work)),
        host_schema: Operand::Immediate(Fr::from(C6_HOST_SCHEMA)),
        carrier_version: Operand::Immediate(Fr::from(C6_PADDED_CARRIER_VERSION)),
        geometry: w.source_params.geometry(), meta_bounds_frames_offset: w.host_prefix.meta_bounds_frames_offset,
        source_params: w.source_params.clone(), pre_meta: ops(&w.host_prefix.meta),
        pre_prog: ops(&w.host_prefix.prog), pre_ctx: ctx.clone(), pre_image: ops(&w.pre_image),
        pre_data: ops(&w.pre_data), pre_helper: ops(&w.pre_helper),
        post_meta: ops(&w.host_prefix.meta), post_prog: ops(&w.host_prefix.prog), post_ctx: ctx,
        post_image: ops(&w.post_image), post_data: ops(&w.post_data), post_helper: ops(&w.post_helper),
    }
}
fn destinations(instructions: &[Instruction]) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for i in instructions {
        match i {
            Instruction::Add { output, .. } | Instruction::Mul { output, .. }
            | Instruction::Neg { output, .. } | Instruction::Not { output, .. }
            | Instruction::CondSelect { output, .. } | Instruction::TestEq { output, .. }
            | Instruction::LessThan { output, .. } | Instruction::Copy { output, .. } =>
                { out.insert(output.0.clone()); },
            Instruction::DivModPowerOfTwo { outputs, .. } =>
                { for o in outputs { out.insert(o.0.clone()); } },
            _ => {},
        }
    }
    out
}
#[test]
fn retained_emitter_namespaces_do_not_alias_or_accept_reserved_inputs() {
    let (r, c) = genuine(1000); let w = witness(&r, &c); let input = inputs(&w);
    let mut em = C6EqualEmptyStepEmitter::new("ee").unwrap();
    let a = em.emit(&input).unwrap(); let serial = em.next_temp();
    let b = em.emit(&input).unwrap(); assert!(em.next_temp() > serial);
    let ad = destinations(&a.instructions); let bd = destinations(&b.instructions);
    assert!(!ad.is_empty() && ad.is_disjoint(&bd));
    for segment in COMPOSED_RESERVED_SEGMENTS {
        let mut bad = input.clone();
        bad.pre_helper[0] = Operand::Variable(Identifier(format!("%ee{segment}0")));
        assert!(matches!(em.emit(&bad), Err(C6EqualEmptyStepError::ReservedOperand { .. })));
    }
    assert!(matches!(C6EqualEmptyStepEmitter::new(" "), Err(C6EqualEmptyStepError::EmptyScope)));
    assert!(matches!(C6EqualEmptyStepEmitter::new("ee.local.temp.x"), Err(C6EqualEmptyStepError::InvalidScope)));
}
