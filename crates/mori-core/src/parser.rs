use crate::*;
use mori_lexer::{TokenIdx, TokenKind};

/// Parses the explicitly versioned core profile; beta grammar is untouched.
pub fn parse(source: &str) -> Result<Program, Error> {
    parse_profile(source, false)
}

/// Syntax-only parser; only stage::load admits the resulting source.
pub(crate) fn parse_v2(source: &str) -> Result<Program, Error> {
    parse_profile(source, true)
}

fn parse_profile(source: &str, staged: bool) -> Result<Program, Error> {
    let lexed = mori_lexer::lex(source).map_err(|diagnostic| {
        let mut error = Error::new("lex", diagnostic.to_string());
        // Lexer label positions are u32 UTF-8 byte offsets under its source bound.
        error.offset = diagnostic.first_label_offset().map(|offset| offset as u32);
        error
    })?;
    let tokens = (0..lexed.len())
        .map(|i| {
            let idx = TokenIdx::new(i);
            let span = lexed.span(idx, source);
            let raw = &source[span.start as usize..span.end as usize];
            let text = if lexed.kind(idx) == TokenKind::String {
                mori_lexer::decode_string(raw)
            } else {
                raw.to_owned()
            };
            Tok {
                text,
                kind: lexed.kind(idx),
                start: span.start,
            }
        })
        .collect();
    let mut parser = Parser {
        tokens,
        pos: 0,
        program: Program {
            language: if staged { crate::stage::LANGUAGE_2 } else { LANGUAGE }.into(),
            semantics: if staged { crate::stage::SEMANTICS_2 } else { SEMANTICS }.into(),
            name: String::new(),
            states: vec![],
            assets: vec![],
            expressions: vec![],
            actions: vec![],
        },
    };
    parser.expect("language")?;
    let profile = parser.string()?;
    let expected_language = if staged { crate::stage::LANGUAGE_2 } else { LANGUAGE };
    if profile != expected_language {
        return Err(parser.error(
            "profile",
            format!("Expected language \"{expected_language}\"; found {profile:?}."),
        ));
    }
    parser.expect(";")?;
    parser.expect("agreement")?;
    parser.program.name = parser.word()?;
    parser.expect("{")?;
    while !parser.eat("}") {
        if parser.eat("state") {
            let name = parser.word()?;
            parser.expect(":")?;
            let ty = parser.ty()?;
            parser.expect("=")?;
            let initial = parser.literal()?;
            parser.expect(";")?;
            parser.program.states.push(StateDecl { name, ty, initial });
        } else if parser.eat("asset") {
            let name = parser.word()?;
            parser.expect("=")?;
            parser.expect("{")?;
            parser.expect("network")?;
            parser.expect(":")?;
            let network = parser.string()?;
            parser.expect(",")?;
            parser.expect("issuer")?;
            parser.expect(":")?;
            let issuer = parser.string()?;
            parser.expect(",")?;
            parser.expect("reference")?;
            parser.expect(":")?;
            let reference = parser.string()?;
            parser.expect(",")?;
            parser.expect("scale")?;
            parser.expect(":")?;
            let Value::UInt(UInt(scale)) = parser.literal()? else {
                return Err(parser.error("asset", "Asset scale must be an unsigned integer."));
            };
            let scale = u8::try_from(scale)
                .map_err(|_| parser.error("asset", "Asset scale must fit u8."))?;
            parser.eat(",");
            parser.expect("}")?;
            parser.expect(";")?;
            parser.program.assets.push(AssetDecl {
                name,
                identity: AssetId {
                    network,
                    issuer,
                    reference,
                    scale,
                },
            });
        } else if parser.eat("action") {
            let name = parser.word()?;
            parser.expect("(")?;
            let mut inputs = vec![];
            if !parser.eat(")") {
                loop {
                    let name = parser.word()?;
                    parser.expect(":")?;
                    let ty = parser.ty()?;
                    inputs.push(InputDecl { name, ty });
                    if parser.eat(")") {
                        break;
                    }
                    parser.expect(",")?;
                }
            }
            parser.expect("{")?;
            let mut statements = vec![];
            while !parser.eat("}") {
                if parser.eat("input") {
                    if !statements.is_empty() {
                        return Err(
                            parser.error("input", "Declare action inputs before statements.")
                        );
                    }
                    let name = parser.word()?;
                    parser.expect(":")?;
                    let ty = parser.ty()?;
                    parser.expect(";")?;
                    inputs.push(InputDecl { name, ty });
                    continue;
                }
                statements.push(parser.statement()?);
                if statements.len() > MAX_STATEMENTS {
                    return Err(parser.error("bound", "Action exceeds 1,024 statements."));
                }
            }
            parser.program.actions.push(Action {
                name,
                inputs,
                statements,
            });
        } else {
            return Err(parser.error("unsupported", "Expected state, asset or action declaration; loops and native calls are unsupported."));
        }
        if parser.program.states.len() + parser.program.assets.len() + parser.program.actions.len()
            > MAX_DECLARATIONS
        {
            return Err(parser.error("bound", "Program exceeds 256 declarations."));
        }
    }
    if parser.peek().kind != TokenKind::Eof {
        return Err(parser.error("syntax", "Unexpected content after agreement."));
    }
    if !staged {
        validate(&parser.program)?;
    }
    Ok(parser.program)
}
struct Tok {
    text: String,
    kind: TokenKind,
    start: u32,
}
struct Parser {
    tokens: Vec<Tok>,
    pos: usize,
    program: Program,
}
impl Parser {
    fn peek(&self) -> &Tok {
        &self.tokens[self.pos]
    }
    fn error(&self, code: &str, message: impl Into<String>) -> Error {
        let mut e = Error::new(code, message);
        e.offset = Some(self.peek().start);
        e
    }
    fn eat(&mut self, text: &str) -> bool {
        if self.peek().text == text && self.peek().kind != TokenKind::String {
            self.pos += 1;
            true
        } else {
            false
        }
    }
    fn expect(&mut self, text: &str) -> Result<(), Error> {
        if self.eat(text) {
            Ok(())
        } else {
            Err(self.error(
                "syntax",
                format!("Expected {text:?}; found {:?}.", self.peek().text),
            ))
        }
    }
    fn word(&mut self) -> Result<String, Error> {
        if self.peek().kind.is_word() {
            let text = self.peek().text.clone();
            self.pos += 1;
            Ok(text)
        } else {
            Err(self.error("syntax", "Expected a name."))
        }
    }
    fn string(&mut self) -> Result<String, Error> {
        if self.peek().kind == TokenKind::String {
            let text = self.peek().text.clone();
            self.pos += 1;
            Ok(text)
        } else {
            Err(self.error("syntax", "Expected a quoted string."))
        }
    }
    fn ty(&mut self) -> Result<Type, Error> {
        match self.word()?.as_str() {
            "UInt" => Ok(Type::UInt),
            "Bool" => Ok(Type::Bool),
            _ => Err(self.error("type", "Supported types are UInt and Bool.")),
        }
    }
    fn literal(&mut self) -> Result<Value, Error> {
        if self.eat("true") {
            return Ok(Value::Bool(true));
        }
        if self.eat("false") {
            return Ok(Value::Bool(false));
        }
        if self.peek().kind != TokenKind::Number {
            return Err(self.error("syntax", "Expected an unsigned integer or boolean literal."));
        }
        let raw = self.peek().text.replace('_', "");
        let n = raw.parse::<u128>().map_err(|_| {
            self.error(
                "number",
                "UInt literal must fit u128; amounts use integral atomic units.",
            )
        })?;
        self.pos += 1;
        Ok(Value::UInt(UInt(n)))
    }
    fn push(&mut self, expr: Expr) -> Result<ExprId, Error> {
        if self.program.expressions.len() >= MAX_EXPRESSIONS {
            return Err(self.error("bound", "Program exceeds 8,192 expression nodes."));
        }
        let id = self.program.expressions.len() as u16;
        self.program.expressions.push(expr);
        Ok(id)
    }
    fn expr(&mut self, minimum: u8, depth: usize) -> Result<ExprId, Error> {
        if depth > 64 {
            return Err(self.error("bound", "Expression nesting exceeds 64."));
        }
        let mut lhs = if self.eat("(") {
            let e = self.expr(0, depth + 1)?;
            self.expect(")")?;
            e
        } else if self.eat("not") {
            let value = self.expr(6, depth + 1)?;
            self.push(Expr::Not { value })?
        } else if self.peek().kind == TokenKind::Number
            || self.peek().text == "true"
            || self.peek().text == "false"
        {
            let value = self.literal()?;
            self.push(Expr::Literal { value })?
        } else {
            let name = self.word()?;
            if matches!(name.as_str(), "liability_present" | "liability_amount") && self.eat("(") {
                let duty = self.string()?;
                self.expect(")")?;
                self.push(if name == "liability_present" {
                    Expr::LiabilityPresent { name: duty }
                } else {
                    Expr::LiabilityAmount { name: duty }
                })?
            } else {
                self.push(Expr::Name { name })?
            }
        };
        loop {
            // A decoded string spelling an operator is still a string token.
            if self.peek().kind == TokenKind::String {
                break;
            }
            let next = self
                .tokens
                .get(self.pos + 1)
                .filter(|t| t.kind != TokenKind::String)
                .map(|t| t.text.as_str());
            let (op, precedence, length) = match (self.peek().text.as_str(), next) {
                ("or", _) => (BinaryOp::Or, 1, 1),
                ("and", _) => (BinaryOp::And, 2, 1),
                ("=", Some("=")) => (BinaryOp::Eq, 3, 2),
                ("not", Some("=")) => (BinaryOp::Ne, 3, 2),
                ("<", Some("=")) => (BinaryOp::Le, 3, 2),
                (">", Some("=")) => (BinaryOp::Ge, 3, 2),
                ("<", _) => (BinaryOp::Lt, 3, 1),
                (">", _) => (BinaryOp::Gt, 3, 1),
                ("+", _) => (BinaryOp::Add, 4, 1),
                ("-", _) => (BinaryOp::Sub, 4, 1),
                ("*", _) => (BinaryOp::Mul, 5, 1),
                _ => break,
            };
            if precedence < minimum {
                break;
            }
            self.pos += length;
            let rhs = self.expr(precedence + 1, depth + 1)?;
            lhs = self.push(Expr::Binary { op, lhs, rhs })?;
        }
        Ok(lhs)
    }
    fn statement(&mut self) -> Result<Statement, Error> {
        let keyword = self.word()?;
        let s=match keyword.as_str() {
            "await" if self.program.language == crate::stage::LANGUAGE_2 => {
                let label = self.word()?;
                self.expect("(")?;
                let mut binders = Vec::new();
                if !self.eat(")") {
                    loop {
                        let name = self.word()?;
                        self.expect(":")?;
                        let ty = self.ty()?;
                        binders.push(InputDecl { name, ty });
                        if self.eat(")") { break; }
                        self.expect(",")?;
                    }
                }
                self.expect("when")?;
                Statement::Await { label, binders, guard: self.expr(0, 0)? }
            },
            "let"=>{let name=self.word()?;self.expect("=")?;Statement::Let{name,value:self.expr(0,0)?}},
            "next"=>{let state=self.word()?;self.expect("=")?;Statement::Next{state,value:self.expr(0,0)?}},
            "requires"=>Statement::Requires{predicate:self.expr(0,0)?},
            "ensures"=>Statement::Ensures{predicate:self.expr(0,0)?},
            "emit"=>{let name=self.string()?;self.expect("(")?;let value=self.expr(0,0)?;self.expect(")")?;Statement::Emit{name,value}},
            "transfer"|"fee"=>{self.expect("(")?;let asset=self.word()?;self.expect(",")?;let from=self.string()?;self.expect(",")?;let to=self.string()?;self.expect(",")?;let amount=self.expr(0,0)?;self.expect(")")?;if keyword=="fee"{Statement::Fee{asset,from,to,amount}}else{Statement::Transfer{asset,from,to,amount}}},
            "liability"=>{self.expect("(")?;let name=self.string()?;self.expect(",")?;let asset=self.word()?;self.expect(",")?;let debtor=self.string()?;self.expect(",")?;let creditor=self.string()?;self.expect(",")?;let amount=self.expr(0,0)?;self.expect(")")?;Statement::Liability{name,asset,debtor,creditor,amount}},
            "discharge"=>{self.expect("(")?;let name=self.string()?;self.expect(",")?;let amount=self.expr(0,0)?;self.expect(")")?;Statement::Discharge{name,amount}},
            _=>return Err(self.error("unsupported",format!("Unsupported statement {keyword:?}; use let, requires, ensures, next, emit, transfer, fee, liability or discharge."))),
        };
        self.expect(";")?;
        Ok(s)
    }
}

/// Read the explicit source profile for CLI routing; this does not admit a program.
pub fn language(source: &str) -> Result<&'static str, Error> {
    let lexed = mori_lexer::lex(source).map_err(|diagnostic| {
        let mut error = Error::new("lex", diagnostic.to_string());
        // Lexer label positions are u32 UTF-8 byte offsets under its source bound.
        error.offset = diagnostic.first_label_offset().map(|offset| offset as u32);
        error
    })?;
    let mut tokens = Vec::new();
    for i in 0..lexed.len() {
        let idx = TokenIdx::new(i);
        let span = lexed.span(idx, source);
        let raw = &source[span.start as usize..span.end as usize];
        let kind = lexed.kind(idx);
        tokens.push(Tok {
            text: if kind == TokenKind::String { mori_lexer::decode_string(raw) } else { raw.to_owned() },
            kind,
            start: span.start,
        });
    }
    let mut parser = Parser { tokens, pos: 0, program: Program {
        language: LANGUAGE.into(), semantics: SEMANTICS.into(), name: String::new(),
        states: vec![], assets: vec![], expressions: vec![], actions: vec![],
    }};
    parser.expect("language")?;
    let profile = parser.string()?;
    match profile.as_str() {
        LANGUAGE => Ok(LANGUAGE),
        crate::stage::LANGUAGE_2 => Ok(crate::stage::LANGUAGE_2),
        _ => Err(parser.error("profile", format!("Expected language \"{LANGUAGE}\"; found {profile:?}."))),
    }
}
