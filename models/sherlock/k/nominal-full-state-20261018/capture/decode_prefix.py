"""Persist complete actual K prefix, then assess the frozen primary relation.

No expected file is opened. There is no model step, primary step, alignment
search or continuation generation here. Raw tool outputs remain untouched.
"""
import argparse
import hashlib
import json
from pathlib import Path

import nominal_trace_codec as parser
from complete_k_wire import CompleteKWire
from kore_continuation import arguments, TAPE, STATE
from render_input import same, write
from primary_relation import relate


def pinned(spec):
    raw = Path(spec["path"]).read_bytes()
    if len(raw) != spec["bytes"] or hashlib.sha256(raw).hexdigest() != spec["sha256"]:
        raise ValueError("Static input pin differs: " + spec["path"])
    return raw


def save(path, value):
    return write(path, (json.dumps(value, ensure_ascii=False, allow_nan=False, indent=2) + "\n").encode())


def main():
    cli = argparse.ArgumentParser(description=__doc__)
    cli.add_argument("manifest", type=Path)
    args = cli.parse_args()
    m = json.loads(args.manifest.read_bytes())
    source = json.loads(pinned(m["safe_input"]))
    schedule = json.loads(pinned(m["boundary_schedule"]))
    compile_document = json.loads(pinned(source["source_public_compile"]))
    raw_primary = pinned(schedule["primary_complete_prefix"])
    primary = [json.loads(line) for line in raw_primary.splitlines()]
    raw_kore = Path(m["actual_kore"]).read_bytes()
    parts, _ = arguments(raw_kore, TAPE, 3)
    arguments(parts[0], STATE, 5)
    arguments(parts[2], STATE, 5)
    parser.configure(m["api"], m["syntax"])
    adapter = CompleteKWire(parser.API, parser.REG)
    printed = Path(m["actual_print"]).read_bytes()
    states = adapter.complete_states(parser.parse_printed(printed.decode("utf-8")))
    if not states or len(states) > source["observation_budget"] + 1:
        raise ValueError("Actual complete tape exceeds declared observation bound")
    declared = parser.parse_printed(Path(m["rendered_initial"]).read_text(), sort="NGState")
    if declared.name != "ngInitial" or not same(adapter.decode(declared.args[0]), source["ng_program"]) or not same(adapter.decode(declared.args[1]), source["initial_environment"]):
        raise ValueError("Rendered input differs from complete fixed source input")
    initial = states[0]
    if not same(initial, {"program": source["ng_program"], "control": {"ngEval": {"expr_id": source["ng_program"]["entry"]}},
                          "environment": source["initial_environment"], "frames_top_first": [], "events": []}):
        raise ValueError("Complete actual initial state differs from input contract")
    for ordinal, state in enumerate(states):
        if len(state["events"]) != ordinal or not same(state["program"], source["ng_program"]):
            raise ValueError("Complete Program/event ordinal identity differs")
    wire = adapter.bind_shared_wire(states, case_id=source["case_id"], profile=source["profile"],
        terminal_step_contract=source["terminalStepContract"], observer_cuts=list(range(len(states))))
    # Complete actual wire is persisted BEFORE attempting any observation relation.
    wire_pin = save(m["actual_wire_output"], wire)
    try:
        relation = relate(compile_document, source["ng_program"], primary, schedule, states)
    except (ValueError, KeyError, IndexError, TypeError) as error:
        relation = {"status": "FAIL", "relation_error": type(error).__name__ + ": " + str(error),
            "actual_wire_preserved": True, "alignment_was_not_changed": True,
            "K_work_carrier": "ABSENT; NO CLOCK/FUEL/WORK EQUALITY CLAIM"}
    relation["actual_wire"] = wire_pin
    relation["raw_primary_prefix"] = schedule["primary_complete_prefix"]
    relation["frozen_boundary_schedule"] = m["boundary_schedule"]
    relation_pin = save(m["relation_output"], relation)
    print(json.dumps({"status": "ACTUAL_CAPTURE_PERSISTED", "relation_status": relation["status"],
        "actual_wire": wire_pin, "relation": relation_pin,
        "actual_state_count": len(states), "actual_advancing_steps": len(states) - 1,
        "actual_last_control": states[-1]["control"], "observerCuts_derived_only_from_actual": True,
        "raw_kore_sha256": hashlib.sha256(raw_kore).hexdigest(),
        "full_print_sha256": hashlib.sha256(printed).hexdigest(), "expected_body_reads": 0,
        "subject_steps_or_children_computed": 0}))


if __name__ == "__main__":
    main()
