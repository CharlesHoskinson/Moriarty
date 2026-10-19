//! Explicit external QA consumer. Not Cargo-autodiscovered: Root selects the
//! exact --test source after corrected production source and operation review.
//! This captures actual data; it does not read independent expected cells.
mod wire;
mod independent_public_generic_inputs;

use mori_core::sherlock_compile_artifact::ext_compile::{compile_ext_host, ExtCompileConfig};
use mori_core::sherlock::helper_machine::{complete_observation as obs, shared::*};
use mori_core::sherlock::helper_machine::shared::owner_ext_source_program::{ExtDomain, ExtProgram};
use serde_json::{json, Value};
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;
use wire::Wire;

// Exactly the independent SAFE-CONFIG run input; no production release defaults.
fn configuration() -> ExtCompileConfig {
    let bounds = BoundsC {
        code_rows:256,function_rows:8,code_child_slots:256,parameter_slots:16,
        pattern_rows:128,match_plan_rows:64,match_arm_rows:32,type_rows:64,
        constructor_rows:64,constructor_field_slots:128,max_arity:4,
        constant_nodes:128,constant_field_slots:128,data_nodes:64,data_field_slots:128,
        environment_cells:64,frames:64,helper_tasks:64,serialization_bytes:1048576,
        max_reference_work:100000,max_helper_work:100000,step_cap:1,
    };
    ExtCompileConfig {
        bounds,
        layout:LayoutSchemaC {
            layout_version:("independent-public-generic-runtime".into(),"1".into()),
            field_id:None,widths:None,scalar_layout_id:None,bounds,
            capacities:LayoutCapacitiesC {
                name_rows:64,name_bytes:1024,occurrence_path_slots:1024,value_slots:4096,
            },
            ext_capacities:Some(ExtCapacitiesC {
                plan_case_slots:256,field_path_slots:512,pattern_child_slots:256,
                input_rows:0,asset_rows:0,state_rows:0,account_rows:0,liability_rows:0,
                await_site_rows:0,await_binder_slots:0,balance_cells:0,journal_rows:0,
                pending_op_rows:0,observations_per_invocation:0,max_lifetime_work:0,resume_rows:0,
            }),
            boundary_capacities_present:false,
        },
        domain:ExtDomain::GENERIC_ABSENT,fuel:RunIntC::Nat(4096),stack:RunIntC::Nat(64),
    }
}

fn new_file(dir:&Path, name:&str) -> BufWriter<File> {
    BufWriter::new(OpenOptions::new().write(true).create_new(true)
        .open(dir.join(name)).expect("fresh complete capture output file"))
}
fn line(out:&mut impl Write, value:&Value) {
    serde_json::to_writer(&mut *out,value).expect("write complete JSON observation");
    out.write_all(b"\n").expect("write observation terminator");
    out.flush().expect("flush complete observation before next owner operation");
}
fn document(dir:&Path,name:&str,value:&Value) { line(&mut new_file(dir,name),value); }

fn logical_image(image:&obs::ImageSnapshot)->Result<Value,String> {
    let mut value=image.wire();
    for (name,len) in [
        ("data",image.active.data),("data_fields",image.active.data_fields),
        ("env",image.active.env),("value_slots",image.active.value_slots),
    ] {
        let array=value.get_mut(name).and_then(Value::as_array_mut)
            .ok_or_else(||format!("missing serialized image array {name}"))?;
        if len>array.len() { return Err(format!("active {name} exceeds actual copied array")); }
        array.truncate(len);
    }
    Ok(value)
}

// This is a structural decode of the actual final bound raw refs only. It does
// not execute an expression, choose a rule or infer a source-semantic trace.
fn decoded_final(snapshot:&obs::HostSnapshot, root:RefC)->Result<Value,String> {
    enum Task { Read(RefC), Build { tag:&'static str, type_id:usize, ctor_id:usize, arity:usize } }
    let mut tasks=vec![Task::Read(root)];
    let mut values:Vec<Value>=Vec::new();
    while let Some(task)=tasks.pop() {
        match task {
            Task::Build{tag,type_id,ctor_id,arity}=>{
                let start=values.len().checked_sub(arity).ok_or("decode result underflow")?;
                let fields=values.split_off(start);
                values.push(if tag=="Nominal" {
                    json!({"Nominal":{"type_id":type_id,"ctor_id":ctor_id,"fields":fields}})
                } else {json!({tag:{"fields":fields}})});
            }
            Task::Read(reference)=>{
                let (tag,type_id,ctor_id,fields):( &str,usize,usize,Vec<RefC>)=match reference {
                    RefC::Const(id)=>{
                        let row=snapshot.admission.view.constants.get(id).ok_or("constant ref out of copied table")?;
                        let (tag,ty,ctor,span)=match row {
                            obs::ConstantSnapshot::U128{n,..}=>{values.push(json!({"UInt":n.to_string()}));continue;}
                            obs::ConstantSnapshot::Bool{b,..}=>{values.push(json!({"Bool":b}));continue;}
                            obs::ConstantSnapshot::Nil{..}=>{values.push(json!("Nil"));continue;}
                            obs::ConstantSnapshot::Cons{fields,..}=>("Cons",0,0,*fields),
                            obs::ConstantSnapshot::Leaf{fields,..}=>("Leaf",0,0,*fields),
                            obs::ConstantSnapshot::Branch{fields,..}=>("Branch",0,0,*fields),
                            obs::ConstantSnapshot::Constructor{type_id,ctor_id,fields,..}=>("Nominal",type_id.0,ctor_id.0,*fields),
                            obs::ConstantSnapshot::U128OutOfRange{..}=>return Err("actual out-of-range constant cannot decode UInt".into()),
                        };
                        let end=span.start.checked_add(span.len).ok_or("constant field span overflow")?;
                        let refs=snapshot.admission.view.constant_fields.get(span.start..end)
                            .ok_or("constant fields outside copied table")?.iter().map(|id|RefC::Const(*id)).collect();
                        (tag,ty,ctor,refs)
                    }
                    RefC::Data(id)=>{
                        if id>=snapshot.image.active.data {return Err("data ref outside actual logical prefix".into());}
                        let row=snapshot.image.data.get(id).ok_or("data ref outside copied array")?;
                        let(tag,ty,ctor,span)=match row {
                            obs::DataSnapshot::U128{n}=>{values.push(json!({"UInt":n.to_string()}));continue;}
                            obs::DataSnapshot::Bool{b}=>{values.push(json!({"Bool":b}));continue;}
                            obs::DataSnapshot::Nil=>{values.push(json!("Nil"));continue;}
                            obs::DataSnapshot::Cons{fields}=>("Cons",0,0,*fields),
                            obs::DataSnapshot::Leaf{fields}=>("Leaf",0,0,*fields),
                            obs::DataSnapshot::Branch{fields}=>("Branch",0,0,*fields),
                            obs::DataSnapshot::Constructor{type_id,ctor_id,fields}=>("Nominal",type_id.0,ctor_id.0,*fields),
                        };
                        let end=span.start.checked_add(span.len).ok_or("data field span overflow")?;
                        if end>snapshot.image.active.data_fields {return Err("data fields outside actual logical prefix".into());}
                        let refs=snapshot.image.data_fields.get(span.start..end)
                            .ok_or("data fields outside copied array")?.to_vec();
                        (tag,ty,ctor,refs)
                    }
                };
                tasks.push(Task::Build{tag,type_id,ctor_id,arity:fields.len()});
                tasks.extend(fields.into_iter().rev().map(Task::Read));
            }
        }
    }
    if values.len()!=1 {return Err("structural decode did not yield exactly one value".into());}
    Ok(values.pop().expect("checked one actual decoded value"))
}

// The full actual admission is in compile.json/genesis.admission. A reference is
// emitted only after exact typed equality; a changed admission is retained inline.
fn record_boundary(actual:&obs::HostSnapshot, genesis:&obs::HostSnapshot)->Value {
    let admission=if actual.admission==genesis.admission {
        json!({"same_as":"compile.json#/genesis/admission"})
    } else {json!({"inline":actual.admission.wire()})};
    json!({"admission":admission,"image":actual.image.wire()})
}

fn run_case(factory:&str, make:fn()->ExtProgram) {
    let root=std::env::var_os("SHERLOCK_PUBLIC_GENERIC_CAPTURE_DIR")
        .expect("Root selects explicit fresh output parent directory");
    let case_dir=Path::new(&root).join(factory);
    std::fs::create_dir(&case_dir).expect("fresh case directory; never overwrite a capture");
    let mut errors=new_file(&case_dir,"errors.jsonl");
    let mut records=new_file(&case_dir,"records.jsonl");
    let config=configuration();
    document(&case_dir,"input.json",&json!({"factory":factory,"configuration":config.wire(),"locator":null,
        "source_factory_file":"independent_public_generic_inputs.rs","expected_cells_read":false}));
    let artifact=match compile_ext_host(make(),config,None) {
        Ok(a)=>a,
        Err(error)=>{line(&mut errors,&json!({"stage":"compile_ext_host","error":error.wire()}));panic!("actual public compile failed; complete error retained");}
    };
    let genesis=match obs::capture(artifact.admitted(),artifact.genesis()) {
        Ok(s)=>s,
        Err(e)=>{line(&mut errors,&json!({"stage":"capture_genesis","error":e.wire()}));panic!("bound genesis observation failed");}
    };
    let report=match artifact.report() {
        Ok(r)=>r,
        Err(e)=>{line(&mut errors,&json!({"stage":"report","error":e.wire()}));panic!("compile report observation failed");}
    };
    let report_owner=obs::copy_observation(report.owner);
    let report_json=json!({"execution_profile":report.execution_profile,
        "configuration":report.configuration.wire(),"owner":report_owner.wire(),
        "preparation":report.preparation.wire(),"plans":serde_json::to_value(&report.plans).expect("actual plans serialization"),
        "evaluation_performed":report.evaluation_performed});
    let initial_preparation=artifact.preparation().wire();
    document(&case_dir,"compile.json",&json!({
        "schema":"sherlock-public-generic-actual-capture/1","factory":factory,
        "configuration":artifact.configuration().wire(),"canonicalProgramView":genesis.admission.view.wire(),
        "identity":genesis.admission.identity.wire(),"genesis":genesis.wire(),
        "genesisLogicalImage":logical_image(&genesis.image).expect("actual logical genesis prefixes"),
        "preparation":initial_preparation,"report":report_json,
        "report_owner_equals_genesis":report_owner==genesis,
        "unbound":["work_map.live_arms independent exactness","private admitted ext_capacities not exposed by LayoutObservation",
            "Plan status/reason normative literal disposition","full physical B prefix and B-to-source semantic mapping"],
        "qualification":"actual complete observations; no oracle comparison or execution/adoption authority"}));
    assert_eq!(report_owner,genesis,"actual report owner must equal same admitted genesis before evaluation");
    assert!(!report.evaluation_performed,"preparation report must not claim evaluation");
    let (admitted,mut image,preparation)=artifact.into_parts();
    let mut record_count=0usize;
    let mut event_count=0usize;
    let mut continuity=true;
    let mut previous=genesis.clone();
    loop {
        let step=match obs::record_step(&admitted,&mut image) {
            Ok(s)=>s,
            Err(e)=>{line(&mut errors,&json!({"stage":"record_step","record":record_count,"error":e.wire()}));panic!("actual owner/capture failed; complete available boundaries retained");}
        };
        match step {
            obs::ObservedStep::Event(record)=>{
                continuity &= previous==record.before;
                line(&mut records,&json!({"record":record_count,"kind":"Event",
                    "before":record_boundary(&record.before,&genesis),"event":record.event.wire(),
                    "after":record_boundary(&record.after,&genesis)}));
                previous=record.after;
                event_count+=1;
            }
            obs::ObservedStep::Terminal{before,after}=>{
                continuity &= previous==before;
                line(&mut records,&json!({"record":record_count,"kind":"Terminal",
                    "before":record_boundary(&before,&genesis),"after":record_boundary(&after,&genesis)}));
                previous=after;
                record_count+=1;
                break;
            }
        }
        record_count+=1;
    }
    let final_snapshot=match obs::capture(&admitted,&image) {
        Ok(s)=>s,
        Err(e)=>{line(&mut errors,&json!({"stage":"capture_final","error":e.wire()}));panic!("actual final observation failed");}
    };
    let complete_decoded_final_value=match final_snapshot.image.outcome {
        OutcomeC::Halted{value}=>match decoded_final(&final_snapshot,value) {
            Ok(v)=>v,
            Err(e)=>{line(&mut errors,&json!({"stage":"decode_final_value","error":e}));panic!("actual structural final decode failed");}
        },
        _=>Value::Null,
    };
    let same_ref=match (final_snapshot.image.control,final_snapshot.image.outcome) {
        (ControlC::Return{value:a},OutcomeC::Halted{value:b})=>Some(a==b), _=>None,
    };
    document(&case_dir,"final.json",&json!({"schema":"sherlock-public-generic-actual-capture/1",
        "factory":factory,"record_count":record_count,"event_count":event_count,
        "owner":final_snapshot.wire(),"preparation":preparation.wire(),
        "complete_decoded_final_value":complete_decoded_final_value,
        "actual_final_invariants":{"control_outcome_same_ref":same_ref,
            "continuity":continuity,"final_equals_last_record":final_snapshot==previous,
            "identity_unchanged":final_snapshot.admission.identity==genesis.admission.identity,
            "program_unchanged":final_snapshot.admission.view==genesis.admission.view,
            "admission_unchanged":final_snapshot.admission==genesis.admission,
            "preparation_unchanged":preparation.wire()==initial_preparation},
        "runtimeProducer":"compile_ext_host once; into_parts; complete_observation::record_step until actual Terminal",
        "unbound":["all independent golden comparisons pending Root actual-data freeze",
            "raw physical trace/counter equality to independent source semantics","generic semantic-return/boundary mapping"],
        "qualification":"No re-admission, reference evaluation, injected engine, predicted result or nominal-runtime eligibility claim"}));
    assert!(continuity && final_snapshot==previous,"actual record boundaries must remain continuous");
    assert!(matches!(final_snapshot.image.outcome,OutcomeC::Halted{..}),"actual owner did not halt with a value; full final capture retained");
    assert_eq!(same_ref,Some(true),"actual halted control and outcome refs differ");
}

#[test]
fn public_recursive_ordered_schedule_tables_genesis_and_runtime() {
    run_case("public_recursive_ordered_schedule",independent_public_generic_inputs::public_recursive_ordered_schedule);
}

#[test]
fn public_mutual_three_argument_tables_genesis_and_runtime() {
    run_case("public_mutual_three_argument_postcaller",independent_public_generic_inputs::public_mutual_three_argument_postcaller);
}
