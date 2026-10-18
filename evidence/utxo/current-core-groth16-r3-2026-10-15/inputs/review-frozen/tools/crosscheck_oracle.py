#!/usr/bin/env python3
"""Reviewer differential: reviewer oracle (tools/core_oracle.py) versus the frozen
non-author /1 oracle fixtures that the candidate's tests consume. Reads fixtures as
data only. Usage: python3 -I crosscheck_oracle.py CANDIDATE_DIR"""
import resource, sys, os, json
resource.setrlimit(resource.RLIMIT_CPU, (60, 60))
resource.setrlimit(resource.RLIMIT_AS, (1 << 30, 1 << 30))
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import core_oracle as C

cand = sys.argv[1]
F = os.path.join(cand, "fixtures", "reused")


def val(w):
    return ("UInt", int(w["value"])) if w["type"] == "UInt" else ("Bool", w["value"])


def aid(a):
    return (a["network"], a["issuer"], a["reference"], a["scale"])


def wire_program(p):
    def wv(v):
        return {"type": v[0], "value": str(v[1]) if v[0] == "UInt" else v[1]}
    exprs = []
    for e in p["expressions"]:
        e = dict(e)
        if e["kind"] == "Literal":
            e["value"] = wv(e["value"])
        exprs.append(e)
    return {"language": p["language"], "semantics": p["semantics"], "name": p["name"],
            "states": [{"name": s["name"], "ty": s["ty"], "initial": wv(s["initial"])} for s in p["states"]],
            "assets": p["assets"], "expressions": exprs, "actions": p["actions"]}


def norm_effect(e):
    e = dict(e)
    if "asset" in e and isinstance(e["asset"], dict):
        e["asset"] = aid(e["asset"])
    for k in ("amount", "before", "after"):
        if k in e and isinstance(e[k], str):
            e[k] = int(e[k])
        elif k in e and isinstance(e[k], dict):
            e[k] = val(e[k])
    if "value" in e and isinstance(e["value"], dict):
        e["value"] = val(e["value"])
    return e


def norm_snap(s):
    return {"state": {k: val(v) for k, v in s["state"].items()},
            "balances": [{"account": b["account"], "asset": aid(b["asset"]), "amount": int(b["amount"])} for b in s["balances"]],
            "liabilities": [{"name": l["name"], "asset": aid(l["asset"]), "debtor": l["debtor"], "creditor": l["creditor"],
                             "amount": int(l["amount"])} for l in s["liabilities"]]}


report = {"programs": {}, "cases": []}
for src, prog_json in (("installment-invoice.mori", "installment-invoice.expected-program.json"),
                       ("conditional-reserve.mori", "conditional-reserve.expected-program.json")):
    prog = C.parse(open(os.path.join(F, src)).read())
    expected = json.load(open(os.path.join(F, prog_json)))
    report["programs"][src] = {"typed_program_equal": wire_program(prog) == expected}
    report["programs"][src + "#prog"] = prog
for case, src in (("invoice-open", "installment-invoice.mori"), ("invoice-partial", "installment-invoice.mori"),
                  ("invoice-finish", "installment-invoice.mori"), ("reserve-allocate", "conditional-reserve.mori"),
                  ("reserve-rebate4", "conditional-reserve.mori"), ("reserve-rebate2", "conditional-reserve.mori")):
    v = json.load(open(os.path.join(F, case + ".json")))
    prog = report["programs"][src + "#prog"]
    sc = v["scenario"]
    scen = {"inputs": {k: val(x) for k, x in sc["inputs"].items()},
            "state": {k: val(x) for k, x in sc["state"].items()},
            "balances": [{"account": b["account"], "asset": aid(b["asset"]), "amount": int(b["amount"])} for b in sc["balances"]],
            "liabilities": [{"name": l["name"], "asset": aid(l["asset"]), "debtor": l["debtor"], "creditor": l["creditor"],
                             "amount": int(l["amount"])} for l in sc["liabilities"]]}
    cap = int(v["expected"]["cost"]["cap"])
    r = C.simulate(prog, v["action"], scen, cap)
    ex = v["expected"]["execution"]
    row = {"case": case, "action": v["action"], "reviewer_accepted": r["accepted"]}
    if r["accepted"]:
        row["effects_equal"] = [norm_effect(e) for e in ex["effects"]] == r["effects"]
        row["post_equal"] = norm_snap(ex["post"]) == r["post"]
        row["pre_equal"] = norm_snap(ex["pre"]) == r["pre"]
        row["work_equal"] = int(v["expected"]["cost"]["used"]) == r["work_used"]
        row["work"] = r["work_used"]
        row["schedule_total"] = sum(int(s["total"]) for s in v["logical_schedule"])
    report["cases"].append(row)
for k in list(report["programs"]):
    if k.endswith("#prog"):
        del report["programs"][k]
print(json.dumps(report, indent=1))
