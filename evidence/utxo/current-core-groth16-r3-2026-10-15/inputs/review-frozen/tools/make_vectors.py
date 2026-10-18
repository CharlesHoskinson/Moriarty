#!/usr/bin/env python3
"""Reviewer-authored held-out vector generator. Derives expectations only from
tools/core_oracle.py (reviewer reimplementation of Core /1 rules) plus hand
arithmetic asserted inline. Imports no candidate or checkout code.
Usage: python3 -I make_vectors.py OUTDIR"""
import resource, sys, os, json, hashlib
resource.setrlimit(resource.RLIMIT_CPU, (60, 60))
resource.setrlimit(resource.RLIMIT_AS, (1 << 30, 1 << 30))
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import core_oracle as C  # reviewer-owned module in the same tools dir

OUT = sys.argv[1]
U = C.U128
P64 = 1 << 64

NET = "holdout-net"
USDX = (NET, "holdout-issuer", "usdx", 6)
PTS = (NET, "holdout-issuer", "pts", 0)
CRD = (NET, "holdout-bank", "credit", 2)
TOK = (NET, "holdout-dag", "tok", 0)

SRC = {}
SRC["H1-settle"] = '''language "moriarty/1";
agreement HoldoutSettle {
  state released: UInt = 0;
  state open: Bool = true;
  state rounds: UInt = 0;
  asset USDX = { network: "holdout-net", issuer: "holdout-issuer", reference: "usdx", scale: 6 };
  asset PTS = { network: "holdout-net", issuer: "holdout-issuer", reference: "pts", scale: 0 };
  action settle(gross: UInt, cut: UInt, hold: UInt, last: Bool) {
    requires open and gross > cut;
    let net = gross - cut;
    transfer(USDX, "buyer", "seller", net);
    fee(USDX, "buyer", "operator", cut);
    transfer(PTS, "seller", "seller", 5);
    liability("warranty", USDX, "seller", "buyer", hold);
    next released = released + net;
    next rounds = rounds + 1;
    ensures released <= 1000000 or not last;
    next open = not last;
  }
  action close(pay: UInt, left: UInt) {
    requires liability_present("warranty");
    transfer(USDX, "seller", "buyer", pay);
    discharge("warranty", pay);
    requires liability_amount("warranty") == left;
    requires liability_present("warranty");
    next open = false;
  }
}
'''
SRC["H2-accrual"] = '''language "moriarty/1";
agreement HoldoutAccrual {
  state periods: UInt = 0;
  state frozen: Bool = false;
  asset CRD = { network: "holdout-net", issuer: "holdout-bank", reference: "credit", scale: 2 };
  action accrue(rate: UInt, units: UInt) {
    requires not frozen;
    let due = rate * units;
    liability("loan", CRD, "borrower", "lender", due);
    requires liability_amount("loan") <= 50000;
    next periods = periods + 1;
  }
  action repay(pay: UInt, stop: Bool) {
    requires liability_present("loan");
    transfer(CRD, "borrower", "lender", pay);
    discharge("loan", pay);
    ensures liability_amount("loan") not= 0 or liability_present("loan");
    fee(CRD, "borrower", "borrower", 1);
    next frozen = stop;
  }
}
'''
SRC["H5-strict"] = '''language "moriarty/1";
agreement HoldoutStrict {
  state hits: UInt = 0;
  action probeOr(a: UInt, b: UInt, skip: Bool) {
    requires skip or a - b >= 0;
    next hits = hits + 1;
  }
  action probeAnd(a: UInt, b: UInt, run: Bool) {
    requires not (run and a * b >= 0) or run;
    next hits = hits + 1;
  }
}
'''
SRC["H6-bounds"] = '''language "moriarty/1";
agreement HoldoutBounds {
  state acc: UInt = 0;
  state flag: Bool = false;
  action add(x: UInt, y: UInt) { next acc = x + y; }
  action sub(x: UInt, y: UInt) { next acc = x - y; }
  action mul(x: UInt, y: UInt) { next acc = x * y; }
  action cmp(x: UInt, y: UInt) {
    requires x < y;
    requires y >= x;
    requires x <= y;
    requires y > x;
    requires x not= y;
    requires not (x == y);
    next flag = x + 1 == y;
  }
  action beq(p: Bool, q: Bool) {
    next flag = p == q;
    requires flag not= (p not= q);
  }
}
'''
SRC["H4-emit"] = '''language "moriarty/1";
agreement HoldoutEmit {
  state n: UInt = 0;
  action tick(k: UInt) {
    emit "before"(n);
    next n = n + k;
    emit "after"(n);
    emit "flag"(n > 10);
  }
}
'''
SRC["H7-staged"] = '''language "moriarty/2";
agreement HoldoutStaged {
  state n: UInt = 0;
  action go(k: UInt) {
    next n = n + k;
    await confirm(ok: Bool) when ok;
    next n = n + 1;
  }
}
'''


def lit(v):
    return {"kind": "Literal", "value": ("Bool", v) if isinstance(v, bool) else ("UInt", v)}


def name(n):
    return {"kind": "Name", "name": n}


def bin_(op, l, r):
    return {"kind": "Binary", "op": op, "lhs": l, "rhs": r}


def ident(t):
    return {"network": t[0], "issuer": t[1], "reference": t[2], "scale": t[3]}


# H3: JSON-IR program with deliberate DAG sharing, dead nodes and a node shared
# across actions under different operand types. Admissible by Core validate().
H3 = {"language": "moriarty/1", "semantics": "moriarty-core/1", "name": "HoldoutDag",
      "states": [{"name": "s", "ty": "UInt", "initial": ("UInt", 0)}],
      "assets": [{"name": "T", "identity": ident(TOK)}],
      "expressions": [
          name("s"),                 # 0
          lit(1),                    # 1
          bin_("Add", 0, 1),         # 2  s + 1 (shared by two Next statements)
          bin_("Mul", 2, 2),         # 3  (s+1)*(s+1), same child twice
          {"kind": "LiabilityAmount", "name": "d"},   # 4 shared before/after discharge
          {"kind": "LiabilityPresent", "name": "d"},  # 5
          lit(0),                    # 6
          bin_("Sub", 6, 1),         # 7  DEAD: would underflow if evaluated
          name("ghost"),             # 8  DEAD: unknown name, never type-checked
          name("v"),                 # 9  UInt in action 'poly_u', Bool in 'poly_b'
          bin_("Eq", 9, 9),          # 10 typed per action
          name("before"),            # 11
          bin_("Eq", 4, 11),         # 12 amount(d) == before
          name("after"),             # 13
          bin_("Eq", 4, 13),         # 14 amount(d) == after (same node 4)
          name("pay"),               # 15
          lit(10),                   # 16
          name("sq"),                # 17
          bin_("Ge", 17, 6),         # 18
          bin_("Add", 3, 3),         # 19 2*(s+1)^2 via doubly shared node
      ],
      "actions": [
          {"name": "open", "inputs": [], "statements": [
              {"kind": "Liability", "name": "d", "asset": "T", "debtor": "x", "creditor": "y", "amount": 16}]},
          {"name": "bump", "inputs": [], "statements": [
              {"kind": "Next", "state": "s", "value": 2},
              {"kind": "Next", "state": "s", "value": 2},
              {"kind": "Let", "name": "sq", "value": 19},
              {"kind": "Requires", "predicate": 18}]},
          {"name": "pay", "inputs": [{"name": "pay", "ty": "UInt"}, {"name": "before", "ty": "UInt"},
                                      {"name": "after", "ty": "UInt"}], "statements": [
              {"kind": "Requires", "predicate": 12},
              {"kind": "Requires", "predicate": 5},
              {"kind": "Transfer", "asset": "T", "from": "x", "to": "y", "amount": 15},
              {"kind": "Discharge", "name": "d", "amount": 15},
              {"kind": "Requires", "predicate": 14},
              {"kind": "Requires", "predicate": 5}]},
          {"name": "poly_u", "inputs": [{"name": "v", "ty": "UInt"}], "statements": [
              {"kind": "Requires", "predicate": 10},
              {"kind": "Transfer", "asset": "T", "from": "x", "to": "y", "amount": 9}]},
          {"name": "poly_b", "inputs": [{"name": "v", "ty": "Bool"}], "statements": [
              {"kind": "Requires", "predicate": 10},
              {"kind": "Requires", "predicate": 9}]},
      ]}


def wire_value(v):
    return {"type": v[0], "value": (str(v[1]) if v[0] == "UInt" else v[1])}


def wire_program(p):
    def wexpr(e):
        if e["kind"] == "Literal":
            return {"kind": "Literal", "value": wire_value(e["value"])}
        return dict(e)
    return {"language": p["language"], "semantics": p["semantics"], "name": p["name"],
            "states": [{"name": s["name"], "ty": s["ty"], "initial": wire_value(s["initial"])} for s in p["states"]],
            "assets": p["assets"], "expressions": [wexpr(e) for e in p["expressions"]],
            "actions": p["actions"]}


def wire_asset(t):
    return ident(t)


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
        if k in e and isinstance(e[k], int):
            e[k] = str(e[k])
    for k in ("before", "after", "value"):
        if k in e and isinstance(e[k], tuple):
            e[k] = wire_value(e[k])
    return e


def bal(acct, asset, amt):
    return {"account": acct, "asset": asset, "amount": amt}


def liab(n, asset, d, c, amt):
    return {"name": n, "asset": asset, "debtor": d, "creditor": c, "amount": amt}


def scen(inputs, balances=(), liabilities=(), state=None):
    return {"inputs": inputs, "balances": list(balances), "liabilities": list(liabilities), "state": state or {}}


def accounting(res):
    """Per (account, asset) gross debit (incl. self moves), fees paid, net delta;
    liability opening/accrued/discharged/closing; residual and present-zero rows."""
    acc = {}
    for e in res["effects"]:
        if e["kind"] in ("Transfer", "Fee"):
            k = (e["from"], e["asset"])
            a = acc.setdefault(k, {"gross_debit": 0, "gross_debit_excluding_self": 0, "fees_paid": 0})
            a["gross_debit"] += e["amount"]
            if e["from"] != e["to"]:
                a["gross_debit_excluding_self"] += e["amount"]
            if e["kind"] == "Fee":
                a["fees_paid"] += e["amount"]
    pre = {(b["account"], b["asset"]): b["amount"] for b in res["pre"]["balances"]}
    post = {(b["account"], b["asset"]): b["amount"] for b in res["post"]["balances"]}
    rows = []
    for k in sorted(set(pre) | set(post), key=lambda k: (k[1], k[0])):
        a = acc.get(k, {"gross_debit": 0, "gross_debit_excluding_self": 0, "fees_paid": 0})
        rows.append({"account": k[0], "asset": wire_asset(k[1]), "pre": str(pre[k]), "post": str(post[k]),
                     "net_delta": str(post[k] - pre[k]) if post[k] >= pre[k] else "-" + str(pre[k] - post[k]),
                     "gross_debit": str(a["gross_debit"]),
                     "gross_debit_excluding_self": str(a["gross_debit_excluding_self"]),
                     "fees_paid": str(a["fees_paid"])})
    duties = []
    prel = {l["name"]: l["amount"] for l in res["pre"]["liabilities"]}
    postl = {l["name"]: l["amount"] for l in res["post"]["liabilities"]}
    for n in sorted(set(prel) | set(postl)):
        accrued = sum(e["after"] - e["before"] for e in res["effects"]
                      if e["kind"] == "Liability" and e["name"] == n and e["after"] >= e["before"])
        discharged = sum(e["before"] - e["after"] for e in res["effects"]
                         if e["kind"] == "Liability" and e["name"] == n and e["after"] < e["before"])
        opening = prel.get(n, 0)
        closing = postl[n]
        assert opening + accrued - discharged == closing
        duties.append({"name": n, "present_before": n in prel, "present_after": n in postl,
                       "opening": str(opening), "accrued": str(accrued), "discharged": str(discharged),
                       "closing": str(closing), "present_zero_after": closing == 0})
    return {"balances": rows, "duties": duties,
            "residual_positive": [d["name"] for d in duties if int(d["closing"]) > 0]}


VECTORS = []


def run(vid, prog_key, prog, action, sc, note, hand=None, cap=None, category="financial"):
    res = C.simulate(prog, action, sc, cap)
    rec = {"id": vid, "category": category, "program": prog_key, "action": action, "note": note,
           "scenario": {"inputs": {k: wire_value(v) for k, v in sorted(sc["inputs"].items())},
                        "state": {k: wire_value(v) for k, v in sorted(sc["state"].items())},
                        "balances": [{"account": b["account"], "asset": wire_asset(b["asset"]), "amount": str(b["amount"])} for b in sc["balances"]],
                        "liabilities": [{"name": l["name"], "asset": wire_asset(l["asset"]), "debtor": l["debtor"],
                                         "creditor": l["creditor"], "amount": str(l["amount"])} for l in sc["liabilities"]]},
           "logical_cap": None if cap is None else str(cap)}
    if res["accepted"]:
        rec["expected"] = {"accepted": True, "work_used": res["work_used"],
                           "effects": [wire_effect(e) for e in res["effects"]],
                           "pre": wire_snapshot(res["pre"]), "post": wire_snapshot(res["post"]),
                           "accounting": accounting(res)}
    else:
        rec["expected"] = {"accepted": False, "error": res["error"], "step": res["step"],
                           "phase": res["phase"], "work_attempted": res["work_used"]}
    if hand:
        hand(res)  # inline hand-arithmetic cross-check (asserts)
    VECTORS.append(rec)
    return res


def main():
    os.makedirs(OUT, exist_ok=True)
    progs = {k: C.parse(v) for k, v in SRC.items() if k != "H7-staged"}
    C.validate(H3)
    progs["H3-dag"] = H3

    # ---------------- H1 settle / close (financial positives + rejections)
    H1 = progs["H1-settle"]
    base_bal = [bal("buyer", USDX, 1_000_000), bal("seller", USDX, 0), bal("operator", USDX, 3),
                bal("seller", PTS, 5), bal("auditor", USDX, 7)]
    legacy = liab("legacy", PTS, "someone", "else", 9)

    def h1_ok(r):
        assert r["accepted"]
        post = {(b["account"], b["asset"]): b["amount"] for b in r["post"]["balances"]}
        assert post[("buyer", USDX)] == 1_000_000 - 250_000
        assert post[("seller", USDX)] == 250_000 - 1_250
        assert post[("operator", USDX)] == 3 + 1_250
        assert post[("seller", PTS)] == 5 and post[("auditor", USDX)] == 7
        assert r["post"]["state"] == {"released": ("UInt", 248_750), "open": ("Bool", True), "rounds": ("UInt", 1)}
        assert [l["name"] for l in r["post"]["liabilities"]] == ["legacy", "warranty"]
        assert r["post"]["liabilities"][1]["amount"] == 0  # present-zero
        # work: 10 statements + nodes: req(and,open,>,gross,cut)=5, let(-,gross,cut)=3, t1(net)=1,
        # fee(cut)=1, t2(5)=1, liab(hold)=1, next(+,released,net)=3, next(+,rounds,1)=3,
        # ensures(or,<=,released,1000000,not,last)=6, next(not,last)=2  => 10+26 = 36
        assert r["work_used"] == 36, r["work_used"]
        assert [e["kind"] for e in r["effects"]] == ["Transfer", "Fee", "Transfer", "Liability", "State", "State", "State"]
    run("H1-P1", "H1-settle", H1, "settle",
        scen({"gross": ("UInt", 250_000), "cut": ("UInt", 1_250), "hold": ("UInt", 0), "last": ("Bool", False)},
             base_bal, [legacy]),
        "positive: net transfer, fee, self-transfer, present-zero duty creation, untouched frame rows (auditor, legacy)", h1_ok)
    run("H1-P2", "H1-settle", H1, "settle",
        scen({"gross": ("UInt", 1_000_000), "cut": ("UInt", 0), "hold": ("UInt", 40), "last": ("Bool", True)},
             base_bal, [liab("warranty", USDX, "seller", "buyer", 2)],
             {"released": ("UInt", 0), "open": ("Bool", True), "rounds": ("UInt", 7)}),
        "positive boundary: released==1000000 with last=true passes <= ; zero fee effect still emitted; accrual onto existing duty 2+40; open becomes false",
        lambda r: (r["accepted"] or (_ for _ in ()).throw(AssertionError())) and None)
    run("H1-R1", "H1-settle", H1, "settle",
        scen({"gross": ("UInt", 10), "cut": ("UInt", 10), "hold": ("UInt", 0), "last": ("Bool", False)}, base_bal),
        "reject: requires gross > cut false at step 0 (strict predicate)", category="rejection")
    run("H1-R2", "H1-settle", H1, "settle",
        scen({"gross": ("UInt", 1_000_002), "cut": ("UInt", 1), "hold": ("UInt", 0), "last": ("Bool", False)}, base_bal),
        "reject: buyer USDX insufficient for net at transfer step 2 (no field wrap)", category="rejection")
    run("H1-R3", "H1-settle", H1, "settle",
        scen({"gross": ("UInt", 100), "cut": ("UInt", 1), "hold": ("UInt", 0), "last": ("Bool", False)},
             [bal("buyer", USDX, 1000), bal("seller", USDX, 0), bal("operator", USDX, 0), bal("seller", PTS, 4)]),
        "reject: SELF-transfer of 5 PTS with balance 4 is still debit-checked (step 4)", category="rejection")
    run("H1-R4", "H1-settle", H1, "settle",
        scen({"gross": ("UInt", 100), "cut": ("UInt", 1), "hold": ("UInt", 0), "last": ("Bool", False)},
             [bal("buyer", USDX, 1000), bal("seller", USDX, U - 50), bal("operator", USDX, 0), bal("seller", PTS, 5)]),
        "reject: recipient credit overflow past u128 at step 2", category="rejection")
    run("H1-R5", "H1-settle", H1, "settle",
        scen({"gross": ("UInt", 2), "cut": ("UInt", 1), "hold": ("UInt", 0), "last": ("Bool", True)}, base_bal, [],
             {"released": ("UInt", 1_000_000), "open": ("Bool", True), "rounds": ("UInt", 0)}),
        "reject: ensures reads UPDATED released (1000001) after Next; fails at step 8 after effects (atomic: no partial effects)",
        category="rejection")
    run("H1-R6", "H1-settle", H1, "settle",
        scen({"gross": ("UInt", 2), "cut": ("UInt", 1), "hold": ("UInt", 0), "last": ("Bool", False)}, base_bal,
             [liab("warranty", USDX, "mallory", "buyer", 3)]),
        "reject at admission: scenario duty identity differs from source-declared identity", category="rejection")
    run("H1-R7", "H1-settle", H1, "settle",
        scen({"gross": ("UInt", 2), "cut": ("UInt", 1), "hold": ("UInt", 0), "last": ("Bool", False)},
             [b for b in base_bal if b["account"] != "operator"]),
        "reject at admission: missing explicit operator USDX balance row (even though amounts are tiny)", category="rejection")
    run("H1-R8", "H1-settle", H1, "settle",
        scen({"gross": ("UInt", 2), "cut": ("UInt", 1), "hold": ("UInt", 0), "last": ("Bool", False)}, base_bal, [],
             {"released": ("UInt", 0), "open": ("Bool", False), "rounds": ("UInt", 0)}),
        "reject: open=false fails step 0", category="rejection")
    run("H1-R9", "H1-settle", H1, "settle",
        scen({"gross": ("UInt", 2), "cut": ("UInt", 1), "hold": ("UInt", U), "last": ("Bool", False)}, base_bal,
             [liab("warranty", USDX, "seller", "buyer", 1)]),
        "reject: liability accrual 1 + (2^128-1) overflows at step 5", category="rejection")
    close_bal = [bal("buyer", USDX, 0), bal("seller", USDX, 500)]
    run("H1-P3", "H1-settle", H1, "close",
        scen({"pay": ("UInt", 30), "left": ("UInt", 0)}, close_bal, [liab("warranty", USDX, "seller", "buyer", 30)]),
        "positive present-zero: full discharge leaves row present with amount 0; liability_present stays true")
    run("H1-P4", "H1-settle", H1, "close",
        scen({"pay": ("UInt", 0), "left": ("UInt", 0)}, close_bal, [liab("warranty", USDX, "seller", "buyer", 0)]),
        "positive: zero transfer + zero discharge on a present-zero row")
    run("H1-P5", "H1-settle", H1, "close",
        scen({"pay": ("UInt", 12), "left": ("UInt", 18)}, close_bal, [liab("warranty", USDX, "seller", "buyer", 30)]),
        "positive partial discharge: residual 18 remains a duty")
    run("H1-R10", "H1-settle", H1, "close",
        scen({"pay": ("UInt", 31), "left": ("UInt", 0)}, close_bal, [liab("warranty", USDX, "seller", "buyer", 30)]),
        "reject: discharge exceeds residual at step 2 (after a successful transfer: atomic)", category="rejection")
    run("H1-R11", "H1-settle", H1, "close",
        scen({"pay": ("UInt", 0), "left": ("UInt", 0)}, close_bal, []),
        "reject: absent duty => liability_present false at step 0 (absent != present-zero)", category="rejection")
    run("H1-R12", "H1-settle", H1, "close",
        scen({"pay": ("UInt", 12), "left": ("UInt", 17)}, close_bal, [liab("warranty", USDX, "seller", "buyer", 30)]),
        "reject: claimed residual mismatch at step 3", category="rejection")

    # ---------------- H2 accrual / repay (contrasting structure)
    H2 = progs["H2-accrual"]
    run("H2-P1", "H2-accrual", H2, "accrue",
        scen({"rate": ("UInt", 125), "units": ("UInt", 400)}, [], [liab("loan", CRD, "borrower", "lender", 0)]),
        "positive: Mul, accrual onto present-zero row to exactly 50000 (boundary <=)")
    run("H2-P2", "H2-accrual", H2, "accrue",
        scen({"rate": ("UInt", 3), "units": ("UInt", 7)}, []),
        "positive: duty created from absent; no balances required (no transfer statements)")
    run("H2-R1", "H2-accrual", H2, "accrue",
        scen({"rate": ("UInt", 125), "units": ("UInt", 401)}, []),
        "reject: liability_amount 50125 > 50000 at step 3 after duty effect", category="rejection")
    run("H2-R2", "H2-accrual", H2, "accrue",
        scen({"rate": ("UInt", P64), "units": ("UInt", P64)}, []),
        "reject: 2^64*2^64 = 2^128 overflows at step 1 (no 256-bit truncation)", category="rejection")
    run("H2-R3", "H2-accrual", H2, "accrue",
        scen({"rate": ("UInt", 1), "units": ("UInt", 1)}, [], [], {"periods": ("UInt", 0), "frozen": ("Bool", True)}),
        "reject: frozen", category="rejection")
    rb = [bal("borrower", CRD, 1000), bal("lender", CRD, 0)]
    run("H2-P3", "H2-accrual", H2, "repay",
        scen({"pay": ("UInt", 600), "stop": ("Bool", True)}, rb, [liab("loan", CRD, "borrower", "lender", 600)]),
        "positive: full repayment, ensures passes ONLY via present-zero (amount 0, present true); self-fee 1 debit-checked, net zero")
    run("H2-P4", "H2-accrual", H2, "repay",
        scen({"pay": ("UInt", 999), "stop": ("Bool", False)}, rb, [liab("loan", CRD, "borrower", "lender", 5000)]),
        "positive boundary: after paying 999 borrower has exactly 1 for the self-fee")
    run("H2-R4", "H2-accrual", H2, "repay",
        scen({"pay": ("UInt", 1000), "stop": ("Bool", False)}, rb, [liab("loan", CRD, "borrower", "lender", 5000)]),
        "reject: self-fee of 1 with balance 0 at step 4 (self move still debit-checked)", category="rejection")
    run("H2-R5", "H2-accrual", H2, "repay",
        scen({"pay": ("UInt", 0), "stop": ("Bool", False)}, rb, []),
        "reject: absent loan", category="rejection")

    # ---------------- H5 strict evaluation (no short circuit)
    H5 = progs["H5-strict"]
    run("H5-R1", "H5-strict", H5, "probeOr",
        scen({"a": ("UInt", 1), "b": ("UInt", 2), "skip": ("Bool", True)}),
        "HOSTILE strictness: lhs of 'or' is true but rhs a-b underflows => Core REJECTS (underflow, step 0). A short-circuit lowering would accept: unsound",
        category="strictness")
    run("H5-P1", "H5-strict", H5, "probeOr",
        scen({"a": ("UInt", 2), "b": ("UInt", 2), "skip": ("Bool", False)}), "positive: a-b=0 >= 0", category="strictness")
    run("H5-R2", "H5-strict", H5, "probeAnd",
        scen({"a": ("UInt", P64), "b": ("UInt", P64), "run": ("Bool", False)}),
        "HOSTILE strictness: lhs of 'and' false but a*b overflows => Core REJECTS (overflow, step 0)", category="strictness")
    run("H5-P2", "H5-strict", H5, "probeAnd",
        scen({"a": ("UInt", P64 - 1), "b": ("UInt", P64 + 1), "run": ("Bool", False)}),
        "positive: (2^64-1)(2^64+1) = 2^128-1 fits; predicate true via not(false and ..)", category="strictness")

    # ---------------- H6 arithmetic and comparison bounds
    H6 = progs["H6-bounds"]
    for vid, act, x, y, note in [
        ("H6-A1", "add", U - 1, 1, "max sum 2^128-1 accepted"),
        ("H6-A2", "add", U, 1, "overflow by one rejected"),
        ("H6-A3", "add", U, U, "double max rejected (no mod 2^128 or field wrap)"),
        ("H6-S1", "sub", 5, 5, "zero result accepted"),
        ("H6-S2", "sub", 0, 1, "underflow rejected (no wrap to p-1 or 2^128-1)"),
        ("H6-S3", "sub", U, 0, "max - 0 accepted"),
        ("H6-M1", "mul", P64, P64, "2^128 rejected"),
        ("H6-M2", "mul", P64 - 1, P64 + 1, "2^128-1 accepted"),
        ("H6-M3", "mul", 0, U, "0 * max accepted"),
        ("H6-M4", "mul", 1 << 127, 2, "2^128 rejected"),
        ("H6-M5", "mul", U, U, "(2^128-1)^2 rejected; 256-bit product high limb nonzero"),
        ("H6-M6", "mul", (1 << 127) + 1, 1, "high-bit operand accepted"),
        ("H6-C1", "cmp", U - 1, U, "adjacent max accepted; flag true"),
        ("H6-C2", "cmp", U, U, "equal max rejected at step 0"),
        ("H6-C3", "cmp", 0, 1 << 127, "accepted; flag false"),
        ("H6-C4", "cmp", U, 0, "reversed rejected at step 0"),
        ("H6-C5", "cmp", U - 1, U - 1, "equal rejected"),
    ]:
        run(vid, "H6-bounds", H6, act, scen({"x": ("UInt", x), "y": ("UInt", y)}), note, category="arithmetic")
    # cmp C1: x+1 == y with x=U-1 -> U fits: flag true.  If x were U, x+1 would overflow; C2 already fails earlier.
    for vid, p, q in [("H6-B1", True, True), ("H6-B2", True, False), ("H6-B3", False, False), ("H6-B4", False, True)]:
        run(vid, "H6-bounds", H6, "beq", scen({"p": ("Bool", p), "q": ("Bool", q)}),
            "Bool Eq/Ne on Bool operands; next updates flag then requires reads updated flag", category="arithmetic")

    # ---------------- H3 DAG / shared nodes / dead nodes / polymorphic node
    run("H3-P1", "H3-dag", H3, "bump", scen({}, [], [], {"s": ("UInt", 4)}),
        "shared node 2 in two Next statements: s 4->5->6 (re-evaluated per statement); node 3=(s+1)^2 with child shared; "
        "sq = 2*49 = 98; work = 4 stmts + 3 + 3 + (19,3,2,0,1 =5) + (18,17,6 =3) = 18; dead nodes 7,8 never evaluated",
        lambda r: (r["accepted"] and r["post"]["state"]["s"] == ("UInt", 6) and r["work_used"] == 18) or (_ for _ in ()).throw(AssertionError(r)),
        category="dag")
    run("H3-P2", "H3-dag", H3, "pay",
        scen({"pay": ("UInt", 4), "before": ("UInt", 10), "after": ("UInt", 6)},
             [bal("x", TOK, 4), bal("y", TOK, 0)], [liab("d", TOK, "x", "y", 10)]),
        "same LiabilityAmount node 4 read before (10) and after (6) discharge within one action", category="dag")
    run("H3-R1", "H3-dag", H3, "pay",
        scen({"pay": ("UInt", 4), "before": ("UInt", 10), "after": ("UInt", 10)},
             [bal("x", TOK, 4), bal("y", TOK, 0)], [liab("d", TOK, "x", "y", 10)]),
        "HOSTILE: witness reusing the first evaluation of shared node 4 after discharge must be rejected (step 4)", category="dag")
    run("H3-P3", "H3-dag", H3, "poly_u", scen({"v": ("UInt", 3)}, [bal("x", TOK, 3), bal("y", TOK, 0)]),
        "node 9/10 typed UInt in this action", category="dag")
    run("H3-P4", "H3-dag", H3, "poly_b", scen({"v": ("Bool", True)}),
        "same nodes 9/10 typed Bool in this action", category="dag")
    run("H3-R2", "H3-dag", H3, "poly_b", scen({"v": ("Bool", False)}),
        "reject at step 1 (v false)", category="dag")
    run("H3-P5", "H3-dag", H3, "open", scen({}, [], []), "create d=10 from absent", category="dag")

    # ---------------- H4 Emit-bearing (Core semantics; compiler may refuse whole program per adopted target policy)
    run("H4-P1", "H4-emit", progs["H4-emit"], "tick", scen({"k": ("UInt", 11)}, [], [], {"n": ("UInt", 0)}),
        "Core outcome with ordered Emit effects (before=0, State, after=11, flag=true). Compiler: either constrains all three Emit effects in order "
        "or refuses the WHOLE program (adopted TargetEmit policy); never silently drops Emit", category="emit")

    # ---------------- work-cap vectors (logical work schedule)
    run("W-1", "H1-settle", H1, "settle",
        scen({"gross": ("UInt", 250_000), "cut": ("UInt", 1_250), "hold": ("UInt", 0), "last": ("Bool", False)}, base_bal, [legacy]),
        "work cap exactly 36 accepted", cap=36, category="work")
    run("W-2", "H1-settle", H1, "settle",
        scen({"gross": ("UInt", 250_000), "cut": ("UInt", 1_250), "hold": ("UInt", 0), "last": ("Bool", False)}, base_bal, [legacy]),
        "work cap 35 rejected (meter) at last statement", cap=35, category="work")

    # ---------------- static refusals (no Core execution; expectations are admission decisions)
    statics = [
        {"id": "S-1", "input": "H7-staged.mori", "expected": "refuse: moriarty/2 source (profile) before any constraint emission; no /1 lowering of an Await program; staged /2 obligations remain open"},
        {"id": "S-2", "input": "in-memory Program labelled moriarty/1 + moriarty-core/1 carrying Statement::Await", "expected": "refuse with mori::core::profile (require_core1) — never strip the Await"},
        {"id": "S-3", "input": "H3 JSON with semantics 'moriarty-core/2' but language 'moriarty/1'", "expected": "refuse profile"},
        {"id": "S-4", "input": "H4-emit.mori", "expected": "either complete ordered Emit lowering or whole-program refusal naming Emit; partial lowering that omits Emit effects is a defect"},
        {"id": "S-5", "input": "H3 JSON with an expression chain of depth 65", "expected": "refuse (Core bound) — compiler must not accept deeper IR than Core validate"},
        {"id": "S-6", "input": "H3 JSON where node 2 references node 2 (self/forward ref)", "expected": "refuse (Core expression ordering)"},
        {"id": "S-7", "input": "H3 JSON (dead node 7 underflow, dead node 8 unknown name)", "expected": "ADMIT (Core validate accepts); dead nodes produce no constraints that could make valid executions unsatisfiable"},
        {"id": "S-8", "input": "H1 with asset USDX scale 39", "expected": "refuse (Core asset rule)"},
        {"id": "S-9", "input": "Program whose layout capacity (balances/duties/states) exceeds compiler caps", "expected": "explicit refusal naming the cap; never truncate rows from the frame"},
    ]

    # ---------------- constraint-level hostile witness / mutation proposals (to be executed later by an authorized runner)
    hostile = [
        ("HW-01", "H6-S2", "assign acc_after = p-1 (field negation of 1) with all limb/range witnesses chosen adversarially", "unsatisfiable"),
        ("HW-02", "H6-M1", "assign product = 0 (2^128 mod 2^128) with high limb hidden", "unsatisfiable"),
        ("HW-03", "H6-A2", "assign sum = 0 with carry bit = 0 or carry bit = 2", "unsatisfiable"),
        ("HW-04", "H6-C2", "flip comparison flag for x<y to 1 while x==y; or supply non-boolean flag value 2", "unsatisfiable"),
        ("HW-05", "H6-*", "input x = true_x + r (scalar field modulus) or x >= 2^128 encoded in one field element", "unsatisfiable (public/private input range checks)"),
        ("HW-06", "H5-R1", "witness that skips evaluation of the right operand of 'or' (rhs wires unconstrained/garbage)", "unsatisfiable: rhs Sub must be constrained non-negative regardless of lhs"),
        ("HW-07", "H5-R2", "witness that skips evaluation of the right operand of 'and'", "unsatisfiable"),
        ("HW-08", "H1-R3", "self-transfer with balance 4, amount 5: witness claiming success since net delta is zero", "unsatisfiable"),
        ("HW-09", "H1-P1", "modify untouched frame row auditor USDX 7->8 or legacy duty 9->0 in post", "unsatisfiable (frame preservation)"),
        ("HW-10", "H1-P1", "omit the warranty present-zero row in post (treat 0 as absent)", "unsatisfiable"),
        ("HW-11", "H1-P3", "after full discharge set present flag false", "unsatisfiable"),
        ("HW-12", "H1-P1", "swap order of Transfer and Fee effects in the public effect list", "unsatisfiable / different public input"),
        ("HW-13", "H1-P1", "change fee recipient operator->seller with identical amounts", "unsatisfiable under the same compiled program"),
        ("HW-14", "H1-P1", "prove with program H1' that differs only in asset reference 'usdx'->'usdy' under H1 key/VK or program-identity public input", "reject"),
        ("HW-15", "H1-P1", "select action 'close' witness under the 'settle' circuit / action id", "reject"),
        ("HW-16", "H3-R1", "reuse a single wire for node 4 across statements", "unsatisfiable"),
        ("HW-17", "H3-P1", "use single wire for node 0 (s) across both Next statements so s_after=5", "unsatisfiable (Next visible to later statements)"),
        ("HW-18", "W-2", "claim work_used 35 / omit charge of one statement or one DAG node", "unsatisfiable if work is a constrained output; otherwise state work is unconstrained"),
        ("HW-19", "H1-R5", "success witness when ensures false after effects", "unsatisfiable"),
        ("HW-20", "any", "set any host-computed 'ok'/'accepted' Boolean wire to 1 with failing constraints", "no such success flag may gate constraints"),
        ("HW-21", "H1-R6", "scenario duty with mismatched debtor accepted by circuit", "unsatisfiable or refused before proving"),
        ("HW-22", "H2-P3", "post duty amount 0 but present flag 0 (deleted) to pass absent-checks", "unsatisfiable"),
        ("HW-23", "H6-B*", "Bool wire value 2 or p-1 used as true", "unsatisfiable (booleanity)"),
        ("HW-24", "H3-P4", "in poly_b, treat v as UInt with value 2 (non-Bool) to satisfy Eq(v,v)", "unsatisfiable"),
        ("HW-25", "H1-P1", "duplicate balance key (two buyer/USDX rows) in pre-state layout to split a debit", "unsatisfiable / refused"),
        ("HW-26", "H1-P1", "unsorted or padding row with nonzero amount in layout slots", "unsatisfiable"),
    ]

    # ---------------- write files
    srcdir = os.path.join(OUT, "programs")
    os.makedirs(srcdir, exist_ok=True)
    for k, v in SRC.items():
        with open(os.path.join(srcdir, k + ".mori"), "w") as f:
            f.write(v)
    with open(os.path.join(srcdir, "H3-dag.program.json"), "w") as f:
        json.dump(wire_program(H3), f, indent=2, sort_keys=True)
        f.write("\n")
    for k in ("H1-settle", "H2-accrual", "H5-strict", "H6-bounds", "H4-emit"):
        with open(os.path.join(srcdir, k + ".reviewer-parse.program.json"), "w") as f:
            json.dump(wire_program(progs[k]), f, indent=2, sort_keys=True)
            f.write("\n")
    doc = {"format": "reviewer-heldout-vectors/1",
           "author": "claude-opus-5-5 (independent non-author reviewer)",
           "derivation": "tools/core_oracle.py: reviewer reimplementation of Core /1 parser/validate/simulate/meter rules from source reading; inline hand-arithmetic asserts in tools/make_vectors.py",
           "semantics": "moriarty-core/1", "logical_cost_profile": "moriarty-reference-logical-cost/1",
           "frozen_before_reading": ["candidate tests/*.rs", "candidate EXPECTED-PACKET/*", "candidate fixtures/*", "candidate REPORT.md", "candidate source/*"],
           "limitations": ["Not executed against Core or the candidate (SOURCEONLY).",
                           "Reviewer oracle is a second implementation of the same written rules, not a formal proof; disagreement with Core must be resolved against actual Core execution in an authorized run.",
                           "Accounting fields (gross_debit, fees_paid, net_delta, duties) are reviewer-defined projections of Core effects."],
           "vectors": VECTORS, "static_refusals": statics,
           "hostile_witness_proposals": [{"id": a, "base_vector": b, "mutation": c, "required": d} for a, b, c, d in hostile]}
    with open(os.path.join(OUT, "HELDOUT-VECTORS.json"), "w") as f:
        json.dump(doc, f, indent=2, sort_keys=True)
        f.write("\n")
    print(len(VECTORS), "vectors;", sum(1 for v in VECTORS if v["expected"]["accepted"]), "accepted")


main()
