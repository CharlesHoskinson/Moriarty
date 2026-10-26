"""Check SHA-pinned public test observations with the evaluator's U031 checks.

The evaluator ran the Rust binary. This script only reads its two plan JSONs;
it does not execute Rust, emit artifacts, or exercise a native/target backend.
"""
import copy
import hashlib
import json
from pathlib import Path
import sys

from audit_u033_observation import checker_functions

EVALUATOR = "7f0942c3d"
CASES = (
    ("u031-scope-rule195-s9.json", "b99cf06f6e5901b539fcc8086357486826bea634dca1f3820ba16416659b2d4a",
     "24", "195", "uiRet(uiU(4))", ["ArenaRead", "ScopeRestore"]),
    ("u031-leftmul-rule149-s9.json", "ae6927815f6c81d8e2d2fb86c22c27ac179c2faa11e23b974ee1c8863f6d7fa6",
     "22", "149", "uiRet(uiU(3))", ["ArenaRead", "ArenaRead"]),
)


def main():
    if len(sys.argv) != 2:
        raise SystemExit("usage: audit_u031_observations.py EVALUATOR_SHARE_DIR")
    checks = checker_functions("qa/holdout/lib/plans.py", {"need", "canon_id", "_typed", "_check_plans"},
                               {"STATUS", "BC_KEYS"}, commit=EVALUATOR)
    check, digest = checks["_check_plans"], checks["canon_id"]
    results = []
    for filename, sha, position, rule, source_control, operations in CASES:
        raw = (Path(sys.argv[1]) / filename).read_bytes()
        assert hashlib.sha256(raw).hexdigest() == sha, filename
        ep = json.loads(raw)
        # Literal expected positions/rules were supplied by the evaluator;
        # returned values 3/4 follow from the public fact/let source fixtures.
        record = {"position": position, "rule_start_line": rule, "source_control": source_control}
        manifest, constraints, witness, _ = check(ep, record, filename, None, ("CheckedU128",))
        ops = manifest["operations"]
        assert [op["op"] for op in ops] == operations
        by_operation = {op["operation_id"]: op for op in ops}
        for op in ops:
            assert op["operation_id"] == digest(op, "operation_id")
            assert all(binding.get("source") and isinstance(binding.get("value"), str)
                       for binding in op["inputs"] + op["outputs"])
        for component in constraints["components"]:
            expected = hashlib.sha256(f"constraint-component:{component['operation_id']}".encode()).hexdigest()
            assert component["component_id"] == expected
            assert component["parameters"]["op"] == by_operation[component["operation_id"]]["op"]
        for node in witness["nodes"]:
            assert node["node_id"] == digest(node, "node_id")
            op = by_operation[node["operation_id"]]
            assert node["typed_inputs"] == op["inputs"] and node["typed_outputs"] == op["outputs"]
        primary = ops[1]
        if rule == "195":
            assert primary["inputs"] == [{"name": "return_value", "source": "before.control.return_value",
                                          "type": "UInt128", "value": "uiU(4)"}]
            assert primary["outputs"] == [{"name": "restored_value", "source": "after.control.return_value",
                                           "type": "UInt128", "value": "uiU(4)"}]
        else:
            assert primary["inputs"][0]["value"] == source_control
            assert primary["outputs"][0]["source"] == "after.control"
            assert primary["outputs"][0]["value"].startswith("uiEval(uiCall(fact,")
        # Probe requested no emissions; emitted file bytes are not checked here.
        assert ep["exports"] == []
        sensitivity = []

        def reject(label, bad, reason):
            try:
                check(bad, record, label, None, ("CheckedU128",))
            except AssertionError as error:
                assert reason in str(error), (label, str(error))
                sensitivity.append(label)
            else:
                raise AssertionError(f"mutation accepted: {label}")

        bad = copy.deepcopy(ep)
        bad["operation_manifest"]["operations"][1]["op"] = "CheckedU128Mul"
        bad["operation_manifest"]["manifest_id"] = digest(bad["operation_manifest"], "manifest_id")
        reject("false multiplication", bad, "operations this step does not perform")
        bad = copy.deepcopy(ep)
        bad["step"]["rule_start_line"] = "999"
        reject("wrong selected rule", bad, "not reference record")
        bad = copy.deepcopy(ep)
        bad["witness_plan"]["stable_topological_order"].reverse()
        bad["witness_plan"]["plan_id"] = digest(bad["witness_plan"], "plan_id")
        reject("reversed dependency order", bad, "not earlier")
        bad = copy.deepcopy(ep)
        bad["constraint_plan"]["components"] = []
        bad["constraint_plan"]["plan_id"] = digest(bad["constraint_plan"], "plan_id")
        reject("uncovered operations", bad, "neither constrained nor listed as omitted")
        bad = copy.deepcopy(ep)
        bad["witness_plan"]["operation_manifest_id"] = "0" * 64
        bad["witness_plan"]["plan_id"] = digest(bad["witness_plan"], "plan_id")
        reject("wrong shared manifest", bad, "do not share the manifest")
        results.append({"file": filename, "sha256": sha, "expected_step": record,
                        "operations": operations, "manifest_id": manifest["manifest_id"],
                        "constraint_plan_id": constraints["plan_id"], "witness_plan_id": witness["plan_id"],
                        "value_bindings_and_id_recipes_checked": True, "sensitivity_rejections": sensitivity})
    print(json.dumps({"evaluator_commit": EVALUATOR, "observations": results,
                      "raw_inputs_stay_external": True, "emitted_file_bytes_checked": False,
                      "rust_executed_by_this_audit": False, "native_or_target_acceptance": "not tested"}, indent=2))


if __name__ == "__main__":
    main()
