"""Root read-only intake of original Task2 producer receipts.

Final native typecheck/test stages are required only with --require-complete.
This does not admit native pilot/export or Task3.
"""
import argparse
import hashlib
import json
from pathlib import Path
import re

ROOT = Path.cwd()  # Run from the experimental repository root.
BASE = ROOT / ".superpowers/sdd/a4-producer-receipts"
parser = argparse.ArgumentParser()
parser.add_argument("--require-complete", action="store_true")
args = parser.parse_args()


def sha(path):
    h = hashlib.sha256()
    with path.open("rb") as stream:
        while block := stream.read(1048576):
            h.update(block)
    return h.hexdigest()


def read(path):
    return json.loads(path.read_bytes())


expected = {
    "task2-renderer-red": 1, "task2-renderer-green": 0, "task2-case-types": 0,
    "task2-aggregate-parse": 0, "task2-boundaries-red": 1,
    "task2-instance-probe-test": 1, "task2-instance-probe-run": 1,
    "task2-literal-probe-test": 0, "task2-literal-probe-run": 0,
    "task2-literal-aggregate-parse": 0, "task2-boundaries-red-literal": 1,
    "task2-boundaries-red-independent": 1, "task2-python-green": 0,
}
pending = []
for final in ("task2-all-wrappers-types", "task2-quint-green"):
    if (BASE / final / "receipt.json").exists():
        expected[final] = 0
    else:
        pending.append(final)
stages = {}
runtime = None
members = 0
for name, code in expected.items():
    stage = BASE / name
    r = read(stage / "receipt.json")
    assert r["exit_code"] == r["recorder_exit_code"] == code
    assert r["cwd"] == str(ROOT)
    assert r["source_stable"] and r["runtime_stable"]
    assert r["beforeDispatchCommit"] == "386bf0ae10f767e051b414a7231be105cc4b0f71"
    assert r["sources_before"] == r["sources_after"]
    assert r["not_yet_created_before"] == r["not_yet_created_after"]
    assert r["runtime_before"] == r["runtime_after"]
    assert r["shared_runtime_before"] == r["shared_runtime_after"]
    assert r["runtime_archive_sha256"] == "f0687656d30c0d59ece8dfd027508a9228020506d29b97aea3abe7568186c31c"
    assert r["runtime_manifest_sha256"] == "fa3e9838c66480557ed6d928a2d159ebc074d05f67f15ef08ab20388d9c49e0b"
    assert r["shared_runtime_before"]["beforeDispatchCommit"] == "900bb2051225b4a3d99bf422c3b2e5e386e3e7bc"
    if runtime is None:
        runtime = r["runtime_before"]
    assert runtime == r["runtime_before"]
    assert r["elapsed_ns"] == r["ended_ns"] - r["started_ns"] > 0
    assert not r["python_bytecode_writes"]
    assert r["python_cache_prefix"] == str(stage / "python-cache")
    assert sha(stage / "runtime-helper.py") == "816c3ad79dc3a67ca9a03729af56d74188c161a7546b9c43e821c222a4bce7f2"
    for when in ("before", "after"):
        closure = read(stage / when / "closure.json")
        assert closure == {"sources": r["sources_" + when],
                           "not_yet_created": r["not_yet_created_" + when]}
        actual_files = {str(p.relative_to(stage / when / "source"))
                        for p in (stage / when / "source").rglob("*") if p.is_file()}
        assert actual_files == set(closure["sources"])
        for path, digest in closure["sources"].items():
            assert sha(stage / when / "source" / path) == digest
            members += 1
    for stream in ("stdout", "stderr"):
        assert sha(stage / (stream + ".bin")) == r[stream + "_sha256"]
    assert {p.name: sha(p) for p in stage.glob("*.itf.json")} == r["artifacts"]
    for tool in r["tools"].values():
        assert runtime["files"][tool["path"]] == tool["sha256"]
    if r["command"][0] == "quint":
        assert r["executed_command"] == [r["tools"]["node"]["path"],
                                          r["tools"]["quint"]["path"], *r["command"][1:]]
    stages[name] = {"exitCode": code, "elapsedNanoseconds": r["elapsed_ns"],
        "sourceFiles": len(r["sources_before"]), "receiptSha256": sha(stage / "receipt.json")}

red = read(BASE / "task2-boundaries-red-independent/receipt.json")
green = read(BASE / "task2-python-green/receipt.json")
assert len(red["sources_before"]) == len(green["sources_before"]) == 120
changes = {p for p in red["sources_before"] if red["sources_before"][p] != green["sources_before"][p]}
lowering = "specs/quint/s02/candidate_a_integrated_lowering.qnt"
wrapper = "specs/quint/s02/candidate_a_integrated_case_032.qnt"
assert changes == {lowering, wrapper}
red_source = BASE / "task2-boundaries-red-independent/before/source"
before = (red_source / lowering).read_text()
fault = "(match args.command { | VerifyA4(v) => canVerify(state, v.id, v.evidence) | _ => directGuardA4(state, args.command) })"
assert before.count(fault) == 1
assert before.replace(fault, "directGuardA4(state, args.command)", 1) == (ROOT / lowering).read_text()
before = (red_source / wrapper).read_text()
assert before.count("pure val CASE_INDEX_A4: int = 31\n") == 1
assert before.replace("pure val CASE_INDEX_A4: int = 31\n", "pure val CASE_INDEX_A4: int = 32\n", 1) == (ROOT / wrapper).read_text()
for path, digest in green["sources_before"].items():
    assert sha(ROOT / path) == digest
for final in ("task2-all-wrappers-types", "task2-quint-green"):
    if final in pending:
        continue
    receipt = read(BASE / final / "receipt.json")
    assert receipt["sources_before"] == green["sources_before"]
    if final.endswith("types"):
        assert receipt["command"] == ["quint", "typecheck",
            "specs/quint/s02/candidate_a_integrated_wrappers_typecheck.qnt"]
        aggregate = (ROOT / receipt["command"][2]).read_text()
        imports = re.findall(r"(?m)^\s*import (candidate_a_integrated_case_\d{3})\b", aggregate)
        assert imports == [f"candidate_a_integrated_case_{i:03d}" for i in range(78)]
        assert 'import candidate_a_integrated_driver as DriverTemplateA4 from "./candidate_a_integrated_driver"' in aggregate
    else:
        assert receipt["command"] == ["quint", "test",
            "specs/quint/s02/candidate_a_integrated_export_test.qnt", "--backend=rust",
            "--seed=42"]  # Actual unfiltered command: all 12 source runs follow.
        names = re.findall(r"(?m)^run (\w+) =", (ROOT / receipt["command"][2]).read_text())
        assert len(names) == 12
        passing = (BASE / final / "stdout.bin").read_text()
        assert re.findall(r"ok (\w+) passed \d+ test\(s\)", passing) == names
        assert re.findall(r"(?m)^\s*(\d+) passing\b", passing) == ["12"]
stdout = (BASE / "task2-boundaries-red-independent/stdout.bin").read_text()
assert re.findall(r"Error \[(QNT\d+)\]", stdout) == ["QNT508", "QNT508"]
assert "dualMutationBoundariesTest failed after 1 test(s)" in stdout
assert "nativeCase032InitialOrdinalTest failed after 1 test(s)" in stdout
assert "Assertion failed" in stdout and "Expect condition does not hold true" in stdout
assert "QNT404" in (BASE / "task2-boundaries-red/stderr.bin").read_text()
assert "QNT202" in (BASE / "task2-boundaries-red-literal/stderr.bin").read_text()
for name in ("task2-instance-probe-test", "task2-instance-probe-run"):
    assert "QNT500" in (BASE / name / "stdout.bin").read_text() + (BASE / name / "stderr.bin").read_text()
assert "13 passed" in (BASE / "task2-python-green/stdout.bin").read_text()
renderer_red = (BASE / "task2-renderer-red/stdout.bin").read_text()
assert "1 failed, 6 passed" in renderer_red and "AssertionError: assert ''" in renderer_red
assert "test_every_selector_lowers_and_case_table_is_literal" in renderer_red
assert sha(BASE / "task2-literal-probe-run/probe.itf.json") == "7b14b09c806313f38548a689f1c909033d6fbb7182350ef82c70c4932744aaf4"
for path, digest in runtime["files"].items():
    assert sha(Path(path)) == digest
for directory, names in runtime["treeMembers"].items():
    assert sorted({str(p.resolve()) for p in Path(directory).rglob("*") if p.is_file()}) == names
if args.require_complete:
    assert not pending, pending
print(json.dumps({"ok": True, "scope": "Task2-original-receipt-intake-only-no-pilot-or-export",
    "stagesAudited": len(stages), "originalSourceMembersAudited": members,
    "runtimePins": len(runtime["files"]), "twoBehavioralFailures": True,
    "soleCorrections": sorted(changes), "pythonTests": 13,
    "pendingNativeCommands": pending, "stages": stages}, indent=2))
