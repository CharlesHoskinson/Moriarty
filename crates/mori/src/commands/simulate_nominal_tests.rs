//! Positive real selected-simulation executor predicates. SOURCE_ONLY / UNRUN.
//! Frozen before production edits; whole-value literals are author predictions.
//! Full-event comparison uses the same owner producer, not an independent oracle.
//! Production retains no event vector. No failure/security-policy tests live here.
use super::{Args, SelectedRun, execute};
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
use std::path::{Path, PathBuf};

fn fixtures() -> PathBuf {
    // Cargo runs tests from the crate directory. Direct rustc uses an absolute
    // source filename and a separate prospective working directory.
    let crate_relative = Path::new("tests/fixtures/nominal-simulate");
    if crate_relative.is_dir() {
        return crate_relative.to_path_buf();
    }
    Path::new(file!())
        .parent()
        .unwrap()
        .join("../../tests/fixtures/nominal-simulate")
}
fn args(source: &str) -> Args {
    let root = fixtures();
    Args {
        file: root.join(source),
        action: None,
        scenario: None,
        resume: None,
        event: None,
        trace: false,
        logical_cap: None,
        target: Some(root.join("target.json")),
        authorization: Default::default(),
        execution_profile: Some("sherlock-helper-microstep/1".into()),
        source_profile: Some("moriarty-sherlock-nominal-body-typed/1".into()),
        entry: true,
        execution_config: Some(root.join("config.json")),
        chunk: None,
        chunk_at: None,
        chunk_events: false,
        chunk_represent: false,
        chunk_witness: false,
        chunk_images: false,
        reference_slices: None,
        optimization: None,
        native_inventory: false,
        residualize: None,
        residual_family: None,
        residual_size_limit: 64,
        export_plan: None,
        export_request: vec![],
        export_dir: None,
    }
}
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
fn queue_type() -> Value {
    json!({"name":"Queue", "args":["UInt"]})
}
fn expected_queue(items: &[&str]) -> Value {
    let mut value = json!({"type":queue_type(), "ctor":"Empty", "fields":[]});
    for item in items.iter().rev() {
        value = json!({"type":queue_type(), "ctor":"Push", "fields":[
            {"name":"z_item", "type":"UInt", "value":{"UInt":item}},
            {"name":"a_tail", "type":queue_type(), "value":value},
        ]});
    }
    value
}
fn expected_envelope() -> Value {
    json!({"type":{"name":"Envelope", "args":[queue_type(),"Bool"]}, "ctor":"Wrap", "fields":[
        {"name":"z_items", "type":queue_type(), "value":expected_queue(&["5","10"])},
        {"name":"a_marker", "type":"Bool", "value":{"Bool":true}},
    ]})
}
fn arbor_type() -> Value {
    json!({"name":"Arbor", "args":["Bool"]})
}
fn tip(value: bool) -> Value {
    json!({"type":arbor_type(), "ctor":"Tip", "fields":[
        {"name":"z_value", "type":"Bool", "value":{"Bool":value}},
    ]})
}
fn fork(left: Value, right: Value) -> Value {
    json!({"type":arbor_type(), "ctor":"Fork", "fields":[
        {"name":"z_left", "type":arbor_type(), "value":left},
        {"name":"a_right", "type":arbor_type(), "value":right},
    ]})
}
fn expected_stamp() -> Value {
    json!({"type":{"name":"Stamp", "args":[arbor_type(),"UInt"]}, "ctor":"Seal", "fields":[
        {"name":"z_tree", "type":arbor_type(), "value":fork(fork(tip(false),tip(true)),tip(false))},
        {"name":"a_code", "type":"UInt", "value":{"UInt":"41"}},
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
fn check(source_file: &str, expected: Value) {
    let args = args(source_file);
    let mut events = Vec::<StepEventC>::new();
    // This is the actual handler's selected executor: real Args, pure selection,
    // bounded production source/config/target reads and the normal Core library.
    let run = match must(execute(&args, &mut |event| events.push(event.clone()))) {
        SelectedRun::Nominal(run) => run,
        SelectedRun::Draft(_) | SelectedRun::Chunked(..) | SelectedRun::Extended(..) => {
            panic!("nominal profile reached the draft compiler")
        }
    };
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

    // Exact whole ordered records, including their payloads, must match a
    // fresh genesis on the already admitted program. This is a shared-owner
    // forwarding baseline, not independent interpreter/whole-state evidence.
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

    let source = std::fs::read_to_string(&args.file).unwrap();
    let config = std::fs::read_to_string(args.execution_config.as_ref().unwrap()).unwrap();
    let target = std::fs::read_to_string(args.target.as_ref().unwrap()).unwrap();
    let preparation = run.preparation().clone();
    assert_eq!(
        preparation,
        must(command::compile(&source, &config, &target))
    );
    assert_eq!(
        preparation["admission"]["program_view"]
            .as_object()
            .unwrap()
            .len(),
        23
    );
    assert_eq!(
        preparation["source"]["source_digest"],
        mori_core::intent::hash_bytes(source.as_bytes())
    );
    assert_eq!(
        preparation["config"]["file_digest"],
        mori_core::intent::hash_bytes(config.as_bytes())
    );
    assert_eq!(
        preparation["target"]["file_digest"],
        mori_core::intent::hash_bytes(target.as_bytes())
    );
    let report = serde_json::to_value(must(run.into_observation())).unwrap();
    assert_eq!(report["format"], "sherlock-nominal-host-observation/1");
    assert_eq!(report["preparation"], preparation);
    assert_eq!(report["execution"]["completed"], true);
    assert_eq!(
        report["execution"]["events_forwarded"],
        events.len().to_string()
    );
    assert_eq!(report["execution"]["event_capture"], "not-retained");
    assert!(report["execution"]["image"].is_object());
}
#[test]
fn selected_nominal_simulation_maps_generic_queue_with_complete_values_and_events() {
    check("map-queue.mori", expected_envelope());
}
#[test]
fn selected_nominal_simulation_mirrors_generic_tree_with_complete_values_and_events() {
    check("mirror-arbor.mori", expected_stamp());
}
