//! Recursive-descent parser over the token array.
//!
//! The grammar, with `WORD` meaning an identifier or keyword:
//!
//! ```text
//! file      = "profile" STRING ";" "agreement" IDENT "{" { item } "}" EOF ;
//! item      = KIND IDENT [ ":" type ] "=" expr ";"
//!           | "action" IDENT "uses" IDENT ";" ;
//! type      = WORD [ "<" type { "," type } ">" ] ;
//! expr      = term { ( "+" | "-" ) term } ;
//! term      = primary { "*" primary } ;
//! primary   = STRING | NUMBER [ IDENT ] | "true" | "false" | "None"
//!           | "SuccessOnly" | "(" expr ")" | "[" [ expr { "," expr } [ "," ] ] "]"
//!           | "{" fields "}" | WORD { "." WORD } [ "(" fields ")" ] ;
//! fields    = [ WORD ":" expr { "," WORD ":" expr } [ "," ] ] ;
//! ```
//!
//! A dotted name must be a call. A number directly followed by an identifier
//! is a quantity; a decimal number must have one.

use mori_ast::{Node, NodeTag};
use mori_diagnostics::MoriDiagnostic;
use mori_lexer::{TokenIdx, TokenKind, Tokens};
use mori_span::Span;
use soa_rs::Soa;

use crate::diagnostics::{self, Context, Expected, Frame};
use crate::{MAX_DEPTH, MAX_FIELDS, MAX_ITEMS, PROFILE};

type Result<T> = std::result::Result<T, MoriDiagnostic>;

pub struct Parser<'a> {
    source: &'a str,
    tokens: &'a Tokens,
    pos: usize,
    nodes: Soa<Node>,
    extra: Vec<u16>,
    /// Children of lists still being parsed. Each list drains its own top.
    scratch: Vec<u16>,
    /// What is being parsed, innermost last, for error messages.
    frames: Vec<Frame>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str, tokens: &'a Tokens) -> Self {
        Self {
            source,
            tokens,
            pos: 0,
            nodes: Soa::with_capacity(tokens.len()),
            extra: Vec::new(),
            scratch: Vec::new(),
            frames: Vec::new(),
        }
    }

    pub fn run(mut self) -> Result<(Soa<Node>, Vec<u16>)> {
        // Reserve node 0 for the root; it is filled in once the items are known.
        self.push(NodeTag::Agreement, TokenIdx::new(0), 0, 0);

        let profile = self.current();
        self.frames.push(Frame::new(Context::Profile, profile));
        self.expect(TokenKind::KwProfile, "mori::syntax::missing_profile")?;
        self.profile_string()?;
        self.expect(TokenKind::Semicolon, "mori::syntax::missing_semicolon")?;
        self.frames.pop();

        let agreement = self.current();
        self.frames.push(Frame::new(Context::Agreement, agreement));
        self.expect(TokenKind::KwAgreement, "mori::syntax::missing_agreement")?;
        let name = self.name()?;
        self.expect(TokenKind::LBrace, "mori::syntax::missing_opening_brace")?;
        self.frames.pop();

        self.frames.push(Frame::new(Context::Body, name));
        let start = self.scratch.len();
        while self.eat(TokenKind::RBrace).is_none() {
            if self.scratch.len() - start >= MAX_ITEMS {
                return Err(diagnostics::too_many_items(self.current_span()));
            }
            let item = self.item()?;
            self.scratch.push(item);
        }
        let (lhs, rhs) = self.finish_list(start);
        self.frames.pop();

        self.nodes.main_token_mut()[0] = name;
        self.nodes.lhs_mut()[0] = lhs;
        self.nodes.rhs_mut()[0] = rhs;

        if self.peek() != TokenKind::Eof {
            return Err(diagnostics::trailing_input(self.current_span()));
        }
        Ok((self.nodes, self.extra))
    }

    fn profile_string(&mut self) -> Result<()> {
        if self.peek() != TokenKind::String {
            return Err(
                self.unexpected("mori::syntax::expected_profile", &[Expected::ProfileString])
            );
        }
        let token = self.bump();
        let span = self.span(token);
        let profile = mori_lexer::decode_string(span.source_text(self.source));
        if profile != PROFILE {
            return Err(diagnostics::wrong_profile(span, &profile));
        }
        Ok(())
    }

    fn item(&mut self) -> Result<u16> {
        let kind = self.peek();
        if kind == TokenKind::KwAction {
            self.action()
        } else if kind.is_declaration_kind() {
            self.declaration()
        } else {
            let span = self.current_span();
            Err(match kind {
                TokenKind::Eof => {
                    let last = self.span(TokenIdx::new(self.pos - 1));
                    let span = Span::empty(last.end);
                    let name = self.span(self.agreement_name());
                    diagnostics::unclosed_agreement(span, name, name.source_text(self.source))
                }
                TokenKind::Ident => diagnostics::unknown_item(
                    span,
                    Some(span.source_text(self.source)),
                    self.source,
                ),
                _ => diagnostics::unknown_item(span, None, self.source),
            })
        }
    }

    /// `KIND NAME [: TYPE] = VALUE ;`
    fn declaration(&mut self) -> Result<u16> {
        let kind = self.bump();
        self.frames.push(Frame::new(Context::Declaration, kind));
        self.name()?;
        let ty = match self.eat(TokenKind::Colon) {
            Some(_) => self.ty(1)?,
            None => 0,
        };
        self.expect(TokenKind::Eq, "mori::syntax::missing_equals")?;
        let value = self.expression(1, 0)?;
        self.expect(TokenKind::Semicolon, "mori::syntax::missing_semicolon")?;
        self.frames.pop();
        Ok(self.push(NodeTag::Declaration, kind, ty, value))
    }

    /// `action NAME uses INTENT ;`
    fn action(&mut self) -> Result<u16> {
        let action = self.bump();
        self.frames.push(Frame::new(Context::Action, action));
        self.name()?;
        self.expect(TokenKind::KwUses, "mori::syntax::missing_uses")?;
        self.name()?;
        self.expect(TokenKind::Semicolon, "mori::syntax::missing_semicolon")?;
        self.frames.pop();
        Ok(self.push(NodeTag::Action, action, 0, 0))
    }

    /// A declaration, action or agreement name: an identifier, not a keyword.
    fn name(&mut self) -> Result<TokenIdx> {
        match self.peek() {
            TokenKind::Ident => Ok(self.bump()),
            kind if kind.is_keyword() => {
                let span = self.current_span();
                Err(diagnostics::keyword_as_name(
                    span,
                    span.source_text(self.source),
                    self.source,
                ))
            }
            _ => Err(self.unexpected("mori::syntax::expected_name", &[Expected::Name])),
        }
    }

    /// `WORD [< TYPE, ... >]`
    fn ty(&mut self, depth: usize) -> Result<u16> {
        self.check_depth(depth)?;
        if !self.peek().is_word() {
            return Err(self.unexpected("mori::syntax::expected_type", &[Expected::Type]));
        }
        let name = self.bump();
        let start = self.scratch.len();
        if let Some(open) = self.eat(TokenKind::Lt) {
            self.frames.push(Frame::new(Context::Type, name));
            loop {
                if self.scratch.len() - start >= MAX_FIELDS {
                    return Err(diagnostics::too_many_type_arguments(self.span(open)));
                }
                let arg = self.ty(depth + 1)?;
                self.scratch.push(arg);
                if self.eat(TokenKind::Comma).is_none() {
                    break;
                }
            }
            self.expect(TokenKind::Gt, "mori::syntax::unclosed_type_arguments")?;
            self.frames.pop();
        }
        let (lhs, rhs) = self.finish_list(start);
        Ok(self.push(NodeTag::Type, name, lhs, rhs))
    }

    /// Binary operators by precedence climbing. All operators associate left.
    fn expression(&mut self, depth: usize, min_precedence: u8) -> Result<u16> {
        self.check_depth(depth)?;
        let mut lhs = self.primary(depth)?;
        loop {
            let (tag, precedence) = match self.peek() {
                TokenKind::Star => (NodeTag::Mul, 2),
                TokenKind::Plus => (NodeTag::Add, 1),
                TokenKind::Minus => (NodeTag::Sub, 1),
                _ => break,
            };
            if precedence < min_precedence {
                break;
            }
            let operator = self.bump();
            let rhs = self.expression(depth + 1, precedence + 1)?;
            lhs = self.push(tag, operator, lhs, rhs);
        }
        Ok(lhs)
    }

    fn primary(&mut self, depth: usize) -> Result<u16> {
        let kind = self.peek();
        match kind {
            TokenKind::String => Ok(self.leaf(NodeTag::String)),
            TokenKind::Number => self.number(),
            TokenKind::KwTrue | TokenKind::KwFalse => Ok(self.leaf(NodeTag::Bool)),
            TokenKind::KwNone | TokenKind::KwSuccessOnly => Ok(self.leaf(NodeTag::Tag)),
            TokenKind::LParen => {
                let open = self.bump();
                self.frames.push(Frame::new(Context::Paren, open));
                let inner = self.expression(depth + 1, 0)?;
                self.expect(TokenKind::RParen, "mori::syntax::unclosed_parenthesis")?;
                self.frames.pop();
                Ok(self.push(NodeTag::Paren, open, inner, 0))
            }
            TokenKind::LBracket => self.array(depth),
            TokenKind::LBrace => {
                let open = self.bump();
                self.frames.push(Frame::new(Context::Record, open));
                let (lhs, rhs) = self.fields(TokenKind::RBrace, depth)?;
                self.frames.pop();
                Ok(self.push(NodeTag::Record, open, lhs, rhs))
            }
            _ if kind.is_word() => self.name_or_call(depth),
            _ => Err(self.unexpected("mori::syntax::expected_value", &[Expected::Expression])),
        }
    }

    /// A number, or a quantity when an asset name follows.
    fn number(&mut self) -> Result<u16> {
        let number = self.bump();
        let number_span = self.span(number);
        if self.peek() == TokenKind::Ident {
            let asset = self.bump();
            let asset_span = self.span(asset);
            if asset_span.start == number_span.end {
                return Err(diagnostics::quantity_separator(
                    number_span,
                    asset_span,
                    self.source,
                ));
            }
            return Ok(self.push(NodeTag::Quantity, number, 0, 0));
        }
        if number_span.source_text(self.source).contains('.') {
            return Err(diagnostics::decimal_without_asset(number_span, self.source));
        }
        Ok(self.push(NodeTag::Number, number, 0, 0))
    }

    /// `[ ITEM, ... ]` with an optional trailing comma.
    fn array(&mut self, depth: usize) -> Result<u16> {
        let open = self.bump();
        self.frames.push(Frame::new(Context::Array, open));
        let start = self.scratch.len();
        if self.eat(TokenKind::RBracket).is_none() {
            loop {
                let item = self.expression(depth + 1, 0)?;
                self.scratch.push(item);
                if self.eat(TokenKind::RBracket).is_some() {
                    break;
                }
                if self.starts_expression() {
                    return Err(self.missing_comma("items"));
                }
                self.expect_one_of(
                    &[TokenKind::Comma, TokenKind::RBracket],
                    "mori::syntax::unclosed_list",
                )?;
                if self.eat(TokenKind::RBracket).is_some() {
                    break;
                }
            }
        }
        let (lhs, rhs) = self.finish_list(start);
        self.frames.pop();
        Ok(self.push(NodeTag::Array, open, lhs, rhs))
    }

    /// A reference, or a call when `(` follows. Dotted names must be calls.
    fn name_or_call(&mut self, depth: usize) -> Result<u16> {
        let first = self.bump();
        let mut last = first;
        while self.eat(TokenKind::Dot).is_some() {
            if !self.peek().is_word() {
                self.frames.push(Frame::new(Context::CallName, first));
                return Err(self.unexpected(
                    "mori::syntax::incomplete_call_name",
                    &[Expected::NameSegment],
                ));
            }
            last = self.bump();
        }
        if self.eat(TokenKind::LParen).is_some() {
            self.frames.push(Frame::new(Context::Call, first));
            let (lhs, rhs) = self.fields(TokenKind::RParen, depth)?;
            self.frames.pop();
            return Ok(self.push(NodeTag::Call, first, lhs, rhs));
        }
        if last != first {
            let span = Span::new(self.span(first).start, self.span(last).end);
            return Err(diagnostics::dotted_reference(
                span,
                span.source_text(self.source),
                self.source,
            ));
        }
        Ok(self.push(NodeTag::Reference, first, 0, 0))
    }

    /// `KEY: VALUE, ...` up to `close`, with an optional trailing comma.
    fn fields(&mut self, close: TokenKind, depth: usize) -> Result<(u16, u16)> {
        let start = self.scratch.len();
        if self.eat(close).is_none() {
            loop {
                if self.scratch.len() - start >= MAX_FIELDS {
                    return Err(diagnostics::too_many_fields(self.current_span()));
                }
                if !self.peek().is_word() {
                    return Err(self
                        .unexpected("mori::syntax::expected_field_name", &[Expected::FieldName]));
                }
                let key = self.bump();
                self.check_duplicate_field(start, key)?;
                self.expect(TokenKind::Colon, "mori::syntax::missing_colon")?;
                let value = self.expression(depth + 1, 0)?;
                let field = self.push(NodeTag::Field, key, value, 0);
                self.scratch.push(field);
                if self.eat(close).is_some() {
                    break;
                }
                if self.peek().is_word() && self.peek_at(1) == TokenKind::Colon {
                    let between = match close {
                        TokenKind::RBrace => "fields",
                        _ => "arguments",
                    };
                    return Err(self.missing_comma(between));
                }
                let unclosed = match close {
                    TokenKind::RBrace => "mori::syntax::unclosed_record",
                    _ => "mori::syntax::unclosed_call",
                };
                self.expect_one_of(&[TokenKind::Comma, close], unclosed)?;
                if self.eat(close).is_some() {
                    break;
                }
            }
        }
        Ok(self.finish_list(start))
    }

    fn check_duplicate_field(&self, start: usize, key: TokenIdx) -> Result<()> {
        let key_span = self.span(key);
        let text = key_span.source_text(self.source);
        let main_tokens = self.nodes.main_token();
        for &field in &self.scratch[start..] {
            let earlier = self.span(main_tokens[usize::from(field)]);
            if earlier.source_text(self.source) == text {
                return Err(diagnostics::duplicate_field(key_span, earlier, text));
            }
        }
        Ok(())
    }

    /// Whether the current token can begin an expression, so a list item
    /// was probably meant to follow.
    fn starts_expression(&self) -> bool {
        let kind = self.peek();
        kind.is_word()
            || matches!(
                kind,
                TokenKind::String
                    | TokenKind::Number
                    | TokenKind::LParen
                    | TokenKind::LBracket
                    | TokenKind::LBrace
            )
    }

    /// Another item follows without a separating comma.
    fn missing_comma(&self, between: &str) -> MoriDiagnostic {
        let previous = self.span(TokenIdx::new(self.pos - 1));
        diagnostics::missing_comma(Span::empty(previous.end), between, self.source)
    }

    fn check_depth(&self, depth: usize) -> Result<()> {
        if depth > MAX_DEPTH {
            return Err(diagnostics::too_deep(self.current_span()));
        }
        Ok(())
    }

    /// The agreement name, recorded on the body frame.
    fn agreement_name(&self) -> TokenIdx {
        self.frames
            .iter()
            .rev()
            .find(|frame| frame.context == Context::Body)
            .expect("items are parsed inside the agreement body")
            .start
    }

    // Token access

    fn current(&self) -> TokenIdx {
        TokenIdx::new(self.pos)
    }

    fn peek(&self) -> TokenKind {
        self.tokens.kind(self.current())
    }

    /// The kind `ahead` tokens after the current one, or end of file.
    fn peek_at(&self, ahead: usize) -> TokenKind {
        let index = (self.pos + ahead).min(self.tokens.len() - 1);
        self.tokens.kind(TokenIdx::new(index))
    }

    fn span(&self, token: TokenIdx) -> Span {
        self.tokens.span(token, self.source)
    }

    fn current_span(&self) -> Span {
        self.span(self.current())
    }

    /// Consumes the current token. The end-of-file token is never consumed.
    fn bump(&mut self) -> TokenIdx {
        let token = self.current();
        debug_assert_ne!(self.peek(), TokenKind::Eof);
        self.pos += 1;
        token
    }

    fn eat(&mut self, kind: TokenKind) -> Option<TokenIdx> {
        (self.peek() == kind).then(|| self.bump())
    }

    /// Consumes `kind`, or reports `code`.
    fn expect(&mut self, kind: TokenKind, code: &'static str) -> Result<TokenIdx> {
        self.expect_one_of(&[kind], code)
    }

    /// Consumes `expected[0]`, or reports `code`. The other kinds only
    /// describe what else was valid here.
    fn expect_one_of(&mut self, expected: &[TokenKind], code: &'static str) -> Result<TokenIdx> {
        match self.eat(expected[0]) {
            Some(token) => Ok(token),
            None => {
                let expected: Vec<Expected> =
                    expected.iter().map(|&k| Expected::Token(k)).collect();
                Err(self.unexpected(code, &expected))
            }
        }
    }

    fn unexpected(&self, code: &'static str, expected: &[Expected]) -> MoriDiagnostic {
        let found = self.current();
        let previous_end = self
            .pos
            .checked_sub(1)
            .map(|previous| self.span(TokenIdx::new(previous)).end);
        diagnostics::unexpected(diagnostics::Unexpected {
            code,
            source: self.source,
            tokens: self.tokens,
            found,
            previous_end,
            expected,
            frames: &self.frames,
        })
    }

    // Node building

    fn push(&mut self, tag: NodeTag, main_token: TokenIdx, lhs: u16, rhs: u16) -> u16 {
        let index = u16::try_from(self.nodes.len()).expect("nodes never outnumber tokens");
        self.nodes.push(Node {
            tag,
            main_token,
            lhs,
            rhs,
        });
        index
    }

    fn leaf(&mut self, tag: NodeTag) -> u16 {
        let token = self.bump();
        self.push(tag, token, 0, 0)
    }

    /// Moves the scratch entries above `start` into `extra`, returning their range.
    fn finish_list(&mut self, start: usize) -> (u16, u16) {
        let from = self.extra.len();
        self.extra.extend(self.scratch.drain(start..));
        let to = self.extra.len();
        (extra_index(from), extra_index(to))
    }
}

fn extra_index(index: usize) -> u16 {
    u16::try_from(index).expect("extra entries never outnumber nodes")
}
