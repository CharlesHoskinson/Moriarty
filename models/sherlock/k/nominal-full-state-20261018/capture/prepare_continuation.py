"""Carry the already observed raw state; no evaluation or expected access."""
import argparse
import json
from pathlib import Path

from decode_prefix import pinned, save
from render_input import same, write
from kore_continuation import resume_query, arguments, TAPE, STATE, OBSERVE


def main():
    cli = argparse.ArgumentParser(description=__doc__)
    cli.add_argument("manifest", type=Path)
    manifest = json.loads(cli.parse_args().manifest.read_bytes())
    source = json.loads(pinned(manifest["safe_input"]))
    prior_freeze = json.loads(pinned(manifest["prior_actual_freeze"]))
    if prior_freeze["actual_wire"] != manifest["prior_actual_wire"]:
        raise ValueError("Prior actual wire is not its recorded before-expected identity")
    if manifest["prior_raw_kore"] not in prior_freeze["products_streams_receipts"]:
        raise ValueError("Prior raw Kore is not bound in its original durable freeze")
    prior = json.loads(pinned(manifest["prior_actual_wire"]))
    ordinal = manifest["carry_ordinal"]
    if (len(prior["states"]) != ordinal + 1 or len(prior["states"][-1]["events"]) != ordinal
            or not same(prior["states"][-1]["program"], source["ng_program"])):
        raise ValueError("Selected actual carry ordinal/program differs")
    raw = pinned(manifest["prior_raw_kore"])
    parts, _ = arguments(raw, TAPE, 3)
    arguments(parts[2], STATE, 5)
    bound = manifest["new_window_bound"]
    if type(bound) is not int or bound <= 0 or ordinal + bound != source["observation_budget"]:
        raise ValueError("Finite new window differs from frozen complete-prefix budget")
    query, proof = resume_query(raw, bound)
    if arguments(query, OBSERVE, 2)[0][0] != parts[2]:
        raise ValueError("Whole raw NGState was changed while carrying it")
    state_pin = write(manifest["raw_carry_output"], parts[2])
    query_pin = write(manifest["query_output"], query)
    receipt = {"status": "VERBATIM_ACTUAL_RAW_STATE_CARRIED", "carry_ordinal": ordinal,
        "new_window_bound": bound, "prior_actual_wire": manifest["prior_actual_wire"],
        "prior_raw_kore": manifest["prior_raw_kore"], "whole_state": state_pin,
        "query": query_pin, "inverse": proof, "model_steps_computed": 0,
        "old_prefix_reexecuted": False, "expected_body_reads": 0}
    save(manifest["carry_receipt_output"], receipt)
    print(json.dumps(receipt))


if __name__ == "__main__":
    main()
