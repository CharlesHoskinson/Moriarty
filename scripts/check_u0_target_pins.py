#!/usr/bin/env python3
"""Check the U0 target-pin ledger against its schema and on-disk evidence."""

from __future__ import annotations

import argparse
import json
import os
import re
from pathlib import Path

from jsonschema import Draft202012Validator, FormatChecker
from jsonschema.exceptions import SchemaError, UnknownType


MORIARTY_ROOT = Path(__file__).resolve().parents[1]
SCHEMA_REL = Path(
    "openspec/changes/consolidated-language-kernel/schemas/target-pins.schema.json"
)
LEDGER_REL = Path("deliverables/u0-semantic-contract-2026-09-23/target-pins.json")
SCHEMA_VERSION = "moriarty-u0-target-pins/1"
AS_OF = "2026-09-23"
LIMITATION = (
    "absent means no pin found by this recorded search; not a proof that none exists"
)
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
CLAIM_KINDS = ("found-in", "conflict", "not-found", "unresolved")
EVIDENCE_KINDS = ("found-in", "conflict")
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
GENERATED_PREFIXES = (
    "deliverables/u0-semantic-contract-2026-09-23/",
    "openspec/changes/consolidated-language-kernel/schemas/",
)
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
# Ruling 1: reject these words in note with no negation parse.
BANNED_NOTE_RE = re.compile(
    r"(?i)\b(?:re-?verified|verified|compatible|current|validated)\b"
)
NOTE_LIMIT = 160
# Checker-owned absence search. The ledger cannot narrow these values.
REQUIRED_ABSENCE_ROOTS = (
    "deliverables",
    "experiments",
    "docs",
    "openspec",
)
REQUIRED_PIN_PATTERNS = (
    r"(?<![0-9a-f])[0-9a-f]{64}(?![0-9a-f])",
    r"(?<![0-9a-f])[0-9a-f]{40}(?![0-9a-f])",
    r"(?<![0-9A-Za-z])[0-9]+\.[0-9]+\.[0-9]+(?![0-9A-Za-z])",
)
REQUIRED_PIN_REGEXES = tuple(re.compile(pattern) for pattern in REQUIRED_PIN_PATTERNS)
REQUIRED_SEARCH_TERMS: dict[str, tuple[str, ...]] = {
    "moriarty-compiler": ("Moriarty commit", "Frozen base"),
    "compact-compiler": ("compact_compiler", "compactCompiler"),
    "zkir": ("midnight-zkir", "zkir-v3"),
    "native-proof-system": ("backendPin", "midnight-zk@"),
    "verifier": ("ivc/verifier.rs", "midnight-zk@"),
    "proving-keys": ("accrue.prover", "initialize.prover"),
    "verifier-keys": ("accrue.verifier", "initialize.verifier"),
    "srs-parameters": ("bls_midnight_2p17", '"k": 17'),
    "ledger": ("midnight-ledger", "ledger8"),
    "proof-server": ("proof-server:", "midnightntwrk/proof-server"),
    "k-reference-toolchain": ('"revision"', "7.1.337"),
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

    schema, schema_error = load_schema(schema_path)
    if schema_error is not None:
        print(schema_error)
        return 1 if schema_error.startswith("FAIL:") else 2
    ledger_text, ledger, ledger_error = load_json_text(ledger_path, LEDGER_REL)
    if ledger_error is not None:
        print(ledger_error)
        return 1 if ledger_error.startswith("FAIL:") else 2

    failures: list[str] = []
    if not schema_document_ok(schema, failures):
        for failure in failures:
            print(f"FAIL: {failure}")
        return 1
    check_canonical(ledger_text, ledger, failures)
    check_schema(schema, ledger, failures)
    # Row evidence checks run even when instance validation failed.
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
        f"compatible tuple NOT established; {LIMITATION}"
    )
    return 0


def load_schema(path: Path) -> tuple[object, str | None]:
    try:
        text = path.read_text(encoding="utf-8")
    except OSError as exc:
        return None, f"blocked: cannot read {SCHEMA_REL.as_posix()}: {exc}"
    except UnicodeDecodeError as exc:
        return None, f"FAIL: {SCHEMA_REL.as_posix()} is not UTF-8: {exc}"
    try:
        return json.loads(text), None
    except json.JSONDecodeError as exc:
        return None, f"FAIL: {SCHEMA_REL.as_posix()} is not JSON: {exc}"


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


def _one_line(exc: BaseException) -> str:
    message = getattr(exc, "message", "")
    if not isinstance(message, str) or not message.strip():
        message = str(exc)
    return " ".join(message.split())


def schema_document_ok(schema: object, failures: list[str]) -> bool:
    if not isinstance(schema, dict):
        failures.append("schema is not an object")
        return False
    try:
        Draft202012Validator.check_schema(schema)
    except SchemaError as exc:
        failures.append(
            "schema is not a valid Draft 2020-12 document: " + _one_line(exc)
        )
        return False
    return True


def check_schema(schema: object, ledger: object, failures: list[str]) -> bool:
    if not isinstance(schema, dict):
        failures.append("schema is not an object")
        return False
    try:
        validator = Draft202012Validator(schema, format_checker=FormatChecker())
    except SchemaError as exc:
        failures.append(
            "schema is not a valid Draft 2020-12 document: " + _one_line(exc)
        )
        return False
    try:
        errors = sorted(
            validator.iter_errors(ledger),
            key=lambda item: list(item.absolute_path),
        )
    except (SchemaError, UnknownType) as exc:
        failures.append(
            "schema is not a valid Draft 2020-12 document: " + _one_line(exc)
        )
        return False
    for error in errors:
        location = "/".join(str(part) for part in error.absolute_path) or "<root>"
        failures.append(f"schema {location}: {error.message}")
    return not errors


def check_ledger(root: Path, schema: object, ledger: object, failures: list[str]) -> None:
    if not isinstance(ledger, dict):
        failures.append("ledger is not an object")
        return
    schema_node = schema if isinstance(schema, dict) else {}
    check_key_order(schema_node, ledger, "<root>", failures)
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
    else:
        for index, item in enumerate(unresolved):
            if isinstance(item, str):
                check_cited_text(root, item, f"unresolved/{index}", [], failures)

    properties = schema_node.get("properties", {})
    if not isinstance(properties, dict):
        properties = {}
    absence_schema = properties.get("absenceSearch", {})
    if not isinstance(absence_schema, dict):
        absence_schema = {}
    check_absence_search(ledger.get("absenceSearch"), absence_schema, failures)

    pins_schema = properties.get("pins", {})
    if not isinstance(pins_schema, dict):
        pins_schema = {}
    pin_item_schema = pins_schema.get("items", {})
    if not isinstance(pin_item_schema, dict):
        pin_item_schema = {}
    row_properties = pin_item_schema.get("properties", {})
    if not isinstance(row_properties, dict):
        row_properties = {}
    component_schema = row_properties.get("component", {})
    pin_kind_schema = row_properties.get("pinKind", {})
    status_schema = row_properties.get("status", {})
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


def check_absence_search(
    absence: object, schema_node: dict[str, object], failures: list[str]
) -> None:
    if not isinstance(absence, dict):
        failures.append("absenceSearch is not an object")
        return
    check_key_order(schema_node, absence, "absenceSearch", failures)
    if absence.get("limitation") != LIMITATION:
        failures.append("absenceSearch limitation is not the required sentence")
    if absence.get("pinPatterns") != list(REQUIRED_PIN_PATTERNS):
        failures.append(
            "absenceSearch pinPatterns are not the required 64-hex, 40-hex, and semver patterns"
        )
    if absence.get("roots") != list(REQUIRED_ABSENCE_ROOTS):
        failures.append(
            "absenceSearch roots are not deliverables, experiments, docs, openspec"
        )


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


def check_row(
    root: Path,
    row: dict[str, object],
    index: int,
    failures: list[str],
) -> None:
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
    elif len(note_text) > NOTE_LIMIT:
        failures.append(f"{label} note is longer than {NOTE_LIMIT} characters")
    banned = list(dict.fromkeys(match.group(0).lower() for match in BANNED_NOTE_RE.finditer(note_text)))
    for word in banned:
        failures.append(f"{label} note contains banned word {word!r}")
    check_claims(root, row, label, failures)

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
        if not isinstance(component, str):
            failures.append(f"{label} component is not a string")
        else:
            check_absent_row(root, row, component, label, failures)
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
    check_cited_text(root, note_text, f"{label} note", cited, failures)


def check_claims(root: Path, row: dict[str, object], label: str, failures: list[str]) -> None:
    claims = row.get("claims")
    if not isinstance(claims, list):
        failures.append(f"{label} claims is not an array")
        return
    if not claims:
        failures.append(f"{label} claims is empty")
    found_in = 0
    claim_schema_properties = ("kind", "text", "evidencePath", "evidenceQuote")
    for index, claim in enumerate(claims):
        claim_label = f"{label} claims/{index}"
        if not isinstance(claim, dict):
            failures.append(f"{claim_label} is not an object")
            continue
        if list(claim) != list(claim_schema_properties):
            failures.append(
                f"{claim_label} keys {list(claim)} are not schema order {list(claim_schema_properties)}"
            )
        kind = claim.get("kind")
        text = claim.get("text")
        evidence_path = claim.get("evidencePath")
        quote = claim.get("evidenceQuote")
        if kind not in CLAIM_KINDS:
            failures.append(f"{claim_label} kind {kind!r} is not a claim kind")
        if not isinstance(text, str) or not text.strip():
            failures.append(f"{claim_label} text is empty")
        if kind == "found-in":
            found_in += 1
        if kind in EVIDENCE_KINDS:
            check_evidence_quote(root, evidence_path, quote, claim_label, required=True, failures=failures)
        else:
            check_evidence_quote(root, evidence_path, quote, claim_label, required=False, failures=failures)
    if row.get("status") == "historical" and found_in < 1:
        failures.append(f"{label} historical row has no found-in claim")
    pin = row.get("pin")
    if row.get("status") == "historical" and isinstance(pin, str) and pin:
        quotes = [
            claim.get("evidenceQuote")
            for claim in claims
            if isinstance(claim, dict) and claim.get("kind") == "found-in"
        ]
        if not any(isinstance(quote, str) and pin in quote for quote in quotes):
            failures.append(
                f"{label} historical row has no found-in evidenceQuote that contains its pin"
            )
    if row.get("status") == "absent":
        for index, claim in enumerate(claims):
            if isinstance(claim, dict) and claim.get("kind") in EVIDENCE_KINDS:
                failures.append(f"{label} absent row has a {claim['kind']} claim")


def check_evidence_quote(
    root: Path,
    evidence_path: object,
    quote: object,
    label: str,
    *,
    required: bool,
    failures: list[str],
) -> None:
    path_set = evidence_path is not None
    quote_set = quote is not None
    if required and (not path_set or not quote_set):
        failures.append(f"{label} found-in or conflict claim lacks evidencePath or evidenceQuote")
        return
    if path_set != quote_set:
        failures.append(f"{label} evidence citation is incomplete")
        return
    if not path_set:
        return
    if not isinstance(evidence_path, str) or not isinstance(quote, str):
        failures.append(f"{label} evidence citation is not a string")
        return
    if len(quote) < 12:
        failures.append(f"{label} evidenceQuote is shorter than 12 characters")
        return
    if not repo_relative(evidence_path):
        failures.append(f"{label} evidencePath {evidence_path!r} is not repo-relative")
        return
    if is_generated(evidence_path):
        failures.append(
            f"{label} evidencePath {evidence_path} is a generated artifact "
            "and is not an independent record"
        )
        return
    path = root / evidence_path
    if not path.is_file():
        failures.append(f"{label} evidencePath does not exist: {evidence_path}")
        return
    try:
        text = path.read_text(encoding="utf-8")
    except (OSError, UnicodeDecodeError) as exc:
        failures.append(f"{label} cannot read {evidence_path}: {exc}")
        return
    if quote not in text:
        shown = quote if len(quote) <= 80 else quote[:80] + "..."
        failures.append(f"{label} evidenceQuote does not occur in {evidence_path}: {shown!r}")


def check_absent_row(
    root: Path,
    row: dict[str, object],
    component: str,
    label: str,
    failures: list[str],
) -> None:
    terms = row.get("searchTerms")
    declared = [item for item in terms if isinstance(item, str)] if isinstance(terms, list) else []
    if len([item for item in declared if item.strip()]) < 2:
        failures.append(f"{label} absent row searchTerms has fewer than 2 terms")
    required_terms = REQUIRED_SEARCH_TERMS.get(component)
    if required_terms is None:
        failures.append(f"{label} component {component} has no required absence search terms")
        required_terms = ()
    for term in required_terms:
        if term not in declared:
            failures.append(f"{label} searchTerms omit required term {term!r}")
    excluded = parsed_exclusions(row, label, failures)
    hits, scan_failures = find_absence_hits(root, component, declared)
    failures.extend(f"{label} {item}" for item in scan_failures)
    pins_on_line: dict[tuple[str, int], set[str]] = {}
    for relative, line_no, pin_text in hits:
        pins_on_line.setdefault((relative, line_no), set()).add(pin_text)
    suppressed: set[tuple[str, int, str]] = set()
    for item in excluded:
        key = (item["path"], item["line"])
        pins_here = pins_on_line.get(key)
        hit_label = f"{label} excludedHits/{item['index']}"
        if not pins_here:
            failures.append(f"{hit_label} is not an absence hit: {item['path']}:{item['line']}")
            continue
        match = item["match"]
        if match is None:
            if len(pins_here) > 1:
                failures.append(
                    f"{hit_label} omits match and {item['path']}:{item['line']} has multiple pins"
                )
                continue
            suppressed.add((item["path"], item["line"], next(iter(pins_here))))
            continue
        if match not in pins_here:
            failures.append(
                f"{hit_label} match is not a pin on {item['path']}:{item['line']}"
            )
            continue
        suppressed.add((item["path"], item["line"], match))
    for relative, line_no, pin_text in hits:
        if (relative, line_no, pin_text) in suppressed:
            continue
        failures.append(
            f"{label} component {component} is absent but {relative}:{line_no} "
            f"matches a search term and a pin pattern: {pin_text}"
        )


def parsed_exclusions(
    row: dict[str, object], label: str, failures: list[str]
) -> list[dict[str, object]]:
    parsed: list[dict[str, object]] = []
    hits = row.get("excludedHits")
    if not isinstance(hits, list):
        failures.append(f"{label} excludedHits is not an array")
        return parsed
    for index, item in enumerate(hits):
        hit_label = f"{label} excludedHits/{index}"
        if not isinstance(item, dict):
            failures.append(f"{hit_label} is not an object")
            continue
        expected = ["path", "line", "reason"]
        if "match" in item:
            expected.append("match")
        if list(item) != expected:
            failures.append(
                f"{hit_label} keys {list(item)} are not schema order {expected}"
            )
        path = item.get("path")
        line_no = item.get("line")
        reason = item.get("reason")
        if not isinstance(path, str) or not path or not repo_relative(path):
            failures.append(f"{hit_label} path {path!r} is not repo-relative")
            continue
        if isinstance(line_no, bool) or not isinstance(line_no, int) or line_no < 1:
            failures.append(f"{hit_label} line {line_no!r} is not a positive integer")
            continue
        if not isinstance(reason, str) or not reason.strip():
            failures.append(f"{hit_label} reason is empty")
            continue
        match: str | None = None
        if "match" in item:
            raw_match = item.get("match")
            if not isinstance(raw_match, str) or not raw_match.strip():
                failures.append(f"{hit_label} match is empty")
                continue
            match = raw_match
        parsed.append({"path": path, "line": line_no, "match": match, "index": index})
    return parsed


def find_absence_hits(
    root: Path,
    component: str,
    extra_terms: list[str],
) -> tuple[list[tuple[str, int, str]], list[str]]:
    """Return (path, pin-line, pin-text) hits and scan problems.

    A hit is a pin-pattern match on the same line as a search term or on one of
    the next two lines. Roots and pin patterns are checker-owned. Non-UTF-8
    bytes are replaced so an ASCII pin is still visible. The ledger directory
    and the schema directory are not searched.
    """
    failures: list[str] = []
    hits: set[tuple[str, int, str]] = set()
    terms = list(REQUIRED_SEARCH_TERMS.get(component, ()))
    for term in extra_terms:
        if term and term not in terms:
            terms.append(term)
    folded_terms = [term.casefold() for term in terms if term]
    if not folded_terms:
        return [], failures
    files: list[tuple[str, Path]] = []
    for rel_root in REQUIRED_ABSENCE_ROOTS:
        if Path(rel_root).is_absolute() or ".." in Path(rel_root).parts:
            failures.append(f"absenceSearch root {rel_root!r} is not a relative directory")
            continue
        base = root / rel_root
        if not base.is_dir():
            failures.append(f"absenceSearch root does not exist: {rel_root}")
            continue
        for dirpath, dirnames, filenames in os.walk(base, followlinks=False):
            dirnames[:] = sorted(name for name in dirnames if name not in {".git", "node_modules"})
            for name in sorted(filenames):
                path = Path(dirpath) / name
                if not path.is_file():
                    continue
                relative = path.relative_to(root).as_posix()
                if is_generated(relative):
                    continue
                files.append((relative, path))
    for relative, path in files:
        try:
            text = path.read_text(encoding="utf-8", errors="replace")
        except OSError as exc:
            failures.append(f"cannot read {relative} during absence search: {exc}")
            continue
        lines = text.split("\n")
        for index, line in enumerate(lines):
            folded = line.casefold()
            if not any(term in folded for term in folded_terms):
                continue
            end = min(len(lines), index + 3)
            for offset in range(index, end):
                for pattern in REQUIRED_PIN_REGEXES:
                    for found in pattern.finditer(lines[offset]):
                        hits.add((relative, offset + 1, found.group(0)))
    return sorted(hits), failures


def check_evidence(
    root: Path,
    component: object,
    pin: str,
    evidence: list[object],
    label: str,
    failures: list[str],
) -> None:
    for item in evidence:
        if not isinstance(item, str) or not repo_relative(item):
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


def check_cited_text(
    root: Path,
    text: str,
    label: str,
    extra_files: list[str],
    failures: list[str],
) -> None:
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


def extract_paths(text: str) -> list[str]:
    return list(dict.fromkeys(match.group(0) for match in PATH_RE.finditer(text)))


def extract_tokens(text: str) -> list[str]:
    return list(dict.fromkeys(match.group(0) for match in TOKEN_RE.finditer(text)))


def repo_relative(relative: str) -> bool:
    path = Path(relative)
    return bool(relative) and not path.is_absolute() and ".." not in path.parts


def is_generated(relative: str) -> bool:
    normalized = Path(relative).as_posix()
    return normalized in GENERATED_EXACT or normalized.startswith(GENERATED_PREFIXES)


def pin_in_file(root: Path, relative: str, pin: str) -> bool:
    path = root / relative
    try:
        blob = path.read_bytes()
    except OSError:
        return False
    return pin_in_blob(pin, blob)


def pin_in_blob(pin: str, blob: bytes) -> bool:
    """Return true when pin occurs literally, including inside a longer token."""
    return pin.encode("utf-8") in blob


if __name__ == "__main__":
    raise SystemExit(main())
