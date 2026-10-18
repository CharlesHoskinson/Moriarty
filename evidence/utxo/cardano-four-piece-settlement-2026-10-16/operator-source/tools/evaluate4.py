#!/usr/bin/env python3
"""OPX7f: outcome classification, evaluated predictions and charged ledger (O''). NOT RUN BY ITS AUTHOR.

Usage: python3 -I evaluate4.py T OUT_JSON

Reads only files under T (exit files, logs, receipts, journal/elapsed.txt) and writes OUT_JSON
with mode "x". It never re-runs, edits or refits anything; it reads what the steps recorded and
applies the classification table of EXECUTION-PROPOSAL.json. Exit 0 when written; 2 on usage.

Axes kept separate: (a) did every command exit 0; (b) did every frozen prediction hold;
(c) the outcome labels. A FAILED prediction never turns a command failure into success, and a
clean command record never hides a missed prediction. OPX0's exit is root-recorded (OPX0 writes
nothing); OPX7f's own exit and elapsed line are written after this record, so GREEN here is
conditional on T/OPX7f.exit = 0, which the root confirms.
"""
import json
import os
import re
import sys

STEPS = ["OPX1", "OPX1t", "OPX1p", "OPX2a", "OPX3", "OPX3p", "OPX4", "OPX5", "OPX6",
         "OPX2", "OPX2p", "OPX2x", "OPX7"]
B_FIXED = 75
B_TOTAL = 1800
OPX7F_MAX = 35
LOCATION = re.compile(r"\[(\./[^\]:]+):(\d+):(\d+)\]")
AS_TEXT = ("memory allocation of", "Cannot allocate memory", "out of memory")
LINEAGE = {
    "attempt1": "current-cardano-sharded-settlement-local-measurement-20261015: FAILED(candidate compile) at shard.ak:186:3 (parser), 14 calls, 90 s charged, 0/472 tests. Retained, not reset.",
    "attempt2": "current-cardano-sharded-settlement-parser-repair-local-measurement-20261016: 14 steps exit 0, 472/472 expected outcomes (61 fail-annotated), PS-S4 FALSE (vs 20,503 B), 934 s charged. Raw; actual-result review pending. Retained, not reset.",
    "attempt3": "first four-piece run current-cardano-four-way-vs-local-measurement-20261016: 15 calls, 446 s charged, 129/147 pass, 18 FAIL, T-147/PC4-4/RANGE-vs1 FALSE. Retained, not reset.",
    "thisRun": "first harness-repair run of corrected candidate M2 (54e0fc56...) through the harness-repair successor operator; not a retry; prior ledgers (attempt1 14 calls/90 s + attempt2 14 calls/934 s + first four-piece 15 calls/446 s = 43 calls/1470 s charged) stay separate.",
    "compileFailureCyclesBefore": 1,
}
STILL_OPEN = ["SR-06", "SR-08", "SR-09", "SR-11 (unit-tested only)", "SR-12", "SR-15", "SR-16",
              "debtor consent for initial liabilities", "D-7", "OBS-SH-01", "RR-03", "RR-08", "RR-16",
              "PB-01..PB-12 (PB-02 predicted worse)", "FX1", "X1/PX04", "OEX7", "K2", "M-HASH", "M-REG", "M-SZ",
              "M-EU", "deploy- and settle-transaction size", "ledger ExUnits and fees",
              "all 20 UTxO pattern families", "ACTUS 277/18/32/DS01-07", "DeFi 72/DA01-24/held-outs/intents/regressions",
              "universal/PCD recursion", "private handoff", "multi-parent composition",
              "every financial obligation (gross debit, fees, net receipts, liabilities, residual duties) at ledger level",
              "every U0-U7, M1-M17, MC, SP and G exit"]


def read(path):
    try:
        with open(path, encoding="utf-8", errors="replace") as f:
            return f.read()
    except OSError:
        return None


def read_json(path):
    t = read(path)
    if t is None:
        return None
    try:
        return json.loads(t)
    except ValueError:
        return {"unparseable": True}


def exit_of(t, sid):
    v = read(os.path.join(t, "logs", sid + ".exit"))
    if v is None:
        return None
    try:
        return int(v.strip())
    except ValueError:
        return "unparseable"


def resource_text(log):
    return log is not None and any(s in log for s in AS_TEXT)


def compile_label(log, failed_label):
    loc = LOCATION.search(log or "")
    first = next((l.strip() for l in (log or "").split("\n") if l.strip().startswith("Error aiken::")), None)
    where = None if loc is None else {"path": loc.group(1), "line": int(loc.group(2)), "col": int(loc.group(3))}
    if where and where["path"].startswith("./build/packages/"):
        return "BLOCKED(pin-compat)", first, where
    if where and (where["path"].startswith("./lib/") or where["path"].startswith("./validators/")):
        return failed_label, first, where
    return "UNCLASSIFIED(%s-exit)" % failed_label.split("(")[1].rstrip(")").replace(" ", "-"), first, where


def aiken_stage(code, log, failed_label):
    """Label of an aiken step (OPX2a, OPX3, OPX2); resource takes precedence."""
    if code is None:
        return {"label": "BLOCKED(tool-abnormal)", "why": "exit file missing"}
    if code == 200:
        return {"label": "BLOCKED(resource-aggregate)"}
    if code == 201:
        return {"label": "NOT-STARTED(gate)"}
    if code in (124, 137) or (code != 0 and resource_text(log)):
        return {"label": "BLOCKED(resource)", "exit": code}
    if code == "unparseable" or (isinstance(code, int) and code >= 128):
        return {"label": "BLOCKED(tool-abnormal)", "exit": code}
    if code == 0:
        return {"label": "PASS"}
    label, first, where = compile_label(log, failed_label)
    return {"label": label, "exit": code, "firstError": first, "firstLocation": where}


def verdict(v):
    return "TRUE" if v is True else "FALSE" if v is False else "UNEVALUATED"


def both(*vals):
    if any(v is False for v in vals):
        return False
    if all(v is True for v in vals):
        return True
    return None


def main():
    if len(sys.argv) != 3:
        sys.stderr.write("usage: evaluate4.py T OUT_JSON\n")
        sys.exit(2)
    t, out = sys.argv[1:3]
    rc = os.path.join(t, "receipts")
    exits = {s: exit_of(t, s) for s in STEPS}
    logs = {s: read(os.path.join(t, "logs", s + ".log")) for s in ("OPX2a", "OPX3", "OPX2", "OPX4")}
    cmp_ = read_json(os.path.join(rc, "OPX2-compare.json"))
    chain = read_json(os.path.join(rc, "chain.json"))
    sizes = read_json(os.path.join(rc, "sizes.json"))
    labels = read_json(os.path.join(rc, "labels.json"))
    trees = {k: read_json(os.path.join(rc, "tree-%s.json" % k)) for k in ("placed", "after-build", "after-tests", "final")}

    stages = {}
    stages["staging"] = {"label": "PASS" if exits["OPX1t"] == 0 else "BLOCKED(tool-identity-or-help)", "exit": exits["OPX1t"]}
    stages["placement"] = {"label": "PASS" if exits["OPX1p"] == 0 else "BLOCKED(layout)", "exit": exits["OPX1p"]}
    stages["typecheck"] = aiken_stage(exits["OPX2a"], logs["OPX2a"], "FAILED(candidate compile)")
    stages["build"] = aiken_stage(exits["OPX3"], logs["OPX3"], "FAILED(candidate build)")
    e4 = exits["OPX4"]
    if e4 in (200, 201):
        stages["hashChain"] = {"label": "BLOCKED(resource-aggregate)" if e4 == 200 else "NOT-STARTED(gate)"}
    elif chain and not chain.get("unparseable"):
        stages["hashChain"] = {"label": "PASS" if chain.get("ok") and e4 == 0 else "FAILED(hash-chain)",
                               "exit": e4, "problems": chain.get("problems")}
    elif e4 in (124, 137):
        stages["hashChain"] = {"label": "BLOCKED(resource)", "exit": e4}
    else:
        stages["hashChain"] = {"label": "UNCLASSIFIED(apply-or-hash-refusal)", "exit": e4, "see": "logs/OPX4.log"}
    tests = aiken_stage(exits["OPX2"], logs["OPX2"], "FAILED(candidate compile)")
    if tests["label"] in ("PASS", "FAILED(candidate compile)", "UNCLASSIFIED(candidate-compile-exit)"):
        if cmp_ is None or cmp_.get("unparseable"):
            tests = {"label": "UNCLASSIFIED(no-comparison)", "exit": exits["OPX2"], "opx2x": exits["OPX2x"]}
        elif not cmp_.get("reportParsed"):
            diag = cmp_.get("compileDiagnostic") or {}
            loc = (diag.get("firstLocation") or {}).get("path", "")
            if loc.startswith("./lib/") or loc.startswith("./validators/"):
                tests = {"label": "FAILED(candidate compile)", "exit": exits["OPX2"], "diagnostic": diag}
            elif loc.startswith("./build/packages/"):
                tests = {"label": "BLOCKED(pin-compat)", "exit": exits["OPX2"], "diagnostic": diag}
            else:
                tests = {"label": "UNCLASSIFIED(report-schema)", "exit": exits["OPX2"], "reason": cmp_.get("reason")}
        else:
            lab = cmp_.get("classification")
            if lab == "PREDICTIONS_HOLD" and exits["OPX2"] != 0:
                lab = "UNCLASSIFIED(exit-vs-report)"
            tests = {"label": lab, "exit": exits["OPX2"], "matchFilterAnomaly": cmp_.get("matchFilterAnomaly"),
                     "summary": cmp_.get("summary")}
    stages["tests"] = tests
    hyg = {}
    for k, sid in (("after-build", "OPX3p"), ("after-tests", "OPX2p"), ("final", "OPX7")):
        tr = trees.get(k)
        step_failed = sid != "OPX7" and exits[sid] not in (0, 200, 201, None)
        if tr is None:
            hyg[k] = {"label": "BLOCKED(hygiene)" if step_failed else "NOT-RECORDED", "stepExit": exits[sid]}
        elif tr.get("ok"):
            hyg[k] = {"label": "BLOCKED(hygiene)" if step_failed else "PASS", "stepExit": exits[sid],
                      "note": "tree check held; another check of the step failed (binary or empty dirs; see its log)" if step_failed else None}
        else:
            cats = tr.get("categories") or []
            hyg[k] = {"label": "BLOCKED(fetch)" if "fetch" in cats else "BLOCKED(hygiene)",
                      "stepExit": exits[sid], "problems": tr.get("problems")}
    stages["hygiene"] = hyg
    stages["integrity"] = {"label": "PASS" if exits["OPX7"] == 0 else "BLOCKED(integrity)", "exit": exits["OPX7"]}
    stages["sizes"] = {"label": ((sizes or {}).get("fit") or {}).get("verdict", "UNMEASURED"),
                       "applied": {g: v.get("appliedBytes") for g, v in ((sizes or {}).get("validators") or {}).items()}}
    stages["labels"] = {"label": ((labels or {}).get("PS4-7") or {}).get("verdict", "UNEVALUATED")}

    dp = (sizes or {}).get("directionalPredictions") or {}
    cp = (chain or {}).get("predictions") or {}
    directional = ((cmp_ or {}).get("directional") or {})

    def pc(key):
        if not directional.get("evaluated"):
            return None, None
        m, c = directional["mem"][key]["holds"], directional["cpu"][key]["holds"]
        return both(m, c), {"mem": directional["mem"][key], "cpu": directional["cpu"][key]}

    tests_holds = {"PREDICTIONS_HOLD": True, "FAILED(prediction)": False}.get(tests.get("label"))
    preds = [
        {"id": "T-147", "claim": "147 tests (125 + 22), names and fail flags exactly EXPECTATIONS.json, every status pass",
         "source": "receipts/OPX2-compare.json", "verdict": verdict(tests_holds), "label": tests.get("label")},
    ]
    for pid, key in (("PS4-1", "PS4-1 each piece < 20,503"), ("PS4-2", "PS4-2 each piece <= 14,336"),
                     ("PS4-3", "PS4-3 sum of pieces > 20,503"), ("PS4-4", "PS4-4 s2p <= 14,336")):
        row = dp.get(key) or {}
        preds.append({"id": pid, "claim": key, "source": "receipts/sizes.json", "verdict": verdict(row.get("holds")),
                      "detail": row})
    preds.append({"id": "PS4-5", "claim": "vi applied = 12,784 B and its B1 compiled-code sha256 equals attempt 2's 593dc053...",
                  "source": "receipts/sizes.json + receipts/chain.json",
                  "verdict": verdict(both((dp.get("PS4-5 size half: vi = 12,784") or {}).get("holds"),
                                          cp.get("PS4-5 viB1CompiledCodeSha256EqualsAttempt2")))})
    preds.append({"id": "PS4-6", "claim": "c2 applied = 2,770 B and hash 6a4b67e7...",
                  "source": "receipts/sizes.json + receipts/chain.json",
                  "verdict": verdict(both((dp.get("PS4-6 size half: c2 = 2,770") or {}).get("holds"),
                                          cp.get("c2HashEqualsAttempt2")))})
    preds.append({"id": "PS4-7", "claim": "no R1 diagnostic label in any compiled validator", "source": "receipts/labels.json",
                  "verdict": verdict(((labels or {}).get("PS4-7") or {}).get("holds"))})
    for pid in ("PC4-1", "PC4-2", "PC4-3", "PC4-4"):
        v, d = pc(pid)
        preds.append({"id": pid, "claim": "EXPECTATIONS directionalPredictions %s (mem and cpu, host CEK)" % pid,
                      "source": "receipts/OPX2-compare.json", "verdict": verdict(v), "detail": d})
    preds.append({"id": "OPX4-vi", "claim": "vi hash equals attempt 2's fc226622...", "source": "receipts/chain.json",
                  "verdict": verdict(cp.get("viHashEqualsAttempt2"))})
    tb = trees.get("after-build") or {}
    e2 = read(os.path.join(t, "journal", "elapsed.txt")) or ""
    elapsed = {}
    for line in e2.split("\n"):
        f = line.split()
        if len(f) == 2 and f[1].isdigit():
            elapsed[f[0]] = elapsed.get(f[0], 0) + int(f[1])
    charged = B_FIXED + sum(elapsed.values())
    preds += [
        {"id": "OP-P1", "claim": "--match-tests pvc2/shard4_test --match-tests pvc2/shard4_cost_test selects exactly the 147 tests of the two modules",
         "source": "receipts/OPX2-compare.json",
         "verdict": verdict(None if not (cmp_ or {}).get("reportParsed") else not cmp_.get("matchFilterAnomaly"))},
        {"id": "OP-P2", "claim": "OPX2 elapsed <= 1,000 s", "source": "journal/elapsed.txt",
         "verdict": verdict(None if "OPX2" not in elapsed or exits["OPX2"] in (200, 201) else elapsed["OPX2"] <= 1000),
         "observed": elapsed.get("OPX2")},
        {"id": "OP-P3", "claim": "OPX2a elapsed <= 120 s", "source": "journal/elapsed.txt",
         "verdict": verdict(None if "OPX2a" not in elapsed or exits["OPX2a"] in (200, 201) else elapsed["OPX2a"] <= 120),
         "observed": elapsed.get("OPX2a")},
        {"id": "OP-P4", "claim": "charged aggregate including OPX7f's maximum <= 1,300 s", "source": "journal/elapsed.txt",
         "verdict": verdict(charged + OPX7F_MAX <= 1300), "observed": charged},
        {"id": "OP-P5", "claim": "aiken.lock bytes equal attempt 2's ec5d6a58...", "source": "receipts/tree-after-build.json",
         "verdict": verdict((tb.get("aikenLock") or {}).get("equalsParentAttempt2Lock"))},
    ]
    for p in (dp.get("pointRangesInformative") or {}).items():
        preds.append({"id": "RANGE-" + p[0], "claim": "author point range %s (informative)" % p[1]["range"],
                      "source": "receipts/sizes.json", "verdict": verdict(p[1]["inRange"]), "informative": True})

    all_exit0 = all(exits[s] == 0 for s in STEPS)
    missed = [p["id"] for p in preds if p["verdict"] == "FALSE"]
    unevaluated = [p["id"] for p in preds if p["verdict"] == "UNEVALUATED"]
    labels_out = []
    green = all_exit0 and tests.get("label") == "PREDICTIONS_HOLD" and stages["hashChain"]["label"] == "PASS"
    if green:
        labels_out.append("GREEN(source-measured), conditional on OPX7f.exit = 0")
    for k in ("staging", "placement", "typecheck", "build", "hashChain", "tests", "integrity"):
        lab = stages[k]["label"]
        if lab not in ("PASS", "PREDICTIONS_HOLD", "NOT-STARTED(gate)"):
            labels_out.append("%s: %s" % (k, lab))
    for k, v in hyg.items():
        if v["label"] not in ("PASS", "NOT-RECORDED"):
            labels_out.append("hygiene %s: %s" % (k, v["label"]))
    labels_out.append(stages["sizes"]["label"])
    for pid in missed:
        labels_out.append("FAILED prediction %s (preserved; never refit)" % pid)
    compile_cycles = LINEAGE["compileFailureCyclesBefore"] + int(
        stages["typecheck"]["label"] == "FAILED(candidate compile)" or stages["tests"].get("label") == "FAILED(candidate compile)")
    doc = {
        "format": "four-way-vs-operator/outcome/1",
        "taskDirectory": t,
        "exits": exits,
        "opx0": "root-recorded (OPX0 writes nothing)",
        "opx7f": "pending at this record; the root reads T/OPX7f.exit",
        "allCommandsExit0": all_exit0,
        "allPredictionsHeld": not missed and not unevaluated,
        "missedPredictions": missed,
        "unevaluatedPredictions": unevaluated,
        "labels": labels_out,
        "stages": stages,
        "predictions": preds,
        "ledger": {"aggregateCapSeconds": B_TOTAL, "fixedCharges": {"OPX0": 70, "OPX1": 5},
                   "elapsedByStep": elapsed, "chargedBeforeOPX7f": charged,
                   "note": "OPX7f appends its own line after this record; the final charge is 75 + the sum of journal/elapsed.txt."},
        "lineage": dict(LINEAGE, compileFailureCyclesAfter=compile_cycles,
                        rule=("A second compile-class failure means: preserve it, reproduce it and change approach "
                              "(for example a whole-package diagnostic pass) before any further broad run.")
                        if compile_cycles >= 2 else "no new compile-class failure"),
        "scope": ("Local Aiken v1.1.24 type check, CEK unit tests through test_ctx, build, parameter application, tool "
                  "hashes, compiled sizes and a label search, offline. Host CEK figures are library-predicate costs under "
                  "aiken's default cost model: no compiled-wrapper or ledger ExUnits, signatures, proofs, keys, "
                  "transactions, registration, deployment or budget fit are measured or implied."),
        "stillOpen": STILL_OPEN,
        "nativeCryptoAuthority": "zero (unchanged)",
        "adoption": "none; a fresh independent actual-result review decides any adoption",
    }
    with open(out, "x", encoding="utf-8") as f:
        json.dump(doc, f, indent=1)
        f.write("\n")
    print("outcome: %s" % "; ".join(labels_out))
    sys.exit(0)


if __name__ == "__main__":
    main()
