from scripts.s02_candidate_a_integrated_inventory import inventory, instructions

def test_exact_inventory():
    cases = inventory()["cases"]
    assert len(cases) == 78
    assert sum(c["event_count"] for c in cases) == 1557
    assert sum(c["lifecycle"] == "installment" for c in cases) == 32
    assert sum(c["event_count"] for c in cases if c["lifecycle"] == "installment") == 638
    assert sum(c["event_count"] for c in cases if c["lifecycle"] == "swap") == 919
    for index in range(0, 78, 2):
        assert cases[index]["profile"] == "SignAfterResolve"
        assert cases[index + 1]["profile"] == "SignBeforeResolve"
        assert cases[index]["scenario"] == cases[index + 1]["scenario"]
        assert cases[index]["control"] == cases[index + 1]["control"]

def test_mutant_dual_boundaries_and_original_bases():
    selected = [(d, steps) for d, steps in instructions() if
                d["control"] in ("unused-successor", "reversed-effects", "reductions", "neutral-chooser")]
    assert len(selected) == 10
    for desc, steps in selected:
        assert steps.count("M:proposed") == steps.count("M:verified") == 1
        assert steps.index("M:proposed") < steps.index("M:verified")
        base = 3 if desc["lifecycle"] == "installment" else 16
        assert steps[base] in ("I:propose:first", "S:propose:disposition")
        assert steps[steps.index("M:verified") - 1].startswith("C:reject:")

import pytest
from scripts.s02_candidate_a_integrated_inventory import checked_quint_instruction, quint_cases

@pytest.mark.parametrize("selector", ["P:invented:first", "M:invented", "S:advance:99", "D:verify:first:foreign"])
def test_unknown_selector_rejected(selector):
    with pytest.raises(ValueError):
        checked_quint_instruction(selector)

def test_every_selector_lowers_and_case_table_is_literal():
    for _, steps in instructions():
        for selector in steps[1:-1]:
            assert checked_quint_instruction(selector)
    text = quint_cases()
    assert text.count("descriptor: {") == 78
    assert "var " not in text


def test_all_literal_case_and_wrapper_bytes_match_renderer():
    from pathlib import Path
    from scripts.s02_candidate_a_integrated_inventory import case_wrapper
    qnt = Path(__file__).resolve().parents[1] / "specs/quint/s02"
    assert (qnt / "candidate_a_integrated_cases.qnt").read_text() == quint_cases()
    actual = {p.name for p in qnt.glob("candidate_a_integrated_case_*.qnt")}
    assert actual == {f"candidate_a_integrated_case_{i:03d}.qnt" for i in range(78)}
    template = (qnt / "candidate_a_integrated_driver.qnt").read_text()
    template_body = template.split("\n", 1)[1][:-2]
    for i in range(78):
        text = (qnt / f"candidate_a_integrated_case_{i:03d}.qnt").read_text()
        assert text == case_wrapper(i)
        body = text.split("\n", 2)[2][:-2]
        table = "INSTALLMENT_CASES_A4" if i < 32 else "SWAP_CASES_A4"
        local = i if i < 32 else i - 32
        body = body.replace(f"pure val CASE_A4: A4Case = {table}.nth({local})\n", "const CASE_A4: A4Case\n")
        body = body.replace(f"pure val CASE_INDEX_A4: int = {i}\n", "const CASE_INDEX_A4: int\n")
        assert body == template_body


@pytest.mark.parametrize("index", [-1, 78, True, "32", None])
def test_wrapper_renderer_rejects_nonfinite_indices(index):
    from scripts.s02_candidate_a_integrated_inventory import case_wrapper
    with pytest.raises(ValueError, match="fixed case index"):
        case_wrapper(index)


def test_recorder_artifact_hash_uses_bounded_reads():
    import hashlib
    import io
    from scripts.record_s02_candidate_a_integrated import artifact_hash
    payload = b"original native bytes" * 100_001
    sizes = []
    class GuardedReader(io.BytesIO):
        def read(self, size=-1):
            assert 0 < size <= 1_048_576, "whole/oversized native artifact read"
            sizes.append(size)
            return super().read(size)
    class GuardedPath:
        def read_bytes(self):
            raise AssertionError("whole native artifact read_bytes")
        def open(self, mode):
            assert mode == "rb"
            return GuardedReader(payload)
    assert artifact_hash(GuardedPath()) == hashlib.sha256(payload).hexdigest()
    assert len(sizes) >= 3 and all(size == 1_048_576 for size in sizes)


def test_recorder_main_publishes_streamed_artifact_hashes():
    import ast
    import inspect
    import scripts.record_s02_candidate_a_integrated as recorder
    tree = ast.parse(inspect.getsource(recorder.main))
    assignments = {target.id: node.value for node in ast.walk(tree)
                   if isinstance(node, ast.Assign) for target in node.targets
                   if isinstance(target, ast.Name)}
    artifact_map = assignments["artifacts"]
    assert isinstance(artifact_map, ast.DictComp)
    assert ast.dump(artifact_map.value) == ast.dump(ast.parse("artifact_hash(path)", mode="eval").body)
    assert ast.dump(artifact_map.generators[0].iter) == ast.dump(
        ast.parse('sorted(stage.glob("*.itf.json"))', mode="eval").body)
    receipt = assignments["receipt"]
    assert isinstance(receipt, ast.Dict)
    published = {key.value: value for key, value in zip(receipt.keys, receipt.values)
                 if isinstance(key, ast.Constant)}
    assert isinstance(published["artifacts"], ast.Name) and published["artifacts"].id == "artifacts"


from scripts.export_s02_candidate_a_integrated import ExportError, validate_inventory

def test_dropped_whole_case_is_rejected():
    fixed = inventory()["cases"]
    with pytest.raises(ExportError, match="case inventory"):
        validate_inventory(fixed[:-1], fixed)

def test_reordered_profiles_are_rejected():
    fixed = inventory()["cases"]
    changed = list(fixed)
    changed[0], changed[1] = changed[1], changed[0]
    with pytest.raises(ExportError, match="case inventory"):
        validate_inventory(changed, fixed)


from copy import deepcopy
from scripts.export_s02_candidate_a_integrated import (load, integer, frozen, safe, fields,
    Types, validate_computations, validate_inventory, ExportError)

@pytest.mark.parametrize("data", [b'{"x":1,"x":2}', b'{"x":NaN}', b'{"x":Infinity}'])
def test_duplicate_or_nonfinite_json(data):
    with pytest.raises(ExportError): load(data)

@pytest.mark.parametrize("value", [True, 0, {"#bigint": "00"}, {"#bigint": "-0"}, {"#bigint": True}])
def test_noncanonical_integer(value):
    with pytest.raises(ExportError): integer(value)

@pytest.mark.parametrize("value", [
    {"#set": [{"#bigint": "0"}, {"#bigint": "0"}]},
    {"#map": [[{"tag": "Alice", "value": {"#tup": []}}, True],
              [{"value": {"#tup": []}, "tag": "Alice"}, False]]},
    {"#map": [["x"]]}, {"#unknown": []},
])
def test_duplicate_or_foreign_itf(value):
    with pytest.raises(ExportError): frozen(value)

def test_absence_is_not_zero():
    absent = {"tag": "NoInt", "value": {"#tup": []}}
    zero = {"tag": "IntValue", "value": {"#bigint": "0"}}
    assert frozen(absent) != frozen(zero)

@pytest.mark.parametrize("name", ["../x", "/x", "a/../x", "a//x", "a\\x", "./x"])
def test_unsafe_path(tmp_path, name):
    with pytest.raises(ExportError): safe(tmp_path, name)

def test_symlink_member_rejected(tmp_path):
    (tmp_path / "real").mkdir()
    (tmp_path / "alias").symlink_to(tmp_path / "real", target_is_directory=True)
    with pytest.raises(ExportError, match="symlink"): safe(tmp_path, "alias/file")

def tiny_types():
    unit = {"kind": "tup", "fields": {"kind": "row", "fields": [], "other": {"kind": "empty"}}}
    sum_type = {"kind": "sum", "fields": {"kind": "row", "fields": [
        {"fieldName": "Only", "fieldType": unit}], "other": {"kind": "empty"}}}
    record = {"kind": "rec", "fields": {"kind": "row", "fields": [
        {"fieldName": "tagged", "fieldType": {"kind": "const", "name": "Tag"}}], "other": {"kind": "empty"}}}
    return Types({"modules": [{"declarations": [
        {"kind": "typedef", "name": "Tag", "type": sum_type},
        {"kind": "typedef", "name": "Record", "type": record}]}], "errors": []})

def test_unknown_tag_and_record_field_rejected():
    types = tiny_types()
    good = {"tagged": {"tag": "Only", "value": {"#tup": []}}}
    types.named("Record", good)
    for bad in ({**good, "foreign": True}, {"tagged": {"tag": "Foreign", "value": {"#tup": []}}}):
        with pytest.raises(ExportError): types.named("Record", bad)

def test_missing_computation_is_not_valid_denial():
    request = {"before": {}, "input": {}, "now": {}}
    raw = {"kind": "denied-probe", "arguments": {"command": {
        "tag": "CoreAcceptedProbeA4", "value": {"request": request}}}, "computations": []}
    with pytest.raises(ExportError, match="computation inventory"):
        validate_computations({}, raw)

import json

def test_fixed_shard_mapping_and_counts():
    from scripts.export_s02_candidate_a_integrated import shards, entries, VARS
    rows = shards()
    assert len(rows) == 78 and sum(row["event_count"] for row in rows) == 1557
    assert sum(row["event_count"] for row in rows[:32]) == 638
    assert sum(row["event_count"] for row in rows[32:]) == 919
    assert VARS == ("authorityState", "caseIndex", "cursor", "latestEvent")
    for i, row in enumerate(rows):
        assert row["global_index"] == i
        assert row["input_path"] == f"raw/case-{i:03d}.itf.json"
        assert row["case_path"] == f"cases/case-{i:03d}.json"
        assert row["entry"] == f"specs/quint/s02/candidate_a_integrated_case_{i:03d}.qnt"
        assert entries()[row["case_id"]] == row["entry"]
        assert row["lifecycle"] == ("installment" if i < 32 else "swap")

def test_per_shard_command_uses_exact_local_event_bound():
    from scripts.export_s02_candidate_a_integrated import shards, native_command
    for row in shards():
        command = native_command(row)
        i = row["global_index"]
        assert command[:3] == ["quint", "run", row["entry"]]
        assert f"--max-steps={row['event_count'] - 1}" in command
        assert command[-1] == f".superpowers/sdd/a4-producer-receipts/case-{i:03d}-export/case-{i:03d}.itf.json"

@pytest.mark.parametrize("suffix", [b" {}", b"x", b' {"events":[]}'])
def test_stored_case_reader_checks_strict_eof(tmp_path, suffix):
    from scripts.export_s02_candidate_a_integrated import stored_events, shards
    from scripts.a4_json_stream import JsonStreamError
    row = {**shards()[0], "event_count": 0}
    data = {key: row[key] for key in ("case_id", "lifecycle", "profile", "scenario", "control")}
    path = tmp_path / "case.json"
    path.write_bytes(json.dumps({**data, "events": []}).encode() + suffix)
    with pytest.raises(JsonStreamError):
        list(stored_events(path, row))

def test_stored_case_late_wrong_descriptor_fails(tmp_path):
    from scripts.export_s02_candidate_a_integrated import stored_events, shards
    row = {**shards()[0], "event_count": 0}
    data = {key: row[key] for key in ("case_id", "lifecycle", "profile", "scenario", "control")}
    path = tmp_path / "case.json"
    path.write_bytes(json.dumps({"events": [], **{**data, "profile": "wrong"}}).encode())
    with pytest.raises(ExportError, match="descriptor"):
        list(stored_events(path, row))

def test_metadata_native_order_is_exact():
    from scripts.export_s02_candidate_a_integrated import VARS
    assert list(VARS) == ["authorityState", "caseIndex", "cursor", "latestEvent"]
    assert list(VARS) != ["authorityState", "latestEvent", "caseIndex", "cursor"]

def test_stream_pin_hash_does_not_read_whole_file(tmp_path, monkeypatch):
    from scripts.export_s02_candidate_a_integrated import verify_pins, digest
    from pathlib import Path
    path = tmp_path / "member"
    path.write_bytes(b"complete original bytes")
    def forbidden(*args, **kwargs):
        raise AssertionError("whole-file read_bytes forbidden")
    monkeypatch.setattr(Path, "read_bytes", forbidden)
    verify_pins(tmp_path, {"member": digest(b"complete original bytes")})

def test_bounded_manifest_resource_failure_is_distinct(tmp_path):
    from scripts.export_s02_candidate_a_integrated import bounded_load
    from scripts.a4_json_stream import ResourceLimit
    path = tmp_path / "too-large.json"
    path.write_bytes(b" " * 17)
    with pytest.raises(ResourceLimit):
        bounded_load(path, limit=16)

def test_cli_existing_output_is_not_rewritten(tmp_path):
    from scripts.export_s02_candidate_a_integrated import main
    path = tmp_path / "existing.json"
    path.write_bytes(b"keep exact prior bytes")
    code = main(["--mode", "stage-cases", "--input-root", str(tmp_path), "--source-root", str(tmp_path),
                 "--parser-receipts", str(tmp_path / "parser"),
                 "--inventory", "unused", "--admission", "unused", "--output", str(path)])
    assert code == 1 and path.read_bytes() == b"keep exact prior bytes"

def test_capture_admission_never_satisfies_full_schema():
    from scripts.export_s02_candidate_a_integrated import fields
    capture = dict.fromkeys(("schema_version", "transport", "inventory_sha256", "source_pins",
                             "input_pins", "receipt_pins", "entries"))
    with pytest.raises(ExportError, match="exact fields"):
        fields(capture, (*capture, "case_pins"), "full admission")

@pytest.mark.parametrize("current", [{"pin": "replacement", "flag": True}, {"pin": "original", "flag": 1}])
def test_coherent_or_type_coercing_admission_replacement_fails(current):
    from scripts.export_s02_candidate_a_integrated import require_same_admission
    original = {"pin": "original", "flag": True}
    require_same_admission(original, dict(original))
    with pytest.raises(ExportError, match="admission changed"):
        require_same_admission(original, current)

@pytest.mark.parametrize("mode", ["nonzero", "oversize", "timeout"])
def test_parser_original_failure_streams_and_budgets_retained(tmp_path, monkeypatch, mode):
    # Synthetic subprocess stub: a receipt/transport unit, not a Quint execution.
    import sys
    from pathlib import Path
    from types import SimpleNamespace
    import scripts.export_s02_candidate_a_integrated as exporter
    executable = Path(sys.executable).resolve()
    tools = {name: {"path": str(executable), "sha256": exporter.file_digest(executable)}
             for name in ("node", "quint")}
    output = b"original parser stdout"
    error = b"original parser stderr" if mode == "nonzero" else b""
    def fake_run(argv, **kwargs):
        assert argv[1] == "--max-old-space-size=4096"
        assert kwargs["timeout"] == 900
        kwargs["stdout"].write(output)
        kwargs["stderr"].write(error)
        if mode == "timeout":
            raise exporter.subprocess.TimeoutExpired(argv, 900)
        return SimpleNamespace(returncode=7 if mode == "nonzero" else 0)
    monkeypatch.setattr(exporter.subprocess, "run", fake_run)
    if mode == "oversize":
        monkeypatch.setattr(exporter, "PARSER_LIMIT", 1)
    receipt_dir = tmp_path / "parser"
    expected = exporter.ExportError if mode == "nonzero" else exporter.ResourceLimit
    with pytest.raises(expected):
        exporter.parser_types(tmp_path, tools, receipt_dir)
    assert (receipt_dir / "stdout.bin").read_bytes() == output
    assert (receipt_dir / "stderr.bin").read_bytes() == error
    terminal = exporter.bounded_load(receipt_dir / "terminal.json")
    assert terminal["timeout_seconds"] == 900 and terminal["node_heap_mib"] == 4096
    assert terminal["subprocess_attempted"] is True
    assert terminal["exit_code"] == (None if mode == "timeout" else 7 if mode == "nonzero" else 0)
    assert terminal["failure"] == ("timeout" if mode == "timeout" else None)
    with pytest.raises(FileExistsError):
        exporter.parser_types(tmp_path, tools, receipt_dir)

from pathlib import Path
from scripts.export_s02_candidate_a_integrated import DESCRIPTOR

A4_ARCHIVE = Path("evidence/s02-candidate-a-completion/a4")

def test_actual_complete_sharded_corpus_matches_manifest(tmp_path):
    from scripts.export_s02_candidate_a_integrated import export_shards, bounded_load
    actual = export_shards("seal-manifest", A4_ARCHIVE, Path("."),
                           A4_ARCHIVE / "inventory.json", A4_ARCHIVE / "admission.json",
                           parser_receipts=tmp_path / "parser")
    assert actual == bounded_load(A4_ARCHIVE / "cases.json")
    assert len(actual["shards"]) == 78
    assert sum(row["event_count"] for row in actual["shards"]) == 1557

def test_actual_case_omission_cannot_pass_fixed_inventory():
    from scripts.export_s02_candidate_a_integrated import bounded_load, validate_inventory
    doc = bounded_load(A4_ARCHIVE / "cases.json")
    changed = [{key: row[key] for key in (*DESCRIPTOR, "event_count")} for row in doc["shards"]]
    changed.pop(3)
    with pytest.raises(ExportError, match="case inventory"):
        validate_inventory(changed, inventory()["cases"])

def test_actual_event_omission_cannot_pass_case_inventory(tmp_path):
    from scripts.export_s02_candidate_a_integrated import stored_events, shards, file_digest
    from scripts.a4_json_stream import write_array_document
    row = shards()[0]
    original = A4_ARCHIVE / row["case_path"]
    original_hash = file_digest(original)
    changed = tmp_path / "case-000-omitted-event.json"
    omitted = {"count": 0}
    def remaining_events():
        for index, event in enumerate(stored_events(original, row)):
            if index == 3:
                omitted["count"] += 1
            else:
                yield event
    with changed.open("xb") as stream:
        count = write_array_document(stream, "events", remaining_events(),
                                     before=[(key, row[key]) for key in DESCRIPTOR])
    assert omitted["count"] == 1 and count == row["event_count"] - 1
    with pytest.raises(ExportError, match="case event inventory"):
        for _ in stored_events(changed, row):
            pass
    assert file_digest(original) == original_hash

def test_actual_reordered_computation_requests_rejected():
    from scripts.export_s02_candidate_a_integrated import stored_events, shards, validate_computations
    found = False
    for row in shards():
        stream = stored_events(A4_ARCHIVE / row["case_path"], row)
        try:
            for event in stream:
                records = event["computations"]
                if len(records) == 2 and records[0]["request"] != records[1]["request"]:
                    raw = {"kind": event["kind"], "arguments": event["arguments"],
                           "computations": list(reversed(records))}
                    with pytest.raises(ExportError, match="binding/order"):
                        validate_computations(event["before"], raw)
                    found = True
                    break
        finally:
            stream.close()
        if found:
            break
    assert found, "actual two-distinct-computation witness missing"
