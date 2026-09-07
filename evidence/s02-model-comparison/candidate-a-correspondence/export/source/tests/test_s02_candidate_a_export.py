"""Structural exporter tests; synthetic records are not semantic reference vectors."""

from copy import deepcopy
from hashlib import sha256
import json
from pathlib import Path
import subprocess
import sys

import pytest

from scripts.export_s02_candidate_a_cases import BASE_PINS, ExportError, GENERATORS, export_cases


def tag(name, value=None):
    return {"tag": name, "value": {"#tup": []} if value is None else value}


def specimen():
    # Intentionally not a valid finite program/state. The exporter preserves
    # structure; only the independent checker interprets these semantic fields.
    program = {"root": tag("N0"), "nodes": {"#map": []}}
    state = {"accounts": {"#map": [["z", {"#bigint": "0"}], ["a", {"#bigint": "1"}]]},
             "choices": {"#map": []}, "continuation": tag("N0"), "minimumTime": tag("Time2")}
    request = {"program": program, "before": state, "input": tag("NoAInput"), "now": tag("Time2")}
    raw = {"accepted": True, "state": state, "error": tag("NoCoreError"), "payments": [],
           "warnings": [], "reductions": {"#bigint": "1"}}
    projected_state = {**state, "continuation": {"program": program, "node": tag("N0")},
                       "minimumTime": {"#bigint": "2"}}
    projection = {**raw, "state": projected_state}
    return {"request": request, "beforeLedger": {"#map": []}, "afterLedger": {"#map": []},
            "payload": tag("SwapComputedA", {"raw": raw, "projection": projection, "effects": []})}


def write_json(path, value):
    path.write_text(json.dumps(value), encoding="utf-8")


def setup_bundle(tmp_path, variable="swapTrace"):
    inputs, sources = tmp_path / "inputs", tmp_path / "sources"
    inputs.mkdir(); sources.mkdir()
    pins = {}
    for name in sorted(BASE_PINS | {GENERATORS[variable][0]}):
        target = sources / name
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes(name.encode())
        pins[name] = sha256(target.read_bytes()).hexdigest()
    record = specimen()
    record["payload"]["tag"] = GENERATORS[variable][1]
    if variable == "diagnosticCases":
        value = [{"case_id": "literal-case", "fixture_id": "close-v1", "request": record["request"],
                  "payload": record["payload"]}]
        states = [{variable: value}]
    else:
        empty = {"agreement": record["request"]["before"], "ledger": {"#map": []}, "records": []}
        full = {**empty, "records": [record]}
        states = [{variable: empty}, {variable: full}, {variable: deepcopy(full)}]
    document = {"#meta": {"format": "ITF", "source": GENERATORS[variable][0]}, "vars": [variable],
                "states": states}
    path = inputs / "sample.itf.json"
    write_json(path, document)
    binding = {"schema_version": 1, "source_pins": pins,
               "input_pins": {path.name: sha256(path.read_bytes()).hexdigest()}}
    write_json(inputs / "source-bindings.json", binding)
    return inputs, sources, document, binding, record


def update_input(inputs, document, binding):
    write_json(inputs / "sample.itf.json", document)
    binding["input_pins"]["sample.itf.json"] = sha256((inputs / "sample.itf.json").read_bytes()).hexdigest()
    write_json(inputs / "source-bindings.json", binding)


def test_exact_raw_fields_and_prefix_provenance(tmp_path):
    inputs, sources, _, binding, record = setup_bundle(tmp_path)
    got = export_cases(inputs, sources)
    assert set(got) == {"schema_version", "source_pins", "cases"}
    assert got["source_pins"] == binding["source_pins"]
    assert got["cases"] == [{"case_id": "sample.itf.json::0", "fixture_id": "canonical-swap-v1",
        "trace_id": "sample.itf.json", "step_index": 0, "request": record["request"],
        "result": record["payload"]["value"]["raw"], "projection": record["payload"]["value"]["projection"],
        "effects": [], "provenance": [{"trace_id": "sample.itf.json", "state_index": i, "step_index": 0,
            "input_path": "sample.itf.json", "input_sha256": binding["input_pins"]["sample.itf.json"]}
            for i in (1, 2)]}]


@pytest.mark.parametrize("variable,fixture", [("installmentTrace", "installment-two-when-v1"),
                                            ("diagnosticCases", "close-v1")])
def test_other_generator_records(tmp_path, variable, fixture):
    inputs, sources, _, _, _ = setup_bundle(tmp_path, variable)
    case = export_cases(inputs, sources)["cases"][0]
    assert case["fixture_id"] == fixture
    assert case["case_id"] == "sample.itf.json::" + ("literal-case" if variable == "diagnosticCases" else "0")


def test_no_cross_trace_dedup_and_deterministic_order(tmp_path):
    inputs, sources, doc, binding, _ = setup_bundle(tmp_path)
    write_json(inputs / "another.itf.json", doc)
    binding["input_pins"]["another.itf.json"] = sha256((inputs / "another.itf.json").read_bytes()).hexdigest()
    write_json(inputs / "source-bindings.json", binding)
    got = export_cases(inputs, sources)
    assert [case["trace_id"] for case in got["cases"]] == ["another.itf.json", "sample.itf.json"]
    assert got == export_cases(inputs, sources)


@pytest.mark.parametrize("mutation,pattern", [
    ("missing-binding", "binding"), ("missing-source", "source"), ("bad-source-hash", "source"),
    ("unknown-source", "source"), ("missing-generator", "generator"),
    ("bad-input-hash", "hash"), ("missing-input", "inventory"), ("extra-input", "inventory"),
    ("unsafe-path", "path"), ("schema-bool", "schema"),
])
def test_source_and_inventory_binding_fail_closed(tmp_path, mutation, pattern):
    inputs, sources, doc, binding, _ = setup_bundle(tmp_path)
    if mutation == "missing-binding":
        (inputs / "source-bindings.json").unlink()
    elif mutation == "missing-source":
        binding["source_pins"].pop("moriarty/core.py")
    elif mutation == "bad-source-hash":
        binding["source_pins"]["moriarty/core.py"] = "0" * 64
    elif mutation == "unknown-source":
        binding["source_pins"]["unknown.py"] = "0" * 64
    elif mutation == "missing-generator":
        binding["source_pins"].pop(GENERATORS["swapTrace"][0])
    elif mutation == "bad-input-hash":
        binding["input_pins"]["sample.itf.json"] = "0" * 64
    elif mutation == "missing-input":
        binding["input_pins"]["missing.itf.json"] = "0" * 64
    elif mutation == "extra-input":
        write_json(inputs / "extra.itf.json", doc)
    elif mutation == "unsafe-path":
        binding["input_pins"]["../escape.itf.json"] = "0" * 64
    elif mutation == "schema-bool":
        binding["schema_version"] = True
    if mutation != "missing-binding":
        write_json(inputs / "source-bindings.json", binding)
    with pytest.raises(ExportError, match=pattern):
        export_cases(inputs, sources)


@pytest.mark.parametrize("mutation,pattern", [
    ("diagnostic", "diagnostic"), ("missing-payload", "payload"), ("missing-result", "payload"),
    ("missing-state-field", "state"), ("missing-projection-field", "projection"),
    ("effects-not-list", "effects"), ("conflicting-prefix", "conflicting"),
    ("shortened-history", "history"), ("missing-variable", "variable"),
    ("unknown-variable", "variable"), ("wrong-source", "source"), ("empty-corpus", "empty"),
])
def test_malformed_or_conflicting_records_fail_closed(tmp_path, mutation, pattern):
    inputs, sources, doc, binding, _ = setup_bundle(tmp_path)
    record = doc["states"][1]["swapTrace"]["records"][0]
    if mutation == "diagnostic":
        record["payload"] = tag("SwapComputationDiagnosticA", tag("OutsideModelDomainA"))
    elif mutation == "missing-payload":
        record.pop("payload")
    elif mutation == "missing-result":
        record["payload"]["value"].pop("raw")
    elif mutation == "missing-state-field":
        record["payload"]["value"]["raw"]["state"].pop("choices")
    elif mutation == "missing-projection-field":
        record["payload"]["value"]["projection"].pop("reductions")
    elif mutation == "effects-not-list":
        record["payload"]["value"]["effects"] = {}
    elif mutation == "conflicting-prefix":
        doc["states"][2]["swapTrace"]["records"][0]["payload"]["value"]["raw"]["reductions"] = {"#bigint": "2"}
    elif mutation == "shortened-history":
        doc["states"][2]["swapTrace"]["records"] = []
    elif mutation == "missing-variable":
        doc["states"][1].pop("swapTrace")
    elif mutation == "unknown-variable":
        doc["vars"] = ["mystery"]
    elif mutation == "wrong-source":
        doc["#meta"]["source"] = "unbound.qnt"
    elif mutation == "empty-corpus":
        doc["states"] = doc["states"][:1]
    update_input(inputs, doc, binding)
    with pytest.raises(ExportError, match=pattern):
        export_cases(inputs, sources)


def test_unknown_fixture_is_not_accepted_as_oracle(tmp_path):
    inputs, sources, doc, binding, _ = setup_bundle(tmp_path, "diagnosticCases")
    doc["states"][0]["diagnosticCases"][0]["fixture_id"] = "arbitrary-program"
    update_input(inputs, doc, binding)
    with pytest.raises(ExportError, match="fixture"):
        export_cases(inputs, sources)


def test_json_duplicate_keys_rejected(tmp_path):
    inputs, sources, _, binding, _ = setup_bundle(tmp_path)
    (inputs / "sample.itf.json").write_text('{"vars":[],"vars":[]}', encoding="utf-8")
    binding["input_pins"]["sample.itf.json"] = sha256((inputs / "sample.itf.json").read_bytes()).hexdigest()
    write_json(inputs / "source-bindings.json", binding)
    with pytest.raises(ExportError, match="duplicate"):
        export_cases(inputs, sources)


def test_cli_rejects_before_writing_output(tmp_path):
    inputs, sources, _, _, _ = setup_bundle(tmp_path)
    (inputs / "source-bindings.json").unlink()
    output = tmp_path / "cases.json"
    script = Path(__file__).resolve().parents[1] / "scripts/export_s02_candidate_a_cases.py"
    result = subprocess.run([sys.executable, str(script), "--itf-dir", str(inputs),
        "--source-root", str(sources), "--out", str(output)], capture_output=True, text=True)
    assert result.returncode == 1 and not output.exists()


def test_existing_output_is_not_overwritten(tmp_path):
    inputs, sources, _, _, _ = setup_bundle(tmp_path)
    output = tmp_path / "cases.json"
    output.write_bytes(b"preserve")
    script = Path(__file__).resolve().parents[1] / "scripts/export_s02_candidate_a_cases.py"
    result = subprocess.run([sys.executable, str(script), "--itf-dir", str(inputs),
        "--source-root", str(sources), "--out", str(output)], capture_output=True, text=True)
    assert result.returncode == 1 and output.read_bytes() == b"preserve"
