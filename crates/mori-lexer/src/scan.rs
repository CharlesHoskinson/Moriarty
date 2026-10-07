//! Token extent rules, shared by the lexer and by token-end recovery.

use crate::TokenKind;

pub fn is_word_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

/// End of the identifier or keyword starting at `start`.
pub fn word_end(bytes: &[u8], start: usize) -> usize {
    start
        + 1
        + bytes[start + 1..]
            .iter()
            .take_while(|&&b| is_word_byte(b))
            .count()
}

/// End of the number starting at `start`. Numbers extend over digits, `_`
/// and `.`; spelling is validated separately.
pub fn number_end(bytes: &[u8], start: usize) -> usize {
    start
        + 1
        + bytes[start + 1..]
            .iter()
            .take_while(|&&b| b.is_ascii_digit() || b == b'_' || b == b'.')
            .count()
}

/// End of the string starting with `"` at `start`, just past the closing
/// quote, or `None` if it is unterminated. A backslash escapes the next byte.
pub fn string_end(bytes: &[u8], start: usize) -> Option<usize> {
    let mut escaped = false;
    for (offset, &b) in bytes[start + 1..].iter().enumerate() {
        if b == b'"' && !escaped {
            return Some(start + 1 + offset + 1);
        }
        escaped = b == b'\\' && !escaped;
    }
    None
}

/// Recovers the end offset of a token from its kind and start.
pub fn token_end(kind: TokenKind, bytes: &[u8], start: u32) -> u32 {
    let start_index = start as usize;
    let end = match kind {
        TokenKind::Ident => word_end(bytes, start_index),
        _ if kind.is_keyword() => word_end(bytes, start_index),
        TokenKind::Number => number_end(bytes, start_index),
        TokenKind::String => string_end(bytes, start_index).expect("lexed strings are terminated"),
        TokenKind::Eof => start_index,
        _ => start_index + 1,
    };
    end as u32
}
