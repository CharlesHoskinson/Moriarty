#!/usr/bin/env python3
"""Check U0 K-reconciliation citations, declarations, and verbatim quotes.

Semantic adequacy of a citation is a reviewed claim. The checker proves C1-C7 only.
"""

from __future__ import annotations

import argparse
import json
import re
from pathlib import Path

from jsonschema import Draft202012Validator
from jsonschema.exceptions import SchemaError


MORIARTY_ROOT = Path(__file__).resolve().parents[1]
SCHEMA_REL = Path(
    "openspec/changes/consolidated-language-kernel/schemas/k-reconciliation.schema.json"
)
ARTIFACT_REL = Path("deliverables/u0-semantic-contract-2026-09-23/k-reconciliation.json")
SPEC_REL = Path(
    "openspec/changes/consolidated-language-kernel/specs/consolidated-language-kernel/spec.md"
)
JUDGMENTS_REL = Path("deliverables/u0-semantic-contract-2026-09-23/judgments.json")
K_ROOT = Path("experiments/moriarty-language/formal/k")
LIMITATION = "Coverage is a reviewed claim; the checker proves citations and evidence quotes only."
UNI_HEADING = re.compile(r"^### Requirement: (UNI-\d{3}) (.+)$", re.M)
MODULE_RE = re.compile(r"\bmodule\s+([A-Za-z0-9-]+)\b(.*?)\bendmodule\b", re.S)
DECL_RE = re.compile(r"(?m)^[ \t]*(syntax|configuration|rule|context)\b")
LABEL_RE = re.compile(r"\[(?:symbol|klabel)\(([A-Za-z_][A-Za-z0-9_]*)\)\]")
RULE_LABEL_RE = re.compile(r"\brule\s*\[([A-Za-z_][A-Za-z0-9_-]*)\]\s*:")
CELL_RE = re.compile(r"<([A-Za-z_][A-Za-z0-9_]*)>")
CTOR_RE = re.compile(r"([A-Za-z_][A-Za-z0-9_]*)\s*\(")
PARTIAL_NOTE = re.compile(r"Covers:\s*\S.*?;\s*Missing:\s*\S", re.S)
LACKS_NOTE = re.compile(r"\blacks\b")


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=MORIARTY_ROOT)
    args = parser.parse_args(argv)
    root = args.root.resolve()
    if not root.is_dir():
        print("blocked: root is not a directory")
        return 2
    return check_root(root)


def input_missing(root: Path, rel: Path) -> bool:
    path = root / rel
    return not path.is_dir() if rel == K_ROOT else not path.is_file()


def check_root(root: Path) -> int:
    blocked = [
        f"blocked: missing {rel.as_posix()}"
        for rel in (SCHEMA_REL, ARTIFACT_REL, SPEC_REL, JUDGMENTS_REL, K_ROOT)
        if input_missing(root, rel)
    ]
    if blocked:
        print("\n".join(blocked))
        return 2
    failures: list[str] = []
    schema, _schema_text = load_json_text(root / SCHEMA_REL, SCHEMA_REL, failures)
    artifact, artifact_text = load_json_text(root / ARTIFACT_REL, ARTIFACT_REL, failures)
    spec = read_text(root / SPEC_REL, SPEC_REL, failures)
    judgments, _judgments_text = load_json_text(root / JUDGMENTS_REL, JUDGMENTS_REL, failures)
    if schema is None or artifact is None or spec is None or judgments is None:
        return report(failures, [])
    check_schema(schema, artifact, artifact_text, failures)
    expected = expected_rows(spec, judgments, failures)
    rows = artifact.get("rows") if isinstance(artifact, dict) else None
    if isinstance(artifact, dict) and expected is not None and isinstance(rows, list):
        check_rows(root, artifact, rows, expected, failures, blocked)
    if not failures and not blocked and isinstance(rows, list):
        counts = {"covered": 0, "partial": 0, "not-covered": 0}
        for row in rows:
            status = row.get("status") if isinstance(row, dict) else None
            if status in counts:
                counts[status] += 1
        covered, partial, uncovered = counts["covered"], counts["partial"], counts["not-covered"]
        print(f"OK: {len(rows)} rows, {covered} covered, {partial} partial, {uncovered} not-covered")
        print(LIMITATION)
    return report(failures, blocked)


def report(failures: list[str], blocked: list[str]) -> int:
    if blocked:
        print("\n".join(blocked))
        return 2
    if failures:
        print("\n".join(failures))
        return 1
    return 0


def load_json_text(path: Path, rel: Path, failures: list[str]) -> tuple[object | None, str]:
    try:
        text = path.read_text(encoding="utf-8")
        return json.loads(text), text
    except (OSError, UnicodeError, json.JSONDecodeError) as error:
        failures.append(f"FAIL: {rel.as_posix()} is not UTF-8 JSON ({error})")
        return None, ""


def read_text(path: Path, rel: Path, failures: list[str]) -> str | None:
    try:
        return path.read_text(encoding="utf-8")
    except (OSError, UnicodeError) as error:
        failures.append(f"FAIL: {rel.as_posix()} is not readable UTF-8 ({error})")
        return None


def check_schema(schema: object, artifact: object, text: str, failures: list[str]) -> None:
    if not isinstance(schema, dict):
        failures.append("FAIL: reconciliation schema is not an object")
        return
    try:
        Draft202012Validator.check_schema(schema)
    except SchemaError as error:
        failures.append(f"FAIL: reconciliation schema is not valid ({error.message})")
        return
    validator = Draft202012Validator(schema)
    errors = sorted(validator.iter_errors(artifact), key=lambda item: (list(item.absolute_path), item.message))
    for error in errors:
        location = ".".join(str(part) for part in error.absolute_path) or "<root>"
        failures.append(f"FAIL: artifact schema: {location}: {error.message}")
    canonical = json.dumps(artifact, indent=2, ensure_ascii=False) + "\n"
    if text != canonical:
        failures.append("FAIL: artifact is not canonical UTF-8 JSON")
    check_key_order(artifact, schema, "", failures)
    rows = artifact.get("rows") if isinstance(artifact, dict) else None
    if isinstance(rows, list):
        seen: set[str] = set()
        for row in rows:
            row_id = row.get("id") if isinstance(row, dict) else None
            if isinstance(row_id, str) and row_id in seen:
                failures.append(f"FAIL: duplicate row id {row_id}")
            if isinstance(row_id, str):
                seen.add(row_id)


def check_key_order(instance: object, schema: object, path: str, failures: list[str]) -> None:
    if not isinstance(schema, dict):
        return
    if schema.get("type") == "array" and isinstance(instance, list):
        for index, item in enumerate(instance):
            check_key_order(item, schema.get("items"), f"{path}[{index}]", failures)
        return
    properties = schema.get("properties")
    if schema.get("type") != "object" or not isinstance(properties, dict) or not isinstance(instance, dict):
        return
    if list(instance) != list(properties):
        failures.append(f"FAIL: {path or 'artifact'} keys are {list(instance)!r}")
    for key, child in properties.items():
        if key in instance:
            child_path = f"{path}.{key}" if path else key
            check_key_order(instance[key], child, child_path, failures)


def expected_rows(
    spec: str, judgments: object, failures: list[str]
) -> list[tuple[str, str, str]] | None:
    found = UNI_HEADING.findall(spec)
    rows = [(uni_id, "uni", title) for uni_id, title in sorted(found)]
    items = judgments.get("judgments") if isinstance(judgments, dict) else None
    if not isinstance(items, list):
        failures.append("FAIL: judgments.json has no judgments array")
        return None
    for item in items:
        key = item.get("key") if isinstance(item, dict) else None
        if not isinstance(key, str):
            failures.append("FAIL: judgments.json has an item without a key")
            return None
        rows.append((key, "judgment", key))
    return rows


def check_rows(
    root: Path,
    artifact: dict,
    rows: list[object],
    expected: list[tuple[str, str, str]],
    failures: list[str],
    blocked: list[str],
) -> None:
    actual = [
        (row.get("id"), row.get("kind"), row.get("title")) if isinstance(row, dict) else None
        for row in rows
    ]
    if actual != expected:
        failures.append("FAIL: rows are not the UNI headings followed by judgments.json keys")
        expected_ids = [item[0] for item in expected]
        actual_ids = [item[0] for item in actual if item is not None]
        if actual_ids != expected_ids:
            failures.append(f"FAIL: row ids are {actual_ids!r}")
        for row, want in zip((row for row in rows if isinstance(row, dict)), expected, strict=False):
            if row.get("title") != want[2] and row.get("id") == want[0]:
                failures.append(f"FAIL: {want[0]} title is {row.get('title')!r}")
            if row.get("kind") != want[1] and row.get("id") == want[0]:
                failures.append(f"FAIL: {want[0]} kind is {row.get('kind')!r}")
    roots = artifact.get("kRoots")
    k_roots = roots if isinstance(roots, list) and all(isinstance(item, str) for item in roots) else []
    cache: dict[str, str] = {}
    modules: dict[str, dict[str, set[str]]] = {}
    for row in rows:
        if isinstance(row, dict):
            check_status(row, failures)
            check_citations(root, row, k_roots, cache, modules, failures, blocked)
            check_evidence(root, row, cache, failures, blocked)
    check_execution(root, artifact.get("executionEvidence"), cache, failures, blocked)


def check_status(row: dict, failures: list[str]) -> None:
    row_id = row.get("id")
    status = row.get("status")
    citations = row.get("kCitations")
    evidence = row.get("evidence")
    note = row.get("note")
    if not isinstance(citations, list) or not isinstance(evidence, list) or not isinstance(note, str):
        return
    label = row_id if isinstance(row_id, str) else "<row>"
    if status == "covered" and (len(citations) < 1 or len(evidence) < 1):
        failures.append(f"FAIL: {label} covered requires a K citation and an execution quote")
    elif status == "partial":
        if len(citations) < 1:
            failures.append(f"FAIL: {label} partial requires a K citation")
        if PARTIAL_NOTE.search(note) is None:
            failures.append(f"FAIL: {label} partial note must say Covers: ...; Missing: ...")
    elif status == "not-covered":
        if citations:
            failures.append(f"FAIL: {label} not-covered has K citations")
        if LACKS_NOTE.search(note) is None:
            failures.append(f"FAIL: {label} not-covered note does not say what K lacks")


def check_citations(
    root: Path,
    row: dict,
    k_roots: list[str],
    cache: dict[str, str],
    modules: dict[str, dict[str, set[str]]],
    failures: list[str],
    blocked: list[str],
) -> None:
    citations = row.get("kCitations")
    if not isinstance(citations, list):
        return
    label = str(row.get("id"))
    for citation in citations:
        if not isinstance(citation, dict):
            continue
        file_name = citation.get("file")
        context = citation.get("context")
        symbol = citation.get("symbol")
        if not isinstance(file_name, str) or not isinstance(context, str) or not isinstance(symbol, str):
            continue
        if not under_k(file_name, k_roots):
            failures.append(f"FAIL: {label} citation file is outside kRoots: {file_name}")
            continue
        text = cached_text(root, file_name, cache, failures, blocked, label)
        if text is None:
            continue
        declared = modules.get(file_name)
        if declared is None:
            declared = module_symbols(text)
            modules[file_name] = declared
        if symbol not in declared.get(context, set()):
            failures.append(
                f"FAIL: {label} symbol {symbol!r} is not declared in module {context} in {file_name}"
            )


def check_evidence(
    root: Path,
    row: dict,
    cache: dict[str, str],
    failures: list[str],
    blocked: list[str],
) -> None:
    evidence = row.get("evidence")
    if not isinstance(evidence, list):
        return
    label = str(row.get("id"))
    for item in evidence:
        if not isinstance(item, dict):
            continue
        path = item.get("path")
        quote = item.get("quote")
        if not isinstance(path, str) or not isinstance(quote, str):
            continue
        if not under_deliverables(path):
            failures.append(f"FAIL: {label} evidence path is outside deliverables/: {path}")
            continue
        text = cached_text(root, path, cache, failures, blocked, label)
        if text is not None and quote not in text:
            failures.append(f"FAIL: {label} evidence quote does not occur in {path}")


def check_execution(
    root: Path,
    evidence: object,
    cache: dict[str, str],
    failures: list[str],
    blocked: list[str],
) -> None:
    if not isinstance(evidence, dict):
        return
    path = evidence.get("path")
    quotes = evidence.get("quotes")
    if not isinstance(path, str) or not isinstance(quotes, list):
        return
    if not under_deliverables(path):
        failures.append(f"FAIL: executionEvidence path is outside deliverables/: {path}")
        return
    text = cached_text(root, path, cache, failures, blocked, "executionEvidence")
    if text is None:
        return
    for quote in quotes:
        if isinstance(quote, str) and quote not in text:
            failures.append(f"FAIL: executionEvidence quote does not occur in {path}")


def cached_text(
    root: Path,
    rel: str,
    cache: dict[str, str],
    failures: list[str],
    blocked: list[str],
    label: str,
) -> str | None:
    if rel in cache:
        return cache[rel]
    kind, path = locate(root, rel)
    if kind == "escape":
        failures.append(f"FAIL: {label} path escapes the root: {rel}")
        return None
    if kind == "missing" or path is None:
        blocked.append(f"blocked: missing {rel}")
        return None
    try:
        cache[rel] = path.read_text(encoding="utf-8")
    except (OSError, UnicodeError) as error:
        failures.append(f"FAIL: {label} file is not readable UTF-8 ({error})")
        return None
    return cache[rel]


def locate(root: Path, rel: str) -> tuple[str, Path | None]:
    path = Path(rel)
    if not rel or path.is_absolute() or ".." in path.parts:
        return "escape", None
    target = root / path
    if not target.is_file():
        return "missing", None
    return "ok", target


def under_deliverables(rel: str) -> bool:
    path = Path(rel)
    return bool(path.parts) and not path.is_absolute() and ".." not in path.parts and path.parts[0] == "deliverables"


def under_k(rel: str, roots: list[str]) -> bool:
    path = Path(rel)
    if not rel or path.is_absolute() or ".." in path.parts:
        return False
    return any(Path(root) in path.parents for root in roots)


def module_symbols(text: str) -> dict[str, set[str]]:
    found: dict[str, set[str]] = {}
    for name, body in MODULE_RE.findall(strip_comments(text)):
        symbols = found.setdefault(name, {name})
        marks = list(DECL_RE.finditer(body))
        for index, mark in enumerate(marks):
            end = marks[index + 1].start() if index + 1 < len(marks) else len(body)
            block = body[mark.start():end]
            kind = mark.group(1)
            if kind == "syntax":
                rhs = re.sub(r"\[[^\[\]]*\]", " ", block.split("::=", 1)[-1])
                symbols.update(LABEL_RE.findall(block))
                symbols.update(CTOR_RE.findall(rhs))
            elif kind == "configuration":
                symbols.update(f"<{cell}>" for cell in CELL_RE.findall(block))
            elif kind == "rule":
                symbols.update(RULE_LABEL_RE.findall(block))
    return found


def strip_comments(text: str) -> str:
    text = re.sub(r"/\*.*?\*/", "", text, flags=re.S)
    return re.sub(r"//[^\n]*", "", text)


if __name__ == "__main__":
    raise SystemExit(main())
