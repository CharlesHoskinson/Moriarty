"""Canonical value typing shared by the catalogue (pinned constant values), AdmissionRecords (circuit_constants)
and bundles (statements). One encoding per value (review A13): decimal strings have no sign on zero and no
leading zeros, Bytes are non-empty lowercase hex, so one value cannot have two statement encodings.

v1.2 (review fdecbe41 B2): every validator uses re.fullmatch over ASCII-only classes. Python's '$' also matches
before a final newline, and '\\d' matches non-ASCII digits, so v1.1's match-plus-'$' accepted "1000000\\n" and
"33..33\\n". Decimal strings are length-bounded so int() never meets CPython's 4300-digit conversion limit
(review B3 class; INF-1 in adapters/AUTH-PACK-v12.json)."""
import re

DEC_MAX_DIGITS = 78  # > 2^256; statement Int bounds in the catalogue are at most 2^63-1 (19 digits)
DEC = re.compile(r"0|-?[1-9][0-9]{0,%d}" % (DEC_MAX_DIGITS - 1), re.ASCII)
HEX = re.compile(r"(?:[0-9a-f]{2})+", re.ASCII)
HEX56 = re.compile(r"[0-9a-f]{56}", re.ASCII)
HEX64 = re.compile(r"[0-9a-f]{64}", re.ASCII)
# versioned predicate id, as in policy grants (falsifier futureSchema grantTyping.predicate_refs)
VID = re.compile(r"MCP-[A-Z]+-[0-9]{2}@(?:0|[1-9][0-9]*)", re.ASCII)
MAX_BYTES_HEX = 2048


def is_dec(s):
    return isinstance(s, str) and DEC.fullmatch(s) is not None


def is_hex64(s):
    return isinstance(s, str) and HEX64.fullmatch(s) is not None


def is_hex56(s):
    return isinstance(s, str) and HEX56.fullmatch(s) is not None


def is_vid(s):
    return isinstance(s, str) and VID.fullmatch(s) is not None


def value_ok(field, value):
    """True iff value is the canonical encoding of the field's type (and range, for Int)."""
    t = field.get("type") if isinstance(field, dict) else None
    if t == "Bytes":
        return isinstance(value, str) and len(value) <= MAX_BYTES_HEX and HEX.fullmatch(value) is not None
    if t == "Bytes28":
        return is_hex56(value)
    if t in ("Bytes32", "TxId"):
        return is_hex64(value)
    if t == "Int":
        lo, hi = field.get("min"), field.get("max")
        return is_dec(value) and is_dec(lo) and is_dec(hi) and int(lo) <= int(value) <= int(hi)
    if t == "OutRef":
        return (isinstance(value, dict) and set(value) == {"tx_id", "index"} and is_hex64(value["tx_id"])
                and is_dec(value["index"]) and 0 <= int(value["index"]) <= 65535)
    return False
