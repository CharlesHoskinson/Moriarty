#!/usr/bin/env python3
"""Validate U0 trust premises and the generated backend requirement matrix."""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
from typing import Any

from jsonschema import Draft202012Validator
from jsonschema.exceptions import SchemaError

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


def schema_failures(label: str, schema: dict[str, Any], instance: Any) -> list[str]:
    try:
        Draft202012Validator.check_schema(schema)
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
                failures.append(f"FAIL: {premise_id} quote file missing: {relative}")
                continue
            try:
                text = cited.read_text(encoding="utf-8")
            except UnicodeDecodeError:
                failures.append(f"FAIL: {premise_id} quote file is not UTF-8: {relative}")
                continue
            if quote not in text:
                failures.append(f"FAIL: {premise_id} quote not in {relative}")
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

    if isinstance(matrix_schema, dict) and matrix is not None:
        failures.extend(schema_failures(MATRIX_REL, matrix_schema, matrix))
    if isinstance(trust_schema, dict) and premises is not None:
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
