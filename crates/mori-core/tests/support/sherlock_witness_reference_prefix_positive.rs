//! Explicit normal consumer of actual saved owner records, not Cargo autodiscovery.
//! Expected prefix bodies are independent opaque includes, never producer inputs.
//! Each test advances one real B image once. The adapter never evaluates A.
use a::helper_machine::shared::owner_host::host_owner_engine;
use a::helper_machine::shared::*;
use a::helper_machine::{
    complete_observation as co, reference_projection as rp, source_input_bridge as bridge,
    witness_plan as wp,
};
use mori_core::sherlock as a;
use serde_json::{Value as J, json};
use std::sync::Arc;

const FACT: &str =
    include_str!("../fixtures/witness-reference-prefix/factorial3-complete-A39-prefix.json");
const LIST: &str =
    include_str!("../fixtures/witness-reference-prefix/list2-5-complete-A39-prefix.json");
const TREE: &str =
    include_str!("../fixtures/witness-reference-prefix/tree4-1-3-complete-A39-prefix.json");
const PENDING: &str =
    include_str!("../fixtures/witness-reference-prefix/factorial-pending-caller3.json");

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
fn factorial() -> Arc<a::Program> {
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
        uint(3),
    )
}
fn list_sum() -> Arc<a::Program> {
    one_function(
        "sum_list",
        "xs",
        a::Type::List,
        Arc::new(a::Expr::Match(
            var("xs"),
            vec![
                a::Arm {
                    pattern: a::Pattern::Nil,
                    body: uint(0),
                },
                a::Arm {
                    pattern: a::Pattern::Cons("h".into(), "t".into()),
                    body: binary(a::BinaryOp::Add, var("h"), call("sum_list", var("t"))),
                },
            ],
        )),
        literal(a::Value::List(vec![2, 5])),
    )
}
fn tree_sum() -> Arc<a::Program> {
    one_function(
        "sum_tree",
        "tr",
        a::Type::Tree,
        Arc::new(a::Expr::Match(
            var("tr"),
            vec![
                a::Arm {
                    pattern: a::Pattern::Leaf("n".into()),
                    body: var("n"),
                },
                a::Arm {
                    pattern: a::Pattern::Branch("l".into(), "r".into()),
                    body: binary(
                        a::BinaryOp::Add,
                        call("sum_tree", var("l")),
                        call("sum_tree", var("r")),
                    ),
                },
            ],
        )),
        literal(a::Value::Tree(a::Tree::Branch(
            Arc::new(a::Tree::Leaf(4)),
            Arc::new(a::Tree::Branch(
                Arc::new(a::Tree::Leaf(1)),
                Arc::new(a::Tree::Leaf(3)),
            )),
        ))),
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
fn exercise(p: Arc<a::Program>, frozen: &str, pending: Option<&str>) {
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
    let source =
        rp::ReferenceSource::new(p.clone(), &initial).expect("complete source association");
    let mut projection = rp::ReferencePrefixProjection::new(p.clone(), &initial, 4096)
        .expect("initial reference projection");
    let mut cache = wp::WitnessRecordCache::default();
    let mut previous = initial.clone();
    let mut spans = Vec::new();
    let mut raw_count = 0usize;
    let stop = wp::stream_owner_steps(&admitted, &mut image, Some(&mut cache), |step| {
        assert_eq!(step.before.raw, previous);
        let before_variants = variants(&step.before);
        let after_variants = variants(&step.after);
        match &step.observed {
            co::ObservedStep::Event(record) => {
                assert_eq!(record.before, step.before.raw);
                assert_eq!(record.after, step.after.raw);
                raw_count += 1;
                match projection
                    .push(record)
                    .expect("actual B to A boundary/interval translation")
                {
                    rp::ProjectionProgress::Pending { raw } => assert_eq!(raw, *record),
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
        wp::StreamAction::Continue
    })
    .unwrap_or_else(|_| panic!("single actual owner stream"));
    assert_eq!(stop, wp::StreamStop::Terminal);
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
    if let Some(frozen_pending) = pending {
        let selected: J =
            serde_json::from_str(frozen_pending).expect("independent pending-caller fixture");
        let indices = selected["A_snapshot_indices"]
            .as_array()
            .expect("pending indices");
        let expected_states = selected["snapshots"]
            .as_array()
            .expect("complete pending snapshots");
        assert!(!indices.is_empty());
        assert_eq!(indices.len(), expected_states.len());
        for (index, expected) in indices.iter().zip(expected_states) {
            let index = usize::try_from(index.as_u64().expect("finite snapshot index"))
                .expect("index width");
            assert_eq!(
                &snapshot(states.get(index).expect("actual complete pending boundary")),
                expected
            );
        }
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
#[test]
fn factorial_three_complete_reference_prefix_and_pending_caller() {
    exercise(factorial(), FACT, Some(PENDING));
}
#[test]
fn list_two_five_complete_reference_prefix() {
    exercise(list_sum(), LIST, None);
}
#[test]
fn tree_four_one_three_complete_reference_prefix() {
    exercise(tree_sum(), TREE, None);
}
