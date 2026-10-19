//! Independent positive expectations frozen before execution; R258 body/output unread.
//! This test retains real full events. It does not generate expected values from a run.
//! Total B prefix/clock/arena/physical evidence remains outside this partial oracle.
use mori_core::sherlock_compile_artifact::draft_execution::run_draft_host;
use mori_core::sherlock::helper_machine::raw::{ConstRow, DataRow};
use mori_core::sherlock::helper_machine::shared::{
    BinaryOp, BoundaryKind, ControlC, EventKind, ExpressionRowC, FinanceInput,
    GrammarDomain, LocatorC, OutcomeC, OutcomeTagC, PatternRowC, ProgramView,
    RefC, RuleIdC, RunIntC, SliceC, SortTag, StepDisposition, StepEventC, TypeIdC, UnaryOp,
};
use serde_json::{json, Value};

const CONFIG: &str = include_str!("fixtures/draft-runtime-independent/config.json");
const TARGET: &str = include_str!("fixtures/draft-runtime-independent/target.json");
const PREPARATION: &str = include_str!("fixtures/draft-runtime-independent/EXPECTED-PREPARATION.json");
const EXPECTED: &str = include_str!("fixtures/draft-runtime-independent/EXPECTED-RUNTIME.json");
const CONFIG_SHA: &str = "3406712e1327ebe87fa9c5265986a6a51ef5856f3498d99ff4a8dd958d70ac17";
const TARGET_SHA: &str = "b7b9069acc831ae962d941c75a75c7bc2780e4029a569209458601f9b3f8d56d";

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
// R115 representation. No private production serializer/helper is called.
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
        // Neither sealed program has another literal-constant kind. Cons/Leaf/
        // Branch remain expression rows and are not pre-evaluated constants.
        _=>panic!("unexpected literal constant kind in sealed R115 program"),
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
        _=>panic!("call source location must be an original source occurrence"),
    }
}

fn compare_public_runtime(source: &str, case: &str) {
    let all: Value = serde_json::from_str(EXPECTED).expect("frozen runtime expectations");
    let expected = &all["cases"][case];
    let static_all: Value = serde_json::from_str(PREPARATION).expect("frozen R115 expectations");
    let static_expected = &static_all[case];
    let wanted: u128 = expected["returnedUInt"].as_str().unwrap().parse().unwrap();
    let wanted_calls = expected["successfulInvokeBindReturnCount"].as_u64().unwrap() as usize;
    let mut events: Vec<StepEventC> = Vec::new();
    let run = match run_draft_host(source, CONFIG, TARGET, &mut |event| events.push(event.clone())) {
        Ok(run)=>run,
        Err(failure)=>panic!("public draft runtime failed: {}",serde_json::to_string(&failure).unwrap()),
    };
    let observed = match run.observe() { Ok(v)=>v,Err(_)=>panic!("actual owner-bound observation failed") };
    let image = &observed.image;
    let summary = &run.completed.summary;

    // The exact same original source/declaration/provenance/type/table records.
    assert_eq!(serde_json::to_value(&run.preparation.source).unwrap(), static_expected["source"]);
    assert_eq!(serde_json::to_value(&run.preparation.preparation).unwrap(), static_expected["preparation"]);
    assert_eq!(run.preparation.admission["program_view"], static_expected["program_view"]);
    assert_eq!(program_view(observed.admission.view), static_expected["program_view"]);
    assert_eq!(run.preparation.source.source_digest, static_expected["source_digest"].as_str().unwrap());
    assert_eq!(run.preparation.config.file_digest, CONFIG_SHA);
    assert_eq!(run.preparation.target.file_digest, TARGET_SHA);
    assert_eq!(run.preparation.target.descriptor,serde_json::from_str::<Value>(TARGET).unwrap());
    let mut wanted_config: Value = serde_json::from_str(CONFIG).unwrap();
    wanted_config.as_object_mut().unwrap().insert("file_digest".into(),json!(CONFIG_SHA));
    assert_eq!(serde_json::to_value(&run.preparation.config).unwrap(),wanted_config);

    // This is explicitly the pre-run pair projection, distinct from completed.
    assert_eq!(run.preparation.genesis["control"],json!({"Eval":{"expr":"0"}}));
    assert_eq!(run.preparation.genesis["outcome"],json!("Running"));
    assert_eq!(run.preparation.genesis["initial_fuel"],json!("4096"));
    assert_eq!(run.preparation.genesis["remaining_work"],json!("4096"));
    for field in ["consumed_work","helper_work","helper_visits","reference_cursor"] {
        assert_eq!(run.preparation.genesis[field],json!("0"));
    }
    assert_eq!(run.preparation.genesis["frames"],json!([]));
    assert_eq!(run.preparation.genesis["env_root"],Value::Null);
    assert_eq!(run.preparation.genesis["helper"],Value::Null);
    assert_eq!(run.preparation.genesis["result"],Value::Null);

    let value = match *image.outcome {OutcomeC::Halted{value}=>value,_=>panic!("positive source did not halt with a value")};
    let actual_uint = match value {
        RefC::Const(i)=>match &observed.admission.view.constants[i] {ConstRow::U128{marker_free:true,n}=>*n,_=>panic!("terminal constant is not UInt")},
        RefC::Data(i)=>match &image.data[i] {DataRow::U128{n}=>*n,_=>panic!("terminal data is not UInt")},
    };
    assert_eq!(actual_uint,wanted,"independently derived source result");
    assert_eq!(*image.control,ControlC::Return{value});
    assert_eq!(summary.outcome,*image.outcome);
    assert!(image.frames.is_empty());
    assert_eq!(image.env_root,None,"outer call restores original empty environment root");
    assert!(image.helper.is_none());
    assert!(image.result.is_none());
    assert_eq!(image.initial_fuel,4096);
    assert_eq!(image.stack_limit,64);
    assert_eq!(image.run_id.fuel_input,RunIntC::Nat(4096));
    assert_eq!(image.run_id.stack_limit_input,RunIntC::Nat(64));
    assert_eq!(observed.admission.identity.profile_id,2);
    assert_eq!(observed.admission.identity.grammar_domain,GrammarDomain::DedicatedSorts);
    assert_eq!(observed.admission.identity.finance,FinanceInput::Absent);
    assert_eq!(observed.admission.identity.layout_version,("moriarty".into(),"sherlock-helper-microstep-hostonly/1".into()));
    assert!(image.consumed_work>0);
    assert_eq!(image.remaining_work.checked_add(image.consumed_work),Some(4096));
    assert_eq!(summary.events,u128::try_from(events.len()).unwrap());
    assert_eq!(run.completed.events_forwarded,summary.events);
    assert!(!events.is_empty());

    let mut work=0u128;
    let mut helper_work=0u128;
    let mut helper_visits=0u128;
    let mut reference_cursor=0u128;
    let mut call_ids=Vec::new();
    let mut call_occurrences=Vec::new();
    for (index,event) in events.iter().enumerate() {
        assert_eq!(event.step_id.ordinal,work,"owner ordinal is prior consumed work");
        assert_eq!(event.step_id.run_id.as_ref(),image.run_id);
        if index>0 {
            assert_eq!(event.logical_depth_before,events[index-1].logical_depth_after);
            assert_eq!(event.task_cursor_before,events[index-1].task_cursor_after);
        } else {
            assert_eq!(event.logical_depth_before,0);
            assert_eq!(event.task_cursor_before,0);
        }
        work=work.checked_add(u128::from(event.work_delta)).unwrap();
        helper_work=helper_work.checked_add(u128::from(event.helper_work_delta)).unwrap();
        helper_visits=helper_visits.checked_add(u128::from(event.helper_visits_delta)).unwrap();
        reference_cursor=reference_cursor.checked_add(u128::from(event.reference_cursor_delta)).unwrap();
        if index+1<events.len() {
            assert_eq!(event.disposition,StepDisposition::Applied);
            assert_eq!(event.work_delta,1);
            assert_eq!(event.outcome_tag,OutcomeTagC::Running);
            assert_eq!(event.boundary,BoundaryKind::None);
        }
        if event.rule_id==RuleIdC::KMachineEvalCallArguments {
            call_ids.push(event.source_expr_id);
            call_occurrences.push(occurrence(&event.source_occurrence));
        }
    }
    assert_eq!(json!(call_ids),expected["evalCallArgumentExprIds"]);
    assert_eq!(json!(call_occurrences),expected["evalCallArgumentOccurrences"]);
    for rule in [RuleIdC::KMachineInvokeFind,RuleIdC::KMachineEnterBind,RuleIdC::KMachineReturnCallValid] {
        assert_eq!(events.iter().filter(|e|e.rule_id==rule).count(),wanted_calls);
    }
    let last=events.last().unwrap();
    assert_eq!(last.rule_id,RuleIdC::KAdminPublishValue);
    assert_eq!(last.event_kind,EventKind::Publish);
    assert_eq!(last.disposition,StepDisposition::Published);
    assert_eq!(last.boundary,BoundaryKind::Terminal);
    assert_eq!(last.outcome_tag,OutcomeTagC::Halted);
    assert_eq!(last.outcome_payload,*image.outcome);
    assert_eq!((last.work_delta,last.helper_work_delta,last.helper_visits_delta,last.reference_cursor_delta),(0,0,0,0));
    assert_eq!((last.logical_depth_before,last.logical_depth_after),(0,0));
    assert_eq!((work,helper_work,helper_visits,reference_cursor),
        (image.consumed_work,image.helper_work,image.helper_visits,image.reference_cursor));
    assert_eq!((work,helper_work,helper_visits,reference_cursor),
        (summary.consumed_work,summary.helper_work,summary.helper_visits,summary.reference_cursor));
    let report=match run.resource_report(){Ok(v)=>v,Err(_)=>panic!("actual resource report failed")};
    assert_eq!(report.observed.consumed_work.0,work);
    assert_eq!(report.observed.helper_work.0,helper_work);
    assert_eq!(report.observed.helper_visits.0,helper_visits);
    assert_eq!(report.observed.reference_cursor.0,reference_cursor);
    assert_eq!(report.run_summary.events.0,summary.events);
    assert_eq!(report.observed.logical_depth,0);
}

#[test]
fn public_chain_total_executes_actual_owner_with_independent_result_and_event_contract(){
    compare_public_runtime(include_str!("fixtures/draft-runtime-independent/chain-total.mori"),"chain_total");
}
#[test]
fn public_branch_total_executes_actual_owner_with_independent_result_and_event_contract(){
    compare_public_runtime(include_str!("fixtures/draft-runtime-independent/branch-total.mori"),"branch_total");
}
