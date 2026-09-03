#!/usr/bin/env python3
"""Generate the Moriarty E00 Compact and translation evidence."""

from __future__ import annotations

import json
from pathlib import Path

from moriarty.certificate import write_experiment
from moriarty.swap import SwapParameters


def main() -> None:
    output = Path(__file__).resolve().parent
    certificate = write_experiment(output, SwapParameters.example(), minimum_traces=1_000)
    print(json.dumps(certificate, indent=2, sort_keys=True))
    if not certificate["stop_test_passed"]:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
