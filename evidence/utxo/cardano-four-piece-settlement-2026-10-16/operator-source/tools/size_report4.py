#!/usr/bin/env python3
"""OPX6: read-only, non-gating compiled-size report for the seven validators (O''). NOT RUN BY ITS AUTHOR.

Usage: python3 -I size_report4.py OUT_JSON PARENT_SIZES_JSON UNAPPLIED_BLUEPRINT [APPLIED_BLUEPRINT]

Size = byte length of the blueprint compiledCode (hex / 2), the quantity attempt 2 reported
(c2 2,770, vi 12,784, vs 20,503, s2p 12,356 applied). PARENT_SIZES_JSON is attempt 2's raw
sizes.json (pinned 804a8c44...), read for comparison only, not adopted. Writes OUT_JSON with
mode "x". Exit 0 when written; 2 on usage or pin.

14,336 B is the working per-script target (M-SZ recommendation), NOT a qualified node or
protocol parameter. 16,384 B was captured from an unauthenticated provider; it is recorded as
context only and receives no verdict. No size here is a deploy-transaction, reference-script
transaction or settle-transaction size, nor ExUnits, nor a fee.

Evaluated (EXPECTATIONS directionalPredictions, verbatim thresholds): PS4-1, PS4-2 (per piece),
PS4-3, PS4-4, and the size halves of PS4-5 (vi 12,784) and PS4-6 (c2 2,770); the author's point
ranges are recorded as informative in-range flags. Each miss is preserved as a FAILED
prediction, independent of every command exiting 0.
"""
import hashlib
import json
import sys

GROUPS = ["c2", "vi", "vs1", "vs2", "vs3", "vs4", "s2p"]
PIECES = ["vs1", "vs2", "vs3", "vs4"]
WORKING_TARGET = 14336
UNAUTHENTICATED_PROVIDER_MAX_TX = 16384
PARENT_SIZES_SHA256 = "804a8c4453897160a86edeca5d8017fe41818dc2f4fca1fc38fa32abdc9ac971"
PARENT_VS_APPLIED = 20503
POINT_RANGES = {"vs1": (7000, 10500), "vs2": (8500, 12000), "vs3": (9500, 13500), "vs4": (9500, 13500),
                "s2p": (12500, 13400), "sumPieces": (35000, 49000)}


def sizes(path):
    with open(path, "rb") as f:
        d = json.loads(f.read())
    out = {}
    for g in GROUPS:
        lens = sorted({len(v.get("compiledCode", "")) // 2 for v in d.get("validators", [])
                       if str(v.get("title", "")).startswith(g + "." + g + ".")})
        out[g] = lens[0] if len(lens) == 1 else (lens or None)
    return out


def main():
    if len(sys.argv) not in (4, 5):
        sys.stderr.write("usage: size_report4.py OUT_JSON PARENT_SIZES_JSON UNAPPLIED_BLUEPRINT [APPLIED_BLUEPRINT]\n")
        sys.exit(2)
    out, psizes = sys.argv[1], sys.argv[2]
    with open(psizes, "rb") as f:
        praw = f.read()
    if hashlib.sha256(praw).hexdigest() != PARENT_SIZES_SHA256:
        sys.stderr.write("parent sizes.json digest differs from pin\n")
        sys.exit(2)
    parent = {g: v.get("appliedBytes") for g, v in json.loads(praw)["validators"].items()}
    unapplied = sizes(sys.argv[3])
    applied = sizes(sys.argv[4]) if len(sys.argv) == 5 else None
    rows = {}
    for g in GROUPS:
        a = applied.get(g) if applied else None
        rows[g] = {"unappliedBytes": unapplied.get(g), "appliedBytes": a,
                   "withinWorkingTarget14336": (a <= WORKING_TARGET) if isinstance(a, int) else None}
    ints = applied is not None and all(isinstance(applied.get(g), int) for g in GROUPS)
    preds, fit, comparisons = None, None, None
    if ints:
        a = applied
        s = sum(a[p] for p in PIECES)
        preds = {
            "PS4-1 each piece < 20,503": {"perPiece": {p: a[p] < PARENT_VS_APPLIED for p in PIECES},
                                          "holds": all(a[p] < PARENT_VS_APPLIED for p in PIECES)},
            "PS4-2 each piece <= 14,336": {"perPiece": {p: a[p] <= WORKING_TARGET for p in PIECES},
                                           "holds": all(a[p] <= WORKING_TARGET for p in PIECES)},
            "PS4-3 sum of pieces > 20,503": {"sum": s, "holds": s > PARENT_VS_APPLIED},
            "PS4-4 s2p <= 14,336": {"holds": a["s2p"] <= WORKING_TARGET},
            "PS4-5 size half: vi = 12,784": {"holds": a["vi"] == 12784},
            "PS4-6 size half: c2 = 2,770": {"holds": a["c2"] == 2770},
            "pointRangesInformative": {k: {"range": list(r), "inRange": r[0] <= (s if k == "sumPieces" else a[k]) <= r[1]}
                                       for k, r in POINT_RANGES.items()},
        }
        over = [g for g in GROUPS if a[g] > WORKING_TARGET]
        fit = {"verdict": "FIT(working-target)" if not over else "NOT-FIT(working-target)", "over14336": over,
               "meaning": "every applied script <= 14,336 B" if not over else "preserved as failed PS4 predictions; the next step is a reviewed source change, never a rerun of these bytes"}
        total = sum(a[g] for g in GROUPS)
        ptotal = sum(parent[g] for g in ("c2", "vi", "vs", "s2p"))
        comparisons = {
            "attempt2AppliedBytes": parent,
            "sumPiecesMinusAttempt2Vs": s - parent["vs"],
            "s2pMinusAttempt2S2p": a["s2p"] - parent["s2p"],
            "viMinusAttempt2Vi": a["vi"] - parent["vi"],
            "c2MinusAttempt2C2": a["c2"] - parent["c2"],
            "allSevenApplied": total,
            "attempt2AllFourApplied": ptotal,
            "totalIncrease": total - ptotal,
            "largestPiece": max(PIECES, key=lambda p: a[p]),
        }
    doc = {
        "format": "four-way-vs-operator/opx6-sizes/1",
        "nonGating": True,
        "unappliedBlueprint": sys.argv[3],
        "appliedBlueprint": sys.argv[4] if applied else None,
        "validators": rows,
        "fit": fit,
        "directionalPredictions": preds,
        "comparisons": comparisons,
        "references": {"workingTargetBytes": WORKING_TARGET,
                       "workingTargetStatus": "M-SZ per-script recommendation; not a qualified node or protocol parameter",
                       "unauthenticatedProviderMaxTxBytes": UNAUTHENTICATED_PROVIDER_MAX_TX,
                       "unauthenticatedProviderStatus": "context only; no verdict",
                       "attempt2PsS4": "FALSE (vs 20,503 B), preserved"},
        "qualifier": "Compiled script bytes only: not a deploy, reference-script or settle transaction size (SR-15, PB-10), not ExUnits (PB-02), not fees, not deployability.",
    }
    with open(out, "x", encoding="utf-8") as f:
        json.dump(doc, f, indent=1)
        f.write("\n")
    print("OPX6 sizes: %s" % json.dumps({g: rows[g]["appliedBytes"] for g in GROUPS}))
    sys.exit(0)


if __name__ == "__main__":
    main()
