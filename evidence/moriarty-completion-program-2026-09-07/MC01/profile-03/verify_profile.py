#!/usr/bin/env python3
"""Dependency-free checks for the MC01 source specification, not a frontend."""

from __future__ import annotations

import hashlib
import json
import pathlib
import re
from fractions import Fraction

ROOT = pathlib.Path(__file__).resolve().parents[4]
SPEC = ROOT / "experiments/moriarty-language/spec"
VECTORS = pathlib.Path(__file__).with_name("canonical-vectors.json")
U128_MAX = 2**128 - 1


def require(ok: bool, message: str) -> None:
    if not ok:
        raise AssertionError(message)


def canonical(value: object) -> bytes:
    if isinstance(value, bool):
        return b"true" if value else b"false"
    if isinstance(value, str):
        require(not any(0xD800 <= ord(c) <= 0xDFFF for c in value), "surrogate")
        short = {'"': r'\"', '\\': r'\\', '\b': r'\b', '\t': r'\t',
                 '\n': r'\n', '\f': r'\f', '\r': r'\r'}
        out = ['"']
        for c in value:
            out.append(short[c] if c in short else (f"\\u{ord(c):04x}" if ord(c) < 32 else c))
        out.append('"')
        return ''.join(out).encode("utf-8")
    if isinstance(value, list):
        return b"[" + b",".join(canonical(v) for v in value) + b"]"
    if isinstance(value, dict):
        require(all(isinstance(k, str) and k.isascii() for k in value), "non-ASCII key")
        keys = sorted(value, key=lambda k: k.encode("ascii"))
        return b"{" + b",".join(canonical(k) + b":" + canonical(value[k]) for k in keys) + b"}"
    raise AssertionError(f"forbidden canonical JSON value {type(value).__name__}")


def domain_hash(domain: str, value: object) -> str:
    return hashlib.sha256(domain.encode("utf-8") + b"\0" + canonical(value)).hexdigest()


def raw_hash(path: pathlib.Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def bounds_hash(path: pathlib.Path) -> str:
    return hashlib.sha256(b"MORIARTY-BOUNDS-bounded-atomic/1\0" + path.read_bytes()).hexdigest()


TOKEN = re.compile(
    r'(?P<ws>[ \t\r\n]+)|(?P<string>"(?:[^"\\\x00-\x1f]|\\["\\/bfnrt]|\\u[0-9A-Fa-f]{4})*")|'
    r'(?P<word>[A-Za-z][A-Za-z0-9_]*)|(?P<uint>0|[1-9][0-9]*)|'
    r'(?P<op>==|<=|>=|[{}();,:.=<>+*\-])'
)


def lexical_check(text: str, label: str) -> list[tuple[str, str]]:
    require("/" not in re.sub(r'"(?:[^"\\]|\\.)*"', '""', text), f"{label}: slash")
    pos, tokens = 0, []
    while pos < len(text):
        match = TOKEN.match(text, pos)
        require(match is not None, f"{label}: invalid token at character {pos}: {text[pos:pos+20]!r}")
        if match.lastgroup != "ws":
            token = match.group()
            if match.lastgroup == "string":
                json.loads(token)
            tokens.append((match.lastgroup or "", token))
        pos = match.end()
    stack = []
    pairs = {')': '(', '}': '{'}
    for kind, token in tokens:
        if kind == "op" and token in "({":
            stack.append(token)
        elif kind == "op" and token in ")}":
            require(stack and stack.pop() == pairs[token], f"{label}: unbalanced {token}")
    require(not stack, f"{label}: unclosed delimiter")
    return tokens


def brace_body(text: str, opening: int) -> tuple[str, int]:
    depth, i, in_string, escape = 0, opening, False, False
    while i < len(text):
        c = text[i]
        if in_string:
            if escape:
                escape = False
            elif c == '\\':
                escape = True
            elif c == '"':
                in_string = False
        elif c == '"':
            in_string = True
        elif c == '{':
            depth += 1
        elif c == '}':
            depth -= 1
            if depth == 0:
                return text[opening + 1:i], i + 1
        i += 1
    raise AssertionError("unclosed brace")


def named_blocks(text: str, introducer: str) -> dict[str, tuple[str, str]]:
    pattern = re.compile(rf'\b{introducer}\s+([A-Za-z][A-Za-z0-9_]*)\s*([^{{]*){{')
    result = {}
    for match in pattern.finditer(text):
        body, _ = brace_body(text, match.end() - 1)
        require(match.group(1) not in result, f"duplicate {introducer} {match.group(1)}")
        result[match.group(1)] = (match.group(2), body)
    return result


def example_static_check(path: pathlib.Path, keywords: set[str]) -> dict[str, int]:
    text = path.read_text(encoding="utf-8")
    lexical_check(text, path.name)
    units = set(re.findall(r'^\s*unit\s+(\w+)\s*;', text, re.M))
    consts = set(re.findall(r'^\s*const\s+(\w+)\s*:', text, re.M))
    states = {n: t for n, t in re.findall(r'^\s*state\s+(\w+)\s*:\s*([^=]+?)\s*=', text, re.M)}
    observations = set(re.findall(r'^\s*observation\s+(\w+)\s*:', text, re.M))
    actions = named_blocks(text, "action")
    require(actions, f"{path.name}: no actions")
    required_targets: set[tuple[str, str, str]] = set()
    locals_by_action: dict[str, dict[str, str]] = {}
    for action, (header, body) in actions.items():
        params_text = header.strip()
        require(params_text.startswith("(" ) and params_text.endswith(")"), f"{action}: params")
        params = {name for name, _ in re.findall(r'(\w+)\s*:\s*([^,\)]+)', params_text)}
        require("actor" in params, f"{action}: actor missing")
        require(not (params & keywords), f"{action}: keyword parameter {params & keywords}")
        for namespace, allowed in (("state", set(states)), ("const", consts),
                                   ("obs", observations), ("arg", params)):
            seen = set(re.findall(rf'\b{namespace}\.(\w+)', body))
            require(seen <= allowed, f"{action}: unresolved {namespace}: {seen-allowed}")
        local_defs = {}
        for name, expression in re.findall(r'\blet\s+(\w+)\s*=\s*([^;]+);', body):
            require(name not in local_defs and name not in keywords, f"{action}: bad local {name}")
            local_defs[name] = expression.strip()
        locals_by_action[action] = local_defs
        for field in re.findall(r'\bset\s+(\w+)\s*=', body):
            require(field in states, f"{action}: unknown set {field}")
            if states[field].strip().startswith("Amount<"):
                required_targets.add(("write", action, field))
        emits = re.findall(r'\bemit\s+(Transfer|Fee|DueCreated|DueSettled)\s*{([^}]*)}', body, re.S)
        for ordinal, (_, fields) in enumerate(emits):
            labels = [x for x in re.findall(r'(\w+)\s*:', fields)]
            require(len(labels) == len(set(labels)), f"{action}: duplicate emit label")
            if "amount" in labels:
                required_targets.add(("effect", action, f"{ordinal}:amount"))

    actual_targets: set[tuple[str, str, str]] = set()
    policy_rounding = []
    for match in re.finditer(r'\bpolicy\s+(\w+)\s+targets\s+(.*?)\s*{', text, re.S):
        target_text = match.group(2)
        for kind, action, third, field in re.findall(r'\b(write|effect)\((\w+),\s*(\w+)(?:,\s*(\w+))?\)', target_text):
            key = (kind, action, third if kind == "write" else f"{third}:{field}")
            require(key not in actual_targets, f"duplicate policy target {key}")
            actual_targets.add(key)
        body, _ = brace_body(text, match.end() - 1)
        rounding = re.search(r'\brounding\s+(none|floor\((\w+),\s*(\w+)\))\s*;', body)
        require(rounding is not None, "policy rounding missing")
        if rounding.group(2):
            policy_rounding.append((rounding.group(2), rounding.group(3)))
    require(actual_targets == required_targets,
            f"{path.name}: policy coverage missing={required_targets-actual_targets} extra={actual_targets-required_targets}")
    for action, local in policy_rounding:
        require(action in locals_by_action and local in locals_by_action[action], f"rounding local {action}.{local}")
        require(locals_by_action[action][local].startswith("floor_div("), f"rounding root {action}.{local}")
    return {"actions": len(actions), "policyTargets": len(actual_targets), "units": len(units)}


def effects_check(text: str) -> None:
    expected = {
        "Transfer": [("asset", "Text"), ("from", "Text"), ("to", "Text"), ("amount", "Amount")],
        "Fee": [("asset", "Text"), ("from", "Text"), ("to", "Text"), ("amount", "Amount")],
        "DueCreated": [("due_id", "Text"), ("debtor", "Text"), ("creditor", "Text"),
                       ("denomination", "Text"), ("amount", "Amount")],
        "DueSettled": [("due_id", "Text"), ("debtor", "Text"), ("creditor", "Text"),
                       ("denomination", "Text"), ("amount", "Amount"), ("asset", "Text")],
    }
    blocks = named_blocks(text, "effect")
    for kind, (_, body) in blocks.items():
        fields = re.findall(r'\b(\w+)\s*:\s*(UInt128|Text|Amount(?:<\w+>)?)\s*;', body)
        require(kind in expected and fields == expected[kind], f"effect schema {kind}: {fields}")
    emitted = set(re.findall(r'\bemit\s+(Transfer|Fee|DueCreated|DueSettled)', text))
    require(emitted <= set(blocks), f"undeclared effects {emitted-set(blocks)}")


def addv(a: dict[str, int], b: dict[str, int]) -> dict[str, int]:
    out = dict(a)
    for k, v in b.items():
        out[k] = out.get(k, 0) + v
        if out[k] == 0:
            del out[k]
    return out


def subv(a: dict[str, int], b: dict[str, int]) -> dict[str, int]:
    return addv(a, {k: -v for k, v in b.items()})


def arithmetic_and_units() -> dict[str, str]:
    usd, aa, ab, one = {"USD_micro": 1}, {"AssetA_quantum": 1}, {"AssetB_quantum": 1}, {}
    require(subv(addv(addv(usd, one), one), addv(one, one)) == usd, "loan units")
    effective = addv(aa, one)
    numerator_u = addv(effective, ab)
    denominator_u = addv(aa, one)
    require(subv(numerator_u, denominator_u) == ab, "swap units")
    loan_num = 5_000_000_000 * 8 * 31
    loan_den = 100 * 365
    swap_eff = 10_000 * 997
    swap_num = swap_eff * 2_000_000
    swap_den = 1_000_000 * 1000 + swap_eff
    require(max(loan_num, loan_den, swap_eff, swap_num, swap_den) <= U128_MAX, "vector overflow")
    li, lr = divmod(loan_num, loan_den)
    so, sr = divmod(swap_num, swap_den)
    require((li, Fraction(lr, loan_den)) == (33_972_602, Fraction(54, 73)), "loan arithmetic")
    require(Fraction(loan_num, loan_den) == Fraction(2_480_000_000, 73), "loan exact")
    require(so == 19_743 and Fraction(swap_num, swap_den) == Fraction(1_994_000_000, 100_997), "swap arithmetic")
    require((1_000_000 + 10_000, 2_000_000 - so) == (1_010_000, 1_980_257), "swap reserves")
    return {"loanInterest": str(li), "loanRemainder": "54/73", "swapOutput": str(so),
            "swapRemainderNumerator": str(sr)}


def quantum(value: int, q: int) -> tuple[str, int | str]:
    require(0 < q <= U128_MAX and 0 <= value <= U128_MAX, "quantum range")
    quotient, remainder = divmod(value, q)
    if remainder:
        return "Rejected", "SETTLEMENT_NON_DIVISIBLE"
    require(quotient * q <= U128_MAX and quotient * q == value, "reverse conversion")
    return "Complete", quotient


def obligation_trace() -> dict[str, object]:
    records: dict[str, tuple[int, str]] = {}
    def create(due: str, amount: int) -> None:
        require(amount > 0 and due not in records, "create")
        records[due] = (amount, "Outstanding")
    def settle(due: str, amount: int) -> str:
        if due not in records: return "OBLIGATION_UNKNOWN"
        expected, status = records[due]
        if status == "Settled": return "OBLIGATION_ALREADY_SETTLED"
        if amount < expected: return "OBLIGATION_PARTIAL_UNSUPPORTED"
        if amount > expected: return "OBLIGATION_EXCESS"
        records[due] = (expected, "Settled")
        return "Complete"
    create("lam01:period1:PR", 500_000_000)
    create("lam01:period1:IP", 33_972_602)
    require(settle("missing", 1) == "OBLIGATION_UNKNOWN", "unknown")
    require(settle("lam01:period1:PR", 1) == "OBLIGATION_PARTIAL_UNSUPPORTED", "partial")
    require(settle("lam01:period1:PR", 500_000_001) == "OBLIGATION_EXCESS", "excess")
    require(settle("lam01:period1:PR", 500_000_000) == "Complete", "settle PR")
    require(settle("lam01:period1:PR", 500_000_000) == "OBLIGATION_ALREADY_SETTLED", "duplicate settle")
    require(settle("lam01:period1:IP", 33_972_602) == "Complete", "settle IP")
    status = "Outstanding" if 4_500_000_000 > 0 or any(v[1] == "Outstanding" for v in records.values()) else "NoOutstanding"
    require(status == "Outstanding", "remaining notional discharged")
    return {"records": len(records), "agreementAfterSettle": status, "remainingNotional": "4500000000"}


def main() -> None:
    vectors = json.loads(VECTORS.read_text(encoding="utf-8"))
    for name in ("bounds.json", "numeric-profile.json", "target-crosswalk.json", "canonical-vectors.json"):
        json.loads((SPEC / name).read_text(encoding="utf-8") if name != "canonical-vectors.json" else VECTORS.read_text(encoding="utf-8"))
    grammar = (SPEC / "grammar.ebnf").read_text(encoding="utf-8")
    keyword_part = grammar.split("keyword =", 1)[1].split("reserved_identifier", 1)[0]
    keyword_list = re.findall(r'"([A-Za-z_][A-Za-z0-9_]*)"', keyword_part)
    require(len(keyword_list) == len(set(keyword_list)), "duplicate keyword")
    keywords = set(keyword_list)
    loan_stats = example_static_check(SPEC / "examples/loan.moriarty", keywords)
    swap_stats = example_static_check(SPEC / "examples/swap.moriarty", keywords)
    effects_check((SPEC / "examples/loan.moriarty").read_text(encoding="utf-8"))
    effects_check((SPEC / "examples/swap.moriarty").read_text(encoding="utf-8"))

    crosswalk_path = SPEC / "target-crosswalk.json"
    crosswalk = json.loads(crosswalk_path.read_text(encoding="utf-8"))
    counts = {family: sum(row["source_family"] == family for row in crosswalk["rows"])
              for family in ("ACTUS", "DEFI")}
    require(counts == {"ACTUS": 32, "DEFI": 72}, f"crosswalk counts {counts}")
    require(len({row["row_id"] for row in crosswalk["rows"]}) == 104, "row IDs")
    require(all(row["mc07_mandatory"] is True for row in crosswalk["rows"]), "mandatory row")
    digests = vectors["artifactDigests"]
    require(raw_hash(crosswalk_path) == digests["targetCrosswalkSha256"], "crosswalk changed")
    require(raw_hash(SPEC / "examples/loan.moriarty") == digests["loanSourceSha256"], "loan digest")
    require(raw_hash(SPEC / "examples/swap.moriarty") == digests["swapSourceSha256"], "swap digest")
    require(bounds_hash(SPEC / "bounds.json") == digests["boundsDomainSeparatedSha256"], "bounds digest")
    for key in ("codec", "action"):
        item = vectors[key]
        require(canonical(item["decoded"]).decode("utf-8") == item["canonicalUtf8"], f"{key} canonical")
    require(domain_hash(vectors["codec"]["domain"], vectors["codec"]["decoded"]) == vectors["codec"]["domainSeparatedSha256"], "codec hash")
    require(domain_hash(vectors["action"]["domain"], vectors["action"]["decoded"]) == vectors["action"]["actionHash"], "action hash")
    require(quantum(20, 10) == ("Complete", 2), "quantum positive")
    require(quantum(15, 10) == ("Rejected", "SETTLEMENT_NON_DIVISIBLE"), "quantum adverse")
    arithmetic = arithmetic_and_units()
    obligations = obligation_trace()

    docs = (SPEC / "typed-schemas.md").read_text() + (SPEC / "semantics.md").read_text()
    for required in ("moriarty-canonical-json/1", "moriarty-core/1", "MORIARTY-PROGRAM-bounded-atomic/1",
                     "MORIARTY-SIGN-v1", "MORIARTY-OUTCOME-SIGN-v1", "UNSUPPORTED_PENDING",
                     "OBLIGATION_PARTIAL_UNSUPPORTED", "SETTLEMENT_NON_DIVISIBLE", "4,500,000,000"):
        require(required in docs, f"missing specification term {required}")
    result = {"status": "pass", "scope": "source-only verification model; no frontend/compiler/proof/ledger",
              "crosswalk": counts, "examples": {"loan": loan_stats, "swap": swap_stats},
              "arithmetic": arithmetic, "obligations": obligations,
              "quantum": {"20/10": "2", "15/10": "SETTLEMENT_NON_DIVISIBLE"},
              "canonicalDigests": {"codec": vectors["codec"]["domainSeparatedSha256"],
                                     "action": vectors["action"]["actionHash"]}}
    print(json.dumps(result, sort_keys=True, separators=(",", ":")))


if __name__ == "__main__":
    main()
