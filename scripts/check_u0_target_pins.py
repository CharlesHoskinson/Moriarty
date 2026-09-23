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
# Files produced by this U0 package are not independent pin evidence.
GENERATED_EXACT = {
    SCHEMA_REL.as_posix(),
    LEDGER_REL.as_posix(),
    "scripts/check_u0_target_pins.py",
    "tests/test_u0_target_pins.py",
    "FOREMAN_REPORT.md",
}
GENERATED_PREFIX = "deliverables/u0-semantic-contract-2026-09-23/"
PATH_RE = re.compile(
    r"(?<![A-Za-z0-9_./-])"
    r"(?:deliverables|docs|experiments|openspec|scripts|tests)"
    r"(?:/[A-Za-z0-9_+-]+)+(?:\.[A-Za-z0-9]+)+"
)
TOKEN_RE = re.compile(
    r"(?<![0-9A-Za-z.])"
    r"(?:[0-9a-f]{64}|[0-9a-f]{40}|[0-9a-f]{7,39}|[0-9]+\.[0-9]+\.[0-9]+)"
    r"(?![0-9A-Za-z])"
)
CLAIM_RE = re.compile(
    r"(?i)\bre-?verified\b|\bre-?verify\b|\bverified\b|\bcompatible\b|"
    r"\bcompatibility\s+is\s+established\b"
)
NEGATION_RE = re.compile(r"(?i)\b(?:not|never|no|without|unknown|unresolved|false)\b|n't")
# Independent records used to reject a false absent row and a self-cited pin.
COMPONENT_PIN_RECORDS: dict[str, tuple[tuple[str, re.Pattern[bytes]], ...]] = {
    "moriarty-compiler": (
        (
            "experiments/moriarty-language/package.json",
            re.compile(rb'"version"\s*:\s*"([0-9]+\.[0-9]+\.[0-9]+)"'),
        ),
    ),
    "compact-compiler": (
        (
            "deliverables/lifecycle-corpus-2026-09-17/environment.json",
            re.compile(rb'"compact_compiler"\s*:\s*"([0-9]+\.[0-9]+\.[0-9]+)"'),
        ),
        (
            "experiments/moriarty-midnight-network/origins.json",
            re.compile(rb'"compactCompiler"\s*:\s*"([0-9]+\.[0-9]+\.[0-9]+)"'),
        ),
    ),
    "zkir": (
        (
            "deliverables/consolidated-design-2026-09-19/astra-backend-requirements.md",
            re.compile(rb"midnight-zkir`, clean tracked working tree at `([0-9a-f]{40})`"),
        ),
    ),
    "native-proof-system": (
        (
            "experiments/moriarty-native-ivc-r3/checked-encoding-resources.json",
            re.compile(rb'"backendPin"\s*:\s*"([0-9a-f]{40})"'),
        ),
    ),
    "verifier": (
        (
            "deliverables/consolidated-design-2026-09-19/astra-backend-integration.md",
            re.compile(rb"midnight-zk@([0-9a-f]{40})`, `aggregation/src/ivc/verifier\.rs"),
        ),
    ),
    "proving-keys": (
        (
            "experiments/moriarty-native-ivc-r3/checked-encoding-resources.json",
            re.compile(
                rb'"(?:provingKey|proving_key|proving-key)[^"\n]{0,40}"\s*:\s*"([0-9a-f]{40,64})"'
            ),
        ),
    ),
    "verifier-keys": (
        (
            "experiments/moriarty-native-ivc-r3/checked-encoding-resources.json",
            re.compile(
                rb'"(?:verifierKey|verifier_key|verifier-key)[^"\n]{0,40}"\s*:\s*"([0-9a-f]{40,64})"'
            ),
        ),
    ),
    "srs-parameters": (
        (
            "experiments/moriarty-native-ivc-r3/checked-encoding-resources.json",
            re.compile(rb'"sha256"\s*:\s*"([0-9a-f]{64})"'),
        ),
    ),
    "ledger": (
        (
            "deliverables/consolidated-design-2026-09-19/astra-backend-requirements.md",
            re.compile(rb"midnight-ledger` at `([0-9a-f]{40})`"),
        ),
    ),
    "proof-server": (
        (
            "experiments/moriarty-midnight-network/compose.preview.yml",
            re.compile(rb"proof-server:[0-9]+\.[0-9]+\.[0-9]+@sha256:([0-9a-f]{64})"),
        ),
    ),
    "k-reference-toolchain": (
        (
            "experiments/moriarty-language/formal/k/toolchain.lock.json",
            re.compile(rb'"revision"\s*:\s*"([0-9a-f]{40})"'),
        ),
    ),
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
    schema_ok = check_schema(schema, ledger, failures)
    check_ledger(root, schema, ledger, failures, check_rows=schema_ok)
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
    if text != json.dumps(ledger, indent=2, ensure_ascii=False) + "\n":
        failures.append(f"{LEDGER_REL.as_posix()} is not canonical 2-space JSON")


def check_schema(schema: object, ledger: object, failures: list[str]) -> bool:
    if not isinstance(schema, dict):
        failures.append("schema is not an object")
        return False
    try:
        validator = Draft202012Validator(schema, format_checker=FormatChecker())
    except SchemaError as exc:
        failures.append(f"schema is not a valid Draft 2020-12 document: {exc.message}")
        return False
    errors = sorted(validator.iter_errors(ledger), key=lambda item: list(item.absolute_path))
    for error in errors:
        location = "/".join(str(part) for part in error.absolute_path) or "<root>"
        failures.append(f"schema {location}: {error.message}")
    return not errors


def check_ledger(
    root: Path,
    schema: object,
    ledger: object,
    failures: list[str],
    *,
    check_rows: bool,
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
    elif isinstance(unresolved, list):
        for index, item in enumerate(unresolved):
            if isinstance(item, str):
                check_cited_text(root, item, f"unresolved/{index}", [], failures)

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
    if not check_rows:
        return
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
    note = row.get("note")
    if not isinstance(status, str) or status not in STATUSES:
        failures.append(f"{label} status {status!r} is not historical or absent")
    if not isinstance(evidence, list):
        failures.append(f"{label} sourceEvidence is not an array")
        evidence_items: list[object] = []
    else:
        evidence_items = list(evidence)
    note_text = note if isinstance(note, str) else ""
    if not note_text.strip():
        failures.append(f"{label} note is empty")

    absent = status == "absent"
    pin_absent = pin is None
    kind_absent = pin_kind == "none"
    if absent != pin_absent or absent != kind_absent:
        failures.append(
            f"{label} breaks absent <=> pin null <=> pinKind none "
            f"(status={status!r}, pin={pin!r}, pinKind={pin_kind!r})"
        )
    if absent:
        if evidence_items:
            failures.append(f"{label} absent row has sourceEvidence")
        if isinstance(component, str):
            check_absent_inventory(root, component, label, failures)
        else:
            failures.append(f"{label} component is not a string")
        check_cited_text(root, note_text, f"{label} note", [], failures)
        return

    cited = [item for item in evidence_items if isinstance(item, str)]
    if not isinstance(pin, str) or not pin:
        failures.append(f"{label} historical pin is empty")
    elif not isinstance(pin_kind, str) or pin_kind not in PIN_PATTERNS:
        failures.append(f"{label} historical pinKind {pin_kind!r} is not a pin kind")
    elif PIN_PATTERNS[pin_kind].fullmatch(pin) is None:
        failures.append(f"{label} pin {pin!r} does not match pinKind {pin_kind}")
    elif not evidence_items:
        failures.append(f"{label} historical row has no sourceEvidence")
    else:
        check_evidence(root, component, pin, evidence_items, label, failures)
        check_independent_record(root, component, pin, cited, label, failures)
    check_cited_text(root, note_text, f"{label} note", cited, failures)


def check_evidence(
    root: Path,
    component: object,
    pin: str,
    evidence: list[object],
    label: str,
    failures: list[str],
) -> None:
    for item in evidence:
        if not isinstance(item, str) or not item or Path(item).is_absolute() or ".." in Path(item).parts:
            failures.append(f"{label} sourceEvidence path {item!r} is not repo-relative")
            continue
        if is_generated(item):
            failures.append(
                f"{label} sourceEvidence {item} is a generated artifact "
                f"and is not an independent record of {component}"
            )
            continue
        path = root / item
        if not path.is_file():
            failures.append(f"{label} sourceEvidence file does not exist: {item}")
            continue
        try:
            blob = path.read_bytes()
        except OSError as exc:
            failures.append(f"{label} cannot read {item}: {exc}")
            continue
        if not pin_in_blob(pin, blob):
            failures.append(f"{label} sourceEvidence file {item} does not contain pin {pin}")


def check_independent_record(
    root: Path,
    component: object,
    pin: str,
    cited: list[str],
    label: str,
    failures: list[str],
) -> None:
    if not isinstance(component, str):
        failures.append(f"{label} component is not a string")
        return
    rules = COMPONENT_PIN_RECORDS.get(component)
    if not rules:
        failures.append(f"{label} component {component} has no independent pin record")
        return
    cited_set = {item for item in cited if not is_generated(item)}
    matched = False
    for relative, pattern in rules:
        captures = inventory_captures(root, relative, pattern, label, failures)
        if relative in cited_set and pin in captures:
            matched = True
    if not matched:
        failures.append(
            f"{label} component {component} does not cite a source file "
            f"that independently records pin {pin}"
        )


def check_absent_inventory(
    root: Path, component: str, label: str, failures: list[str]
) -> None:
    rules = COMPONENT_PIN_RECORDS.get(component, ())
    reported: set[tuple[str, str]] = set()
    for relative, pattern in rules:
        for captured in inventory_captures(root, relative, pattern, label, failures):
            key = (relative, captured)
            if key in reported:
                continue
            reported.add(key)
            failures.append(
                f"{label} component {component} is absent but {relative} records pin {captured}"
            )


def inventory_captures(
    root: Path,
    relative: str,
    pattern: re.Pattern[bytes],
    label: str,
    failures: list[str],
) -> list[str]:
    path = root / relative
    if not path.is_file():
        failures.append(f"{label} pin inventory file does not exist: {relative}")
        return []
    try:
        blob = path.read_bytes()
    except OSError as exc:
        failures.append(f"{label} cannot read pin inventory {relative}: {exc}")
        return []
    return [match.group(1).decode("ascii") for match in pattern.finditer(blob)]


def check_cited_text(
    root: Path,
    text: str,
    label: str,
    extra_files: list[str],
    failures: list[str],
) -> None:
    check_claims(text, label, failures)
    search_files: list[str] = []
    for relative in extract_paths(text):
        if is_generated(relative):
            failures.append(f"{label} path {relative} is a generated artifact")
            continue
        if not (root / relative).is_file():
            failures.append(f"{label} path {relative} does not exist")
            continue
        search_files.append(relative)
    for relative in extra_files:
        if is_generated(relative) or relative in search_files:
            continue
        if (root / relative).is_file():
            search_files.append(relative)
    for token in extract_tokens(text):
        if any(pin_in_file(root, relative, token) for relative in search_files):
            continue
        failures.append(f"{label} token {token} does not occur in a cited or mentioned file")


def check_claims(text: str, label: str, failures: list[str]) -> None:
    for match in CLAIM_RE.finditer(text):
        start = max(0, match.start() - 80)
        end = min(len(text), match.end() + 80)
        if NEGATION_RE.search(text[start:end]):
            continue
        failures.append(
            f"{label} claims re-verification or compatibility near {match.group()!r}"
        )


def extract_paths(text: str) -> list[str]:
    return list(dict.fromkeys(match.group(0) for match in PATH_RE.finditer(text)))


def extract_tokens(text: str) -> list[str]:
    return list(dict.fromkeys(match.group(0) for match in TOKEN_RE.finditer(text)))


def is_generated(relative: str) -> bool:
    normalized = Path(relative).as_posix()
    return normalized in GENERATED_EXACT or normalized.startswith(GENERATED_PREFIX)


def pin_in_file(root: Path, relative: str, pin: str) -> bool:
    path = root / relative
    try:
        blob = path.read_bytes()
    except OSError:
        return False
    return pin_in_blob(pin, blob)


def pin_in_blob(pin: str, blob: bytes) -> bool:
    pattern = rb"(?<![0-9A-Za-z.])" + re.escape(pin.encode("utf-8")) + rb"(?![0-9A-Za-z])"
    return re.search(pattern, blob) is not None


if __name__ == "__main__":
    raise SystemExit(main())
