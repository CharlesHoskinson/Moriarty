#!/usr/bin/env python3
"""Check the U0 enforcement map against the frozen canonical stage relation.

Leaf definition: walk the stage schema's ``properties``. An object recurses as
``a.b``. An array of objects recurses as ``a[]`` and then ``.c``. Any other
type is a leaf. An array whose items are not objects is the leaf ``a[]``.

Whether a cited line truly enforces its field is a reviewed claim. This checker
proves C1-C7 only: schema validity, the frozen-schema hash, a sorted bijection
with those leaves, the status and mechanism rules, file and line existence,
a whole-identifier citation, and native-root membership.
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
STAGE_REL = Path(
    "openspec/changes/consolidated-language-kernel/schemas/stage-relation.schema.json"
)
MAP_SCHEMA_REL = Path(
    "openspec/changes/consolidated-language-kernel/schemas/enforcement-map.schema.json"
)
ARTIFACT_REL = Path("deliverables/u0-semantic-contract-2026-09-23/enforcement-map.json")
HOST_ROOT = Path("experiments/moriarty-language/src")
IDENTIFIER = re.compile(r"[A-Za-z_][A-Za-z0-9_]*\Z")
NON_HOST = {"circuit", "ledgerPrimitive", "signature", "nativeBoundary"}


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=MORIARTY_ROOT)
    args = parser.parse_args(argv)
    return check_root(args.root.resolve())


def check_root(root: Path) -> int:
    if not root.is_dir():
        print("blocked: root is not a directory")
        return 2
    blocked = [
        f"blocked: missing {rel.as_posix()}"
        for rel in (MAP_SCHEMA_REL, ARTIFACT_REL, STAGE_REL)
        if not (root / rel).is_file()
    ]
    if blocked:
        return emit(blocked, 2)

    map_schema, schema_error = load_json(root / MAP_SCHEMA_REL)
    artifact, artifact_error = load_json(root / ARTIFACT_REL)
    stage, stage_error = load_json(root / STAGE_REL)
    failures: list[str] = []
    if schema_error or not isinstance(map_schema, dict):
        failures.append(f"FAIL: enforcement map schema is not a JSON object ({schema_error})")
    if artifact_error:
        failures.append(f"FAIL: enforcement map is not UTF-8 JSON ({artifact_error})")
    if stage_error or not isinstance(stage, dict):
        failures.append(f"FAIL: frozen stage schema is not a JSON object ({stage_error})")
    if failures:
        return emit(failures, 1)
    assert isinstance(map_schema, dict)
    assert isinstance(stage, dict)

    try:
        Draft202012Validator.check_schema(map_schema)
    except SchemaError as error:
        return emit([f"FAIL: enforcement map schema is invalid ({one_line(error.message)})"], 1)
    validator = Draft202012Validator(map_schema)
    schema_failures = [
        f"FAIL: {'/'.join(str(part) for part in error.path) or '<root>'}: {one_line(error.message)}"
        for error in validator.iter_errors(artifact)
    ]
    if schema_failures or not isinstance(artifact, dict):
        return emit(schema_failures or ["FAIL: enforcement map is not a JSON object"], 1)
    check_key_order(artifact, map_schema, "enforcement map", failures)
    rows = artifact.get("rows")
    if isinstance(rows, list):
        row_schema = map_schema["properties"]["rows"]["items"]
        mechanism_schema = row_schema["properties"]["mechanisms"]["items"]
        for index, row in enumerate(rows):
            if isinstance(row, dict):
                check_key_order(row, row_schema, f"row {index}", failures)
                mechanisms = row.get("mechanisms")
                if isinstance(mechanisms, list):
                    for mech_index, mechanism in enumerate(mechanisms):
                        if isinstance(mechanism, dict):
                            check_key_order(
                                mechanism,
                                mechanism_schema,
                                f"row {index} mechanism {mech_index}",
                                failures,
                            )

    digest = hashlib.sha256((root / STAGE_REL).read_bytes()).hexdigest()
    if artifact.get("stageSchema") != STAGE_REL.as_posix():
        failures.append("FAIL: stageSchema is not the frozen stage schema path")
    if artifact.get("stageSchemaSha256") != digest:
        failures.append("FAIL: stageSchemaSha256 does not equal the sha256 of the frozen schema")

    try:
        leaves = collect_leaves(stage, "")
    except ValueError as error:
        failures.append(f"FAIL: frozen stage schema leaves cannot be walked ({error})")
        leaves = []
    check_rows(artifact.get("rows"), leaves, failures)

    root_blocked = native_root_block(root, artifact.get("nativeRoots"))
    if root_blocked:
        return emit([*failures, *root_blocked], 2)
    native_roots = [Path(item) for item in artifact["nativeRoots"]]
    citation_blocked = check_citations(root, artifact.get("rows"), native_roots, failures)
    if citation_blocked:
        return emit([*failures, *citation_blocked], 2)
    if failures:
        return emit(failures, 1)

    counts = {"enforced": 0, "host-only": 0, "NOT_ENFORCED": 0}
    for row in artifact["rows"]:
        counts[row["status"]] += 1
    print(
        f"OK: {len(leaves)} leaf fields, {counts['enforced']} enforced, "
        f"{counts['host-only']} host-only, {counts['NOT_ENFORCED']} NOT_ENFORCED"
    )
    return 0


def emit(lines: list[str], code: int) -> int:
    for line in lines:
        print(line)
    return code


def one_line(text: str) -> str:
    return " ".join(text.split())


def load_json(path: Path) -> tuple[object | None, str | None]:
    try:
        return json.loads(path.read_text(encoding="utf-8")), None
    except (OSError, UnicodeError, json.JSONDecodeError) as error:
        return None, str(error)


def check_key_order(value: dict, schema: dict, label: str, failures: list[str]) -> None:
    properties = schema.get("properties")
    if isinstance(properties, dict) and list(value) != list(properties):
        failures.append(f"FAIL: {label} keys are {list(value)!r}")


def collect_leaves(node: object, path: str) -> list[str]:
    """Return dotted leaf paths. Array items insert ``[]`` before the next name."""

    if not isinstance(node, dict):
        raise ValueError(path or "<root>")
    declared = node.get("type")
    if declared == "object":
        properties = node.get("properties")
        if not isinstance(properties, dict):
            raise ValueError(path or "<root>")
        found: list[str] = []
        for key, child in properties.items():
            if not isinstance(key, str):
                raise ValueError(path or "<root>")
            child_path = f"{path}.{key}" if path else key
            found.extend(collect_leaves(child, child_path))
        return found
    if declared == "array":
        items = node.get("items")
        if isinstance(items, dict) and items.get("type") == "object":
            return collect_leaves(items, f"{path}[]")
        if not path:
            raise ValueError("<root>")
        return [f"{path}[]"]
    if not path:
        raise ValueError("<root>")
    return [path]


def check_rows(rows: object, leaves: list[str], failures: list[str]) -> None:
    if not isinstance(rows, list):
        failures.append("FAIL: rows is not an array")
        return
    fields: list[str] = []
    for index, row in enumerate(rows):
        if not isinstance(row, dict) or not isinstance(row.get("schemaField"), str):
            failures.append(f"FAIL: row {index} has no schemaField")
            continue
        fields.append(row["schemaField"])
        check_status(row["schemaField"], row.get("status"), row.get("mechanisms"), failures)
    if len(fields) != len(set(fields)):
        duplicates = sorted({field for field in fields if fields.count(field) > 1})
        failures.append(f"FAIL: duplicate schemaField values {duplicates}")
    if fields != sorted(fields):
        failures.append("FAIL: rows are not sorted by schemaField")
    missing = sorted(set(leaves) - set(fields))
    extra = sorted(set(fields) - set(leaves))
    if missing or extra or len(fields) != len(leaves):
        failures.append(
            f"FAIL: rows are not a bijection with the stage leaf set "
            f"(missing {missing}, extra {extra})"
        )


def check_status(field: str, status: object, mechanisms: object, failures: list[str]) -> None:
    if not isinstance(mechanisms, list):
        failures.append(f"FAIL: {field} mechanisms is not an array")
        return
    kinds = [item.get("kind") for item in mechanisms if isinstance(item, dict)]
    if status == "enforced" and not any(kind in NON_HOST for kind in kinds):
        failures.append(f"FAIL: {field} status enforced requires a non-host mechanism")
    elif status == "host-only" and (not mechanisms or any(kind != "hostCheck" for kind in kinds)):
        failures.append(f"FAIL: {field} status host-only requires only hostCheck mechanisms")
    elif status == "NOT_ENFORCED" and mechanisms:
        failures.append(f"FAIL: {field} status NOT_ENFORCED requires no mechanisms")
    for item in mechanisms:
        if not isinstance(item, dict):
            failures.append(f"FAIL: {field} mechanism is not an object")
            continue
        kind = item.get("kind")
        file_name = item.get("file")
        if kind == "hostCheck" and not isinstance(file_name, str):
            failures.append(f"FAIL: {field} hostCheck file is missing")
        elif kind == "hostCheck" and not under_root(Path(file_name), HOST_ROOT):
            failures.append(
                f"FAIL: {field} hostCheck file is outside {HOST_ROOT.as_posix()}: {file_name}"
            )


def native_root_block(root: Path, native_roots: object) -> list[str]:
    if not isinstance(native_roots, list):
        return ["blocked: nativeRoots is not an array"]
    blocked: list[str] = []
    for item in native_roots:
        if not isinstance(item, str) or not item:
            blocked.append("blocked: nativeRoots entry is empty")
            continue
        path = Path(item)
        if path.is_absolute() or ".." in path.parts or not (root / path).is_dir():
            blocked.append(f"blocked: missing native root {item}")
    return blocked


def check_citations(
    root: Path,
    rows: object,
    native_roots: list[Path],
    failures: list[str],
) -> list[str]:
    blocked: list[str] = []
    if not isinstance(rows, list):
        return blocked
    for row in rows:
        if not isinstance(row, dict) or not isinstance(row.get("mechanisms"), list):
            continue
        field = row.get("schemaField")
        label = field if isinstance(field, str) else "<row>"
        for mechanism in row["mechanisms"]:
            if isinstance(mechanism, dict):
                check_mechanism(root, label, mechanism, native_roots, failures, blocked)
    return blocked


def check_mechanism(
    root: Path,
    field: str,
    mechanism: dict,
    native_roots: list[Path],
    failures: list[str],
    blocked: list[str],
) -> None:
    kind = mechanism.get("kind")
    file_name = mechanism.get("file")
    if kind in NON_HOST:
        path = Path(file_name) if isinstance(file_name, str) else None
        if path is None or not any(under_root(path, native) for native in native_roots):
            failures.append(f"FAIL: {field} {kind} file is outside nativeRoots: {file_name}")
    relative = repo_relative(file_name)
    if relative is None:
        failures.append(f"FAIL: {field} citation file is not a relative path: {file_name}")
        return
    # Read the lexical path. ``..`` is rejected above, so a directory symlink
    # inside the root still names a repository file.
    target = root / relative
    if not target.is_file():
        blocked.append(f"blocked: missing {relative.as_posix()}")
        return
    line_number = mechanism.get("line")
    try:
        lines = target.read_text(encoding="utf-8").splitlines()
    except (OSError, UnicodeError) as error:
        failures.append(f"FAIL: {field} citation file is not readable UTF-8 ({error})")
        return
    if not isinstance(line_number, int) or isinstance(line_number, bool) or not 1 <= line_number <= len(lines):
        failures.append(
            f"FAIL: {field} line {line_number} is outside the file {relative.as_posix()}"
        )
        return
    symbol = mechanism.get("symbol")
    if not isinstance(symbol, str) or IDENTIFIER.fullmatch(symbol) is None:
        failures.append(f"FAIL: {field} symbol {symbol!r} is not an identifier")
        return
    if not has_identifier(lines[line_number - 1], symbol):
        failures.append(
            f"FAIL: {field} symbol {symbol} is not an identifier on line {line_number} "
            f"of {relative.as_posix()}"
        )


def repo_relative(file_name: object) -> Path | None:
    if not isinstance(file_name, str) or not file_name:
        return None
    path = Path(file_name)
    if path.is_absolute() or ".." in path.parts:
        return None
    return path


def under_root(path: Path, root: Path) -> bool:
    return path != root and path.is_relative_to(root)


def has_identifier(line: str, symbol: str) -> bool:
    return re.search(rf"\b{re.escape(symbol)}\b", strip_comments(line), flags=re.ASCII) is not None


def strip_comments(line: str) -> str:
    """Drop ``//`` and ``#`` line comments and ``/* */`` comments on this line."""

    pieces: list[str] = []
    index = 0
    length = len(line)
    while index < length:
        if line.startswith("/*", index):
            end = line.find("*/", index + 2)
            if end < 0:
                break
            index = end + 2
            continue
        if line.startswith("//", index) or line[index] == "#":
            break
        pieces.append(line[index])
        index += 1
    return "".join(pieces)


if __name__ == "__main__":
    raise SystemExit(main())
