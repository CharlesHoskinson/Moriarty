#!/usr/bin/env python3
"""Validate U0 trust premises and the generated backend requirement matrix.

Semantic fit of a quote to its statement label is a reviewed claim, not
mechanically proven. The checker verifies structure, literal quote occurrence,
identifier resolution, required topics, label length, one sentence without
', and' or a semicolon, and closure-word absence. Source paths under the U0
deliverable, kernel schemas, scripts or tests are rejected. A missing cited
file is a check failure.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import re
import sys
from pathlib import Path
from typing import Any

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
# Amendment 2: citations must be repository documents outside these trees.
FORBIDDEN_SOURCE_PREFIXES = (
    ("deliverables", "u0-semantic-contract-2026-09-23"),
    ("openspec", "changes", "consolidated-language-kernel", "schemas"),
    ("scripts",),
    ("tests",),
)
REQUIREMENT_HEADING = re.compile(
    r"^#{1,6}\s+Requirement:\s+((?:UNI|MPLR)-\d{3})\b"
)
REQUIREMENT_ID = re.compile(r"\b((?:UNI|MPLR)-\d{3})\b")
ABBREVIATED_REQUIREMENT_IDS = re.compile(r"\b(UNI|MPLR)-(\d{3})((?:,\d{3})+)\b")
REQUIRED_TOPICS = (
    "native-target",
    "recursion-horizon",
    "observations-finality",
    "federation-optional",
    "timeout-not-nonexecution",
    "private-handoff",
    "intent-auth-boundary",
)
ALLOWED_PREMISE_STATUS = ("open", "accepted-assumption")
CLOSURE_WORDS = (
    "closed",
    "satisfied",
    "established",
    "verified",
    "proven",
    "complete",
    "resolved",
    "guaranteed",
)
REVIEW_MARKERS = ("reviewed claim", "not mechanically proven")
SENTENCE = re.compile(r"^[^.!?;\n]+[.!?]$")
# A heading or an em-dash title is not support. 20 is the minimum quote length.
NONHEADING_BODY = 20


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
        return found, ["FAIL: requirement catalog missing: openspec/changes"]
    paths = sorted(change_root.glob("*/specs/**/spec.md"))
    paths.extend(sorted(change_root.glob("*/traceability.md")))
    if not paths:
        return found, [
            "FAIL: requirement catalog missing: openspec/changes/*/specs/**/spec.md",
            "FAIL: requirement catalog missing: openspec/changes/*/traceability.md",
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


def premise_label(premise: dict[str, Any]) -> str:
    premise_id = premise.get("id")
    if isinstance(premise_id, str):
        return premise_id
    return "<unknown>"


def limitation_failures(payload: Any) -> list[str]:
    if not isinstance(payload, dict):
        return []
    text = payload.get("limitation")
    if not isinstance(text, str) or any(marker not in text for marker in REVIEW_MARKERS):
        return ["FAIL: trust premises limitation does not state the review boundary"]
    return []


def topic_failures(payload: Any) -> list[str]:
    if not isinstance(payload, dict) or not isinstance(payload.get("premises"), list):
        return []
    failures: list[str] = []
    present: set[str] = set()
    for premise in payload["premises"]:
        if not isinstance(premise, dict):
            continue
        label = premise_label(premise)
        topic = premise.get("topic")
        if not isinstance(topic, str) or not topic:
            failures.append(f"FAIL: {label} topic is missing")
            continue
        present.add(topic)
    for topic in REQUIRED_TOPICS:
        if topic not in present:
            failures.append(f"FAIL: missing required topic {topic}")
    return failures


def statement_failures(payload: Any) -> list[str]:
    if not isinstance(payload, dict) or not isinstance(payload.get("premises"), list):
        return []
    failures: list[str] = []
    for premise in payload["premises"]:
        if not isinstance(premise, dict):
            continue
        label = premise_label(premise)
        statement = premise.get("statement")
        if not isinstance(statement, str):
            failures.append(f"FAIL: {label} statement is missing")
            continue
        if len(statement) > 160:
            failures.append(f"FAIL: {label} statement exceeds 160 characters")
        if ";" in statement:
            failures.append(f"FAIL: {label} statement is spliced")
        if ", and " in statement:
            failures.append(f"FAIL: {label} statement joins claims with ', and'")
        if not SENTENCE.fullmatch(statement):
            failures.append(f"FAIL: {label} statement is not one sentence")
        for word in CLOSURE_WORDS:
            if re.search(rf"\b{word}\b", statement, re.IGNORECASE):
                failures.append(f"FAIL: {label} statement contains closure word: {word}")
    return failures


def premise_status_failures(payload: Any) -> list[str]:
    if not isinstance(payload, dict) or not isinstance(payload.get("premises"), list):
        return []
    failures: list[str] = []
    for premise in payload["premises"]:
        if not isinstance(premise, dict):
            continue
        if premise.get("status") not in ALLOWED_PREMISE_STATUS:
            failures.append(
                f"FAIL: {premise_label(premise)} status is not open or accepted-assumption"
            )
    return failures


def quote_has_nonheading_span(quote: str, related_id: str) -> bool:
    """True when related_id shares quote with at least 20 non-space body characters."""

    if related_id not in quote:
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
    reduced = reduced.replace(related_id, "")
    body = re.sub(r"\s+", "", reduced)
    return len(body) >= NONHEADING_BODY


def related_id_failures(
    premise_id: str,
    related_ids: list[Any],
    quotes: list[str],
    matrix_ids: set[str] | None,
    catalog: set[str] | None,
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
            if catalog is not None and related_id not in catalog:
                failures.append(
                    f"FAIL: {premise_id} related id {related_id} is not a "
                    "requirement heading or traceability row"
                )
        else:
            failures.append(f"FAIL: {premise_id} related id {related_id} is not a known id family")
        quotes_with_id = [quote for quote in quotes if related_id in quote]
        if not quotes_with_id:
            failures.append(f"FAIL: {premise_id} related id {related_id} is not in a cited quote")
        elif not any(quote_has_nonheading_span(quote, related_id) for quote in quotes_with_id):
            failures.append(
                f"FAIL: {premise_id} related id {related_id} "
                "is not tied to a non-heading span in a cited quote"
            )
    return failures


def premise_rule_failures(
    root: Path,
    payload: Any,
    matrix_ids: set[str] | None,
) -> list[str]:
    failures = limitation_failures(payload)
    failures.extend(topic_failures(payload))
    failures.extend(statement_failures(payload))
    failures.extend(premise_status_failures(payload))
    if not isinstance(payload, dict) or not isinstance(payload.get("premises"), list):
        return failures
    catalog, catalog_failures = load_requirement_ids(root)
    failures.extend(catalog_failures)
    catalog_missing = any(
        line.startswith("FAIL: requirement catalog missing:") for line in catalog_failures
    )
    for premise in payload["premises"]:
        if not isinstance(premise, dict):
            continue
        premise_id = premise_label(premise)
        related = premise.get("relatedIds")
        if not isinstance(related, list):
            continue
        failures.extend(
            related_id_failures(
                premise_id,
                related,
                quoted_strings(premise),
                matrix_ids,
                None if catalog_missing else catalog,
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


def source_path_is_forbidden(relative: str) -> bool:
    parts = Path(relative).parts
    return any(parts[: len(prefix)] == prefix for prefix in FORBIDDEN_SOURCE_PREFIXES)


def quote_failures(root: Path, payload: Any) -> list[str]:
    if not isinstance(payload, dict) or not isinstance(payload.get("premises"), list):
        return ["FAIL: trust premises are missing"]
    failures: list[str] = []
    seen: set[str] = set()
    for premise in payload["premises"]:
        if not isinstance(premise, dict):
            failures.append("FAIL: trust premise is not an object")
            continue
        premise_id = premise_label(premise)
        if premise_id in seen:
            failures.append(f"FAIL: duplicate trust premise {premise_id}")
        seen.add(premise_id)
        refs = premise.get("sourceRefs")
        if not isinstance(refs, list):
            failures.append(f"FAIL: {premise_id} has no states-premise quote")
            continue
        if not any(
            isinstance(ref, dict) and ref.get("quoteRole") == "states-premise" for ref in refs
        ):
            failures.append(f"FAIL: {premise_id} has no states-premise quote")
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
            if source_path_is_forbidden(relative):
                failures.append(
                    f"FAIL: {premise_id} source ref path is not an allowed document: {relative}"
                )
                continue
            cited = root / path
            if not cited.is_file():
                failures.append(f"FAIL: {premise_id} cited file missing: {relative}")
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
        label = premise_label(premise)
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


def check(root: Path) -> tuple[int, list[str]]:
    missing = [relative for relative in REQUIRED_PATHS if not (root / relative).is_file()]
    if missing:
        return 2, [f"blocked: missing {relative}" for relative in missing]

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
    failures.extend(premise_rule_failures(root, premises, matrix_ids))

    if any(line.startswith("FAIL:") for line in failures):
        return 1, failures
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
