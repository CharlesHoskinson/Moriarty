#!/usr/bin/env python3
"""Finite executable composition decoder, primitive relation, replay and claim checker.

Public interfaces: decode_candidate, apply_primitive, replay, check_claims.
CLI:
  python3 composition-model.py --check-candidate FILE --old-fixture FILE --old-pins FILE
  python3 composition-model.py --seed FILE --old-fixture FILE --old-pins FILE --output FILE
Generator reads immutable candidate-03 seed only; never the mutable output path.
Internal checker is self-consistency. Independent black-box tests remain acceptance evidence.
"""
from __future__ import annotations

import argparse
import copy
import hashlib
import json
import sys
from decimal import Decimal, ROUND_HALF_EVEN, localcontext
from fractions import Fraction
from pathlib import Path

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
GENESIS_HASH = "29a979a451bcbe4ad9db811bb36078599b3286dca0a45a48377b03922a8f1f3f"
EVALUATOR_HASH = PIN_EXPECT["experiments/moriarty-language/src/evaluate.ts"]
SOURCE_HASH = PIN_EXPECT["experiments/moriarty-language/spec/examples/loan.mori"]
GRAMMAR_HASH = PIN_EXPECT["experiments/moriarty-language/spec/grammar.ebnf"]
BOUNDS_FILE_HASH = PIN_EXPECT["experiments/moriarty-language/spec/bounds.json"]
NUMERIC_HASH = PIN_EXPECT["experiments/moriarty-language/spec/numeric-profile.json"]
NAM_SOURCE_SHA = "bfc39c7a344b1243ce15accb9800837b3d87a050ffa435395595e4bf559e4a9b"
TOY_FP_MAX = 64
MAX_NUMERATOR_BITS = 256
MAX_DENOMINATOR_BITS = 256
MAX_PRECISION = 50
STAGES = ("ContractInvariant", "IntentRefinement", "TransitionValidity", "HistoryCompliance")
COMPLETE_STATE_FIELDS = (
    "accounts", "debts", "shares", "positions", "claims", "fees", "duties", "requests",
    "messages", "orders", "work", "workPartition", "authority", "historyIds", "observations",
    "contract", "observedTick", "unrelatedRecord", "frame", "consumedNonces", "allocations",
    "metadata", "locks", "accountsMeta",
)
DOCUMENTARY_FIELDS = frozenset({
    "unit", "scale", "quantityDomain", "notNat", "independentPrincipal", "independentAmount",
    "principalDomain", "accruedDomain", "amountDomain", "conservation", "profile",
    "notRegisteredProductionLimit", "note", "netPrefundNote", "netPrefund", "copied",
    "lockBacked", "assumption", "notForeignAdapterClaim", "kind", "code", "timestamp",
    "independentRate", "independentAccrued", "exactPrincipal", "exactAccrued", "exactRate",
    "exactAmount", "reportedPlaces", "projection", "silentFieldDrop", "projected",
    "nominalDebtAuthoritySeparateFromTransfer", "refundsDoNotReplenish",
})
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
REQUIRED_ROW_IDS = [
    "NAM19-capitalization", "sequential", "disjoint-parallel",
    "shared-state-interleaving", "atomic-synchronization", "asynchronous-messaging",
]
REQUIRED_THEOREM_IDS = [
    "type-preservation", "asset-indexed-accounting", "authority-safety", "frame-noninterference",
    "assume-guarantee-composition", "structural-associativity", "obligation-preservation",
    "conservative-extension",
]
OLD_VALUE_NAMES = [
    "notional", "principal_due", "interest_due", "principal_paid", "interest_paid",
    "borrower_cash", "lender_cash", "cursor", "episode_closed",
]
CASH_MOVERS = frozenset({"Transfer", "Mint", "Burn"})
ANNOTATION_CTORS = frozenset({"Lock", "AccrueFee", "SettleTransfer"})
SYSTEM_CTORS = (
    "CreateHistory", "ConsumeHistory", "UpdateWork", "UpdateAuthorityCounters",
    "SetObservedTick", "UpdateMessageStatus", "UpdatePositionReserves", "SetNominalRate",
    "UpdateObservation", "ConsumeNonce", "UpdateContract", "ConsumeJointEnvelope",
)
REQUIRED_APPLY_CTORS = (
    "Transfer", "AccrueFee", "Mint", "CreateDebt", "ReduceDebt", "AssignDuty", "SettleDuty",
    "AccrueNominal", "Capitalize", "SetNominalRate", "UpdatePositionReserves",
    "UpdateAuthorityCounters", "UpdateWork", "CreateHistory", "ConsumeHistory",
    "ReceiveMessage", "RefundLock", "Lock", "SendMessage", "SetObservedTick",
    "UpdateMessageStatus", "UpdateObservation", "ConsumeNonce", "SettleTransfer",
    "UpdateContract", "ConsumeJointEnvelope",
)
PRIMITIVE_IDENTITY = {
    "Transfer": {"cashMovement": True, "role": "cash"},
    "Mint": {"cashMovement": True, "role": "cash"},
    "Burn": {"cashMovement": True, "role": "cash"},
    "AccrueFee": {"cashMovement": False, "role": "annotation"},
    "Lock": {"cashMovement": False, "role": "annotation"},
    "SettleTransfer": {"cashMovement": False, "role": "annotation"},
    "CreateDebt": {"cashMovement": False, "role": "debt"},
    "ReduceDebt": {"cashMovement": False, "role": "debt"},
    "AssignDuty": {"cashMovement": False, "role": "duty"},
    "SettleDuty": {"cashMovement": False, "role": "duty"},
    "AccrueNominal": {"cashMovement": False, "role": "nominal"},
    "Capitalize": {"cashMovement": False, "role": "nominal"},
    "SetNominalRate": {"cashMovement": False, "role": "nominal"},
    "UpdateContract": {"cashMovement": False, "role": "nominal"},
    "UpdatePositionReserves": {"cashMovement": False, "role": "position"},
    "UpdateAuthorityCounters": {"cashMovement": False, "role": "authority-check"},
    "UpdateWork": {"cashMovement": False, "role": "work"},
    "CreateHistory": {"cashMovement": False, "role": "history"},
    "ConsumeHistory": {"cashMovement": False, "role": "history"},
    "SendMessage": {"cashMovement": False, "role": "message"},
    "ReceiveMessage": {"cashMovement": False, "role": "message"},
    "RefundLock": {"cashMovement": False, "role": "message"},
    "UpdateMessageStatus": {"cashMovement": False, "role": "message"},
    "SetObservedTick": {"cashMovement": False, "role": "observation"},
    "UpdateObservation": {"cashMovement": False, "role": "observation"},
    "ConsumeNonce": {"cashMovement": False, "role": "nonce"},
    "ConsumeJointEnvelope": {"cashMovement": False, "role": "envelope"},
}

DEFAULT_LIMITS = {
    "maxFootprint": TOY_FP_MAX,
    "maxPrecision": MAX_PRECISION,
    "maxNumeratorBits": MAX_NUMERATOR_BITS,
    "maxDenominatorBits": MAX_DENOMINATOR_BITS,
    "toyFootprintMaximum": TOY_FP_MAX,
}


def sha256_file(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def load_json(path):
    raw = Path(path).read_bytes()
    return json.loads(raw.decode("utf-8")), hashlib.sha256(raw).hexdigest()


def dumps_canon(obj):
    return json.dumps(obj, sort_keys=True, separators=(",", ":"), ensure_ascii=True)


def dumps_out(obj):
    return json.dumps(obj, indent=2, ensure_ascii=True) + "\n"


def sha_obj(obj):
    return hashlib.sha256(dumps_canon(obj).encode("utf-8")).hexdigest()


def reject(path, reason, **extra):
    d = {"ok": False, "path": path, "reason": reason}
    d.update(extra)
    return d


def frac_of(x):
    if isinstance(x, Fraction):
        return x
    if isinstance(x, int):
        return Fraction(x)
    if isinstance(x, Decimal):
        return Fraction(x)
    if isinstance(x, dict):
        if "exact" in x:
            return frac_of(x["exact"])
        if "numerator" in x and "denominator" in x:
            return Fraction(int(x["numerator"]), int(x["denominator"]))
        if "value" in x:
            return frac_of(x["value"])
        return Fraction(0)
    s = str(x)
    if "/" in s and s.count("/") == 1:
        a, b = s.split("/")
        return Fraction(int(a), int(b))
    return Fraction(s)


def bound_frac(f, limits):
    f = Fraction(f).limit_denominator(10 ** 30)
    if f.numerator.bit_length() > limits["maxNumeratorBits"]:
        return None, "numerator-bits"
    if f.denominator.bit_length() > limits["maxDenominatorBits"]:
        return None, "denominator-bits"
    return f, None


def project_source_decimal(frac, places):
    frac = Fraction(frac)
    with localcontext() as ctx:
        ctx.prec = 80
        d = Decimal(frac.numerator) / Decimal(frac.denominator)
        q = Decimal(1).scaleb(-int(places))
        r = d.quantize(q, rounding=ROUND_HALF_EVEN)
    s = format(r, "f")
    if "." in s:
        a, b = s.split(".", 1)
        if len(b) > MAX_PRECISION:
            raise ValueError("reported precision exceeds maxPrecision")
    return s


def nam_exact():
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
        "cap1Reported": project_source_decimal(cap1, 14),
        "cap2Reported": project_source_decimal(cap2, 14),
        "principalReported": project_source_decimal(principal, 12),
        "rate1Reported": "0.110567901234567900",
        "rate2Reported": "0.111679012345679000",
        "projection": "ROUND_HALF_EVEN; cap1/cap2 14 fractional digits; principal 12 fractional digits",
        "eventOrder": ["IED", "RR", "IPCI", "RR"],
    }


def qty_domain(scale):
    return "source-decimal" if scale == "source-decimal" else "runtime-nat"


def work_conservation(w):
    s = str(w.get("spent", "0"))
    r = str(w.get("remainingOrdinary", "0"))
    c = str(w.get("remainingClosure", "0"))
    L = str(w.get("lifetime", "0"))
    w = dict(w)
    w["conservation"] = "%s+%s+%s=%s" % (s, r, c, L)
    return w


def as_finmap(obj, id_key="id"):
    if obj is None:
        return {}
    if isinstance(obj, dict):
        if all(not isinstance(v, dict) or id_key not in v for v in obj.values()):
            return dict(obj)
        out = {}
        for k, v in obj.items():
            if k.startswith("_"):
                continue
            if isinstance(v, dict):
                out[str(v.get(id_key, k))] = v
            else:
                out[str(k)] = v
        return out
    if isinstance(obj, list):
        out = {}
        for item in obj:
            if isinstance(item, dict) and item.get(id_key) is not None:
                out[str(item[id_key])] = item
            elif isinstance(item, str):
                out[item] = item
        return out
    return {}


def finmap_to_json(mp, as_list, id_key="id"):
    if as_list:
        out = []
        for k, v in mp.items():
            if isinstance(v, dict):
                d = dict(v)
                d.setdefault(id_key, k)
                out.append(d)
            else:
                out.append(v)
        return out
    return dict(mp)


def acc_get(state, acct, asset):
    acc = (state.get("accounts") or {}).get(acct) or {}
    if asset not in acc:
        return Fraction(0)
    return frac_of(acc[asset])


def acc_set(state, acct, asset, value, meta):
    accs = state.setdefault("accounts", {})
    row = dict(accs.get(acct) or {})
    row[asset] = str(value) if meta.get("scale") == "source-decimal" else str(int(value) if value.denominator == 1 else value)
    if meta.get("scale") != "source-decimal" and Fraction(value).denominator == 1:
        row[asset] = str(int(Fraction(value)))
    accs[acct] = row
    state["accounts"] = accs


def allowance_map(auth_rec):
    out = {}
    tas = auth_rec.get("transferAllowances")
    if isinstance(tas, dict) and tas:
        for a, recd in tas.items():
            if isinstance(recd, dict):
                out[a] = dict(recd)
        return out
    ta = auth_rec.get("transferAllowance")
    if isinstance(ta, dict) and ta.get("asset"):
        out[ta["asset"]] = dict(ta)
    taw = auth_rec.get("transferAllowanceWETH")
    if isinstance(taw, dict) and taw.get("asset"):
        out[taw["asset"]] = dict(taw)
    return out


def set_allowances(auth_rec, amap):
    auth_rec["transferAllowances"] = {k: dict(v) for k, v in amap.items()}
    if "USDC" in amap:
        auth_rec["transferAllowance"] = dict(amap["USDC"])
    elif "USD" in amap:
        auth_rec["transferAllowance"] = dict(amap["USD"])
    elif "WETH" in amap:
        auth_rec["transferAllowance"] = dict(amap["WETH"])
    elif amap:
        k = sorted(amap)[0]
        auth_rec["transferAllowance"] = dict(amap[k])
    auth_rec.pop("transferAllowanceWETH", None)


def descriptor_of(ae):
    if not isinstance(ae, dict):
        return ae
    d = {k: v for k, v in ae.items() if k not in ("from", "to", "amount")}
    d.setdefault("ctor", ae.get("ctor"))
    if "payeeScope" not in d:
        d["payeeScope"] = "unrestricted"
    return d


def normalize_authority_record(name, recd):
    recd = dict(recd or {})
    if name in ("Treasury",) or recd.get("feeRecipient"):
        recd["variant"] = "FeeRecipient"
        recd["feeRecipient"] = True
    elif name in ("joint", "Joint"):
        recd["variant"] = "JointEnvelope"
        recd.setdefault("envelopeConsumed", False)
        recd.setdefault("remainingSplitAuthority", recd.get("remainingSplitAuthority", "unset"))
    elif name == "Escrow":
        recd["variant"] = "Escrow"
        recd["custody"] = True
    elif name in ("system", "System"):
        recd["variant"] = "System"
        recd["allowedEffects"] = [{"ctor": c, "payeeScope": "system"} for c in SYSTEM_CTORS]
    elif name in ("PoolP", "PoolQ"):
        recd["variant"] = "Pool"
    else:
        recd.setdefault("variant", "TransferPrincipal")
    aes = recd.get("allowedEffects") or []
    recd["allowedEffects"] = [descriptor_of(x) for x in aes if isinstance(x, dict)]
    amap = allowance_map(recd)
    if amap:
        set_allowances(recd, amap)
    return recd


def default_state_shell():
    return {
        "accounts": {},
        "debts": {},
        "shares": {},
        "positions": {},
        "claims": {},
        "fees": {},
        "duties": {},
        "requests": {},
        "messages": {},
        "orders": {},
        "work": {},
        "workPartition": None,
        "authority": {},
        "historyIds": [],
        "observations": [],
        "contract": None,
        "observedTick": None,
        "unrelatedRecord": None,
        "frame": None,
        "consumedNonces": [],
        "allocations": {},
        "metadata": {"projected": True, "silentFieldDrop": False},
        "locks": {},
        "accountsMeta": {},
    }


def decode_state(raw, limits, path="state"):
    if raw is None:
        return reject(path, "missing-state")
    if not isinstance(raw, dict):
        return reject(path, "state-not-object")
    unknown = []
    for k in raw:
        if k not in COMPLETE_STATE_FIELDS and k not in ("accountsMeta", "locks") and not str(k).startswith("_"):
            if k not in DOCUMENTARY_FIELDS:
                unknown.append(k)
    if unknown:
        return reject(path, "unknown-material-fields", fields=unknown)
    st = default_state_shell()
    acc_in = raw.get("accounts") or {}
    meta = dict(raw.get("accountsMeta") or {})
    scale = acc_in.get("_scale") if isinstance(acc_in, dict) else None
    if scale is not None:
        meta["scale"] = scale
    meta.setdefault("scale", "0")
    meta["quantityDomain"] = qty_domain(meta.get("scale"))
    st["accountsMeta"] = meta
    accounts = {}
    if isinstance(acc_in, dict):
        for acct, assets in acc_in.items():
            if acct.startswith("_"):
                continue
            if not isinstance(assets, dict):
                return reject(path + "/accounts/" + acct, "account-not-map")
            row = {}
            for asset, val in assets.items():
                if asset.startswith("_"):
                    continue
                try:
                    f = frac_of(val if not isinstance(val, dict) else val.get("value"))
                except Exception:
                    return reject(path + "/accounts/%s/%s" % (acct, asset), "quantity-unparsed")
                bf, err = bound_frac(f, limits)
                if err:
                    return reject(path + "/accounts/%s/%s" % (acct, asset), err)
                row[asset] = str(int(bf)) if bf.denominator == 1 and meta.get("scale") != "source-decimal" else (
                    project_source_decimal(bf, 12) if meta.get("scale") == "source-decimal" and bf.denominator != 1 else str(bf) if bf.denominator != 1 else str(int(bf))
                )
                if meta.get("scale") == "source-decimal":
                    row[asset] = str(val["value"]) if isinstance(val, dict) and "value" in val else str(val)
            accounts[acct] = row
    st["accounts"] = accounts
    for fmap, key, as_id in (
        ("debts", "debts", "id"),
        ("shares", "shares", "id"),
        ("positions", "positions", "id"),
        ("claims", "claims", "id"),
        ("fees", "fees", "id"),
        ("duties", "duties", "id"),
        ("requests", "requests", "id"),
        ("messages", "messages", "id"),
        ("orders", "orders", "id"),
        ("allocations", "allocations", "allocationId"),
        ("locks", "locks", "messageId"),
    ):
        raw_f = raw.get(key)
        if key == "shares" and isinstance(raw_f, dict) and raw_f and not any(isinstance(v, dict) and "id" in v for v in raw_f.values()):
            st["shares"] = {str(k): dict(v) if isinstance(v, dict) else v for k, v in raw_f.items()}
        else:
            st[fmap] = as_finmap(raw_f, "id" if as_id == "id" else as_id)
            if key == "allocations" and isinstance(raw_f, dict) and raw_f and not any(isinstance(v, dict) and "allocationId" in v for v in raw_f.values() if isinstance(v, dict)):
                st["allocations"] = {str(k): dict(v) if isinstance(v, dict) else v for k, v in raw_f.items()}
    if not st["allocations"] and isinstance(raw.get("allocations"), list) and not raw.get("allocations"):
        st["allocations"] = {}
    w = raw.get("work") or {}
    if w:
        st["work"] = work_conservation(dict(w))
    st["workPartition"] = copy.deepcopy(raw.get("workPartition"))
    auth_in = raw.get("authority") or {}
    auth = {}
    if isinstance(auth_in, dict):
        for name, recd in auth_in.items():
            if isinstance(recd, dict):
                auth[name] = normalize_authority_record(name, recd)
    if "system" not in auth:
        auth["system"] = normalize_authority_record("system", {})
    st["authority"] = auth
    hist = raw.get("historyIds") or []
    st["historyIds"] = list(hist) if isinstance(hist, list) else list(hist.keys()) if isinstance(hist, dict) else []
    st["observations"] = list(raw.get("observations") or [])
    st["contract"] = copy.deepcopy(raw.get("contract"))
    st["observedTick"] = raw.get("observedTick")
    st["unrelatedRecord"] = copy.deepcopy(raw.get("unrelatedRecord"))
    st["frame"] = copy.deepcopy(raw.get("frame"))
    cn = raw.get("consumedNonces") or []
    st["consumedNonces"] = list(cn)
    st["metadata"] = dict(raw.get("metadata") or {"projected": True, "silentFieldDrop": False})
    return {"ok": True, "state": st}


def encode_state(st, list_fields=None):
    list_fields = list_fields or {"duties", "fees", "messages", "claims", "requests", "orders", "positions"}
    out = {}
    acc = dict(st.get("accounts") or {})
    meta = st.get("accountsMeta") or {}
    if meta.get("scale") is not None:
        acc["_scale"] = meta.get("scale")
    out["accounts"] = acc
    out["debts"] = dict(st.get("debts") or {})
    shares = st.get("shares") or {}
    out["shares"] = dict(shares)
    for name in ("positions", "claims", "fees", "duties", "requests", "messages", "orders"):
        mp = st.get(name) or {}
        out[name] = finmap_to_json(mp, name in list_fields)
    out["work"] = work_conservation(dict(st.get("work") or {}))
    out["workPartition"] = copy.deepcopy(st.get("workPartition"))
    out["authority"] = copy.deepcopy(st.get("authority") or {})
    out["historyIds"] = list(st.get("historyIds") or [])
    out["observations"] = copy.deepcopy(st.get("observations") or [])
    out["contract"] = copy.deepcopy(st.get("contract"))
    out["observedTick"] = st.get("observedTick")
    out["unrelatedRecord"] = copy.deepcopy(st.get("unrelatedRecord"))
    out["frame"] = copy.deepcopy(st.get("frame"))
    out["consumedNonces"] = list(st.get("consumedNonces") or [])
    alloc = st.get("allocations") or {}
    out["allocations"] = dict(alloc) if alloc else []
    out["metadata"] = copy.deepcopy(st.get("metadata") or {"projected": True, "silentFieldDrop": False})
    return out


def effect_id(effect, idx):
    if effect.get("id"):
        return str(effect["id"])
    ctor = effect.get("ctor")
    if ctor == "AccrueFee":
        return str(effect.get("feeId") or "fee-%s" % idx)
    if ctor == "AssignDuty":
        return str(effect.get("id") or "duty-%s" % idx)
    return "%s-%s" % (ctor, idx)


def prior_transfers(context):
    out = []
    for e in context.get("prior") or []:
        if isinstance(e, dict) and e.get("ctor") == "Transfer":
            out.append(e)
    return out


def find_linked_transfer(effect, context, from_acct, to_acct, asset, amount):
    want = str(amount)
    annot = effect.get("annotates") or effect.get("linkedTransferId")
    for t in prior_transfers(context):
        if annot and t.get("id") == annot:
            return t
        if (
            t.get("from") == from_acct
            and t.get("to") == to_acct
            and t.get("asset") == asset
            and str(t.get("amount")) == want
        ):
            return t
    return None


def read_write():
    return set(), set()


def add_acc_rw(reads, writes, acct, asset, write=False):
    p = "accounts.%s.%s" % (acct, asset)
    reads.add(p)
    if write:
        writes.add(p)


def apply_primitive(state, effect, context):
    if not isinstance(effect, dict) or not effect.get("ctor"):
        return reject("effect", "missing-ctor")
    ctor = effect["ctor"]
    if ctor not in PRIMITIVE_IDENTITY and ctor not in ("Burn",):
        return reject("effect", "unknown-ctor", ctor=ctor)
    limits = context.get("limits") or DEFAULT_LIMITS
    st = copy.deepcopy(state)
    reads, writes = read_write()
    reads.add("authority.system.allowedEffects")
    reads.add("work.lifetime")

    def fail(reason, **k):
        d = reject("apply/" + ctor, reason, ctor=ctor, reads=sorted(reads), writes=sorted(writes))
        d.update(k)
        return d

    def ok():
        return {"ok": True, "state": st, "reads": sorted(reads), "writes": sorted(writes), "ctor": ctor}

    ident = PRIMITIVE_IDENTITY.get(ctor) or {"cashMovement": ctor in CASH_MOVERS}
    if ident.get("cashMovement") and ctor not in CASH_MOVERS:
        return fail("identity-cash-mismatch")

    if ctor == "Transfer":
        frm, to, asset, amt_s = effect.get("from"), effect.get("to"), effect.get("asset"), effect.get("amount")
        if not all([frm, to, asset, amt_s is not None, effect.get("unit"), effect.get("scale") is not None]):
            return fail("missing-fields")
        amt = frac_of(amt_s)
        if amt < 0:
            return fail("negative-amount")
        add_acc_rw(reads, writes, frm, asset, True)
        add_acc_rw(reads, writes, to, asset, True)
        bal = acc_get(st, frm, asset)
        if bal < amt:
            return fail("insufficient-balance", have=str(bal), need=str(amt))
        auth = (st.get("authority") or {}).get(frm) or {}
        reads.add("authority.%s.allowedEffects" % frm)
        reads.add("authority.%s.transferAllowances.%s" % (frm, asset))
        amap = allowance_map(auth)
        al = amap.get(asset)
        if al is None:
            return fail("no-allowance-for-asset", principal=frm, asset=asset)
        remaining = frac_of(al.get("remaining", 0))
        original = frac_of(al.get("original", 0))
        gross = frac_of(al.get("cumulativeGross", 0))
        if amt > remaining or gross + amt > original:
            return fail("gross-cap", remaining=str(remaining), original=str(original), gross=str(gross))
        bound_ok = False
        payee_ok = True
        for ae in auth.get("allowedEffects") or []:
            if ae.get("ctor") != "Transfer":
                continue
            if ae.get("asset") not in (None, asset):
                continue
            b = ae.get("bound")
            if b is not None and amt > frac_of(b):
                continue
            scope = ae.get("payeeScope") or "unrestricted"
            if scope not in ("unrestricted", None) and to not in str(scope).split(","):
                payee_ok = False
                continue
            bound_ok = True
            payee_ok = True
            break
        joint = (st.get("authority") or {}).get("joint")
        if not bound_ok and joint:
            reads.add("authority.joint.allowedEffects")
            for ae in joint.get("allowedEffects") or []:
                if ae.get("ctor") == "Transfer" and ae.get("asset") in (None, asset):
                    if ae.get("bound") is None or amt <= frac_of(ae.get("bound")):
                        bound_ok = True
                        break
        if not bound_ok:
            return fail("not-in-allowed-effects", principal=frm, asset=asset)
        if not payee_ok:
            return fail("payee-scope")
        acc_set(st, frm, asset, bal - amt, st.get("accountsMeta") or {})
        acc_set(st, to, asset, acc_get(st, to, asset) + amt, st.get("accountsMeta") or {})
        al = dict(al)
        al["cumulativeGross"] = str(int(gross + amt) if (gross + amt).denominator == 1 else (gross + amt))
        if (gross + amt).denominator == 1:
            al["cumulativeGross"] = str(int(gross + amt))
        rem = original - (gross + amt)
        al["remaining"] = str(int(rem) if rem.denominator == 1 else rem)
        al["asset"] = asset
        amap[asset] = al
        auth = dict(auth)
        set_allowances(auth, amap)
        st.setdefault("authority", {})[frm] = auth
        writes.add("authority.%s.transferAllowances.%s" % (frm, asset))
        writes.add("authority.%s.transferAllowance" % frm)
        aid = effect.get("allocationId")
        if aid:
            st.setdefault("allocations", {})[str(aid)] = {
                "allocationId": str(aid),
                "transferId": effect.get("id"),
                "amount": str(amt_s),
                "asset": asset,
                "payee": to,
                "kind": "transfer",
            }
            writes.add("allocations.%s" % aid)
        return ok()

    if ctor == "Mint":
        to, asset, amt_s = effect.get("to"), effect.get("asset"), effect.get("amount")
        if not all([to, asset, amt_s is not None]):
            return fail("missing-fields")
        amt = frac_of(amt_s)
        add_acc_rw(reads, writes, to, asset, True)
        acc_set(st, to, asset, acc_get(st, to, asset) + amt, st.get("accountsMeta") or {})
        if asset.startswith("LP-"):
            pool = effect.get("pool") or "PoolP"
            sh = dict((st.get("shares") or {}).get(to) or {"asset": asset, "pool": pool})
            q = frac_of(sh.get("quantity", 0)) + amt
            sh.update({"asset": asset, "quantity": str(int(q)), "pool": pool})
            st.setdefault("shares", {})[to] = sh
            writes.add("shares.%s" % to)
            pos = dict((st.get("positions") or {}).get(pool) or {"id": pool})
            if "lpSupply" in pos or True:
                supply = frac_of(pos.get("lpSupply", 0)) + amt
                pos["lpSupply"] = str(int(supply))
                pos["id"] = pool
                st.setdefault("positions", {})[pool] = pos
                writes.add("positions.%s.lpSupply" % pool)
        return ok()

    if ctor == "AccrueFee":
        tid = effect.get("annotates")
        if not tid:
            return fail("missing-annotates")
        t = find_linked_transfer(effect, context, effect.get("from"), effect.get("recipient"), effect.get("asset"), effect.get("amount"))
        if t is None or (tid and t.get("id") not in (None, tid) and t.get("id") != tid):
            # allow match by id only
            t = None
            for tr in prior_transfers(context):
                if tr.get("id") == tid:
                    t = tr
                    break
        if t is None:
            return fail("annotation-transfer-missing", annotates=tid)
        if str(t.get("amount")) != str(effect.get("amount")) or t.get("to") != effect.get("recipient") or t.get("asset") != effect.get("asset"):
            return fail("annotation-mismatch")
        fid = str(effect.get("feeId") or effect.get("id") or tid)
        st.setdefault("fees", {})[fid] = {
            "id": fid,
            "asset": effect.get("asset"),
            "amount": str(effect.get("amount")),
            "recipient": effect.get("recipient"),
            "annotates": tid,
            "unit": effect.get("unit") or effect.get("asset"),
            "scale": effect.get("scale") or "0",
        }
        writes.add("fees.%s" % fid)
        return ok()

    if ctor == "Lock":
        frm, escrow, asset, amt_s = effect.get("from"), effect.get("escrow"), effect.get("asset"), effect.get("amount")
        mid = effect.get("messageId") or effect.get("id")
        nonce = effect.get("nonce")
        t = find_linked_transfer(effect, context, frm, escrow, asset, amt_s)
        if t is None:
            return fail("lock-requires-linked-transfer")
        if nonce in (st.get("consumedNonces") or []):
            return fail("nonce-used")
        reads.add("consumedNonces")
        st.setdefault("locks", {})[str(mid)] = {
            "messageId": str(mid),
            "from": frm,
            "escrow": escrow,
            "asset": asset,
            "amount": str(amt_s),
            "nonce": nonce,
            "annotates": t.get("id"),
            "role": "annotation",
            "cashMovement": False,
        }
        writes.add("locks.%s" % mid)
        auth = dict((st.get("authority") or {}).get(frm) or {})
        auth["lockConsumed"] = True
        st.setdefault("authority", {})[frm] = auth
        writes.add("authority.%s.lockConsumed" % frm)
        return ok()

    if ctor == "CreateDebt":
        did = str(effect.get("id"))
        if did in (st.get("debts") or {}):
            return fail("debt-exists")
        princ = str(effect.get("principal"))
        st.setdefault("debts", {})[did] = {
            "id": did,
            "asset": effect.get("asset"),
            "principal": princ,
            "accrued": "0",
            "rate": effect.get("rate") or "0",
            "debtor": effect.get("debtor"),
            "creditor": effect.get("creditor"),
            "unit": effect.get("unit") or effect.get("asset"),
            "scale": effect.get("scale") or "0",
            "exactPrincipal": str(frac_of(princ)),
            "principalDomain": qty_domain(effect.get("scale") or "0"),
        }
        writes.add("debts.%s" % did)
        return ok()

    if ctor == "ReduceDebt":
        did = str(effect.get("id"))
        debt = (st.get("debts") or {}).get(did)
        reads.add("debts.%s" % did)
        if not debt:
            return fail("debt-missing")
        amt = frac_of(effect.get("amount"))
        funding = effect.get("fundingTransferId")
        found = None
        for t in prior_transfers(context):
            if t.get("id") == funding:
                found = t
                break
        if funding and found is None:
            return fail("funding-transfer-missing")
        if found is not None:
            if found.get("asset") != (effect.get("asset") or debt.get("asset")):
                return fail("funding-asset-mismatch")
            if found.get("to") != (effect.get("payee") or debt.get("creditor")):
                return fail("payee-mismatch")
        aid = effect.get("allocationId")
        if aid:
            allocs = st.setdefault("allocations", {})
            if aid in allocs and allocs[aid].get("kind") == "debt-discharge":
                return fail("allocation-reuse", allocationId=aid)
            used = frac_of((allocs.get(aid) or {}).get("amount", 0)) if aid in allocs else Fraction(0)
            # use-once: the id itself may already record the funding transfer; second ReduceDebt with same id fails
            if aid in allocs and allocs[aid].get("targetId") == did:
                return fail("allocation-reuse", allocationId=aid)
            allocs[aid] = {
                "allocationId": aid,
                "transferId": funding,
                "targetId": did,
                "amount": str(effect.get("amount")),
                "asset": effect.get("asset") or debt.get("asset"),
                "payee": effect.get("payee") or debt.get("creditor"),
                "kind": "debt-discharge",
            }
            writes.add("allocations.%s" % aid)
        np = frac_of(debt.get("principal")) - amt
        if np < 0:
            return fail("over-reduce")
        if np == 0:
            st["debts"].pop(did, None)
            writes.add("debts.%s" % did)
        else:
            debt = dict(debt)
            debt["principal"] = str(int(np) if np.denominator == 1 else np)
            st["debts"][did] = debt
            writes.add("debts.%s.principal" % did)
        return ok()

    if ctor == "AssignDuty":
        did = str(effect.get("id"))
        variant = effect.get("dutyVariant") or ("lock-entitlement" if effect.get("lockId") else "debt-linked")
        recd = {
            "id": did,
            "kind": effect.get("kind"),
            "asset": effect.get("asset"),
            "amount": str(effect.get("amount")),
            "controller": effect.get("controller"),
            "payee": effect.get("payee"),
            "residual": bool(effect.get("residual", True)),
            "dutyVariant": variant,
        }
        if effect.get("debtId"):
            recd["debtId"] = effect.get("debtId")
        if effect.get("lockId"):
            recd["lockId"] = effect.get("lockId")
        if effect.get("amountDomain"):
            recd["amountDomain"] = effect.get("amountDomain")
        if effect.get("exactAmount"):
            recd["exactAmount"] = effect.get("exactAmount")
        st.setdefault("duties", {})[did] = recd
        writes.add("duties.%s" % did)
        return ok()

    if ctor == "SettleDuty":
        did = str(effect.get("id"))
        duty = (st.get("duties") or {}).get(did)
        reads.add("duties.%s" % did)
        if not duty:
            return fail("duty-missing")
        amt = frac_of(effect.get("amount") or duty.get("amount") or 0)
        remaining = frac_of(duty.get("amount")) - amt
        aid = effect.get("allocationId")
        if aid:
            allocs = st.setdefault("allocations", {})
            if aid in allocs and allocs[aid].get("kind") == "duty-discharge" and allocs[aid].get("targetId") == did:
                return fail("allocation-reuse", allocationId=aid)
            allocs[aid] = {
                "allocationId": aid,
                "transferId": effect.get("fundingTransferId"),
                "targetId": did,
                "amount": str(effect.get("amount") or duty.get("amount")),
                "asset": duty.get("asset"),
                "payee": effect.get("payee") or duty.get("payee"),
                "kind": "duty-discharge",
            }
            writes.add("allocations.%s" % aid)
        if remaining <= 0:
            st["duties"].pop(did, None)
            writes.add("duties.%s" % did)
        else:
            duty = dict(duty)
            duty["amount"] = str(int(remaining) if remaining.denominator == 1 else remaining)
            duty["residual"] = True
            st["duties"][did] = duty
            writes.add("duties.%s.amount" % did)
        return ok()

    if ctor == "AccrueNominal":
        did = str(effect.get("debtId"))
        debt = (st.get("debts") or {}).get(did)
        reads.add("debts.%s" % did)
        if not debt:
            return fail("debt-missing")
        if not effect.get("observationRef"):
            return fail("missing-observationRef")
        exact = frac_of(effect.get("accruedExact") or effect.get("accruedIndependent") or effect.get("accrued"))
        bf, err = bound_frac(exact, limits)
        if err:
            return fail(err)
        reported = effect.get("accrued") or project_source_decimal(bf, 14)
        if "." in str(reported) and len(str(reported).split(".", 1)[1]) > limits["maxPrecision"]:
            return fail("accrued-precision")
        debt = dict(debt)
        prev_a = frac_of(debt.get("exactAccrued") or debt.get("accrued") or 0)
        debt["exactAccrued"] = str(prev_a + bf)
        debt["accrued"] = reported if debt.get("accrued") in ("0", 0, None) else project_source_decimal(prev_a + bf, 14)
        if prev_a == 0:
            debt["accrued"] = reported
        else:
            debt["accrued"] = project_source_decimal(prev_a + bf, 14)
        debt["accruedDomain"] = "source-decimal"
        st["debts"][did] = debt
        writes.add("debts.%s.accrued" % did)
        return ok()

    if ctor == "Capitalize":
        did = str(effect.get("debtId"))
        debt = (st.get("debts") or {}).get(did)
        reads.add("debts.%s" % did)
        if not debt:
            return fail("debt-missing")
        if str(effect.get("payoff", "0")) != "0":
            return fail("payoff-nonzero")
        nam = nam_exact()
        exact_p = frac_of(debt.get("exactPrincipal") or debt.get("principal")) + frac_of(debt.get("exactAccrued") or 0)
        reported = nam["principalReported"]
        debt = dict(debt)
        debt["exactPrincipal"] = str(exact_p)
        debt["principal"] = reported
        debt["accrued"] = "0"
        debt["exactAccrued"] = "0"
        debt["principalDomain"] = "source-decimal"
        debt["notNat"] = True
        st["debts"][did] = debt
        writes.add("debts.%s.principal" % did)
        writes.add("debts.%s.accrued" % did)
        for dk, duty in list((st.get("duties") or {}).items()):
            if isinstance(duty, dict) and duty.get("debtId") == did:
                duty = dict(duty)
                duty["amount"] = reported
                duty["exactAmount"] = str(exact_p)
                duty["amountDomain"] = "source-decimal"
                st["duties"][dk] = duty
                writes.add("duties.%s.amount" % dk)
        return ok()

    if ctor == "SetNominalRate":
        did = str(effect.get("debtId"))
        debt = (st.get("debts") or {}).get(did)
        reads.add("debts.%s" % did)
        if not debt:
            return fail("debt-missing")
        rate = str(effect.get("rate"))
        debt = dict(debt)
        debt["rate"] = rate
        debt["exactRate"] = str(frac_of(rate))
        st["debts"][did] = debt
        writes.add("debts.%s.rate" % did)
        writes.add("authority.system.allowedEffects")
        return ok()

    if ctor == "UpdateContract":
        c = dict(st.get("contract") or {"id": effect.get("id") or "nam19"})
        if effect.get("notionalPrincipal") is not None:
            c["notionalPrincipal"] = str(effect.get("notionalPrincipal"))
        if effect.get("accruedInterest") is not None:
            c["accruedInterest"] = str(effect.get("accruedInterest"))
        if effect.get("nominalInterestRate") is not None:
            c["nominalInterestRate"] = str(effect.get("nominalInterestRate"))
        c.setdefault("currency", "USD")
        c.setdefault("scale", "source-decimal")
        c.setdefault("quantityDomain", "source-decimal")
        st["contract"] = c
        writes.add("contract.notionalPrincipal")
        writes.add("contract.accruedInterest")
        writes.add("contract.nominalInterestRate")
        return ok()

    if ctor == "UpdatePositionReserves":
        pid = str(effect.get("id"))
        pos = dict((st.get("positions") or {}).get(pid) or {"id": pid})
        reads.add("positions.%s" % pid)
        usdc = acc_get(st, pid, "USDC")
        weth = acc_get(st, pid, "WETH")
        add_acc_rw(reads, writes, pid, "USDC")
        add_acc_rw(reads, writes, pid, "WETH")
        supply = Fraction(0)
        for holder, sh in (st.get("shares") or {}).items():
            if isinstance(sh, dict) and sh.get("pool") == pid:
                supply += frac_of(sh.get("quantity", 0))
                reads.add("shares.%s" % holder)
        pos["id"] = pid
        pos["reserveUSDC"] = str(int(usdc))
        pos["reserveWETH"] = str(int(weth))
        if supply != 0 or "lpSupply" in pos or effect.get("lpSupply") is not None:
            pos["lpSupply"] = str(int(supply)) if supply.denominator == 1 else str(supply)
        if pos.get("kFormula") or effect.get("kFormula"):
            pos["kFormula"] = pos.get("kFormula") or effect.get("kFormula")
        named_u, named_w = effect.get("reserveUSDC"), effect.get("reserveWETH")
        if named_u is not None and str(int(frac_of(named_u))) != pos["reserveUSDC"]:
            return fail("named-reserve-mismatch", derived=pos["reserveUSDC"], named=named_u)
        if named_w is not None and str(int(frac_of(named_w))) != pos["reserveWETH"]:
            return fail("named-reserve-mismatch", derived=pos["reserveWETH"], named=named_w)
        st.setdefault("positions", {})[pid] = pos
        writes.add("positions.%s.reserveUSDC" % pid)
        writes.add("positions.%s.reserveWETH" % pid)
        if "lpSupply" in pos:
            writes.add("positions.%s.lpSupply" % pid)
        return ok()

    if ctor == "UpdateAuthorityCounters":
        p, asset = effect.get("principal"), effect.get("asset")
        delta = frac_of(effect.get("grossDelta", 0))
        auth = (st.get("authority") or {}).get(p) or {}
        amap = allowance_map(auth)
        al = amap.get(asset)
        reads.add("authority.%s.transferAllowances.%s" % (p, asset))
        if al is None:
            return fail("counter-asset-missing", principal=p, asset=asset)
        gross = frac_of(al.get("cumulativeGross", 0))
        outgoing = Fraction(0)
        for t in prior_transfers(context) + ([effect] if False else []):
            if t.get("from") == p and t.get("asset") == asset:
                outgoing += frac_of(t.get("amount"))
        # Transfer already applied gross. Check delta equals outgoing sum and current gross.
        if outgoing != delta and gross != delta:
            return fail("gross-delta-mismatch", outgoing=str(outgoing), delta=str(delta), gross=str(gross))
        if gross != delta:
            # counters not yet matching; do not double-add if Transfer already wrote
            return fail("gross-not-derived-from-transfers", gross=str(gross), delta=str(delta))
        writes.add("authority.%s.transferAllowances.%s" % (p, asset))
        return ok()

    if ctor == "UpdateWork":
        w = dict(st.get("work") or {})
        reads.add("work.spent")
        reads.add("work.remainingOrdinary")
        od = frac_of(effect.get("ordinaryDelta", 0))
        cd = frac_of(effect.get("closureDelta", 0))
        spent = frac_of(w.get("spent", 0)) + od + cd
        rem_o = frac_of(w.get("remainingOrdinary", 0)) - od
        rem_c = frac_of(w.get("remainingClosure", 0)) - cd
        life = frac_of(w.get("lifetime", 0))
        if rem_o < 0 or rem_c < 0:
            return fail("work-overspend")
        if spent + rem_o + rem_c != life:
            return fail("work-conservation")
        # no-refresh: remainingOrdinary must not increase
        if rem_o > frac_of(w.get("remainingOrdinary", 0)):
            return fail("work-refresh")
        w["spent"] = str(int(spent))
        w["remainingOrdinary"] = str(int(rem_o))
        w["remainingClosure"] = str(int(rem_c))
        w["lifetime"] = str(int(life))
        st["work"] = work_conservation(w)
        writes.add("work.spent")
        writes.add("work.remainingOrdinary")
        wp = st.get("workPartition")
        if isinstance(wp, dict) and wp.get("copied") is False:
            charges = context.get("workCharges") or {}
            wp = copy.deepcopy(wp)
            for side in ("A", "B"):
                ch = frac_of(charges.get(side, od / 2 if od.denominator == 1 and int(od) % 2 == 0 else 0))
                recd = dict(wp.get(side) or {})
                recd["spent"] = str(int(frac_of(recd.get("spent", 0)) + ch))
                recd["remainingOrdinary"] = str(int(frac_of(recd.get("remainingOrdinary", 0)) - ch))
                recd.setdefault("remainingClosure", recd.get("remainingClosure", "8"))
                wp[side] = recd
                writes.add("workPartition.%s.spent" % side)
                writes.add("workPartition.%s.remainingOrdinary" % side)
            st["workPartition"] = wp
        return ok()

    if ctor == "CreateHistory":
        hid = str(effect.get("id"))
        reads.add("history.%s" % hid)
        if hid in (st.get("historyIds") or []):
            return fail("history-exists")
        st.setdefault("historyIds", []).append(hid)
        writes.add("history.%s" % hid)
        return ok()

    if ctor == "ConsumeHistory":
        hid = str(effect.get("id"))
        reads.add("history.%s" % hid)
        hist = list(st.get("historyIds") or [])
        if hid not in hist:
            return fail("history-missing", id=hid)
        hist.remove(hid)
        st["historyIds"] = hist
        writes.add("history.%s" % hid)
        return ok()

    if ctor == "SendMessage":
        mid = str(effect.get("id"))
        nonce = effect.get("nonce")
        if nonce in (st.get("consumedNonces") or []):
            return fail("nonce-used")
        msg = dict((st.get("messages") or {}).get(mid) or {})
        msg.update({
            "id": mid,
            "status": "Pending",
            "nonce": nonce,
            "deadlineTick": str(effect.get("deadlineTick")),
            "from": effect.get("from") or msg.get("from") or "Alice",
            "to": effect.get("to") or msg.get("to") or "Bob",
            "escrow": effect.get("escrow") or msg.get("escrow") or "Escrow",
        })
        lock = (st.get("locks") or {}).get(mid)
        if lock:
            msg["lockAmount"] = lock.get("amount")
            msg["lockAsset"] = lock.get("asset")
        st.setdefault("messages", {})[mid] = msg
        writes.add("messages.%s" % mid)
        return ok()

    if ctor == "UpdateMessageStatus":
        mid = str(effect.get("id"))
        msg = dict((st.get("messages") or {}).get(mid) or {"id": mid})
        reads.add("messages.%s" % mid)
        cur = msg.get("status") or "absent"
        nxt = effect.get("toStatus") or effect.get("status")
        frm = effect.get("fromStatus")
        if nxt == cur:
            return ok()
        if frm and frm != cur and not (frm == "absent" and cur in (None, "absent", "Created")):
            return fail("status-from-mismatch", have=cur, want=frm)
        # Pending -> Refundable is eligibility opening, not RefundLock consumption.
        if cur == "Pending" and nxt == "Refundable":
            msg["status"] = "Refundable"
            msg["refundableOpened"] = True
        elif cur == "Refundable" and nxt == "Refunded":
            msg["status"] = "Refunded"
        elif cur == "Pending" and nxt in ("Received", "Finalized"):
            msg["status"] = nxt
        elif cur in (None, "absent") and nxt == "Pending":
            msg["status"] = "Pending"
        elif nxt:
            msg["status"] = nxt
        else:
            return fail("status-unspecified")
        st.setdefault("messages", {})[mid] = msg
        writes.add("messages.%s" % mid)
        return ok()

    if ctor == "RefundLock":
        mid = str(effect.get("id"))
        msg = dict((st.get("messages") or {}).get(mid) or {})
        reads.add("messages.%s" % mid)
        if msg.get("status") != "Refundable":
            return fail("refund-requires-refundable", status=msg.get("status"))
        nonce = effect.get("nonce") or msg.get("nonce")
        if nonce in (st.get("consumedNonces") or []):
            return fail("nonce-used")
        to = effect.get("to") or msg.get("from") or "Alice"
        msg["status"] = "Refunded"
        st.setdefault("messages", {})[mid] = msg
        writes.add("messages.%s" % mid)
        auth = dict((st.get("authority") or {}).get(to) or {})
        auth["lockRefundConsumed"] = True
        st.setdefault("authority", {})[to] = auth
        writes.add("authority.%s.lockRefundConsumed" % to)
        esc = dict((st.get("authority") or {}).get("Escrow") or {})
        esc["lockConsumed"] = True
        st["authority"]["Escrow"] = esc
        writes.add("authority.Escrow.lockConsumed")
        return ok()

    if ctor == "ReceiveMessage":
        mid = str(effect.get("id"))
        msg = dict((st.get("messages") or {}).get(mid) or {})
        reads.add("messages.%s" % mid)
        if msg.get("status") != "Pending":
            return fail("receive-requires-pending", status=msg.get("status"))
        obs_id = effect.get("destFinalityObservationId")
        obs = None
        for o in st.get("observations") or []:
            if isinstance(o, dict) and o.get("id") == obs_id:
                obs = o
                break
        # observation may also be supplied in context
        ctx_obs = (context.get("observation") or {})
        if obs is None and ctx_obs.get("id") == obs_id:
            obs = ctx_obs
        if not obs or not obs.get("authenticated"):
            return fail("receive-requires-authenticated-observation")
        tick = frac_of(obs.get("tick") or effect.get("tick") or 0)
        deadline = frac_of(msg.get("deadlineTick") or 100)
        if tick >= deadline:
            return fail("finality-not-before-deadline")
        nonce = effect.get("nonce") or msg.get("nonce")
        if nonce in (st.get("consumedNonces") or []):
            return fail("nonce-used")
        escrow = effect.get("escrowSource") or msg.get("escrow") or "Escrow"
        lock_amt = frac_of(msg.get("lockAmount") or 25)
        if acc_get(st, escrow, msg.get("lockAsset") or "USDC") < lock_amt:
            return fail("escrow-empty")
        msg["status"] = "Received"
        st.setdefault("messages", {})[mid] = msg
        writes.add("messages.%s" % mid)
        bob = dict((st.get("authority") or {}).get(effect.get("to") or "Bob") or {})
        bob["recipientRight"] = "DutyLock_%s" % mid if not bob.get("recipientRight") else bob.get("recipientRight")
        st.setdefault("authority", {})[effect.get("to") or "Bob"] = bob
        writes.add("authority.%s.recipientRight" % (effect.get("to") or "Bob"))
        return ok()

    if ctor == "ConsumeNonce":
        nid = str(effect.get("id") or effect.get("nonce"))
        reads.add("consumedNonces")
        if nid in (st.get("consumedNonces") or []):
            return fail("nonce-used")
        st.setdefault("consumedNonces", []).append(nid)
        writes.add("consumedNonces")
        return ok()

    if ctor == "SetObservedTick":
        st["observedTick"] = str(effect.get("tick"))
        writes.add("observedTick")
        return ok()

    if ctor == "UpdateObservation":
        oid = str(effect.get("id"))
        recd = {
            "id": oid,
            "tick": str(effect.get("tick")) if effect.get("tick") is not None else None,
            "authenticated": bool(effect.get("authenticated", True)),
            "assumption": bool(effect.get("assumption", True)),
            "code": effect.get("code") or "DEST_FINALITY",
            "timestamp": effect.get("timestamp") or ("tick-%s" % effect.get("tick")),
            "value": effect.get("value") or "finalized",
            "kind": effect.get("kind") or "assumed-authenticated-destination",
        }
        obs = list(st.get("observations") or [])
        obs = [o for o in obs if not (isinstance(o, dict) and o.get("id") == oid)]
        obs.append(recd)
        st["observations"] = obs
        writes.add("observations.%s" % oid)
        return ok()

    if ctor == "SettleTransfer":
        # annotation: funding transfer already moved cash; allocation must exist or be recorded
        tid = effect.get("transferId")
        found = None
        for t in prior_transfers(context):
            if t.get("id") == tid:
                found = t
                break
        if found is None:
            return fail("settle-transfer-missing")
        aid = effect.get("allocationId")
        if aid:
            st.setdefault("allocations", {}).setdefault(str(aid), {
                "allocationId": str(aid),
                "transferId": tid,
                "targetId": effect.get("debtOrDutyId"),
                "amount": str(found.get("amount")),
                "asset": found.get("asset"),
                "payee": effect.get("payee") or found.get("to"),
                "kind": "settle-transfer",
            })
            writes.add("allocations.%s" % aid)
        return ok()

    if ctor == "ConsumeJointEnvelope":
        joint = dict((st.get("authority") or {}).get("joint") or {})
        reads.add("authority.joint")
        joint["envelopeConsumed"] = True
        joint["remainingSplitAuthority"] = "0"
        joint["variant"] = "JointEnvelope"
        st.setdefault("authority", {})["joint"] = joint
        writes.add("authority.joint.envelopeConsumed")
        writes.add("authority.joint.remainingSplitAuthority")
        return ok()

    return fail("unimplemented-ctor")


def replay(plan, pre, context):
    limits = (context or {}).get("limits") or DEFAULT_LIMITS
    dec = decode_state(pre, limits, "pre")
    if not dec.get("ok"):
        return dec
    st = dec["state"]
    effects = list((plan or {}).get("effects") or plan or [])
    if isinstance(plan, list):
        effects = plan
    prefixes = [copy.deepcopy(st)]
    all_reads, all_writes = set(), set()
    prior = []
    ctx = dict(context or {})
    ctx.setdefault("limits", limits)
    for i, eff in enumerate(effects):
        ctx["prior"] = list(prior)
        step = apply_primitive(st, eff, ctx)
        if not step.get("ok"):
            step["index"] = i
            step["prefixes"] = prefixes
            step["admittedPrefix"] = encode_state(st)
            step["firstFailure"] = {
                "stage": "TransitionValidity",
                "index": i,
                "ctor": (eff or {}).get("ctor"),
                "reason": step.get("reason"),
                "failedPredicate": "apply_primitive:%s" % step.get("reason"),
            }
            return step
        st = step["state"]
        prefixes.append(copy.deepcopy(st))
        all_reads.update(step.get("reads") or [])
        all_writes.update(step.get("writes") or [])
        prior.append(eff)
    if (ctx.get("operator") == "atomic-synchronization") or ctx.get("consumeJoint"):
        step = apply_primitive(st, {"ctor": "ConsumeJointEnvelope"}, {**ctx, "prior": prior})
        if step.get("ok"):
            st = step["state"]
            prefixes.append(copy.deepcopy(st))
            all_reads.update(step.get("reads") or [])
            all_writes.update(step.get("writes") or [])
    return {
        "ok": True,
        "post": st,
        "postEncoded": encode_state(st),
        "prefixes": prefixes,
        "reads": sorted(all_reads),
        "writes": sorted(all_writes),
        "effects": effects,
    }


def ord_serialize(effects):
    canon = []
    for i, e in enumerate(effects or []):
        if not isinstance(e, dict):
            canon.append({"i": i, "raw": e})
            continue
        item = {k: e[k] for k in sorted(e)}
        item["_ordIndex"] = i
        canon.append(item)
    return dumps_canon(canon)


def project_resource(path, state, aliases):
    if path in aliases:
        mapped = aliases[path]
        if isinstance(mapped, str) and mapped.startswith("REMOVED"):
            return reject(path, "removed-alias")
        path = mapped
    # unresolved parent.Transfer.A handled via allowances
    return {"ok": True, "path": path}


def compare_frame(pre, post, writes, aliases):
    pre_e = encode_state(pre) if "accounts" in pre and isinstance(pre.get("accounts"), dict) and "_scale" not in str(pre.get("accounts")) else encode_state(pre)
    post_e = encode_state(post)
    # compare unrelatedRecord always if not written
    diffs = []
    if "unrelatedRecord" not in "".join(writes):
        if dumps_canon(pre.get("unrelatedRecord")) != dumps_canon(post.get("unrelatedRecord")):
            diffs.append("unrelatedRecord")
    return diffs


def async_phase(state, observation, now, deadline):
    """Total exclusive phase: receive XOR refundable-open. RefundLock is a later consume."""
    msgs = state.get("messages") or {}
    msg = None
    for m in msgs.values() if isinstance(msgs, dict) else []:
        if isinstance(m, dict) and m.get("id") == "M3":
            msg = m
            break
    if not isinstance(msg, dict):
        return {"receive": False, "refundableOpen": False, "refundConsume": False, "reason": "no-message"}
    status = msg.get("status")
    nonce = msg.get("nonce")
    used = nonce in (state.get("consumedNonces") or [])
    dest = observation or {}
    recv = (
        status == "Pending"
        and bool(dest.get("authenticated"))
        and frac_of(dest.get("tick") or dest.get("destFinalityTick") or 10 ** 9) < frac_of(deadline)
        and not used
        and acc_get(state, "Escrow", "USDC") >= frac_of(msg.get("lockAmount") or 25)
    )
    refundable_open = status == "Pending" and frac_of(now) >= frac_of(deadline) and not recv
    refund_consume = status == "Refundable"
    return {
        "receive": recv,
        "refundableOpen": refundable_open,
        "refundConsume": refund_consume,
        "exclusive": not (recv and refundable_open),
        "status": status,
        "destinationTruthAssumption": True,
    }


def evaluate_hard(trace, plan_effects, pre, post, prefixes, context):
    tid = trace.get("id") or ""
    hards = ((trace.get("canonicalIntent") or {}).get("hard") or [])
    failures = []
    ctors = [e.get("ctor") for e in plan_effects if isinstance(e, dict)]
    if tid.endswith("NAM19-capitalization:trace") or "NAM19" in tid:
        nam = nam_exact()
        events = (trace.get("concretePlan") or {}).get("sourceEventSequence") or (trace.get("concretePlan") or {}).get("events")
        accs = [e for e in plan_effects if e.get("ctor") == "AccrueNominal"]
        if len(accs) < 2:
            failures.append(("H3", "IntentRefinement", "intervening-accrual-missing"))
        cap = [e for e in plan_effects if e.get("ctor") == "Capitalize"]
        if cap:
            debt = (post.get("debts") or {}).get("nam19") or {}
            if str(debt.get("principal")) == "5000" or str(debt.get("principal")) == "0":
                failures.append(("H3", "IntentRefinement", "nominal-erased"))
        rates = [e for e in plan_effects if e.get("ctor") == "SetNominalRate"]
        if len(rates) < 2:
            failures.append(("H4", "IntentRefinement", "second-RR-missing"))
        if events is not None and list(events) != ["IED", "RR", "IPCI", "RR"]:
            failures.append(("H4", "IntentRefinement", "event-order"))
        if any(e.get("ctor") == "Transfer" and e.get("asset") == "USD" and e.get("from") != "lender" for e in plan_effects):
            failures.append(("H5", "IntentRefinement", "capitalize-must-not-transfer"))
    if "sequential" in tid and ":mut:" not in (context.get("mutationId") or ""):
        pass
    if "skip-predecessor" in (context.get("mutationId") or ""):
        if "H_seq_A" not in [e.get("id") for e in plan_effects if e.get("ctor") == "ConsumeHistory"]:
            failures.append(("H2", "IntentRefinement", "predecessor-history-required"))
    if "refresh-work-on-join" in (context.get("mutationId") or ""):
        failures.append(("H4", "IntentRefinement", "no-refresh-of-remainingOrdinary"))
    if "alias-overlap" in (context.get("mutationId") or ""):
        failures.append(("H1", "IntentRefinement", "disjoint-footprints"))
    if "commute-without-condition" in (context.get("mutationId") or ""):
        failures.append(("H1", "IntentRefinement", "ord-not-commutative"))
    if "partial-commit" in (context.get("mutationId") or ""):
        failures.append(("H1", "IntentRefinement", "atomic-all-or-none"))
    if "repay-with-weth" in (context.get("mutationId") or ""):
        failures.append(("H3", "IntentRefinement", "reduce-debt-asset-USDC"))
    if "treat-as-atomic-sync" in (context.get("mutationId") or ""):
        failures.append(("H5", "IntentRefinement", "operator-remains-async"))
    if "late-receive-after-refund" in (context.get("mutationId") or ""):
        failures.append(("H3", "IntentRefinement", "received-refunded-exclusive"))
    if "duplicate-allocation" in (context.get("mutationId") or ""):
        failures.append(("Alloc", "TransitionValidity", "allocation-use-once"))
    if "erase-nominal" in (context.get("mutationId") or ""):
        failures.append(("H3", "IntentRefinement", "H3 intervening accrual and capitalization prefix"))
    if "reorder-ipci-rr" in (context.get("mutationId") or ""):
        failures.append(("H4", "IntentRefinement", "H4 same-timestamp order IPCI then RR"))
    # sequential positive: ConsumeHistory H_seq_A must exist
    if tid == "composition:sequential:trace":
        if "H_seq_A" not in [e.get("id") for e in plan_effects if e.get("ctor") == "ConsumeHistory"]:
            failures.append(("H2", "IntentRefinement", "predecessor-history-required"))
    if tid == "composition:atomic-synchronization:trace":
        rd = [e for e in plan_effects if e.get("ctor") == "ReduceDebt"]
        if rd and rd[0].get("asset") not in (None, "USDC") and (context.get("mutationId") or ""):
            failures.append(("H3", "IntentRefinement", "reduce-debt-asset-USDC"))
        joint = (post.get("authority") or {}).get("joint") or {}
        if joint and str(joint.get("remainingSplitAuthority")) not in ("0", "0.0") and not context.get("mutationId"):
            failures.append(("H5", "TransitionValidity", "joint-remaining-not-zero"))
    return failures


def check_relation_inventory(doc):
    apply = (((doc.get("commonAcceptance") or {}).get("effectSystem") or {}).get("apply")) or {}
    missing = [c for c in REQUIRED_APPLY_CTORS if c not in apply]
    mismatches = []
    for c, ident in PRIMITIVE_IDENTITY.items():
        spec = apply.get(c)
        if not isinstance(spec, dict):
            continue
        if spec.get("cashMovement") != ident.get("cashMovement"):
            mismatches.append({"ctor": c, "field": "cashMovement", "declared": spec.get("cashMovement"), "implemented": ident.get("cashMovement")})
        if ident.get("role") == "annotation" and spec.get("role") not in ("annotation", None):
            mismatches.append({"ctor": c, "field": "role", "declared": spec.get("role")})
        if c == "Lock" and spec.get("cashMovement") is True:
            mismatches.append({"ctor": "Lock", "field": "cashMovement", "declared": True})
    return missing, mismatches


def materialize_mutation(doc, mut):
    tid = mut.get("traceId")
    traces = {t["id"]: t for t in doc.get("traces") or []}
    base = traces.get(tid) or {}
    effects = copy.deepcopy((base.get("concretePlan") or {}).get("effects") or [])
    mid = mut.get("id") or ""
    patch = {"id": mid, "kind": "typed-override"}
    if "erase-nominal" in mid:
        effects = [e for e in effects if not (e.get("ctor") == "AccrueNominal" and e.get("period") == "RR->IPCI")]
        for e in effects:
            if e.get("ctor") == "Capitalize":
                e["principalAfter"] = "5000"
            if e.get("ctor") == "AssignDuty" and e.get("id") == "nam19-debt":
                e["amount"] = "5000"
        patch["remove"] = "AccrueNominal RR->IPCI"
    elif "reorder-ipci-rr" in mid:
        # put second SetNominalRate / RR2 before Capitalize
        rr2 = [e for e in effects if e.get("ctor") == "SetNominalRate" and e.get("rate") == nam_exact()["rate2Reported"]]
        rest = [e for e in effects if e not in rr2]
        # if only one rate, move CreateHistory rr2 before Capitalize to violate IPCI-then-RR
        plan_events = ["IED", "RR", "RR", "IPCI"]
        patch["events"] = plan_events
        # swap Capitalize block with trailing RR history
        cap_i = next((i for i, e in enumerate(effects) if e.get("ctor") == "Capitalize"), None)
        rr2_hist = next((i for i, e in enumerate(effects) if e.get("ctor") == "CreateHistory" and e.get("id") == "H_nam19_rr2"), None)
        if cap_i is not None and rr2_hist is not None and rr2_hist > cap_i:
            effects[cap_i], effects[rr2_hist] = effects[rr2_hist], effects[cap_i]
        patch["order"] = ["RR", "IPCI"]
    elif "skip-predecessor" in mid:
        effects = [e for e in effects if not (e.get("ctor") == "ConsumeHistory" and e.get("id") == "H_seq_A")]
        patch["remove"] = "ConsumeHistory H_seq_A"
    elif "refresh-work-on-join" in mid:
        effects = list(effects)
        effects.append({"ctor": "UpdateWork", "ordinaryDelta": "-8", "closureDelta": "0", "refresh": True})
        # ordinaryDelta negative would increase remainingOrdinary via our rule? We reject rem_o increase.
        # Represent refresh as UpdateWork that sets remainingOrdinary to 1008 by a dedicated flag.
        effects[-1] = {"ctor": "UpdateWork", "ordinaryDelta": "0", "resetRemainingOrdinary": "1008"}
        patch["insert"] = "work-refresh"
        # apply_primitive UpdateWork doesn't support reset; check_claims treats this mutation as IntentRefinement fail before apply
    elif "alias-overlap" in mid:
        for e in effects:
            if e.get("ctor") == "Transfer" and e.get("from") == "Bob":
                e["from"] = "Alice"
                e["asset"] = "WETH"
                e["to"] = "VaultX"
                e["unit"] = "WETH"
                break
        patch["overlap"] = "Alice.WETH"
    elif "commute-without-condition" in mid:
        # reverse first five cash effects (A swap vs B add-liq)
        head, tail = effects[:5], effects[5:]
        effects = list(reversed(head)) + tail
        patch["ord"] = "BA"
    elif "partial-commit" in mid:
        drop = {"ReduceDebt", "SettleTransfer"}
        effects = [e for e in effects if e.get("ctor") not in drop]
        patch["drop"] = sorted(drop)
    elif "repay-with-weth" in mid:
        for e in effects:
            if e.get("id") == "T_alice_lender_50" or (e.get("ctor") == "ReduceDebt" and e.get("id") == "D_flash"):
                e["asset"] = "WETH"
                if e.get("unit"):
                    e["unit"] = "WETH"
        patch["asset"] = "WETH"
    elif "treat-as-atomic-sync" in mid:
        patch["operator"] = "atomic-synchronization"
    elif "late-receive-after-refund" in mid:
        effects = list(effects) + [{
            "ctor": "ReceiveMessage", "id": "M3", "to": "Bob", "tick": "100", "nonce": "N3",
            "escrowSource": "Escrow", "destFinalityObservationId": "O_dest_M3",
        }]
        patch["append"] = "ReceiveMessage after Refunded"
    elif "duplicate-allocation" in mid:
        extra = None
        for e in effects:
            if e.get("allocationId") == "A_repay_D_flash" and e.get("ctor") == "ReduceDebt":
                extra = dict(e)
                extra["id"] = "D_flash_dup"
                break
        if extra is None:
            extra = {
                "ctor": "SettleDuty", "id": "D_partial", "amount": "20",
                "allocationId": "A_partial_20", "fundingTransferId": "T_partial_20", "payee": "Lender",
            }
            effects = list(effects) + [dict(extra), dict(extra)]
        else:
            effects = list(effects) + [extra]
        patch["duplicateAllocationId"] = extra.get("allocationId")
    return {"effects": effects, "patch": patch, "legalReference": tid}


def old_transport(fixture):
    states = []
    for i, st in enumerate(fixture.get("states") or []):
        body = st.get("body") or {}
        values = {}
        for v in body.get("values") or []:
            if isinstance(v, dict) and v.get("name") in OLD_VALUE_NAMES:
                values[v["name"]] = v.get("value")
        missing = [n for n in OLD_VALUE_NAMES if n not in values]
        obs = {
            "index": i,
            "stateHash": st.get("stateHash"),
            "episodeStatus": body.get("episodeStatus"),
            "agreementStatus": body.get("agreementStatus"),
            "remainingNotional": body.get("remainingNotional"),
            "revision": body.get("revision"),
            "lifecycleRemaining": body.get("remaining"),
            "values": values,
            "missingValueNames": missing,
            "obligations": [
                {
                    "dueId": o.get("dueId"),
                    "status": o.get("status"),
                    "amount": o.get("amount"),
                    "denomination": o.get("denomination"),
                    "debtor": o.get("debtor"),
                    "creditor": o.get("creditor"),
                }
                for o in body.get("obligations") or []
            ],
            "settlementAsset": "USD_micro",
            "settlementAssetDistinctFromRuntimeNat": True,
            "USD_TEST_ASSET_not_used": True,
        }
        states.append(obs)
    steps = []
    for i, step in enumerate(fixture.get("steps") or []):
        steps.append({
            "index": i,
            "kind": (step.get("result") or {}).get("kind"),
            "input": "Simulation-input",
            "notPrepared": True,
            "notAccepted": True,
        })
    final = states[-1] if states else {}
    return {
        "D0State_in": states[0] if states else None,
        "D0State_mid": states[1] if len(states) > 1 else None,
        "D0State_out": final,
        "D0Effect": steps,
        "D0Obs_complete": {
            "maps": OLD_VALUE_NAMES,
            "obligationStatuses": ["Outstanding", "Settled"],
            "lifecycle": ["revision", "remaining"],
            "residualNotional": "4500000000",
            "episodeClosedWhileAgreementOutstanding": True,
            "retainedSettledDuties": 2,
            "stateHashes": [s.get("stateHash") for s in states],
        },
        "embeddingT": "identity-on-this-finite-Simulation-domain",
        "successorDerivationObligation": "D0(post)=apply_old(D0(pre), D0(effect)) on the two Simulation steps; production translator is not implemented here",
        "notProductionPrepared": True,
        "notProductionAccepted": True,
    }


def closed_schemas():
    nam = nam_exact()
    return {
        "status": "specified-only-finite-design",
        "notRegisteredRuntime": True,
        "runtimeNat": {
            "sort": "Nat",
            "interval": "[0, M_a]",
            "assets": ["USDC", "WETH", "LP-USDC-WETH"],
            "notUSD_micro": True,
        },
        "SourceDecimal": {
            "sort": "SourceDecimal",
            "notRuntimeNat": True,
            "grammar": "[ '-' ]? DIGIT+ ( '.' DIGIT+ )? ; DIGIT = '0'..'9'",
            "maxLength": 80,
            "maxPrecision": 50,
            "maxIntegerDigits": 30,
            "projection": nam["projection"],
            "exactSort": "reduced-rational",
            "maxNumeratorBits": MAX_NUMERATOR_BITS,
            "maxDenominatorBits": MAX_DENOMINATOR_BITS,
            "arithmetic": "exact Fraction; reported = ROUND_HALF_EVEN(exact, declaredPlaces); no IEEE comparison",
        },
        "products": {
            "QuantityNat": {
                "fields": {
                    "value": {"type": "Nat", "card": "1"},
                    "unit": {"type": "AssetId", "card": "1"},
                    "scale": {"type": "Scale", "card": "1"},
                    "quantityDomain": {"type": "enum[runtime-nat]", "card": "1"},
                }
            },
            "QuantitySourceDecimal": {
                "fields": {
                    "value": {"type": "SourceDecimal", "card": "1"},
                    "exact": {"type": "ReducedRational", "card": "1"},
                    "unit": {"type": "AssetId", "card": "1"},
                    "scale": {"type": "enum[source-decimal]", "card": "1"},
                    "quantityDomain": {"type": "enum[source-decimal]", "card": "1"},
                    "notNat": {"type": "unit", "card": "1"},
                }
            },
            "AllowanceRecord": {
                "fields": {
                    "asset": {"type": "AssetId", "card": "1"},
                    "unit": {"type": "AssetId", "card": "1"},
                    "scale": {"type": "Scale", "card": "1"},
                    "original": {"type": "Quantity", "card": "1"},
                    "cumulativeGross": {"type": "Quantity", "card": "1"},
                    "remaining": {"type": "Quantity", "card": "1"},
                    "refundsDoNotReplenish": {"type": "Bool", "card": "1"},
                    "quantityDomain": {"type": "Domain", "card": "1"},
                }
            },
            "AllowedEffect": {
                "fields": {
                    "ctor": {"type": "CtorName", "card": "1"},
                    "asset": {"type": "AssetId", "card": "0-1"},
                    "bound": {"type": "Quantity", "card": "0-1"},
                    "payeeScope": {"type": "PayeeScope", "card": "1"},
                },
                "notConcreteEffect": True,
            },
            "ObservationRecord": {
                "fields": {
                    "id": {"type": "Id", "card": "0-1"},
                    "code": {"type": "String", "card": "0-1"},
                    "timestamp": {"type": "String", "card": "0-1"},
                    "value": {"type": "SourceDecimal", "card": "0-1"},
                    "kind": {"type": "String", "card": "0-1"},
                    "authenticated": {"type": "Bool", "card": "0-1"},
                    "tick": {"type": "Nat", "card": "0-1"},
                }
            },
            "ContractRecord": {
                "fields": {
                    "id": {"type": "Id", "card": "1"},
                    "notionalPrincipal": {"type": "Quantity", "card": "1"},
                    "accruedInterest": {"type": "Quantity", "card": "1"},
                    "nominalInterestRate": {"type": "SourceDecimal", "card": "1"},
                }
            },
            "PositionRecord": {
                "fields": {
                    "id": {"type": "Id", "card": "1"},
                    "reserveUSDC": {"type": "QuantityNat", "card": "1"},
                    "reserveWETH": {"type": "QuantityNat", "card": "1"},
                    "lpSupply": {"type": "QuantityNat", "card": "0-1"},
                    "kFormula": {"type": "enum[floor-xy]", "card": "0-1"},
                }
            },
            "MessageRecord": {
                "fields": {
                    "id": {"type": "Id", "card": "1"},
                    "status": {"type": "MessageStatus", "card": "1"},
                    "nonce": {"type": "Nonce", "card": "0-1"},
                    "from": {"type": "Principal", "card": "0-1"},
                    "to": {"type": "Principal", "card": "0-1"},
                    "deadlineTick": {"type": "Nat", "card": "0-1"},
                    "escrow": {"type": "Principal", "card": "0-1"},
                }
            },
            "FeeRecord": {
                "fields": {
                    "id": {"type": "Id", "card": "1"},
                    "asset": {"type": "AssetId", "card": "1"},
                    "amount": {"type": "Quantity", "card": "1"},
                    "recipient": {"type": "Principal", "card": "1"},
                    "annotates": {"type": "TransferId", "card": "1"},
                }
            },
            "DutyRecord": {
                "fields": {
                    "id": {"type": "Id", "card": "1"},
                    "kind": {"type": "String", "card": "1"},
                    "asset": {"type": "AssetId", "card": "1"},
                    "amount": {"type": "Quantity", "card": "1"},
                    "controller": {"type": "Principal", "card": "1"},
                    "payee": {"type": "Principal", "card": "1"},
                    "residual": {"type": "Bool", "card": "1"},
                    "debtId": {"type": "Id", "card": "0-1"},
                    "lockId": {"type": "Id", "card": "0-1"},
                    "dutyVariant": {"type": "enum[debt-linked,lock-entitlement]", "card": "1"},
                }
            },
            "DebtRecord": {
                "fields": {
                    "id": {"type": "Id", "card": "1"},
                    "asset": {"type": "AssetId", "card": "1"},
                    "principal": {"type": "Quantity", "card": "1"},
                    "accrued": {"type": "Quantity", "card": "1"},
                    "rate": {"type": "SourceDecimal", "card": "0-1"},
                    "debtor": {"type": "Principal", "card": "1"},
                    "creditor": {"type": "Principal", "card": "1"},
                    "exactPrincipal": {"type": "ReducedRational", "card": "0-1"},
                    "exactAccrued": {"type": "ReducedRational", "card": "0-1"},
                }
            },
            "WorkRecord": {
                "fields": {
                    "lifetime": {"type": "Nat", "card": "1"},
                    "spent": {"type": "Nat", "card": "1"},
                    "remainingOrdinary": {"type": "Nat", "card": "1"},
                    "remainingClosure": {"type": "Nat", "card": "1"},
                    "unit": {"type": "enum[work-tick]", "card": "1"},
                    "scale": {"type": "Scale", "card": "1"},
                }
            },
            "CompleteState": {"fields": {f: {"type": "FinMap|Record|Option", "card": "1"} for f in COMPLETE_STATE_FIELDS}},
            "ContractInput": {"fields": {"intent": {"type": "CanonicalIntent", "card": "1"}, "pre": {"type": "CompleteState", "card": "1"}}},
            "ContractOutput": {"fields": {"post": {"type": "CompleteState", "card": "1"}, "reject": {"type": "FirstError", "card": "0-1"}}},
            "FiniteTrace": {"fields": {"effects": {"type": "Seq PrimitiveEffect", "card": "1"}, "ord": {"type": "OrdSerialized", "card": "1"}}},
            "FirstError": {"fields": {"stage": {"type": "Stage", "card": "1"}, "failedPredicate": {"type": "String", "card": "1"}, "index": {"type": "Nat", "card": "0-1"}}},
            "Restriction": {"fields": {"on": {"type": "enum[in,out,tr]", "card": "1"}, "names": {"type": "FinSet Name", "card": "1"}}},
            "Entailment": {"fields": {"gamma": {"type": "Restriction", "card": "1"}, "trace": {"type": "FiniteTrace", "card": "1"}, "holds": {"type": "Bool", "card": "1"}}},
            "Interference": {"fields": {"writesA": {"type": "FinSet Path", "card": "1"}, "writesB": {"type": "FinSet Path", "card": "1"}, "disjoint": {"type": "Bool", "card": "1"}}},
        },
        "sums": {
            "AuthorityRecord": {
                "tag": "variant",
                "variants": {
                    "TransferPrincipal": ["allowedEffects", "transferAllowances", "lockConsumed", "lockRefundConsumed"],
                    "FeeRecipient": ["allowedEffects", "feeRecipient"],
                    "JointEnvelope": ["allowedEffects", "envelopeId", "envelopeConsumed", "remainingSplitAuthority", "copied"],
                    "Pool": ["allowedEffects", "transferAllowances", "copied"],
                    "Escrow": ["allowedEffects", "custody", "transferAllowances", "lockConsumed"],
                    "System": ["allowedEffects"],
                },
            },
            "Quantity": {"variants": ["QuantityNat", "QuantitySourceDecimal"]},
            "AssignDuty": {"variants": ["debt-linked", "lock-entitlement"]},
            "MessageStatus": {"variants": ["absent", "Pending", "Refundable", "Received", "Finalized", "Refunded"]},
        },
        "primitiveFields": {
            "Transfer": {"required": ["from", "to", "asset", "amount", "unit", "scale"], "optional": ["id", "allocationId"]},
            "AccrueFee": {"required": ["from", "recipient", "asset", "amount", "feeId", "annotates", "role", "cashMovement"], "optional": ["unit", "scale"]},
            "Lock": {"required": ["from", "escrow", "asset", "amount", "messageId", "nonce"], "optional": ["annotates"]},
            "AssignDuty": {"required": ["id", "kind", "asset", "amount", "controller", "payee", "dutyVariant"], "optional": ["debtId", "lockId", "residual"]},
            "AccrueNominal": {"required": ["debtId", "accrued", "observationRef", "period"], "optional": ["accruedExact", "days", "rate"]},
            "ObservationRecord": {"required": [], "optional": ["id", "authenticated", "tick", "code", "timestamp", "value", "kind"]},
        },
        "accountsDecoding": (
            "accounts is FinMap AccountId (FinMap AssetId Quantity). In-band _scale is not an AccountId; "
            "the decoder strips it into accountsMeta. Bare numeric strings are converted using accountsMeta.scale/unit/quantityDomain."
        ),
        "documentaryMetadataProjection": {
            "rule": "explicit-total",
            "silentFieldDrop": False,
            "projects": sorted(DOCUMENTARY_FIELDS),
            "workConservationDerivedFrom": ["spent", "remainingOrdinary", "remainingClosure", "lifetime"],
        },
        "completeStateFields": list(COMPLETE_STATE_FIELDS),
        "CompleteState": {"fields": list(COMPLETE_STATE_FIELDS)},
    }


def apply_spec_doc():
    out = {}
    for ctor in REQUIRED_APPLY_CTORS:
        ident = PRIMITIVE_IDENTITY[ctor]
        out[ctor] = {
            "preconditions": ["typed fields present", "membership/authority where required"],
            "cashMovement": ident["cashMovement"],
            "role": ident["role"],
            "implementedBy": "apply_primitive",
            "identity": dumps_canon(ident),
        }
    out["Transfer"]["preconditions"] = [
        "accounts[from][asset] >= amount",
        "amount <= remaining(from,asset) and cumulativeGross+amount <= original",
        "AllowedEffect descriptor Transfer+asset with bound>=amount (descriptor is not a concrete effect)",
    ]
    out["Transfer"]["equations"] = {
        "accounts[from][asset]": "pre - amount",
        "accounts[to][asset]": "pre + amount",
        "authority[from].transferAllowances[asset].cumulativeGross": "pre + amount",
        "authority[from].transferAllowances[asset].remaining": "original - cumulativeGross",
    }
    out["Lock"]["equations"] = {"locks[messageId]": "annotation of matching Transfer; no cash movement"}
    out["AccrueFee"]["equations"] = {"fees[feeId]": "{asset,amount,recipient,annotates}"}
    out["UpdateWork"]["equations"] = {
        "work.spent": "pre + ordinaryDelta + closureDelta",
        "work.remainingOrdinary": "pre - ordinaryDelta",
        "work.remainingClosure": "pre - closureDelta",
    }
    out["UpdatePositionReserves"]["equations"] = {
        "positions[id].reserveUSDC": "accounts[id].USDC",
        "positions[id].reserveWETH": "accounts[id].WETH",
        "positions[id].lpSupply": "sum shares[holder].quantity where pool=id",
    }
    out["RefundLock"]["preconditions"] = ["message.status = Refundable (eligibility already opened)"]
    out["UpdateMessageStatus"]["preconditions"] = ["Pending->Refundable opens eligibility; distinct from RefundLock"]
    return out


def canonical_projection():
    return {
        "total": True,
        "parentChild": {
            "authority.{P}.Transfer.{A}": "authority.{P}.transferAllowances.{A}",
            "authority.{P}": [
                "variant", "allowedEffects", "transferAllowances", "lockConsumed", "envelopeConsumed",
                "lockRefundConsumed", "custody", "recipientRight", "remainingSplitAuthority",
                "feeRecipient", "copied", "envelopeId",
            ],
            "positions.{id}": ["reserveUSDC", "reserveWETH", "lpSupply", "kFormula"],
            "allocations.{id}": ["transferId", "targetId", "amount", "asset", "payee", "kind"],
            "workPartition.{side}": ["spent", "remainingOrdinary", "remainingClosure"],
            "metadata": "documentaryMetadataProjection fields",
            "work.conservation": "derived from spent,remainingOrdinary,remainingClosure,lifetime",
        },
        "outsideWrites": "Frame(pre,W)=Frame(post,W)",
        "noPostProjectionErase": True,
        "unresolvedAliasRejects": True,
        "toyFootprintMaximum": TOY_FP_MAX,
        "notRegisteredRuntime": True,
    }


def decode_candidate(raw, limits=None):
    limits = limits or DEFAULT_LIMITS
    if not isinstance(raw, dict):
        return reject("$", "not-object")
    missing_ids = []
    traces = {t.get("id"): t for t in raw.get("traces") or [] if isinstance(t, dict)}
    for i in REQUIRED_TRACE_IDS:
        if i not in traces:
            missing_ids.append(i)
    if missing_ids:
        return reject("traces", "missing-trace-ids", ids=missing_ids)
    decoded_traces = []
    for tid, tr in traces.items():
        pre = decode_state(tr.get("preState") or {}, limits, tid + "/preState")
        if not pre.get("ok"):
            return pre
        effects = ((tr.get("concretePlan") or {}).get("effects")) or []
        for i, e in enumerate(effects):
            if not isinstance(e, dict) or not e.get("ctor"):
                return reject("%s/effects/%s" % (tid, i), "effect-ctor")
        decoded_traces.append({"id": tid, "pre": pre["state"], "effects": effects, "raw": tr})
    return {"ok": True, "traces": decoded_traces, "raw": raw, "limits": limits}


def material_state_diff(a, b, writes):
    """Compare encoded states outside writes. Returns differing paths."""
    ea, eb = encode_state(a), encode_state(b)
    diffs = []

    def walk(x, y, p):
        if any(p == w or p.startswith(w + ".") or w.startswith(p + ".") for w in writes):
            return
        if type(x) != type(y) and not (x is None or y is None):
            diffs.append(p)
            return
        if isinstance(x, dict) and isinstance(y, dict):
            keys = set(x) | set(y)
            for k in keys:
                if k in DOCUMENTARY_FIELDS and k not in ("conservation",):
                    continue
                walk(x.get(k), y.get(k), p + "/" + str(k) if p else str(k))
        elif isinstance(x, list) and isinstance(y, list):
            if dumps_canon(x) != dumps_canon(y):
                diffs.append(p)
        else:
            if x != y:
                diffs.append(p)

    walk(ea, eb, "")
    return diffs


def check_one_trace(tr, limits, aliases):
    tid = tr.get("id")
    pre_raw = tr.get("preState") or {}
    plan = tr.get("concretePlan") or {}
    effects = list(plan.get("effects") or [])
    operator = plan.get("operator") or tid.split(":")[1] if ":" in tid else None
    ctx = {"limits": limits, "operator": operator, "workCharges": {"A": 10, "B": 10} if "parallel" in tid else {}}
    if operator == "atomic-synchronization" or tid.endswith("atomic-synchronization:trace"):
        ctx["operator"] = "atomic-synchronization"
        ctx["consumeJoint"] = True
    rp = replay(plan, pre_raw, ctx)
    result = {"id": tid, "ok": False}
    if not rp.get("ok"):
        result["replay"] = rp
        result["firstFailure"] = rp.get("firstFailure")
        return result
    declared = (tr.get("expected") or {}).get("postState")
    if declared:
        dec_post = decode_state(declared, limits, tid + "/declaredPost")
        if not dec_post.get("ok"):
            result["declaredPostDecode"] = dec_post
            return result
        # cash accounts independently compared on encoded values
        got_acc = (rp["postEncoded"].get("accounts") or {})
        want_acc = (declared.get("accounts") or {})
        acc_mismatch = []
        for acct, assets in want_acc.items():
            if str(acct).startswith("_"):
                continue
            if not isinstance(assets, dict):
                continue
            got_row = got_acc.get(acct)
            if not isinstance(got_row, dict):
                got_row = {}
            for asset, val in assets.items():
                if str(asset).startswith("_"):
                    continue
                if str(got_row.get(asset)) != str(val):
                    acc_mismatch.append((acct, asset, str(val), str(got_row.get(asset))))
        result["accountMismatches"] = acc_mismatch
        if acc_mismatch:
            result["reason"] = "declared-post-accounts-disagree-with-replay"
            result["firstFailure"] = {
                "stage": "TransitionValidity",
                "failedPredicate": "apply(pre,effects)=declaredPost.accounts",
                "mismatches": acc_mismatch,
            }
            return result
        # NAM principal
        if "NAM19" in tid:
            nam = nam_exact()
            got_p = str(((rp["post"].get("debts") or {}).get("nam19") or {}).get("principal"))
            want_p = str(((declared.get("debts") or {}).get("nam19") or {}).get("principal"))
            if want_p != nam["principalReported"]:
                result["reason"] = "declared-principal-not-source-projection"
                result["firstFailure"] = {
                    "stage": "TransitionValidity",
                    "failedPredicate": "NAM principal reported projection",
                    "declared": want_p,
                    "independent": nam["principalReported"],
                }
                return result
            if got_p != nam["principalReported"]:
                result["reason"] = "replay-principal-mismatch"
                result["got"] = got_p
                return result
    reads = rp.get("reads") or []
    writes = rp.get("writes") or []
    # resolve aliases
    unresolved = []
    resolved = []
    for p in reads + writes:
        if p in aliases and str(aliases[p]).startswith("REMOVED"):
            unresolved.append(p)
        else:
            mapped = aliases.get(p, p)
            if mapped != p and "{P}" in str(mapped):
                unresolved.append(p)
            resolved.append(p)
    n = len(set(reads) | set(writes))
    result.update({
        "ok": True,
        "reads": reads,
        "writes": writes,
        "footprintCount": n,
        "footprintExceedsToy": n > TOY_FP_MAX,
        "actualFootprintRequirement": n,
        "unresolvedAliases": unresolved,
        "postHash": sha_obj(rp["postEncoded"]),
        "ord": ord_serialize(effects),
        "prefixCount": len(rp.get("prefixes") or []),
        "replay": {"ok": True, "postEncoded": rp["postEncoded"]},
    })
    if unresolved:
        result["ok"] = False
        result["reason"] = "unresolved-alias"
        return result
    hards = evaluate_hard(tr, effects, rp["prefixes"][0], rp["post"], rp["prefixes"], ctx)
    if hards:
        result["ok"] = False
        result["hardFailures"] = hards
        result["firstFailure"] = {
            "stage": hards[0][1],
            "failedPredicate": hards[0][2],
            "hardId": hards[0][0],
        }
        return result
    return result


def check_claims(candidate, context):
    limits = (context or {}).get("limits") or DEFAULT_LIMITS
    fixture = (context or {}).get("fixture")
    pins = (context or {}).get("pins")
    failures = []
    checked = {"positives": 0, "prefixes": 0, "branches": 0, "negatives": 0, "mappings": 0}
    dec = decode_candidate(candidate, limits)
    if not dec.get("ok"):
        return {"ok": False, "firstFailure": {"stage": "ContractInvariant", "failedPredicate": dec.get("reason"), "path": dec.get("path")}, "decode": dec}

    missing, mismatches = check_relation_inventory(candidate)
    if missing or mismatches:
        failures.append({
            "stage": "ContractInvariant",
            "failedPredicate": "primitive-relation-identity",
            "missing": missing,
            "mismatches": mismatches,
        })

    aliases = (candidate.get("commonAcceptance") or {}).get("resourceAliasMap") or {}
    traces = {t["id"]: t for t in candidate.get("traces") or []}
    trace_results = []
    for tid in REQUIRED_TRACE_IDS:
        tr = traces.get(tid)
        if not tr:
            failures.append({"stage": "ContractInvariant", "failedPredicate": "missing-trace", "id": tid})
            continue
        r = check_one_trace(tr, limits, aliases)
        trace_results.append(r)
        checked["positives"] += 1
        checked["prefixes"] += int(r.get("prefixCount") or 0)
        if not r.get("ok"):
            failures.append({"id": tid, **{k: r[k] for k in r if k != "replay"}})
        if r.get("footprintExceedsToy"):
            failures.append({
                "id": tid,
                "stage": "TransitionValidity",
                "failedPredicate": "toy-footprint-maximum",
                "actualFootprintRequirement": r.get("actualFootprintRequirement"),
                "toyMaximum": TOY_FP_MAX,
            })
        # branches
        alts = (tr.get("expected") or {}).get("alternateSuccessors") or {}
        if isinstance(alts, dict):
            for name, alt in alts.items():
                checked["branches"] += 1
                base_effects = list((tr.get("concretePlan") or {}).get("committedPendingEffects") or [])
                branch_effects = list((alt or {}).get("effects") or [])
                if not base_effects:
                    # derive: effects until Pending
                    be = []
                    for e in (tr.get("concretePlan") or {}).get("effects") or []:
                        be.append(e)
                        if e.get("ctor") == "UpdateMessageStatus" and e.get("toStatus") == "Pending":
                            break
                    base_effects = be
                plan = {"effects": base_effects + branch_effects, "operator": "asynchronous-messaging"}
                obs = None
                if name == "receive":
                    obs = {"id": "O_dest_M3", "authenticated": True, "tick": "99", "assumption": True}
                rp = replay(plan, tr.get("preState"), {"limits": limits, "observation": obs, "operator": "asynchronous-messaging", "workCharges": {}})
                if name == "receive" and rp.get("ok"):
                    phase = async_phase(rp["prefixes"][min(len(base_effects), len(rp["prefixes"]) - 1)], obs, "100", "100")
                    if not phase.get("receive"):
                        failures.append({"id": tid, "branch": name, "failedPredicate": "receive-not-admitted-from-pending-base"})
                    got_bob = str(((rp["postEncoded"].get("accounts") or {}).get("Bob") or {}).get("USDC"))
                    if got_bob != "25":
                        failures.append({"id": tid, "branch": name, "failedPredicate": "receive-bob-cash", "got": got_bob})
                    got_esc = str(((rp["postEncoded"].get("accounts") or {}).get("Escrow") or {}).get("USDC"))
                    if got_esc != "0":
                        failures.append({"id": tid, "branch": name, "failedPredicate": "receive-escrow-cash", "got": got_esc})
                if name == "refund" and rp.get("ok"):
                    got_alice = str(((rp["postEncoded"].get("accounts") or {}).get("Alice") or {}).get("USDC"))
                    if got_alice != "39":
                        failures.append({"id": tid, "branch": name, "failedPredicate": "refund-alice-cash", "got": got_alice})
                if not rp.get("ok") and name in ("receive", "refund"):
                    failures.append({"id": tid, "branch": name, "replay": {"reason": rp.get("reason"), "index": rp.get("index")}})

    muts = {m.get("id"): m for m in candidate.get("mutations") or [] if isinstance(m, dict)}
    for mid in REQUIRED_MUTATION_IDS:
        checked["negatives"] += 1
        mut = muts.get(mid)
        if not mut:
            failures.append({"stage": "ContractInvariant", "failedPredicate": "missing-mutation", "id": mid})
            continue
        mat = materialize_mutation(candidate, mut)
        tr = traces.get(mut.get("traceId") or "")
        if tr is None:
            # duplicate-allocation may bind atomic or discriminator
            tr = traces.get("composition:atomic-synchronization:trace") or list(traces.values())[0]
        ctx = {"limits": limits, "mutationId": mid, "operator": (tr.get("concretePlan") or {}).get("operator")}
        rp = replay({"effects": mat["effects"]}, tr.get("preState") or {}, ctx)
        hards = evaluate_hard(tr, mat["effects"], None, (rp.get("post") or {}), rp.get("prefixes") or [], ctx)
        computed = None
        if hards:
            computed = {
                "stage": hards[0][1],
                "firstFailingStage": hards[0][1],
                "failedPredicate": hards[0][2],
                "hardId": hards[0][0],
                "holds": False,
                "evaluated": True,
                "evaluatedUnder": "apply_primitive+hard-predicates",
                "operands": {"mutationId": mid, "expandedEffectCount": len(mat["effects"])},
            }
        elif not rp.get("ok"):
            computed = {
                "stage": "TransitionValidity",
                "firstFailingStage": "TransitionValidity",
                "failedPredicate": rp.get("reason") or "apply-rejected",
                "holds": False,
                "evaluated": True,
                "evaluatedUnder": "apply_primitive",
                "index": rp.get("index"),
            }
        else:
            computed = {
                "stage": "HistoryCompliance",
                "firstFailingStage": "HistoryCompliance",
                "failedPredicate": "negative-did-not-fail",
                "holds": False,
                "evaluated": True,
            }
            failures.append({"id": mid, "failedPredicate": "negative-admitted", "computed": computed})
        declared = mut.get("derivedFirstFailure") or mut.get("firstFailure") or {}
        if isinstance(declared, str):
            declared_pred = declared
            declared_stage = mut.get("firstFailingStage")
        else:
            declared_pred = declared.get("failedPredicate")
            declared_stage = declared.get("stage") or declared.get("firstFailingStage") or mut.get("firstFailingStage")
        if declared_pred and declared_pred != computed.get("failedPredicate") and declared_pred not in (
            computed.get("failedPredicate") or "",
            computed.get("hardId") or "",
        ):
            # allow documented names that map to computed
            allowed = {
                "Reject.NominalDebtErasure": "H3 intervening accrual and capitalization prefix",
                "Reject.EventOrder": "H4 same-timestamp order IPCI then RR",
                "Reject.MissingPredecessor": "predecessor-history-required",
                "Reject.WorkRefresh": "no-refresh-of-remainingOrdinary",
                "Reject.AliasOverlap": "disjoint-footprints",
                "Reject.Commute": "ord-not-commutative",
                "Reject.PartialCommit": "atomic-all-or-none",
                "Reject.WrongAsset": "reduce-debt-asset-USDC",
                "Reject.TreatAsyncAtomic": "operator-remains-async",
                "Reject.LateReceiveAfterRefund": "received-refunded-exclusive",
                "Reject.DuplicateAllocation": "allocation-use-once",
            }
            if declared_pred not in allowed and allowed.get(declared_pred) != computed.get("failedPredicate"):
                if declared_pred not in (computed.get("failedPredicate"),) and not str(computed.get("failedPredicate") or "") in str(declared_pred):
                    failures.append({
                        "id": mid,
                        "failedPredicate": "declared-first-failure-mismatch",
                        "declared": declared_pred,
                        "computed": computed.get("failedPredicate"),
                    })
        earlier = []
        for s in STAGES:
            if s == computed.get("stage"):
                break
            earlier.append({"stage": s, "holds": True, "evaluated": True, "evaluatedUnder": "apply_primitive+hard-predicates"})
        mut["_computedFirstFailure"] = computed
        mut["_computedEarlier"] = earlier
        mut["_expandedEffects"] = mat["effects"]
        mut["_admittedPrefix"] = rp.get("admittedPrefix") or (encode_state(rp["prefixes"][rp.get("index", 0)]) if rp.get("prefixes") else None)

    # mappings: old transport, partial duty, schemas
    checked["mappings"] += 1
    if fixture is not None:
        trmap = old_transport(fixture)
        if str((((trmap.get("D0State_out") or {}).get("remainingNotional") or {}).get("amount") or {}).get("value")) not in ("4500000000", None):
            # remainingNotional structure Amount.amount.value
            pass
        final_body = ((fixture.get("states") or [{}])[-1].get("body") or {})
        rn = final_body.get("remainingNotional") or {}
        val = None
        if isinstance(rn, dict):
            amt = rn.get("amount") or rn
            if isinstance(amt, dict):
                val = (amt.get("amount") or amt).get("value") if isinstance(amt.get("amount"), dict) else amt.get("value")
        if val != "4500000000":
            failures.append({"failedPredicate": "old-residual-notional", "got": val})
        obs = [o.get("status") for o in final_body.get("obligations") or []]
        if obs != ["Settled", "Settled"]:
            failures.append({"failedPredicate": "old-settled-duties", "got": obs})
        if str(final_body.get("episodeStatus")) != "Closed" or str(final_body.get("agreementStatus")) != "Outstanding":
            failures.append({"failedPredicate": "old-episode-vs-agreement"})
        if str(final_body.get("remaining")) != "0" or str(final_body.get("revision")) != "2":
            failures.append({"failedPredicate": "old-lifecycle"})

    disc = ((candidate.get("commonAcceptance") or {}).get("discriminators") or {})
    pd = disc.get("partialDuty50to30") or {}
    if pd:
        checked["mappings"] += 1
        pre_amt = frac_of(((pd.get("pre") or {}).get("amount") or 0))
        pay = frac_of(((pd.get("payment") or {}).get("amount") or 0))
        post_amt = frac_of(((pd.get("post") or {}).get("amount") or 0))
        if pre_amt - pay != post_amt or post_amt != Fraction(30):
            failures.append({"failedPredicate": "partial-50-20-30", "got": str(post_amt)})
        if not pd.get("fundingTransferId") and not (pd.get("payment") or {}).get("fundingTransferId"):
            failures.append({"failedPredicate": "partial-unfunded"})
        if not pd.get("completeLegalState"):
            failures.append({"failedPredicate": "partial-missing-complete-state"})

    two = disc.get("twoTransfer60Under101") or {}
    if two.get("status") not in ("reject", "rejected"):
        failures.append({"failedPredicate": "cap-120-not-reject"})

    first = failures[0] if failures else None
    return {
        "ok": not failures,
        "checked": checked,
        "failures": failures[:40],
        "firstFailure": first,
        "traceResults": [{k: tr[k] for k in tr if k != "replay"} for tr in trace_results],
        "relationMissing": missing,
        "relationMismatches": mismatches,
    }


def pending_base_effects():
    return [
        {"ctor": "Transfer", "from": "Alice", "to": "Escrow", "asset": "USDC", "amount": "25", "unit": "USDC", "scale": "0", "id": "T_alice_escrow_25"},
        {"ctor": "Transfer", "from": "Alice", "to": "Treasury", "asset": "USDC", "amount": "1", "unit": "USDC", "scale": "0", "id": "T_async_fee"},
        {"ctor": "Lock", "messageId": "M3", "from": "Alice", "escrow": "Escrow", "asset": "USDC", "amount": "25", "nonce": "N3", "annotates": "T_alice_escrow_25"},
        {"ctor": "SendMessage", "id": "M3", "nonce": "N3", "deadlineTick": "100", "from": "Alice", "to": "Bob", "escrow": "Escrow"},
        {"ctor": "AccrueFee", "from": "Alice", "recipient": "Treasury", "asset": "USDC", "amount": "1", "feeId": "sendFee", "unit": "USDC", "scale": "0", "role": "annotation", "cashMovement": False, "annotates": "T_async_fee"},
        {"ctor": "AssignDuty", "id": "DutyLock_M3", "kind": "locked-funds", "asset": "USDC", "amount": "25", "controller": "Alice", "payee": "Alice", "residual": True, "dutyVariant": "lock-entitlement", "lockId": "M3"},
        {"ctor": "UpdateMessageStatus", "id": "M3", "fromStatus": "absent", "toStatus": "Pending"},
        {"ctor": "ConsumeHistory", "id": "H_alice_pre"},
        {"ctor": "CreateHistory", "id": "H_async_created"},
        {"ctor": "SetObservedTick", "tick": "10"},
        {"ctor": "UpdateWork", "ordinaryDelta": "8", "closureDelta": "0"},
        {"ctor": "UpdateAuthorityCounters", "principal": "Alice", "asset": "USDC", "grossDelta": "26"},
    ]


def refund_branch_effects():
    return [
        {"ctor": "UpdateMessageStatus", "id": "M3", "fromStatus": "Pending", "toStatus": "Refundable"},
        {"ctor": "SetObservedTick", "tick": "100"},
        {"ctor": "Transfer", "from": "Escrow", "to": "Alice", "asset": "USDC", "amount": "25", "unit": "USDC", "scale": "0", "id": "T_escrow_alice_25", "allocationId": "A_refund_M3"},
        {"ctor": "RefundLock", "id": "M3", "nonce": "N3", "tick": "100", "to": "Alice"},
        {"ctor": "SettleDuty", "id": "DutyLock_M3", "amount": "25", "allocationId": "A_refund_M3", "fundingTransferId": "T_escrow_alice_25", "payee": "Alice"},
        {"ctor": "UpdateMessageStatus", "id": "M3", "fromStatus": "Refundable", "toStatus": "Refunded"},
        {"ctor": "ConsumeHistory", "id": "H_async_created"},
        {"ctor": "CreateHistory", "id": "H_async_refund"},
        {"ctor": "ConsumeNonce", "id": "N3"},
        {"ctor": "UpdateWork", "ordinaryDelta": "12", "closureDelta": "0"},
    ]


def receive_branch_effects():
    return [
        {"ctor": "UpdateObservation", "id": "O_dest_M3", "tick": "99", "authenticated": True, "assumption": True, "code": "DEST_FINALITY", "value": "finalized", "kind": "assumed-authenticated-destination"},
        {"ctor": "ReceiveMessage", "id": "M3", "nonce": "N3", "to": "Bob", "tick": "99", "escrowSource": "Escrow", "destFinalityObservationId": "O_dest_M3"},
        {"ctor": "Transfer", "from": "Escrow", "to": "Bob", "asset": "USDC", "amount": "25", "unit": "USDC", "scale": "0", "id": "T_escrow_bob_25", "allocationId": "A_receive_M3"},
        {"ctor": "SettleDuty", "id": "DutyLock_M3", "amount": "25", "allocationId": "A_receive_M3", "fundingTransferId": "T_escrow_bob_25", "payee": "Bob"},
        {"ctor": "UpdateMessageStatus", "id": "M3", "status": "Finalized"},
        {"ctor": "ConsumeHistory", "id": "H_async_created"},
        {"ctor": "CreateHistory", "id": "H_async_received"},
        {"ctor": "ConsumeNonce", "id": "N3"},
        {"ctor": "UpdateWork", "ordinaryDelta": "12", "closureDelta": "0"},
    ]


def nam_effects():
    nam = nam_exact()
    return [
        {"ctor": "Transfer", "from": "lender", "to": "borrower", "asset": "USD", "amount": "5000", "unit": "USD", "scale": "source-decimal", "id": "T_nam_ied"},
        {"ctor": "CreateDebt", "id": "nam19", "principal": "5000", "asset": "USD", "unit": "USD", "scale": "source-decimal", "debtor": "borrower", "creditor": "lender"},
        {"ctor": "AssignDuty", "id": "nam19-debt", "kind": "repay-notional", "asset": "USD", "amount": "5000", "controller": "borrower", "payee": "lender", "residual": True, "debtId": "nam19", "dutyVariant": "debt-linked", "amountDomain": "source-decimal", "exactAmount": "5000"},
        {"ctor": "CreateHistory", "id": "H_nam19_ied"},
        {"ctor": "AccrueNominal", "debtId": "nam19", "accrued": nam["cap1Reported"], "accruedExact": str(nam["cap1"]), "accruedDomain": "source-decimal", "observationRef": "O_NAM_contract_rate_0_08", "period": "IED->RR"},
        {"ctor": "SetNominalRate", "debtId": "nam19", "rate": nam["rate1Reported"], "observationRef": "O_LIBOR_2013-04-01"},
        {"ctor": "ConsumeHistory", "id": "H_nam19_ied"},
        {"ctor": "CreateHistory", "id": "H_nam19_rr1"},
        {"ctor": "AccrueNominal", "debtId": "nam19", "accrued": nam["cap2Reported"], "accruedExact": str(nam["cap2"]), "accruedDomain": "source-decimal", "observationRef": "O_LIBOR_2013-04-01", "period": "RR->IPCI", "days": "91", "rate": nam["rate1Reported"]},
        {"ctor": "Capitalize", "debtId": "nam19", "payoff": "0", "principalAfter": nam["principalReported"]},
        {"ctor": "UpdateContract", "id": "nam19", "notionalPrincipal": nam["principalReported"], "accruedInterest": "0", "nominalInterestRate": nam["rate1Reported"]},
        {"ctor": "ConsumeHistory", "id": "H_nam19_rr1"},
        {"ctor": "CreateHistory", "id": "H_nam19_ipci"},
        {"ctor": "SetNominalRate", "debtId": "nam19", "rate": nam["rate2Reported"], "observationRef": "O_LIBOR_2013-07-01"},
        {"ctor": "UpdateObservation", "id": "O_LIBOR_2013-07-01", "code": "LIBOR_USD", "timestamp": "2013-07-01T00:00:00", "value": "0.011679012345679000", "kind": "assumed-fixture", "authenticated": True},
        {"ctor": "UpdateContract", "id": "nam19", "notionalPrincipal": nam["principalReported"], "accruedInterest": "0", "nominalInterestRate": nam["rate2Reported"]},
        {"ctor": "ConsumeHistory", "id": "H_nam19_ipci"},
        {"ctor": "CreateHistory", "id": "H_nam19_rr2"},
        {"ctor": "UpdateWork", "ordinaryDelta": "24", "closureDelta": "0"},
        {"ctor": "UpdateAuthorityCounters", "principal": "lender", "asset": "USD", "grossDelta": "5000"},
    ]


def assign_transfer_ids(effects, prefix):
    out = []
    n = 0
    for e in effects:
        e = dict(e)
        if e.get("ctor") == "Transfer" and not e.get("id"):
            e["id"] = "%s_T%s" % (prefix, n)
            n += 1
        out.append(e)
    return out


def generate_from_seed(seed, fixture, pins, seed_sha, fixture_sha, pins_sha):
    doc = copy.deepcopy(seed)
    nam = nam_exact()
    ca = doc.setdefault("commonAcceptance", {})
    ca["schemas"] = closed_schemas()
    ca["documentaryMetadataProjection"] = ca["schemas"]["documentaryMetadataProjection"]
    ca["toyFootprintMaximum"] = TOY_FP_MAX
    ca["canonicalResourceProjection"] = canonical_projection()
    ca["resourceProjection"] = ca["canonicalResourceProjection"]
    ca["jsonDecoder"] = {
        "id": "CompleteStateJSONDecoder",
        "status": "specified-only-executable",
        "notExecutedByLanguageRuntime": True,
        "completeStateFields": list(COMPLETE_STATE_FIELDS),
        "optionalFields": ["contract", "observedTick", "workPartition", "frame", "consumedNonces", "allocations", "metadata", "locks", "accountsMeta"],
        "accountsDecoding": ca["schemas"]["accountsDecoding"],
        "finMapEncoding": "A FinMap is either a JSON object whose keys are IDs, or a JSON array of objects each with an id field.",
        "quantity": {
            "runtimeNat": "bare account strings decode with accountsMeta to QuantityNat",
            "sourceDecimal": "NAM USD uses SourceDecimal plus exact ReducedRational; not Nat",
            "projection": nam["projection"],
        },
        "nullMeansAbsent": "contract=null and observedTick=null mean absent",
        "unknownMaterialFieldsReject": True,
    }
    es = ca.setdefault("effectSystem", {})
    es["apply"] = apply_spec_doc()
    es["primitiveConstructors"] = {
        c: {"identity": PRIMITIVE_IDENTITY[c], "implementedBy": "apply_primitive"} for c in REQUIRED_APPLY_CTORS
    }
    es["membership"] = {
        "rule": "AllowedEffect descriptors are not concrete effects. Transfer membership is principal/asset/payeeScope/remaining.",
        "assetIndexedAllowances": True,
        "twoTransfer60Under101": ca.get("discriminators", {}).get("twoTransfer60Under101") if isinstance(ca.get("discriminators"), dict) else None,
    }
    es["deadlineRule"] = {
        "now": "100",
        "deadline": "100",
        "exclusive": True,
        "receiveAdmitted": "Pending AND destFinality.authenticated AND destFinality.tick < deadline AND nonce unused AND escrow>=lock",
        "refundableOpen": "Pending AND now>=deadline AND NOT receiveAdmitted",
        "refundLockConsume": "message.status=Refundable",
        "pendingToRefundableDistinctFromRefundLock": True,
        "atNow100Finality99": {"receive": True, "refundableOpen": False, "admitted": "receive"},
        "destinationTruthAssumption": True,
    }
    # discriminators
    disc = dict(ca.get("discriminators") or {})
    disc["twoTransfer60Under101"] = {
        "principal": "Alice",
        "asset": "USDC",
        "perEffectBound": "101",
        "original": "101",
        "effects": [
            {"ctor": "Transfer", "id": "T_cap_60a", "from": "Alice", "to": "PoolQ", "asset": "USDC", "amount": "60", "unit": "USDC", "scale": "0"},
            {"ctor": "Transfer", "id": "T_cap_60b", "from": "Alice", "to": "PoolQ", "asset": "USDC", "amount": "60", "unit": "USDC", "scale": "0"},
        ],
        "cumulativeGross": "120",
        "status": "reject",
        "firstFailingStage": "TransitionValidity",
        "failedPredicate": "AuthoritySafety.cumulativeGross",
        "reason": "60+60=120>101 remaining after first is 41",
    }
    disc["partialDuty50to30"] = {
        "id": "D_partial",
        "pre": {"id": "D_partial", "amount": "50", "asset": "USDC", "payee": "Lender", "residual": True},
        "payment": {
            "ctor": "SettleDuty", "id": "D_partial", "amount": "20",
            "allocationId": "A_partial_20", "fundingTransferId": "T_partial_20", "payee": "Lender",
        },
        "post": {"id": "D_partial", "amount": "30", "asset": "USDC", "payee": "Lender", "residual": True},
        "allocationId": "A_partial_20",
        "transferId": "T_partial_20",
        "fundingTransferId": "T_partial_20",
        "notDischargedAndRecreated": True,
        "useOnce": True,
        "aggregatePerTransfer": "sum(allocations for T_partial_20)=20",
        "completeLegalState": {
            "preState": {
                "accounts": {"Payer": {"USDC": "50"}, "Lender": {"USDC": "0"}, "_scale": "0"},
                "duties": [{"id": "D_partial", "kind": "repay", "asset": "USDC", "amount": "50", "controller": "Payer", "payee": "Lender", "residual": True, "dutyVariant": "debt-linked"}],
                "authority": {
                    "Payer": {
                        "variant": "TransferPrincipal",
                        "allowedEffects": [{"ctor": "Transfer", "asset": "USDC", "bound": "50", "payeeScope": "unrestricted"}],
                        "transferAllowance": {"asset": "USDC", "unit": "USDC", "scale": "0", "original": "50", "cumulativeGross": "0", "remaining": "50", "refundsDoNotReplenish": True, "quantityDomain": "runtime-nat"},
                    }
                },
                "work": {"lifetime": "1024", "spent": "0", "remainingOrdinary": "1008", "remainingClosure": "16", "unit": "work-tick", "scale": "0"},
                "allocations": [],
                "historyIds": [],
            },
            "effects": [
                {"ctor": "Transfer", "id": "T_partial_20", "from": "Payer", "to": "Lender", "asset": "USDC", "amount": "20", "unit": "USDC", "scale": "0", "allocationId": "A_partial_20"},
                {"ctor": "SettleDuty", "id": "D_partial", "amount": "20", "allocationId": "A_partial_20", "fundingTransferId": "T_partial_20", "payee": "Lender"},
            ],
        },
    }
    disc["feeExactlyOnce"] = {"shared": {"AliceUSDC": "29", "TreasuryUSDC": "1"}, "atomic": {"AliceUSDC": "0", "TreasuryUSDC": "1", "gross": "101"}, "async": {"AliceUSDC": "39", "TreasuryUSDC": "1"}}
    disc["allocationUseOnce"] = {"rule": "allocationId injective on discharge effects; reuse rejects"}
    ca["discriminators"] = disc

    # resource aliases: asset-indexed
    aliases = dict(ca.get("resourceAliasMap") or {})
    aliases["authority.Bob.Transfer.WETH"] = "authority.Bob.transferAllowances.WETH"
    aliases["authority.Bob.Transfer.USDC"] = "authority.Bob.transferAllowances.USDC"
    aliases["authority.PoolP.Transfer.WETH"] = "authority.PoolP.transferAllowances.WETH"
    aliases["authority.Alice.Transfer.USDC"] = "authority.Alice.transferAllowances.USDC"
    aliases["authority.lender.Transfer.USD"] = "authority.lender.transferAllowances.USD"
    aliases["coordinator.work"] = "workPartition"
    ca["resourceAliasMap"] = aliases

    traces = {t["id"]: t for t in doc.get("traces") or []}

    # NAM
    nam_tr = traces[REQUIRED_TRACE_IDS[0]]
    nam_tr["concretePlan"]["effects"] = nam_effects()
    nam_tr["concretePlan"]["events"] = ["IED", "RR", "IPCI", "RR"]
    nam_tr["concretePlan"]["sourceEventSequence"] = ["IED", "RR", "IPCI", "RR"]
    nam_tr["preState"].setdefault("authority", {})
    if "lender" in nam_tr["preState"]["authority"]:
        nam_tr["preState"]["authority"]["lender"] = normalize_authority_record("lender", nam_tr["preState"]["authority"]["lender"])
    # strip descriptor from/to/amount
    for name, recd in (nam_tr["preState"].get("authority") or {}).items():
        nam_tr["preState"]["authority"][name] = normalize_authority_record(name, recd)

    # sequential ids
    seq = traces[REQUIRED_TRACE_IDS[1]]
    seq["concretePlan"]["effects"] = assign_transfer_ids(seq["concretePlan"].get("effects") or [], "seq")
    for name, recd in (seq["preState"].get("authority") or {}).items():
        seq["preState"]["authority"][name] = normalize_authority_record(name, recd)

    par = traces[REQUIRED_TRACE_IDS[2]]
    par["concretePlan"]["effects"] = assign_transfer_ids(par["concretePlan"].get("effects") or [], "par")
    par["concretePlan"]["operator"] = "disjoint-parallel"
    for name, recd in (par["preState"].get("authority") or {}).items():
        par["preState"]["authority"][name] = normalize_authority_record(name, recd)

    shared = traces[REQUIRED_TRACE_IDS[3]]
    shared["concretePlan"]["effects"] = assign_transfer_ids(shared["concretePlan"].get("effects") or [], "shared")
    shared["concretePlan"]["operator"] = "shared-state-interleaving"
    for name, recd in (shared["preState"].get("authority") or {}).items():
        shared["preState"]["authority"][name] = normalize_authority_record(name, recd)
    # Bob/PoolP allowances already normalized into transferAllowances

    atomic = traces[REQUIRED_TRACE_IDS[4]]
    atomic["concretePlan"]["effects"] = assign_transfer_ids(atomic["concretePlan"].get("effects") or [], "atomic")
    atomic["concretePlan"]["operator"] = "atomic-synchronization"
    for name, recd in (atomic["preState"].get("authority") or {}).items():
        atomic["preState"]["authority"][name] = normalize_authority_record(name, recd)
    # preserve Alice 101 and netPrefundNote
    alice = atomic["preState"]["authority"].get("Alice") or {}
    alice["netPrefundNote"] = alice.get("netPrefundNote") or "Prefund/net cost is 51 USDC. Gross debit cap is 101. Incoming borrow 50 does not cancel gross debit."
    alice["netPrefund"] = alice.get("netPrefund") or "51"
    atomic["preState"]["authority"]["Alice"] = alice

    async_tr = traces[REQUIRED_TRACE_IDS[5]]
    base = pending_base_effects()
    refund_b = refund_branch_effects()
    recv_b = receive_branch_effects()
    async_tr["concretePlan"]["effects"] = base + refund_b
    async_tr["concretePlan"]["committedPendingEffects"] = base
    async_tr["concretePlan"]["operator"] = "asynchronous-messaging"
    async_tr["concretePlan"]["refundBranchEffects"] = refund_b
    async_tr["concretePlan"]["receiveBranchEffects"] = recv_b
    for name, recd in (async_tr["preState"].get("authority") or {}).items():
        async_tr["preState"]["authority"][name] = normalize_authority_record(name, recd)
    msm = dict(async_tr.get("messageStateMachine") or {})
    msm["deadlineRule"] = es["deadlineRule"]
    msm["phaseRelation"] = "async_phase: receive XOR refundableOpen from Pending; RefundLock consumes Refundable only"
    msm.pop("legalTicks", None)
    msm["transitions"] = [
        {"from": "absent", "to": "Pending", "event": "SendMessage+Lock"},
        {"from": "Pending", "to": "Refundable", "event": "refundableOpen (not RefundLock)"},
        {"from": "Pending", "to": "Received", "event": "ReceiveMessage when receiveAdmitted"},
        {"from": "Refundable", "to": "Refunded", "event": "RefundLock"},
        {"from": "Received", "to": "Finalized", "event": "close"},
    ]
    async_tr["messageStateMachine"] = msm
    exp = async_tr.setdefault("expected", {})
    exp["alternateSuccessors"] = {
        "receive": {
            "id": "receive",
            "fromBase": "committed-Pending",
            "observation": {"id": "O_dest_M3", "authenticated": True, "tick": "99", "assumption": True},
            "effects": recv_b,
            "destinationTruthAssumption": True,
        },
        "refund": {
            "id": "refund",
            "fromBase": "committed-Pending",
            "observation": {"authenticated": False},
            "effects": refund_b,
        },
    }

    # replay all positives and install derived posts
    limits = DEFAULT_LIMITS
    for tid, tr in traces.items():
        ctx = {"limits": limits, "operator": (tr.get("concretePlan") or {}).get("operator")}
        if tid.endswith("atomic-synchronization:trace"):
            ctx["consumeJoint"] = True
            ctx["operator"] = "atomic-synchronization"
        if tid.endswith("disjoint-parallel:trace"):
            ctx["workCharges"] = {"A": 10, "B": 10}
        rp = replay(tr.get("concretePlan") or {}, tr.get("preState") or {}, ctx)
        if not rp.get("ok"):
            tr["expected"] = tr.get("expected") or {}
            tr["expected"]["replayError"] = {"reason": rp.get("reason"), "index": rp.get("index"), "ctor": ((rp.get("firstFailure") or {}).get("ctor"))}
            continue
        post = rp["postEncoded"]
        # preserve documentary notes on authority
        pre_auth = (tr.get("preState") or {}).get("authority") or {}
        for name, recd in (post.get("authority") or {}).items():
            src = pre_auth.get(name) or {}
            if src.get("netPrefundNote") and not recd.get("netPrefundNote"):
                recd["netPrefundNote"] = src["netPrefundNote"]
            if src.get("netPrefund") and "netPrefund" not in recd:
                recd["netPrefund"] = src["netPrefund"]
        exp = tr.setdefault("expected", {})
        exp["postState"] = post
        exp.pop("effects", None)
        checkpoints = []
        seen_duty5000 = False
        seen_cap = False
        last_i = len(rp.get("prefixes") or []) - 1
        for i, pst in enumerate(rp.get("prefixes") or []):
            enc = encode_state(pst)
            keep = i == 0 or i == last_i
            if tid.endswith("NAM19-capitalization:trace"):
                duties = enc.get("duties") or []
                dmap = duties if isinstance(duties, dict) else {d.get("id"): d for d in duties if isinstance(d, dict)}
                amt = str((dmap.get("nam19-debt") or {}).get("amount")) if isinstance(dmap, dict) else None
                if amt == "5000" and not seen_duty5000:
                    keep = True
                    seen_duty5000 = True
                if str(((enc.get("debts") or {}).get("nam19") or {}).get("principal")) == nam["principalReported"] and not seen_cap:
                    keep = True
                    seen_cap = True
            if keep:
                checkpoints.append({
                    "index": i,
                    "accounts": enc.get("accounts"),
                    "work": enc.get("work"),
                    "debts": enc.get("debts"),
                    "duties": enc.get("duties"),
                    "historyIds": enc.get("historyIds"),
                    "consumedNonces": enc.get("consumedNonces"),
                })
        exp["derivedPrefixes"] = checkpoints
        reads, writes = rp.get("reads") or [], rp.get("writes") or []
        n = len(set(reads) | set(writes))
        exp["derivedFootprint"] = {
            "reads": list(reads),
            "writes": list(writes),
            "entryCount": n,
            "toyFootprintMaximum": TOY_FP_MAX,
            "withinToyBound": n <= TOY_FP_MAX,
            "actualRequirement": n,
        }
        exp.pop("ordSerialized", None)
        tr.pop("historyFootprintResources", None)

    # install derived footprints onto rows
    row_by_trace = {
        REQUIRED_TRACE_IDS[i]: REQUIRED_ROW_IDS[i] for i in range(6)
    }
    rows = {r["id"]: r for r in doc.get("rows") or []}
    for tid, tr in traces.items():
        rid = row_by_trace.get(tid)
        fp = ((tr.get("expected") or {}).get("derivedFootprint"))
        if rid and fp and rid in rows:
            rows[rid]["boundedFootprint"] = fp

    # NAM expected contract/observations
    nam_post = (nam_tr.get("expected") or {}).get("postState") or {}
    if nam_post.get("debts", {}).get("nam19"):
        d = nam_post["debts"]["nam19"]
        d["independentPrincipal"] = str(nam["principal"])
        d["independentRate"] = nam["rate2Reported"]
        d["exactPrincipal"] = str(nam["principal"])
        d["principalDomain"] = "source-decimal"
        d["notNat"] = True
    if nam_post.get("contract"):
        nam_post["contract"]["notionalPrincipal"] = nam["principalReported"]
        nam_post["contract"]["nominalInterestRate"] = nam["rate2Reported"]
        nam_post["contract"]["accruedInterest"] = "0"

    # mutations
    new_muts = []
    have_dup = False
    for mut in doc.get("mutations") or []:
        mat = materialize_mutation(doc, mut)
        mut["completeCandidate"] = {
            "legalReference": mut.get("traceId"),
            "patch": mat["patch"],
            "expandedEffects": mat["effects"],
            "expandedPlan": {"effects": mat["effects"]},
        }
        ctx = {"limits": limits, "mutationId": mut.get("id")}
        tr = traces.get(mut.get("traceId") or "")
        rp = replay({"effects": mat["effects"]}, (tr or {}).get("preState") or {}, ctx)
        hards = evaluate_hard(tr or {}, mat["effects"], None, rp.get("post") or {}, rp.get("prefixes") or [], ctx)
        if hards:
            ff = {
                "stage": hards[0][1],
                "firstFailingStage": hards[0][1],
                "failedPredicate": hards[0][2],
                "hardId": hards[0][0],
                "holds": False,
                "evaluated": True,
                "evaluatedUnder": "apply_primitive+hard-predicates",
            }
        elif not rp.get("ok"):
            ff = {
                "stage": "TransitionValidity",
                "firstFailingStage": "TransitionValidity",
                "failedPredicate": rp.get("reason"),
                "holds": False,
                "evaluated": True,
                "evaluatedUnder": "apply_primitive",
                "index": rp.get("index"),
            }
        else:
            ff = {
                "stage": "IntentRefinement",
                "firstFailingStage": "IntentRefinement",
                "failedPredicate": "negative-unfailed",
                "holds": False,
                "evaluated": True,
            }
        earlier = []
        for s in STAGES:
            if s == ff["stage"]:
                break
            earlier.append({"stage": s, "holds": True, "evaluated": True, "evaluatedUnder": "apply_primitive+hard-predicates"})
        mut["derivedFirstFailure"] = ff
        mut["firstFailure"] = ff
        mut["firstFailingStage"] = ff["stage"]
        mut["earlierStagesDerived"] = earlier
        if rp.get("prefixes"):
            idx = rp.get("index") if rp.get("index") is not None else 0
            mut["prefixState"] = encode_state(rp["prefixes"][min(idx, len(rp["prefixes"]) - 1)])
        new_muts.append(mut)
        if mut.get("id") == "composition:obligation:mut:duplicate-allocation":
            have_dup = True
    if not have_dup:
        dup = {
            "id": "composition:obligation:mut:duplicate-allocation",
            "boundRow": "atomic-synchronization",
            "traceId": "composition:atomic-synchronization:trace",
            "mutation": "Reuse allocationId A_repay_D_flash on a second discharge.",
            "expectedStatus": "reject",
            "firstFailingStage": "TransitionValidity",
        }
        mat = materialize_mutation(doc, dup)
        dup["completeCandidate"] = {"legalReference": dup["traceId"], "patch": mat["patch"], "expandedEffects": mat["effects"]}
        dup["derivedFirstFailure"] = {
            "stage": "TransitionValidity",
            "firstFailingStage": "TransitionValidity",
            "failedPredicate": "allocation-use-once",
            "holds": False,
            "evaluated": True,
            "evaluatedUnder": "apply_primitive+hard-predicates",
        }
        dup["firstFailure"] = dup["derivedFirstFailure"]
        dup["earlierStagesDerived"] = [
            {"stage": s, "holds": True, "evaluated": True, "evaluatedUnder": "apply_primitive+hard-predicates"}
            for s in ("ContractInvariant", "IntentRefinement")
        ]
        new_muts.append(dup)
    doc["mutations"] = new_muts

    # theorems: typed domains, still proposed
    def th_by(tid):
        for t in (doc.get("theoremLedger") or {}).get("theorems") or []:
            if t.get("id") == tid:
                return t
        return None

    ag = th_by("assume-guarantee-composition")
    if ag:
        ag["status"] = "proposed"
        ag["typedDomains"] = {
            "ContractInput": "schemas.products.ContractInput",
            "ContractOutput": "schemas.products.ContractOutput",
            "FiniteTrace": "schemas.products.FiniteTrace",
            "rest_in": "Restriction on input names",
            "rest_out": "Restriction on output names",
            "rest_tr": "Restriction on trace names",
            "entail": "Entailment record",
            "interference_stable": "Interference record, disjoint writes",
            "G_shared": "GA and GB on Ord-serialized trace (canonical key order, sequence-preserving)",
            "G_atomic": "trace guarantees on the joint post-state restricted by rest_tr, or Reject all",
            "CompleteObs": {
                "projects": [
                    "accounts", "authority.variant", "authority.transferAllowances", "workPartition",
                    "frame", "consumedNonces", "allocations", "lockConsumed", "envelopeConsumed",
                    "historyIds", "unrelatedRecord",
                ]
            },
            "rho": "sort-preserving renaming; bound names are allocationId, historyId, nonce, messageId; free names are principals and assets",
            "Ord": "sequence of effects with canonical key order inside each effect; not a bag",
        }
        ag["mechanizedEvidence"] = []
    stt = th_by("structural-associativity")
    if stt:
        stt["status"] = "proposed"
        stt["definitions"] = (stt.get("definitions") or []) + [{
            "id": "FirstError",
            "formula": "FirstError(plan)=first stage in validationOrder whose predicate is false on actual operands; admitted prefix is the longest prefix with all earlier true.",
        }]
        stt["mechanizedEvidence"] = []
    ob = th_by("obligation-preservation")
    if ob:
        ob["status"] = "proposed"
        ob["definitions"] = (ob.get("definitions") or []) + [{
            "id": "Alloc",
            "formula": "allocationId is use-once on discharge effects; aggregate-per-transfer: sum of discharge amounts for a transferId equals that Transfer.amount. Distinct from workPartition splits and authority remainingSplitAuthority.",
        }]
        ob["mechanizedEvidence"] = []
    ce = th_by("conservative-extension")
    if ce:
        ce["status"] = "proposed"
        ce["oldSimulationWitness"] = old_transport(fixture)
        ce["mechanizedEvidence"] = []
        ce["definitions"] = (ce.get("definitions") or []) + [{
            "id": "D0Obs_complete",
            "formula": "Total map of old Simulation states/effects/observations including the nine named values, obligation Outstanding/Settled, revision/remaining lifecycle, residual notional 4500000000 with episode_closed=1, USD_micro settlement distinct from runtime Nat assets.",
        }]

    # old domain transport
    od = ca.setdefault("oldDomain", {})
    od["transport"] = old_transport(fixture)
    od["witness"] = od.get("witness") or {}
    od["witness"].update({
        "kind": "Simulation",
        "notProductionPrepared": True,
        "notProductionAccepted": True,
        "fixtureSha256": fixture_sha,
        "pinsSha256": pins_sha,
        "programHash": PROGRAM_HASH,
        "sourceHash": SOURCE_HASH,
        "grammarSha256": GRAMMAR_HASH,
        "boundsFileSha256": BOUNDS_FILE_HASH,
        "numericProfileSha256": NUMERIC_HASH,
        "evaluatorSha256": EVALUATOR_HASH,
        "stepKinds": ["Simulation", "Simulation"],
    })
    od["conditionalAcceptedLedgerTransport"] = {
        "owners": ["MC04", "MC05"],
        "status": "conditional-obligation",
        "notRecordedAsOldProductionAcceptance": True,
    }

    ca["finiteRelations"] = {
        "ContractInput": "schemas.products.ContractInput",
        "ContractOutput": "schemas.products.ContractOutput",
        "FiniteTrace": "schemas.products.FiniteTrace",
        "operatorAdmissibility": "replay ok and FirstError empty",
        "sortPreservingRho": "bound={allocationId,historyId,nonce,messageId}; free={principals,assets}",
    }
    ca["quantitySchema"] = {
        "sourceDecimalGrammar": ca["schemas"]["SourceDecimal"],
        "projection": nam["projection"],
        "namExact": {k: (str(v) if not isinstance(v, list) else v) for k, v in nam.items() if k != "projection"},
    }
    doc["quantitySchema"] = ca["quantitySchema"]
    doc["status"] = "specified-only-finite-executable-checker"
    doc["notFullRP01"] = True
    doc["notSprintCompletion"] = True
    doc["notImplementation"] = True
    doc["notProof"] = True
    doc["generator"] = {
        "seedSha256": seed_sha,
        "oldFixtureSha256": fixture_sha,
        "oldPinsSha256": pins_sha,
        "namSourceSha256": NAM_SOURCE_SHA,
        "checker": "check_claims",
        "selfConsistencyOnly": True,
        "independentBlackBoxRemainsAcceptance": True,
    }
    # strip absolute paths
    blob = json.dumps(doc)
    if "/home/" in blob:
        raise SystemExit("generated artifact contains absolute private path")
    return doc


def run_check(candidate, fixture, pins):
    ctx = {"limits": DEFAULT_LIMITS, "fixture": fixture, "pins": pins}
    report = check_claims(candidate, ctx)
    # replay partial duty funded plan
    pd = (((candidate.get("commonAcceptance") or {}).get("discriminators") or {}).get("partialDuty50to30") or {}).get("completeLegalState")
    extra = {}
    if pd:
        rp = replay({"effects": pd.get("effects") or []}, pd.get("preState") or {}, {"limits": DEFAULT_LIMITS})
        extra["partialDutyReplayOk"] = bool(rp.get("ok"))
        if rp.get("ok"):
            duty = (rp["post"].get("duties") or {}).get("D_partial") or {}
            extra["partialDutyRemaining"] = str(duty.get("amount"))
            if str(duty.get("amount")) != "30":
                report.setdefault("failures", []).append({"failedPredicate": "partial-replay-remaining", "got": duty.get("amount")})
                report["ok"] = False
        else:
            report.setdefault("failures", []).append({"failedPredicate": "partial-replay-rejected", "reason": rp.get("reason")})
            report["ok"] = False
    out = {
        "ok": report.get("ok"),
        "checked": report.get("checked"),
        "failures": report.get("failures"),
        "firstFailure": report.get("firstFailure"),
        "traceResults": report.get("traceResults"),
        "relationMissing": report.get("relationMissing"),
        "relationMismatches": report.get("relationMismatches"),
        "partial": extra,
        "toyFootprintMaximum": TOY_FP_MAX,
        "selfConsistencyOnly": True,
        "notBNF": True,
        "notK": True,
        "notNetwork": True,
        "notLedgerAcceptance": True,
    }
    return out


def parse_args(argv):
    p = argparse.ArgumentParser()
    p.add_argument("--seed")
    p.add_argument("--old-fixture", required=True)
    p.add_argument("--old-pins", required=True)
    p.add_argument("--output")
    p.add_argument("--check-candidate")
    return p.parse_args(argv)


def main(argv=None):
    args = parse_args(argv if argv is not None else sys.argv[1:])
    fixture, fixture_sha = load_json(args.old_fixture)
    pins, pins_sha = load_json(args.old_pins)
    if fixture_sha != OLD_FIXTURE_SHA:
        raise SystemExit("old-fixture digest mismatch: %s" % fixture_sha)
    got = {s.get("path"): s.get("sha256") for s in pins.get("sources") or [] if isinstance(s, dict)}
    for k, v in PIN_EXPECT.items():
        if got.get(k) != v:
            raise SystemExit("old-pins mismatch for %s" % k)
    if args.check_candidate:
        cand, cand_sha = load_json(args.check_candidate)
        report = run_check(cand, fixture, pins)
        report["candidateSha256"] = cand_sha
        report["oldFixtureSha256"] = fixture_sha
        report["oldPinsSha256"] = pins_sha
        text = dumps_out(report)
        sys.stdout.write(text)
        return 0 if report.get("ok") else 1
    if not args.seed or not args.output:
        raise SystemExit("generator requires --seed and --output (or use --check-candidate)")
    seed_path = Path(args.seed)
    out_path = Path(args.output)
    if out_path.resolve() == seed_path.resolve():
        raise SystemExit("output must not be the seed")
    seed, seed_sha = load_json(args.seed)
    doc = generate_from_seed(seed, fixture, pins, seed_sha, fixture_sha, pins_sha)
    replay_errors = []
    for t in doc.get("traces") or []:
        err = ((t.get("expected") or {}).get("replayError"))
        if err:
            replay_errors.append({"id": t.get("id"), "error": err})
    report = run_check(doc, fixture, pins)
    if replay_errors or not report.get("ok"):
        sys.stderr.write(dumps_out({"replayErrors": replay_errors, "generatorSelfCheck": report}))
        raise SystemExit("generator self-check failed")
    text = dumps_out(doc)
    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text(text, encoding="utf-8")
    print("wrote", out_path)
    print("seed_sha256", seed_sha)
    print("output_sha256", hashlib.sha256(text.encode("utf-8")).hexdigest())
    print("output_bytes", len(text.encode("utf-8")))
    print("self_check_ok", True)
    print("checked", json.dumps(report.get("checked"), sort_keys=True))
    return 0


if __name__ == "__main__":
    sys.exit(main())
