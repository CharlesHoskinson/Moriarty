#!/usr/bin/env python3
"""Check the U0 stage relation, embeddings, and judgments. Proves only C1-C9.
Absence means not found by the recorded search. It is not a proof of non-realisation.
Semantic adequacy of a cited declaration is a reviewed claim. The checker proves
only declaration, context, and profile membership. C1 requires stageSchemaSha256
to equal the sha256 of the schema bytes. Embeddings and judgments are validated
against inline draft 2020-12 schemas before C2-C9. A violation names its JSON path.
C4 splits each raw path on '/' and rejects a '.' or '..' segment before resolving
under an absenceSearch root. A missing root or file is blocked.
Parser subset, after comments are removed. TypeScript string literals are removed.
EBNF keeps quotes, but a quoted terminal is not a declaration. K keeps quotes.
TypeScript: function, const, class, type, and interface names only at brace depth 0.
Direct members of an interface or type object body only. Nested locals are not
declarations. EBNF: only `name =` at line start is a declaration.
K syntax continues until the next syntax, rule, configuration, or endmodule.
A constructor is an identifier before '(', a quoted terminal, or a klabel or symbol
attribute. A bare sort reference is a subsort, not a constructor.
A <cell> counts only inside a configuration block, until the next syntax, rule, or endmodule.
"""
from __future__ import annotations
import argparse
import hashlib
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
def closed(properties: dict) -> dict:
    return {"type": "object", "additionalProperties": False, "required": list(properties), "properties": properties}
STR = {"type": "string", "minLength": 1}
NULL_STR = {"type": ["string", "null"], "minLength": 1}
STRINGS = {"type": "array", "items": STR}
EMBEDDINGS_SCHEMA = closed({
    "schemaVersion": {"const": "moriarty-u0-embeddings/1"},
    "stageSchemaSha256": {"type": "string", "minLength": 64, "maxLength": 64},
    "sourceProfile": {"const": SOURCE_PROFILE},
    "coreProfile": {"const": CORE_PROFILE},
    "classificationRule": STR,
    "profileFiles": closed({SOURCE_PROFILE: STRINGS, CORE_PROFILE: STRINGS}),
    "classificationTable": {"type": "array", "items": closed({
        "context": STR, "realisationClass": {"enum": ["present", "partial"]}, "rationale": STR,
    })},
    "absenceSearch": closed({"roots": STRINGS, "method": STR, "limitation": {"const": LIMITATION}}),
    "rows": {"type": "array", "items": closed({
        "schemaField": STR,
        "sourceFile": NULL_STR, "sourceSymbol": NULL_STR, "sourceContext": NULL_STR,
        "coreFile": NULL_STR, "coreSymbol": NULL_STR, "coreContext": NULL_STR,
        "realisation": {"enum": ["present", "partial", "absent"]},
        "present": {"type": "boolean"},
        "note": STR,
    })},
})
JUDGMENTS_SCHEMA = closed({
    "schemaVersion": {"const": "moriarty-u0-judgments/1"},
    "judgments": {"type": "array", "items": closed({
        "key": STR, "designDocJudgment": STR, "definition": STR, "schemaFields": STRINGS,
    })},
})
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
K_SYNTAX = re.compile(rf"(?m)^[ \t]*syntax\s+({IDENT})\s*::=")
K_STOP = re.compile(r"(?m)^[ \t]*(?:syntax|rule|configuration|endmodule)\b")
CELL = re.compile(rf"<({IDENT})>")
CONFIG = re.compile(r"(?m)^[ \t]*configuration\b")
FILE_KEYS = ("sourceFile", "coreFile")
def main(argv: list[str] | None = None) -> int:
    try:
        return run_checks(argv)
    except Exception as exc:
        print(f"FAIL: internal error: {type(exc).__name__}: {exc}")
        return 1
def run_checks(argv: list[str] | None) -> int:
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
    schema_bytes, schema = load_json(root / SCHEMA_REL, SCHEMA_REL.as_posix(), failures)
    _, embeddings = load_json(root / EMBEDDINGS_REL, EMBEDDINGS_REL.as_posix(), failures)
    _, judgments = load_json(root / JUDGMENTS_REL, JUDGMENTS_REL.as_posix(), failures)
    if not isinstance(schema, dict) or not isinstance(embeddings, dict) or not isinstance(judgments, dict):
        print("\n".join(failures))
        return 1
    typed = input_failures(EMBEDDINGS_REL.as_posix(), EMBEDDINGS_SCHEMA, embeddings)
    typed += input_failures(JUDGMENTS_REL.as_posix(), JUDGMENTS_SCHEMA, judgments)
    check_c1(schema, embeddings, schema_bytes, failures)
    if typed:
        print("\n".join(typed + failures))
        return 1
    leaves = collect_leaves(schema, "", failures)
    rows = embeddings.get("rows") if isinstance(embeddings.get("rows"), list) else []
    if not isinstance(embeddings.get("rows"), list):
        failures.append("FAIL: embeddings rows is not a list")
    check_c2(rows, leaves, failures)
    check_c3(rows, failures)
    if block_on_inputs(root, embeddings, rows):
        return 2
    check_c4(root, embeddings, rows, failures)
    check_c5(root, rows, failures)
    check_c6(embeddings, rows, failures)
    check_c7(embeddings, rows, failures)
    check_c8_shape(embeddings, failures)
    check_c9(judgments, set(leaves), failures)
    if failures:
        print("\n".join(failures))
        return 1
    present = sum(isinstance(row, dict) and row.get("realisation") == "present" for row in rows)
    partial = sum(isinstance(row, dict) and row.get("realisation") == "partial" for row in rows)
    absent = sum(isinstance(row, dict) and row.get("realisation") == "absent" for row in rows)
    print(f"OK: {len(leaves)} leaf fields, {present} present, {partial} partial, {absent} absent")
    print(LIMITATION)
    return 0
def load_json(path: Path, label: str, failures: list[str]) -> tuple[bytes, dict | None]:
    raw = path.read_bytes()
    try:
        value = json.loads(raw.decode("utf-8"))
    except UnicodeDecodeError as exc:
        failures.append(f"FAIL: {label} is not UTF-8 ({exc.reason})")
        return raw, None
    except json.JSONDecodeError as exc:
        failures.append(f"FAIL: {label} is not JSON ({exc.msg})")
        return raw, None
    if not isinstance(value, dict):
        failures.append(f"FAIL: {label} is not a JSON object")
        return raw, None
    return raw, value
def input_failures(label: str, schema: dict, instance: object) -> list[str]:
    try:
        errors = sorted(Draft202012Validator(schema).iter_errors(instance), key=lambda error: error.json_path)
    except SchemaError as exc:
        return [f"FAIL: {label} checker schema is invalid ({exc.message.splitlines()[0]})"]
    return [f"FAIL: {label} {error.json_path}: {error.message.splitlines()[0]}" for error in errors]
def check_c1(schema: dict, embeddings: dict, schema_bytes: bytes, failures: list[str]) -> None:
    digest = hashlib.sha256(schema_bytes).hexdigest()
    if embeddings.get("stageSchemaSha256") != digest:
        failures.append("FAIL: stageSchemaSha256 does not match the schema bytes")
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
        required = node.get("required")
        if not isinstance(required, list) or required != list(props):
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
    names = [field for field in fields if isinstance(field, str)]
    duplicates = sorted({field for field in names if names.count(field) > 1})
    if duplicates:
        failures.append(f"FAIL: duplicate embedding rows for {duplicates!r}")
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
        note = row["note"]
        if not isinstance(note, str) or not note.strip():
            failures.append(f"FAIL: {label} note is not a non-empty string")
        elif realisation == "partial" and not PARTIAL_NOTE.fullmatch(note):
            failures.append(f"FAIL: {label} partial note is not 'Exists: ...; Missing: ...'")
        sides = [(row[file_key], row[symbol_key], row[context_key]) for _side, file_key, symbol_key, context_key in SIDES]
        if realisation == "absent":
            if any(value is not None for side in sides for value in side):
                failures.append(f"FAIL: {label} is absent but a citation field is set")
            continue
        whole = [all(value is None for value in side) or all(isinstance(value, str) and value for value in side) for side in sides]
        cited = [all(isinstance(value, str) and value for value in side) for side in sides]
        if not all(whole) or not any(cited):
            failures.append(f"FAIL: {label} citation sides are incomplete")
def block_on_inputs(root: Path, embeddings: dict, rows: list) -> bool:
    missing = [item for item in embeddings["absenceSearch"]["roots"] if not (root / item).is_dir()]
    if missing:
        print("\n".join(f"blocked: missing {item}" for item in missing))
        return True
    missing_files = [rel for rel in dict.fromkeys(iter_rels(embeddings, rows)) if not (root / rel).is_file()]
    if missing_files:
        print("\n".join(f"blocked: missing {item}" for item in missing_files))
        return True
    return False
def iter_rels(embeddings: dict, rows: list) -> list[str]:
    rels = [row[key] for row in rows for key in FILE_KEYS if isinstance(row[key], str)]
    for files in embeddings["profileFiles"].values():
        rels.extend(item for item in files if isinstance(item, str))
    return rels
def check_c4(root: Path, embeddings: dict, rows: list, failures: list[str]) -> None:
    roots = embeddings["absenceSearch"]["roots"]
    for row in rows:
        for key in FILE_KEYS:
            rel = row[key]
            if isinstance(rel, str) and not contained(root, rel, roots):
                failures.append(f"FAIL: {row['schemaField']} cited file {rel} is outside the declared roots")
    for files in embeddings["profileFiles"].values():
        for rel in files:
            if not isinstance(rel, str) or not rel or not contained(root, rel, roots):
                failures.append(f"FAIL: profileFiles entry {rel} is outside the declared roots")
def contained(root: Path, rel: str, roots: list[str]) -> bool:
    parts = rel.split("/")
    if Path(rel).is_absolute() or any(part in {".", ".."} for part in parts):
        return False
    resolved = (root / rel).resolve()
    return any(resolved.is_relative_to((root / item).resolve()) for item in roots)
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
    profiles = embeddings["profileFiles"]
    if embeddings["sourceProfile"] != SOURCE_PROFILE or embeddings["coreProfile"] != CORE_PROFILE:
        failures.append("FAIL: embeddings sourceProfile or coreProfile is wrong")
    if list(profiles) != [SOURCE_PROFILE, CORE_PROFILE]:
        failures.append("FAIL: profileFiles keys are not the declared source and Core profiles")
        return
    allowed = {"source": set(profiles[SOURCE_PROFILE]), "core": set(profiles[CORE_PROFILE])}
    for row in rows:
        if row["realisation"] == "absent":
            continue
        for side, file_key, _symbol_key, _context_key in SIDES:
            rel = row[file_key]
            if isinstance(rel, str) and rel not in allowed[side]:
                profile = SOURCE_PROFILE if side == "source" else CORE_PROFILE
                failures.append(f"FAIL: {row['schemaField']} {side} file {rel} is not listed for {profile}")
def check_c7(embeddings: dict, rows: list, failures: list[str]) -> None:
    classes: dict[str, str] = {}
    for entry in embeddings["classificationTable"]:
        if list(entry) != ["context", "realisationClass", "rationale"]:
            failures.append(f"FAIL: classificationTable entry keys are {list(entry)!r}")
            continue
        context = entry["context"]
        if context in classes:
            failures.append(f"FAIL: duplicate classificationTable context {context!r}")
        if entry["realisationClass"] not in ("present", "partial"):
            failures.append(f"FAIL: classificationTable {context} realisationClass is {entry['realisationClass']!r}")
        if not entry["rationale"].strip():
            failures.append(f"FAIL: classificationTable {context} rationale is empty")
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
            failures.append("FAIL: judgments row is not an object")
            continue
        fields = row.get("schemaFields")
        if not isinstance(fields, list):
            failures.append(f"FAIL: judgments.{row.get('key')} schemaFields is not a list")
            continue
        for field in fields:
            if not isinstance(field, str) or field not in leaves:
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
        return k_declarations(text)
    if path.suffix in {".ts", ".tsx", ".js", ".mjs", ".cjs"}:
        return ts_declarations(strip_code(text))
    return {}
def strip_code(text: str, *, strings: bool = True) -> str:
    chars, index, length = list(text), 0, len(text)
    def blank(start: int, end: int) -> None:
        for cursor in range(start, end):
            if chars[cursor] != "\n":
                chars[cursor] = " "
    while index < length:
        pair = text[index:index + 2]
        if pair in ("//", "/*"):
            marker = "\n" if pair == "//" else "*/"
            end = text.find(marker, index + 2)
            end = length if end < 0 else end + (0 if pair == "//" else 2)
            blank(index, end)
            index = end
            continue
        if strings and chars[index] in "'\"`":
            quote, end = chars[index], index + 1
            while end < length and chars[end] != quote:
                end += 2 if chars[end] == "\\" else 1
            blank(index, min(length, end + 1))
            index = min(length, end + 1)
            continue
        index += 1
    return "".join(chars)
def strip_ebnf(text: str) -> str:
    def blank(match: re.Match[str]) -> str:
        return "".join("\n" if char == "\n" else " " for char in match.group(0))
    return re.sub(r"\(\*.*?\*\)", blank, text, flags=re.DOTALL)
def brace_depth(text: str, index: int) -> int:
    return text.count("{", 0, index) - text.count("}", 0, index)
def ts_declarations(text: str) -> dict[str, set[str]]:
    found: dict[str, set[str]] = {}
    for match in TOP_LEVEL.finditer(text):
        if brace_depth(text, match.start()) == 0:
            found.setdefault(match.group(1), set()).add(match.group(1))
    for match in OBJECT_TYPE.finditer(text):
        if brace_depth(text, match.start()) != 0:
            continue
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
    depth = start = 0
    for index, char in enumerate(body + ";"):
        if char == "{":
            depth += 1
        elif char == "}":
            depth = max(0, depth - 1)
        elif char == ";" and depth == 0:
            match = MEMBER.match(body, start)
            if match:
                members.add(match.group(1))
            start = index + 1
    return members
def ebnf_declarations(text: str) -> dict[str, set[str]]:
    return {match.group(1): {match.group(1)} for match in PRODUCTION.finditer(text)}
def k_declarations(text: str) -> dict[str, set[str]]:
    commented = strip_code(text, strings=False)
    found: dict[str, set[str]] = {}
    for match in K_SYNTAX.finditer(commented):
        stop = K_STOP.search(commented, match.end())
        rhs = commented[match.end(): stop.start() if stop else len(commented)]
        found.setdefault(match.group(1), set()).update(k_constructors(rhs))
    for match in CONFIG.finditer(commented):
        stop = K_STOP.search(commented, match.end())
        block = commented[match.end(): stop.start() if stop else len(commented)]
        for cell in CELL.findall(strip_code(block)):
            found.setdefault(cell, set()).add(cell)
    return found
def k_constructors(rhs: str) -> set[str]:
    names = {item for group in re.findall(r"\[([^\[\]]*)\]", rhs) for item in re.findall(rf"(?:klabel|symbol)\(\s*({IDENT})", group)}
    depth = index = 0
    while index < len(rhs):
        char = rhs[index]
        if char in "'\"":
            end = index + 1
            while end < len(rhs) and rhs[end] != char:
                end += 2 if rhs[end] == "\\" else 1
            if char == '"' and re.fullmatch(IDENT, rhs[index + 1:end]):
                names.add(rhs[index + 1:end])
            index = min(len(rhs), end + 1)
            continue
        if char in "([{":
            depth += 1
        elif char in ")]}":
            depth = max(0, depth - 1)
        elif (match := re.match(IDENT, rhs[index:])) and depth == 0:
            cursor = index + match.end()
            while cursor < len(rhs) and rhs[cursor].isspace():
                cursor += 1
            if cursor < len(rhs) and rhs[cursor] == "(":
                names.add(match.group(0))
            index += match.end()
            continue
        index += 1
    return names
if __name__ == "__main__":
    raise SystemExit(main())
