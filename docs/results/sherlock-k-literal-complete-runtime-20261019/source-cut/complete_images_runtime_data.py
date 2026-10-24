"""Ordinary framing and complete-data checks; no launcher or evaluator.

Framing changes only the generated K configuration around the parsed LCRun.
Audit reads every value from actual decoded K output. Comparison may open an
independent golden only after checking Root's durable actual-freeze binding.
"""
import argparse
import hashlib
import json
from pathlib import Path

from balanced_kore import arguments, application
import literal_complete_images as wire

RUN = b"LbllcRun'LParUndsCommUndsCommUndsCommUndsCommUndsRParUnds'MORIARTY-LITERAL-COMPLETE-IMAGES-SYNTAX'Unds'LCRun'Unds'LCContext'Unds'LCCaps'Unds'LCImage'Unds'LCImage'Unds'Int{}"
INJECTION = b"inj{SortLCRun{},SortKItem{}}"
TOP = b"Lbl'-LT-'generatedTop'-GT-'{}"
K_CELL = b"Lbl'-LT-'k'-GT-'{}"
COUNTER = b"Lbl'-LT-'generatedCounter'-GT-'{}"
COUNTER_ZERO = COUNTER + b'(\\dv{SortInt{}}("0"))'
OBSERVATION_KEYS = {"format", "initial", "afters", "final", "events", "statement",
                    "logical_layout", "context_authentication", "source"}


def same(a, b):
    if type(a) is not type(b):
        return False
    if type(a) is dict:
        return a.keys() == b.keys() and all(same(a[k], b[k]) for k in a)
    if type(a) is list:
        return len(a) == len(b) and all(same(x, y) for x, y in zip(a, b))
    return a == b


def pin(path):
    path = Path(path)
    raw = path.read_bytes()
    return {"path": str(path), "bytes": len(raw), "sha256": hashlib.sha256(raw).hexdigest()}


def pinned(row):
    if pin(row["path"]) != row:
        raise ValueError("Pinned data changed: " + row["path"])
    return Path(row["path"]).read_bytes()


def read(path):
    return json.loads(Path(path).read_bytes(), object_pairs_hook=wire.unique_object)


def save(path, obj):
    raw = (json.dumps(obj, sort_keys=True, separators=(",", ":"), allow_nan=False) + "\n").encode()
    with Path(path).open("xb") as output:
        output.write(raw)
    return pin(path)


def frame(parsed):
    """Preserve the actual parsed run bytes; add only declared configuration."""
    trimmed = parsed.strip()
    head, _ = application(trimmed)
    if head == RUN:
        arguments(trimmed, RUN, 5)
        item = INJECTION + b"(" + trimmed + b")"
        injection_added = True
    elif head == INJECTION:
        inner = arguments(trimmed, INJECTION, 1)[0][0]
        arguments(inner, RUN, 5)
        item = trimmed
        injection_added = False
    else:
        raise ValueError("Expected parsed LCRun or its exact KItem injection")
    framed = TOP + b"(" + K_CELL + b"(kseq{}(" + item + b",dotk{}()))," + COUNTER_ZERO + b")\n"
    top = arguments(framed, TOP, 2)[0]
    sequence = arguments(arguments(top[0], K_CELL, 1)[0][0], b"kseq{}", 2)[0]
    if top[1] != COUNTER_ZERO or sequence[1] != b"dotk{}()" or sequence[0] != item:
        raise ValueError("Generated configuration frame changed")
    recovered = arguments(sequence[0], INJECTION, 1)[0][0] if injection_added else sequence[0]
    if recovered != trimmed:
        raise ValueError("Raw parsed-run inverse differs")
    return framed, {"parsed_LCRun_bytes_preserved": True,
                    "only_outer_whitespace_trimmed": True,
                    "run_to_KItem_injection_added": injection_added,
                    "generated_counter_initial_value": 0,
                    "semantic_steps_computed": 0}


def flatten(image):
    return [*image["root"], *(lane for table in image["tables"] for row in table for lane in row)]


def checked_machine(machine, source):
    wire.keys(machine, {"context", "capacities", "program", "state"}, "actual machine")
    for key in ("context", "capacities", "program"):
        if not same(machine[key], source[key]):
            raise ValueError("Complete supplied " + key + " changed")
    caps = source["capacities"]
    wire.image(machine["state"], 45, caps["state"], wire.STATE_WIDTHS, "actual state")


def audit(source_document, observed):
    source = wire.request(source_document)
    if source["observation_budget"] != 2:
        raise ValueError("This selected finite task has observation budget two")
    wire.keys(observed, OBSERVATION_KEYS, "actual observation")
    if observed["format"] != wire.OUTPUT_FORMAT or type(observed["afters"]) is not list or len(observed["afters"]) != 2:
        raise ValueError("Complete two-event observation required")
    initial, middle, done = observed["initial"], *observed["afters"]
    for machine in (initial, middle, done, observed["final"]):
        checked_machine(machine, source)
    if not same(initial["state"], source["init"]) or not same(observed["final"], done):
        raise ValueError("Whole initial or final carrier differs")
    events = observed["events"]
    if type(events) is not list or len(events) != 2:
        raise ValueError("Both complete event records required")
    for event, envelope, before, after in zip(events, [(102, 3, 1), (80, 0, 0)],
                                               [initial, middle], [middle, done]):
        wire.keys(event, {"source_line", "rule_tag", "charge", "before", "after"}, "actual event")
        for key, expected in zip(("source_line", "rule_tag", "charge"), envelope):
            if type(event[key]) is not int or event[key] != expected:
                raise ValueError("Wrong declared A102/A80 envelope: " + key)
        if not same(event["before"], before) or not same(event["after"], after):
            raise ValueError("Complete event boundary differs from complete trace")
    actual_statement = wire.vector(observed["statement"], 335, "actual statement", wire.UINT128)
    concatenation = [*initial["context"], *flatten(initial["program"]),
                     *flatten(initial["state"]), *flatten(middle["state"]), *flatten(done["state"]),
                     *(lane for event in events for lane in (event["rule_tag"], event["charge"]))]
    if not same(actual_statement, concatenation):
        raise ValueError("K-emitted 335-lane statement differs from actual whole trace order")
    if not same(observed["logical_layout"], wire.logical_layout(initial)):
        raise ValueError("Derived coordinate metadata differs from actual capacities")
    if observed["context_authentication"] != "NOT_ESTABLISHED":
        raise ValueError("Context authenticity was not established here")
    if observed["source"] != "complete K-print constructor tree; no expected-output fill":
        raise ValueError("Unmatched actual decoder provenance")
    return {"status": "PASS_ACTUAL_FULL_IMAGE_CONTINUITY_AND_335_LANE_SERIALIZATION",
            "complete_advancing_event_records": 2, "distinct_state_positions": 3,
            "all_335_statement_lanes_retained": True, "context_binds_counterpart_source": False,
            "context_authentication": "NOT_ESTABLISHED", "transition_states_predicted": 0,
            "expected_documents_opened": 0,
            "source_rule_application_or_native_proof_validity_claim": False}


def main():
    cli = argparse.ArgumentParser(description=__doc__)
    cli.add_argument("action", choices=("frame", "audit", "compare"))
    cli.add_argument("--input", type=Path, required=True)
    cli.add_argument("--output", type=Path, required=True)
    cli.add_argument("--safe-input", type=Path)
    cli.add_argument("--freeze", type=Path)
    cli.add_argument("--expected-binding", type=Path)
    args = cli.parse_args()
    if args.action == "frame":
        framed, receipt = frame(args.input.read_bytes())
        with args.output.open("xb") as stream:
            stream.write(framed)
        print(json.dumps({**receipt, "input": pin(args.input), "output": pin(args.output)}))
    elif args.action == "audit":
        if args.safe_input is None:
            cli.error("audit requires --safe-input")
        receipt = {**audit(read(args.safe_input), read(args.input)),
                   "safe_input": pin(args.safe_input), "actual_observation": pin(args.input)}
        save(args.output, receipt)
        print(json.dumps(receipt))
    else:
        if args.freeze is None or args.expected_binding is None:
            cli.error("compare requires --freeze and --expected-binding")
        # Neither expected body nor its binding is opened before actual barrier checks.
        freeze = read(args.freeze)
        if type(freeze["expected_body_reads"]) is not int or freeze["expected_body_reads"] != 0 or freeze["actual_observation"] != pin(args.input):
            raise ValueError("Actual observation is not its durable before-expected identity")
        actual = json.loads(pinned(freeze["actual_observation"]), object_pairs_hook=wire.unique_object)
        checked_audit = json.loads(pinned(freeze["actual_audit"]), object_pairs_hook=wire.unique_object)
        if checked_audit["status"] != "PASS_ACTUAL_FULL_IMAGE_CONTINUITY_AND_335_LANE_SERIALIZATION":
            raise ValueError("Complete actual audit did not pass")
        if checked_audit["actual_observation"] != freeze["actual_observation"]:
            raise ValueError("Audit belongs to another actual observation")
        for product in freeze["products_streams_receipts"]:
            pinned(product)
        binding = read(args.expected_binding)
        expected = json.loads(pinned(binding["expected_complete_observation"]), object_pairs_hook=wire.unique_object)
        equal = same(actual, expected)
        receipt = {"status": "PASS_WHOLE_TYPED_INDEPENDENT_OBSERVATION" if equal else "FAIL_WHOLE_TYPED_INDEPENDENT_OBSERVATION",
                   "complete_typed_equal": equal, "actual": pin(args.input),
                   "opaque_expected": binding["expected_complete_observation"],
                   "durable_actual_freeze": pin(args.freeze), "context_binds_counterpart_source": False,
                   "runtime_or_native_proof_equivalence_claim": False}
        save(args.output, receipt)
        print(json.dumps(receipt))
        if not equal:
            raise SystemExit(1)


if __name__ == "__main__":
    main()
