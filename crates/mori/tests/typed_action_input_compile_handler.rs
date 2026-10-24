//! Initial13 cases preceded production; two controls and review fixes came later.
//! C3 revises one actual failed HostOnly identity assertion. Revised case UNRUN.
//! Prior original04-06 passes remain historical; no whole-suite/red-green claim.
//! Author-derived structural/value assertions; not an independent oracle.
//! Actual production Args/handler/read/publication and normal external Core.
#[path = "../src/cli_args/compile.rs"] pub(crate) mod compile_args;
mod cli_args { pub(crate) use crate::compile_args as compile; }
#[path = "../src/commands/source_io.rs"] mod source_io;
#[path = "../src/commands/publish_io.rs"] mod publish_io;
#[path = "../src/commands/compile_sherlock.rs"] mod selected_compile;
#[path = "../src/display.rs"] mod display;
use clap::{Args as ClapArgs, FromArgMatches};
use mori_core::sherlock_compile_artifact as host;
use serde_json::{json, Value};
use std::{collections::BTreeMap, path::PathBuf, sync::atomic::{AtomicUsize, Ordering}};
const SOURCE: &str = include_str!("fixtures/typed-action-inputs/input-bound-quotes.mori");
const QUOTE: &str = include_str!("fixtures/typed-action-inputs/quote.inputs.json");
const INSPECT: &str = include_str!("fixtures/typed-action-inputs/inspect.inputs.json");
const CONFIG: &str = include_str!("fixtures/typed-action-inputs/core-config.json");
const TARGET: &str = include_str!("fixtures/typed-action-inputs/target.json");
const EMPTY_ACTION: &str = "language \"moriarty/1\"; agreement Empty { action check() { emit \"value\"(1 + 2); } }";
const SCOPE: &str = "--execution-inputs is supported only for Core/1 action compilation";
static NEXT: AtomicUsize = AtomicUsize::new(0);
struct Temp(PathBuf);
impl Temp { fn new() -> Self {
    let p = std::env::temp_dir().join(format!("mori-typed-input-{}-{}", std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed)));
    std::fs::create_dir(&p).unwrap(); Self(p)
} }
impl Drop for Temp { fn drop(&mut self) { let _ = std::fs::remove_dir_all(&self.0); } }
fn parsed(argv: Vec<String>) -> Result<compile_args::Args, clap::Error> {
    let m = compile_args::Args::augment_args(clap::Command::new("compile")).try_get_matches_from(argv)?;
    compile_args::Args::from_arg_matches(&m)
}
struct Invocation { temp: Temp, args: compile_args::Args }
impl Invocation {
    fn new(source: &str, action: &str, inputs: Option<&str>, observe: bool) -> Self {
        let temp = Temp::new();
        for (name, text) in [("source.mori", source), ("config.json", CONFIG), ("target.json", TARGET)] {
            std::fs::write(temp.0.join(name), text).unwrap();
        }
        let mut argv = vec!["compile".into(), temp.0.join("source.mori").display().to_string(),
            "--target".into(), temp.0.join("target.json").display().to_string(), "--out".into(), temp.0.join("out").display().to_string(),
            "--execution-profile".into(), host::EXECUTION_PROFILE.into(), "--action".into(), action.into(),
            "--execution-config".into(), temp.0.join("config.json").display().to_string()];
        if let Some(text) = inputs {
            std::fs::write(temp.0.join("inputs.json"), text).unwrap();
            argv.extend(["--execution-inputs".into(), temp.0.join("inputs.json").display().to_string()]);
        }
        if observe { argv.push("--observe".into()); }
        let args = parsed(argv).unwrap(); Self { temp, args }
    }
    fn run(&self) -> Result<String, String> {
        let source = source_io::read_source(&self.args.file).unwrap();
        let mut status = Vec::new();
        let result = selected_compile::run(&self.args, &source, |verb, message| status.push((verb.to_owned(), message)));
        match result {
            Ok(code) => {
                assert_eq!(code, std::process::ExitCode::SUCCESS); assert_eq!(status.len(), 1); assert_eq!(status[0].0, "Prepared");
                assert_eq!(std::fs::read_dir(&self.args.out).unwrap().count(), 1);
                Ok(std::fs::read_to_string(self.args.out.join("sherlock-artifact.json")).unwrap())
            }
            Err(e) => { assert!(status.is_empty()); Err(e.to_string()) }
        }
    }
}
fn core(source: &str, action: &str, inputs: &str, observe: bool) -> Value {
    let a = host::compile_host_with_inputs(source, action, CONFIG, TARGET, inputs, observe)
        .unwrap_or_else(|e| panic!("{}", serde_json::to_string(&e).unwrap()));
    serde_json::to_value(a).unwrap()
}
fn failure(source: &str, action: &str, config: &str, target: &str, inputs: &str) -> Value {
    serde_json::to_value(host::compile_host_with_inputs(source, action, config, target, inputs, false).err().expect("refusal")).unwrap()
}
fn refusal(v: &Value, stage: &str, error: Value) {
    assert_eq!(*v, json!({"stage":stage,"actual_error":error,"evaluation_performed":false,"events_forwarded":"0","completed":false}));
}
// Compare the real Report Display form, including prefix and display escaping.
// It is not raw JSON: source_io::operation also escapes pretty JSON newlines.
fn handler_failure_text(failure: &host::CompileFailure) -> String {
    format!("mori::command: {}",display::text(&mori_core::json::export(failure).unwrap()))
}
fn handler_refusal(actual: &str, stage: &'static str, error: Value) {
    let expected=host::CompileFailure { stage, actual_error:error,
        evaluation_performed:false,events_forwarded:"0".into(),completed:false };
    assert_eq!(actual,handler_failure_text(&expected));
}
fn id(v: &Value) -> usize { v.as_str().unwrap().parse().unwrap() }
fn scalar_row(a: &Value, r: &Value) -> Value {
    if let Some(i) = r.get("Const") { a["admission"]["program_view"]["constants"][id(i)].clone() }
    else { a["observation"]["image"]["data"][id(&r["Data"])].clone() }
}
fn halted_value(a: &Value) -> Value {
    let outcome = &a["observation"]["summary"]["outcome"];
    assert!(outcome["Halted"].is_object(), "{outcome}");
    assert_eq!(*outcome, a["observation"]["image"]["outcome"]);
    let row = scalar_row(a, &outcome["Halted"]["value"]);
    if let Some(n) = row.get("U128") { json!({"type":"UInt","value":n["n"]}) }
    else { json!({"type":"Bool","value":row["Bool"]["b"]}) }
}
fn structure(a: &Value, declared: &[&str], expected_args: &[Value], ops: &[&str]) {
    let v = &a["admission"]["program_view"]; assert_eq!(v.as_object().unwrap().len(), 23);
    let expr = v["expressions"].as_array().unwrap();
    let entry = &expr[id(&v["entry"])]["Call"]; assert!(entry.is_object());
    let start = id(&entry["args"]["start"]); let len = id(&entry["args"]["len"]); assert_eq!(len, declared.len());
    for (i, expected) in expected_args.iter().enumerate() {
        let child = &expr[id(&v["children"][start+i])];
        // Existing scalar entry literals are owner literal references, not computed final results.
        let constant = &v["constants"][id(&child["Literal"]["constant"])];
        if expected.get("U128").is_some() { assert_eq!(constant["U128"]["n"], expected["U128"]["n"]); }
        else { assert_eq!(constant["Bool"]["b"], expected["Bool"]["b"]); }
    }
    let f = &v["functions"][0]; assert_eq!(id(&f["params"]["len"]), declared.len());
    assert!(expr[id(&f["body"])].get("Let").is_some());
    assert!(expr.iter().any(|e| e.get("Var").is_some()));
    for op in ops { assert!(expr.iter().any(|e| e["Binary"]["op"] == *op), "{op}"); }
    assert_eq!(a["preparation"]["inputs"].as_array().unwrap().iter().map(|x|x["name"].as_str().unwrap()).collect::<Vec<_>>(), declared);
    assert_eq!(a["input_policy"]["declared"], json!(declared));
    assert_eq!(a["format"], "sherlock-host-artifact/1"); assert_eq!(a["evidence"]["native"], "unrun");
    assert_eq!(a["plans"]["export"]["status"], "unsupported");
}
#[test]
fn quote_preserves_parameters_vars_operators_and_declaration_order() {
    let invoke = Invocation::new(SOURCE, "quote", Some(QUOTE), false);
    let text = invoke.run().unwrap(); let a: Value = serde_json::from_str(&text).unwrap();
    structure(&a, &["units","flat"], &[json!({"U128":{"n":"9"}}),json!({"U128":{"n":"4"}})], &["Mul","Sub"]);
    assert_eq!(a["input_policy"]["kind"], "explicit-typed-map"); assert_eq!(a["input_policy"]["supplied"], json!(["flat","units"]));
    assert_eq!(a["input_policy"]["binding"]["values"], serde_json::from_str::<Value>(QUOTE).unwrap());
    assert_eq!(a["input_policy"]["binding"]["file_digest"], mori_core::intent::hash_bytes(QUOTE.as_bytes()));
    assert_eq!(a["observation"], json!({"status":"not-requested","evaluation_performed":false}));
    for (name, input) in [("source.mori",SOURCE),("inputs.json",QUOTE),("config.json",CONFIG),("target.json",TARGET)] {
        assert_eq!(std::fs::read(invoke.temp.0.join(name)).unwrap(), input.as_bytes());
    }
}
#[test]
fn two_real_selected_observations_return_contrasting_complete_values() {
    // Author arithmetic only. Independent source-denotation comparison is separate.
    for (action, inputs, expected) in [("quote",QUOTE,json!({"type":"UInt","value":"95"})),
        ("inspect",INSPECT,json!({"type":"Bool","value":false}))] {
        let text = Invocation::new(SOURCE, action, Some(inputs), true).run().unwrap();
        let a: Value = serde_json::from_str(&text).unwrap(); assert_eq!(halted_value(&a), expected);
        assert_eq!(a["observation"]["status"], "observed");
        assert_eq!(a["observation"]["events_forwarded"], a["observation"]["summary"]["events"]);
        if action == "inspect" {
            structure(&a,&["right","approved","left"],&[json!({"U128":{"n":"5"}}),json!({"Bool":{"b":true}}),json!({"U128":{"n":"8"}})],&["Lt"]);
            assert!(a["admission"]["program_view"]["expressions"].as_array().unwrap().iter().any(|e|e.get("If").is_some()), "existing eager And lowering retained");
            assert_eq!(a["input_policy"]["supplied"],json!(["approved","left","right"]));
        }
    }
}
#[test]
fn binding_serialization_preserves_legacy_keys_and_explicit_field_order() {
    let p = host::InputPolicy { kind:"parameter-free", declared:vec![], supplied:vec![], binding:None };
    assert_eq!(serde_json::to_string(&p).unwrap(), r#"{"kind":"parameter-free","declared":[],"supplied":[]}"#);
    let d = host::InputPolicy { kind:"closed-entry", declared:vec![], supplied:vec![], binding:None };
    assert_eq!(serde_json::to_string(&d).unwrap(), r#"{"kind":"closed-entry","declared":[],"supplied":[]}"#);
    let b = host::InputBinding { values:BTreeMap::new(), file_digest:"raw".into() };
    assert_eq!(serde_json::to_string(&b).unwrap(), r#"{"values":{},"file_digest":"raw"}"#);
    let old = host::compile_host(EMPTY_ACTION,"check",CONFIG,TARGET,false).unwrap_or_else(|_|panic!("legacy"));
    assert!(serde_json::to_value(old).unwrap()["input_policy"].get("binding").is_none());
    let explicit = core(EMPTY_ACTION,"check","{}",false);
    assert_eq!(explicit["input_policy"]["kind"],"explicit-typed-map"); assert_eq!(explicit["input_policy"]["binding"]["values"],json!({}));
    let e = serde_json::to_value(host::compile_host(SOURCE,"quote",CONFIG,TARGET,false).err().unwrap()).unwrap();
    assert_eq!(e["stage"],"select"); assert_eq!(e["actual_error"]["unsupported"],"parameterized-action");
}
#[test]
fn input_digest_binds_original_bytes_and_values_change_program_tables() {
    let compact: Value = serde_json::from_str(QUOTE).unwrap();
    let a = core(SOURCE,"quote",QUOTE,false); let b = core(SOURCE,"quote",&compact.to_string(),false);
    assert_eq!(a["input_policy"]["binding"]["values"],b["input_policy"]["binding"]["values"]);
    assert_ne!(a["input_policy"]["binding"]["file_digest"],b["input_policy"]["binding"]["file_digest"]);
    assert_eq!(a["admission"],b["admission"]);
    let mut different = compact; different["units"]["value"]="10".into();
    let c = core(SOURCE,"quote",&different.to_string(),false);
    // HostOnly reports deliberately leave digests Unselected; the concrete
    // input binding is in the admitted entry constants/program tables.
    assert_eq!(a["admission"]["identity"],c["admission"]["identity"]);
    assert_eq!(c["admission"]["identity"]["program_membership_digest"],"Unselected");
    assert_eq!(c["input_policy"]["binding"]["values"],different);
    assert_ne!(a["admission"]["program_view"],c["admission"]["program_view"]);
    structure(&a,&["units","flat"],&[json!({"U128":{"n":"9"}}),json!({"U128":{"n":"4"}})],&["Mul","Sub"]);
    structure(&c,&["units","flat"],&[json!({"U128":{"n":"10"}}),json!({"U128":{"n":"4"}})],&["Mul","Sub"]);
}
#[test]
fn strict_input_errors_keep_actual_existing_codec_payloads() {
    for input in [r#"{"units":{"type":"UInt","value":"9"},"\u0075nits":{"type":"UInt","value":"8"}}"#, "{} true"] {
        let error = mori_core::json::parse(input).err().unwrap();
        refusal(&failure("invalid source","quote",CONFIG,TARGET,input),"inputs",json!(error));
    }
    for input in ["null","[]","true",r#"{"units":{"type":"UInt","value":9}}"#,
        r#"{"units":{"type":"UInt","value":"09"}}"#,r#"{"units":{"type":"UInt","value":"-1"}}"#,
        r#"{"units":{"type":"UInt","value":"340282366920938463463374607431768211456"}}"#,
        r#"{"approved":{"type":"Bool","value":"true"}}"#,r#"{"units":{"type":"UInt","value":"9","extra":0}}"#] {
        let value = mori_core::json::parse(input).unwrap();
        let error = serde_json::from_value::<BTreeMap<String,mori_core::Value>>(value).err().unwrap();
        refusal(&failure("invalid source","quote",CONFIG,TARGET,input),"inputs",json!({"message":error.to_string()}));
    }
}
#[test]
fn uint_boundaries_and_bool_false_use_existing_value_wire() {
    let source = "language \"moriarty/1\"; agreement Echo { action u(value: UInt) { emit \"u\"(value); } action b(value: Bool) { emit \"b\"(value); } }";
    for (action,value) in [("u",json!({"type":"UInt","value":"0"})),("u",json!({"type":"UInt","value":u128::MAX.to_string()})),("b",json!({"type":"Bool","value":false}))] {
        let input=json!({"value":value}).to_string(); let a=core(source,action,&input,true);
        assert_eq!(halted_value(&a),value);
    }
}
#[test]
fn original_guard_precedence_and_payload_are_forwarded() {
    let cases=[("{}",json!({"MissingInput":{"name":"units"}})),
        (r#"{"flat":{"type":"Bool","value":false}}"#,json!({"MissingInput":{"name":"units"}})),
        (r#"{"flat":{"type":"Bool","value":false},"units":{"type":"Bool","value":true},"z":{"type":"UInt","value":"0"}}"#,json!({"TypeMismatch":{"name":"units","declared":"UInt","supplied":"Bool"}})),
        (r#"{"a":{"type":"UInt","value":"0"},"flat":{"type":"UInt","value":"4"},"units":{"type":"UInt","value":"9"},"z":{"type":"UInt","value":"0"}}"#,json!({"ExtraInput":{"name":"a"}}))];
    for (inputs,error) in cases { refusal(&failure(SOURCE,"quote",CONFIG,TARGET,inputs),"host-guard",error); }
    for (field,bound,measure,required) in [("max_supplied_inputs","1","SuppliedInputs","2"),("max_input_name_bytes","4","InputNameBytes","5")] {
        let mut cfg:Value=serde_json::from_str(CONFIG).unwrap(); cfg["host"][field]=bound.into();
        refusal(&failure(SOURCE,"quote",&cfg.to_string(),TARGET,QUOTE),"host-guard",json!({"Exceeded":{"measure":measure,"required":required,"bound":bound}}));
    }
}
#[test]
fn core_decode_order_preserves_config_target_guard_then_inputs_then_source() {
    assert_eq!(failure("bad","quote","{",TARGET,"{")["stage"],"config");
    assert_eq!(failure("bad","quote",CONFIG,"{","{")["stage"],"config");
    let mut cfg:Value=serde_json::from_str(CONFIG).unwrap(); cfg["host"]["max_supplied_inputs"]="0".into();
    assert_eq!(failure("bad","quote",&cfg.to_string(),TARGET,"{")["stage"],"host-guard");
    assert_eq!(failure("bad","quote",CONFIG,TARGET,"{")["stage"],"inputs");
    assert_eq!(failure("bad","quote",CONFIG,TARGET,"{}")["stage"],"parse");
    assert_eq!(failure(SOURCE,"missing",CONFIG,TARGET,"{}")["stage"],"select");
}
#[test]
fn scope_refusal_follows_selection_and_precedes_observe_config_and_io() {
    for profile in ["moriarty-sherlock-draft/1","moriarty-sherlock-nominal/1","moriarty-sherlock-nominal-body-typed/1"] {
        let mut i=Invocation::new(SOURCE,"quote",Some(QUOTE),true);
        i.args.source_profile=Some(profile.into()); i.args.action=None; i.args.entry=true; i.args.execution_config=None;
        std::fs::remove_file(i.args.execution_inputs.as_ref().unwrap()).unwrap();
        handler_refusal(&i.run().unwrap_err(),"select",json!({"message":SCOPE})); assert!(!i.args.out.exists());
        i.args.entry=false; handler_refusal(&i.run().unwrap_err(),"select",json!({"selection_error":"MissingEntry","source_profile":profile,"entry":false,"action":null}));
        i.args.source_profile=Some("unknown".into()); handler_refusal(&i.run().unwrap_err(),"select",json!({"selection_error":"UnknownProfile","source_profile":"unknown","entry":false,"action":null}));
    }
    let mut i=Invocation::new(SOURCE,"quote",Some(QUOTE),false); i.args.execution_profile=None;
    handler_refusal(&i.run().unwrap_err(),"select",json!({"message":"missing --execution-profile"}));
    i.args.execution_profile=Some("unknown".into());
    handler_refusal(&i.run().unwrap_err(),"select",json!({"message":"unsupported execution profile: unknown"}));
    i.args.execution_profile=Some(host::EXECUTION_PROFILE.into());i.args.action=None;
    handler_refusal(&i.run().unwrap_err(),"select",json!({"message":"selected Sherlock compilation requires --action"}));
}
#[test]
fn input_io_follows_config_target_io_but_precedes_core_config_decode() {
    let i=Invocation::new(SOURCE,"quote",Some(QUOTE),false);
    let input=i.args.execution_inputs.as_ref().unwrap(); std::fs::remove_file(input).unwrap();
    std::fs::write(i.args.execution_config.as_ref().unwrap(),"{").unwrap();
    let message=source_io::read_source(input).err().unwrap().to_string();
    handler_refusal(&i.run().unwrap_err(),"inputs",json!({"message":message}));
    std::fs::remove_file(&i.args.target).unwrap();
    let message=source_io::read_source(&i.args.target).err().unwrap().to_string();
    handler_refusal(&i.run().unwrap_err(),"config",json!({"message":message}));
    std::fs::remove_file(i.args.execution_config.as_ref().unwrap()).unwrap();
    let message=source_io::read_source(i.args.execution_config.as_ref().unwrap()).err().unwrap().to_string();
    handler_refusal(&i.run().unwrap_err(),"config",json!({"message":message})); assert!(!i.args.out.exists());
}
#[test]
fn new_flag_constraints_and_absent_flag_defaults_are_actual_clap() {
    let base=vec!["compile","file","--target","target","--out","out"];
    let parse=|extra:&[&str]| parsed(base.iter().chain(extra).map(|s|s.to_string()).collect());
    assert!(parse(&["--execution-inputs","inputs"]).is_err());
    assert!(parse(&["--execution-profile",host::EXECUTION_PROFILE,"--execution-inputs","inputs"]).is_err());
    assert!(parse(&["--execution-profile",host::EXECUTION_PROFILE,"--action","quote","--entry","--execution-inputs","inputs"]).is_err());
    assert!(parse(&["--execution-profile",host::EXECUTION_PROFILE,"--action","quote","--execution-inputs","one","--execution-inputs","two"]).is_err());
    let a=parse(&[]).unwrap(); assert!(a.execution_inputs.is_none());assert!(a.execution_profile.is_none());assert!(!a.observe);
}
#[test]
fn selected_legacy_artifact_matches_existing_api_and_publication_failures_keep_run_metadata() {
    let text=Invocation::new(EMPTY_ACTION,"check",None,false).run().unwrap();
    let old=host::compile_host(EMPTY_ACTION,"check",CONFIG,TARGET,false).unwrap_or_else(|_|panic!("legacy"));
    // Shared-producer API/handler equivalence, not a separate semantic oracle.
    assert_eq!(text,serde_json::to_string_pretty(&old).unwrap());
    let i=Invocation::new(SOURCE,"quote",Some(QUOTE),true);std::fs::create_dir(&i.args.out).unwrap();
    let reference=core(SOURCE,"quote",QUOTE,true);let e=i.run().unwrap_err();
    let source=std::fs::create_dir(&i.args.out).unwrap_err();
    let actual_error=publish_io::WriteError { path:i.args.out.clone(), source };
    let expected=host::CompileFailure { stage:"publish",actual_error:json!({"message":actual_error.to_string()}),
        evaluation_performed:true,events_forwarded:reference["observation"]["events_forwarded"].as_str().unwrap().into(),completed:true };
    assert_eq!(e,handler_failure_text(&expected));
    assert_eq!(std::fs::read_dir(&i.args.out).unwrap().count(),0);
}
#[test]
fn real_writer_refusal_preserves_completed_run_without_claiming_cli_serialization_error() {
    struct Refuse;
    impl std::io::Write for Refuse {
        fn write(&mut self,_:&[u8])->std::io::Result<usize>{Err(std::io::Error::other("retained output unavailable"))}
        fn flush(&mut self)->std::io::Result<()>{Ok(())}
    }
    let a=host::compile_host_with_inputs(SOURCE,"quote",CONFIG,TARGET,QUOTE,true).unwrap_or_else(|_|panic!("actual owner"));
    let reference=serde_json::to_value(&a).unwrap();let error=serde_json::to_writer(Refuse,&a).unwrap_err();
    let f=serde_json::to_value(a.failure("serialization",error.to_string())).unwrap();
    assert_eq!(f["evaluation_performed"],true);assert_eq!(f["completed"],true);
    assert_eq!(f["events_forwarded"],reference["observation"]["events_forwarded"]);
    assert_eq!(f["actual_error"],json!({"message":"retained output unavailable"}));
}

// Supplemental controls added after the initial test-first freeze and producer
// source edits; still UNRUN.
// Except for the C3-corrected fourth case, the initial 13 tests remain
// byte-identical above; the two supplemental controls remain UNRUN.
#[test]
fn input_reader_utf8_and_size_refusals_keep_exact_payload_without_publication() {
    let i=Invocation::new(SOURCE,"quote",Some(QUOTE),false);
    let path=i.args.execution_inputs.as_ref().unwrap();
    for bytes in [vec![0xff],vec![b' ';1_048_577]] {
        std::fs::write(path,bytes).unwrap();
        let message=source_io::read_source(path).err().unwrap().to_string();
        handler_refusal(&i.run().unwrap_err(),"inputs",json!({"message":message}));assert!(!i.args.out.exists());
    }
}
#[test]
fn typed_observation_keeps_real_rejection_and_fuel_exhaustion_distinct() {
    let mut config:Value=serde_json::from_str(CONFIG).unwrap();config["fuel"]["value"]="0".into();
    let exhausted=host::compile_host_with_inputs(SOURCE,"quote",&config.to_string(),TARGET,QUOTE,true)
        .unwrap_or_else(|_|panic!("actual exhausted run"));
    let exhausted=serde_json::to_value(exhausted).unwrap();
    assert_eq!(exhausted["observation"]["summary"]["outcome"],json!({"Exhausted":{"reason":"Fuel"}}));
    let mut inputs:Value=serde_json::from_str(QUOTE).unwrap();inputs["flat"]["value"]="100".into();
    let rejected=core(SOURCE,"quote",&inputs.to_string(),true);
    assert_eq!(rejected["observation"]["summary"]["outcome"],json!({"Rejected":{"code":10}}));
}
