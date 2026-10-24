//! Pre-implementation source-derived public-boundary expectations. ALL UNRUN.
//! Normal external consumer of the full mori-core rlib; no private owner/mock.
use mori_core::sherlock_compile_artifact::ext_compile::ExtCompileConfig;
use mori_core::sherlock_compile_artifact::nominal_body_compile::*;
use mori_core::sherlock_nominal_body_types::{NominalBodyTypeErrorKind as Kind, TypeIdC};
use mori_core::sherlock_nominal_source::{DraftParseBudget, DraftError};
use mori_core::sherlock::helper_machine::shared::{
    AdmissionReason, BoundsC, CapacityKind, CounterWidthField, ExtCapacitiesC,
    LayoutCapacitiesC, LayoutSchemaC, NeededNatC, RunIntC, Tooling, WidthTable,
};
use mori_core::sherlock::helper_machine::shared::owner_ext_source_program::ExtDomain;
use mori_core::sherlock::helper_machine::shared::owner_ext_type_builder::{ExtTypeBuildError, ExtTypeRejectReason, ExtTypeCapKind};

fn config() -> ExtCompileConfig {
    // Inspected existing generic public compiler configuration, explicitly given.
    let bounds = BoundsC { code_rows:256, function_rows:8, code_child_slots:256,
        parameter_slots:16, pattern_rows:128, match_plan_rows:64, match_arm_rows:32,
        type_rows:64, constructor_rows:64, constructor_field_slots:128, max_arity:4,
        constant_nodes:128, constant_field_slots:128, data_nodes:64, data_field_slots:128,
        environment_cells:64, frames:64, helper_tasks:64, serialization_bytes:1048576,
        max_reference_work:100000, max_helper_work:100000, step_cap:1 };
    ExtCompileConfig { bounds, layout:LayoutSchemaC {
        layout_version:("independent-public-generic-runtime".into(),"1".into()),
        field_id:None, widths:None, scalar_layout_id:None, bounds,
        capacities:LayoutCapacitiesC { name_rows:64, name_bytes:1024,
            occurrence_path_slots:1024, value_slots:4096 },
        ext_capacities:Some(ExtCapacitiesC { plan_case_slots:256, field_path_slots:512,
            pattern_child_slots:256, ..ExtCapacitiesC::all_zero() }),
        boundary_capacities_present:false }, domain:ExtDomain::GENERIC_ABSENT,
        fuel:RunIntC::Nat(4096), stack:RunIntC::Nat(64) }
}
fn budget() -> DraftParseBudget {
    DraftParseBudget { max_source_bytes:32768, max_tokens:8192, max_functions:8,
        max_parameters_per_function:16, max_total_parameters:16, max_ast_occurrences:512,
        max_parser_recursion_depth:128, max_structural_ast_depth:64 }
}
fn text(body: &str) -> String {
    format!("language \"moriarty-sherlock-nominal/1\"; program Check {{ {body} }}")
}
fn compile(body: &str) -> Result<NominalBodyCompileArtifact, NominalBodyCompileFailure> {
    compile_nominal_source_host_bounded(&text(body),budget(),config(),None)
}
fn fail(body: &str) -> NominalBodyCompileFailure {
    compile(body).err().expect("source-derived refusal")
}
fn body_error(body: &str, function: Option<usize>, path: &[usize], snippet: &str) -> Kind {
    let source=text(body);
    let failure=compile_nominal_source_host_bounded(&source,budget(),config(),None)
        .err().expect("source-derived body refusal");
    assert!(!failure.body_check_completed());
    let span=failure.span.expect("precise body site");
    assert_eq!(&source[span.start as usize..span.end as usize],snippet);
    match failure.error {
        NominalBodyCompileError::BodyType(error) => {
            assert_eq!(error.source.function_index,function);
            assert_eq!(error.source.child_path,path); error.kind
        }
        other => panic!("wrong stage: {other:?}"),
    }
}
#[test]
fn real_generic_recursive_source_reports_completed_judgments_and_retains_owner() {
    let artifact=compile("type Even { Zero; Step(pred: Odd); } type Odd { Step(pred: Even); } type Box<T> { Wrap(item: T); } fn wrap(x: Even): Box<Even> { construct Box<Even>::Wrap(x) } entry wrap(construct Even::Zero());").unwrap();
    let report=artifact.report().unwrap_or_else(|_|panic!("actual owner pair observation"));
    assert_eq!(report.source_profile,NOMINAL_BODY_TYPED_PROFILE);
    assert_eq!(report.body_check.status,"completed");
    assert_eq!(report.body_check.judgments,ADOPTED_NOMINAL_JUDGMENTS);
    assert!(!report.owner.evaluation_performed);
    assert_eq!(artifact.name(),"Check");
    assert_eq!(artifact.owner().configuration().bounds.step_cap,1);
}
#[test]
fn mutual_calls_are_checked_by_signatures_without_running() {
    compile("fn even(n: UInt): Bool { odd(n) } fn odd(n: UInt): Bool { even(n) } entry even(1);").unwrap();
}
#[test]
fn unused_function_return_error_uses_function_body_not_entry_ordinal() {
    assert!(matches!(body_error("fn unused(n: UInt): Bool { n } entry true;",Some(0),&[],"n"),Kind::Expected {expected:TypeIdC::Bool,actual:TypeIdC::UInt}));
}
#[test]
fn duplicate_parameter_maps_second_binder_even_when_unused() {
    assert!(matches!(body_error("fn unused(x: UInt, x: UInt): UInt { x } entry 0;",Some(0),&[],"x"),Kind::DuplicateParameter {name,index:1} if name=="x"));
}
#[test]
fn later_same_named_declaration_is_checked() {
    assert!(matches!(body_error("fn f(x: UInt): UInt { x } fn f(y: UInt,y: UInt): UInt { y } entry f(1);",Some(1),&[],"y"),Kind::DuplicateParameter {name,index:1} if name=="y"));
}
#[test]
fn eq_any_accepts_distinct_individually_typed_operands() { compile("entry 1 == true;").unwrap(); }
#[test]
fn eq_any_still_checks_operand() {
    assert!(matches!(body_error("entry (true + 1) == 0;",None,&[0,0],"true"),Kind::Expected {expected:TypeIdC::UInt,actual:TypeIdC::Bool}));
}
#[test]
fn eq_result_is_bool_and_parentheses_do_not_add_path() {
    assert!(matches!(body_error("entry (1 == true) + 1;",None,&[0],"(1 == true)"),Kind::Expected {expected:TypeIdC::UInt,actual:TypeIdC::Bool}));
}
#[test]
fn duplicate_unplanned_arm_is_body_error_at_arm_body() {
    assert!(matches!(body_error("entry match cons(1,nil) { cons(x,x) => 0, fallback => 1 };",None,&[1],"0"),Kind::DuplicatePatternBinder(name) if name=="x"));
}
#[test]
fn wildcard_sibling_keeps_same_body_rejection() {
    assert!(matches!(body_error("entry match cons(1,nil) { cons(x,x) => 0, wildcard => 1 };",None,&[1],"0"),Kind::DuplicatePatternBinder(name) if name=="x"));
}
#[test]
fn qualified_pattern_keeps_same_body_rejection() {
    assert!(matches!(body_error("entry match cons(1,nil) { List::Cons(x,x) => 0, fallback => 1 };",None,&[1],"0"),Kind::DuplicatePatternBinder(name) if name=="x"));
}
#[test]
fn unselected_arm_is_still_checked() {
    assert!(matches!(body_error("entry match nil { nil => 0, cons(x,x) => 1 };",None,&[2],"1"),Kind::DuplicatePatternBinder(name) if name=="x"));
}
#[test]
fn separate_arms_and_outer_shadowing_allow_reused_names() {
    compile("entry let x = 1 in match cons(2,nil) { cons(x,t) => x, nil => x };").unwrap();
    compile("entry match cons(1,nil) { cons(x,y) => x, cons(y,x) => y };").unwrap();
}
#[test]
fn parse_failure_preserves_original_payload() {
    let failure=compile_nominal_source_host_bounded("",budget(),config(),None).err().unwrap();
    assert!(!failure.body_check_completed()); assert!(failure.span.is_none());
    assert!(matches!(failure.error,NominalBodyCompileError::Parse(DraftError::Syntax {expected:"language",..})));
}
#[test]
fn first_preparation_failure_is_distinct_from_body_and_owner() {
    let failure=fail("entry construct List::Cons(1,nil);");
    assert!(!failure.body_check_completed()); assert!(failure.span.is_none());
    assert!(matches!(failure.error,NominalBodyCompileError::Preparation(ExtTypeBuildError::ExtAdmissionRejected {reason:ExtTypeRejectReason::BuiltinConstruct,..})));
}
#[test]
fn owner_v3_remains_later_than_successful_body_typing() {
    let failure=fail("fn f(x: UInt): UInt { x } fn f(y: UInt): UInt { y } entry f(1);");
    assert!(failure.body_check_completed()); assert!(failure.span.is_none());
    assert!(matches!(failure.error,NominalBodyCompileError::OwnerAdmission(ExtTypeBuildError::CoreTooling(Tooling::AdmissionRejected {reason:AdmissionReason::ProgramViewMalformed,detail,..})) if detail=="V3"));
}
#[test]
fn genesis_refusal_preserves_actual_owner_fuel_error() {
    let mut cfg=config(); cfg.fuel=RunIntC::Nat(100001);
    let failure=compile_nominal_source_host_bounded(&text("entry 0;"),budget(),cfg,None).err().unwrap();
    assert!(failure.body_check_completed()); assert!(failure.span.is_none());
    assert!(matches!(failure.error,NominalBodyCompileError::Genesis(Tooling::CapacityNotAdmitted {kind:CapacityKind::MaxReferenceWork,needed:NeededNatC::Exact(100001),limit:100000,..})));
}
#[test]
fn body_failure_precedes_zero_step_cap_owner_guard() {
    let mut cfg=config(); cfg.bounds.step_cap=0; cfg.layout.bounds.step_cap=0;
    let failure=compile_nominal_source_host_bounded(&text("entry true + 1;"),budget(),cfg,None).err().unwrap();
    assert!(matches!(failure.error,NominalBodyCompileError::BodyType(error) if matches!(error.kind,Kind::Expected {expected:TypeIdC::UInt,actual:TypeIdC::Bool}) && error.source.child_path==vec![0]));
}
#[test]
fn welltyped_body_reaches_actual_zero_step_cap_guard() {
    let mut cfg=config(); cfg.bounds.step_cap=0; cfg.layout.bounds.step_cap=0;
    let failure=compile_nominal_source_host_bounded(&text("entry 0;"),budget(),cfg,None).err().unwrap();
    assert!(failure.body_check_completed());
    assert!(matches!(failure.error,NominalBodyCompileError::OwnerAdmission(ExtTypeBuildError::CoreTooling(Tooling::CapacityNotAdmitted {kind:CapacityKind::StepCap,needed:NeededNatC::Exact(1),limit:0,..}))));
}
#[test]
fn mismatched_layout_is_not_replaced_before_owner_guard() {
    let mut cfg=config(); cfg.layout.bounds.step_cap=2;
    let failure=compile_nominal_source_host_bounded(&text("entry 0;"),budget(),cfg,None).err().unwrap();
    assert!(failure.body_check_completed());
    assert!(matches!(failure.error,NominalBodyCompileError::OwnerAdmission(ExtTypeBuildError::CoreTooling(Tooling::AdmissionRejected {reason:AdmissionReason::LayoutBoundsMismatch,detail,..})) if detail=="layout.bounds != bounds"));
}
#[test]
fn supplied_widths_remain_actual_owner_refusal_after_body_success() {
    let mut cfg=config(); cfg.layout.widths=Some(WidthTable::new_validated(vec![],vec![],vec![]).unwrap());
    let failure=compile_nominal_source_host_bounded(&text("entry 0;"),budget(),cfg,None).err().unwrap();
    assert!(failure.body_check_completed());
    assert!(matches!(failure.error,NominalBodyCompileError::OwnerAdmission(ExtTypeBuildError::CoreTooling(Tooling::CapacityNotAdmitted {kind:CapacityKind::CounterWidth {field:CounterWidthField::WorkCounter},needed:NeededNatC::Exact(17),limit:0,..}))));
}

// Span supplemental assertions added after the wrapper source; unlike the first
// 21 boundary functions these do not claim pre-production freeze provenance.
#[test]
fn duplicate_parameter_span_is_exactly_second_parameter_binder() {
    let source=text("fn f(x: UInt, x: UInt): UInt { x } entry 0;");
    let failure=compile_nominal_source_host_bounded(&source,budget(),config(),None).err().unwrap();
    let span=failure.span.unwrap();
    let expected=source.find(", x:").unwrap()+2;
    assert_eq!((span.start as usize,span.end as usize),(expected,expected+1));
}
#[test]
fn literal_value_field_suffix_maps_owning_expression() {
    use mori_core::sherlock_nominal_source::parse_nominal_source;
    use mori_core::sherlock_nominal_body_types::{NominalBodyTypeError,SourceLocation};
    use mori_core::sherlock::helper_machine::shared::owner_ext_t3_uses::{ExtExpr,ExtValue};
    use mori_core::sherlock::helper_machine::shared::owner_ext_type_builder::TypeExprSrc;
    use mori_core::sherlock::helper_machine::shared::CoreValue;
    let source=text("entry 0;");
    let mut parsed=parse_nominal_source(&source).unwrap();
    parsed.program.entry=ExtExpr::Literal {value:ExtValue::Constructor {
        ty:TypeExprSrc::Named {name:"Outer".into()},ctor:"Hold".into(),
        fields:vec![ExtValue::Constructor {ty:TypeExprSrc::Named {name:"Inner".into()},
            ctor:"Wrap".into(),fields:vec![ExtValue::Core {value:CoreValue::Bool {b:true}}]}]}};
    let error=NominalBodyTypeError {source:SourceLocation {function_index:None,child_path:vec![0,0]},
        kind:Kind::Expected {expected:TypeIdC::UInt,actual:TypeIdC::Bool}};
    let span=nominal_body_error_span(&source,&parsed.program,&parsed.provenance,&error).unwrap();
    assert_eq!(&source[span.start as usize..span.end as usize],"0");
    let invalid=NominalBodyTypeError {source:SourceLocation {function_index:None,child_path:vec![0,1]},kind:error.kind};
    assert!(nominal_body_error_span(&source,&parsed.program,&parsed.provenance,&invalid).is_none());
}
#[test]
fn missing_owner_path_or_parameter_site_never_fabricates_span() {
    use mori_core::sherlock_nominal_source::parse_nominal_source;
    use mori_core::sherlock_nominal_body_types::{NominalBodyTypeError,SourceLocation};
    let source=text("entry 0;"); let parsed=parse_nominal_source(&source).unwrap();
    for (function_index,child_path,kind) in [
        (Some(9),vec![],Kind::UnknownVariable("missing".into())),
        (None,vec![2],Kind::UnknownVariable("missing".into())),
        (Some(0),vec![],Kind::DuplicateParameter {name:"x".into(),index:1}),
        (None,vec![],Kind::UnsupportedDomain),
        (None,vec![],Kind::IncompletePreparation("missing record")),
    ] {
        let error=NominalBodyTypeError {source:SourceLocation {function_index,child_path},kind};
        assert!(nominal_body_error_span(&source,&parsed.program,&parsed.provenance,&error).is_none());
    }
}
#[test]
fn nominal_instantiation_mismatch_is_body_error_with_exact_field_site() {
    assert!(matches!(body_error("type Box<T> { Wrap(item: T); } entry construct Box<UInt>::Wrap(true);",None,&[0],"true"),Kind::Expected {expected:TypeIdC::UInt,actual:TypeIdC::Bool}));
}
#[test]
fn recursive_call_argument_mismatch_is_checked_without_entry_execution() {
    assert!(matches!(body_error("fn f(n: UInt): UInt { f(false) } entry 0;",Some(0),&[0],"false"),Kind::Expected {expected:TypeIdC::UInt,actual:TypeIdC::Bool}));
}
#[test]
fn first_t3_uses_supplied_layout_even_when_config_bounds_differ() {
    let mut cfg=config(); cfg.layout.bounds.type_rows=0;
    let failure=compile_nominal_source_host_bounded(&text("type Box { Wrap; } entry construct Box::Wrap();"),budget(),cfg,None).err().unwrap();
    assert!(!failure.body_check_completed());
    assert!(matches!(failure.error,NominalBodyCompileError::Preparation(ExtTypeBuildError::ExtCapacityNotAdmitted {kind:ExtTypeCapKind::TypeRows,needed:NeededNatC::Exact(5),limit:0,locator:None})));
}
