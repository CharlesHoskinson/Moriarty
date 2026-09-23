#!/usr/bin/env python3
"""Check the U0 stage-relation schema, source/Core embeddings, and judgments."""

from __future__ import annotations

import argparse
import json
from pathlib import Path

from jsonschema import Draft202012Validator
from jsonschema.exceptions import SchemaError


MORIARTY_ROOT = Path(__file__).resolve().parents[1]
SCHEMA_REL = Path(
    "openspec/changes/consolidated-language-kernel/schemas/stage-relation.schema.json"
)
EMBEDDINGS_REL = Path("deliverables/u0-semantic-contract-2026-09-23/source-core-embeddings.json")
JUDGMENTS_REL = Path("deliverables/u0-semantic-contract-2026-09-23/judgments.json")
LANGUAGE_ROOT = Path("experiments/moriarty-language")

METASCHEMA = "https://json-schema.org/draft/2020-12/schema"
SCHEMA_ID = "https://moriarty.invalid/u0/stage-relation/1"
SCHEMA_TITLE = "Moriarty canonical stage relation"
STAGE_VERSION = "moriarty-stage-relation/1"
EMBEDDINGS_VERSION = "moriarty-u0-embeddings/1"
JUDGMENTS_VERSION = "moriarty-u0-judgments/1"
SOURCE_PROFILE = "moriarty-financial-agreement-source/5"
CORE_PROFILE = "moriarty-financial-lifecycle/1"
JUDGMENT_KEYS = ["stage", "intent", "effect", "authority", "history", "failure"]
STATUS_ENUM = ["held", "violated", "unchecked"]
OUTCOME_ENUM = ["continuation", "terminal"]
DESIGN_DOC_JUDGMENT = {
    "stage": "contract properties",
    "intent": "intent refinement",
    "effect": "valid state/effect transition",
    "authority": "valid state/effect transition",
    "history": "compliant history",
    "failure": "rejection/partial failure transition",
}
EMBEDDING_KEYS = [
    "schemaVersion",
    "sourceProfile",
    "coreProfile",
    "rows",
]
ROW_KEYS = [
    "schemaField",
    "sourceFile",
    "sourceSymbol",
    "coreFile",
    "coreSymbol",
    "present",
    "note",
]
JUDGMENT_FILE_KEYS = ["schemaVersion", "judgments"]
JUDGMENT_ROW_KEYS = ["key", "designDocJudgment", "definition", "schemaFields"]


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=MORIARTY_ROOT)
    args = parser.parse_args(argv)
    return check_root(args.root.resolve())


def check_root(root: Path) -> int:
    if not root.is_dir():
        print("blocked: root is not a directory")
        return 2
    missing = [rel for rel in (SCHEMA_REL, EMBEDDINGS_REL, JUDGMENTS_REL) if not (root / rel).is_file()]
    if missing:
        for rel in missing:
            print(f"blocked: missing {rel.as_posix()}")
        return 2

    failures: list[str] = []
    schema = load_json(root / SCHEMA_REL, SCHEMA_REL, failures)
    embeddings = load_json(root / EMBEDDINGS_REL, EMBEDDINGS_REL, failures)
    judgments = load_json(root / JUDGMENTS_REL, JUDGMENTS_REL, failures)
    if failures or schema is None or embeddings is None or judgments is None:
        return report(failures)

    leaves = check_stage_schema(schema, failures)
    check_embeddings(root, embeddings, leaves, failures)
    check_judgments(judgments, leaves, failures)
    if failures:
        return report(failures)
    present = sum(1 for row in embeddings["rows"] if row["present"] is True)
    absent = len(leaves) - present
    print(f"OK: {len(leaves)} leaf fields, {present} present, {absent} absent")
    return 0


def report(failures: list[str]) -> int:
    for failure in failures:
        print(failure)
    return 1


def load_json(path: Path, rel: Path, failures: list[str]) -> object | None:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except (OSError, UnicodeError, json.JSONDecodeError) as error:
        failures.append(f"FAIL: {rel.as_posix()} is not UTF-8 JSON ({error})")
        return None


def check_stage_schema(schema: object, failures: list[str]) -> list[str]:
    if not isinstance(schema, dict):
        failures.append("FAIL: stage schema is not a JSON object")
        return []
    try:
        Draft202012Validator.check_schema(schema)
    except SchemaError as error:
        failures.append(f"FAIL: stage schema is not a valid draft 2020-12 schema ({error.message})")
        return []
    if schema.get("$schema") != METASCHEMA:
        failures.append(f"FAIL: $schema is {schema.get('$schema')!r}")
    if schema.get("$id") != SCHEMA_ID:
        failures.append(f"FAIL: $id is {schema.get('$id')!r}")
    if schema.get("title") != SCHEMA_TITLE:
        failures.append(f"FAIL: title is {schema.get('title')!r}")
    check_closed_objects(schema, "", failures)
    leaves = collect_leaves(schema, "", failures)
    check_identity_and_enums(schema, failures)
    return leaves


def check_closed_objects(node: object, path: str, failures: list[str]) -> None:
    if not isinstance(node, dict):
        failures.append(f"FAIL: {path or '<root>'} is not a schema object")
        return
    declared = node.get("type")
    if declared == "object":
        label = path or "<root>"
        if node.get("additionalProperties") is not False:
            failures.append(f"FAIL: {label} does not set additionalProperties to false")
        properties = node.get("properties")
        required = node.get("required")
        if not isinstance(properties, dict) or not properties:
            failures.append(f"FAIL: {label} has no properties")
            return
        if not isinstance(required, list) or required != list(properties):
            failures.append(f"FAIL: {label} required keys do not match its properties in order")
        for key, child in properties.items():
            child_path = f"{path}.{key}" if path else key
            check_closed_objects(child, child_path, failures)
        return
    if declared == "array":
        items = node.get("items")
        if not isinstance(items, dict):
            failures.append(f"FAIL: {path} array has no single item schema")
            return
        check_closed_objects(items, f"{path}[]", failures)
        return
    if declared == "string" or declared == ["string", "null"]:
        return
    failures.append(f"FAIL: {path} has unsupported type {declared!r}")


def collect_leaves(node: object, path: str, failures: list[str]) -> list[str]:
    """Scalar leaves use a dotted path. Array items insert ``[]`` before the next name."""

    if not isinstance(node, dict):
        failures.append(f"FAIL: cannot walk {path or '<root>'}")
        return []
    declared = node.get("type")
    if declared == "object":
        properties = node.get("properties")
        if not isinstance(properties, dict):
            failures.append(f"FAIL: {path or '<root>'} object has no properties")
            return []
        found: list[str] = []
        for key, child in properties.items():
            child_path = f"{path}.{key}" if path else key
            found.extend(collect_leaves(child, child_path, failures))
        return found
    if declared == "array":
        items = node.get("items")
        if not isinstance(items, dict):
            failures.append(f"FAIL: {path} array has no single item schema")
            return []
        if items.get("type") == "object":
            return collect_leaves(items, f"{path}[]", failures)
        return [f"{path}[]"]
    if not path:
        failures.append("FAIL: stage schema root is not an object")
        return []
    return [path]


def check_identity_and_enums(schema: dict, failures: list[str]) -> None:
    version = object_at(schema, ["schemaVersion"])
    if not isinstance(version, dict) or version.get("const") != STAGE_VERSION:
        failures.append("FAIL: schemaVersion const is not moriarty-stage-relation/1")
    outcome = object_at(schema, ["outcome", "kind"])
    if not isinstance(outcome, dict) or outcome.get("enum") != OUTCOME_ENUM:
        found = outcome.get("enum") if isinstance(outcome, dict) else None
        failures.append(f"FAIL: outcome.kind enum is {found!r}")
    judgments = object_at(schema, ["judgments"])
    properties = judgments.get("properties") if isinstance(judgments, dict) else None
    if not isinstance(properties, dict) or list(properties) != JUDGMENT_KEYS:
        failures.append(f"FAIL: judgments properties are {list(properties) if isinstance(properties, dict) else None!r}")
        return
    for key in JUDGMENT_KEYS:
        status = object_at(schema, ["judgments", key, "status"])
        if not isinstance(status, dict) or status.get("enum") != STATUS_ENUM:
            found = status.get("enum") if isinstance(status, dict) else None
            failures.append(f"FAIL: judgments.{key}.status enum is {found!r}")
        ref = object_at(schema, ["judgments", key, "enforcementRef"])
        if not isinstance(ref, dict) or ref.get("type") != ["string", "null"]:
            found = ref.get("type") if isinstance(ref, dict) else None
            failures.append(f"FAIL: judgments.{key}.enforcementRef type is {found!r}")


def object_at(schema: dict, keys: list[str]) -> object:
    node: object = schema
    for key in keys:
        if not isinstance(node, dict):
            return None
        properties = node.get("properties")
        if not isinstance(properties, dict) or key not in properties:
            return None
        node = properties[key]
    return node


def check_embeddings(
    root: Path,
    embeddings: object,
    leaves: list[str],
    failures: list[str],
) -> None:
    if not isinstance(embeddings, dict):
        failures.append("FAIL: embeddings artifact is not a JSON object")
        return
    if list(embeddings) != EMBEDDING_KEYS:
        failures.append(f"FAIL: embeddings keys are {list(embeddings)!r}")
    if embeddings.get("schemaVersion") != EMBEDDINGS_VERSION:
        failures.append(f"FAIL: embeddings schemaVersion is {embeddings.get('schemaVersion')!r}")
    if embeddings.get("sourceProfile") != SOURCE_PROFILE:
        failures.append(f"FAIL: embeddings sourceProfile is {embeddings.get('sourceProfile')!r}")
    if embeddings.get("coreProfile") != CORE_PROFILE:
        failures.append(f"FAIL: embeddings coreProfile is {embeddings.get('coreProfile')!r}")
    rows = embeddings.get("rows")
    if not isinstance(rows, list):
        failures.append("FAIL: embeddings rows is not an array")
        return
    fields: list[str] = []
    for index, row in enumerate(rows):
        field = check_row(root, row, index, failures)
        if field is not None:
            fields.append(field)
    if len(fields) != len(set(fields)):
        duplicates = sorted({field for field in fields if fields.count(field) > 1})
        failures.append(f"FAIL: duplicate embedding rows for {duplicates}")
    if fields != sorted(fields):
        failures.append("FAIL: embeddings rows are not sorted by schemaField")
    leaf_set = set(leaves)
    missing = sorted(leaf_set - set(fields))
    extra = sorted(set(fields) - leaf_set)
    if missing:
        failures.append(f"FAIL: embeddings missing leaf fields {missing}")
    if extra:
        failures.append(f"FAIL: embeddings extra fields {extra}")


def check_row(root: Path, row: object, index: int, failures: list[str]) -> str | None:
    label = f"embeddings row {index}"
    if not isinstance(row, dict):
        failures.append(f"FAIL: {label} is not an object")
        return None
    if list(row) != ROW_KEYS:
        failures.append(f"FAIL: {label} keys are {list(row)!r}")
    field = row.get("schemaField")
    if not isinstance(field, str) or not field:
        failures.append(f"FAIL: {label} schemaField is not a string")
        return None
    label = field
    present = row.get("present")
    note = row.get("note")
    if not isinstance(note, str) or not note.strip():
        failures.append(f"FAIL: {label} note is empty")
    pairs = (
        ("sourceFile", "sourceSymbol"),
        ("coreFile", "coreSymbol"),
    )
    if present is True:
        cited = 0
        for file_key, symbol_key in pairs:
            file_name = row.get(file_key)
            symbol = row.get(symbol_key)
            if file_name is None and symbol is None:
                continue
            cited += 1
            check_symbol(root, label, file_key, file_name, symbol, failures)
        if cited == 0:
            failures.append(f"FAIL: {label} is present without a source or core citation")
        return field
    if present is False:
        for file_key, symbol_key in pairs:
            if row.get(file_key) is not None or row.get(symbol_key) is not None:
                failures.append(f"FAIL: {label} is absent but {file_key} or {symbol_key} is set")
        return field
    failures.append(f"FAIL: {label} present is not a boolean")
    return field


def check_symbol(
    root: Path,
    field: str,
    file_key: str,
    file_name: object,
    symbol: object,
    failures: list[str],
) -> None:
    side = "source" if file_key == "sourceFile" else "core"
    if not isinstance(file_name, str) or not isinstance(symbol, str) or not file_name or not symbol:
        failures.append(f"FAIL: {field} {side} citation is incomplete")
        return
    rel = Path(file_name)
    if rel.is_absolute() or any(part == ".." for part in rel.parts):
        failures.append(f"FAIL: {field} {side} file is not a relative language path")
        return
    try:
        rel.relative_to(LANGUAGE_ROOT)
    except ValueError:
        failures.append(f"FAIL: {field} {side} file is outside experiments/moriarty-language")
        return
    language_root = (root / LANGUAGE_ROOT).resolve()
    target = (root / rel).resolve()
    if not target.is_relative_to(language_root):
        failures.append(f"FAIL: {field} {side} file escapes experiments/moriarty-language")
        return
    if not target.is_file():
        failures.append(f"FAIL: {field} {side} file does not exist: {file_name}")
        return
    try:
        text = target.read_text(encoding="utf-8")
    except (OSError, UnicodeError) as error:
        failures.append(f"FAIL: {field} {side} file is not readable UTF-8 ({error})")
        return
    if symbol not in text:
        failures.append(f"FAIL: {field} {side} symbol {symbol} does not occur in {file_name}")


def check_judgments(judgments: object, leaves: list[str], failures: list[str]) -> None:
    if not isinstance(judgments, dict):
        failures.append("FAIL: judgments artifact is not a JSON object")
        return
    if list(judgments) != JUDGMENT_FILE_KEYS:
        failures.append(f"FAIL: judgments keys are {list(judgments)!r}")
    if judgments.get("schemaVersion") != JUDGMENTS_VERSION:
        failures.append(f"FAIL: judgments schemaVersion is {judgments.get('schemaVersion')!r}")
    rows = judgments.get("judgments")
    if not isinstance(rows, list):
        failures.append("FAIL: judgments array is missing")
        return
    keys = [row.get("key") if isinstance(row, dict) else None for row in rows]
    if keys != JUDGMENT_KEYS:
        failures.append(f"FAIL: judgment keys are {keys!r}")
    leaf_set = set(leaves)
    for index, row in enumerate(rows):
        if not isinstance(row, dict):
            failures.append(f"FAIL: judgment row {index} is not an object")
            continue
        if list(row) != JUDGMENT_ROW_KEYS:
            failures.append(f"FAIL: judgment row {index} keys are {list(row)!r}")
        key = row.get("key")
        phrase = DESIGN_DOC_JUDGMENT.get(key) if isinstance(key, str) else None
        if phrase is None:
            continue
        if row.get("designDocJudgment") != phrase:
            failures.append(
                f"FAIL: judgments.{key} designDocJudgment is {row.get('designDocJudgment')!r}"
            )
        definition = row.get("definition")
        if not isinstance(definition, str) or not definition.strip():
            failures.append(f"FAIL: judgments.{key} definition is empty")
        fields = row.get("schemaFields")
        if not isinstance(fields, list) or not fields:
            failures.append(f"FAIL: judgments.{key} schemaFields is empty")
            continue
        seen: set[str] = set()
        for field in fields:
            if not isinstance(field, str) or field not in leaf_set:
                failures.append(f"FAIL: judgments.{key} schemaFields entry {field!r} is not a leaf")
                continue
            if field in seen:
                failures.append(f"FAIL: judgments.{key} repeats leaf {field}")
            seen.add(field)


if __name__ == "__main__":
    raise SystemExit(main())
