from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
MATRIX = ROOT / "evidence" / "moriarty-goal-completion-matrix-2026-09-03.csv"


def test_goal_completion_matrix_is_complete_and_machine_readable() -> None:
    with MATRIX.open(newline="", encoding="utf-8") as handle:
        rows = list(csv.DictReader(handle))

    assert list(rows[0]) == [
        "kind",
        "id",
        "title",
        "status",
        "evidence",
        "disposition",
    ]
    assert len(rows) == 60
    assert {row["id"] for row in rows if row["kind"] == "workstream"} == {
        f"W{number}" for number in range(1, 13)
    }
    assert {row["id"] for row in rows if row["kind"] == "experiment"} == {
        f"E{number:02d}" for number in range(26)
    }
    assert {row["id"] for row in rows if row["kind"] == "deliverable"} == {
        f"D{number:02d}" for number in range(1, 23)
    }
    assert {row["status"] for row in rows} <= {
        "implemented",
        "not-started",
        "reproduced",
        "specified-only",
    }
    assert all(row["evidence"] and row["disposition"] for row in rows)
