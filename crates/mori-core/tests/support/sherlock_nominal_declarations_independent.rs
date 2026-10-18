// Independent normal declaration-only consumer. Explicit rustc target, not Cargo autodiscovery.
use mori_core::sherlock::helper_machine::shared::{BoundsC, LayoutCapacitiesC, LayoutSchemaC, GrammarDomain};
use mori_core::sherlock::helper_machine::shared::owner_ext_type_builder::{TypeExprSrc, TypeDeclSrc, CtorDeclSrc, CtorFieldSrc, CheckedTypeDeclView};
use mori_core::sherlock::helper_machine::shared::{prepare_nominal_declarations, PreparedDeclarationScope};
use mori_core::sherlock::helper_machine::shared::nominal_declaration_order::walk_declaration_type_occurrences;
use serde_json::{json, Value};
fn expression(x: &TypeExprSrc) -> Value {
    match x {
        TypeExprSrc::UInt => json!("UInt"), TypeExprSrc::Bool => json!("Bool"),
        TypeExprSrc::List => json!("List"), TypeExprSrc::Tree => json!("Tree"),
        TypeExprSrc::Named { name } => json!({"Named":{"name":name}}),
        TypeExprSrc::Param { name } => json!({"Param":{"name":name}}),
        TypeExprSrc::Apply { name, args } => json!({"Apply":{"name":name,"args":args.iter().map(expression).collect::<Vec<_>>()}}),
    }
}
#[test]
fn independent_nominal_declaration_order_mutual_references_and_fifo_tables() {
    let decls = vec![TypeDeclSrc { name: "Wrap".into(), parameters: vec!["T".into()], ctors: vec![CtorDeclSrc { name: "WrapValue".into(), fields: vec![CtorFieldSrc { name: "inner".into(), ty: TypeExprSrc::Param { name: "T".into() } }] }] },
        TypeDeclSrc { name: "Zeta".into(), parameters: vec![], ctors: vec![CtorDeclSrc { name: "ToAlpha".into(), fields: vec![CtorFieldSrc { name: "next".into(), ty: TypeExprSrc::Named { name: "Alpha".into() } }, CtorFieldSrc { name: "cargo".into(), ty: TypeExprSrc::Apply { name: "Wrap".into(), args: vec![TypeExprSrc::Bool] } }] }, CtorDeclSrc { name: "EndZeta".into(), fields: vec![] }] },
        TypeDeclSrc { name: "Alpha".into(), parameters: vec![], ctors: vec![CtorDeclSrc { name: "ToZeta".into(), fields: vec![CtorFieldSrc { name: "again".into(), ty: TypeExprSrc::Named { name: "Zeta".into() } }, CtorFieldSrc { name: "weight".into(), ty: TypeExprSrc::UInt }] }] }];
    let layout = LayoutSchemaC {
        layout_version: ("B06-independent-normal".into(), "1".into()),
        field_id: None, widths: None, scalar_layout_id: None,
        bounds: BoundsC { type_rows:32, constructor_rows:32, constructor_field_slots:64, max_arity:4, ..BoundsC::default() },
        capacities: LayoutCapacitiesC { name_rows:32, name_bytes:256, occurrence_path_slots:0, value_slots:0 },
        boundary_capacities_present:false,
    };
    let prepared = prepare_nominal_declarations(&decls, &layout, GrammarDomain::GenericNominalExtensionReserved, None)
        .unwrap_or_else(|_| panic!("normal declaration preparation returned an error"));
    let view = CheckedTypeDeclView::new_checked(GrammarDomain::GenericNominalExtensionReserved, &decls, &prepared.decl_name_ids, &layout.bounds)
        .unwrap_or_else(|_| panic!("positive prepared declaration view returned an error"));
    let mut occurrences=Vec::new();
    walk_declaration_type_occurrences(&view, |field,path,ty| {
        occurrences.push(json!({"declaration":field.declaration.0,"constructor_ordinal":field.constructor_ordinal,"field_ordinal":field.field_ordinal,"argument_path":path,"expression":expression(ty)}));
        Ok(())
    }).unwrap_or_else(|_| panic!("normal local occurrence walk returned an error"));
    let expected_occurrences:Value=serde_json::from_str(include_str!("fixtures/b06_nominal/LOCAL-TYPE-OCCURRENCES.freeze.json")).unwrap();
    assert_eq!(json!(occurrences),expected_occurrences,"complete local declaration type occurrence order");
    assert!(matches!(prepared.scope,PreparedDeclarationScope::DeclarationOnly));
    assert!(matches!(prepared.domain,GrammarDomain::GenericNominalExtensionReserved));
    let t=&prepared.tables;
    let actual=json!({
        "scope":"DeclarationOnly","domain":"GenericNominalExtensionReserved",
        "interned_names":prepared.interned_names,
        "decl_name_ids":{"name_count":prepared.decl_name_ids.name_count,"decls":prepared.decl_name_ids.decls.iter().map(|d|json!({"type_name_id":d.type_name_id,"ctor_name_ids":d.ctor_name_ids,"field_name_ids":d.field_name_ids})).collect::<Vec<_>>()},
        "tables":{
            "types":t.types.iter().map(|r|json!({"name_id":r.name_id,"decl":r.decl,"args":{"start":r.args.start,"len":r.args.len},"ctor_start":r.ctor_start.0,"ctor_len":r.ctor_len})).collect::<Vec<_>>(),
            "type_args":t.type_args.iter().map(|r|r.0).collect::<Vec<_>>(),
            "constructors":t.constructors.iter().map(|r|json!({"owning_type":r.owning_type.0,"name_id":r.name_id,"source_order":r.source_order,"field_start":r.field_start,"field_len":r.field_len})).collect::<Vec<_>>(),
            "ctor_fields":t.ctor_fields.iter().map(|r|json!({"name_id":r.name_id,"type_id":r.type_id.0})).collect::<Vec<_>>(),
            "decl_param_counts":t.decl_param_counts,"decl_ctor_counts":t.decl_ctor_counts,"max_arity":t.max_arity,"name_count":t.name_count
        }
    });
    let expected:Value=serde_json::from_str(include_str!("fixtures/b06_nominal/EXPECTED.freeze.json")).unwrap();
    assert_eq!(actual,expected,"full nominal preparation result, all rows and metadata; no sampled table cells");
}
