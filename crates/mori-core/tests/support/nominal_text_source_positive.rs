//! Author-owned ordinary source-level tests; not independent runtime oracles.
//! Actual public text parser and existing owner type-preparation consumer only.
use mori_core::sherlock_nominal_source::{parse_nominal_source_bounded,prepare_nominal_source_types_bounded,DraftParseBudget,SourceOwner};
use mori_core::sherlock::helper_machine::shared::{
    BoundsC,LayoutCapacitiesC,LayoutSchemaC,CorePattern,
    owner_ext_source_program::ExtDomain,
    owner_ext_t3_uses::{ExtExpr,ExtPattern,T3UseSite},
    owner_ext_type_builder::TypeExprSrc,
};
fn budget()->DraftParseBudget {
    DraftParseBudget {max_source_bytes:4096,max_tokens:1024,max_functions:8,
        max_parameters_per_function:8,max_total_parameters:32,max_ast_occurrences:256,
        max_parser_recursion_depth:64,max_structural_ast_depth:32}
}
fn layout()->LayoutSchemaC {
    LayoutSchemaC {layout_version:("nominal-text-positive".into(),"1".into()),field_id:None,widths:None,scalar_layout_id:None,
        bounds:BoundsC {code_rows:256,function_rows:16,code_child_slots:256,parameter_slots:64,
            pattern_rows:128,match_plan_rows:128,match_arm_rows:128,type_rows:64,
            constructor_rows:128,constructor_field_slots:256,max_arity:16,constant_nodes:128,
            constant_field_slots:128,data_nodes:128,data_field_slots:256,environment_cells:128,
            frames:64,helper_tasks:256,serialization_bytes:65536,max_reference_work:4096,
            max_helper_work:4096,step_cap:1},
        capacities:LayoutCapacitiesC {name_rows:256,name_bytes:4096,occurrence_path_slots:1024,value_slots:256},ext_capacities: None, boundary_capacities_present:false}
}
fn ty(t:&TypeExprSrc)->String {
    match t {
        TypeExprSrc::UInt=>"UInt".into(),TypeExprSrc::Bool=>"Bool".into(),
        TypeExprSrc::List=>"List".into(),TypeExprSrc::Tree=>"Tree".into(),
        TypeExprSrc::Named {name}=>format!("named:{name}"),
        TypeExprSrc::Param {name}=>format!("param:{name}"),
        TypeExprSrc::Apply {name,args}=>format!("apply:{name}<{}>",args.iter().map(ty).collect::<Vec<_>>().join(",")),
    }
}
fn pat(p:&ExtPattern)->String {
    match p {
        ExtPattern::Wildcard=>"wildcard".into(),
        ExtPattern::Dedicated {pattern:CorePattern::PVar {x}}=>format!("bind:{x}"),
        ExtPattern::Dedicated {..}=>"dedicated".into(),
        ExtPattern::Ctor {ty:t,ctor,fields}=>format!("{}::{ctor}({})",ty(t),fields.iter().map(pat).collect::<Vec<_>>().join(",")),
    }
}
#[test]
fn text_schedule_preserves_recursive_declarations_ordered_patterns_and_owner_types() {
    let source=include_str!("../fixtures/nominal-text-source/schedule.mori");
    let p=parse_nominal_source_bounded(source,budget()).expect("normal nominal text parses");
    assert_eq!(p.name,"OrderedSchedule");assert_eq!(p.program.types.len(),1);
    let d=&p.program.types[0];assert_eq!(d.name,"Schedule");assert!(d.parameters.is_empty());
    assert_eq!(d.ctors.iter().map(|c|c.name.as_str()).collect::<Vec<_>>(),["Done","Pay","Both"]);
    assert_eq!(d.ctors.iter().map(|c|c.fields.iter().map(|f|(f.name.as_str(),ty(&f.ty))).collect::<Vec<_>>()).collect::<Vec<_>>(),
        vec![vec![],vec![("amount","UInt".into()),("tail","named:Schedule".into())],vec![("left","named:Schedule".into()),("right","named:Schedule".into())]]);
    let f=&p.program.functions[0];assert_eq!(f.name,"choose");assert_eq!(ty(&f.parameters[0].ty),"named:Schedule");assert_eq!(ty(&f.return_type),"UInt");
    let ExtExpr::Match {scrutinee,arms}=&f.body else {panic!("source Match preserved")};
    assert!(matches!(&**scrutinee,ExtExpr::Var {name} if name=="s"));
    assert_eq!(arms.iter().map(|a|pat(&a.pattern)).collect::<Vec<_>>(),[
        "named:Schedule::Both(named:Schedule::Pay(bind:x,wildcard),wildcard)",
        "named:Schedule::Both(wildcard,bind:r)","named:Schedule::Pay(bind:x,bind:t)","named:Schedule::Done()"]);
    assert!(matches!(&arms[0].body,ExtExpr::Var {name} if name=="x"));
    assert!(matches!(&arms[1].body,ExtExpr::Call {name,args} if name=="choose"&&args.len()==1));
    let prepared=prepare_nominal_source_types_bounded(source,budget(),&layout(),ExtDomain::GENERIC_ABSENT,None)
        .unwrap_or_else(|_|panic!("normal source reaches actual owner type preparation"));
    assert_eq!(prepared.prepared.tables.types.len(),5);
    assert_eq!(prepared.prepared.tables.constructors.len(),7);
    assert_eq!(prepared.prepared.tables.ctor_fields.len(),9);
    assert_eq!(prepared.prepared.tables.decl_param_counts,[0]);
    assert_eq!(prepared.prepared.tables.decl_ctor_counts,[3]);
    assert!(prepared.prepared.t3_records.iter().any(|r|matches!(r.site,T3UseSite::ParamType {function:0,param:0})&&r.type_id.0==4));
    assert!(prepared.prepared.t3_records.iter().any(|r|matches!(r.site,T3UseSite::CtorPattern {arm:0,pattern:1,..})&&r.type_id.0==4&&r.ctor_id.is_some()));
    assert_eq!(prepared.provenance,p.provenance);
}
#[test]
fn text_mutual_forward_references_and_generic_parameter_bindings_reach_owner() {
    let source=include_str!("../fixtures/nominal-text-source/mutual-generic.mori");
    let p=parse_nominal_source_bounded(source,budget()).expect("normal mutual/generic source parses");
    assert_eq!(p.program.types.iter().map(|d|d.name.as_str()).collect::<Vec<_>>(),["Even","Odd","Box"]);
    assert_eq!(ty(&p.program.types[0].ctors[1].fields[0].ty),"named:Odd");
    assert_eq!(ty(&p.program.types[1].ctors[0].fields[0].ty),"named:Even");
    assert_eq!(p.program.types[2].parameters,["T"]);assert_eq!(ty(&p.program.types[2].ctors[0].fields[0].ty),"param:T");
    assert_eq!(ty(&p.program.functions[0].return_type),"apply:Box<named:Even>");
    assert_eq!(ty(&p.program.functions[1].parameters[0].ty),"apply:Box<named:Even>");
    let prepared=prepare_nominal_source_types_bounded(source,budget(),&layout(),ExtDomain::GENERIC_ABSENT,None)
        .unwrap_or_else(|_|panic!("mutual and generic types use existing checked owner"));
    assert_eq!(prepared.prepared.tables.types.len(),7);
    assert_eq!(prepared.prepared.tables.decl_param_counts,[0,0,1]);
    assert_eq!(prepared.prepared.tables.decl_ctor_counts,[2,1,1]);
    assert_eq!(prepared.prepared.program.functions.iter().map(|f|f.name.as_str()).collect::<Vec<_>>(),["wrap","unwrap"]);
    assert!(prepared.prepared.t3_records.iter().any(|r|matches!(r.site,T3UseSite::ReturnType {function:0})&&r.type_id.0==6));
    assert!(prepared.prepared.t3_records.iter().any(|r|matches!(r.site,T3UseSite::ParamType {function:1,param:0})&&r.type_id.0==6));
}
#[test]
fn text_nested_applications_patterns_and_complete_occurrence_paths() {
    let source=include_str!("../fixtures/nominal-text-source/nested-application.mori");
    let p=parse_nominal_source_bounded(source,budget()).expect("normal nested source parses");
    assert_eq!(ty(&p.program.functions[0].parameters[0].ty),"apply:Box<apply:Box<UInt>>");
    let ExtExpr::Match {arms,..}=&p.program.functions[0].body else {panic!("match")};
    assert_eq!(arms.iter().map(|a|pat(&a.pattern)).collect::<Vec<_>>(),[
        "apply:Box<apply:Box<UInt>>::Box(apply:Box<UInt>::Box(bind:value))","wildcard"]);
    let patterns=p.provenance.patterns.iter().filter(|s|s.owner==SourceOwner::Function(0)).collect::<Vec<_>>();
    assert_eq!(patterns.iter().map(|s|(s.arm,s.field_path.clone(),s.kind)).collect::<Vec<_>>(),[
        (0,vec![],"constructor-pattern"),(0,vec![0],"constructor-pattern"),(0,vec![0,0],"binder"),(1,vec![],"wildcard")]);
    for s in patterns {assert!(s.match_path.is_empty());assert!(!source[s.span.start as usize..s.span.end as usize].is_empty());}
    let expressions=p.provenance.expressions.iter().filter(|s|s.owner==SourceOwner::Entry).map(|s|(s.child_path.clone(),s.kind)).collect::<Vec<_>>();
    assert_eq!(expressions,[(vec![],"call"),(vec![0],"construct"),(vec![0,0],"construct"),(vec![0,0,0],"uint")]);
    let prepared=prepare_nominal_source_types_bounded(source,budget(),&layout(),ExtDomain::GENERIC_ABSENT,None)
        .unwrap_or_else(|_|panic!("nested applications reach owner"));
    assert_eq!(prepared.prepared.tables.types.len(),6);
    assert_eq!(prepared.prepared.tables.decl_param_counts,[1]);
    assert_eq!(prepared.prepared.tables.type_args.len(),2);
}
