//! Strict lowercase hexadecimal and canonical decimal helpers.
use crate::error::DriverError;

/// Lowercase hex through Core's encoder (the same one every compiler digest uses).
pub fn encode(bytes: &[u8]) -> String {
    mori_core::intent::hex(bytes)
}

fn nibble(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        _ => None,
    }
}

/// Decodes lowercase hex of at most `max_bytes` bytes. Uppercase, odd length, whitespace or a
/// `0x` prefix is refused.
pub fn decode_bounded(text: &str, max_bytes: usize, what: &str) -> Result<Vec<u8>, DriverError> {
    let raw = text.as_bytes();
    if raw.len() % 2 != 0 {
        return Err(DriverError::Input(format!("{what}: hex has odd length {}", raw.len())));
    }
    if raw.len() / 2 > max_bytes {
        return Err(DriverError::Input(format!("{what}: {} bytes exceed the {max_bytes}-byte bound", raw.len() / 2)));
    }
    let mut out = Vec::with_capacity(raw.len() / 2);
    for pair in raw.chunks(2) {
        match (nibble(pair[0]), nibble(pair[1])) {
            (Some(hi), Some(lo)) => out.push((hi << 4) | lo),
            _ => return Err(DriverError::Input(format!("{what}: hex must be lowercase [0-9a-f]"))),
        }
    }
    Ok(out)
}

/// Decodes exactly `N` bytes of lowercase hex.
pub fn decode_exact<const N: usize>(text: &str, what: &str) -> Result<[u8; N], DriverError> {
    if text.len() != 2 * N {
        return Err(DriverError::Input(format!("{what}: expected {} hex characters, got {}", 2 * N, text.len())));
    }
    let bytes = decode_bounded(text, N, what)?;
    bytes.try_into().map_err(|_| DriverError::Input(format!("{what}: expected {N} bytes")))
}

/// Canonical unsigned decimal of a `u128`: digits only, no sign, no leading zero except "0".
pub fn parse_u128_decimal(text: &str, what: &str) -> Result<u128, DriverError> {
    let ok = !text.is_empty()
        && text.len() <= 39
        && text.bytes().all(|b| b.is_ascii_digit())
        && !(text.len() > 1 && text.starts_with('0'));
    if !ok {
        return Err(DriverError::Input(format!("{what}: expected a canonical unsigned decimal")));
    }
    text.parse::<u128>().map_err(|_| DriverError::Input(format!("{what}: decimal exceeds u128")))
}
