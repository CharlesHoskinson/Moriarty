"""Audit two evaluator-supplied observations using the pinned holdout checks.

No binary, network, proof or wallet operation is run. The observed JSON files
remain external; only hashes and a small audit result belong to this worktree.
"""
import ast
import copy
import hashlib
import json
from pathlib import Path
import re
import subprocess
import sys
from types import SimpleNamespace

CHECKER_COMMIT = "abe5ee146"
RESIDUAL_SHA256 = "558640fd9314e34b412cfc1ca189e574952a3d8e7001e60362e8a40b5ccc92ef"
REFERENCE_SHA256 = "ea4f3b2cfe1d99078ec0914b1a9cf0e1f90bf88a1a999da28e9c1e2b6e7cfb3a"


def checker_functions(path, functions, constants=()):
    source = subprocess.run(["git", "show", f"{CHECKER_COMMIT}:{path}"],
                            check=True, capture_output=True, text=True).stdout
    tree = ast.parse(source)
    nodes = [node for node in tree.body if
             isinstance(node, ast.FunctionDef) and node.name in functions or
             isinstance(node, ast.Assign) and any(isinstance(t, ast.Name) and t.id in constants for t in node.targets)]
    # These are the actual function/constant nodes, without rewriting checks.
    # Only the exception class is adapted; no engine run or oracle is injected.
    namespace = {"json": json, "hashlib": hashlib, "re": re, "Fail": AssertionError}
    exec(compile(ast.Module(body=nodes, type_ignores=[]), f"{CHECKER_COMMIT}:{path}", "exec"), namespace)
    return namespace


def observation(path, expected):
    raw = Path(path).read_bytes()
    assert hashlib.sha256(raw).hexdigest() == expected, f"observation hash changed: {path}"
    return json.loads(raw)


def main():
    if len(sys.argv) != 4:
        raise SystemExit("usage: audit_u033_observation.py RESIDUAL.json REFERENCE.json PROBE.py")
    residual_doc = observation(sys.argv[1], RESIDUAL_SHA256)
    reference_doc = observation(sys.argv[2], REFERENCE_SHA256)
    probe = ast.parse(Path(sys.argv[3]).read_text())
    sources = [ast.literal_eval(n.value) for n in probe.body if isinstance(n, ast.Assign)
               and any(isinstance(t, ast.Name) and t.id == "src" for t in n.targets)]
    assert len(sources) == 1
    source_hash = hashlib.sha256(sources[0].encode()).hexdigest()
    plans = checker_functions("qa/holdout/lib/plans.py",
                              {"need", "canon_id", "_ref_rule_charges", "_check_ir", "_residual_exec"},
                              {"HEX64", "FOLDABLE", "DYNAMIC", "BLOCK_KINDS", "IR_KEYS", "REALIZATION_KIND"})
    slices = checker_functions("qa/holdout/lib/uireq.py", {"_flat", "_check_slices"})
    slices["need"] = plans["need"]
    plans["U"] = SimpleNamespace(_flat=slices["_flat"], _check_slices=slices["_check_slices"])
    ref = reference_doc["reference_slices"]
    fuel, stack = int(ref["fuel"]), int(ref["stack_limit"])
    ref_records = slices["_check_slices"](ref, 1, fuel, stack, "external reference")
    residual = residual_doc["residual"]
    ir = plans["_check_ir"](residual, "per-program", None, "external residual", ref_records)
    records = plans["_residual_exec"](residual, "external residual", fuel, stack, ref)
    assert ir["program_image_id"] == source_hash
    assert residual_doc["execution"]["summary"] == reference_doc["execution"]["summary"]
    assert ref["outcome"] == {"kind": "Done", "value": "uiU(6)"}
    # The complete snapshots, beyond the evaluator's flattened projection.
    for candidate, baseline in zip(residual["execution"]["slices"], ref["slices"], strict=True):
        assert candidate["start"] == baseline["start"] and candidate["end"] == baseline["end"]
    assert len(ir["rule_boundary_map"]) == 39
    assert len({r["rule_start_line"] for r in ir["rule_boundary_map"]}) == 39

    sensitivity = []

    def rejected(label, bad, reason, ir_only=False):
        try:
            if ir_only:
                plans["_check_ir"](bad, "per-program", None, label, ref_records)
            else:
                plans["_residual_exec"](bad, label, fuel, stack, ref)
        except AssertionError as error:
            assert reason in str(error), (label, str(error))
            sensitivity.append({"mutation": label, "rejected": True, "reason": str(error)})
        else:
            raise AssertionError(f"mutation was accepted: {label}")

    bad = copy.deepcopy(residual)
    bad["execution"]["slices"][0]["records"][0]["block_id"] = "absent"
    rejected("unknown producing block", bad, "names no residual block")
    bad = copy.deepcopy(residual)
    bad["execution"]["slices"][0]["records"][0]["block_id"] = "scalar"
    rejected("wrong producing block", bad, "does not realize")
    bad = copy.deepcopy(residual)
    call = next(b for b in bad["ir"]["blocks"] if b["block_id"] == "call")
    call["successors"].remove("frame")
    rejected("missing exercised call-to-frame edge", bad, "not a residual successor")
    bad = copy.deepcopy(residual)
    bad["execution"]["slices"][0]["records"][0]["source_control"] = "uiRet(uiU(999))"
    rejected("altered reference record", bad, "differs from the reference stream")
    bad = copy.deepcopy(residual)
    bad["ir"]["ir_id"] = "0" * 64
    rejected("wrong canonical IR hash", bad, "canonical IR", ir_only=True)
    print(json.dumps({
        "checker_commit": CHECKER_COMMIT,
        "residual_sha256": RESIDUAL_SHA256, "reference_sha256": REFERENCE_SHA256,
        "program_image_id": source_hash, "ir_id": ir["ir_id"],
        "blocks": len(ir["blocks"]), "rule_boundaries": len(ir["rule_boundary_map"]),
        "records": len(records), "reference_work": ref["reference_work"], "outcome": ref["outcome"],
        "fuel": fuel, "stack_limit": stack,
        "reference_stream_equal": True, "complete_boundary_snapshots_equal": True,
        "host_summary_unchanged": True, "sensitivity_cases": sensitivity,
        "external_execution_claim": "evaluator's same saved s5 abc64975d binary; reference companion freshly run",
        "rust_executed_by_this_audit": False, "native_or_target_acceptance": "not tested",
    }, indent=2))


if __name__ == "__main__":
    main()
