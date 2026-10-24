//! Named caller-owned nominal AST constants for the existing body-typed route.
//!
//! Source still uses the real bounded parser/lexer. This typed Rust interface
//! does not parse an alternate string/JSON language or relax either wire limit.
//! Literal constructor/type correctness remains with actual first T3/body/owner.
use super::helper_machine::shared::{CoreValue,i128_class::KIntC};
use super::helper_machine::shared::owner_ext_source_program::ExtProgram;
use super::helper_machine::shared::owner_ext_t3_uses::{ExtExpr,ExtValue};
use super::helper_machine::shared::owner_ext_type_builder::TypeExprSrc;
use super::nominal_external_inputs::{BoundArgumentProvenance,BoundNominalEntry,
    NominalBindingProvenance,NominalInputError,NominalInputLimits};

pub const NOMINAL_AST_INPUTS_FORMAT:&str="sherlock-nominal-ast-inputs/1";
pub struct NominalAstArgument {pub name:String,pub value:ExtValue}
pub struct NominalAstBindingRequest {pub function_name:String,pub arguments:Vec<NominalAstArgument>}
type Result<T>=std::result::Result<T,NominalInputError>;
fn add(a:usize,b:usize)->Result<usize>{a.checked_add(b).ok_or(NominalInputError::CountOverflow)}

/// Iterative exact applied-type rendering for binding provenance only.
fn type_text(root:&TypeExprSrc)->String {
    enum F<'a>{Ty(&'a TypeExprSrc),Text(&'static str)}
    let mut work=vec![F::Ty(root)];let mut text=String::new();
    while let Some(f)=work.pop(){match f{
        F::Text(s)=>text.push_str(s),
        F::Ty(ty)=>match ty{
            TypeExprSrc::UInt=>text.push_str("UInt"),TypeExprSrc::Bool=>text.push_str("Bool"),
            TypeExprSrc::List=>text.push_str("List"),TypeExprSrc::Tree=>text.push_str("Tree"),
            TypeExprSrc::Named{name}|TypeExprSrc::Param{name}=>text.push_str(name),
            TypeExprSrc::Apply{name,args}=>{
                text.push_str(name);text.push('<');work.push(F::Text(">"));
                for (i,arg) in args.iter().enumerate().rev(){work.push(F::Ty(arg));if i>0{work.push(F::Text(","));}}
            }
        }
    }}text
}
fn value_text(root:&ExtValue)->String {
    enum F<'a>{Value(&'a ExtValue),Text(&'static str)}
    let mut work=vec![F::Value(root)];let mut out=String::new();
    while let Some(f)=work.pop(){match f{
        F::Text(s)=>out.push_str(s),F::Value(v)=>match v{
            ExtValue::Core{value:CoreValue::UInt{n:KIntC::InRange(n)}}=>out.push_str(&format!("UInt({n})")),
            ExtValue::Core{value:CoreValue::UInt{..}}=>out.push_str("UInt(out-of-range)"),
            ExtValue::Core{value:CoreValue::Bool{b}}=>out.push_str(if *b{"Bool(true)"}else{"Bool(false)"}),
            ExtValue::Core{..}=>out.push_str("UnsupportedCore"),
            ExtValue::Constructor{ty,ctor,fields}=>{
                out.push_str(&type_text(ty));out.push_str("::");out.push_str(ctor);out.push('(');work.push(F::Text(")"));
                for (i,field) in fields.iter().enumerate().rev(){work.push(F::Value(field));if i>0{work.push(F::Text(","));}}
            }
        }
    }}out
}
struct Counts {values:usize,types:usize,depth:usize}
fn count(root:&ExtValue,limits:NominalInputLimits)->Result<Counts>{
    let mut c=Counts{values:0,types:0,depth:0};let mut work=vec![(root,1usize)];
    while let Some((v,depth))=work.pop(){
        c.values=add(c.values,1)?;c.depth=c.depth.max(depth);
        if depth>limits.max_structural_ast_depth{return Err(NominalInputError::StructuralDepthExceeded{depth,limit:limits.max_structural_ast_depth})}
        match v{
            ExtValue::Core{value:CoreValue::UInt{..}|CoreValue::Bool{..}}=>{},
            ExtValue::Core{..}=>return Err(NominalInputError::UnsupportedValueKind{kind:"typed AST supports UInt, Bool and nominal constructors".into()}),
            ExtValue::Constructor{ty,fields,..}=>{
                let mut types=vec![(ty,depth)];
                while let Some((t,d))=types.pop(){c.types=add(c.types,1)?;c.depth=c.depth.max(d);
                    if d>limits.max_structural_ast_depth{return Err(NominalInputError::StructuralDepthExceeded{depth:d,limit:limits.max_structural_ast_depth})}
                    if let TypeExprSrc::Apply{args,..}=t{for child in args.iter().rev(){types.push((child,add(d,1)?));}}
                }
                for child in fields.iter().rev(){work.push((child,add(depth,1)?));}
            }
        }
    }Ok(c)
}

/// Bind exact named arguments without JSON recursion/byte representation. The
/// SAME ExtValue objects move into Literal nodes; no evaluator/planner is called.
/// All source functions/declarations and old entry Drop semantics are unchanged.
/// Unsupported builtin List/Tree literals are explicit at this interface version.
pub fn bind_nominal_ast_inputs(program:&ExtProgram,function_name:&str,
    supplied:Vec<NominalAstArgument>,limits:NominalInputLimits,existing_ast_occurrences:usize)
    ->Result<BoundNominalEntry>
{
    if limits.max_ast_occurrences==0{return Err(NominalInputError::LimitsInvalid{field:"max_ast_occurrences"})}
    if limits.max_structural_ast_depth==0{return Err(NominalInputError::LimitsInvalid{field:"max_structural_ast_depth"})}
    if function_name.is_empty(){return Err(NominalInputError::EmptyName{context:"function name"})}
    let found:Vec<_>=program.functions.iter().enumerate().filter(|(_,f)|f.name==function_name).collect();
    if found.is_empty(){return Err(NominalInputError::UnknownFunction{name:function_name.into()})}
    if found.len()!=1{return Err(NominalInputError::AmbiguousFunction{name:function_name.into(),count:found.len()})}
    let (function_index,function)=found[0];
    for (i,arg) in supplied.iter().enumerate(){
        if arg.name.is_empty(){return Err(NominalInputError::EmptyName{context:"argument name"})}
        if supplied[..i].iter().any(|a|a.name==arg.name){return Err(NominalInputError::DuplicateArgument{name:arg.name.clone(),index:i})}
    }
    for (i,(a,p)) in supplied.iter().zip(&function.parameters).enumerate(){
        if a.name!=p.name{return Err(NominalInputError::ArgumentNameMismatch{index:i,expected:p.name.clone(),actual:a.name.clone()})}
    }
    if supplied.len()<function.parameters.len(){let index=supplied.len();return Err(NominalInputError::MissingArgument{index,name:function.parameters[index].name.clone()})}
    if supplied.len()>function.parameters.len(){let index=function.parameters.len();return Err(NominalInputError::ExtraArgument{index,name:supplied[index].name.clone()})}
    let mut per_argument=Vec::with_capacity(supplied.len());let mut total=Counts{values:0,types:0,depth:0};
    for arg in &supplied {
        let c=count(&arg.value,limits)?;total.values=add(total.values,c.values)?;
        total.types=add(total.types,c.types)?;total.depth=total.depth.max(c.depth);
        per_argument.push(c);
    }
    let expr_nodes=add(1,supplied.len())?;let synthesized=add(expr_nodes,add(total.values,total.types)?)?;
    let combined=add(existing_ast_occurrences,synthesized)?;
    if combined>limits.max_ast_occurrences{return Err(NominalInputError::CombinedAstBudgetExceeded{existing:existing_ast_occurrences,synthesized,limit:limits.max_ast_occurrences})}
    let depth=add(total.depth,2)?;
    if depth>limits.max_structural_ast_depth{return Err(NominalInputError::StructuralDepthExceeded{depth,limit:limits.max_structural_ast_depth})}
    // Render complete visible-value provenance only after both caller extents.
    let arguments=supplied.iter().zip(&function.parameters).zip(per_argument).enumerate()
        .map(|(index,((arg,param),c))| BoundArgumentProvenance {
            index,name:arg.name.clone(),declared_type:type_text(&param.ty),
            value_kind:match &arg.value{ExtValue::Core{value:CoreValue::UInt{..}}=>"UInt",ExtValue::Core{..}=>"Bool",ExtValue::Constructor{..}=>"Constructor"},
            bound_value:value_text(&arg.value),value_nodes:c.values,type_nodes:c.types,
        }).collect();
    Ok(BoundNominalEntry{entry:ExtExpr::Call{name:function_name.into(),args:supplied.into_iter().map(|a|ExtExpr::Literal{value:a.value}).collect()},
        binding:NominalBindingProvenance{format:NOMINAL_AST_INPUTS_FORMAT,function_index,function_name:function_name.into(),arguments,
            synthesized_expr_nodes:expr_nodes,synthesized_value_nodes:total.values,synthesized_type_nodes:total.types,
            synthesized_structural_depth:depth,combined_ast_occurrences:combined}})
}
