#!/usr/bin/env python3
"""POST-FREEZE ADDENDUM 2 (reviewer-authored after reading the candidate): an isolated
absent-duty discharge control in which the in-circuit presence assertion is the only
check that can refuse. Expectations from tools/core_oracle.py only.
Usage: python3 -I make_addendum2.py OUT_DIR"""
import resource, sys, os, json
resource.setrlimit(resource.RLIMIT_CPU, (60, 60))
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import core_oracle as C
import make_vectors_lib as V

SRC = '''language "moriarty/1";
agreement HoldoutDischarge {
  asset T = { network: "holdout-net", issuer: "holdout-dis", reference: "t", scale: 0 };
  action open(x: UInt) {
    liability("d", T, "p", "q", x);
  }
  action pay(x: UInt) {
    discharge("d", x);
  }
}
'''
T = ("holdout-net", "holdout-dis", "t", 0)
out = sys.argv[1]
os.makedirs(os.path.join(out, "programs"), exist_ok=True)
with open(os.path.join(out, "programs", "H8-discharge.mori"), "w") as f:
    f.write(SRC)
p = C.parse(SRC)
rows = []
for vid, x, liabs, note in [
    ("H8-R1", 0, [], "absent duty, discharge 0: Core refuses (Unknown residual liability); only the presence assertion refuses in-circuit"),
    ("H8-P1", 0, [{"name": "d", "asset": T, "debtor": "p", "creditor": "q", "amount": 0}], "present-zero duty, discharge 0: accepted"),
    ("H8-P2", 3, [{"name": "d", "asset": T, "debtor": "p", "creditor": "q", "amount": 3}], "full discharge keeps a present-zero row"),
]:
    sc = {"inputs": {"x": ("UInt", x)}, "state": {}, "balances": [], "liabilities": liabs}
    r = C.simulate(p, "pay", sc)
    row = {"id": vid, "program": "H8-discharge", "action": "pay", "note": note,
           "scenario": {"inputs": {"x": V.wire_value(("UInt", x))}, "state": {}, "balances": [],
                        "liabilities": [{"name": l["name"], "asset": V.wire_asset(l["asset"]), "debtor": l["debtor"],
                                         "creditor": l["creditor"], "amount": str(l["amount"])} for l in liabs]}}
    if r["accepted"]:
        row["expected"] = {"accepted": True, "work_used": r["work_used"], "effects": [V.wire_effect(e) for e in r["effects"]],
                           "pre": V.wire_snapshot(r["pre"]), "post": V.wire_snapshot(r["post"])}
    else:
        row["expected"] = {"accepted": False, "error": r["error"], "step": r["step"], "phase": r["phase"], "work_attempted": r["work_used"]}
        row["hostile_claim"] = {"post": {"state": {}, "balances": [], "liabilities": []},
                                "effects": [{"kind": "Liability", "name": "d", "asset": V.wire_asset(T), "debtor": "p", "creditor": "q",
                                             "before": "0", "after": "0"}],
                                "required": "UNSATISFIABLE: presence-blind lowering would accept (0 - 0 = 0 on the canonical absent cell)"}
    rows.append(row)
doc = {"format": "reviewer-heldout-addendum/2", "status": "POST-FREEZE, reviewer-authored after reading the candidate; expectations from the reviewer oracle only; not executed against Core or the candidate.",
       "vectors": rows}
with open(os.path.join(out, "HELDOUT-ADDENDUM-2.json"), "w") as f:
    json.dump(doc, f, indent=2, sort_keys=True)
    f.write("\n")
for r in rows:
    print(r["id"], r["expected"])
