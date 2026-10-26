//! TEMP s6 fold step-0 reproducer (not committed).
use mori_backend::sherlock_machine::c6_eval_schedule_consumer::{
    build_c6_eval_schedule_circuit, check_c6_eval_schedule_circuit, witness_from_host_pair,
};
use mori_backend::sherlock_machine::c6_eval_schedule_step::EVAL_SCHEDULE_RULES;
use mori_core::sherlock::helper_machine::c6_phys::C6CodecBudgets;
use mori_core::sherlock::helper_machine::c6_phys_host::{C6HostConfig, C6HostLimits};
use mori_core::sherlock::helper_machine::complete_observation::capture;
use mori_core::sherlock::helper_machine::shared::owner_host::host_owner_engine;
use mori_core::sherlock::helper_machine::shared::{OwnerEngine, StepResult};
use mori_core::sherlock_compile_artifact::draft_execution::draft_genesis;

const CONFIG: &str = include_str!("../src/sherlock_machine/testdata/dispatch/config.json");
const FOLD: &str = "language \"moriarty-sherlock-draft/1\";\nprogram P {\n  fn fold(t: Tree): UInt { match t { leaf(x) => x, branch(l, r) => fold(l) + fold(r) } }\n  entry fold(branch(leaf(4), leaf(5)));\n}\n";

fn limits() -> C6HostLimits {
    C6HostLimits {
        max_total_lanes: 1 << 20,
        max_state_lanes: 1 << 20,
        max_metadata_bytes: 8192,
        max_string_bytes: 512,
        max_width_entries: 64,
    }
}

#[test]
fn fold_step0_diag() {
    let g = draft_genesis(FOLD, CONFIG)
        .unwrap_or_else(|f| panic!("genesis: {}", serde_json::to_string(&f).unwrap()));
    let cfg = C6HostConfig::from_retained_layout(
        &g.layout,
        C6CodecBudgets {
            type_expr_nodes: 128,
            template_bytes: 4096,
        },
    );
    let engine = host_owner_engine();
    let mut image = g.image;
    for i in 0..3 {
        let pre = capture(&g.admitted, &image).expect("capture pre");
        match engine.step(&g.admitted, &mut image).expect("owner step") {
            StepResult::Terminal => {
                println!("terminal at {i}");
                break;
            }
            StepResult::Event { event } => {
                let post = capture(&g.admitted, &image).expect("capture post");
                let fired = event.rule_id;
                println!(
                    "step {i}: fired {fired:?} eval={}",
                    EVAL_SCHEDULE_RULES.contains(&fired)
                );
                if i == 0 {
                    let w = witness_from_host_pair(true, &pre, &post, &cfg, &limits())
                        .expect("witness");
                    let diff: Vec<usize> = (0..w.pre_image.len())
                        .filter(|&k| w.pre_image[k] != w.post_image[k])
                        .collect();
                    println!("image diff lanes: {diff:?}");
                    for k in &diff {
                        println!("  lane {k}: {} -> {}", w.pre_image[*k], w.post_image[*k]);
                    }
                    let ddiff: Vec<usize> = (0..w.pre_data.len())
                        .filter(|&k| w.pre_data[k] != w.post_data[k])
                        .collect();
                    println!(
                        "data diff lanes ({}): {:?}",
                        ddiff.len(),
                        &ddiff[..ddiff.len().min(20)]
                    );
                    let hdiff: Vec<usize> = (0..w.pre_helper.len())
                        .filter(|&k| w.pre_helper[k] != w.post_helper[k])
                        .collect();
                    println!("helper diff lanes: {hdiff:?}");
                    let mut c = build_c6_eval_schedule_circuit(
                        fired,
                        w.source_params.clone(),
                        w.host_prefix.meta.len(),
                        w.host_prefix.prog.len(),
                        w.host_prefix.meta_bounds_frames_offset,
                        "evalsched-foldstep0",
                    )
                    .expect("build");
                    let _ = &mut c;
                    match check_c6_eval_schedule_circuit(&c, &w) {
                        Ok(_) => println!("check: OK"),
                        Err(e) => println!("check: ERR {e}"),
                    }
                }
            }
        }
    }
}
