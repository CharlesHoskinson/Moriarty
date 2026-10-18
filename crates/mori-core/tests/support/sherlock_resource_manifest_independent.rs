//! Report-only checks on independently frozen complete supplied observations.
//! No Machine/evaluator/advance_m/owner/SDK invocation. Explicit standalone QA.
//! Twelve A3 reports and twelve accepted-JSON-write receipts froze before source.
use std::sync::Arc;
use std::io::Write;
use mori_core::sherlock::{BinaryOp, Control, Environment, Expr, Frame, Function,
    Outcome, Parameter, Program, Snapshot, StepRecord, Type, Value, PROFILE, K_SOURCE_SHA256};
use mori_core::sherlock::reference_display::{KDisplay, environment_to_k};
use mori_core::sherlock::reference_slice::{cumulative_resource_report_with_manifest,
    ReferenceManifestResourceReport, ReferenceSlice, SliceBoundary, SliceBoundaryKind, SliceStop};
use mori_core::sherlock::helper_machine::resource_report::write_report_json;
use serde_json::{json, Value as Json};

#[path = "fixtures/sherlock_resource_manifest/typed_states.rs"]
mod typed_states;
use typed_states::FrozenCase;

const N05_INPUT: &str=include_str!("fixtures/sherlock_resource_manifest/N05-FROZEN-INPUT.json");
const N05_PROGRAM: &str=include_str!("fixtures/sherlock_resource_manifest/N05-FROZEN-PROGRAM.json");
const N08_INPUT: &str=include_str!("fixtures/sherlock_resource_manifest/N08-FROZEN-INPUT.json");
const N08_PROGRAM: &str=include_str!("fixtures/sherlock_resource_manifest/N08-FROZEN-PROGRAM.json");
const ERROR_INPUT: &str=include_str!("fixtures/sherlock_resource_manifest/ERROR-INPUT-AND-EXPECTATIONS.freeze.json");

// Static include membership, no runtime golden-path selection or directory scan.
fn goldens(case: &str) -> [(&'static str,&'static str);4] {
    match case {
        "N05"=>[
            (include_str!("fixtures/sherlock_resource_manifest/N05-WHOLE.json"),include_str!("fixtures/sherlock_resource_manifest/N05-WHOLE-WRITE.json")),
            (include_str!("fixtures/sherlock_resource_manifest/N05-FIRST.json"),include_str!("fixtures/sherlock_resource_manifest/N05-FIRST-WRITE.json")),
            (include_str!("fixtures/sherlock_resource_manifest/N05-SECOND.json"),include_str!("fixtures/sherlock_resource_manifest/N05-SECOND-WRITE.json")),
            (include_str!("fixtures/sherlock_resource_manifest/N05-CUMULATIVE.json"),include_str!("fixtures/sherlock_resource_manifest/N05-CUMULATIVE-WRITE.json"))],
        "N08-FUEL40"=>[
            (include_str!("fixtures/sherlock_resource_manifest/N08-FUEL40-WHOLE.json"),include_str!("fixtures/sherlock_resource_manifest/N08-FUEL40-WHOLE-WRITE.json")),
            (include_str!("fixtures/sherlock_resource_manifest/N08-FUEL40-FIRST.json"),include_str!("fixtures/sherlock_resource_manifest/N08-FUEL40-FIRST-WRITE.json")),
            (include_str!("fixtures/sherlock_resource_manifest/N08-FUEL40-SECOND.json"),include_str!("fixtures/sherlock_resource_manifest/N08-FUEL40-SECOND-WRITE.json")),
            (include_str!("fixtures/sherlock_resource_manifest/N08-FUEL40-CUMULATIVE.json"),include_str!("fixtures/sherlock_resource_manifest/N08-FUEL40-CUMULATIVE-WRITE.json"))],
        "ERROR10"=>[
            (include_str!("fixtures/sherlock_resource_manifest/ERROR10-WHOLE.json"),include_str!("fixtures/sherlock_resource_manifest/ERROR10-WHOLE-WRITE.json")),
            (include_str!("fixtures/sherlock_resource_manifest/ERROR10-FIRST.json"),include_str!("fixtures/sherlock_resource_manifest/ERROR10-FIRST-WRITE.json")),
            (include_str!("fixtures/sherlock_resource_manifest/ERROR10-SECOND.json"),include_str!("fixtures/sherlock_resource_manifest/ERROR10-SECOND-WRITE.json")),
            (include_str!("fixtures/sherlock_resource_manifest/ERROR10-CUMULATIVE.json"),include_str!("fixtures/sherlock_resource_manifest/ERROR10-CUMULATIVE-WRITE.json"))],
        _=>panic!("only the three independently frozen supplied cases exist"),
    }
}

fn assert_state(actual: &Snapshot, wanted: &Json, fuel: u128) {
    assert_eq!(actual.initial_fuel,fuel);
    assert_eq!(actual.stack_limit,8);
    let charged=wanted["charged_steps"].as_u64().unwrap() as u128;
    assert_eq!(actual.remaining_fuel,fuel.checked_sub(charged).unwrap());
    assert_eq!(actual.control.as_ref().map(KDisplay::to_k).unwrap_or_else(||".K".into()),wanted["control_k"]);
    assert_eq!(environment_to_k(&actual.environment),wanted["environment_k"]);
    assert_eq!(actual.frames.iter().map(KDisplay::to_k).collect::<Vec<_>>(),
        wanted["frames_top_first_k"].as_array().unwrap().iter().map(|v|v.as_str().unwrap()).collect::<Vec<_>>());
    assert_eq!(actual.frames.len() as u64,wanted["depth"].as_u64().unwrap());
    let outcome=wanted.get("output_k").and_then(Json::as_str).unwrap_or("uiPending()");
    assert_eq!(actual.outcome.to_k(),outcome);
}
fn assert_literal_inputs(case: &str, data: &FrozenCase) {
    assert_eq!(PROFILE,"sherlock-k-a/1");
    assert_eq!(K_SOURCE_SHA256,"b3746ff51dd7999b046ccb23237595605420adcfeebc0c04c0ad0f1396b11d41");
    let (input,program,fuel,count)=match case {
        "N05"=>(N05_INPUT,Some(N05_PROGRAM),27u128,28usize),
        "N08-FUEL40"=>(N08_INPUT,Some(N08_PROGRAM),40u128,41usize),
        "ERROR10"=>(ERROR_INPUT,None,9u128,11usize),_=>unreachable!(),
    };
    let input:Json=serde_json::from_str(input).unwrap();
    assert_eq!(data.states.len(),if case=="ERROR10" {count} else {count+1});
    assert_eq!(data.rules.len()+1,data.states.len());
    assert_eq!(data.charges.len(),data.rules.len());
    for (index,state) in data.states.iter().take(count).enumerate() {
        assert_state(state,&input["states"][index],fuel);
        if index>0 {assert_eq!(data.rules[index-1] as u64,input["states"][index]["via_rule_start_line"].as_u64().unwrap());}
        assert!(Arc::ptr_eq(&state.program,&data.states[0].program));
    }
    if let Some(program)=program {
        let program:Json=serde_json::from_str(program).unwrap();
        for state in &data.states {
            assert_eq!(state.program.to_k(),program["code_k"]);
            assert_eq!(state.program.entry.to_k(),program["entry_k"]);
        }
        let previous=&data.states[count-1];let last=&data.states[count];
        assert_eq!(last.control,None);
        assert_eq!(last.environment,previous.environment);
        assert_eq!(last.frames,previous.frames);
        assert_eq!(last.remaining_fuel,0);
        assert_eq!(last.initial_fuel,fuel);
        assert_eq!(last.stack_limit,8);
        assert_eq!(data.rules[count-1],if case=="N05" {80} else {84});
        assert_eq!(last.outcome,if case=="N05" {Outcome::Done(Value::UInt(6))} else {Outcome::Exhausted});
        if case=="N08-FUEL40" {
            assert_eq!(input["ordinary_fuel_prefixes"][40],json!([40,40,0,"uiExhausted()",84]));
            assert_eq!(last.frames.len(),5);
        }
    } else {
        assert_eq!(data.states[0].program.to_k(),input["code_k"]);
        assert_eq!(format!("uiEval({})",data.states[0].program.entry.to_k()),input["states"][0]["control_k"]);
        assert_eq!(data.states.last().unwrap().outcome,Outcome::Rejected(10));
        assert_eq!(data.states.last().unwrap().frames.len(),1);
    }
    assert_eq!(*data.charges.last().unwrap(),0);
    assert!(data.charges[..data.charges.len()-1].iter().all(|c|*c==1));
}

fn supplied_slices(data: &FrozenCase) -> [ReferenceSlice;3] {
    // DATA assembly only: literal states/rule lines/charges/prefixes are frozen.
    // No source control is evaluated and no semantic transition is inferred.
    let records:Vec<StepRecord>=data.rules.iter().zip(&data.charges).enumerate().map(|(i,(rule,charge))|
        StepRecord{rule_start_line:*rule,charge:*charge,before:data.states[i].clone(),after:data.states[i+1].clone()}).collect();
    let n=records.len();
    let make=|lo:usize,hi:usize,prefix:Vec<u128>,terminal:bool| ReferenceSlice {
        profile_id:PROFILE,start:data.states[lo].clone(),records:records[lo..hi].to_vec(),
        prefix_work:prefix,boundaries:vec![SliceBoundary{position:hi-lo,
            kind:if terminal {SliceBoundaryKind::Terminal} else {SliceBoundaryKind::TechnicalCut}}],
        end:data.states[hi].clone(),stop:if terminal {SliceStop::Outcome(data.states[hi].outcome.clone())} else {SliceStop::TechnicalCut},
    };
    [make(0,n,data.prefixes[0].clone(),true),make(0,data.cut,data.prefixes[1].clone(),false),
        make(data.cut,n,data.prefixes[2].clone(),true)]
}
fn write_and_read(case: &str, role: &str, report: &ReferenceManifestResourceReport,
    golden: &str, write_golden: &str)
{
    let wanted:Json=serde_json::from_str(golden).unwrap();
    let wanted_write:Json=serde_json::from_str(write_golden).unwrap();
    assert_eq!(serde_json::to_value(report).unwrap(),wanted,"complete {case}/{role} report");
    let output=std::env::var_os("SHERLOCK_RESOURCE_MANIFEST_OUTPUT_DIR")
        .expect("explicit QA supplies an existing fresh report output directory");
    let output=std::path::PathBuf::from(output).join(format!("{case}-{role}.json"));
    let mut file=std::fs::OpenOptions::new().write(true).create_new(true).open(&output)
        .expect("new exact report file; do not replace prior evidence");
    let receipt=write_report_json(report,&mut file).expect("complete report JSON accepted by real File writer");
    file.flush().expect("flush successful report before readback");
    drop(file);
    let written=std::fs::read(&output).expect("read back exact newly written report bytes");
    assert_eq!(serde_json::from_slice::<Json>(&written).unwrap(),wanted,"complete persisted {case}/{role} fields");
    assert_eq!(serde_json::to_value(&receipt).unwrap(),wanted_write,"complete Write-boundary receipt");
    assert_eq!(written.len().to_string(),wanted_write["serialization_bytes"]);
    assert_eq!(written.len().to_string(),wanted_write["io_bytes"]);
    assert!(!written.ends_with(b"\n"),"compact actual write has no newline");
    println!("accepted_report_file={};write_observation={}",output.display(),serde_json::to_string(&receipt).unwrap());
}
fn check_case(case: &str, data: FrozenCase) {
    assert_literal_inputs(case,&data);
    let [whole,first,second]=supplied_slices(&data);
    let original=(whole.clone(),first.clone(),second.clone());
    assert_eq!(first.end,second.start,"complete inter-chunk state identity");
    let whole_report=whole.resource_report_with_manifest().expect("whole supplied report");
    let first_report=first.resource_report_with_manifest().expect("first supplied report");
    let second_report=second.resource_report_with_manifest().expect("second supplied report");
    let cumulative=cumulative_resource_report_with_manifest(&[&first,&second]).expect("two supplied chunks");
    assert_eq!(serde_json::to_value(&whole_report).unwrap(),serde_json::to_value(&cumulative).unwrap());
    let expected=goldens(case);
    for ((role,report),(golden,write_golden)) in [("WHOLE",&whole_report),("FIRST",&first_report),
        ("SECOND",&second_report),("CUMULATIVE",&cumulative)].into_iter().zip(expected) {
        write_and_read(case,role,report,golden,write_golden);
    }
    assert_eq!((whole,first,second),original,"reporting preserves all supplied states/records/prefixes/boundaries");
}
#[test]
fn resource_manifest_n05_calls_branches_and_chunk_work() {check_case("N05",typed_states::n05());}
#[test]
fn resource_manifest_n08_fuel_cut_and_retained_frames() {check_case("N08-FUEL40",typed_states::n08_fuel40());}
#[test]
fn resource_manifest_error10_preserves_work_and_separate_planes() {check_case("ERROR10",typed_states::error10());}
