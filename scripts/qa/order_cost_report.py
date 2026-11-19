"""Export replayed native-local cost receipts to the pinned assessment format."""
import hashlib
import json
import os
import platform
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
REPORT = Path("deliverables/competitor-report/III.1")
SCRIPT = Path("scripts/qa/measure-order-lifecycle.mjs")
PROFILE = "specifications/execution_cost/profiles-1.json"


def ref(path):
    return {"path": path.relative_to(ROOT).as_posix(), "sha256": hashlib.sha256(path.read_bytes()).hexdigest()}


def save(path, value):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, indent=2, allow_nan=False) + "\n")
    return ref(path)


def read(path):
    return json.loads(path.read_bytes())


def export_sample(base, spec, measured, semantic):
    rows = {r["label"]: r for r in read(base / "receipts.json")}
    directory = base / "assessment"
    results = [{"id": key, "passed": value} for key, value in semantic["results"].items()]
    check = {"sample_id": "sample-1", "execution_id": semantic["executionId"], "results": results,
             "command": semantic["command"], "observations": {key: semantic[key] for key in
             ("initialBalances", "finalBalances", "deployment", "protocolFee")}}
    sample = {"id": "sample-1", "execution_id": semantic["executionId"], "transactions": [],
              "checks": {"artifact":save(directory / "checks.json", check), "results":results}}
    for tx in measured["transactions"]:
        row = rows[tx["label"]]
        for index, event in enumerate(row["dustSpendEvents"]):
            raw = event["serialized"]
            data = bytes(raw) if isinstance(raw, list) else bytes(raw[str(i)] for i in range(len(raw)))
            path = directory / (row["label"] + f"-event-{index}.bin")
            path.write_bytes(data)
            event["serialized"] = ref(path)
        raw_receipt = save(directory / (row["label"] + "-raw.json"), row)
        normalized = {"transaction_id":row["transactionHash"], "success":True, "resources":tx["resources"],
                      "fee":tx["fee"], "source":raw_receipt}
        receipt = save(directory / (row["label"] + "-receipt.json"), normalized)
        sample["transactions"].append({"step_id":row["label"], "chain":"X", "transaction_id":row["transactionHash"],
            "success":True, "transaction":ref(base / row["transaction"]["file"]), "receipt":receipt,
            "raw_receipt":raw_receipt, "resources":tx["resources"], "fee_paid":tx["fee"], "other_fee_components":[]})
    return {"recipe_id":spec["id"], "unit":spec["unit"], "environment":spec["environment"],
            "boundary":spec["boundary"], "outcome":"completed", "samples":[sample]}


def check_costs():
    directory = ROOT / REPORT
    settings = read(directory / "recipe.json")
    profile = read(directory / "workloads.json")
    recipes, values, totals = [], {}, {}
    validator = os.environ.get("MORIARTY_ORDER_VALIDATOR", str(ROOT / "target/debug/examples/validate_call"))
    for definition in profile["operations"]:
        op = definition["id"]
        environment = {"chain":"Midnight", "network":"undeployed", "hardware":platform.machine(),
                       "revision":"No integration selected"}
        ctx = {"execution_mode":"native-local", "chains":{},
               "protocol_fees":{"amount":0,"reason":"Zero application fee in the pinned workload."},
               "exclusions":["host effort","proving effort",*settings["exclusions"]],
               "state_policy":{"fixture":"Fresh independent 100 A order","reset":settings["reset"],
                 "preexisting_shared_setup":settings["preexisting"], "order_specific_setup":"deploy, mint and fund from the enrolled wallet"}}
        recipe = {key:definition[key] for key in ("order_id","workload","boundary")}
        recipe.update({"id":op, "operation_id":op, "metric":"transaction_cost", "environment":environment,
                       "context":ctx, "steps":[], "unit":profile["unit"]})
        recipes.append(recipe)
        if op in settings["unavailable"]:
            values[op] = {"outcome":"unavailable", "reason":settings["unavailable"][op]}
            continue
        base = directory / "native" / op
        checks = read(base / "checks.json")
        environment["revision"] = checks["revision"]
        proc = subprocess.run(["node", str(SCRIPT), str(base), op, validator], cwd=ROOT,
                              capture_output=True, text=True, timeout=180)
        if proc.returncode:
            raise ValueError(f"{op} strict cost replay failed: {proc.stderr[-1800:]}")
        measured = json.loads(proc.stdout)
        totals[op] = {"phases":measured["phases"], "total":measured["total"], "executionId":measured["executionId"]}
        recipe["artifact"] = ref(ROOT / "scripts/qa/order_cost_report.py")
        recipe["command"] = ["python3","scripts/qa/order_cost_report.py",op]
        recipe["steps"] = [{"id":step,"phase":phase,"chain":"X"} for phase in ("setup","operation","recovery")
                           for step in settings["operations"][op][phase]]
        ctx["chains"]["X"] = {"family":"midnight","network":"undeployed","vm":"Midnight ledger v9, ZKIRv3",
            "client":"midnight-ledger 9.1.0 and SDK WASM ledger-v9 1.0.0",
            "protocol_parameters":ref(base / "parameters.bin"),"native_fee_asset":"DUST","native_fee_decimals":15,
            "finality_policy":{"mode":"local-acceptance","descendant_blocks":0,"rule":settings["finality"]["rule"]},
            "fee_policy":{"mode":"observed","source":settings["feePolicy"]["rule"],"scope":"all mandatory chain fees",
                          "required_other_fee_components":[]}}
        values[op] = export_sample(base,recipe,measured,checks)
    if len({row["executionId"] for row in totals.values()}) != 2:
        raise ValueError("Each workload requires its own independent execution")
    inputs = {"rubric_version":"execution_cost-2", "measurement_profile":{"version":profile["version"],
              "path":PROFILE, "sha256":ref(directory / "workloads.json")["sha256"]}, "recipes":recipes,"values":values}
    packet = save(directory / "assessment-input.json",inputs)
    save(directory / "result.json",{"requirement":"III.1","predicate":"F31","passed":True,
         "assessment":"pending independent assessment", "assessmentInput":packet, "units":settings["resourceUnits"],
         "feePolicy":settings["feePolicy"], "finality":settings["finality"], "totals":totals,
         "recipe":ref(directory / "recipe.json"), "profile":ref(directory / "workloads.json"),
         "rawExecutions":{op:ref(directory/"native"/op/"receipts.json") for op in totals},
         "excludedBootstrap":settings["preexisting"], "exclusions":settings["exclusions"],
         "unavailable":settings["unavailable"],"commands":{"test":["node","--test","packages/moriarty-orders/test/measurement.test.mjs"],
         "runner":["python3","scripts/qa/order-report.py","III.1"]}})
    return values


if __name__ == "__main__":
    import sys
    selected = sys.argv[1]
    if selected not in ("o1-fill","cancel-unfilled"):
        raise SystemExit("choose o1-fill or cancel-unfilled")
    print(json.dumps(check_costs()[selected], allow_nan=False))
