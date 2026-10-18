"""Decode only the five reserve lifecycle Observation states in Quint 0.32.0 ITF.

This module does not execute a model, generate trace states, or authenticate input
provenance. The caller supplies a captured file and an independent expectation.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import re
import sys
from pathlib import Path
from typing import Any

STATE_COUNT = 5
NORMAL_MAX = 142
PENDING_FIELDS = {
    "specimen", "balances", "programState", "dutyPresent", "dutyAmount",
    "site", "captures", "committed",
}
INVOCATION_FIELDS = {
    "status", "attempted", "advanced", "hasSegment", "newContinuation",
    "effects", "sourceSteps", "errorSteps",
}
STATUSES = {"NotInvoked", "Waiting", "Complete", "BusinessRejected"}


class DecodeError(ValueError):
    """The input does not have the explicitly supported normal ITF shape."""


def fail(path: str, message: str) -> None:
    raise DecodeError(f"{path}: {message}")


def record(value: Any, fields: set[str], path: str) -> dict[str, Any]:
    if type(value) is not dict:
        fail(path, "expected an object")
    if set(value) != fields:
        fail(path, f"expected fields {sorted(fields)}, found {sorted(value)}")
    return value


def array(value: Any, path: str) -> list[Any]:
    if type(value) is not list:
        fail(path, "expected an array")
    return value


def string(value: Any, path: str) -> str:
    if type(value) is not str:
        fail(path, "expected a string")
    return value


def boolean(value: Any, path: str) -> bool:
    if type(value) is not bool:
        fail(path, "expected a boolean")
    return value


def integer(value: Any, path: str) -> int:
    wrapped = record(value, {"#bigint"}, path)
    decimal = string(wrapped["#bigint"], path + "/#bigint")
    if re.fullmatch(r"0|[1-9][0-9]*", decimal) is None:
        fail(path, "expected a canonical nonnegative normal-example integer")
    decoded = int(decimal)
    if decoded > NORMAL_MAX:
        fail(path, "outside the disclosed common adapter's normal 0..142 domain")
    return decoded


def integers(value: Any, path: str) -> list[int]:
    return [integer(item, f"{path}/{i}") for i, item in enumerate(array(value, path))]


def unit_variant(value: Any, tags: set[str], path: str) -> str:
    variant = record(value, {"tag", "value"}, path)
    tag = string(variant["tag"], path + "/tag")
    if tag not in tags:
        fail(path, f"expected a declared unit constructor in {sorted(tags)}")
    # ToIrListener.unitValue is Tup([]); itf.toItf maps it to {"#tup":[]}.
    payload = record(variant["value"], {"#tup"}, path + "/value")
    if array(payload["#tup"], path + "/value/#tup"):
        fail(path, "a nullary constructor must retain its empty-tuple payload")
    return tag


def integer_map(value: Any, path: str) -> dict[str, int]:
    entries = array(record(value, {"#map"}, path)["#map"], path + "/#map")
    result: dict[str, int] = {}
    for i, item in enumerate(entries):
        pair = array(item, f"{path}/#map/{i}")
        if len(pair) != 2:
            fail(f"{path}/#map/{i}", "expected one key/value pair")
        key = string(pair[0], f"{path}/#map/{i}/0")
        if key in result:
            fail(path, f"duplicate map key {key!r}")
        result[key] = integer(pair[1], f"{path}/#map/{i}/1")
    return result


def effect(value: Any, path: str) -> dict[str, Any]:
    variant = record(value, {"tag", "value"}, path)
    tag = string(variant["tag"], path + "/tag")
    payload = variant["value"]
    if tag == "StateValue":
        data = record(payload, {"name", "before", "after"}, path + "/value")
        return {"tag": tag, "name": string(data["name"], path + "/value/name"),
                "before": integer(data["before"], path + "/value/before"),
                "after": integer(data["after"], path + "/value/after")}
    if tag == "DutyValue":
        data = record(payload, {"before", "after"}, path + "/value")
        return {"tag": tag,
                "before": integer(data["before"], path + "/value/before"),
                "after": integer(data["after"], path + "/value/after")}
    if tag in {"TransferAmount", "FeeAmount"}:
        return {"tag": tag, "amount": integer(payload, path + "/value")}
    if tag == "EmitQuote":
        return {"tag": tag, "quote": integer(payload, path + "/value")}
    fail(path, f"unsupported ResumeEffect constructor {tag!r}")


def pending(value: Any, path: str) -> dict[str, Any]:
    data = record(value, PENDING_FIELDS, path)
    return {
        "specimen": unit_variant(data["specimen"], {"Reserve"}, path + "/specimen"),
        "balances": integer_map(data["balances"], path + "/balances"),
        "programState": integer_map(data["programState"], path + "/programState"),
        "dutyPresent": boolean(data["dutyPresent"], path + "/dutyPresent"),
        "dutyAmount": integer(data["dutyAmount"], path + "/dutyAmount"),
        "site": integer(data["site"], path + "/site"),
        "captures": integers(data["captures"], path + "/captures"),
        "committed": integer(data["committed"], path + "/committed"),
    }


def invocation(value: Any, path: str) -> dict[str, Any]:
    data = record(value, INVOCATION_FIELDS, path)
    return {
        "status": unit_variant(data["status"], STATUSES, path + "/status"),
        "attempted": integer(data["attempted"], path + "/attempted"),
        "advanced": boolean(data["advanced"], path + "/advanced"),
        "hasSegment": boolean(data["hasSegment"], path + "/hasSegment"),
        "newContinuation": boolean(data["newContinuation"], path + "/newContinuation"),
        "effects": [effect(item, f"{path}/effects/{i}")
                    for i, item in enumerate(array(data["effects"], path + "/effects"))],
        "sourceSteps": integers(data["sourceSteps"], path + "/sourceSteps"),
        "errorSteps": integers(data["errorSteps"], path + "/errorSteps"),
    }


def observation(value: Any, path: str) -> dict[str, Any]:
    data = record(value, {"c", "last", "attemptedTotal"}, path)
    return {"c": pending(data["c"], path + "/c"),
            "last": invocation(data["last"], path + "/last"),
            "attemptedTotal": integer(data["attemptedTotal"], path + "/attemptedTotal")}


def decode_trace(document: Any, variable: str) -> list[dict[str, Any]]:
    """Return exactly the captured Observation sequence; invent no other frames."""
    data = record(document, {"#meta", "vars", "states"}, "ITF")
    meta = data["#meta"]
    if type(meta) is not dict or meta.get("format") != "ITF" or meta.get("status") != "passed":
        fail("ITF/#meta", "expected CLI ITF metadata with status=passed")
    if array(data["vars"], "ITF/vars") != [variable]:
        fail("ITF/vars", "expected exactly the explicitly named observation variable")
    states = array(data["states"], "ITF/states")
    if len(states) != STATE_COUNT:
        fail("ITF/states", "expected exactly five captured lifecycle states")
    decoded = []
    for i, state in enumerate(states):
        state = record(state, {"#meta", variable}, f"ITF/states/{i}")
        state_meta = record(state["#meta"], {"index"}, f"ITF/states/{i}/#meta")
        if type(state_meta["index"]) is not int or state_meta["index"] != i:
            fail(f"ITF/states/{i}/#meta/index", "expected the original ordered state index")
        decoded.append(observation(state[variable], f"ITF/states/{i}/{variable}"))
    return decoded


def expected_observations(value: Any) -> list[Any]:
    """Select only the expected observations, explicitly excluding pure frames."""
    if type(value) is dict:
        if not set(value).issubset({"initial", "prefix", "observations"}) or "observations" not in value:
            fail("expected", "expected the documented common projection wrapper")
        value = value["observations"]
    result = array(value, "expected/observations")
    if len(result) != STATE_COUNT:
        fail("expected/observations", "expected exactly five frozen observations")
    return result


def pointer(path: str, key: str | int) -> str:
    return path + "/" + str(key).replace("~", "~0").replace("/", "~1")


def first_difference(actual: Any, expected: Any, path: str = "") -> dict[str, Any] | None:
    # Python considers True == 1; shared JSON comparison must retain type equality.
    if type(actual) is not type(expected):
        return {"path": path, "reason": "JSON types differ",
                "actual_type": type(actual).__name__, "expected_type": type(expected).__name__}
    if type(actual) is dict:
        if set(actual) != set(expected):
            return {"path": path, "reason": "record/map keys differ",
                    "actual_keys": sorted(actual), "expected_keys": sorted(expected)}
        for key in sorted(actual):
            mismatch = first_difference(actual[key], expected[key], pointer(path, key))
            if mismatch is not None:
                return mismatch
    elif type(actual) is list:
        if len(actual) != len(expected):
            return {"path": path, "reason": "array lengths differ",
                    "actual_length": len(actual), "expected_length": len(expected)}
        for i, (left, right) in enumerate(zip(actual, expected)):
            mismatch = first_difference(left, right, pointer(path, i))
            if mismatch is not None:
                return mismatch
    elif actual != expected:
        return {"path": path, "reason": "values differ", "actual": actual, "expected": expected}
    return None


def unique_object(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for key, value in pairs:
        if key in result:
            raise DecodeError(f"duplicate JSON object key {key!r}")
        result[key] = value
    return result


def invalid_constant(value: str) -> None:
    raise DecodeError(f"non-JSON numeric constant {value}")


def read_document(path: Path) -> tuple[Any, str]:
    raw = path.read_bytes()
    return (json.loads(raw.decode("utf-8"), object_pairs_hook=unique_object,
                       parse_constant=invalid_constant), hashlib.sha256(raw).hexdigest())


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--input", type=Path, required=True, help="actual captured ITF file")
    parser.add_argument("--variable", required=True, help="exact declared ITF variable name")
    parser.add_argument("--expected", type=Path, required=True,
                        help="separately frozen common projection or five-observation array")
    parser.add_argument("--output", type=Path, required=True,
                        help="new file for decoded input observations; never initial/prefix")
    args = parser.parse_args(argv)
    try:
        document, input_hash = read_document(args.input)
        reference, expected_hash = read_document(args.expected)
        decoded = decode_trace(document, args.variable)
        expected = expected_observations(reference)
        mismatch = first_difference(decoded, expected, "/observations")
        output = (json.dumps({"observations": decoded}, indent=2, sort_keys=True) + "\n").encode()
        with args.output.open("xb") as stream:
            stream.write(output)
        receipt = {"status": "matched" if mismatch is None else "mismatch",
                   "scope": "five captured lifecycle Observation records only",
                   "variable": args.variable, "state_count": len(decoded),
                   "input_sha256": input_hash, "expected_sha256": expected_hash,
                   "output_sha256": hashlib.sha256(output).hexdigest(),
                   "first_difference": mismatch}
        print(json.dumps(receipt, sort_keys=True))
        return 0 if mismatch is None else 1
    except (DecodeError, OSError, UnicodeError, json.JSONDecodeError, RecursionError) as error:
        print(json.dumps({"status": "error", "error": str(error)}, sort_keys=True), file=sys.stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
