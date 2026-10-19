//! New explicit ordinary positive consumer; not autodiscovered by default Cargo.
//! Three fixed normal inputs exercise the real admitted owner only once each.
//! On those actual saved snapshots, compare uncached/cold/warm/last-use outputs.
//! This is variant preservation + literal result checking, not an independent
//! complete B-prefix/clock oracle. No old protected oracle/assertion is copied.
//! The QA sink retains one previous raw boundary and counts observations; it
//! does not retain a trace history or claim measured total memory/time savings.
use mori_core::sherlock::helper_machine::{complete_observation as co, witness_plan as wp};
use mori_core::sherlock::helper_machine::shared::*;
use mori_core::sherlock::helper_machine::shared::owner_host::host_owner_engine;
use std::collections::BTreeSet;

fn tooling<T>(result: Result<T, Tooling>) -> T {
    match result {
        Ok(value) => value,
        Err(Tooling::AdmissionRejected { reason, detail, locator }) =>
            panic!("normal admission {reason:?}: {detail} at {locator:?}"),
        Err(Tooling::CapacityNotAdmitted { kind, needed, limit, locator }) =>
            panic!("normal capacity {kind:?}: {needed:?}/{limit} at {locator:?}"),
        Err(Tooling::UnsupportedFeature { feature, locator }) =>
            panic!("normal feature {feature:?} at {locator:?}"),
    }
}
// Same explicit positive case capacities as the adopted baseline input CONFIG.
// These are fixture inputs, not new admission/resource policy or defaults.
fn layout() -> (BoundsC, LayoutSchemaC) {
    let bounds = BoundsC {
        code_rows: 64, function_rows: 4, code_child_slots: 16, parameter_slots: 8,
        pattern_rows: 8, match_plan_rows: 8, match_arm_rows: 8, type_rows: 8,
        constructor_rows: 8, constructor_field_slots: 16, max_arity: 8,
        constant_nodes: 16, constant_field_slots: 16, data_nodes: 64,
        data_field_slots: 64, environment_cells: 64, frames: 64, helper_tasks: 64,
        serialization_bytes: 65536, max_reference_work: 4096,
        max_helper_work: 4096, step_cap: 4096,
    };
    let schema = LayoutSchemaC {
        layout_version: ("moriarty".into(), "sherlock-helper-microstep-hostonly/1".into()),
        field_id: None, widths: None, scalar_layout_id: None, bounds,
        capacities: LayoutCapacitiesC {
            name_rows: 32, name_bytes: 256, occurrence_path_slots: 256, value_slots: 4096,
        }, ext_capacities: None, boundary_capacities_present: false,
    };
    (bounds, schema)
}
// Only actual source AST construction below; no source evaluation/substitution.
fn var(name: &str) -> CoreExpr { CoreExpr::Var { name: name.into() } }
fn uint(n: u128) -> CoreExpr {
    CoreExpr::Literal { value: CoreValue::UInt { n: i128_class::KIntC::InRange(n) } }
}
fn binary(op: BinaryOp, left: CoreExpr, right: CoreExpr) -> CoreExpr {
    CoreExpr::Binary { op, left: Box::new(left), right: Box::new(right) }
}
fn call(name: &str, argument: CoreExpr) -> CoreExpr {
    CoreExpr::Call { name: name.into(), args: vec![argument] }
}
fn program(name: &str, binder: &str, sort: SortTag, body: CoreExpr, argument: CoreExpr) -> CoreProgram {
    CoreProgram {
        functions: vec![CoreFunction {
            name: name.into(), parameters: vec![CoreParameter { name: binder.into(), sort }],
            return_type: SortTag::UInt, body,
        }], entry: call(name, argument),
    }
}
fn factorial() -> CoreProgram {
    program("fact", "n", SortTag::UInt, CoreExpr::If {
        cond: Box::new(binary(BinaryOp::Eq, var("n"), uint(0))),
        then_expr: Box::new(uint(1)),
        else_expr: Box::new(binary(BinaryOp::Mul, var("n"),
            call("fact", binary(BinaryOp::Sub, var("n"), uint(1))))),
    }, uint(3))
}
fn list_sum() -> CoreProgram {
    program("sum_list", "xs", SortTag::List, CoreExpr::Match {
        scrutinee: Box::new(var("xs")), arms: vec![
            CoreArm { pattern: CorePattern::PNil, body: uint(0) },
            CoreArm { pattern: CorePattern::PCons { x: "h".into(), y: "t".into() },
                body: binary(BinaryOp::Add, var("h"), call("sum_list", var("t"))) },
        ],
    }, CoreExpr::Literal { value: CoreValue::List {
        elements: vec![i128_class::KIntC::InRange(2), i128_class::KIntC::InRange(5)],
    } })
}
fn tree_sum() -> CoreProgram {
    let leaf = |n| CoreTree::Leaf { n: i128_class::KIntC::InRange(n) };
    program("sum_tree", "tr", SortTag::Tree, CoreExpr::Match {
        scrutinee: Box::new(var("tr")), arms: vec![
            CoreArm { pattern: CorePattern::PLeaf { x: "n".into() }, body: var("n") },
            CoreArm { pattern: CorePattern::PBranch { x: "l".into(), y: "r".into() },
                body: binary(BinaryOp::Add, call("sum_tree", var("l")), call("sum_tree", var("r"))) },
        ],
    }, CoreExpr::Literal { value: CoreValue::Tree { tree: CoreTree::Branch {
        left: Box::new(leaf(4)), right: Box::new(CoreTree::Branch {
            left: Box::new(leaf(1)), right: Box::new(leaf(3)),
        }),
    } } })
}

fn inspect_variants(materialized: &wp::StrategyMaterializedSnapshot) {
    let raw = &materialized.raw;
    let plan = &materialized.plan;
    let baseline = wp::evaluate_uncached(plan, raw).unwrap_or_else(|e| panic!("uncached {e:?}"));
    assert_eq!(materialized.evaluation.witness.decoded, baseline.decoded);
    let reclaimed = wp::evaluate_last_use(plan, raw).unwrap_or_else(|e| panic!("last-use {e:?}"));
    assert_eq!(reclaimed.witness.decoded, baseline.decoded);
    let live_outs = plan.live_outs.iter().copied().collect::<BTreeSet<_>>().len();
    assert_eq!(reclaimed.scheduling.reclaimed_slots, plan.nodes.len() - live_outs);
    assert_eq!(reclaimed.scheduling.retained_slots_at_publication, live_outs);
    assert!(reclaimed.scheduling.peak_live_slots <= plan.nodes.len());
    assert_eq!(reclaimed.witness.diagnostics.evaluated_nodes, plan.nodes.len());
    assert_eq!(reclaimed.witness.diagnostics.value_constructions, baseline.diagnostics.value_constructions);
    assert_eq!(reclaimed.witness.diagnostics.environment_constructions, baseline.diagnostics.environment_constructions);
    assert_eq!(reclaimed.witness.diagnostics.copied_value_nodes, baseline.diagnostics.copied_value_nodes);
    assert_eq!(reclaimed.witness.diagnostics.copied_environment_entries, baseline.diagnostics.copied_environment_entries);

    let mut cache = wp::WitnessCache::default();
    let cold = wp::evaluate_memoized(plan, raw, &mut cache, wp::SlotStorage::RetainAll)
        .unwrap_or_else(|e| panic!("cold {e:?}"));
    assert_eq!(cold.witness.decoded, baseline.decoded);
    assert!(cold.scheduling.scope_reset);
    assert_eq!(cold.scheduling.cache_hits, 0);
    assert_eq!(cold.scheduling.cache_misses, plan.nodes.len() - plan.inputs.len());
    assert_eq!(cold.witness.diagnostics, baseline.diagnostics);
    let warm = wp::evaluate_memoized(plan, raw, &mut cache, wp::SlotStorage::LastUse)
        .unwrap_or_else(|e| panic!("warm {e:?}"));
    assert_eq!(warm.witness.decoded, baseline.decoded);
    assert!(!warm.scheduling.scope_reset);
    assert_eq!(warm.scheduling.cache_misses, 0);
    assert_eq!(warm.scheduling.cache_hits, plan.nodes.len() - plan.inputs.len());
    assert_eq!(warm.witness.diagnostics.input_reads, plan.inputs.len());
    assert_eq!(warm.witness.diagnostics.evaluated_nodes, plan.inputs.len());
    assert_eq!(warm.witness.diagnostics.value_constructions, 0);
    assert_eq!(warm.witness.diagnostics.environment_constructions, 0);
    assert_eq!(warm.witness.diagnostics.payload_constructions, 0);
    assert_eq!(warm.scheduling.copied_cached_slot_values, warm.scheduling.cache_hits);
    assert_eq!(warm.scheduling.cached_value_slots, plan.nodes.len() - plan.inputs.len());
    assert_eq!(warm.scheduling.reclaimed_slots, plan.nodes.len() - live_outs);
    assert_eq!(warm.scheduling.retained_slots_at_publication, live_outs);
    assert_eq!(warm.witness.diagnostics.measured_host_bytes, None);
    assert_eq!(warm.witness.diagnostics.measured_host_time_ns, None);
}
fn resolved_result(snapshot: &wp::DecodedSnapshot) -> &wp::DecodedValue {
    let outcome = &snapshot.fields.iter().find(|f| f.field == "outcome").unwrap().value;
    let wp::SlotValue::Payload(payload) = outcome else { panic!("complete outcome payload") };
    assert_eq!(payload.kind, wp::PayloadKind::Outcome);
    let value = &payload.fields.iter().find(|f| f.field == "value").unwrap().value;
    let wp::SlotValue::Value(value) = value else { panic!("resolved returned value") };
    value
}
fn exercise(source: CoreProgram, expected: u128, stop_and_resume: bool) {
    let (bounds, schema) = layout();
    let owner = host_owner_engine();
    let admitted = tooling(owner.admit(&source, &bounds, &schema));
    let mut image = tooling(owner.admit_run(&admitted, &RunIntC::Nat(4096),
        &RunIntC::Nat(64), &FinanceInput::Absent));
    let initial = co::capture(&admitted, &image).unwrap_or_else(|_| panic!("bound initial"));
    let mut previous = initial;
    let mut records = 0usize;
    let mut saw_pending_call = false;
    let mut final_decoded = None;
    let mut cache = wp::WitnessRecordCache::default();
    let mut sink = |step: &wp::StrategyObservedStep| {
        assert_eq!(step.before.raw, previous);
        let (raw_before, raw_after) = match &step.observed {
            co::ObservedStep::Event(record) => (&record.before, &record.after),
            co::ObservedStep::Terminal { before, after } => (before, after),
        };
        assert_eq!(&step.before.raw, raw_before);
        assert_eq!(&step.after.raw, raw_after);
        inspect_variants(&step.before);
        inspect_variants(&step.after);
        saw_pending_call |= matches!(raw_before.image.control, ControlC::Invoke { .. })
            && !raw_before.image.frames.is_empty();
        if matches!(raw_after.image.outcome, OutcomeC::Halted { .. }) {
            final_decoded = Some(step.after.evaluation.witness.decoded.clone());
        }
        previous = raw_after.clone();
        records += 1;
        if stop_and_resume && records == 1 { wp::StreamAction::Stop }
        else { wp::StreamAction::Continue }
    };
    let first = wp::stream_owner_steps(&admitted, &mut image, Some(&mut cache), &mut sink)
        .unwrap_or_else(|_| panic!("real owner stream"));
    if stop_and_resume {
        assert_eq!(first, wp::StreamStop::Sink);
        // Resume the SAME image/cache after the delivered boundary: no replay.
        let resumed = wp::stream_owner_steps(&admitted, &mut image, Some(&mut cache), &mut sink)
            .unwrap_or_else(|_| panic!("same-image continuation"));
        assert_eq!(resumed, wp::StreamStop::Terminal);
    } else { assert_eq!(first, wp::StreamStop::Terminal); }
    drop(sink);
    let returned = co::capture(&admitted, &image).unwrap_or_else(|_| panic!("bound returned"));
    assert_eq!(returned, previous);
    assert!(records > 1);
    assert!(saw_pending_call);
    assert!(matches!(returned.image.outcome, OutcomeC::Halted { .. }));
    let decoded = final_decoded.expect("actual halted boundary");
    assert_eq!(resolved_result(&decoded), &wp::DecodedValue {
        nodes: vec![wp::ValueNode::UInt(expected)], root: 0,
    });
}

#[test]
fn factorial_three_cold_warm_last_use_and_stopped_stream() { exercise(factorial(), 6, true); }
#[test]
fn list_two_five_cold_warm_last_use_and_stream() { exercise(list_sum(), 7, false); }
#[test]
fn tree_four_one_three_cold_warm_last_use_and_stream() { exercise(tree_sum(), 8, false); }
