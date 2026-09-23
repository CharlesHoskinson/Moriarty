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
PRESENCE_RULE = (
    "Presence rule: a row is present only when every cited symbol is a declared "
    "field of the record that fills that schema slot, or an explicit partial proxy of that record."
)
PARTIAL_PROXY = "Classification: partial-proxy."
EXACT_FIELD = "Classification: exact-field."
IDENTITY_PREFIXES = ("programIdentity.", "profiles.", "circuitIdentity.", "lifecycleIds.")
SEARCH_RELATIVE = (
    Path("spec/successor"),
    Path("src/successor"),
    Path("formal/k"),
)
PROFILE_CONSTANT_RE = re.compile(
    r"(?<![A-Za-z0-9_])([A-Za-z_][A-Za-z0-9_]*)\s*=\s*(['\"])(moriarty-[A-Za-z0-9-]+/\d+)\2"
)
QUALIFIED_SYMBOL = re.compile(r"([A-Za-z_][A-Za-z0-9_]*)\.([A-Za-z_][A-Za-z0-9_]*)$")
CELL_SYMBOL = re.compile(r"<([A-Za-z_][A-Za-z0-9_]*)>$")
BARE_SYMBOL = re.compile(r"[A-Za-z_][A-Za-z0-9_]*$")
MENTIONED_QUALIFIED = re.compile(r"\b([A-Z][A-Za-z0-9_]*\.[A-Za-z_][A-Za-z0-9_]*)\b")
EVIDENCE_RE = re.compile(r"\|\| evidence declares=(\S+) missing=(\S+)$")
ROOT_SCHEMA_KEYS = [
    "$schema",
    "$id",
    "title",
    "type",
    "additionalProperties",
    "required",
    "properties",
]
# Declarations that fill a slot whose leaf name differs from the field name.
# A present citation must be one of these, or a declared field whose name is the leaf.
# An absent row is a false absence when any of these declarations still exists.
SLOT_FILLERS: dict[str, frozenset[tuple[str, str]]] = {
    "authority.consumed": frozenset({
        (
            "experiments/moriarty-language/spec/successor/financial-agreement-source-v5-grammar.ebnf",
            "financialRead.allowance_spent",
        ),
        (
            "experiments/moriarty-language/src/successor/financial-lifecycle.ts",
            "Allowance.spent",
        ),
    }),
    "authority.remaining": frozenset({
        (
            "experiments/moriarty-language/spec/successor/financial-agreement-source-v5-grammar.ebnf",
            "financialRead.allowance_remaining",
        ),
        (
            "experiments/moriarty-language/src/successor/financial-lifecycle.ts",
            "Allowance.remaining",
        ),
    }),
    "authority.replayState": frozenset({
        (
            "experiments/moriarty-language/src/successor/financial-lifecycle.ts",
            "LifecycleState.usedTransferIds",
        ),
    }),
    "profiles.semanticProfile": frozenset({
        (
            "experiments/moriarty-language/src/successor/frontend.ts",
            "ProfileDecl.value",
        ),
    }),
}
TS_SUFFIXES = {".ts", ".tsx", ".js", ".mjs", ".cjs"}
DECLARATION_SUFFIXES = TS_SUFFIXES | {".ebnf", ".k", ".py"}
PROFILE_SCAN_SUFFIXES = DECLARATION_SUFFIXES | {".md", ".json"}

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
        text = path.read_text(encoding="utf-8")
        parsed = json.loads(text)
    except (OSError, UnicodeError, json.JSONDecodeError) as error:
        failures.append(f"FAIL: {rel.as_posix()} is not UTF-8 JSON ({error})")
        return None
    canonical = json.dumps(parsed, indent=2, ensure_ascii=False) + "\n"
    if text != canonical:
        failures.append(
            f"FAIL: {rel.as_posix()} is not canonical UTF-8 JSON "
            "(2-space indent, trailing newline)"
        )
    return parsed


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
    check_schema_keywords(schema, "", failures, root=True)
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


def allowed_schema_keywords(node: dict, *, root: bool) -> list[str]:
    if root:
        return list(ROOT_SCHEMA_KEYS)
    declared = node.get("type")
    if declared == "object":
        return ["type", "additionalProperties", "required", "properties"]
    if declared == "array":
        return ["type", "items"]
    if declared == "string":
        extras = [key for key in ("const", "pattern", "enum") if key in node]
        return ["type", *extras]
    if declared == ["string", "null"]:
        return ["type"]
    return ["type"]


def check_schema_keywords(node: object, path: str, failures: list[str], *, root: bool = False) -> None:
    """Reject keywords that can change the stage relation, at every schema node."""

    if not isinstance(node, dict):
        return
    allowed = allowed_schema_keywords(node, root=root)
    allowed_set = set(allowed)
    keys = list(node)
    label = path or "<root>"
    for key in keys:
        if key not in allowed_set:
            failures.append(f"FAIL: unapproved schema keyword {key!r} at {label}")
    ordered = [key for key in keys if key in allowed_set]
    expect = [key for key in allowed if key in node]
    if ordered != expect:
        failures.append(f"FAIL: schema keywords at {label} are {keys!r}")
    properties = node.get("properties")
    if isinstance(properties, dict):
        for name, child in properties.items():
            child_path = f"{path}.{name}" if path else name
            check_schema_keywords(child, child_path, failures)
    items = node.get("items")
    if isinstance(items, dict):
        check_schema_keywords(items, f"{path}[]", failures)


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
    scan = scan_language(root)
    fields: list[str] = []
    for index, row in enumerate(rows):
        field = check_row(root, scan, row, index, failures)
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


def check_row(
    root: Path,
    scan: "LanguageScan",
    row: object,
    index: int,
    failures: list[str],
) -> str | None:
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
        note = ""
    if not note.startswith(PRESENCE_RULE):
        failures.append(f"FAIL: {label} note does not state the presence rule")
    pairs = (
        ("sourceFile", "sourceSymbol"),
        ("coreFile", "coreSymbol"),
    )
    if present is True:
        if PARTIAL_PROXY not in note and EXACT_FIELD not in note:
            failures.append(
                f"FAIL: {label} note does not classify the citation as an exact field or a partial proxy"
            )
        cited = 0
        for file_key, symbol_key in pairs:
            file_name = row.get(file_key)
            symbol = row.get(symbol_key)
            if file_name is None and symbol is None:
                continue
            cited += 1
            check_present_symbol(root, scan, label, file_key, file_name, symbol, note, failures)
        if cited == 0:
            failures.append(f"FAIL: {label} is present without a source or core citation")
        return field
    if present is False:
        for file_key, symbol_key in pairs:
            if row.get(file_key) is not None or row.get(symbol_key) is not None:
                failures.append(f"FAIL: {label} is absent but {file_key} or {symbol_key} is set")
        check_absence(root, scan, label, note, failures)
        return field
    failures.append(f"FAIL: {label} present is not a boolean")
    return field


def check_absence(
    root: Path,
    scan: "LanguageScan",
    field: str,
    note: str,
    failures: list[str],
) -> None:
    for file_name, symbol in sorted(SLOT_FILLERS.get(field, ())):
        if symbol_declared(root, scan, file_name, symbol):
            failures.append(f"FAIL: false absence: {field} is realised by {symbol} in {file_name}")
    match = EVIDENCE_RE.search(note)
    if match is None:
        failures.append(f"FAIL: {field} absent row has no verifiable absence evidence")
        return
    declares = parse_declares(match.group(1))
    missing = parse_missing(match.group(2))
    if declares is None or missing is None:
        failures.append(f"FAIL: {field} absence evidence is malformed")
        return
    witnessed: set[str] = set()
    for file_name, symbol in declares:
        if not search_path(file_name):
            failures.append(f"FAIL: {field} absence witness {file_name} is outside the searched trees")
            continue
        if not symbol_declared(root, scan, file_name, symbol):
            failures.append(
                f"FAIL: {field} absence witness {symbol} does not occur as a declaration in {file_name}"
            )
            continue
        witnessed.add(symbol)
    prose = note[: match.start()]
    for symbol in MENTIONED_QUALIFIED.findall(prose):
        if symbol not in witnessed:
            failures.append(f"FAIL: {field} absence note names {symbol} but does not witness it")
    leaf = leaf_name(field)
    accounted = any(symbol_leaf(symbol) == leaf for symbol in witnessed)
    if leaf in scan.names:
        if leaf in missing:
            failures.append(f"FAIL: false absence: {field} claims {leaf} is missing but it is declared")
        if not accounted:
            failures.append(f"FAIL: {field} absence does not account for declared {leaf}")
    elif leaf not in missing:
        failures.append(f"FAIL: {field} absence does not record {leaf} as missing")
    for name in missing:
        if name in scan.names:
            failures.append(f"FAIL: false absence: {field} claims {name} is missing but it is declared")


def parse_declares(text: str) -> list[tuple[str, str]] | None:
    if text == "-":
        return []
    found: list[tuple[str, str]] = []
    for item in text.split(","):
        file_name, separator, symbol = item.partition("#")
        if not separator or not file_name or not symbol:
            return None
        found.append((file_name, symbol))
    return found


def parse_missing(text: str) -> set[str] | None:
    if text == "-":
        return set()
    names = text.split(",")
    if any(not BARE_SYMBOL.fullmatch(name) for name in names):
        return None
    return set(names)


def check_present_symbol(
    root: Path,
    scan: "LanguageScan",
    field: str,
    file_key: str,
    file_name: object,
    symbol: object,
    note: str,
    failures: list[str],
) -> None:
    side = "source" if file_key == "sourceFile" else "core"
    located = locate_symbol(root, field, side, file_name, symbol, failures)
    if located is None:
        return
    file_text, symbol_text = located
    if isinstance(symbol, str) and symbol not in note:
        failures.append(f"FAIL: {field} note does not name cited {side} symbol {symbol}")
    if is_identity_field(field) and any(name in scan.constants for name in symbol_identifiers(symbol_text)):
        failures.append(
            f"FAIL: {field} {side} symbol {symbol_text} "
            "is a profile version constant, not a program identity"
        )
        return
    if not declaration_in_text(file_text, Path(str(file_name)).suffix, symbol_text):
        failures.append(
            f"FAIL: {field} {side} symbol {symbol_text} does not occur as a declaration in {file_name}"
        )
        return
    if not citation_fills_slot(field, str(file_name), symbol_text):
        failures.append(
            f"FAIL: {field} {side} symbol {symbol_text} "
            "is not a field of the record that fills that slot"
        )


def locate_symbol(
    root: Path,
    field: str,
    side: str,
    file_name: object,
    symbol: object,
    failures: list[str],
) -> tuple[str, str] | None:
    if not isinstance(file_name, str) or not isinstance(symbol, str) or not file_name or not symbol:
        failures.append(f"FAIL: {field} {side} citation is incomplete")
        return None
    if symbol_kind(symbol) is None:
        failures.append(f"FAIL: {field} {side} symbol {symbol!r} is not an identifier or K cell")
        return None
    rel = Path(file_name)
    if rel.is_absolute() or any(part == ".." for part in rel.parts):
        failures.append(f"FAIL: {field} {side} file is not a relative language path")
        return None
    try:
        rel.relative_to(LANGUAGE_ROOT)
    except ValueError:
        failures.append(f"FAIL: {field} {side} file is outside experiments/moriarty-language")
        return None
    language_root = (root / LANGUAGE_ROOT).resolve()
    target = (root / rel).resolve()
    if not target.is_relative_to(language_root):
        failures.append(f"FAIL: {field} {side} file escapes experiments/moriarty-language")
        return None
    if not target.is_file():
        failures.append(f"FAIL: {field} {side} file does not exist: {file_name}")
        return None
    try:
        text = target.read_text(encoding="utf-8")
    except (OSError, UnicodeError) as error:
        failures.append(f"FAIL: {field} {side} file is not readable UTF-8 ({error})")
        return None
    return text, symbol


def citation_fills_slot(field: str, file_name: str, symbol: str) -> bool:
    """Known slots accept only their filling declarations. Other slots accept a field of the leaf name."""

    fillers = SLOT_FILLERS.get(field)
    if fillers is not None:
        return (file_name, symbol) in fillers
    return symbol_leaf(symbol) == leaf_name(field)


def is_identity_field(field: str) -> bool:
    return field.startswith(IDENTITY_PREFIXES)


def leaf_name(field: str) -> str:
    return field.split(".")[-1].replace("[]", "")


def symbol_kind(symbol: str) -> str | None:
    if QUALIFIED_SYMBOL.fullmatch(symbol):
        return "qualified"
    if CELL_SYMBOL.fullmatch(symbol):
        return "cell"
    if BARE_SYMBOL.fullmatch(symbol):
        return "bare"
    return None


def symbol_leaf(symbol: str) -> str | None:
    qualified = QUALIFIED_SYMBOL.fullmatch(symbol)
    if qualified:
        return qualified.group(2)
    cell = CELL_SYMBOL.fullmatch(symbol)
    if cell:
        return cell.group(1)
    if BARE_SYMBOL.fullmatch(symbol):
        return symbol
    return None


def symbol_identifiers(symbol: str) -> tuple[str, ...]:
    qualified = QUALIFIED_SYMBOL.fullmatch(symbol)
    if qualified:
        return qualified.group(1), qualified.group(2)
    leaf = symbol_leaf(symbol)
    if leaf is None:
        return ()
    return (leaf,)


def search_path(file_name: str) -> bool:
    rel = Path(file_name)
    try:
        inside = rel.relative_to(LANGUAGE_ROOT)
    except ValueError:
        return False
    return any(inside == root or inside.is_relative_to(root) for root in SEARCH_RELATIVE)


class LanguageScan:
    def __init__(self, names: set[str], constants: set[str], files: dict[str, str]) -> None:
        self.names = names
        self.constants = constants
        self.files = files


def scan_language(root: Path) -> LanguageScan:
    language = (root / LANGUAGE_ROOT).resolve()
    names: set[str] = set()
    files: dict[str, str] = {}
    if language.is_dir():
        for path in sorted(language.rglob("*")):
            if not path.is_file():
                continue
            suffix = path.suffix
            if suffix not in DECLARATION_SUFFIXES:
                continue
            try:
                inside = path.relative_to(language)
            except ValueError:
                continue
            if not any(inside == rel or inside.is_relative_to(rel) for rel in SEARCH_RELATIVE):
                continue
            try:
                text = path.read_text(encoding="utf-8")
            except (OSError, UnicodeError):
                continue
            code = strip_comments(text, suffix)
            repo_rel = (LANGUAGE_ROOT / inside).as_posix()
            files[repo_rel] = code
            names.update(declared_names(code, suffix))
    return LanguageScan(names, profile_constant_names(root), files)


def profile_constant_names(root: Path) -> set[str]:
    language = (root / LANGUAGE_ROOT).resolve()
    names: set[str] = set()
    if not language.is_dir():
        return names
    for path in language.rglob("*"):
        if not path.is_file() or path.suffix not in PROFILE_SCAN_SUFFIXES:
            continue
        try:
            text = path.read_text(encoding="utf-8")
        except (OSError, UnicodeError):
            continue
        for match in PROFILE_CONSTANT_RE.finditer(text):
            names.add(match.group(1))
    return names


def symbol_declared(root: Path, scan: LanguageScan, file_name: str, symbol: str) -> bool:
    code = scan.files.get(file_name)
    if code is None:
        target = root / file_name
        if not target.is_file():
            return False
        try:
            code = strip_comments(target.read_text(encoding="utf-8"), Path(file_name).suffix)
        except (OSError, UnicodeError):
            return False
    return declaration_in_text(code, Path(file_name).suffix, symbol)


def declared_names(code: str, suffix: str) -> set[str]:
    names: set[str] = set()
    if suffix in TS_SUFFIXES:
        names.update(BARE_DECL_RE.findall(code))
        for body in ts_bodies(code):
            names.update(member_fields(body))
    elif suffix == ".ebnf":
        names.update(match.group(1) for match in EBNF_RULE_RE.finditer(code))
        names.update(EBNF_TERMINAL_RE.findall(code))
    elif suffix == ".k":
        names.update(CELL_RE.findall(code))
        names.update(k_constructors(code))
    elif suffix == ".py":
        for match in PY_DEF_RE.finditer(code):
            names.add(next(group for group in match.groups() if group))
        names.update(PY_CONST_RE.findall(code))
    return names


def declaration_in_text(code: str, suffix: str, symbol: str) -> bool:
    stripped = strip_comments(code, suffix)
    kind = symbol_kind(symbol)
    if kind == "cell":
        return suffix == ".k" and re.search(
            rf"(?<![A-Za-z0-9_<]){re.escape(symbol)}(?![A-Za-z0-9_>])",
            stripped,
        ) is not None
    if kind == "qualified":
        match = QUALIFIED_SYMBOL.fullmatch(symbol)
        if match is None:
            return False
        owner, member = match.group(1), match.group(2)
        if suffix == ".ebnf":
            body = ebnf_body(stripped, owner)
            return body is not None and f'"{member}"' in body
        if suffix in TS_SUFFIXES:
            body = ts_owner_body(stripped, owner)
            return body is not None and field_in_body(body, member)
        return False
    if kind == "bare":
        if suffix in TS_SUFFIXES:
            return re.search(
                rf"(?m)^(?:export\s+)?(?:declare\s+)?(?:async\s+)?(?:function|const|class|interface|enum|type)\s+{re.escape(symbol)}\b",
                stripped,
            ) is not None
        if suffix == ".ebnf":
            return ebnf_body(stripped, symbol) is not None or f'"{symbol}"' in stripped
        if suffix == ".k":
            return f"<{symbol}>" in stripped or symbol in k_constructors(stripped)
        if suffix == ".py":
            return re.search(rf"(?m)^(?:async\s+)?def\s+{re.escape(symbol)}\b", stripped) is not None or (
                symbol.isupper() and re.search(rf"(?m)^{re.escape(symbol)}\s*=", stripped) is not None
            )
    return False


def field_in_body(body: str, member: str) -> bool:
    return member in member_fields(body)


def member_fields(body: str) -> set[str]:
    """Property names at member level. Nested method bodies and parameter lists do not count."""

    names: set[str] = set()
    index = 0
    length = len(body)
    depth = 0
    paren = 0
    quote: str | None = None
    while index < length:
        char = body[index]
        if quote is not None:
            if char == "\\":
                index += 2
                continue
            if char == quote:
                quote = None
            index += 1
            continue
        if char in {"'", '"', "`"}:
            quote = char
            index += 1
            continue
        if char == "{":
            depth += 1
        elif char == "}":
            depth = max(0, depth - 1)
        elif char == "(":
            paren += 1
        elif char == ")":
            paren = max(0, paren - 1)
        elif depth == 0 and paren == 0 and (char.isalpha() or char == "_"):
            end = index + 1
            while end < length and (body[end].isalnum() or body[end] == "_"):
                end += 1
            cursor = end
            while cursor < length and body[cursor] in " \t\r\n":
                cursor += 1
            if cursor < length and body[cursor] == "?":
                cursor += 1
                while cursor < length and body[cursor] in " \t\r\n":
                    cursor += 1
            if cursor < length and body[cursor] == ":":
                names.add(body[index:end])
            index = end
            continue
        index += 1
    return names


BARE_DECL_RE = re.compile(
    r"(?m)^(?:export\s+)?(?:declare\s+)?(?:async\s+)?(?:function|const|class|interface|enum|type)\s+([A-Za-z_][A-Za-z0-9_]*)\b"
)
EBNF_RULE_RE = re.compile(r"(?m)^([A-Za-z_][A-Za-z0-9_]*)\s*=")
EBNF_TERMINAL_RE = re.compile(r'"([A-Za-z_][A-Za-z0-9_]*)"')
CELL_RE = re.compile(r"<([A-Za-z_][A-Za-z0-9_]*)>")
PY_DEF_RE = re.compile(r"(?m)^(?:async\s+)?def\s+([A-Za-z_][A-Za-z0-9_]*)\b|^class\s+([A-Za-z_][A-Za-z0-9_]*)\b")
PY_CONST_RE = re.compile(r"(?m)^([A-Z][A-Z0-9_]*)\s*=")


def ts_bodies(code: str) -> list[str]:
    bodies: list[str] = []
    for match in re.finditer(
        r"(?m)^(?:export\s+)?(?:declare\s+)?(?:abstract\s+)?(?:interface|class|enum)\s+[A-Za-z_][A-Za-z0-9_]*\b",
        code,
    ):
        body = brace_after(code, match.end())
        if body is not None:
            bodies.append(body)
    for match in re.finditer(r"(?m)^(?:export\s+)?type\s+[A-Za-z_][A-Za-z0-9_]*\b", code):
        body = type_alias_body(code, match.end())
        if body is not None:
            bodies.append(body)
    return bodies


def ts_owner_body(code: str, owner: str) -> str | None:
    match = re.search(
        rf"(?m)^(?:export\s+)?(?:declare\s+)?(?:abstract\s+)?(?:interface|class|enum)\s+{re.escape(owner)}\b",
        code,
    )
    if match:
        return brace_after(code, match.end())
    match = re.search(rf"(?m)^(?:export\s+)?type\s+{re.escape(owner)}\b", code)
    if match is None:
        return None
    return type_alias_body(code, match.end())


def type_alias_body(code: str, start: int) -> str | None:
    window = code[start : start + 4000]
    eq = window.find("=")
    if eq == -1:
        return None
    after = window[eq + 1 :].lstrip()
    if not after.startswith("{"):
        return None
    offset = start + eq + 1 + (len(window[eq + 1 :]) - len(after))
    return brace_body(code, offset)


def brace_after(code: str, start: int) -> str | None:
    brace = code.find("{", start)
    if brace == -1 or brace - start > 400:
        return None
    return brace_body(code, brace)


def brace_body(code: str, open_index: int) -> str | None:
    if open_index >= len(code) or code[open_index] != "{":
        return None
    depth = 0
    index = open_index
    quote: str | None = None
    while index < len(code):
        char = code[index]
        if quote is not None:
            if char == "\\":
                index += 2
                continue
            if char == quote:
                quote = None
            index += 1
            continue
        if char in {"'", '"', "`"}:
            quote = char
            index += 1
            continue
        if char == "{":
            depth += 1
        elif char == "}":
            depth -= 1
            if depth == 0:
                return code[open_index + 1 : index]
        index += 1
    return None


def ebnf_body(code: str, owner: str) -> str | None:
    match = re.search(rf"(?m)^{re.escape(owner)}\s*=", code)
    if match is None:
        return None
    index = match.end()
    in_string = False
    in_special = False
    while index < len(code):
        char = code[index]
        if in_string:
            if char == '"':
                in_string = False
            index += 1
            continue
        if in_special:
            if char == "?":
                in_special = False
            index += 1
            continue
        if char == '"':
            in_string = True
        elif char == "?":
            in_special = True
        elif char == ";":
            return code[match.end() : index]
        index += 1
    return None


def k_constructors(code: str) -> set[str]:
    names: set[str] = set()
    for match in re.finditer(
        r"\bsyntax\s+[A-Za-z_][A-Za-z0-9_]*\s*::=\s*(.*?)(?=\n\s*syntax\s|\n\s*configuration\b|\n\s*endmodule\b|\n\s*rule\b|\Z)",
        code,
        re.S,
    ):
        names.update(item.group(1) for item in re.finditer(r"\b([A-Za-z_][A-Za-z0-9_]*)\s*\(", match.group(1)))
    return names


def strip_comments(text: str, suffix: str) -> str:
    if suffix == ".py":
        return strip_hash_comments(text)
    if suffix == ".ebnf":
        return strip_ebnf_comments(text)
    return strip_c_comments(text)


def strip_c_comments(text: str) -> str:
    out: list[str] = []
    index = 0
    length = len(text)
    while index < length:
        char = text[index]
        if char in {"'", '"', "`"}:
            end = skip_quote(text, index, char)
            out.append(text[index:end])
            index = end
            continue
        if char == "/" and index + 1 < length and text[index + 1] == "/":
            newline = text.find("\n", index)
            if newline == -1:
                break
            out.append("\n")
            index = newline + 1
            continue
        if char == "/" and index + 1 < length and text[index + 1] == "*":
            end = text.find("*/", index + 2)
            if end == -1:
                break
            out.append("\n" * text.count("\n", index, end))
            index = end + 2
            continue
        out.append(char)
        index += 1
    return "".join(out)


def strip_ebnf_comments(text: str) -> str:
    out: list[str] = []
    index = 0
    length = len(text)
    while index < length:
        if text[index] == '"':
            end = skip_quote(text, index, '"')
            out.append(text[index:end])
            index = end
            continue
        if text[index] == "(" and index + 1 < length and text[index + 1] == "*":
            end = text.find("*)", index + 2)
            if end == -1:
                break
            out.append("\n" * text.count("\n", index, end))
            index = end + 2
            continue
        out.append(text[index])
        index += 1
    return "".join(out)


def strip_hash_comments(text: str) -> str:
    out: list[str] = []
    index = 0
    length = len(text)
    quote: str | None = None
    triple = False
    while index < length:
        char = text[index]
        if quote is not None:
            out.append(char)
            if triple and text.startswith(quote, index):
                out.append(quote[1:])
                index += len(quote)
                quote = None
                triple = False
                continue
            if not triple and char == "\\":
                if index + 1 < length:
                    out.append(text[index + 1])
                index += 2
                continue
            if not triple and char == quote:
                quote = None
            index += 1
            continue
        if text.startswith(("'''", '"""'), index):
            quote = text[index : index + 3]
            triple = True
            out.append(quote)
            index += 3
            continue
        if char in {"'", '"'}:
            quote = char
            triple = False
            out.append(char)
            index += 1
            continue
        if char == "#":
            newline = text.find("\n", index)
            if newline == -1:
                break
            out.append("\n")
            index = newline + 1
            continue
        out.append(char)
        index += 1
    return "".join(out)


def skip_quote(text: str, start: int, quote: str) -> int:
    index = start + 1
    while index < len(text):
        if text[index] == "\\":
            index += 2
            continue
        if text[index] == quote:
            return index + 1
        index += 1
    return len(text)


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
