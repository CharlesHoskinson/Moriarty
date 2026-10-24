//! Raw-domain / composition tests for `c6_scalar_tag_consumer`.
//! Source-only and UNRUN. Every base witness comes from the genuine
//! owner (`host_owner_engine`, `admit_ext_program`, one actual
//! `record_step`); mutations are applied to the raw `Vec<Fr>`
//! statement only and reach real SDK `ProofPreimage::check`
//! preprocessing in both statement modes. LedgerCall commitments are
//! recomputed from the mutated statement plus its mutated post-flat
//! output. Raw rows are RAW DOMAIN, never owner-admitted states.
//! REQUIRED_OPEN: SI08/SI17/SI21/SI22, StepInv, program membership,
//! nominal induction/checked-sort bridge theorem, P1/native/full-39.

use std::collections::BTreeSet;

use midnight_transient_crypto::curve::Fr;
use midnight_transient_crypto::hash::transient_commit;
use midnight_transient_crypto::proofs::{KeyLocation, ProofPreimage};
use midnight_zkir_v3::ir::Operand;
use midnight_zkir_v3::{Identifier, Instruction};

use mori_core::sherlock::helper_machine::c6_phys::{
    C6CodecBudgets, L_CONSUMED_WORK, L_CONTROL, L_HELPER_VISITS, L_HELPER_WORK, L_OUTCOME,
    L_REFERENCE_CURSOR, L_REMAINING_WORK,
};
use mori_core::sherlock::helper_machine::c6_phys_data::{ENV_ROW_LANES, REF_LANES};
use mori_core::sherlock::helper_machine::c6_phys_helper::L_HELPER_RECORD;
use mori_core::sherlock::helper_machine::c6_phys_host::{
    C6_HOST_SCHEMA, C6HostConfig, C6HostLimits, CTX_LANES,
};
use mori_core::sherlock::helper_machine::c6_phys_program::{decode_program, encode_program};
use mori_core::sherlock::helper_machine::complete_observation::{
    CaptureStepError, CompleteStepRecord, ConstantSnapshot, HostSnapshot, ObservedStep,
    record_step as owner_record,
};
use mori_core::sherlock::helper_machine::shared::owner_ext_source_program::{
    ExtDomain, ExtProgram,
};
use mori_core::sherlock::helper_machine::shared::owner_ext_t3_uses::{
    ExtExpr, ExtFunction, ExtValue,
};
use mori_core::sherlock::helper_machine::shared::owner_ext_type_builder::{
    CtorDeclSrc, ExtTypeBuildError, TypeDeclSrc, TypeExprSrc,
};
use mori_core::sherlock::helper_machine::shared::owner_host::host_owner_engine;
use mori_core::sherlock::helper_machine::shared::*;

use super::super::c6_push_step::{C6_PADDED_CARRIER_VERSION, C6PushStatementBindingMode};
use super::super::c6_scalar_tag_step::{
    C6ScalarTagStepEmitter, C6ScalarTagStepError, C6ScalarTagStepInputs,
};
use super::{
    C6ScalarTagConsumerError, C6ScalarTagStepCircuit, C6ScalarTagStepWitness, CHECK_KEY_LOCATION,
    build_c6_scalar_tag_step_circuit, check_c6_scalar_tag_step_circuit, witness_from_owner_record,
};

// ---------- fixture admission helpers (genuine owner only) ----------

fn bounds(max_helper_work: u128) -> BoundsC {
    BoundsC {
        code_rows: 64,
        function_rows: 8,
        code_child_slots: 64,
        parameter_slots: 16,
        pattern_rows: 16,
        match_plan_rows: 16,
        match_arm_rows: 16,
        type_rows: 16,
        constructor_rows: 16,
        constructor_field_slots: 16,
        max_arity: 4,
        constant_nodes: 64,
        constant_field_slots: 64,
        data_nodes: 16,
        data_field_slots: 16,
        environment_cells: 16,
        frames: 16,
        helper_tasks: 4,
        serialization_bytes: 1_000_000,
        max_reference_work: 1_000_000,
        max_helper_work,
        step_cap: 1_000_000,
    }
}

fn capacities() -> LayoutCapacitiesC {
    LayoutCapacitiesC {
        name_rows: 32,
        name_bytes: 512,
        occurrence_path_slots: 64,
        value_slots: 512,
    }
}

fn layout(b: BoundsC, ext: Option<ExtCapacitiesC>) -> LayoutSchemaC {
    LayoutSchemaC {
        layout_version: ("lv".into(), "1".into()),
        field_id: None,
        widths: None,
        scalar_layout_id: None,
        bounds: b,
        capacities: capacities(),
        ext_capacities: ext,
        boundary_capacities_present: false,
    }
}

fn cfg_of(b: BoundsC, ext: Option<ExtCapacitiesC>) -> C6HostConfig {
    C6HostConfig::from_retained_layout(
        &layout(b, ext),
        C6CodecBudgets {
            type_expr_nodes: 256,
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

fn violation_label(v: &HostViolation) -> String {
    format!(
        "kind={:?}, clause={:?}, rule={:?}, locator={}",
        v.kind,
        v.clause,
        v.rule,
        v.locator.as_deref().unwrap_or("<none>")
    )
}

fn capture_label(e: &CaptureStepError) -> String {
    match e {
        CaptureStepError::BeforeObservation(v) => {
            format!("before observation: {}", violation_label(v))
        }
        CaptureStepError::Step {
            violation, after, ..
        } => match after {
            Ok(_) => format!(
                "owner step: {}; after snapshot captured",
                violation_label(violation)
            ),
            Err(after_violation) => format!(
                "owner step: {}; after observation: {}",
                violation_label(violation),
                violation_label(after_violation)
            ),
        },
        CaptureStepError::AfterObservation {
            violation, event, ..
        } => format!(
            "after observation: {}; event_present={}",
            violation_label(violation),
            event.is_some()
        ),
    }
}

fn tooling_label(e: &Tooling) -> String {
    match e {
        Tooling::AdmissionRejected {
            reason,
            detail,
            locator,
        } => format!("admission rejected: reason={reason:?}, detail={detail}, locator={locator:?}"),
        Tooling::CapacityNotAdmitted {
            kind,
            needed,
            limit,
            locator,
        } => format!(
            "capacity not admitted: kind={kind:?}, needed={needed:?}, limit={limit}, locator={locator:?}"
        ),
        Tooling::UnsupportedFeature { feature, locator } => {
            format!("unsupported feature: feature={feature:?}, locator={locator:?}")
        }
    }
}

fn ext_label(e: &ExtTypeBuildError) -> String {
    match e {
        ExtTypeBuildError::CoreTooling(t) => tooling_label(t),
        ExtTypeBuildError::ExtCapacityNotAdmitted {
            kind,
            needed,
            limit,
            locator,
        } => format!(
            "ext capacity: kind={kind:?}, needed={needed:?}, limit={limit}, locator={locator:?}"
        ),
        ExtTypeBuildError::ExtAdmissionRejected {
            reason,
            detail,
            locator,
        } => format!(
            "ext admission rejected: reason={reason:?}, detail={detail}, locator={locator:?}"
        ),
        ExtTypeBuildError::ExtUnsupportedFeature { feature, locator } => {
            format!("ext unsupported feature: feature={feature:?}, locator={locator:?}")
        }
        ExtTypeBuildError::Host(v) => format!("ext host violation: {}", violation_label(v)),
    }
}

fn is_selected(s: &HostSnapshot, mode: ScalarMode) -> bool {
    match s.image.helper {
        Some(HelperControlC::Scalar {
            mode: m,
            phase: ScalarPhase::First,
            reg_a: 0,
            staged: ScalarStaged::None,
            ..
        }) => m == mode,
        _ => false,
    }
}

fn finish_selected(
    admitted: &Admitted,
    image: &mut MachineImage,
    cfg: &C6HostConfig,
    label: &str,
) -> (C6ScalarTagStepWitness, CompleteStepRecord) {
    let record = match owner_record(admitted, image) {
        Ok(ObservedStep::Event(r)) => r,
        Ok(ObservedStep::Terminal { .. }) => {
            panic!("{label}: selected step must be an event")
        }
        Err(e) => panic!("{label}: selected step failed: {}", capture_label(&e)),
    };
    let w = witness_from_owner_record(&record, cfg, &limits())
        .unwrap_or_else(|e| panic!("{label}: owner-record witness: {e}"));
    (w, record)
}

fn search_finish(
    admitted: Admitted,
    mut image: MachineImage,
    cfg: &C6HostConfig,
    mode: ScalarMode,
    label: &str,
) -> (C6ScalarTagStepWitness, CompleteStepRecord) {
    for _ in 0..512 {
        let step = owner_record(&admitted, &mut image)
            .unwrap_or_else(|e| panic!("{label}: search step: {}", capture_label(&e)));
        let after = match &step {
            ObservedStep::Event(r) => &r.after,
            ObservedStep::Terminal { after, .. } => {
                panic!(
                    "{label}: terminal before selected state: {:?}",
                    after.image.outcome
                )
            }
        };
        if is_selected(after, mode) {
            return finish_selected(&admitted, &mut image, cfg, label);
        }
    }
    panic!("{label}: genuine owner never reached Scalar First {mode:?}")
}

fn lit_u(n: u128) -> CoreExpr {
    CoreExpr::Literal {
        value: CoreValue::UInt {
            n: i128_class::KIntC::InRange(n),
        },
    }
}

fn reach_core(
    b: BoundsC,
    mode: ScalarMode,
    label: &str,
    program: CoreProgram,
) -> (C6ScalarTagStepWitness, CompleteStepRecord) {
    let engine = host_owner_engine();
    let admitted = engine
        .admit(&program, &b, &layout(b, None))
        .unwrap_or_else(|e| panic!("{label}: admit: {}", tooling_label(&e)));
    let image = engine
        .admit_run(
            &admitted,
            &RunIntC::Nat(64),
            &RunIntC::Nat(8),
            &FinanceInput::Absent,
        )
        .unwrap_or_else(|e| panic!("{label}: genesis: {}", tooling_label(&e)));
    search_finish(admitted, image, &cfg_of(b, None), mode, label)
}

fn nominal_program(return_alpha: bool) -> ExtProgram {
    let (ctor, ty) = if return_alpha {
        ("MkAlpha", "Alpha")
    } else {
        ("MkBeta", "Beta")
    };
    ExtProgram {
        types: vec![
            TypeDeclSrc {
                name: "Alpha".into(),
                parameters: Vec::new(),
                ctors: vec![CtorDeclSrc {
                    name: "MkAlpha".into(),
                    fields: Vec::new(),
                }],
            },
            TypeDeclSrc {
                name: "Beta".into(),
                parameters: Vec::new(),
                ctors: vec![CtorDeclSrc {
                    name: "MkBeta".into(),
                    fields: Vec::new(),
                }],
            },
        ],
        assets: Vec::new(),
        states: Vec::new(),
        inputs: Vec::new(),
        functions: vec![ExtFunction {
            name: "f".into(),
            parameters: Vec::new(),
            return_type: TypeExprSrc::Named {
                name: "Alpha".into(),
            },
            body: ExtExpr::Literal {
                value: ExtValue::Constructor {
                    ty: TypeExprSrc::Named { name: ty.into() },
                    ctor: ctor.into(),
                    fields: Vec::new(),
                },
            },
        }],
        entry: ExtExpr::Call {
            name: "f".into(),
            args: Vec::new(),
        },
    }
}

fn reach_ext(
    b: BoundsC,
    mode: ScalarMode,
    label: &str,
    program: ExtProgram,
) -> (C6ScalarTagStepWitness, CompleteStepRecord) {
    let ext = Some(ExtCapacitiesC::all_zero());
    let (admitted, _report) = admit_ext_program(
        program,
        ExtDomain::GENERIC_ABSENT,
        &b,
        &layout(b, ext),
        None,
    )
    .unwrap_or_else(|e| panic!("{label}: admit_ext_program: {}", ext_label(&e)));
    let engine = host_owner_engine();
    let image = engine
        .admit_run(
            &admitted,
            &RunIntC::Nat(64),
            &RunIntC::Nat(8),
            &FinanceInput::Absent,
        )
        .unwrap_or_else(|e| panic!("{label}: ext genesis: {}", tooling_label(&e)));
    search_finish(admitted, image, &cfg_of(b, ext), mode, label)
}

fn add_witness() -> (C6ScalarTagStepWitness, CompleteStepRecord) {
    reach_core(
        bounds(100),
        ScalarMode::Add,
        "raw Add Const UInt",
        CoreProgram {
            functions: Vec::new(),
            entry: CoreExpr::Binary {
                op: BinaryOp::Add,
                left: Box::new(lit_u(40)),
                right: Box::new(lit_u(2)),
            },
        },
    )
}

fn testtype_uint_witness() -> (C6ScalarTagStepWitness, CompleteStepRecord) {
    reach_core(
        bounds(100),
        ScalarMode::TestType,
        "raw TestType UInt",
        CoreProgram {
            functions: vec![CoreFunction {
                name: "f".into(),
                parameters: Vec::new(),
                return_type: SortTag::UInt,
                body: lit_u(0),
            }],
            entry: CoreExpr::Call {
                name: "f".into(),
                args: Vec::new(),
            },
        },
    )
}

fn nominal_witness() -> (C6ScalarTagStepWitness, CompleteStepRecord) {
    reach_ext(
        bounds(100),
        ScalarMode::TestType,
        "raw TestType nominal Alpha",
        nominal_program(true),
    )
}

fn kind_only_ctor_witness() -> (C6ScalarTagStepWitness, CompleteStepRecord) {
    let program = ExtProgram {
        types: nominal_program(true).types,
        assets: Vec::new(),
        states: Vec::new(),
        inputs: Vec::new(),
        functions: Vec::new(),
        entry: ExtExpr::Binary {
            op: BinaryOp::Add,
            left: Box::new(ExtExpr::Literal {
                value: ExtValue::Constructor {
                    ty: TypeExprSrc::Named {
                        name: "Alpha".into(),
                    },
                    ctor: "MkAlpha".into(),
                    fields: Vec::new(),
                },
            }),
            right: Box::new(ExtExpr::Literal {
                value: ExtValue::Core {
                    value: CoreValue::UInt {
                        n: i128_class::KIntC::InRange(1),
                    },
                },
            }),
        },
    };
    reach_ext(
        bounds(100),
        ScalarMode::Add,
        "raw Add Constructor kind-only",
        program,
    )
}

fn live_call_witness(second: u128, label: &str) -> (C6ScalarTagStepWitness, CompleteStepRecord) {
    reach_core(
        bounds(100),
        ScalarMode::TestType,
        label,
        CoreProgram {
            functions: vec![CoreFunction {
                name: "f".into(),
                parameters: vec![
                    CoreParameter {
                        name: "a".into(),
                        sort: SortTag::UInt,
                    },
                    CoreParameter {
                        name: "b".into(),
                        sort: SortTag::UInt,
                    },
                ],
                return_type: SortTag::UInt,
                body: CoreExpr::Var { name: "a".into() },
            }],
            entry: CoreExpr::Call {
                name: "f".into(),
                args: vec![lit_u(7), lit_u(second)],
            },
        },
    )
}

fn selected_operand_a(record: &CompleteStepRecord) -> RefC {
    match record.before.image.helper {
        Some(HelperControlC::Scalar {
            mode: ScalarMode::TestType,
            phase: ScalarPhase::First,
            operand_a,
            expected: Some(_),
            ..
        }) => operand_a,
        _ => panic!("live fixture must select Scalar TestType First"),
    }
}

fn assert_live_premises(w: &C6ScalarTagStepWitness, record: &CompleteStepRecord, label: &str) {
    assert_eq!(
        record.event.rule_id,
        RuleIdC::BHelperScalarTag,
        "{label}: must reach a real BHelperScalarTag event"
    );
    assert!(
        !record.before.image.env.is_empty(),
        "{label}: chosen First state must carry a nonempty bound env"
    );
    // Physical retained prefix, not logical argument count: genesis
    // is empty; Append #1 (src len 0) pushes only its Tail (len 1);
    // Append #2 (src len 1) pushes one AppendCopy of the retained
    // completed slice plus its Tail (len 1 -> 3). The logical
    // two-argument slice is the latest AppendDone seq
    // (dest_start 1, len 2) carried into Invoke/Bind; it does not
    // truncate value_slots. Hence exactly 3 physical rows, whose
    // final two rows are the logical arguments.
    assert_eq!(
        record.before.image.value_slots.len(),
        3,
        "{label}: chosen First state must carry the retained 3-row physical value-slot prefix for two logical arguments"
    );
    assert_eq!(
        record.after.image.env, record.before.image.env,
        "{label}: genuine Tag step preserves env"
    );
    assert_eq!(
        record.after.image.value_slots, record.before.image.value_slots,
        "{label}: genuine Tag step preserves value slots"
    );
    assert_ok_both(w, label);
}

// ---------- raw statement harness (both modes, recomputed CC) ----------

fn circuit_for(
    w: &C6ScalarTagStepWitness,
    mode: C6PushStatementBindingMode,
) -> C6ScalarTagStepCircuit {
    build_c6_scalar_tag_step_circuit(
        w.source_params().clone(),
        w.host_prefix().meta.len(),
        w.host_prefix().prog.len(),
        w.host_prefix().meta_bounds_frames_offset,
        mode,
        "scalar_tag_raw_test",
    )
    .expect("circuit builds")
}

fn assert_ok_both(w: &C6ScalarTagStepWitness, label: &str) {
    for mode in [
        C6PushStatementBindingMode::LocalPublic,
        C6PushStatementBindingMode::LedgerCall,
    ] {
        let c = circuit_for(w, mode);
        assert!(
            check_c6_scalar_tag_step_circuit(&c, w).is_ok(),
            "{label}: typed positive control must pass in {mode:?}"
        );
    }
}

fn two_pow_128() -> Fr {
    Fr::from(u128::MAX) + Fr::from(1u64)
}

fn prefix_len(w: &C6ScalarTagStepWitness) -> usize {
    w.host_prefix().meta.len() + w.host_prefix().prog.len() + CTX_LANES
}

fn state_len(w: &C6ScalarTagStepWitness) -> usize {
    w.pre_image().len() + w.pre_data().len() + w.pre_helper().len()
}

fn pre_state_start(w: &C6ScalarTagStepWitness) -> usize {
    4 + prefix_len(w)
}

fn post_prefix_start(w: &C6ScalarTagStepWitness) -> usize {
    pre_state_start(w) + state_len(w)
}

fn post_state_start(w: &C6ScalarTagStepWitness) -> usize {
    post_prefix_start(w) + prefix_len(w)
}

fn pre_prog_at(w: &C6ScalarTagStepWitness, i: usize) -> usize {
    4 + w.host_prefix().meta.len() + i
}

fn post_prog_at(w: &C6ScalarTagStepWitness, i: usize) -> usize {
    post_prefix_start(w) + w.host_prefix().meta.len() + i
}

fn pre_ctx_at(w: &C6ScalarTagStepWitness, i: usize) -> usize {
    4 + w.host_prefix().meta.len() + w.host_prefix().prog.len() + i
}

fn post_ctx_at(w: &C6ScalarTagStepWitness, i: usize) -> usize {
    post_prefix_start(w) + w.host_prefix().meta.len() + w.host_prefix().prog.len() + i
}

fn pre_image_at(w: &C6ScalarTagStepWitness, lane: usize) -> usize {
    pre_state_start(w) + lane
}

fn post_image_at(w: &C6ScalarTagStepWitness, lane: usize) -> usize {
    post_state_start(w) + lane
}

fn post_data_at(w: &C6ScalarTagStepWitness, lane: usize) -> usize {
    post_state_start(w) + w.pre_image().len() + lane
}

fn pre_helper_at(w: &C6ScalarTagStepWitness, rel: usize) -> usize {
    pre_state_start(w) + w.pre_image().len() + w.pre_data().len() + L_HELPER_RECORD + rel
}

fn post_helper_at(w: &C6ScalarTagStepWitness, rel: usize) -> usize {
    post_state_start(w) + w.pre_image().len() + w.pre_data().len() + L_HELPER_RECORD + rel
}

fn check_raw(
    c: &C6ScalarTagStepCircuit,
    w: &C6ScalarTagStepWitness,
    statement: Vec<Fr>,
    comm_override: Option<Fr>,
) -> Result<Vec<Option<usize>>, C6ScalarTagConsumerError> {
    let preimage = match c.mode {
        C6PushStatementBindingMode::LocalPublic => ProofPreimage {
            inputs: Vec::new(),
            private_transcript: Vec::new(),
            public_transcript_inputs: statement.clone(),
            public_transcript_outputs: statement,
            binding_input: Fr::from(0u64),
            communications_commitment: None,
            key_location: KeyLocation(CHECK_KEY_LOCATION.into()),
        },
        C6PushStatementBindingMode::LedgerCall => {
            let randomness = Fr::from(0u64);
            let post = statement[post_state_start(w)..].to_vec();
            let mut comm_inputs = statement.clone();
            comm_inputs.extend(post.iter().copied());
            let comm = comm_override.unwrap_or_else(|| transient_commit(&comm_inputs, randomness));
            ProofPreimage {
                inputs: statement.clone(),
                private_transcript: Vec::new(),
                public_transcript_inputs: statement,
                public_transcript_outputs: Vec::new(),
                binding_input: Fr::from(0u64),
                communications_commitment: Some((comm, randomness)),
                key_location: KeyLocation(CHECK_KEY_LOCATION.into()),
            }
        }
    };
    preimage.check(&c.ir).map_err(|e| {
        C6ScalarTagConsumerError::Backend(format!("SDK ProofPreimage::check failed: {e}"))
    })
}

fn assert_raw_ok(w: &C6ScalarTagStepWitness, statement: Vec<Fr>, label: &str) {
    for mode in [
        C6PushStatementBindingMode::LocalPublic,
        C6PushStatementBindingMode::LedgerCall,
    ] {
        let c = circuit_for(w, mode);
        assert!(
            check_raw(&c, w, statement.clone(), None).is_ok(),
            "{label}: raw positive control must pass in {mode:?}"
        );
    }
}

fn assert_raw_rejects(w: &C6ScalarTagStepWitness, statement: Vec<Fr>, label: &str) {
    for mode in [
        C6PushStatementBindingMode::LocalPublic,
        C6PushStatementBindingMode::LedgerCall,
    ] {
        let c = circuit_for(w, mode);
        match check_raw(&c, w, statement.clone(), None) {
            Err(C6ScalarTagConsumerError::Backend(m)) => assert!(
                m.contains("SDK ProofPreimage::check failed"),
                "{label} {mode:?}: must be SDK preprocessing, got: {m}"
            ),
            other => panic!(
                "{label} {mode:?}: expected raw SDK rejection, ok={}",
                other.is_ok()
            ),
        }
    }
}

fn set_prog(s: &mut [Fr], w: &C6ScalarTagStepWitness, idx: usize, v: u128) {
    s[pre_prog_at(w, idx)] = Fr::from(v);
    s[post_prog_at(w, idx)] = Fr::from(v);
}

fn disabled_statement(w: &C6ScalarTagStepWitness) -> Vec<Fr> {
    let mut s = w.to_statement_frs();
    s[0] = Fr::from(0u64);
    s[1] = Fr::from(0u64);
    let pre = s[pre_state_start(w)..pre_state_start(w) + state_len(w)].to_vec();
    let at = post_state_start(w);
    s[at..at + state_len(w)].copy_from_slice(&pre);
    s
}

// ---------- PROG lane offsets over the actual encode_program order ---

struct ProgOffsets {
    type_row: usize,
    ctor_row: usize,
    const_row: usize,
    ctor_field_count: usize,
    n_types: usize,
    n_ctors: usize,
}

fn prog_offsets(prog: &[u128], type_idx: usize, ctor_idx: usize, const_idx: usize) -> ProgOffsets {
    fn skip_string(prog: &[u128], p: &mut usize) {
        let n = prog[*p] as usize;
        *p += 1 + n;
    }
    fn skip_type_expr(prog: &[u128], p: &mut usize) {
        let tag = prog[*p];
        *p += 1;
        match tag {
            0 | 1 | 2 | 3 => {}
            4 | 5 => skip_string(prog, p),
            6 => {
                skip_string(prog, p);
                let arity = prog[*p] as usize;
                *p += 1;
                for _ in 0..arity {
                    skip_type_expr(prog, p);
                }
            }
            t => panic!("prog type_expr tag {t}"),
        }
    }
    let mut p = 4usize;
    let n_expr = prog[p] as usize;
    p += 1;
    for _ in 0..n_expr {
        p += match prog[p] {
            0 | 1 => 2,
            2 | 3 | 5 => 4,
            4 => 3,
            6 => 4,
            7 => 6,
            8 => 7,
            t => panic!("prog expr tag {t}"),
        };
    }
    let n_occ = prog[p] as usize;
    p += 1 + n_occ * 4;
    let n_paths = prog[p] as usize;
    p += 1 + n_paths;
    let n_children = prog[p] as usize;
    p += 1 + n_children;
    let n_funcs = prog[p] as usize;
    p += 1 + n_funcs * 5;
    let n_params = prog[p] as usize;
    p += 1 + n_params * 2;
    let n_arms = prog[p] as usize;
    p += 1 + n_arms * 2;
    let n_pats = prog[p] as usize;
    p += 1;
    for _ in 0..n_pats {
        p += match prog[p] {
            0 | 3 => 2,
            1 | 5 => 1,
            2 | 4 => 3,
            6 => 5,
            t => panic!("prog pattern tag {t}"),
        };
    }
    let n_pchildren = prog[p] as usize;
    p += 1 + n_pchildren;
    let n_types = prog[p] as usize;
    // Constant-only lookup in a valid program with empty type/ctor
    // tables (dedicated CoreProgram snapshot) must not panic before
    // the constants table is reached; rows are then unused sentinels.
    let type_row = if n_types == 0 {
        assert_eq!(type_idx, 0, "empty type table takes only index 0");
        0
    } else {
        assert!(type_idx < n_types, "prog type index present");
        p + 1 + type_idx * 7
    };
    p += 1 + n_types * 7;
    let n_targs = prog[p] as usize;
    p += 1 + n_targs;
    let n_ctors = prog[p] as usize;
    let ctor_row = if n_ctors == 0 {
        assert_eq!(ctor_idx, 0, "empty ctor table takes only index 0");
        0
    } else {
        assert!(ctor_idx < n_ctors, "prog ctor index present");
        p + 1 + ctor_idx * 5
    };
    p += 1 + n_ctors * 5;
    let ctor_field_count = prog[p] as usize;
    p += 1 + ctor_field_count * 2;
    let n_dpc = prog[p] as usize;
    p += 1 + n_dpc;
    let n_dcc = prog[p] as usize;
    p += 1 + n_dcc;
    let n_decls = prog[p] as usize;
    p += 1;
    for _ in 0..n_decls {
        skip_string(prog, &mut p);
        let np = prog[p] as usize;
        p += 1;
        for _ in 0..np {
            skip_string(prog, &mut p);
        }
        let nc = prog[p] as usize;
        p += 1;
        for _ in 0..nc {
            skip_string(prog, &mut p);
            let nf = prog[p] as usize;
            p += 1;
            for _ in 0..nf {
                skip_string(prog, &mut p);
                skip_type_expr(prog, &mut p);
            }
        }
    }
    let n_plans = prog[p] as usize;
    p += 1;
    for _ in 0..n_plans {
        p += match prog[p] {
            0 => 8,
            1 => 6,
            2 => 3,
            3 => 2,
            t => panic!("prog plan tag {t}"),
        };
    }
    let n_cases = prog[p] as usize;
    p += 1 + n_cases * 2;
    let n_fpaths = prog[p] as usize;
    p += 1 + n_fpaths;
    let n_consts = prog[p] as usize;
    assert!(const_idx < n_consts, "prog const index present");
    p += 1;
    let mut const_row = usize::MAX;
    for i in 0..n_consts {
        if i == const_idx {
            const_row = p;
        }
        p += match prog[p] {
            0 | 1 => 3,
            2 | 7 => 2,
            3 | 4 | 5 => 4,
            6 => 6,
            t => panic!("prog const tag {t}"),
        };
    }
    ProgOffsets {
        type_row,
        ctor_row,
        const_row,
        ctor_field_count,
        n_types,
        n_ctors,
    }
}

fn selected_const_index(w: &C6ScalarTagStepWitness) -> usize {
    assert_eq!(
        w.pre_helper()[L_HELPER_RECORD + 6],
        0,
        "fixture operandA must be in Const space"
    );
    w.pre_helper()[L_HELPER_RECORD + 7] as usize
}

fn set_post_typetest_payload(s: &mut [Fr], w: &C6ScalarTagStepWitness, payload: u128) {
    // The genuine TestType post already carries phase Done (2) and
    // staged tag TypeTest (6); only the independently computed branch
    // result (staged payload) is reconciled here. No other post lane
    // — phase, regA, counters, operand Ref — is touched.
    assert_eq!(
        s[post_helper_at(w, 13)],
        Fr::from(2u64),
        "coherent TypeTest companion requires post phase Done"
    );
    assert_eq!(
        s[post_helper_at(w, 15)],
        Fr::from(6u64),
        "coherent TypeTest companion requires staged tag TypeTest"
    );
    s[post_helper_at(w, 16)] = Fr::from(payload);
}

// ---------- R1 raw Marker fixture (codec domain, rebuilt length) ---

fn marker_circuit(
    w: &C6ScalarTagStepWitness,
    prog_len: usize,
    mode: C6PushStatementBindingMode,
) -> C6ScalarTagStepCircuit {
    build_c6_scalar_tag_step_circuit(
        w.source_params().clone(),
        w.host_prefix().meta.len(),
        prog_len,
        w.host_prefix().meta_bounds_frames_offset,
        mode,
        "scalar_tag_raw_test",
    )
    .expect("marker circuit builds")
}

fn marker_post_state_start(w: &C6ScalarTagStepWitness, prog_len: usize) -> usize {
    let prefix = w.host_prefix().meta.len() + prog_len + CTX_LANES;
    4 + prefix + state_len(w) + prefix
}

fn marker_statement(w: &C6ScalarTagStepWitness, prog: &[u128], staged_payload: u128) -> Vec<Fr> {
    let hp = w.host_prefix();
    let mut post_helper = w.post_helper().to_vec();
    post_helper[L_HELPER_RECORD + 16] = staged_payload;
    let mut s = vec![
        Fr::from(w.enable()),
        Fr::from(w.disposition_is_refused_helper_work()),
        Fr::from(C6_HOST_SCHEMA),
        Fr::from(C6_PADDED_CARRIER_VERSION),
    ];
    for l in hp
        .meta
        .iter()
        .chain(prog.iter())
        .chain(hp.ctx.iter())
        .chain(w.pre_image())
        .chain(w.pre_data())
        .chain(w.pre_helper())
    {
        s.push(Fr::from(*l));
    }
    for l in hp
        .meta
        .iter()
        .chain(prog.iter())
        .chain(hp.ctx.iter())
        .chain(w.post_image())
        .chain(w.post_data())
        .chain(post_helper.iter())
    {
        s.push(Fr::from(*l));
    }
    s
}

fn marker_check_raw(
    c: &C6ScalarTagStepCircuit,
    w: &C6ScalarTagStepWitness,
    prog_len: usize,
    statement: Vec<Fr>,
) -> Result<Vec<Option<usize>>, C6ScalarTagConsumerError> {
    let preimage = match c.mode {
        C6PushStatementBindingMode::LocalPublic => ProofPreimage {
            inputs: Vec::new(),
            private_transcript: Vec::new(),
            public_transcript_inputs: statement.clone(),
            public_transcript_outputs: statement,
            binding_input: Fr::from(0u64),
            communications_commitment: None,
            key_location: KeyLocation(CHECK_KEY_LOCATION.into()),
        },
        C6PushStatementBindingMode::LedgerCall => {
            let randomness = Fr::from(0u64);
            let post = statement[marker_post_state_start(w, prog_len)..].to_vec();
            let mut comm_inputs = statement.clone();
            comm_inputs.extend(post.iter().copied());
            let comm = transient_commit(&comm_inputs, randomness);
            ProofPreimage {
                inputs: statement.clone(),
                private_transcript: Vec::new(),
                public_transcript_inputs: statement,
                public_transcript_outputs: Vec::new(),
                binding_input: Fr::from(0u64),
                communications_commitment: Some((comm, randomness)),
                key_location: KeyLocation(CHECK_KEY_LOCATION.into()),
            }
        }
    };
    preimage.check(&c.ir).map_err(|e| {
        C6ScalarTagConsumerError::Backend(format!("SDK ProofPreimage::check failed: {e}"))
    })
}

fn assert_marker_ok(w: &C6ScalarTagStepWitness, prog_len: usize, statement: Vec<Fr>, label: &str) {
    for mode in [
        C6PushStatementBindingMode::LocalPublic,
        C6PushStatementBindingMode::LedgerCall,
    ] {
        let c = marker_circuit(w, prog_len, mode);
        assert!(
            marker_check_raw(&c, w, prog_len, statement.clone()).is_ok(),
            "{label}: raw Marker positive must pass in {mode:?}"
        );
    }
}

fn assert_marker_rejects(
    w: &C6ScalarTagStepWitness,
    prog_len: usize,
    statement: Vec<Fr>,
    label: &str,
) {
    for mode in [
        C6PushStatementBindingMode::LocalPublic,
        C6PushStatementBindingMode::LedgerCall,
    ] {
        let c = marker_circuit(w, prog_len, mode);
        match marker_check_raw(&c, w, prog_len, statement.clone()) {
            Err(C6ScalarTagConsumerError::Backend(m)) => assert!(
                m.contains("SDK ProofPreimage::check failed"),
                "{label} {mode:?}: must be SDK preprocessing, got: {m}"
            ),
            other => panic!(
                "{label} {mode:?}: expected raw SDK rejection, ok={}",
                other.is_ok()
            ),
        }
    }
}

// ---------- R1: raw Const Marker (proper codec-domain fixture) -----
// A dedicated RAW Marker ProgramSnapshot, encoded through the actual
// Core program codec. This is a raw program fixture only: it is not
// an owner-admitted held Marker and proves nothing about SI17.
#[test]
fn raw_marker_testtype_false_passes_wrong_true_rejects() {
    let (w, record) = testtype_uint_witness();
    assert_ok_both(&w, "R1 genuine UInt control");
    let ci = selected_const_index(&w);
    let base_prog = record.before.admission.view.clone();
    // Dedicated CoreProgram route: the snapshot type/ctor tables are
    // empty, so the old prog_offsets(&prog, 0, 0, ..) call panicked
    // before any Marker semantics were reached.
    assert!(
        base_prog.types.is_empty(),
        "R1 dedicated snapshot has no types"
    );
    assert!(
        base_prog.constructors.is_empty(),
        "R1 dedicated snapshot has no ctors"
    );
    let marker_free = match &base_prog.constants[ci] {
        ConstantSnapshot::U128 { marker_free, n } => {
            assert_eq!(*n, 0, "R1 selected constant must be UInt 0");
            *marker_free
        }
        _ => panic!("R1 selected constant must be U128"),
    };
    // Full proper Marker snapshot: only the selected constant changes;
    // entry, expressions, functions, names ("f") and every other table
    // are retained exactly.
    let mut marker_prog = base_prog.clone();
    marker_prog.constants[ci] = ConstantSnapshot::U128OutOfRange { marker_free };
    let caps = w.source_params().caps().phys;
    let encoded =
        encode_program(&marker_prog, &caps).unwrap_or_else(|e| panic!("R1 marker encode: {e:?}"));
    // Strict inverse with exact end: decode_program refuses trailing
    // lanes, so equality here proves the counted constant_fields and
    // counted names/UTF-8 tail are coherent at the new length.
    let decoded =
        decode_program(&encoded, &caps).unwrap_or_else(|e| panic!("R1 marker decode: {e:?}"));
    assert_eq!(decoded, marker_prog, "R1 marker codec roundtrip");
    assert_eq!(
        decoded.constants[ci],
        ConstantSnapshot::U128OutOfRange { marker_free },
        "R1 decoded selected constant is Marker with retained marker_free"
    );
    assert_eq!(decoded.names, base_prog.names, "R1 names tail retained");
    assert_eq!(
        decoded.expressions, base_prog.expressions,
        "R1 expression body retained"
    );
    assert_eq!(
        decoded.functions, base_prog.functions,
        "R1 function table retained"
    );
    assert_eq!(decoded.entry, base_prog.entry, "R1 entry retained");
    // Marker row is width 2 where U128 was width 3: the coherent
    // program is exactly one lane shorter, never retag-and-resized.
    assert_eq!(
        encoded.len() + 1,
        w.host_prefix().prog.len(),
        "R1 marker program length is the true codec length"
    );
    // Coherent false post passes in BOTH modes against a circuit
    // rebuilt for the correctly encoded program length; the wrong
    // true post reaches real relation rejection in BOTH modes, with
    // the LedgerCall commitment recomputed over this statement.
    let s = marker_statement(&w, &encoded, 0);
    assert_marker_ok(
        &w,
        encoded.len(),
        s.clone(),
        "R1 Marker with coherent false post",
    );
    let s = marker_statement(&w, &encoded, 1);
    assert_marker_rejects(&w, encoded.len(), s, "R1 Marker with wrong true post");
}

// ---------- R2: nominal tuples, TestType gate and kind-only mode ----
#[test]
fn raw_nominal_invalid_tuples_reject_only_in_testtype() {
    let (w, _r) = nominal_witness();
    assert_ok_both(&w, "R2 nominal Alpha control");
    let prog = w.host_prefix().prog.clone();
    let ci = selected_const_index(&w);
    let off = prog_offsets(&prog, 4, 4, ci);
    assert_eq!(prog[off.const_row], 6, "R2 row must be Constructor");
    assert_eq!(prog[off.const_row + 2], 4, "R2 type_id must be Alpha 4");
    assert_eq!(prog[off.const_row + 3], 4, "R2 ctor_id must be MkAlpha 4");
    let base = w.to_statement_frs();

    let mut s = base.clone();
    set_prog(&mut s, &w, off.ctor_row, 5);
    assert_raw_rejects(&w, s, "R2 owning-type mismatch");

    let mut s = base.clone();
    set_prog(&mut s, &w, off.type_row + 5, 5);
    assert_raw_rejects(&w, s, "R2 ctor outside declared ctor span");

    let mut s = base.clone();
    set_prog(
        &mut s,
        &w,
        off.ctor_row + 4,
        (off.ctor_field_count + 1) as u128,
    );
    assert_raw_rejects(&w, s, "R2 field end beyond ctor_field_count");

    let mut s = base.clone();
    set_prog(&mut s, &w, off.const_row + 2, 3);
    // Selected type 3: emitted sort is the Constructor type_id (3),
    // compared with expected 4, so the actual TypeTest result is
    // false. Post is reconciled; rejection is the nominal gate
    // (type below 4), not a stale true-post mismatch.
    set_post_typetest_payload(&mut s, &w, 0);
    assert_raw_rejects(&w, s, "R2 selected type below 4");

    let mut s = base.clone();
    set_prog(&mut s, &w, off.const_row + 2, 99);
    // Selected type 99: emitted sort 99 vs expected 4 derives false.
    set_post_typetest_payload(&mut s, &w, 0);
    assert_raw_rejects(&w, s, "R2 beyond-cap selected type row (99 >= cap 16)");

    let mut s = base.clone();
    set_prog(&mut s, &w, off.const_row + 3, 99);
    assert_raw_rejects(&w, s, "R2 beyond-cap selected ctor row (99 >= cap 16)");

    // Absent physical row within the unchanged cap: actual count
    // <= id < cap 16. For the selected-type case the emitted sort
    // becomes the absent id, which differs from expected 4, so the
    // independently computed TypeTest result is false and the post
    // is reconciled to false below; rejection is then attributable
    // to the nominal capture (no actual type row), not to a stale
    // result mismatch. For the selected-ctor case the type remains
    // 4, sort still equals expected, and the genuine true post is
    // already coherent, so it is left untouched.
    let absent_type = off.n_types as u128;
    assert!(absent_type < 16, "R2 within-cap premise: count below cap");
    let mut s = base.clone();
    set_prog(&mut s, &w, off.const_row + 2, absent_type);
    set_post_typetest_payload(&mut s, &w, 0);
    assert_raw_rejects(&w, s, "R2 absent-within-cap selected type row");

    let absent_ctor = off.n_ctors as u128;
    assert!(
        absent_ctor < 16,
        "R2 within-cap ctor premise: count below cap"
    );
    let mut s = base.clone();
    set_prog(&mut s, &w, off.const_row + 3, absent_ctor);
    assert_raw_rejects(&w, s, "R2 absent-within-cap selected ctor row");

    // Outside TestType the actual Core branch is kind-only: the same
    // nominally invalid PROG must still reach PrimError{11}/Done.
    let (kw, _kr) = kind_only_ctor_witness();
    assert_ok_both(&kw, "R2 kind-only Constructor control");
    let kprog = kw.host_prefix().prog.clone();
    let koff = prog_offsets(&kprog, 4, 4, selected_const_index(&kw));
    let mut ks = kw.to_statement_frs();
    set_prog(&mut ks, &kw, koff.ctor_row, 5);
    assert_raw_ok(
        &kw,
        ks,
        "R2 kind-only Add keeps PrimError11 with invalid nominal row",
    );
}

// ---------- R3: Fr / gate / ref-space / derived-slice domains -------
#[test]
fn raw_fr_gate_refspace_and_slice_domains() {
    let (w, _r) = add_witness();
    assert_ok_both(&w, "R3 Add control");
    let base = w.to_statement_frs();

    // Unconditional 128-bit range on an unchanged uncapped lane
    // (initial fuel is preserved and carries no step cap here).
    let mut s = base.clone();
    s[pre_image_at(&w, 26)] = Fr::from(u128::MAX);
    s[post_image_at(&w, 26)] = Fr::from(u128::MAX);
    assert_raw_ok(&w, s, "R3 max-2^128-minus-1 control");

    let mut s = base.clone();
    s[pre_image_at(&w, 26)] = two_pow_128();
    s[post_image_at(&w, 26)] = two_pow_128();
    assert_raw_rejects(&w, s, "R3 value at 2^128 gate 1");

    let mut s = disabled_statement(&w);
    assert_raw_ok(&w, s.clone(), "R3 disabled baseline");
    s[pre_image_at(&w, 26)] = two_pow_128();
    s[post_image_at(&w, 26)] = two_pow_128();
    assert_raw_rejects(&w, s, "R3 value at 2^128 gate 0");

    // Strictly above 2^128 on the same unchanged uncapped lane, both
    // gates, against the correct baselines above. Each rejection is
    // asserted to be SDK preprocessing inside assert_raw_rejects.
    let above = Fr::from(u128::MAX) + Fr::from(2u64); // 2^128 + 1
    let mut s = base.clone();
    s[pre_image_at(&w, 26)] = above;
    s[post_image_at(&w, 26)] = above;
    assert_raw_rejects(&w, s, "R3 value above 2^128 gate 1");

    let mut s = disabled_statement(&w);
    s[pre_image_at(&w, 26)] = above;
    s[post_image_at(&w, 26)] = above;
    assert_raw_rejects(&w, s, "R3 value above 2^128 gate 0");

    let mut s = base.clone();
    s[0] = Fr::from(2u64);
    assert_raw_rejects(&w, s, "R3 enable gate value 2");

    let mut s = base.clone();
    s[pre_helper_at(&w, 6)] = Fr::from(2u64);
    s[post_helper_at(&w, 6)] = Fr::from(2u64);
    assert_raw_rejects(&w, s, "R3 ref space value 2");

    // Derived slice ends are 129-bit objects; under gate 0 the DATA
    // canonical slice check is not imposed, while each raw lane
    // remains within its unconditional 128-bit range.
    let start = pre_state_start(&w) + w.pre_image().len() + 7 + 3;
    let len = start + 1;
    let post_start = post_state_start(&w) + w.pre_image().len() + 7 + 3;
    let mut s = disabled_statement(&w);
    s[start] = Fr::from(u128::MAX);
    s[len] = Fr::from(1u64);
    s[post_start] = Fr::from(u128::MAX);
    s[post_start + 1] = Fr::from(1u64);
    assert_raw_ok(&w, s, "R3 disabled slice end exactly 2^128");

    let mut s = disabled_statement(&w);
    s[start] = Fr::from(u128::MAX);
    s[len] = Fr::from(u128::MAX);
    s[post_start] = Fr::from(u128::MAX);
    s[post_start + 1] = Fr::from(u128::MAX);
    assert_raw_ok(&w, s, "R3 disabled slice end 2^129-2");
}

// ---------- R4: namespace composition -------------------------------
const SEGMENTS: [&str; 9] = [
    ".local.temp.",
    ".c6data.temp.",
    ".c6helper.temp.",
    ".c6meta.temp.",
    ".c6progctx.temp.",
    ".tagread.local.temp.",
    ".tagread.c6progctx.temp.",
    ".tagread.c6data.temp.",
    ".tagnom.c6progctx.temp.",
];

fn imm(v: u128) -> Operand {
    Operand::Immediate(Fr::from(v))
}

fn valid_inputs(w: &C6ScalarTagStepWitness) -> C6ScalarTagStepInputs {
    let hp = w.host_prefix();
    let ops = |xs: &[u128]| xs.iter().map(|&v| imm(v)).collect::<Vec<_>>();
    let ctx: [Operand; CTX_LANES] = hp
        .ctx
        .iter()
        .map(|&v| imm(v))
        .collect::<Vec<_>>()
        .try_into()
        .expect("ctx lanes");
    let ctx2 = ctx.clone();
    C6ScalarTagStepInputs {
        enable: imm(1),
        disposition_is_refused_helper_work: imm(u128::from(w.disposition_is_refused_helper_work())),
        host_schema: imm(C6_HOST_SCHEMA),
        carrier_version: imm(C6_PADDED_CARRIER_VERSION),
        geometry: w.source_params().geometry(),
        meta_bounds_frames_offset: hp.meta_bounds_frames_offset,
        source_params: w.source_params().clone(),
        pre_meta: ops(&hp.meta),
        pre_prog: ops(&hp.prog),
        pre_ctx: ctx,
        pre_image: ops(w.pre_image()),
        pre_data: ops(w.pre_data()),
        pre_helper: ops(w.pre_helper()),
        post_meta: ops(&hp.meta),
        post_prog: ops(&hp.prog),
        post_ctx: ctx2,
        post_image: ops(w.post_image()),
        post_data: ops(w.post_data()),
        post_helper: ops(w.post_helper()),
    }
}

fn dests(ins: &[Instruction]) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for i in ins {
        let o = match i {
            Instruction::Add { output, .. }
            | Instruction::Mul { output, .. }
            | Instruction::Not { output, .. }
            | Instruction::CondSelect { output, .. }
            | Instruction::TestEq { output, .. }
            | Instruction::LessThan { output, .. }
            | Instruction::Copy { output, .. } => Some(output),
            _ => None,
        };
        if let Some(o) = o {
            out.insert(o.0.clone());
        }
    }
    out
}

#[test]
fn raw_namespace_reserved_operands_scopes_and_repeated_emit() {
    let (w, _r) = add_witness();
    for seg in SEGMENTS {
        let expected = format!("%tagraw{seg}77");
        let reserved = Operand::Variable(Identifier(expected.clone()));
        for group in 0..16usize {
            let mut inputs = valid_inputs(&w);
            match group {
                0 => inputs.enable = reserved.clone(),
                1 => inputs.disposition_is_refused_helper_work = reserved.clone(),
                2 => inputs.host_schema = reserved.clone(),
                3 => inputs.carrier_version = reserved.clone(),
                4 => inputs.pre_meta[0] = reserved.clone(),
                5 => inputs.pre_prog[0] = reserved.clone(),
                6 => inputs.pre_ctx[0] = reserved.clone(),
                7 => inputs.pre_image[0] = reserved.clone(),
                8 => inputs.pre_data[0] = reserved.clone(),
                9 => inputs.pre_helper[0] = reserved.clone(),
                10 => inputs.post_meta[0] = reserved.clone(),
                11 => inputs.post_prog[0] = reserved.clone(),
                12 => inputs.post_ctx[0] = reserved.clone(),
                13 => inputs.post_image[0] = reserved.clone(),
                14 => inputs.post_data[0] = reserved.clone(),
                _ => inputs.post_helper[0] = reserved.clone(),
            }
            let mut em = C6ScalarTagStepEmitter::new("tagraw").expect("scope");
            match em.emit(&inputs) {
                Err(C6ScalarTagStepError::ReservedOperand { identifier }) => {
                    assert_eq!(identifier, expected, "R4 segment {seg} group {group}")
                }
                other => panic!(
                    "R4 segment {seg} group {group}: expected \
                     ReservedOperand, ok={}",
                    other.is_ok()
                ),
            }
        }
        match C6ScalarTagStepEmitter::new(&format!("bad{seg}x")) {
            Err(C6ScalarTagStepError::InvalidScope) => {}
            other => panic!(
                "R4 scope {seg}: expected InvalidScope, ok={}",
                other.is_ok()
            ),
        }
    }
    match C6ScalarTagStepEmitter::new("") {
        Err(C6ScalarTagStepError::EmptyScope) => {}
        other => panic!("R4 empty scope: ok={}", other.is_ok()),
    }
    match C6ScalarTagStepEmitter::new("bad%scope") {
        Err(C6ScalarTagStepError::InvalidScope) => {}
        other => panic!("R4 percent scope: ok={}", other.is_ok()),
    }

    let mut em = C6ScalarTagStepEmitter::new("tagraw_repeat").expect("repeat scope");
    let inputs = valid_inputs(&w);
    let first = em.emit(&inputs).expect("first emit");
    let after_first = em.next_temp();
    let second = em.emit(&inputs).expect("second emit");
    assert!(em.next_temp() > after_first, "R4 serial advances");
    let d1 = dests(&first.instructions);
    let d2 = dests(&second.instructions);
    assert!(
        !d1.is_empty() && !d2.is_empty(),
        "R4 definition destinations must be nonempty"
    );
    assert!(
        d1.is_disjoint(&d2),
        "R4 repeated same-scope emit destinations must be disjoint"
    );
}

// ---------- R5: genuine max_types == 0 ------------------------------
#[test]
fn raw_max_types_zero_builtin_and_expected_4_domain() {
    let mut b = bounds(100);
    b.type_rows = 0;
    let (w, record) = reach_core(
        b,
        ScalarMode::TestType,
        "R5 max_types0 TestType UInt",
        CoreProgram {
            functions: vec![CoreFunction {
                name: "f".into(),
                parameters: Vec::new(),
                return_type: SortTag::UInt,
                body: lit_u(42),
            }],
            entry: CoreExpr::Call {
                name: "f".into(),
                args: Vec::new(),
            },
        },
    );
    assert_eq!(
        record.before.admission.bounds.type_rows, 0,
        "R5 original admission bounds must carry type_rows 0"
    );
    assert!(
        record.before.admission.view.types.is_empty(),
        "R5 empty type table"
    );
    assert_eq!(
        w.source_params().caps().phys.max_types,
        0,
        "R5 derived phys max_types must be 0"
    );
    assert_ok_both(&w, "R5 builtin TestType at max_types 0");

    let mut s = w.to_statement_frs();
    s[pre_helper_at(&w, 11)] = Fr::from(1u64);
    s[pre_helper_at(&w, 12)] = Fr::from(4u64);
    s[post_helper_at(&w, 11)] = Fr::from(1u64);
    s[post_helper_at(&w, 12)] = Fr::from(4u64);
    // Coherent branch/result companion: the operand remains the
    // actual UInt constant, so with expected TypeId 4 the emitted
    // checked-sort comparison (builtin UInt sort 0 vs expected 4)
    // independently derives TypeTest ok=false. The post staged
    // payload is therefore false, not the stale genuine true.
    // Admission type_rows 0, derived max_types 0, the actual empty
    // types table, and builtin sorts 0..3 are all preserved above;
    // rejection must now reach the strict expected-index boundary
    // (expected 4 is outside the builtin domain at max_types 0)
    // rather than a stale result mismatch.
    set_post_typetest_payload(&mut s, &w, 0);
    assert_eq!(
        s[pre_helper_at(&w, 12)],
        Fr::from(4u64),
        "R5 target: pre expected TypeId is 4 in both helper copies"
    );
    assert_eq!(
        s[post_helper_at(&w, 12)],
        Fr::from(4u64),
        "R5 target: post expected TypeId is 4 in both helper copies"
    );
    assert_eq!(
        s[post_helper_at(&w, 16)],
        Fr::from(0u64),
        "R5 target: coherent post is TypeTest false for UInt vs 4"
    );
    assert_raw_rejects(
        &w,
        s,
        "R5 expected TypeId 4 fails strict builtin index domain",
    );
}

// ---------- R6/R7: independent wrong post, P3, overflow, stale CC ---
#[test]
fn raw_wrong_post_transition_cases_reject() {
    let (w, _r) = add_witness();
    assert_ok_both(&w, "R6 applied control");
    let base = w.to_statement_frs();

    let mut s = base.clone();
    s[post_helper_at(&w, 13)] = Fr::from(2u64);
    assert_raw_rejects(&w, s, "R6 wrong post phase Done");

    let mut s = base.clone();
    s[post_helper_at(&w, 14)] = Fr::from(999u64);
    assert_raw_rejects(&w, s, "R6 wrong post regA");

    let mut s = base.clone();
    s[post_helper_at(&w, 15)] = Fr::from(6u64);
    s[post_helper_at(&w, 16)] = Fr::from(1u64);
    assert_raw_rejects(&w, s, "R6 wrong post staged tag/payload");

    let mut s = base.clone();
    s[post_helper_at(&w, 7)] = Fr::from(w.pre_helper()[L_HELPER_RECORD + 7] + 1);
    assert_raw_rejects(&w, s, "R6 wrong post operand Ref index");

    let mut s = base.clone();
    s[post_image_at(&w, L_CONSUMED_WORK)] = Fr::from(999_999u64);
    assert_raw_rejects(&w, s, "R6 wrong post consumed counter");

    let mut s = base.clone();
    s[post_image_at(&w, L_REFERENCE_CURSOR)] = Fr::from(999_999u64);
    assert_raw_rejects(&w, s, "R6 wrong post reference cursor");

    let mut s = base.clone();
    s[post_image_at(&w, L_HELPER_VISITS)] = Fr::from(999_999u64);
    assert_raw_rejects(&w, s, "R6 wrong post helper visits");

    let mut s = base.clone();
    s[post_data_at(&w, 10)] = Fr::from(777u64);
    assert_raw_rejects(&w, s, "R6 wrong post DATA lane");

    let mut s = base.clone();
    s[post_image_at(&w, L_CONTROL)] = Fr::from(999u64);
    assert_raw_rejects(&w, s, "R6 wrong post control lane");

    let mut s = base.clone();
    s[post_image_at(&w, 48)] = Fr::from(999u64);
    assert_raw_rejects(&w, s, "R6 wrong post first frame lane");

    let mut s = base.clone();
    s[post_prog_at(&w, 0)] = Fr::from(0u64);
    assert_raw_rejects(&w, s, "R6 wrong post PROG lane");

    let mut s = base.clone();
    s[post_ctx_at(&w, 2)] = Fr::from(999u64);
    assert_raw_rejects(&w, s, "R6 wrong post CTX lane");

    // Explicit stale-commitment case only: this is not a relation
    // rejection, and no other case above uses it as a proxy.
    let c = circuit_for(&w, C6PushStatementBindingMode::LedgerCall);
    let randomness = Fr::from(0u64);
    let mut stale_inputs = base.clone();
    stale_inputs.extend(base[post_state_start(&w)..].iter().copied());
    let stale = transient_commit(&stale_inputs, randomness);
    let mut wrong = base.clone();
    wrong[post_helper_at(&w, 14)] = Fr::from(999u64);
    match check_raw(&c, &w, wrong, Some(stale)) {
        Err(C6ScalarTagConsumerError::Backend(m)) => assert!(
            m.contains("SDK ProofPreimage::check failed"),
            "R7 stale CC must fail preprocessing: {m}"
        ),
        other => panic!("R7 stale CC: expected rejection, ok={}", other.is_ok()),
    }
}

fn p3_statement(w: &C6ScalarTagStepWitness) -> Vec<Fr> {
    let mut s = w.to_statement_frs();
    let max = w.source_params().admission_bounds().max_helper_work;
    s[pre_image_at(w, L_HELPER_WORK)] = Fr::from(max);
    let pre = s[pre_state_start(w)..pre_state_start(w) + state_len(w)].to_vec();
    let at = post_state_start(w);
    s[at..at + state_len(w)].copy_from_slice(&pre);
    s[post_image_at(w, L_HELPER_WORK)] = Fr::from(max);
    s[post_image_at(w, L_OUTCOME)] = Fr::from(3u64);
    s[post_image_at(w, L_OUTCOME + 1)] = Fr::from(5u64);
    s[post_image_at(w, L_OUTCOME + 2)] = Fr::from(0u64);
    s[1] = Fr::from(1u64);
    s
}

#[test]
fn raw_p3_and_overflow_cases() {
    let (w, _r) = add_witness();
    let p3 = p3_statement(&w);
    assert_raw_ok(&w, p3.clone(), "R7 raw P3 coherent control");

    let mut s = p3.clone();
    s[1] = Fr::from(0u64);
    assert_raw_rejects(&w, s, "R7 P3 wrong disposition flag");

    let mut s = p3.clone();
    s[post_image_at(&w, L_CONSUMED_WORK)] = Fr::from(w.pre_image()[L_CONSUMED_WORK] + 1);
    assert_raw_rejects(&w, s, "R7 P3 wrong claimed consumed charge");

    let mut s = p3.clone();
    s[post_image_at(&w, L_OUTCOME)] = Fr::from(0u64);
    s[post_image_at(&w, L_OUTCOME + 1)] = Fr::from(0u64);
    assert_raw_rejects(&w, s, "R7 P3 wrong Running outcome claim");

    let base = w.to_statement_frs();
    let mut s = base.clone();
    s[pre_image_at(&w, L_CONSUMED_WORK)] = Fr::from(u128::MAX);
    s[post_image_at(&w, L_CONSUMED_WORK)] = Fr::from(0u64);
    assert_raw_rejects(&w, s, "R7 consumed-work u128 maximum");

    let mut s = base.clone();
    s[pre_image_at(&w, L_HELPER_WORK)] = Fr::from(u128::MAX);
    s[post_image_at(&w, L_HELPER_WORK)] = Fr::from(0u64);
    assert_raw_rejects(&w, s, "R7 helper-work u128 maximum");

    let mut s = base.clone();
    s[pre_image_at(&w, L_HELPER_VISITS)] = Fr::from(u128::MAX);
    s[post_image_at(&w, L_HELPER_VISITS)] = Fr::from(0u64);
    assert_raw_rejects(&w, s, "R7 helper-visits u128 maximum");

    let mut s = base.clone();
    s[pre_image_at(&w, L_REMAINING_WORK)] = Fr::from(1u64);
    s[post_image_at(&w, L_REMAINING_WORK)] = Fr::from(1u64);
    assert_raw_rejects(&w, s, "R7 wrong post remaining-work charge");
}

#[test]
fn raw_live_post_env_and_value_slot_refs_reject_both_modes() {
    let (w, record) = live_call_witness(9, "F6 live env/value TestType");
    assert_live_premises(&w, &record, "F6 live env/value control");
    let operand = selected_operand_a(&record);
    let selected = match operand {
        RefC::Const(i) => i,
        other => panic!("F6 operand must be a Const Ref, got {other:?}"),
    };
    // The other argument's genuinely admitted Const Ref: a different,
    // canonical in-program Ref used as the mutated post identity.
    let other = record
        .before
        .image
        .value_slots
        .iter()
        .find_map(|r| match r {
            RefC::Const(i) if *i != selected => Some(*i),
            _ => None,
        })
        .expect("F6 second argument Const Ref must exist");
    match &record.before.admission.view.constants[other] {
        ConstantSnapshot::U128 { n: 9, .. } => {}
        _ => panic!("F6 other Const Ref must decode to UInt 9"),
    }
    let geom = w.source_params().geometry();
    let base = w.to_statement_frs();

    // Post env cell: the live cell whose value is the selected Ref.
    // Row layout is ENV_ROW_LANES [binder, ref space, ref index,
    // previous tag, previous index]; mutate only the ref-index lane
    // of the POST copy to the other valid Const Ref.
    let cell_pos = record
        .before
        .image
        .env
        .iter()
        .position(|c| c.value == operand)
        .expect("F6 live env cell holding the selected Ref must exist");
    let env_index_lane = geom.env_base() + cell_pos * ENV_ROW_LANES + 2;
    assert_eq!(
        w.pre_data()[env_index_lane],
        selected as u128,
        "F6 pre env value Ref index lane must hold the selected Ref"
    );
    assert_eq!(
        w.pre_data()[env_index_lane - 1],
        0,
        "F6 pre env value Ref space must be Const"
    );
    let mut s = base.clone();
    s[post_data_at(&w, env_index_lane)] = Fr::from(other as u128);
    assert_raw_rejects(&w, s, "F6 wrong post live env-cell value Ref");

    // Post value slot: the live slot holding the selected Ref.
    // Slot layout is REF_LANES [space, index]; same mutation domain.
    let slot_pos = record
        .before
        .image
        .value_slots
        .iter()
        .position(|r| *r == operand)
        .expect("F6 live value slot holding the selected Ref must exist");
    let slot_index_lane = geom.value_slots_base() + slot_pos * REF_LANES + 1;
    assert_eq!(
        w.pre_data()[slot_index_lane],
        selected as u128,
        "F6 pre value-slot Ref index lane must hold the selected Ref"
    );
    assert_eq!(
        w.pre_data()[slot_index_lane - 1],
        0,
        "F6 pre value-slot Ref space must be Const"
    );
    let mut s = base.clone();
    s[post_data_at(&w, slot_index_lane)] = Fr::from(other as u128);
    assert_raw_rejects(&w, s, "F6 wrong post live value-slot Ref");
}

#[test]
fn raw_equal_payload_distinct_const_ref_post_identity_rejects() {
    let (w, record) = live_call_witness(7, "F6 equal-payload Ref TestType");
    assert_live_premises(&w, &record, "F6 equal-payload control");
    let operand = selected_operand_a(&record);
    let selected = match operand {
        RefC::Const(i) => i,
        other => panic!("F6 operand must be a Const Ref, got {other:?}"),
    };
    let other = record
        .before
        .image
        .value_slots
        .iter()
        .find_map(|r| match r {
            RefC::Const(i) if *i != selected => Some(*i),
            _ => None,
        })
        .expect("F6 distinct second Const Ref must exist");
    assert_ne!(selected, other, "F6 Const Ref indices must differ");
    for i in [selected, other] {
        match &record.before.admission.view.constants[i] {
            ConstantSnapshot::U128 { n: 7, .. } => {}
            _ => panic!("F6 both Const Refs must decode to UInt 7"),
        }
    }
    // Change ONLY the post helper operand Ref index to the other
    // equal-payload valid Ref. Post phase/regA/staged result and all
    // counters remain the genuine derived values; rejection is the
    // emitted relation's post-helper identity preservation over the
    // pre-helper operand lanes. This is a POST Ref identity vector
    // only: it proves no stronger PRE Ref origin, StepInv, or
    // program-membership binding, and mutating both pre and post to
    // the other equal-valued Ref is not claimed to reject.
    let base = w.to_statement_frs();
    let mut s = base.clone();
    s[post_helper_at(&w, 7)] = Fr::from(other as u128);
    assert_eq!(
        s[post_helper_at(&w, 6)],
        base[post_helper_at(&w, 6)],
        "F6 Ref space lane must be preserved"
    );
    assert_eq!(
        s[post_helper_at(&w, 16)],
        base[post_helper_at(&w, 16)],
        "F6 derived TypeTest payload must be preserved"
    );
    assert_raw_rejects(&w, s, "F6 post operand Ref identity with equal payload");
}
