//! Explicit normal consumer of actual saved owner records, not Cargo autodiscovery.
//! Expected prefix bodies are independent opaque includes, never producer inputs.
//! New035metadata/work checks advance each mandatory real B image once.
//! Expected arrays are opaque includes and are parsed only after capture.
use mori_core::sherlock as a;
use a::helper_machine::{complete_observation as co, reference_projection as rp, witness_plan as wp, source_input_bridge as bridge};
use a::helper_machine::shared::*;
use a::helper_machine::shared::owner_host::host_owner_engine;
use serde_json::{json, Value as J};
use std::sync::Arc;

const FACT: &str = include_str!("../fixtures/witness-reference-prefix/factorial3-complete-A39-prefix.json");
const LIST: &str = include_str!("../fixtures/witness-reference-prefix/list2-5-complete-A39-prefix.json");
const TREE: &str = include_str!("../fixtures/witness-reference-prefix/tree4-1-3-complete-A39-prefix.json");
const PENDING: &str = include_str!("../fixtures/witness-reference-prefix/factorial-pending-caller3.json");

fn tree(t: &a::Tree) -> J { match t {
    a::Tree::Leaf(n) => json!({"Leaf":n}),
    a::Tree::Branch(l,r) => json!({"Branch":[tree(l),tree(r)]}),
} }
fn value(v: &a::Value) -> J { match v {
    a::Value::UInt(n)=>json!({"UInt":n}),a::Value::Bool(b)=>json!({"Bool":b}),
    a::Value::List(xs)=>json!({"List":xs}),a::Value::Tree(t)=>json!({"Tree":tree(t)}),
} }
fn env(e: &a::Environment) -> J { J::Array(e.iter().map(|(n,v)|json!({"name":n,"value":value(v)})).collect()) }
fn pattern(p:&a::Pattern)->J { match p {
    a::Pattern::Var(n)=>json!({"Var":n}),a::Pattern::Nil=>json!({"Nil":null}),
    a::Pattern::Cons(x,y)=>json!({"Cons":[x,y]}),a::Pattern::Leaf(n)=>json!({"Leaf":n}),
    a::Pattern::Branch(x,y)=>json!({"Branch":[x,y]}),
} }
fn arm(x:&a::Arm)->J {json!({"pattern":pattern(&x.pattern),"body":expr(&x.body)})}
fn arms(xs:&[a::Arm])->J {J::Array(xs.iter().map(arm).collect())}
fn expr(x:&a::Expr)->J {match x {
    a::Expr::Literal(v)=>json!({"Literal":value(v)}),a::Expr::Var(n)=>json!({"Var":n}),
    a::Expr::Let(n,x,y)=>json!({"Let":{"name":n,"value":expr(x),"body":expr(y)}}),
    a::Expr::Binary(op,x,y)=>json!({"Binary":{"op":format!("{op:?}"),"left":expr(x),"right":expr(y)}}),
    a::Expr::Unary(op,x)=>json!({"Unary":{"op":format!("{op:?}"),"operand":expr(x)}}),
    a::Expr::If(x,y,z)=>json!({"If":{"condition":expr(x),"yes":expr(y),"no":expr(z)}}),
    a::Expr::Call(n,xs)=>json!({"Call":{"name":n,"args":xs.iter().map(|x|expr(x)).collect::<Vec<_>>()}}),
    a::Expr::Match(x,xs)=>json!({"Match":{"value":expr(x),"arms":arms(xs)}}),
} }
fn function(f:&a::Function)->J {json!({"name":f.name,"parameters":f.parameters.iter().map(|p|json!({"name":p.name,"ty":format!("{:?}",p.ty)})).collect::<Vec<_>>(),"return_type":format!("{:?}",f.return_type),"body":expr(&f.body)})}
fn program(p:&a::Program)->J {json!({"functions":p.functions.iter().map(function).collect::<Vec<_>>(),"entry":expr(&p.entry)})}
fn frame(f:&a::Frame)->J {match f {
    a::Frame::Let(n,b,e)=>json!({"Let":{"name":n,"body":expr(b),"saved_environment":env(e)}}),
    a::Frame::Left(op,r,e)=>json!({"Left":{"op":format!("{op:?}"),"right":expr(r),"saved_environment":env(e)}}),
    a::Frame::Right(op,l)=>json!({"Right":{"op":format!("{op:?}"),"left":value(l)}}),
    a::Frame::Unary(op)=>json!({"Unary":{"op":format!("{op:?}")}}),
    a::Frame::If(y,n,e)=>json!({"If":{"yes":expr(y),"no":expr(n),"saved_environment":env(e)}}),
    a::Frame::Match(xs,e)=>json!({"Match":{"arms":arms(xs),"saved_environment":env(e)}}),
    a::Frame::Args(n,vs,xs,e)=>json!({"Args":{"function_name":n,"completed_values":vs.iter().map(value).collect::<Vec<_>>(),"remaining_exprs":xs.iter().map(|x|expr(x)).collect::<Vec<_>>(),"saved_environment":env(e)}}),
    a::Frame::Return(t,e)=>json!({"Return":{"expected_type":format!("{t:?}"),"saved_environment":env(e)}}),
    a::Frame::Scope(e)=>json!({"Scope":{"saved_environment":env(e)}}),
} }
fn control(c:&a::Control)->J {match c {
    a::Control::Eval(x)=>json!({"Eval":expr(x)}),a::Control::Return(v)=>json!({"Return":value(v)}),
    a::Control::Push(f,c,e)=>json!({"Push":{"frame":frame(f),"next":control(c),"environment":env(e)}}),
    a::Control::Invoke(n,vs)=>json!({"Invoke":{"name":n,"values":vs.iter().map(value).collect::<Vec<_>>()}}),
    a::Control::Enter(f,vs)=>json!({"Enter":{"function":f.as_ref().map(function),"values":vs.iter().map(value).collect::<Vec<_>>()}}),
    a::Control::Bound(e,t,b)=>json!({"Bound":{"bindings":e.as_ref().map(env),"type":format!("{t:?}"),"body":expr(b)}}),
    a::Control::Cases(v,xs,e)=>json!({"Cases":{"value":value(v),"arms":arms(xs),"saved_environment":env(e)}}),
    a::Control::CaseResult(b,v,x,xs,e)=>json!({"CaseResult":{"bindings":b.as_ref().map(env),"value":value(v),"body":expr(x),"remaining_arms":arms(xs),"saved_environment":env(e)}}),
    a::Control::PrimitiveResult(Ok(v))=>json!({"PrimitiveResult":{"Ok":value(v)}}),
    a::Control::PrimitiveResult(Err(c))=>json!({"PrimitiveResult":{"Err":c}}),
    a::Control::Error(c)=>json!({"Error":c}),
} }
fn outcome(o:&a::Outcome)->J {match o {
    a::Outcome::Pending=>json!("Pending"),a::Outcome::Done(v)=>json!({"Done":value(v)}),
    a::Outcome::Rejected(c)=>json!({"Rejected":c}),a::Outcome::Exhausted=>json!("Exhausted"),
} }
fn snapshot(s:&a::Snapshot)->J {json!({"program":program(&s.program),"control":s.control.as_ref().map(control),"environment":env(&s.environment),"frames_top_first":s.frames.iter().map(frame).collect::<Vec<_>>(),"stack_limit":s.stack_limit,"remaining_fuel":s.remaining_fuel,"initial_fuel":s.initial_fuel,"outcome":outcome(&s.outcome)})}

fn literal(v:a::Value)->a::ExprRef {Arc::new(a::Expr::Literal(v))}
fn uint(n:u128)->a::ExprRef {literal(a::Value::UInt(n))}
fn var(n:&str)->a::ExprRef {Arc::new(a::Expr::Var(n.into()))}
fn binary(op:a::BinaryOp,x:a::ExprRef,y:a::ExprRef)->a::ExprRef {Arc::new(a::Expr::Binary(op,x,y))}
fn call(n:&str,x:a::ExprRef)->a::ExprRef {Arc::new(a::Expr::Call(n.into(),vec![x]))}
fn one_function(n:&str,p:&str,t:a::Type,b:a::ExprRef,arg:a::ExprRef)->Arc<a::Program> {Arc::new(a::Program{functions:vec![a::Function{name:n.into(),parameters:vec![a::Parameter{name:p.into(),ty:t}],return_type:a::Type::UInt,body:b}],entry:call(n,arg)})}
fn factorial()->Arc<a::Program> {one_function("fact","n",a::Type::UInt,Arc::new(a::Expr::If(binary(a::BinaryOp::Eq,var("n"),uint(0)),uint(1),binary(a::BinaryOp::Mul,var("n"),call("fact",binary(a::BinaryOp::Sub,var("n"),uint(1)))))),uint(3))}
fn list_sum()->Arc<a::Program> {one_function("sum_list","xs",a::Type::List,Arc::new(a::Expr::Match(var("xs"),vec![a::Arm{pattern:a::Pattern::Nil,body:uint(0)},a::Arm{pattern:a::Pattern::Cons("h".into(),"t".into()),body:binary(a::BinaryOp::Add,var("h"),call("sum_list",var("t")))}])),literal(a::Value::List(vec![2,5])))}
fn tree_sum()->Arc<a::Program> {one_function("sum_tree","tr",a::Type::Tree,Arc::new(a::Expr::Match(var("tr"),vec![a::Arm{pattern:a::Pattern::Leaf("n".into()),body:var("n")},a::Arm{pattern:a::Pattern::Branch("l".into(),"r".into()),body:binary(a::BinaryOp::Add,call("sum_tree",var("l")),call("sum_tree",var("r")))}])),literal(a::Value::Tree(a::Tree::Branch(Arc::new(a::Tree::Leaf(4)),Arc::new(a::Tree::Branch(Arc::new(a::Tree::Leaf(1)),Arc::new(a::Tree::Leaf(3))))))))}

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
        }, boundary_capacities_present: false,
    };
    (bounds, schema)
}

fn tooling<T>(r:Result<T,Tooling>)->T {match r {
    Ok(x)=>x,
    Err(Tooling::AdmissionRejected{reason,detail,locator})=>panic!("admission {reason:?} {detail} {locator:?}"),
    Err(Tooling::CapacityNotAdmitted{kind,needed,limit,locator})=>panic!("capacity {kind:?} {needed:?}/{limit} {locator:?}"),
    Err(Tooling::UnsupportedFeature{feature,locator})=>panic!("feature {feature:?} {locator:?}"),
}}

fn b_rule_id(rule:RuleIdC)->&'static str {match rule {
    RuleIdC::KMachineEvalLiteralValid=>"k.machine.eval.literal.valid",
    RuleIdC::KMachineEvalLiteralInvalid=>"k.machine.eval.literal.invalid",
    RuleIdC::KMachineEvalVariablePresent=>"k.machine.eval.variable.present",
    RuleIdC::KMachineEvalVariableAbsent=>"k.machine.eval.variable.absent",
    RuleIdC::KMachineEvalLet=>"k.machine.eval.let",
    RuleIdC::KMachineEvalBinary=>"k.machine.eval.binary",
    RuleIdC::KMachineEvalUnary=>"k.machine.eval.unary",
    RuleIdC::KMachineEvalIf=>"k.machine.eval.if",
    RuleIdC::KMachineEvalMatch=>"k.machine.eval.match",
    RuleIdC::KMachineEvalCallEmpty=>"k.machine.eval.call.empty",
    RuleIdC::KMachineEvalCallArguments=>"k.machine.eval.call.arguments",
    RuleIdC::KMachinePushAccept=>"k.machine.push.accept",
    RuleIdC::KMachinePushRefuse=>"k.machine.push.refuse",
    RuleIdC::KMachineReturnLet=>"k.machine.return.let",
    RuleIdC::KMachineReturnLeft=>"k.machine.return.left",
    RuleIdC::KMachineReturnRight=>"k.machine.return.right",
    RuleIdC::KMachineReturnUnary=>"k.machine.return.unary",
    RuleIdC::KMachineReturnIfTrue=>"k.machine.return.if.true",
    RuleIdC::KMachineReturnIfFalse=>"k.machine.return.if.false",
    RuleIdC::KMachineReturnIfInvalid=>"k.machine.return.if.invalid",
    RuleIdC::KMachineReturnMatch=>"k.machine.return.match",
    RuleIdC::KMachineReturnArgumentsNext=>"k.machine.return.arguments.next",
    RuleIdC::KMachineReturnArgumentsFinal=>"k.machine.return.arguments.final",
    RuleIdC::KMachineReturnCallValid=>"k.machine.return.call.valid",
    RuleIdC::KMachineReturnCallInvalid=>"k.machine.return.call.invalid",
    RuleIdC::KMachineReturnScope=>"k.machine.return.scope",
    RuleIdC::KMachineInvokeFind=>"k.machine.invoke.find",
    RuleIdC::KMachineEnterMissing=>"k.machine.enter.missing",
    RuleIdC::KMachineEnterBind=>"k.machine.enter.bind",
    RuleIdC::KMachineBoundInvalid=>"k.machine.bound.invalid",
    RuleIdC::KMachineBoundValid=>"k.machine.bound.valid",
    RuleIdC::KMachineCasesEmpty=>"k.machine.cases.empty",
    RuleIdC::KMachineCasesTry=>"k.machine.cases.try",
    RuleIdC::KMachineCasesMiss=>"k.machine.cases.miss",
    RuleIdC::KMachineCasesHit=>"k.machine.cases.hit",
    RuleIdC::KMachinePrimitiveValue=>"k.machine.primitive.value",
    RuleIdC::KMachinePrimitiveError=>"k.machine.primitive.error",
    RuleIdC::BHelperScalarEntry=>"b.helper.Scalar.entry",
    RuleIdC::BHelperScalarTag=>"b.helper.Scalar.tag",
    RuleIdC::BHelperScalarArithmetic=>"b.helper.Scalar.arithmetic",
    RuleIdC::BHelperScalarConstruct=>"b.helper.Scalar.construct",
    RuleIdC::BHelperScalarExit=>"b.helper.Scalar.exit",
    RuleIdC::BHelperScalarFailure=>"b.helper.Scalar.failure",
    RuleIdC::BHelperEnvGetEntry=>"b.helper.EnvGet.entry",
    RuleIdC::BHelperEnvGetCell=>"b.helper.EnvGet.cell",
    RuleIdC::BHelperEnvGetEmpty=>"b.helper.EnvGet.empty",
    RuleIdC::BHelperEnvGetExit=>"b.helper.EnvGet.exit",
    RuleIdC::BHelperEnvGetFailure=>"b.helper.EnvGet.failure",
    RuleIdC::BHelperEnvPutEntry=>"b.helper.EnvPut.entry",
    RuleIdC::BHelperEnvPutCell=>"b.helper.EnvPut.cell",
    RuleIdC::BHelperEnvPutInsert=>"b.helper.EnvPut.insert",
    RuleIdC::BHelperEnvPutEmpty=>"b.helper.EnvPut.empty",
    RuleIdC::BHelperEnvPutExit=>"b.helper.EnvPut.exit",
    RuleIdC::BHelperEnvPutFailure=>"b.helper.EnvPut.failure",
    RuleIdC::BHelperFindEntry=>"b.helper.Find.entry",
    RuleIdC::BHelperFindRow=>"b.helper.Find.row",
    RuleIdC::BHelperFindEmpty=>"b.helper.Find.empty",
    RuleIdC::BHelperFindExit=>"b.helper.Find.exit",
    RuleIdC::BHelperFindFailure=>"b.helper.Find.failure",
    RuleIdC::BHelperBindEntry=>"b.helper.Bind.entry",
    RuleIdC::BHelperBindPair=>"b.helper.Bind.pair",
    RuleIdC::BHelperBindEmpty=>"b.helper.Bind.empty",
    RuleIdC::BHelperBindArity=>"b.helper.Bind.arity",
    RuleIdC::BHelperBindType=>"b.helper.Bind.type",
    RuleIdC::BHelperBindDuplicate=>"b.helper.Bind.duplicate",
    RuleIdC::BHelperBindInsert=>"b.helper.Bind.insert",
    RuleIdC::BHelperBindPropagate=>"b.helper.Bind.propagate",
    RuleIdC::BHelperBindExit=>"b.helper.Bind.exit",
    RuleIdC::BHelperBindFailure=>"b.helper.Bind.failure",
    RuleIdC::BHelperAppendEntry=>"b.helper.Append.entry",
    RuleIdC::BHelperAppendCell=>"b.helper.Append.cell",
    RuleIdC::BHelperAppendEmpty=>"b.helper.Append.empty",
    RuleIdC::BHelperAppendTail=>"b.helper.Append.tail",
    RuleIdC::BHelperAppendCopy=>"b.helper.Append.copy",
    RuleIdC::BHelperAppendExit=>"b.helper.Append.exit",
    RuleIdC::BHelperAppendFailure=>"b.helper.Append.failure",
    RuleIdC::BHelperValidateEntry=>"b.helper.Validate.entry",
    RuleIdC::BHelperValidateOccurrence=>"b.helper.Validate.occurrence",
    RuleIdC::BHelperValidateField=>"b.helper.Validate.field",
    RuleIdC::BHelperValidateDeclaration=>"b.helper.Validate.declaration",
    RuleIdC::BHelperValidateEmpty=>"b.helper.Validate.empty",
    RuleIdC::BHelperValidateInvalid=>"b.helper.Validate.invalid",
    RuleIdC::BHelperValidateExit=>"b.helper.Validate.exit",
    RuleIdC::BHelperValidateFailure=>"b.helper.Validate.failure",
    RuleIdC::BHelperEqualEntry=>"b.helper.Equal.entry",
    RuleIdC::BHelperEqualPair=>"b.helper.Equal.pair",
    RuleIdC::BHelperEqualField=>"b.helper.Equal.field",
    RuleIdC::BHelperEqualEmpty=>"b.helper.Equal.empty",
    RuleIdC::BHelperEqualDifferent=>"b.helper.Equal.different",
    RuleIdC::BHelperEqualExit=>"b.helper.Equal.exit",
    RuleIdC::BHelperEqualFailure=>"b.helper.Equal.failure",
    RuleIdC::BHelperMatchEntry=>"b.helper.Match.entry",
    RuleIdC::BHelperMatchArm=>"b.helper.Match.arm",
    RuleIdC::BHelperMatchOccurrence=>"b.helper.Match.occurrence",
    RuleIdC::BHelperMatchField=>"b.helper.Match.field",
    RuleIdC::BHelperMatchBinding=>"b.helper.Match.binding",
    RuleIdC::BHelperMatchDistinct=>"b.helper.Match.distinct",
    RuleIdC::BHelperMatchDiscard=>"b.helper.Match.discard",
    RuleIdC::BHelperMatchNext=>"b.helper.Match.next",
    RuleIdC::BHelperMatchSelected=>"b.helper.Match.selected",
    RuleIdC::BHelperMatchEmpty=>"b.helper.Match.empty",
    RuleIdC::BHelperMatchExit=>"b.helper.Match.exit",
    RuleIdC::BHelperMatchFailure=>"b.helper.Match.failure",
    RuleIdC::BBookkeepingTaskEnqueue=>"b.bookkeeping.task.enqueue",
    RuleIdC::BBookkeepingTaskRelease=>"b.bookkeeping.task.release",
    RuleIdC::BBookkeepingResumeStore=>"b.bookkeeping.resume.store",
    RuleIdC::BBookkeepingResumeRestore=>"b.bookkeeping.resume.restore",
    RuleIdC::KAdminStartupValid=>"k.admin.startup.valid",
    RuleIdC::KAdminStartupInvalid=>"k.admin.startup.invalid",
    RuleIdC::KAdminPublishValue=>"k.admin.publish.value",
    RuleIdC::KAdminPublishError=>"k.admin.publish.error",
    RuleIdC::KAdminPublishExhausted=>"k.admin.publish.exhausted",
}}


const FACT_B0:&str=include_str!("../fixtures/witness-reference-work-independent/factorial3-complete-A39-prefix-B0-clocks.json");
const LIST_B0:&str=include_str!("../fixtures/witness-reference-work-independent/list2-5-complete-A39-prefix-B0-clocks.json");
const TREE_B0:&str=include_str!("../fixtures/witness-reference-work-independent/tree4-1-3-complete-A39-prefix-B0-clocks.json");
const STRATEGIES:[&str;7]=["uncached","last-use","cold-retain-all","warm-last-use","streamed","warm-retain-all","cleared-retain-all"];
#[derive(Default)]
struct CopyObservations { value:bool, environment:bool, cache:bool }

fn metadata_matches_raw(raw:&co::HostSnapshot,decoded:&wp::DecodedSnapshot) {
    let m=&decoded.metadata;let r=&raw.image;
    assert_eq!(m.admission,raw.admission);assert_eq!(m.run_id,r.run_id);
    assert_eq!(m.initial_fuel,r.initial_fuel);assert_eq!(m.stack_limit,r.stack_limit);
    assert_eq!(m.remaining_work,r.remaining_work);assert_eq!(m.consumed_work,r.consumed_work);
    assert_eq!(m.helper_work,r.helper_work);assert_eq!(m.helper_visits,r.helper_visits);
    assert_eq!(m.reference_cursor,r.reference_cursor);
    assert_eq!(m.consumed_work.checked_sub(m.helper_work),Some(m.reference_cursor));
    assert_eq!(m.remaining_work.checked_add(m.consumed_work),Some(m.initial_fuel));
}
fn reference_work(decoded:&wp::DecodedSnapshot,reference:&a::Snapshot) {
    let m=&decoded.metadata;
    assert_eq!(m.initial_fuel,reference.initial_fuel);
    let work=reference.initial_fuel.checked_sub(reference.remaining_fuel).expect("complete A work");
    assert_eq!(m.reference_cursor,work);
    assert_eq!(m.consumed_work.checked_sub(m.helper_work),Some(work));
}
fn variants(s:&wp::StrategyMaterializedSnapshot,copies:&mut CopyObservations)->Vec<wp::DecodedSnapshot> {
    let baseline=wp::evaluate_uncached(&s.plan,&s.raw).expect("actual uncached witness");
    let last=wp::evaluate_last_use(&s.plan,&s.raw).expect("actual last-use witness");
    let mut cache=wp::WitnessCache::default();
    let cold=wp::evaluate_memoized(&s.plan,&s.raw,&mut cache,wp::SlotStorage::RetainAll).expect("actual cold witness");
    let warm=wp::evaluate_memoized(&s.plan,&s.raw,&mut cache,wp::SlotStorage::LastUse).expect("actual warm last-use witness");
    let warm_all=wp::evaluate_memoized(&s.plan,&s.raw,&mut cache,wp::SlotStorage::RetainAll).expect("actual warm retain-all witness");
    assert!(cold.scheduling.scope_reset);assert_eq!(cold.scheduling.cache_hits,0);
    assert_eq!(cold.scheduling.copied_cached_slot_values,0);
    assert_eq!(cold.witness.diagnostics,baseline.diagnostics);
    let memo_nodes=s.plan.nodes.len()-s.plan.inputs.len();
    assert!(memo_nodes>0);
    for result in [&warm,&warm_all] {
        assert!(!result.scheduling.scope_reset);assert_eq!(result.scheduling.cache_misses,0);
        assert_eq!(result.scheduling.cache_hits,memo_nodes);
        assert_eq!(result.scheduling.copied_cached_slot_values,result.scheduling.cache_hits);
        assert_eq!(result.scheduling.cached_value_slots,memo_nodes);
        assert_eq!(result.scheduling.cached_argument_slots,cache.argument_slots());
        assert!(result.scheduling.cached_argument_slots>0);
        assert_eq!(result.witness.diagnostics.input_reads,s.plan.inputs.len());
        assert_eq!(result.witness.diagnostics.value_constructions,0);
        assert_eq!(result.witness.diagnostics.environment_constructions,0);
        assert_eq!(result.witness.diagnostics.payload_constructions,0);
        assert_eq!(result.witness.diagnostics.copied_value_nodes,0);
        assert_eq!(result.witness.diagnostics.copied_environment_entries,0);
        assert_eq!(result.witness.diagnostics.measured_host_bytes,None);
        assert_eq!(result.witness.diagnostics.measured_host_time_ns,None);
    }
    let live=s.plan.live_outs.iter().copied().collect::<std::collections::BTreeSet<_>>().len();
    for result in [&last,&warm] {
        assert_eq!(result.scheduling.retained_slots_at_publication,live);
        assert_eq!(result.scheduling.reclaimed_slots,s.plan.nodes.len()-live);
        assert!(result.scheduling.peak_live_slots>=live);
        assert!(result.scheduling.peak_live_slots<=s.plan.nodes.len());
    }
    for result in [&cold,&warm_all] {
        assert_eq!(result.scheduling.retained_slots_at_publication,s.plan.nodes.len());
        assert_eq!(result.scheduling.reclaimed_slots,0);
        assert_eq!(result.scheduling.peak_live_slots,s.plan.nodes.len());
    }
    assert_eq!(last.witness.diagnostics.copied_value_nodes,baseline.diagnostics.copied_value_nodes);
    assert_eq!(last.witness.diagnostics.copied_environment_entries,baseline.diagnostics.copied_environment_entries);
    copies.value|=baseline.diagnostics.copied_value_nodes>0;
    copies.environment|=baseline.diagnostics.copied_environment_entries>0;
    copies.cache|=warm.scheduling.copied_cached_slot_values>0;
    cache.clear();assert_eq!(cache.value_slots(),0);assert_eq!(cache.argument_slots(),0);
    let cleared=wp::evaluate_memoized(&s.plan,&s.raw,&mut cache,wp::SlotStorage::RetainAll).expect("clear then rematerialize actual witness");
    assert!(cleared.scheduling.scope_reset);assert_eq!(cleared.scheduling.cache_hits,0);
    assert_eq!(cleared.scheduling.copied_cached_slot_values,0);
    assert_eq!(cleared.witness.diagnostics,baseline.diagnostics);
    let results=vec![baseline.decoded,last.witness.decoded,cold.witness.decoded,warm.witness.decoded,s.evaluation.witness.decoded.clone(),warm_all.witness.decoded,cleared.witness.decoded];
    assert_eq!(results.len(),STRATEGIES.len());
    for decoded in &results {assert_eq!(decoded,&results[0]);metadata_matches_raw(&s.raw,decoded);}
    results
}
fn verify_record_counters(record:&co::CompleteStepRecord) {
    let (b,a,e)=(&record.before.image,&record.after.image,&record.event);
    assert_eq!(a.run_id,b.run_id);assert_eq!(a.initial_fuel,b.initial_fuel);
    assert_eq!(Some(a.consumed_work),b.consumed_work.checked_add(e.work_delta as u128));
    assert_eq!(Some(a.helper_work),b.helper_work.checked_add(e.helper_work_delta as u128));
    assert_eq!(Some(a.helper_visits),b.helper_visits.checked_add(e.helper_visits_delta as u128));
    assert_eq!(Some(a.reference_cursor),b.reference_cursor.checked_add(e.reference_cursor_delta as u128));
    assert_eq!(a.remaining_work.checked_add(e.work_delta as u128),Some(b.remaining_work));
    assert_eq!(a.remaining_work.checked_add(a.consumed_work),Some(a.initial_fuel));
    assert_eq!(e.outcome_payload,a.outcome);
}
fn clock_columns()->J {json!(["A_record_index_or_null","preceding_B_rule_id_or_null","consumed_work","helper_work","helper_visits","reference_cursor","remaining_work","erased_semantic_A_snapshot_index"])}
fn decoded_clock(a:Option<usize>,rule:Option<&str>,m:&wp::Metadata,semantic:usize)->J {
    json!([a,rule,m.consumed_work,m.helper_work,m.helper_visits,m.reference_cursor,m.remaining_work,semantic])
}
fn raw_clock(a:Option<usize>,rule:Option<&str>,r:&co::HostSnapshot,semantic:usize)->J {
    let m=&r.image;json!([a,rule,m.consumed_work,m.helper_work,m.helper_visits,m.reference_cursor,m.remaining_work,semantic])
}
fn interval_rows(spans:&[rp::ReferenceSpan])->J {
    let mut rows=Vec::new();
    for (i,span) in spans.iter().enumerate() {
        assert_eq!(span.source_event.checked_add(1),Some(span.b_end_exclusive));
        let mut helpers=Vec::new();let mut completion=None;
        for (offset,r) in span.raw.iter().enumerate() {
            verify_record_counters(r);
            if span.b_start+offset==span.source_event {
                assert!(matches!(r.event.event_kind,EventKind::LegacyComplete|EventKind::Publish));
                completion=Some(b_rule_id(r.event.rule_id));
            } else {
                assert!(!matches!(r.event.event_kind,EventKind::LegacyComplete|EventKind::Publish));
                if r.event.work_delta!=0 {helpers.push(b_rule_id(r.event.rule_id));}
            }
        }
        rows.push(json!({"A_record_index":i,"A_rule_line":span.reference.rule_start_line,"first_B_record":span.b_start,"B_records_exclusive":span.b_end_exclusive,"helper_rule_ids":helpers,"legacy_rule_id":completion.expect("actual interval completion")}));
    }
    json!(rows)
}
fn compare_clocks(initial:&co::HostSnapshot,raw_rows:&[J],decoded_rows:&[Vec<J>],spans:&[rp::ReferenceSpan],frozen:&str) {
    // All actual raw/decoded arrays are complete before expected cells are parsed.
    let intervals=interval_rows(spans);let columns=clock_columns();
    let expected:J=serde_json::from_str(frozen).expect("prefrozen independent full B0 schedule");
    let profile=match initial.admission.identity.profile_id {2=>"sherlock-helper-microstep/1",_=>panic!("selected admitted B profile")};
    assert_eq!(expected["profile"],json!(profile));
    assert_eq!(expected["grammarDomain"],json!(format!("{:?}",initial.admission.identity.grammar_domain)));
    assert_eq!(expected["initialFuel"],json!(initial.image.initial_fuel));
    assert_eq!(expected["clockPrefixColumns"],columns);
    assert_eq!(expected["clockPrefixes"],json!(raw_rows));assert_eq!(expected["intervals"],intervals);
    assert_eq!(decoded_rows.len(),STRATEGIES.len());
    for (name,rows) in STRATEGIES.iter().zip(decoded_rows) {
        assert_eq!(json!(rows),expected["clockPrefixes"],"independent complete decoded clocks: {name}");
    }
}
fn exercise(p:Arc<a::Program>,frozen:&str,pending:Option<&str>,b0:&str,compound:bool) {
    let budget=bridge::SourceCopyBudget{max_functions:4,max_parameters_per_function:8,max_total_parameters:8,max_expr_nodes:128,max_copy_depth:32,max_match_arms:16,max_name_bytes:512,max_list_elements:64,max_tree_nodes:64,max_tree_depth:32};
    let core=bridge::from_program_bounded(&p,&budget).unwrap_or_else(|_|panic!("bounded original source copy"));
    let (bounds,schema)=layout();let owner=host_owner_engine();
    let admitted=tooling(owner.admit(&core,&bounds,&schema));
    let mut image=tooling(owner.admit_run(&admitted,&RunIntC::Nat(4096),&RunIntC::Nat(64),&FinanceInput::Absent));
    let initial=co::capture(&admitted,&image).unwrap_or_else(|_|panic!("actual initial snapshot"));
    let source=rp::ReferenceSource::new(p.clone(),&initial).expect("complete source association");
    let mut projection=rp::ReferencePrefixProjection::new(p.clone(),&initial,4096).expect("initial reference projection");
    let mut cache=wp::WitnessRecordCache::default();let mut previous=initial.clone();
    let mut copies=CopyObservations::default();let mut spans=Vec::new();let mut raw_count=0usize;
    let mut raw_rows=vec![raw_clock(None,None,&initial,0)];
    let mut decoded_rows:Vec<Vec<J>>=vec![Vec::new();STRATEGIES.len()];
    let mut span_before:Option<Vec<wp::DecodedSnapshot>>=None;
    let mut first_callback=true;let mut stopped=false;
    let cleared_stream_cache=std::cell::Cell::new(false);
    let mut sink=|step:&wp::StrategyObservedStep| {
        assert_eq!(step.before.raw,previous);
        if cleared_stream_cache.replace(false) {
            assert!(step.before.evaluation.scheduling.scope_reset);
            assert!(step.after.evaluation.scheduling.scope_reset);
            assert_eq!(step.before.evaluation.scheduling.cache_hits,0);
            assert_eq!(step.after.evaluation.scheduling.cache_hits,0);
        }
        let before=variants(&step.before,&mut copies);let after=variants(&step.after,&mut copies);
        if first_callback {
            for (rows,decoded) in decoded_rows.iter_mut().zip(&before) {
                reference_work(decoded,projection.initial());
                rows.push(decoded_clock(None,None,&decoded.metadata,0));
            }
            first_callback=false;
        }
        match &step.observed {
            co::ObservedStep::Event(record)=>{
                assert_eq!(record.before,step.before.raw);assert_eq!(record.after,step.after.raw);verify_record_counters(record);
                if projection.pending_records().is_empty() {assert!(span_before.is_none());span_before=Some(before.clone());}
                let a_index=projection.records().len();raw_count+=1;
                match projection.push(record).expect("actual saved B record projection") {
                    rp::ProjectionProgress::Pending{raw}=>assert_eq!(raw,*record),
                    rp::ProjectionProgress::Complete(span)=>{
                        assert_eq!(span.b_start,spans.last().map_or(0,|s:&rp::ReferenceSpan|s.b_end_exclusive));
                        assert_eq!(span.b_end_exclusive,raw_count);assert_eq!(span.raw.len(),span.b_end_exclusive-span.b_start);
                        let origin=&span.raw[span.source_event-span.b_start].event;
                        let occurrence=span.raw[0].before.admission.view.occurrences.get(origin.source_expr_id).expect("actual source occurrence");
                        let end=occurrence.path.start.checked_add(occurrence.path.len).expect("occurrence path");
                        assert_eq!(origin.source_occurrence,LocatorC::Occurrence{owner_is_function:occurrence.owner_is_function,function:occurrence.function,ast_path:span.raw[0].before.admission.view.occurrence_paths[occurrence.path.start..end].to_vec()});
                        for decoded in &after {
                            reference_work(decoded,&span.reference.after);
                            let boundary=rp::project_reference_boundary(&source,&step.after.raw,decoded,decoded.metadata.initial_fuel,span.reference.after.remaining_fuel).expect("actual decoded after boundary");
                            let rp::ReferenceBoundary::Complete{reference,..}=boundary else {panic!("complete after boundary")};assert_eq!(reference,span.reference.after);
                        }
                        let first=span.raw.first().expect("nonempty actual span");
                        for decoded in span_before.take().expect("saved first span boundary") {
                            metadata_matches_raw(&first.before,&decoded);reference_work(&decoded,&span.reference.before);
                            let boundary=rp::project_reference_boundary(&source,&first.before,&decoded,decoded.metadata.initial_fuel,span.reference.before.remaining_fuel).expect("actual decoded first-span boundary");
                            let rp::ReferenceBoundary::Complete{reference,..}=boundary else {panic!("complete before boundary")};assert_eq!(reference,span.reference.before);
                        }
                        spans.push(span);
                    }
                }
                let rule=b_rule_id(record.event.rule_id);let semantic=projection.records().len();
                raw_rows.push(raw_clock(Some(a_index),Some(rule),&record.after,semantic));
                for (rows,decoded) in decoded_rows.iter_mut().zip(&after) {rows.push(decoded_clock(Some(a_index),Some(rule),&decoded.metadata,semantic));}
            },
            co::ObservedStep::Terminal{before:raw_before,after:raw_after}=>{
                assert_eq!(raw_before,raw_after);assert_eq!(raw_before,&step.before.raw);assert_eq!(raw_after,&step.after.raw);assert_eq!(before,after);
            }
        }
        previous=step.after.raw.clone();
        if !stopped {stopped=true;wp::StreamAction::Stop}else{wp::StreamAction::Continue}
    };
    let first=wp::stream_owner_steps(&admitted,&mut image,Some(&mut cache),&mut sink).unwrap_or_else(|_|panic!("actual first record"));assert_eq!(first,wp::StreamStop::Sink);
    cache.clear();cleared_stream_cache.set(true);
    let resumed=wp::stream_owner_steps(&admitted,&mut image,Some(&mut cache),&mut sink).unwrap_or_else(|_|panic!("same image continuation"));assert_eq!(resumed,wp::StreamStop::Terminal);drop(sink);
    assert!(stopped);assert!(!cleared_stream_cache.get());assert!(span_before.is_none());assert!(copies.cache);
    if compound {assert!(copies.value);assert!(copies.environment);}
    let returned=co::capture(&admitted,&image).unwrap_or_else(|_|panic!("actual returned boundary"));assert_eq!(returned,previous);
    let final_reference=projection.finish().expect("no omitted trailing helper interval");assert!(projection.pending_records().is_empty());
    assert_eq!(spans.iter().map(|s|s.raw.len()).sum::<usize>(),raw_count);assert_eq!(projection.records().len(),spans.len());
    assert_eq!(raw_rows.len(),raw_count+1);for rows in &decoded_rows {assert_eq!(rows.len(),raw_rows.len());}
    compare_clocks(&initial,&raw_rows,&decoded_rows,&spans,b0);
    let mut states=vec![projection.initial().clone()];
    for (i,r) in projection.records().iter().enumerate() {assert_eq!(&r.before,&states[i]);states.push(r.after.clone());}
    assert_eq!(states.last(),Some(final_reference));
    let snapshots=states.iter().map(snapshot).collect::<Vec<_>>();
    let records=projection.records().iter().enumerate().map(|(i,r)|json!({"rule_start_line":r.rule_start_line,"charge":r.charge,"before_snapshot":i,"after_snapshot":i+1})).collect::<Vec<_>>();
    let work=states.iter().map(|s|s.initial_fuel.checked_sub(s.remaining_fuel).expect("reference work invariant")).collect::<Vec<_>>();
    let charged:u128=projection.records().iter().map(|r|r.charge as u128).sum();assert_eq!(work.last().copied(),Some(charged));
    let actual=json!({"complete_program":program(&p),"snapshots":snapshots,"records":records,"prefix_work_A":work,"charged_A_work":charged,"A_event_count":projection.records().len(),"terminal_outcome":outcome(&final_reference.outcome),"logical_depth_peak_A":states.iter().map(|s|s.frames.len()).max().unwrap()});
    let expected:J=serde_json::from_str(frozen).expect("independent full A prefix");
    for key in ["complete_program","snapshots","records","prefix_work_A","charged_A_work","A_event_count","terminal_outcome","logical_depth_peak_A"] {assert!(expected.get(key).is_some());assert_eq!(&actual[key],&expected[key],"complete independent A field {key}");}
    if let Some(frozen_pending)=pending {
        let selected:J=serde_json::from_str(frozen_pending).expect("independent pending-caller fixture");
        let indices=selected["A_snapshot_indices"].as_array().expect("pending indices");let expected_states=selected["snapshots"].as_array().expect("pending states");assert!(!indices.is_empty());assert_eq!(indices.len(),expected_states.len());
        for (index,expected) in indices.iter().zip(expected_states) {let i=usize::try_from(index.as_u64().expect("snapshot index")).expect("index width");assert_eq!(&snapshot(states.get(i).expect("actual pending caller")),expected);}
    }
    assert_eq!(spans.last().expect("publication span").raw.last().unwrap().after,returned);
}
#[test]
fn factorial_three_decoded_metadata_reference_work_and_b0(){exercise(factorial(),FACT,Some(PENDING),FACT_B0,false);}
#[test]
fn list_two_five_decoded_metadata_reference_work_and_b0(){exercise(list_sum(),LIST,None,LIST_B0,true);}
#[test]
fn tree_four_one_three_decoded_metadata_reference_work_and_b0(){exercise(tree_sum(),TREE,None,TREE_B0,true);}

fn admit_program(p:&a::Program,schema:&LayoutSchemaC)->Admitted {
    let budget=bridge::SourceCopyBudget{max_functions:4,max_parameters_per_function:8,max_total_parameters:8,max_expr_nodes:128,max_copy_depth:32,max_match_arms:16,max_name_bytes:512,max_list_elements:64,max_tree_nodes:64,max_tree_depth:32};
    let core=bridge::from_program_bounded(p,&budget).unwrap_or_else(|_|panic!("valid bounded contrast input"));
    tooling(host_owner_engine().admit(&core,&schema.bounds,schema))
}
fn admitted_initial(admitted:&Admitted,fuel:u128,stack:u128)->co::HostSnapshot {
    let image=tooling(host_owner_engine().admit_run(admitted,&RunIntC::Nat(fuel),&RunIntC::Nat(stack),&FinanceInput::Absent));
    co::capture(admitted,&image).unwrap_or_else(|_|panic!("valid admitted initial contrast"))
}
fn cached_actual(raw:&co::HostSnapshot,cache:&mut wp::WitnessCache)->wp::StrategyEvaluation {
    let plan=wp::construct_plan(raw).expect("actual contrast plan");
    let expected=wp::evaluate_uncached(&plan,raw).expect("actual uncached contrast");
    let actual=wp::evaluate_memoized(&plan,raw,cache,wp::SlotStorage::RetainAll).expect("actual memoized contrast");
    assert_eq!(actual.witness.decoded,expected.decoded);metadata_matches_raw(raw,&actual.witness.decoded);actual
}
#[test]
fn valid_program_layout_run_inputs_clear_cache_contrasts() {
    let (_,schema)=layout();let program=factorial();let admitted=admit_program(&program,&schema);
    let initial=admitted_initial(&admitted,4096,64);let changed_run=admitted_initial(&admitted,4095,63);
    assert_eq!(initial.admission,changed_run.admission);assert_ne!(initial.image.initial_fuel,changed_run.image.initial_fuel);assert_ne!(initial.image.stack_limit,changed_run.image.stack_limit);
    let mut cache=wp::WitnessCache::default();let cold=cached_actual(&initial,&mut cache);assert!(cold.scheduling.scope_reset);assert_eq!(cold.scheduling.cache_hits,0);
    let changed=cached_actual(&changed_run,&mut cache);assert!(!changed.scheduling.scope_reset);assert!(changed.scheduling.cache_hits>0);assert!(changed.scheduling.cache_misses>0);
    let warm=cached_actual(&changed_run,&mut cache);assert!(!warm.scheduling.scope_reset);assert_eq!(warm.scheduling.cache_misses,0);assert!(warm.scheduling.cache_hits>0);
    cache.clear();assert_eq!(cache.value_slots(),0);assert_eq!(cache.argument_slots(),0);let cleared=cached_actual(&changed_run,&mut cache);assert!(cleared.scheduling.scope_reset);assert_eq!(cleared.scheduling.cache_hits,0);
    let mut layout_changed=schema.clone();layout_changed.capacities.name_rows-=1;
    let layout_admitted=admit_program(&program,&layout_changed);let layout_initial=admitted_initial(&layout_admitted,4096,64);
    assert_eq!(initial.admission.view,layout_initial.admission.view);assert_ne!(initial.admission.layout,layout_initial.admission.layout);
    let layout_result=cached_actual(&layout_initial,&mut cache);assert!(layout_result.scheduling.scope_reset);assert_eq!(layout_result.scheduling.cache_hits,0);
    let different=Arc::new(a::Program{functions:program.functions.clone(),entry:call("fact",uint(2))});
    let program_admitted=admit_program(&different,&schema);let program_initial=admitted_initial(&program_admitted,4096,64);assert_ne!(initial.admission.view,program_initial.admission.view);
    let program_result=cached_actual(&program_initial,&mut cache);assert!(program_result.scheduling.scope_reset);assert_eq!(program_result.scheduling.cache_hits,0);
}
