#!/usr/bin/env python3
"""Generate the Moriarty E00 Compact and translation evidence."""

from __future__ import annotations

import json
from pathlib import Path

from moriarty.certificate import write_experiment
from moriarty.compact import disclosure_negative_control
from moriarty.swap import SwapParameters


def main() -> None:
    output = Path(__file__).resolve().parent
    certificate = write_experiment(output, SwapParameters.example(), minimum_traces=1_000)
    source = (output / "swap.compact").read_text(encoding="utf-8")
    negative_directory = output / "negative"
    negative_directory.mkdir(parents=True, exist_ok=True)
    (negative_directory / "undisclosed-decision.compact").write_text(
        disclosure_negative_control(source),
        encoding="utf-8",
    )
    print(json.dumps(certificate, indent=2, sort_keys=True))
    if not certificate["stop_test_passed"]:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
