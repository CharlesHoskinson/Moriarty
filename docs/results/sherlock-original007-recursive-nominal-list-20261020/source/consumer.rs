//! One externally selected ordinary UIREQ-007 case. SOURCE_ONLY_UNRUN.
//! Actual public text compiler/admission/genesis/owner stepping; no new engine.
//! Expected named value is source-aware and frozen before this consumer.
mod wire;
mod nominal_wire;
use wire::Wire;
use mori_core::sherlock::helper_machine::{complete_observation as obs, shared::*};
use mori_core::sherlock::helper_machine::shared::{adt::{TypeIdC,CtorIdC},
    owner_ext_source_program::ExtDomain, owner_ext_type_builder::TypeExprSrc};
use mori_core::sherlock_compile_artifact::ext_compile::ExtCompileConfig;
use mori_core::sherlock_compile_artifact::nominal_body_compile::compile_nominal_source_host_bounded;
use mori_core::sherlock_nominal_source::DraftParseBudget;
use serde_json::{json,Value};
use std::collections::BTreeSet;
use std::fs::{File,OpenOptions};
use std::io::Write;
use std::path::Path;

include!("configuration.rs");
const SOURCE:&str=include_str!("original007-recursive-list.mori");
const EXPECTED:&str=include_str!("EXPECTED-SEMANTICS.json");
const CALL_CEILING:usize=512;
const OUTPUT_NAMES:[&str;8]=["source.mori","input.json","compile.json","genesis.json",
    "records.jsonl","final.json","checks.json","errors.jsonl"];

fn fresh_file(dir:&Path,name:&str)->File {
    OpenOptions::new().write(true).create_new(true).open(dir.join(name))
        .expect("fresh output; never overwrite actual evidence")
}
fn line(out:&mut File,value:&Value) {
    serde_json::to_writer(&mut *out,value).expect("serialize complete actual public data");
    out.write_all(b"\n").expect("JSON record delimiter");
    out.flush().expect("flush before next owner operation");
}
fn document(dir:&Path,name:&str,value:&Value) {
    let mut out=fresh_file(dir,name);line(&mut out,value);
    out.sync_all().expect("sync complete document");
}
fn fail(errors:&mut File,stage:&str,payload:Value)->! {
    line(errors,&json!({"stage":stage,"actual_error":payload}));
    errors.sync_all().expect("sync original error before failing");
    panic!("{stage}: original payload retained in errors.jsonl");
}
fn name(view:&obs::ProgramSnapshot,id:usize)->Result<&str,String> {
    view.names.get(id).map(String::as_str).ok_or_else(||format!("name index {id} outside actual table"))
}
#[derive(Clone,Copy)]
struct ListIdentity { ty:TypeIdC, nil:CtorIdC, cons:CtorIdC }
fn identity(view:&obs::ProgramSnapshot)->Result<ListIdentity,String> {
    let matches:Vec<_>=view.types.iter().enumerate().filter(|(_,t)|
        view.names.get(t.name_id).map(String::as_str)==Some("ListUInt128") && t.decl.is_some()).collect();
    if matches.len()!=1 {return Err("expected exactly one actual declared ListUInt128 type".into());}
    let (id,t)=matches[0];let ty=TypeIdC(id);
    if [TypeIdC::UInt,TypeIdC::Bool,TypeIdC::List,TypeIdC::Tree].contains(&ty) {
        return Err("declared nominal type cannot be a dedicated builtin".into());
    }
    if t.args.len!=0 || t.ctor_len!=2 {return Err("ListUInt128 parameters/constructor arity differ".into());}
    let decl_index=t.decl.ok_or("missing nominal declaration")?;
    let decl=view.decl_templates.get(decl_index).ok_or("declaration index outside actual templates")?;
    if decl.name!="ListUInt128" || !decl.parameters.is_empty() || decl.ctors.len()!=2 {
        return Err("actual declaration template differs".into());
    }
    let dc=&decl.ctors;
    if dc[0].name!="Nil" || !dc[0].fields.is_empty() || dc[1].name!="Cons" || dc[1].fields.len()!=2 {
        return Err("actual source constructor declarations/order differ".into());
    }
    if dc[1].fields[0].name!="head" || !matches!(&dc[1].fields[0].ty,TypeExprSrc::UInt)
        || dc[1].fields[1].name!="tail"
        || !matches!(&dc[1].fields[1].ty,TypeExprSrc::Named{name} if name=="ListUInt128") {
        return Err("actual source Cons field names/types/order differ".into());
    }
    if view.decl_param_counts.get(decl_index)!=Some(&0) || view.decl_ctor_counts.get(decl_index)!=Some(&2) {
        return Err("actual declaration counts differ".into());
    }
    let end=t.ctor_start.0.checked_add(t.ctor_len).ok_or("constructor span overflow")?;
    let ctors=view.constructors.get(t.ctor_start.0..end).ok_or("constructor span outside actual table")?;
    let nil=t.ctor_start;let cons=CtorIdC(nil.0.checked_add(1).ok_or("constructor id overflow")?);
    for (ordinal,c) in ctors.iter().enumerate() {
        if c.owning_type!=ty || c.source_order!=ordinal || name(view,c.name_id)?!=["Nil","Cons"][ordinal]
            || c.field_len!=[0,2][ordinal] {return Err("actual canonical constructor rows differ".into());}
        let field_end=c.field_start.checked_add(c.field_len).ok_or("constructor field span overflow")?;
        let fields=view.ctor_fields.get(c.field_start..field_end).ok_or("field span outside actual table")?;
        if ordinal==1 && (name(view,fields[0].name_id)?!="head" || fields[0].type_id!=TypeIdC::UInt
            || name(view,fields[1].name_id)?!="tail" || fields[1].type_id!=ty) {
            return Err("actual canonical Cons signature differs".into());
        }
    }
    Ok(ListIdentity{ty,nil,cons})
}

// Decode the one supplied actual result's references; no expression evaluation,
// rule selection, expected result injection or speculative nominal ID assignment.
fn decode(snapshot:&obs::HostSnapshot,root:RefC,ids:ListIdentity,
    ancestors:&mut Vec<RefC>,visits:&mut usize)->Result<Value,String> {
    *visits=visits.checked_add(1).ok_or("decode visit overflow")?;
    if *visits>512 || ancestors.contains(&root) {return Err("actual result cycle/decode ceiling".into());}
    let (ty,ctor,refs)=match root {
        RefC::Const(id)=>{
            let c=snapshot.admission.view.constants.get(id).ok_or("constant reference out of range")?;
            match c {
                obs::ConstantSnapshot::U128{marker_free:true,n}=>return Ok(json!({"type":"UInt","value":n.to_string()})),
                obs::ConstantSnapshot::Constructor{marker_free:true,type_id,ctor_id,fields}=>{
                    let end=fields.start.checked_add(fields.len).ok_or("constant field span overflow")?;
                    let refs=snapshot.admission.view.constant_fields.get(fields.start..end)
                        .ok_or("constant field span out of range")?.iter().map(|id|RefC::Const(*id)).collect::<Vec<_>>();
                    (*type_id,*ctor_id,refs)
                }
                _=>return Err("result contains a non-nominal constructor, non-UInt or marked constant".into()),
            }
        }
        RefC::Data(id)=>{
            if id>=snapshot.image.active.data {return Err("data reference outside actual active prefix".into());}
            match snapshot.image.data.get(id).ok_or("data reference outside copied table")? {
                obs::DataSnapshot::U128{n}=>return Ok(json!({"type":"UInt","value":n.to_string()})),
                obs::DataSnapshot::Constructor{type_id,ctor_id,fields}=>{
                    let end=fields.start.checked_add(fields.len).ok_or("data field span overflow")?;
                    if end>snapshot.image.active.data_fields {return Err("data fields outside active prefix".into());}
                    let refs=snapshot.image.data_fields.get(fields.start..end).ok_or("data fields outside copied table")?.to_vec();
                    (*type_id,*ctor_id,refs)
                }
                _=>return Err("result contains a dedicated/non-nominal or non-UInt value".into()),
            }
        }
    };
    if ty!=ids.ty {return Err("actual constructor has a different nominal type".into());}
    let (constructor,arity)=if ctor==ids.nil {("Nil",0)} else if ctor==ids.cons {("Cons",2)}
        else {return Err("actual constructor is not declared Nil/Cons".into());};
    if refs.len()!=arity {return Err("actual constructor result arity differs".into());}
    ancestors.push(root);
    let result=refs.into_iter().map(|r|decode(snapshot,r,ids,ancestors,visits)).collect::<Result<Vec<_>,_>>();
    ancestors.pop();
    Ok(json!({"type":"ListUInt128","constructor":constructor,"fields":result?}))
}
fn immutable_prefix(before:&obs::HostSnapshot,after:&obs::HostSnapshot)->bool {
    let b=&before.image;let a=&after.image;
    before.admission==after.admission && a.active.data>=b.active.data && a.active.data_fields>=b.active.data_fields
        && match (b.data.get(..b.active.data),a.data.get(..b.active.data)) { (Some(x),Some(y))=>x==y,_=>false }
        && match (b.data_fields.get(..b.active.data_fields),a.data_fields.get(..b.active.data_fields)) {
            (Some(x),Some(y))=>x==y,_=>false }
}
fn frames_seen(s:&obs::HostSnapshot,ids:ListIdentity,sites:&mut BTreeSet<usize>) {
    for frame in &s.image.frames {
        if let FrameRowC::ConstructorFieldsF{type_id,ctor_id,origin,..}=frame {
            if *type_id==ids.ty && *ctor_id==ids.cons {sites.insert(*origin);}
        }
    }
}

#[test]
fn original007_declared_recursive_list_has_ordered_nominal_value() {
    let dir=std::env::var_os("UIREQ007_CAPTURE_DIR").expect("Root supplies a new exact output directory");
    let dir=Path::new(&dir);std::fs::create_dir(dir).expect("new capture directory only");
    let mut errors=fresh_file(dir,"errors.jsonl");let mut records=fresh_file(dir,"records.jsonl");
    let mut src=fresh_file(dir,"source.mori");src.write_all(SOURCE.as_bytes()).expect("retain actual input");
    src.sync_all().expect("sync actual input");
    let expected:Value=serde_json::from_str(EXPECTED).expect("frozen source-aware expectation JSON");
    let config=configuration();
    document(dir,"input.json",&json!({"source":SOURCE,"configuration":config.wire(),
        "parser_budget":parser_budget().wire(),"locator":null,"record_step_call_ceiling":CALL_CEILING,
        "expectation_provenance":expected["provenance"],"expected_semantics_file":"EXPECTED-SEMANTICS.json",
        "qualification":"One original007 ordinary case; no full trace/clock golden or execution/adoption authority"}));
    let typed=match compile_nominal_source_host_bounded(SOURCE,parser_budget(),config,None) {
        Ok(a)=>a,Err(e)=>fail(&mut errors,"bounded_nominal_body_compile",e.wire()),
    };
    let genesis=match obs::capture(typed.owner().admitted(),typed.owner().genesis()) {
        Ok(s)=>s,Err(e)=>fail(&mut errors,"capture_genesis",e.wire()),
    };
    document(dir,"genesis.json",&genesis.wire());
    let report=match typed.report() {Ok(r)=>r,Err(e)=>fail(&mut errors,"compile_report",e.wire())};
    let report_owner=obs::copy_observation(report.owner.owner);
    let initial_preparation=report.owner.preparation.wire();
    document(dir,"compile.json",&json!({"name":typed.name(),"provenance":typed.provenance().wire(),
        "source_profile":report.source_profile,"body_check":{"status":report.body_check.status,
            "judgments":report.body_check.judgments},
        "owner":{"execution_profile":report.owner.execution_profile,"configuration":report.owner.configuration.wire(),
            "observation":report_owner.wire(),"preparation":report.owner.preparation.wire(),
            "plans":serde_json::to_value(&report.owner.plans).expect("actual plans JSON"),
            "evaluation_performed":report.owner.evaluation_performed},
        "report_owner_equals_genesis":report_owner==genesis}));
    if report_owner!=genesis || report.owner.evaluation_performed {
        fail(&mut errors,"compile_genesis_correspondence",json!("actual report/genesis differ or preparation claims evaluation"));
    }
    let genesis_input_matches=genesis.image.initial_fuel==4096 && genesis.image.remaining_work==4096
        && genesis.image.stack_limit==64 && genesis.image.outcome==OutcomeC::Running
        && matches!(genesis.image.control,ControlC::Eval{expr} if expr==genesis.admission.view.entry)
        && (genesis.image.consumed_work,genesis.image.helper_work,
            genesis.image.helper_visits,genesis.image.reference_cursor)==(0,0,0,0);
    if !genesis_input_matches || report.source_profile!="moriarty-sherlock-nominal-body-typed/1"
        || report.body_check.status!="completed" || typed.name()!="Original007RecursiveList" {
        fail(&mut errors,"actual_source_profile_or_genesis_inputs",json!("actual profile/name/body report or initial fuel/stack/control/counters differ"));
    }
    let ids=match identity(&genesis.admission.view) {Ok(x)=>x,Err(e)=>fail(&mut errors,"nominal_identity",json!(e))};
    let mut cons_sites=BTreeSet::new();let mut nil_sites=BTreeSet::new();
    for (id,expr) in genesis.admission.view.expressions.iter().enumerate() {
        if let obs::ExpressionSnapshot::Construct{type_id,ctor_id,fields,constant}=expr {
            if *type_id==ids.ty && *ctor_id==ids.cons && fields.len==2 && constant.is_none() {cons_sites.insert(id);}
            if *type_id==ids.ty && *ctor_id==ids.nil && fields.len==0 && constant.is_some() {nil_sites.insert(id);}
        }
    }
    let (_,_,owner)=typed.into_parts();let (admitted,mut image,preparation)=owner.into_parts();
    let mut previous=genesis.clone();let mut event_count=0usize;let mut call_count=0usize;
    let mut terminal=false;let mut continuity=true;let mut admission_unchanged=true;let mut values_immutable=true;
    let mut field_events=BTreeSet::new();let mut nullary_events=BTreeSet::new();let mut frame_sites=BTreeSet::new();
    for call in 0..CALL_CEILING {
        call_count=call+1;
        let observed=match obs::record_step(&admitted,&mut image) {
            Ok(s)=>s,Err(e)=>fail(&mut errors,"record_step",json!({"call":call,"failure":e.wire()})),
        };
        // Write every complete original value before supplementary checks or projection.
        line(&mut records,&json!({"call":call,"actual":observed.wire()}));
        match observed {
            obs::ObservedStep::Event(record)=>{
                continuity &= previous==record.before;
                admission_unchanged &= record.before.admission==genesis.admission && record.after.admission==genesis.admission;
                values_immutable &= immutable_prefix(&record.before,&record.after);
                frames_seen(&record.before,ids,&mut frame_sites);frames_seen(&record.after,ids,&mut frame_sites);
                if record.event.rule_id==RuleIdC::XAdtEvalConstructFields {field_events.insert(record.event.source_expr_id);}
                if record.event.rule_id==RuleIdC::XAdtEvalConstructNullary {nullary_events.insert(record.event.source_expr_id);}
                previous=record.after;event_count+=1;
            }
            obs::ObservedStep::Terminal{before,after}=>{
                continuity &= previous==before;
                admission_unchanged &= before.admission==genesis.admission && after.admission==genesis.admission;
                values_immutable &= immutable_prefix(&before,&after);previous=after;terminal=true;break;
            }
        }
    }
    records.sync_all().expect("sync all actual records before final assertions");
    let final_snapshot=match obs::capture(&admitted,&image) {
        Ok(s)=>s,Err(e)=>fail(&mut errors,"capture_final",e.wire()),
    };
    let mut visits=0;
    let root=match final_snapshot.image.outcome {OutcomeC::Halted{value}=>Some(value),_=>None};
    let decoded=match root {Some(r)=>decode(&final_snapshot,r,ids,&mut Vec::new(),&mut visits),
        None=>Err("actual final outcome is not Halted".into())};
    let decoded_wire=match &decoded {Ok(v)=>json!({"Ok":v}),Err(e)=>json!({"Err":e})};
    let final_same=final_snapshot==previous;
    let semantic_equal=decoded.as_ref().map(|v|v==&expected["expected_final_value"]).unwrap_or(false);
    let control_same=matches!((final_snapshot.image.control,root),(ControlC::Return{value:a},Some(b)) if a==b);
    document(dir,"final.json",&json!({"owner":final_snapshot.wire(),"preparation":preparation.wire(),
        "actual_root_ref":root.wire(),"resolved_whole_typed_value":decoded_wire,"reachable_value_occurrences":visits,
        "actual_nominal_ids":{"type_id":ids.ty.wire(),"nil_ctor_id":ids.nil.wire(),"cons_ctor_id":ids.cons.wire()},
        "call_count":call_count,"event_count":event_count,"terminal_observed":terminal,
        "unbound":["exact independent B clock","full independent trace golden","all constructor programs/refinements","native/proof/target"],
        "qualification":"Actual copied tables and structural reference decode, not a second evaluator or whole007 acceptance"}));
    let preparation_unchanged=preparation.wire()==initial_preparation;
    let passed=terminal && continuity && admission_unchanged && values_immutable && final_same && semantic_equal
        && control_same && cons_sites.len()==2 && nil_sites.len()==1 && field_events==cons_sites
        && frame_sites==cons_sites && nullary_events==nil_sites && visits==5 && preparation_unchanged;
    document(dir,"checks.json",&json!({"passed":passed,"terminal":terminal,"continuity":continuity,
        "admission_unchanged":admission_unchanged,"active_value_prefixes_immutable":values_immutable,
        "final_equals_last_record":final_same,"control_outcome_same_reference":control_same,
        "genesis_run_inputs_match":genesis_input_matches,"preparation_unchanged":preparation_unchanged,
        "source_aware_expected_named_value_equal":semantic_equal,"reachable_occurrences":visits,
        "actual_cons_source_sites":cons_sites,"actual_nil_source_sites":nil_sites,
        "observed_fields_entry_sites":field_events,"observed_constructor_frame_sites":frame_sites,
        "observed_nullary_entry_sites":nullary_events,
        "scope":"One finite original007 case; binder/formal/full-consumer predicates and all acceptance remain separate"}));
    if !passed {
        line(&mut errors,&json!({"stage":"original007_predicate",
            "actual_error":"complete captures/checks retained; at least one predicate failed"}));
        errors.sync_all().expect("sync predicate failure before output inventory");
    }
    let output_rows=OUTPUT_NAMES.iter().map(|n|json!({"path":n,
        "bytes":std::fs::metadata(dir.join(n)).expect("listed actual output exists").len()})).collect::<Vec<_>>();
    document(dir,"output-manifest.json",&json!({"files":output_rows,
        "qualification":"Actual file lengths only; Root collector separately hashes/freezes outputs. No self-hash or predicted output."}));
    assert!(passed,"original007 predicate failed; complete actual evidence retained");
    errors.sync_all().expect("sync empty or complete error stream");
}
