"""MCJ/1: the canonical JSON profile used for every v1 catalogue, admission and bundle digest.

Supported value domain (anything else is refused with an MCJError code):
  - object with string keys (duplicate keys refused when parsing text)
  - array
  - string of Unicode scalar values (lone surrogates refused)
  - integer n with -(2^53-1) <= n <= 2^53-1 (Python bool is not an integer here)
  - true, false, null
Refused: every float or fraction/exponent number form (including 1.0 and -0.0), NaN/Infinity,
integers outside the range, non-string keys, tuples, bytes, other Python types, invalid UTF-8,
a UTF-8 BOM, trailing text, and (v1.1, review A7) nesting deeper than MAX_DEPTH containers
(MCJ_DEPTH), so malformed input is refused with a code instead of raising RecursionError.

Serialization rules:
  - no insignificant whitespace; separators ',' and ':'
  - object members sorted by the UTF-16 code-unit sequence of their keys
  - strings: '"' -> \\", '\\' -> \\\\, U+0008 \\b, U+0009 \\t, U+000A \\n, U+000C \\f,
    U+000D \\r, other U+0000..U+001F -> \\u00xx (lowercase hex); every other character literal
  - integers in shortest decimal form; output encoded as UTF-8

For values inside this domain the rules are intended to coincide with RFC 8785 (JCS). That
coincidence is [EXT]: the RFC text was not captured or inspected in this task, and no reviewed JCS
implementation is installed. MCJ/1 is therefore named as its own profile and makes no full-JCS claim.
Large integers that must exceed 2^53-1 (lovelace bounds, UInt128) are carried as decimal strings.
"""
import json

INT_LIMIT = 2 ** 53 - 1
MAX_DEPTH = 64


class MCJError(ValueError):
    def __init__(self, code, detail=""):
        super().__init__(f"{code}: {detail}" if detail else code)
        self.code = code
        self.detail = detail


def _check_str(s, where):
    for ch in s:
        if 0xD800 <= ord(ch) <= 0xDFFF:
            raise MCJError("MCJ_LONE_SURROGATE", where)


def check(value, where="$", depth=0):
    """Raise MCJError unless value lies in the MCJ/1 domain."""
    if value is None or value is True or value is False:
        return
    t = type(value)
    if t is int:
        if not -INT_LIMIT <= value <= INT_LIMIT:
            raise MCJError("MCJ_INT_RANGE", where)
        return
    if t is float:
        raise MCJError("MCJ_FLOAT", where)
    if t is str:
        _check_str(value, where)
        return
    if t in (list, dict) and depth >= MAX_DEPTH:
        raise MCJError("MCJ_DEPTH", where)
    if t is list:
        for i, v in enumerate(value):
            check(v, f"{where}[{i}]", depth + 1)
        return
    if t is dict:
        for k, v in value.items():
            if type(k) is not str:
                raise MCJError("MCJ_NON_STRING_KEY", where)
            _check_str(k, where + ".<key>")
            check(v, f"{where}.{k}", depth + 1)
        return
    raise MCJError("MCJ_UNSUPPORTED_TYPE", f"{where}: {t.__name__}")


_SHORT = {0x08: "\\b", 0x09: "\\t", 0x0A: "\\n", 0x0C: "\\f", 0x0D: "\\r", 0x22: '\\"', 0x5C: "\\\\"}


def _enc_str(s):
    out = ['"']
    for ch in s:
        o = ord(ch)
        if o in _SHORT:
            out.append(_SHORT[o])
        elif o < 0x20:
            out.append("\\u%04x" % o)
        else:
            out.append(ch)
    out.append('"')
    return "".join(out)


def _utf16_key(k):
    return k.encode("utf-16-be")


def _enc(v):
    if v is None:
        return "null"
    if v is True:
        return "true"
    if v is False:
        return "false"
    if type(v) is int:
        return str(v)
    if type(v) is str:
        return _enc_str(v)
    if type(v) is list:
        return "[" + ",".join(_enc(x) for x in v) + "]"
    keys = sorted(v.keys(), key=_utf16_key)
    return "{" + ",".join(_enc_str(k) + ":" + _enc(v[k]) for k in keys) + "}"


def canonical_bytes(value):
    """MCJ/1 bytes of value; raises MCJError outside the domain."""
    check(value)
    return _enc(value).encode("utf-8")


def _no_float(s):
    raise MCJError("MCJ_FLOAT", s)


def _int(s):
    n = int(s)
    if not -INT_LIMIT <= n <= INT_LIMIT:
        raise MCJError("MCJ_INT_RANGE", s)
    return n


def _nonfinite(s):
    raise MCJError("MCJ_NONFINITE", s)


def _pairs(pairs):
    d = {}
    for k, v in pairs:
        if k in d:
            raise MCJError("MCJ_DUPLICATE_KEY", k)
        d[k] = v
    return d


def parse(data):
    """Parse JSON text (bytes or str) into the MCJ/1 domain, refusing anything outside it."""
    if isinstance(data, (bytes, bytearray)):
        if bytes(data[:3]) == b"\xef\xbb\xbf":
            raise MCJError("MCJ_BOM")
        try:
            text = bytes(data).decode("utf-8", errors="strict")
        except UnicodeDecodeError as e:
            raise MCJError("MCJ_INVALID_UTF8", str(e)) from None
    else:
        text = data
        if text.startswith("﻿"):
            raise MCJError("MCJ_BOM")
    try:
        value = json.loads(text, parse_float=_no_float, parse_int=_int,
                           parse_constant=_nonfinite, object_pairs_hook=_pairs)
    except MCJError:
        raise
    except RecursionError:
        raise MCJError("MCJ_DEPTH", "nesting exceeds the parser limit") from None
    except (json.JSONDecodeError, ValueError) as e:
        raise MCJError("MCJ_SYNTAX", str(e)) from None
    check(value)
    return value
