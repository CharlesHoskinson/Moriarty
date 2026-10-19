//! SOURCE_ONLY / UNRUN. Positive public ONE-builder preparation consumer.
//! Expected values/factories are independently frozen before execution.
//! No evaluator, admission, typed runtime or C/P lowering is exercised.
use mori_core::sherlock::helper_machine::shared::{self, PreparedExtProgramSource};
use shared::owner_ext_t3_uses::{BodyOwner, T3Record, T3UseSite};
use serde_json::{Value, json};

fn owner(owner: BodyOwner) -> Value {
    match owner {
        BodyOwner::Entry => json!("Entry"),
        BodyOwner::Function(index) => json!({"Function": index}),
    }
}
fn site(site: T3UseSite) -> Value {
    match site {
        T3UseSite::ParamType { function, param } => json!({"kind":"ParamType", "function":function, "param":param}),
        T3UseSite::ReturnType { function } => json!({"kind":"ReturnType", "function":function}),
        T3UseSite::Construct { owner: body, expr } => json!({"kind":"Construct", "owner":owner(body), "expr":expr}),
        T3UseSite::LiteralCtor { owner: body, expr, value } => json!({"kind":"LiteralCtor", "owner":owner(body), "expr":expr, "value":value}),
        T3UseSite::CtorPattern { owner: body, expr, arm, pattern } => json!({"kind":"CtorPattern", "owner":owner(body), "expr":expr, "arm":arm, "pattern":pattern}),
    }
}
fn records(records: &[T3Record]) -> Value {
    json!(records.iter().map(|record| json!({
        "site": site(record.site), "type_id": record.type_id.0,
        "ctor_id": record.ctor_id.map(|id| id.0),
    })).collect::<Vec<_>>())
}
fn counts(records: &[T3Record]) -> Value {
    let mut counts = [0_usize; 5];
    for record in records {
        let index = match record.site {
            T3UseSite::ParamType { .. } => 0,
            T3UseSite::ReturnType { .. } => 1,
            T3UseSite::Construct { .. } => 2,
            T3UseSite::LiteralCtor { .. } => 3,
            T3UseSite::CtorPattern { .. } => 4,
        };
        counts[index] += 1;
    }
    json!({"ParamType":counts[0], "ReturnType":counts[1], "Construct":counts[2],
        "LiteralCtor":counts[3], "CtorPattern":counts[4], "total":records.len()})
}
fn prepared_tables(prepared: &PreparedExtProgramSource) -> Value {
    let tables = &prepared.tables;
    json!({
        "types": tables.types.iter().map(|row| json!({
            "name_id":row.name_id, "decl":row.decl,
            "args":{"start":row.args.start,"len":row.args.len},
            "ctor_start":row.ctor_start.0,"ctor_len":row.ctor_len,
        })).collect::<Vec<_>>(),
        "type_args": tables.type_args.iter().map(|id| id.0).collect::<Vec<_>>(),
        "constructors": tables.constructors.iter().map(|row| json!({
            "owning_type":row.owning_type.0,"name_id":row.name_id,
            "source_order":row.source_order,"field_start":row.field_start,"field_len":row.field_len,
        })).collect::<Vec<_>>(),
        "ctor_fields": tables.ctor_fields.iter().map(|row| json!({
            "name_id":row.name_id,"type_id":row.type_id.0,
        })).collect::<Vec<_>>(),
        "decl_param_counts":tables.decl_param_counts,
        "decl_ctor_counts":tables.decl_ctor_counts,
        "max_arity":tables.max_arity,"name_count":tables.name_count,
    })
}
fn declaration_names(prepared: &PreparedExtProgramSource) -> Value {
    let names = &prepared.decl_name_ids;
    json!({
        "owner_verified":names.is_owner_verified(),"name_count":names.name_count,
        "decls":names.decls.iter().map(|row| json!({
            "type_name_id":row.type_name_id,"ctor_name_ids":row.ctor_name_ids,
            "field_name_ids":row.field_name_ids,
        })).collect::<Vec<_>>(),
    })
}

use shared::owner_ext_source_program::{ExtDomain, ExtProgram};
use shared::owner_ext_t3_uses::{ExtExpr, ExtPattern, ExtValue};
use shared::owner_ext_type_builder::TypeExprSrc;
use shared::{BinaryOp, CorePattern, CoreValue, UnaryOp};

// Test-only literal JSON views of the small frozen source trees. This is not
// an interpreter, builder, automatic recursive Debug/Clone, or source hash.
fn ty(ty: &TypeExprSrc) -> Value {
    match ty {
        TypeExprSrc::UInt => json!("UInt"),
        TypeExprSrc::Bool => json!("Bool"),
        TypeExprSrc::List => json!("List"),
        TypeExprSrc::Tree => json!("Tree"),
        TypeExprSrc::Named { name } => json!({"Named":name}),
        TypeExprSrc::Param { name } => json!({"Param":name}),
        TypeExprSrc::Apply { name, args } => json!({"Apply":{
            "name":name,"args":args.iter().map(self::ty).collect::<Vec<_>>()}}),
    }
}
fn core_value(value: &CoreValue) -> Value {
    match value {
        CoreValue::UInt { n: shared::i128_class::KIntC::InRange(n) } => {
            json!({"UInt":{"InRange":n.to_string()}})
        }
        _ => panic!("frozen positive corpus selects UInt InRange source literals only"),
    }
}
fn value(value: &ExtValue) -> Value {
    match value {
        ExtValue::Core { value: core } => json!({"Core":core_value(core)}),
        ExtValue::Constructor { ty: source_type, ctor, fields } => json!({"Constructor":{
            "ty":ty(source_type),"ctor":ctor,"fields":fields.iter().map(self::value).collect::<Vec<_>>()}}),
    }
}
fn pattern(pattern: &ExtPattern) -> Value {
    match pattern {
        ExtPattern::Wildcard => json!("Wildcard"),
        ExtPattern::Ctor { ty: source_type, ctor, fields } => json!({"Ctor":{
            "ty":ty(source_type),"ctor":ctor,"fields":fields.iter().map(self::pattern).collect::<Vec<_>>()}}),
        ExtPattern::Dedicated { pattern: core } => {
            let core = match core {
                CorePattern::PVar { x } => json!({"PVar":{"x":x}}),
                CorePattern::PNil => json!("PNil"),
                CorePattern::PCons { x, y } => json!({"PCons":{"x":x,"y":y}}),
                CorePattern::PLeaf { x } => json!({"PLeaf":{"x":x}}),
                CorePattern::PBranch { x, y } => json!({"PBranch":{"x":x,"y":y}}),
            };
            json!({"Dedicated":core})
        }
    }
}
fn binary(op: BinaryOp) -> &'static str {
    match op {
        BinaryOp::Add => "Add", BinaryOp::Sub => "Sub", BinaryOp::Mul => "Mul",
        BinaryOp::Eq => "Eq", BinaryOp::Lt => "Lt", BinaryOp::Cons => "Cons", BinaryOp::Branch => "Branch",
    }
}
fn expr(expr: &ExtExpr) -> Value {
    match expr {
        ExtExpr::Literal { value: source_value } => json!({"Literal":value(source_value)}),
        ExtExpr::Var { name } => json!({"Var":{"name":name}}),
        ExtExpr::Let { name, value, body } => json!({"Let":{
            "name":name,"value":self::expr(value),"body":self::expr(body)}}),
        ExtExpr::Binary { op, left, right } => json!({"Binary":{
            "op":binary(*op),"left":self::expr(left),"right":self::expr(right)}}),
        ExtExpr::Unary { op, operand } => json!({"Unary":{
            "op":match op { UnaryOp::Leaf => "Leaf", UnaryOp::Not => "Not" },"operand":self::expr(operand)}}),
        ExtExpr::If { cond, then_expr, else_expr } => json!({"If":{
            "cond":self::expr(cond),"then_expr":self::expr(then_expr),"else_expr":self::expr(else_expr)}}),
        ExtExpr::Call { name, args } => json!({"Call":{
            "name":name,"args":args.iter().map(self::expr).collect::<Vec<_>>()}}),
        ExtExpr::Match { scrutinee, arms } => json!({"Match":{
            "scrutinee":self::expr(scrutinee),"arms":arms.iter().map(|arm| json!({
                "pattern":pattern(&arm.pattern),"body":self::expr(&arm.body)})).collect::<Vec<_>>()}}),
        ExtExpr::Construct { ty: source_type, ctor, fields } => json!({"Construct":{
            "ty":ty(source_type),"ctor":ctor,"fields":fields.iter().map(self::expr).collect::<Vec<_>>()}}),
        ExtExpr::StateRead { .. } | ExtExpr::LiabilityPresent { .. } |
        ExtExpr::LiabilityAmount { .. } | ExtExpr::Fin { .. } | ExtExpr::Await { .. } => {
            panic!("frozen GenericNominal/FinanceAbsent positive corpus has no finance source forms")
        }
    }
}
fn source(program: &ExtProgram) -> Value {
    assert!(program.assets.is_empty(), "complete frozen source has no assets");
    assert!(program.states.is_empty(), "complete frozen source has no states");
    assert!(program.inputs.is_empty(), "complete frozen source has no finance inputs");
    json!({
        "types":program.types.iter().map(|decl| json!({
            "name":decl.name,"parameters":decl.parameters,
            "ctors":decl.ctors.iter().map(|ctor| json!({
                "name":ctor.name,"fields":ctor.fields.iter().map(|field| json!({
                    "name":field.name,"ty":ty(&field.ty)})).collect::<Vec<_>>()
            })).collect::<Vec<_>>()
        })).collect::<Vec<_>>(),
        "assets":[],"states":[],"inputs":[],
        "functions":program.functions.iter().map(|function| json!({
            "name":function.name,"parameters":function.parameters.iter().map(|param| json!({
                "name":param.name,"ty":ty(&param.ty)})).collect::<Vec<_>>(),
            "return_type":ty(&function.return_type),"body":expr(&function.body),
        })).collect::<Vec<_>>(),
        "entry":expr(&program.entry),
    })
}

fn usize_cell(value: &Value, key: &str) -> usize {
    usize::try_from(value[key].as_u64().expect("frozen fixture natural")).expect("fixture usize")
}
fn u128_cell(value: &Value, key: &str) -> u128 {
    u128::from(value[key].as_u64().expect("small frozen fixture natural"))
}
fn layout(input: &Value) -> shared::LayoutSchemaC {
    let b = &input["bounds"];
    let c = &input["capacities"];
    // Every22bound and4capacity comes from the frozen fixture, no defaults.
    shared::LayoutSchemaC {
        layout_version: (input["layout_version"][0].as_str().unwrap().to_owned(),
            input["layout_version"][1].as_str().unwrap().to_owned()),
        field_id: None, widths: None, scalar_layout_id: None,
        bounds: shared::BoundsC {
            code_rows:usize_cell(b,"code_rows"),function_rows:usize_cell(b,"function_rows"),
            code_child_slots:usize_cell(b,"code_child_slots"),parameter_slots:usize_cell(b,"parameter_slots"),
            pattern_rows:usize_cell(b,"pattern_rows"),match_plan_rows:usize_cell(b,"match_plan_rows"),
            match_arm_rows:usize_cell(b,"match_arm_rows"),type_rows:usize_cell(b,"type_rows"),
            constructor_rows:usize_cell(b,"constructor_rows"),constructor_field_slots:usize_cell(b,"constructor_field_slots"),
            max_arity:usize_cell(b,"max_arity"),constant_nodes:usize_cell(b,"constant_nodes"),
            constant_field_slots:usize_cell(b,"constant_field_slots"),data_nodes:usize_cell(b,"data_nodes"),
            data_field_slots:usize_cell(b,"data_field_slots"),environment_cells:usize_cell(b,"environment_cells"),
            frames:usize_cell(b,"frames"),helper_tasks:usize_cell(b,"helper_tasks"),
            serialization_bytes:usize_cell(b,"serialization_bytes"),max_reference_work:u128_cell(b,"max_reference_work"),
            max_helper_work:u128_cell(b,"max_helper_work"),step_cap:u128_cell(b,"step_cap"),
        },
        capacities:shared::LayoutCapacitiesC {
            name_rows:usize_cell(c,"name_rows"),name_bytes:usize_cell(c,"name_bytes"),
            occurrence_path_slots:usize_cell(c,"occurrence_path_slots"),value_slots:usize_cell(c,"value_slots"),
        },
        ext_capacities: None, boundary_capacities_present:input["boundary_capacities_present"].as_bool().unwrap(),
    }
}

const LAYOUT: &str = include_str!("../fixtures/generic-adt-one-builder/FIXTURE-LAYOUT-AND-DOMAIN.json");
pub(super) fn prepare_case(program: ExtProgram, text: &str) {
    let expected: Value = serde_json::from_str(text).expect("complete independently frozen expected JSON");
    let config: Value = serde_json::from_str(LAYOUT).expect("frozen explicit fixture layout");
    assert!(config["field_id"].is_null());
    assert!(config["widths"].is_null());
    assert!(config["scalar_layout_id"].is_null());
    assert_eq!(config["domain"], json!({
        "grammar_domain":"GenericNominalExtensionReserved","finance":"Absent"}));
    assert!(config["locator"].is_null());
    let prepared = match shared::prepare_ext_program_types(
        program, &layout(&config), ExtDomain::GENERIC_ABSENT, None,
    ) {
        Ok(prepared) => prepared,
        // This error type has no Debug bound. No guard/refusal case is selected.
        Err(_) => panic!("fixed independently frozen positive source did not prepare"),
    };
    let PreparedExtProgramSource {
        program, domain, interned_names, decl_name_ids: _, tables, t3_records,
    } = &prepared;
    assert_eq!(*domain, ExtDomain::GENERIC_ABSENT);
    assert_eq!(json!(interned_names), expected["interned_names"], "complete ordered global pool");
    assert_eq!(json!(interned_names.len()), expected["name_count"]);
    assert_eq!(json!(interned_names.iter().map(|name| name.len()).sum::<usize>()),
        expected["name_utf8_bytes"], "UTF8 bytes, no inferred name rows");
    assert_eq!(declaration_names(&prepared), expected["decl_name_ids"],
        "every declaration/ctor/field NameId and actual owner verification");
    assert_eq!(prepared_tables(&prepared), expected["tables"], "complete tables, no leaf sampling");
    assert_eq!(json!({"types":tables.types.len(),"type_args":tables.type_args.len(),
        "constructors":tables.constructors.len(),"ctor_fields":tables.ctor_fields.len()}),
        expected["table_lengths"]);
    assert_eq!(counts(t3_records), expected["t3_counts"], "every public T3 site category");
    for record in t3_records {
        let constructor_site = matches!(record.site, T3UseSite::Construct { .. }
            | T3UseSite::LiteralCtor { .. } | T3UseSite::CtorPattern { .. });
        assert_eq!(record.ctor_id.is_some(), constructor_site);
    }
    compare_record_order(&records(t3_records), &expected);
    assert_eq!(source(program), expected["source_projection"],
        "complete returned owned source, signatures, AST/pattern/value fields and vector order");
}

fn sorted_records(records: &Value) -> Vec<String> {
    let mut full = records.as_array().expect("complete record array").iter()
        .map(|record| serde_json::to_string(record).expect("complete flat record JSON"))
        .collect::<Vec<_>>();
    full.sort();
    full
}
fn compare_record_order(actual: &Value, expected: &Value) {
    match expected["record_order"].as_str().expect("frozen record-order contract") {
        "exact" => assert_eq!(actual, &expected["t3_records"], "every T3 record in exact frozen order"),
        "PD1_UNADOPTED_COMPARE_EXACT_RECORD_MULTISET_AND_ALLOWED_SEQUENCES" => {
            assert_eq!(expected["case"], json!("original_ordered_match"));
            assert_eq!(sorted_records(actual), sorted_records(&expected["t3_records"]),
                "complete site/type/ctor multiset including multiplicity");
            let allowed = expected["allowed_record_sequences"].as_array().expect("two frozen full sequences");
            assert_eq!(allowed.len(), 2);
            for sequence in allowed {
                assert_eq!(sorted_records(sequence), sorted_records(&expected["t3_records"]));
            }
            assert!(allowed.iter().any(|sequence| sequence == actual),
                "actual entire sequence must be one of the two frozen orders; no PD1 adoption");
        }
        _ => panic!("unrecognized frozen positive ordering contract"),
    }
}
