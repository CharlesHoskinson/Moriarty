//! Normal public codec capture. No expected lane literals or private oracle input.
use mori_core::sherlock::helper_machine::{c6_phys::*, c6_phys_data::C6StateCaps, c6_phys_helper::*};
use mori_core::sherlock::helper_machine::shared::*;
use mori_core::sherlock::helper_machine::shared::adt::{CtorIdC, TypeIdC};
mod positive_inputs;
use positive_inputs::{helper_cases, result_cases};

type CaptureResult<T> = Result<T, String>;
fn q(s: &str) -> String { serde_json::to_string(s).expect("JSON string serialization") }
fn object(fields: &[(&str, String)]) -> String {
    format!("{{{}}}", fields.iter().map(|(k,v)|format!("{}:{}",q(k),v)).collect::<Vec<_>>().join(","))
}
fn array(rows: &[String]) -> String { format!("[{}]", rows.join(",")) }
fn tagged(tag: &str, value: String) -> String { object(&[(tag, value)]) }
fn unit(tag: &str) -> String { tagged(tag, "null".into()) }
fn number(n: impl std::fmt::Display) -> String { n.to_string() }
fn lanes(xs: &[u128]) -> String { array(&xs.iter().map(number).collect::<Vec<_>>()) }
fn option_index(v: Option<usize>) -> String {
    match v { Some(n) => tagged("Some", number(n)), None => unit("None") }
}
fn reference(v: RefC) -> String {
    match v { RefC::Const(n) => tagged("Const",number(n)), RefC::Data(n) => tagged("Data",number(n)) }
}
fn option_ref(v: Option<RefC>) -> String {
    match v { Some(r) => tagged("Some", reference(r)), None => unit("None") }
}
fn slice(s: SliceC) -> String { object(&[("start",number(s.start)),("len",number(s.len))]) }
fn common(c: HelperCommon) -> String {
    object(&[("site",q(&format!("{:?}",c.site))),
        ("origin_reference_cursor",number(c.origin_reference_cursor)),
        ("parent_resume",option_index(c.parent_resume))])
}
macro_rules! obj { ($($key:literal => $value:expr),* $(,)?) => { object(&[$(($key, $value)),*]) }; }
fn helper_json(h: Option<HelperControlC>) -> CaptureResult<String> {
    use HelperControlC as H;
    Ok(match h {
        None => "null".into(),
        Some(H::Scalar{..}) => return Err("Scalar helper is outside the independently frozen cohort".into()),
        Some(H::EnvGet{common:c,binder,cursor,found,status}) => tagged("EnvGet",obj!{
            "common"=>common(c),"binder"=>number(binder),"cursor"=>option_index(cursor),"found"=>option_ref(found),"status"=>q(&format!("{status:?}"))}),
        Some(H::EnvPut{common:c,binder,value,base,new_root,status}) => tagged("EnvPut",obj!{
            "common"=>common(c),"binder"=>number(binder),"value"=>reference(value),"base"=>option_index(base),"new_root"=>option_index(new_root),"status"=>q(&format!("{status:?}"))}),
        Some(H::Find{common:c,function_name,row_cursor,found,status}) => tagged("Find",obj!{
            "common"=>common(c),"function_name"=>number(function_name),"row_cursor"=>number(row_cursor),"found"=>option_index(found),"status"=>q(&format!("{status:?}"))}),
        Some(H::Bind{common:c,function,params,args,p_cursor,a_cursor,partial,dup_cursor,phase,status}) => tagged("Bind",obj!{
            "common"=>common(c),"function"=>number(function),"params"=>slice(params),"args"=>slice(args),"p_cursor"=>number(p_cursor),"a_cursor"=>number(a_cursor),"partial"=>option_index(partial),"dup_cursor"=>option_index(dup_cursor),"phase"=>q(&format!("{phase:?}")),"status"=>q(&format!("{status:?}"))}),
        Some(H::Append{common:c,src,value,src_cursor,dest_start,staged,phase}) => tagged("Append",obj!{
            "common"=>common(c),"src"=>slice(src),"value"=>reference(value),"src_cursor"=>number(src_cursor),"dest_start"=>number(dest_start),"staged"=>option_ref(staged),"phase"=>q(&format!("{phase:?}"))}),
        Some(H::Validate{common:c,root,status}) => tagged("Validate",obj!{
            "common"=>common(c),"root"=>reference(root),"status"=>q(&format!("{status:?}"))}),
        Some(H::Equal{common:c,left,right,status}) => tagged("Equal",obj!{
            "common"=>common(c),"left"=>reference(left),"right"=>reference(right),"status"=>q(&format!("{status:?}"))}),
        Some(H::Match{common:c,scrutinee,arm,saved_env,partial,pat_tag,x,y,field_idx,staged,phase,status}) => tagged("Match",obj!{
            "common"=>common(c),"scrutinee"=>reference(scrutinee),"arm"=>number(arm),"saved_env"=>option_index(saved_env),"partial"=>option_index(partial),"pat_tag"=>q(&format!("{pat_tag:?}")),"x"=>number(x),"y"=>number(y),"field_idx"=>number(field_idx),"staged"=>option_ref(staged),"phase"=>q(&format!("{phase:?}")),"status"=>q(&format!("{status:?}"))}),
        Some(H::Plan{common:c,source_match,scrutinee,saved_env,partial,node,phase,after_walk,cur,path,path_cursor,arm,status}) => tagged("Plan",obj!{
            "common"=>common(c),"source_match"=>number(source_match),"scrutinee"=>reference(scrutinee),"saved_env"=>option_index(saved_env),"partial"=>option_index(partial),"node"=>number(node),"phase"=>q(&format!("{phase:?}")),"after_walk"=>q(&format!("{after_walk:?}")),"cur"=>reference(cur),"path"=>slice(path),"path_cursor"=>number(path_cursor),"arm"=>option_index(arm),"status"=>q(&format!("{status:?}"))}),
        Some(H::Build{common:c,type_id,ctor_id,src,src_cursor,field_start,staged,node,phase}) => tagged("Build",obj!{
            "common"=>common(c),"type_id"=>number(type_id.0),"ctor_id"=>number(ctor_id.0),"src"=>slice(src),"src_cursor"=>number(src_cursor),"field_start"=>number(field_start),"staged"=>option_ref(staged),"node"=>option_ref(node),"phase"=>q(&format!("{phase:?}"))}),
    })
}
fn result_json(r: Option<HelperResultC>) -> String {
    use HelperResultC as R;
    match r {
        None => "null".into(),
        Some(R::Validated{ok}) => tagged("Validated",obj!{"ok"=>number(ok)}),
        Some(R::EnvFound{value}) => tagged("EnvFound",obj!{"value"=>reference(value)}),
        Some(R::EnvMissing) => unit("EnvMissing"),
        Some(R::EnvUpdated{cell}) => tagged("EnvUpdated",obj!{"cell"=>number(cell)}),
        Some(R::FindFound{function}) => tagged("FindFound",obj!{"function"=>number(function)}),
        Some(R::FindMissing) => unit("FindMissing"),
        Some(R::BindOk{env}) => tagged("BindOk",obj!{"env"=>option_index(env)}),
        Some(R::BindBad) => unit("BindBad"),
        Some(R::AppendDone{seq}) => tagged("AppendDone",obj!{"seq"=>slice(seq)}),
        Some(R::ScalarValue{value}) => tagged("ScalarValue",obj!{"value"=>reference(value)}),
        Some(R::ScalarError{code}) => tagged("ScalarError",obj!{"code"=>number(code)}),
        Some(R::ScalarSelector{sel}) => tagged("ScalarSelector",obj!{"sel"=>q(&format!("{sel:?}"))}),
        Some(R::ScalarTypeTest{ok}) => tagged("ScalarTypeTest",obj!{"ok"=>number(ok)}),
        Some(R::EqualDone{value}) => tagged("EqualDone",obj!{"value"=>reference(value)}),
        Some(R::MatchMatched{env}) => tagged("MatchMatched",obj!{"env"=>option_index(env)}),
        Some(R::MatchNoMatch) => unit("MatchNoMatch"),
        Some(R::PlanMatched{arm,env}) => tagged("PlanMatched",obj!{"arm"=>number(arm),"env"=>option_index(env)}),
        Some(R::PlanNoMatch) => unit("PlanNoMatch"),
        Some(R::BuildDone{value}) => tagged("BuildDone",obj!{"value"=>reference(value)}),
    }
}
fn helper_variant(h: Option<HelperControlC>) -> &'static str {
    use HelperControlC::*;
    match h {None=>"None",Some(Scalar{..})=>"Scalar",Some(EnvGet{..})=>"EnvGet",Some(EnvPut{..})=>"EnvPut",Some(Find{..})=>"Find",Some(Bind{..})=>"Bind",Some(Append{..})=>"Append",Some(Validate{..})=>"Validate",Some(Equal{..})=>"Equal",Some(Match{..})=>"Match",Some(Plan{..})=>"Plan",Some(Build{..})=>"Build"}
}
fn result_variant(r: Option<HelperResultC>) -> &'static str {
    use HelperResultC::*;
    match r {None=>"None",Some(Validated{..})=>"Validated",Some(EnvFound{..})=>"EnvFound",Some(EnvMissing)=>"EnvMissing",Some(EnvUpdated{..})=>"EnvUpdated",Some(FindFound{..})=>"FindFound",Some(FindMissing)=>"FindMissing",Some(BindOk{..})=>"BindOk",Some(BindBad)=>"BindBad",Some(AppendDone{..})=>"AppendDone",Some(ScalarValue{..})=>"ScalarValue",Some(ScalarError{..})=>"ScalarError",Some(ScalarSelector{..})=>"ScalarSelector",Some(ScalarTypeTest{..})=>"ScalarTypeTest",Some(EqualDone{..})=>"EqualDone",Some(MatchMatched{..})=>"MatchMatched",Some(MatchNoMatch)=>"MatchNoMatch",Some(PlanMatched{..})=>"PlanMatched",Some(PlanNoMatch)=>"PlanNoMatch",Some(BuildDone{..})=>"BuildDone"}
}
fn caps() -> C6StateCaps {
    C6StateCaps { phys: C6PhysCaps {
        max_frames:4096,max_expressions:4096,max_names:4096,max_arms:4096,
        max_children:4096,max_value_slots:4096,max_constants:4096,max_data_nodes:4096,
        max_env_cells:4096,max_functions:4096,max_arity:4096,max_types:4096,
        max_occurrences:4096,max_occurrence_paths:4096,max_parameters:4096,
        max_patterns:4096,max_pattern_children:4096,max_constructors:4096,
        max_ctor_fields:4096,max_decl_templates:4096,max_plan_nodes:4096,
        max_plan_cases:4096,max_field_paths:4096,max_constant_fields:4096,
        max_name_bytes:4096,max_type_expr_nodes:4096,max_template_bytes:4096,
    }, max_data_field_slots:4096,max_helper_tasks:4096 }
}
fn caps_json(c: &C6StateCaps) -> String {
    obj!{ "phys"=>object(&[
        ("max_frames",number(c.phys.max_frames)),
        ("max_expressions",number(c.phys.max_expressions)),
        ("max_names",number(c.phys.max_names)),
        ("max_arms",number(c.phys.max_arms)),
        ("max_children",number(c.phys.max_children)),
        ("max_value_slots",number(c.phys.max_value_slots)),
        ("max_constants",number(c.phys.max_constants)),
        ("max_data_nodes",number(c.phys.max_data_nodes)),
        ("max_env_cells",number(c.phys.max_env_cells)),
        ("max_functions",number(c.phys.max_functions)),
        ("max_arity",number(c.phys.max_arity)),
        ("max_types",number(c.phys.max_types)),
        ("max_occurrences",number(c.phys.max_occurrences)),
        ("max_occurrence_paths",number(c.phys.max_occurrence_paths)),
        ("max_parameters",number(c.phys.max_parameters)),
        ("max_patterns",number(c.phys.max_patterns)),
        ("max_pattern_children",number(c.phys.max_pattern_children)),
        ("max_constructors",number(c.phys.max_constructors)),
        ("max_ctor_fields",number(c.phys.max_ctor_fields)),
        ("max_decl_templates",number(c.phys.max_decl_templates)),
        ("max_plan_nodes",number(c.phys.max_plan_nodes)),
        ("max_plan_cases",number(c.phys.max_plan_cases)),
        ("max_field_paths",number(c.phys.max_field_paths)),
        ("max_constant_fields",number(c.phys.max_constant_fields)),
        ("max_name_bytes",number(c.phys.max_name_bytes)),
        ("max_type_expr_nodes",number(c.phys.max_type_expr_nodes)),
        ("max_template_bytes",number(c.phys.max_template_bytes)),
    ]),"max_data_field_slots"=>number(c.max_data_field_slots),"max_helper_tasks"=>number(c.max_helper_tasks),"admitted"=>"false".into()}
}
fn capture() -> CaptureResult<String> {
    let c=caps();
    let mut hv=Vec::new(); let mut hd=Vec::new();
    let mut rv=Vec::new(); let mut rd=Vec::new();
    for (id,input) in helper_cases() {
        let record=encode_helper_record(input);
        let decoded=decode_helper_record_checked(&record,&c).map_err(|e|format!("{id}: record decode: {e:?}"))?;
        let tables=C6HelperTables{helper:input,result:None,tasks:Vec::new()};
        let section=encode_helper_tables(&tables,&c).map_err(|e|format!("{id}: section encode: {e:?}"))?;
        let back=decode_helper_tables(&section,&c).map_err(|e|format!("{id}: section decode: {e:?}"))?;
        if decoded!=input || back!=tables || section[L_HELPER_RECORD..L_RESULT_RECORD]!=record || section.len()!=HELPER_FIXED_LANES {
            return Err(format!("{id}: positive typed roundtrip/framing mismatch"));
        }
        hv.push(obj!{"case_id"=>q(id),"variant"=>if input.is_none() { "null".into() } else { q(helper_variant(input)) },"lanes"=>lanes(&record)});
        hd.push(obj!{"case_id"=>q(id),"input"=>helper_json(input)?,"decoded_record"=>helper_json(decoded)?,
            "decoded_section_helper"=>helper_json(back.helper)?,"decoded_section_result"=>result_json(back.result),
            "decoded_section_tasks"=>array(&[]),"section_lanes"=>lanes(&section)});
    }
    for (id,input) in result_cases() {
        let record=encode_result_record(input);
        let decoded=decode_result_record_checked(&record,&c).map_err(|e|format!("{id}: record decode: {e:?}"))?;
        let tables=C6HelperTables{helper:None,result:input,tasks:Vec::new()};
        let section=encode_helper_tables(&tables,&c).map_err(|e|format!("{id}: section encode: {e:?}"))?;
        let back=decode_helper_tables(&section,&c).map_err(|e|format!("{id}: section decode: {e:?}"))?;
        if decoded!=input || back!=tables || section[L_RESULT_RECORD..L_TASKS]!=record || section.len()!=HELPER_FIXED_LANES {
            return Err(format!("{id}: positive typed roundtrip/framing mismatch"));
        }
        rv.push(obj!{"case_id"=>q(id),"variant"=>if input.is_none() { "null".into() } else { q(result_variant(input)) },"lanes"=>lanes(&record)});
        rd.push(obj!{"case_id"=>q(id),"input"=>result_json(input),"decoded_record"=>result_json(decoded),
            "decoded_section_helper"=>helper_json(back.helper)?,"decoded_section_result"=>result_json(back.result),
            "decoded_section_tasks"=>array(&[]),"section_lanes"=>lanes(&section)});
    }
    Ok(obj!{"schema"=>q("sherlock-c6-3b-positive-record-actual/1"),
        "helper_vectors"=>array(&hv),"result_vectors"=>array(&rv),"helper_decoded"=>array(&hd),"result_decoded"=>array(&rd),
        "framing"=>obj!{"magic"=>number(C6_PHYS_MAGIC),"version"=>number(C6_PHYS_VERSION),"section_tag"=>number(HELPER_SECTION_TAG),
            "helper_record_offset"=>number(L_HELPER_RECORD),"helper_record_lanes"=>number(HELPER_LANES),
            "result_record_offset"=>number(L_RESULT_RECORD),"result_record_lanes"=>number(RESULT_LANES),
            "tasks_offset"=>number(L_TASKS),"fixed_section_lanes"=>number(HELPER_FIXED_LANES)},
        "caps"=>caps_json(&c),
        "provenance"=>obj!{"safe_inputs_sha256"=>q("d824cd01c0be8b9668bed718d8db671db7c5f22d9532714d1df043770a365633"),
            "core_rlib_sha256"=>q("35c7dd855629b1178d67218d7e453d6510c403ad80fa7b062652cdc9f9cd9816"),"private_expected_read"=>"false".into()},
        "scope"=>q("Synthetic positive owned carriers; record lanes and decoded values only. Caller caps are not admission or reachability. No hostile/security, full ABI/task/state/admission/native/EARS claim."),
        "missing"=>q("HelperControlC::Scalar/ScalarStaged independent tag mapping remains unbound; no Scalar helper input selected.")})
}
fn main() {
    match capture() { Ok(document)=>println!("{document}"),Err(e)=>{eprintln!("{e}");std::process::exit(1)} }
}
