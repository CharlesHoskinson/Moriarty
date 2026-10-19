//! Source-only ordinary production-handler tests; no backend-linked CLI claim.
#[path = "../src/cli_args/compile.rs"]
pub(crate) mod compile_args;
mod cli_args { pub(crate) use crate::compile_args as compile; }
#[path = "../src/commands/source_io.rs"] mod source_io;
#[path = "../src/commands/publish_io.rs"] mod publish_io;
#[path = "../src/commands/compile_sherlock.rs"] mod selected_compile;
#[path = "../src/display.rs"] mod display;
#[path = "../src/styles.rs"] mod styles;
#[path = "../src/status_output.rs"] mod status_output;
use clap::{Args as ClapArgs, FromArgMatches};
use serde_json::Value;
use std::{collections::BTreeSet, path::{Path,PathBuf}, sync::atomic::{AtomicUsize,Ordering}};
static NEXT: AtomicUsize = AtomicUsize::new(0);
struct Temp(PathBuf);
impl Temp {
    fn new() -> Self {
        let path=std::env::temp_dir().join(format!("mori-nominal-command-{}-{}",std::process::id(),NEXT.fetch_add(1,Ordering::Relaxed)));
        std::fs::create_dir(&path).unwrap(); Self(path)
    }
}
impl Drop for Temp { fn drop(&mut self) { let _=std::fs::remove_dir_all(&self.0); } }
fn fixtures() -> PathBuf {
    // Cargo supplies the package root. A direct rustc invocation uses an
    // absolute source path and no CARGO_MANIFEST_DIR (as in the reviewed recipe).
    match option_env!("CARGO_MANIFEST_DIR") {
        Some(root) => Path::new(root).join("tests/fixtures/nominal-compile-handler"),
        None => {
            assert!(Path::new(file!()).is_absolute(), "direct rustc requires an absolute test source path");
            Path::new(file!()).parent().unwrap().join("fixtures/nominal-compile-handler")
        }
    }
}
fn read(path:&Path)->Value { serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap() }
fn arguments(case:&str,out:&Path,config:&Path,entry:bool,observe:bool)->compile_args::Args {
    let f=fixtures();let mut argv=vec!["compile".to_owned(),f.join(format!("{case}.mori")).display().to_string(),"--target".into(),f.join("target.json").display().to_string(),"--out".into(),out.display().to_string(),"--execution-profile".into(),"sherlock-helper-microstep/1".into(),"--source-profile".into(),"moriarty-sherlock-nominal/1".into(),"--execution-config".into(),config.display().to_string()];
    if entry {argv.push("--entry".into());} if observe {argv.push("--observe".into());}
    let matches=compile_args::Args::augment_args(clap::Command::new("compile")).try_get_matches_from(argv).unwrap();
    compile_args::Args::from_arg_matches(&matches).unwrap()
}
fn positive(case:&str) {
    let temp=Temp::new();let out=temp.0.join("out");let f=fixtures();
    let args=arguments(case,&out,&f.join("config.json"),true,false);
    let source=source_io::read_source(&args.file).unwrap();
    let mut statuses=Vec::new();
    assert_eq!(selected_compile::run(&args,&source,|verb,message|statuses.push((verb.to_owned(),message))).unwrap(),std::process::ExitCode::SUCCESS);
    let actual=read(&out.join("sherlock-artifact.json"));let expected=read(&f.join("AUTHOR-EXPECTATIONS.json"));let e=&expected["cases"][case];
    assert_eq!(std::fs::read_dir(&out).unwrap().count(),1);
    assert_eq!(statuses.len(),1);assert_eq!(statuses[0].0,"Prepared");
    for (key,value) in expected["common"].as_object().unwrap() {assert_eq!(&actual[key],value,"{key}");}
    assert_eq!(actual["source"]["language"],"moriarty-sherlock-nominal/1");assert_eq!(actual["source"]["source_digest"],e["source_digest"]);
    assert_eq!(actual["config"]["fields"],read(&f.join("config.json")));
    assert_eq!(actual["target"]["descriptor"],read(&f.join("target.json")));
    let view=&actual["admission"]["program_view"];
    let keys:BTreeSet<_>=view.as_object().unwrap().keys().map(String::as_str).collect();
    assert_eq!(keys,expected["programFields"].as_array().unwrap().iter().map(|x|x.as_str().unwrap()).collect());
    assert_eq!(view["names"],e["names"]);
    for (table,count) in [("expressions","expression_count"),("types","type_count"),("constructors","constructor_count"),("ctor_fields","field_count"),("constants","constant_count")] {assert_eq!(view[table].as_array().unwrap().len() as u64,e[count].as_u64().unwrap());}
    let names=view["names"].as_array().unwrap();let function_names:Vec<_>=view["functions"].as_array().unwrap().iter().map(|row|names[row["function_name"].as_str().unwrap().parse::<usize>().unwrap()].clone()).collect();
    assert_eq!(serde_json::json!(function_names),e["functions"]);
    assert!(view["expressions"][0].get("Call").is_some(),"actual entry remains a call");
    assert!(view["expressions"].as_array().unwrap().iter().any(|x|x.get("Construct").is_some()),"generic constructor retained");
    assert_eq!(actual["plans"]["export"]["status"],"unsupported");assert_eq!(actual["evidence"]["native"],"unrun");
}
#[test] fn nominal_recursive_source_publishes_actual_tables() {positive("wrapped-countdown");}
#[test] fn nominal_mutual_generic_source_publishes_actual_tables() {positive("mutual-generic");}
#[test] fn nominal_explicit_entry_and_compile_only_selection_are_retained() {
    let temp=Temp::new();let f=fixtures();
    for (entry,observe) in [(false,false),(true,true)] {
        // Missing config proves selection/observe refusal precedes config I/O.
        let out=temp.0.join(format!("{entry}-{observe}"));let args=arguments("wrapped-countdown",&out,&temp.0.join("missing-config.json"),entry,observe);
        let source=source_io::read_source(&args.file).unwrap();
        let error=selected_compile::run(&args,&source,|_,_|panic!("refusal must not report Prepared")).unwrap_err();
        let actual_error=if entry {serde_json::json!({"message":"nominal preparation is compile-only; --observe is unsupported"})}
            else {serde_json::json!({"selection_error":"MissingEntry","source_profile":"moriarty-sherlock-nominal/1","entry":false,"action":null})};
        let expected=mori_core::sherlock_compile_artifact::CompileFailure {stage:"select",actual_error,
            evaluation_performed:false,events_forwarded:"0".into(),completed:false};
        let diagnostic=mori_core::json::export(&expected).unwrap();
        assert_eq!(error.to_string(),format!("mori::command: {}",display::text(&diagnostic)));
        assert!(!out.exists());
    }
}
#[test] fn nominal_existing_parse_and_genesis_payloads_remain_structured() {
    let f=fixtures();let source=std::fs::read_to_string(f.join("wrapped-countdown.mori")).unwrap();let config=read(&f.join("config.json"));let target=std::fs::read_to_string(f.join("target.json")).unwrap();
    for (stage,key,value) in [("parse","max_source_bytes","1"),("admit-run","fuel","4097")] {
        let mut input=config.clone();if key=="fuel" {input["fuel"]["value"]=value.into();} else {input["preprocessing"][key]=value.into();}
        let result=mori_core::sherlock_compile_artifact::nominal_command::compile(source.as_str(),&input.to_string(),&target);
        let failure=match result {Err(e)=>e,Ok(_)=>panic!("existing finite refusal must survive")};
        assert_eq!(failure.stage,stage);assert!(!failure.evaluation_performed);assert_eq!(failure.events_forwarded,"0");assert!(!failure.completed);
        if stage=="parse" {assert_eq!(failure.actual_error["Budget"]["resource"],"SourceBytes");assert_eq!(failure.actual_error["Budget"]["limit"],"1");}
        else {assert_eq!(failure.actual_error["CapacityNotAdmitted"]["kind"],"ValueSlots");assert_eq!(failure.actual_error["CapacityNotAdmitted"]["needed"]["Exact"],"4097");assert_eq!(failure.actual_error["CapacityNotAdmitted"]["limit"],"4096");}
    }
}

#[test]
fn nominal_selection_is_distinct_and_old_profiles_keep_their_contract() {
    use mori_core::sherlock_compile_artifact::draft_contract::{select_source,SourceSelection,SelectionError,SOURCE_PROFILE};
    assert!(matches!(select_source(None,Some("pay"),false),Ok(SourceSelection::CoreAction("pay"))));
    assert!(matches!(select_source(Some("moriarty/1"),Some("pay"),false),Ok(SourceSelection::CoreAction("pay"))));
    assert!(matches!(select_source(Some(SOURCE_PROFILE),None,true),Ok(SourceSelection::DraftEntry)));
    assert!(matches!(select_source(Some("moriarty-sherlock-nominal/1"),None,true),Ok(SourceSelection::NominalEntry)));
    assert!(matches!(select_source(Some("moriarty-sherlock-nominal/1"),Some("pay"),true),Err(SelectionError::ConflictingSelection)));
    assert!(matches!(select_source(Some("moriarty-sherlock-nominal/2"),None,true),Err(SelectionError::UnknownProfile)));
}
