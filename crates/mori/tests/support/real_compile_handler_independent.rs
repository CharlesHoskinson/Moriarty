//! Ordinary positive actual production Args/selected-handler/publication consumer.
//! SOURCE_ONLY/UNRUN. Expectations froze before extraction proposals were read.
//! This binary is not the complete backend-linked mori CLI.
#[path = "../../src/cli_args/compile.rs"]
pub(crate) mod compile_args;
mod cli_args { pub(crate) use crate::compile_args as compile; }
#[path = "../../src/commands/source_io.rs"]
mod source_io;
#[path = "../../src/commands/publish_io.rs"]
mod publish_io;
#[path = "../../src/commands/compile_sherlock.rs"]
mod selected_compile;
#[path = "../../src/display.rs"]
mod display;
#[path = "../../src/styles.rs"]
mod styles;
#[path = "../../src/status_output.rs"]
mod status_output;
use clap::{Args as ClapArgs, FromArgMatches};
use serde_json::{json, Value};
use std::{collections::BTreeSet, ffi::OsString, path::{Path,PathBuf}, process::ExitCode};
type Result<T> = std::result::Result<T,mori_diagnostics::Report>;
fn keys(v:&Value)->BTreeSet<&str>{v.as_object().expect("JSON object").keys().map(String::as_str).collect()}
fn expected_keys(actual:&Value,expected:&Value){assert_eq!(keys(actual),expected.as_array().unwrap().iter().map(|v|v.as_str().unwrap()).collect());}
fn pointers(actual:&Value,expected:&Value){for (p,v) in expected.as_object().unwrap(){assert_eq!(actual.pointer(p).unwrap_or_else(||panic!("missing {p}")),v,"{p}");}}
fn read(path:&Path)->Result<Value>{mori_core::json::parse(&source_io::read_source(path)?).map_err(source_io::operation)}
fn number(v:&Value)->u128{
    let s=v.as_str().expect("canonical decimal string");assert!(s=="0"||(!s.is_empty()&&!s.starts_with('0')&&s.bytes().all(|b|b.is_ascii_digit())));s.parse().unwrap()
}
fn idx(v:&Value)->usize{usize::try_from(number(v)).unwrap()}
// Wire config owns one bounds object at its top level. Public typed
// LayoutObservation/LayoutSchemaC also carry that same bounds inside layout.
// This adapter follows those declarations; it never copies subject output.
fn admission_layout(config:&Value)->Value{
    let mut layout=config["layout"].clone();
    layout.as_object_mut().unwrap().insert("bounds".into(),config["bounds"].clone());
    layout
}
fn core(actual:&Value,case:&str,observe:bool,expected:&Value){
    let e=&expected["core"];let c=&e["cases"][case];expected_keys(actual,&e["topLevelExactKeys"]);pointers(actual,&e["commonPointers"]);
    pointers(actual,if observe{&e["observePointers"]}else{&e["compileOnlyPointers"]});
    assert_eq!(actual["source"],c["source"]);assert_eq!(actual["config"],e["exactConfig"]);assert_eq!(actual["target"],e["exactTarget"]);
    assert_eq!(actual["admission"]["bounds"],e["exactConfig"]["bounds"]);assert_eq!(actual["admission"]["layout"],admission_layout(&e["exactConfig"]));
    assert_eq!(actual["admission"]["identity"]["layout_version"],e["exactConfig"]["layout"]["layout_version"]);
    let view=&actual["admission"]["program_view"];expected_keys(view,&e["programViewExactKeys"]);
    let expressions=view["expressions"].as_array().unwrap();let entry=&expressions[idx(&view["entry"])]["Call"];assert!(entry.is_object());assert_eq!(entry["args"]["len"],"0");
    let funcs=view["functions"].as_array().unwrap();assert_eq!(funcs.len(),1);assert_eq!(funcs[0]["params"]["len"],"0");
    assert!(expressions[idx(&funcs[0]["body"])].get("Let").is_some(),"source retained, not a computed result literal");
    assert!(view["parameters"].as_array().unwrap().is_empty());assert!(view["arms"].as_array().unwrap().is_empty());assert!(view["patterns"].as_array().unwrap().is_empty());
    assert!(expressions.iter().filter(|x|x.get("Let").is_some()).count()>=c["requiredLets"].as_u64().unwrap() as usize);
    for op in c["requiredBinaryOps"].as_array().unwrap(){assert!(expressions.iter().any(|x|x["Binary"]["op"]==*op));}
    let report=&actual["preparation"];assert_eq!(report["action"],c["action"]);assert_eq!(report["action_index"],0);assert_eq!(report["generated_function_name"],c["action"]);assert_eq!(report["inputs"],json!([]));
    assert_eq!(report["lets"].as_array().unwrap().iter().map(|x|x["name"].as_str().unwrap()).collect::<Vec<_>>(),c["letNames"].as_array().unwrap().iter().map(|x|x.as_str().unwrap()).collect::<Vec<_>>());
    assert_eq!(report["emit"]["name"],c["emit"]);assert_eq!(report["emit"]["role"],"value-projection; ordered emit effect remains in Core");
    if !observe {assert!(actual.get("genesis").is_none());return;}
    let observation=&actual["observation"];let image=&observation["image"];let summary=&observation["summary"];let resource=&observation["resource_report"];
    assert_eq!(keys(summary),BTreeSet::from(["outcome","events","consumed_work","helper_work","helper_visits","reference_cursor"]));
    let halted=image["outcome"]["Halted"].as_object().expect("actual normal Halted outcome");assert_eq!(halted.keys().map(String::as_str).collect::<BTreeSet<_>>(),BTreeSet::from(["value"]));
    let reference=&halted["value"];let r=reference.as_object().unwrap();assert_eq!(r.len(),1);let (kind,id)=r.iter().next().unwrap();
    let row=match kind.as_str(){"Const"=>&view["constants"][idx(id)],"Data"=>&image["data"][idx(id)],_=>panic!("actual scalar reference")};
    let tag=c["result"]["kind"].as_str().unwrap();let member=c["result"]["field"].as_str().unwrap();assert_eq!(row[tag][member],c["result"]["value"]);
    assert_eq!(image["control"],json!({"Return":{"value":reference}}));assert_eq!(summary["outcome"],image["outcome"]);
    let forwarded=number(&observation["events_forwarded"]);assert!(forwarded>0);assert_eq!(forwarded,number(&summary["events"]));assert_eq!(observation["events_forwarded"],resource["run_summary"]["events"]);
    for counter in ["consumed_work","helper_work","helper_visits","reference_cursor"]{
        number(&summary[counter]);assert_eq!(summary[counter],image[counter]);assert_eq!(summary[counter],resource["observed"][counter]);assert_eq!(summary[counter],resource["run_summary"][counter]);
    }
    let fuel=number(&e["exactConfig"]["fuel"]["value"]);assert_eq!(number(&image["initial_fuel"]),fuel);assert!(number(&image["consumed_work"])>0);assert_eq!(number(&image["remaining_work"]).checked_add(number(&image["consumed_work"])),Some(fuel));
    for group in resource["unmeasured"].as_object().unwrap().values(){assert_eq!(group["status"],"unmeasured");}
    assert!(observation.get("events").is_none()); // count is not a retained trace
}
fn draft(actual:&Value,case:&str,expected:&Value,independent:&Value){
    let e=&expected["draft"];expected_keys(actual,&e["topLevelExactKeys"]);pointers(actual,&e["commonPointers"]);
    assert_eq!(actual["config"],e["exactConfig"]);assert_eq!(actual["target"],e["exactTarget"]);
    assert_eq!(actual["source"],independent[case]["source"]);assert_eq!(actual["preparation"],independent[case]["preparation"]);assert_eq!(actual["admission"]["program_view"],independent[case]["program_view"]);
    assert_eq!(actual["admission"]["bounds"],e["exactConfig"]["bounds"]);assert_eq!(actual["admission"]["layout"],admission_layout(&e["exactConfig"]));
    assert_eq!(actual["genesis"]["initial_fuel"],e["exactConfig"]["fuel"]["value"]);assert_eq!(actual["genesis"]["remaining_work"],e["exactConfig"]["fuel"]["value"]);
    assert_eq!(actual["supported_profile"]["format"],"sherlock-supported-profile/1");assert_eq!(actual["supported_profile"]["execution_profile"],actual["execution_profile"]);
    // Mathematical7/8 are not reported as observed results by compile-only.
}
fn verify_published(out:&Path,fixtures:&Path,case:&str,observe:bool,expected:&Value,is_draft:bool)->Result<()> {
    assert!(out.is_dir());
    let names=std::fs::read_dir(out).unwrap().map(|entry|entry.unwrap().file_name().into_string().unwrap()).collect::<BTreeSet<_>>();
    assert_eq!(names,BTreeSet::from(["sherlock-artifact.json".to_owned()]));
    let actual=read(&out.join("sherlock-artifact.json"))?;
    if is_draft{draft(&actual,case,expected,&read(&fixtures.join("DRAFT-INDEPENDENT-PREPARATION.json"))?);}else{core(&actual,case,observe,expected);}
    Ok(())
}
fn run()->Result<ExitCode>{
    // Consumer invocation: CASE compile|observe OUTDIR FIXTURES_ROOT.
    // Root chooses only normal fixed combinations; this is not another public CLI.
    let mut input:Vec<OsString>=std::env::args_os().skip(1).collect();
    let verify_only=input.first().is_some_and(|v|v=="--verify-published");
    if verify_only{input.remove(0);}
    assert_eq!(input.len(),4);
    let case=input[0].to_str().unwrap();let observe=match input[1].to_str().unwrap(){"compile"=>false,"observe"=>true,_=>panic!("normal selected mode")};
    let out=PathBuf::from(&input[2]);let fixtures=PathBuf::from(&input[3]);let expected=read(&fixtures.join("EXPECTED-OUTPUT.json"))?;
    let is_draft=case=="chain_total"||case=="branch_total";assert!(!is_draft||!observe,"no draft observe positive");
    let group=if is_draft{"draft"}else{"core"};let record=&expected[group]["cases"][case];assert!(record.is_object());
    // Harness-only readback mode returns BEFORE source reading, production Args
    // parsing or the handler call. It cannot rerun the passing handler prefix.
    if verify_only{
        verify_published(&out,&fixtures,case,observe,&expected,is_draft)?;
        println!("PASS persisted artifact verification ONLY: {case}; handler not invoked");
        return Ok(ExitCode::SUCCESS);
    }
    let file=fixtures.join(record["file"].as_str().unwrap());let target=fixtures.join("target.json");let config=fixtures.join(if is_draft{"draft-config.json"}else{"core-config.json"});
    let mut argv=vec![OsString::from("compile"),file.clone().into_os_string(),"--target".into(),target.into_os_string(),"--out".into(),out.clone().into_os_string(),"--execution-profile".into(),"sherlock-helper-microstep/1".into(),"--execution-config".into(),config.into_os_string()];
    if is_draft{argv.extend(["--source-profile".into(),"moriarty-sherlock-draft/1".into(),"--entry".into()]);}else{argv.extend(["--action".into(),record["action"].as_str().unwrap().into()]);}
    if observe{argv.push("--observe".into());}
    let matches=compile_args::Args::augment_args(clap::Command::new("compile")).try_get_matches_from(argv).expect("actual production positive Args");
    let args=compile_args::Args::from_arg_matches(&matches).expect("actual production Args decoding");
    let source=source_io::read_source(&args.file)?; // same genuine outer reader
    let mut err=anstream::AutoStream::new(std::io::stderr(),anstream::ColorChoice::Never);
    assert!(!out.exists(),"fresh ordinary publication destination");
    let code=selected_compile::run(&args,&source,|verb,message|status_output::status(&mut err,status_output::Verbosity::Normal,verb,message))?;
    assert_eq!(code,ExitCode::SUCCESS);
    verify_published(&out,&fixtures,case,observe,&expected,is_draft)?;
    println!("PASS actual production compile handler: {case} {} wrote {}",if observe{"observe"}else{"compile-only"},out.join("sherlock-artifact.json").display());
    Ok(code)
}
fn main()->ExitCode{match run(){Ok(code)=>code,Err(report)=>{eprintln!("{report}");ExitCode::FAILURE}}}
