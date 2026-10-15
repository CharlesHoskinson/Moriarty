"""Reviewer-owned helpers for post-freeze tools. Loads the FROZEN held-out programs
from heldout/programs (never regenerates them) and provides wire encoders."""
import os, json
import core_oracle as C
from crosscheck_helpers import val

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
P = os.path.join(ROOT, "heldout", "programs")


def _ir_from_wire(w):
    def v(x):
        return val(x)
    exprs = []
    for e in w["expressions"]:
        e = dict(e)
        if e["kind"] == "Literal":
            e["value"] = v(e["value"])
        exprs.append(e)
    return {"language": w["language"], "semantics": w["semantics"], "name": w["name"],
            "states": [{"name": s["name"], "ty": s["ty"], "initial": v(s["initial"])} for s in w["states"]],
            "assets": w["assets"], "expressions": exprs, "actions": w["actions"]}


def programs():
    out = {}
    for k in ("H1-settle", "H2-accrual", "H5-strict", "H6-bounds", "H4-emit"):
        out[k] = C.parse(open(os.path.join(P, k + ".mori")).read())
    out["H3-dag"] = _ir_from_wire(json.load(open(os.path.join(P, "H3-dag.program.json"))))
    C.validate(out["H3-dag"])
    return out


def wire_value(v):
    return {"type": v[0], "value": (str(v[1]) if v[0] == "UInt" else v[1])}


def wire_asset(t):
    return None if t is None else {"network": t[0], "issuer": t[1], "reference": t[2], "scale": t[3]}


def wire_snapshot(s):
    return {"state": {k: wire_value(v) for k, v in sorted(s["state"].items())},
            "balances": [{"account": b["account"], "asset": wire_asset(b["asset"]), "amount": str(b["amount"])} for b in s["balances"]],
            "liabilities": [{"name": l["name"], "asset": wire_asset(l["asset"]), "debtor": l["debtor"],
                             "creditor": l["creditor"], "amount": str(l["amount"])} for l in s["liabilities"]]}


def wire_effect(e):
    e = dict(e)
    if "asset" in e:
        e["asset"] = wire_asset(e["asset"])
    for k in ("amount", "before", "after"):
        if k in e and isinstance(e[k], int) and not isinstance(e[k], bool):
            e[k] = str(e[k])
    for k in ("before", "after", "value"):
        if k in e and isinstance(e[k], tuple):
            e[k] = wire_value(e[k])
    return e
