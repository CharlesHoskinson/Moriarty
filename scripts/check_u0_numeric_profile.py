#!/usr/bin/env python3
"""Check the U0 numeric profile against the owner decision and successor sources."""

from __future__ import annotations

import argparse
import hashlib
import json
import re
from pathlib import Path
from typing import Any

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
# A posting of the modulus or remainder onto a protocol-reserve name.
# closureReserve is work accounting and does not match this name.
RESERVE_POSTING = re.compile(
    r"protocol[-_ ]?reserve\w*(?:\s*\.\s*\w+)*\s*(?:\+=|=(?!=))"
    r"\s*[^=;\n]*(?:remainder|%\s*\w+)",
    re.IGNORECASE,
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
    primitives, gaps = counts or (0, 0)
    print(f"OK: {primitives} primitives, {gaps} open conformance gaps")
    return 0


def check(root: Path) -> tuple[str | None, list[str], tuple[int, int] | None]:
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
    rendered = json.dumps(profile, indent=2) + "\n"
    if profile_text != rendered:
        failures.append(
            "FAIL: numeric profile is not UTF-8 JSON with 2-space indent"
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
    check_price_types(profile, sources, failures)
    check_widths(profile, sources, failures)
    check_vectors(profile, failures)
    primitive_count, gap_count = check_primitives(profile, sources, failures)
    check_coverage(profile, sources, failures)
    return None, failures, (primitive_count, gap_count)


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
        if ch == "/" and not operand_before(out):
            # A slash that does not follow an operand is a regex literal,
            # including its flags. It is not the division operator.
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
    if not lines:
        return []
    masked = mask_non_code("\n".join(lines)).split("\n")
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
        expected = base_per_quote_mantissa(
            row["quotePerBaseMantissa"],
            row["quotePerBaseScale"],
            row["targetScale"],
            row["role"],
        )
        if row["expectedBasePerQuoteMantissa"] != expected:
            failures.append(
                "FAIL: defiformal test vector "
                f"{index} expected {row['expectedBasePerQuoteMantissa']} "
                f"but formula gives {expected}"
            )


def base_per_quote_mantissa(
    mantissa: int, source_scale: int, target_scale: int, role: str
) -> int:
    numerator = 10 ** (source_scale + target_scale)
    if role == "receipt":
        return numerator // mantissa
    if role == "obligation":
        return (numerator + mantissa - 1) // mantissa
    raise ValueError(role)


def check_primitives(
    profile: dict[str, Any],
    sources: dict[str, list[str]],
    failures: list[str],
) -> tuple[int, int]:
    rows = profile["primitives"]
    ids = [row["id"] for row in rows]
    if ids != sorted(ids):
        failures.append("FAIL: primitives are not sorted by id")
    if len(ids) != len(set(ids)):
        failures.append("FAIL: primitive ids are not unique")
    gaps = 0
    for row in rows:
        failures.extend(check_primitive(row, sources))
        if row["conformance"] == "open-gap":
            gaps += 1
    return len(rows), gaps


def check_primitive(
    row: dict[str, Any], sources: dict[str, list[str]]
) -> list[str]:
    failures: list[str] = []
    identity = str(row["id"])
    lines = sources.get(str(row["file"]))
    if lines is None:
        return [f"FAIL: primitive {identity} file {row['file']} is missing"]
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
        ]
    try:
        code_lines = masked_lines(lines)
    except ValueError as error:
        return failures + [f"FAIL: {row['file']} {error}"]
    line_text = lines[line_number - 1]
    code_text = code_lines[line_number - 1]
    if DIVISION.search(code_text) is None and ROUNDING_OR_SCALE.search(line_text) is None:
        failures.append(
            f"FAIL: primitive {identity} line {line_number} "
            "has no division, rounding, or scaling operation"
        )
    if DIVISION.search(code_text) and row["requiredDirection"] == "none":
        failures.append(
            f"FAIL: primitive {identity} divides and cannot use requiredDirection none"
        )
    span = enclosing_function(lines, line_number)
    if span is None:
        failures.append(
            f"FAIL: primitive {identity} line {line_number} is not inside a function"
        )
        return failures
    start, end, name = span
    body = "\n".join(lines[start - 1 : end])
    symbol = str(row["symbol"])
    if symbol != name and re.search(rf"\b{re.escape(symbol)}\b", body) is None:
        failures.append(
            f"FAIL: primitive {identity} symbol {symbol} "
            f"is not in function {name}"
        )
    selectable = author_selectable(body)
    if bool(row["authorSelectable"]) != selectable:
        failures.append(
            f"FAIL: primitive {identity} authorSelectable does not match the source"
        )
    fixed = None if selectable else fixed_direction(body)
    beneficiary_ok = remainder_reaches_protocol_reserve(row, name, body, sources)
    reasons = conformance_reasons(
        selectable, str(row["requiredDirection"]), fixed, beneficiary_ok
    )
    expected = "open-gap" if reasons else "conforms"
    if row["conformance"] != expected:
        detail = "; ".join(reasons) if reasons else "the source matches the policy"
        failures.append(
            f"FAIL: primitive {identity} conformance is {row['conformance']} "
            f"but the source requires {expected} ({detail})"
        )
    if str(row["requiredDirection"]) != "none" and not beneficiary_ok:
        note = row["gapNote"]
        if not isinstance(note, str) or BENEFICIARY_CAVEAT.search(note) is None:
            failures.append(
                f"FAIL: primitive {identity} remainder is not posted to a protocol reserve"
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
    return failures


def author_selectable(body: str) -> bool:
    if "FloorDiv" in body and "CeilDiv" in body:
        return True
    return re.search(r"\.rounding\s*===", body) is not None


def fixed_direction(body: str) -> str | None:
    if DIVISION.search(mask_non_code(body)) is None:
        return None
    if (
        re.search(r"remainder\s*!==\s*0n", body) is not None
        and re.search(r"\+\s*1n", body) is not None
    ):
        return "ceil"
    return "floor"


def conformance_reasons(
    selectable: bool,
    required_direction: str,
    fixed: str | None,
    beneficiary_ok: bool,
) -> list[str]:
    reasons: list[str] = []
    if selectable:
        reasons.append("author can select the rounding")
    elif fixed is None:
        reasons.append("the source has no fixed direction")
    elif fixed != required_direction:
        reasons.append(
            f"fixed direction {fixed} differs from required {required_direction}"
        )
    if required_direction != "none" and not beneficiary_ok:
        reasons.append("remainder is not posted to a protocol reserve")
    return reasons


def remainder_reaches_protocol_reserve(
    row: dict[str, Any],
    name: str,
    body: str,
    sources: dict[str, list[str]],
) -> bool:
    """True only when this division's remainder is posted to a protocol reserve.

    requiredDirection none has no dust. closureReserve is not that reserve.
    A comment that names the reserve is not a posting.
    """
    if str(row["requiredDirection"]) == "none":
        return True
    texts = [body, *caller_bodies(str(row["file"]), name, sources)]
    return any(RESERVE_POSTING.search(mask_non_code(text)) is not None for text in texts)


def check_coverage(
    profile: dict[str, Any],
    sources: dict[str, list[str]],
    failures: list[str],
) -> None:
    rows = profile["primitives"]
    citations = [
        (str(row["file"]), int(row["line"]))
        for row in rows
    ]
    try:
        masked = {name: masked_lines(lines) for name, lines in sources.items()}
    except ValueError as error:
        failures.append(f"FAIL: {error}")
        return
    check_ignored(IGNORED_ROUNDING_SITES, sources, ROUNDING_WORD, "rounding", failures)
    check_ignored(IGNORED_DIVISION_SITES, masked, DIVISION, "division", failures)
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
            roles = required_roles(relative, number, sources)
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
    relative: str, number: int, sources: dict[str, list[str]]
) -> set[str]:
    """Result roles D2 requires at this division, derived from the source."""
    lines = sources[relative]
    span = enclosing_function(lines, number)
    if span is None:
        return set()
    start, end, name = span
    body = "\n".join(lines[start - 1 : end])
    roles = roles_from_division_body(body)
    for caller in caller_bodies(relative, name, sources):
        roles |= roles_from_call(caller, name)
    return roles


def caller_bodies(
    relative: str, name: str, sources: dict[str, list[str]]
) -> list[str]:
    """Same-file callers. A same spelling in another file is a different function."""
    lines = sources.get(relative)
    if lines is None:
        return []
    pattern = re.compile(rf"\b{re.escape(name)}\s*\(")
    spans = function_spans(lines)
    bodies: list[str] = []
    seen: set[tuple[int, int]] = set()
    for number, line in enumerate(lines, start=1):
        if pattern.search(line) is None:
            continue
        span = innermost(spans, number)
        if span is None or span[2] == name:
            continue
        key = (span[0], span[1])
        if key in seen:
            continue
        seen.add(key)
        bodies.append("\n".join(lines[span[0] - 1 : span[1]]))
    return bodies


def roles_from_division_body(body: str) -> set[str]:
    roles: set[str] = set()
    if re.search(
        r"\baddU128\(\s*previous(?:Accrued|Outstanding|Incurred)\s*,\s*interest\s*\)",
        body,
    ):
        roles.add("obligation")
    if (
        re.search(r"\bFloorDiv\b", body) is not None
        and re.search(r"\bCeilDiv\b", body) is not None
        and DIVISION.search(mask_non_code(body)) is not None
    ):
        # The reducer implements both directions and does not separate
        # an amount owed from an amount received.
        roles.update(("obligation", "receipt"))
    return roles


def roles_from_call(body: str, callee: str) -> set[str]:
    roles: set[str] = set()
    assign = re.compile(
        rf"(?:const|let)\s+(\w+)\s*=\s*{re.escape(callee)}\s*\("
    )
    for match in assign.finditer(body):
        var = re.escape(match.group(1))
        if re.search(
            rf"\bsubU128\(\s*funded\.remaining\s*,\s*{var}\.value\s*\)",
            body,
        ):
            roles.add("obligation")
        if re.search(rf"\b{var}\.value\s*>\s*funded\.remaining\b", body):
            roles.add("obligation")
        if re.search(
            rf"\b{var}\.value\s*!==\s*funded\.amount\b",
            body,
        ) and re.search(r"\bfunded\.to\s*!==\s*action\.debtor\b", body):
            roles.add("receipt")
        if re.search(rf"\bprincipal\s*-\s*{var}\.value\.dP\b", body):
            roles.add("receipt")
    return roles


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
