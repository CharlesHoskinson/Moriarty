//! Unregistered draft syntax preparation using the real Moriarty lexer.
//! Integration proposal: a Core lib.rs path module outside Sherlock's standalone
//! pure-Std graph. No current export, profile registration or execution.
use crate::sherlock::{
    Arm, BinaryOp, Expr, ExprRef, Function, Name, Parameter, Pattern, Program, Type, UnaryOp, Value,
};
use mori_lexer::{TokenIdx, TokenKind, Tokens};
use std::sync::Arc;

pub const DRAFT_LANGUAGE: &str = "moriarty-sherlock-draft/1";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SourceSpan {
    pub start: u32,
    pub end: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BinderSite {
    pub name: Name,
    pub span: SourceSpan,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParameterSite {
    pub binder: BinderSite,
    pub type_span: SourceSpan,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FunctionSite {
    pub index: usize,
    pub span: SourceSpan,
    pub name_span: SourceSpan,
    pub parameters: Vec<ParameterSite>,
    pub return_type_span: SourceSpan,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceLocation {
    /// None is the entry; Some(i) is body i in the unchanged function table.
    pub function_index: Option<usize>,
    /// Existing AST child order, independent of storage/pointer sharing.
    pub child_path: Vec<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExpressionSite {
    pub source: SourceLocation,
    pub span: SourceSpan,
    pub kind: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PatternSite {
    pub match_source: SourceLocation,
    pub arm_index: usize,
    pub span: SourceSpan,
    pub binders: Vec<BinderSite>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceProvenance {
    pub language_span: SourceSpan,
    pub program_name_span: SourceSpan,
    pub functions: Vec<FunctionSite>,
    // Image order: entry preorder, then bodies in original function order.
    pub expressions: Vec<ExpressionSite>,
    pub patterns: Vec<PatternSite>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParsedDraft {
    pub name: Name,
    pub program: Arc<Program>,
    pub provenance: SourceProvenance,
}

/// Preparation diagnostics only, never an existing machine Outcome/error code.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DraftError {
    Lex(String),
    Syntax {
        span: SourceSpan,
        expected: &'static str,
        found: String,
    },
    UIntLiteral {
        span: SourceSpan,
        text: String,
    },
}

///
/// # Security Warning (H1 — Unmetered Host Evaluator)
/// This machine is a host diagnostic reference with zero native authority and no host time,
/// heap, or native stack admission bounds. Do not expose it to untrusted inputs.
pub fn parse_draft_source(source: &str) -> Result<ParsedDraft, DraftError> {
    let tokens = mori_lexer::lex(source).map_err(|error| DraftError::Lex(error.to_string()))?;
    Parser {
        source,
        tokens,
        cursor: 0,
        end: 0,
    }
    .program()
}

struct Node {
    expression: ExprRef,
    span: SourceSpan,
    children: Vec<Node>,
    patterns: Vec<(SourceSpan, Vec<BinderSite>)>,
}

impl Node {
    fn new(expression: Expr, span: SourceSpan, children: Vec<Node>) -> Self {
        Self {
            expression: Arc::new(expression),
            span,
            children,
            patterns: vec![],
        }
    }
}

struct Parser<'a> {
    source: &'a str,
    tokens: Tokens,
    cursor: usize,
    end: u32,
}

impl Parser<'_> {
    fn index(&self, ahead: usize) -> TokenIdx {
        TokenIdx::new((self.cursor + ahead).min(self.tokens.len() - 1))
    }
    fn kind(&self, ahead: usize) -> TokenKind {
        self.tokens.kind(self.index(ahead))
    }
    fn span(&self) -> SourceSpan {
        let span = self.tokens.span(self.index(0), self.source);
        SourceSpan {
            start: span.start,
            end: span.end,
        }
    }
    fn text(&self) -> &str {
        let span = self.span();
        &self.source[span.start as usize..span.end as usize]
    }
    fn word(&self, word: &str) -> bool {
        self.kind(0).is_word() && self.text() == word
    }
    fn error(&self, expected: &'static str) -> DraftError {
        DraftError::Syntax {
            span: self.span(),
            expected,
            found: self.text().into(),
        }
    }
    fn bump(&mut self) {
        self.end = self.span().end;
        if self.kind(0) != TokenKind::Eof {
            self.cursor += 1;
        }
    }
    fn expect(&mut self, kind: TokenKind, expected: &'static str) -> Result<(), DraftError> {
        if self.kind(0) != kind {
            return Err(self.error(expected));
        }
        self.bump();
        Ok(())
    }
    fn expect_word(&mut self, word: &'static str) -> Result<(), DraftError> {
        if !self.word(word) {
            return Err(self.error(word));
        }
        self.bump();
        Ok(())
    }
    fn name(&mut self) -> Result<BinderSite, DraftError> {
        if self.kind(0) != TokenKind::Ident || reserved(self.text()) {
            return Err(self.error("identifier"));
        }
        let site = BinderSite {
            name: self.text().into(),
            span: self.span(),
        };
        self.bump();
        Ok(site)
    }
    fn ty(&mut self) -> Result<(Type, SourceSpan), DraftError> {
        let ty = match self.text() {
            "UInt" => Type::UInt,
            "Bool" => Type::Bool,
            "List" => Type::List,
            "Tree" => Type::Tree,
            _ => return Err(self.error("UInt, Bool, List or Tree")),
        };
        let span = self.span();
        self.bump();
        Ok((ty, span))
    }
    fn program(mut self) -> Result<ParsedDraft, DraftError> {
        self.expect_word("language")?;
        if self.kind(0) != TokenKind::String {
            return Err(self.error("draft language string"));
        }
        let language_span = self.span();
        if mori_lexer::decode_string(self.text()) != DRAFT_LANGUAGE {
            return Err(self.error(DRAFT_LANGUAGE));
        }
        self.bump();
        self.expect(TokenKind::Semicolon, ";")?;
        self.expect_word("program")?;
        let name = self.name()?;
        self.expect(TokenKind::LBrace, "{")?;
        let mut functions = Vec::new();
        let mut sites = Vec::new();
        let mut bodies = Vec::new();
        while self.word("fn") {
            let index = functions.len();
            let start = self.span().start;
            self.bump();
            let function_name = self.name()?;
            self.expect(TokenKind::LParen, "(")?;
            let mut parameters = Vec::new();
            let mut parameter_sites = Vec::new();
            if self.kind(0) != TokenKind::RParen {
                loop {
                    let binder = self.name()?;
                    self.expect(TokenKind::Colon, ":")?;
                    let (ty, type_span) = self.ty()?;
                    parameters.push(Parameter {
                        name: binder.name.clone(),
                        ty,
                    });
                    parameter_sites.push(ParameterSite { binder, type_span });
                    if self.kind(0) != TokenKind::Comma {
                        break;
                    }
                    self.bump();
                }
            }
            self.expect(TokenKind::RParen, ")")?;
            self.expect(TokenKind::Colon, ":")?;
            let (return_type, return_type_span) = self.ty()?;
            let body = self.block()?;
            functions.push(Function {
                name: function_name.name,
                parameters,
                return_type,
                body: body.expression.clone(),
            });
            sites.push(FunctionSite {
                index,
                span: SourceSpan {
                    start,
                    end: self.end,
                },
                name_span: function_name.span,
                parameters: parameter_sites,
                return_type_span,
            });
            bodies.push(body);
        }
        self.expect_word("entry")?;
        let entry = self.expression(0)?;
        self.expect(TokenKind::Semicolon, ";")?;
        self.expect(TokenKind::RBrace, "}")?;
        self.expect(TokenKind::Eof, "end of draft program")?;
        let program = Arc::new(Program {
            functions,
            entry: entry.expression.clone(),
        });
        let mut provenance = SourceProvenance {
            language_span,
            program_name_span: name.span,
            functions: sites,
            expressions: Vec::new(),
            patterns: Vec::new(),
        };
        collect_sites(&entry, None, &mut provenance);
        for (index, body) in bodies.iter().enumerate() {
            collect_sites(body, Some(index), &mut provenance);
        }
        Ok(ParsedDraft {
            name: name.name,
            program,
            provenance,
        })
    }
    fn block(&mut self) -> Result<Node, DraftError> {
        self.expect(TokenKind::LBrace, "{")?;
        let node = self.expression(0)?;
        self.expect(TokenKind::RBrace, "}")?;
        Ok(node)
    }
    fn expression(&mut self, minimum: u8) -> Result<Node, DraftError> {
        let mut left = self.atom()?;
        while let Some((op, precedence, width)) = self.binary() {
            if precedence < minimum {
                break;
            }
            for _ in 0..width {
                self.bump();
            }
            let right = self.expression(precedence + 1)?;
            let span = SourceSpan {
                start: left.span.start,
                end: right.span.end,
            };
            left = Node::new(
                Expr::Binary(op, left.expression.clone(), right.expression.clone()),
                span,
                vec![left, right],
            );
        }
        Ok(left)
    }
    fn binary(&self) -> Option<(BinaryOp, u8, u8)> {
        Some(match self.kind(0) {
            TokenKind::Eq if self.kind(1) == TokenKind::Eq => (BinaryOp::Eq, 1, 2),
            TokenKind::Lt => (BinaryOp::Lt, 1, 1),
            TokenKind::Plus => (BinaryOp::Add, 2, 1),
            TokenKind::Minus => (BinaryOp::Sub, 2, 1),
            TokenKind::Star => (BinaryOp::Mul, 3, 1),
            _ => return None,
        })
    }
    fn atom(&mut self) -> Result<Node, DraftError> {
        let start = self.span().start;
        if self.word("let") {
            self.bump();
            let binder = self.name()?;
            self.expect(TokenKind::Eq, "=")?;
            let value = self.expression(0)?;
            self.expect_word("in")?;
            let body = self.expression(0)?;
            return Ok(Node::new(
                Expr::Let(
                    binder.name,
                    value.expression.clone(),
                    body.expression.clone(),
                ),
                SourceSpan {
                    start,
                    end: body.span.end,
                },
                vec![value, body],
            ));
        }
        if self.word("if") {
            self.bump();
            let condition = self.expression(0)?;
            let yes = self.block()?;
            self.expect_word("else")?;
            let no = self.block()?;
            return Ok(Node::new(
                Expr::If(
                    condition.expression.clone(),
                    yes.expression.clone(),
                    no.expression.clone(),
                ),
                SourceSpan {
                    start,
                    end: self.end,
                },
                vec![condition, yes, no],
            ));
        }
        if self.word("match") {
            return self.match_expression(start);
        }
        if self.kind(0) == TokenKind::LParen {
            self.bump();
            let mut node = self.expression(0)?;
            self.expect(TokenKind::RParen, ")")?;
            node.span = SourceSpan {
                start,
                end: self.end,
            };
            return Ok(node);
        }
        if self.kind(0) == TokenKind::Number {
            let text = self.text().to_owned();
            let span = self.span();
            let value = text
                .replace('_', "")
                .parse::<u128>()
                .map_err(|_| DraftError::UIntLiteral { span, text })?;
            self.bump();
            return Ok(Node::new(Expr::Literal(Value::UInt(value)), span, vec![]));
        }
        if matches!(self.kind(0), TokenKind::KwTrue | TokenKind::KwFalse) {
            let value = self.kind(0) == TokenKind::KwTrue;
            self.bump();
            return Ok(Node::new(
                Expr::Literal(Value::Bool(value)),
                SourceSpan {
                    start,
                    end: self.end,
                },
                vec![],
            ));
        }
        if self.word("nil") {
            self.bump();
            return Ok(Node::new(
                Expr::Literal(Value::List(vec![])),
                SourceSpan {
                    start,
                    end: self.end,
                },
                vec![],
            ));
        }
        if ["cons", "leaf", "branch", "not"]
            .iter()
            .any(|word| self.word(word))
        {
            let constructor = self.text().to_owned();
            self.bump();
            let children = self.arguments()?;
            let expression = match (constructor.as_str(), children.as_slice()) {
                ("leaf", [child]) => Expr::Unary(UnaryOp::Leaf, child.expression.clone()),
                ("not", [child]) => Expr::Unary(UnaryOp::Not, child.expression.clone()),
                ("cons", [left, right]) => Expr::Binary(
                    BinaryOp::Cons,
                    left.expression.clone(),
                    right.expression.clone(),
                ),
                ("branch", [left, right]) => Expr::Binary(
                    BinaryOp::Branch,
                    left.expression.clone(),
                    right.expression.clone(),
                ),
                _ => return Err(self.error("constructor arguments in declared order/arity")),
            };
            return Ok(Node::new(
                expression,
                SourceSpan {
                    start,
                    end: self.end,
                },
                children,
            ));
        }
        let name = self.name()?;
        if self.kind(0) == TokenKind::LParen {
            let arguments = self.arguments()?;
            let expression = Expr::Call(
                name.name,
                arguments
                    .iter()
                    .map(|argument| argument.expression.clone())
                    .collect(),
            );
            Ok(Node::new(
                expression,
                SourceSpan {
                    start,
                    end: self.end,
                },
                arguments,
            ))
        } else {
            Ok(Node::new(Expr::Var(name.name), name.span, vec![]))
        }
    }
    fn arguments(&mut self) -> Result<Vec<Node>, DraftError> {
        self.expect(TokenKind::LParen, "(")?;
        let mut arguments = Vec::new();
        if self.kind(0) != TokenKind::RParen {
            loop {
                arguments.push(self.expression(0)?);
                if self.kind(0) != TokenKind::Comma {
                    break;
                }
                self.bump();
            }
        }
        self.expect(TokenKind::RParen, ")")?;
        Ok(arguments)
    }
    fn match_expression(&mut self, start: u32) -> Result<Node, DraftError> {
        self.bump();
        let scrutinee = self.expression(0)?;
        self.expect(TokenKind::LBrace, "{")?;
        let mut arms = Vec::new();
        let mut patterns = Vec::new();
        let mut children = vec![scrutinee];
        loop {
            let (pattern, span, binders) = self.pattern()?;
            self.expect(TokenKind::Eq, "=>")?;
            self.expect(TokenKind::Gt, "=>")?;
            let body = self.expression(0)?;
            arms.push(Arm {
                pattern,
                body: body.expression.clone(),
            });
            patterns.push((span, binders));
            children.push(body);
            if self.kind(0) != TokenKind::Comma {
                break;
            }
            self.bump();
        }
        self.expect(TokenKind::RBrace, "}")?;
        let mut node = Node::new(
            Expr::Match(children[0].expression.clone(), arms),
            SourceSpan {
                start,
                end: self.end,
            },
            children,
        );
        node.patterns = patterns;
        Ok(node)
    }
    fn pattern(&mut self) -> Result<(Pattern, SourceSpan, Vec<BinderSite>), DraftError> {
        let start = self.span().start;
        if self.word("nil") {
            self.bump();
            return Ok((
                Pattern::Nil,
                SourceSpan {
                    start,
                    end: self.end,
                },
                vec![],
            ));
        }
        if ["cons", "leaf", "branch"]
            .iter()
            .any(|word| self.word(word))
        {
            let constructor = self.text().to_owned();
            self.bump();
            self.expect(TokenKind::LParen, "(")?;
            let first = self.name()?;
            let mut binders = vec![first];
            if constructor != "leaf" {
                self.expect(TokenKind::Comma, ",")?;
                binders.push(self.name()?);
            }
            self.expect(TokenKind::RParen, ")")?;
            let pattern = match constructor.as_str() {
                "leaf" => Pattern::Leaf(binders[0].name.clone()),
                "cons" => Pattern::Cons(binders[0].name.clone(), binders[1].name.clone()),
                _ => Pattern::Branch(binders[0].name.clone(), binders[1].name.clone()),
            };
            Ok((
                pattern,
                SourceSpan {
                    start,
                    end: self.end,
                },
                binders,
            ))
        } else {
            let binder = self.name()?;
            Ok((Pattern::Var(binder.name.clone()), binder.span, vec![binder]))
        }
    }
}

fn reserved(name: &str) -> bool {
    matches!(
        name,
        "language"
            | "program"
            | "fn"
            | "entry"
            | "if"
            | "else"
            | "let"
            | "in"
            | "match"
            | "UInt"
            | "Bool"
            | "List"
            | "Tree"
            | "nil"
            | "cons"
            | "leaf"
            | "branch"
            | "not"
    )
}

fn collect_sites(root: &Node, function_index: Option<usize>, provenance: &mut SourceProvenance) {
    let mut todo = vec![(root, Vec::new())];
    while let Some((node, path)) = todo.pop() {
        let source = SourceLocation {
            function_index,
            child_path: path.clone(),
        };
        provenance.expressions.push(ExpressionSite {
            source: source.clone(),
            span: node.span,
            kind: node_kind(&node.expression),
        });
        for (arm_index, (span, binders)) in node.patterns.iter().enumerate() {
            provenance.patterns.push(PatternSite {
                match_source: source.clone(),
                arm_index,
                span: *span,
                binders: binders.clone(),
            });
        }
        for (index, child) in node.children.iter().enumerate().rev() {
            let mut child_path = path.clone();
            child_path.push(index);
            todo.push((child, child_path));
        }
    }
}

fn node_kind(expression: &Expr) -> &'static str {
    match expression {
        Expr::Literal(Value::UInt(_)) => "uint",
        Expr::Literal(Value::Bool(_)) => "bool",
        Expr::Literal(Value::List(_)) => "nil",
        Expr::Literal(Value::Tree(_)) => "tree-literal",
        Expr::Var(_) => "var",
        Expr::Let(..) => "let",
        Expr::If(..) => "if",
        Expr::Call(..) => "call",
        Expr::Match(..) => "match",
        Expr::Binary(op, ..) => match op {
            BinaryOp::Add => "add",
            BinaryOp::Sub => "sub",
            BinaryOp::Mul => "mul",
            BinaryOp::Eq => "eq",
            BinaryOp::Lt => "lt",
            BinaryOp::Cons => "cons",
            BinaryOp::Branch => "branch",
        },
        Expr::Unary(UnaryOp::Leaf, _) => "leaf",
        Expr::Unary(UnaryOp::Not, _) => "not",
    }
}
