"""UNRUN proposed full-bundle predicate. Inputs must be independently frozen first.
No compiler/parser/SDK/evaluator invocation; failure is explicit for missing files.
This only compares supplied bytes and cannot establish oracle independence.
"""
import json
import sys
from pathlib import Path

expected = Path(sys.argv[1])
actual = Path(sys.argv[2])
want = (expected / "artifact.json").read_bytes()
got = (actual / "artifact.json").read_bytes()
assert got == want, "complete artifact.json byte mismatch"
assert not want.endswith(b"\n"), "unexpected final newline"
artifact = json.loads(want)
assert artifact["resources"]["artifact_bytes"] == len(want)
leaves = {"artifact.json", "source-projection.json"}
for index, circuit in enumerate(artifact["circuits"]):
    leaf = f"{index:03}-{circuit['action']}.zkir"
    leaves.add(leaf)
    ir = (expected / leaf).read_bytes()
    assert ir == circuit["zkir"].encode("utf-8"), "golden bundle inconsistent"
    assert (actual / leaf).read_bytes() == ir, leaf
assert {p.name for p in actual.iterdir()} == leaves, "unexpected or missing output leaf"
print("complete supplied Artifact and every circuit file match frozen bytes")
