#!/usr/bin/env python3
"""Check the U0 numeric profile against the owner decision and successor sources."""

from __future__ import annotations

import argparse
import hashlib
import json
import re
from pathlib import Path
from typing import Any, NamedTuple

from jsonschema import Draft202012Validator
from jsonschema.exceptions import SchemaError


MORIARTY_ROOT = Path(__file__).resolve().parents[1]
DECISION_FILE = "docs/decisions/u0-numeric-profile-decision.md"
SCHEMA_FILE = (
    "openspec/changes/consolidated-language-kernel/schemas/numeric-profile.schema.json"
)
PROFILE_FILE = "deliverables/u0-semantic-contract-2026-09-23/numeric-profile.json"
SUCCESSOR_DIR = "experiments/moriarty-language/src/successor"
SCHEMA_DIALECT = "https://json-schema.org/draft/2020-12/schema"

# Quote-per-base mantissa M at scale s denotes M/10^s quote units per base unit.
# The base-per-quote mantissa at target scale t is the directed reciprocal
# 10^(s+t)/M. Receipt uses floor (numerator//M). Obligation uses ceil
# ((numerator+M-1)//M). Scales stay explicit.
FORMULA = (
    "A positive quote-per-base mantissa M at scale s denotes M/10^s quote units "
    "per one base unit. The base-per-quote mantissa at target scale t is the "
    "directed reciprocal 10^(s+t)/M. The numerator is 10^(s+t). A receipt uses "
    "floor: numerator//M. An obligation uses ceil: (numerator+M-1)//M. The source "
    "scale and the target scale stay explicit. This is not a rename and not a "
    "second rounding of an already rounded reciprocal."
)

DECISION_PHRASES = (
    "base-per-quote",
    "`ceil`",
    "`floor`",
    "`none`",
    "protocol reserve",
    "never become field elements",
    "exact domain-qualified integers",
)
ROLE_DIRECTION = {
    "obligation": "ceil",
    "receipt": "floor",
    "exact": "none",
}

# An operand on each side of `/`, after comments and string literals are blanked.
# Spacing does not matter. `//` and `/*` are not division operators.
_OPERAND = r"(?:[A-Za-z_$][\w$]*|\d+n?|\)|\])"
DIVISION = re.compile(rf"{_OPERAND}\s*/(?!/|\*)\s*{_OPERAND}")
ROUNDING_WORD = re.compile(r"\bfloor\b|\bceil\b")
ROUNDING_OR_SCALE = re.compile(r"\bfloor\b|\bceil\b|Rounding|\bscale\b|\bdiv\b")
BENEFICIARY_CAVEAT = re.compile(
    r"remainder is not posted to (?:a |the )?protocol reserve",
    re.IGNORECASE,
)
BENEFICIARY_POSTED = re.compile(
    r"remainder is posted to (?:a |the )?protocol reserve",
    re.IGNORECASE,
)
# floor(256 * log10(2)). 10^77 fits in UInt256 and 10^78 does not.
# The reciprocal numerator is 10^(s+t), so the largest checked mantissa is 10^154.
MAX_SCALE = 77
MAX_RECIPROCAL = 10 ** (MAX_SCALE * 2)
REMAINDER_WINDOW = 8
RESERVE_NAMES = frozenset({"protocolReserve", "protocol_reserve"})
REGEX_PREFIX_KEYWORDS = frozenset({
    "return",
    "typeof",
    "case",
    "in",
    "of",
    "void",
    "delete",
    "throw",
    "yield",
    "await",
    "instanceof",
    "new",
    "else",
    "do",
})
PRICE_FILE = "experiments/moriarty-language/src/successor/financial-expression-v1.ts"
AMOUNT_BIND = re.compile(
    r"const\s+amount\s*=\s*left\[0\]\s*===\s*'Amount'\s*\?\s*left\s*:"
    r"\s*right\[0\]\s*===\s*'Amount'\s*\?\s*right\s*:\s*null"
)
OTHER_BIND = re.compile(
    r"const\s+other\s*=\s*left\[0\]\s*===\s*'Amount'\s*\?\s*right\s*:\s*left"
)
PRICE_MAPPING = re.compile(
    r"if\s*\(\s*amount\s*&&\s*other\[0\]\s*===\s*'Price'\s*&&\s*"
    r"other\[2\]\s*===\s*amount\[1\]\s*\)\s*return\s*\[\s*'ScaledAmount'\s*,\s*"
    r"other\[1\]\s*,\s*other\[3\]\s*\]"
)
LIT_PRICE = re.compile(
    r"case\s+'LitPrice'\s*:\s*return\s*\{\s*type\s*:\s*\[\s*'Price'\s*,\s*"
    r"o\.base\s*,\s*o\.quote\s*,\s*o\.scale\s*\]"
)
DIVISION_ASSIGN = re.compile(
    r"(?:(?:const|let|var)\s+)?(?P<quotient>[A-Za-z_]\w*)\s*=\s*"
    r"(?P<numerator>[A-Za-z_]\w*)\s*/\s*(?P<divisor>[A-Za-z_]\w*)\b"
)
OVERFLOW_LINE = re.compile(
    r"numericFits\s*\(|UINT(?:64|128)_MAX|\?\s*null\s*:"
)
OVERFLOW_RETURN = re.compile(
    r"return\s+(?:"
    r"(?P<sum>[A-Za-z_]\w*)\s*>\s*UINT(?:64|128)_MAX\s*\?\s*null\s*:\s*(?P=sum)"
    r"|"
    r"(?P<right>[A-Za-z_]\w*)\s*>\s*(?P<left>[A-Za-z_]\w*)\s*\?\s*null\s*:\s*"
    r"(?P=left)\s*-\s*(?P=right)"
    r")\s*;"
)
UPDATE = re.compile(
    r"(?P<lhs>[A-Za-z_]\w*(?:\s*\.\s*[A-Za-z_]\w*)*)\s*"
    r"(?P<op>\+=|=(?!=))\s*(?P<rhs>[^;]+)"
)
FUNCTION_START = re.compile(
    r"^(?:export\s+)?(?:async\s+)?function\s+([A-Za-z_]\w*)\s*\("
)
METHOD_START = re.compile(
    r"^([ \t]+)(?:async\s+)?([A-Za-z_]\w*)\s*\((?:[^()]|\([^()]*\))*\)\s*(?::[^{]+)?\{\s*$"
)
CONTROL_WORDS = {"if", "for", "while", "switch", "catch", "else"}
WIDTH_NAME = re.compile(r"^(?:UInt|SInt)(\d+)$")

# floor/ceil lines inside an arithmetic helper that are not a source/5 primitive.
# repayment.ts convertNominal is the earlier kernel. Source/5 uses financial-lifecycle.ts.
IGNORED_ROUNDING_SITES: tuple[tuple[str, int, str], ...] = (
    (
        "experiments/moriarty-language/src/successor/repayment.ts",
        926,
        "Same convertNominal floor branch as financial-lifecycle.ts. "
        "Source/5 uses the lifecycle kernel.",
    ),
)

# Precision-losing divisions that are duplicates of a listed source/5 site,
# or that do not round an amount, price, or rate.
IGNORED_DIVISION_SITES: tuple[tuple[str, int, str], ...] = (
    (
        "experiments/moriarty-language/src/successor/repayment.ts",
        918,
        "Same convertNominal division as financial-lifecycle.ts. "
        "Source/5 uses the lifecycle kernel.",
    ),
    (
        "experiments/moriarty-language/src/successor/repayment.ts",
        965,
        "Same ProRata division as financial-lifecycle.ts allocateNominal. "
        "Source/5 uses the lifecycle kernel.",
    ),
    (
        "experiments/moriarty-language/src/successor/expression-v1.ts",
        241,
        "Same FloorDiv and CeilDiv reducer as financial-expression-v1.ts. "
        "Source/5 evaluates contract/4 in that file.",
    ),
    (
        "experiments/moriarty-language/src/successor/financial-agreement-source-compiler.ts",
        364,
        "Counts Quantity type-argument pairs. "
        "This is not an amount, price, or rate rounding.",
    ),
)

# Definitions and reducer guards that reject overflow instead of wrapping.
# repayment.ts and expression-v1.ts repeat the source/5 helpers.
IGNORED_OVERFLOW_SITES: tuple[tuple[str, int, str], ...] = (
    (
        "experiments/moriarty-language/src/successor/repayment.ts",
        207,
        "Same addU128 overflow rejection as financial-lifecycle.ts. "
        "Source/5 uses the lifecycle kernel.",
    ),
    (
        "experiments/moriarty-language/src/successor/repayment.ts",
        211,
        "Same subU128 overflow rejection as financial-lifecycle.ts. "
        "Source/5 uses the lifecycle kernel.",
    ),
    (
        "experiments/moriarty-language/src/successor/repayment.ts",
        216,
        "Same mulU128 overflow rejection as financial-lifecycle.ts. "
        "Source/5 uses the lifecycle kernel.",
    ),
    (
        "experiments/moriarty-language/src/successor/expression-v1.ts",
        244,
        "Same Add, Sub, and Mul numericFits guard as financial-expression-v1.ts. "
        "Source/5 evaluates contract/4 in that file.",
    ),
)


class RoleEvidence(NamedTuple):
    """One fail-closed role entry.

    A division receives a result role only when an entry's pattern matches the
    cited region. The checker re-reads that region from source. A division with
    no matching entry fails closed with "no derived result role".
    region is "division-function" or "caller". caller is empty for the former.
    """

    file: str
    line: int
    role: str
    region: str
    caller: str
    pattern: str


# Deliberate fail-closed allowlist. Patterns are source evidence, not a guess.
ROLE_EVIDENCE: tuple[RoleEvidence, ...] = (
    RoleEvidence(
        "experiments/moriarty-language/src/successor/financial-lifecycle.ts",
        1779,
        "obligation",
        "division-function",
        "",
        r"\baddU128\(\s*previous(?:Accrued|Outstanding|Incurred)\s*,\s*interest\s*\)",
    ),
    RoleEvidence(
        "experiments/moriarty-language/src/successor/financial-expression-v1.ts",
        404,
        "obligation",
        "division-function",
        "",
        r"\bFloorDiv\b",
    ),
    RoleEvidence(
        "experiments/moriarty-language/src/successor/financial-expression-v1.ts",
        404,
        "receipt",
        "division-function",
        "",
        r"\bCeilDiv\b",
    ),
    RoleEvidence(
        "experiments/moriarty-language/src/successor/financial-lifecycle.ts",
        1376,
        "obligation",
        "caller",
        "applyRepay",
        r"\bsubU128\(\s*funded\.remaining\s*,\s*settlement\.value\s*\)",
    ),
    RoleEvidence(
        "experiments/moriarty-language/src/successor/financial-lifecycle.ts",
        1376,
        "receipt",
        "caller",
        "applyOriginate",
        r"\bfunded\.to\s*!==\s*action\.debtor\b[\s\S]*\bsettlement\.value\s*!==\s*funded\.amount\b",
    ),
    RoleEvidence(
        "experiments/moriarty-language/src/successor/financial-lifecycle.ts",
        1423,
        "receipt",
        "caller",
        "applyRepay",
        r"\bprincipal\s*-\s*parts\.value\.dP\b",
    ),
)


class Division(NamedTuple):
    quotient: str
    numerator: str
    divisor: str


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--root",
        type=Path,
        default=MORIARTY_ROOT,
        help="Repository root. Defaults to the Moriarty checkout.",
    )
    args = parser.parse_args(argv)
    blocked, failures, counts = check(args.root.resolve())
    if blocked is not None:
        print(f"blocked: {blocked}")
        return 2
    if failures:
        for failure in failures:
            print(failure)
        return 1
    primitives, gaps, beneficiary_gaps = counts or (0, 0, 0)
    print(f"OK: {primitives} primitives, {gaps} open conformance gaps")
    print(f"beneficiary gaps: {beneficiary_gaps}")
    return 0


def check(root: Path) -> tuple[str | None, list[str], tuple[int, int, int] | None]:
    decision_path = root / DECISION_FILE
    if not decision_path.is_file():
        return "decision file missing", [], None

    schema_path = root / SCHEMA_FILE
    if not schema_path.is_file():
        return "schema file missing", [], None
    profile_path = root / PROFILE_FILE
    if not profile_path.is_file():
        return "numeric profile missing", [], None
    successor = root / SUCCESSOR_DIR
    if not successor.is_dir():
        return "successor sources missing", [], None

    failures: list[str] = []
    decision = read_utf8(decision_path, DECISION_FILE, failures)
    if decision is None:
        return None, failures, None
    decision_bytes, decision_text = decision
    for phrase in DECISION_PHRASES:
        if phrase not in decision_text:
            failures.append(f"FAIL: decision file does not state {phrase}")

    schema, schema_text = load_json(
        schema_path, "numeric profile schema", SCHEMA_FILE, failures
    )
    profile, profile_text = load_json(
        profile_path, "numeric profile", PROFILE_FILE, failures
    )
    if schema is None or profile is None:
        return None, failures, None
    if not isinstance(schema, dict) or not isinstance(profile, dict):
        failures.append("FAIL: numeric profile and its schema must be JSON objects")
        return None, failures, None

    if not profile_text.endswith("\n") or profile_text.endswith("\n\n"):
        failures.append(
            "FAIL: numeric profile must end with one trailing newline"
        )
    rendered = json.dumps(profile, indent=2, ensure_ascii=False) + "\n"
    if profile_text != rendered:
        failures.append(
            "FAIL: numeric profile is not UTF-8 JSON with 2-space indent "
            "and unescaped non-ASCII"
        )
    if schema.get("$schema") != SCHEMA_DIALECT:
        failures.append("FAIL: schema is not draft 2020-12")
    try:
        Draft202012Validator.check_schema(schema)
    except SchemaError as error:
        failures.append(f"FAIL: numeric profile schema is invalid: {one_line(error.message)}")
        return None, failures, None

    assert_schema_closed(schema, "<schema>", failures)
    validator = Draft202012Validator(schema)
    schema_errors = sorted(
        validator.iter_errors(profile),
        key=lambda error: ([str(part) for part in error.absolute_path], error.message),
    )
    if schema_errors:
        for error in schema_errors:
            location = "/".join(str(part) for part in error.absolute_path) or "<root>"
            failures.append(f"FAIL: {location} {one_line(error.message)}")
        return None, failures, None

    check_key_order(profile, schema, "<root>", failures)
    digest = hashlib.sha256(decision_bytes).hexdigest()
    if profile["decisionSha256"] != digest:
        failures.append(
            "FAIL: decisionSha256 does not match "
            f"{DECISION_FILE} ({digest})"
        )
    if profile["defaultPolicy"] != {
        "obligation": "ceil",
        "receipt": "floor",
        "exact": "none",
        "remainderBeneficiary": "protocol-reserve",
    }:
        failures.append("FAIL: defaultPolicy does not match the decision phrases")
    if profile["overrides"] != []:
        failures.append("FAIL: U0 overrides must be empty")
    if profile["defiformalConversion"]["formula"] != FORMULA:
        failures.append("FAIL: defiformalConversion.formula does not match the checker formula")

    sources: dict[str, list[str]] = {}
    for path in sorted(successor.glob("*.ts")):
        relative = path.relative_to(root).as_posix()
        loaded = read_utf8(path, relative, failures)
        if loaded is None:
            return None, failures, None
        sources[relative] = loaded[1].splitlines()
    try:
        masked = {name: masked_lines(lines) for name, lines in sources.items()}
    except ValueError as error:
        failures.append(f"FAIL: {error}")
        return None, failures, None
    check_price_types(profile, sources, failures)
    check_price_orientation(profile, sources, masked, failures)
    check_widths(profile, sources, failures)
    check_vectors(profile, failures)
    primitive_count, gap_count, beneficiary_gaps = check_primitives(
        profile, sources, masked, failures
    )
    check_coverage(profile, sources, masked, failures)
    return None, failures, (primitive_count, gap_count, beneficiary_gaps)


def read_utf8(
    path: Path, display: str, failures: list[str]
) -> tuple[bytes, str] | None:
    data = path.read_bytes()
    try:
        return data, data.decode("utf-8")
    except UnicodeDecodeError:
        failures.append(f"FAIL: {display} is not UTF-8")
        return None


def load_json(
    path: Path, label: str, display: str, failures: list[str]
) -> tuple[object | None, str]:
    loaded = read_utf8(path, display, failures)
    if loaded is None:
        return None, ""
    text = loaded[1]
    try:
        return json.loads(text), text
    except json.JSONDecodeError as error:
        failures.append(f"FAIL: {label} is not JSON: {one_line(str(error))}")
        return None, text


def one_line(message: str) -> str:
    return " ".join(message.split())


def operand_before(emitted: list[str]) -> bool:
    for ch in reversed(emitted):
        if ch in " \t":
            continue
        return ch.isalnum() or ch in "_$)]"
    return False


def preceding_token(emitted: list[str]) -> str:
    """Return the token before a slash, skipping whitespace."""
    index = len(emitted) - 1
    while index >= 0 and emitted[index].isspace():
        index -= 1
    if index < 0:
        return ""
    if not (emitted[index].isalnum() or emitted[index] in "_$"):
        return emitted[index]
    end = index
    while index >= 0 and (emitted[index].isalnum() or emitted[index] in "_$"):
        index -= 1
    return "".join(emitted[index + 1 : end + 1])


def mask_non_code(text: str, *, blank_strings: bool = True) -> str:
    """Blank comments, and optionally string literals, without moving newlines."""
    out: list[str] = []
    i = 0
    n = len(text)
    while i < n:
        ch = text[i]
        nxt = text[i + 1] if i + 1 < n else ""
        if ch == "/" and nxt == "/":
            while i < n and text[i] != "\n":
                out.append(" ")
                i += 1
            continue
        if ch == "/" and nxt == "*":
            out.append(" ")
            out.append(" ")
            i += 2
            while i < n and not (text[i] == "*" and i + 1 < n and text[i + 1] == "/"):
                out.append("\n" if text[i] == "\n" else " ")
                i += 1
            if i < n:
                out.append(" ")
                i += 1
            if i < n:
                out.append(" ")
                i += 1
            continue
        token = preceding_token(out)
        if ch == "/" and (
            token in REGEX_PREFIX_KEYWORDS or not operand_before(out)
        ):
            # A slash after a keyword such as `return` or `typeof` starts a
            # regex literal, as does a slash that does not follow an operand.
            # The flags are included. A quote inside the literal is not a string.
            out.append(" ")
            i += 1
            in_class = False
            while i < n:
                current = text[i]
                if current == "\\":
                    out.append(" ")
                    i += 1
                    if i < n:
                        out.append("\n" if text[i] == "\n" else " ")
                        i += 1
                    continue
                if current == "[" and not in_class:
                    in_class = True
                elif current == "]" and in_class:
                    in_class = False
                elif current == "/" and not in_class:
                    out.append(" ")
                    i += 1
                    break
                if current == "\n":
                    out.append("\n")
                    i += 1
                    break
                out.append(" ")
                i += 1
            while i < n and text[i].isascii() and text[i].isalpha():
                out.append(" ")
                i += 1
            continue
        if blank_strings and ch in "'\"`":
            quote = ch
            out.append(" ")
            i += 1
            while i < n:
                current = text[i]
                if current == "\\":
                    out.append(" ")
                    i += 1
                    if i < n:
                        out.append("\n" if text[i] == "\n" else " ")
                        i += 1
                    continue
                if current == quote:
                    out.append(" ")
                    i += 1
                    break
                out.append("\n" if current == "\n" else " ")
                i += 1
            continue
        out.append(ch)
        i += 1
    return "".join(out)


def masked_lines(lines: list[str]) -> list[str]:
    return _split_masked(lines, blank_strings=True)


def comment_masked_lines(lines: list[str]) -> list[str]:
    """Blank comments only. String literals stay, so 'FloorDiv' remains visible."""
    return _split_masked(lines, blank_strings=False)


def _split_masked(lines: list[str], *, blank_strings: bool) -> list[str]:
    if not lines:
        return []
    masked = mask_non_code("\n".join(lines), blank_strings=blank_strings).split("\n")
    if len(masked) != len(lines):
        raise ValueError("mask changed the line count")
    return masked


def assert_schema_closed(schema: object, path: str, failures: list[str]) -> None:
    if not isinstance(schema, dict):
        return
    if schema.get("type") == "object":
        properties = list(schema.get("properties", {}))
        if schema.get("additionalProperties") is not False:
            failures.append(f"FAIL: {path} must set additionalProperties to false")
        if list(schema.get("required", [])) != properties:
            failures.append(f"FAIL: {path} required keys must match properties in order")
        for name, child in schema.get("properties", {}).items():
            assert_schema_closed(child, f"{path}.{name}", failures)
    items = schema.get("items")
    if isinstance(items, dict):
        assert_schema_closed(items, f"{path}.items", failures)
    for key in ("allOf", "anyOf", "oneOf"):
        for index, child in enumerate(schema.get(key, [])):
            assert_schema_closed(child, f"{path}.{key}[{index}]", failures)


def check_key_order(
    instance: object, schema: object, path: str, failures: list[str]
) -> None:
    if not isinstance(schema, dict):
        return
    if schema.get("type") == "object" and isinstance(instance, dict):
        properties = list(schema.get("properties", {}))
        if list(instance) != properties:
            failures.append(
                f"FAIL: {path} keys are not in schema order"
            )
        for name, child in schema.get("properties", {}).items():
            if name in instance:
                check_key_order(instance[name], child, f"{path}.{name}", failures)
    if schema.get("type") == "array" and isinstance(instance, list):
        items = schema.get("items")
        if isinstance(items, dict):
            for index, item in enumerate(instance):
                check_key_order(item, items, f"{path}[{index}]", failures)


def check_price_types(
    profile: dict[str, Any],
    sources: dict[str, list[str]],
    failures: list[str],
) -> None:
    rows = profile["priceOrientation"]["sourceTypes"]
    keys = [
        (row["file"], row["symbol"])
        for row in rows
    ]
    if keys != sorted(keys):
        failures.append("FAIL: priceOrientation.sourceTypes is not sorted by file and symbol")
    for row in rows:
        require_symbol(str(row["file"]), str(row["symbol"]), sources, "price type", failures)


def check_price_orientation(
    profile: dict[str, Any],
    sources: dict[str, list[str]],
    masked: dict[str, list[str]],
    failures: list[str],
) -> None:
    """Require Amount<B> × Price<A,B,S> → ScaledAmount<A,S> from source.

    Symbol presence is not enough. LitPrice stores ['Price', base, quote, scale].
    arithmeticType then requires the amount asset to equal the quote and returns
    a scaled amount of the base at that scale.
    """
    raw = sources.get(PRICE_FILE)
    if raw is None:
        failures.append(f"FAIL: price orientation file {PRICE_FILE} is missing")
        return
    lines = comment_masked_lines(raw)
    span = next(
        (item for item in function_spans(raw) if item[2] == "arithmeticType"),
        None,
    )
    if span is None:
        failures.append(f"FAIL: {PRICE_FILE} has no arithmeticType function")
        return
    body = "\n".join(lines[span[0] - 1 : span[1]])
    mapping_hits = [
        number
        for number, line in enumerate(lines, start=1)
        if PRICE_MAPPING.search(line) is not None
    ]
    if (
        len(mapping_hits) != 1
        or AMOUNT_BIND.search(body) is None
        or OTHER_BIND.search(body) is None
        or not (span[0] <= mapping_hits[0] <= span[1])
    ):
        failures.append(
            "FAIL: price orientation is not derived from Amount x Price -> "
            f"ScaledAmount in {PRICE_FILE}"
        )
        mapping_line = None
    else:
        mapping_line = mapping_hits[0]
    lit_hits = [
        number
        for number, line in enumerate(lines, start=1)
        if LIT_PRICE.search(line) is not None
    ]
    if len(lit_hits) != 1:
        failures.append(
            "FAIL: LitPrice does not store base, quote, scale in "
            f"{PRICE_FILE}"
        )
        lit_line = None
    else:
        lit_line = lit_hits[0]
    note = str(profile["priceOrientation"]["note"])
    if mapping_line is not None and f"financial-expression-v1.ts:{mapping_line}" not in note:
        failures.append(
            "FAIL: priceOrientation.note does not cite "
            f"financial-expression-v1.ts:{mapping_line}"
        )
    if lit_line is not None and f"financial-expression-v1.ts:{lit_line}" not in note:
        failures.append(
            "FAIL: priceOrientation.note does not cite "
            f"financial-expression-v1.ts:{lit_line}"
        )


def require_symbol(
    relative: str,
    symbol: str,
    sources: dict[str, list[str]],
    label: str,
    failures: list[str],
) -> None:
    lines = sources.get(relative)
    if lines is None:
        failures.append(f"FAIL: {label} file {relative} is missing")
        return
    if re.search(rf"\b{re.escape(symbol)}\b", "\n".join(lines)) is None:
        failures.append(f"FAIL: {label} symbol {symbol} does not occur in {relative}")


def check_widths(
    profile: dict[str, Any],
    sources: dict[str, list[str]],
    failures: list[str],
) -> None:
    rows = profile["units"]["widths"]
    keys = [(row["name"], row["file"], row["symbol"]) for row in rows]
    if keys != sorted(keys):
        failures.append("FAIL: units.widths is not sorted by name, file, and symbol")
    for row in rows:
        name = row["name"]
        match = WIDTH_NAME.fullmatch(name)
        if match is None or int(match.group(1)) != row["bits"]:
            failures.append(f"FAIL: width {name} bits do not match the type name")
            continue
        lines = sources.get(row["file"])
        if lines is None:
            failures.append(f"FAIL: width file {row['file']} is missing")
            continue
        text = "\n".join(lines)
        if re.search(rf"\b{re.escape(row['symbol'])}\b", text) is None:
            failures.append(
                f"FAIL: width symbol {row['symbol']} does not occur in {row['file']}"
            )
        span = numeric_fits_span(lines)
        if span is None:
            failures.append(
                f"FAIL: width {name} has no numericFits function in {row['file']}"
            )
            continue
        start, end = span
        reason = width_bound_mismatch(name, int(row["bits"]), "\n".join(lines[start - 1 : end]))
        if reason is not None:
            failures.append(
                f"FAIL: width {name} numericFits at {row['file']}:{start} {reason}"
            )


def numeric_fits_span(lines: list[str]) -> tuple[int, int] | None:
    spans = [span for span in function_spans(lines) if span[2] == "numericFits"]
    if len(spans) != 1:
        return None
    return spans[0][0], spans[0][1]


def width_bound_mismatch(name: str, bits: int, body: str) -> str | None:
    """Match the numericFits range, not an unrelated constant in the same file."""
    code = mask_non_code(body, blank_strings=False)
    if name.startswith("SInt"):
        shift = bits - 1
        signed = re.compile(
            rf"'{re.escape(name)}'[\s\S]{{0,240}}"
            rf"return\s+n\s*>=\s*-\(\s*1n\s*<<\s*{shift}n\s*\)\s*&&\s*"
            rf"n\s*<\s*\(\s*1n\s*<<\s*{shift}n\s*\)"
        )
        if signed.search(code) is None:
            return f"does not bound {name} with 1n << {shift}n"
        return None
    match = re.search(
        r"return\s+n\s*>=\s*0n\s*&&\s*n\s*<\s*\(\s*1n\s*<<\s*\((?P<expr>.*)\)\s*\)\s*;",
        code,
        re.S,
    )
    if match is None:
        return "has no unsigned numericFits return"
    if re.search(rf"'{re.escape(name)}'", code[: match.start()]) is not None:
        return f"handles {name} before the unsigned return"
    mapped = unsigned_shift_bits(match.group("expr"), name)
    if mapped != bits:
        shown = "no width" if mapped is None else f"{mapped}n"
        return f"maps {name} to {shown}, not {bits}n"
    return None


def unsigned_shift_bits(expr: str, tag: str) -> int | None:
    token = eval_shift_ternary(expr.strip(), tag)
    if token is None:
        return None
    match = re.fullmatch(r"(\d+)n", token)
    if match is None:
        return None
    return int(match.group(1))


def eval_shift_ternary(expr: str, tag: str) -> str | None:
    expr = expr.strip()
    question = top_level(expr, "?")
    if question is None:
        compact = re.sub(r"\s+", "", expr)
        if re.fullmatch(r"\d+n", compact):
            return compact
        return None
    rest = expr[question + 1 :]
    colon = top_level(rest, ":")
    if colon is None:
        return None
    if condition_selects(expr[:question], tag):
        return eval_shift_ternary(rest[:colon], tag)
    return eval_shift_ternary(rest[colon + 1 :], tag)


def top_level(expr: str, token: str) -> int | None:
    depth = 0
    quote: str | None = None
    index = 0
    while index < len(expr):
        ch = expr[index]
        if quote is not None:
            if ch == "\\":
                index += 2
                continue
            if ch == quote:
                quote = None
            index += 1
            continue
        if ch in "'\"`":
            quote = ch
            index += 1
            continue
        if ch in "([{":
            depth += 1
        elif ch in ")]}":
            depth = max(0, depth - 1)
        elif depth == 0 and expr.startswith(token, index):
            return index
        index += 1
    return None


def condition_selects(condition: str, tag: str) -> bool:
    if re.search(rf"\btag\s*===\s*'{re.escape(tag)}'", condition):
        return True
    listed = re.search(
        r"\[(.*?)\]\s*\.\s*includes\(\s*tag\s*\)",
        condition,
        re.S,
    )
    if listed is None:
        return False
    return tag in re.findall(r"'([^']*)'", listed.group(1))


def check_vectors(profile: dict[str, Any], failures: list[str]) -> None:
    vectors = profile["defiformalConversion"]["testVectors"]
    keys = [
        (
            row["quotePerBaseMantissa"],
            row["quotePerBaseScale"],
            row["targetScale"],
            row["role"],
        )
        for row in vectors
    ]
    if keys != sorted(keys):
        failures.append("FAIL: defiformal test vectors are not sorted")
    for index, row in enumerate(vectors):
        source_scale = int(row["quotePerBaseScale"])
        target_scale = int(row["targetScale"])
        expected_value = int(row["expectedBasePerQuoteMantissa"])
        if source_scale > MAX_SCALE or target_scale > MAX_SCALE:
            failures.append(
                f"FAIL: defiformal test vector {index} scale exceeds {MAX_SCALE}"
            )
            continue
        if expected_value > MAX_RECIPROCAL:
            failures.append(
                f"FAIL: defiformal test vector {index} reciprocal exceeds 10^{MAX_SCALE * 2}"
            )
            continue
        expected = base_per_quote_mantissa(
            int(row["quotePerBaseMantissa"]),
            source_scale,
            target_scale,
            str(row["role"]),
        )
        if expected is None or expected_value != expected:
            failures.append(
                "FAIL: defiformal test vector "
                f"{index} expected {expected_value} "
                f"but formula gives {expected}"
            )


def base_per_quote_mantissa(
    mantissa: int, source_scale: int, target_scale: int, role: str
) -> int | None:
    if mantissa <= 0 or source_scale < 0 or target_scale < 0:
        return None
    if source_scale > MAX_SCALE or target_scale > MAX_SCALE:
        return None
    numerator = 10 ** (source_scale + target_scale)
    if role == "receipt":
        return numerator // mantissa
    if role == "obligation":
        return (numerator + mantissa - 1) // mantissa
    return None


def check_primitives(
    profile: dict[str, Any],
    sources: dict[str, list[str]],
    masked: dict[str, list[str]],
    failures: list[str],
) -> tuple[int, int, int]:
    rows = profile["primitives"]
    ids = [row["id"] for row in rows]
    if ids != sorted(ids):
        failures.append("FAIL: primitives are not sorted by id")
    if len(ids) != len(set(ids)):
        failures.append("FAIL: primitive ids are not unique")
    gaps = 0
    beneficiary_gaps = 0
    for row in rows:
        row_failures, beneficiary_gap = check_primitive(row, sources, masked)
        failures.extend(row_failures)
        if row["conformance"] == "open-gap":
            gaps += 1
        if beneficiary_gap:
            beneficiary_gaps += 1
    return len(rows), gaps, beneficiary_gaps


def check_primitive(
    row: dict[str, Any],
    sources: dict[str, list[str]],
    masked: dict[str, list[str]],
) -> tuple[list[str], bool]:
    failures: list[str] = []
    identity = str(row["id"])
    lines = sources.get(str(row["file"]))
    code_lines = masked.get(str(row["file"]))
    if lines is None or code_lines is None:
        return [f"FAIL: primitive {identity} file {row['file']} is missing"], False
    if re.search(rf"\b{re.escape(str(row['symbol']))}\b", "\n".join(lines)) is None:
        failures.append(
            f"FAIL: primitive {identity} symbol {row['symbol']} "
            f"does not occur in {row['file']}"
        )
    line_number = int(row["line"])
    if line_number > len(lines):
        return failures + [
            f"FAIL: primitive {identity} line {line_number} "
            f"is past the end of {row['file']}"
        ], False
    line_text = lines[line_number - 1]
    code_text = code_lines[line_number - 1]
    required = str(row["requiredDirection"])
    if not line_has_operation(line_text, code_text, required):
        failures.append(
            f"FAIL: primitive {identity} line {line_number} "
            "has no division, rounding, scaling, or overflow-checked operation"
        )
    if DIVISION.search(code_text) and required == "none":
        failures.append(
            f"FAIL: primitive {identity} divides and cannot use requiredDirection none"
        )
    span = enclosing_function(lines, line_number)
    if span is None:
        failures.append(
            f"FAIL: primitive {identity} line {line_number} is not inside a function"
        )
        return failures, False
    start, end, name = span
    body = "\n".join(lines[start - 1 : end])
    kept_body = mask_non_code(body, blank_strings=False)
    symbol = str(row["symbol"])
    if symbol != name and re.search(rf"\b{re.escape(symbol)}\b", body) is None:
        failures.append(
            f"FAIL: primitive {identity} symbol {symbol} "
            f"is not in function {name}"
        )
    selectable = author_selectable(kept_body, code_text, required)
    if bool(row["authorSelectable"]) != selectable:
        failures.append(
            f"FAIL: primitive {identity} authorSelectable does not match the source"
        )
    fixed = None if selectable else fixed_direction(kept_body, required)
    reasons = conformance_reasons(selectable, required, fixed)
    expected = "open-gap" if reasons else "conforms"
    if row["conformance"] != expected:
        detail = "; ".join(reasons) if reasons else "the source matches the policy"
        failures.append(
            f"FAIL: primitive {identity} conformance is {row['conformance']} "
            f"but the source requires {expected} ({detail})"
        )
    posted = remainder_posted(row, sources, masked)
    if required != "none":
        check_beneficiary_claim(row, identity, posted, failures)
    if DIVISION.search(code_text) and str(row["resultRole"]) != "exact":
        roles = required_roles(str(row["file"]), line_number, sources, masked)
        if str(row["resultRole"]) not in roles:
            failures.append(
                f"FAIL: primitive {identity} resultRole {row['resultRole']} "
                f"is not derived for {row['file']}:{line_number}"
            )
    direction = ROLE_DIRECTION[str(row["resultRole"])]
    if row["requiredDirection"] != direction:
        failures.append(
            f"FAIL: primitive {identity} requiredDirection does not match {row['resultRole']}"
        )
    if row["requiredDirection"] == "none":
        if row["remainderBeneficiary"] is not None:
            failures.append(
                f"FAIL: primitive {identity} remainderBeneficiary must be null"
            )
    elif row["remainderBeneficiary"] != "protocol-reserve":
        failures.append(
            f"FAIL: primitive {identity} remainderBeneficiary must be protocol-reserve"
        )
    if row["conformance"] == "open-gap":
        if not isinstance(row["gapNote"], str) or not row["gapNote"].strip():
            failures.append(f"FAIL: primitive {identity} open-gap requires gapNote")
    elif row["gapNote"] is not None:
        failures.append(f"FAIL: primitive {identity} conforms requires a null gapNote")
    beneficiary_gap = required != "none" and not posted
    return failures, beneficiary_gap


def line_has_operation(raw_line: str, code_line: str, required: str) -> bool:
    if required == "none":
        return (
            OVERFLOW_LINE.search(code_line) is not None
            and DIVISION.search(code_line) is None
        )
    return (
        DIVISION.search(code_line) is not None
        or ROUNDING_OR_SCALE.search(raw_line) is not None
    )


def author_selectable(body: str, line_text: str, required: str) -> bool:
    """True when the cited operation lets the program choose floor, ceil, or none."""
    if required == "none" or DIVISION.search(line_text) is None:
        return False
    if "FloorDiv" in body and "CeilDiv" in body:
        return True
    return re.search(r"\.rounding\s*===", body) is not None


def fixed_direction(body: str, required: str) -> str | None:
    if required == "none":
        return "none"
    if DIVISION.search(body) is None:
        return None
    if re.search(r"!==\s*0n", body) is not None and re.search(r"\+\s*1n", body) is not None:
        return "ceil"
    return "floor"


def conformance_reasons(
    selectable: bool,
    required_direction: str,
    fixed: str | None,
) -> list[str]:
    """Open-gap iff the author can select rounding or the fixed direction differs.

    A missing protocol-reserve posting is a beneficiary gap. It does not change
    this direction result.
    """
    if selectable:
        return ["author can select the rounding"]
    if fixed is None:
        return ["the source has no fixed direction"]
    if fixed != required_direction:
        return [
            f"fixed direction {fixed} differs from required {required_direction}"
        ]
    return []


def check_beneficiary_claim(
    row: dict[str, Any],
    identity: str,
    posted: bool,
    failures: list[str],
) -> None:
    description = str(row["description"])
    says_posted = BENEFICIARY_POSTED.search(description) is not None
    says_caveat = BENEFICIARY_CAVEAT.search(description) is not None
    if says_posted == says_caveat:
        failures.append(
            f"FAIL: primitive {identity} description must state whether "
            "the remainder is posted to a protocol reserve"
        )
    elif says_posted != posted:
        if posted:
            failures.append(
                f"FAIL: primitive {identity} description must state that "
                "the remainder is posted to a protocol reserve"
            )
        else:
            failures.append(
                f"FAIL: primitive {identity} description must state that "
                "the remainder is not posted to a protocol reserve"
            )
    if row["conformance"] != "open-gap":
        return
    note = row["gapNote"] if isinstance(row["gapNote"], str) else ""
    note_caveat = BENEFICIARY_CAVEAT.search(note) is not None
    if posted and note_caveat:
        failures.append(
            f"FAIL: primitive {identity} gapNote still says that "
            "the remainder is not posted to a protocol reserve"
        )
    if not posted and not note_caveat:
        failures.append(
            f"FAIL: primitive {identity} gapNote must state that "
            "the remainder is not posted to a protocol reserve"
        )


def remainder_posted(
    row: dict[str, Any],
    sources: dict[str, list[str]],
    masked: dict[str, list[str]],
) -> bool:
    """True when this division's own remainder is posted to a protocol reserve.

    The posting must name the remainder bound beside this division, or the
    modulus of this division's numerator and divisor. The reserve name must be
    in scope, and a later reachable read or a parameter-object write must
    observe it. closureReserve is not that reserve. A posting in a caller
    counts only for the caller that supplies this row's role, and only when
    the posted value is still this division's remainder.
    """
    if str(row["requiredDirection"]) == "none":
        return True
    relative = str(row["file"])
    raw = sources.get(relative)
    code = masked.get(relative)
    if raw is None or code is None:
        return False
    number = int(row["line"])
    if number < 1 or number > len(code):
        return False
    division = parse_division(code[number - 1])
    if division is None:
        return False
    span = enclosing_function(raw, number)
    if span is None:
        return False
    if region_posts_remainder(raw, code, span[0], span[1], number, division, ""):
        return True
    for entry in ROLE_EVIDENCE:
        if (
            entry.file != relative
            or entry.line != number
            or entry.role != str(row["resultRole"])
            or entry.region != "caller"
        ):
            continue
        caller = named_function(raw, entry.caller)
        if caller is None:
            continue
        if not re.search(rf"\b{re.escape(span[2])}\s*\(", "\n".join(code[caller[0] - 1 : caller[1]])):
            continue
        result = call_result_name(code[caller[0] - 1 : caller[1]], span[2])
        if result is None:
            continue
        if region_posts_remainder(
            raw, code, caller[0], caller[1], number, division, result
        ):
            return True
    return False


def parse_division(line: str) -> Division | None:
    match = DIVISION_ASSIGN.search(line)
    if match is None:
        return None
    return Division(match.group("quotient"), match.group("numerator"), match.group("divisor"))


def region_posts_remainder(
    raw: list[str],
    code: list[str],
    start: int,
    end: int,
    division_line: int,
    division: Division,
    result_name: str,
) -> bool:
    """Look for a real posting. division_line may lie outside a caller region."""
    blocks = block_ids(code)
    dead = dead_block_ids(code, blocks)
    scope_params = parameter_names(raw, start)
    functions = {item[2] for item in function_spans(raw)}
    division_inside = start <= division_line <= end
    names = remainder_bindings(
        code, blocks, dead, start, end, division_line, division
    )
    direct = re.sub(r"\s+", "", f"{division.numerator}%{division.divisor}")
    for index in range(start - 1, end):
        if division_inside and not on_path(code, blocks, dead, division_line - 1, index):
            continue
        for match in UPDATE.finditer(code[index]):
            lhs = match.group("lhs")
            if reserve_component(lhs) is None:
                continue
            rhs = match.group("rhs")
            if not rhs_is_remainder(match.group("op"), lhs, rhs, names, direct):
                continue
            if result_name and re.search(rf"\b{re.escape(result_name)}\b", rhs) is None:
                continue
            scope = names_in_scope(
                code, blocks, scope_params, index, match.start()
            )
            if not lhs_is_valid(lhs, scope, raw) or not rhs_is_in_scope(rhs, scope, functions):
                continue
            if effect_escapes(lhs, scope_params, code, blocks, dead, index, match.end()):
                return True
    return False


def remainder_bindings(
    code: list[str],
    blocks: list[tuple[int, ...]],
    dead: set[tuple[int, ...]],
    start: int,
    end: int,
    division_line: int,
    division: Division,
) -> dict[str, int]:
    """Names bound to this division's modulus within a few lines of it."""
    if not (start <= division_line <= end):
        return {}
    binding = re.compile(
        rf"(?:const|let|var|,)\s*([A-Za-z_]\w*)\s*=\s*"
        rf"{re.escape(division.numerator)}\s*%\s*{re.escape(division.divisor)}\b"
    )
    found: dict[str, int] = {}
    last = min(end, division_line + REMAINDER_WINDOW) 
    for number in range(division_line, last + 1):
        index = number - 1
        if not on_path(code, blocks, dead, division_line - 1, index):
            continue
        for match in binding.finditer(code[index]):
            found[match.group(1)] = index
    return found


def rhs_is_remainder(
    op: str,
    lhs: str,
    rhs: str,
    names: dict[str, int],
    direct: str,
) -> bool:
    compact = re.sub(r"\s+", "", rhs)
    lhs_compact = re.sub(r"\s+", "", lhs)
    atoms = set(names) | {direct}
    if op == "+=":
        return compact in atoms
    if compact in atoms:
        return True
    return any(
        compact == f"{lhs_compact}+{atom}"
        or compact == f"addU128({lhs_compact},{atom})"
        or compact == f"addU64({lhs_compact},{atom})"
        for atom in atoms
    )


def rhs_identifiers(rhs: str) -> list[str]:
    return re.findall(r"[A-Za-z_]\w*", rhs)


def rhs_is_in_scope(rhs: str, scope: set[str], functions: set[str]) -> bool:
    for name in rhs_identifiers(rhs):
        if name in scope or name in functions or name in {"BigInt"}:
            continue
        return False
    return True


def reserve_component(lhs: str) -> str | None:
    parts = re.split(r"\s*\.\s*", lhs.strip())
    last = parts[-1]
    if last in RESERVE_NAMES:
        return last
    return None


def lhs_is_valid(lhs: str, scope: set[str], raw: list[str]) -> bool:
    parts = re.split(r"\s*\.\s*", lhs.strip())
    if parts[0] not in scope:
        return False
    if len(parts) == 1:
        return True
    text = "\n".join(raw)
    prop = parts[-1]
    if re.search(rf"^\s*{re.escape(prop)}\s*:", text, re.M):
        return True
    return re.search(rf"['\"]{re.escape(prop)}['\"]", text) is not None


def effect_escapes(
    lhs: str,
    params: set[str],
    code: list[str],
    blocks: list[tuple[int, ...]],
    dead: set[tuple[int, ...]],
    posting_index: int,
    after_column: int,
) -> bool:
    parts = re.split(r"\s*\.\s*", lhs.strip())
    if len(parts) > 1 and parts[0] in params:
        return True
    pattern = re.compile(rf"\b{re.escape(parts[0])}\b")
    if pattern.search(code[posting_index][after_column:]):
        return True
    for index in range(posting_index + 1, len(code)):
        if not on_path(code, blocks, dead, posting_index, index):
            continue
        if pattern.search(code[index]):
            return True
    return False


def names_in_scope(
    code: list[str],
    blocks: list[tuple[int, ...]],
    params: set[str],
    index: int,
    before_column: int,
) -> set[str]:
    scope = set(params)
    decl = re.compile(r"(?:(?:const|let|var)\s+|,\s*)([A-Za-z_]\w*)\s*=")
    for earlier in range(index):
        if not is_prefix(blocks[earlier], blocks[index]):
            continue
        scope.update(decl.findall(code[earlier]))
    scope.update(decl.findall(code[index][:before_column]))
    return scope


def parameter_names(lines: list[str], start: int) -> set[str]:
    chunk = "\n".join(lines[start - 1 : start + 20])
    brace = chunk.find("{")
    header = chunk if brace < 0 else chunk[:brace]
    paren = header.find("(")
    if paren < 0:
        return set()
    return set(re.findall(r"\b([A-Za-z_]\w*)\s*:", header[paren + 1 :]))


def call_result_name(lines: list[str], callee: str) -> str | None:
    match = re.search(
        rf"(?:const|let)\s+([A-Za-z_]\w*)\s*=\s*{re.escape(callee)}\s*\(",
        "\n".join(lines),
    )
    if match is None:
        return None
    return match.group(1)


def named_function(
    lines: list[str], name: str
) -> tuple[int, int, str] | None:
    matches = [span for span in function_spans(lines) if span[2] == name]
    if len(matches) != 1:
        return None
    return matches[0]


def block_ids(lines: list[str]) -> list[tuple[int, ...]]:
    stack = [0]
    next_id = 1
    ids: list[tuple[int, ...]] = []
    for line in lines:
        ids.append(tuple(stack))
        for ch in line:
            if ch == "{":
                stack.append(next_id)
                next_id += 1
            elif ch == "}" and len(stack) > 1:
                stack.pop()
    return ids


def dead_block_ids(
    lines: list[str], blocks: list[tuple[int, ...]]
) -> set[tuple[int, ...]]:
    dead: set[tuple[int, ...]] = set()
    for index, line in enumerate(lines):
        if re.search(r"\bif\s*\(\s*(?:false|0|0n)\s*\)", line) is None or "{" not in line:
            continue
        if index + 1 < len(lines) and len(blocks[index + 1]) > len(blocks[index]):
            dead.add(blocks[index + 1])
    return dead


def is_prefix(outer: tuple[int, ...], inner: tuple[int, ...]) -> bool:
    return inner[: len(outer)] == outer


def on_path(
    lines: list[str],
    blocks: list[tuple[int, ...]],
    dead: set[tuple[int, ...]],
    origin: int,
    index: int,
) -> bool:
    if index < origin or index >= len(lines):
        return False
    source = blocks[origin]
    dest = blocks[index]
    if not (is_prefix(source, dest) or is_prefix(dest, source)):
        return False
    if any(is_prefix(item, dest) for item in dead):
        return False
    for between in range(origin, index):
        if re.match(r"\s*return\b", lines[between]) and is_prefix(blocks[between], dest):
            return False
    return True


def check_coverage(
    profile: dict[str, Any],
    sources: dict[str, list[str]],
    masked: dict[str, list[str]],
    failures: list[str],
) -> None:
    rows = profile["primitives"]
    citations = [
        (str(row["file"]), int(row["line"]))
        for row in rows
    ]
    check_ignored(IGNORED_ROUNDING_SITES, sources, ROUNDING_WORD, "rounding", failures)
    check_ignored(IGNORED_DIVISION_SITES, masked, DIVISION, "division", failures)
    check_role_evidence(sources, masked, failures)
    detected = overflow_sites(sources, masked)
    check_overflow_coverage(rows, detected, failures)
    ignored_rounding = {(site[0], site[1]) for site in IGNORED_ROUNDING_SITES}
    ignored_division = {(site[0], site[1]) for site in IGNORED_DIVISION_SITES}
    for relative, lines in sources.items():
        code_lines = masked[relative]
        spans = function_spans(lines)
        for number, line in enumerate(lines, start=1):
            if ROUNDING_WORD.search(line) and in_arithmetic_helper(
                spans, lines, code_lines, number
            ):
                site = (relative, number)
                covered = covered_by(site, citations)
                if site in ignored_rounding:
                    if covered:
                        failures.append(
                            f"FAIL: ignored rounding site {relative}:{number} "
                            "is already covered"
                        )
                elif not covered:
                    failures.append(
                        f"FAIL: rounding site {relative}:{number} "
                        "is not covered by a primitive within 15 lines"
                    )
            if DIVISION.search(code_lines[number - 1]) is None:
                continue
            site = (relative, number)
            if site in ignored_division:
                continue
            if not covered_by(site, citations):
                failures.append(
                    f"FAIL: division site {relative}:{number} "
                    "is not covered by a primitive within 15 lines"
                )
            roles = required_roles(relative, number, sources, masked)
            if not roles:
                failures.append(
                    f"FAIL: division site {relative}:{number} has no derived result role"
                )
                continue
            for role in sorted(roles):
                if not role_covered(relative, number, role, rows):
                    failures.append(
                        f"FAIL: division site {relative}:{number} role {role} "
                        "is not covered by a primitive within 15 lines"
                    )


def required_roles(
    relative: str,
    number: int,
    sources: dict[str, list[str]],
    masked: dict[str, list[str]],
) -> set[str]:
    """Roles from ROLE_EVIDENCE entries whose patterns still match the source."""
    roles: set[str] = set()
    for entry in ROLE_EVIDENCE:
        if entry.file != relative or entry.line != number:
            continue
        text = evidence_text(entry, sources, masked)
        if text is not None and re.search(entry.pattern, text) is not None:
            roles.add(entry.role)
    return roles


def check_role_evidence(
    sources: dict[str, list[str]],
    masked: dict[str, list[str]],
    failures: list[str],
) -> None:
    for entry in ROLE_EVIDENCE:
        if entry.region not in {"division-function", "caller"}:
            failures.append(
                f"FAIL: role evidence {entry.file}:{entry.line} has region {entry.region}"
            )
            continue
        lines = masked.get(entry.file)
        if lines is None or entry.line < 1 or entry.line > len(lines):
            failures.append(
                f"FAIL: role evidence {entry.file}:{entry.line} does not exist"
            )
            continue
        if DIVISION.search(lines[entry.line - 1]) is None:
            failures.append(
                f"FAIL: role evidence {entry.file}:{entry.line} is not a division"
            )
        text = evidence_text(entry, sources, masked)
        if text is None or re.search(entry.pattern, text) is None:
            failures.append(
                f"FAIL: role evidence {entry.file}:{entry.line} "
                f"{entry.role} does not match the source"
            )


def evidence_text(
    entry: RoleEvidence,
    sources: dict[str, list[str]],
    masked: dict[str, list[str]],
) -> str | None:
    raw = sources.get(entry.file)
    if raw is None or entry.file not in masked:
        return None
    kept = comment_masked_lines(raw)
    if entry.region == "division-function":
        span = enclosing_function(raw, entry.line)
        if span is None:
            return None
        return "\n".join(kept[span[0] - 1 : span[1]])
    caller = named_function(raw, entry.caller)
    division = enclosing_function(raw, entry.line)
    if caller is None or division is None:
        return None
    body = "\n".join(kept[caller[0] - 1 : caller[1]])
    if re.search(rf"\b{re.escape(division[2])}\s*\(", body) is None:
        return None
    return body


def overflow_sites(
    sources: dict[str, list[str]],
    masked: dict[str, list[str]],
) -> list[tuple[str, int, frozenset[str]]]:
    """Overflow-checked helper definitions and Add/Sub/Mul numericFits guards."""
    found: list[tuple[str, int, frozenset[str]]] = []
    for relative, raw in sources.items():
        code = comment_masked_lines(raw)
        spans = function_spans(raw)
        for start, end, name in spans:
            body = "\n".join(code[start - 1 : end])
            match = OVERFLOW_RETURN.search(body)
            if match is not None:
                line = start + body[: match.start()].count("\n")
                owner = innermost(spans, line)
                if owner is not None and owner[0] == start and owner[2] == name:
                    found.append((relative, line, frozenset({name})))
            body_lines = code[start - 1 : end]
            add_at = next(
                (
                    index
                    for index, line in enumerate(body_lines)
                    if re.search(r"k\s*===\s*'Add'\)\s*n\s*=", line)
                ),
                None,
            )
            if add_at is None:
                continue
            window = body_lines[add_at : add_at + 16]
            if not any(re.search(r"k\s*===\s*'Sub'\)\s*n\s*=", line) for line in window):
                continue
            if not any(re.search(r"k\s*===\s*'Mul'\)\s*n\s*=", line) for line in window):
                continue
            fits = next(
                (index for index, line in enumerate(window) if "numericFits" in line),
                None,
            )
            if fits is None:
                continue
            line = start + add_at + fits
            owner = innermost(spans, line)
            if owner is not None and owner[0] == start and owner[2] == name:
                found.append((relative, line, frozenset({"Add", "Sub", "Mul"})))
    return found


def check_overflow_coverage(
    rows: list[dict[str, Any]],
    detected: list[tuple[str, int, frozenset[str]]],
    failures: list[str],
) -> None:
    detected_at = {(item[0], item[1]) for item in detected}
    ignored = {(site[0], site[1]) for site in IGNORED_OVERFLOW_SITES}
    seen: set[tuple[str, int]] = set()
    for relative, number, reason in IGNORED_OVERFLOW_SITES:
        if not reason.strip():
            failures.append(f"FAIL: ignored overflow site {relative}:{number} has no reason")
        if (relative, number) in seen:
            failures.append(f"FAIL: ignored overflow site {relative}:{number} is duplicated")
        seen.add((relative, number))
        if (relative, number) not in detected_at:
            failures.append(
                f"FAIL: ignored overflow site {relative}:{number} "
                "is not an overflow-checked arithmetic site"
            )
    for relative, number, symbols in detected:
        covered = [
            row
            for row in rows
            if str(row["file"]) == relative
            and str(row["resultRole"]) == "exact"
            and abs(int(row["line"]) - number) <= 15
        ]
        if (relative, number) in ignored:
            if covered:
                failures.append(
                    f"FAIL: ignored overflow site {relative}:{number} is already covered"
                )
            continue
        for symbol in sorted(symbols):
            if not any(str(row["symbol"]) == symbol for row in covered):
                failures.append(
                    f"FAIL: overflow site {relative}:{number} symbol {symbol} "
                    "is not covered by an exact primitive within 15 lines"
                )


def role_covered(
    relative: str, number: int, role: str, rows: list[dict[str, Any]]
) -> bool:
    return any(
        str(row["file"]) == relative
        and str(row["resultRole"]) == role
        and abs(int(row["line"]) - number) <= 15
        for row in rows
    )


def check_ignored(
    sites: tuple[tuple[str, int, str], ...],
    sources: dict[str, list[str]],
    pattern: re.Pattern[str],
    label: str,
    failures: list[str],
) -> None:
    seen: set[tuple[str, int]] = set()
    for relative, number, reason in sites:
        if not reason.strip():
            failures.append(f"FAIL: ignored {label} site {relative}:{number} has no reason")
        if (relative, number) in seen:
            failures.append(f"FAIL: ignored {label} site {relative}:{number} is duplicated")
        seen.add((relative, number))
        lines = sources.get(relative)
        if lines is None or number < 1 or number > len(lines):
            failures.append(f"FAIL: ignored {label} site {relative}:{number} does not exist")
            continue
        if pattern.search(lines[number - 1]) is None:
            failures.append(
                f"FAIL: ignored {label} site {relative}:{number} does not match {label}"
            )


def covered_by(site: tuple[str, int], citations: list[tuple[str, int]]) -> bool:
    relative, number = site
    return any(
        cited_file == relative and abs(cited_line - number) <= 15
        for cited_file, cited_line in citations
    )


def in_arithmetic_helper(
    spans: list[tuple[int, int, str]],
    lines: list[str],
    masked: list[str],
    number: int,
) -> bool:
    span = innermost(spans, number)
    if span is None:
        return False
    start, end, _name = span
    raw_body = "\n".join(lines[start - 1 : end])
    code_body = "\n".join(masked[start - 1 : end])
    return DIVISION.search(code_body) is not None and ROUNDING_WORD.search(raw_body) is not None


def enclosing_function(
    lines: list[str], number: int
) -> tuple[int, int, str] | None:
    return innermost(function_spans(lines), number)


def innermost(
    spans: list[tuple[int, int, str]], number: int
) -> tuple[int, int, str] | None:
    containing = [span for span in spans if span[0] <= number <= span[1]]
    if not containing:
        return None
    return min(containing, key=lambda span: span[1] - span[0])


def function_spans(lines: list[str]) -> list[tuple[int, int, str]]:
    starts: list[tuple[int, str, int]] = []
    for number, line in enumerate(lines, start=1):
        function = FUNCTION_START.match(line)
        if function is not None:
            starts.append((number, function.group(1), 0))
            continue
        method = METHOD_START.match(line)
        if method is not None and method.group(2) not in CONTROL_WORDS:
            starts.append((number, method.group(2), len(method.group(1))))
    spans: list[tuple[int, int, str]] = []
    for index, (start, name, indent) in enumerate(starts):
        end = len(lines)
        for later_start, _later_name, later_indent in starts[index + 1 :]:
            if later_indent <= indent:
                end = later_start - 1
                break
        spans.append((start, end, name))
    return spans


if __name__ == "__main__":
    raise SystemExit(main())
