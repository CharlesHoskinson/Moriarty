"""Decode a new actual window, drop one equal boundary, then relate it.

No old prefix is executed; state objects are decoded only from actual K output
and retained actual wire. No expected metadata supplies an actual field.
"""
import argparse
import json
from pathlib import Path

import nominal_trace_codec as parser
from complete_k_wire import CompleteKWire
from complete_chunk_wire import join_complete_chunks
from decode_prefix import pinned, save
from render_input import same
from primary_relation import relate
from kore_continuation import arguments, TAPE, STATE, OBSERVE


def main():
    cli = argparse.ArgumentParser(description=__doc__)
    cli.add_argument("manifest", type=Path)
    m = json.loads(cli.parse_args().manifest.read_bytes())
    source = json.loads(pinned(m["safe_input"]))
    schedule = json.loads(pinned(m["boundary_schedule"]))
    compile_document = json.loads(pinned(source["source_public_compile"]))
    primary = [json.loads(line) for line in pinned(schedule["primary_complete_prefix"]).splitlines()]
    old_freeze = json.loads(pinned(m["prior_actual_freeze"]))
    if old_freeze["actual_wire"] != m["prior_actual_wire"]:
        raise ValueError("Retained actual wire differs from original durable freeze")
    prior = json.loads(pinned(m["prior_actual_wire"]))
    prior_parts, _ = arguments(pinned(m["prior_raw_kore"]), TAPE, 3)
    query = Path(m["query_output"]).read_bytes()
    if arguments(query, OBSERVE, 2)[0][0] != prior_parts[2] or Path(m["raw_carry_output"]).read_bytes() != prior_parts[2]:
        raise ValueError("Selected query did not preserve the entire actual final state")
    raw = Path(m["actual_kore"]).read_bytes()
    parts, _ = arguments(raw, TAPE, 3)
    for state in (parts[0], parts[2]):
        arguments(state, STATE, 5)
    if parts[0] != prior_parts[2]:
        raise ValueError("Actual new window initial raw state differs from carried state")
    parser.configure(m["api"], m["syntax"])
    adapter = CompleteKWire(parser.API, parser.REG)
    tail = adapter.complete_states(parser.parse_printed(Path(m["actual_print"]).read_text()))
    if not same(prior["states"][-1], tail[0]):
        raise ValueError("Complete decoded boundary differs")
    states = join_complete_chunks([(m["carry_ordinal"], prior["states"]),
        (m["new_window_bound"], tail)], overall_bound=source["observation_budget"])
    if not same(states[0], {"program": source["ng_program"],
            "control": {"ngEval": {"expr_id": source["ng_program"]["entry"]}},
            "environment": source["initial_environment"], "frames_top_first": [], "events": []}):
        raise ValueError("Joined complete initial state differs from frozen source input")
    for ordinal, state in enumerate(states):
        if len(state["events"]) != ordinal or not same(state["program"], source["ng_program"]):
            raise ValueError("Joined cumulative event ordinal/Program identity differs")
    wire = adapter.bind_shared_wire(states, case_id=source["case_id"], profile=source["profile"],
        terminal_step_contract=source["terminalStepContract"], observer_cuts=list(range(len(states))))
    # Both raw output and complete joined actual wire remain frozen before any
    # observation-relation failure or independent expected access.
    tail_pin = save(m["actual_tail_states_output"], {"complete_actual_states": tail,
        "source": "actual K window only", "carry_ordinal": m["carry_ordinal"]})
    wire_pin = save(m["actual_wire_output"], wire)
    try:
        relation = relate(compile_document, source["ng_program"], primary, schedule, states)
    except (ValueError, KeyError, IndexError, TypeError) as error:
        relation = {"status": "FAIL", "relation_error": type(error).__name__ + ": " + str(error),
            "actual_wire_preserved": True, "alignment_was_not_changed": True,
            "K_work_carrier": "ABSENT; NO CLOCK/FUEL/WORK EQUALITY CLAIM"}
    relation["actual_wire"] = wire_pin
    relation["actual_tail_states"] = tail_pin
    relation["retained_prior_actual_wire"] = m["prior_actual_wire"]
    relation["raw_primary_prefix"] = schedule["primary_complete_prefix"]
    relation["frozen_boundary_schedule"] = m["boundary_schedule"]
    relation_pin = save(m["relation_output"], relation)
    print(json.dumps({"status": "ACTUAL_CAPTURE_PERSISTED", "relation_status": relation["status"],
        "actual_wire": wire_pin, "relation": relation_pin, "actual_tail_states": tail_pin,
        "actual_state_count": len(states), "new_advancing_steps": len(tail) - 1,
        "only_equal_boundary_duplicate_removed": True, "old_prefix_reexecuted": False,
        "observerCuts_derived_only_from_actual": True, "expected_body_reads": 0,
        "subject_steps_or_children_computed": 0}))


if __name__ == "__main__":
    main()
