//! Source-derived finite Core/1 provenance assertions; expectations froze first.
//! Explicit standalone QA consumer under tests/support; no automatic Cargo test.
//! Two pure preparation observations and exactly one actual public B-owner run.
//! Full StepEventC Debug is diagnostic, noncanonical, and caller-owned.
use mori_core::sherlock_compile_artifact::{compile_host, parse_config};
use mori_core::sherlock::helper_machine::core_action_consumer::{
    prepare_scalar_action_host, run_scalar_action,
};
use mori_core::sherlock::helper_machine::raw::{ConstRow, DataRow};
use mori_core::sherlock::helper_machine::shared::owner_host::ImageObservation;
use mori_core::sherlock::helper_machine::shared::{
    ActiveLengths, AllocCounters, BinaryOp, ControlC, ExpressionRowC, FinanceInput,
    GrammarDomain, LocatorC, OutcomeC, PatternRowC, ProgramView, RefC, RuleIdC,
    RunIntC, SliceC, SortTag, StepEventC, UnaryOp,
};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::io::Write;

const SOURCE: &str = include_str!("fixtures/sherlock_core1_provenance/source.mori");
const CONFIG: &str = include_str!("fixtures/sherlock_core1_provenance/config.json");
const TARGET: &str = include_str!("fixtures/sherlock_core1_provenance/target.json");
const EXPECTED: &str = include_str!("fixtures/sherlock_core1_provenance/EXPECTED.json");

fn dec(n: usize) -> String { n.to_string() }
fn slice(s: SliceC) -> Value { json!({"start":dec(s.start),"len":dec(s.len)}) }
fn sort(s: SortTag) -> &'static str {
    match s { SortTag::UInt=>"UInt",SortTag::Bool=>"Bool",SortTag::List=>"List",SortTag::Tree=>"Tree" }
}
fn binary(op: BinaryOp) -> &'static str {
    match op { BinaryOp::Add=>"Add",BinaryOp::Sub=>"Sub",BinaryOp::Mul=>"Mul",
        BinaryOp::Eq=>"Eq",BinaryOp::Lt=>"Lt",BinaryOp::Cons=>"Cons",BinaryOp::Branch=>"Branch" }
}
fn unary(op: UnaryOp) -> &'static str { match op {UnaryOp::Leaf=>"Leaf",UnaryOp::Not=>"Not"} }
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
        ExpressionRowC::Match{scrutinee,arms}=>json!({"Match":{"scrutinee":dec(*scrutinee),"arms":slice(*arms)}}),
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


fn reference(r: RefC) -> Value {
    match r { RefC::Const(i)=>json!({"Const":dec(i)}),RefC::Data(i)=>json!({"Data":dec(i)}) }
}
fn active(a: &ActiveLengths) -> Value {
    json!({"data":dec(a.data),"data_fields":dec(a.data_fields),
        "env":dec(a.env),"value_slots":dec(a.value_slots)})
}
fn alloc(a: &AllocCounters) -> Value {
    json!({"data_nodes":dec(a.data_nodes),"data_fields":dec(a.data_fields),
        "environment_cells":dec(a.environment_cells)})
}
// Read-only projection of every storage/control field selected by this finite
// oracle. Empty-only payloads fail before projection rather than being erased.
fn storage(image: &ImageObservation<'_>) -> Value {
    assert!(image.frames.is_empty(),"sealed endpoints have no saved frames");
    assert!(image.tasks.is_empty(),"sealed endpoints have no helper tasks");
    assert!(image.helper.is_none(),"sealed endpoints have no helper control");
    assert!(image.result.is_none(),"sealed endpoints have no staged helper result");
    let control=match image.control {
        ControlC::Eval{expr}=>json!({"Eval":{"expr":dec(*expr)}}),
        ControlC::Return{value}=>json!({"Return":{"value":reference(*value)}}),
        _=>panic!("unexpected control at sealed endpoint"),
    };
    let outcome=match image.outcome {
        OutcomeC::Running=>json!("Running"),
        OutcomeC::Halted{value}=>json!({"Halted":{"value":reference(*value)}}),
        _=>panic!("sealed normal endpoint must not reject or exhaust"),
    };
    let data:Vec<Value>=image.data.iter().map(|row|match row {
        DataRow::Bool{b}=>json!({"Bool":{"b":b}}),
        DataRow::U128{n}=>json!({"U128":{"n":n.to_string()}}),
        _=>panic!("sealed scalar endpoint has no constructor data"),
    }).collect();
    let env:Vec<Value>=image.env.iter().map(|row|json!({"binder":dec(row.binder),
        "value":reference(row.value),"previous":row.previous.map(dec)})).collect();
    json!({"control":control,"control_origin":dec(image.control_origin),
        "env_root":image.env_root.map(dec),"stack_limit":dec(image.stack_limit),
        "outcome":outcome,"alloc":alloc(image.alloc),"active":active(image.active),
        "helper":null,"result":null,"data":data,
        "data_fields":image.data_fields.iter().map(|r|reference(*r)).collect::<Vec<_>>(),
        "env":env,"frames":[],"tasks":[],
        "value_slots":image.value_slots.iter().map(|r|reference(*r)).collect::<Vec<_>>()})
}
fn assert_run_inputs_and_zero_segment(image: &ImageObservation<'_>) {
    assert_eq!(image.run_id.fuel_input,RunIntC::Nat(1000));
    assert_eq!(image.run_id.stack_limit_input,RunIntC::Nat(32));
    assert_eq!(image.run_id.identity.profile_id,2);
    assert_eq!(image.run_id.identity.grammar_domain,GrammarDomain::DedicatedSorts);
    assert_eq!(image.run_id.identity.finance,FinanceInput::Absent);
    assert_eq!(image.run_id.identity.layout_version,
        ("moriarty".into(),"sherlock-helper-microstep-hostonly/1".into()));
    assert_eq!(image.seg_base.active(),ActiveLengths::default());
    assert_eq!(image.seg_base.alloc(),AllocCounters::default());
    assert_eq!(image.seg_base.consumed_work(),0);
}
fn rule_origins(events: &[StepEventC], rule: RuleIdC) -> Value {
    json!(events.iter().filter(|e|e.rule_id==rule).map(|e|e.source_expr_id).collect::<Vec<_>>())
}

#[test]
fn core1_lowering_preserves_complete_static_map_and_every_public_sink_origin() {
    let expected:Value=serde_json::from_str(EXPECTED).expect("frozen manual literals");
    let output=std::env::var_os("SHERLOCK_CORE1_PROVENANCE_EVENT_OUTPUT")
        .expect("explicit QA job supplies one fresh complete Debug event path");
    let mut debug=std::fs::OpenOptions::new().write(true).create_new(true)
        .open(&output).expect("reserve fresh full-event capture before subject work");

    // Real parameter-free public compiler, with observe=false: no B run.
    let artifact=compile_host(SOURCE,"compare",CONFIG,TARGET,false).unwrap_or_else(|e|
        panic!("public compile-only preparation failed: {}",serde_json::to_string(&e).unwrap()));
    println!("actual_compile_only_artifact={}",serde_json::to_string(&artifact).unwrap());
    assert_eq!(serde_json::to_value(&artifact.source).unwrap(),expected["source"]);
    assert_eq!(serde_json::to_value(&artifact.preparation).unwrap(),expected["preparation"]);
    assert_eq!(artifact.admission["program_view"],expected["program_view"]);
    assert_eq!(artifact.admission["representation"],json!("HostOnly"));
    assert_eq!(serde_json::to_value(&artifact.input_policy).unwrap(),
        json!({"kind":"parameter-free","declared":[],"supplied":[]}));
    assert_eq!(serde_json::to_value(&artifact.observation).unwrap(),
        json!({"status":"not-requested","evaluation_performed":false}));
    let mut expected_config:Value=serde_json::from_str(CONFIG).unwrap();
    expected_config.as_object_mut().unwrap().insert("file_digest".into(),expected["config_digest"].clone());
    assert_eq!(serde_json::to_value(&artifact.config).unwrap(),expected_config);
    assert_eq!(artifact.target.file_digest,expected["target_digest"].as_str().unwrap());
    assert_eq!(artifact.target.descriptor,serde_json::from_str::<Value>(TARGET).unwrap());

    // Exact same public preparer and runtime callee consumed by compile_host.
    // This typed preparation observes the complete actual unstepped genesis;
    // the subsequent run_scalar_action owns its own single pair and runs once.
    let parsed=mori_core::parse(SOURCE).unwrap_or_else(|_|panic!("sealed Core/1 source parse"));
    assert_eq!(serde_json::to_value(&parsed).unwrap(),expected["core_ast"]);
    let config=parse_config(CONFIG).unwrap_or_else(|_|panic!("existing Core wire config"))
        .consumer().unwrap_or_else(|_|panic!("existing config conversion"));
    let inputs=BTreeMap::new();
    let prepared=prepare_scalar_action_host(&parsed,0,&inputs,&config)
        .unwrap_or_else(|_|panic!("public typed preparation failed"));
    let genesis=prepared.observe().unwrap_or_else(|_|panic!("actual genesis observation refused"));
    assert_eq!(serde_json::to_value(&prepared.preparation).unwrap(),expected["preparation"]);
    assert_eq!(program_view(genesis.admission.view),expected["program_view"]);
    assert_eq!(genesis.image.run_id.identity,*genesis.admission.identity);
    assert_run_inputs_and_zero_segment(&genesis.image);
    let mut g=storage(&genesis.image);
    for (key,value) in [("initial_fuel",genesis.image.initial_fuel),
        ("remaining_work",genesis.image.remaining_work),("consumed_work",genesis.image.consumed_work),
        ("helper_work",genesis.image.helper_work),("helper_visits",genesis.image.helper_visits),
        ("reference_cursor",genesis.image.reference_cursor)] {g[key]=json!(value.to_string());}
    g["seg_base"]=json!({"active":active(&genesis.image.seg_base.active()),
        "alloc":alloc(&genesis.image.seg_base.alloc()),"consumed_work":"0"});
    assert_eq!(g,expected["genesis"],"complete selected unstepped genesis, no evaluation");

    let mut events=Vec::<StepEventC>::new();
    let result=run_scalar_action(&parsed,0,&inputs,&config,&mut |e|events.push(e.clone()));
    // Preserve every full event even when the actual run returns an error.
    writeln!(debug,"StepEventC full Debug; noncanonical; one event per line").unwrap();
    for event in &events {writeln!(debug,"{event:?}").unwrap();}
    debug.flush().expect("persist complete actual events before runtime assertions");
    println!("complete_step_events_debug_path={}",std::path::Path::new(&output).display());
    let run=result.unwrap_or_else(|_|panic!("public scalar owner run failed; full forwarded prefix retained"));
    let observed=run.observe().unwrap_or_else(|_|panic!("actual completed observation refused"));
    let image=&observed.image;
    let summary=&run.completed.summary;
    assert_eq!(serde_json::to_value(&run.preparation).unwrap(),expected["preparation"]);
    assert_eq!(program_view(observed.admission.view),expected["program_view"]);
    assert_eq!(image.run_id.identity,*observed.admission.identity);
    assert_run_inputs_and_zero_segment(image);
    assert_eq!(storage(image),expected["runtime"]["terminal_storage_and_control"]);
    assert_eq!(*image.control,ControlC::Return{value:RefC::Data(0)});
    assert_eq!(*image.outcome,OutcomeC::Halted{value:RefC::Data(0)});
    assert_eq!(expected["runtime"]["returnedBool"],json!(true));
    assert_eq!(summary.outcome,*image.outcome);
    assert!(!events.is_empty());
    assert_eq!(u128::try_from(events.len()).unwrap(),run.completed.events_forwarded);
    assert_eq!(summary.events,run.completed.events_forwarded);
    let map=expected["static_site_map"].as_array().unwrap();
    let mut sums=(0u128,0u128,0u128,0u128);
    for event in &events {
        assert_eq!(event.step_id.run_id.as_ref(),image.run_id);
        assert_eq!(event.step_id.ordinal,sums.0,"expanded ordinal is prior B work, not dense index");
        let site=map.get(event.source_expr_id).expect("every event names a literal indexed source site");
        assert_eq!(site["source_expr_id"],json!(event.source_expr_id));
        assert_eq!(occurrence(&event.source_occurrence),site["locator"],"full owner/function/path join");
        if let Some(origin)=site["core_origin"].as_object() {
            let id=origin["expression"].as_u64().unwrap() as usize;
            assert_eq!(serde_json::to_value(&parsed.expressions[id]).unwrap(),origin["source_expression"],
                "generated lowering site joins the original Core expression, never a fabricated Core node");
        } else {assert_eq!(event.source_expr_id,0,"only generated entry wrapper lacks a Core expression");}
        sums.0=sums.0.checked_add(u128::from(event.work_delta)).unwrap();
        sums.1=sums.1.checked_add(u128::from(event.helper_work_delta)).unwrap();
        sums.2=sums.2.checked_add(u128::from(event.helper_visits_delta)).unwrap();
        sums.3=sums.3.checked_add(u128::from(event.reference_cursor_delta)).unwrap();
    }
    for (name,rule) in [("KMachineEvalCallEmpty",RuleIdC::KMachineEvalCallEmpty),
        ("KMachineEvalLet",RuleIdC::KMachineEvalLet),
        ("KMachineEvalLiteralValid",RuleIdC::KMachineEvalLiteralValid),
        ("KMachineEvalBinary",RuleIdC::KMachineEvalBinary),
        ("KMachineEvalVariablePresent",RuleIdC::KMachineEvalVariablePresent)] {
        assert_eq!(rule_origins(&events,rule),expected["runtime"]["expected_eval_origins"][name]);
    }
    for id in expected["runtime"]["executed_lowering_site_ids"].as_array().unwrap() {
        let id=id.as_u64().unwrap() as usize;
        assert!(events.iter().any(|e|e.source_expr_id==id),"actual synthetic lowering origin {id}");
    }
    for rule in [RuleIdC::KMachineInvokeFind,RuleIdC::KMachineEnterBind,RuleIdC::KMachineReturnCallValid] {
        assert_eq!(events.iter().filter(|e|e.rule_id==rule).count(),1,"single actual named call");
    }
    assert_eq!(sums,(image.consumed_work,image.helper_work,image.helper_visits,image.reference_cursor));
    assert_eq!(sums,(summary.consumed_work,summary.helper_work,summary.helper_visits,summary.reference_cursor));
    assert_eq!(image.initial_fuel,1000);
    assert_eq!(image.remaining_work.checked_add(image.consumed_work),Some(image.initial_fuel));
    println!("actual_runtime_summary={}",json!({"returnedBool":true,
        "events":summary.events.to_string(),"consumedWork":sums.0.to_string(),
        "helperWork":sums.1.to_string(),"helperVisits":sums.2.to_string(),
        "referenceCursor":sums.3.to_string(),
        "qualification":"HostOnly actual full sink source join; no independent exact B clock/fullprefix/native claim"}));
}
