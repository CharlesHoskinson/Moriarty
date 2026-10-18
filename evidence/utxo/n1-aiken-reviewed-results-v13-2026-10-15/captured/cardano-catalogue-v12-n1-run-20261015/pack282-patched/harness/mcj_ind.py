"""Independent MCJ/1 serializer used only to BUILD test inputs (digests that a world needs).

Written by the falsifier author from the MCJ/1 rules stated in the v1.1 docstring (sorted keys by UTF-16
code units, no whitespace, short escapes for \\b \\t \\n \\f \\r \\" \\\\, other C0 controls as lowercase
\\u00xx, integers in shortest decimal, UTF-8 output). It does not import or copy the checker's mcj.py.
It is a world-construction helper, not a conformance claim; positive controls catch any divergence.
"""

INT_LIMIT = 2 ** 53 - 1
_SHORT = {0x08: "\\b", 0x09: "\\t", 0x0A: "\\n", 0x0C: "\\f", 0x0D: "\\r", 0x22: '\\"', 0x5C: "\\\\"}


class Unsupported(ValueError):
    pass


def _s(x):
    parts = ['"']
    for ch in x:
        o = ord(ch)
        if 0xD800 <= o <= 0xDFFF:
            raise Unsupported("lone surrogate")
        if o in _SHORT:
            parts.append(_SHORT[o])
        elif o < 0x20:
            parts.append("\\u%04x" % o)
        else:
            parts.append(ch)
    parts.append('"')
    return "".join(parts)


def _v(x, depth):
    if depth > 64:
        raise Unsupported("depth")
    if x is None:
        return "null"
    if x is True:
        return "true"
    if x is False:
        return "false"
    if type(x) is int:
        if not -INT_LIMIT <= x <= INT_LIMIT:
            raise Unsupported("int range")
        return str(x)
    if type(x) is str:
        return _s(x)
    if type(x) is list:
        return "[" + ",".join(_v(e, depth + 1) for e in x) + "]"
    if type(x) is dict:
        for k in x:
            if type(k) is not str:
                raise Unsupported("non-string key")
        ks = sorted(x, key=lambda k: k.encode("utf-16-be"))
        return "{" + ",".join(_s(k) + ":" + _v(x[k], depth + 1) for k in ks) + "}"
    raise Unsupported(type(x).__name__)


def canon(x):
    return _v(x, 0).encode("utf-8")
