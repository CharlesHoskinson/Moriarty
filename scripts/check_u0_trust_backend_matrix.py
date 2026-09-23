#!/usr/bin/env python3
"""Validate U0 trust premises and the generated backend requirement matrix."""

from __future__ import annotations

import argparse
import hashlib
import json
import re
import sys
from pathlib import Path
from typing import Any, NamedTuple

from jsonschema import Draft202012Validator
from jsonschema.exceptions import SchemaError

sys.path.insert(0, str(Path(__file__).resolve().parent))

from build_u0_backend_matrix import (
    EXPECTED_IDS,
    MATRIX_REL,
    SOURCE_REL,
    TableFormatError,
    build_matrix,
    canonical_matrix,
)


MORIARTY_ROOT = Path(__file__).resolve().parents[1]
TRUST_REL = "deliverables/u0-semantic-contract-2026-09-23/trust-premises.json"
MATRIX_SCHEMA_REL = (
    "openspec/changes/consolidated-language-kernel/schemas/"
    "backend-requirement-matrix.schema.json"
)
TRUST_SCHEMA_REL = (
    "openspec/changes/consolidated-language-kernel/schemas/"
    "trust-premises.schema.json"
)
REQUIRED_PATHS = (
    SOURCE_REL,
    MATRIX_REL,
    TRUST_REL,
    MATRIX_SCHEMA_REL,
    TRUST_SCHEMA_REL,
)
REQUIREMENT_HEADING = re.compile(
    r"^#{1,6}\s+Requirement:\s+((?:UNI|MPLR)-\d{3})\b"
)
REQUIREMENT_ID = re.compile(r"\b((?:UNI|MPLR)-\d{3})\b")
ABBREVIATED_REQUIREMENT_IDS = re.compile(r"\b(UNI|MPLR)-(\d{3})((?:,\d{3})+)\b")


class PremiseContract(NamedTuple):
    """Claim phrases and per-id quote topics for one required trust premise."""

    id: str
    kind: str
    status: str
    related_ids: tuple[str, ...]
    claims: tuple[str, ...]
    topics: tuple[str, ...]
    id_support: tuple[tuple[str, tuple[str, ...]], ...]


PREMISE_CONTRACT = (
    PremiseContract(
        "TP01",
        "unresolved-interface",
        "open",
        ("UNI-003", "UNI-017", "ZR01"),
        ("ZKIRv3", "a comprehensive compatible release tuple is unknown."),
        ("compatible released tuple",),
        (
            ("UNI-003", ("ZKIRv3",)),
            ("UNI-017", ("compatible released tuple",)),
            ("ZR01", ("a comprehensive compatible release tuple is unknown.",)),
        ),
    ),
    PremiseContract(
        "TP02",
        "planning-assumption",
        "accepted-assumption",
        ("UNI-009", "UNI-017", "ZR02"),
        (
            "The user supplied a six-month planning assumption on 2026-09-19: "
            "comprehensive Midnight recursion around March 2027.",
            "independently verified release",
        ),
        ("native recursive", "recursively composing"),
        (
            ("UNI-009", ("native recursive",)),
            ("UNI-017", ("native recursive",)),
            ("ZR02", ("recursively composing",)),
        ),
    ),
    PremiseContract(
        "TP03",
        "trust-assumption",
        "open",
        ("MPLR-010", "UNI-007", "ZR03"),
        ("signers remain explicit trust assumptions", "oracle honesty"),
        ("predecessors, observations", "observation boundary", "finality"),
        (
            ("MPLR-010", ("observation boundary",)),
            ("UNI-007", ("finality",)),
            ("ZR03", ("predecessors, observations",)),
        ),
    ),
    PremiseContract(
        "TP04",
        "trust-assumption",
        "accepted-assumption",
        ("MPLR-030", "UNI-011", "ZR14"),
        (
            "Moriarty can also run on Midnight without this federation.",
            "remains an explicit trust boundary.",
            "without the federated kernel",
            "threshold, hardware, observation and recovery assumptions",
        ),
        (
            "without the federated kernel",
            "threshold, hardware, observation and recovery assumptions",
        ),
        (
            ("MPLR-030", ("threshold, hardware, observation and recovery assumptions",)),
            ("UNI-011", ("remains an explicit trust boundary.",)),
            ("ZR14", ("without the federated kernel",)),
        ),
    ),
    PremiseContract(
        "TP05",
        "trust-assumption",
        "accepted-assumption",
        ("MPLR-010", "UNI-007", "UNI-008"),
        ("Timeout is not evidence of nonexecution.", "another chain did not execute"),
        ("mere timeout", "proof of nonexecution", "unresolved outcomes", "Refunds"),
        (
            ("MPLR-010", ("proof of nonexecution",)),
            ("UNI-007", ("mere timeout",)),
            ("UNI-008", ("Refunds",)),
        ),
    ),
    PremiseContract(
        "TP06",
        "unresolved-interface",
        "open",
        ("MPLR-013", "MPLR-029", "UNI-010", "ZR14"),
        (
            "the stated witness-handoff and availability mechanism",
            "explicit liveness assumption",
            "not a private handoff theorem.",
        ),
        ("witness needed to continue", "authenticated state domain"),
        (
            ("MPLR-013", ("witness needed to continue",)),
            ("MPLR-029", ("authenticated state domain",)),
            ("UNI-010", ("the stated witness-handoff and availability mechanism",)),
            ("ZR14", ("not a private handoff theorem.",)),
        ),
    ),
    PremiseContract(
        "TP07",
        "unresolved-interface",
        "open",
        ("UNI-002", "UNI-004", "ZR03"),
        (
            "authentication occurs in the circuit, a bound ledger primitive",
            "another explicitly justified native boundary.",
        ),
        ("signed intent", "signed intention", "signed constraint"),
        (
            ("UNI-002", ("signed intention",)),
            ("UNI-004", ("signed constraint",)),
            ("ZR03", ("signed intent",)),
        ),
    ),
)


def parse_args(argv: list[str] | None = None) -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Check U0 trust premises and the backend requirement matrix."
    )
    parser.add_argument("--root", type=Path, default=MORIARTY_ROOT)
    return parser.parse_args(argv)


def load_json(path: Path, label: str) -> tuple[Any | None, str | None]:
    try:
        return json.loads(path.read_text(encoding="utf-8")), None
    except UnicodeDecodeError:
        return None, f"FAIL: {label}: invalid UTF-8"
    except json.JSONDecodeError as exc:
        return None, f"FAIL: {label}: invalid JSON: {exc.msg}"


def schema_object_failure(label: str, schema: Any) -> str | None:
    if not isinstance(schema, dict):
        return f"FAIL: {label} schema is not an object"
    try:
        Draft202012Validator.check_schema(schema)
    except SchemaError as exc:
        message = " ".join(str(exc.message).split())
        return f"FAIL: {label} schema: {message}"
    return None


def schema_failures(label: str, schema: dict[str, Any], instance: Any) -> list[str]:
    try:
        validator = Draft202012Validator(schema)
    except SchemaError as exc:
        message = " ".join(str(exc.message).split())
        return [f"FAIL: {label} schema: {message}"]
    failures: list[str] = []
    errors = sorted(validator.iter_errors(instance), key=lambda error: list(error.absolute_path))
    for error in errors:
        location = "/".join(str(part) for part in error.absolute_path) or "<root>"
        message = " ".join(error.message.split())
        failures.append(f"FAIL: {label} {location}: {message}")
    return failures


def load_requirement_ids(root: Path) -> tuple[set[str], list[str]]:
    """Collect UNI and MPLR ids from requirement headings and table rows."""

    failures: list[str] = []
    found: set[str] = set()
    change_root = root / "openspec" / "changes"
    if not change_root.is_dir():
        return found, ["blocked: missing openspec/changes"]
    paths = sorted(change_root.glob("*/specs/**/spec.md"))
    paths.extend(sorted(change_root.glob("*/traceability.md")))
    if not paths:
        return found, [
            "blocked: missing openspec/changes/*/specs/**/spec.md",
            "blocked: missing openspec/changes/*/traceability.md",
        ]
    for path in paths:
        relative = path.relative_to(root).as_posix()
        try:
            text = path.read_text(encoding="utf-8")
        except UnicodeDecodeError:
            failures.append(f"FAIL: requirement file is not UTF-8: {relative}")
            continue
        for raw_line in text.splitlines():
            line = raw_line.strip()
            heading = REQUIREMENT_HEADING.match(line)
            if heading:
                found.add(heading.group(1))
            if not line.startswith("|"):
                continue
            for match in ABBREVIATED_REQUIREMENT_IDS.finditer(line):
                prefix = match.group(1)
                found.add(f"{prefix}-{match.group(2)}")
                for part in match.group(3).split(","):
                    if part:
                        found.add(f"{prefix}-{part}")
            for match in REQUIREMENT_ID.finditer(line):
                found.add(match.group(1))
    return found, failures


def quoted_strings(premise: dict[str, Any]) -> list[str]:
    refs = premise.get("sourceRefs")
    if not isinstance(refs, list):
        return []
    quotes: list[str] = []
    for ref in refs:
        if isinstance(ref, dict) and isinstance(ref.get("quote"), str):
            quotes.append(ref["quote"])
    return quotes


def contract_for(premise_id: str) -> PremiseContract | None:
    for item in PREMISE_CONTRACT:
        if item.id == premise_id:
            return item
    return None


def tie_phrases(premise_id: str, related_id: str) -> tuple[str, ...]:
    """Phrases that must appear with this id, not phrases that support a different id."""

    item = contract_for(premise_id)
    if item is None:
        return ()
    for candidate, phrases in item.id_support:
        if candidate == related_id:
            return phrases
    return ()


def contract_integrity_failures() -> list[str]:
    failures: list[str] = []
    for item in PREMISE_CONTRACT:
        supported = [related_id for related_id, _phrases in item.id_support]
        if supported != list(item.related_ids):
            failures.append(f"FAIL: {item.id} id support does not match related ids")
        allowed = item.claims + item.topics
        for related_id, phrases in item.id_support:
            if not phrases:
                failures.append(f"FAIL: {item.id} id support for {related_id} is empty")
            for phrase in phrases:
                if phrase not in allowed:
                    failures.append(
                        f"FAIL: {item.id} id support phrase is not a claim or topic: {phrase}"
                    )
    return failures


def statement_sentences(statement: str) -> list[str]:
    return [part for part in re.split(r"(?<=[.!?])\s+", statement.strip()) if part]


def sentence_is_source_backed(
    sentence: str,
    quotes: list[str],
    claims: tuple[str, ...],
) -> bool:
    if any(claim in sentence for claim in claims):
        return True
    if len(sentence) < 20:
        return any(sentence in quote for quote in quotes)
    for quote in quotes:
        for index in range(0, len(sentence) - 19):
            if sentence[index : index + 20] in quote:
                return True
    return False


def grounding_failures(
    premise_id: str,
    statement: Any,
    quotes: list[str],
    claims: tuple[str, ...],
) -> list[str]:
    if not isinstance(statement, str):
        return []
    failures: list[str] = []
    for sentence in statement_sentences(statement):
        if not sentence_is_source_backed(sentence, quotes, claims):
            failures.append(f"FAIL: {premise_id} statement is not source-backed: {sentence}")
    return failures


def quote_ties_related_id(quote: str, related_id: str, phrases: tuple[str, ...]) -> bool:
    """Require the id and a claim or topic outside that id's own heading or title."""

    if related_id not in quote or not phrases:
        return False
    reduced = re.sub(
        rf"Requirement:\s+{re.escape(related_id)}\b[^\n|]*",
        "\n",
        quote,
    )
    reduced = re.sub(
        rf"{re.escape(related_id)} \u2014 [^\n|]*",
        "\n",
        reduced,
    )
    return any(phrase in reduced for phrase in phrases)


def related_id_failures(
    premise_id: str,
    related_ids: list[Any],
    quotes: list[str],
    matrix_ids: set[str] | None,
    catalog: set[str],
) -> list[str]:
    failures: list[str] = []
    for related_id in related_ids:
        if not isinstance(related_id, str):
            failures.append(f"FAIL: {premise_id} related id is not a string")
            continue
        if related_id.startswith(("ZR", "MNR")):
            if matrix_ids is None or related_id not in matrix_ids:
                failures.append(
                    f"FAIL: {premise_id} related id {related_id} is not in the backend matrix"
                )
        elif related_id.startswith(("UNI-", "MPLR-")):
            if related_id not in catalog:
                failures.append(
                    f"FAIL: {premise_id} related id {related_id} is not a "
                    "requirement heading or traceability row"
                )
        else:
            failures.append(f"FAIL: {premise_id} related id {related_id} is not a known id family")
        quotes_with_id = [quote for quote in quotes if related_id in quote]
        phrases = tie_phrases(premise_id, related_id)
        if not quotes_with_id:
            failures.append(f"FAIL: {premise_id} related id {related_id} is not in a cited quote")
        elif phrases and not any(
            quote_ties_related_id(quote, related_id, phrases) for quote in quotes_with_id
        ):
            failures.append(
                f"FAIL: {premise_id} related id {related_id} "
                "is not tied to a premise claim in a cited quote"
            )
    return failures


def premise_contract_failures(
    root: Path,
    payload: Any,
    matrix_ids: set[str] | None,
) -> list[str]:
    failures = contract_integrity_failures()
    if not isinstance(payload, dict) or not isinstance(payload.get("premises"), list):
        return failures
    catalog, catalog_failures = load_requirement_ids(root)
    failures.extend(catalog_failures)
    expected_ids = [item.id for item in PREMISE_CONTRACT]
    expected_set = set(expected_ids)
    by_id: dict[str, dict[str, Any]] = {}
    actual_ids: list[str | None] = []
    for premise in payload["premises"]:
        if not isinstance(premise, dict) or not isinstance(premise.get("id"), str):
            actual_ids.append(None)
            continue
        premise_id = premise["id"]
        actual_ids.append(premise_id)
        if premise_id not in by_id:
            by_id[premise_id] = premise
    present_contract = [premise_id for premise_id in actual_ids if premise_id in expected_set]
    if present_contract != expected_ids:
        for expected_id in expected_ids:
            if expected_id not in present_contract:
                failures.append(f"FAIL: missing trust premise {expected_id}")
        if not any(expected_id not in present_contract for expected_id in expected_ids):
            failures.append("FAIL: trust premises are not in contract order")
    for item in PREMISE_CONTRACT:
        premise = by_id.get(item.id)
        if premise is None:
            continue
        if premise.get("kind") != item.kind:
            failures.append(f"FAIL: {item.id} kind is not {item.kind}")
        if premise.get("status") != item.status:
            failures.append(f"FAIL: {item.id} status is not {item.status}")
        statement = premise.get("statement")
        if not isinstance(statement, str):
            failures.append(f"FAIL: {item.id} statement is missing")
        else:
            for claim in item.claims:
                if claim not in statement:
                    failures.append(f"FAIL: {item.id} statement lacks source-backed claim: {claim}")
        quotes = quoted_strings(premise)
        for claim in item.claims:
            if not any(claim in quote for quote in quotes):
                failures.append(f"FAIL: {item.id} claim is not in a cited quote: {claim}")
        for topic in item.topics:
            if not any(topic in quote for quote in quotes):
                failures.append(f"FAIL: {item.id} topic is not in a cited quote: {topic}")
        failures.extend(grounding_failures(item.id, statement, quotes, item.claims))
        related = premise.get("relatedIds")
        if related != list(item.related_ids):
            failures.append(f"FAIL: {item.id} relatedIds do not match the source-backed contract")
    for premise in payload["premises"]:
        if not isinstance(premise, dict):
            continue
        premise_id = premise.get("id")
        if not isinstance(premise_id, str):
            premise_id = "<unknown>"
        quotes = quoted_strings(premise)
        item = contract_for(premise_id)
        if item is None:
            failures.extend(grounding_failures(premise_id, premise.get("statement"), quotes, ()))
        related = premise.get("relatedIds")
        if not isinstance(related, list):
            continue
        failures.extend(
            related_id_failures(
                premise_id,
                related,
                quotes,
                matrix_ids,
                catalog,
            )
        )
    return failures


def row_ids(payload: Any) -> list[Any] | None:
    if not isinstance(payload, dict) or not isinstance(payload.get("rows"), list):
        return None
    found: list[Any] = []
    for row in payload["rows"]:
        found.append(row.get("id") if isinstance(row, dict) else None)
    return found


def status_failures(payload: Any) -> list[str]:
    if not isinstance(payload, dict) or not isinstance(payload.get("rows"), list):
        return ["FAIL: backend matrix rows are missing"]
    failures: list[str] = []
    for row in payload["rows"]:
        if not isinstance(row, dict):
            failures.append("FAIL: backend matrix row is not an object")
            continue
        row_id = row.get("id", "<unknown>")
        if row.get("status") != "specified-only":
            failures.append(f"FAIL: {row_id} status is not specified-only")
    return failures


def quote_failures(root: Path, payload: Any) -> list[str]:
    if not isinstance(payload, dict) or not isinstance(payload.get("premises"), list):
        return ["FAIL: trust premises are missing"]
    failures: list[str] = []
    seen: set[str] = set()
    for premise in payload["premises"]:
        if not isinstance(premise, dict):
            failures.append("FAIL: trust premise is not an object")
            continue
        premise_id = str(premise.get("id", "<unknown>"))
        if premise_id in seen:
            failures.append(f"FAIL: duplicate trust premise {premise_id}")
        seen.add(premise_id)
        refs = premise.get("sourceRefs")
        if not isinstance(refs, list):
            continue
        for ref in refs:
            if not isinstance(ref, dict):
                failures.append(f"FAIL: {premise_id} source ref is not an object")
                continue
            relative = ref.get("path")
            quote = ref.get("quote")
            if not isinstance(relative, str) or not isinstance(quote, str):
                failures.append(f"FAIL: {premise_id} source ref is incomplete")
                continue
            path = Path(relative)
            if path.is_absolute() or ".." in path.parts:
                failures.append(f"FAIL: {premise_id} quote path escapes root: {relative}")
                continue
            cited = root / path
            if not cited.is_file():
                failures.append(f"blocked: missing {relative}")
                continue
            try:
                text = cited.read_text(encoding="utf-8")
            except UnicodeDecodeError:
                failures.append(f"FAIL: {premise_id} quote file is not UTF-8: {relative}")
                continue
            if quote not in text:
                failures.append(f"FAIL: {premise_id} quote not in {relative}")
    return failures


def required_key_order(node: Any) -> list[str] | None:
    if not isinstance(node, dict):
        return None
    required = node.get("required")
    if not isinstance(required, list) or not all(isinstance(key, str) for key in required):
        return None
    return list(required)


def key_order_failures(payload: dict[str, Any], schema: dict[str, Any]) -> list[str]:
    failures: list[str] = []
    definitions = schema.get("$defs")
    if not isinstance(definitions, dict):
        definitions = {}
    root_keys = required_key_order(schema)
    premise_keys = required_key_order(definitions.get("premise"))
    ref_keys = required_key_order(definitions.get("sourceRef"))
    if root_keys is not None and list(payload) != root_keys:
        failures.append("FAIL: trust premises key order does not match the schema")
    premises = payload.get("premises")
    if not isinstance(premises, list) or premise_keys is None:
        return failures
    for premise in premises:
        if not isinstance(premise, dict):
            continue
        premise_id = premise.get("id")
        label = premise_id if isinstance(premise_id, str) else "<unknown>"
        if list(premise) != premise_keys:
            failures.append(f"FAIL: {label} key order does not match the schema")
        refs = premise.get("sourceRefs")
        if not isinstance(refs, list) or ref_keys is None:
            continue
        for ref in refs:
            if isinstance(ref, dict) and list(ref) != ref_keys:
                failures.append(
                    f"FAIL: {label} source ref key order does not match the schema"
                )
    return failures


def canonical_premises(payload: Any) -> str:
    return json.dumps(payload, indent=2, ensure_ascii=False) + "\n"


def serialization_failures(path: Path, payload: Any, schema: Any) -> list[str]:
    if not isinstance(payload, dict):
        return []
    failures: list[str] = []
    if isinstance(schema, dict):
        failures.extend(key_order_failures(payload, schema))
    rendered = canonical_premises(payload).encode("utf-8")
    if path.read_bytes() != rendered:
        failures.append("FAIL: trust premises are not canonically serialised")
    return failures


def missing_cited_and_catalog(root: Path) -> list[str]:
    """Return blocked lines for absent cited files and an absent requirement catalog."""

    blocked: list[str] = []
    seen: set[str] = set()
    try:
        payload = json.loads((root / TRUST_REL).read_text(encoding="utf-8"))
    except (OSError, UnicodeDecodeError, json.JSONDecodeError):
        payload = None
    if isinstance(payload, dict) and isinstance(payload.get("premises"), list):
        for premise in payload["premises"]:
            if not isinstance(premise, dict) or not isinstance(premise.get("sourceRefs"), list):
                continue
            for ref in premise["sourceRefs"]:
                if not isinstance(ref, dict) or not isinstance(ref.get("path"), str):
                    continue
                relative = ref["path"]
                path = Path(relative)
                if path.is_absolute() or ".." in path.parts or relative in seen:
                    continue
                seen.add(relative)
                if not (root / path).is_file():
                    blocked.append(f"blocked: missing {relative}")
    change_root = root / "openspec" / "changes"
    if not change_root.is_dir():
        blocked.append("blocked: missing openspec/changes")
    else:
        catalog = [
            *change_root.glob("*/specs/**/spec.md"),
            *change_root.glob("*/traceability.md"),
        ]
        if not catalog:
            blocked.append("blocked: missing openspec/changes/*/specs/**/spec.md")
            blocked.append("blocked: missing openspec/changes/*/traceability.md")
    return blocked


def check(root: Path) -> tuple[int, list[str]]:
    missing = [relative for relative in REQUIRED_PATHS if not (root / relative).is_file()]
    if missing:
        return 2, [f"blocked: missing {relative}" for relative in missing]
    blocked = missing_cited_and_catalog(root)
    if blocked:
        return 2, blocked

    failures: list[str] = []
    matrix_schema, matrix_schema_error = load_json(root / MATRIX_SCHEMA_REL, MATRIX_SCHEMA_REL)
    trust_schema, trust_schema_error = load_json(root / TRUST_SCHEMA_REL, TRUST_SCHEMA_REL)
    matrix, matrix_error = load_json(root / MATRIX_REL, MATRIX_REL)
    premises, premises_error = load_json(root / TRUST_REL, TRUST_REL)
    for error in (matrix_schema_error, trust_schema_error, matrix_error, premises_error):
        if error is not None:
            failures.append(error)

    if matrix_schema_error is None:
        problem = schema_object_failure(MATRIX_SCHEMA_REL, matrix_schema)
        if problem is not None:
            failures.append(problem)
        elif matrix is not None:
            failures.extend(schema_failures(MATRIX_REL, matrix_schema, matrix))
    if trust_schema_error is None:
        problem = schema_object_failure(TRUST_SCHEMA_REL, trust_schema)
        if problem is not None:
            failures.append(problem)
        elif premises is not None:
            failures.extend(schema_failures(TRUST_REL, trust_schema, premises))

    source_path = root / SOURCE_REL
    source_bytes = source_path.read_bytes()
    digest = hashlib.sha256(source_bytes).hexdigest()
    try:
        parsed = build_matrix(source_bytes)
    except TableFormatError as exc:
        failures.append(exc.message)
        parsed = None

    if parsed is not None:
        fresh = canonical_matrix(parsed).encode("utf-8")
        if (root / MATRIX_REL).read_bytes() != fresh:
            failures.append("FAIL: backend matrix drifted")
        parsed_ids = [row["id"] for row in parsed["rows"]]
        # build_matrix raises TableFormatError unless the parsed ids are exactly
        # EXPECTED_IDS. Keep this as a defensive assertion of that guarantee.
        if parsed_ids != EXPECTED_IDS:
            failures.append(
                "FAIL: source document ids are not exactly ZR01-ZR16 and MNR01-MNR08"
            )
        loaded_ids = row_ids(matrix)
        if loaded_ids != parsed_ids:
            failures.append(
                "FAIL: backend row ids are not a bijection with the source document"
            )

    if isinstance(matrix, dict) and matrix.get("sourceSha256") != digest:
        failures.append(
            "FAIL: sourceSha256 does not match docs/MORIARTY-BACKEND-REQUIREMENTS.md"
        )
    loaded_ids = row_ids(matrix)
    if loaded_ids != EXPECTED_IDS:
        failures.append(
            "FAIL: backend row ids are not exactly ZR01-ZR16 and MNR01-MNR08"
        )
    failures.extend(status_failures(matrix))
    failures.extend(quote_failures(root, premises))
    if isinstance(premises, dict):
        failures.extend(
            serialization_failures(
                root / TRUST_REL,
                premises,
                trust_schema if isinstance(trust_schema, dict) else None,
            )
        )
    matrix_ids = {str(row["id"]) for row in parsed["rows"]} if parsed is not None else None
    failures.extend(premise_contract_failures(root, premises, matrix_ids))

    blocked_lines = [line for line in failures if line.startswith("blocked:")]
    if blocked_lines:
        return 2, blocked_lines
    if failures:
        return 1, failures
    count = len(premises["premises"]) if isinstance(premises, dict) else 0
    return 0, [f"OK: 24 backend rows specified-only, {count} trust premises"]


def main(argv: list[str] | None = None) -> int:
    args = parse_args(argv)
    code, lines = check(args.root.resolve())
    for line in lines:
        print(line)
    return code


if __name__ == "__main__":
    raise SystemExit(main())
