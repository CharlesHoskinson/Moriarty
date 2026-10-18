"""Consume one predeclared actual window; no children or expected data access.

The Root executor owns the finite operation list. This data consumer only
verifies captured states, persists their joined prefix, and carries the exact
raw final state into the already declared next input when continuation remains.
"""
from pathlib import Path
import hashlib
import json
import sys

import nominal_trace_codec as codec
from complete_k_wire import CompleteKWire
from complete_chunk_wire import join_complete_chunks, _same, TERMINAL
from kore_continuation import resume_query, seed_query, arguments, OBSERVE, TAPE, STATE


def pin_bytes(path, raw):
    return {"path": path, "bytes": len(raw), "sha256": hashlib.sha256(raw).hexdigest()}


def read_pinned(row):
    raw = Path(row["path"]).read_bytes()
    if pin_bytes(row["path"], raw) != row:
        raise ValueError("Selected captured input pin differs")
    return raw


def persist_raw(path, raw):
    with Path(path).open("xb") as stream:
        stream.write(raw)
    return pin_bytes(path, raw)


def persist_json(path, value):
    raw = (json.dumps(value, sort_keys=True, ensure_ascii=False, indent=2) + "\n").encode()
    return persist_raw(path, raw)


def prior_inputs(contract, window):
    prior = window["prior"]
    if prior["kind"] == "frozen_actual":
        if window["start"] != contract["resume_actual_ordinal"]:
            raise ValueError("Frozen seed only belongs to the first selected window")
        return prior["raw_tape"], prior["prefix_wire"], prior["query"]
    if prior["kind"] != "previous_actual_checkpoint":
        raise ValueError("Unknown actual dependency kind")
    # The Root capture interface pins this produced checkpoint after its child
    # and before any following operation. No expected file supplies these pins.
    checkpoint = json.loads(Path(prior["path"]).read_bytes())
    if checkpoint["status"] != "CONTINUE":
        raise ValueError("Prior actual checkpoint forbids another query")
    if checkpoint["case_id"] != contract["case_id"] or checkpoint["full_case_bound"] != 512:
        raise ValueError("Actual checkpoint case/bound differs")
    if checkpoint["actual_advancing_records"] != window["start"]:
        raise ValueError("Actual dependency ordinal differs")
    fields = [("raw_tape", "expected_raw_tape"), ("prefix_wire", "expected_prefix_wire"),
              ("next_query", "expected_query")]
    rows = []
    for field, selected_path in fields:
        row = checkpoint[field]
        if row["path"] != prior[selected_path]:
            raise ValueError("Produced artifact escaped the predeclared dependency")
        rows.append(row)
    return tuple(rows)


def consume(contract, index):
    if contract["expected_documents"] != "NONE_SELECTED_OR_OPENED":
        raise ValueError("Expected documents are outside this actual-only capture")
    if (contract["full_case_observation_bound"], contract["resume_actual_ordinal"],
        contract["fixed_window_bound"], contract["maximum_new_windows"]) != (512, 0, 16, 32):
        raise ValueError("Unselected finite continuation bounds")
    windows = contract["windows"]
    if len(windows) != 32 or type(index) is not int or not 0 <= index < len(windows):
        raise ValueError("Unselected finite window")
    window = windows[index]
    start, end, bound = window["start"], window["end"], window["bound"]
    if (start, end, bound) != (16 * index, 16 + 16 * index, 16):
        raise ValueError("Window differs from the finite selected schedule")
    initial_window = index == 0
    raw_carry, prior_states = None, None
    if initial_window:
        prior = window["prior"]
        if start != 0 or prior["kind"] != "parsed_initial_query" or prior["query"] != window["query"]:
            raise ValueError("Unselected parsed initial query")
        # Root pins the actual kast stdout BEFORE reduce, together with its
        # frozen surface input. This consumer creates no initial K state.
        query = Path(prior["query"]).read_bytes()
        selected_seed = seed_query(query, bound)
        raw_carry = {"source": "actual kast initial query, no prior actual tape",
                     "query_bytes": len(query), "query_sha256": hashlib.sha256(query).hexdigest()}
        if query != selected_seed:
            raise ValueError("Initial parsed observation bound differs")
        initial_source = read_pinned(prior["surface_initial"])
        surface_query = read_pinned(prior["surface_query"])
        if surface_query != b"ngObserve(" + initial_source.strip() + b",16)\n":
            raise ValueError("Frozen initial Program/source query identity differs")
    else:
        raw_pin, prefix_pin, query_pin = prior_inputs(contract, window)
        if query_pin["path"] != window["query"]:
            raise ValueError("Selected query dependency path differs")
        prior_raw = read_pinned(raw_pin)
        prior_wire = json.loads(read_pinned(prefix_pin))
        query = read_pinned(query_pin)
        exact_query, raw_carry = resume_query(prior_raw, bound)
        if query != exact_query:
            raise ValueError("Query is not the exact raw actual final-state continuation")
        if (prior_wire["caseId"], prior_wire["profile"], prior_wire["schema"], prior_wire["terminalStepContract"]) != (
            contract["case_id"], contract["profile"], contract["schema"], contract["terminalStepContract"]):
            raise ValueError("Captured actual wire contract changed")
        prior_states = prior_wire["states"]
        if len(prior_states) != start + 1 or prior_wire["observerCuts"] != list(range(len(prior_states))):
            raise ValueError("Prior actual prefix ordinals differ")
        if next(iter(prior_states[-1]["control"])) in TERMINAL:
            raise ValueError("Another query followed an actual terminal state")

    actual_kore = Path(window["new_actual_kore"]).read_bytes()
    raw_parts, _ = arguments(actual_kore, TAPE, 3)
    arguments(raw_parts[0], STATE, 5)
    arguments(raw_parts[2], STATE, 5)
    printed = Path(window["new_actual_print"]).read_bytes()
    codec.configure(contract["api"], contract["syntax"])
    adapter = CompleteKWire(codec.API, codec.REG)
    if adapter.SCHEMA != contract["schema"]:
        raise ValueError("Complete decoded wire schema changed")
    new_states = adapter.complete_states(codec.parse_printed(printed.decode("utf-8")))
    if initial_window:
        # Decode ONLY declared input fields for an admission-identity check.
        # This does not evaluate, construct or emit an initial/next model state.
        declared = codec.parse_printed(initial_source.decode("utf-8"), sort="NGState")
        if declared.name != "ngInitial" or len(declared.args) != 2:
            raise ValueError("Declared initial input shape differs")
        program = adapter.decode(declared.args[0])
        environment = adapter.decode(declared.args[1])
        initial = new_states[0]
        if (not _same(initial["program"], program) or not _same(initial["environment"], environment)
            or not _same(initial["control"], {"ngEval": {"expr_id": program["entry"]}})
            or initial["frames_top_first"] != [] or initial["events"] != []):
            raise ValueError("Complete observed initial state differs from declared input")
        joined = join_complete_chunks([(bound, new_states)], overall_bound=end)
    else:
        if not _same(prior_states[-1], new_states[0]):
            raise ValueError("Complete typed actual continuation boundary differs")
        # end bounds this selected PREFIX, while the retained full-case bound is512.
        joined = join_complete_chunks([(start, prior_states), (bound, new_states)], overall_bound=end)
    for ordinal, state in enumerate(joined):
        if len(state["events"]) != ordinal:
            raise ValueError("Actual global event/state ordinals differ")
    used = len(joined) - 1
    last = next(iter(joined[-1]["control"]))
    status = ("STOP_HALT" if last == "ngHalted" else "STOP_OUTSIDE_MODEL" if last == "ngOutsideModel"
              else "STOP_BOUND" if used == 512 else "CONTINUE")
    if status == "CONTINUE" and used != end:
        raise ValueError("Nonterminal observation did not fill its selected window")
    wire = adapter.bind_shared_wire(
        joined, case_id=contract["case_id"], profile=contract["profile"],
        terminal_step_contract=contract["terminalStepContract"],
        observer_cuts=list(range(len(joined))),
    )
    window_pin = persist_json(window["window_states_output"], {
        "kind": "ACTUAL_CONTINUATION_WINDOW_NOT_WHOLE_CASE_WIRE", "first_actual_ordinal": start,
        "states": new_states, "global_actual_ordinals": list(range(start, used + 1)),
    })
    prefix_wire = persist_json(window["joined_prefix_output"], wire)
    raw_tape = pin_bytes(window["new_actual_kore"], actual_kore)
    next_query, complete_wire = None, None
    if status == "CONTINUE":
        if index + 1 >= len(windows) or window["next_query_output"] != windows[index + 1]["query"]:
            raise ValueError("No predeclared next input remains")
        payload, next_carry = resume_query(actual_kore, windows[index + 1]["bound"])
        next_query = persist_raw(window["next_query_output"], payload)
    else:
        complete_wire = persist_json(contract["completed_case_output"], wire)
    checkpoint = {
        "status": status, "case_id": contract["case_id"], "full_case_bound": 512,
        "actual_advancing_records": used, "raw_tape": raw_tape,
        "prefix_wire": prefix_wire, "next_query": next_query,
    }
    checkpoint_pin = persist_json(window["checkpoint_output"], checkpoint)
    return {
        "status": status, "kind": "FINITE_DECLARED_CASE_ACTUAL_CAPTURE_NO_ORACLE_COMPARISON",
        "window_index": index, "case_id": contract["case_id"], "selected_window": [start, end],
        "raw_state_carry": raw_carry, "complete_typed_boundary_equal": True,
        "duplicate_boundary_rows_removed": 0 if initial_window else 1, "new_advancing_records": len(new_states) - 1,
        "actual_state_count": len(joined), "actual_advancing_records": used,
        "actual_last_control": joined[-1]["control"], "actual_last_event": joined[-1]["events"][-1],
        "full_case_observation_bound": 512, "remaining_budget": 512 - used,
        "normal_halted": status == "STOP_HALT", "window_states": window_pin,
        "prefix_wire": prefix_wire, "checkpoint": checkpoint_pin,
        "next_query": next_query, "complete_case_observed_wire": complete_wire,
        "actual_kore_sha256": hashlib.sha256(actual_kore).hexdigest(),
        "actual_print_sha256": hashlib.sha256(printed).hexdigest(),
        "expected_documents_opened": 0, "whole_four_case_comparison": "PENDING_ALL_FOUR_ACTUAL_WIRES_BEFORE_ANY_EXPECTED_READ",
        "subject_children_launched_by_consumer": 0,
    }


if __name__ == "__main__":
    selected = json.loads(Path(sys.argv[1]).read_bytes())
    contract = json.loads(Path(selected["contract"]).read_bytes())
    print(json.dumps(consume(contract, selected["window_index"]), indent=2), flush=True)
