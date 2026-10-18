//! Canonical number spelling.
//!
//! A number is an integer part, optionally followed by `.` and a fraction
//! part. The integer part is `0` or starts with `1`–`9`. In both parts `_` may
//! only separate two digits. This matches the reference spelling
//! `(0|[1-9][0-9]*(_[0-9]+)*)(\.[0-9]+(_[0-9]+)*)?`.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumberError {
    /// The integer part has more than one digit and starts with `0`.
    LeadingZero,
    /// A `_` at this byte offset is not between two digits.
    MisplacedUnderscore { at: usize },
    /// The `.` at the end has no digits after it.
    MissingFraction,
    /// A second `.` at this byte offset.
    ExtraDot { at: usize },
}

/// Checks the spelling of a number scanned by `scan::number_end`.
pub fn validate(raw: &str) -> Result<(), NumberError> {
    let mut dots = raw.match_indices('.').map(|(at, _)| at);
    let dot = dots.next();
    if let Some(at) = dots.next() {
        return Err(NumberError::ExtraDot { at });
    }

    let integer = &raw[..dot.unwrap_or(raw.len())];
    if integer.len() > 1 && integer.starts_with('0') {
        return Err(NumberError::LeadingZero);
    }
    check_underscores(integer, 0)?;

    if let Some(dot) = dot {
        let fraction = &raw[dot + 1..];
        if fraction.is_empty() {
            return Err(NumberError::MissingFraction);
        }
        check_underscores(fraction, dot + 1)?;
    }
    Ok(())
}

/// `part` contains only digits and `_` and starts at `offset` in the number.
fn check_underscores(part: &str, offset: usize) -> Result<(), NumberError> {
    let bytes = part.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        let between_digits = i > 0
            && bytes[i - 1].is_ascii_digit()
            && bytes.get(i + 1).is_some_and(u8::is_ascii_digit);
        if b == b'_' && !between_digits {
            return Err(NumberError::MisplacedUnderscore { at: offset + i });
        }
    }
    Ok(())
}

/// The number with redundant leading zeros removed, for suggestions.
pub fn without_leading_zeros(raw: &str) -> String {
    let (integer, rest) = raw.split_at(raw.find('.').unwrap_or(raw.len()));
    let trimmed = integer.trim_start_matches(['0', '_']);
    let integer = if trimmed.is_empty() { "0" } else { trimmed };
    format!("{integer}{rest}")
}
