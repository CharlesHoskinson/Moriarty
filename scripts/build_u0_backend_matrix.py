#!/usr/bin/env python3
"""Parse the backend requirements markdown into the U0 matrix JSON."""

from __future__ import annotations

import argparse
import hashlib
import json
import re
from pathlib import Path
from typing import Any


MORIARTY_ROOT = Path(__file__).resolve().parents[1]
SOURCE_REL = "docs/MORIARTY-BACKEND-REQUIREMENTS.md"
MATRIX_REL = "deliverables/u0-semantic-contract-2026-09-23/backend-requirement-matrix.json"
SCHEMA_VERSION = "moriarty-u0-backend-matrix/1"

CAPABILITY_HEADER = (
    "| ID and class | Concrete SHALL proposal | Acceptance test/evidence | "
    "Present-support boundary |"
)
RESPONSIBILITY_HEADER = (
    "| Requirements | Primary implementation owner | "
    "Moriarty responsibility / milestone |"
)
REFINEMENT_HEADER = (
    "| ID | Requirement | Refines | Positive and hostile evidence | Source basis |"
)

ZR_IDS = [f"ZR{number:02d}" for number in range(1, 17)]
MNR_IDS = [f"MNR{number:02d}" for number in range(1, 9)]
EXPECTED_IDS = ZR_IDS + MNR_IDS

ROW_ID = re.compile(r"^(ZR\d{2}|MNR\d{2}) \u2014 (.+)$")
CLASS_PREFIX = re.compile(r"^([A-Z](?:/[A-Z])*), .+$")
ID_RANGE = re.compile(
    r"^(ZR|MNR)(\d{2})\s*[\u2013\u2014-]\s*(ZR|MNR)(\d{2})$"
)
ID_SINGLE = re.compile(r"^(ZR|MNR)(\d{2})$")
SEPARATOR_CELL = re.compile(r"^:?-{3,}:?$")


class TableFormatError(Exception):
    """The requirements markdown no longer matches the parsed tables."""

    def __init__(self, message: str) -> None:
        super().__init__(message)
        self.message = message


def split_cells(line: str) -> list[str]:
    """Split a markdown row on unescaped pipes.

    A pipe is escaped only when an odd number of backslashes precedes it.
    An even number leaves the pipe as a column delimiter. Each pair of
    backslashes contributes one literal backslash to the cell text.
    """

    cells: list[str] = []
    current: list[str] = []
    index = 0
    length = len(line)
    while index < length:
        char = line[index]
        if char != "\\":
            if char == "|":
                cells.append("".join(current).strip())
                current = []
            else:
                current.append(char)
            index += 1
            continue
        count = 0
        while index < length and line[index] == "\\":
            count += 1
            index += 1
        if index < length and line[index] == "|":
            pairs, escaped = divmod(count, 2)
            current.extend("\\" * pairs)
            index += 1
            if escaped:
                current.append("|")
                continue
            cells.append("".join(current).strip())
            current = []
            continue
        current.extend("\\" * count)
    cells.append("".join(current).strip())
    if cells and cells[0] == "":
        cells.pop(0)
    if cells and cells[-1] == "":
        cells.pop()
    return cells


def extract_table(lines: list[str], header: str) -> list[list[str]]:
    if lines.count(header) != 1:
        raise TableFormatError(f"FAIL: table format changed: {header}")
    start = lines.index(header)
    expected_columns = len(split_cells(header))
    rows: list[list[str]] = []
    for line in lines[start + 1 :]:
        if not line.startswith("|"):
            break
        cells = split_cells(line)
        if len(cells) != expected_columns:
            raise TableFormatError(f"FAIL: table format changed: {header}")
        if all(SEPARATOR_CELL.fullmatch(cell) for cell in cells):
            continue
        rows.append(cells)
    return rows


def expand_id_list(cell: str) -> list[str]:
    found: list[str] = []
    for raw in cell.split(","):
        token = raw.strip()
        range_match = ID_RANGE.fullmatch(token)
        single_match = ID_SINGLE.fullmatch(token)
        if range_match:
            start_family, start_text, end_family, end_text = range_match.groups()
            if start_family != end_family:
                raise TableFormatError(f"FAIL: owner id range crosses families: {cell}")
            start = int(start_text)
            end = int(end_text)
            if end < start:
                raise TableFormatError(f"FAIL: owner id range is reversed: {cell}")
            found.extend(f"{start_family}{number:02d}" for number in range(start, end + 1))
        elif single_match:
            family, number = single_match.groups()
            found.append(f"{family}{number}")
        else:
            raise TableFormatError(f"FAIL: owner id list: {cell}")
    if len(found) != len(set(found)):
        raise TableFormatError(f"FAIL: duplicate owner id: {cell}")
    return found


def expand_refines(cell: str, row_id: str) -> list[str]:
    parts = [part.strip() for part in cell.split("/")]
    if not parts or any(part == "" for part in parts):
        raise TableFormatError(f"FAIL: refines format: {row_id}: {cell}")
    expanded: list[str] = []
    for index, part in enumerate(parts):
        if re.fullmatch(r"ZR\d{2}", part):
            expanded.append(part)
        elif index > 0 and re.fullmatch(r"\d{2}", part):
            expanded.append(f"ZR{part}")
        else:
            raise TableFormatError(f"FAIL: refines format: {row_id}: {cell}")
    if len(expanded) != len(set(expanded)):
        raise TableFormatError(f"FAIL: duplicate refines: {row_id}")
    return expanded


def id_sort_key(row_id: str) -> tuple[int, int]:
    if row_id.startswith("ZR"):
        return (0, int(row_id[2:]))
    if row_id.startswith("MNR"):
        return (1, int(row_id[3:]))
    raise TableFormatError(f"FAIL: unrecognized requirement id: {row_id}")


def parse_identity(cell: str) -> tuple[str, str, str]:
    match = ROW_ID.fullmatch(cell)
    if not match:
        raise TableFormatError(f"FAIL: unrecognized requirement row: {cell}")
    row_id, title = match.group(1), match.group(2)
    family = "ZR" if row_id.startswith("ZR") else "MNR"
    return row_id, family, title


def owner_index(rows: list[list[str]]) -> dict[str, tuple[str, str]]:
    owners: dict[str, tuple[str, str]] = {}
    for requirements, owner, milestone in rows:
        if owner == "" or milestone == "":
            raise TableFormatError(f"FAIL: empty owner or milestone: {requirements}")
        for row_id in expand_id_list(requirements):
            if row_id in owners:
                raise TableFormatError(f"FAIL: duplicate owner for {row_id}")
            owners[row_id] = (owner, milestone)
    return owners


def capability_row(cells: list[str], owners: dict[str, tuple[str, str]]) -> dict[str, Any]:
    row_id, family, title = parse_identity(cells[0])
    if family != "ZR":
        raise TableFormatError(f"FAIL: capability table contains {row_id}")
    class_match = CLASS_PREFIX.fullmatch(title)
    if not class_match:
        raise TableFormatError(f"FAIL: unrecognized class: {row_id}: {title}")
    requirement, acceptance, present_support = cells[1], cells[2], cells[3]
    if "SHALL" not in requirement or acceptance == "" or present_support == "":
        raise TableFormatError(f"FAIL: incomplete capability row: {row_id}")
    if row_id not in owners:
        raise TableFormatError(f"FAIL: no owner for {row_id}")
    owner, milestone = owners[row_id]
    return {
        "id": row_id,
        "family": family,
        "title": title,
        "class": class_match.group(1),
        "requirement": requirement,
        "acceptance": acceptance,
        "presentSupport": present_support,
        "refines": [],
        "owner": owner,
        "milestone": milestone,
        "status": "specified-only",
    }


def refinement_row(cells: list[str], owners: dict[str, tuple[str, str]]) -> dict[str, Any]:
    row_id, family, title = parse_identity(cells[0])
    if family != "MNR":
        raise TableFormatError(f"FAIL: refinement table contains {row_id}")
    requirement, acceptance = cells[1], cells[3]
    if "SHALL" not in requirement or acceptance == "":
        raise TableFormatError(f"FAIL: incomplete refinement row: {row_id}")
    assigned = owners.get(row_id)
    owner, milestone = assigned if assigned is not None else (None, None)
    return {
        "id": row_id,
        "family": family,
        "title": title,
        "class": None,
        "requirement": requirement,
        "acceptance": acceptance,
        "presentSupport": None,
        "refines": expand_refines(cells[2], row_id),
        "owner": owner,
        "milestone": milestone,
        "status": "specified-only",
    }


def build_matrix(source_bytes: bytes) -> dict[str, Any]:
    try:
        text = source_bytes.decode("utf-8")
    except UnicodeDecodeError as exc:
        raise TableFormatError("FAIL: source is not UTF-8") from exc
    lines = text.splitlines()
    owners = owner_index(extract_table(lines, RESPONSIBILITY_HEADER))
    rows = [
        capability_row(cells, owners)
        for cells in extract_table(lines, CAPABILITY_HEADER)
    ]
    rows.extend(
        refinement_row(cells, owners)
        for cells in extract_table(lines, REFINEMENT_HEADER)
    )
    rows.sort(key=lambda row: id_sort_key(str(row["id"])))
    actual_ids = [str(row["id"]) for row in rows]
    if len(actual_ids) != len(set(actual_ids)):
        raise TableFormatError("FAIL: duplicate requirement id")
    if actual_ids != EXPECTED_IDS:
        raise TableFormatError(
            "FAIL: id set is not exactly ZR01-ZR16 and MNR01-MNR08"
        )
    extra_owner_ids = [row_id for row_id in owners if row_id not in set(actual_ids)]
    if extra_owner_ids:
        names = ", ".join(sorted(extra_owner_ids, key=id_sort_key))
        raise TableFormatError(
            f"FAIL: responsibility table ids outside requirement set: {names}"
        )
    zr_ids = set(ZR_IDS)
    for row in rows:
        if row["family"] != "MNR":
            continue
        outside = [refined for refined in row["refines"] if refined not in zr_ids]
        if outside:
            names = ", ".join(outside)
            raise TableFormatError(
                f"FAIL: refines id outside ZR01-ZR16: {row['id']}: {names}"
            )
    return {
        "schemaVersion": SCHEMA_VERSION,
        "source": SOURCE_REL,
        "sourceSha256": hashlib.sha256(source_bytes).hexdigest(),
        "rows": rows,
    }


def canonical_matrix(payload: dict[str, Any]) -> str:
    return json.dumps(payload, indent=2, ensure_ascii=False) + "\n"


def parse_args(argv: list[str] | None = None) -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Build the U0 backend requirement matrix from the source markdown."
    )
    parser.add_argument("--root", type=Path, default=MORIARTY_ROOT)
    parser.add_argument(
        "--check",
        action="store_true",
        help="Compare the parsed matrix with the committed file and do not write.",
    )
    return parser.parse_args(argv)


def run(argv: list[str] | None = None) -> int:
    args = parse_args(argv)
    root = args.root.resolve()
    source = root / SOURCE_REL
    target = root / MATRIX_REL
    if not source.is_file():
        print(f"blocked: missing {SOURCE_REL}")
        return 2
    if args.check and not target.is_file():
        print(f"blocked: missing {MATRIX_REL}")
        return 2
    try:
        payload = build_matrix(source.read_bytes())
    except TableFormatError as exc:
        print(exc.message)
        return 1
    rendered = canonical_matrix(payload).encode("utf-8")
    if args.check:
        if target.read_bytes() != rendered:
            print("FAIL: backend matrix drifted")
            return 1
        print("OK: backend matrix matches source")
        return 0
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_bytes(rendered)
    print(f"Wrote {MATRIX_REL}")
    return 0


def main(argv: list[str] | None = None) -> int:
    try:
        return run(argv)
    except Exception as exc:
        print(f"FAIL: internal error: {type(exc).__name__}: {exc}")
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
