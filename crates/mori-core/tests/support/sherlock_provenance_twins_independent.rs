//! Independent positive source-site oracle; literal expectations froze first.
//! One real public run. No compiler/evaluator creates expected tables or values.
//! Complete events are retained as typed values and persisted as Debug, not wire.
use mori_core::sherlock_compile_artifact::draft_execution::run_draft_host;
use mori_core::sherlock::helper_machine::raw::{ConstRow, DataRow};
use mori_core::sherlock::helper_machine::shared::{
    BinaryOp, ControlC, ExpressionRowC, FinanceInput, GrammarDomain, LocatorC,
    OutcomeC, PatternRowC, ProgramView, RefC, RuleIdC, RunIntC, SliceC, SortTag,
    StepEventC, TypeIdC, UnaryOp,
};
use serde_json::{json, Value};
use std::io::Write;

const SOURCE: &str = include_str!("fixtures/sherlock_provenance_twins/source.mori");
const CONFIG: &str = include_str!("fixtures/sherlock_provenance_twins/config.json");
const TARGET: &str = include_str!("fixtures/sherlock_provenance_twins/target.json");
const EXPECTED: &str = include_str!("fixtures/sherlock_provenance_twins/EXPECTED.json");

fn dec(n: usize) -> String { n.to_string() }
fn slice(s: SliceC) -> Value { json!({"start":dec(s.start),"len":dec(s.len)}) }
fn sort(s: TypeIdC) -> &'static str {
    match s { TypeIdC::UInt=>"UInt",TypeIdC::Bool=>"Bool",TypeIdC::List=>"List",TypeIdC::Tree=>"Tree", _=>panic!("unexpected generic type id in sealed dedicated program") }
}
fn binary(op: BinaryOp) -> &'static str {
    match op { BinaryOp::Add=>"Add",BinaryOp::Sub=>"Sub",BinaryOp::Mul=>"Mul",
        BinaryOp::Eq=>"Eq",BinaryOp::Lt=>"Lt",BinaryOp::Cons=>"Cons",BinaryOp::Branch=>"Branch" }
}
fn unary(op: UnaryOp) -> &'static str { match op {UnaryOp::Leaf=>"Leaf",UnaryOp::Not=>"Not"} }
fn empty_table<T>(field: &str, rows: &[T]) -> Value {
    assert!(rows.is_empty(), "unexpected nonempty {field} in sealed DedicatedSorts program");
    json!([])
}
// Independent read-only projection of actual admitted tables into the frozen
// frozen independent representation. No private production serializer/helper is called.
fn program_view(view: &ProgramView) -> Value {
    let expressions: Vec<Value> = view.expressions.iter().map(|e| match e {
        ExpressionRowC::Literal{constant}=>json!({"Literal":{"constant":dec(*constant)}}),
        ExpressionRowC::Var{binder}=>json!({"Var":{"binder":dec(*binder)}}),
        ExpressionRowC::Let{binder,value,body}=>json!({"Let":{"binder":dec(*binder),"value":dec(*value),"body":dec(*body)}}),
        ExpressionRowC::Binary{op,left,right}=>json!({"Binary":{"op":binary(*op),"left":dec(*left),"right":dec(*right)}}),
        ExpressionRowC::Unary{op,operand}=>json!({"Unary":{"op":unary(*op),"operand":dec(*operand)}}),
        ExpressionRowC::If{cond,then_expr,else_expr}=>json!({"If":{"cond":dec(*cond),"then_expr":dec(*then_expr),"else_expr":dec(*else_expr)}}),
        ExpressionRowC::Call{function_name,args}=>json!({"Call":{"function_name":dec(*function_name),"args":slice(*args)}}),
        ExpressionRowC::Match{scrutinee,arms,plan}=>json!({"Match":{"scrutinee":dec(*scrutinee),"arms":slice(*arms),"plan":(*plan).map(dec)}}),
        ExpressionRowC::Construct{type_id,ctor_id,fields,constant}=>json!({"Construct":{"type_id":dec(type_id.0),"ctor_id":dec(ctor_id.0),"fields":slice(*fields),"constant":(*constant).map(dec)}}),
    }).collect();
    let occurrences: Vec<Value> = view.occurrences.iter().map(|r|
        json!({"owner_is_function":r.owner_is_function,"function":dec(r.function),"path":slice(r.path)})).collect();
    let functions: Vec<Value> = view.functions.iter().map(|r|
        json!({"function_name":dec(r.function_name),"params":slice(r.params),"return_type":sort(r.return_type),"body":dec(r.body)})).collect();
    let parameters: Vec<Value> = view.parameters.iter().map(|r|
        json!({"binder":dec(r.binder),"sort":sort(r.sort)})).collect();
    let arms: Vec<Value> = view.arms.iter().map(|r|json!({"pattern":dec(r.pattern),"body":dec(r.body)})).collect();
    let patterns: Vec<Value> = view.patterns.iter().map(|r|match r {
        PatternRowC::PVar{x}=>json!({"PVar":{"x":dec(*x)}}), PatternRowC::PNil=>json!("PNil"),
        PatternRowC::PCons{x,y}=>json!({"PCons":{"x":dec(*x),"y":dec(*y)}}),
        PatternRowC::PLeaf{x}=>json!({"PLeaf":{"x":dec(*x)}}),
        PatternRowC::PBranch{x,y}=>json!({"PBranch":{"x":dec(*x),"y":dec(*y)}}),
        PatternRowC::Wildcard=>json!("Wildcard"),
        PatternRowC::Constructor{type_id,ctor_id,children}=>json!({"Constructor":{"type_id":dec(type_id.0),"ctor_id":dec(ctor_id.0),"children":slice(*children)}}),
    }).collect();
    let constants: Vec<Value> = view.constants.iter().map(|r|match r {
        ConstRow::U128{marker_free,n}=>json!({"U128":{"marker_free":marker_free,"n":n.to_string()}}),
        ConstRow::Bool{marker_free,b}=>json!({"Bool":{"marker_free":marker_free,"b":b}}),
        ConstRow::Nil{marker_free}=>json!({"Nil":{"marker_free":marker_free}}),
        // This sealed source has only UInt constants. Cons/Leaf/
        // Branch remain expression rows and are not pre-evaluated constants.
        _=>panic!("unexpected literal constant kind in sealed provenance source"),
    }).collect();
    json!({"entry":dec(view.entry),"expressions":expressions,"occurrences":occurrences,
        "occurrence_paths":view.occurrence_paths.iter().map(|n|dec(*n)).collect::<Vec<_>>(),
        "children":view.children.iter().map(|n|dec(*n)).collect::<Vec<_>>(),
        "functions":functions,"parameters":parameters,"arms":arms,"patterns":patterns,
        "pattern_children":empty_table("pattern_children",&view.pattern_children),
        "types":empty_table("types",&view.types),
        "type_args":empty_table("type_args",&view.type_args),
        "constructors":empty_table("constructors",&view.constructors),
        "ctor_fields":empty_table("ctor_fields",&view.ctor_fields),
        "decl_param_counts":empty_table("decl_param_counts",&view.decl_param_counts),
        "decl_ctor_counts":empty_table("decl_ctor_counts",&view.decl_ctor_counts),
        "decl_templates":empty_table("decl_templates",&view.decl_templates),
        "plan_nodes":empty_table("plan_nodes",&view.plan_nodes),
        "plan_cases":empty_table("plan_cases",&view.plan_cases),
        "field_paths":empty_table("field_paths",&view.field_paths),
        "constants":constants,"constant_fields":view.constant_fields.iter().map(|n|dec(*n)).collect::<Vec<_>>(),
        "names":view.names})
}
fn occurrence(locator: &LocatorC) -> Value {
    match locator {
        LocatorC::Occurrence{owner_is_function,function,ast_path}=>
            json!({"owner_is_function":owner_is_function,"function":function,"ast_path":ast_path}),
        _=>panic!("event source location must be an original source occurrence"),
    }
}


#[test]
fn provenance_twins_preserves_static_occurrences_and_dynamic_call_origins() {
    let expected: Value = serde_json::from_str(EXPECTED).expect("frozen independent literals");
    let output = std::env::var_os("SHERLOCK_PROVENANCE_EVENT_OUTPUT")
        .expect("operation supplies one fresh full Debug event output path");
    // Reserve the artifact before the sole subject invocation. A pre-existing
    // path is an ordinary harness error; this never replaces captured evidence.
    let mut debug = std::fs::OpenOptions::new().write(true).create_new(true)
        .open(&output).expect("fresh complete Debug event output file");
    let mut events: Vec<StepEventC> = Vec::new();
    let run = match run_draft_host(SOURCE, CONFIG, TARGET, &mut |e| events.push(e.clone())) {
        Ok(run) => run,
        Err(failure) => panic!("public draft runtime failed: {}",
            serde_json::to_string(&failure).unwrap()),
    };
    // Every field is retained by the existing full Debug derive. This artifact
    // is explicitly diagnostic Debug, not canonical JSON, a codec or proof.
    writeln!(debug,"StepEventC full Debug; noncanonical; one event per line").unwrap();
    for event in &events { writeln!(debug,"{event:?}").unwrap(); }
    debug.flush().expect("persist every complete event before assertions");
    println!("complete_step_events_debug_path={}",std::path::Path::new(&output).display());
    println!("actual_preparation={}",serde_json::to_string(&run.preparation).unwrap());
    let observed = match run.observe() { Ok(o)=>o,Err(_)=>panic!("same-pair observation failed") };
    let image = &observed.image;
    let summary = &run.completed.summary;

    // Compare every literal cell, not a map reconstructed from subject results.
    assert_eq!(serde_json::to_value(&run.preparation.source).unwrap(),expected["source"]);
    assert_eq!(serde_json::to_value(&run.preparation.preparation).unwrap(),expected["preparation"]);
    assert_eq!(run.preparation.admission["program_view"],expected["program_view"]);
    assert_eq!(program_view(observed.admission.view),expected["program_view"]);
    assert_eq!(run.preparation.source.source_digest,expected["source_digest"].as_str().unwrap());
    assert_eq!(run.preparation.config.file_digest,expected["config_digest"].as_str().unwrap());
    assert_eq!(run.preparation.target.file_digest,expected["target_digest"].as_str().unwrap());
    assert_eq!(run.preparation.target.descriptor,serde_json::from_str::<Value>(TARGET).unwrap());
    let mut config: Value=serde_json::from_str(CONFIG).unwrap();
    config.as_object_mut().unwrap().insert("file_digest".into(),expected["config_digest"].clone());
    assert_eq!(serde_json::to_value(&run.preparation.config).unwrap(),config);

    // The returned preparation is the original unstepped snapshot, with all
    // calls/operators/unused functions. The actual completed pair is separate.
    assert_eq!(run.preparation.genesis["control"],json!({"Eval":{"expr":"0"}}));
    assert_eq!(run.preparation.genesis["outcome"],json!("Running"));
    assert_eq!(run.preparation.genesis["initial_fuel"],json!("4096"));
    assert_eq!(run.preparation.genesis["remaining_work"],json!("4096"));
    for key in ["consumed_work","helper_work","helper_visits","reference_cursor"] {
        assert_eq!(run.preparation.genesis[key],json!("0"));
    }
    assert_eq!(run.preparation.genesis["frames"],json!([]));
    for key in ["env_root","helper","result"] { assert_eq!(run.preparation.genesis[key],Value::Null); }
    assert_eq!(serde_json::to_value(&run.preparation.observation).unwrap()["evaluation_performed"],json!(false));
    assert_eq!(run.preparation.admission["representation"],json!("HostOnly"));
    assert_eq!(observed.admission.identity.profile_id,2);
    assert_eq!(observed.admission.identity.grammar_domain,GrammarDomain::DedicatedSorts);
    assert_eq!(observed.admission.identity.finance,FinanceInput::Absent);
    assert_eq!(observed.admission.identity.layout_version,
        ("moriarty".into(),"sherlock-helper-microstep-hostonly/1".into()));
    assert_eq!(image.run_id.fuel_input,RunIntC::Nat(4096));
    assert_eq!(image.run_id.stack_limit_input,RunIntC::Nat(64));

    // Source-only arithmetic: each down(1) calls down(0) and returns1; sum2.
    let value=match *image.outcome {OutcomeC::Halted{value}=>value,_=>panic!("expected normal halt")};
    let actual=match value {
        RefC::Const(i)=>match &observed.admission.view.constants[i] {
            ConstRow::U128{marker_free:true,n}=>*n,_=>panic!("terminal constant is not UInt")},
        RefC::Data(i)=>match &image.data[i] {DataRow::U128{n}=>*n,_=>panic!("terminal data is not UInt")},
    };
    assert_eq!(actual,expected["runtime"]["returnedUInt"].as_str().unwrap().parse::<u128>().unwrap());
    assert_eq!(*image.control,ControlC::Return{value});
    assert_eq!(summary.outcome,*image.outcome);
    assert!(image.frames.is_empty());
    assert_eq!(image.env_root,None);
    assert!(image.helper.is_none() && image.result.is_none());
    assert!(!events.is_empty());
    assert_eq!(u128::try_from(events.len()).unwrap(),run.completed.events_forwarded);
    assert_eq!(summary.events,run.completed.events_forwarded);

    let site_map=expected["static_site_map"].as_array().unwrap();
    let mut sums=(0u128,0u128,0u128,0u128);
    let mut calls=Vec::new();
    let mut call_origins=Vec::new();
    let mut call_ordinals=Vec::new();
    for event in &events {
        assert_eq!(event.step_id.run_id.as_ref(),image.run_id);
        assert_eq!(event.step_id.ordinal,sums.0,"ordinal is prior B charge, never dense event index");
        let wanted=site_map.get(event.source_expr_id).expect("event must name indexed source occurrence");
        assert_eq!(wanted["source_expr_id"],json!(event.source_expr_id));
        assert_eq!(occurrence(&event.source_occurrence),wanted["locator"],"full owner/function/path join");
        assert!(![5usize,15].contains(&event.source_expr_id),"unused bodies indexed but never executed");
        sums.0=sums.0.checked_add(u128::from(event.work_delta)).unwrap();
        sums.1=sums.1.checked_add(u128::from(event.helper_work_delta)).unwrap();
        sums.2=sums.2.checked_add(u128::from(event.helper_visits_delta)).unwrap();
        sums.3=sums.3.checked_add(u128::from(event.reference_cursor_delta)).unwrap();
        if event.rule_id==RuleIdC::KMachineEvalCallArguments {
            calls.push(event.source_expr_id);
            call_origins.push(occurrence(&event.source_occurrence));
            call_ordinals.push(event.step_id.ordinal);
        }
    }
    assert_eq!(json!(calls),expected["runtime"]["evalCallArgumentExprIds"]);
    assert_eq!(json!(call_origins),expected["runtime"]["evalCallArgumentOccurrences"]);
    assert!(call_ordinals.windows(2).all(|w|w[0]<w[1]),"four distinct expanded IDs in one run");
    assert_eq!(call_origins[1],call_origins[3],"recursive origin repeats without duplicated source row");
    assert_ne!(call_origins[0],call_origins[2],"identical syntax at distinct entry sites");
    for rule in [RuleIdC::KMachineInvokeFind,RuleIdC::KMachineEnterBind,RuleIdC::KMachineReturnCallValid] {
        assert_eq!(events.iter().filter(|e|e.rule_id==rule).count(),4);
    }
    assert_eq!(sums,(image.consumed_work,image.helper_work,image.helper_visits,image.reference_cursor));
    assert_eq!(sums,(summary.consumed_work,summary.helper_work,summary.helper_visits,summary.reference_cursor));
    assert_eq!(image.initial_fuel,4096);
    assert_eq!(image.remaining_work.checked_add(image.consumed_work),Some(image.initial_fuel));
    println!("actual_runtime_summary={}",json!({"returnedUInt":actual.to_string(),
        "events":summary.events.to_string(),"callExprIds":calls,"callOrigins":call_origins,
        "callOrdinals":call_ordinals.iter().map(u128::to_string).collect::<Vec<_>>(),
        "consumedWork":sums.0.to_string(),"helperWork":sums.1.to_string(),
        "helperVisits":sums.2.to_string(),"referenceCursor":sums.3.to_string(),
        "qualification":"HostOnly; actual full events in diagnostic Debug file; no native/proof/A-B-prefix claim"}));
}
