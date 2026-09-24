#!/usr/bin/env python3
"""Check G1-G8 and generate the deterministic U0 exit receipt."""

from __future__ import annotations

import argparse
import hashlib
import json
import subprocess
import sys
from pathlib import Path
from typing import Any

from jsonschema import Draft202012Validator
from jsonschema.exceptions import SchemaError

from check_u0_stage_schema import EMBEDDINGS_SCHEMA, JUDGMENTS_SCHEMA


MORIARTY_ROOT = Path(__file__).resolve().parents[1]
DELIVERABLE = Path("deliverables/u0-semantic-contract-2026-09-23")
SCHEMAS = Path("openspec/changes/consolidated-language-kernel/schemas")
RECEIPT = DELIVERABLE / "EXIT-GATE.md"
DECISION = Path("docs/decisions/u0-numeric-profile-decision.md")
ARTIFACT_SCHEMAS = {
    "source-core-embeddings.json": EMBEDDINGS_SCHEMA,
    "judgments.json": JUDGMENTS_SCHEMA,
    "numeric-profile.json": "numeric-profile.schema.json",
    "k-reconciliation.json": "k-reconciliation.schema.json",
    "target-pins.json": "target-pins.schema.json",
    "enforcement-map.json": "enforcement-map.schema.json",
    "trust-premises.json": "trust-premises.schema.json",
    "backend-requirement-matrix.json": "backend-requirement-matrix.schema.json",
}
STAGE_SCHEMA = SCHEMAS / "stage-relation.schema.json"
CHECKER_COMMANDS = (
    ("check_u0_stage_schema.py", ("--root",)),
    ("check_u0_numeric_profile.py", ("--root",)),
    ("check_u0_k_reconciliation.py", ("--root",)),
    ("check_u0_target_pins.py", ("--root",)),
    ("check_u0_enforcement_map.py", ("--root",)),
    ("check_u0_trust_backend_matrix.py", ("--root",)),
    ("build_u0_backend_matrix.py", ("--root", "--check")),
)
ROADMAP_PHRASES = (
    "Versioned source/Core embeddings",
    "stage, intent, effect, authority, history and failure judgments",
    "numeric profile",
    "K-reference reconciliation status",
    "actual compiler/ZKIRv3/verifier/key/ledger pins",
    "field-by-field enforcement map",
    "explicit trust and unresolved interface premises",
    "next ZKIR/recursion requirements",
)
ITEM_NAMES = (
    "Versioned source/Core embeddings",
    "Stage/intent/effect/authority/history/failure judgments",
    "Numeric profile",
    "K-reference reconciliation",
    "Compiler/ZKIRv3/verifier/key/ledger pins",
    "Field-by-field enforcement map",
    "Trust and unresolved interface premises",
    "Next-backend requirement matrix",
)


class FailArgumentParser(argparse.ArgumentParser):
    def error(self, message: str) -> None:
        print(f"FAIL: invalid arguments: {message}")
        raise SystemExit(1)


def parse_args(argv: list[str] | None = None) -> argparse.Namespace:
    parser = FailArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=MORIARTY_ROOT)
    parser.add_argument("--write", action="store_true")
    return parser.parse_args(argv)


def json_path(parts: Any) -> str:
    path = "$"
    for part in parts:
        path += f"[{part}]" if isinstance(part, int) else f".{part}"
    return path


def read_json(path: Path, label: str) -> Any:
    try:
        return json.loads(
            path.read_text(encoding="utf-8"),
            parse_constant=lambda value: (_ for _ in ()).throw(ValueError(value)),
        )
    except (UnicodeDecodeError, json.JSONDecodeError, ValueError) as exc:
        raise ValueError(f"{label} $: invalid JSON or UTF-8: {exc}") from exc


def validate_typed_inputs(root: Path) -> tuple[dict[str, Any], list[str]]:
    """Type all JSON inputs before invoking a checker or dereferencing a field."""
    failures: list[str] = []
    schemas: dict[str, dict[str, Any]] = {}
    stage = read_json(root / STAGE_SCHEMA, STAGE_SCHEMA.as_posix())
    for name, schema_name in ARTIFACT_SCHEMAS.items():
        if isinstance(schema_name, str):
            schema_path = SCHEMAS / schema_name
            schema = read_json(root / schema_path, schema_path.as_posix())
        else:
            schema = schema_name
        schemas[name] = schema
    for label, schema in [(STAGE_SCHEMA.as_posix(), stage), *schemas.items()]:
        try:
            Draft202012Validator.check_schema(schema)
        except SchemaError as exc:
            failures.append(f"FAIL: {label} $: invalid draft 2020-12 schema: {exc.message.splitlines()[0]}")
    if failures:
        return {}, failures
    artifacts: dict[str, Any] = {}
    for name, schema in schemas.items():
        label = (DELIVERABLE / name).as_posix()
        try:
            payload = read_json(root / DELIVERABLE / name, label)
        except ValueError as exc:
            failures.append(f"FAIL: {exc}")
            continue
        artifacts[name] = payload
        for error in sorted(Draft202012Validator(schema).iter_errors(payload), key=lambda e: e.json_path):
            failures.append(f"FAIL: {label} {json_path(error.absolute_path)}: {error.message.splitlines()[0]}")
    return artifacts, failures


def required_paths() -> list[Path]:
    return [
        Path("ROADMAP.md"), DECISION, STAGE_SCHEMA,
        *[SCHEMAS / name for name in ARTIFACT_SCHEMAS.values() if isinstance(name, str)],
        *[DELIVERABLE / name for name in ARTIFACT_SCHEMAS],
        *[Path("scripts") / name for name, _ in CHECKER_COMMANDS],
    ]


def checker_runs(root: Path) -> tuple[list[tuple[str, int, str]], list[str], bool]:
    runs: list[tuple[str, int, str]] = []
    failures: list[str] = []
    blocked = False
    for script, options in CHECKER_COMMANDS:
        args = [sys.executable, f"scripts/{script}", "--root", "."]
        if "--check" in options:
            args.append("--check")
        command = f"python3 scripts/{script} --root ." + (" --check" if "--check" in options else "")
        try:
            result = subprocess.run(args, cwd=root, capture_output=True, text=True, check=False)
        except OSError as exc:
            failures.append(f"FAIL: G1 {command}: {type(exc).__name__}: {exc}")
            runs.append((command, 1, ""))
            continue
        ok = next((line for line in result.stdout.splitlines() if line.startswith("OK:")), "")
        runs.append((command, result.returncode, ok))
        if result.returncode == 2:
            blocked = True
            reason = next((line for line in result.stdout.splitlines() if line.startswith("blocked:")), "blocked: checker input unavailable")
            failures.append(reason)
        elif result.returncode != 0 or not ok:
            detail = " ".join((result.stdout + " " + result.stderr).split())[:240]
            failures.append(f"FAIL: G1 {command} exited {result.returncode} without an OK summary: {detail}")
    return runs, failures, blocked


def consistency_failures(root: Path, data: dict[str, Any]) -> list[str]:
    failures: list[str] = []
    embeddings = data["source-core-embeddings.json"]
    enforcement = data["enforcement-map.json"]
    judgments = data["judgments.json"]
    k_rows = data["k-reconciliation.json"]["rows"]
    numeric = data["numeric-profile.json"]
    pins = data["target-pins.json"]
    matrix = data["backend-requirement-matrix.json"]
    schema_sha = hashlib.sha256((root / STAGE_SCHEMA).read_bytes()).hexdigest()
    if enforcement["stageSchemaSha256"] != embeddings["stageSchemaSha256"] or embeddings["stageSchemaSha256"] != schema_sha:
        failures.append("FAIL: G2 stage schema hashes disagree")
    left = {row["schemaField"] for row in enforcement["rows"]}
    right = {row["schemaField"] for row in embeddings["rows"]}
    if left != right or len(left) != len(enforcement["rows"]) or len(right) != len(embeddings["rows"]):
        failures.append("FAIL: G3 enforcement and embedding leaf row sets differ")
    judgment_keys = [row["key"] for row in judgments["judgments"]]
    expected_judgments = ["stage", "intent", "effect", "authority", "history", "failure"]
    k_judgments = [row["id"] for row in k_rows if row["kind"] == "judgment"]
    k_uni = [row["id"] for row in k_rows if row["kind"] == "uni"]
    expected_uni = [f"UNI-{number:03d}" for number in range(1, 18)]
    if judgment_keys != expected_judgments or k_judgments != judgment_keys or k_uni != expected_uni or len(k_rows) != 23:
        failures.append("FAIL: G4 K judgment or UNI rows differ from the required order")
    decision_sha = hashlib.sha256((root / DECISION).read_bytes()).hexdigest()
    if numeric["decisionSha256"] != decision_sha:
        failures.append("FAIL: G5 numeric decision hash disagrees with the decision file")
    zr01 = next((row for row in matrix["rows"] if row["id"] == "ZR01"), None)
    if pins["compatibleTupleEstablished"] is not False or zr01 is None or zr01["status"] != "specified-only":
        failures.append("FAIL: G6 compatible tuple is not consistently open")
    return failures


def count(rows: list[dict[str, Any]], field: str, value: str) -> int:
    return sum(row[field] == value for row in rows)


def evidence_items(data: dict[str, Any]) -> list[tuple[str, str, bool, str, str]]:
    emb = data["source-core-embeddings.json"]
    judgments = data["judgments.json"]
    numeric = data["numeric-profile.json"]
    k = data["k-reconciliation.json"]
    pins = data["target-pins.json"]
    enforcement = data["enforcement-map.json"]
    trust = data["trust-premises.json"]
    matrix = data["backend-requirement-matrix.json"]
    erows, krows, prows = emb["rows"], k["rows"], numeric["primitives"]
    n_present = count(erows, "realisation", "present")
    n_partial = count(erows, "realisation", "partial")
    n_absent = count(erows, "realisation", "absent")
    k_covered = count(krows, "status", "covered")
    k_partial = count(krows, "status", "partial")
    k_absent = len(krows) - k_covered - k_partial
    conformance_gap = count(prows, "conformance", "open-gap")
    reserve_status = numeric["reserveMechanism"]["status"]
    pin_historical = count(pins["pins"], "status", "historical")
    unenforced = sum(row["status"] != "enforced" for row in enforcement["rows"])
    premise_open = count(trust["premises"], "status", "open")
    specified = count(matrix["rows"], "status", "specified-only")
    rel = lambda *names: ", ".join(f"`{(DELIVERABLE / name).as_posix()}`" for name in names)
    return [
        (ITEM_NAMES[0], rel("source-core-embeddings.json") + f", `{STAGE_SCHEMA.as_posix()}`", n_partial == 0 and n_absent == 0, f"{n_present} present, {n_partial} partial, {n_absent} absent", "Source/Core leaf embeddings are incomplete" if n_partial or n_absent else "—"),
        (ITEM_NAMES[1], rel("judgments.json", "source-core-embeddings.json"), n_partial == 0 and n_absent == 0, f"{len(judgments['judgments'])} judgments; {n_partial} partial, {n_absent} absent embeddings", "Judgments are recorded; source/Core realization remains partial" if n_partial or n_absent else "—"),
        (ITEM_NAMES[2], rel("numeric-profile.json"), conformance_gap == 0 and reserve_status == "present", f"{len(prows)} primitives, {conformance_gap} open gaps; reserve {reserve_status}", "Primitive conformance or reserve posting remains open" if conformance_gap or reserve_status != "present" else "—"),
        (ITEM_NAMES[3], rel("k-reconciliation.json"), k_covered == len(krows), f"{k_covered} covered, {k_partial} partial, {k_absent} not covered", "K reference coverage remains incomplete" if k_covered != len(krows) else "—"),
        (ITEM_NAMES[4], rel("target-pins.json"), pin_historical == 0 and not pins["unresolved"] and pins["compatibleTupleEstablished"], f"{pin_historical} historical, {len(pins['unresolved'])} unresolved", "Historical pins are not reverified; compatible tuple or source alignment is open" if pin_historical or pins["unresolved"] or not pins["compatibleTupleEstablished"] else "—"),
        (ITEM_NAMES[5], rel("enforcement-map.json"), unenforced == 0, f"{len(enforcement['rows']) - unenforced} enforced, {unenforced} unenforced", "Native enforcement is not established for every leaf" if unenforced else "—"),
        (ITEM_NAMES[6], rel("trust-premises.json"), premise_open == 0, f"{len(trust['premises'])} premises, {premise_open} open", "Unresolved trust or interface premises remain" if premise_open else "—"),
        (ITEM_NAMES[7], rel("backend-requirement-matrix.json"), specified == 0, f"{len(matrix['rows'])} rows, {specified} specified-only", "Backend requirements are specified, not demonstrated" if specified else "—"),
    ]


def limitation_lines(data: dict[str, Any]) -> list[str]:
    locations = (
        ("source-core-embeddings.json", ("absenceSearch", "limitation")),
        ("k-reconciliation.json", ("limitation",)),
        ("target-pins.json", ("absenceSearch", "limitation")),
        ("enforcement-map.json", ("limitation",)),
        ("trust-premises.json", ("limitation",)),
    )
    lines: list[str] = []
    for name, keys in locations:
        value = data[name]
        for key in keys:
            value = value[key]
        lines.append(f"- `{(DELIVERABLE / name).as_posix()}`: “{value}”")
    return lines


def render_receipt(items: list[tuple[str, str, bool, str, str]], runs: list[tuple[str, int, str]], data: dict[str, Any]) -> str:
    headline = "U0 contract recorded; capabilities closed" if all(item[2] for item in items) else "U0 contract recorded; capabilities open"
    lines = [
        "# U0 exit gate — 2026-09-23", "", f"**{headline}**", "",
        "| Item | Evidence file(s) | Evidence RECORDED | Capability status | Counts | Open reason |",
        "| --- | --- | --- | --- | --- | --- |",
    ]
    for name, files, closed, counts, reason in items:
        lines.append(f"| {name} | {files} | yes | {'CLOSED' if closed else 'OPEN'} | {counts} | {reason} |")
    lines += ["", "## Checker runs", "", "| Command | Exit code | OK line |", "| --- | --- | --- |"]
    for command, code, ok in runs:
        lines.append(f"| `{command}` | {code} | {ok} |")
    lines += ["", "## Limitations", "", *limitation_lines(data), ""]
    return "\n".join(lines)


def honesty_failures(receipt: str, items: list[tuple[str, str, bool, str, str]]) -> list[str]:
    failures: list[str] = []
    expected_headline = "U0 contract recorded; capabilities closed" if all(item[2] for item in items) else "U0 contract recorded; capabilities open"
    if f"**{expected_headline}**" not in receipt:
        failures.append("FAIL: G7 receipt headline hides an open capability")
    rows = {parts[1].strip(): parts for line in receipt.splitlines() if line.startswith("| ") and (parts := line.split("|")) and len(parts) == 8}
    for name, _, closed, _, _ in items:
        row = rows.get(name)
        expected = "CLOSED" if closed else "OPEN"
        if row is None or row[3].strip() != "yes" or row[4].strip() != expected:
            failures.append(f"FAIL: G7 {name} is not recorded as yes / {expected}")
    return failures


def run(argv: list[str] | None = None) -> int:
    args = parse_args(argv)
    root = args.root.resolve()
    missing = [path for path in required_paths() if not (root / path).is_file()]
    if not args.write and not (root / RECEIPT).is_file():
        missing.append(RECEIPT)
    if missing:
        for path in missing:
            print(f"blocked: missing {path.as_posix()}")
        return 2
    try:
        data, failures = validate_typed_inputs(root)
    except ValueError as exc:
        print(f"FAIL: {exc}")
        return 1
    if failures:
        print("\n".join(failures))
        return 1
    roadmap = (root / "ROADMAP.md").read_text(encoding="utf-8")
    failures = [f"FAIL: G7 ROADMAP U0 evidence item absent: {phrase}" for phrase in ROADMAP_PHRASES if phrase not in roadmap]
    runs, run_failures, blocked = checker_runs(root)
    failures.extend(run_failures)
    failures.extend(consistency_failures(root, data))
    if blocked:
        print("\n".join(failures))
        return 2
    if failures:
        print("\n".join(failures))
        return 1
    items = evidence_items(data)
    rendered = render_receipt(items, runs, data)
    if args.write:
        (root / RECEIPT).write_text(rendered, encoding="utf-8")
    committed = (root / RECEIPT).read_text(encoding="utf-8")
    failures = honesty_failures(committed, items)
    if committed.encode("utf-8") != rendered.encode("utf-8"):
        failures.append("FAIL: G8 EXIT-GATE.md differs from generated text")
    if failures:
        print("\n".join(failures))
        return 1
    print("OK: U0 exit gate G1-G8 passed; contract recorded, capabilities open")
    return 0


def main(argv: list[str] | None = None) -> int:
    try:
        return run(argv)
    except Exception as exc:
        print(f"FAIL: internal error: {type(exc).__name__}: {exc}")
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
