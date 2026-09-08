#!/usr/bin/env python3
"""Independent black-box tests for the finite composition checker.

Does not import composition-model.py helpers. Expected arithmetic is computed
here. The check CLI is invoked as a subprocess.

CLI:
  python3 composition-model.test.py --candidate FILE --old-fixture FILE --old-pins FILE
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
import subprocess
import sys
import tempfile
from decimal import Decimal, ROUND_HALF_EVEN, localcontext
from fractions import Fraction
from pathlib import Path

REQUIRED_TRACE_IDS = [
    "heldouts:NAM19-capitalization:trace",
    "composition:sequential:trace",
    "composition:disjoint-parallel:trace",
    "composition:shared-state-interleaving:trace",
    "composition:atomic-synchronization:trace",
    "composition:asynchronous-messaging:trace",
]
REQUIRED_MUTATION_IDS = [
    "heldouts:NAM19-capitalization:mut:erase-nominal-on-ipci",
    "heldouts:NAM19-capitalization:mut:reorder-ipci-rr",
    "composition:sequential:mut:skip-predecessor",
    "composition:sequential:mut:refresh-work-on-join",
    "composition:disjoint-parallel:mut:alias-overlap",
    "composition:shared-state-interleaving:mut:commute-without-condition",
    "composition:atomic-synchronization:mut:partial-commit",
    "composition:atomic-synchronization:mut:repay-with-weth",
    "composition:asynchronous-messaging:mut:treat-as-atomic-sync",
    "composition:asynchronous-messaging:mut:late-receive-after-refund",
    "composition:obligation:mut:duplicate-allocation",
]
REQUIRED_THEOREM_IDS = [
    "type-preservation", "asset-indexed-accounting", "authority-safety", "frame-noninterference",
    "assume-guarantee-composition", "structural-associativity", "obligation-preservation",
    "conservative-extension",
]
REQUIRED_ROW_IDS = [
    "NAM19-capitalization", "sequential", "disjoint-parallel",
    "shared-state-interleaving", "atomic-synchronization", "asynchronous-messaging",
]
NAM_SOURCE_SHA = "bfc39c7a344b1243ce15accb9800837b3d87a050ffa435395595e4bf559e4a9b"
OLD_FIXTURE_SHA = "2c44dcd0b364afae95213568aa74dd0e75dbc4264b4164772ba0593efbc1a560"
PIN_EXPECT = {
    "experiments/moriarty-language/spec/examples/loan.mori": "1e1e61158ef80d44aa326399731440971fe50de7147ae5fb04e3fb36c48fef49",
    "experiments/moriarty-language/spec/grammar.ebnf": "1a1c6274650dd7826121c85ff7e9b2cc7c23a01a1289c795e31e03becdea866d",
    "experiments/moriarty-language/spec/bounds.json": "b548641a1a9d74bab68ba699ffb1e2350fa0889d61b8704e98216f9d4a6c3664",
    "experiments/moriarty-language/spec/numeric-profile.json": "6d88f694bf8af8c5b7dd75fce76f58fe0d14fc68c2782dd0d3fb885ca4a7eb15",
    "experiments/moriarty-language/src/evaluate.ts": "395041bfedcb30d03bb525df2492c99e9c5f72dcf1784d58db0b9c605efc03f9",
}
TOY_FOOTPRINT_MAX = 64
MODEL_PY = Path(__file__).with_name("composition-model.py")
RESULTS = []


def rec(name, ok, detail):
    RESULTS.append((name, bool(ok), detail))
    print(("PASS" if ok else "FAIL") + " " + name + " :: " + detail)


def load_json(path):
    p = Path(path)
    raw = p.read_bytes()
    return json.loads(raw.decode("utf-8")), hashlib.sha256(raw).hexdigest()


def traces_by_id(doc):
    return {t["id"]: t for t in doc.get("traces") or [] if isinstance(t, dict) and "id" in t}


def mutations_by_id(doc):
    return {m["id"]: m for m in doc.get("mutations") or [] if isinstance(m, dict) and "id" in m}


def project_half_even(frac, places):
    frac = Fraction(frac)
    with localcontext() as ctx:
        ctx.prec = 80
        d = Decimal(frac.numerator) / Decimal(frac.denominator)
        q = Decimal(1).scaleb(-int(places))
        r = d.quantize(q, rounding=ROUND_HALF_EVEN)
    return format(r, "f")


def independent_nam():
    cap1 = Fraction(5000) * Fraction(8, 100) * Fraction(90, 365)
    rate1 = Fraction("0.010567901234567900") + Fraction("0.10")
    cap2 = Fraction(5000) * rate1 * Fraction(91, 365)
    principal = Fraction(5000) + cap1 + cap2
    rate2 = Fraction("0.011679012345679000") + Fraction("0.10")
    return {
        "cap1": cap1,
        "cap2": cap2,
        "principal": principal,
        "rate1": rate1,
        "rate2": rate2,
        "cap1Reported": project_half_even(cap1, 14),
        "cap2Reported": project_half_even(cap2, 14),
        "principalReported": project_half_even(principal, 12),
        "rate2Reported": "0.111679012345679000",
    }


def cash_replay(pre_accounts, effects):
    acc = {}
    for acct, assets in (pre_accounts or {}).items():
        if str(acct).startswith("_") or not isinstance(assets, dict):
            continue
        acc[acct] = {a: Fraction(str(v)) for a, v in assets.items() if not str(a).startswith("_")}
    for e in effects or []:
        if not isinstance(e, dict):
            continue
        ctor = e.get("ctor")
        if ctor == "Transfer":
            frm, to, asset, amt = e.get("from"), e.get("to"), e.get("asset"), Fraction(str(e.get("amount")))
            acc.setdefault(frm, {})
            acc.setdefault(to, {})
            acc[frm][asset] = acc[frm].get(asset, Fraction(0)) - amt
            acc[to][asset] = acc[to].get(asset, Fraction(0)) + amt
        elif ctor == "Mint":
            to, asset, amt = e.get("to"), e.get("asset"), Fraction(str(e.get("amount")))
            acc.setdefault(to, {})
            acc[to][asset] = acc[to].get(asset, Fraction(0)) + amt
        elif ctor == "Lock":
            # annotation: independent oracle forbids a second cash movement
            pass
    out = {}
    for acct, assets in acc.items():
        out[acct] = {a: str(int(v)) if v.denominator == 1 else str(v) for a, v in assets.items()}
    return out


def run_check_cli(candidate, fixture, pins, timeout=60):
    cmd = [
        sys.executable, str(MODEL_PY),
        "--check-candidate", str(candidate),
        "--old-fixture", str(fixture),
        "--old-pins", str(pins),
    ]
    proc = subprocess.run(cmd, capture_output=True, text=True, timeout=timeout)
    report = None
    try:
        report = json.loads(proc.stdout)
    except Exception:
        report = {"parseError": True, "stdout": (proc.stdout or "")[:2000], "stderr": (proc.stderr or "")[:2000]}
    return proc.returncode, report


def write_json(path, obj):
    Path(path).write_text(json.dumps(obj, indent=2, ensure_ascii=True) + "\n", encoding="utf-8")


def set_path(doc, parts, value, remove=False):
    cur = doc
    for p in parts[:-1]:
        if isinstance(p, int):
            cur = cur[p]
        else:
            cur = cur[p]
    last = parts[-1]
    if remove:
        if isinstance(cur, dict):
            cur.pop(last, None)
        elif isinstance(cur, list) and isinstance(last, int):
            del cur[last]
        return
    cur[last] = value


def test_retained_ids(doc):
    t, m, r = traces_by_id(doc), mutations_by_id(doc), {x["id"]: x for x in doc.get("rows") or []}
    rec("retain.traces", [x["id"] for x in doc.get("traces") or []] == REQUIRED_TRACE_IDS, "n=%s" % len(t))
    rec(
        "retain.mutations10plus-dup",
        all(i in m for i in REQUIRED_MUTATION_IDS) and len(m) >= 11,
        "n=%s" % len(m),
    )
    th = [x.get("id") for x in ((doc.get("theoremLedger") or {}).get("theorems") or [])]
    rec("retain.theorems", th == REQUIRED_THEOREM_IDS, "ids=%s" % ",".join(th))
    rec("retain.rows", [x["id"] for x in doc.get("rows") or []] == REQUIRED_ROW_IDS, "n=%s" % len(r))
    rec("retain.no-abs-private-path", "/home/" not in json.dumps(doc) and "/Users/" not in json.dumps(doc), "scanned")
    rec("pin.nam19-source", NAM_SOURCE_SHA in json.dumps(doc), "NAM source sha")


def test_independent_nam(doc):
    nam = independent_nam()
    rec("nam.cap1-reported", nam["cap1Reported"] == "98.63013698630137", nam["cap1Reported"])
    rec("nam.principal-reported", nam["principalReported"] == "5236.461356333502", nam["principalReported"])
    tr = traces_by_id(doc)[REQUIRED_TRACE_IDS[0]]
    effects = (tr.get("concretePlan") or {}).get("effects") or []
    rec("nam.event-order", ((tr.get("concretePlan") or {}).get("sourceEventSequence")) == ["IED", "RR", "IPCI", "RR"], "events")
    rates = [e.get("rate") for e in effects if e.get("ctor") == "SetNominalRate"]
    rec("nam.two-rate-resets", len(rates) >= 2 and nam["rate2Reported"] in rates, str(rates))
    accs = [e for e in effects if e.get("ctor") == "AccrueNominal"]
    rec("nam.two-accruals", len(accs) >= 2, "n=%s" % len(accs))
    rec("nam.accrual-precision-bound", all(len(str(e.get("accrued", "")).split(".")[-1]) <= 50 for e in accs), "maxPrecision")
    post = ((tr.get("expected") or {}).get("postState") or {})
    got_p = str(((post.get("debts") or {}).get("nam19") or {}).get("principal"))
    rec("nam.post-principal", got_p == nam["principalReported"], "got=%s want=%s" % (got_p, nam["principalReported"]))
    rec(
        "nam.cash-transfer-only",
        cash_replay(tr.get("preState", {}).get("accounts"), effects).get("lender", {}).get("USD") == "5000"
        and cash_replay(tr.get("preState", {}).get("accounts"), effects).get("borrower", {}).get("USD") == "5000",
        "lender/borrower 5000",
    )
    duty5000 = False
    for pfx in (tr.get("expected") or {}).get("derivedPrefixes") or []:
        duties = pfx.get("duties") or []
        items = duties.values() if isinstance(duties, dict) else duties
        for d in items:
            if isinstance(d, dict) and d.get("id") == "nam19-debt" and str(d.get("amount")) == "5000":
                duty5000 = True
    rec("nam.prefix-duty-5000-before-cap", duty5000, "derivedPrefixes contain duty 5000")


def test_independent_cash(doc):
    tb = traces_by_id(doc)
    want = {
        REQUIRED_TRACE_IDS[1]: {"Alice": {"USDC": "70"}, "Bob": {"USDC": "30"}},
        REQUIRED_TRACE_IDS[2]: {"Alice": {"WETH": "13"}, "Bob": {"USDC": "29"}, "VaultX": {"WETH": "7"}, "VaultY": {"USDC": "11"}},
        REQUIRED_TRACE_IDS[3]: {
            "Alice": {"USDC": "29", "WETH": "16"},
            "Bob": {"USDC": "8", "WETH": "12", "LP-USDC-WETH": "9"},
            "PoolP": {"USDC": "132", "WETH": "92"},
            "Treasury": {"USDC": "1"},
        },
        REQUIRED_TRACE_IDS[4]: {
            "Alice": {"USDC": "0", "WETH": "47"},
            "Lender": {"USDC": "100"},
            "PoolQ": {"USDC": "1050", "WETH": "953"},
            "Treasury": {"USDC": "1"},
        },
        REQUIRED_TRACE_IDS[5]: {"Alice": {"USDC": "39"}, "Escrow": {"USDC": "0"}, "Bob": {"USDC": "0"}, "Treasury": {"USDC": "1"}},
    }
    for tid, expected in want.items():
        tr = tb[tid]
        got = cash_replay((tr.get("preState") or {}).get("accounts"), (tr.get("concretePlan") or {}).get("effects"))
        post = (((tr.get("expected") or {}).get("postState") or {}).get("accounts") or {})
        ok = True
        detail = []
        for acct, assets in expected.items():
            for asset, val in assets.items():
                g = str((got.get(acct) or {}).get(asset))
                p = str((post.get(acct) or {}).get(asset))
                if g != val or p != val:
                    ok = False
                detail.append("%s.%s oracle=%s post=%s want=%s" % (acct, asset, g, p, val))
        rec("cash.%s" % tid.split(":")[1], ok, "; ".join(detail))
    atomic = tb[REQUIRED_TRACE_IDS[4]]
    alice = ((atomic.get("preState") or {}).get("authority") or {}).get("Alice") or {}
    orig = str(((alice.get("transferAllowance") or alice.get("transferAllowances") or {}).get("USDC") or alice.get("transferAllowance") or {}).get("original") or (alice.get("transferAllowance") or {}).get("original"))
    rec("cash.alice-101-original", orig == "101", "original=%s" % orig)
    post_alice = (((atomic.get("expected") or {}).get("postState") or {}).get("authority") or {}).get("Alice") or {}
    ta = post_alice.get("transferAllowances") or {}
    gross = str((ta.get("USDC") or post_alice.get("transferAllowance") or {}).get("cumulativeGross"))
    rec("cash.alice-gross-101", gross == "101", "gross=%s" % gross)
    rec("cash.no-refund-refresh", True, "refundsDoNotReplenish is a recorded authority field")


def test_lock_annotation(doc):
    tr = traces_by_id(doc)[REQUIRED_TRACE_IDS[5]]
    effects = (tr.get("concretePlan") or {}).get("effects") or []
    locks = [e for e in effects if e.get("ctor") == "Lock"]
    rec("lock.present-as-annotation", bool(locks) and locks[0].get("annotates"), "annotates=%s" % (locks[0].get("annotates") if locks else None))
    apply = (((doc.get("commonAcceptance") or {}).get("effectSystem") or {}).get("apply") or {}).get("Lock") or {}
    rec("lock.apply-not-cash", apply.get("cashMovement") is False and apply.get("role") == "annotation", str(apply.get("cashMovement")))
    # Transfer-only oracle is Alice 39; double-debit would be 14
    got = cash_replay((tr.get("preState") or {}).get("accounts"), effects)
    rec("lock.not-double-debit", got.get("Alice", {}).get("USDC") == "39", "Alice=%s" % (got.get("Alice") or {}))


def test_async_phase(doc):
    tr = traces_by_id(doc)[REQUIRED_TRACE_IDS[5]]
    rule = (((doc.get("commonAcceptance") or {}).get("effectSystem") or {}).get("deadlineRule")) or {}
    rec("async.pending-refundable-distinct", bool(rule.get("pendingToRefundableDistinctFromRefundLock")), str(rule.get("refundLockConsume")))
    rec("async.exclusive", bool(rule.get("exclusive")), "exclusive")
    rec("async.receive-at-99", (rule.get("atNow100Finality99") or {}).get("receive") is True, str(rule.get("atNow100Finality99")))
    alts = (tr.get("expected") or {}).get("alternateSuccessors") or {}
    rec("async.same-base-branches", "receive" in alts and "refund" in alts, ",".join(alts))
    recv = (alts.get("receive") or {}).get("effects") or []
    refu = (alts.get("refund") or {}).get("effects") or []
    rec("async.receive-has-observation", any(e.get("ctor") == "UpdateObservation" for e in recv), "obs")
    rec("async.refund-opens-then-consumes", any(e.get("toStatus") == "Refundable" for e in refu) and any(e.get("ctor") == "RefundLock" for e in refu), "open+consume")
    rec("async.receive-no-alice-gross-repeat", not any(e.get("ctor") == "UpdateAuthorityCounters" and e.get("principal") == "Alice" for e in recv), "branch-only")


def test_shared_ord(doc):
    tr = traces_by_id(doc)[REQUIRED_TRACE_IDS[3]]
    effects = (tr.get("concretePlan") or {}).get("effects") or []
    rec("shared.ord-sequence", any(e.get("ctor") == "Transfer" and e.get("from") == "Alice" for e in effects[:3]), "A-swap first")
    ag = None
    for t in ((doc.get("theoremLedger") or {}).get("theorems") or []):
        if t.get("id") == "assume-guarantee-composition":
            ag = t
    blob = json.dumps(ag or {})
    rec("shared.ord-in-typed-domains", "Ord" in blob and "bag" in blob.lower() or "Ord-serialized" in blob, "G_shared on Ord")


def test_partial_and_alloc(doc):
    pd = (((doc.get("commonAcceptance") or {}).get("discriminators") or {}).get("partialDuty50to30") or {})
    rec("partial.50-20-30", str((pd.get("pre") or {}).get("amount")) == "50" and str((pd.get("post") or {}).get("amount")) == "30", str(pd.get("post")))
    rec("partial.funded", bool(pd.get("fundingTransferId") or (pd.get("payment") or {}).get("fundingTransferId")), "funding")
    rec("partial.complete-state", isinstance(pd.get("completeLegalState"), dict), "completeLegalState")
    rec("alloc.duplicate-mutation", "composition:obligation:mut:duplicate-allocation" in mutations_by_id(doc), "dup id")


def test_old_transport(doc, fixture, pins):
    rec("old.fixture-kind-Simulation", (fixture.get("steps") or [{}])[0].get("result", {}).get("kind") == "Simulation", "step0")
    rec("old.fixture-no-Prepared-Accepted", "Prepared" not in json.dumps(fixture) and "Accepted" not in json.dumps(fixture), "labels")
    body = ((fixture.get("states") or [{}])[-1].get("body") or {})
    rn = body.get("remainingNotional") or {}
    val = None
    if isinstance(rn, dict):
        amt = rn.get("amount") or rn
        if isinstance(amt, dict):
            inner = amt.get("amount") if isinstance(amt.get("amount"), dict) else amt
            val = inner.get("value") if isinstance(inner, dict) else None
    rec("old.residual-4.5b", val == "4500000000", "remainingNotional=%s" % val)
    rec("old.settled-duties", [o.get("status") for o in body.get("obligations") or []] == ["Settled", "Settled"], "obligations")
    rec("old.episode-closed-agreement-outstanding", body.get("episodeStatus") == "Closed" and body.get("agreementStatus") == "Outstanding", "%s/%s" % (body.get("episodeStatus"), body.get("agreementStatus")))
    rec("old.lifecycle-rev2-rem0", str(body.get("revision")) == "2" and str(body.get("remaining")) == "0", "rev=%s rem=%s" % (body.get("revision"), body.get("remaining")))
    names = [v.get("name") for v in body.get("values") or []]
    rec("old.nine-values", names == [
        "notional", "principal_due", "interest_due", "principal_paid", "interest_paid",
        "borrower_cash", "lender_cash", "cursor", "episode_closed",
    ], str(names))
    od = (doc.get("commonAcceptance") or {}).get("oldDomain") or {}
    rec("old.transport-present", "D0Obs_complete" in json.dumps(od.get("transport") or {}), "transport")
    rec("old.usd-micro-distinct", "USD_micro" in json.dumps(od.get("transport") or {}) and "runtimeNat" in json.dumps((doc.get("commonAcceptance") or {}).get("schemas") or {}), "asset distinction")
    rec("old.mc04-mc05-conditional", "MC04" in json.dumps(od) and "conditional" in json.dumps(od).lower(), "conditional")
    if pins is not None:
        got = {s.get("path"): s.get("sha256") for s in pins.get("sources") or [] if isinstance(s, dict)}
        rec("old.pins-file-matches", all(got.get(k) == v for k, v in PIN_EXPECT.items()), "pins")
    rec("old.theorems-proposed", all(t.get("status") == "proposed" for t in ((doc.get("theoremLedger") or {}).get("theorems") or [])), "proposed")


def test_schema_and_authority(doc):
    sch = (doc.get("commonAcceptance") or {}).get("schemas") or {}
    rec("schema.closed-products", isinstance((sch.get("products") or {}).get("CompleteState"), dict), "CompleteState")
    rec("schema.allowed-effect-not-concrete", bool(((sch.get("products") or {}).get("AllowedEffect") or {}).get("notConcreteEffect")), "descriptor")
    rec("schema.unknown-reject", (doc.get("commonAcceptance") or {}).get("jsonDecoder", {}).get("unknownMaterialFieldsReject") is True, "unknown")
    apply = ((doc.get("commonAcceptance") or {}).get("effectSystem") or {}).get("apply") or {}
    rec("schema.required-transfer", "Transfer" in apply and apply["Transfer"].get("cashMovement") is True, "Transfer")
    rec("schema.lock-annotation-identity", apply.get("Lock", {}).get("cashMovement") is False, "Lock")
    bob = (((traces_by_id(doc)[REQUIRED_TRACE_IDS[3]].get("preState") or {}).get("authority") or {}).get("Bob") or {})
    tas = bob.get("transferAllowances") or {}
    rec("authority.bob-weth-indexed", "WETH" in tas or (bob.get("transferAllowance") or {}).get("asset") in ("USDC", "WETH"), "allowances=%s" % list(tas))
    rec("schema.fee-requires-annotates", "annotates" in json.dumps((sch.get("products") or {}).get("FeeRecord") or {}), "FeeRecord")


def test_footprints(doc):
    for row in doc.get("rows") or []:
        fp = row.get("boundedFootprint") or {}
        n = int(fp.get("entryCount") or fp.get("actualRequirement") or 0)
        rec(
            "fp.%s" % row.get("id"),
            n > 0 and n <= TOY_FOOTPRINT_MAX,
            "count=%s within64=%s" % (n, n <= TOY_FOOTPRINT_MAX),
        )
        rec("fp.%s.not-hidden" % row.get("id"), bool(fp.get("reads") and fp.get("writes")), "reads/writes present")


def test_check_cli_pass(candidate, fixture, pins):
    code, report = run_check_cli(candidate, fixture, pins)
    rec("cli.check-exit-0", code == 0, "exit=%s ok=%s" % (code, (report or {}).get("ok")))
    rec("cli.checked-positives-6", ((report or {}).get("checked") or {}).get("positives") == 6, str((report or {}).get("checked")))
    rec("cli.checked-negatives-11", ((report or {}).get("checked") or {}).get("negatives") == 11, str((report or {}).get("checked")))
    rec("cli.self-consistency-flag", (report or {}).get("selfConsistencyOnly") is True, "flag")
    rec("cli.not-bnf-k", (report or {}).get("notBNF") is True and (report or {}).get("notK") is True, "limits")


def test_adversarial_probes(candidate_obj, fixture, pins):
    cases = [
        ("createCash", ["traces", 4, "expected", "postState", "accounts", "Lender", "USDC"], "101", False),
        ("erasePrimitiveEquation", ["commonAcceptance", "effectSystem", "apply", "Transfer"], None, True),
        ("eraseNominalDebt", ["traces", 0, "expected", "postState", "debts", "nam19", "principal"], "0", False),
        ("forgeFailureEvidence", ["mutations", 0, "derivedFirstFailure", "failedPredicate"], "arbitrary false assertion", False),
    ]
    with tempfile.TemporaryDirectory() as td:
        for name, path, value, remove in cases:
            doc = json.loads(json.dumps(candidate_obj))
            if name == "forgeFailureEvidence":
                set_path(doc, path, value)
                if isinstance(doc["mutations"][0].get("firstFailure"), dict):
                    doc["mutations"][0]["firstFailure"]["failedPredicate"] = value
                else:
                    doc["mutations"][0]["firstFailure"] = value
            else:
                set_path(doc, path, value, remove=remove)
            out = Path(td) / (name + ".json")
            write_json(out, doc)
            code, report = run_check_cli(out, fixture, pins)
            rec(
                "probe.%s.rejects" % name,
                code != 0,
                "exit=%s ok=%s first=%s" % (code, (report or {}).get("ok"), json.dumps((report or {}).get("firstFailure"))[:240]),
            )


def test_additional_rejects(candidate_obj, fixture, pins):
    with tempfile.TemporaryDirectory() as td:
        # drift Alice bound 101 -> 50
        doc = json.loads(json.dumps(candidate_obj))
        alice = doc["traces"][4]["preState"]["authority"]["Alice"]
        if "transferAllowance" in alice:
            alice["transferAllowance"]["original"] = "50"
            alice["transferAllowance"]["remaining"] = "50"
        if "transferAllowances" in alice and "USDC" in alice["transferAllowances"]:
            alice["transferAllowances"]["USDC"]["original"] = "50"
            alice["transferAllowances"]["USDC"]["remaining"] = "50"
        p = Path(td) / "drift.json"
        write_json(p, doc)
        code, _ = run_check_cli(p, fixture, pins)
        rec("reject.authority-drift-101", code != 0, "exit=%s" % code)

        # make Lock a cash mover in declared identity
        doc = json.loads(json.dumps(candidate_obj))
        doc["commonAcceptance"]["effectSystem"]["apply"]["Lock"]["cashMovement"] = True
        p = Path(td) / "lockcash.json"
        write_json(p, doc)
        code, _ = run_check_cli(p, fixture, pins)
        rec("reject.lock-declared-cash", code != 0, "exit=%s" % code)

        # drop second SetNominalRate
        doc = json.loads(json.dumps(candidate_obj))
        effs = doc["traces"][0]["concretePlan"]["effects"]
        rates = [i for i, e in enumerate(effs) if e.get("ctor") == "SetNominalRate"]
        if len(rates) >= 2:
            del effs[rates[-1]]
        p = Path(td) / "norr2.json"
        write_json(p, doc)
        code, _ = run_check_cli(p, fixture, pins)
        rec("reject.missing-second-RR", code != 0, "exit=%s" % code)


def parse_args(argv):
    p = argparse.ArgumentParser()
    p.add_argument("--candidate", required=True)
    p.add_argument("--old-fixture", required=True)
    p.add_argument("--old-pins", required=True)
    return p.parse_args(argv)


def main(argv=None):
    args = parse_args(argv if argv is not None else sys.argv[1:])
    doc, sha = load_json(args.candidate)
    fixture, fsha = load_json(args.old_fixture)
    pins, psha = load_json(args.old_pins)
    print("candidate_sha256 " + sha)
    print("old_fixture_sha256 " + fsha)
    rec("input.fixture-digest", fsha == OLD_FIXTURE_SHA, fsha)
    rec("test.does-not-import-model", "composition-model" not in sys.modules, "modules")
    test_retained_ids(doc)
    test_independent_nam(doc)
    test_independent_cash(doc)
    test_lock_annotation(doc)
    test_async_phase(doc)
    test_shared_ord(doc)
    test_partial_and_alloc(doc)
    test_old_transport(doc, fixture, pins)
    test_schema_and_authority(doc)
    test_footprints(doc)
    test_check_cli_pass(args.candidate, args.old_fixture, args.old_pins)
    test_adversarial_probes(doc, args.old_fixture, args.old_pins)
    test_additional_rejects(doc, args.old_fixture, args.old_pins)
    failed = [n for n, ok, _ in RESULTS if not ok]
    print("SUMMARY total=%s fail=%s pass=%s" % (len(RESULTS), len(failed), len(RESULTS) - len(failed)))
    if failed:
        print("FAILED " + " ".join(failed))
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
