"""Mutation controls for the independent Candidate A finite-record checker."""

from __future__ import annotations

import importlib.util
import copy
import hashlib
import json
import os
from pathlib import Path
import sys

import pytest


ROOT = Path(__file__).parents[1]
CHECKER = Path(os.environ.get("S02_CHECKER_PATH", ROOT / "scripts" / "check_s02_candidate_a_correspondence.py"))
SPEC = importlib.util.spec_from_file_location("candidate_a_correspondence", CHECKER)
assert SPEC is not None and SPEC.loader is not None
MODULE = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = MODULE
SPEC.loader.exec_module(MODULE)


def test_frozen_source_hash_required() -> None:
    """The checker accepts only a schema-1 document with frozen source pins."""
    report = MODULE.check_document({"schema_version": 1, "source_pins": {}, "cases": [{}]})
    assert not report.ok
    assert "source_pins" in report.differences[0]


def test_bool_schema_version_is_rejected() -> None:
    report = MODULE.check_document({"schema_version": True, "source_pins": MODULE.PINNED, "cases": [{}]})
    assert not report.ok
    assert "schema_version" in report.differences[0]


def test_empty_corpus_is_rejected() -> None:
    report = MODULE.check_document({"schema_version": 1, "source_pins": MODULE.PINNED, "cases": []})
    assert not report.ok
    assert "nonempty" in report.differences[0]


# These are independently assembled schema fixtures from archived actual ITFs.
# No exporter or Candidate A projection/evaluation helper is imported.
def enum(tag, value=None):
    return {"tag": tag, "value": {"#tup": []} if value is None else value}


def integer(value):
    return {"#bigint": str(value)}


def archived_trace(name="two-fills"):
    return json.loads((ROOT / "evidence/s02-model-comparison/candidate-a-installment/stages/samples" / (name + ".itf.json")).read_text())


def sample_state():
    return copy.deepcopy(archived_trace()["states"][0]["installmentTrace"]["agreement"])


def program_node(program, node):
    return next(value for key, value in program["nodes"]["#map"] if key["tag"] == node)


def declared_pins():
    pins = dict(MODULE.PINNED)
    for path in MODULE.OPTIONAL_GENERATORS.values():
        if (ROOT / path).is_file():
            pins[path] = hashlib.sha256((ROOT / path).read_bytes()).hexdigest()
    return pins


def bundle(tmp_path, name="two-fills"):
    raw = archived_trace(name)
    input_path = tmp_path / "trace.itf.json"
    input_path.write_text(json.dumps(raw))
    digest = hashlib.sha256(input_path.read_bytes()).hexdigest()
    records = raw["states"][-1]["installmentTrace"]["records"]
    cases = []
    for step, entry in enumerate(records):
        payload = entry["payload"]["value"]
        provenance = [{"trace_id": input_path.name, "input_path": input_path.name,
            "input_sha256": digest, "step_index": step, "state_index": index}
            for index, state in enumerate(raw["states"])
            if len(state["installmentTrace"]["records"]) > step]
        cases.append({"case_id": f"{input_path.name}::{step}", "fixture_id": "installment-two-when-v1",
            "trace_id": input_path.name, "step_index": step, "request": copy.deepcopy(entry["request"]),
            "result": copy.deepcopy(payload["raw"]), "projection": copy.deepcopy(payload["projection"]),
            "effects": copy.deepcopy(payload["effects"]), "provenance": provenance})
    return {"schema_version": 1, "source_pins": declared_pins(), "cases": cases}


def checked(document, tmp_path):
    return MODULE.check_document(document, source_root=ROOT, input_root=tmp_path)


@pytest.mark.parametrize("raw", [0, integer(2), integer(99), enum("Time3"), True])
def test_raw_time_requires_finite_enum(raw):
    with pytest.raises(MODULE.DecodeError):
        MODULE._time(raw, "raw_time")


@pytest.mark.parametrize("quantity", [-2, 3, 22, True])
def test_deposit_input_finite_domain(quantity):
    raw = enum("PresentAInput", enum("DepositInputA", {"account": sample_state()["accounts"]["#map"][0][0],
        "depositor": enum("Alice"), "quantity": quantity}))
    with pytest.raises(MODULE.DecodeError):
        MODULE._input(raw, "input")


@pytest.mark.parametrize("chosen", [-2, 3, 100])
def test_choice_input_finite_domain(chosen):
    with pytest.raises(MODULE.DecodeError):
        MODULE._input(enum("PresentAInput", enum("ChoiceInputA", {"id": enum("FirstFillId"),
            "chooser": enum("Bob"), "chosen": integer(chosen)})), "input")


def test_state_potential_bound():
    raw = sample_state()
    raw["accounts"]["#map"][0][1] = integer(261)  # N4: 261+20*4 > 340.
    with pytest.raises(MODULE.DecodeError):
        MODULE._state(raw, "state")


def test_state_stored_choice_domain():
    raw = sample_state()
    raw["choices"]["#map"][0][1] = enum("IntValue", integer(3))
    with pytest.raises(MODULE.DecodeError):
        MODULE._state(raw, "state")


def test_duplicate_zero_account_rejected():
    raw = sample_state()
    raw["accounts"]["#map"].append(copy.deepcopy(raw["accounts"]["#map"][1]))
    with pytest.raises(MODULE.DecodeError):
        MODULE._state(raw, "state")


def test_duplicate_absent_choice_and_missing_choice_rejected():
    raw = sample_state()
    raw["choices"]["#map"][1] = copy.deepcopy(raw["choices"]["#map"][0])
    with pytest.raises(MODULE.DecodeError):
        MODULE._state(raw, "state")


def test_absence_is_not_zero():
    raw = sample_state()
    absent = MODULE._state(raw, "state")
    raw["choices"]["#map"][0][1] = enum("IntValue", integer(0))
    assert MODULE._state(raw, "state") != absent


def test_cycle_is_explicit_domain_diagnostic():
    raw = archived_trace()["states"][-1]["installmentTrace"]["records"][0]["request"]["program"]
    program_node(raw, "N3")["value"]["continuation"] = enum("N3")
    with pytest.raises(MODULE.DecodeError, match="rank|cycle"):
        MODULE._decode_program(raw, "program")


@pytest.mark.parametrize("name", ["two-fills", "refund-ten", "refund-five", "residual-deadline-cleanup"])
def test_actual_installment_records_and_repeated_prefix_provenance(tmp_path, name):
    result = checked(bundle(tmp_path, name), tmp_path)
    assert result.ok, result.differences


def test_full_check_requires_raw_input_root(tmp_path):
    document = bundle(tmp_path)
    result = MODULE.check_document(document, source_root=ROOT)
    assert not result.ok and "input_root" in str(result.differences)


@pytest.mark.parametrize("field", ["request", "result", "projection", "effects"])
def test_raw_provenance_field_substitution(tmp_path, field):
    document = bundle(tmp_path)
    assert checked(document, tmp_path).ok
    entry = document["cases"][0]
    if field == "request": entry[field]["now"] = enum("Time1")
    elif field == "result": entry[field]["reductions"] = integer(0)
    elif field == "projection": entry[field]["reductions"] = integer(0)
    else: entry[field] = []
    result = checked(document, tmp_path)
    assert not result.ok


@pytest.mark.parametrize("mutation", ["hash", "state", "step", "path", "absolute", "traversal", "duplicate", "omit", "wrong_trace"])
def test_raw_provenance_controls(tmp_path, mutation):
    document = bundle(tmp_path)
    assert checked(document, tmp_path).ok
    p = document["cases"][0]["provenance"][0]
    if mutation == "hash": p["input_sha256"] = "0" * 64
    elif mutation == "state": p["state_index"] = 999
    elif mutation == "step": p["step_index"] = 999
    elif mutation == "path": p["input_path"] = "absent.itf.json"
    elif mutation == "absolute": p["input_path"] = str(tmp_path / "trace.itf.json")
    elif mutation == "traversal": p["input_path"] = "../trace.itf.json"
    elif mutation == "duplicate": document["cases"][0]["provenance"].append(copy.deepcopy(p))
    elif mutation == "omit": document["cases"][0]["provenance"].pop()
    else: p["trace_id"] = "other.itf.json"
    assert not checked(document, tmp_path).ok


@pytest.mark.parametrize("mutation", ["unused", "edge", "root", "projection_unused", "projection_edge"])
def test_exact_symbolic_node_table(tmp_path, mutation):
    document = bundle(tmp_path, "refund-ten")
    assert checked(document, tmp_path).ok
    program = document["cases"][0]["request"]["program"]
    if mutation.startswith("projection"):
        program = document["cases"][0]["projection"]["state"]["continuation"]["program"]
    if mutation in {"unused", "projection_unused"}:
        program_node(program, "N15").update(copy.deepcopy(program_node(program, "N1")))
    elif mutation in {"edge", "projection_edge"}:
        # N2 is a valid lower-ranked Close alias in canonical choice/Close fixtures;
        # here replacing timeoutN0 with N1 also must not hide behind root comparison.
        program_node(program, "N4")["value"]["timeoutNode"] = enum("N1")
    else: program["root"] = enum("N15")
    assert not checked(document, tmp_path).ok


@pytest.mark.parametrize("field", ["result", "projection"])
def test_exact_close_node_identity(tmp_path, field):
    document = bundle(tmp_path, "refund-ten")
    assert checked(document, tmp_path).ok
    if field == "result": document["cases"][0][field]["state"]["continuation"] = enum("N15")
    else: document["cases"][0][field]["state"]["continuation"]["node"] = enum("N15")
    assert not checked(document, tmp_path).ok


def test_optional_generator_pin_checked(tmp_path):
    document = bundle(tmp_path)
    assert checked(document, tmp_path).ok
    document["source_pins"]["specs/quint/s02/candidate_a_installment_harness.qnt"] = "0" * 64
    assert not checked(document, tmp_path).ok
