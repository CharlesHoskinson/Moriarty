#!/usr/bin/env python3
"""Check the U0 numeric profile against the owner decision and successor sources.

Cited lines use two separate tests. A rounding or scaling primitive must cite a
line that contains a division, rounding, or scaling operation (`/`, floor, ceil,
Rounding, scale, or div). An overflow-checked exact primitive must cite a code
line that contains `numericFits(`, `UINT64_MAX`, or `UINT128_MAX`, or the
operand underflow form `b > a ? null : a - b`. Comments and string literals are
removed before that test. A comment cannot satisfy either test, and one test
does not satisfy the other.

`symbol` is the declared function whose body contains `line`. A reducer case
sets `constructor` to its label. That label must be a string literal within
8 lines of `line`. Eight is the smallest window that reaches `Add` at line 399
from the `numericFits` guard at line 407, and `FloorDiv` at line 397 from the
division at line 404.

Reserve citations are type-level. A declaration is a top-level function, const,
class, type, interface, or enum name, or a member that starts a line in an
interface body or a `type Name = { ... }` body. Method parameters,
property-initializer calls, class-body members, object-literal keys, and labels
are not declarations.
"""

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

# Schema enum for the D4 sentence "They never become field elements".
# The checker emits this only when that sentence is the unique field-element
# sentence in D4. A different sentence does not produce this value.
FIELD_ELEMENT_PROHIBITION = "forbidden"
MISSING_RESERVE_POSTING = "missing protocol-reserve posting"

# An operand on each side of `/`, after comments and string literals are blanked.
# Spacing does not matter. `//` and `/*` are not division operators.
_OPERAND = r"(?:[A-Za-z_$][\w$]*|\d+n?|\)|\])"
DIVISION = re.compile(rf"{_OPERAND}\s*/(?!/|\*)\s*{_OPERAND}")
ROUNDING_WORD = re.compile(r"\bfloor\b|\bceil\b")
ROUNDING_OR_SCALE = re.compile(r"\bfloor\b|\bceil\b|Rounding|\bscale\b|\bdiv\b")
# Bound tokens for an overflow-checked exact primitive. The search uses the
# code line with comments and string literals removed.
OVERFLOW_LINE = re.compile(r"numericFits\s*\(|UINT(?:64|128)_MAX")
# subU128 rejects underflow as `b > a ? null : a - b`. Its body has no
# UINT128_MAX token. A bare `? null :` ternary does not match.
UNDERFLOW_RETURN = re.compile(
    r"\b(\w+)\s*>\s*(\w+)\s*\?\s*null\s*:\s*\2\s*-\s*\1\b"
)
_NUMERIC_FITS_CALL = re.compile(r"(?<![\w$])numericFits\s*\(")
_FUNCTION_NUMERIC_FITS = re.compile(r"\bfunction\s+numericFits\s*\(")
_CONSTRUCTOR_LABEL = re.compile(
    r"\bk\s*===\s*['\"]([A-Za-z_][A-Za-z0-9_]*)['\"]"
)
# Add at financial-expression-v1.ts:399 is 8 lines from numericFits at :407.
# FloorDiv at :397 is 7 lines from the division at :404. A window of 3 cannot
# cite the operation line and the constructor label for those rows.
CONSTRUCTOR_WINDOW = 8
_TYPE_BODY_INTERFACE = re.compile(
    r"\binterface\s+[A-Za-z_$][\w$]*\b[^;{}]*$"
)
_TYPE_BODY_ALIAS = re.compile(
    r"\btype\s+[A-Za-z_$][\w$]*\s*(?:<[^;]{0,400}>)?\s*=[^;{}]*$"
)
_TOP_LEVEL_DECLARATION = re.compile(
    r"^\s*(?:export\s+)?(?:declare\s+)?(?:default\s+)?"
    r"(?:async\s+)?(?:abstract\s+)?"
    r"(?:function|const|class|type|interface|enum)\s+"
    r"(?P<name>[A-Za-z_$][\w$]*)\b"
)
# floor(256 * log10(2)). 10^77 fits in UInt256 and 10^78 does not.
# The reciprocal numerator is 10^(s+t), so the largest checked mantissa is 10^154.
MAX_SCALE = 77
MAX_RECIPROCAL = 10 ** (MAX_SCALE * 2)
REMAINDER_WINDOW = 8
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
OVERFLOW_RETURN = re.compile(
    r"return\s+(?:"
    r"(?P<sum>[A-Za-z_]\w*)\s*>\s*UINT(?:64|128)_MAX\s*\?\s*null\s*:\s*(?P=sum)"
    r"|"
    r"(?P<right>[A-Za-z_]\w*)\s*>\s*(?P<left>[A-Za-z_]\w*)\s*\?\s*null\s*:\s*"
    r"(?P=left)\s*-\s*(?P=right)"
    r")\s*;"
)
FUNCTION_START = re.compile(
    r"^(?:export\s+)?(?:async\s+)?function\s+([A-Za-z_]\w*)\s*\("
)
METHOD_START = re.compile(
    r"^([ \t]+)(?:async\s+)?([A-Za-z_]\w*)\s*\((?:[^()]|\([^()]*\))*\)\s*(?::[^{]+)?\{\s*$"
)
CONTROL_WORDS = {"if", "for", "while", "switch", "catch", "else"}
WIDTH_NAME = re.compile(r"^(?:UInt|SInt)(\d+)$")
AUTHOR_SELECT = re.compile(r"\b(?:author|program)\b[\s\S]{0,120}\bselect")
_SECTION = re.compile(r"(?m)^##\s+D([1-4])\b[^\n]*")

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
    (
        "experiments/moriarty-language/src/successor/financial-expression-v1.ts",
        435,
        "numericFits on a stored obligation-field read. "
        "It rejects a value that does not fit. "
        "It does not round or narrow a computed amount.",
    ),
    (
        "experiments/moriarty-language/src/successor/financial-expression-types-v1.ts",
        181,
        "valueDomain calls numericFits on an existing value. "
        "This is not a source/5 arithmetic operation. "
        "Reducer call sites are listed as primitives.",
    ),
    (
        "experiments/moriarty-language/src/successor/expression-types-v1.ts",
        158,
        "Same valueDomain numericFits call as financial-expression-types-v1.ts. "
        "Source/5 does not use this file.",
    ),
)


class RoleEvidence(NamedTuple):
    """One fail-closed source entry.

    division-function and caller entries assign result role when pattern matches
    the cited region. author-selected-both-roles does not assign a role. Its
    pattern proves the reducer takes rounding from the constructor. The checker
    then requires one obligation row and one receipt row at that line.
    """

    file: str
    line: int
    role: str
    region: str
    caller: str
    pattern: str


# division-function and caller patterns are role evidence re-read from source.
# author-selected-both-roles is not role evidence. FloorDiv and CeilDiv names
# in the reducer do not prove which result role the quotient has.
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
        "",
        "author-selected-both-roles",
        "",
        r"k\s*===\s*'CeilDiv'\s*&&\s*r\s*!==\s*0n\s*\?\s*q\s*\+\s*1n\s*:\s*q",
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


class DerivedPolicy(NamedTuple):
    canonical: str | None
    obligation: str | None
    receipt: str | None
    exact: str | None
    beneficiary: str | None
    representation: str | None
    field_element_coercion: str | None


def main(argv: list[str] | None = None) -> int:
    try:
        return _main(argv)
    except Exception as error:
        print(
            "FAIL: internal error: "
            f"{type(error).__name__}: {one_line(str(error))}"
        )
        return 1


def _main(argv: list[str] | None = None) -> int:
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

    schema, _schema_text = load_json(
        schema_path, "numeric profile schema", SCHEMA_FILE, failures
    )
    profile, profile_text = load_json(
        profile_path, "numeric profile", PROFILE_FILE, failures
    )
    if schema is None or profile is None:
        return None, failures, None
    if not isinstance(schema, dict) or not isinstance(profile, dict):
        failures.append(
            "FAIL: <root> numeric profile and its schema must be JSON objects"
        )
        return None, failures, None

    if schema.get("$schema") != SCHEMA_DIALECT:
        failures.append("FAIL: schema is not draft 2020-12")
    try:
        Draft202012Validator.check_schema(schema)
    except SchemaError as error:
        failures.append(
            f"FAIL: numeric profile schema is invalid: {one_line(error.message)}"
        )
        return None, failures, None

    assert_schema_closed(schema, "<schema>", failures)
    validator = Draft202012Validator(schema)
    schema_errors = sorted(
        validator.iter_errors(profile),
        key=lambda error: ([str(part) for part in error.absolute_path], error.message),
    )
    if schema_errors:
        typed = [
            "FAIL: "
            + ("/".join(str(part) for part in error.absolute_path) or "<root>")
            + " "
            + one_line(error.message)
            for error in schema_errors
        ]
        return None, typed, None

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

    derived = derive_policy(decision_text, failures)
    check_key_order(profile, schema, "<root>", failures)
    digest = hashlib.sha256(decision_bytes).hexdigest()
    if profile["decisionSha256"] != digest:
        failures.append(
            "FAIL: decisionSha256 does not match "
            f"{DECISION_FILE} ({digest})"
        )
    compare_derived(profile, derived, failures)
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
        comments = {name: comment_masked_lines(lines) for name, lines in sources.items()}
    except ValueError as error:
        failures.append(f"FAIL: {error}")
        return None, failures, None
    reserve_absent = check_reserve_mechanism(profile, sources, comments, failures)
    check_price_types(profile, sources, failures)
    check_price_orientation(profile, sources, masked, failures)
    check_widths(profile, sources, failures)
    check_vectors(profile, failures)
    role_direction = {
        "obligation": derived.obligation,
        "receipt": derived.receipt,
        "exact": derived.exact,
    }
    primitive_count, gap_count = check_primitives(
        profile,
        sources,
        masked,
        comments,
        role_direction,
        reserve_absent,
        derived.beneficiary,
        failures,
    )
    check_coverage(profile, sources, masked, failures)
    return None, failures, (primitive_count, gap_count)


def decision_sections(text: str) -> dict[str, str] | None:
    matches = list(_SECTION.finditer(text))
    keys = [match.group(1) for match in matches]
    if len(keys) != len(set(keys)):
        return None
    bodies: dict[str, str] = {}
    for index, match in enumerate(matches):
        start = match.end()
        end = matches[index + 1].start() if index + 1 < len(matches) else len(text)
        bodies[match.group(1)] = text[start:end]
    return bodies


def sole_group(pattern: str, text: str) -> str | None:
    found = re.findall(pattern, text)
    if len(found) != 1:
        return None
    value = found[0]
    if isinstance(value, tuple):
        value = value[0]
    value = str(value).strip()
    if not value:
        return None
    return value


def hyphenate(value: str) -> str:
    return "-".join(value.strip().lower().split())


def representation_slug(section: str) -> str | None:
    found = re.findall(r"Amounts are ([^.]+)", section)
    if len(found) != 1:
        return None
    head = found[0].split(" in ", 1)[0].strip()
    if not head:
        return None
    slug = hyphenate(head)
    if slug.endswith("s"):
        slug = slug[:-1]
    return slug or None


def field_element_coercion(section: str) -> str | None:
    parts = [
        part.strip()
        for part in re.split(r"\.(?:\s+|$)", section.strip())
        if part.strip()
    ]
    mentions = [part for part in parts if "field elements" in part]
    prohibitions = [
        part
        for part in mentions
        if re.search(r"\bnever become field elements\b", part) is not None
    ]
    if len(prohibitions) == 1 and len(mentions) == 1:
        return FIELD_ELEMENT_PROHIBITION
    return None


def derive_policy(text: str, failures: list[str]) -> DerivedPolicy:
    """Read D1, D2, and D4 normative markers. Do not use phrases from elsewhere."""
    sections = decision_sections(text)
    if sections is None:
        failures.append("FAIL: decision section headings are duplicated")
        sections = {}
    for key, label in (("1", "D1"), ("2", "D2"), ("4", "D4")):
        if key not in sections:
            failures.append(f"FAIL: decision {label} is missing or ambiguous")
    d1 = sections.get("1", "")
    d2 = sections.get("2", "")
    d4 = sections.get("4", "")
    bolds = re.findall(r"\*\*([^*]+)\*\*", d1) if d1 else []
    canonical = bolds[0].strip() if len(bolds) == 1 and bolds[0].strip() else None
    if d1 and canonical is None:
        failures.append(
            "FAIL: decision D1 canonical orientation is missing or ambiguous"
        )
    obligation = (
        sole_group(r"rounds\s+\*\*up\*\*\s+\(`([a-z]+)`\)", d2) if d2 else None
    )
    if d2 and obligation is None:
        failures.append("FAIL: decision D2 obligation direction is missing or ambiguous")
    receipt = (
        sole_group(r"rounds\s+\*\*down\*\*\s+\(`([a-z]+)`\)", d2) if d2 else None
    )
    if d2 and receipt is None:
        failures.append("FAIL: decision D2 receipt direction is missing or ambiguous")
    exact = sole_group(r"no rounding\s+\(`([a-z]+)`\)", d2) if d2 else None
    if d2 and exact is None:
        failures.append("FAIL: decision D2 exact direction is missing or ambiguous")
    beneficiary_text = (
        sole_group(r"accrues to the \*\*([^*]+)\*\*", d2) if d2 else None
    )
    beneficiary = hyphenate(beneficiary_text) if beneficiary_text else None
    if d2 and beneficiary is None:
        failures.append(
            "FAIL: decision D2 remainder beneficiary is missing or ambiguous"
        )
    representation = representation_slug(d4) if d4 else None
    if d4 and representation is None:
        failures.append("FAIL: decision D4 representation is missing or ambiguous")
    coercion = field_element_coercion(d4) if d4 else None
    if d4 and coercion is None:
        failures.append(
            "FAIL: decision D4 field-element rule is missing or ambiguous"
        )
    return DerivedPolicy(
        canonical,
        obligation,
        receipt,
        exact,
        beneficiary,
        representation,
        coercion,
    )


def compare_derived(
    profile: dict[str, Any], derived: DerivedPolicy, failures: list[str]
) -> None:
    pairs = (
        (derived.canonical, profile["priceOrientation"]["canonical"], "priceOrientation.canonical", "D1"),
        (derived.obligation, profile["defaultPolicy"]["obligation"], "defaultPolicy.obligation", "D2"),
        (derived.receipt, profile["defaultPolicy"]["receipt"], "defaultPolicy.receipt", "D2"),
        (derived.exact, profile["defaultPolicy"]["exact"], "defaultPolicy.exact", "D2"),
        (
            derived.beneficiary,
            profile["defaultPolicy"]["remainderBeneficiary"],
            "defaultPolicy.remainderBeneficiary",
            "D2",
        ),
        (derived.representation, profile["units"]["representation"], "units.representation", "D4"),
        (
            derived.field_element_coercion,
            profile["units"]["fieldElementCoercion"],
            "units.fieldElementCoercion",
            "D4",
        ),
    )
    for expected, actual, label, section in pairs:
        if expected is None:
            continue
        if actual != expected:
            failures.append(
                f"FAIL: {label} does not match derived {section} value {expected}"
            )


def _opens_type_body(pretext: str) -> bool:
    """True when `{` opens an interface or `type Name =` body."""
    tail = pretext[-500:]
    if _TYPE_BODY_INTERFACE.search(tail) is not None:
        return True
    return _TYPE_BODY_ALIAS.search(tail) is not None


def _type_body_stack(text: str, stop: int) -> list[bool]:
    stack: list[bool] = []
    for index, ch in enumerate(text):
        if index >= stop:
            break
        if ch == "{":
            stack.append(_opens_type_body(text[:index]))
        elif ch == "}" and stack:
            stack.pop()
    return stack


def declares_type_level(lines: list[str], line_number: int, symbol: str) -> bool:
    """True when symbol is a type-level declaration on line_number.

    Accepted sites are a top-level function, const, class, type, interface, or
    enum name, or a member of an interface body or a `type Name = { ... }`
    body. The member name starts the line, after optional access modifiers.
    A method parameter, a property-initializer call, a class-body member, an
    object-literal key, a label, and a local variable are rejected. The checker
    does not decide whether the declaration is a reserve account.
    """
    if line_number < 1 or line_number > len(lines):
        return False
    masked = mask_non_code("\n".join(lines), blank_strings=True)
    parts = masked.split("\n")
    if len(parts) != len(lines):
        return False
    line = parts[line_number - 1]
    name = re.escape(symbol)
    member = re.match(
        rf"^\s*(?:(?:readonly|public|private|protected|static)\s+)*"
        rf"(?P<name>{name})\s*\??\s*[:(]",
        line,
    )
    if member is not None and re.search(r"[,(=]\s*$", line[: member.start("name")]):
        member = None
    top = _TOP_LEVEL_DECLARATION.match(line)
    top_hit = top is not None and top.group("name") == symbol
    if member is None and not top_hit:
        return False
    if member is not None:
        name_at = member.start("name")
    else:
        assert top is not None
        name_at = top.start("name")
    prefix = "\n".join(parts[: line_number - 1])
    if line_number > 1:
        prefix += "\n"
    stack = _type_body_stack(prefix + line, len(prefix) + name_at)
    if not stack:
        return top_hit
    return bool(stack[-1]) and member is not None


def check_reserve_mechanism(
    profile: dict[str, Any],
    sources: dict[str, list[str]],
    comments: dict[str, list[str]],
    failures: list[str],
) -> bool:
    """Return True when the reserve mechanism is absent.

    A present status counts only when at least one citation is a type-level
    declaration. An object-literal key or a label does not establish a reserve.
    Semantic adequacy of a cited declaration is a reviewed claim. This checker
    does not decide whether that declaration is a reserve account or whether a
    division posts its remainder there.
    """
    reserve = profile["reserveMechanism"]
    citations = reserve["citations"]
    keys = [
        (str(item["file"]), str(item["symbol"]), int(item["line"]))
        for item in citations
    ]
    if keys != sorted(keys):
        failures.append(
            "FAIL: reserveMechanism.citations is not sorted by file, symbol, and line"
        )
    valid = 0
    for index, citation in enumerate(citations):
        relative = str(citation["file"])
        symbol = str(citation["symbol"])
        line_number = int(citation["line"])
        lines = sources.get(relative)
        kept = comments.get(relative)
        if lines is None or kept is None:
            failures.append(f"FAIL: reserve citation file {relative} is missing")
            continue
        if line_number < 1 or line_number > len(lines):
            failures.append(
                f"FAIL: reserveMechanism.citations[{index}] line {line_number} "
                f"is past the end of {relative}"
            )
            continue
        if declares_type_level(lines, line_number, symbol):
            valid += 1
            continue
        failures.append(
            f"FAIL: reserveMechanism.citations[{index}] {symbol} "
            f"is not a declaration at {relative}:{line_number}"
        )
    status_absent = str(reserve["status"]) == "absent"
    if not status_absent and valid == 0:
        failures.append(
            "FAIL: reserveMechanism.status is present but no type-level "
            "declaration is cited"
        )
        return True
    return status_absent


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


def is_price_type(text: str, symbol: str) -> bool:
    """True when symbol is a type declaration or a type-tag check.

    A literal constructor (`case 'LitPrice'`, an operand-table key) is not a
    price type. The symbol must be declared with type, interface, class, or
    enum, or tested as a type tag (`tag === 'Price'`, `t.name === 'Price'`,
    or `other[0] === 'Price'`).
    """
    name = re.escape(symbol)
    declared = re.compile(rf"\b(?:type|interface|class|enum)\s+{name}\b")
    tested = re.compile(
        rf"(?:\btag\b|\.name|\[\s*0\s*\])\s*===\s*['\"]{name}['\"]"
    )
    return declared.search(text) is not None or tested.search(text) is not None


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
        relative = str(row["file"])
        symbol = str(row["symbol"])
        lines = sources.get(relative)
        if lines is None:
            failures.append(f"FAIL: price type file {relative} is missing")
            continue
        if re.search(rf"\b{re.escape(symbol)}\b", "\n".join(lines)) is None:
            failures.append(
                f"FAIL: price type symbol {symbol} does not occur in {relative}"
            )
            continue
        if not is_price_type("\n".join(comment_masked_lines(lines)), symbol):
            failures.append(
                f"FAIL: price type symbol {symbol} in {relative} "
                "is not a type declaration or type-tag check"
            )


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
    comments: dict[str, list[str]],
    role_direction: dict[str, str | None],
    reserve_absent: bool,
    beneficiary: str | None,
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
        failures.extend(
            check_primitive(
                row,
                sources,
                masked,
                comments,
                role_direction,
                reserve_absent,
                beneficiary,
            )
        )
        if row["conformance"] == "open-gap":
            gaps += 1
    return len(rows), gaps


def check_primitive(
    row: dict[str, Any],
    sources: dict[str, list[str]],
    masked: dict[str, list[str]],
    comments: dict[str, list[str]],
    role_direction: dict[str, str | None],
    reserve_absent: bool,
    beneficiary: str | None,
) -> list[str]:
    failures: list[str] = []
    identity = str(row["id"])
    relative = str(row["file"])
    lines = sources.get(relative)
    code_lines = masked.get(relative)
    comment_lines = comments.get(relative)
    if lines is None or code_lines is None or comment_lines is None:
        return [f"FAIL: primitive {identity} file {relative} is missing"]
    if re.search(rf"\b{re.escape(str(row['symbol']))}\b", "\n".join(lines)) is None:
        failures.append(
            f"FAIL: primitive {identity} symbol {row['symbol']} "
            f"does not occur in {relative}"
        )
    line_number = int(row["line"])
    if line_number > len(lines):
        return failures + [
            f"FAIL: primitive {identity} line {line_number} "
            f"is past the end of {relative}"
        ]
    code_text = code_lines[line_number - 1]
    comment_text = comment_lines[line_number - 1]
    required = str(row["requiredDirection"])
    if not line_has_operation(comment_text, code_text, required):
        if required == "none":
            detail = "has no overflow bound check"
        else:
            detail = "has no division, rounding, or scaling operation"
        failures.append(
            f"FAIL: primitive {identity} line {line_number} {detail}"
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
        return failures
    start, end, name = span
    body = "\n".join(lines[start - 1 : end])
    kept_body = mask_non_code(body, blank_strings=False)
    symbol = str(row["symbol"])
    if symbol != name:
        failures.append(
            f"FAIL: primitive {identity} symbol {symbol} "
            f"is not the declared function whose body contains line {line_number}"
        )
    constructor = row["constructor"]
    if constructor is not None and not constructor_literal_near(
        comment_lines, line_number, str(constructor)
    ):
        failures.append(
            f"FAIL: primitive {identity} constructor {constructor} "
            f"is not a string literal within {CONSTRUCTOR_WINDOW} lines "
            f"of {relative}:{line_number}"
        )
    selectable = author_selectable(kept_body, code_text, required)
    if bool(row["authorSelectable"]) != selectable:
        failures.append(
            f"FAIL: primitive {identity} authorSelectable does not match the source"
        )
    fixed = None if selectable else fixed_direction(comment_lines, line_number, required)
    reasons = conformance_reasons(selectable, required, fixed, reserve_absent)
    expected = "open-gap" if reasons else "conforms"
    if row["conformance"] != expected:
        detail = "; ".join(reasons) if reasons else "the source matches the policy"
        failures.append(
            f"FAIL: primitive {identity} conformance is {row['conformance']} "
            f"but the source requires {expected} ({detail})"
        )
    if DIVISION.search(code_text) and str(row["resultRole"]) != "exact":
        roles = required_roles(relative, line_number, sources, masked)
        if str(row["resultRole"]) not in roles:
            failures.append(
                f"FAIL: primitive {identity} resultRole {row['resultRole']} "
                f"is not derived for {relative}:{line_number}"
            )
    derived_direction = role_direction.get(str(row["resultRole"]))
    if derived_direction is not None and row["requiredDirection"] != derived_direction:
        failures.append(
            f"FAIL: primitive {identity} requiredDirection does not match derived "
            f"{row['resultRole']} value {derived_direction}"
        )
    if row["requiredDirection"] == "none":
        if row["remainderBeneficiary"] is not None:
            failures.append(
                f"FAIL: primitive {identity} remainderBeneficiary must be null"
            )
    elif beneficiary is not None and row["remainderBeneficiary"] != beneficiary:
        failures.append(
            f"FAIL: primitive {identity} remainderBeneficiary does not match "
            f"derived D2 value {beneficiary}"
        )
    note = row["gapNote"] if isinstance(row["gapNote"], str) else ""
    if row["conformance"] == "open-gap":
        if not note.strip():
            failures.append(f"FAIL: primitive {identity} open-gap requires gapNote")
    elif row["gapNote"] is not None:
        failures.append(f"FAIL: primitive {identity} conforms requires a null gapNote")
    if selectable and AUTHOR_SELECT.search(note) is None:
        failures.append(
            f"FAIL: primitive {identity} gapNote must name the author-selectable rounding"
        )
    if reserve_absent and required in {"ceil", "floor"}:
        if MISSING_RESERVE_POSTING not in note:
            failures.append(
                f"FAIL: primitive {identity} gapNote must name the "
                "missing protocol-reserve posting"
            )
    elif MISSING_RESERVE_POSTING in note:
        failures.append(
            f"FAIL: primitive {identity} gapNote names a missing "
            "protocol-reserve posting but reserveMechanism.status is present"
        )
    return failures


def constructor_literal_near(
    comment_lines: list[str], line_number: int, label: str
) -> bool:
    """True when label is a string literal within CONSTRUCTOR_WINDOW lines."""
    if line_number < 1 or line_number > len(comment_lines):
        return False
    start = max(1, line_number - CONSTRUCTOR_WINDOW)
    end = min(len(comment_lines), line_number + CONSTRUCTOR_WINDOW)
    pattern = re.compile("['\"]" + re.escape(label) + "['\"]")
    return any(
        pattern.search(comment_lines[number - 1]) is not None
        for number in range(start, end + 1)
    )


def line_has_operation(comment_line: str, code_line: str, required: str) -> bool:
    """Apply the cited-line split.

    Exact rows match OVERFLOW_LINE or UNDERFLOW_RETURN on the code line, with
    comments and string literals removed, and reject a division on that line.
    Rounding and scaling rows match division on the code line, or
    ROUNDING_OR_SCALE on the comment-masked line. A comment cannot satisfy
    either test, and a bound check is not a rounding operation.
    """
    if required == "none":
        return (
            (
                OVERFLOW_LINE.search(code_line) is not None
                or UNDERFLOW_RETURN.search(code_line) is not None
            )
            and DIVISION.search(code_line) is None
        )
    return (
        DIVISION.search(code_line) is not None
        or ROUNDING_OR_SCALE.search(comment_line) is not None
    )


def author_selectable(body: str, line_text: str, required: str) -> bool:
    """True when the cited operation lets the program choose floor, ceil, or none."""
    if required == "none" or DIVISION.search(line_text) is None:
        return False
    if "FloorDiv" in body and "CeilDiv" in body:
        return True
    return re.search(r"\.rounding\s*===", body) is not None


def fixed_direction(
    comment_lines: list[str], line_number: int, required: str
) -> str | None:
    """Classify one parsed division. Unrelated ceil tokens do not count.

    Ceil requires a remainder bound to `<numerator> % <divisor>` from
    parse_division, `<remainder> !== 0n`, and `<quotient> + 1n` or
    `addU128(<quotient>, 1n)`, all inside the remainder window.
    """
    if required == "none":
        return "none"
    if line_number < 1 or line_number > len(comment_lines):
        return None
    division = parse_division(comment_lines[line_number - 1])
    if division is None:
        return None
    last = min(len(comment_lines), line_number + REMAINDER_WINDOW)
    window = "\n".join(comment_lines[line_number - 1 : last])
    remainder_names = re.findall(
        rf"(?:const|let|var|,)\s+([A-Za-z_]\w*)\s*=\s*"
        rf"{re.escape(division.numerator)}\s*%\s*{re.escape(division.divisor)}\b",
        window,
    )
    quotient = re.escape(division.quotient)
    increment = re.search(
        rf"(?:\b{quotient}\s*\+\s*1n\b|\baddU128\(\s*{quotient}\s*,\s*1n\s*\))",
        window,
    )
    if increment is not None:
        for name in remainder_names:
            if re.search(rf"\b{re.escape(name)}\s*!==\s*0n\b", window) is not None:
                return "ceil"
    return "floor"


def conformance_reasons(
    selectable: bool,
    required_direction: str,
    fixed: str | None,
    reserve_absent: bool,
) -> list[str]:
    """Open-gap when rounding is selectable, the fixed direction differs, or the reserve is absent."""
    reasons: list[str] = []
    if selectable:
        reasons.append("author can select the rounding")
    elif fixed is None:
        reasons.append("the source has no fixed direction")
    elif fixed != required_direction:
        reasons.append(
            f"fixed direction {fixed} differs from required {required_direction}"
        )
    if reserve_absent and required_direction in {"ceil", "floor"}:
        reasons.append("protocol-reserve posting is absent")
    return reasons



def parse_division(line: str) -> Division | None:
    match = DIVISION_ASSIGN.search(line)
    if match is None:
        return None
    return Division(match.group("quotient"), match.group("numerator"), match.group("divisor"))



def named_function(
    lines: list[str], name: str
) -> tuple[int, int, str] | None:
    matches = [span for span in function_spans(lines) if span[2] == name]
    if len(matches) != 1:
        return None
    return matches[0]



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
    check_constructor_role_rows(rows, sources, masked, failures)
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
    """Rows a division must have.

    division-function and caller patterns contribute their role.
    author-selected-both-roles does not identify a role. When its constructor
    pattern matches, the site still needs one obligation row and one receipt row.
    """
    roles: set[str] = set()
    for entry in ROLE_EVIDENCE:
        if entry.file != relative or entry.line != number:
            continue
        text = evidence_text(entry, sources, masked)
        if text is None or re.search(entry.pattern, text) is None:
            continue
        if entry.region == "author-selected-both-roles":
            roles.update(("obligation", "receipt"))
        elif entry.role:
            roles.add(entry.role)
    return roles


def check_constructor_role_rows(
    rows: list[dict[str, Any]],
    sources: dict[str, list[str]],
    masked: dict[str, list[str]],
    failures: list[str],
) -> None:
    for entry in ROLE_EVIDENCE:
        if entry.region != "author-selected-both-roles":
            continue
        text = evidence_text(entry, sources, masked)
        if text is None or re.search(entry.pattern, text) is None:
            continue
        for role in ("obligation", "receipt"):
            matched = [
                row
                for row in rows
                if str(row["file"]) == entry.file
                and int(row["line"]) == entry.line
                and str(row["resultRole"]) == role
            ]
            if len(matched) != 1:
                failures.append(
                    f"FAIL: division site {entry.file}:{entry.line} "
                    f"requires exactly one {role} row at that line"
                )


def check_role_evidence(
    sources: dict[str, list[str]],
    masked: dict[str, list[str]],
    failures: list[str],
) -> None:
    allowed = {"division-function", "caller", "author-selected-both-roles"}
    for entry in ROLE_EVIDENCE:
        label = (
            "author-selected-both-roles"
            if entry.region == "author-selected-both-roles"
            else "role evidence"
        )
        if entry.region not in allowed:
            failures.append(
                f"FAIL: {label} {entry.file}:{entry.line} has region {entry.region}"
            )
            continue
        lines = masked.get(entry.file)
        if lines is None or entry.line < 1 or entry.line > len(lines):
            failures.append(f"FAIL: {label} {entry.file}:{entry.line} does not exist")
            continue
        if DIVISION.search(lines[entry.line - 1]) is None:
            failures.append(
                f"FAIL: {label} {entry.file}:{entry.line} is not a division"
            )
        text = evidence_text(entry, sources, masked)
        if text is None or re.search(entry.pattern, text) is None:
            if entry.region == "author-selected-both-roles":
                failures.append(
                    f"FAIL: author-selected-both-roles {entry.file}:{entry.line} "
                    "does not match the source"
                )
            else:
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
    if entry.region in {"division-function", "author-selected-both-roles"}:
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
    """Overflow helpers, reducer guards, and every numericFits( call.

    Helper sites carry the declared function name. The Add, Sub, and Mul guard
    carries those constructor labels. Every other numericFits( call carries
    constructor labels from the three lines above it, or the enclosing function
    name when that window has no label. A string cannot create a site.
    """
    found: dict[tuple[str, int], set[str]] = {}

    def add(relative: str, line: int, symbols: frozenset[str] | set[str]) -> None:
        found.setdefault((relative, line), set()).update(symbols)

    for relative, raw in sources.items():
        kept = comment_masked_lines(raw)
        spans = function_spans(raw)
        for start, end, name in spans:
            body = "\n".join(kept[start - 1 : end])
            match = OVERFLOW_RETURN.search(body)
            if match is not None:
                line = start + body[: match.start()].count("\n")
                owner = innermost(spans, line)
                if owner is not None and owner[0] == start and owner[2] == name:
                    add(relative, line, {name})
            body_lines = kept[start - 1 : end]
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
                add(relative, line, {"Add", "Sub", "Mul"})
        code = masked.get(relative, [])
        for number, line_text in enumerate(code, start=1):
            if _NUMERIC_FITS_CALL.search(line_text) is None:
                continue
            if number <= len(raw) and _FUNCTION_NUMERIC_FITS.search(raw[number - 1]):
                continue
            if (relative, number) in found:
                continue
            labels = constructor_labels_above(kept, number)
            if labels:
                add(relative, number, labels)
                continue
            owner = innermost(spans, number)
            add(relative, number, {owner[2]} if owner is not None else {"numericFits"})
    return [
        (relative, number, frozenset(symbols))
        for (relative, number), symbols in sorted(found.items())
    ]


def constructor_labels_above(kept: list[str], number: int) -> set[str]:
    """Constructor labels in `k === 'Label'` on this line and the three above."""
    start = max(1, number - 3)
    labels: set[str] = set()
    for index in range(start, number + 1):
        labels.update(_CONSTRUCTOR_LABEL.findall(kept[index - 1]))
    return labels


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
            if not any(exact_row_covers_symbol(row, symbol) for row in covered):
                failures.append(
                    f"FAIL: overflow site {relative}:{number} symbol {symbol} "
                    "is not covered by an exact primitive within 15 lines"
                )


def exact_row_covers_symbol(row: dict[str, Any], symbol: str) -> bool:
    """Match a constructor label on reducer rows and the function name on helpers."""
    if row.get("constructor") is not None:
        return str(row["constructor"]) == symbol
    return str(row["symbol"]) == symbol


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
