#!/usr/bin/env python3
"""POST-FREEZE ADDENDUM generator (reviewer-authored, after reading the candidate).
For every rejected execution vector of heldout/HELDOUT-VECTORS.json, runs a deliberately
PERMISSIVE evaluator (u128 wrap-around, short-circuit and/or, ignored requires/ensures,
no debit/discharge/presence checks, ignored cap) and records the full claim such a
buggy lowering would accept. The compiled relation must refuse every one of them.
Also emits targeted mutations of accepted vectors. Derived only from the frozen inputs
and the documented permissive rules; no candidate code is imported or executed.
Usage: python3 -I make_hostile_claims.py OUT_JSON"""
import resource, sys, os, json, copy
resource.setrlimit(resource.RLIMIT_CPU, (60, 60))
resource.setrlimit(resource.RLIMIT_AS, (1 << 30, 1 << 30))
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import core_oracle as C
from crosscheck_helpers import val, aid
import make_vectors_lib as V

M = 1 << 128
ROOT = os.path.dirname(HERE)
VEC = json.load(open(os.path.join(ROOT, "heldout", "HELDOUT-VECTORS.json")))


def pev(prog, eid, env, snap, cache):
    if eid in cache:
        return cache[eid]
    e = prog["expressions"][eid]
    k = e["kind"]
    if k in ("LiabilityPresent", "LiabilityAmount"):
        row = next((l for l in snap["liabilities"] if l["name"] == e["name"]), None)
        r = ("Bool", row is not None) if k == "LiabilityPresent" else ("UInt", row["amount"] if row else 0)
    elif k == "Literal":
        r = tuple(e["value"])
    elif k == "Name":
        r = env[e["name"]]
    elif k == "Not":
        r = ("Bool", not pev(prog, e["value"], env, snap, cache)[1])
    else:
        op = e["op"]
        a = pev(prog, e["lhs"], env, snap, cache)
        if op == "And" and a[1] is False:
            r = ("Bool", False)          # short circuit: rhs never evaluated
        elif op == "Or" and a[1] is True:
            r = ("Bool", True)
        else:
            b = pev(prog, e["rhs"], env, snap, cache)
            r = {"Add": lambda: ("UInt", (a[1] + b[1]) % M), "Sub": lambda: ("UInt", (a[1] - b[1]) % M),
                 "Mul": lambda: ("UInt", (a[1] * b[1]) % M), "Eq": lambda: ("Bool", a == b),
                 "Ne": lambda: ("Bool", a != b), "Lt": lambda: ("Bool", a[1] < b[1]),
                 "Le": lambda: ("Bool", a[1] <= b[1]), "Gt": lambda: ("Bool", a[1] > b[1]),
                 "Ge": lambda: ("Bool", a[1] >= b[1]), "And": lambda: ("Bool", a[1] and b[1]),
                 "Or": lambda: ("Bool", a[1] or b[1])}[op]()
    cache[eid] = r
    return r


def permissive(prog, action, sc):
    act = next(a for a in prog["actions"] if a["name"] == action)
    pre = C.snapshot(prog, act, sc)          # admission rules still apply (scenario must be well-formed)
    cur = copy.deepcopy(pre)
    env = dict(cur["state"]); env.update(sc["inputs"])
    effects = []
    for s in act["statements"]:
        k = s["kind"]
        ev = lambda eid: pev(prog, eid, env, cur, {})
        if k == "Let":
            env[s["name"]] = ev(s["value"])
        elif k in ("Requires", "Ensures"):
            ev(s["predicate"])               # value ignored
        elif k == "Next":
            after = ev(s["value"]); before = cur["state"][s["state"]]
            cur["state"][s["state"]] = after; env[s["state"]] = after
            effects.append({"kind": "State", "name": s["state"], "before": before, "after": after})
        elif k == "Emit":
            effects.append({"kind": "Emit", "name": s["name"], "value": ev(s["value"])})
        elif k in ("Transfer", "Fee"):
            amt = ev(s["amount"])[1]; a = C.asset_id(prog, s["asset"])
            f = next(b for b in cur["balances"] if b["asset"] == a and b["account"] == s["from"])
            t = next(b for b in cur["balances"] if b["asset"] == a and b["account"] == s["to"])
            if f is not t:
                f["amount"] = (f["amount"] - amt) % M
                t["amount"] = (t["amount"] + amt) % M
            effects.append({"kind": k, "asset": a, "from": s["from"], "to": s["to"], "amount": amt})
        elif k == "Liability":
            amt = ev(s["amount"])[1]; a = C.asset_id(prog, s["asset"])
            row = next((l for l in cur["liabilities"] if l["name"] == s["name"]), None)
            before = row["amount"] if row else 0
            after = (before + amt) % M
            if row:
                row["amount"] = after
            else:
                cur["liabilities"].append({"name": s["name"], "asset": a, "debtor": s["debtor"], "creditor": s["creditor"], "amount": after})
                cur["liabilities"].sort(key=lambda l: l["name"])
            effects.append({"kind": "Liability", "name": s["name"], "asset": a, "debtor": s["debtor"], "creditor": s["creditor"], "before": before, "after": after})
        elif k == "Discharge":
            amt = ev(s["amount"])[1]
            row = next((l for l in cur["liabilities"] if l["name"] == s["name"]), None)
            if row is None:
                # a presence-blind lowering treats an absent duty as (0) and keeps it absent
                effects.append({"kind": "Liability", "name": s["name"], "asset": None, "debtor": None, "creditor": None,
                                "before": 0, "after": (0 - amt) % M, "absent_row": True})
                continue
            before = row["amount"]; row["amount"] = (before - amt) % M
            effects.append({"kind": "Liability", "name": s["name"], "asset": row["asset"], "debtor": row["debtor"],
                            "creditor": row["creditor"], "before": before, "after": row["amount"]})
    return pre, cur, effects


def scen_from_vector(v):
    s = v["scenario"]
    return {"inputs": {k: val(x) for k, x in s["inputs"].items()}, "state": {k: val(x) for k, x in s["state"].items()},
            "balances": [{"account": b["account"], "asset": aid(b["asset"]), "amount": int(b["amount"])} for b in s["balances"]],
            "liabilities": [{"name": l["name"], "asset": aid(l["asset"]), "debtor": l["debtor"], "creditor": l["creditor"],
                             "amount": int(l["amount"])} for l in s["liabilities"]]}


progs = V.programs()
claims = []
for v in VEC["vectors"]:
    ex = v["expected"]
    if ex["accepted"]:
        continue
    prog = progs[v["program"]]
    sc = scen_from_vector(v)
    entry = {"base_vector": v["id"], "core_expected": {"error": ex["error"], "step": ex["step"], "phase": ex["phase"]},
             "logical_cap": v["logical_cap"]}
    if ex["phase"] == "admission":
        entry["claim"] = None
        entry["required"] = "not encodable through witness::from_records (MappingError) — but see note"
        entry["note"] = {"H1-R6": "identity of a registry duty is a relation constant; the claim cannot name another debtor. Target must authenticate the ledger row identity (premise).",
                         "H1-R7": "Core refuses a MISSING operator balance row; the relation has no row-presence bit for balances, so a prover can claim operator pre = 0 and the relation is SATISFIABLE. Row existence is an unauthenticated premise (finding F-04)."}.get(v["id"], "")
        claims.append(entry)
        continue
    pre, post, effects = permissive(prog, v["action"], sc)
    entry["claim"] = {"inputs": v["scenario"]["inputs"], "pre": V.wire_snapshot(pre), "post": V.wire_snapshot(post),
                      "effects": [V.wire_effect(e) for e in effects],
                      "cap": v["logical_cap"]}
    entry["permissive_rules"] = "u128 wrap for Add/Sub/Mul/credit/debit/accrual/discharge; short-circuit and/or; requires/ensures ignored; no debit/presence/discharge-bound checks; cap ignored"
    entry["required"] = "UNSATISFIABLE under the compiled relation (or not encodable)"
    claims.append(entry)

mutations = [
    {"id": "M-01", "base_vector": "H1-P1", "edit": "post.liabilities: delete the present-zero 'warranty' row (encodes as absent (false,0))", "required": "UNSATISFIABLE"},
    {"id": "M-02", "base_vector": "H1-P3", "edit": "post.liabilities: delete 'warranty' (amount 0) row", "required": "UNSATISFIABLE"},
    {"id": "M-03", "base_vector": "H1-P1", "edit": "swap the amounts of effect 0 (Transfer 248750) and effect 1 (Fee 1250), post unchanged", "required": "UNSATISFIABLE"},
    {"id": "M-04", "base_vector": "H1-P1", "edit": "effect 1 Fee recipient operator -> seller", "required": "not encodable (recipient is a relation constant); MappingError::Shape"},
    {"id": "M-05", "base_vector": "H1-P1", "edit": "post auditor USDX 7 -> 8 under FrameCells::Include([USDX/auditor])", "required": "UNSATISFIABLE"},
    {"id": "M-06", "base_vector": "H1-P1", "edit": "post auditor USDX 7 -> 8 under FrameCells::TouchedOnly", "required": "MappingError::FrameChanged from the encoder; a hostile prover bypassing the encoder faces no slot: frame completeness is a target obligation"},
    {"id": "M-07", "base_vector": "H1-P1", "edit": "post 'legacy' (unregistered incoming duty) 9 -> 0", "required": "no FrameCells option can represent an unregistered duty; finding F-05 (target obligation, needs a duty-frame option)"},
    {"id": "M-08", "base_vector": "H2-P3", "edit": "post.liabilities: delete fully repaid 'loan' row", "required": "UNSATISFIABLE"},
    {"id": "M-09", "base_vector": "H3-P1", "edit": "post s = 5 and effects (4->5, 5->5): one shared wire for node 2", "required": "UNSATISFIABLE"},
    {"id": "M-10", "base_vector": "H3-P4", "edit": "public slot input.v = 2 (non-Bool)", "required": "UNSATISFIABLE (booleanity)"},
    {"id": "M-11", "base_vector": "H6-A1", "edit": "public input.x limbs (lo + 2^64, hi - 1) and (lo + r, hi)", "required": "UNSATISFIABLE (bit-sum binding; no modular alias)"},
    {"id": "M-12", "base_vector": "W-1", "edit": "claim cap 35 with otherwise honest values", "required": "UNSATISFIABLE"},
    {"id": "M-13", "base_vector": "H1-P1", "edit": "header program_hash / shape / action_index field +1", "required": "UNSATISFIABLE"},
    {"id": "M-14", "base_vector": "H1-R7", "edit": "claim operator USDX pre = 0 (row actually missing in the Core scenario)", "required": "SATISFIABLE today: documents finding F-04; must become a target-bound premise or a presence bit"},
]
doc = {"format": "reviewer-hostile-claims-addendum/1", "status": "POST-FREEZE ADDENDUM, reviewer-authored after reading the candidate source; derived mechanically from the frozen HELDOUT-VECTORS.json inputs by a documented permissive evaluator. Not executed against Core or the candidate.",
       "heldout_freeze_aggregate_sha256": json.load(open(os.path.join(ROOT, "HELDOUT-FREEZE.json")))["aggregate_sha256"],
       "rejected_vector_claims": claims, "accepted_vector_mutations": mutations}
with open(sys.argv[1], "w") as f:
    json.dump(doc, f, indent=2, sort_keys=True)
    f.write("\n")
print(len(claims), "claims;", len(mutations), "mutations")
for c in claims:
    if c.get("claim"):
        print(c["base_vector"], "post states:", c["claim"]["post"]["state"])
