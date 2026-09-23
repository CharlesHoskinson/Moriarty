#!/usr/bin/env python3
"""Check the U0 stage-relation schema, source/Core embeddings, and judgments."""

from __future__ import annotations

import argparse
import json
import re
from pathlib import Path

from jsonschema import Draft202012Validator
from jsonschema.exceptions import SchemaError


MORIARTY_ROOT = Path(__file__).resolve().parents[1]
SCHEMA_REL = Path(
    "openspec/changes/consolidated-language-kernel/schemas/stage-relation.schema.json"
)
EMBEDDINGS_REL = Path("deliverables/u0-semantic-contract-2026-09-23/source-core-embeddings.json")
JUDGMENTS_REL = Path("deliverables/u0-semantic-contract-2026-09-23/judgments.json")
DESIGN_DOC_REL = Path("docs/MORIARTY-CONSOLIDATED-DESIGN.md")
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
AMOUNT_PATTERN = "^-?[0-9]+$"
DESIGN_DOC_JUDGMENT = {
    "stage": "contract properties",
    "intent": "intent refinement",
    "effect": "valid state/effect transition",
    "authority": "valid state/effect transition",
    "history": "compliant history",
    "failure": "rejection/partial failure transition",
}
CANONICAL_HEADING = "## Canonical stage statement"
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
IDENTITY_FIELDS = {"programIdentity.coreRef", "programIdentity.sourceRef"}

EFFECT_LINE = [
    "object",
    {"asset": "string", "account": "string", "amount": "amount"},
]
LIABILITY_LINE = [
    "object",
    {
        "liabilityId": "string",
        "debtor": "string",
        "creditor": "string",
        "asset": "string",
        "amount": "amount",
    },
]
JUDGMENT_OBJECT = {"status": "status", "enforcementRef": "nullable"}
# Property names and nesting required by the U0 specification. Leaf discovery
# still walks the schema file. This tree does not.
EXPECTED_STAGE: dict[str, object] = {
    "schemaVersion": "version",
    "profiles": {
        "semanticProfile": "string",
        "numericProfile": "string",
    },
    "programIdentity": {
        "sourceRef": "string",
        "coreRef": "string",
        "programId": "string",
        "entryPoint": "string",
    },
    "circuitIdentity": {
        "compilerPin": "string",
        "zkirVersion": "string",
        "circuitId": "string",
        "verifierKeyId": "string",
    },
    "domain": {
        "chainId": "string",
        "domainId": "string",
        "stateFrameRef": "string",
    },
    "signedIntent": {
        "intentId": "string",
        "signer": "string",
        "consentPolicy": "string",
        "delegationPolicy": "string",
        "assetIdentities": ["strings"],
        "recipients": ["strings"],
        "grossDebitCap": "amount",
        "feeCap": "amount",
        "minNetOutcome": "amount",
        "validity": "string",
        "replayPolicy": "string",
        "recoveryPolicy": "string",
    },
    "lifecycleIds": {
        "lifecycleId": "string",
        "stageId": "string",
        "logicalRequestId": "string",
    },
    "predecessorCommitments": [
        "object",
        {"predecessorId": "string", "commitment": "string"},
    ],
    "obligationCommitments": [
        "object",
        {"obligationId": "string", "commitment": "string"},
    ],
    "observations": [
        "object",
        {
            "kind": "string",
            "issuer": "string",
            "domain": "string",
            "time": "string",
            "finality": "string",
        },
    ],
    "effects": {
        "gross": EFFECT_LINE,
        "fees": EFFECT_LINE,
        "net": EFFECT_LINE,
        "supplyChanges": EFFECT_LINE,
    },
    "liabilities": {
        "opening": LIABILITY_LINE,
        "closing": LIABILITY_LINE,
    },
    "authority": {
        "consumed": "string",
        "remaining": "string",
        "replayState": "string",
    },
    "resources": {
        "resourceCertificate": "string",
        "cumulativeReservations": "string",
    },
    "disclosures": [
        "object",
        {"party": "string", "fields": ["strings"]},
    ],
    "failurePolicy": {
        "phasePolicy": "string",
        "retainedEffects": "string",
        "retainedFees": "string",
    },
    "outcome": {
        "kind": "outcome",
        "continuations": ["strings"],
    },
    "judgments": {key: JUDGMENT_OBJECT for key in JUDGMENT_KEYS},
}


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=MORIARTY_ROOT)
    args = parser.parse_args(argv)
    return check_root(args.root.resolve())


def check_root(root: Path) -> int:
    if not root.is_dir():
        print("blocked: root is not a directory")
        return 2
    blocked = missing_inputs(root)
    if blocked:
        for line in blocked:
            print(line)
        return 2
    try:
        design_text = (root / DESIGN_DOC_REL).read_text(encoding="utf-8")
    except (OSError, UnicodeError) as error:
        print(f"blocked: {DESIGN_DOC_REL.as_posix()} is not readable UTF-8 ({error})")
        return 2

    failures: list[str] = []
    schema = load_json(root / SCHEMA_REL, SCHEMA_REL, failures)
    embeddings = load_json(root / EMBEDDINGS_REL, EMBEDDINGS_REL, failures)
    judgments = load_json(root / JUDGMENTS_REL, JUDGMENTS_REL, failures)
    check_design_doc(design_text, failures)
    if schema is None or embeddings is None or judgments is None:
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


def missing_inputs(root: Path) -> list[str]:
    blocked: list[str] = []
    for rel in (SCHEMA_REL, EMBEDDINGS_REL, JUDGMENTS_REL, DESIGN_DOC_REL):
        if not (root / rel).is_file():
            blocked.append(f"blocked: missing {rel.as_posix()}")
    if not (root / LANGUAGE_ROOT).is_dir():
        blocked.append("blocked: missing experiments/moriarty-language")
    return blocked


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
    check_specified_shape(schema, failures)
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


def check_specified_shape(schema: dict, failures: list[str]) -> None:
    expect_object(schema, "", EXPECTED_STAGE, failures)


def expect_node(node: object, path: str, expected: object, failures: list[str]) -> None:
    if isinstance(expected, dict):
        expect_object(node, path, expected, failures)
        return
    if isinstance(expected, list):
        expect_array(node, path, expected, failures)
        return
    if isinstance(expected, str):
        expect_scalar(node, path, expected, failures)
        return
    failures.append(f"FAIL: {path or '<root>'} has no expected shape")


def expect_object(node: object, path: str, fields: dict[str, object], failures: list[str]) -> None:
    label = path or "<root>"
    if not isinstance(node, dict) or node.get("type") != "object":
        found = node.get("type") if isinstance(node, dict) else None
        failures.append(f"FAIL: {label} type is {found!r}")
        return
    properties = node.get("properties")
    if not isinstance(properties, dict):
        failures.append(f"FAIL: {label} has no properties")
        return
    expected_names = list(fields)
    actual_names = list(properties)
    for name in expected_names:
        if name not in properties:
            child = f"{path}.{name}" if path else name
            failures.append(f"FAIL: missing required property {child}")
    for name in actual_names:
        if name not in fields:
            child = f"{path}.{name}" if path else name
            failures.append(f"FAIL: unexpected property {child}")
    if actual_names != expected_names:
        failures.append(f"FAIL: {label} properties are {actual_names!r}")
    for name, child_expected in fields.items():
        if name not in properties:
            continue
        child_path = f"{path}.{name}" if path else name
        expect_node(properties[name], child_path, child_expected, failures)


def expect_array(node: object, path: str, expected: list[object], failures: list[str]) -> None:
    label = path or "<root>"
    if not isinstance(node, dict) or node.get("type") != "array":
        found = node.get("type") if isinstance(node, dict) else None
        failures.append(f"FAIL: {label} type is {found!r}")
        return
    items = node.get("items")
    if expected == ["strings"]:
        if not isinstance(items, dict) or items.get("type") != "string":
            failures.append(f"FAIL: {label} items are not strings")
        return
    if len(expected) == 2 and expected[0] == "object" and isinstance(expected[1], dict):
        if not isinstance(items, dict):
            failures.append(f"FAIL: {label} array has no single item schema")
            return
        expect_object(items, f"{path}[]", expected[1], failures)
        return
    failures.append(f"FAIL: {label} array shape is unsupported")


def expect_scalar(node: object, path: str, kind: str, failures: list[str]) -> None:
    label = path or "<root>"
    if not isinstance(node, dict):
        failures.append(f"FAIL: {label} is not a schema object")
        return
    declared = node.get("type")
    if kind == "nullable":
        if declared != ["string", "null"]:
            failures.append(f"FAIL: {label} type is {declared!r}")
        return
    if declared != "string":
        failures.append(f"FAIL: {label} type is {declared!r}")
        return
    if kind == "version":
        if node.get("const") != STAGE_VERSION:
            failures.append("FAIL: schemaVersion const is not moriarty-stage-relation/1")
        return
    if kind == "amount":
        if node.get("pattern") != AMOUNT_PATTERN:
            failures.append(f"FAIL: {label} pattern is {node.get('pattern')!r}")
        return
    if kind == "status":
        if node.get("enum") != STATUS_ENUM:
            failures.append(f"FAIL: {label} enum is {node.get('enum')!r}")
        return
    if kind == "outcome":
        if node.get("enum") != OUTCOME_ENUM:
            failures.append(f"FAIL: {label} enum is {node.get('enum')!r}")
        return
    if kind != "string":
        failures.append(f"FAIL: {label} has unknown shape {kind}")


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


def profile_version_assignment(symbol: str, text: str) -> str | None:
    """Return the profile id when ``symbol`` is assigned a ``moriarty-…/N`` string."""

    match = re.search(
        rf"(?<![A-Za-z0-9_]){re.escape(symbol)}(?![A-Za-z0-9_])"
        r"""\s*=\s*(['"])(moriarty-[A-Za-z0-9-]+/\d+)\1""",
        text,
    )
    if match is None:
        return None
    return match.group(2)


def symbol_pattern(symbol: str) -> re.Pattern[str] | None:
    """Accept an identifier, or a K configuration cell such as ``<financialPre>``."""

    if re.fullmatch(r"[A-Za-z_][A-Za-z0-9_]*", symbol):
        return re.compile(rf"(?<![A-Za-z0-9_]){re.escape(symbol)}(?![A-Za-z0-9_])")
    if re.fullmatch(r"<(?:[A-Za-z_][A-Za-z0-9_]*)>", symbol):
        return re.compile(rf"(?<![A-Za-z0-9_<]){re.escape(symbol)}(?![A-Za-z0-9_>])")
    return None


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
    pattern = symbol_pattern(symbol)
    if pattern is None:
        failures.append(f"FAIL: {field} {side} symbol {symbol!r} is not an identifier or K cell")
        return
    if pattern.search(text) is None:
        failures.append(f"FAIL: {field} {side} symbol {symbol} does not occur in {file_name}")
        return
    if field in IDENTITY_FIELDS:
        profile = profile_version_assignment(symbol, text)
        if profile is not None:
            failures.append(
                f"FAIL: {field} {side} symbol {symbol} is a profile version constant, not a program identity"
            )


def canonical_stage_section(text: str) -> str | None:
    match = re.search(rf"(?m)^{re.escape(CANONICAL_HEADING)}[ \t]*\n", text)
    if match is None:
        return None
    rest = text[match.end():]
    next_heading = re.search(r"(?m)^#{1,6} ", rest)
    if next_heading is None:
        return rest
    return rest[: next_heading.start()]


def check_design_doc(text: str, failures: list[str]) -> None:
    section = canonical_stage_section(text)
    if section is None:
        failures.append("FAIL: Canonical stage statement section is missing")
        return
    seen: list[str] = []
    for key in JUDGMENT_KEYS:
        if key == "failure":
            continue
        phrase = DESIGN_DOC_JUDGMENT[key]
        if phrase not in seen:
            seen.append(phrase)
    for phrase in seen:
        if phrase not in section:
            failures.append(f"FAIL: Canonical stage statement lacks {phrase!r}")
    if re.search(r"rejection/partial failure", section, re.IGNORECASE) is None:
        failures.append("FAIL: Canonical stage statement lacks 'Rejection/partial failure'")


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
