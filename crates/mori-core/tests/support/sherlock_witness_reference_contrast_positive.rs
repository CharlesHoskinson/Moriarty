//! Explicit normal consumer of actual saved owner records, not Cargo autodiscovery.
//! Expected prefix bodies are independent opaque includes, never producer inputs.
//! New countercases advance one real B image each; old passing prefixes are not replayed.
//! Cache contrast seeds only a valid fact3 startup capture, with zero source steps.
use a::helper_machine::shared::owner_host::host_owner_engine;
use a::helper_machine::shared::*;
use a::helper_machine::{
    complete_observation as co, reference_projection as rp, source_input_bridge as bridge,
    witness_plan as wp,
};
use mori_core::sherlock as a;
use serde_json::{Value as J, json};
use std::sync::Arc;

fn tree(t: &a::Tree) -> J {
    match t {
        a::Tree::Leaf(n) => json!({"Leaf":n}),
        a::Tree::Branch(l, r) => json!({"Branch":[tree(l),tree(r)]}),
    }
}
fn value(v: &a::Value) -> J {
    match v {
        a::Value::UInt(n) => json!({"UInt":n}),
        a::Value::Bool(b) => json!({"Bool":b}),
        a::Value::List(xs) => json!({"List":xs}),
        a::Value::Tree(t) => json!({"Tree":tree(t)}),
    }
}
fn env(e: &a::Environment) -> J {
    J::Array(
        e.iter()
            .map(|(n, v)| json!({"name":n,"value":value(v)}))
            .collect(),
    )
}
fn pattern(p: &a::Pattern) -> J {
    match p {
        a::Pattern::Var(n) => json!({"Var":n}),
        a::Pattern::Nil => json!({"Nil":null}),
        a::Pattern::Cons(x, y) => json!({"Cons":[x,y]}),
        a::Pattern::Leaf(n) => json!({"Leaf":n}),
        a::Pattern::Branch(x, y) => json!({"Branch":[x,y]}),
    }
}
fn arm(x: &a::Arm) -> J {
    json!({"pattern":pattern(&x.pattern),"body":expr(&x.body)})
}
fn arms(xs: &[a::Arm]) -> J {
    J::Array(xs.iter().map(arm).collect())
}
fn expr(x: &a::Expr) -> J {
    match x {
        a::Expr::Literal(v) => json!({"Literal":value(v)}),
        a::Expr::Var(n) => json!({"Var":n}),
        a::Expr::Let(n, x, y) => json!({"Let":{"name":n,"value":expr(x),"body":expr(y)}}),
        a::Expr::Binary(op, x, y) => {
            json!({"Binary":{"op":format!("{op:?}"),"left":expr(x),"right":expr(y)}})
        }
        a::Expr::Unary(op, x) => json!({"Unary":{"op":format!("{op:?}"),"operand":expr(x)}}),
        a::Expr::If(x, y, z) => json!({"If":{"condition":expr(x),"yes":expr(y),"no":expr(z)}}),
        a::Expr::Call(n, xs) => {
            json!({"Call":{"name":n,"args":xs.iter().map(|x|expr(x)).collect::<Vec<_>>()}})
        }
        a::Expr::Match(x, xs) => json!({"Match":{"value":expr(x),"arms":arms(xs)}}),
    }
}
fn function(f: &a::Function) -> J {
    json!({"name":f.name,"parameters":f.parameters.iter().map(|p|json!({"name":p.name,"ty":format!("{:?}",p.ty)})).collect::<Vec<_>>(),"return_type":format!("{:?}",f.return_type),"body":expr(&f.body)})
}
fn program(p: &a::Program) -> J {
    json!({"functions":p.functions.iter().map(function).collect::<Vec<_>>(),"entry":expr(&p.entry)})
}
fn frame(f: &a::Frame) -> J {
    match f {
        a::Frame::Let(n, b, e) => {
            json!({"Let":{"name":n,"body":expr(b),"saved_environment":env(e)}})
        }
        a::Frame::Left(op, r, e) => {
            json!({"Left":{"op":format!("{op:?}"),"right":expr(r),"saved_environment":env(e)}})
        }
        a::Frame::Right(op, l) => json!({"Right":{"op":format!("{op:?}"),"left":value(l)}}),
        a::Frame::Unary(op) => json!({"Unary":{"op":format!("{op:?}")}}),
        a::Frame::If(y, n, e) => {
            json!({"If":{"yes":expr(y),"no":expr(n),"saved_environment":env(e)}})
        }
        a::Frame::Match(xs, e) => json!({"Match":{"arms":arms(xs),"saved_environment":env(e)}}),
        a::Frame::Args(n, vs, xs, e) => {
            json!({"Args":{"function_name":n,"completed_values":vs.iter().map(value).collect::<Vec<_>>(),"remaining_exprs":xs.iter().map(|x|expr(x)).collect::<Vec<_>>(),"saved_environment":env(e)}})
        }
        a::Frame::Return(t, e) => {
            json!({"Return":{"expected_type":format!("{t:?}"),"saved_environment":env(e)}})
        }
        a::Frame::Scope(e) => json!({"Scope":{"saved_environment":env(e)}}),
    }
}
fn control(c: &a::Control) -> J {
    match c {
        a::Control::Eval(x) => json!({"Eval":expr(x)}),
        a::Control::Return(v) => json!({"Return":value(v)}),
        a::Control::Push(f, c, e) => {
            json!({"Push":{"frame":frame(f),"next":control(c),"environment":env(e)}})
        }
        a::Control::Invoke(n, vs) => {
            json!({"Invoke":{"name":n,"values":vs.iter().map(value).collect::<Vec<_>>()}})
        }
        a::Control::Enter(f, vs) => {
            json!({"Enter":{"function":f.as_ref().map(function),"values":vs.iter().map(value).collect::<Vec<_>>()}})
        }
        a::Control::Bound(e, t, b) => {
            json!({"Bound":{"bindings":e.as_ref().map(env),"type":format!("{t:?}"),"body":expr(b)}})
        }
        a::Control::Cases(v, xs, e) => {
            json!({"Cases":{"value":value(v),"arms":arms(xs),"saved_environment":env(e)}})
        }
        a::Control::CaseResult(b, v, x, xs, e) => {
            json!({"CaseResult":{"bindings":b.as_ref().map(env),"value":value(v),"body":expr(x),"remaining_arms":arms(xs),"saved_environment":env(e)}})
        }
        a::Control::PrimitiveResult(Ok(v)) => json!({"PrimitiveResult":{"Ok":value(v)}}),
        a::Control::PrimitiveResult(Err(c)) => json!({"PrimitiveResult":{"Err":c}}),
        a::Control::Error(c) => json!({"Error":c}),
    }
}
fn outcome(o: &a::Outcome) -> J {
    match o {
        a::Outcome::Pending => json!("Pending"),
        a::Outcome::Done(v) => json!({"Done":value(v)}),
        a::Outcome::Rejected(c) => json!({"Rejected":c}),
        a::Outcome::Exhausted => json!("Exhausted"),
    }
}
fn snapshot(s: &a::Snapshot) -> J {
    json!({"program":program(&s.program),"control":s.control.as_ref().map(control),"environment":env(&s.environment),"frames_top_first":s.frames.iter().map(frame).collect::<Vec<_>>(),"stack_limit":s.stack_limit,"remaining_fuel":s.remaining_fuel,"initial_fuel":s.initial_fuel,"outcome":outcome(&s.outcome)})
}

fn literal(v: a::Value) -> a::ExprRef {
    Arc::new(a::Expr::Literal(v))
}
fn uint(n: u128) -> a::ExprRef {
    literal(a::Value::UInt(n))
}
fn var(n: &str) -> a::ExprRef {
    Arc::new(a::Expr::Var(n.into()))
}
fn binary(op: a::BinaryOp, x: a::ExprRef, y: a::ExprRef) -> a::ExprRef {
    Arc::new(a::Expr::Binary(op, x, y))
}
fn call(n: &str, x: a::ExprRef) -> a::ExprRef {
    Arc::new(a::Expr::Call(n.into(), vec![x]))
}
fn one_function(n: &str, p: &str, t: a::Type, b: a::ExprRef, arg: a::ExprRef) -> Arc<a::Program> {
    Arc::new(a::Program {
        functions: vec![a::Function {
            name: n.into(),
            parameters: vec![a::Parameter {
                name: p.into(),
                ty: t,
            }],
            return_type: a::Type::UInt,
            body: b,
        }],
        entry: call(n, arg),
    })
}
fn factorial(entry_n: u128) -> Arc<a::Program> {
    one_function(
        "fact",
        "n",
        a::Type::UInt,
        Arc::new(a::Expr::If(
            binary(a::BinaryOp::Eq, var("n"), uint(0)),
            uint(1),
            binary(
                a::BinaryOp::Mul,
                var("n"),
                call("fact", binary(a::BinaryOp::Sub, var("n"), uint(1))),
            ),
        )),
        uint(entry_n),
    )
}

fn layout() -> (BoundsC, LayoutSchemaC) {
    let bounds = BoundsC {
        code_rows: 64,
        function_rows: 4,
        code_child_slots: 16,
        parameter_slots: 8,
        pattern_rows: 8,
        match_plan_rows: 8,
        match_arm_rows: 8,
        type_rows: 8,
        constructor_rows: 8,
        constructor_field_slots: 16,
        max_arity: 8,
        constant_nodes: 16,
        constant_field_slots: 16,
        data_nodes: 64,
        data_field_slots: 64,
        environment_cells: 64,
        frames: 64,
        helper_tasks: 64,
        serialization_bytes: 65536,
        max_reference_work: 4096,
        max_helper_work: 4096,
        step_cap: 4096,
    };
    let schema = LayoutSchemaC {
        layout_version: (
            "moriarty".into(),
            "sherlock-helper-microstep-hostonly/1".into(),
        ),
        field_id: None,
        widths: None,
        scalar_layout_id: None,
        bounds,
        capacities: LayoutCapacitiesC {
            name_rows: 32,
            name_bytes: 256,
            occurrence_path_slots: 256,
            value_slots: 4096,
        },
        ext_capacities: None,
        boundary_capacities_present: false,
    };
    (bounds, schema)
}

fn tooling<T>(r: Result<T, Tooling>) -> T {
    match r {
        Ok(x) => x,
        Err(Tooling::AdmissionRejected {
            reason,
            detail,
            locator,
        }) => panic!("admission {reason:?} {detail} {locator:?}"),
        Err(Tooling::CapacityNotAdmitted {
            kind,
            needed,
            limit,
            locator,
        }) => panic!("capacity {kind:?} {needed:?}/{limit} {locator:?}"),
        Err(Tooling::UnsupportedFeature { feature, locator }) => {
            panic!("feature {feature:?} {locator:?}")
        }
    }
}
// Reconstruct the SAME saved observation. No owner/reference transition occurs.
fn variants(s: &wp::StrategyMaterializedSnapshot) -> Vec<wp::DecodedSnapshot> {
    let baseline = wp::evaluate_uncached(&s.plan, &s.raw).expect("uncached actual witness");
    assert_eq!(s.evaluation.witness.decoded, baseline.decoded);
    let last = wp::evaluate_last_use(&s.plan, &s.raw).expect("last-use actual witness");
    let mut cache = wp::WitnessCache::default();
    let cold = wp::evaluate_memoized(&s.plan, &s.raw, &mut cache, wp::SlotStorage::RetainAll)
        .expect("cold actual witness");
    let warm = wp::evaluate_memoized(&s.plan, &s.raw, &mut cache, wp::SlotStorage::LastUse)
        .expect("warm actual witness");
    assert!(cold.scheduling.scope_reset);
    assert_eq!(cold.scheduling.cache_hits, 0);
    assert!(!warm.scheduling.scope_reset);
    assert_eq!(warm.scheduling.cache_misses, 0);
    assert_eq!(
        warm.scheduling.cache_hits,
        s.plan.nodes.len() - s.plan.inputs.len()
    );
    assert_eq!(warm.witness.diagnostics.input_reads, s.plan.inputs.len());
    assert_eq!(warm.witness.diagnostics.value_constructions, 0);
    assert_eq!(warm.witness.diagnostics.environment_constructions, 0);
    assert_eq!(warm.witness.diagnostics.payload_constructions, 0);
    let live = s
        .plan
        .live_outs
        .iter()
        .copied()
        .collect::<std::collections::BTreeSet<_>>()
        .len();
    assert_eq!(last.scheduling.retained_slots_at_publication, live);
    assert_eq!(last.scheduling.reclaimed_slots, s.plan.nodes.len() - live);
    assert_eq!(warm.witness.diagnostics.measured_host_bytes, None);
    assert_eq!(warm.witness.diagnostics.measured_host_time_ns, None);
    let variants = vec![
        baseline.decoded,
        last.witness.decoded,
        cold.witness.decoded,
        warm.witness.decoded,
        s.evaluation.witness.decoded.clone(),
    ];
    for decoded in &variants {
        assert_eq!(decoded, &variants[0]);
    }
    variants
}
fn exercise(p: Arc<a::Program>, frozen: &str, ordinary_probes: bool) {
    let budget = bridge::SourceCopyBudget {
        max_functions: 4,
        max_parameters_per_function: 8,
        max_total_parameters: 8,
        max_expr_nodes: 128,
        max_copy_depth: 32,
        max_match_arms: 16,
        max_name_bytes: 512,
        max_list_elements: 64,
        max_tree_nodes: 64,
        max_tree_depth: 32,
    };
    let core = bridge::from_program_bounded(&p, &budget)
        .unwrap_or_else(|_| panic!("bounded original source copy"));
    let (bounds, schema) = layout();
    let owner = host_owner_engine();
    let admitted = tooling(owner.admit(&core, &bounds, &schema));
    let mut image = tooling(owner.admit_run(
        &admitted,
        &RunIntC::Nat(4096),
        &RunIntC::Nat(64),
        &FinanceInput::Absent,
    ));
    let initial =
        co::capture(&admitted, &image).unwrap_or_else(|_| panic!("actual initial snapshot"));
    if ordinary_probes {
        distinct_program_cache_contrast(&initial);
        let gap = rp::ReferenceSource::new(factorial(3), &initial)
            .err()
            .expect("different valid source cannot associate");
        match gap {
            // Category is independently specified. Exact section/row is API
            // diagnostic data, not an independently bound source-ID oracle.
            rp::ProjectionGap::SourceAssociation { section, row } => {
                assert!(!section.is_empty());
                assert!(row < initial.admission.view.expressions.len());
            }
            other => panic!("ordinary source mismatch category: {other:?}"),
        }
    }
    let source =
        rp::ReferenceSource::new(p.clone(), &initial).expect("complete source association");
    let mut projection = rp::ReferencePrefixProjection::new(p.clone(), &initial, 4096)
        .expect("initial reference projection");
    let mut cache = wp::WitnessRecordCache::default();
    let mut previous = initial.clone();
    let mut spans = Vec::new();
    let mut raw_count = 0usize;
    let mut probed_incomplete = false;
    let mut sink = |step: &wp::StrategyObservedStep| {
        let mut stop_this = false;
        assert_eq!(step.before.raw, previous);
        let before_variants = variants(&step.before);
        let after_variants = variants(&step.after);
        match &step.observed {
            co::ObservedStep::Event(record) => {
                assert_eq!(record.before, step.before.raw);
                assert_eq!(record.after, step.after.raw);
                raw_count += 1;
                let prior_reference = projection.returned().clone();
                match projection
                    .push(record)
                    .expect("actual B to A boundary/interval translation")
                {
                    rp::ProjectionProgress::Pending { raw } => {
                        assert_eq!(raw, *record);
                        assert_eq!(projection.returned(), &prior_reference);
                        if ordinary_probes
                            && !probed_incomplete
                            && record.event.event_kind == EventKind::HelperEnter
                        {
                            assert_eq!(
                                projection.finish().unwrap_err(),
                                rp::ProjectionGap::IncompleteInterval
                            );
                            assert!(!projection.pending_records().is_empty());
                            assert_eq!(projection.pending_records().last(), Some(record));
                            for pair in projection.pending_records().windows(2) {
                                assert_eq!(pair[0].after, pair[1].before);
                            }
                            assert_eq!(projection.returned(), &prior_reference);
                            probed_incomplete = true;
                            stop_this = true;
                        }
                    }
                    rp::ProjectionProgress::Complete(span) => {
                        assert_eq!(
                            span.b_start,
                            spans
                                .last()
                                .map_or(0, |s: &rp::ReferenceSpan| s.b_end_exclusive)
                        );
                        assert_eq!(span.b_end_exclusive, raw_count);
                        assert_eq!(span.raw.len(), span.b_end_exclusive - span.b_start);
                        assert!(
                            span.source_event >= span.b_start
                                && span.source_event < span.b_end_exclusive
                        );
                        let origin = &span.raw[span.source_event - span.b_start].event;
                        assert!(matches!(
                            origin.event_kind,
                            EventKind::LegacyComplete | EventKind::Publish
                        ));
                        let occurrence = span.raw[0]
                            .before
                            .admission
                            .view
                            .occurrences
                            .get(origin.source_expr_id)
                            .expect("real source occurrence row");
                        let path_end = occurrence
                            .path
                            .start
                            .checked_add(occurrence.path.len)
                            .expect("actual occurrence span");
                        let path = &span.raw[0].before.admission.view.occurrence_paths
                            [occurrence.path.start..path_end];
                        assert_eq!(
                            origin.source_occurrence,
                            LocatorC::Occurrence {
                                owner_is_function: occurrence.owner_is_function,
                                function: occurrence.function,
                                ast_path: path.to_vec()
                            }
                        );
                        // No B ordinal/helper count is manufactured from A work.
                        // All source events and counters remain in span.raw.
                        for decoded in &after_variants {
                            let actual = rp::project_reference_boundary(
                                &source,
                                &step.after.raw,
                                decoded,
                                4096,
                                span.reference.after.remaining_fuel,
                            )
                            .expect("cached boundary projection");
                            let rp::ReferenceBoundary::Complete { reference, .. } = actual else {
                                panic!("completed semantic boundary")
                            };
                            assert_eq!(reference, span.reference.after);
                        }
                        if span.raw.len() == 1 {
                            for decoded in &before_variants {
                                let actual = rp::project_reference_boundary(
                                    &source,
                                    &step.before.raw,
                                    decoded,
                                    4096,
                                    span.reference.before.remaining_fuel,
                                )
                                .expect("before variant projection");
                                let rp::ReferenceBoundary::Complete { reference, .. } = actual
                                else {
                                    panic!("complete source-before")
                                };
                                assert_eq!(reference, span.reference.before);
                            }
                        }
                        spans.push(span);
                    }
                }
            }
            co::ObservedStep::Terminal { before, after } => {
                assert_eq!(before, after);
                assert_eq!(before, &step.before.raw);
                assert_eq!(after, &step.after.raw);
                assert_eq!(before_variants, after_variants);
            }
        }
        previous = step.after.raw.clone();
        if stop_this {
            wp::StreamAction::Stop
        } else {
            wp::StreamAction::Continue
        }
    };
    let first = wp::stream_owner_steps(&admitted, &mut image, Some(&mut cache), &mut sink)
        .unwrap_or_else(|_| panic!("single actual owner stream"));
    if ordinary_probes {
        assert_eq!(first, wp::StreamStop::Sink);
        // Continue the SAME admitted image and record cache, without replay.
        let resumed = wp::stream_owner_steps(&admitted, &mut image, Some(&mut cache), &mut sink)
            .unwrap_or_else(|_| panic!("same-image pending helper continuation"));
        assert_eq!(resumed, wp::StreamStop::Terminal);
    } else {
        assert_eq!(first, wp::StreamStop::Terminal);
    }
    drop(sink);
    assert_eq!(probed_incomplete, ordinary_probes);
    let returned =
        co::capture(&admitted, &image).unwrap_or_else(|_| panic!("actual returned boundary"));
    assert_eq!(returned, previous);
    assert!(matches!(returned.image.outcome, OutcomeC::Halted { .. }));
    let final_reference = projection
        .finish()
        .expect("no omitted trailing helper interval");
    assert!(projection.pending_records().is_empty());
    assert_eq!(spans.iter().map(|s| s.raw.len()).sum::<usize>(), raw_count);
    assert_eq!(projection.records().len(), spans.len());
    let mut states = vec![projection.initial().clone()];
    for (i, r) in projection.records().iter().enumerate() {
        assert_eq!(&r.before, &states[i]);
        states.push(r.after.clone());
    }
    assert_eq!(states.last(), Some(final_reference));
    if ordinary_probes {
        compare_b0_schedule(&initial, &spans);
    }
    let snapshots = states.iter().map(snapshot).collect::<Vec<_>>();
    let records=projection.records().iter().enumerate().map(|(i,r)|json!({"rule_start_line":r.rule_start_line,"charge":r.charge,"before_snapshot":i,"after_snapshot":i+1})).collect::<Vec<_>>();
    let work = states
        .iter()
        .map(|s| {
            s.initial_fuel
                .checked_sub(s.remaining_fuel)
                .expect("reference work invariant")
        })
        .collect::<Vec<_>>();
    let charged: u128 = projection.records().iter().map(|r| r.charge as u128).sum();
    assert_eq!(work.last().copied(), Some(charged));
    let actual = json!({"complete_program":program(&p),"snapshots":snapshots,"records":records,"prefix_work_A":work,"charged_A_work":charged,"A_event_count":projection.records().len(),"terminal_outcome":outcome(&final_reference.outcome),"logical_depth_peak_A":states.iter().map(|s|s.frames.len()).max().unwrap()});
    let expected: J =
        serde_json::from_str(frozen).expect("independently frozen full prefix fixture");
    for key in [
        "complete_program",
        "snapshots",
        "records",
        "prefix_work_A",
        "charged_A_work",
        "A_event_count",
        "terminal_outcome",
        "logical_depth_peak_A",
    ] {
        assert!(
            expected.get(key).is_some(),
            "missing independent field {key}"
        );
        assert_eq!(
            &actual[key], &expected[key],
            "complete independent reference field {key}"
        );
    }
    // Raw B work is retained above; it is deliberately not compared to A fuel.
    assert_eq!(
        spans
            .last()
            .expect("publication span")
            .raw
            .last()
            .unwrap()
            .after,
        returned
    );
}

// This existing independently frozen contrast predates the new consumer source.
const FACT2: &str = include_str!(
    "../fixtures/witness-reference-contrast-independent/factorial2-normal-program-contrast-A39-prefix.json"
);

// A valid distinct program is admitted/captured only; no old owner step is run.
fn capture_initial(p: &a::Program) -> co::HostSnapshot {
    let budget = bridge::SourceCopyBudget {
        max_functions: 4,
        max_parameters_per_function: 8,
        max_total_parameters: 8,
        max_expr_nodes: 128,
        max_copy_depth: 32,
        max_match_arms: 16,
        max_name_bytes: 512,
        max_list_elements: 64,
        max_tree_nodes: 64,
        max_tree_depth: 32,
    };
    let core = bridge::from_program_bounded(p, &budget)
        .unwrap_or_else(|_| panic!("bounded valid contrast source"));
    let (bounds, schema) = layout();
    let owner = host_owner_engine();
    let admitted = tooling(owner.admit(&core, &bounds, &schema));
    let image = tooling(owner.admit_run(
        &admitted,
        &RunIntC::Nat(4096),
        &RunIntC::Nat(64),
        &FinanceInput::Absent,
    ));
    co::capture(&admitted, &image).unwrap_or_else(|_| panic!("valid actual initial capture"))
}
fn distinct_program_cache_contrast(initial: &co::HostSnapshot) {
    let seed = capture_initial(&factorial(3));
    assert_ne!(seed.admission.view, initial.admission.view);
    let seed_plan = wp::construct_plan(&seed).expect("valid seed plan");
    let actual_plan = wp::construct_plan(initial).expect("valid contrast plan");
    let mut cache = wp::WitnessCache::default();
    let seed_cold =
        wp::evaluate_memoized(&seed_plan, &seed, &mut cache, wp::SlotStorage::RetainAll)
            .expect("initial-only seed");
    assert!(seed_cold.scheduling.scope_reset);
    let changed =
        wp::evaluate_memoized(&actual_plan, initial, &mut cache, wp::SlotStorage::LastUse)
            .expect("exact changed program scope");
    assert!(changed.scheduling.scope_reset);
    assert_eq!(changed.scheduling.cache_hits, 0);
    let baseline = wp::evaluate_uncached(&actual_plan, initial).expect("actual contrast baseline");
    assert_eq!(changed.witness.decoded, baseline.decoded);
    let warm = wp::evaluate_memoized(&actual_plan, initial, &mut cache, wp::SlotStorage::LastUse)
        .expect("same exact contrast scope");
    assert!(!warm.scheduling.scope_reset);
    assert_eq!(warm.scheduling.cache_misses, 0);
    assert_eq!(warm.witness.decoded, baseline.decoded);
}

const LET_LEAF_VAR: &str = include_str!(
    "../fixtures/witness-reference-contrast-independent/let-leaf-pattern-var-complete-A39-prefix.json"
);
fn let_leaf_pattern_var() -> Arc<a::Program> {
    let held = Arc::new(a::Expr::Let(
        "x".into(),
        uint(3),
        Arc::new(a::Expr::Unary(a::UnaryOp::Leaf, var("x"))),
    ));
    Arc::new(a::Program {
        functions: vec![],
        entry: Arc::new(a::Expr::Match(
            held,
            vec![a::Arm {
                pattern: a::Pattern::Var("whole".into()),
                body: var("whole"),
            }],
        )),
    })
}
#[test]
fn factorial_two_complete_prefix_program_cache_and_incomplete_stream() {
    exercise(factorial(2), FACT2, true);
}
#[test]
fn let_leaf_pattern_variable_complete_reference_prefix() {
    exercise(let_leaf_pattern_var(), LET_LEAF_VAR, false);
}

// Real B counters are read from the saved owner observations. None of these
// counters is computed from A charges, expected rows or a reconstructed engine.
fn verify_record_counters(record: &co::CompleteStepRecord) {
    let before = &record.before.image;
    let after = &record.after.image;
    let event = &record.event;
    assert_eq!(after.run_id, before.run_id);
    assert_eq!(after.initial_fuel, before.initial_fuel);
    assert_eq!(
        after.consumed_work,
        before
            .consumed_work
            .checked_add(event.work_delta as u128)
            .expect("B work addition")
    );
    assert_eq!(
        after.helper_work,
        before
            .helper_work
            .checked_add(event.helper_work_delta as u128)
            .expect("B helper work addition")
    );
    assert_eq!(
        after.helper_visits,
        before
            .helper_visits
            .checked_add(event.helper_visits_delta as u128)
            .expect("B helper visits addition")
    );
    assert_eq!(
        after.reference_cursor,
        before
            .reference_cursor
            .checked_add(event.reference_cursor_delta as u128)
            .expect("B reference cursor addition")
    );
    assert_eq!(
        after.remaining_work.checked_add(event.work_delta as u128),
        Some(before.remaining_work)
    );
    assert_eq!(
        after.remaining_work.checked_add(after.consumed_work),
        Some(after.initial_fuel)
    );
    assert_eq!(event.outcome_payload, after.outcome);
}

// Entire public normative inventory formatting, not selected schedule metadata.
fn b_rule_id(rule: RuleIdC) -> &'static str {
    match rule {
        RuleIdC::KMachineEvalLiteralValid => "k.machine.eval.literal.valid",
        RuleIdC::KMachineEvalLiteralInvalid => "k.machine.eval.literal.invalid",
        RuleIdC::KMachineEvalVariablePresent => "k.machine.eval.variable.present",
        RuleIdC::KMachineEvalVariableAbsent => "k.machine.eval.variable.absent",
        RuleIdC::KMachineEvalLet => "k.machine.eval.let",
        RuleIdC::KMachineEvalBinary => "k.machine.eval.binary",
        RuleIdC::KMachineEvalUnary => "k.machine.eval.unary",
        RuleIdC::KMachineEvalIf => "k.machine.eval.if",
        RuleIdC::KMachineEvalMatch => "k.machine.eval.match",
        RuleIdC::KMachineEvalCallEmpty => "k.machine.eval.call.empty",
        RuleIdC::KMachineEvalCallArguments => "k.machine.eval.call.arguments",
        RuleIdC::KMachinePushAccept => "k.machine.push.accept",
        RuleIdC::KMachinePushRefuse => "k.machine.push.refuse",
        RuleIdC::KMachineReturnLet => "k.machine.return.let",
        RuleIdC::KMachineReturnLeft => "k.machine.return.left",
        RuleIdC::KMachineReturnRight => "k.machine.return.right",
        RuleIdC::KMachineReturnUnary => "k.machine.return.unary",
        RuleIdC::KMachineReturnIfTrue => "k.machine.return.if.true",
        RuleIdC::KMachineReturnIfFalse => "k.machine.return.if.false",
        RuleIdC::KMachineReturnIfInvalid => "k.machine.return.if.invalid",
        RuleIdC::KMachineReturnMatch => "k.machine.return.match",
        RuleIdC::KMachineReturnArgumentsNext => "k.machine.return.arguments.next",
        RuleIdC::KMachineReturnArgumentsFinal => "k.machine.return.arguments.final",
        RuleIdC::KMachineReturnCallValid => "k.machine.return.call.valid",
        RuleIdC::KMachineReturnCallInvalid => "k.machine.return.call.invalid",
        RuleIdC::KMachineReturnScope => "k.machine.return.scope",
        RuleIdC::KMachineInvokeFind => "k.machine.invoke.find",
        RuleIdC::KMachineEnterMissing => "k.machine.enter.missing",
        RuleIdC::KMachineEnterBind => "k.machine.enter.bind",
        RuleIdC::KMachineBoundInvalid => "k.machine.bound.invalid",
        RuleIdC::KMachineBoundValid => "k.machine.bound.valid",
        RuleIdC::KMachineCasesEmpty => "k.machine.cases.empty",
        RuleIdC::KMachineCasesTry => "k.machine.cases.try",
        RuleIdC::KMachineCasesMiss => "k.machine.cases.miss",
        RuleIdC::KMachineCasesHit => "k.machine.cases.hit",
        RuleIdC::KMachinePrimitiveValue => "k.machine.primitive.value",
        RuleIdC::KMachinePrimitiveError => "k.machine.primitive.error",
        RuleIdC::BHelperScalarEntry => "b.helper.Scalar.entry",
        RuleIdC::BHelperScalarTag => "b.helper.Scalar.tag",
        RuleIdC::BHelperScalarArithmetic => "b.helper.Scalar.arithmetic",
        RuleIdC::BHelperScalarConstruct => "b.helper.Scalar.construct",
        RuleIdC::BHelperScalarExit => "b.helper.Scalar.exit",
        RuleIdC::BHelperScalarFailure => "b.helper.Scalar.failure",
        RuleIdC::BHelperEnvGetEntry => "b.helper.EnvGet.entry",
        RuleIdC::BHelperEnvGetCell => "b.helper.EnvGet.cell",
        RuleIdC::BHelperEnvGetEmpty => "b.helper.EnvGet.empty",
        RuleIdC::BHelperEnvGetExit => "b.helper.EnvGet.exit",
        RuleIdC::BHelperEnvGetFailure => "b.helper.EnvGet.failure",
        RuleIdC::BHelperEnvPutEntry => "b.helper.EnvPut.entry",
        RuleIdC::BHelperEnvPutCell => "b.helper.EnvPut.cell",
        RuleIdC::BHelperEnvPutInsert => "b.helper.EnvPut.insert",
        RuleIdC::BHelperEnvPutEmpty => "b.helper.EnvPut.empty",
        RuleIdC::BHelperEnvPutExit => "b.helper.EnvPut.exit",
        RuleIdC::BHelperEnvPutFailure => "b.helper.EnvPut.failure",
        RuleIdC::BHelperFindEntry => "b.helper.Find.entry",
        RuleIdC::BHelperFindRow => "b.helper.Find.row",
        RuleIdC::BHelperFindEmpty => "b.helper.Find.empty",
        RuleIdC::BHelperFindExit => "b.helper.Find.exit",
        RuleIdC::BHelperFindFailure => "b.helper.Find.failure",
        RuleIdC::BHelperBindEntry => "b.helper.Bind.entry",
        RuleIdC::BHelperBindPair => "b.helper.Bind.pair",
        RuleIdC::BHelperBindEmpty => "b.helper.Bind.empty",
        RuleIdC::BHelperBindArity => "b.helper.Bind.arity",
        RuleIdC::BHelperBindType => "b.helper.Bind.type",
        RuleIdC::BHelperBindDuplicate => "b.helper.Bind.duplicate",
        RuleIdC::BHelperBindInsert => "b.helper.Bind.insert",
        RuleIdC::BHelperBindPropagate => "b.helper.Bind.propagate",
        RuleIdC::BHelperBindExit => "b.helper.Bind.exit",
        RuleIdC::BHelperBindFailure => "b.helper.Bind.failure",
        RuleIdC::BHelperAppendEntry => "b.helper.Append.entry",
        RuleIdC::BHelperAppendCell => "b.helper.Append.cell",
        RuleIdC::BHelperAppendEmpty => "b.helper.Append.empty",
        RuleIdC::BHelperAppendTail => "b.helper.Append.tail",
        RuleIdC::BHelperAppendCopy => "b.helper.Append.copy",
        RuleIdC::BHelperAppendExit => "b.helper.Append.exit",
        RuleIdC::BHelperAppendFailure => "b.helper.Append.failure",
        RuleIdC::BHelperValidateEntry => "b.helper.Validate.entry",
        RuleIdC::BHelperValidateOccurrence => "b.helper.Validate.occurrence",
        RuleIdC::BHelperValidateField => "b.helper.Validate.field",
        RuleIdC::BHelperValidateDeclaration => "b.helper.Validate.declaration",
        RuleIdC::BHelperValidateEmpty => "b.helper.Validate.empty",
        RuleIdC::BHelperValidateInvalid => "b.helper.Validate.invalid",
        RuleIdC::BHelperValidateExit => "b.helper.Validate.exit",
        RuleIdC::BHelperValidateFailure => "b.helper.Validate.failure",
        RuleIdC::BHelperEqualEntry => "b.helper.Equal.entry",
        RuleIdC::BHelperEqualPair => "b.helper.Equal.pair",
        RuleIdC::BHelperEqualField => "b.helper.Equal.field",
        RuleIdC::BHelperEqualEmpty => "b.helper.Equal.empty",
        RuleIdC::BHelperEqualDifferent => "b.helper.Equal.different",
        RuleIdC::BHelperEqualExit => "b.helper.Equal.exit",
        RuleIdC::BHelperEqualFailure => "b.helper.Equal.failure",
        RuleIdC::BHelperMatchEntry => "b.helper.Match.entry",
        RuleIdC::BHelperMatchArm => "b.helper.Match.arm",
        RuleIdC::BHelperMatchOccurrence => "b.helper.Match.occurrence",
        RuleIdC::BHelperMatchField => "b.helper.Match.field",
        RuleIdC::BHelperMatchBinding => "b.helper.Match.binding",
        RuleIdC::BHelperMatchDistinct => "b.helper.Match.distinct",
        RuleIdC::BHelperMatchDiscard => "b.helper.Match.discard",
        RuleIdC::BHelperMatchNext => "b.helper.Match.next",
        RuleIdC::BHelperMatchSelected => "b.helper.Match.selected",
        RuleIdC::BHelperMatchEmpty => "b.helper.Match.empty",
        RuleIdC::BHelperMatchExit => "b.helper.Match.exit",
        RuleIdC::BHelperMatchFailure => "b.helper.Match.failure",
        RuleIdC::BBookkeepingTaskEnqueue => "b.bookkeeping.task.enqueue",
        RuleIdC::BBookkeepingTaskRelease => "b.bookkeeping.task.release",
        RuleIdC::BBookkeepingResumeStore => "b.bookkeeping.resume.store",
        RuleIdC::BBookkeepingResumeRestore => "b.bookkeeping.resume.restore",
        RuleIdC::KAdminStartupValid => "k.admin.startup.valid",
        RuleIdC::KAdminStartupInvalid => "k.admin.startup.invalid",
        RuleIdC::KAdminPublishValue => "k.admin.publish.value",
        RuleIdC::KAdminPublishError => "k.admin.publish.error",
        RuleIdC::KAdminPublishExhausted => "k.admin.publish.exhausted",
        // Generic ADT rules are outside this dedicated B0 inventory. Their normative
        // strings are not supplied here, so they are refused rather than invented.
        RuleIdC::XAdtEvalConstructNullary
        | RuleIdC::XAdtEvalConstructFields
        | RuleIdC::XAdtReturnCtorFieldsTyped
        | RuleIdC::XAdtReturnCtorFieldsMistyped
        | RuleIdC::XAdtReturnCtorFieldsNext
        | RuleIdC::XAdtReturnCtorFieldsFinal
        | RuleIdC::XAdtMaterializeDone
        | RuleIdC::XAdtCasesPlanHit
        | RuleIdC::XAdtCasesPlanMiss
        | RuleIdC::XAdtHelperBuildEntry
        | RuleIdC::XAdtHelperBuildCell
        | RuleIdC::XAdtHelperBuildField
        | RuleIdC::XAdtHelperBuildExit
        | RuleIdC::XAdtHelperPlanEntry
        | RuleIdC::XAdtHelperPlanWalk
        | RuleIdC::XAdtHelperPlanSwitch
        | RuleIdC::XAdtHelperPlanBind
        | RuleIdC::XAdtHelperPlanSelect
        | RuleIdC::XAdtHelperPlanNoMatch
        | RuleIdC::XAdtHelperPlanExit
        | RuleIdC::XAdtHelperPlanFailure => {
            panic!("generic ADT rule outside the dedicated B0 inventory: {rule:?}")
        }
    }
}
const FACT2_B0: &str = include_str!(
    "../fixtures/witness-reference-contrast-independent/factorial2-normal-program-contrast-A39-prefix-B0-clocks.json"
);
fn b_clock_row(
    a_record: Option<usize>,
    preceding_rule: Option<&str>,
    raw: &co::HostSnapshot,
    semantic_index: usize,
) -> J {
    let image = &raw.image;
    json!([
        a_record,
        preceding_rule,
        image.consumed_work,
        image.helper_work,
        image.helper_visits,
        image.reference_cursor,
        image.remaining_work,
        semantic_index
    ])
}
fn compare_b0_schedule(initial: &co::HostSnapshot, spans: &[rp::ReferenceSpan]) {
    // Construct the COMPLETE actual prefix first; no expected cell supplies any
    // field, rule, counter, missing interval or semantic association.
    let columns = json!([
        "A_record_index_or_null",
        "preceding_B_rule_id_or_null",
        "consumed_work",
        "helper_work",
        "helper_visits",
        "reference_cursor",
        "remaining_work",
        "erased_semantic_A_snapshot_index"
    ]);
    let mut prefixes = vec![b_clock_row(None, None, initial, 0)];
    let mut intervals = Vec::new();
    let mut previous = initial.clone();
    let mut ordinal = 0usize;
    for (a_index, span) in spans.iter().enumerate() {
        assert_eq!(span.b_start, ordinal);
        assert_eq!(span.raw.len(), span.b_end_exclusive - span.b_start);
        assert_eq!(
            span.source_event.checked_add(1),
            Some(span.b_end_exclusive),
            "normal B0 separate completion ends interval"
        );
        let mut helper_ids = Vec::new();
        let mut completion_id = None;
        for (offset, record) in span.raw.iter().enumerate() {
            assert_eq!(record.before, previous);
            verify_record_counters(record);
            let id = b_rule_id(record.event.rule_id);
            let is_completion = span.b_start + offset == span.source_event;
            if is_completion {
                assert!(matches!(
                    record.event.event_kind,
                    EventKind::LegacyComplete | EventKind::Publish
                ));
                completion_id = Some(id);
            } else {
                assert!(!matches!(
                    record.event.event_kind,
                    EventKind::LegacyComplete | EventKind::Publish
                ));
                // Charged bookkeeping belongs here as well; event kind cannot
                // hide any prefix row or charged preceding rule.
                if record.event.work_delta != 0 {
                    helper_ids.push(id);
                }
            }
            prefixes.push(b_clock_row(
                Some(a_index),
                Some(id),
                &record.after,
                if is_completion { a_index + 1 } else { a_index },
            ));
            previous = record.after.clone();
            ordinal += 1;
        }
        intervals.push(json!({"A_record_index":a_index,"A_rule_line":span.reference.rule_start_line,"first_B_record":span.b_start,"B_records_exclusive":span.b_end_exclusive,"helper_rule_ids":helper_ids,"legacy_rule_id":completion_id.expect("actual source completion")}));
    }
    assert_eq!(prefixes.len(), ordinal + 1);
    let actual =
        json!({"clockPrefixColumns":columns,"clockPrefixes":prefixes,"intervals":intervals});
    // B0 describes this explicit DedicatedSorts run. No A/B work equality is
    // asserted; the previous full A39 comparison remains independently bound.
    let profile = match initial.admission.identity.profile_id {
        2 => "sherlock-helper-microstep/1",
        _ => panic!("selected B profile"),
    };
    let grammar = format!("{:?}", initial.admission.identity.grammar_domain);
    let expected: J =
        serde_json::from_str(FACT2_B0).expect("opaque independent complete B0 schedule");
    assert_eq!(expected.get("profile"), Some(&json!(profile)));
    assert_eq!(expected.get("grammarDomain"), Some(&json!(grammar)));
    assert_eq!(
        expected.get("initialFuel"),
        Some(&json!(initial.image.initial_fuel))
    );
    for key in ["clockPrefixColumns", "clockPrefixes", "intervals"] {
        assert!(
            expected.get(key).is_some(),
            "missing independent B0 field {key}"
        );
        assert_eq!(
            &actual[key], &expected[key],
            "complete independent B0 field {key}"
        );
    }
}
