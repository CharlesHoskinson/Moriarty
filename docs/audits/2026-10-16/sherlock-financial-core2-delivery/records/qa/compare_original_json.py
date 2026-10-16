"""Compare retained normal outputs directly with original independent O JSON.

Read-only over subject captures and O. No Rust, parser/evaluator imports,
translation, replay, resource policy or subject execution.
"""
from pathlib import Path
import hashlib
import json

ROOT = Path(__file__).resolve().parent
ORIGINAL = ROOT.parent / "financial-core2-oracles-01"
ACTUAL = ROOT / "attempt-financial-core2-reuse-01"


def identity(path):
    raw = path.read_bytes()
    return {"path": str(path), "bytes": len(raw), "sha256": hashlib.sha256(raw).hexdigest()}


def first_difference(actual, expected, path="$"):
    if type(actual) is not type(expected):
        return path + ": JSON value types differ"
    if isinstance(expected, dict):
        if set(actual) != set(expected):
            return path + ": object members differ"
        for key in expected:
            difference = first_difference(actual[key], expected[key], path + "/" + key)
            if difference:
                return difference
    elif isinstance(expected, list):
        if len(actual) != len(expected):
            return path + ": array lengths differ"
        for index, (left, right) in enumerate(zip(actual, expected, strict=True)):
            difference = first_difference(left, right, path + "/" + str(index))
            if difference:
                return difference
    elif actual != expected:
        return path + ": scalar values differ"
    return None


def compare():
    result = json.loads((ACTUAL / "result.json").read_text())
    output_path = Path(result["steps"][-1]["stdout"]["path"])
    stdout = output_path.read_text()
    records = []
    prefix = "test financial_core2_consumer::independent_core2_financial_two_programs_match_complete_frozen_views ... "
    for line in stdout.splitlines():
        if line.startswith(prefix):
            line = line[len(prefix):]
        if line.startswith("{"):
            records.append(json.loads(line))
    expected = json.loads((ORIGINAL / "expected.json").read_text())
    inputs = json.loads((ORIGINAL / "inputs.json").read_text())
    comparisons = []

    def equal(name, actual, wanted):
        difference = first_difference(actual, wanted)
        comparisons.append({"name": name, "equal": difference is None, "first_difference": difference})

    equal("case membership", [r["case"] for r in records if r.get("operation") == "load"],
          [case["case_id"] for case in expected["cases"]])
    for case, command in zip(expected["cases"], inputs["cases"], strict=True):
        case_id = case["case_id"]
        case_records = [r for r in records if r["case"] == case_id]
        loads = [r for r in case_records if r.get("operation") == "load"]
        equal(case_id + " load count", len(loads), 1)
        load = loads[0]
        for actual_key, expected_key in [("actual_program", "program_ir"), ("actual_plan", "stage_plan"),
                                          ("actual_program_hash", "program_hash"), ("actual_plan_hash", "plan_hash")]:
            equal(case_id + "/" + expected_key, load[actual_key], case[expected_key])
        equal(case_id + "/labels", load["labels"],
              [[key, value] for key, value in expected["existing_cli_labels"].items()])
        invocations = [r for r in case_records if r.get("operation") in ("start", "resume")]
        equal(case_id + "/invocation sequence", [(r["id"], r["operation"]) for r in invocations],
              [(r["id"], r["operation"]) for r in case["invocations"]])
        # The sequence comparison is a Python tuple comparison, not JSON data;
        # recursive equality still compares both string fields in exact order.
        debug = [r for r in case_records if "actual_stage_projection_debug" in r]
        equal(case_id + "/projection-record sequence", [r["id"] for r in debug],
              [r["id"] for r in case["invocations"]])
        for actual, wanted, call in zip(invocations, case["invocations"], command["sequence"], strict=True):
            name = case_id + "/" + wanted["id"]
            if wanted["operation"] == "start":
                equal(name + "/complete result", actual["actual_result"], {"Ok": wanted["expected"]})
                equal(name + "/action", actual["action"], command["action"])
                equal(name + "/cap", actual["cap"], command["start_cap"])
                equal(name + "/scenario", actual["scenario"], json.loads((ORIGINAL / command["scenario"]).read_text()))
            else:
                resume = wanted["expected"]
                equal(name + "/complete result", actual["actual_result"], {"Ok": resume["outcome"]})
                equal(name + "/complete input", actual["actual_input"], resume["input"])
                equal(name + "/complete event", actual["actual_event"], resume["event"])
                equal(name + "/command event", actual["actual_event"], json.loads((ORIGINAL / call["event"]).read_text()))
                equal(name + "/cli_action", actual["actual_cli_action"], wanted["cli_action"])
                equal(name + "/cli_cap", actual["actual_cli_cap"], wanted["cli_cap"])
                equal(name + "/command cli_action", actual["actual_cli_action"], call["cli_action"])
                equal(name + "/command cli_cap", actual["actual_cli_cap"], call["cli_cap"])
                equal(name + "/continuation_from", actual["continuation_from"], wanted["continuation_from"])
    return {
        "scope": "direct complete normal JSON values against original independent O; no typed translation reused",
        "status": "pass" if all(row["equal"] for row in comparisons) else "mismatch",
        "inputs": [identity(ORIGINAL / "SOURCE-FREEZE.json"), identity(ORIGINAL / "expected.json"),
                   identity(ORIGINAL / "inputs.json"), identity(output_path), identity(ACTUAL / "result.json")],
        "comparator": identity(Path(__file__)),
        "json_record_count": len(records),
        "complete_invocations": sum(len(c["invocations"]) for c in expected["cases"]),
        "complete_resume_inputs": sum(i["operation"] == "resume" for c in expected["cases"] for i in c["invocations"]),
        "comparison_count": len(comparisons),
        "comparisons": comparisons,
        "limits": ["Debug view strings are not a separate wire ABI and are not independently interpreted here",
                   "Typed projection assertions remain separately evidenced by the exact test output",
                   "No rejection, non-null CLI override, CLI file decoder or native/target/Sherlock lowering result"],
    }


if __name__ == "__main__":
    report = compare()
    destination = ROOT / "DIRECT-ORIGINAL-JSON-COMPARISON.json"
    destination.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({key: report[key] for key in ["status", "json_record_count", "complete_invocations",
                                                "complete_resume_inputs", "comparison_count"]}))
    raise SystemExit(0 if report["status"] == "pass" else 1)
