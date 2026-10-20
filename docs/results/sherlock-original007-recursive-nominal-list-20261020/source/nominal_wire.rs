//! Complete output-only mappings for the additional public nominal diagnostics.
//! The unchanged historical wire.rs retains owner observations and failures.
use crate::wire::Wire;
use mori_core::sherlock_nominal_source::{DraftBudgetExceeded, DraftBudgetResource, DraftError,
    DraftParseBudget, SourceSpan};
use mori_core::sherlock_nominal_source as nominal;
use mori_core::sherlock_nominal_body_types::{SourceLocation, NominalBodyTypeError, NominalBodyTypeErrorKind};
use mori_core::sherlock_compile_artifact::nominal_body_compile::{
    NominalBodyCompileError, NominalBodyCompileFailure};
use serde_json::{json, Value};

impl Wire for SourceSpan { fn wire(&self)->Value { json!({"start":self.start,"end":self.end}) } }
impl Wire for SourceLocation { fn wire(&self)->Value {
    json!({"function_index":self.function_index,"child_path":self.child_path})
} }
impl Wire for DraftBudgetResource { fn wire(&self)->Value { json!(match self {
    Self::SourceBytes=>"SourceBytes", Self::Tokens=>"Tokens", Self::Functions=>"Functions",
    Self::ParametersPerFunction=>"ParametersPerFunction", Self::TotalParameters=>"TotalParameters",
    Self::AstOccurrences=>"AstOccurrences", Self::ParserRecursionDepth=>"ParserRecursionDepth",
    Self::StructuralAstDepth=>"StructuralAstDepth",
}) } }
impl Wire for DraftBudgetExceeded { fn wire(&self)->Value {
    json!({"resource":self.resource.wire(),"limit":self.limit,"observed":self.observed})
} }
impl Wire for DraftError { fn wire(&self)->Value { match self {
    Self::Lex(s)=>json!({"Lex":s}),
    Self::Syntax{span,expected,found}=>json!({"Syntax":{"span":span.wire(),"expected":expected,"found":found}}),
    Self::UIntLiteral{span,text}=>json!({"UIntLiteral":{"span":span.wire(),"text":text}}),
    Self::Budget(e)=>json!({"Budget":e.wire()}),
} } }
impl Wire for DraftParseBudget { fn wire(&self)->Value { json!({
    "max_source_bytes":self.max_source_bytes,"max_tokens":self.max_tokens,
    "max_functions":self.max_functions,"max_parameters_per_function":self.max_parameters_per_function,
    "max_total_parameters":self.max_total_parameters,"max_ast_occurrences":self.max_ast_occurrences,
    "max_parser_recursion_depth":self.max_parser_recursion_depth,
    "max_structural_ast_depth":self.max_structural_ast_depth}) } }
impl Wire for NominalBodyTypeErrorKind { fn wire(&self)->Value { match self {
    Self::UnknownVariable(n)=>json!({"UnknownVariable":n}),
    Self::UnknownFunction(n)=>json!({"UnknownFunction":n}),
    Self::Arity{name,expected,actual}=>json!({"Arity":{"name":name,"expected":expected,"actual":actual}}),
    Self::Expected{expected,actual}=>json!({"Expected":{"expected":expected.wire(),"actual":actual.wire()}}),
    Self::BranchTypes{yes,no}=>json!({"BranchTypes":{"yes":yes.wire(),"no":no.wire()}}),
    Self::ArmTypes{expected,actual}=>json!({"ArmTypes":{"expected":expected.wire(),"actual":actual.wire()}}),
    Self::DuplicateParameter{name,index}=>json!({"DuplicateParameter":{"name":name,"index":index}}),
    Self::DuplicatePatternBinder(n)=>json!({"DuplicatePatternBinder":n}),
    Self::EmptyMatch=>json!("EmptyMatch"), Self::UnsupportedDomain=>json!("UnsupportedDomain"),
    Self::UnsupportedExpression(s)=>json!({"UnsupportedExpression":s}),
    Self::IncompletePreparation(s)=>json!({"IncompletePreparation":s}),
    Self::Preparation(e)=>json!({"Preparation":e.wire()}),
} } }
impl Wire for NominalBodyTypeError { fn wire(&self)->Value {
    json!({"source":self.source.wire(),"kind":self.kind.wire()})
} }
impl Wire for NominalBodyCompileError { fn wire(&self)->Value { match self {
    Self::Parse(e)=>json!({"Parse":e.wire()}),
    Self::Preparation(e)=>json!({"Preparation":e.wire()}),
    Self::BodyType(e)=>json!({"BodyType":e.wire()}),
    Self::OwnerAdmission(e)=>json!({"OwnerAdmission":e.wire()}),
    Self::Genesis(e)=>json!({"Genesis":e.wire()}),
} } }
impl Wire for NominalBodyCompileFailure { fn wire(&self)->Value {
    json!({"error":self.error.wire(),"span":self.span.wire(),
        "body_check_completed":self.body_check_completed()})
} }
impl Wire for nominal::SourceOwner { fn wire(&self)->Value { match self {
    Self::Entry=>json!("Entry"), Self::Function(i)=>json!({"Function":i}),
} } }
impl Wire for nominal::NamedSite { fn wire(&self)->Value {
    json!({"name":self.name,"span":self.span.wire()})
} }
impl Wire for nominal::FieldSite { fn wire(&self)->Value {
    json!({"binder":self.binder.wire(),"type_span":self.type_span.wire()})
} }
impl Wire for nominal::ConstructorSite { fn wire(&self)->Value {
    json!({"name":self.name.wire(),"span":self.span.wire(),"fields":self.fields.wire()})
} }
impl Wire for nominal::DeclarationSite { fn wire(&self)->Value {
    json!({"name":self.name.wire(),"span":self.span.wire(),"parameters":self.parameters.wire(),
        "constructors":self.constructors.wire()})
} }
impl Wire for nominal::FunctionSite { fn wire(&self)->Value {
    json!({"name":self.name.wire(),"span":self.span.wire(),"parameters":self.parameters.wire(),
        "return_type_span":self.return_type_span.wire()})
} }
impl Wire for nominal::ExpressionOccurrence { fn wire(&self)->Value {
    json!({"owner":self.owner.wire(),"child_path":self.child_path,"span":self.span.wire(),"kind":self.kind})
} }
impl Wire for nominal::PatternOccurrence { fn wire(&self)->Value {
    json!({"owner":self.owner.wire(),"match_path":self.match_path,"arm":self.arm,
        "field_path":self.field_path,"span":self.span.wire(),"kind":self.kind})
} }
impl Wire for nominal::NominalProvenance { fn wire(&self)->Value {
    json!({"language_span":self.language_span.wire(),"program_name_span":self.program_name_span.wire(),
        "declarations":self.declarations.wire(),"functions":self.functions.wire(),
        "expressions":self.expressions.wire(),"patterns":self.patterns.wire()})
} }
