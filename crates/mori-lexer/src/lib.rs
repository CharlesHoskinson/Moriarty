//! Lexer for `moriarty-beta/1` source.
//!
//! Turns source text into [`Tokens`] in a single pass.
//!
//! Lexing stops at the first error. Whitespace is skipped; comments are kept
//! separately for the formatter and do not count toward [`MAX_TOKENS`].

mod diagnostics;
mod number;
mod scan;
mod string;
mod token;

#[cfg(test)]
mod tests;

use mori_diagnostics::MoriDiagnostic;
use mori_span::Span;
use soa_rs::Soa;

use crate::number::NumberError;
use crate::string::StringErrorKind;

pub use crate::token::{Comment, Token, TokenIdx, TokenKind, Tokens};

/// Largest accepted source file, in bytes.
pub const MAX_SOURCE_BYTES: usize = 65_536;
/// Most tokens in one file, including the end-of-file token.
pub const MAX_TOKENS: usize = 8_192;
/// Longest identifier or keyword, in bytes.
pub const MAX_IDENT_BYTES: usize = 64;
/// Longest decoded string, in UTF-8 bytes.
pub const MAX_STRING_BYTES: usize = 1_024;

/// The text of a string token, with escapes decoded.
///
/// `raw` must be the full text of a [`TokenKind::String`] token, quotes
/// included, as produced by [`lex`].
pub fn decode_string(raw: &str) -> String {
    let mut text = String::with_capacity(raw.len());
    string::decode_with(raw, |c| text.push(c)).expect("lexed strings are valid");
    text
}

/// Splits `source` into tokens and comments, or reports the first error.
pub fn lex(source: &str) -> Result<Tokens, MoriDiagnostic> {
    if source.len() > MAX_SOURCE_BYTES {
        return Err(diagnostics::source_too_large(source.len()));
    }
    Lexer::new(source).run()
}

struct Lexer<'src> {
    source: &'src str,
    bytes: &'src [u8],
    pos: usize,
    tokens: Soa<Token>,
    comments: Soa<Comment>,
}

impl<'src> Lexer<'src> {
    fn new(source: &'src str) -> Self {
        Self {
            source,
            bytes: source.as_bytes(),
            pos: 0,
            // Roughly one token per eight bytes, as in Zig.
            tokens: Soa::with_capacity(source.len() / 8 + 1),
            comments: Soa::new(),
        }
    }

    fn run(mut self) -> Result<Tokens, MoriDiagnostic> {
        while let Some(&b) = self.bytes.get(self.pos) {
            let start = self.pos;
            match b {
                b' ' | b'\t' | b'\r' | b'\n' => self.pos += 1,
                b'/' if self.peek(1) == Some(b'/') => self.line_comment(start),
                b'/' if self.peek(1) == Some(b'*') => self.block_comment(start)?,
                b'"' => self.string(start)?,
                b'0'..=b'9' => self.number(start)?,
                b'A'..=b'Z' | b'a'..=b'z' => self.word(start)?,
                _ => match TokenKind::punctuation(b) {
                    Some(kind) => self.push(kind, start, start + 1)?,
                    None => return Err(self.unexpected_character(start)),
                },
            }
        }
        // The end-of-file token is always pushed: the bound reserves room for it.
        self.tokens.push(Token {
            kind: TokenKind::Eof,
            start: offset(self.bytes.len()),
        });
        Ok(Tokens {
            tokens: self.tokens,
            comments: self.comments,
        })
    }

    fn peek(&self, ahead: usize) -> Option<u8> {
        self.bytes.get(self.pos + ahead).copied()
    }

    /// Records a token ending at `end` and moves past it.
    fn push(&mut self, kind: TokenKind, start: usize, end: usize) -> Result<(), MoriDiagnostic> {
        if self.tokens.len() >= MAX_TOKENS - 1 {
            return Err(diagnostics::too_many_tokens(span(start, end)));
        }
        self.tokens.push(Token {
            kind,
            start: offset(start),
        });
        self.pos = end;
        Ok(())
    }

    fn push_comment(&mut self, start: usize, end: usize) {
        self.comments.push(Comment {
            start: offset(start),
            end: offset(end),
        });
        self.pos = end;
    }

    /// `//` up to, but not including, the next line break.
    fn line_comment(&mut self, start: usize) {
        let end = self.bytes[start..]
            .iter()
            .position(|&b| b == b'\r' || b == b'\n')
            .map_or(self.bytes.len(), |len| start + len);
        self.push_comment(start, end);
    }

    /// `/*` through the first `*/`. Block comments do not nest.
    fn block_comment(&mut self, start: usize) -> Result<(), MoriDiagnostic> {
        let body = start + 2;
        let Some(len) = self.source[body..].find("*/") else {
            return Err(diagnostics::unterminated_block_comment(span(start, body)));
        };
        self.push_comment(start, body + len + 2);
        Ok(())
    }

    fn string(&mut self, start: usize) -> Result<(), MoriDiagnostic> {
        let Some(end) = scan::string_end(self.bytes, start) else {
            return Err(diagnostics::unterminated_string(span(start, start + 1)));
        };
        let raw = &self.source[start..end];
        let mut decoded_len = 0;
        if let Err(error) = string::decode_with(raw, |c| decoded_len += c.len_utf8()) {
            let at = span(start + error.start, start + error.end);
            return Err(match error.kind {
                StringErrorKind::ControlCharacter('\r' | '\n') => {
                    diagnostics::string_line_break(at)
                }
                StringErrorKind::ControlCharacter(c) => {
                    diagnostics::string_control_character(at, c)
                }
                StringErrorKind::InvalidEscape => {
                    diagnostics::invalid_escape(at, at.source_text(self.source))
                }
                StringErrorKind::InvalidUnicodeEscape => diagnostics::invalid_unicode_escape(at),
                StringErrorKind::LoneSurrogate => diagnostics::lone_surrogate(at),
            });
        }
        if decoded_len > MAX_STRING_BYTES {
            return Err(diagnostics::string_too_long(span(start, end), decoded_len));
        }
        self.push(TokenKind::String, start, end)
    }

    fn number(&mut self, start: usize) -> Result<(), MoriDiagnostic> {
        let end = scan::number_end(self.bytes, start);
        let raw = &self.source[start..end];
        if let Err(error) = number::validate(raw) {
            return Err(match error {
                NumberError::LeadingZero => diagnostics::number_leading_zero(
                    span(start, end),
                    &number::without_leading_zeros(raw),
                ),
                NumberError::MisplacedUnderscore { at } => {
                    diagnostics::number_misplaced_underscore(span(start + at, start + at + 1))
                }
                NumberError::MissingFraction => {
                    diagnostics::number_missing_fraction(span(end - 1, end))
                }
                NumberError::ExtraDot { at } => {
                    diagnostics::number_extra_dot(span(start + at, start + at + 1))
                }
            });
        }
        self.push(TokenKind::Number, start, end)
    }

    /// An identifier, or a keyword when the text matches one.
    fn word(&mut self, start: usize) -> Result<(), MoriDiagnostic> {
        let end = scan::word_end(self.bytes, start);
        if end - start > MAX_IDENT_BYTES {
            return Err(diagnostics::identifier_too_long(span(start, end)));
        }
        let kind = TokenKind::keyword(&self.source[start..end]).unwrap_or(TokenKind::Ident);
        self.push(kind, start, end)
    }

    fn unexpected_character(&self, start: usize) -> MoriDiagnostic {
        let c = self.source[start..]
            .chars()
            .next()
            .expect("a character remains at the current position");
        let at = span(start, start + c.len_utf8());
        match c {
            '/' => diagnostics::division(at),
            '“' | '”' | '‘' | '’' => diagnostics::curly_quote(at),
            '\'' => diagnostics::single_quote(at),
            '_' => diagnostics::leading_underscore(at),
            c if !c.is_ascii() => diagnostics::non_ascii(at, c),
            c => diagnostics::unexpected_character(at, c),
        }
    }
}

/// Converts a byte index to an offset. Sources fit in `u32` after the bound check.
fn offset(index: usize) -> u32 {
    u32::try_from(index).expect("source offsets fit in u32")
}

fn span(start: usize, end: usize) -> Span {
    Span::new(offset(start), offset(end))
}
