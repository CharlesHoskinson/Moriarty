"""Decode every complete actual wire before opening independent golden files.

The allObservedStateOrdinals policy derives observerCuts from actual state
count. No expected field supplies actual states, metadata, cuts or transitions.
This data consumer launches no child and imports no model evaluator.
"""

from pathlib import Path
from typing import Any
import hashlib
import json
import sys

import nominal_trace_codec as codec
from complete_k_wire import CompleteKWire


OBSERVER_CUT_POLICY = "allObservedStateOrdinals"


def first_difference(actual: Any, expected: Any, path: str = "$") -> str | None:
    if type(actual) is not type(expected):
        return path + ": type"
    if isinstance(expected, dict):
        if set(actual) != set(expected):
            return path + ": fields"
        for key in expected:
            difference = first_difference(actual[key], expected[key], path + "." + key)
            if difference is not None:
                return difference
    elif isinstance(expected, list):
        if len(actual) != len(expected):
            return path + ": length"
        for index, (left, right) in enumerate(zip(actual, expected)):
            difference = first_difference(left, right, path + "[" + str(index) + "]")
            if difference is not None:
                return difference
    elif actual != expected:
        return path + ": value"
    return None


def encoded(value: Any) -> bytes:
    return (json.dumps(value, sort_keys=True, ensure_ascii=False, indent=2) + "\n").encode()


def persist(path: str, value: Any) -> dict:
    payload = encoded(value)
    destination = Path(path)
    with destination.open("xb") as stream:
        stream.write(payload)
    return {
        "path": str(destination),
        "bytes": len(payload),
        "sha256": hashlib.sha256(payload).hexdigest(),
    }


def compare_manifest(manifest_path: str) -> tuple[int, dict]:
    manifest = json.loads(Path(manifest_path).read_bytes())
    if manifest["observer_cut_policy"] != OBSERVER_CUT_POLICY:
        raise ValueError("Unselected observer ordinal policy")
    codec.configure(manifest["api"], manifest["syntax"])
    adapter = CompleteKWire(codec.API, codec.REG)
    records = []
    actuals = []
    errors = (ValueError, KeyError, IndexError, TypeError, OSError, UnicodeError, RecursionError, AssertionError)

    # Complete and persist ALL actual wires before any golden is opened.
    for case in manifest["cases"]:
        row = {"case_id": case["case_id"], "complete_wire_equal": False}
        records.append(row)
        try:
            raw_print = Path(case["actual_print"]).read_bytes()
            raw_kore = Path(case["actual_kore"]).read_bytes()
            tape = codec.parse_printed(raw_print.decode("utf-8"))
            states = adapter.complete_states(tape)
            wire = adapter.bind_shared_wire(
                states,
                case_id=case["case_id"],
                profile=manifest["profile"],
                terminal_step_contract=manifest["terminalStepContract"],
                observer_cuts=list(range(len(states))),
            )
            wire_pin = persist(case["actual_shared_wire"], wire)
            row.update({
                "actual_print_sha256": hashlib.sha256(raw_print).hexdigest(),
                "actual_print_bytes": len(raw_print),
                "actual_kore_sha256": hashlib.sha256(raw_kore).hexdigest(),
                "actual_kore_bytes": len(raw_kore),
                "actual_shared_wire": wire_pin,
                "all_actual_states_decoded_without_expected": True,
                "observerCuts_source": OBSERVER_CUT_POLICY,
                "observerCuts_evidence": "all actual observed state ordinals; no separate budget-cut/resume calls",
            })
            actuals.append((case, row, wire, wire_pin))
        except errors as error:
            row["actual_decode_error"] = type(error).__name__ + ": " + str(error)

    if len(actuals) != len(manifest["cases"]):
        return 1, {
            "kind": "COMPLETE_ACTUAL_K_WIRE_VS_INDEPENDENT_FULL_STATE",
            "all_actual_decoded_before_expected_read": False,
            "expected_documents_opened": 0,
            "all_cases_equal": False,
            "cases": records,
        }

    expected_opened = 0
    for case, row, wire, wire_pin in actuals:
        try:
            expected_bytes = Path(case["expected_wire"]).read_bytes()
            expected_opened += 1
            expected_sha = hashlib.sha256(expected_bytes).hexdigest()
            if expected_sha != case["expected_wire_sha256"]:
                raise ValueError("Independent shared-wire pin changed")
            if len(expected_bytes) != case["expected_wire_bytes"]:
                raise ValueError("Independent shared-wire byte count changed")
            expected = json.loads(expected_bytes)
            # The entire actual wire was derived and persisted independently.
            if hashlib.sha256(encoded(wire)).hexdigest() != wire_pin["sha256"]:
                raise ValueError("Actual wire changed after golden read")
            difference = first_difference(wire, expected)
            row.update({
                "expected_wire_sha256": expected_sha,
                "complete_wire_equal": difference is None,
                "first_difference": difference,
            })
        except errors as error:
            row["comparison_error"] = type(error).__name__ + ": " + str(error)

    success = len(records) == len(manifest["cases"]) and all(row["complete_wire_equal"] for row in records)
    return (0 if success else 1), {
        "kind": "COMPLETE_ACTUAL_K_WIRE_VS_INDEPENDENT_FULL_STATE",
        "all_actual_decoded_before_expected_read": True,
        "expected_documents_opened": expected_opened,
        "all_cases_equal": success,
        "comparison": "full Program/control/current+saved maps/ordered frames/values/source IDs/occurrences/all cumulative events/initial/everyafter/final; exact JSON scalar types",
        "observer_cut_policy": OBSERVER_CUT_POLICY,
        "clock": "K uncharged observation ordinals only; no A/B/Quint/fuel/helperWork equivalence",
        "cases": records,
    }


if __name__ == "__main__":
    exit_code, report = compare_manifest(sys.argv[1])
    print(json.dumps(report, indent=2), flush=True)
    raise SystemExit(exit_code)
