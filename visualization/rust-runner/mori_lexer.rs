#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TokenIdx(usize);
impl TokenIdx {
    pub fn new(i: usize) -> Self {
        Self(i)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
    Eof,
    Ident,
    String,
    Number,
    KwTrue,
    KwFalse,
    Semicolon,
    LBrace,
    RBrace,
    LParen,
    RParen,
    Colon,
    Comma,
    Eq,
    Lt,
    Gt,
    Plus,
    Minus,
    Star,
    Other,
}

impl TokenKind {
    pub fn is_word(self) -> bool {
        matches!(self, TokenKind::Ident | TokenKind::KwTrue | TokenKind::KwFalse)
    }
}

#[derive(Clone, Debug)]
pub struct Tokens {
    kinds: Vec<TokenKind>,
    spans: Vec<Span>,
}

impl Tokens {
    pub fn len(&self) -> usize {
        self.kinds.len()
    }
    pub fn kind(&self, idx: TokenIdx) -> TokenKind {
        self.kinds.get(idx.0).copied().unwrap_or(TokenKind::Eof)
    }
    pub fn span(&self, idx: TokenIdx, _source: &str) -> Span {
        self.spans.get(idx.0).copied().unwrap_or(Span { start: 0, end: 0 })
    }
}

pub fn decode_string(raw: &str) -> String {
    if raw.len() >= 2 && raw.starts_with('"') && raw.ends_with('"') {
        raw[1..raw.len() - 1].to_string()
    } else {
        raw.to_string()
    }
}

pub fn lex(source: &str) -> Result<Tokens, String> {
    let bytes = source.as_bytes();
    let mut i = 0usize;
    let mut kinds = Vec::new();
    let mut spans = Vec::new();

    while i < bytes.len() {
        let b = bytes[i];
        if b.is_ascii_whitespace() {
            i += 1;
            continue;
        }
        if b == b'/' && i + 1 < bytes.len() && bytes[i + 1] == b'/' {
            i += 2;
            while i < bytes.len() && bytes[i] != b'\n' {
                i += 1;
            }
            continue;
        }
        let start = i as u32;
        if b == b'"' {
            i += 1;
            while i < bytes.len() && bytes[i] != b'"' {
                if bytes[i] == b'\\' && i + 1 < bytes.len() {
                    i += 2;
                } else {
                    i += 1;
                }
            }
            if i >= bytes.len() {
                return Err("unterminated string literal".to_string());
            }
            i += 1;
            kinds.push(TokenKind::String);
            spans.push(Span { start, end: i as u32 });
            continue;
        }
        if b.is_ascii_digit() {
            while i < bytes.len() && (bytes[i].is_ascii_digit() || bytes[i] == b'_') {
                i += 1;
            }
            kinds.push(TokenKind::Number);
            spans.push(Span { start, end: i as u32 });
            continue;
        }
        if b.is_ascii_alphabetic() || b == b'_' {
            while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                i += 1;
            }
            let text = &source[start as usize..i];
            let kind = match text {
                "true" => TokenKind::KwTrue,
                "false" => TokenKind::KwFalse,
                _ => TokenKind::Ident,
            };
            kinds.push(kind);
            spans.push(Span { start, end: i as u32 });
            continue;
        }
        i += 1;
        let kind = match b {
            b';' => TokenKind::Semicolon,
            b'{' => TokenKind::LBrace,
            b'}' => TokenKind::RBrace,
            b'(' => TokenKind::LParen,
            b')' => TokenKind::RParen,
            b':' => TokenKind::Colon,
            b',' => TokenKind::Comma,
            b'=' => TokenKind::Eq,
            b'<' => TokenKind::Lt,
            b'>' => TokenKind::Gt,
            b'+' => TokenKind::Plus,
            b'-' => TokenKind::Minus,
            b'*' => TokenKind::Star,
            _ => return Err(format!("unexpected character '{}'", b as char)),
        };
        kinds.push(kind);
        spans.push(Span { start, end: i as u32 });
    }
    let end = bytes.len() as u32;
    kinds.push(TokenKind::Eof);
    spans.push(Span { start: end, end });
    Ok(Tokens { kinds, spans })
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenLimitedLexError {
    Lex(String),
    TokenLimit { limit: usize, observed: usize },
}

pub fn lex_with_token_limit(source: &str, limit: usize) -> Result<Tokens, TokenLimitedLexError> {
    let tokens = lex(source).map_err(TokenLimitedLexError::Lex)?;
    let observed = tokens.len();
    if observed > limit {
        return Err(TokenLimitedLexError::TokenLimit { limit, observed });
    }
    Ok(tokens)
}

