"""Validate one new actual continuation window; never read an expected file.

This consumer runs no child, computes no model transition, and generates no
further query. Its only outputs are complete actual states and their pins.
"""
from pathlib import Path
import hashlib
import json
import sys

import nominal_trace_codec as codec
from complete_k_wire import CompleteKWire
from complete_chunk_wire import join_complete_chunks, _same, TERMINAL
from kore_continuation import resume_query, arguments, TAPE, STATE


def read_pinned(row):
    raw = Path(row["path"]).read_bytes()
    if len(raw) != row["bytes"] or hashlib.sha256(raw).hexdigest() != row["sha256"]:
        raise ValueError("Selected input pin differs: " + row["path"])
    return raw


def persist(path, value):
    raw = (json.dumps(value, sort_keys=True, ensure_ascii=False, indent=2) + "\n").encode()
    with Path(path).open("xb") as stream:
        stream.write(raw)
    return {"path": path, "bytes": len(raw), "sha256": hashlib.sha256(raw).hexdigest()}


def consume(manifest):
    if manifest["expected_documents"] != "NONE_SELECTED_OR_OPENED":
        raise ValueError("No expected documents are part of this window")
    if (manifest["full_case_observation_bound"], manifest["prior_observation_bound"],
        manifest["selected_new_window_bound"], manifest["selected_prefix_maximum_after_window"]) != (512, 16, 16, 32):
        raise ValueError("Unselected first-window bounds")

    prior_raw = read_pinned(manifest["prior_raw_tape"])
    prior_wire = json.loads(read_pinned(manifest["prior_actual_prefix"]))
    query = read_pinned(manifest["query"])
    exact_query, raw_carry_proof = resume_query(prior_raw, 16)
    if query != exact_query:
        raise ValueError("Query does not carry the exact actual final NGState")
    if prior_wire["caseId"] != manifest["case_id"] or prior_wire["profile"] != manifest["profile"]:
        raise ValueError("Prior actual case/profile differs")
    if prior_wire["terminalStepContract"] != manifest["terminalStepContract"]:
        raise ValueError("Terminal contract changed")
    prior_states = prior_wire["states"]
    if prior_wire["observerCuts"] != list(range(len(prior_states))):
        raise ValueError("Prior actual ordinal policy differs")
    if len(prior_states) - 1 != manifest["expected_prior_advancing_records_from_actual"]:
        raise ValueError("Prior captured actual ordinal differs")
    if len(prior_states[-1]["events"]) != len(prior_states) - 1:
        raise ValueError("Prior actual cumulative event count differs")

    actual_kore = Path(manifest["new_actual_kore"]).read_bytes()
    raw_parts, _ = arguments(actual_kore, TAPE, 3)
    arguments(raw_parts[0], STATE, 5)
    arguments(raw_parts[2], STATE, 5)
    printed = Path(manifest["new_actual_print"]).read_bytes()
    codec.configure(manifest["api"], manifest["syntax"])
    adapter = CompleteKWire(codec.API, codec.REG)
    if prior_wire["schema"] != adapter.SCHEMA:
        raise ValueError("Prior actual wire schema differs")
    window_states = adapter.complete_states(codec.parse_printed(printed.decode("utf-8")))
    if not _same(prior_states[-1], window_states[0]):
        raise ValueError("Complete actual boundary differs")
    # 32 is this selected PREFIX maximum, not the full case's retained512 bound.
    joined = join_complete_chunks([(16, prior_states), (16, window_states)], overall_bound=32)
    for ordinal, state in enumerate(joined):
        if len(state["events"]) != ordinal:
            raise ValueError("Actual global state/event ordinals differ")
    joined_wire = adapter.bind_shared_wire(
        joined, case_id=manifest["case_id"], profile=manifest["profile"],
        terminal_step_contract=manifest["terminalStepContract"],
        observer_cuts=list(range(len(joined))),
    )
    window_pin = persist(manifest["window_states_output"], {
        "kind": "ACTUAL_CONTINUATION_WINDOW_NOT_WHOLE_CASE_WIRE",
        "first_actual_ordinal": len(prior_states) - 1,
        "states": window_states,
        "global_actual_ordinals": list(range(len(prior_states) - 1, len(joined))),
    })
    joined_pin = persist(manifest["joined_prefix_output"], joined_wire)
    last_control = next(iter(joined[-1]["control"]))
    return {
        "status": "FIRST_NEW_WINDOW_ACTUAL_PREFIX_VALIDATED_ONLY",
        "case_id": manifest["case_id"],
        "raw_state_carry": raw_carry_proof,
        "complete_typed_boundary_equal": True,
        "duplicate_boundary_rows_removed": 1,
        "new_window_advancing_records": len(window_states) - 1,
        "joined_actual_state_count": len(joined),
        "joined_actual_advancing_records": len(joined) - 1,
        "actual_last_control": joined[-1]["control"],
        "actual_last_event": joined[-1]["events"][-1],
        "last_control_is_terminal": last_control in TERMINAL,
        "last_control_is_halted": last_control == "ngHalted",
        "full_case_observation_bound": 512,
        "remaining_full_case_budget": 512 - (len(joined) - 1),
        "window_actual_states": window_pin,
        "joined_partial_prefix_wire": joined_pin,
        "actual_kore_sha256": hashlib.sha256(actual_kore).hexdigest(),
        "actual_print_sha256": hashlib.sha256(printed).hexdigest(),
        "observerCuts_source": "all actual joined state ordinals, before expected access",
        "expected_documents_opened": 0,
        "future_queries_generated": 0,
        "whole_case_oracle_comparison": "NOT_SELECTED",
        "other_three_cases": "UNRUN_BY_THIS_SELECTION",
    }


if __name__ == "__main__":
    manifest = json.loads(Path(sys.argv[1]).read_bytes())
    print(json.dumps(consume(manifest), indent=2), flush=True)
