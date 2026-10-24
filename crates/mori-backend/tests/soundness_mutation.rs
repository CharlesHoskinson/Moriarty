//! T1 soundness oracle: witness-mutation run over real Sherlock machine
//! circuits whose valid witnesses `MockProver` already accepts.
//!
//! The fixtures are the public-API reconstructions of the in-crate
//! `MockProver` tests of `c6_data_canonical`, `c6_checked_value_read` and
//! `c6_scalar_failure_step`. Each circuit is run through
//! `mori_backend::qa_mutation::run_suite` with a fixed seed and a bounded
//! number of mutated cells. Any accepted single-cell mutation fails the
//! test and is listed as `(circuit, column, row)`.
//!
//! Known under-constrained cells found by this oracle are kept in separate
//! `#[ignore = "UNDER-CONSTRAINED: ..."]` tests; the default tests stay green
//! only while their allowlist (with written reasons) is the only exclusion.

use midnight_proofs::circuit::Value;
use midnight_transient_crypto::curve::{Fr, outer};
use midnight_zk_stdlib::MidnightCircuit;
use midnight_zkir_v3::ir::{IrMinorVersion, Operand, TypedIdentifier};
use midnight_zkir_v3::ir_types::{IrType, IrValue};
use midnight_zkir_v3::{Identifier, Instruction, IrSource, Preprocessed};
use mori_backend::qa_mutation::{Allow, Config, Report, inspect_row, run_suite};
use mori_backend::sherlock_machine::c6_checked_value_read::{
    C6CheckedValueReadInputs, emit_c6_checked_value_read_block,
};
use mori_backend::sherlock_machine::c6_data_canonical::{
    C6DataCanonicalInputs, c6_data_geometry, emit_c6_data_canonical_block,
};
use mori_backend::sherlock_machine::c6_scalar_failure_step::{
    C6ScalarFailureStepInputs, emit_c6_scalar_failure_step_block, witness_from_host_pair,
};
use mori_core::sherlock::helper_machine::c6_phys::{C6_PHYS_MAGIC, C6_PHYS_VERSION, C6PhysCaps};
use mori_core::sherlock::helper_machine::c6_phys_data::{
    C6StateCaps, DATA_ROW_LANES, DATA_SECTION_TAG, ENV_ROW_LANES, REF_LANES, encode_data_row,
    encode_env_row, encode_ref_row,
};
use mori_core::sherlock::helper_machine::c6_phys_host::{
    C6_HOST_SCHEMA, C6HostConfig, C6HostLimits, CTX_LANES,
};
use mori_core::sherlock::helper_machine::c6_phys_program::encode_program;
use mori_core::sherlock::helper_machine::complete_observation::{
    AdmissionSnapshot, ConstantSnapshot, DataSnapshot, ExpressionSnapshot, HostSnapshot,
    ImageSnapshot, LayoutSnapshot, ProgramSnapshot,
};
use mori_core::sherlock::helper_machine::shared::host_implied_prefix::SegBase;
use mori_core::sherlock::helper_machine::shared::{
    ActiveLengths, AllocCounters, BinaryOp, BoundsC, ControlC, DigestC, EnvCellC, FinanceInput,
    FrameRowC, GrammarDomain, HelperCommon, HelperControlC, HelperResultC, IdentityC,
    LayoutCapacitiesC, OutcomeC, RefC, RunIdC, RunIntC, ScalarMode, ScalarPhase, ScalarStaged,
    SiteTag,
};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

const SEED: u64 = 0x5EED_50DA_7E57;

// ------------------------------------------------------------ common glue

/// Load an `IrSource` through the on-disk v3 form, as every fixture does.
fn load_ir(instructions: Vec<Instruction>, inputs: Vec<TypedIdentifier>) -> IrSource {
    let ir = IrSource {
        version: IrMinorVersion::V0,
        inputs,
        outputs: Vec::new(),
        do_communications_commitment: false,
        instructions: Arc::new(instructions),
    };
    let mut disk = serde_json::to_value(&ir).expect("serialize ir");
    disk["version"] = json!({ "major": 3, "minor": 0 });
    let serialized = serde_json::to_string(&disk).expect("serialize disk ir");
    IrSource::load(serialized.as_bytes()).expect("load ir")
}

/// Allocate `n` public native lanes named `%{scope}.public.{label}.{i}`
/// (or `%{scope}.public.{i}` when `label` is empty).
fn public_lanes(
    scope: &str,
    label: &str,
    n: usize,
    ins: &mut Vec<Instruction>,
    tr: &mut Vec<Operand>,
    ids: &mut Vec<Identifier>,
) -> Vec<Operand> {
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let name = if label.is_empty() {
            format!("%{scope}.public.{i}")
        } else {
            format!("%{scope}.public.{label}.{i}")
        };
        let id = Identifier(name);
        ins.push(Instruction::PublicInput {
            guard: None,
            val_t: IrType::Native,
            output: id.clone(),
        });
        let op = Operand::Variable(id.clone());
        tr.push(op.clone());
        ids.push(id);
        out.push(op);
    }
    out
}

/// Build the `MidnightCircuit` and `MockProver` instance for a public-lane
/// statement, exactly as the in-crate synthesis tests do.
fn midnight<'a>(
    ir: &'a IrSource,
    ids: &[Identifier],
    statement: &[Fr],
    k: Option<u32>,
) -> (MidnightCircuit<'a, IrSource>, Vec<Vec<outer::Scalar>>) {
    assert_eq!(ids.len(), statement.len());
    let mut memory: HashMap<Identifier, IrValue> = HashMap::new();
    for (id, value) in ids.iter().zip(statement.iter()) {
        memory.insert(id.clone(), IrValue::Native(*value));
    }
    let instance: Vec<outer::Scalar> = statement.iter().map(|v| v.0).collect();
    let mut pis: Vec<outer::Scalar> = vec![Fr::from(0u64).0];
    pis.extend(instance.iter().copied());
    let preimage = Preprocessed {
        memory,
        pis: pis.clone(),
        pi_skips: vec![None],
        binding_input: Fr::from(0u64).0,
        comm_comm: None,
    };
    let circuit = MidnightCircuit::new(ir, Value::known(instance), Value::known(preimage), k);
    (circuit, vec![Vec::new(), pis])
}

/// Zero-size limb padding of `P2RDecompositionChip::decompose_core`
/// (midnight-circuits 7.2.4, `field/decomposition/chip.rs`): "by convention
/// the coefficient of a zero sized limb is 0 so no constraint needs to be
/// imposed in the corresponding limb". In the native `arith_gate`
/// (`field/native/native_chip.rs`) advice column `i` enters through the
/// linear coefficient `fixed[4 + i]`, and columns 1 and 2 additionally
/// through `mul_ab = fixed[1]` / `mul_ac = fixed[2]` (column 0 also feeds
/// `q_next`). A slot whose coefficient columns are all zero feeds no gate;
/// it is only range-checked by the chunk's lookup tag.
fn decomposition_padding() -> Vec<Allow> {
    const REASON: &str = "documented zero-size limb padding: all fixed coefficients of the slot are zero, cell feeds no gate";
    vec![
        Allow {
            column: 1,
            rows: 0..usize::MAX,
            region_contains: "decompose core",
            fixed_zero: &[5, 1],
            reason: REASON,
        },
        Allow {
            column: 2,
            rows: 0..usize::MAX,
            region_contains: "decompose core",
            fixed_zero: &[6, 2],
            reason: REASON,
        },
        Allow {
            column: 3,
            rows: 0..usize::MAX,
            region_contains: "decompose core",
            fixed_zero: &[7],
            reason: REASON,
        },
        Allow {
            column: 4,
            rows: 0..usize::MAX,
            region_contains: "decompose core",
            fixed_zero: &[8],
            reason: REASON,
        },
    ]
}

/// Mutation budget of one suite run. One `MockProver` run costs seconds to
/// about a minute in the dev profile for these circuits (hundreds of
/// thousands to millions of advice cells), so the default budgets are small
/// and the `deep_*` tests (ignored) sweep further on demand.
#[derive(Clone, Copy)]
struct Budget {
    cells: usize,
    mutations: usize,
    swaps: usize,
}

fn config(b: Budget, allow: Vec<Allow>) -> Config {
    Config {
        seed: SEED,
        max_cells: Some(b.cells),
        max_mutations: Some(b.mutations),
        swaps: b.swaps,
        allow,
    }
}

fn run_named(
    name: &str,
    ir: &IrSource,
    ids: &[Identifier],
    statement: &[Fr],
    k: Option<u32>,
    cfg: &Config,
) -> Report {
    let start = Instant::now();
    let (circuit, instance) = midnight(ir, ids, statement, k);
    let report = run_suite(name, &circuit, &instance, cfg)
        .unwrap_or_else(|e| panic!("{name}: harness could not start: {e}"));
    eprintln!("{} [{:.1?}]", report.summary(), start.elapsed());
    for a in report.accepted.iter().take(3) {
        eprintln!(
            "  row context for circuit={name} column={} row={}:\n{}",
            a.column,
            a.row,
            inspect_row(&circuit, &instance, a.row).unwrap_or_default()
        );
    }
    for a in &report.accepted_swaps {
        eprintln!(
            "  note: accepted swap (symmetric or free) circuit={name} column={} row={} <-> {:?}",
            a.column, a.row, a.kind
        );
    }
    report
}

fn assert_sound(report: &Report) {
    assert!(
        report.cells_tested > 0,
        "{}: no cells tested",
        report.circuit
    );
    assert!(
        report.mutations_run > 0,
        "{}: no mutations run",
        report.circuit
    );
    assert!(
        report.accepted.is_empty(),
        "UNDER-CONSTRAINED cells ({} accepted single-cell mutations) in {}:\n{}",
        report.accepted.len(),
        report.circuit,
        report.describe_accepted()
    );
}

// ------------------------------------------------- c6_data_canonical

fn data_caps() -> C6StateCaps {
    let mut phys = C6PhysCaps::test_caps();
    phys.max_data_nodes = 2;
    phys.max_constants = 3;
    phys.max_types = 2;
    phys.max_constructors = 2;
    phys.max_names = 2;
    phys.max_arity = 2;
    phys.max_env_cells = 2;
    phys.max_value_slots = 2;
    C6StateCaps {
        phys,
        max_data_field_slots: 3,
        max_helper_tasks: 0,
    }
}

fn data_lanes(caps: &C6StateCaps) -> Vec<u128> {
    let g = c6_data_geometry(caps).expect("geometry");
    let mut out = vec![0u128; g.total];
    let data = [
        DataSnapshot::U128 { n: u128::MAX },
        DataSnapshot::Bool { b: true },
    ];
    let fields = [RefC::Const(2), RefC::Data(1), RefC::Const(0)];
    let env = [
        EnvCellC {
            binder: 1,
            value: RefC::Data(1),
            previous: Some(0),
        },
        EnvCellC {
            binder: 0,
            value: RefC::Const(0),
            previous: None,
        },
    ];
    let values = [RefC::Data(1), RefC::Const(2)];
    out[0] = C6_PHYS_MAGIC;
    out[1] = C6_PHYS_VERSION;
    out[2] = DATA_SECTION_TAG;
    out[3] = data.len() as u128;
    out[4] = fields.len() as u128;
    out[5] = env.len() as u128;
    out[6] = values.len() as u128;
    for (i, row) in data.iter().enumerate() {
        let b = g.data_base + i * DATA_ROW_LANES;
        out[b..b + DATA_ROW_LANES].copy_from_slice(&encode_data_row(row));
    }
    for (i, r) in fields.iter().enumerate() {
        let b = g.fields_base + i * REF_LANES;
        out[b..b + REF_LANES].copy_from_slice(&encode_ref_row(*r));
    }
    for (i, cell) in env.iter().enumerate() {
        let b = g.env_base + i * ENV_ROW_LANES;
        out[b..b + ENV_ROW_LANES].copy_from_slice(&encode_env_row(cell));
    }
    for (i, r) in values.iter().enumerate() {
        let b = g.values_base + i * REF_LANES;
        out[b..b + REF_LANES].copy_from_slice(&encode_ref_row(*r));
    }
    out
}

fn run_data_canonical(b: Budget, allow: Vec<Allow>) -> Report {
    let caps = data_caps();
    let lanes = data_lanes(&caps);
    let mut ins = Vec::new();
    let mut tr = Vec::new();
    let mut ids = Vec::new();
    let ops = public_lanes(
        "c6data_test",
        "",
        lanes.len() + 1,
        &mut ins,
        &mut tr,
        &mut ids,
    );
    let block = emit_c6_data_canonical_block(
        &C6DataCanonicalInputs {
            gate: ops[0].clone(),
            data: ops[1..].to_vec(),
            caps,
        },
        "c6data_test",
    )
    .expect("emit data canonical");
    ins.extend(block.instructions);
    ins.push(Instruction::Impact {
        guard: Operand::Immediate(Fr::from(1u64)),
        inputs: tr,
    });
    let ir = load_ir(ins, Vec::new());
    let mut statement = vec![Fr::from(1u64)];
    statement.extend(lanes.iter().map(|&l| Fr::from(l)));
    run_named(
        "c6_data_canonical",
        &ir,
        &ids,
        &statement,
        Some(14),
        &config(b, allow),
    )
}

// ------------------------------------------------ c6_checked_value_read

/// Small physical caps: the in-crate test uses `test_caps()` (hundreds of
/// rows per table), which makes one `MockProver` run cost tens of seconds.
/// The emitted gates are identical; only the number of table rows differs.
fn read_caps() -> C6StateCaps {
    let mut phys = C6PhysCaps::test_caps();
    phys.max_frames = 4;
    phys.max_expressions = 4;
    phys.max_names = 4;
    phys.max_arms = 4;
    phys.max_children = 4;
    phys.max_value_slots = 2;
    phys.max_constants = 4;
    phys.max_data_nodes = 2;
    phys.max_env_cells = 2;
    phys.max_functions = 4;
    phys.max_arity = 2;
    phys.max_types = 4;
    phys.max_occurrences = 4;
    phys.max_occurrence_paths = 4;
    phys.max_parameters = 4;
    phys.max_patterns = 4;
    phys.max_pattern_children = 4;
    phys.max_constructors = 4;
    phys.max_ctor_fields = 4;
    phys.max_decl_templates = 4;
    phys.max_plan_nodes = 4;
    phys.max_plan_cases = 4;
    phys.max_field_paths = 4;
    phys.max_constant_fields = 4;
    phys.max_name_bytes = 64;
    phys.max_type_expr_nodes = 16;
    phys.max_template_bytes = 64;
    C6StateCaps {
        phys,
        max_data_field_slots: 2,
        max_helper_tasks: 0,
    }
}

fn run_checked_value_read(b: Budget, allow: Vec<Allow>) -> Report {
    let caps = read_caps();
    let mut program = ProgramSnapshot {
        entry: 0,
        expressions: Vec::new(),
        occurrences: Vec::new(),
        occurrence_paths: Vec::new(),
        children: Vec::new(),
        functions: Vec::new(),
        parameters: Vec::new(),
        arms: Vec::new(),
        patterns: Vec::new(),
        pattern_children: Vec::new(),
        types: Vec::new(),
        type_args: Vec::new(),
        constructors: Vec::new(),
        ctor_fields: Vec::new(),
        decl_param_counts: Vec::new(),
        decl_ctor_counts: Vec::new(),
        decl_templates: Vec::new(),
        plan_nodes: Vec::new(),
        plan_cases: Vec::new(),
        field_paths: Vec::new(),
        constants: Vec::new(),
        constant_fields: Vec::new(),
        names: Vec::new(),
    };
    program.constants = vec![ConstantSnapshot::U128 {
        marker_free: true,
        n: 42,
    }];
    let prog = encode_program(&program, &caps.phys).expect("program encodes");
    let g = c6_data_geometry(&caps).expect("geometry");
    let mut data = vec![0u128; g.total];
    data[0] = C6_PHYS_MAGIC;
    data[1] = C6_PHYS_VERSION;
    data[2] = DATA_SECTION_TAG;
    data[3] = 1;
    let base = g.data_base;
    data[base..base + DATA_ROW_LANES]
        .copy_from_slice(&encode_data_row(&DataSnapshot::U128 { n: 7 }));
    let expected: [u128; 23] = [
        1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 42, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    let scope = "cvr_test";
    let total = 3 + prog.len() + data.len() + 23;
    let mut ins = Vec::new();
    let mut tr = Vec::new();
    let mut ids = Vec::new();
    let t = public_lanes(scope, "", total, &mut ins, &mut tr, &mut ids);
    let block = emit_c6_checked_value_read_block(
        &C6CheckedValueReadInputs {
            gate: t[0].clone(),
            ref_kind_is_data: t[1].clone(),
            ref_index: t[2].clone(),
            pre_prog: t[3..3 + prog.len()].to_vec(),
            pre_data: t[3 + prog.len()..3 + prog.len() + data.len()].to_vec(),
            caps,
        },
        scope,
    )
    .expect("emit checked value read");
    let outputs = vec![
        block.selected_found.clone(),
        block.is_data.clone(),
        block.kind_code.clone(),
        block.is_uint.clone(),
        block.is_bool.clone(),
        block.is_nil.clone(),
        block.is_cons.clone(),
        block.is_leaf.clone(),
        block.is_branch.clone(),
        block.is_constructor.clone(),
        block.is_marker.clone(),
        block.marker_free_present.clone(),
        block.marker_free.clone(),
        block.uint_present.clone(),
        block.uint_value.clone(),
        block.bool_present.clone(),
        block.bool_value.clone(),
        block.ctor_present.clone(),
        block.type_id.clone(),
        block.ctor_id.clone(),
        block.slice_present.clone(),
        block.slice_start.clone(),
        block.slice_len.clone(),
    ];
    ins.extend(block.instructions);
    let exp_ops = &t[3 + prog.len() + data.len()..];
    for (out, exp) in outputs.into_iter().zip(exp_ops.iter()) {
        ins.push(Instruction::ConstrainEq {
            a: out,
            b: exp.clone(),
        });
    }
    ins.push(Instruction::Impact {
        guard: Operand::Immediate(Fr::from(1u64)),
        inputs: tr,
    });
    let ir = load_ir(ins, Vec::new());
    let mut statement = vec![Fr::from(1u64), Fr::from(0u64), Fr::from(0u64)];
    statement.extend(prog.iter().map(|&v| Fr::from(v)));
    statement.extend(data.iter().map(|&v| Fr::from(v)));
    statement.extend(expected.iter().map(|&v| Fr::from(v)));
    run_named(
        "c6_checked_value_read",
        &ir,
        &ids,
        &statement,
        Some(16),
        &config(b, allow),
    )
}

// ------------------------------------------- c6_scalar_failure_step

fn sf_limits() -> C6HostLimits {
    C6HostLimits {
        max_total_lanes: 1 << 20,
        max_state_lanes: 1 << 20,
        max_metadata_bytes: 8192,
        max_string_bytes: 512,
        max_width_entries: 64,
    }
}

fn sf_bounds(frames: usize, max_hw: u128) -> BoundsC {
    let mut b = BoundsC {
        frames,
        ..BoundsC::default()
    };
    b.code_rows = 16;
    b.function_rows = 8;
    b.code_child_slots = 16;
    b.parameter_slots = 16;
    b.pattern_rows = 16;
    b.match_plan_rows = 16;
    b.match_arm_rows = 8;
    b.type_rows = 16;
    b.constructor_rows = 16;
    b.constructor_field_slots = 16;
    b.max_arity = 4;
    b.constant_nodes = 16;
    b.constant_field_slots = 16;
    b.data_nodes = 2;
    b.data_field_slots = 2;
    b.environment_cells = 2;
    b.helper_tasks = 2;
    b.serialization_bytes = 1_000_000;
    b.max_helper_work = max_hw;
    b.step_cap = 1_000_000;
    b.max_reference_work = 1_000_000;
    b
}

fn sf_identity() -> IdentityC {
    IdentityC {
        profile_id: 2,
        grammar_domain: GrammarDomain::GenericNominalExtensionReserved,
        finance: FinanceInput::Absent,
        layout_version: ("c6".into(), "1".into()),
        layout_digest: DigestC::Unselected,
        bounds_digest: DigestC::Selected {
            format: ("f".into(), "1".into()),
            bytes: vec![1, 2],
        },
        field_id: Some(("fid".into(), "".into())),
        program_membership_digest: DigestC::Unselected,
    }
}

fn sf_config() -> C6HostConfig {
    C6HostConfig {
        layout: LayoutSnapshot {
            layout_version: ("lv".into(), "1".into()),
            field_id: None,
            widths: None,
            scalar_layout_id: None,
            bounds: sf_bounds(3, 0),
            capacities: LayoutCapacitiesC {
                name_rows: 16,
                name_bytes: 256,
                occurrence_path_slots: 32,
                value_slots: 2,
            },
            boundary_capacities_present: false,
        },
        ext: None,
        budgets: mori_core::sherlock::helper_machine::c6_phys::C6CodecBudgets {
            type_expr_nodes: 0,
            template_bytes: 0,
        },
    }
}

fn sf_sample(helper_work: u128, max_hw: u128) -> HostSnapshot {
    let cfg = sf_config();
    let id = sf_identity();
    HostSnapshot {
        admission: AdmissionSnapshot {
            identity: id.clone(),
            bounds: sf_bounds(3, max_hw),
            view: ProgramSnapshot {
                entry: 0,
                expressions: vec![
                    ExpressionSnapshot::Literal { constant: 0 },
                    ExpressionSnapshot::Binary {
                        op: BinaryOp::Add,
                        left: 0,
                        right: 0,
                    },
                ],
                occurrences: Vec::new(),
                occurrence_paths: Vec::new(),
                children: Vec::new(),
                functions: Vec::new(),
                parameters: Vec::new(),
                arms: Vec::new(),
                patterns: Vec::new(),
                pattern_children: Vec::new(),
                types: Vec::new(),
                type_args: Vec::new(),
                constructors: Vec::new(),
                ctor_fields: Vec::new(),
                decl_param_counts: Vec::new(),
                decl_ctor_counts: Vec::new(),
                decl_templates: Vec::new(),
                plan_nodes: Vec::new(),
                plan_cases: Vec::new(),
                field_paths: Vec::new(),
                constants: vec![ConstantSnapshot::U128 {
                    marker_free: true,
                    n: u128::MAX,
                }],
                constant_fields: Vec::new(),
                names: vec!["x".into()],
            },
            layout: cfg.layout.clone(),
        },
        image: ImageSnapshot {
            run_id: RunIdC {
                identity: id,
                fuel_input: RunIntC::Nat(64),
                stack_limit_input: RunIntC::Nat(2),
            },
            control: ControlC::Return {
                value: RefC::Data(0),
            },
            control_origin: 1,
            env_root: Some(0),
            stack_limit: 2,
            outcome: OutcomeC::Running,
            initial_fuel: 64,
            remaining_work: 10,
            consumed_work: 5,
            helper_work,
            helper_visits: 1,
            reference_cursor: 7,
            alloc: AllocCounters {
                data_nodes: 1,
                data_fields: 1,
                environment_cells: 2,
            },
            active: ActiveLengths {
                data: 1,
                data_fields: 1,
                env: 2,
                value_slots: 1,
            },
            helper: Some(HelperControlC::Scalar {
                common: HelperCommon {
                    site: SiteTag::RetRight,
                    origin_reference_cursor: 7,
                    parent_resume: None,
                },
                mode: ScalarMode::Add,
                operand_a: RefC::Const(0),
                operand_b: Some(RefC::Data(0)),
                expected: None,
                phase: ScalarPhase::Done,
                reg_a: u128::MAX,
                staged: ScalarStaged::PrimError { code: 10 },
            }),
            result: None,
            seg_base: SegBase::genesis_uncut(),
            data: vec![DataSnapshot::U128 { n: 1 }],
            data_fields: vec![RefC::Const(0)],
            env: vec![
                EnvCellC {
                    binder: 0,
                    value: RefC::Const(0),
                    previous: None,
                },
                EnvCellC {
                    binder: 0,
                    value: RefC::Data(0),
                    previous: Some(0),
                },
            ],
            frames: vec![FrameRowC::RightF {
                op: BinaryOp::Add,
                left: RefC::Const(0),
                origin: 1,
            }],
            tasks: Vec::new(),
            value_slots: vec![RefC::Data(0)],
        },
    }
}

fn sf_applied_post(pre: &HostSnapshot) -> HostSnapshot {
    let mut post = pre.clone();
    post.image.helper = None;
    post.image.result = Some(HelperResultC::ScalarError { code: 10 });
    post.image.remaining_work -= 1;
    post.image.consumed_work += 1;
    post.image.helper_work += 1;
    post
}

fn run_scalar_failure_step(b: Budget, allow: Vec<Allow>) -> Report {
    use mori_backend::sherlock_machine::c6_push_step::C6PushHostPrefixLanes;
    let cfg = sf_config();
    let limits = sf_limits();
    let pre = sf_sample(2, 100);
    let post = sf_applied_post(&pre);
    let w = witness_from_host_pair(true, &pre, &post, &cfg, &limits).expect("witness");
    let prefix = C6PushHostPrefixLanes::from_host_snapshot(&pre, &cfg, &limits).expect("prefix");
    let params = w.source_params().clone();
    let g = params.geometry();
    let meta_len = prefix.meta.len();
    let prog_len = prefix.prog.len();

    let scope = "c6_sf_test";
    let mut ins = Vec::new();
    let mut tr = Vec::new();
    let mut ids = Vec::new();
    let mut lane = |label: &str, n: usize, ins: &mut Vec<Instruction>| {
        public_lanes(scope, label, n, ins, &mut tr, &mut ids)
    };
    let enable = lane("enable", 1, &mut ins).remove(0);
    let disp = lane("disposition_is_refused", 1, &mut ins).remove(0);
    let schema = lane("host_schema", 1, &mut ins).remove(0);
    let carrier = lane("carrier_version", 1, &mut ins).remove(0);
    let pre_meta = lane("pre_meta", meta_len, &mut ins);
    let pre_prog = lane("pre_prog", prog_len, &mut ins);
    let pre_ctx: [Operand; CTX_LANES] = lane("pre_ctx", CTX_LANES, &mut ins)
        .try_into()
        .expect("ctx lanes");
    let pre_image = lane("pre_image", g.image_total_lanes, &mut ins);
    let pre_data = lane("pre_data", g.data_total_lanes, &mut ins);
    let pre_helper = lane("pre_helper", g.helper_total_lanes, &mut ins);
    let post_meta = lane("post_meta", meta_len, &mut ins);
    let post_prog = lane("post_prog", prog_len, &mut ins);
    let post_ctx: [Operand; CTX_LANES] = lane("post_ctx", CTX_LANES, &mut ins)
        .try_into()
        .expect("ctx lanes");
    let post_image = lane("post_image", g.image_total_lanes, &mut ins);
    let post_data = lane("post_data", g.data_total_lanes, &mut ins);
    let post_helper = lane("post_helper", g.helper_total_lanes, &mut ins);
    let block = emit_c6_scalar_failure_step_block(
        &C6ScalarFailureStepInputs {
            enable,
            disposition_is_refused_helper_work: disp,
            host_schema: schema,
            carrier_version: carrier,
            geometry: g,
            meta_bounds_frames_offset: prefix.meta_bounds_frames_offset,
            source_params: params,
            pre_meta,
            pre_prog,
            pre_ctx,
            pre_image,
            pre_data,
            pre_helper,
            post_meta,
            post_prog,
            post_ctx,
            post_image,
            post_data,
            post_helper,
        },
        scope,
    )
    .expect("emit scalar failure step");
    ins.extend(block.instructions);
    ins.push(Instruction::Impact {
        guard: Operand::Immediate(Fr::from(1u64)),
        inputs: tr,
    });
    let ir = load_ir(ins, Vec::new());
    let statement = w.to_statement_frs();
    assert_eq!(statement[2], Fr::from(C6_HOST_SCHEMA));
    run_named(
        "c6_scalar_failure_step",
        &ir,
        &ids,
        &statement,
        Some(18),
        &config(b, allow),
    )
}

// ------------------------------------------------------------------ tests

#[test]
fn data_canonical_mutations_are_rejected() {
    let b = Budget {
        cells: 24,
        mutations: 96,
        swaps: 4,
    };
    assert_sound(&run_data_canonical(b, decomposition_padding()));
}

#[test]
fn checked_value_read_mutations_are_rejected() {
    let b = Budget {
        cells: 3,
        mutations: 12,
        swaps: 0,
    };
    assert_sound(&run_checked_value_read(b, decomposition_padding()));
}

#[test]
fn scalar_failure_step_mutations_are_rejected() {
    let b = Budget {
        cells: 1,
        mutations: 2,
        swaps: 0,
    };
    assert_sound(&run_scalar_failure_step(b, decomposition_padding()));
}

#[test]
#[ignore = "deep sweep, many minutes; run explicitly with --ignored"]
fn deep_data_canonical_mutations_are_rejected() {
    let b = Budget {
        cells: 400,
        mutations: 1600,
        swaps: 40,
    };
    assert_sound(&run_data_canonical(b, decomposition_padding()));
}

#[test]
#[ignore = "deep sweep, many minutes; run explicitly with --ignored"]
fn deep_checked_value_read_mutations_are_rejected() {
    let b = Budget {
        cells: 60,
        mutations: 240,
        swaps: 4,
    };
    assert_sound(&run_checked_value_read(b, decomposition_padding()));
}

#[test]
#[ignore = "deep sweep, an hour or more; run explicitly with --ignored"]
fn deep_scalar_failure_step_mutations_are_rejected() {
    let b = Budget {
        cells: 20,
        mutations: 80,
        swaps: 2,
    };
    assert_sound(&run_scalar_failure_step(b, decomposition_padding()));
}

// ------------------------------------------------ positive control

mod control {
    use midnight_proofs::circuit::{Layouter, SimpleFloorPlanner, Value};
    use midnight_proofs::plonk::{
        Advice, Circuit, Column, ConstraintSystem, Constraints, Error, Expression, Selector,
    };
    use midnight_proofs::poly::Rotation;
    use midnight_transient_crypto::curve::outer::Scalar;
    use mori_backend::qa_mutation::{Config, run_suite};

    #[derive(Clone)]
    struct Cfg {
        a: Column<Advice>,
        b: Column<Advice>,
        free: Column<Advice>,
        q: Selector,
    }

    /// Proves `a * b = 6` and also assigns one advice cell, `free`, that no
    /// constraint touches. The oracle must flag exactly that cell.
    struct Leaky;

    impl Circuit<Scalar> for Leaky {
        type Config = Cfg;
        type FloorPlanner = SimpleFloorPlanner;
        type Params = ();

        fn without_witnesses(&self) -> Self {
            Leaky
        }

        fn configure(meta: &mut ConstraintSystem<Scalar>) -> Cfg {
            let a = meta.advice_column();
            let b = meta.advice_column();
            let free = meta.advice_column();
            let q = meta.selector();
            meta.create_gate("a*b=6", |meta| {
                let a = meta.query_advice(a, Rotation::cur());
                let b = meta.query_advice(b, Rotation::cur());
                Constraints::with_selector(
                    q,
                    vec![a * b - Expression::Constant(Scalar::from(6u64))],
                )
            });
            Cfg { a, b, free, q }
        }

        fn synthesize(&self, cfg: Cfg, mut layouter: impl Layouter<Scalar>) -> Result<(), Error> {
            layouter.assign_region(
                || "leaky",
                |mut region| {
                    cfg.q.enable(&mut region, 0)?;
                    region.assign_advice(|| "a", cfg.a, 0, || Value::known(Scalar::from(2u64)))?;
                    region.assign_advice(|| "b", cfg.b, 0, || Value::known(Scalar::from(3u64)))?;
                    region.assign_advice(
                        || "free",
                        cfg.free,
                        0,
                        || Value::known(Scalar::from(99u64)),
                    )?;
                    Ok(())
                },
            )
        }
    }

    #[test]
    fn oracle_flags_an_unconstrained_cell_and_only_it() {
        let cfg = Config {
            seed: 1,
            max_cells: None,
            max_mutations: None,
            swaps: 0,
            allow: Vec::new(),
        };
        let report = run_suite("leaky_control", &Leaky, &[], &cfg).expect("valid witness");
        assert_eq!(report.cells_total, 3);
        // Column 2 (`free`) accepts every kind of mutation; a and b none.
        assert!(
            report.accepted.iter().all(|a| a.column == 2 && a.row == 0),
            "unexpected accepted mutations:\n{}",
            report.describe_accepted()
        );
        assert_eq!(report.accepted.len(), 4, "{}", report.describe_accepted());
        assert!(report.rejected >= 8, "{}", report.summary());
    }
}
