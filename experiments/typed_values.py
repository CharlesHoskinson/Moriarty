#!/usr/bin/env python3
"""A minimal Marlowe V2 kind-checking experiment with reproducible cases."""

from __future__ import annotations

import json
from dataclasses import dataclass
from pathlib import Path


@dataclass(frozen=True)
class Kind:
    family: str
    unit: str | None = None


def add(left: Kind, right: Kind) -> Kind:
    if left == right and left.family in {"amount", "duration", "integer", "ratio"}:
        return left
    if left.family == "timestamp" and right.family == "duration":
        return left
    if left.family == "duration" and right.family == "timestamp":
        return right
    raise TypeError(f"cannot add {left} and {right}")


def run_case(name: str, left: Kind, right: Kind, should_pass: bool) -> dict[str, object]:
    try:
        inferred = add(left, right)
        actual = {"accepted": True, "result": str(inferred)}
    except TypeError as error:
        actual = {"accepted": False, "diagnostic": str(error)}
    actual["name"] = name
    actual["expected_acceptance"] = should_pass
    actual["test_passed"] = actual["accepted"] is should_pass
    return actual


def main() -> None:
    ada = Kind("amount", "ADA")
    usd = Kind("amount", "USD")
    timestamp = Kind("timestamp", "POSIX-ms")
    duration = Kind("duration", "ms")
    cases = [
        run_case("same-token addition", ada, ada, True),
        run_case("cross-token addition", ada, usd, False),
        run_case("amount-duration addition", ada, duration, False),
        run_case("timestamp-duration addition", timestamp, duration, True),
        run_case("timestamp-amount addition", timestamp, ada, False),
    ]
    result = {
        "scope": "Authoring/elaboration kind check only; this is not a Core 2 implementation.",
        "all_tests_passed": all(case["test_passed"] for case in cases),
        "cases": cases,
    }
    output = Path(__file__).with_name("typed-values-results-2026-09-02.json")
    output.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    print(json.dumps(result, indent=2, sort_keys=True))
    if not result["all_tests_passed"]:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
