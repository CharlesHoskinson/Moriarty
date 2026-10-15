#!/usr/bin/env python3
"""Pin the frozen reviewer financial/work expectations before any harness assertion is
written, and check their internal consistency with independent stdlib arithmetic over
the same frozen records (stdlib only; imports no candidate, Core, operator or reviewer
code).

Inputs (read only): HELDOUT-VECTORS.json and HELDOUT-ADDENDUM-2.json of the frozen
packet. Output: FINANCIAL-EXPECTATIONS-FREEZE.json in this directory, created
exclusively. No frozen value is changed, re-derived into the harness or replaced: the
harness reads the frozen file itself; this record only pins it and lists any internal
contradiction verbatim.

Usage: python3 -I tools/financial_freeze.py
"""
import hashlib
import json
import os
import resource
from datetime import datetime, timezone

resource.setrlimit(resource.RLIMIT_CPU, (60, 60))
resource.setrlimit(resource.RLIMIT_AS, (1 << 30, 1 << 30))

BASE = "/home/charl/moriarty-eutxo-validation-2026-10-12"
HERE = os.path.join(BASE, "current-core-groth16-evidence-repair-source-20261015")
PACKET = os.path.join(BASE, "current-core-groth16-compiler-independent-source-review-20261015")
VECTORS = os.path.join(PACKET, "heldout", "HELDOUT-VECTORS.json")
ADDENDUM2 = os.path.join(PACKET, "heldout-addendum-2", "HELDOUT-ADDENDUM-2.json")

BALANCE_FIELDS = ["account", "asset", "fees_paid", "gross_debit", "gross_debit_excluding_self", "net_delta", "post", "pre"]
DUTY_FIELDS = ["accrued", "closing", "discharged", "name", "opening", "present_after", "present_before", "present_zero_after"]
NOT_REPRESENTED = ["reserve", "dust", "DUST plane", "service fees", "recovery reservations"]


def sha(b):
    return hashlib.sha256(b).hexdigest()


def akey(asset):
    return json.dumps(asset, sort_keys=True, separators=(",", ":"))


def signed(post, pre):
    return str(post - pre) if post >= pre else "-" + str(pre - post)


def check(v):
    """Contradictions between a frozen accounting block and the frozen pre/post/effects."""
    ex = v["expected"]
    acc = ex["accounting"]
    problems = []
    pre = {(akey(b["asset"]), b["account"]): int(b["amount"]) for b in ex["pre"]["balances"]}
    post = {(akey(b["asset"]), b["account"]): int(b["amount"]) for b in ex["post"]["balances"]}
    rows = {(akey(r["asset"]), r["account"]): r for r in acc["balances"]}
    if set(rows) != set(pre) or set(pre) != set(post):
        problems.append({"field": "balances key set", "accounting": sorted(map(list, rows)), "pre": sorted(map(list, pre)), "post": sorted(map(list, post))})
    for key, r in rows.items():
        if sorted(r) != BALANCE_FIELDS:
            problems.append({"row": list(key), "field": "field set", "fields": sorted(r)})
        g = gx = fee = 0
        for e in ex["effects"]:
            if e["kind"] in ("Transfer", "Fee") and (akey(e["asset"]), e["from"]) == key:
                g += int(e["amount"])
                gx += int(e["amount"]) if e["to"] != e["from"] else 0
                fee += int(e["amount"]) if e["kind"] == "Fee" else 0
        derived = {"pre": str(pre.get(key)), "post": str(post.get(key)), "net_delta": signed(post.get(key, 0), pre.get(key, 0)),
                   "gross_debit": str(g), "gross_debit_excluding_self": str(gx), "fees_paid": str(fee)}
        for k, d in derived.items():
            if r.get(k) != d:
                problems.append({"row": list(key), "field": k, "frozen": r.get(k), "derived_from_frozen_records": d})
    lpre = {l["name"]: int(l["amount"]) for l in ex["pre"]["liabilities"]}
    lpost = {l["name"]: int(l["amount"]) for l in ex["post"]["liabilities"]}
    duties = {d["name"]: d for d in acc["duties"]}
    if set(duties) != set(lpre) | set(lpost):
        problems.append({"field": "duties name set", "accounting": sorted(duties), "pre_or_post": sorted(set(lpre) | set(lpost))})
    for name, d in duties.items():
        if sorted(d) != DUTY_FIELDS:
            problems.append({"duty": name, "field": "field set", "fields": sorted(d)})
        accrued = discharged = 0
        for e in ex["effects"]:
            if e["kind"] == "Liability" and e["name"] == name:
                b, a = int(e["before"]), int(e["after"])
                accrued += max(a - b, 0)
                discharged += max(b - a, 0)
        opening, closing = lpre.get(name, 0), lpost.get(name, 0)
        derived = {"opening": str(opening), "closing": str(closing), "accrued": str(accrued), "discharged": str(discharged),
                   "present_before": name in lpre, "present_after": name in lpost,
                   "present_zero_after": name in lpost and closing == 0}
        for k, dv in derived.items():
            if d.get(k) != dv:
                problems.append({"duty": name, "field": k, "frozen": d.get(k), "derived_from_frozen_records": dv})
        if opening + accrued - discharged != closing:
            problems.append({"duty": name, "field": "evolution", "opening": opening, "accrued": accrued, "discharged": discharged, "closing": closing})
    residual = sorted(n for n in lpost if lpost[n] > 0)
    if sorted(acc["residual_positive"]) != residual:
        problems.append({"field": "residual_positive", "frozen": acc["residual_positive"], "derived_from_frozen_records": residual})
    if sorted(acc) != ["balances", "duties", "residual_positive"]:
        problems.append({"field": "accounting block keys", "keys": sorted(acc)})
    return problems


def main():
    vb = open(VECTORS, "rb").read()
    ab = open(ADDENDUM2, "rb").read()
    doc, add2 = json.loads(vb), json.loads(ab)
    accepted, rejected, contradictions = [], [], {}
    for v in doc["vectors"]:
        ex = v["expected"]
        if ex["accepted"]:
            block = json.dumps(ex["accounting"], sort_keys=True, separators=(",", ":")).encode()
            accepted.append({"id": v["id"], "program": v["program"], "action": v["action"], "logical_cap": v.get("logical_cap"),
                             "work_used": ex["work_used"], "accounting_compact_sha256": sha(block),
                             "balance_rows": len(ex["accounting"]["balances"]), "duty_rows": len(ex["accounting"]["duties"]),
                             "debited_rows": sum(1 for r in ex["accounting"]["balances"] if r["gross_debit"] != "0"),
                             "assets": len({akey(r["asset"]) for r in ex["accounting"]["balances"]}),
                             "self_transfer_rows": sum(1 for r in ex["accounting"]["balances"] if r["gross_debit"] != r["gross_debit_excluding_self"])})
            p = check(v)
            if p:
                contradictions[v["id"]] = p
        else:
            rejected.append({"id": v["id"], "program": v["program"], "action": v["action"], "logical_cap": v.get("logical_cap"),
                             "error": ex["error"], "step": ex["step"], "phase": ex["phase"], "work_attempted": ex["work_attempted"]})
    add2_rows = [{"id": v["id"], "accepted": v["expected"]["accepted"], "has_accounting": "accounting" in v["expected"],
                  "phase": v["expected"].get("phase"), "work_attempted": v["expected"].get("work_attempted")} for v in add2["vectors"]]
    rec = {
        "format": "evidence-repair-financial-expectations-freeze/1",
        "author": "claude-opus-5-5 (evidence-repair source author); pins only, no new expectation",
        "frozen_at_utc": datetime.now(timezone.utc).isoformat(),
        "sources": {"HELDOUT-VECTORS.json": {"path": VECTORS, "sha256": sha(vb), "bytes": len(vb)},
                    "HELDOUT-ADDENDUM-2.json": {"path": ADDENDUM2, "sha256": sha(ab), "bytes": len(ab)}},
        "provenance": "Reviewer-frozen (claude-opus-5-5 independent source reviewer, HELDOUT-FREEZE aggregate 7f6a4d1e...) before that reviewer read the candidate; projections defined by that reviewer ('Accounting fields ... are reviewer-defined projections of Core effects'). Addendum-2 is post-freeze.",
        "accepted_vectors_with_accounting": accepted,
        "rejected_vectors_phase_work": rejected,
        "addendum_2_rows": add2_rows,
        "addendum_2_note": "H8-P1/H8-P2 carry no frozen accounting block; the harness reports them as 'no frozen accounting', never as a zero-default pass.",
        "frozen_field_inventory": {"balance_row": BALANCE_FIELDS, "duty_row": DUTY_FIELDS, "block": ["balances", "duties", "residual_positive"],
                                   "work": ["work_used (accepted)", "work_attempted, phase, step, error (rejected)"]},
        "not_represented_in_frozen_or_relation": NOT_REPRESENTED,
        "internal_consistency": {"rule": "Each frozen accounting field recomputed by this script from the same frozen pre/post/effects records (stdlib arithmetic). A difference is recorded verbatim as a contradiction; no frozen value is changed.",
                                 "accepted_checked": len(accepted), "vectors_with_contradictions": len(contradictions), "contradictions": contradictions},
        "counts": {"accepted": len(accepted), "rejected": len(rejected),
                   "accepted_multi_asset": sum(1 for a in accepted if a["assets"] > 1),
                   "accepted_with_self_transfer": sum(1 for a in accepted if a["self_transfer_rows"]),
                   "accepted_with_debits": sum(1 for a in accepted if a["debited_rows"]),
                   "accepted_capped": sum(1 for a in accepted if a["logical_cap"]),
                   "rejected_admission_phase": sum(1 for r in rejected if r["phase"] == "admission")},
    }
    out = os.path.join(HERE, "FINANCIAL-EXPECTATIONS-FREEZE.json")
    with open(out, "x") as f:
        json.dump(rec, f, indent=1, sort_keys=True)
        f.write("\n")
    print(json.dumps({"counts": rec["counts"], "contradictions": contradictions, "sha256": sha(open(out, "rb").read())}, indent=1))


if __name__ == "__main__":
    main()
