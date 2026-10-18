#!/usr/bin/env python3
"""Reviewer check of the author's payroll P1/P5/P4/P10 and self-move/arith expectations
using the reviewer oracle. Usage: python3 -I crosscheck_author.py CANDIDATE_DIR"""
import resource, sys, os, json
resource.setrlimit(resource.RLIMIT_CPU, (60, 60))
resource.setrlimit(resource.RLIMIT_AS, (1 << 30, 1 << 30))
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import core_oracle as C
from crosscheck_helpers import val, aid, norm_effect, norm_snap, scen_of

cand = sys.argv[1]
E = os.path.join(cand, "EXPECTED-PACKET")
A = json.load(open(os.path.join(E, "AUTHOR-EXPECTATIONS.json")))
pay = C.parse(open(os.path.join(E, "fresh", "payroll-ledger.mori")).read())
out = {}
p1 = next(v for v in A["vectors"] if v["id"] == "P1")
sc = scen_of(p1["scenario"])
r = C.simulate(pay, "pay", sc, 45)
out["P1"] = {"accepted": r["accepted"], "work": r.get("work_used")}
if r["accepted"]:
    out["P1"]["effects_equal"] = [norm_effect(e) for e in p1["expected"]["effects"]] == r["effects"]
    out["P1"]["post_equal"] = norm_snap(p1["expected"]["post"]) == r["post"]
r44 = C.simulate(pay, "pay", sc, 44)
out["P10_cap44"] = {"accepted": r44["accepted"], "error": r44.get("error"), "step": r44.get("step")}
sc4 = json.loads(json.dumps(p1["scenario"]))
sc4["inputs"]["bonus"] = {"type": "Bool", "value": False}
sc4["inputs"]["ceiling"] = {"type": "UInt", "value": "20"}
r4 = C.simulate(pay, "pay", scen_of(sc4), 45)
out["P4"] = {"accepted": r4["accepted"], "error": r4.get("error"), "step": r4.get("step")}
sc5 = json.loads(json.dumps(p1["scenario"]))
sc5["inputs"]["ceiling"] = {"type": "UInt", "value": "0"}
r5 = C.simulate(pay, "pay", scen_of(sc5), 45)
out["P5"] = {"accepted": r5["accepted"], "same_post_as_P1": r5.get("post") == r.get("post"), "same_effects": r5.get("effects") == r.get("effects")}
# settle from P1 post
post = r["post"]
for amount, closing in ((7, 0), (3, 4), (8, None)):
    s = {"inputs": {"amount": ("UInt", amount)}, "state": dict(post["state"]),
         "balances": [dict(b) for b in post["balances"]], "liabilities": [dict(l) for l in post["liabilities"]]}
    rs = C.simulate(pay, "settle", s, None)
    out[f"settle_{amount}"] = {"accepted": rs["accepted"], "work": rs.get("work_used"), "error": rs.get("error"),
                               "closing": next((l["amount"] for l in rs["post"]["liabilities"] if l["name"] == "benefit"), None) if rs["accepted"] else None}
out["pay_schedule_claim_W45_and_shape"] = "see test; reviewer W=" + str(r.get("work_used"))
print(json.dumps(out, indent=1))
