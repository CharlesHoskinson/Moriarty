//! Source-first R4 grammar regression expectations, not yet compiled/run.
use mori_core::sherlock_nominal_source::{parse_nominal_source,parse_nominal_source_bounded,DraftParseBudget,DraftError,DraftBudgetResource};
use mori_core::sherlock::helper_machine::shared::{CoreValue,owner_ext_t3_uses::{ExtExpr,ExtValue},owner_ext_type_builder::TypeExprSrc};
fn budget()->DraftParseBudget{DraftParseBudget{max_source_bytes:65536,max_tokens:8192,max_functions:16,max_parameters_per_function:16,max_total_parameters:64,max_ast_occurrences:4096,max_structural_ast_depth:128,max_parser_recursion_depth:512}}
const ALL_ROLES:&str=r#"language "moriarty-sherlock-nominal/1";
program profile {
 type asset<T> { None; Some(pool: T); }
 fn account(asset: asset<UInt>): UInt { match asset {
  asset<UInt>::None() => 0,
  asset<UInt>::Some(pool) => let party = pool in party
 } }
 entry account(construct asset<UInt>::Some(7));
}"#;
#[test]
fn financial_lexer_words_are_names_in_nominal_identifier_roles(){
 for parsed in [parse_nominal_source(ALL_ROLES).unwrap(),parse_nominal_source_bounded(ALL_ROLES,budget()).unwrap()]{
  assert_eq!(parsed.name,"profile");assert_eq!(parsed.program.types[0].name,"asset");
  assert_eq!(parsed.program.types[0].ctors[0].name,"None");
  assert_eq!(parsed.program.types[0].ctors[1].fields[0].name,"pool");
  assert_eq!(parsed.program.functions[0].name,"account");assert_eq!(parsed.program.functions[0].parameters[0].name,"asset");
  assert!(matches!(&parsed.program.functions[0].parameters[0].ty,TypeExprSrc::Apply{name,..} if name=="asset"));
  let ExtExpr::Match{scrutinee,arms}=&parsed.program.functions[0].body else{panic!("match preserved")};
  assert!(matches!(&**scrutinee,ExtExpr::Var{name} if name=="asset"));
  assert!(matches!(&arms[1].body,ExtExpr::Let{name,body,..} if name=="party" && matches!(&**body,ExtExpr::Var{name} if name=="party")));
  assert!(matches!(&parsed.program.entry,ExtExpr::Call{name,..} if name=="account"));
 }
}
#[test]
fn none_constructor_decl_construct_and_pattern_preserve_source_spans(){
 let source=r#"language "moriarty-sherlock-nominal/1"; program A {
 type Maybe<T> { None; Some(value: T); }
 fn choose(x: Maybe<UInt>): UInt { match x { Maybe<UInt>::None() => 0, Maybe<UInt>::Some(v) => v } }
 entry choose(construct Maybe<UInt>::None()); }"#;
 let parsed=parse_nominal_source_bounded(source,budget()).unwrap();
 let ctor=&parsed.provenance.declarations[0].constructors[0].name;
 assert_eq!(ctor.name,"None");assert_eq!(&source[ctor.span.start as usize..ctor.span.end as usize],"None");
 assert_eq!(parsed.program.types[0].ctors[0].name,"None");
 let ExtExpr::Call{args,..}=&parsed.program.entry else{panic!("call preserved")};
 assert!(matches!(&args[0],ExtExpr::Construct{ctor,fields,..} if ctor=="None" && fields.is_empty()));
 let ExtExpr::Match{arms,..}=&parsed.program.functions[0].body else{panic!("match preserved")};
 assert!(matches!(&arms[0].pattern,mori_core::sherlock::helper_machine::shared::owner_ext_t3_uses::ExtPattern::Ctor{ctor,fields,..} if ctor=="None" && fields.is_empty()));
 assert!(parsed.provenance.patterns.iter().any(|p|p.kind=="constructor-pattern" && source[p.span.start as usize..p.span.end as usize].starts_with("Maybe<UInt>::None(")));
}
#[test]
fn boolean_and_dedicated_core_syntax_keep_existing_meanings(){
 for (expression,expected) in [("true",true),("false",false)]{
  let source=format!("language \"moriarty-sherlock-nominal/1\"; program A {{ entry {expression}; }}");
  let parsed=parse_nominal_source_bounded(&source,budget()).unwrap();
  assert!(matches!(parsed.program.entry,ExtExpr::Literal{value:ExtValue::Core{value:CoreValue::Bool{b}}} if b==expected));
 }
 let parsed=parse_nominal_source_bounded("language \"moriarty-sherlock-nominal/1\"; program A { entry nil; }",budget()).unwrap();
 assert!(matches!(&parsed.program.entry,ExtExpr::Literal{value:ExtValue::Core{value:CoreValue::List{elements}}} if elements.is_empty()));
}
#[test]
fn nominal_reserved_names_stay_refused(){
 for name in ["language","program","type","construct","wildcard","fn","entry","let","in","if","else","match","UInt","Bool","List","Tree","nil","cons","leaf","branch","not"]{
  let source=format!("language \"moriarty-sherlock-nominal/1\"; program A {{ type {name} {{ C; }} entry 0; }}");
  assert!(matches!(parse_nominal_source_bounded(&source,budget()),Err(DraftError::Syntax{..})),"reserved{name}");
 }
}
#[test]
fn boolean_tokens_cannot_be_nominal_names(){
 for name in ["true","false"]{
  for body in [format!("type {name} {{ C; }} entry 0;"),format!("type A {{ {name}; }} entry 0;"),format!("fn {name}(): UInt {{ 0 }} entry 0;"),format!("fn f({name}: UInt): UInt {{ 0 }} entry 0;"),format!("entry let {name} = 0 in 0;")]{
   let source=format!("language \"moriarty-sherlock-nominal/1\"; program A {{ {body} }}");
   assert!(matches!(parse_nominal_source_bounded(&source,budget()),Err(DraftError::Syntax{..})),"boolean name{name}");
  }
 }
}
#[test]
fn punctuation_and_numeric_tokens_are_not_nominal_names(){
 for name in ["1","+",";","("]{
  let source=format!("language \"moriarty-sherlock-nominal/1\"; program A {{ type X {{ {name}; }} entry 0; }}");
  assert!(matches!(parse_nominal_source_bounded(&source,budget()),Err(DraftError::Syntax{..})));
 }
}
#[test]
fn caller_function_budget_still_refuses_contextual_names(){
 let mut b=budget();b.max_functions=0;
 assert!(matches!(parse_nominal_source_bounded(ALL_ROLES,b),Err(DraftError::Budget(e)) if e.resource==DraftBudgetResource::Functions && e.limit==0 && e.observed==1));
}
