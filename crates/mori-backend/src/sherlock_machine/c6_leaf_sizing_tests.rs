//! U023 synthesis-only decision gate. No SRS, keygen or proof production.

#[test]
fn qualification_leaf_shape() {
    let (inputs, head) = qualification_inputs();
    let rules = fragment_rules();
    assert!(rules.contains(&RuleIdC::KAdminPublishValue));
    assert_eq!(
        rules
            .iter()
            .filter(|r| format!("{r:?}").starts_with("BHelperValidate"))
            .count(),
        5
    );
    let leaf = leaf(&inputs, &head, &[], &[]);
    assert_eq!(
        leaf.instructions
            .iter()
            .filter(|i| matches!(i, Instruction::PublicInput { .. }))
            .count(),
        5
    );
    assert!(
        leaf.instructions
            .iter()
            .any(|i| matches!(i, Instruction::TransientHash { .. }))
    );
}

#[test]
#[ignore = "synthesizes the complete C6 union; no keygen or proving"]
fn qualification_leaf_sizing() {
    measure();
}

use super::c6_step_kit::C6StepInputs;
use super::*;
use crate::sherlock_c6_child::{DigestSections, StateOutcome, c6_child_relation, rule_lane};
use midnight_proofs::dev::cost_model::CircuitModel;
use midnight_transient_crypto::curve::Fr;
use midnight_zk_stdlib::{Relation, cost_model};
use midnight_zkir_v3::ir::{IrMinorVersion, Operand};
use midnight_zkir_v3::ir_types::IrType;
use midnight_zkir_v3::{Identifier, Instruction, IrSource};
use mori_core::sherlock::helper_machine::{
    c6_phys::{C6CodecBudgets, L_OUTCOME},
    c6_phys_host::{C6HostConfig, C6HostLimits},
    complete_observation::capture,
    shared::RuleIdC,
};
use mori_core::sherlock_compile_artifact::draft_execution::draft_genesis;
use std::sync::Arc;

const CONFIG: &str =
    include_str!("../../../../docs/sherlock/qualification/qualification-host-config.json");
const PROGRAMS: [&str; 4] = [
    "language \"moriarty-sherlock-draft/1\"; program P { entry 7; }",
    "language \"moriarty-sherlock-draft/1\"; program P { entry 3 + 4; }",
    include_str!("../../../../docs/sherlock/qualification/programs/fact3.mori"),
    include_str!("../../../../docs/sherlock/qualification/programs/list123.mori"),
];
fn imm(n: u128) -> Operand {
    Operand::Immediate(Fr::from(n))
}
fn var(name: &str) -> Operand {
    Operand::Variable(Identifier(format!("%sizing.{name}")))
}
fn input(ins: &mut Vec<Instruction>, name: &str) -> Operand {
    let output = Identifier(format!("%sizing.{name}"));
    ins.push(Instruction::PublicInput {
        guard: None,
        val_t: IrType::Native,
        output: output.clone(),
    });
    Operand::Variable(output)
}
fn lanes(ins: &mut Vec<Instruction>, name: &str, n: usize) -> Vec<Operand> {
    (0..n).map(|j| input(ins, &format!("{name}.{j}"))).collect()
}
fn qualification_inputs() -> (C6StepInputs, Vec<Instruction>) {
    let limits = C6HostLimits {
        max_total_lanes: 1 << 20,
        max_state_lanes: 1 << 20,
        max_metadata_bytes: 8192,
        max_string_bytes: 512,
        max_width_entries: 64,
    };
    let mut witnesses = Vec::new();
    for source in PROGRAMS {
        let g = draft_genesis(source, CONFIG)
            .unwrap_or_else(|e| panic!("genesis: {}", serde_json::to_string(&e).unwrap()));
        let cfg = C6HostConfig::from_retained_layout(
            &g.layout,
            C6CodecBudgets {
                type_expr_nodes: 128,
                template_bytes: 4096,
            },
        );
        let snap = capture(&g.admitted, &g.image).unwrap();
        witnesses.push(
            c6_step_harness::step_witness_from_host_pair(true, &snap, &snap, &cfg, &limits)
                .unwrap(),
        );
    }
    let w = witnesses
        .iter()
        .max_by_key(|w| w.host_prefix.prog.len())
        .unwrap();
    let mut ins = Vec::new();
    let host_schema = input(&mut ins, "schema");
    let carrier_version = input(&mut ins, "version");
    let pre_meta = lanes(&mut ins, "meta", w.host_prefix.meta.len());
    let pre_prog = lanes(&mut ins, "prog", w.host_prefix.prog.len());
    let pre_ctx: [Operand; mori_core::sherlock::helper_machine::c6_phys_host::CTX_LANES] =
        lanes(&mut ins, "ctx", w.host_prefix.ctx.len())
            .try_into()
            .unwrap();
    let pre_image = lanes(&mut ins, "pre.image", w.pre_image.len());
    let pre_data = lanes(&mut ins, "pre.data", w.pre_data.len());
    let pre_helper = lanes(&mut ins, "pre.helper", w.pre_helper.len());
    let post_image = lanes(&mut ins, "post.image", w.post_image.len());
    let post_data = lanes(&mut ins, "post.data", w.post_data.len());
    let post_helper = lanes(&mut ins, "post.helper", w.post_helper.len());
    (
        C6StepInputs {
            enable: imm(1),
            host_schema,
            carrier_version,
            geometry: w.source_params.geometry(),
            meta_bounds_frames_offset: w.host_prefix.meta_bounds_frames_offset,
            source_params: w.source_params.clone(),
            post_meta: pre_meta.clone(),
            post_prog: pre_prog.clone(),
            post_ctx: pre_ctx.clone(),
            pre_meta,
            pre_prog,
            pre_ctx,
            pre_image,
            pre_data,
            pre_helper,
            post_image,
            post_data,
            post_helper,
        },
        ins,
    )
}

fn fragment_rules() -> Vec<RuleIdC> {
    c6_dispatch_step::supported_rules()
        .into_iter()
        .filter(|r| has_fragment(*r))
        .collect()
}
fn has_fragment(r: RuleIdC) -> bool {
    c6_admin_publish_step::admin_publish_supported(r)
        || c6_eval_schedule_step::eval_schedule_supported(r)
        || c6_call_bind_step::call_bind_supported(r)
        || c6_call_helper_step::call_helper_supported(r)
        || c6_append_helper_step::append_helper_supported(r)
        || c6_bind_helper_step::bind_helper_supported(r)
        || c6_match_helper_step::match_helper_supported(r)
        || c6_scalar_construct_step::scalar_construct_supported(r)
        || c6_equal_helper_step::equal_helper_supported(r)
        || c6_validate_helper_step::validate_helper_supported(r)
        || c6_legacy_control_step::legacy_control_supported(r)
        || matches!(
            r,
            RuleIdC::KMachinePushAccept
                | RuleIdC::KMachinePushRefuse
                | RuleIdC::KMachineReturnCallValid
                | RuleIdC::KMachineReturnCallInvalid
                | RuleIdC::KMachineReturnLeft
                | RuleIdC::KMachineReturnRight
                | RuleIdC::KMachineReturnScope
                | RuleIdC::BHelperScalarEntry
                | RuleIdC::BHelperScalarTag
                | RuleIdC::BHelperScalarArithmetic
                | RuleIdC::BHelperScalarExit
                | RuleIdC::BHelperScalarFailure
                | RuleIdC::BHelperEqualEmpty
                | RuleIdC::BHelperValidateField
                | RuleIdC::XAdtEvalConstructNullary
        )
}
macro_rules! step_inputs {
    ($module:ident::$ty:ident, $i:ident $(, $extra:ident: $value:expr)*) => {
        $module::$ty {
            enable: $i.enable.clone(),
            host_schema: $i.host_schema.clone(),
            carrier_version: $i.carrier_version.clone(),
            geometry: $i.geometry,
            meta_bounds_frames_offset: $i.meta_bounds_frames_offset,
            source_params: $i.source_params.clone(),
            pre_meta: $i.pre_meta.clone(),
            pre_prog: $i.pre_prog.clone(),
            pre_ctx: $i.pre_ctx.clone(),
            pre_image: $i.pre_image.clone(),
            pre_data: $i.pre_data.clone(),
            pre_helper: $i.pre_helper.clone(),
            post_meta: $i.post_meta.clone(),
            post_prog: $i.post_prog.clone(),
            post_ctx: $i.post_ctx.clone(),
            post_image: $i.post_image.clone(),
            post_data: $i.post_data.clone(),
            post_helper: $i.post_helper.clone(),
            $($extra: $value,)*
        }
    };
}

fn fragment(rule: RuleIdC, i: &C6StepInputs, scope: &str) -> Vec<Instruction> {
    if c6_admin_publish_step::admin_publish_supported(rule) {
        return c6_admin_publish_step::emit_c6_admin_publish_block(rule, i, scope)
            .unwrap()
            .instructions;
    }
    if c6_eval_schedule_step::eval_schedule_supported(rule) {
        return c6_eval_schedule_step::emit_c6_eval_schedule_block(rule, i, scope)
            .unwrap()
            .instructions;
    }
    if c6_call_bind_step::call_bind_supported(rule) {
        return c6_call_bind_step::emit_c6_call_bind_block(rule, i, scope)
            .unwrap()
            .instructions;
    }
    if c6_call_helper_step::call_helper_supported(rule) {
        return c6_call_helper_step::emit_c6_call_helper_block(rule, i, scope)
            .unwrap()
            .instructions;
    }
    if c6_append_helper_step::append_helper_supported(rule) {
        return c6_append_helper_step::emit_c6_append_helper_block(rule, i, scope)
            .unwrap()
            .instructions;
    }
    if c6_bind_helper_step::bind_helper_supported(rule) {
        return c6_bind_helper_step::emit_c6_bind_helper_block(rule, i, scope)
            .unwrap()
            .instructions;
    }
    if c6_match_helper_step::match_helper_supported(rule) {
        return c6_match_helper_step::emit_c6_match_helper_block(rule, i, scope)
            .unwrap()
            .instructions;
    }
    if c6_scalar_construct_step::scalar_construct_supported(rule) {
        return c6_scalar_construct_step::emit_c6_scalar_construct_block(rule, i, scope)
            .unwrap()
            .instructions;
    }
    if c6_equal_helper_step::equal_helper_supported(rule) {
        return c6_equal_helper_step::emit_c6_equal_helper_block(rule, i, scope)
            .unwrap()
            .instructions;
    }
    if c6_validate_helper_step::validate_helper_supported(rule) {
        return c6_validate_helper_step::emit_c6_validate_helper_block(rule, i, scope)
            .unwrap()
            .instructions;
    }
    if c6_legacy_control_step::legacy_control_supported(rule) {
        return c6_legacy_control_step::emit_c6_legacy_control_block(rule, i, scope)
            .unwrap()
            .instructions;
    }
    match rule {
        RuleIdC::KMachinePushAccept | RuleIdC::KMachinePushRefuse => c6_push_step::emit_c6_push_step_block(&step_inputs!(c6_push_step::C6PushStepInputs, i , selector_is_accept: imm(u128::from(rule == RuleIdC::KMachinePushAccept))), scope).unwrap().instructions,
        RuleIdC::KMachineReturnCallValid | RuleIdC::KMachineReturnCallInvalid => c6_return_call_step::emit_c6_return_call_step_block(&step_inputs!(c6_return_call_step::C6ReturnCallStepInputs, i , selector_is_valid: imm(u128::from(rule == RuleIdC::KMachineReturnCallValid))), scope).unwrap().instructions,
        RuleIdC::KMachineReturnLeft => c6_return_left_step::emit_c6_return_left_step_block(&step_inputs!(c6_return_left_step::C6ReturnLeftStepInputs, i), scope).unwrap().instructions,
        RuleIdC::KMachineReturnRight => c6_return_right_step::emit_c6_return_right_step_block(&step_inputs!(c6_return_right_step::C6ReturnRightStepInputs, i), scope).unwrap().instructions,
        RuleIdC::KMachineReturnScope => c6_return_scope_step::emit_c6_return_scope_step_block(&step_inputs!(c6_return_scope_step::C6ReturnScopeStepInputs, i), scope).unwrap().instructions,
        RuleIdC::BHelperScalarEntry => c6_scalar_entry_step::emit_c6_scalar_entry_step_block(&step_inputs!(c6_scalar_entry_step::C6ScalarEntryStepInputs, i , disposition_is_refused_helper_work: imm(0)), scope).unwrap().instructions,
        RuleIdC::BHelperScalarTag => c6_scalar_tag_step::emit_c6_scalar_tag_step_block(&step_inputs!(c6_scalar_tag_step::C6ScalarTagStepInputs, i , disposition_is_refused_helper_work: imm(0)), scope).unwrap().instructions,
        RuleIdC::BHelperScalarArithmetic => c6_scalar_arithmetic_step::emit_c6_scalar_arithmetic_step_block(&step_inputs!(c6_scalar_arithmetic_step::C6ScalarArithmeticStepInputs, i , disposition_is_refused_helper_work: imm(0)), scope).unwrap().instructions,
        RuleIdC::BHelperScalarExit => c6_scalar_exit_step::emit_c6_scalar_exit_step_block(&step_inputs!(c6_scalar_exit_step::C6ScalarExitStepInputs, i , disposition_is_refused_helper_work: imm(0)), scope).unwrap().instructions,
        RuleIdC::BHelperScalarFailure => c6_scalar_failure_step::emit_c6_scalar_failure_step_block(&step_inputs!(c6_scalar_failure_step::C6ScalarFailureStepInputs, i , disposition_is_refused_helper_work: imm(0)), scope).unwrap().instructions,
        RuleIdC::BHelperEqualEmpty => c6_equal_empty_step::emit_c6_equal_empty_step_block(&step_inputs!(c6_equal_empty_step::C6EqualEmptyStepInputs, i , disposition_is_refused_helper_work: imm(0)), scope).unwrap().instructions,
        RuleIdC::BHelperValidateField => c6_validate_field_step::emit_c6_validate_field_step_block(&step_inputs!(c6_validate_field_step::C6ValidateFieldStepInputs, i , disposition_is_refused_helper_work: imm(0), disposition_is_refused_helper_tasks: imm(0)), scope).unwrap().instructions,
        RuleIdC::XAdtEvalConstructNullary => c6_construct_nullary_step::emit_c6_construct_nullary_step_block(&step_inputs!(c6_construct_nullary_step::C6ConstructNullaryStepInputs, i), scope).unwrap().instructions,
        _ => panic!("missing fragment {rule:?}"),
    }
}

fn gated_inputs(i: &C6StepInputs, gate: Operand, scope: &str) -> (C6StepInputs, Vec<Instruction>) {
    let mut out = i.clone();
    out.enable = gate.clone();
    let mut ins = Vec::new();
    for (name, pre, post) in [
        ("image", &i.pre_image, &mut out.post_image),
        ("data", &i.pre_data, &mut out.post_data),
        ("helper", &i.pre_helper, &mut out.post_helper),
    ] {
        for (j, (a, b)) in post.iter_mut().zip(pre).enumerate() {
            let output = Identifier(format!("%{scope}.mux.{name}.{j}"));
            ins.push(Instruction::CondSelect {
                bit: gate.clone(),
                a: a.clone(),
                b: b.clone(),
                output: output.clone(),
            });
            *a = Operand::Variable(output);
        }
    }
    (out, ins)
}

fn dispatch(i: &C6StepInputs, rules: &[RuleIdC]) -> Sharing {
    let mut shared = Sharing::default();
    let mut ins = Vec::new();
    let mut sum = imm(0);
    for rule in rules {
        let output = Identifier(format!("%sizing.gate.{rule:?}"));
        ins.push(Instruction::TestEq {
            a: var("rule"),
            b: Operand::Immediate(rule_lane(&format!("{rule:?}"))),
            output: output.clone(),
        });
        let gate = Operand::Variable(output);
        let total = Identifier(format!("%sizing.sum.{rule:?}"));
        ins.push(Instruction::Add {
            a: sum,
            b: gate.clone(),
            output: total.clone(),
        });
        sum = Operand::Variable(total);
        shared.extend(std::mem::take(&mut ins));
        shared.extend(
            c6_dispatch_step::emit_c6_dispatch_block(
                *rule,
                &C6DispatchInputs {
                    enable: gate,
                    caps: i.geometry.caps,
                    pre_prog: i.pre_prog.clone(),
                    pre_image: i.pre_image.clone(),
                    pre_data: i.pre_data.clone(),
                    pre_helper: i.pre_helper.clone(),
                },
                &format!("dispatch.{rule:?}"),
            )
            .unwrap()
            .instructions,
        );
    }
    if !rules.is_empty() {
        ins.push(Instruction::ConstrainEq { a: sum, b: imm(1) });
    }
    shared.extend(ins);
    shared
}

fn leaf(
    i: &C6StepInputs,
    head: &[Instruction],
    dispatch: &[Instruction],
    fragments: &[Instruction],
) -> IrSource {
    let mut body = head.to_vec();
    let published = head
        .iter()
        .map(|ins| match ins {
            Instruction::PublicInput { output, .. } => Operand::Variable(output.clone()),
            _ => unreachable!(),
        })
        .collect();
    for name in ["rule", "h"] {
        body.push(Instruction::PrivateInput {
            guard: None,
            val_t: IrType::Native,
            output: Identifier(format!("%sizing.{name}")),
        });
    }
    body.extend_from_slice(dispatch);
    body.extend_from_slice(fragments);
    body.push(Instruction::Impact {
        guard: imm(1),
        inputs: published,
    });
    let sections = DigestSections {
        prog_lanes: 2 + i.pre_meta.len() + i.pre_prog.len() + i.pre_ctx.len(),
        pre_lanes: i.pre_image.len() + i.pre_data.len() + i.pre_helper.len(),
        post_lanes: i.post_image.len() + i.post_data.len() + i.post_helper.len(),
    };
    let raw = IrSource {
        version: IrMinorVersion::V0,
        inputs: vec![],
        outputs: vec![],
        do_communications_commitment: false,
        instructions: Arc::new(body),
    };
    // Reuse the adapter's exact arity-4 carrier tree and SHK_SD outer hash.
    let mut leaf =
        c6_child_relation(&raw, sections, StateOutcome::RUNNING, StateOutcome::RUNNING).unwrap();
    let mut ins = leaf.instructions.as_ref().clone();
    // The outcome must vary at publication; resolve the UInt through the C6 reader.
    for (side, image, data) in [
        ("pre", &i.pre_image, &i.pre_data),
        ("post", &i.post_image, &i.post_data),
    ] {
        let tag = Identifier(format!("%c6.const.{side}.tag"));
        let value = Identifier(format!("%c6.const.{side}.value"));
        let reader = emit_c6_checked_value_read_block(
            &C6CheckedValueReadInputs {
                gate: Operand::Variable(tag.clone()),
                ref_kind_is_data: image[L_OUTCOME + 1].clone(),
                ref_index: image[L_OUTCOME + 2].clone(),
                pre_prog: i.pre_prog.clone(),
                pre_data: data.clone(),
                caps: i.geometry.caps,
            },
            &format!("outcome.{side}"),
        )
        .unwrap();
        let mut replacement = vec![
            Instruction::ConstrainEq {
                a: Operand::Variable(tag.clone()),
                b: image[L_OUTCOME].clone(),
            },
            Instruction::ConstrainToBoolean {
                val: Operand::Variable(tag.clone()),
            },
        ];
        replacement.extend(reader.instructions);
        replacement.push(Instruction::ConstrainEq {
            a: reader.uint_present,
            b: Operand::Variable(tag.clone()),
        });
        replacement.push(Instruction::ConstrainEq {
            a: Operand::Variable(value.clone()),
            b: reader.uint_value,
        });
        // Insert after the value input and remove only the adapter's outcome constants.
        ins.retain(|op| !matches!(op, Instruction::ConstrainEq { a: Operand::Variable(id), .. } if *id == tag || *id == value));
        let at = ins
            .iter()
            .position(
                |op| matches!(op, Instruction::PrivateInput { output, .. } if *output == value),
            )
            .unwrap()
            + 1;
        ins.splice(at..at, replacement);
    }
    for name in ["rule", "h"] {
        ins.push(Instruction::ConstrainEq {
            a: Operand::Variable(Identifier(format!("%c6.public.{name}"))),
            b: var(name),
        });
    }
    leaf.instructions = Arc::new(ins);
    leaf
}

fn minimum_model(ir: &IrSource) -> CircuitModel {
    let mut lo = 9;
    let mut hi = 25;
    while lo < hi {
        let k = (lo + hi) / 2;
        let m = cost_model(ir, Some(k));
        if m.k <= k {
            hi = k;
        } else {
            lo = k + 1;
        }
    }
    let m = cost_model(ir, Some(lo));
    assert!(m.k <= lo, "relation exceeds SDK sizing range");
    m
}
fn report(label: &str, m: &CircuitModel) {
    println!(
        "MODEL {label} k={} rows={} advice={} fixed={} lookups={} table={} unusable={} permutations={}",
        m.k,
        m.rows,
        m.advice_columns,
        m.fixed_columns,
        m.lookups,
        m.table_rows,
        m.nb_unusable_rows,
        m.permutations
    );
}
fn measure() {
    let (i, head) = qualification_inputs();
    let rules = fragment_rules();
    println!(
        "CARRIERS prefix={} image={} data={} helper={} rules={}",
        head.len() - 2 * (i.pre_image.len() + i.pre_data.len() + i.pre_helper.len()),
        i.pre_image.len(),
        i.pre_data.len(),
        i.pre_helper.len(),
        rules.len()
    );
    println!(
        "MISSING {:?}",
        c6_dispatch_step::supported_rules()
            .into_iter()
            .filter(|r| !has_fragment(*r))
            .collect::<Vec<_>>()
    );
    let mut shared = dispatch(&i, &rules);
    let dispatch_len = shared.ins.len();
    for r in &rules {
        let (gated, mux) =
            gated_inputs(&i, var(&format!("gate.{r:?}")), &format!("fragment.{r:?}"));
        let mut block = mux;
        block.extend(fragment(*r, &gated, &format!("fragment.{r:?}")));
        shared.extend(block);
        println!("SHARED {r:?} instructions={}", shared.ins.len());
    }
    let fragments = shared.ins;
    drop(shared.seen);
    drop(shared.external);
    drop(shared.inputs);
    let d = fragments[..dispatch_len].to_vec();
    let universal = attach(leaf(&i, &head, &[], &[]), head.len() + 2, fragments);
    println!("UNIVERSAL instructions={}", universal.instructions.len());
    let u = minimum_model(&universal);
    report("universal", &u);
    let baseline = cost_model(&leaf(&i, &head, &[], &[]), Some(u.k));
    report("digest-shell", &baseline);
    let dispatched = cost_model(
        &attach(leaf(&i, &head, &[], &[]), head.len() + 2, d),
        Some(u.k),
    );
    report("dispatch-shell", &dispatched);
    println!("DISPATCH rows={}", dispatched.rows - baseline.rows);
    drop(universal);
    let mut isolated_sum = 0;
    let mut family_k = 0;
    for r in rules {
        let mut shared = dispatch(&i, &[r]);
        let dispatch_len = shared.ins.len();
        let (gated, mut one) =
            gated_inputs(&i, var(&format!("gate.{r:?}")), &format!("fragment.{r:?}"));
        one.extend(fragment(r, &gated, &format!("fragment.{r:?}")));
        shared.extend(one);
        let one_dispatch = &shared.ins[..dispatch_len];
        let one_ir = attach(
            leaf(&i, &head, &[], &[]),
            head.len() + 2,
            shared.ins.clone(),
        );
        assert_eq!(one_ir.used_chips(), leaf(&i, &head, &[], &[]).used_chips());
        let m = minimum_model(&one_ir);
        family_k = family_k.max(m.k);
        report(&format!("{r:?}"), &m);
        let at_u = cost_model(&one_ir, Some(u.k));
        let without_fragment = cost_model(&leaf(&i, &head, &one_dispatch, &[]), Some(u.k));
        let contribution = at_u.rows - without_fragment.rows;
        isolated_sum += contribution;
        println!("CONTRIBUTION {r:?} rows={contribution}");
    }
    println!(
        "TOTAL isolated_fragments={isolated_sum} combined_fragments={} family_k={family_k}",
        u.rows - dispatched.rows
    );
}

#[test]
fn sizing_shares_only_identical_instructions() {
    let mut cse = Sharing::default();
    let a = Identifier("%a".into());
    let b = Identifier("%b".into());
    cse.extend(vec![
        Instruction::Add {
            a: var("x"),
            b: imm(1),
            output: a.clone(),
        },
        Instruction::Add {
            a: var("x"),
            b: imm(1),
            output: b.clone(),
        },
        Instruction::ConstrainEq {
            a: Operand::Variable(a),
            b: Operand::Variable(b),
        },
    ]);
    assert_eq!(cse.ins.len(), 2);
    assert!(matches!(&cse.ins[1], Instruction::ConstrainEq { a, b } if a == b));
}

#[derive(Default)]
struct Sharing {
    ins: Vec<Instruction>,
    seen: std::collections::HashMap<Vec<u8>, Vec<Identifier>>,
    external: std::collections::HashMap<Identifier, Operand>,
    inputs: std::collections::HashMap<Identifier, u64>,
    next: u64,
}
impl Sharing {
    fn extend(&mut self, instructions: Vec<Instruction>) {
        let mut aliases = self.external.clone();
        for mut ins in instructions {
            let mut key = Vec::new();
            let mut operand = |op: &mut Operand| {
                if let Operand::Variable(id) = op {
                    if let Some(replacement) = aliases.get(id) {
                        *op = replacement.clone();
                    }
                }
                match op {
                    Operand::Immediate(f) => {
                        key.push(0);
                        key.extend(f.as_le_bytes());
                    }
                    Operand::Variable(id) => {
                        key.push(1);
                        let n = if let Some(s) = id.0.strip_prefix("%sizing.cse.") {
                            s.parse::<u64>().unwrap()
                        } else {
                            *self.inputs.entry(id.clone()).or_insert_with(|| {
                                let n = self.next;
                                self.next += 1;
                                n
                            })
                        };
                        key.extend(n.to_le_bytes());
                    }
                }
            };
            let mut outputs = Vec::new();
            let opcode = match &mut ins {
                Instruction::Add { a, b, output } => {
                    operand(a);
                    operand(b);
                    outputs.push(output);
                    0
                }
                Instruction::Mul { a, b, output } => {
                    operand(a);
                    operand(b);
                    outputs.push(output);
                    1
                }
                Instruction::TestEq { a, b, output } => {
                    operand(a);
                    operand(b);
                    outputs.push(output);
                    2
                }
                Instruction::LessThan { a, b, bits, output } => {
                    operand(a);
                    operand(b);
                    outputs.push(output);
                    key.extend(bits.to_le_bytes());
                    3
                }
                Instruction::CondSelect { bit, a, b, output } => {
                    operand(bit);
                    operand(a);
                    operand(b);
                    outputs.push(output);
                    4
                }
                Instruction::Not { a, output } => {
                    operand(a);
                    outputs.push(output);
                    5
                }
                Instruction::Neg { a, output } => {
                    operand(a);
                    outputs.push(output);
                    6
                }
                Instruction::Copy { val, output } => {
                    operand(val);
                    outputs.push(output);
                    7
                }
                Instruction::ConstrainEq { a, b } => {
                    operand(a);
                    operand(b);
                    8
                }
                Instruction::ConstrainToBoolean { val } => {
                    operand(val);
                    9
                }
                Instruction::ConstrainBits { val, bits } => {
                    operand(val);
                    key.extend(bits.to_le_bytes());
                    10
                }
                Instruction::Assert { cond } => {
                    operand(cond);
                    11
                }
                Instruction::DivModPowerOfTwo {
                    val,
                    bits,
                    outputs: os,
                } => {
                    operand(val);
                    outputs.extend(os);
                    key.extend(bits.to_le_bytes());
                    12
                }
                _ => panic!("unexpected fragment instruction {ins:?}"),
            };
            key.push(opcode);
            if let Some(existing) = self.seen.get(&key) {
                for (out, canonical) in outputs.iter().zip(existing) {
                    aliases.insert((*out).clone(), Operand::Variable(canonical.clone()));
                }
            } else {
                let mut canonical = Vec::new();
                for out in outputs {
                    let id = Identifier(format!("%sizing.cse.{}", self.next));
                    self.next += 1;
                    aliases.insert(out.clone(), Operand::Variable(id.clone()));
                    *out = id.clone();
                    canonical.push(id);
                }
                self.seen.insert(key, canonical);
                self.ins.push(ins);
            }
        }
        self.external.extend(aliases.into_iter().filter(|(id, _)| {
            id.0.starts_with("%sizing.gate.") || id.0.starts_with("%sizing.sum.")
        }));
    }
}

fn attach(mut shell: IrSource, at: usize, body: Vec<Instruction>) -> IrSource {
    let mut instructions = Arc::try_unwrap(shell.instructions).unwrap();
    let tail = instructions.split_off(at);
    instructions.reserve_exact(body.len() + tail.len());
    instructions.extend(body);
    instructions.extend(tail);
    shell.instructions = Arc::new(instructions);
    shell
}

#[test]
#[ignore = "synthesis-only dispatch reproducer"]
fn qualification_dispatch_sizing() {
    let (i, head) = qualification_inputs();
    let shared = dispatch(&i, &fragment_rules());
    println!("DISPATCH instructions={}", shared.ins.len());
    let ins = shared.ins;
    drop(shared.seen);
    drop(shared.external);
    drop(shared.inputs);
    let ir = attach(leaf(&i, &head, &[], &[]), head.len() + 2, ins);
    report("dispatch-only", &cost_model(&ir, Some(17)));
}
