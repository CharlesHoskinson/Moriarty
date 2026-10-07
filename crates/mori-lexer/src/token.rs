use mori_span::Span;
use soa_rs::{Soa, Soars};

use crate::scan;

/// The kind of a token. Keywords get their own kinds so the parser compares
/// tags instead of text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TokenKind {
    /// A name that is not a keyword: `[A-Za-z][A-Za-z0-9_]*`, at most 64 bytes.
    Ident,
    /// A canonical decimal number, such as `1_000` or `10.50`.
    Number,
    /// A JSON string literal, such as `"Midnight"`.
    String,

    // Keywords stay contiguous from `KwProfile` to `KwShareClass`; see `is_keyword`.
    KwProfile,
    KwAgreement,
    KwAction,
    KwUses,
    KwTrue,
    KwFalse,
    KwNone,
    KwSuccessOnly,

    // Declaration kinds stay contiguous from `KwDomain` to `KwShareClass`; see
    // `is_declaration_kind`.
    KwDomain,
    KwAccount,
    KwAsset,
    KwConst,
    KwIntent,
    KwObligation,
    KwPool,
    KwInstrument,
    KwObservation,
    KwPolicy,
    KwGrant,
    KwStage,
    KwEpisode,
    KwParty,
    KwShareClass,

    LBrace,
    RBrace,
    LBracket,
    RBracket,
    LParen,
    RParen,
    Colon,
    Semicolon,
    Comma,
    Eq,
    Lt,
    Gt,
    Dot,
    Plus,
    Minus,
    Star,

    Eof,
}

impl TokenKind {
    /// The keyword spelled `text`, if any.
    pub fn keyword(text: &str) -> Option<Self> {
        let kind = match text {
            "profile" => Self::KwProfile,
            "agreement" => Self::KwAgreement,
            "action" => Self::KwAction,
            "uses" => Self::KwUses,
            "true" => Self::KwTrue,
            "false" => Self::KwFalse,
            "None" => Self::KwNone,
            "SuccessOnly" => Self::KwSuccessOnly,
            "domain" => Self::KwDomain,
            "account" => Self::KwAccount,
            "asset" => Self::KwAsset,
            "const" => Self::KwConst,
            "intent" => Self::KwIntent,
            "obligation" => Self::KwObligation,
            "pool" => Self::KwPool,
            "instrument" => Self::KwInstrument,
            "observation" => Self::KwObservation,
            "policy" => Self::KwPolicy,
            "grant" => Self::KwGrant,
            "stage" => Self::KwStage,
            "episode" => Self::KwEpisode,
            "party" => Self::KwParty,
            "share_class" => Self::KwShareClass,
            _ => return None,
        };
        Some(kind)
    }

    /// The punctuation token for byte `b`, if any.
    pub fn punctuation(b: u8) -> Option<Self> {
        let kind = match b {
            b'{' => Self::LBrace,
            b'}' => Self::RBrace,
            b'[' => Self::LBracket,
            b']' => Self::RBracket,
            b'(' => Self::LParen,
            b')' => Self::RParen,
            b':' => Self::Colon,
            b';' => Self::Semicolon,
            b',' => Self::Comma,
            b'=' => Self::Eq,
            b'<' => Self::Lt,
            b'>' => Self::Gt,
            b'.' => Self::Dot,
            b'+' => Self::Plus,
            b'-' => Self::Minus,
            b'*' => Self::Star,
            _ => return None,
        };
        Some(kind)
    }

    pub fn is_keyword(self) -> bool {
        (Self::KwProfile as u8..=Self::KwShareClass as u8).contains(&(self as u8))
    }

    /// Keywords that start a declaration, such as `const` or `asset`.
    pub fn is_declaration_kind(self) -> bool {
        (Self::KwDomain as u8..=Self::KwShareClass as u8).contains(&(self as u8))
    }

    /// Identifiers and keywords. Record fields and call names accept either.
    pub fn is_word(self) -> bool {
        self == Self::Ident || self.is_keyword()
    }
}

/// A token's kind and start offset. Its end is recovered by re-scanning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Soars)]
#[soa_derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub start: u32,
}

/// A `//` or `/* */` comment, kept for the formatter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Soars)]
#[soa_derive(Debug, PartialEq)]
pub struct Comment {
    pub start: u32,
    pub end: u32,
}

/// Index of a token in [`Tokens`]. Sources hold at most 8,192 tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TokenIdx(u16);

impl TokenIdx {
    pub fn new(index: usize) -> Self {
        Self(u16::try_from(index).expect("token index exceeds u16"))
    }

    pub fn index(self) -> usize {
        usize::from(self.0)
    }
}

/// The output of [`lex`](crate::lex): every token, ending with
/// [`TokenKind::Eof`], and every comment, both in source order.
#[derive(Debug, PartialEq)]
pub struct Tokens {
    pub tokens: Soa<Token>,
    pub comments: Soa<Comment>,
}

impl Tokens {
    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    /// Always false: a token list ends with [`TokenKind::Eof`].
    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }

    pub fn kind(&self, idx: TokenIdx) -> TokenKind {
        self.tokens.kind()[idx.index()]
    }

    pub fn start(&self, idx: TokenIdx) -> u32 {
        self.tokens.start()[idx.index()]
    }

    /// The token's span, recovered by re-scanning `source` from its start.
    pub fn span(&self, idx: TokenIdx, source: &str) -> Span {
        let start = self.start(idx);
        Span::new(
            start,
            scan::token_end(self.kind(idx), source.as_bytes(), start),
        )
    }

    pub fn comment_span(&self, index: usize) -> Span {
        Span::new(self.comments.start()[index], self.comments.end()[index])
    }
}
