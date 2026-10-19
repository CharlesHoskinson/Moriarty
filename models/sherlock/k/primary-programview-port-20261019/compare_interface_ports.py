#!/usr/bin/env python3
"""Compare complete independent interface mappings, without running the port.

All actual carriers and input-only origin obligations are checked and frozen
before the first independently authored expected body is opened. No evaluator,
K invocation, admission check, child process, or fixture-specific mapping occurs.
"""

import argparse
import hashlib
import json
from pathlib import Path


def pin(path, expected=None):
    path = Path(path)
    raw = path.read_bytes()
    result = {"path": str(path), "bytes": len(raw), "sha256": hashlib.sha256(raw).hexdigest()}
    if expected is not None:
        assert result["bytes"] == expected["bytes"], "byte pin drift: " + str(path)
        assert result["sha256"] == expected["sha256"], "digest pin drift: " + str(path)
    return raw, result


def exact(left, right, path=""):
    """Strict scalar kinds, unordered object key sets, ordered arrays."""
    if type(left) is not type(right):
        return path + ":scalar-or-carrier-kind"
    if type(left) is dict:
        if left.keys() != right.keys():
            return path + ":object-key-set"
        for key in left:
            failure = exact(left[key], right[key], path + "/" + key.replace("~", "~0").replace("/", "~1"))
            if failure is not None:
                return failure
    elif type(left) is list:
        if len(left) != len(right):
            return path + ":array-length"
        for index, (a, b) in enumerate(zip(left, right)):
            failure = exact(a, b, path + "/" + str(index))
            if failure is not None:
                return failure
    elif left != right:
        return path + ":scalar-value"
    return None


def same(left, right, label):
    failure = exact(left, right)
    assert failure is None, label + " mismatch at " + str(failure)


def pointer(value, path):
    assert path == "" or path.startswith("/"), "explicit JSON Pointer required"
    for token in path[1:].split("/") if path else []:
        token = token.replace("~1", "/").replace("~0", "~")
        value = value[int(token)] if type(value) is list else value[token]
    return value


def sliced(rows, spec):
    return rows[spec["start"]:spec["start"] + spec["len"]]


def check_origins(actual, program):
    """Input-only preservation checks; no expression translation is duplicated."""
    origins = actual["origins"]
    assert len(origins["expressions"]) == len(program["expressions"]), "all occurrence origins required"
    for index, origin in enumerate(origins["expressions"]):
        row = program["occurrences"][index]
        assert type(origin["expression_id"]) is int and origin["expression_id"] == index
        assert type(origin["ng_occurrence_id"]) is int and origin["ng_occurrence_id"] == index
        same(origin["source_expression"], program["expressions"][index], "source expression")
        same(origin["occurrence_row"], row, "complete occurrence")
        same(origin["root_first_source_path"], sliced(program["occurrence_paths"], row["path"]), "source path")
        owner = program["functions"][row["function"]] if row["owner_is_function"] else None
        same(origin["owner_function_row"], owner, "function owner")
    for table, key in (("patterns", "pattern_row"), ("constants", "constant_row")):
        assert len(origins[table]) == len(program[table])
        for index, origin in enumerate(origins[table]):
            assert type(origin[key]) is int and origin[key] == index
            same(origin["source_row"], program[table][index], "complete " + table + " row")
    assert len(origins["functions"]) == len(program["functions"])
    for index, origin in enumerate(origins["functions"]):
        row = program["functions"][index]
        assert type(origin["function_row"]) is int and origin["function_row"] == index
        same(origin["source_row"], row, "function row")
        same(origin["ordered_parameter_rows"], sliced(program["parameters"], row["params"]), "ordered parameter rows")
    assert len(origins["declarations"]) == len(program["types"])
    for index, origin in enumerate(origins["declarations"]):
        row = program["types"][index]
        assert type(origin["type_id"]) is int and origin["type_id"] == index
        same(origin["source_row"], row, "type row")
        same(origin["source_type_arguments"], sliced(program["type_args"], row["args"]), "type arguments")
    assert len(origins["constructor_rows"]) == len(program["constructors"])
    for index, origin in enumerate(origins["constructor_rows"]):
        row = program["constructors"][index]
        assert type(origin["ctor_id"]) is int and origin["ctor_id"] == index
        same(origin["source_row"], row, "constructor row")
        fields = program["ctor_fields"][row["field_start"]:row["field_start"] + row["field_len"]]
        same(origin["source_fields"], fields, "constructor fields")
    ordered_arms = []
    for exprid, expression in enumerate(program["expressions"]):
        if "Match" not in expression:
            continue
        spec = expression["Match"]["arms"]
        for local, arm in enumerate(sliced(program["arms"], spec)):
            ordered_arms.append({"match_expr_id": exprid, "local_arm_ordinal": local,
                "global_arm_row": spec["start"] + local, "pattern_row": arm["pattern"], "source_row": arm})
    same(origins["arms"], ordered_arms, "all ordered arm origins")


def create_json(path, value):
    raw = (json.dumps(value, ensure_ascii=False, allow_nan=False, indent=2) + "\n").encode("utf-8")
    with Path(path).open("xb") as stream:
        stream.write(raw)
    return {"path": str(path), "bytes": len(raw), "sha256": hashlib.sha256(raw).hexdigest()}


def compare(manifest):
    cases = manifest["cases"]
    actuals, sources, actual_pins = [], [], []
    for case in cases:
        raw_input, input_pin = pin(case["input"]["path"], case["input"])
        source = json.loads(raw_input)
        raw_actual, actual_pin = pin(case["actual_output"])
        actual = json.loads(raw_actual)
        same(actual["source_document"], source, "ENTIRE source_document")
        same(actual["program_pointer"], case["program_pointer"], "explicit pointer")
        same(actual["source_file"], {"bytes": input_pin["bytes"], "sha256": input_pin["sha256"]}, "source file pin")
        assert actual["partial_counterpart"] is True
        assert set(actual["field_inventory"]["source_program_fields"]) == set(pointer(source, case["program_pointer"]))
        assert len(actual["field_inventory"]["source_program_fields"]) == len(pointer(source, case["program_pointer"]))
        if actual["ng_program"] is not None:
            assert actual["status"] == "ported" and actual["unhandled"] == []
            check_origins(actual, pointer(source, case["program_pointer"]))
        else:
            assert actual["status"] == "unhandled" and actual["unhandled"], "explicit unsupported diagnostic required"
        actuals.append(actual)
        sources.append(source)
        actual_pins.append({"case_id": case["case_id"], "input": input_pin, "actual": actual_pin})
    # This write is exclusive and precedes ANY pin/read/parse of expected bodies.
    frozen = create_json(manifest["actual_freeze_output"], {
        "status": "ALL_INTERFACE_ACTUALS_CHECKED_AND_FROZEN_BEFORE_FIRST_EXPECTED_BODY_READ",
        "actuals": actual_pins, "source_roundtrip": "strict typed whole JSON",
        "origin_checks": "input-only complete rows/slices/order for every emitted NGProgram",
        "expected_body_read_before_this_write": False,
    })
    results = []
    for case, actual, source in zip(cases, actuals, sources):
        raw_expected, expected_pin = pin(case["expected"]["path"], case["expected"])
        expected = json.loads(raw_expected)
        failure = exact(actual["ng_program"], expected["ng_program"], "/ng_program")
        if expected["ng_program"] is None:
            same(pointer(source, expected["required_unsupported_input_pointer"]), expected["unsupported_variant"], "unsupported input operator")
            assert any(issue.get("code", "").startswith("unsupported") for issue in actual["unhandled"]), "unsupported-form diagnostic required"
        results.append({"case_id": case["case_id"], "complete_ng_program_typed_equal": failure is None,
            "mismatch_path_and_kind": failure, "expected_pin": expected_pin,
            "whole_source_roundtrip_equal": True,
            "origins_input_only_checked": actual["ng_program"] is not None,
            "exact_diagnostic_and_origin_golden_comparison": "UNBOUND; no private oracle was filled from implementation"})
    result = {"status": "PASS" if all(row["complete_ng_program_typed_equal"] for row in results) else "FAIL",
        "scope": "Interface-only independently hand-authored inputs; NOT admitted compiler/runtime/model evidence",
        "comparison": "strict scalar kinds and values, ordered arrays, unordered object key sets; no projection of ng_program",
        "actual_freeze": frozen, "results": results}
    create_json(manifest["comparison_output"], result)
    print(json.dumps(result))
    return 0 if result["status"] == "PASS" else 1


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("manifest", type=Path)
    args = parser.parse_args()
    return compare(json.loads(args.manifest.read_text()))


if __name__ == "__main__":
    raise SystemExit(main())
