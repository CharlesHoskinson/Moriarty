#!/usr/bin/env python3
"""Check the U0 stage relation, embeddings, and judgments. Proves only C1-C9.

Absence means not found by the recorded search. It is not a proof of non-realisation.
Semantic adequacy of a cited declaration is a reviewed claim. The checker proves
only declaration, context, and profile membership.

Parser subset, after comments are removed. TypeScript and K string literals are
removed. EBNF double quotes are kept because they are terminals.
TypeScript: ``interface`` and ``type X = {...}`` members that start a depth-0
``;`` segment, plus top-level function, const, class, type, and interface names.
Ternaries, case labels, parameters, and locals are not declarations.
EBNF: ``name =`` declares ``name`` and its quoted identifier terminals. A
referenced nonterminal is not a declaration. K: ``syntax Sort ::=`` constructors
belong to Sort, and ``<cell>`` belongs to that cell.
"""

from __future__ import annotations

import argparse
import json
import re
from pathlib import Path

from jsonschema import Draft202012Validator
from jsonschema.exceptions import SchemaError


MORIARTY_ROOT = Path(__file__).resolve().parents[1]
SCHEMA_REL = Path("openspec/changes/consolidated-language-kernel/schemas/stage-relation.schema.json")
EMBEDDINGS_REL = Path("deliverables/u0-semantic-contract-2026-09-23/source-core-embeddings.json")
JUDGMENTS_REL = Path("deliverables/u0-semantic-contract-2026-09-23/judgments.json")
SCHEMA_ID = "https://moriarty.invalid/u0/stage-relation/1"
SCHEMA_TITLE = "Moriarty canonical stage relation"
SOURCE_PROFILE = "moriarty-financial-agreement-source/5"
CORE_PROFILE = "moriarty-financial-lifecycle/1"
JUDGMENT_KEYS = ["stage", "intent", "effect", "authority", "history", "failure"]
SEARCH_ROOTS = [
    "experiments/moriarty-language/spec/successor",
    "experiments/moriarty-language/src/successor",
    "experiments/moriarty-language/formal/k",
]
LIMITATION = (
    "absence means not found by this recorded search; it is not a proof of non-realisation. "
    "Semantic adequacy of a cited declaration is a reviewed claim; the checker proves "
    "declaration, context, and profile membership only."
)
ROW_KEYS = [
    "schemaField", "sourceFile", "sourceSymbol", "sourceContext",
    "coreFile", "coreSymbol", "coreContext", "realisation", "present", "note",
]
SIDES = (
    ("source", "sourceFile", "sourceSymbol", "sourceContext"),
    ("core", "coreFile", "coreSymbol", "coreContext"),
)
PARTIAL_NOTE = re.compile(r"^Exists: .+; Missing: .+$", re.DOTALL)
IDENT = r"[A-Za-z_][A-Za-z0-9_]*"
TOP_LEVEL = re.compile(
    rf"(?m)^(?:export\s+)?(?:declare\s+)?(?:async\s+)?(?:function|const|class|type|interface)\s+({IDENT})\b"
)
OBJECT_TYPE = re.compile(rf"(?m)^(?:export\s+)?(?:interface|type)\s+({IDENT})\b")
MEMBER = re.compile(rf"\s*(?:readonly\s+)?({IDENT})\s*\??\s*:")
PRODUCTION = re.compile(rf"(?m)^({IDENT})\s*=")
SYNTAX = re.compile(rf"syntax\s+({IDENT})\s*::=\s*([^\n]+)")
CELL = re.compile(rf"<({IDENT})>")

def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=MORIARTY_ROOT)
    root = parser.parse_args(argv).root.resolve()
    blocked = [
        f"blocked: missing {rel.as_posix()}"
        for rel in (SCHEMA_REL, EMBEDDINGS_REL, JUDGMENTS_REL)
        if not (root / rel).is_file()
    ]
    if blocked:
        print("\n".join(blocked))
        return 2
    failures: list[str] = []
    schema = read_json(root / SCHEMA_REL, SCHEMA_REL.as_posix(), failures)
    embeddings = read_json(root / EMBEDDINGS_REL, EMBEDDINGS_REL.as_posix(), failures)
    judgments = read_json(root / JUDGMENTS_REL, JUDGMENTS_REL.as_posix(), failures)
    if not isinstance(schema, dict) or not isinstance(embeddings, dict) or not isinstance(judgments, dict):
        print("\n".join(failures))
        return 1
    check_c1(schema, failures)
    leaves = collect_leaves(schema, "", failures)
    rows = embeddings.get("rows") if isinstance(embeddings.get("rows"), list) else []
    if not isinstance(embeddings.get("rows"), list):
        failures.append("FAIL: embeddings rows is not a list")
    check_c2(rows, leaves, failures)
    check_c3(rows, failures)
    if block_on_inputs(root, embeddings, rows):
        return 2
    check_c5(root, rows, failures)
    check_c6(embeddings, rows, failures)
    check_c7(embeddings, rows, failures)
    check_c8_shape(embeddings, failures)
    check_c9(judgments, set(leaves), failures)
    if failures:
        print("\n".join(failures))
        return 1
    present = sum(isinstance(row, dict) and row.get("present") is True for row in rows)
    absent = sum(isinstance(row, dict) and row.get("realisation") == "absent" for row in rows)
    partial = sum(isinstance(row, dict) and row.get("realisation") == "partial" for row in rows)
    print(f"OK: {len(leaves)} leaf fields, {present} present, {absent} absent")
    print(f"partial: {partial}")
    print(LIMITATION)
    return 0

def read_json(path: Path, label: str, failures: list[str]) -> object | None:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        failures.append(f"FAIL: {label} is not JSON ({exc.msg})")
        return None

def check_c1(schema: dict, failures: list[str]) -> None:
    try:
        Draft202012Validator.check_schema(schema)
    except SchemaError as exc:
        failures.append(f"FAIL: schema is not a valid draft 2020-12 schema ({exc.message.splitlines()[0]})")
    if schema.get("$id") != SCHEMA_ID or schema.get("title") != SCHEMA_TITLE:
        failures.append("FAIL: schema $id or title is wrong")
    walk_closed_objects(schema, "<root>", failures)
    props = schema.get("properties")
    judgments = props.get("judgments") if isinstance(props, dict) else None
    keys = judgments.get("properties") if isinstance(judgments, dict) else None
    if not isinstance(keys, dict) or list(keys) != JUDGMENT_KEYS:
        failures.append(f"FAIL: judgments properties are {list(keys) if isinstance(keys, dict) else None!r}")

def walk_closed_objects(node: object, path: str, failures: list[str]) -> None:
    if not isinstance(node, dict):
        return
    if node.get("type") == "object" and isinstance(node.get("properties"), dict):
        props = node["properties"]
        if node.get("additionalProperties") is not False:
            failures.append(f"FAIL: {path} additionalProperties is not false")
        if list(node.get("required") or []) != list(props):
            failures.append(f"FAIL: {path} required keys do not match properties")
        for key, child in props.items():
            walk_closed_objects(child, key if path == "<root>" else f"{path}.{key}", failures)
    if node.get("type") == "array" and isinstance(node.get("items"), dict):
        walk_closed_objects(node["items"], f"{path}[]", failures)

def collect_leaves(node: object, path: str, failures: list[str]) -> list[str]:
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
            found.extend(collect_leaves(child, f"{path}.{key}" if path else key, failures))
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

def check_c2(rows: list, leaves: list[str], failures: list[str]) -> None:
    fields = [row.get("schemaField") if isinstance(row, dict) else None for row in rows]
    duplicates = sorted({field for field in fields if field is not None and fields.count(field) > 1})
    if duplicates:
        failures.append(f"FAIL: duplicate embedding rows for {duplicates!r}")
    names = [field for field in fields if isinstance(field, str)]
    if fields != sorted(names):
        failures.append("FAIL: embedding rows are not sorted by schemaField")
    missing = [leaf for leaf in leaves if leaf not in fields]
    extra = [field for field in fields if field not in leaves]
    if missing:
        failures.append(f"FAIL: embeddings missing leaf fields {missing!r}")
    if extra:
        failures.append(f"FAIL: embeddings extra fields {extra!r}")

def check_c3(rows: list, failures: list[str]) -> None:
    for row in rows:
        if not isinstance(row, dict) or list(row) != ROW_KEYS:
            failures.append(f"FAIL: embedding row keys are {list(row) if isinstance(row, dict) else None!r}")
            continue
        label = row["schemaField"]
        realisation = row["realisation"]
        if realisation not in ("present", "partial", "absent"):
            failures.append(f"FAIL: {label} realisation is {realisation!r}")
            continue
        if row["present"] is not (realisation != "absent"):
            failures.append(f"FAIL: {label} present is not (realisation != absent)")
        sides = [(row[file_key], row[symbol_key], row[context_key]) for _side, file_key, symbol_key, context_key in SIDES]
        if realisation == "absent":
            if any(value is not None for side in sides for value in side):
                failures.append(f"FAIL: {label} is absent but a citation field is set")
            continue
        good = [all(isinstance(value, str) and value for value in side) for side in sides]
        partial = [any(value is not None for value in side) and not all(value is not None for value in side) for side in sides]
        if not any(good) or any(partial):
            failures.append(f"FAIL: {label} citation sides are incomplete")
        if realisation == "partial" and (not isinstance(row["note"], str) or not PARTIAL_NOTE.fullmatch(row["note"])):
            failures.append(f"FAIL: {label} partial note is not 'Exists: ...; Missing: ...'")

def block_on_inputs(root: Path, embeddings: dict, rows: list) -> bool:
    search = embeddings.get("absenceSearch")
    roots = search.get("roots") if isinstance(search, dict) else None
    if isinstance(roots, list):
        missing = [item for item in roots if isinstance(item, str) and not (root / item).is_dir()]
        if missing:
            print("\n".join(f"blocked: missing {item}" for item in missing))
            return True
    missing_files = []
    for row in rows:
        if not isinstance(row, dict):
            continue
        for key in ("sourceFile", "coreFile"):
            rel = row.get(key)
            if isinstance(rel, str) and rel not in missing_files and not (root / rel).is_file():
                missing_files.append(rel)
    if missing_files:
        print("\n".join(f"blocked: missing {item}" for item in missing_files))
        return True
    return False

def check_c8_shape(embeddings: dict, failures: list[str]) -> None:
    node = embeddings.get("absenceSearch")
    if not isinstance(node, dict) or list(node) != ["roots", "method", "limitation"]:
        failures.append(f"FAIL: absenceSearch keys are {list(node) if isinstance(node, dict) else None!r}")
        return
    if node.get("roots") != SEARCH_ROOTS:
        failures.append(f"FAIL: absenceSearch roots are {node.get('roots')!r}")
    if not isinstance(node.get("method"), str) or not str(node.get("method")).endswith("."):
        failures.append("FAIL: absenceSearch method is not one sentence")
    if node.get("limitation") != LIMITATION:
        failures.append("FAIL: absenceSearch limitation is not the required sentence")

def check_c6(embeddings: dict, rows: list, failures: list[str]) -> None:
    profiles = embeddings.get("profileFiles")
    if embeddings.get("sourceProfile") != SOURCE_PROFILE or embeddings.get("coreProfile") != CORE_PROFILE:
        failures.append("FAIL: embeddings sourceProfile or coreProfile is wrong")
    if not isinstance(profiles, dict) or list(profiles) != [SOURCE_PROFILE, CORE_PROFILE]:
        failures.append("FAIL: profileFiles keys are not the declared source and Core profiles")
        return
    allowed = {
        "source": set(profiles[SOURCE_PROFILE]) if isinstance(profiles[SOURCE_PROFILE], list) else set(),
        "core": set(profiles[CORE_PROFILE]) if isinstance(profiles[CORE_PROFILE], list) else set(),
    }
    for row in rows:
        if not isinstance(row, dict) or row.get("realisation") == "absent":
            continue
        for side, file_key, _symbol_key, _context_key in SIDES:
            rel = row.get(file_key)
            if isinstance(rel, str) and rel not in allowed[side]:
                profile = SOURCE_PROFILE if side == "source" else CORE_PROFILE
                failures.append(f"FAIL: {row.get('schemaField')} {side} file {rel} is not listed for {profile}")

def check_c7(embeddings: dict, rows: list, failures: list[str]) -> None:
    table = embeddings.get("classificationTable")
    if not isinstance(table, list):
        failures.append("FAIL: classificationTable is not a list")
        return
    classes: dict[str, str] = {}
    for entry in table:
        if not isinstance(entry, dict) or list(entry) != ["context", "realisationClass", "rationale"]:
            found = list(entry) if isinstance(entry, dict) else None
            failures.append(f"FAIL: classificationTable entry keys are {found!r}")
            continue
        context = entry["context"]
        if not isinstance(context, str) or context in classes:
            failures.append(f"FAIL: duplicate classificationTable context {context!r}")
        if entry["realisationClass"] not in ("present", "partial"):
            failures.append(f"FAIL: classificationTable {context} realisationClass is {entry['realisationClass']!r}")
        if not isinstance(entry["rationale"], str) or not entry["rationale"].strip():
            failures.append(f"FAIL: classificationTable {context} rationale is empty")
        if isinstance(context, str):
            classes[context] = entry["realisationClass"]
    cited: set[str] = set()
    for row in rows:
        if not isinstance(row, dict) or row.get("realisation") not in ("present", "partial"):
            continue
        for side, _file_key, _symbol_key, context_key in SIDES:
            context = row.get(context_key)
            if not isinstance(context, str):
                continue
            cited.add(context)
            if context not in classes:
                failures.append(f"FAIL: {row.get('schemaField')} {side} context {context} has no classificationTable entry")
            elif classes[context] != row["realisation"]:
                failures.append(
                    f"FAIL: classificationTable context {context} is {classes[context]} "
                    f"but {row.get('schemaField')} realisation is {row['realisation']}"
                )
    for context in classes:
        if context not in cited:
            failures.append(f"FAIL: classificationTable context {context} is not cited")

def check_c9(judgments: dict, leaves: set[str], failures: list[str]) -> None:
    rows = judgments.get("judgments")
    if not isinstance(rows, list):
        failures.append("FAIL: judgments is not a list")
        return
    keys = [row.get("key") if isinstance(row, dict) else None for row in rows]
    if keys != JUDGMENT_KEYS:
        failures.append(f"FAIL: judgments keys are {keys!r}")
    for row in rows:
        if not isinstance(row, dict):
            continue
        for field in row.get("schemaFields") or []:
            if field not in leaves:
                failures.append(f"FAIL: judgments.{row.get('key')} schemaFields entry {field!r} is not a leaf")

def check_c5(root: Path, rows: list, failures: list[str]) -> None:
    cache: dict[str, dict[str, set[str]]] = {}
    for row in rows:
        if not isinstance(row, dict) or row.get("realisation") not in ("present", "partial"):
            continue
        for side, file_key, symbol_key, context_key in SIDES:
            rel, symbol, context = row.get(file_key), row.get(symbol_key), row.get(context_key)
            if not all(isinstance(value, str) for value in (rel, symbol, context)):
                continue
            if not (root / rel).is_file():
                continue
            declared = cache.setdefault(rel, declarations(root / rel))
            if symbol not in declared.get(context, set()):
                failures.append(f"FAIL: {row.get('schemaField')} {side} symbol {symbol} is not a declaration in {context} in {rel}")

def declarations(path: Path) -> dict[str, set[str]]:
    text = path.read_text(encoding="utf-8")
    if path.suffix == ".ebnf":
        return ebnf_declarations(strip_ebnf(text))
    if path.suffix == ".k":
        return k_declarations(strip_code(text))
    if path.suffix in {".ts", ".tsx", ".js", ".mjs", ".cjs"}:
        return ts_declarations(strip_code(text))
    return {}

def strip_code(text: str) -> str:
    chars = list(text)
    index, length = 0, len(chars)
    def blank(start: int, end: int) -> None:
        for cursor in range(start, end):
            if chars[cursor] != "\n":
                chars[cursor] = " "
    while index < length:
        char = chars[index]
        nxt = chars[index + 1] if index + 1 < length else ""
        if char == "/" and nxt == "/":
            end = index
            while end < length and chars[end] != "\n":
                end += 1
            blank(index, end)
            index = end
            continue
        if char == "/" and nxt == "*":
            end = text.find("*/", index + 2)
            end = length if end < 0 else end + 2
            blank(index, end)
            index = end
            continue
        if char in "'\"`":
            end = index + 1
            while end < length and chars[end] != char:
                end += 2 if chars[end] == "\\" else 1
            blank(index, min(length, end + 1))
            index = min(length, end + 1)
            continue
        index += 1
    return "".join(chars)

def strip_ebnf(text: str) -> str:
    chars = list(text)
    index = 0
    while index < len(chars):
        if text.startswith("(*", index):
            end = text.find("*)", index + 2)
            if end < 0:
                break
            for cursor in range(index, end + 2):
                if chars[cursor] != "\n":
                    chars[cursor] = " "
            index = end + 2
            continue
        index += 1
    return "".join(chars)

def ts_declarations(text: str) -> dict[str, set[str]]:
    found: dict[str, set[str]] = {}
    for match in TOP_LEVEL.finditer(text):
        found.setdefault(match.group(1), set()).add(match.group(1))
    for match in OBJECT_TYPE.finditer(text):
        body = object_body(text, match.end())
        if body is not None:
            found.setdefault(match.group(1), set()).update(object_members(body))
    return found

def object_body(text: str, start: int) -> str | None:
    index, length = start, len(text)
    while index < length and text[index].isspace():
        index += 1
    if index < length and text[index] == "<":
        depth = 1
        index += 1
        while index < length and depth:
            depth += text[index] == "<"
            depth -= text[index] == ">"
            index += 1
        while index < length and text[index].isspace():
            index += 1
    if index < length and text[index] == "=":
        index += 1
        while index < length and text[index] not in "{;":
            index += 1
    if index >= length or text[index] != "{":
        return None
    end, depth = index + 1, 1
    while end < length and depth:
        depth += text[end] == "{"
        depth -= text[end] == "}"
        end += 1
    return text[index + 1:end - 1]

def object_members(body: str) -> set[str]:
    members: set[str] = set()
    depth = 0
    start = 0
    for index, char in enumerate(body):
        if char == "{":
            depth += 1
        elif char == "}":
            depth = max(0, depth - 1)
        elif char == ";" and depth == 0:
            match = MEMBER.match(body, start)
            if match:
                members.add(match.group(1))
            start = index + 1
    match = MEMBER.match(body, start)
    if match:
        members.add(match.group(1))
    return members

def ebnf_declarations(text: str) -> dict[str, set[str]]:
    found: dict[str, set[str]] = {}
    for match in PRODUCTION.finditer(text):
        body = production_body(text, match.end())
        symbols = {match.group(1), *(set(re.findall(rf'"({IDENT})"', body)))}
        found.setdefault(match.group(1), set()).update(symbols)
    return found

def production_body(text: str, start: int) -> str:
    depth = 0
    quote = False
    for index in range(start, len(text)):
        char = text[index]
        if char == '"':
            quote = not quote
        elif not quote and char in "([{":
            depth += 1
        elif not quote and char in ")]}":
            depth = max(0, depth - 1)
        elif not quote and char == ";" and depth == 0:
            return text[start:index]
    return text[start:]

def k_declarations(text: str) -> dict[str, set[str]]:
    found: dict[str, set[str]] = {}
    for match in SYNTAX.finditer(text):
        found.setdefault(match.group(1), set()).update(constructors(match.group(2)))
    for cell in CELL.findall(text):
        found.setdefault(cell, set()).add(cell)
    return found

def constructors(rhs: str) -> set[str]:
    names: set[str] = set()
    depth = 0
    index = 0
    while index < len(rhs):
        char = rhs[index]
        if char in "([{":
            depth += 1
        elif char in ")]}":
            depth = max(0, depth - 1)
        else:
            match = re.match(IDENT, rhs[index:])
            if match and depth == 0:
                names.add(match.group(0))
                index += match.end()
                continue
        index += 1
    return names

if __name__ == "__main__":
    raise SystemExit(main())
