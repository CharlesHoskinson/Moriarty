"""Structural ITF export only; no candidate or reference semantic evaluation."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]
BASE_PINS = frozenset({"moriarty/core.py", "moriarty/swap.py"} | {
    f"specs/quint/s02/{name}.qnt" for name in (
        "candidate_a_core", "candidate_a_types", "candidate_a_programs",
        "candidate_a_projection", "effects", "observations")})
GENERATORS = {
    "swapTrace": ("specs/quint/s02/candidate_a_harness.qnt", "SwapComputedA", "canonical-swap-v1"),
    "installmentTrace": ("specs/quint/s02/candidate_a_installment_harness.qnt", "InstallmentComputedA", "installment-two-when-v1"),
    "diagnosticCases": ("specs/quint/s02/candidate_a_cases.qnt", "CaseComputedA", None),
}
OPTIONAL_PINS = frozenset({value[0] for value in GENERATORS.values()} | {"scripts/export_s02_candidate_a_cases.py"})
FIXTURE_IDS = frozenset({"canonical-swap-v1", "installment-two-when-v1", "close-v1",
    "pay-zero-v1", "pay-negative-v1", "pay-ten-v1", "if-settle-zero-v1",
    "deposit-five-close-v1", "deposit-zero-close-v1", "pre-deposit-post-v1",
    "pre-warning-deposit-v1", "ordered-choice-v1", "ordered-choice-overlap-v1"})


class ExportError(ValueError):
    """A structural or source-binding error; never a Core result."""


def export_cases(itf_dir: Path, source_root: Path = ROOT) -> dict:
    return {"schema_version": 1, "source_pins": {}, "cases": []}


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--itf-dir", required=True, type=Path)
    parser.add_argument("--out", required=True, type=Path)
    parser.add_argument("--source-root", type=Path, default=ROOT)
    args = parser.parse_args(argv)
    try:
        document = export_cases(args.itf_dir, args.source_root)
        with args.out.open("x", encoding="utf-8") as stream:
            json.dump(document, stream, ensure_ascii=False, indent=2)
            stream.write("\n")
    except (ExportError, OSError) as exc:
        print(json.dumps({"status": "error", "error": str(exc)}), file=sys.stderr)
        return 1
    print(json.dumps({"status": "exported", "cases": len(document["cases"]), "out": str(args.out)}))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
