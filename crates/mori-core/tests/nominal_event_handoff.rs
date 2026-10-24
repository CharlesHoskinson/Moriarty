//! Positive nominal source -> existing owner handoff predicates. SOURCE_ONLY / UNRUN.
//! Authored before production changes; manual semantic expectations are AUTHOR
//! predictions, not an independent oracle. Direct-owner/old-observer comparisons
//! are shared-producer equivalence checks. No negative/security-policy tests.
use mori_core::sherlock::helper_machine::{
    raw::{ConstRow, DataRow},
    shared::{
        FinanceInput, OutcomeC, OwnerEngine, ProgramView, RefC, RunIntC, RunSummary, StepEventC,
        adt::TypeIdC,
        owner_host::{HostObservation, host_owner_engine},
    },
};
use mori_core::sherlock_compile_artifact::nominal_body_command as command;
use serde_json::{Value, json};
const SOURCE: &str = include_str!("fixtures/nominal-handoff/ribbon-relay.mori");
const CONFIG: &str = include_str!("fixtures/nominal-handoff/config.json");
const TARGET: &str = include_str!("fixtures/nominal-handoff/target.json");
const EMPTY: &str = include_str!("fixtures/nominal-handoff/empty.inputs.json");
const NONEMPTY: &str = include_str!("fixtures/nominal-handoff/nonempty.inputs.json");
fn must<T>(value: Result<T, mori_core::sherlock_compile_artifact::CompileFailure>) -> T {
    value.unwrap_or_else(|failure| panic!("{}", serde_json::to_string(&failure).unwrap()))
}

// Fixture-only positive projection: all children/field names/concrete generic
// arguments retained. Allocated IDs are read from actual tables, never predicted.
fn type_tree(view: &ProgramView, id: TypeIdC) -> Value {
    if id == TypeIdC::UInt {
        return json!("UInt");
    }
    if id == TypeIdC::Bool {
        return json!("Bool");
    }
    let row = &view.types[id.0];
    json!({"name": view.names[row.name_id], "args":
        view.type_args[row.args.start..row.args.start + row.args.len].iter()
            .map(|id| type_tree(view, *id)).collect::<Vec<_>>()})
}
fn value_tree(actual: &HostObservation<'_>, r: RefC) -> Value {
    let view = actual.admission.view;
    let (type_id, ctor_id, children) = match r {
        RefC::Const(i) => match &view.constants[i] {
            ConstRow::U128 { n, .. } => return json!({"UInt": n.to_string()}),
            ConstRow::Bool { b, .. } => return json!({"Bool": b}),
            ConstRow::Constructor {
                type_id,
                ctor_id,
                fields,
                ..
            } => (
                TypeIdC(type_id.0),
                ctor_id.0,
                view.constant_fields[fields.start.0..fields.start.0 + fields.len]
                    .iter()
                    .copied()
                    .map(RefC::Const)
                    .collect::<Vec<_>>(),
            ),
            _ => panic!("unexpected dedicated fixture value"),
        },
        RefC::Data(i) => match &actual.image.data[i] {
            DataRow::U128 { n } => return json!({"UInt": n.to_string()}),
            DataRow::Bool { b } => return json!({"Bool": b}),
            DataRow::Constructor {
                type_id,
                ctor_id,
                fields,
            } => (
                TypeIdC(type_id.0),
                ctor_id.0,
                actual.image.data_fields[fields.start.0..fields.start.0 + fields.len].to_vec(),
            ),
            _ => panic!("unexpected dedicated fixture value"),
        },
    };
    let ctor = &view.constructors[ctor_id];
    assert_eq!(ctor.owning_type, type_id);
    assert_eq!(ctor.field_len, children.len());
    let fields = view.ctor_fields[ctor.field_start..ctor.field_start + ctor.field_len]
        .iter()
        .zip(children)
        .map(|(field, child)| {
            json!({
                "name": view.names[field.name_id], "type": type_tree(view, field.type_id),
                "value": value_tree(actual, child),
            })
        })
        .collect::<Vec<_>>();
    json!({"type": type_tree(view, type_id), "ctor": view.names[ctor.name_id], "fields": fields})
}
fn ribbon_type() -> Value {
    json!({"name":"Ribbon", "args":["Bool"]})
}
fn expected_ribbon(bits: &[bool]) -> Value {
    let mut value = json!({"type":ribbon_type(), "ctor":"End", "fields":[]});
    for mark in bits.iter().rev() {
        value = json!({"type":ribbon_type(), "ctor":"Link", "fields":[
            {"name":"z_mark", "type":"Bool", "value":{"Bool":mark}},
            {"name":"a_rest", "type":ribbon_type(), "value":value},
        ]});
    }
    value
}
fn expected_parcel(bits: &[bool], tag: &str) -> Value {
    json!({"type":{"name":"Parcel", "args":[ribbon_type(),"UInt"]}, "ctor":"Seal", "fields":[
        {"name":"z_contents", "type":ribbon_type(), "value":expected_ribbon(bits)},
        {"name":"a_tag", "type":"UInt", "value":{"UInt":tag}},
    ]})
}
fn same_summary(a: &RunSummary, b: &RunSummary) {
    assert_eq!(a.outcome, b.outcome);
    assert_eq!(a.events, b.events);
    assert_eq!(a.consumed_work, b.consumed_work);
    assert_eq!(a.helper_work, b.helper_work);
    assert_eq!(a.helper_visits, b.helper_visits);
    assert_eq!(a.reference_cursor, b.reference_cursor);
}
fn check(inputs: Option<&str>, expected: Value) {
    let mut events = Vec::<StepEventC>::new();
    let run = must(match inputs {
        None => command::run(SOURCE, CONFIG, TARGET, &mut |event| {
            events.push(event.clone())
        }),
        Some(inputs) => command::run_with_nominal_inputs(
            SOURCE,
            CONFIG,
            TARGET,
            inputs,
            "stamp",
            &mut |event| events.push(event.clone()),
        ),
    });
    assert!(!events.is_empty());
    assert_eq!(events.len() as u128, run.completed().events_forwarded);
    assert_eq!(
        run.completed().events_forwarded,
        run.completed().summary.events
    );
    assert_eq!(
        events.last().unwrap().outcome_payload,
        run.completed().summary.outcome
    );
    let actual = run
        .observe()
        .unwrap_or_else(|_| panic!("real completed pair observation"));
    assert_eq!(*actual.image.outcome, run.completed().summary.outcome);
    for event in &events {
        assert_eq!(event.step_id.run_id.as_ref(), actual.image.run_id);
    }
    let value = match run.completed().summary.outcome {
        OutcomeC::Halted { value } => value,
        outcome => panic!("expected observed Halted, got {outcome:?}"),
    };
    assert_eq!(value_tree(&actual, value), expected);
    assert_eq!(actual.admission.view.decl_param_counts, vec![1, 2]);
    assert!(!actual.admission.view.plan_nodes.is_empty());

    // The same already-admitted real source program gets a fresh genesis under
    // the exact fixed fixture fuel/stack. This existing owner baseline tests
    // full-event forwarding; it is NOT an independent interpreter oracle or a
    // claim that the production consumer repeats admission/genesis.
    let admitted = &run.completed().admitted;
    let mut image = host_owner_engine()
        .admit_run(
            admitted,
            &RunIntC::Nat(4096),
            &RunIntC::Nat(64),
            &FinanceInput::Absent,
        )
        .unwrap_or_else(|_| panic!("existing owner fresh baseline genesis"));
    let mut direct_events = Vec::new();
    let summary = host_owner_engine()
        .run(admitted, &mut image, &mut |event| {
            direct_events.push(event.clone())
        })
        .unwrap_or_else(|_| panic!("direct existing owner"));
    assert_eq!(events, direct_events);
    same_summary(&run.completed().summary, &summary);

    let compile_only = must(match inputs {
        None => command::compile(SOURCE, CONFIG, TARGET),
        Some(inputs) => {
            command::compile_with_nominal_inputs(SOURCE, CONFIG, TARGET, inputs, "stamp")
        }
    });
    assert_eq!(run.preparation(), &compile_only);
    assert_eq!(
        run.preparation()["admission"]["program_view"]
            .as_object()
            .unwrap()
            .len(),
        23
    );
    assert_eq!(run.preparation()["evaluation_performed"], false);
    let old = must(match inputs {
        None => command::observe(SOURCE, CONFIG, TARGET),
        Some(inputs) => {
            command::observe_with_nominal_inputs(SOURCE, CONFIG, TARGET, inputs, "stamp")
        }
    });
    assert_eq!(
        serde_json::to_value(must(run.into_observation())).unwrap(),
        serde_json::to_value(old).unwrap()
    );
}
#[test]
fn closed_recursive_generic_adt_handoff_preserves_every_real_event_and_complete_value() {
    check(None, expected_parcel(&[false, true], "37"));
}
#[test]
fn external_empty_generic_value_handoff_preserves_binding_and_full_projection() {
    check(Some(EMPTY), expected_parcel(&[], "19"));
}
#[test]
fn external_nonempty_generic_value_handoff_preserves_binding_and_ordered_children() {
    check(Some(NONEMPTY), expected_parcel(&[true, false], "23"));
}
