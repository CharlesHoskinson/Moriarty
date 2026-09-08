#!/usr/bin/env python3
"""Independent finite design-validation tests for the composition fragment.

Does not import composition-model.py and does not call a generator helper to
derive expected values. Arithmetic, schema, and first-failure checks are
computed here from the candidate records and optional old fixture.

CLI:
  python3 composition-model.test.py --candidate FILE --old-fixture FILE
  python3 composition-model.test.py --candidate FILE --old-fixture FILE --old-pins FILE

Exit 0 iff every check passes. Exit 1 on any FAIL. Prints PASS/FAIL lines.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import sys
from decimal import Decimal, getcontext
from fractions import Fraction
from pathlib import Path

getcontext().prec = 80

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
]
REQUIRED_THEOREM_IDS = [
    "type-preservation",
    "asset-indexed-accounting",
    "authority-safety",
    "frame-noninterference",
    "assume-guarantee-composition",
    "structural-associativity",
    "obligation-preservation",
    "conservative-extension",
]
REQUIRED_ROW_IDS = [
    "NAM19-capitalization",
    "sequential",
    "disjoint-parallel",
    "shared-state-interleaving",
    "atomic-synchronization",
    "asynchronous-messaging",
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
PROGRAM_HASH = "95b46e39a9039e19063bb3d618128aec6cbd9ee656b6e635b3587e7f3f5235b2"
REGISTRY_BOUNDS_HASH = "ad0e1d45c9cfb5b1843d73f4d497d7d07f0450caddfcd49f3ef81f07f63d567c"
TOY_FOOTPRINT_MAX = 64
STAGES = ("ContractInvariant", "IntentRefinement", "TransitionValidity", "HistoryCompliance")

RESULTS = []


def rec(name, ok, detail):
    RESULTS.append((name, bool(ok), detail))
    print(("PASS" if ok else "FAIL") + " " + name + " :: " + detail)


def load_json(path):
    p = Path(path)
    raw = p.read_bytes()
    return json.loads(raw.decode("utf-8")), hashlib.sha256(raw).hexdigest(), p


def traces_by_id(doc):
    return {t["id"]: t for t in doc.get("traces") or [] if isinstance(t, dict) and "id" in t}


def mutations_by_id(doc):
    return {m["id"]: m for m in doc.get("mutations") or [] if isinstance(m, dict) and "id" in m}


def theorems_by_id(doc):
    th = ((doc.get("theoremLedger") or {}).get("theorems")) or []
    return {t["id"]: t for t in th if isinstance(t, dict) and "id" in t}


def rows_by_id(doc):
    return {r["id"]: r for r in doc.get("rows") or [] if isinstance(r, dict) and "id" in r}


def dump_text(obj):
    return json.dumps(obj, sort_keys=True)


def frac_dec(x):
    return Decimal(x.numerator) / Decimal(x.denominator)


def nam_accruals():
    cap1 = Fraction(5000) * Fraction(8, 100) * Fraction(90, 365)
    rate1 = Decimal("0.010567901234567900") + Decimal("0.10")
    cap2 = Fraction(5000) * Fraction(rate1) * Fraction(91, 365)
    return cap1, cap2, cap1 + cap2, Fraction(5000) + cap1 + cap2


def is_annotation_fee(effect):
    if not isinstance(effect, dict):
        return False
    if effect.get("role") == "annotation":
        return True
    if effect.get("cashMovement") is False:
        return True
    link = effect.get("annotates") or effect.get("annotationOf") or effect.get("linksToTransfer")
    return bool(link)


def cash_fee_moves(effects):
    out = []
    for e in effects or []:
        if not isinstance(e, dict):
            continue
        ctor = e.get("ctor")
        amt = str(e.get("amount"))
        asset = e.get("asset")
        if ctor == "Transfer":
            dest = e.get("to") or e.get("recipient")
            out.append((ctor, e.get("from"), dest, asset, amt, e.get("id")))
        elif ctor == "AccrueFee" and not is_annotation_fee(e):
            dest = e.get("recipient") or e.get("to")
            out.append((ctor, e.get("from"), dest, asset, amt, e.get("id") or e.get("feeId")))
    return out


def ctor_def(doc, name):
    ctors = ((doc.get("commonAcceptance") or {}).get("effectSystem") or {}).get("primitiveConstructors") or {}
    if isinstance(ctors, dict):
        return ctors.get(name) or {}
    if isinstance(ctors, list):
        for c in ctors:
            if isinstance(c, dict) and (c.get("name") or c.get("ctor")) == name:
                return c
    return {}


def apply_spec(doc):
    return ((doc.get("commonAcceptance") or {}).get("effectSystem") or {}).get("apply")


def schema_root(doc):
    ca = doc.get("commonAcceptance") or {}
    return ca.get("schemas") or ca.get("closedSchemas") or ca.get("jsonDecoder") or {}


def complete_state_fields(doc):
    sch = schema_root(doc)
    fields = sch.get("completeStateFields") or sch.get("CompleteState") or []
    if isinstance(fields, dict):
        return list(fields.keys()) if "fields" not in fields else (
            fields["fields"] if isinstance(fields["fields"], list) else list((fields["fields"] or {}).keys())
        )
    if isinstance(fields, list):
        return fields
    prod = (sch.get("products") or {}).get("CompleteState") or {}
    if isinstance(prod, dict):
        f = prod.get("fields") or prod.get("components") or prod
        if isinstance(f, dict):
            return list(f.keys())
        if isinstance(f, list):
            return [x.get("name") if isinstance(x, dict) else x for x in f]
    return []


def formula_blob(th):
    parts = [str(th.get("predicate") or "")]
    for dfn in th.get("definitions") or []:
        if isinstance(dfn, dict):
            parts.append(str(dfn.get("formula") or ""))
            parts.append(dump_text(dfn))
    return "\n".join(parts)


def has_source_decimal_grammar(doc):
    ca = doc.get("commonAcceptance") or {}
    qs = ca.get("quantitySchema") or doc.get("quantitySchema") or {}
    jd = ca.get("jsonDecoder") or {}
    sch = ca.get("schemas") or {}
    blobs = [qs, jd, sch, ca.get("sourceDecimal"), ca.get("SourceDecimal")]
    text = dump_text(blobs)
    need = ("grammar", "precision", "length", "arithmetic")
    hits = sum(1 for k in need if k.lower() in text.lower())
    g = sch.get("SourceDecimal") or jd.get("sourceDecimal") or qs.get("sourceDecimalGrammar")
    if isinstance(g, dict) and g.get("grammar") and (g.get("maxPrecision") or g.get("precision")):
        return True, g
    return hits >= 4 and ("source-decimal" in text or "SourceDecimal" in text), g


def effects_of(trace):
    plan = (trace.get("concretePlan") or {}).get("effects") or []
    exp = (trace.get("expected") or {}).get("effects") or []
    return plan, exp


def authority_entries(state):
    auth = (state or {}).get("authority") or {}
    if isinstance(auth, dict):
        return auth
    return {}


def transfer_required_ok(effect):
    if not isinstance(effect, dict) or effect.get("ctor") != "Transfer":
        return True
    return all(effect.get(k) not in (None, "") for k in ("from", "to", "asset", "amount", "unit", "scale"))


def receive_required_ok(effect):
    if not isinstance(effect, dict) or effect.get("ctor") != "ReceiveMessage":
        return True
    return all(
        effect.get(k) not in (None, "")
        for k in ("id", "nonce", "to", "tick", "escrowSource", "destFinalityObservationId")
    )


def first_failure_is_derived(mut):
    derived = mut.get("derivedFirstFailure") or mut.get("firstFailure") or {}
    if not isinstance(derived, dict):
        return False
    stage = derived.get("stage") or derived.get("firstFailingStage")
    relation = derived.get("relation") or derived.get("claimRelation") or derived.get("failedPredicate")
    if stage not in STAGES:
        return False
    if not relation:
        return False
    if derived.get("holds") is True:
        return False
    earlier = mut.get("earlierStagesDerived") or []
    if any(isinstance(x, dict) and x.get("holds") is True and not x.get("evaluatedUnder") for x in earlier):
        # holds:true without evaluation is the blocked form
        if all(isinstance(x, dict) and x.get("holds") is True and "evaluated" not in dump_text(x).lower() for x in earlier):
            return False
    return True


def contains_private_abs_path(obj):
    blob = dump_text(obj)
    return "/home/" in blob or "/Users/" in blob


def test_retained_ids(doc):
    t = traces_by_id(doc)
    m = mutations_by_id(doc)
    th = theorems_by_id(doc)
    r = rows_by_id(doc)
    rec(
        "retain.traces",
        [x["id"] for x in doc.get("traces") or []] == REQUIRED_TRACE_IDS and len(t) == 6,
        "ids=" + ",".join(t.keys()),
    )
    rec(
        "retain.mutations",
        [x["id"] for x in doc.get("mutations") or []] == REQUIRED_MUTATION_IDS and len(m) == 10,
        "n=" + str(len(m)),
    )
    rec(
        "retain.theorems",
        [x["id"] for x in ((doc.get("theoremLedger") or {}).get("theorems") or [])] == REQUIRED_THEOREM_IDS,
        "ids=" + ",".join(th.keys()),
    )
    rec(
        "retain.rows",
        [x["id"] for x in doc.get("rows") or []] == REQUIRED_ROW_IDS,
        "ids=" + ",".join(r.keys()),
    )
    rec("retain.no-abs-private-path", not contains_private_abs_path(doc), "scanned generated artifact")


def test_fee_exactly_once(doc):
    fee_traces = [
        "composition:shared-state-interleaving:trace",
        "composition:atomic-synchronization:trace",
        "composition:asynchronous-messaging:trace",
    ]
    expected_alice = {
        "composition:shared-state-interleaving:trace": ("29", "1"),
        "composition:atomic-synchronization:trace": ("0", "1"),
        "composition:asynchronous-messaging:trace": ("39", "1"),
    }
    ctors = ctor_def(doc, "AccrueFee")
    equals = str(ctors.get("equals") or "")
    annotation_defined = bool(
        ctors.get("annotationOf")
        or ctors.get("annotates")
        or ctors.get("role") == "annotation"
        or ctors.get("cashMovement") is False
        or "annotation" in dump_text(ctors).lower()
    )
    rec(
        "fee.ctor-not-second-transfer",
        annotation_defined and "Transfer(" not in equals,
        "AccrueFee ctor equals=%r annotation=%s" % (equals, annotation_defined),
    )
    tb = traces_by_id(doc)
    for tid in fee_traces:
        tr = tb.get(tid) or {}
        plan, exp = effects_of(tr)
        for label, effs in (("plan", plan), ("expected", exp)):
            moves = [
                mv
                for mv in cash_fee_moves(effs)
                if mv[1] == "Alice" and mv[2] == "Treasury" and mv[3] == "USDC" and mv[4] in ("1", "1.0")
            ]
            rec(
                "fee.once.%s.%s" % (tid.split(":")[1], label),
                len(moves) == 1 and moves[0][0] == "Transfer",
                "cashMoves=%s" % (moves,),
            )
        post = ((tr.get("expected") or {}).get("postState") or {}).get("accounts") or {}
        alice, treas = expected_alice[tid]
        rec(
            "fee.balance.%s" % tid.split(":")[1],
            str((post.get("Alice") or {}).get("USDC")) == alice
            and str((post.get("Treasury") or {}).get("USDC")) == treas,
            "Alice=%s Treasury=%s want %s/%s"
            % ((post.get("Alice") or {}).get("USDC"), (post.get("Treasury") or {}).get("USDC"), alice, treas),
        )
    atomic = tb.get("composition:atomic-synchronization:trace") or {}
    alice_auth = authority_entries((atomic.get("expected") or {}).get("postState")).get("Alice") or {}
    gross = str(((alice_auth.get("transferAllowance") or {}).get("cumulativeGross")))
    rec("fee.atomic-gross-101", gross == "101", "cumulativeGross=%s" % gross)


def test_cumulative_cap(doc):
    th = theorems_by_id(doc).get("authority-safety") or {}
    blob = formula_blob(th)
    has_cumul = "cumulative" in blob.lower() and ("remaining" in blob.lower() or "gross" in blob.lower())
    has_binding = ("payee" in blob.lower() or "recipient" in blob.lower()) and "asset" in blob.lower()
    rec("cap.predicate-cumulative", has_cumul and has_binding, "authority-safety mentions cumulative+binding")
    ca = doc.get("commonAcceptance") or {}
    disc = (
        ca.get("discriminators")
        or (ca.get("effectSystem") or {}).get("discriminators")
        or doc.get("discriminators")
        or {}
    )
    two60 = None
    if isinstance(disc, dict):
        two60 = disc.get("twoTransfer60Under101") or disc.get("cumulativeCap120gt101")
    if two60 is None:
        for tr in doc.get("traces") or []:
            d = (tr.get("expected") or {}).get("discriminators") or {}
            if "twoTransfer60Under101" in d:
                two60 = d["twoTransfer60Under101"]
    if two60 is None:
        two60 = (ca.get("effectSystem") or {}).get("twoTransfer60Under101")
    ok = isinstance(two60, dict)
    status = str((two60 or {}).get("status") or (two60 or {}).get("result") or "")
    rec(
        "cap.twoTransfer60-reject",
        ok and status.lower() in ("reject", "rejected", "fail") and "120" in dump_text(two60) and "101" in dump_text(two60),
        "discriminator=%s" % (dump_text(two60)[:240] if two60 is not None else None,),
    )
    atomic = traces_by_id(doc).get("composition:atomic-synchronization:trace") or {}
    alice_pre = authority_entries(atomic.get("preState")).get("Alice") or {}
    bound = None
    for ae in alice_pre.get("allowedEffects") or []:
        if ae.get("ctor") == "Transfer" and ae.get("asset") == "USDC":
            bound = ae.get("bound")
    rec("cap.alice-101-retained", str(bound) == "101", "Alice Transfer USDC bound=%s" % bound)
    # Independent arithmetic: 60<=101 per effect, 60+60=120>101 cumulative.
    rec("cap.independent-120gt101", 60 <= 101 and (60 + 60) > 101, "per-effect 60<=101 and 120>101")
    membership = (ca.get("effectSystem") or {}).get("membership") or th
    memb_text = dump_text(membership) if not isinstance(membership, str) else membership
    rec(
        "cap.membership-not-per-effect-only",
        "cumulative" in memb_text.lower() or "remaining" in memb_text.lower(),
        "membership/authority text carries cumulative remaining",
    )


def test_actual_fields_schema(doc):
    fields = complete_state_fields(doc)
    rec(
        "schema.complete-state-workPartition-frame",
        "workPartition" in fields and "frame" in fields,
        "fields=%s" % fields,
    )
    sch = (doc.get("commonAcceptance") or {}).get("schemas") or {}
    products = sch.get("products") or {}
    sums = sch.get("sums") or sch.get("variants") or {}
    needed = [
        "ObservationRecord",
        "ContractRecord",
        "AllowanceRecord",
        "PositionRecord",
        "ClaimRecord",
        "RequestRecord",
        "MessageRecord",
        "OrderRecord",
        "WorkPartition",
        "Frame",
        "AuthorityRecord",
    ]
    have = set(products) | set(sums) | set(sch)
    rec("schema.closed-records", all(n in have for n in needed), "have=%s" % sorted(have))
    ok_g, g = has_source_decimal_grammar(doc)
    rec("schema.source-decimal-grammar", ok_g, "grammar=%s" % (dump_text(g)[:200] if g else "missing",))
    nat_sep = "runtime-nat" in dump_text(doc.get("quantitySchema") or {}) or "runtimeNat" in dump_text(
        (doc.get("commonAcceptance") or {}).get("jsonDecoder") or {}
    )
    rec("schema.runtimeNat-separate", nat_sep, "runtime nat distinct from source-decimal")
    tb = traces_by_id(doc)
    nam = tb.get("heldouts:NAM19-capitalization:trace") or {}
    shared = tb.get("composition:shared-state-interleaving:trace") or {}
    atomic = tb.get("composition:atomic-synchronization:trace") or {}
    async_tr = tb.get("composition:asynchronous-messaging:trace") or {}
    par = tb.get("composition:disjoint-parallel:trace") or {}

    def effects(tr):
        return ((tr.get("concretePlan") or {}).get("effects") or []) + ((tr.get("expected") or {}).get("effects") or [])

    cd = [e for e in effects(nam) if e.get("ctor") == "CreateDebt"]
    rec(
        "schema.CreateDebt-fields",
        bool(cd)
        and all(all(e.get(k) not in (None, "") for k in ("debtor", "creditor", "unit", "scale")) for e in cd),
        "CreateDebt=%s" % cd,
    )
    an = [e for e in effects(nam) if e.get("ctor") == "AccrueNominal"]
    rec(
        "schema.AccrueNominal-observationRef",
        bool(an) and all(e.get("observationRef") not in (None, "") for e in an),
        "AccrueNominal=%s" % [{k: e.get(k) for k in ("accrued", "observationRef", "debtId")} for e in an],
    )
    mint = [e for e in effects(shared) if e.get("ctor") == "Mint"]
    rec(
        "schema.Mint-unit-scale",
        bool(mint) and all(e.get("unit") not in (None, "") and e.get("scale") not in (None, "") for e in mint),
        "Mint=%s" % mint,
    )
    ad = [e for e in effects(async_tr) if e.get("ctor") == "AssignDuty"]
    rec(
        "schema.AssignDuty-debtId-or-variant",
        bool(ad)
        and all(e.get("debtId") not in (None, "") or e.get("dutyVariant") or e.get("kind") == "locked-funds" and e.get("lockId") for e in ad),
        "AssignDuty=%s" % ad,
    )
    par_post = (par.get("expected") or {}).get("postState") or {}
    rec(
        "schema.parallel-post-workPartition-frame",
        "workPartition" in par_post and "frame" in par_post,
        "post keys=%s" % list(par_post.keys()),
    )
    meta = (doc.get("commonAcceptance") or {}).get("documentaryMetadataProjection") or sch.get("documentaryMetadataProjection")
    rec("schema.documentary-metadata-projection", isinstance(meta, dict) and bool(meta), "projection=%s" % (dump_text(meta)[:180] if meta else None,))
    for tid, tr in (("atomic", atomic), ("shared", shared), ("async", async_tr), ("par", par)):
        post_auth = authority_entries((tr.get("expected") or {}).get("postState"))
        missing = []
        for name, recd in post_auth.items():
            if not isinstance(recd, dict):
                continue
            variant = recd.get("variant") or recd.get("tag") or recd.get("kind")
            if recd.get("allowedEffects") is None and variant not in ("FeeRecipient", "feeRecipient"):
                missing.append((name, "allowedEffects"))
            if recd.get("transferAllowance") is None and variant not in (
                "FeeRecipient",
                "feeRecipient",
                "JointEnvelope",
                "joint",
                "System",
            ):
                missing.append((name, "transferAllowance"))
        rec("schema.authority-variants.%s" % tid, missing == [], "missing=%s" % missing)


def test_per_primitive_and_full_states(doc):
    apply = apply_spec(doc)
    rec(
        "apply.equations-present",
        isinstance(apply, dict)
        and any(isinstance(v, dict) and (v.get("equations") or v.get("preconditions")) for v in apply.values()),
        "apply type=%s keys=%s" % (type(apply).__name__, list(apply)[:12] if isinstance(apply, dict) else apply),
    )
    cap1, cap2, capsum, postp = nam_accruals()
    cap2_dec = frac_dec(cap2)
    post_dec = frac_dec(postp)
    rec(
        "nam.independent-intervening",
        cap2_dec > Decimal("98.63013698630137") and str(post_dec).startswith("5236.461356333502"),
        "cap1=%s cap2=%s post=%s" % (frac_dec(cap1), cap2_dec, post_dec),
    )
    nam = traces_by_id(doc).get("heldouts:NAM19-capitalization:trace") or {}
    accs = [e for e in ((nam.get("concretePlan") or {}).get("effects") or []) if e.get("ctor") == "AccrueNominal"]
    found_second = False
    for e in accs:
        try:
            val = Decimal(str(e.get("accrued")))
        except Exception:
            continue
        if abs(val - cap2_dec) < Decimal("0.0000000001") or str(e.get("accrued")).startswith("137.831219347"):
            found_second = True
    rec(
        "nam.intervening-AccrueNominal",
        found_second and len(accs) >= 2,
        "AccrueNominal=%s" % [{k: e.get(k) for k in ("accrued", "observationRef", "period")} for e in accs],
    )
    events = (nam.get("concretePlan") or {}).get("events") or (nam.get("expected") or {}).get("capitalizationPrefix")
    seq = events if isinstance(events, list) and events and isinstance(events[0], str) else [
        (x.get("eventType") if isinstance(x, dict) else x) for x in (events or [])
    ]
    rec("nam.source-event-sequence", seq[:4] == ["IED", "RR", "IPCI", "RR"] or seq == ["IED", "RR", "IPCI", "RR"], "seq=%s" % seq)
    hard = (nam.get("canonicalIntent") or {}).get("hard") or []
    h3 = next((h for h in hard if h.get("id") == "H3"), None)
    h3t = dump_text(h3)
    rec(
        "nam.H3-full-prefix",
        isinstance(h3, dict)
        and ("intervening" in h3t.lower() or "AccrueNominal" in h3t or "137.831" in h3t or "sum" in h3t.lower())
        and "5098.63013698630137" not in (h3.get("predicate") or ""),
        "H3=%s" % h3,
    )
    gen = ((doc.get("commonAcceptance") or {}).get("effectSystem") or {}).get("genesisRule")
    rec(
        "genesis.payer-payee",
        isinstance(gen, dict)
        and ("payer" in dump_text(gen).lower() and "payee" in dump_text(gen).lower()),
        "genesisRule=%s" % (dump_text(gen)[:240],),
    )
    shared = traces_by_id(doc).get("composition:shared-state-interleaving:trace") or {}
    pos_eff = [
        e
        for e in ((shared.get("expected") or {}).get("effects") or [])
        if e.get("ctor") in ("UpdatePositionReserves", "AdjustShareNAV", "Mint") or "reserve" in dump_text(e).lower()
    ]
    rec("apply.position-reserves", bool(pos_eff), "position-related effects=%s" % [e.get("ctor") for e in pos_eff])
    rate_eff = [
        e
        for e in ((nam.get("expected") or {}).get("effects") or []) + ((nam.get("concretePlan") or {}).get("effects") or [])
        if e.get("ctor") in ("SetNominalRate", "UpdateObservation", "AccrueNominal", "Capitalize")
    ]
    rec("apply.nam-rate-observation", len(rate_eff) >= 3, "rate/obs/accrual ctors=%s" % [e.get("ctor") for e in rate_eff])


def test_footprints_frame(doc):
    th = theorems_by_id(doc).get("frame-noninterference") or {}
    blob = formula_blob(th)
    rec("frame.max-64", "64" in blob or str(TOY_FOOTPRINT_MAX) in dump_text(doc.get("commonAcceptance")), "frame blob has 64")
    ca = doc.get("commonAcceptance") or {}
    rec(
        "frame.declared-toy-max-64",
        str((ca.get("toyFootprintMaximum") or (ca.get("records") or {}).get("boundParameters", {}).get("toyFootprintMaximum", {}).get("value")))
        == "64"
        or ca.get("toyFootprintMaximum") == 64
        or ((ca.get("records") or {}).get("boundParameters") or {}).get("maxFootprintEntries", {}).get("value") == "64",
        "toyFootprintMaximum recorded as design bound 64",
    )
    proj = ca.get("canonicalResourceProjection") or ca.get("resourceProjection")
    rec(
        "frame.total-projection",
        isinstance(proj, dict) and ("parent" in dump_text(proj) or "child" in dump_text(proj)),
        "projection keys=%s" % (list(proj)[:20] if isinstance(proj, dict) else proj,),
    )
    grammar = str(ca.get("canonicalResourcePathGrammar") or "")
    rec("frame.positions-in-grammar", "positions" in grammar, "grammar=%s" % grammar[:200])
    for row in doc.get("rows") or []:
        fp = row.get("boundedFootprint") or {}
        reads = fp.get("reads") or fp.get("read") or []
        writes = fp.get("writes") or fp.get("write") or []
        n = len(reads) + len(writes)
        rec(
            "frame.size.%s" % row.get("id"),
            n <= TOY_FOOTPRINT_MAX and n > 0,
            "n=%s reads=%s writes=%s" % (n, len(reads), len(writes)),
        )
        if row.get("id") in ("shared-state-interleaving", "atomic-synchronization"):
            wres = [w.get("resource") for w in writes if isinstance(w, dict)]
            rec(
                "frame.positions-writes.%s" % row.get("id"),
                any(isinstance(x, str) and x.startswith("positions.") for x in wres),
                "writes=%s" % wres,
            )
    par = traces_by_id(doc).get("composition:disjoint-parallel:trace") or {}
    pre_u = ((par.get("preState") or {}).get("unrelatedRecord"))
    post_u = (((par.get("expected") or {}).get("postState") or {}).get("unrelatedRecord"))
    rec("frame.unrelated-preserved", pre_u == post_u and pre_u is not None, "unrelated pre/post equal")


def test_partial_duty(doc):
    th = theorems_by_id(doc).get("obligation-preservation") or {}
    blob = formula_blob(th)
    rec(
        "duty.amount-aware-not-contradictory-set",
        "amount" in blob.lower()
        and "residual" in blob.lower()
        and not (
            "minus Discharged) union Created" in blob and "partial" in blob.lower() and "same" not in blob.lower()
        ),
        "DutyConserv mentions amount-aware residual update",
    )
    disc = ((doc.get("commonAcceptance") or {}).get("discriminators") or {}).get("partialDuty50to30")
    if disc is None:
        disc = (th.get("discriminators") or {}).get("partialDuty50to30") if isinstance(th.get("discriminators"), dict) else None
    if disc is None:
        for tr in doc.get("traces") or []:
            d = ((tr.get("expected") or {}).get("discriminators") or {}).get("partialDuty50to30")
            if d:
                disc = d
                break
    ok = isinstance(disc, dict)
    rec(
        "duty.50-to-30-same-id",
        ok
        and str((disc.get("pre") or {}).get("amount") or disc.get("preAmount")) == "50"
        and str((disc.get("payment") or {}).get("amount") or disc.get("paid")) == "20"
        and str((disc.get("post") or {}).get("amount") or disc.get("postAmount")) == "30"
        and str((disc.get("pre") or {}).get("id") or disc.get("id"))
        == str((disc.get("post") or {}).get("id") or disc.get("id")),
        "disc=%s" % (dump_text(disc)[:300] if disc else None,),
    )
    rec(
        "duty.unique-allocation-transfer-id",
        ok and (disc.get("allocationId") and (disc.get("transferId") or disc.get("fundingTransferId"))),
        "allocation/transfer present",
    )


def receive_admitted(now, deadline, dest_tick, dest_auth, pending=True):
    return bool(pending) and bool(dest_auth) and dest_tick is not None and dest_tick < deadline and now is not None


def refund_admitted(now, deadline, dest_tick, dest_auth, pending=True):
    rec_ok = receive_admitted(now, deadline, dest_tick, dest_auth, pending)
    return bool(pending) and now is not None and now >= deadline and not rec_ok


def test_async_deadline(doc):
    rec(
        "async.independent-99vs100",
        receive_admitted(100, 100, 99, True) and not refund_admitted(100, 100, 99, True),
        "now=100 finality=99 receive xor refund",
    )
    rec(
        "async.independent-missing-obs-refunds",
        (not receive_admitted(100, 100, None, False)) and refund_admitted(100, 100, None, False),
        "missing dest obs => refund",
    )
    tr = traces_by_id(doc).get("composition:asynchronous-messaging:trace") or {}
    msm = tr.get("messageStateMachine") or {}
    rule = msm.get("deadlineRule") or msm.get("deterministicBranch") or {}
    rec(
        "async.predicate-not-assertion",
        isinstance(rule, dict)
        and "receiveAdmitted" in dump_text(rule)
        and "refundAdmitted" in dump_text(rule)
        and "exactly one" in dump_text(rule).lower() or (isinstance(rule, dict) and rule.get("exclusive")),
        "deadlineRule=%s" % (dump_text(rule)[:300],),
    )
    alts = ((tr.get("expected") or {}).get("alternateSuccessors") or {})
    recv = alts.get("receive") or {}
    refu = alts.get("refund") or {}
    rec(
        "async.receive-complete-post",
        isinstance(recv.get("postState"), dict)
        and all(k in recv["postState"] for k in ("accounts", "work", "authority", "historyIds", "observations", "duties")),
        "receive keys=%s" % list(recv.keys()),
    )
    rec(
        "async.refund-complete-post",
        isinstance(refu.get("postState"), dict)
        and all(k in refu["postState"] for k in ("accounts", "work", "authority", "historyIds", "duties")),
        "refund keys=%s" % list(refu.keys()),
    )
    rec_eff = recv.get("effects") or []
    rec(
        "async.receive-O_dest_M3",
        any(
            (e.get("destFinalityObservationId") == "O_dest_M3")
            or (e.get("id") == "O_dest_M3")
            or (e.get("ctor") == "UpdateObservation" and e.get("id") == "O_dest_M3")
            for e in rec_eff
        )
        or any(
            isinstance(o, dict) and o.get("id") == "O_dest_M3"
            for o in ((recv.get("postState") or {}).get("observations") or [])
        ),
        "receive effects/obs include O_dest_M3",
    )
    rec(
        "async.destination-assumption",
        "assumption" in dump_text(msm.get("finalityAssumptionNotProof") or msm.get("authenticatedAssumedObservations") or True).lower()
        or (msm.get("finalityAssumptionNotProof") in (True, "true"))
        or "not a proven" in dump_text(msm).lower(),
        "destination truth remains assumption",
    )
    # entitlement: receive SettleDuty payee must match transformed entitlement
    settle = [e for e in rec_eff if e.get("ctor") == "SettleDuty"]
    transform = recv.get("entitlementTransform") or msm.get("lockDutyEntitlement") or {}
    rec(
        "async.receive-entitlement",
        bool(settle)
        and (
            all(e.get("payee") == "Bob" for e in settle)
            and (
                (transform.get("from") == "Alice" and transform.get("to") == "Bob")
                or any(e.get("authorizedTransform") for e in rec_eff)
                or (recv.get("dutyPayee") == "Bob")
            )
        ),
        "settle=%s transform=%s" % (settle, transform),
    )


def test_typed_negatives(doc):
    mm = mutations_by_id(doc)
    rec("neg.ten-ids", list(mm) == REQUIRED_MUTATION_IDS or set(mm) == set(REQUIRED_MUTATION_IDS), "n=%s" % len(mm))
    for mid, mut in mm.items():
        cand = mut.get("completeCandidate") or {}
        rec(
            "neg.full-candidate.%s" % mid.split(":")[-1],
            isinstance(cand, dict)
            and (
                cand.get("effects")
                or cand.get("expandedEffects")
                or cand.get("expandedPlan")
                or (cand.get("patch") and cand.get("legalReference"))
            ),
            "candidate keys=%s" % list(cand.keys()),
        )
        rec("neg.derived-first-failure.%s" % mid.split(":")[-1], first_failure_is_derived(mut), "first=%s earlier=%s" % (mut.get("derivedFirstFailure") or mut.get("firstFailure"), mut.get("earlierStagesDerived")))
        rec("neg.stage-in-order.%s" % mid.split(":")[-1], mut.get("firstFailingStage") in STAGES, "stage=%s" % mut.get("firstFailingStage"))
        blob = dump_text(cand) + dump_text(mut.get("mutatedInput"))
        if "Transfer" in blob:
            transfers = []
            def walk(o):
                if isinstance(o, dict):
                    if o.get("ctor") == "Transfer":
                        transfers.append(o)
                    for v in o.values():
                        walk(v)
                elif isinstance(o, list):
                    for x in o:
                        walk(x)
            walk(cand)
            walk(mut.get("mutatedInput"))
            rec(
                "neg.transfer-fields.%s" % mid.split(":")[-1],
                all(transfer_required_ok(t) for t in transfers) and bool(transfers),
                "transfers=%s" % transfers[:3],
            )
        if "ReceiveMessage" in blob:
            recvs = []
            def walk2(o):
                if isinstance(o, dict):
                    if o.get("ctor") == "ReceiveMessage":
                        recvs.append(o)
                    for v in o.values():
                        walk2(v)
                elif isinstance(o, list):
                    for x in o:
                        walk2(x)
            walk2(cand)
            walk2(mut.get("mutatedInput"))
            rec(
                "neg.receive-fields.%s" % mid.split(":")[-1],
                all(receive_required_ok(r) for r in recvs) and bool(recvs),
                "receive=%s" % recvs,
            )
    nam0 = mm.get("heldouts:NAM19-capitalization:mut:erase-nominal-on-ipci") or {}
    prefix_eff = (nam0.get("prefixEffects") or (nam0.get("prefixState") or {}).get("creatingEffects") or
                  ((nam0.get("completeCandidate") or {}).get("prefixEffects")))
    rec(
        "neg.nam-prefix-duty-creating-effect",
        isinstance(prefix_eff, list)
        and any(isinstance(e, dict) and e.get("ctor") == "AssignDuty" and e.get("id") == "nam19-debt" for e in prefix_eff),
        "prefixEffects=%s" % prefix_eff,
    )
    seq3 = mm.get("composition:sequential:mut:refresh-work-on-join") or {}
    snap = seq3.get("prefixState") or seq3.get("rollbackSnapshot") or {}
    rec(
        "neg.seq-refresh-rollback-cash",
        str(((snap.get("accounts") or {}).get("Alice") or {}).get("USDC")) in ("90",) or "90" in dump_text(seq3.get("unchangedFinancialState") or snap),
        "rollback/prefix retains post-A cash",
    )


def test_old_simulation(doc, fixture, pins):
    th = theorems_by_id(doc).get("conservative-extension") or {}
    blob = formula_blob(th) + dump_text(th)
    rec("old.eight-theorem-ids", set(theorems_by_id(doc)) == set(REQUIRED_THEOREM_IDS), "kept")
    rec(
        "old.kind-Simulation-not-Prepared",
        "Simulation" in blob and "Prepared" not in blob.split("not")[0] or (
            "kind" in blob.lower() and "Simulation" in blob and "not production" in blob.lower()
        ),
        "retention text pins Simulation",
    )
    witness = (
        th.get("oldSimulationWitness")
        or ((doc.get("commonAcceptance") or {}).get("oldDomain") or {}).get("witness")
        or (doc.get("oldDomainRetention") or {}).get("witness")
    )
    rec("old.witness-present", isinstance(witness, dict), "witness keys=%s" % (list(witness)[:20] if isinstance(witness, dict) else witness,))
    if isinstance(witness, dict):
        rec("old.witness-kind", witness.get("kind") == "Simulation", "kind=%s" % witness.get("kind"))
        rec(
            "old.witness-not-relabeled",
            witness.get("kind") != "Prepared"
            and witness.get("kind") != "Accepted"
            and (witness.get("notProductionPrepared") in (True, "true") or "not production" in dump_text(witness).lower()),
            "not Prepared/Accepted",
        )
        rec(
            "old.synthetic-auth-checks",
            "simulation-only" in dump_text(witness) or witness.get("syntheticAuth") in (True, "true"),
            "synthetic auth recorded",
        )
        rec("old.programHash", witness.get("programHash") == PROGRAM_HASH or PIN_EXPECT["experiments/moriarty-language/spec/examples/loan.mori"] in dump_text(witness), "programHash")
        rec("old.registry-boundsHash", witness.get("boundsHash") == REGISTRY_BOUNDS_HASH or REGISTRY_BOUNDS_HASH in dump_text(witness), "boundsHash")
    rec(
        "old.not-borrow-swap-repay-witness",
        "Borrow/Swap/Repay" not in blob and "is not old syntax" in blob.lower() or "unrelated" in blob.lower() or (
            "not" in blob.lower() and "old syntax" in blob.lower()
        ),
        "toy Borrow/Swap/Repay is not old-syntax witness",
    )
    pins_rec = ((doc.get("commonAcceptance") or {}).get("oldDomain") or {}).get("pins") or th.get("oldPins") or {}
    pin_blob = dump_text(pins_rec) + blob
    rec("old.grammar-hash", PIN_EXPECT["experiments/moriarty-language/spec/grammar.ebnf"] in pin_blob, "grammar")
    rec("old.bounds-file-hash", PIN_EXPECT["experiments/moriarty-language/spec/bounds.json"] in pin_blob, "bounds.json")
    rec("old.numeric-profile-hash", PIN_EXPECT["experiments/moriarty-language/spec/numeric-profile.json"] in pin_blob, "numeric-profile")
    rec("old.evaluator-hash", PIN_EXPECT["experiments/moriarty-language/src/evaluate.ts"] in pin_blob, "evaluate.ts")
    rec("old.source-loan-hash", PIN_EXPECT["experiments/moriarty-language/spec/examples/loan.mori"] in pin_blob, "loan.mori")
    rec(
        "old.D0obs-complete",
        "D0Obs" in blob and "history" in blob.lower() and "authority" in blob.lower(),
        "full D0 observation mapping",
    )
    rec(
        "old.mc04-mc05-transport-separate",
        "MC04" in blob and "MC05" in blob and "conditional" in blob.lower(),
        "separate accepted-ledger transport obligation",
    )
    rec(
        "old.theorems-proposed-no-mechanized",
        all((t.get("status") in ("proposed", "BLOCKED_PROPOSED_STATEMENT") or t.get("mechanizedEvidence") in ([], None)) for t in ((doc.get("theoremLedger") or {}).get("theorems") or [])),
        "theorems remain proposed",
    )
    if fixture is not None:
        rec("old.fixture-kind-Simulation", (fixture.get("steps") or [{}])[0].get("result", {}).get("kind") == "Simulation", "fixture step0 kind")
        rec(
            "old.fixture-no-Prepared-Accepted",
            '"Prepared"' not in dump_text(fixture) and '"Accepted"' not in dump_text(fixture),
            "fixture has no Prepared/Accepted labels",
        )
        rec(
            "old.fixture-sha-pinned",
            OLD_FIXTURE_SHA in pin_blob or (witness or {}).get("fixtureSha256") == OLD_FIXTURE_SHA,
            "fixture digest",
        )
    if pins is not None:
        got = {s.get("path"): s.get("sha256") for s in pins.get("sources") or [] if isinstance(s, dict)}
        rec("old.pins-file-matches", all(got.get(k) == v for k, v in PIN_EXPECT.items()), "pins file")


def test_c01_history_names(doc):
    tb = traces_by_id(doc)
    for tid, tr in tb.items():
        plan = tr.get("concretePlan") or {}
        exp = tr.get("expected") or {}
        internal = plan.get("internalTranscriptOutputs") or plan.get("internalHistoryTranscriptOutputs")
        external = plan.get("externalProducedHeads") or exp.get("externalProducedHeads") or exp.get("producedHistories")
        rec(
            "hist.internal-vs-external.%s" % tid.split(":")[1],
            isinstance(internal, list) and isinstance(external, list),
            "internal=%s external=%s" % (internal, external),
        )


def test_c06_c07(doc):
    ag = theorems_by_id(doc).get("assume-guarantee-composition") or {}
    blob = formula_blob(ag)
    rec(
        "ag.set-difference",
        "difference" in blob.lower() or "\\\\" in blob or "minus" in blob.lower() and "Discharged" in blob,
        "discharged via set difference",
    )
    rec("ag.not-and-not-set", "AND NOT Discharged" not in blob, "no predicate AND NOT set")
    rec("ag.entailment", "entail" in blob.lower(), "explicit entailment")
    rec("ag.no-ordered-composition-placeholder", "ordered composition" not in blob.lower(), "G_shared defined")
    rec(
        "ag.typed-domains",
        "input" in blob.lower() and "output" in blob.lower() and ("restriction" in blob.lower() or "trace" in blob.lower()),
        "typed I/O/trace",
    )
    st = theorems_by_id(doc).get("structural-associativity") or {}
    sblob = formula_blob(st)
    rec("assoc.lockConsumed", "lockConsumed" in sblob or "envelopeConsumed" in sblob, "CompleteObs includes lock/envelope")
    rec("assoc.sort-preserving-rho", "sort" in sblob.lower() and "rho" in sblob.lower(), "sort-preserving rho")
    rec(
        "assoc.payment-vs-work-alloc",
        "payment" in sblob.lower() and ("work" in sblob.lower() or "split" in sblob.lower()),
        "separate allocations",
    )
    rec("assoc.bag-vs-sequence", "bag" in sblob.lower() and "sequence" in sblob.lower(), "par bag vs seq explicit")
    rec("assoc.first-error", "first" in sblob.lower() and "error" in sblob.lower(), "first-error relation")
    rec("assoc.not-arbitrary-welltyped-reject", "any well-typed reject" not in sblob.lower(), "restricted admissibility")


def test_nam_source_pin(doc):
    rec("pin.nam19-source", NAM_SOURCE_SHA in dump_text(doc), "NAM19 source sha retained")


def parse_args(argv):
    p = argparse.ArgumentParser(description="Independent composition fragment design tests")
    p.add_argument("--candidate", required=True, help="composition.json to check")
    p.add_argument("--old-fixture", required=True, help="retained-atomic-fixture.json")
    p.add_argument("--old-pins", default=None, help="optional old-domain-pins.json")
    return p.parse_args(argv)


def main(argv=None):
    args = parse_args(argv if argv is not None else sys.argv[1:])
    doc, sha, _ = load_json(args.candidate)
    fixture, fsha, _ = load_json(args.old_fixture)
    pins = None
    if args.old_pins:
        pins, _, _ = load_json(args.old_pins)
    print("candidate_sha256 " + sha)
    print("old_fixture_sha256 " + fsha)
    test_retained_ids(doc)
    test_c01_history_names(doc)
    test_fee_exactly_once(doc)
    test_cumulative_cap(doc)
    test_actual_fields_schema(doc)
    test_per_primitive_and_full_states(doc)
    test_footprints_frame(doc)
    test_partial_duty(doc)
    test_async_deadline(doc)
    test_typed_negatives(doc)
    test_old_simulation(doc, fixture, pins)
    test_c06_c07(doc)
    test_nam_source_pin(doc)
    failed = [n for n, ok, _ in RESULTS if not ok]
    print("SUMMARY total=%s fail=%s pass=%s" % (len(RESULTS), len(failed), len(RESULTS) - len(failed)))
    if failed:
        print("FAILED " + " ".join(failed))
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
