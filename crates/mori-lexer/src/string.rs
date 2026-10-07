//! JSON string literals.
//!
//! Strings use JSON syntax: no raw control characters, and only the escapes
//! `\" \\ \/ \b \f \n \r \t \uXXXX`. A `\u` escape that encodes half of a
//! surrogate pair without its other half is rejected, but only after the
//! whole literal is valid JSON, matching the reference implementation.

use std::iter::Peekable;
use std::str::CharIndices;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringErrorKind {
    /// A raw character below U+0020, such as a line break.
    ControlCharacter(char),
    /// A backslash followed by a character that is not a JSON escape.
    InvalidEscape,
    /// `\u` not followed by four hexadecimal digits.
    InvalidUnicodeEscape,
    /// A `\u` escape for half of a surrogate pair, without its other half.
    LoneSurrogate,
}

/// An error at byte range `start..end` of the literal, quotes included.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringError {
    pub kind: StringErrorKind,
    pub start: usize,
    pub end: usize,
}

/// Decodes the literal `raw`, quotes included, passing each character to
/// `emit`. `raw` must start and end with an unescaped `"`.
pub fn decode_with(raw: &str, mut emit: impl FnMut(char)) -> Result<(), StringError> {
    let body = &raw[1..raw.len() - 1];
    let mut chars = body.char_indices().peekable();
    let mut pending_high: Option<(u32, usize)> = None;
    let mut first_lone: Option<(usize, usize)> = None;

    let error = |kind, start: usize, end: usize| StringError {
        kind,
        start: start + 1,
        end: end + 1,
    };

    while let Some((i, c)) = chars.next() {
        if c == '\\' {
            let Some((_, escape)) = chars.next() else {
                return Err(error(StringErrorKind::InvalidEscape, i, i + 1));
            };
            if escape == 'u' {
                let unit = hex4(&mut chars).ok_or_else(|| {
                    let end = chars.peek().map_or(body.len(), |&(j, _)| j);
                    error(StringErrorKind::InvalidUnicodeEscape, i, end)
                })?;
                let end = i + 6;
                match (pending_high.take(), unit) {
                    (Some((high, _)), 0xDC00..=0xDFFF) => {
                        let scalar = 0x10000 + ((high - 0xD800) << 10) + (unit - 0xDC00);
                        emit(char::from_u32(scalar).expect("surrogate pair is a scalar"));
                    }
                    (pending, _) => {
                        if let Some((_, start)) = pending {
                            first_lone.get_or_insert((start, start + 6));
                            emit(char::REPLACEMENT_CHARACTER);
                        }
                        match unit {
                            0xD800..=0xDBFF => pending_high = Some((unit, i)),
                            0xDC00..=0xDFFF => {
                                first_lone.get_or_insert((i, end));
                                emit(char::REPLACEMENT_CHARACTER);
                            }
                            _ => emit(char::from_u32(unit).expect("non-surrogate is a scalar")),
                        }
                    }
                }
                continue;
            }

            let decoded = match escape {
                '"' => '"',
                '\\' => '\\',
                '/' => '/',
                'b' => '\u{8}',
                'f' => '\u{c}',
                'n' => '\n',
                'r' => '\r',
                't' => '\t',
                _ => {
                    let end = i + 1 + escape.len_utf8();
                    return Err(error(StringErrorKind::InvalidEscape, i, end));
                }
            };
            flush_high(&mut pending_high, &mut first_lone, &mut emit);
            emit(decoded);
        } else if c < ' ' {
            return Err(error(StringErrorKind::ControlCharacter(c), i, i + 1));
        } else {
            flush_high(&mut pending_high, &mut first_lone, &mut emit);
            emit(c);
        }
    }
    flush_high(&mut pending_high, &mut first_lone, &mut emit);

    match first_lone {
        Some((start, end)) => Err(error(StringErrorKind::LoneSurrogate, start, end)),
        None => Ok(()),
    }
}

/// A high surrogate not followed by a low surrogate escape is lone.
fn flush_high(
    pending_high: &mut Option<(u32, usize)>,
    first_lone: &mut Option<(usize, usize)>,
    emit: &mut impl FnMut(char),
) {
    if let Some((_, start)) = pending_high.take() {
        first_lone.get_or_insert((start, start + 6));
        emit(char::REPLACEMENT_CHARACTER);
    }
}

/// Reads exactly four hexadecimal digits.
fn hex4(chars: &mut Peekable<CharIndices<'_>>) -> Option<u32> {
    let mut value = 0;
    for _ in 0..4 {
        let digit = chars.peek()?.1.to_digit(16)?;
        chars.next();
        value = value * 16 + digit;
    }
    Some(value)
}
