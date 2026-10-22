//! Explicit nominal text preparation into the existing owned ExtProgram.
//! No evaluator, generic admission or inference. Existing caller-owned parser
//! extents are reused; no release defaults or owner-preparation policy is added.
use crate::sherlock::helper_machine::shared::{
    BinaryOp, CorePattern, CoreValue, LayoutSchemaC, LocatorC, UnaryOp,
    owner_ext_source_program::{ExtDomain, ExtProgram},
    owner_ext_t3_uses::{ExtArm, ExtExpr, ExtFunction, ExtParamSrc, ExtPattern, ExtValue},
    owner_ext_type_builder::{CtorDeclSrc, CtorFieldSrc, ExtTypeBuildError, TypeDeclSrc, TypeExprSrc},
    prepare_ext_program_types, PreparedExtProgramSource,
    i128_class::KIntC,
};
use mori_lexer::{TokenIdx, TokenKind, Tokens};
pub use crate::sherlock_draft::{DraftError, SourceSpan, DraftParseBudget,
    DraftBudgetResource, DraftBudgetExceeded};

pub const NOMINAL_LANGUAGE: &str = "moriarty-sherlock-nominal/1";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NamedSite { pub name: String, pub span: SourceSpan }
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldSite { pub binder: NamedSite, pub type_span: SourceSpan }
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConstructorSite { pub name: NamedSite, pub span: SourceSpan, pub fields: Vec<FieldSite> }
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeclarationSite {
    pub name: NamedSite, pub span: SourceSpan, pub parameters: Vec<NamedSite>,
    pub constructors: Vec<ConstructorSite>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FunctionSite {
    pub name: NamedSite, pub span: SourceSpan, pub parameters: Vec<FieldSite>,
    pub return_type_span: SourceSpan,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SourceOwner { Entry, Function(usize) }
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExpressionOccurrence {
    pub owner: SourceOwner, pub child_path: Vec<usize>, pub span: SourceSpan,
    pub kind: &'static str,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PatternOccurrence {
    pub owner: SourceOwner, pub match_path: Vec<usize>, pub arm: usize,
    pub field_path: Vec<usize>, pub span: SourceSpan, pub kind: &'static str,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NominalProvenance {
    pub language_span: SourceSpan, pub program_name_span: SourceSpan,
    pub declarations: Vec<DeclarationSite>, pub functions: Vec<FunctionSite>,
    /// Entry preorder, then function bodies in unchanged function-table order.
    pub expressions: Vec<ExpressionOccurrence>,
    /// Match occurrence order, arm order, then nested field preorder.
    pub patterns: Vec<PatternOccurrence>,
}
pub struct ParsedNominal {
    pub name: String, pub program: ExtProgram, pub provenance: NominalProvenance,
}
pub enum NominalSourceError { Parse(DraftError), Preparation(ExtTypeBuildError) }
pub struct PreparedNominalText {
    pub name: String, pub provenance: NominalProvenance, pub prepared: PreparedExtProgramSource,
}

/// Parse a selected nominal text source without evaluating it.
/// H1: this unbounded host preparation API has no new native-stack/heap/time
/// admission guarantee. Existing lexer hard limits remain. The existing bounded
/// draft parser is unchanged. The separate bounded nominal entry consumes
/// explicit parser extents; owner preparation and full H1 remain separate.
pub fn parse_nominal_source(source: &str) -> Result<ParsedNominal, DraftError> {
    let tokens=mori_lexer::lex(source).map_err(|e| DraftError::Lex(e.to_string()))?;
    Parser::new(source,tokens,None).program()
}

/// Caller-bounded parsing using the existing draft budget fields and errors.
/// Expr/Pattern occurrences and expression structural depth keep their existing
/// meanings; nominal type recursion uses the same active parser-descent bound.
/// Declaration/type/field storage is token bounded, not a separate invented cap.
/// No defaults, owner-preparation budget or process-wide H1 guarantee is supplied.
pub fn parse_nominal_source_bounded(source: &str, budget: DraftParseBudget)
    -> Result<ParsedNominal,DraftError>
{
    within(DraftBudgetResource::SourceBytes,budget.max_source_bytes,source.len())?;
    let tokens=mori_lexer::lex_with_token_limit(source,budget.max_tokens)
        .map_err(|error| match error {
            mori_lexer::TokenLimitedLexError::Lex(error)=>DraftError::Lex(error.to_string()),
            mori_lexer::TokenLimitedLexError::TokenLimit {limit,observed}=>
                DraftError::Budget(DraftBudgetExceeded {resource:DraftBudgetResource::Tokens,limit,observed}),
        })?;
    Parser::new(source,tokens,Some(budget)).program()
}

/// The bounded text parser is consumed before the SAME existing owner API.
/// Owner Pass-B/Pass-T allocation and typing refusal semantics remain unchanged.
pub fn prepare_nominal_source_types_bounded(source: &str, budget: DraftParseBudget,
    layout: &LayoutSchemaC, domain: ExtDomain, locator: Option<LocatorC>)
    -> Result<PreparedNominalText,NominalSourceError>
{
    let parsed=parse_nominal_source_bounded(source,budget).map_err(NominalSourceError::Parse)?;
    let prepared=prepare_ext_program_types(parsed.program,layout,domain,locator)
        .map_err(NominalSourceError::Preparation)?;
    Ok(PreparedNominalText {name:parsed.name,provenance:parsed.provenance,prepared})
}

fn within(resource: DraftBudgetResource,limit: usize,observed: usize)->Result<(),DraftError> {
    if observed>limit {Err(DraftError::Budget(DraftBudgetExceeded {resource,limit,observed}))}
    else {Ok(())}
}

/// Real existing owner type preparation; explicit caller domain/layout, no
/// domain inference or replacement of owner errors with parser diagnostics.
pub fn prepare_nominal_source_types(source: &str, layout: &LayoutSchemaC,
    domain: ExtDomain, locator: Option<LocatorC>) -> Result<PreparedNominalText, NominalSourceError>
{
    let parsed=parse_nominal_source(source).map_err(NominalSourceError::Parse)?;
    let prepared=prepare_ext_program_types(parsed.program,layout,domain,locator)
        .map_err(NominalSourceError::Preparation)?;
    Ok(PreparedNominalText { name: parsed.name, provenance: parsed.provenance, prepared })
}

struct Site { depth: usize, span: SourceSpan, kind: &'static str, children: Vec<Site>, patterns: Vec<Site> }
impl Site {
    fn new(span: SourceSpan, kind: &'static str, children: Vec<Site>) -> Self {
        let depth=children.iter().map(|c|c.depth).max().unwrap_or(0).saturating_add(1);
        Self { depth, span, kind, children, patterns: Vec::new() }
    }
}
struct E { value: ExtExpr, site: Site }
struct P { value: ExtPattern, site: Site }
fn separate(nodes: Vec<E>) -> (Vec<ExtExpr>, Vec<Site>) {
    nodes.into_iter().map(|n| (n.value,n.site)).unzip()
}
struct Parser<'a> { source: &'a str, tokens: Tokens, cursor: usize, end: u32,
    budget: Option<DraftParseBudget>,total_parameters: usize,ast_occurrences: usize,parser_depth: usize }
impl<'a> Parser<'a> {
    fn new(source: &'a str,tokens: Tokens,budget: Option<DraftParseBudget>)->Self {
        Self {source,tokens,cursor:0,end:0,budget,total_parameters:0,ast_occurrences:0,parser_depth:0}
    }
    fn descend<T>(&mut self,parse:impl FnOnce(&mut Self)->Result<T,DraftError>)->Result<T,DraftError> {
        if let Some(b)=self.budget {within(DraftBudgetResource::ParserRecursionDepth,
            b.max_parser_recursion_depth,self.parser_depth.saturating_add(1))?;}
        self.parser_depth+=1;let result=parse(self);self.parser_depth-=1;result
    }
    fn reserve_occurrence(&mut self)->Result<(),DraftError> {
        if let Some(b)=self.budget {within(DraftBudgetResource::AstOccurrences,
            b.max_ast_occurrences,self.ast_occurrences.saturating_add(1))?;}
        self.ast_occurrences+=1;Ok(())
    }
    fn structural_depth(&self,child:usize)->Result<(),DraftError> {
        if let Some(b)=self.budget {within(DraftBudgetResource::StructuralAstDepth,
            b.max_structural_ast_depth,child.saturating_add(1))?;}
        Ok(())
    }
    fn structural_sites(&self,children:&[Site])->Result<(),DraftError> {
        self.structural_depth(children.iter().map(|s|s.depth).max().unwrap_or(0))
    }

    fn index(&self, ahead: usize) -> TokenIdx { TokenIdx::new((self.cursor+ahead).min(self.tokens.len()-1)) }
    fn kind(&self, ahead: usize) -> TokenKind { self.tokens.kind(self.index(ahead)) }
    fn span(&self) -> SourceSpan {
        let x=self.tokens.span(self.index(0),self.source); SourceSpan { start: x.start,end: x.end }
    }
    fn text(&self) -> &str { let x=self.span(); &self.source[x.start as usize..x.end as usize] }
    fn word(&self, s: &str) -> bool { self.kind(0).is_word() && self.text()==s }
    fn error(&self, expected: &'static str) -> DraftError {
        DraftError::Syntax { span: self.span(),expected,found: self.text().into() }
    }
    fn bump(&mut self) { self.end=self.span().end; if self.kind(0)!=TokenKind::Eof { self.cursor+=1; } }
    fn expect(&mut self, k: TokenKind, s: &'static str) -> Result<(),DraftError> {
        if self.kind(0)!=k { return Err(self.error(s)); } self.bump(); Ok(())
    }
    fn expect_word(&mut self, s: &'static str) -> Result<(),DraftError> {
        if !self.word(s) { return Err(self.error(s)); } self.bump(); Ok(())
    }
    // The shared lexer also tags the financial grammar's words (for example
    // asset, pool and None). They have no syntax role in this nominal grammar.
    // Keep nominal syntax and booleans reserved in every identifier position.
    fn nominal_name_token(&self) -> bool {
        self.kind(0).is_word() && !reserved(self.text())
            && !matches!(self.kind(0),TokenKind::KwTrue|TokenKind::KwFalse)
    }
    fn name(&mut self) -> Result<NamedSite,DraftError> {
        if !self.nominal_name_token() { return Err(self.error("identifier")); }
        let n=NamedSite { name: self.text().into(),span: self.span() }; self.bump(); Ok(n)
    }
    fn type_expr(&mut self, parameters: &[String]) -> Result<(TypeExprSrc,SourceSpan),DraftError> {
        self.descend(|p|p.type_expr_inner(parameters))
    }
    fn type_expr_inner(&mut self, parameters: &[String]) -> Result<(TypeExprSrc,SourceSpan),DraftError> {
        if !self.nominal_name_token()
            && !matches!(self.text(),"UInt"|"Bool"|"List"|"Tree")
        { return Err(self.error("type name")); }
        let start=self.span().start; let name=self.text().to_owned(); self.bump();
        let ty=if self.kind(0)==TokenKind::Lt {
            self.bump(); let mut args=vec![self.type_expr(parameters)?.0];
            while self.kind(0)==TokenKind::Comma { self.bump(); args.push(self.type_expr(parameters)?.0); }
            self.expect(TokenKind::Gt,">")?; TypeExprSrc::Apply { name,args }
        } else { match name.as_str() {
            "UInt"=>TypeExprSrc::UInt,"Bool"=>TypeExprSrc::Bool,
            "List"=>TypeExprSrc::List,"Tree"=>TypeExprSrc::Tree,
            _ if parameters.iter().any(|p| p==&name)=>TypeExprSrc::Param { name },
            _=>TypeExprSrc::Named { name },
        }};
        Ok((ty,SourceSpan { start,end: self.end }))
    }
    fn program(mut self) -> Result<ParsedNominal,DraftError> {
        self.expect_word("language")?;
        if self.kind(0)!=TokenKind::String { return Err(self.error("nominal language string")); }
        let language_span=self.span();
        if mori_lexer::decode_string(self.text())!=NOMINAL_LANGUAGE { return Err(self.error(NOMINAL_LANGUAGE)); }
        self.bump(); self.expect(TokenKind::Semicolon,";")?;self.expect_word("program")?;
        let name=self.name()?;self.expect(TokenKind::LBrace,"{")?;
        let mut types=Vec::new();let mut declarations=Vec::new();
        while self.word("type") { let (ty,site)=self.declaration()?;types.push(ty);declarations.push(site); }
        let mut functions=Vec::new();let mut function_sites=Vec::new();let mut body_sites=Vec::new();
        while self.word("fn") {
            if let Some(b)=self.budget {within(DraftBudgetResource::Functions,b.max_functions,functions.len().saturating_add(1))?;}
            let start=self.span().start;self.bump();let name=self.name()?;
            self.expect(TokenKind::LParen,"(")?;let mut parameters=Vec::new();let mut parameter_sites=Vec::new();
            if self.kind(0)!=TokenKind::RParen { loop {
                if let Some(b)=self.budget {
                    within(DraftBudgetResource::ParametersPerFunction,b.max_parameters_per_function,parameters.len().saturating_add(1))?;
                    within(DraftBudgetResource::TotalParameters,b.max_total_parameters,self.total_parameters.saturating_add(1))?;
                }
                self.total_parameters+=1;
                let binder=self.name()?;self.expect(TokenKind::Colon,":")?;
                let (ty,type_span)=self.type_expr(&[])?;
                parameters.push(ExtParamSrc { name: binder.name.clone(),ty });
                parameter_sites.push(FieldSite { binder,type_span });
                if self.kind(0)!=TokenKind::Comma { break; } self.bump();
            }}
            self.expect(TokenKind::RParen,")")?;self.expect(TokenKind::Colon,":")?;
            let (return_type,return_type_span)=self.type_expr(&[])?;let body=self.block()?;
            functions.push(ExtFunction { name: name.name.clone(),parameters,return_type,body: body.value });
            function_sites.push(FunctionSite { name,span: SourceSpan { start,end: self.end },parameters: parameter_sites,return_type_span });body_sites.push(body.site);
        }
        self.expect_word("entry")?;let entry=self.expression(0)?;self.expect(TokenKind::Semicolon,";")?;
        self.expect(TokenKind::RBrace,"}")?;self.expect(TokenKind::Eof,"end of nominal program")?;
        let mut provenance=NominalProvenance { language_span,program_name_span: name.span,
            declarations,functions: function_sites,expressions: Vec::new(),patterns: Vec::new() };
        collect(&entry.site,SourceOwner::Entry,&mut provenance,self.budget)?;
        for (i,site) in body_sites.iter().enumerate() { collect(site,SourceOwner::Function(i),&mut provenance,self.budget)?; }
        Ok(ParsedNominal { name: name.name,program: ExtProgram { types,assets: Vec::new(),states: Vec::new(),inputs: Vec::new(),functions,entry: entry.value },provenance })
    }
    fn declaration(&mut self) -> Result<(TypeDeclSrc,DeclarationSite),DraftError> {
        let start=self.span().start;self.bump();let name=self.name()?;
        let mut parameters=Vec::new();
        if self.kind(0)==TokenKind::Lt {
            self.bump();parameters.push(self.name()?);
            while self.kind(0)==TokenKind::Comma { self.bump();parameters.push(self.name()?); }
            self.expect(TokenKind::Gt,">")?;
        }
        let parameter_names:Vec<String>=parameters.iter().map(|p| p.name.clone()).collect();
        self.expect(TokenKind::LBrace,"{")?;let mut ctors=Vec::new();let mut constructors=Vec::new();
        while self.kind(0)!=TokenKind::RBrace {
            let ctor_start=self.span().start;let ctor=self.name()?;let mut fields=Vec::new();let mut field_sites=Vec::new();
            if self.kind(0)==TokenKind::LParen {
                self.bump();if self.kind(0)!=TokenKind::RParen { loop {
                    let binder=self.name()?;self.expect(TokenKind::Colon,":")?;
                    let (ty,type_span)=self.type_expr(&parameter_names)?;
                    fields.push(CtorFieldSrc { name: binder.name.clone(),ty });field_sites.push(FieldSite { binder,type_span });
                    if self.kind(0)!=TokenKind::Comma { break; }self.bump();
                }}self.expect(TokenKind::RParen,")")?;
            }
            self.expect(TokenKind::Semicolon,";")?;
            ctors.push(CtorDeclSrc { name: ctor.name.clone(),fields });constructors.push(ConstructorSite { name: ctor,span: SourceSpan { start: ctor_start,end: self.end },fields: field_sites });
        }
        self.bump();let site=DeclarationSite { name: name.clone(),span: SourceSpan { start,end: self.end },parameters,constructors };
        Ok((TypeDeclSrc { name: name.name,parameters: parameter_names,ctors },site))
    }
    fn block(&mut self) -> Result<E,DraftError> {
        self.descend(Self::block_inner)
    }
    fn block_inner(&mut self) -> Result<E,DraftError> {
        self.expect(TokenKind::LBrace,"{")?;let x=self.expression(0)?;self.expect(TokenKind::RBrace,"}")?;Ok(x)
    }
    fn binary(&self) -> Option<(BinaryOp,u8,usize)> { Some(match self.kind(0) {
        TokenKind::Eq if self.kind(1)==TokenKind::Eq=>(BinaryOp::Eq,1,2),
        TokenKind::Lt=>(BinaryOp::Lt,1,1),TokenKind::Plus=>(BinaryOp::Add,2,1),
        TokenKind::Minus=>(BinaryOp::Sub,2,1),TokenKind::Star=>(BinaryOp::Mul,3,1),_=>return None,
    }) }
    fn expression(&mut self, minimum: u8) -> Result<E,DraftError> {
        self.descend(|p|p.expression_inner(minimum))
    }
    fn expression_inner(&mut self, minimum: u8) -> Result<E,DraftError> {
        let mut left=self.atom()?;
        while let Some((op,precedence,width))=self.binary() {
            if precedence<minimum { break; }for _ in 0..width { self.bump(); }
            let right=self.expression(precedence+1)?;self.reserve_occurrence()?;self.structural_depth(left.site.depth.max(right.site.depth))?;let span=SourceSpan { start:left.site.span.start,end:right.site.span.end };
            left=E { value: ExtExpr::Binary { op,left: Box::new(left.value),right: Box::new(right.value) },site: Site::new(span,"binary",vec![left.site,right.site]) };
        }Ok(left)
    }
    fn atom(&mut self) -> Result<E,DraftError> {
        self.descend(Self::atom_inner)
    }
    fn atom_inner(&mut self) -> Result<E,DraftError> {
        let start=self.span().start;
        if self.kind(0)==TokenKind::LParen { self.bump();let mut e=self.expression(0)?;self.expect(TokenKind::RParen,")")?;e.site.span=SourceSpan { start,end:self.end };return Ok(e); }
        self.reserve_occurrence()?;self.structural_depth(0)?;
        if self.word("let") {
            self.bump();let n=self.name()?;self.expect(TokenKind::Eq,"=")?;let value=self.expression(0)?;self.expect_word("in")?;let body=self.expression(0)?;self.structural_depth(value.site.depth.max(body.site.depth))?;
            return Ok(E { value:ExtExpr::Let { name:n.name,value:Box::new(value.value),body:Box::new(body.value) },site:Site::new(SourceSpan {start,end:self.end},"let",vec![value.site,body.site]) });
        }
        if self.word("if") {
            self.bump();let c=self.expression(0)?;let t=self.block()?;self.expect_word("else")?;let e=self.block()?;self.structural_depth(c.site.depth.max(t.site.depth).max(e.site.depth))?;
            return Ok(E {value:ExtExpr::If {cond:Box::new(c.value),then_expr:Box::new(t.value),else_expr:Box::new(e.value)},site:Site::new(SourceSpan {start,end:self.end},"if",vec![c.site,t.site,e.site])});
        }
        if self.word("match") { return self.match_expression(); }
        if self.word("construct") {
            self.bump();let (ty,_)=self.type_expr(&[])?;self.qualification()?;let ctor=self.name()?;let (fields,sites)=separate(self.arguments()?);self.structural_sites(&sites)?;
            return Ok(E {value:ExtExpr::Construct {ty,ctor:ctor.name,fields},site:Site::new(SourceSpan {start,end:self.end},"construct",sites)});
        }
        if self.kind(0)==TokenKind::Number {
            let text=self.text().to_owned();let span=self.span();let n=text.replace('_',"").parse::<u128>().map_err(|_| DraftError::UIntLiteral {span,text})?;self.bump();
            return Ok(E {value:ExtExpr::Literal {value:ExtValue::Core {value:CoreValue::UInt {n:KIntC::InRange(n)}}},site:Site::new(span,"uint",vec![])});
        }
        if matches!(self.kind(0),TokenKind::KwTrue|TokenKind::KwFalse) {
            let b=self.kind(0)==TokenKind::KwTrue;self.bump();return Ok(E {value:ExtExpr::Literal {value:ExtValue::Core {value:CoreValue::Bool {b}}},site:Site::new(SourceSpan {start,end:self.end},"bool",vec![])});
        }
        if self.word("nil") { self.bump();return Ok(E {value:ExtExpr::Literal {value:ExtValue::Core {value:CoreValue::List {elements:vec![]}}},site:Site::new(SourceSpan {start,end:self.end},"nil",vec![])}); }
        if ["cons","leaf","branch","not"].iter().any(|s|self.word(s)) {
            let n=self.text().to_owned();self.bump();let mut args=self.arguments()?;
            self.structural_depth(args.iter().map(|a|a.site.depth).max().unwrap_or(0))?;
            let expected=if n=="leaf"||n=="not" {1}else{2};if args.len()!=expected {return Err(self.error("dedicated constructor arguments in declared order/arity"));}
            let (value,sites)=if expected==1 {let a=args.remove(0);(ExtExpr::Unary {op:if n=="leaf" {UnaryOp::Leaf}else{UnaryOp::Not},operand:Box::new(a.value)},vec![a.site])}else{let b=args.pop().expect("two arguments");let a=args.pop().expect("two arguments");(ExtExpr::Binary {op:if n=="cons" {BinaryOp::Cons}else{BinaryOp::Branch},left:Box::new(a.value),right:Box::new(b.value)},vec![a.site,b.site])};
            return Ok(E {value,site:Site::new(SourceSpan {start,end:self.end},"dedicated-constructor",sites)});
        }
        let n=self.name()?;
        if self.kind(0)==TokenKind::LParen {let (args,sites)=separate(self.arguments()?);self.structural_sites(&sites)?;Ok(E {value:ExtExpr::Call {name:n.name,args},site:Site::new(SourceSpan {start,end:self.end},"call",sites)})}
        else {Ok(E {value:ExtExpr::Var {name:n.name},site:Site::new(n.span,"var",vec![])})}
    }
    fn arguments(&mut self) -> Result<Vec<E>,DraftError> {
        self.descend(Self::arguments_inner)
    }
    fn arguments_inner(&mut self) -> Result<Vec<E>,DraftError> {
        self.expect(TokenKind::LParen,"(")?;let mut args=Vec::new();
        if self.kind(0)!=TokenKind::RParen {loop {args.push(self.expression(0)?);if self.kind(0)!=TokenKind::Comma {break;}self.bump();}}
        self.expect(TokenKind::RParen,")")?;Ok(args)
    }
    fn qualification(&mut self) -> Result<(),DraftError> {self.expect(TokenKind::Colon,"::")?;self.expect(TokenKind::Colon,"::")}
    fn match_expression(&mut self) -> Result<E,DraftError> {
        self.descend(Self::match_expression_inner)
    }
    fn match_expression_inner(&mut self) -> Result<E,DraftError> {
        let start=self.span().start;self.bump();let scrutinee=self.expression(0)?;self.expect(TokenKind::LBrace,"{")?;
        let mut arms=Vec::new();let mut children=vec![scrutinee.site];let mut patterns=Vec::new();
        loop {
            let pattern=self.pattern()?;self.expect(TokenKind::Eq,"=>")?;self.expect(TokenKind::Gt,"=>")?;let body=self.expression(0)?;
            arms.push(ExtArm {pattern:pattern.value,body:body.value});patterns.push(pattern.site);children.push(body.site);
            if self.kind(0)!=TokenKind::Comma {break;}self.bump();
        }
        self.expect(TokenKind::RBrace,"}")?;self.structural_sites(&children)?;let mut site=Site::new(SourceSpan {start,end:self.end},"match",children);site.patterns=patterns;
        Ok(E {value:ExtExpr::Match {scrutinee:Box::new(scrutinee.value),arms},site})
    }
    fn pattern(&mut self) -> Result<P,DraftError> {
        self.descend(Self::pattern_inner)
    }
    fn pattern_inner(&mut self) -> Result<P,DraftError> {
        self.reserve_occurrence()?;
        let start=self.span().start;
        if self.word("wildcard") {self.bump();return Ok(P {value:ExtPattern::Wildcard,site:Site::new(SourceSpan {start,end:self.end},"wildcard",vec![])});}
        if self.word("nil") {self.bump();return Ok(P {value:ExtPattern::Dedicated {pattern:CorePattern::PNil},site:Site::new(SourceSpan {start,end:self.end},"nil",vec![])});}
        if ["cons","leaf","branch"].iter().any(|s|self.word(s)) {
            let n=self.text().to_owned();self.bump();self.expect(TokenKind::LParen,"(")?;let a=self.name()?;
            let pattern=if n=="leaf" {CorePattern::PLeaf {x:a.name}}else{self.expect(TokenKind::Comma,",")?;let b=self.name()?;if n=="cons" {CorePattern::PCons {x:a.name,y:b.name}}else{CorePattern::PBranch {x:a.name,y:b.name}}};self.expect(TokenKind::RParen,")")?;
            return Ok(P {value:ExtPattern::Dedicated {pattern},site:Site::new(SourceSpan {start,end:self.end},"dedicated-pattern",vec![])});
        }
        // An explicit :: or < starts a nominal type-qualified constructor pattern.
        // No casing convention or declaration lookup decides variable vs constructor.
        if self.kind(0).is_word() && (self.kind(1)==TokenKind::Lt||self.kind(1)==TokenKind::Colon) {
            let (ty,_)=self.type_expr(&[])?;self.qualification()?;let ctor=self.name()?;self.expect(TokenKind::LParen,"(")?;
            let mut fields=Vec::new();let mut sites=Vec::new();
            if self.kind(0)!=TokenKind::RParen {loop {let p=self.pattern()?;fields.push(p.value);sites.push(p.site);if self.kind(0)!=TokenKind::Comma {break;}self.bump();}}
            self.expect(TokenKind::RParen,")")?;
            Ok(P {value:ExtPattern::Ctor {ty,ctor:ctor.name,fields},site:Site::new(SourceSpan {start,end:self.end},"constructor-pattern",sites)})
        } else {let n=self.name()?;Ok(P {value:ExtPattern::Dedicated {pattern:CorePattern::PVar {x:n.name}},site:Site::new(n.span,"binder",vec![])})}
    }
}
fn reserved(s:&str)->bool {matches!(s,"language"|"program"|"type"|"construct"|"wildcard"|"fn"|"entry"|"let"|"in"|"if"|"else"|"match"|"UInt"|"Bool"|"List"|"Tree"|"nil"|"cons"|"leaf"|"branch"|"not")}
fn collect(root:&Site,owner:SourceOwner,out:&mut NominalProvenance,budget:Option<DraftParseBudget>)->Result<(),DraftError> {
    let mut todo=vec![(root,Vec::new())];
    while let Some((node,path))=todo.pop() {
        if let Some(b)=budget {
            within(DraftBudgetResource::StructuralAstDepth,b.max_structural_ast_depth,path.len().saturating_add(1))?;
            within(DraftBudgetResource::AstOccurrences,b.max_ast_occurrences,out.expressions.len().saturating_add(out.patterns.len()).saturating_add(1))?;
        }
        out.expressions.push(ExpressionOccurrence {owner,child_path:path.clone(),span:node.span,kind:node.kind});
        for (arm,pattern) in node.patterns.iter().enumerate() {
            let mut pending=vec![(pattern,Vec::new())];
            while let Some((p,field_path))=pending.pop() {
                if let Some(b)=budget {within(DraftBudgetResource::AstOccurrences,b.max_ast_occurrences,out.expressions.len().saturating_add(out.patterns.len()).saturating_add(1))?;}
                out.patterns.push(PatternOccurrence {owner,match_path:path.clone(),arm,field_path:field_path.clone(),span:p.span,kind:p.kind});
                for (i,c) in p.children.iter().enumerate().rev() {let mut next=field_path.clone();next.push(i);pending.push((c,next));}
            }
        }
        for (i,c) in node.children.iter().enumerate().rev() {let mut next=path.clone();next.push(i);todo.push((c,next));}
    }
    Ok(())
}
