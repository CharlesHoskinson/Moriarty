"""Export replayed native-local cost receipts to the pinned assessment format."""
from contextlib import contextmanager
import fcntl
import hashlib
import json
import os
import subprocess
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
REPORT = Path("deliverables/competitor-report/III.1")
SCRIPT = Path("scripts/qa/measure-order-lifecycle.mjs")
PROFILE = "specifications/execution_cost/profiles-1.json"
LOCAL_OPERATIONS = ("o1-fill", "cancel-unfilled")


def ref(path):
    return {"path": path.relative_to(ROOT).as_posix(), "sha256": hashlib.sha256(path.read_bytes()).hexdigest()}


def save(path, value):
    path.parent.mkdir(parents=True, exist_ok=True)
    data = json.dumps(value, indent=2, allow_nan=False) + "\n"
    with tempfile.NamedTemporaryFile(mode="w", dir=path.parent, delete=False) as temporary:
        try:
            temporary.write(data)
            temporary.close()
            os.replace(temporary.name, path)
        finally:
            Path(temporary.name).unlink(missing_ok=True)
    return ref(path)


def read(path):
    return json.loads(path.read_bytes())


def export_sample(base, measured, semantic, sample_id):
    rows = {r["label"]: r for r in read(base / "receipts.json")}
    directory = base / "assessment"
    results = [{"id": key, "passed": value} for key, value in semantic["results"].items()]
    check = {"sample_id": sample_id, "execution_id": semantic["executionId"], "results": results,
             "command": semantic["command"], "observations": {key: semantic[key] for key in
             ("initialBalances", "finalBalances", "deployment", "protocolFee", "generator")}}
    sample = {"id": sample_id, "execution_id": semantic["executionId"], "transactions": [],
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
    return sample


def bootstrap_fees(base):
    """The excluded bootstrap registration pays its fee from newly generated DUST, without a spend event."""
    rows = []
    for row in read(base / "receipts.json"):
        if row["label"].startswith("setup-"):
            rows.append({"label": row["label"], "requiredFeeSpeck": row["requiredFeeSpeck"],
                         "dustBefore": row.get("dustBefore"), "dustAfter": row.get("dustAfter"),
                         "spendEvents": len(row.get("dustSpendEvents") or [])})
    return rows


def sample_directories(base):
    found = sorted((d for d in base.iterdir() if d.is_dir() and d.name.startswith("sample-")),
                   key=lambda d: int(d.name.split("-", 1)[1]))
    if len(found) < 2 or [d.name for d in found] != [f"sample-{i}" for i in range(1, len(found) + 1)]:
        raise ValueError(f"{base.name} needs at least two consecutively numbered serial samples")
    return found


@contextmanager
def locked_report(directory):
    """Serialize sample export, packet publication and pinned workflow validation."""
    directory.mkdir(parents=True, exist_ok=True)
    fd = os.open(directory, os.O_RDONLY)
    try:
        fcntl.flock(fd, fcntl.LOCK_EX)
        yield
    finally:
        os.close(fd)


def check_costs(preflight=None):
    directory = ROOT / REPORT
    packet = directory / "assessment-input.json"
    workflow = directory / "workflow-check.json"
    result = directory / "result.json"
    with locked_report(directory):
        pending = {"requirement":"III.1", "predicate":"F31", "passed":False,
                   "assessment":"Cost replay in progress; operation results withheld."}
        try:
            save(result, pending)
            packet.unlink(missing_ok=True)
            # A byte-identical successful replay can reuse its pinned validation.
            try:
                prior_validation = read(workflow)
            except (FileNotFoundError, ValueError):
                prior_validation = None
            workflow.unlink(missing_ok=True)
            metadata = preflight() if preflight else {}
            settings = read(directory / "recipe.json")
            profile_bytes = (directory / "workloads.json").read_bytes()
            pin = read(directory / "workflow-pins.json")["sha256"][PROFILE]
            if hashlib.sha256(profile_bytes).hexdigest() != pin or settings["profile"]["sha256"] != pin:
                raise ValueError("cost replay requires the pinned workload profile")
            profile = json.loads(profile_bytes)
            unavailable = {op["id"] for op in profile["operations"]} - set(LOCAL_OPERATIONS)
            if (set(settings["operations"]) != set(LOCAL_OPERATIONS)
                    or set(settings["unavailable"]) != unavailable
                    or not all(isinstance(reason, str) and reason.strip() for reason in settings["unavailable"].values())):
                raise ValueError("measure o1-fill and cancel-unfilled and give reasons for unavailable workloads")
            values = measure(directory, settings, profile, metadata)
            if isinstance(prior_validation, dict) and prior_validation.get("inputSha256") == ref(packet)["sha256"]:
                save(workflow, prior_validation)
            return values
        except BaseException as error:
            packet.unlink(missing_ok=True)
            workflow.unlink(missing_ok=True)
            save(result, {**pending, "error":str(error)[-2400:] or type(error).__name__,
                          "assessment":"Cost replay failed; operation results withheld.",
                          "retainedSamples":sorted(str(d.relative_to(ROOT)) for d in
                              (directory / "native").glob("*/sample-*") if d.is_dir())})
            raise


def measure(directory, settings, profile, metadata):
    recipes, values, totals, bootstrap = [], {}, {}, {}
    validator = os.environ.get("MORIARTY_ORDER_VALIDATOR", str(ROOT / "target/debug/examples/validate_call"))
    for definition in profile["operations"]:
        op = definition["id"]
        environment = {"chain":"Midnight", "network":"undeployed", "hardware":"No integration selected",
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
        samples, rows = [], []
        for sample_dir in sample_directories(base):
            checks = read(sample_dir / "checks.json")
            proc = subprocess.run(["node", str(SCRIPT), str(sample_dir), op, validator], cwd=ROOT,
                                  capture_output=True, text=True, timeout=180)
            if proc.returncode:
                raise ValueError(f"{op} {sample_dir.name} strict cost replay failed: {proc.stderr[-1800:]}")
            measured = json.loads(proc.stdout)
            samples.append(export_sample(sample_dir, measured, checks, sample_dir.name))
            rows.append({"sample":sample_dir.name, "phases":measured["phases"], "total":measured["total"],
                         "executionId":measured["executionId"], "revision":measured["revision"], "host":measured["host"]})
            bootstrap[f"{op}/{sample_dir.name}"] = bootstrap_fees(sample_dir)
        # The environment is the measuring host and code, identical across this workload's samples.
        hosts = {json.dumps(r["host"], sort_keys=True) for r in rows}
        revisions = {r["revision"] for r in rows}
        if len(hosts) != 1 or len(revisions) != 1:
            raise ValueError(f"{op} samples differ in measuring host or revision")
        host = rows[0]["host"]
        environment["hardware"] = f"{host['architecture']} ({host['platform']})"
        environment["revision"] = rows[0]["revision"]
        totals[op] = {"samples": rows}
        recipe["artifact"] = ref(ROOT / "scripts/qa/order_cost_report.py")
        recipe["command"] = ["python3","scripts/qa/order_cost_report.py",op]
        recipe["steps"] = [{"id":step,"phase":phase,"chain":"X"} for phase in ("setup","operation","recovery")
                           for step in settings["operations"][op][phase]]
        ctx["chains"]["X"] = {"family":"midnight","network":"undeployed","vm":"Midnight ledger v9, ZKIRv3",
            "client":"midnight-ledger 9.1.0 and SDK WASM ledger-v9 1.0.0",
            "protocol_parameters":ref(base / "sample-1" / "parameters.bin"),"native_fee_asset":"DUST","native_fee_decimals":15,
            "finality_policy":{"mode":"local-acceptance","descendant_blocks":0,"rule":settings["finality"]["rule"]},
            "fee_policy":{"mode":"observed","source":settings["feePolicy"]["rule"],"scope":"all mandatory chain fees",
                          "required_other_fee_components":[]}}
        values[op] = {"recipe_id":recipe["id"], "unit":recipe["unit"], "environment":environment,
                      "boundary":recipe["boundary"], "outcome":"completed", "samples":samples}
    executions = [row["executionId"] for op in totals for row in totals[op]["samples"]]
    if len(set(executions)) != len(executions):
        raise ValueError("Each sample requires its own independent execution")
    zero_phases = {op: {phase: settings["operations"][op]["emptyRecovery"]}
                   for op in totals for phase in ("recovery",) if not settings["operations"][op][phase]}
    inputs = {"rubric_version":"execution_cost-2", "measurement_profile":{"version":profile["version"],
              "path":PROFILE, "sha256":ref(directory / "workloads.json")["sha256"]}, "recipes":recipes,"values":values}
    packet = save(directory / "assessment-input.json",inputs)
    save(directory / "result.json",{"requirement":"III.1","predicate":"F31","passed":True, **metadata,
         "assessment":"pending independent assessment", "assessmentInput":packet, "units":settings["resourceUnits"],
         "feePolicy":settings["feePolicy"], "finality":settings["finality"], "totals":totals,
         "zeroPhases":zero_phases, "recipe":ref(directory / "recipe.json"), "profile":ref(directory / "workloads.json"),
         "rawExecutions":{f"{op}/{row['sample']}":ref(directory/"native"/op/row["sample"]/"receipts.json")
                          for op in totals for row in totals[op]["samples"]},
         "excludedBootstrap":{"rule":settings["preexisting"], "observedFees":bootstrap},
         "exclusions":settings["exclusions"], "proverEffort":"Excluded from Midnight resources; not measured in this sample set.",
         "unavailable":settings["unavailable"],"commands":{"test":["node","--test","packages/moriarty-orders/test/measurement.test.mjs"],
         "runner":["python3","scripts/qa/order-report.py","III.1"]}})
    return values


if __name__ == "__main__":
    import sys
    selected = sys.argv[1]
    if selected not in LOCAL_OPERATIONS:
        raise SystemExit("choose o1-fill or cancel-unfilled")
    print(json.dumps(check_costs()[selected], allow_nan=False))
