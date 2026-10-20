//! Frozen source-derived selected-handler expectations BEFORE CLI body wiring.
//! ALL UNRUN. Initial20expectations were frozen in d3aed04c/ba03f7ee;
//! the21st post-compile expectation predates its metadata-retention code only.
//! Before execution, primitive expected wire labels were corrected from internal
//! IDs0/1 to existing OwnerWire's UInt/Bool (wire.rs::OwnerWire for TypeIdC).
//! This correction is after initial production wiring, not a preproduction claim.
//! Real production args, source I/O, handler and publication modules;
//! full normal mori-core is an external dependency, never cfg(test) Core/SDK.
#[path = "../src/cli_args/compile.rs"] pub(crate) mod compile_args;
mod cli_args { pub(crate) use crate::compile_args as compile; }
#[path = "../src/commands/source_io.rs"] mod source_io;
#[path = "../src/commands/publish_io.rs"] mod publish_io;
#[path = "../src/commands/compile_sherlock.rs"] mod selected_compile;
#[path = "../src/display.rs"] mod display;
use clap::{Args as ClapArgs, FromArgMatches};
use serde_json::{json, Value};
use std::{path::PathBuf, sync::atomic::{AtomicUsize, Ordering}};
const TYPED: &str = "moriarty-sherlock-nominal-body-typed/1";
const STRUCTURAL: &str = "moriarty-sherlock-nominal/1";
const JUDGMENTS: &[&str] = &["eq-any", "unique-parameters-every-declaration", "unique-binders-every-arm"];
static NEXT: AtomicUsize = AtomicUsize::new(0);
struct Temp(PathBuf);
impl Temp {
    fn new() -> Self {
        let path=std::env::temp_dir().join(format!("mori-body-typed-handler-{}-{}",std::process::id(),NEXT.fetch_add(1,Ordering::Relaxed)));
        std::fs::create_dir(&path).unwrap();Self(path)
    }
}
impl Drop for Temp { fn drop(&mut self) { let _=std::fs::remove_dir_all(&self.0); } }
fn config() -> Value { serde_json::from_str(include_str!("fixtures/nominal-compile-handler/config.json")).unwrap() }
fn text(body:&str)->String { format!("language \"moriarty-sherlock-nominal/1\"; program Check {{ {body} }}") }
struct Outcome { result:Result<(),String>, artifact:Option<Value>, statuses:Vec<(String,String)>, published:bool }
fn invoke(source:&str,cfg:&Value,profile:&str,entry:bool,observe:bool,readable_config:bool)->Outcome {
    let temp=Temp::new();let file=temp.0.join("program.mori");let conf=temp.0.join("config.json");let target=temp.0.join("target.json");let out=temp.0.join("out");
    std::fs::write(&file,source).unwrap();if readable_config {std::fs::write(&conf,cfg.to_string()).unwrap();}
    std::fs::write(&target,"{}").unwrap();
    let mut argv=vec!["compile".to_owned(),file.display().to_string(),"--target".into(),target.display().to_string(),"--out".into(),out.display().to_string(),"--execution-profile".into(),"sherlock-helper-microstep/1".into(),"--source-profile".into(),profile.into(),"--execution-config".into(),conf.display().to_string()];
    if entry {argv.push("--entry".into());}if observe {argv.push("--observe".into());}
    let parsed=compile_args::Args::augment_args(clap::Command::new("compile")).try_get_matches_from(argv).unwrap();
    let args=compile_args::Args::from_arg_matches(&parsed).unwrap();
    let source=source_io::read_source(&args.file).unwrap();let mut statuses=Vec::new();
    let result=selected_compile::run(&args,&source,|verb,message|statuses.push((verb.into(),message)))
        .map(|exit|assert_eq!(exit,std::process::ExitCode::SUCCESS)).map_err(|e|e.to_string());
    let published=out.exists();
    let artifact=if result.is_ok() {assert_eq!(std::fs::read_dir(&out).unwrap().count(),1);Some(serde_json::from_str(&std::fs::read_to_string(out.join("sherlock-artifact.json")).unwrap()).unwrap())} else {None};
    Outcome {result,artifact,statuses,published}
}
fn compiled(source:&str,cfg:&Value,profile:&str)->Value {
    let outcome=invoke(source,cfg,profile,true,false,true);outcome.result.unwrap();
    assert_eq!(outcome.statuses.len(),1);assert_eq!(outcome.statuses[0].0,"Prepared");
    assert!(outcome.published);outcome.artifact.unwrap()
}
fn expected_error(stage:&'static str,error:Value,span:Value,status:&str)->String {
    let completed=status=="completed";
    let failure=mori_core::sherlock_compile_artifact::CompileFailure {stage,actual_error:json!({
        "source_profile":TYPED,"error":error,"span":span,
        "body_typechecking":{"status":status,"judgments":if completed {Some(JUDGMENTS)} else {None}},
    }),evaluation_performed:false,events_forwarded:"0".into(),completed:false};
    format!("mori::command: {}",display::text(&mori_core::json::export(&failure).unwrap()))
}
fn refused(source:&str,cfg:&Value,stage:&'static str,error:Value,span:Value,status:&str) {
    let outcome=invoke(source,cfg,TYPED,true,false,true);
    assert_eq!(outcome.result.unwrap_err(),expected_error(stage,error,span,status));
    assert!(!outcome.published);assert!(outcome.artifact.is_none());assert!(outcome.statuses.is_empty());
}
fn site(source:&str,snippet:&str)->Value {
    let start=source.find(snippet).unwrap();json!({"start":start,"end":start+snippet.len()})
}
fn body_kind(function:Option<usize>,path:&[usize],kind:Value)->Value {
    json!({"source":{"function_index":function.map(|i|i.to_string()),"child_path":path.iter().map(usize::to_string).collect::<Vec<_>>()},"kind":kind})
}
fn expected_u_b()->Value {json!({"Expected":{"expected":"UInt","actual":"Bool"}})}
fn assert_typed(artifact:&Value,cfg:&Value) {
    assert_eq!(artifact["format"],"sherlock-nominal-body-typed-host-artifact/1");
    assert_eq!(artifact["source_profile"],TYPED);assert_eq!(artifact["execution_profile"],"sherlock-helper-microstep/1");
    assert_eq!(artifact["body_typechecking"],json!({"status":"completed","judgments":JUDGMENTS}));
    assert_eq!(artifact["source"]["language"],STRUCTURAL);assert_eq!(artifact["config"]["fields"],*cfg);
    assert_eq!(artifact["admission"]["bounds"],cfg["bounds"]);assert_eq!(artifact["admission"]["layout"]["bounds"],cfg["bounds"]);
    assert_eq!(artifact["admission"]["layout"]["layout_version"],cfg["layout"]["layout_version"]);
    assert_eq!(artifact["admission"]["identity"]["grammar_domain"],"GenericNominalExtensionReserved");
    assert_eq!(artifact["admission"]["identity"]["finance"],"Absent");
    assert_eq!(artifact["evaluation_performed"],false);assert_eq!(artifact["semantic_correspondence"],"unproved");
    assert_eq!(artifact["plans"]["export"]["status"],"unsupported");assert_eq!(artifact["evidence"]["native"],"unrun");
    assert!(artifact.get("entry_type").is_none());assert!(artifact.get("function_returns").is_none());
}
#[test]
fn selected_recursive_generic_source_publishes_body_completed_owner_tables() {
    let cfg=config();let a=compiled(include_str!("fixtures/nominal-compile-handler/wrapped-countdown.mori"),&cfg,TYPED);assert_typed(&a,&cfg);
    let rows=a["admission"]["program_view"]["expressions"].as_array().unwrap();assert!(rows[0].get("Call").is_some());assert!(rows.iter().any(|r|r.get("Construct").is_some()));
    assert_eq!(a["source"]["name"],"WrappedCountdown");
}
#[test]
fn selected_mutual_generic_source_retains_recursive_types_and_function_membership() {
    let cfg=config();let a=compiled(include_str!("fixtures/nominal-compile-handler/mutual-generic.mori"),&cfg,TYPED);assert_typed(&a,&cfg);
    assert_eq!(a["admission"]["program_view"]["functions"].as_array().unwrap().len(),2);
    assert!(a["admission"]["program_view"]["types"].as_array().unwrap().len()>4);assert_eq!(a["source"]["name"],"MutualAndGeneric");
}
#[test]
fn unused_return_mismatch_is_located_before_publication() {
    let source=text("fn unused(n: UInt): Bool { n } entry true;");
    let start=source.find("{ n }").unwrap()+2;
    refused(&source,&config(),"body-type",body_kind(Some(0),&[],json!({"Expected":{"expected":"Bool","actual":"UInt"}})),json!({"start":start,"end":start+1}),"failed");
}
#[test]
fn unused_duplicate_parameter_selects_second_binder() {
    let source=text("fn f(x: UInt, x: UInt): UInt { x } entry 0;");let start=source.find(", x:").unwrap()+2;
    refused(&source,&config(),"body-type",body_kind(Some(0),&[],json!({"DuplicateParameter":{"name":"x","index":"1"}})),json!({"start":start,"end":start+1}),"failed");
}
#[test]
fn unplanned_duplicate_pattern_binder_maps_arm_body() {
    let source=text("entry match cons(1,nil) { cons(x,x) => 7, fallback => 8 };");
    refused(&source,&config(),"body-type",body_kind(None,&[1],json!({"DuplicatePatternBinder":"x"})),site(&source,"7"),"failed");
}
#[test]
fn eq_any_is_body_complete_through_selected_handler() {
    let cfg=config();let a=compiled(&text("entry 1 == true;"),&cfg,TYPED);assert_typed(&a,&cfg);
    assert!(a["admission"]["program_view"]["expressions"].as_array().unwrap().iter().any(|r|r["Binary"]["op"]=="Eq"));
}
#[test]
fn eq_operand_failure_retains_original_path_and_parenthesis_span_convention() {
    let source=text("entry (true + 1) == 0;");refused(&source,&config(),"body-type",body_kind(None,&[0,0],expected_u_b()),site(&source,"true"),"failed");
}
#[test]
fn body_failure_precedes_zero_step_cap() {
    let source=text("entry true + 1;");let mut cfg=config();cfg["bounds"]["step_cap"]="0".into();
    refused(&source,&cfg,"body-type",body_kind(None,&[0],expected_u_b()),site(&source,"true"),"failed");
}
#[test]
fn welltyped_body_reaches_owner_zero_step_cap_with_completed_body_flag() {
    let mut cfg=config();cfg["bounds"]["step_cap"]="0".into();
    refused(&text("entry 0;"),&cfg,"owner-admission",json!({"CoreTooling":{"CapacityNotAdmitted":{"kind":"StepCap","needed":{"Exact":"1"},"limit":"0","locator":{"Field":{"name":"bounds.stepCap"}}}}}),Value::Null,"completed");
}
#[test]
fn builtin_constructor_first_t3_refusal_has_preparation_stage() {
    refused(&text("entry construct List::Cons(1,nil);"),&config(),"preparation",json!({"ExtAdmissionRejected":{"reason":"BuiltinConstruct","detail":"Cons","locator":null}}),Value::Null,"not-performed");
}
#[test]
fn bounded_parse_refusal_retains_resource_and_counts() {
    let source=text("entry 0;");let mut cfg=config();cfg["preprocessing"]["max_source_bytes"]="1".into();
    refused(&source,&cfg,"parse",json!({"Budget":{"resource":"SourceBytes","limit":"1","observed":source.len().to_string()}}),Value::Null,"not-performed");
}
#[test]
fn genesis_failure_retains_actual_value_slots_payload_and_body_completion() {
    let mut cfg=config();cfg["fuel"]["value"]="4097".into();
    refused(&text("entry 0;"),&cfg,"genesis",json!({"CapacityNotAdmitted":{"kind":"ValueSlots","needed":{"Exact":"4097"},"limit":"4096","locator":null}}),Value::Null,"completed");
}
#[test]
fn owner_v3_is_distinct_after_successful_duplicate_function_body_checks() {
    refused(&text("fn f(x: UInt): UInt { x } fn f(y: UInt): UInt { y } entry f(1);"),&config(),"owner-admission",json!({"CoreTooling":{"AdmissionRejected":{"reason":"ProgramViewMalformed","detail":"V3","locator":null}}}),Value::Null,"completed");
}
#[test]
fn supplied_type_row_limit_is_used_by_first_preparation() {
    let mut cfg=config();cfg["bounds"]["type_rows"]="0".into();
    refused(&text("type Box { Wrap; } entry construct Box::Wrap();"),&cfg,"preparation",json!({"ExtCapacityNotAdmitted":{"kind":"TypeRows","needed":{"Exact":"5"},"limit":"0","locator":null}}),Value::Null,"not-performed");
}
#[test]
fn unsupported_finance_wire_is_refused_as_configuration_without_domain_inference() {
    let mut cfg=config();cfg["finance"]="TypedExtensionReserved".into();
    let o=invoke(&text("entry 0;"),&cfg,TYPED,true,false,true);let error=o.result.unwrap_err();assert!(error.contains("\"stage\": \"config\""));assert!(!o.published);assert!(o.statuses.is_empty());
}
#[test]
fn structural_profile_keeps_not_performed_and_accepts_unused_body_mismatch() {
    let a=compiled(&text("fn unused(x: UInt): Bool { x } entry 0;"),&config(),STRUCTURAL);
    assert_eq!(a["format"],"sherlock-nominal-host-artifact/1");assert_eq!(a["body_typechecking"],"not-performed");assert!(a.get("source_profile").is_none());
}
#[test]
fn explicit_entry_selection_is_required_before_config_io() {
    let o=invoke(&text("entry 0;"),&config(),TYPED,false,false,false);let error=o.result.unwrap_err();
    assert!(error.contains("\"selection_error\": \"MissingEntry\""));assert!(error.contains(TYPED));assert!(!o.published);assert!(o.statuses.is_empty());
}
#[test]
fn observe_reads_config_for_body_typed_profile_before_source_work() {
    // Successor observation design explicitly retires the prior compile-only
    // rejection. Its historical freeze/results remain valid for their old cut.
    let o=invoke(&text("entry 0;"),&config(),TYPED,true,true,false);let error=o.result.unwrap_err();
    assert!(error.contains("\"stage\": \"config\""));assert!(!o.published);assert!(o.statuses.is_empty());
}
#[test]
fn unknown_body_profile_version_remains_a_selection_error() {
    use mori_core::sherlock_compile_artifact::draft_contract::{select_source,SourceSelection};
    assert!(matches!(select_source(Some(TYPED),None,true),Ok(SourceSelection::NominalEntry)));
    let o=invoke(&text("entry 0;"),&config(),"moriarty-sherlock-nominal-body-typed/2",true,false,false);
    assert!(o.result.unwrap_err().contains("\"selection_error\": \"UnknownProfile\""));assert!(!o.published);assert!(o.statuses.is_empty());
}
#[test]
fn recursive_argument_failure_checks_unused_body_without_runtime_unfolding() {
    let source=text("fn f(n: UInt): UInt { f(false) } entry 0;");refused(&source,&config(),"body-type",body_kind(Some(0),&[0],expected_u_b()),site(&source,"false"),"failed");
}

// Incremental expectation frozen after initial CLI wiring but BEFORE adding
// post-compile failure metadata retention. Not part of the original20 freeze.
#[test]
fn publication_failure_retains_completed_body_metadata_without_runtime_completion() {
    let temp=Temp::new();let file=temp.0.join("program.mori");let conf=temp.0.join("config.json");let target=temp.0.join("target.json");let out=temp.0.join("out");
    std::fs::write(&file,text("entry 0;")).unwrap();std::fs::write(&conf,config().to_string()).unwrap();std::fs::write(&target,"{}").unwrap();std::fs::create_dir(&out).unwrap();
    let args=compile_args::Args {file,target,out,execution_profile:Some("sherlock-helper-microstep/1".into()),source_profile:Some(TYPED.into()),entry:true,action:None,execution_config:Some(conf),observe:false};
    let source=source_io::read_source(&args.file).unwrap();let mut statuses=0;
    let error=selected_compile::run(&args,&source,|_,_|statuses+=1).unwrap_err().to_string();
    assert!(error.contains("\"stage\": \"publish\""));assert!(error.contains("\"status\": \"completed\""));
    assert!(error.contains("eq-any"));assert!(error.contains("\"completed\": false"));assert_eq!(statuses,0);
}
