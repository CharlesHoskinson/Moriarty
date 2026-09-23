#!/usr/bin/env python3
"""Validate U0 trust premises and the generated backend requirement matrix."""

from __future__ import annotations

import argparse
import hashlib
import json
import re
from pathlib import Path
from typing import Any, NamedTuple

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
REQUIREMENT_HEADING = re.compile(
    r"^#{1,6}\s+Requirement:\s+((?:UNI|MPLR)-\d{3})\b"
)
REQUIREMENT_ID = re.compile(r"\b((?:UNI|MPLR)-\d{3})\b")
ABBREVIATED_REQUIREMENT_IDS = re.compile(r"\b(UNI|MPLR)-(\d{3})((?:,\d{3})+)\b")


class PremiseContract(NamedTuple):
    """One trust premise whose claims are fixed independently of the artifact."""

    id: str
    kind: str
    status: str
    statement: str
    related_ids: tuple[str, ...]
    claims: tuple[str, ...]


PREMISE_CONTRACT = (
    PremiseContract(
        "TP01",
        "unresolved-interface",
        "open",
        (
            "The native execution target is Midnight ZKIRv3. "
            "The exact compiler, ZKIR, verifier, key and ledger tuple is not pinned, "
            "and a comprehensive compatible release tuple is unknown. "
            "This is an open ZR01 and UNI-003 interface. "
            "UNI-017 does not treat an unpinned tuple as qualified."
        ),
        ("UNI-003", "UNI-017", "ZR01"),
        ("ZKIRv3", "a comprehensive compatible release tuple is unknown."),
    ),
    PremiseContract(
        "TP02",
        "planning-assumption",
        "accepted-assumption",
        (
            "The user supplied a six-month planning assumption on 2026-09-19: "
            "comprehensive Midnight recursion around March 2027. "
            "That date is not an independently verified release. "
            "It does not close ZR02, UNI-009 or UNI-017."
        ),
        ("UNI-009", "UNI-017", "ZR02"),
        (
            "The user supplied a six-month planning assumption on 2026-09-19: "
            "comprehensive Midnight recursion around March 2027.",
            "independently verified release",
        ),
    ),
    PremiseContract(
        "TP03",
        "trust-assumption",
        "open",
        (
            "Issuers, oracles and signers remain explicit trust assumptions. "
            "An observation carries an issuer, a domain, a time and a finality claim, "
            "and each external domain supplies only its stated finality. "
            "A safety proof does not establish oracle honesty. "
            "ZR03, UNI-007 and MPLR-010 stay open on this point."
        ),
        ("MPLR-010", "UNI-007", "ZR03"),
        ("signers remain explicit trust assumptions", "oracle honesty"),
    ),
    PremiseContract(
        "TP04",
        "trust-assumption",
        "accepted-assumption",
        (
            "Federation trust is optional. "
            "Direct Midnight use has no federation requirement, "
            "and Moriarty can run on Midnight without the federated kernel. "
            "When a kernel is used, bare-threshold compromise remains an explicit trust boundary. "
            "ZR14, UNI-011 and MPLR-030 record that boundary."
        ),
        ("MPLR-030", "UNI-011", "ZR14"),
        ("no federation requirement", "remains an explicit trust boundary."),
    ),
    PremiseContract(
        "TP05",
        "trust-assumption",
        "accepted-assumption",
        (
            "Timeout is not evidence of nonexecution. "
            "A timeout can change which authorized transition may be attempted. "
            "It does not prove that another chain did not execute, "
            "and it does not prove entitlement to a refund. "
            "This boundary is accepted for UNI-007, UNI-008 and MPLR-010."
        ),
        ("MPLR-010", "UNI-007", "UNI-008"),
        ("Timeout is not evidence of nonexecution.", "another chain did not execute"),
    ),
    PremiseContract(
        "TP06",
        "unresolved-interface",
        "open",
        (
            "Private handoff and witness availability are unresolved. "
            "A private continuation requires the stated witness-handoff and availability mechanism, "
            "and witness availability is an explicit liveness assumption. "
            "Native zero-knowledge is not a private handoff theorem. "
            "ZR14, UNI-010, MPLR-013 and MPLR-029 stay open."
        ),
        ("MPLR-013", "MPLR-029", "UNI-010", "ZR14"),
        (
            "the stated witness-handoff and availability mechanism",
            "explicit liveness assumption",
            "not a private handoff theorem.",
        ),
    ),
    PremiseContract(
        "TP07",
        "unresolved-interface",
        "open",
        (
            "Signed-intent authentication has no selected boundary. "
            "The enforcement map must state whether that authentication occurs in the circuit, "
            "a bound ledger primitive, or another explicitly justified native boundary. "
            "ZR03, UNI-002 and UNI-004 require the bound intention and do not make this choice."
        ),
        ("UNI-002", "UNI-004", "ZR03"),
        (
            "authentication occurs in the circuit, a bound ledger primitive",
            "another explicitly justified native boundary.",
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
        return found, ["FAIL: openspec requirement catalog is missing"]
    paths = sorted(change_root.glob("*/specs/**/spec.md"))
    paths.extend(sorted(change_root.glob("*/traceability.md")))
    if not paths:
        return found, ["FAIL: openspec requirement catalog is missing"]
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
        if not any(related_id in quote for quote in quotes):
            failures.append(f"FAIL: {premise_id} related id {related_id} is not in a cited quote")
    return failures


def premise_contract_failures(
    root: Path,
    payload: Any,
    matrix_ids: set[str] | None,
) -> list[str]:
    if not isinstance(payload, dict) or not isinstance(payload.get("premises"), list):
        return []
    failures: list[str] = []
    for item in PREMISE_CONTRACT:
        for claim in item.claims:
            if claim not in item.statement:
                failures.append(f"FAIL: {item.id} contract claim is absent from its statement")
    catalog, catalog_failures = load_requirement_ids(root)
    failures.extend(catalog_failures)
    expected_ids = [item.id for item in PREMISE_CONTRACT]
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
    if actual_ids != expected_ids:
        for expected_id in expected_ids:
            if expected_id not in actual_ids:
                failures.append(f"FAIL: missing trust premise {expected_id}")
        for actual_id in actual_ids:
            if actual_id is not None and actual_id not in expected_ids:
                failures.append(
                    f"FAIL: trust premise {actual_id} is outside the source-backed contract"
                )
        if set(actual_ids) == set(expected_ids):
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
        if statement != item.statement:
            failures.append(f"FAIL: {item.id} statement does not match the source-backed contract")
        if not isinstance(statement, str):
            failures.append(f"FAIL: {item.id} statement is missing")
        elif any(claim not in statement for claim in item.claims):
            for claim in item.claims:
                if claim not in statement:
                    failures.append(f"FAIL: {item.id} statement lacks source-backed claim: {claim}")
        quotes = quoted_strings(premise)
        for claim in item.claims:
            if not any(claim in quote for quote in quotes):
                failures.append(f"FAIL: {item.id} claim is not in a cited quote: {claim}")
        related = premise.get("relatedIds")
        if related != list(item.related_ids):
            failures.append(f"FAIL: {item.id} relatedIds do not match the source-backed contract")
    for premise in payload["premises"]:
        if not isinstance(premise, dict):
            continue
        premise_id = premise.get("id")
        if not isinstance(premise_id, str):
            premise_id = "<unknown>"
        related = premise.get("relatedIds")
        if not isinstance(related, list):
            continue
        failures.extend(
            related_id_failures(
                premise_id,
                related,
                quoted_strings(premise),
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
    matrix_ids = {str(row["id"]) for row in parsed["rows"]} if parsed is not None else None
    failures.extend(premise_contract_failures(root, premises, matrix_ids))

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
