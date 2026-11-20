#!/usr/bin/env python3
"""Dry-run the pinned upstream assessment workflow on the retained public handoff."""
import argparse
import hashlib
import json
import shutil
import sys
import tempfile
from pathlib import Path
from order_cost_report import locked_report, ref, save

ROOT = Path(__file__).resolve().parents[2]
BASE = ROOT / "deliverables/competitor-report/III.1"
UPSTREAM = "1ced5fe1e0a83d665184bb496bea01bcb9940449"
PROFILE = "specifications/execution_cost/profiles-1.json"


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("assessment_root", type=Path, help="directory containing the pinned upstream onboarding package")
    args = parser.parse_args()
    with locked_report(BASE):
        (BASE/"workflow-check.json").unlink(missing_ok=True)
        validate(args.assessment_root.resolve())


def validate(upstream):
    pins = (BASE/"workflow-pins.json").read_bytes()
    expected = json.loads(pins)
    for name, sha in expected["sha256"].items():
        if hashlib.sha256((upstream/name).read_bytes()).hexdigest() != sha:
            raise ValueError("incorrect assessment workflow file " + name)
    # The local workload copy must be byte-identical to the pinned upstream profile.
    local = hashlib.sha256((BASE/"workloads.json").read_bytes()).hexdigest()
    if local != expected["sha256"][PROFILE]:
        raise ValueError("workloads.json differs from the pinned upstream profile")
    sys.path.insert(0, str(upstream))
    from onboarding.execution_cost import validate_selection, validate_value, summarize
    result = json.loads((BASE/"result.json").read_bytes())
    packet = BASE/"assessment-input.json"
    if result.get("passed") is not True or result.get("assessmentInput") != ref(packet):
        raise ValueError("cost replay requires a passing result linked to this assessment input")
    payload = json.loads(packet.read_bytes())
    with tempfile.TemporaryDirectory(prefix="order-assessment-") as tmp:
        tree = Path(tmp)
        shutil.copytree(BASE, tree/BASE.relative_to(ROOT))
        for recipe in payload["recipes"]:
            if "artifact" in recipe:
                path = Path(recipe["artifact"]["path"])
                (tree/path).parent.mkdir(parents=True, exist_ok=True)
                shutil.copyfile(ROOT/path, tree/path)
        profile = tree/payload["measurement_profile"]["path"]
        profile.parent.mkdir(parents=True)
        shutil.copyfile(upstream/PROFILE, profile)
        recipes = validate_selection(tree, payload)
        checks = {}
        for op in ("o1-fill", "cancel-unfilled"):
            value = validate_value(tree, recipes[op],payload["values"][op])
            summarized = summarize(recipes[op],value)["chains"]["X"]["samples"]
            recorded_samples = result["totals"][op]["samples"]
            if [s["sample_id"] for s in summarized] != [r["sample"] for r in recorded_samples] or len(summarized) < 2:
                raise ValueError(f"{op} samples differ from the pinned workflow")
            for sample, recorded in zip(summarized, recorded_samples):
                for phase in ("setup","operation","recovery","total"):
                    actual = sample["total"] if phase=="total" else sample["phases"][phase]
                    ours = recorded["total"] if phase=="total" else recorded["phases"][phase]
                    for metric in ("resources","fee_paid","transaction_count"):
                        # The upstream summarizer omits resource keys for an empty phase.
                        observed = actual[metric]
                        if metric == "resources" and actual["transaction_count"] == 0 and observed == {}:
                            observed = dict.fromkeys(ours[metric], 0)
                        if observed != ours[metric]:
                            raise ValueError(f"{op} {sample['sample_id']} {phase} {metric} differs from pinned workflow")
            checks[op] = {s["sample_id"]: s["total"] for s in summarized}
        record = {"upstreamRevision":UPSTREAM,"pinsSha256":hashlib.sha256(pins).hexdigest(),
                  "inputSha256":result["assessmentInput"]["sha256"],
                  "command":["python3","scripts/qa/validate_order_cost_handoff.py","<pinned-assessment-root>"],
                  "validated":checks,"assessment":"Workflow input and arithmetic checked. Independent semantic review pending."}
        save(BASE/"workflow-check.json", record)


if __name__ == "__main__":
    main()
