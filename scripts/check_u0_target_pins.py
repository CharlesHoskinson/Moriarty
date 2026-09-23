#!/usr/bin/env python3
"""Check the U0 target-pin ledger against its schema and on-disk evidence."""

from __future__ import annotations

import argparse
import json
import re
from pathlib import Path

from jsonschema import Draft202012Validator, FormatChecker
from jsonschema.exceptions import SchemaError


MORIARTY_ROOT = Path(__file__).resolve().parents[1]
SCHEMA_REL = Path(
    "openspec/changes/consolidated-language-kernel/schemas/target-pins.schema.json"
)
LEDGER_REL = Path("deliverables/u0-semantic-contract-2026-09-23/target-pins.json")
SCHEMA_VERSION = "moriarty-u0-target-pins/1"
AS_OF = "2026-09-23"
COMPONENTS = (
    "moriarty-compiler",
    "compact-compiler",
    "zkir",
    "native-proof-system",
    "verifier",
    "proving-keys",
    "verifier-keys",
    "srs-parameters",
    "ledger",
    "proof-server",
    "k-reference-toolchain",
)
PIN_KINDS = ("git-commit", "version", "sha256", "none")
STATUSES = ("historical", "absent")
PIN_PATTERNS = {
    "git-commit": re.compile(r"^[0-9a-f]{40}$"),
    "sha256": re.compile(r"^[0-9a-f]{64}$"),
    "version": re.compile(r"^[A-Za-z0-9][A-Za-z0-9._+-]*$"),
}


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description="Check the U0 target-pin ledger.")
    parser.add_argument(
        "--root",
        type=Path,
        default=MORIARTY_ROOT,
        help="Repository root that contains the ledger, schema, and evidence.",
    )
    args = parser.parse_args(argv)
    return check(args.root.resolve())


def check(root: Path) -> int:
    if not root.is_dir():
        print(f"blocked: root is not a directory: {root}")
        return 2
    schema_path = root / SCHEMA_REL
    ledger_path = root / LEDGER_REL
    if not schema_path.is_file():
        print(f"blocked: missing schema {SCHEMA_REL.as_posix()}")
        return 2
    if not ledger_path.is_file():
        print(f"blocked: missing ledger {LEDGER_REL.as_posix()}")
        return 2

    schema, schema_error = load_json(schema_path, SCHEMA_REL)
    if schema_error is not None:
        print(schema_error)
        return 1 if schema_error.startswith("FAIL:") else 2
    ledger_text, ledger, ledger_error = load_json_text(ledger_path, LEDGER_REL)
    if ledger_error is not None:
        print(ledger_error)
        return 1 if ledger_error.startswith("FAIL:") else 2

    failures: list[str] = []
    check_canonical(ledger_text, ledger, failures)
    check_schema(schema, ledger, failures)
    check_ledger(root, schema, ledger, failures)
    if failures:
        for failure in failures:
            print(f"FAIL: {failure}")
        return 1

    pins = ledger["pins"]
    historical = sum(1 for row in pins if row["status"] == "historical")
    absent = sum(1 for row in pins if row["status"] == "absent")
    print(
        f"OK: {historical} historical, {absent} absent, "
        "compatible tuple NOT established"
    )
    return 0


def load_json(path: Path, rel: Path) -> tuple[object, str | None]:
    text, obj, error = load_json_text(path, rel)
    del text
    return obj, error


def load_json_text(path: Path, rel: Path) -> tuple[str, object, str | None]:
    try:
        text = path.read_text(encoding="utf-8")
    except OSError as exc:
        return "", None, f"blocked: cannot read {rel.as_posix()}: {exc}"
    except UnicodeDecodeError as exc:
        return "", None, f"FAIL: {rel.as_posix()} is not UTF-8: {exc}"
    try:
        return text, json.loads(text), None
    except json.JSONDecodeError as exc:
        return text, None, f"FAIL: {rel.as_posix()} is not JSON: {exc}"


def check_canonical(text: str, ledger: object, failures: list[str]) -> None:
    if text != json.dumps(ledger, indent=2) + "\n":
        failures.append(
            f"{LEDGER_REL.as_posix()} is not UTF-8 JSON with 2-space indent "
            "and a trailing newline"
        )


def check_schema(schema: object, ledger: object, failures: list[str]) -> None:
    if not isinstance(schema, dict):
        failures.append("schema is not an object")
        return
    try:
        validator = Draft202012Validator(schema, format_checker=FormatChecker())
    except SchemaError as exc:
        failures.append(f"schema is not a valid Draft 2020-12 document: {exc.message}")
        return
    errors = sorted(validator.iter_errors(ledger), key=lambda item: list(item.absolute_path))
    for error in errors:
        location = "/".join(str(part) for part in error.absolute_path) or "<root>"
        failures.append(f"schema {location}: {error.message}")


def check_ledger(
    root: Path, schema: object, ledger: object, failures: list[str]
) -> None:
    if not isinstance(schema, dict):
        return
    if not isinstance(ledger, dict):
        failures.append("ledger is not an object")
        return
    check_key_order(schema, ledger, "<root>", failures)
    if ledger.get("schemaVersion") != SCHEMA_VERSION:
        failures.append(f"schemaVersion is not {SCHEMA_VERSION}")
    if ledger.get("asOf") != AS_OF:
        failures.append(f"asOf is not {AS_OF}")
    if ledger.get("compatibleTupleEstablished") is not False:
        failures.append("compatibleTupleEstablished is not false")
    unresolved = ledger.get("unresolved")
    if not isinstance(unresolved, list) or not unresolved:
        failures.append("unresolved is empty")
    elif any(not isinstance(item, str) or not item.strip() for item in unresolved):
        failures.append("unresolved contains an empty entry")

    pin_item_schema = schema.get("properties", {}).get("pins", {}).get("items", {})
    if not isinstance(pin_item_schema, dict):
        pin_item_schema = {}
    pins_schema = pin_item_schema.get("properties", {})
    if not isinstance(pins_schema, dict):
        pins_schema = {}
    component_schema = pins_schema.get("component", {})
    pin_kind_schema = pins_schema.get("pinKind", {})
    status_schema = pins_schema.get("status", {})
    component_enum = component_schema.get("enum") if isinstance(component_schema, dict) else None
    pin_kind_enum = pin_kind_schema.get("enum") if isinstance(pin_kind_schema, dict) else None
    status_enum = status_schema.get("enum") if isinstance(status_schema, dict) else None
    if component_enum != list(COMPONENTS):
        failures.append("schema component enum is not the required component order")
    if pin_kind_enum != list(PIN_KINDS):
        failures.append("schema pinKind enum is not git-commit, version, sha256, none")
    if status_enum != list(STATUSES):
        failures.append("schema status enum is not historical, absent")

    pins = ledger.get("pins")
    if not isinstance(pins, list):
        failures.append("pins is not an array")
        return
    components = [row.get("component") if isinstance(row, dict) else None for row in pins]
    if components != list(COMPONENTS):
        failures.append(
            "pins do not contain exactly one row per component in enum order: "
            + ", ".join(str(component) for component in components)
        )
    for index, row in enumerate(pins):
        if not isinstance(row, dict):
            failures.append(f"pins/{index} is not an object")
            continue
        check_key_order(pin_item_schema, row, f"pins/{index}", failures)
        check_row(root, row, index, failures)


def check_key_order(
    schema_node: dict[str, object], value: dict[str, object], label: str, failures: list[str]
) -> None:
    properties = schema_node.get("properties")
    if not isinstance(properties, dict):
        return
    expected = list(properties)
    actual = list(value)
    if actual != expected:
        failures.append(f"{label} keys {actual} are not schema order {expected}")


def check_row(root: Path, row: dict[str, object], index: int, failures: list[str]) -> None:
    label = f"pins/{index}"
    component = row.get("component")
    status = row.get("status")
    pin = row.get("pin")
    pin_kind = row.get("pinKind")
    evidence = row.get("sourceEvidence")
    if status not in STATUSES:
        failures.append(f"{label} status {status!r} is not historical or absent")
    if not isinstance(evidence, list):
        failures.append(f"{label} sourceEvidence is not an array")
        evidence = []

    absent = status == "absent"
    pin_absent = pin is None
    kind_absent = pin_kind == "none"
    if absent != pin_absent or absent != kind_absent:
        failures.append(
            f"{label} breaks absent <=> pin null <=> pinKind none "
            f"(status={status!r}, pin={pin!r}, pinKind={pin_kind!r})"
        )
    if absent:
        if evidence:
            failures.append(f"{label} absent row has sourceEvidence")
        return
    if not isinstance(pin, str) or not pin:
        failures.append(f"{label} historical pin is empty")
        return
    if pin_kind not in PIN_PATTERNS:
        failures.append(f"{label} historical pinKind {pin_kind!r} is not a pin kind")
        return
    if PIN_PATTERNS[pin_kind].fullmatch(pin) is None:
        failures.append(f"{label} pin {pin!r} does not match pinKind {pin_kind}")
    if not evidence:
        failures.append(f"{label} historical row has no sourceEvidence")
        return
    check_evidence(root, component, pin, evidence, label, failures)


def check_evidence(
    root: Path,
    component: object,
    pin: str,
    evidence: list[object],
    label: str,
    failures: list[str],
) -> None:
    found = False
    opened = 0
    for item in evidence:
        if not isinstance(item, str) or not item or Path(item).is_absolute() or ".." in Path(item).parts:
            failures.append(f"{label} sourceEvidence path {item!r} is not repo-relative")
            continue
        path = root / item
        if not path.is_file():
            failures.append(f"{label} sourceEvidence file does not exist: {item}")
            continue
        opened += 1
        try:
            blob = path.read_bytes()
        except OSError as exc:
            failures.append(f"{label} cannot read {item}: {exc}")
            continue
        if pin.encode("utf-8") in blob:
            found = True
    if opened and not found:
        failures.append(
            f"{label} component {component} pin {pin} does not occur in any sourceEvidence file"
        )


if __name__ == "__main__":
    raise SystemExit(main())
