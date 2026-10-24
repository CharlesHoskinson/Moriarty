"""Strict JSON and byte identity helpers; no foreign tools are invoked."""
import hashlib
import json
import math
from pathlib import Path, PurePosixPath

class Error(ValueError): pass

def canonical(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False, allow_nan=False).encode("utf-8")

def digest(raw): return hashlib.sha256(raw).hexdigest()

def _pairs(pairs):
    result = {}
    for key, value in pairs:
        if key in result: raise Error("duplicate JSON key: " + key)
        result[key] = value
    return result

def _finite_float(token):
    value = float(token)
    if not math.isfinite(value): raise Error("non-finite JSON number: " + token)
    return value

def parse_json(raw):
    try:
        return json.loads(raw, object_pairs_hook=_pairs, parse_float=_finite_float,
                          parse_constant=lambda value: (_ for _ in ()).throw(Error("non-finite JSON: " + value)))
    except (UnicodeError, json.JSONDecodeError) as exc:
        raise Error(str(exc)) from exc

def load_json(path):
    try: return parse_json(Path(path).read_bytes())
    except OSError as exc: raise Error(str(exc)) from exc

def sha(value):
    if not isinstance(value, str) or len(value) != 64 or any(c not in "0123456789abcdef" for c in value):
        raise Error("expected lowercase SHA256")
    return value

def relative(value):
    if not isinstance(value, str) or not value or "\\" in value or "\x00" in value:
        raise Error("invalid relative member")
    parts = PurePosixPath(value)
    if parts.is_absolute() or any(p in (".", "..") for p in value.split("/")) or str(parts) != value:
        raise Error("unsafe relative member: " + value)
    return value

def regular(path):
    path = Path(path)
    if not path.is_file() or path.is_symlink(): raise Error("not a regular source: " + str(path))
    return path.read_bytes()
