"""Reviewer-owned JSON <-> reviewer-oracle value helpers (no candidate code)."""


def val(w):
    return ("UInt", int(w["value"])) if w["type"] == "UInt" else ("Bool", w["value"])


def aid(a):
    return (a["network"], a["issuer"], a["reference"], a["scale"])


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


def scen_of(sc):
    return {"inputs": {k: val(x) for k, x in sc["inputs"].items()},
            "state": {k: val(x) for k, x in sc.get("state", {}).items()},
            "balances": [{"account": b["account"], "asset": aid(b["asset"]), "amount": int(b["amount"])} for b in sc["balances"]],
            "liabilities": [{"name": l["name"], "asset": aid(l["asset"]), "debtor": l["debtor"], "creditor": l["creditor"],
                             "amount": int(l["amount"])} for l in sc["liabilities"]]}
