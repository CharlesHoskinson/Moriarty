//// Byte, ordering and JSON-text helpers for ABI v2 (P3S1, PROPOSED, uncompiled).
////
//// Every encoder fails (halts) outside its declared range, so an out-of-range value can
//// never be silently truncated into a statement, digest or signing message.

use aiken/builtin
use aiken/collection/list
use aiken/primitive/bytearray
use aiken/primitive/int
use pvc2/types.{AssetIdV2}

pub const two_64: Int = 18446744073709551616

pub const two_128: Int = 340282366920938463463374607431768211456

pub const two_144: Int = 22300745198530623141535718272648361505980416

/// Cardano asset quantities are signed 64-bit; P3-1 bound for every physical quantity.
pub const two_63: Int = 9223372036854775808

pub const zero32: ByteArray =
  #"0000000000000000000000000000000000000000000000000000000000000000"

pub fn len(b: ByteArray) -> Int {
  builtin.length_of_bytearray(b)
}

pub fn cat(parts: List<ByteArray>) -> ByteArray {
  list.foldr(parts, #"", fn(p, acc) { builtin.append_bytearray(p, acc) })
}

pub fn u8(n: Int) -> ByteArray {
  expect n >= 0 && n < 256
  builtin.integer_to_bytearray(True, 1, n)
}

pub fn u16(n: Int) -> ByteArray {
  expect n >= 0 && n < 65536
  builtin.integer_to_bytearray(True, 2, n)
}

pub fn u32(n: Int) -> ByteArray {
  expect n >= 0 && n < 4294967296
  builtin.integer_to_bytearray(True, 4, n)
}

pub fn u64(n: Int) -> ByteArray {
  expect n >= 0 && n < two_64
  builtin.integer_to_bytearray(True, 8, n)
}

pub fn u128(n: Int) -> ByteArray {
  expect is_u128(n)
  builtin.integer_to_bytearray(True, 16, n)
}

/// 18-byte big-endian integer below 2^144 (accounting sums).
pub fn wide(n: Int) -> ByteArray {
  expect n >= 0 && n < two_144
  builtin.integer_to_bytearray(True, 18, n)
}

pub fn is_u128(n: Int) -> Bool {
  n >= 0 && n < two_128
}

/// u16 length prefix, then the bytes.
pub fn lp(b: ByteArray) -> ByteArray {
  builtin.append_bytearray(u16(len(b)), b)
}

pub fn bool_byte(b: Bool) -> ByteArray {
  if b {
    #"01"
  } else {
    #"00"
  }
}

pub fn pow10(e: Int) -> Int {
  expect e >= 0
  if e == 0 {
    1
  } else {
    10 * pow10(e - 1)
  }
}

// ## Ordering (Rust derived Ord on Strings is bytewise)

pub fn bytes_lt(a: ByteArray, b: ByteArray) -> Bool {
  builtin.less_than_bytearray(a, b)
}

/// Core `AssetId` order: (network, issuer, reference, scale).
pub fn asset_lt(a: AssetIdV2, b: AssetIdV2) -> Bool {
  if a.network != b.network {
    bytes_lt(a.network, b.network)
  } else if a.issuer != b.issuer {
    bytes_lt(a.issuer, b.issuer)
  } else if a.reference != b.reference {
    bytes_lt(a.reference, b.reference)
  } else {
    a.scale < b.scale
  }
}

pub fn strictly_ascending(xs: List<a>, lt: fn(a, a) -> Bool) -> Bool {
  when xs is {
    [] -> True
    [_] -> True
    [a, b, ..rest] -> lt(a, b) && strictly_ascending([b, ..rest], lt)
  }
}

pub fn ints_distinct(xs: List<Int>) -> Bool {
  strictly_ascending(list.sort(xs, int.compare), fn(a, b) { a < b })
}

// ## Text (JSON) helpers

/// The fail-closed JSON string subset: printable ASCII without `"` and `\`. Every
/// string S2 emits into a Core canonical JSON message must satisfy this, so no JSON
/// escaping is ever needed and the bytes equal serde_json's compact output.
pub fn json_safe(b: ByteArray) -> Bool {
  bytearray.foldl(
    b,
    True,
    fn(c, ok) { ok && c >= 0x20 && c <= 0x7e && c != 0x22 && c != 0x5c },
  )
}

/// Core `prepare_intent` string rule (non-empty, at most 1,024 bytes, no control
/// character), restricted to the safe subset.
pub fn core_string(b: ByteArray) -> Bool {
  let n = len(b)
  n > 0 && n <= 1024 && json_safe(b)
}

/// A JSON string literal; fails outside the safe subset.
pub fn jstr(b: ByteArray) -> ByteArray {
  expect json_safe(b)
  cat([#"22", b, #"22"])
}

fn hex_digit(n: Int) -> Int {
  if n < 10 {
    n + 48
  } else {
    n + 87
  }
}

/// Lowercase hexadecimal text of a byte string.
pub fn hex(b: ByteArray) -> ByteArray {
  bytearray.foldr(
    b,
    #"",
    fn(byte, acc) {
      builtin.cons_bytearray(
        hex_digit(byte / 16),
        builtin.cons_bytearray(hex_digit(byte % 16), acc),
      )
    },
  )
}

/// Canonical unsigned decimal text (no leading zeros).
pub fn decimal(n: Int) -> ByteArray {
  expect n >= 0
  if n < 10 {
    builtin.cons_bytearray(n + 48, #"")
  } else {
    builtin.append_bytearray(
      decimal(n / 10),
      builtin.cons_bytearray(n % 10 + 48, #""),
    )
  }
}

/// A JSON string holding a canonical decimal (Core `UInt` serialisation).
pub fn juint(n: Int) -> ByteArray {
  expect is_u128(n)
  cat([#"22", decimal(n), #"22"])
}

/// Joins JSON fragments with commas.
pub fn join(parts: List<ByteArray>) -> ByteArray {
  when parts is {
    [] -> #""
    [p] -> p
    [p, ..rest] -> cat([p, ",", join(rest)])
  }
}

pub fn jarray(parts: List<ByteArray>) -> ByteArray {
  cat(["[", join(parts), "]"])
}

/// An object from (key, fragment) pairs that the caller supplies in bytewise key order.
pub fn jobject(fields: List<(ByteArray, ByteArray)>) -> ByteArray {
  cat(["{", join(list.map(fields, fn(kv) { cat([jstr(kv.1st), ":", kv.2nd]) })), "}"])
}
