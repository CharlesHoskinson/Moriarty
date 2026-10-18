"""AUTH predictions of the r3 harness row counts (stdlib only; read-only inputs).

Inputs: the frozen reviewer vectors, hostile addendum and addendum 2 (counts and ids only)
and program facts read from the frozen program sources by this author (TOUCHED, DEBIT,
REGISTRY below; H3 from its JSON). Output: records/PREDICTED-ROWS.json (exclusive).
These are predictions for review, never evidence; an actual run prints the real rows.
"""
import json, os, resource

resource.setrlimit(resource.RLIMIT_CPU, (60, 60))
resource.setrlimit(resource.RLIMIT_AS, (1 << 30, 1 << 30))

PKT = "/home/charl/moriarty-eutxo-validation-2026-10-12/current-core-groth16-compiler-independent-source-review-20261015"
OWN = "/home/charl/moriarty-eutxo-validation-2026-10-12/current-core-groth16-evidence-repair-resume-source-20261015"
V = json.load(open(PKT + "/heldout/HELDOUT-VECTORS.json"))["vectors"]
A2 = json.load(open(PKT + "/heldout-addendum-2/HELDOUT-ADDENDUM-2.json"))["vectors"]

# Read by this author from H1-settle.mori, H2-accrual.mori, H3-dag.program.json, H8-discharge.mori.
TOUCHED = {("H1-settle", "settle"): {("usdx", "buyer"), ("usdx", "seller"), ("usdx", "operator"), ("pts", "seller")},
           ("H1-settle", "close"): {("usdx", "seller"), ("usdx", "buyer")},
           ("H2-accrual", "repay"): {("credit", "borrower"), ("credit", "lender")},
           ("H3-dag", "pay"): {("tok", "x"), ("tok", "y")}, ("H3-dag", "poly_u"): {("tok", "x"), ("tok", "y")}}
DEBIT = {("H1-settle", "settle"): 2, ("H1-settle", "close"): 1, ("H2-accrual", "repay"): 1, ("H3-dag", "pay"): 1, ("H3-dag", "poly_u"): 1}
REGISTRY = {"H1-settle": {"warranty"}, "H2-accrual": {"loan"}, "H3-dag": {"d"}, "H8-discharge": {"d"}}

acc = [v for v in V if v["expected"]["accepted"]]
rej = [v for v in V if not v["expected"]["accepted"]]
pred = {"format": "evidence-repair-resume-predicted-rows/1", "status": "AUTH predictions only; not evidence", "tables": {}}


def put(name, rows, ok, fail, opn, classes, note=""):
    assert ok + fail + opn == rows, name
    pred["tables"][name] = {"rows": rows, "ok": ok, "fail": fail, "open": opn, "classes": classes, "note": note}


put("heldout", 3 * len(acc) + len(rej), 3 * len(acc) + len(rej), 0, 0,
    {"CORE-EQUAL": len(acc), "W": len(acc), "SAT": len(acc), "CORE-REFUSAL": len(rej)}, "same rows and expectations as the executed r2 run")
put("hostile", 32, 28, 0, 4, {"UNSAT": 27, "PREMISE": 2, "NOT-DECODABLE": 2, "COVERAGE": 1},
    "27 relation refusals (N-01); H1-R6/H1-R7 PREMISE and H1-R11/H2-R5 NOT-DECODABLE are OPEN, never OK")
put("mutations", 23, 20, 0, 3, {"UNSAT": 14, "SAT": 4, "NOT-ENCODABLE": 2, "TARGET-OBLIGATION": 1, "TARGET-GAP": 1, "CONTRADICTION": 1},
    "M-14 SAT is the expected F-04 gap, never security closure")
put("addendum2", 8, 8, 0, 0, {"CORE-REFUSAL": 1, "UNSAT": 1, "CORE-EQUAL": 2, "SAT": 2, "W": 2})
put("static", 18, 18, 0, 0, {"REFUSED": 7, "ADMITTED": 6, "BOUND": 5})
put("addendum3", 4, 4, 0, 0, {"CORE-REFUSAL": 2, "UNSAT": 2})

outside = 0
for v in acc:
    key = (v["program"], v["action"])
    reps = TOUCHED.get(key, set())
    a = v["expected"]["accounting"]
    rows_out = [b for b in a["balances"] if (b["asset"]["reference"], b["account"]) not in reps]
    duties_out = [d for d in a["duties"] if d["name"] not in REGISTRY.get(v["program"], set())]
    assert all(b["gross_debit"] == "0" for b in rows_out), v["id"]
    if rows_out or duties_out:
        outside += 1
put("accounting", 3 * len(acc) + outside + len(A2) - 1, 2 * len(acc), 0, len(acc) + outside + len(A2) - 1,
    {"CORE-ACCOUNTING": len(acc), "STATEMENT-SLOTS": len(acc), "NOT-REPRESENTED": len(acc), "NOT-IN-STATEMENT": outside, "NO-FROZEN-ACCOUNTING": len(A2) - 1},
    "reserve/DUST/service/recovery planes are OPEN NOT-REPRESENTED rows, never a zero-default pass")

tamper = sum(2 * DEBIT.get((v["program"], v["action"]), 0) for v in acc)
frame_rows = 1 + 1 + 2 * 2 + 1
put("grossfees", 2 * len(acc) + tamper + frame_rows, 2 * len(acc) + tamper + frame_rows, 0, 0,
    {"SAT": len(acc) + 1, "BOUND-SUMS": len(acc) + 1, "UNSAT": tamper + 4, "STATEMENT-SLOTS": 1},
    "tamper rows = 2 per plan debit cell (static per action), frame variant H1-P1/frame adds 7 rows")
rej_all = rej + [v for v in A2 if not v["expected"]["accepted"]]
uncapped = sum(1 for v in rej_all if not v.get("logical_cap"))
put("rejected", 2 * len(rej_all) + uncapped, 2 * len(rej_all) + uncapped, 0, 0,
    {"PHASE": len(rej_all), "WORK-ATTEMPTED": len(rej_all), "METHOD-CONTROL": uncapped})
put("hw25", 4, 2, 0, 2, {"CORE-REFUSAL": 1, "LAYOUT-ONE-SLOT": 1, "TARGET-PREMISE": 1, "<observed class>": 1}, "the encoder row is an OPEN observation")
put("hwtrace", 28, 2, 0, 26, {"TRACE-FORM": 2, "RELATION-EXECUTED": 14, "RELATION-PARTIAL": 8, "ENCODER-ONLY": 1, "CORE-AND-PREMISE": 2, "STRUCTURAL-SOURCE": 1},
    "every HW entry is OPEN here; the post-run parser resolves refs")

configs = set()
for v in V:
    cap = bool(v.get("logical_cap"))
    configs.add((v["program"], v["action"], cap, False, False))
    if v["expected"]["accepted"]:
        configs.add((v["program"], v["action"], cap, False, True))
for a in ["open", "bump", "pay", "poly_u", "poly_b"]:
    configs.add(("H3-dag", a, False, False, False))
configs |= {("H4-emit", "tick", False, False, False), ("H8-discharge", "pay", False, False, False), ("H8-discharge", "open", False, False, False),
            ("H1-settle", "settle", False, True, False), ("H1-settle", "settle", False, True, True), ("H1-settle-usdy", "settle", False, False, False)}
put("probe", len(configs), len(configs), 0, 0, {"PROBE": len(configs)}, "values are future observations; only build-digest equality and nonzero r1cs/constraints are asserted")
put("addendum4", 12, 11, 0, 1, {"TRACE-FORM": 1, "IDENTITY": 1, "SAT": 2, "UNSAT": 6, "ASSIGNMENT-SHAPE": 1, "NOT-APPLICABLE": 1})
pred["tests"] = {"E5-r3": 14, "E4": 41}
pred["identity_lines"] = 3
pred["totals"] = {k: sum(t[k] for t in pred["tables"].values()) for k in ("rows", "ok", "fail", "open")}
fd = os.open(OWN + "/records/PREDICTED-ROWS.json", os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o644)
with os.fdopen(fd, "w") as f:
    json.dump(pred, f, indent=1, sort_keys=True)
    f.write("\n")
print(json.dumps({k: (v["rows"], v["ok"], v["open"]) for k, v in pred["tables"].items()}), pred["totals"])
