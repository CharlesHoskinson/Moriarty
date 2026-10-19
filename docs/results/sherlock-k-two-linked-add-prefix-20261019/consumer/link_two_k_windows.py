"""Compare two complete, already decoded K windows; never evaluate a step.

This ordinary data consumer establishes boundary equality and history shape.
It does not verify the K relation, a production interpreter step, or a proof.
"""
import argparse
import hashlib
import json
from pathlib import Path


ROOT = {"schema", "profile", "caseId", "states", "terminalStepContract", "observerCuts"}
STATE = {"program", "control", "environment", "frames_top_first", "events"}


def same(left, right):
    if type(left) is not type(right):
        return False
    if type(left) is dict:
        return left.keys() == right.keys() and all(same(left[k], right[k]) for k in left)
    if type(left) is list:
        return len(left) == len(right) and all(same(a, b) for a, b in zip(left, right))
    return left == right


def json_domain(value):
    """Keep JSON integer/Boolean distinctions; reject non-finite/float carriers."""
    if type(value) in (str, int, bool) or value is None:
        return
    if type(value) is list:
        for item in value:
            json_domain(item)
        return
    if type(value) is dict and all(type(k) is str for k in value):
        for item in value.values():
            json_domain(item)
        return
    raise ValueError("Outside the declared integral JSON wire domain")


def terminal(control):
    if type(control) is not dict or len(control) != 1:
        raise ValueError("Expected one declared control constructor")
    tag, payload = next(iter(control.items()))
    return tag in {"ngHalted", "ngOutsideModel"} or (
        tag == "ngLtError" and type(payload) is dict
        and type(payload.get("code")) is int and payload["code"] == 11
    )


def checked_window(wire, source):
    json_domain(wire)
    if type(wire) is not dict or set(wire) != ROOT:
        raise ValueError("Complete six-field wire required")
    for key, wanted in (("schema", "independent-nominal-full-state/1"),
                        ("profile", source["profile"]),
                        ("caseId", source["case_id"]),
                        ("terminalStepContract", source["terminalStepContract"])):
        if type(wire[key]) is not str or wire[key] != wanted:
            raise ValueError("Unmatched frozen wire metadata: " + key)
    states = wire["states"]
    if type(states) is not list or len(states) != 2 or not same(wire["observerCuts"], [0, 1]):
        raise ValueError("Exactly one advancing step and actual local ordinals required")
    for state in states:
        if type(state) is not dict or set(state) != STATE:
            raise ValueError("Every complete five-field state required")
        if not same(state["program"], source["ng_program"]):
            raise ValueError("Entire immutable Program differs from input")
        if any(type(state[k]) is not list for k in ("environment", "frames_top_first", "events")):
            raise ValueError("Map/frame/event arrays required without projection")
    before, after = states
    if terminal(before["control"]):
        raise ValueError("No advancing query may start at a terminal boundary")
    if len(after["events"]) != len(before["events"]) + 1 or not same(
            before["events"], after["events"][:-1]):
        raise ValueError("Full cumulative event history must extend by exactly one")
    return states


def link(first, second, source):
    """Retain original state objects, removing only one equal boundary copy."""
    json_domain(source)
    if type(source) is not dict or type(source.get("observation_budget")) is not int or source["observation_budget"] != 2:
        raise ValueError("This finite task is exactly two uncharged advancing steps")
    left, right = checked_window(first, source), checked_window(second, source)
    initial = {"program": source["ng_program"],
               "control": {"ngEval": {"expr_id": source["ng_program"]["entry"]}},
               "environment": source["initial_environment"], "frames_top_first": [], "events": []}
    if not same(left[0], initial):
        raise ValueError("First complete boundary differs from the declared initial state")
    if not same(left[-1], right[0]):
        raise ValueError("Whole first output differs from whole second input")
    # No expected data, field rebuilding, state filtering or evaluation here.
    return {**{k: first[k] for k in ("schema", "profile", "caseId", "terminalStepContract")},
            "states": [*left, right[-1]], "observerCuts": [0, 1, 2]}


def read(path):
    raw = Path(path).read_bytes()
    return json.loads(raw), {"path": str(path), "bytes": len(raw),
                             "sha256": hashlib.sha256(raw).hexdigest()}


def main():
    cli = argparse.ArgumentParser(description=__doc__)
    cli.add_argument("--first", type=Path, required=True)
    cli.add_argument("--second", type=Path, required=True)
    cli.add_argument("--input", type=Path, required=True)
    cli.add_argument("--output-dir", type=Path, required=True)
    args = cli.parse_args()
    first, first_pin = read(args.first)
    second, second_pin = read(args.second)
    source, input_pin = read(args.input)
    joined = link(first, second, source)
    raw = (json.dumps(joined, indent=2, ensure_ascii=False, allow_nan=False) + "\n").encode()
    output = args.output_dir / "linked.complete-wire.json"
    with output.open("xb") as stream:
        stream.write(raw)
    receipt = {"status": "PASS_COMPLETE_K_BOUNDARY_LINK_ONLY",
               "inputs": [first_pin, second_pin, input_pin],
               "joined_actual_wire": {"path": str(output), "bytes": len(raw),
                                      "sha256": hashlib.sha256(raw).hexdigest()},
               "whole_boundary_typed_equal": True, "only_equal_middle_duplicate_removed": True,
               "complete_state_positions": 3, "observerCuts_derived_from_actual": True,
               "K_steps_evaluated_by_consumer": 0, "expected_documents_opened": 0,
               "subject_transition_validity_verified_by_consumer": False,
               "production_step_or_recursive_proof_validity_claim": False}
    with (args.output_dir / "link-readback.json").open("x") as stream:
        json.dump(receipt, stream, indent=2)
        stream.write("\n")
    print(json.dumps(receipt))


if __name__ == "__main__":
    main()
