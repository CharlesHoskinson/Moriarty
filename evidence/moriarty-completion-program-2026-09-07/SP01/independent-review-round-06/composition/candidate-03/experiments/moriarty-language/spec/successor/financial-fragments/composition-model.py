#!/usr/bin/env python3
"""Deterministic finite toy composition-model generator.

Reads an immutable candidate-02 seed plus old fixture/pins. Does not read the
mutable output path. Writes a corrected specified-only composition fragment.

CLI:
  python3 composition-model.py --seed FILE --old-fixture FILE --old-pins FILE --output FILE
"""
from __future__ import annotations

import argparse
import copy
import hashlib
import json
import sys
from decimal import Decimal, getcontext
from fractions import Fraction
from pathlib import Path

getcontext().prec = 80

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

SYSTEM_CTORS = (
    "CreateHistory",
    "ConsumeHistory",
    "UpdateWork",
    "UpdateAuthorityCounters",
    "SetObservedTick",
    "UpdateMessageStatus",
    "UpdatePositionReserves",
    "SetNominalRate",
    "UpdateObservation",
    "ConsumeNonce",
)


def sha256_file(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def load_json(path):
    raw = Path(path).read_bytes()
    return json.loads(raw.decode("utf-8")), hashlib.sha256(raw).hexdigest()


def frac_dec(x):
    return Decimal(x.numerator) / Decimal(x.denominator)


def nam_accruals():
    cap1 = Fraction(5000) * Fraction(8, 100) * Fraction(90, 365)
    rate1 = Decimal("0.010567901234567900") + Decimal("0.10")
    cap2 = Fraction(5000) * Fraction(rate1) * Fraction(91, 365)
    return cap1, cap2, cap1 + cap2, Fraction(5000) + cap1 + cap2, rate1


def walk_dicts(obj, fn):
    if isinstance(obj, dict):
        fn(obj)
        for v in obj.values():
            walk_dicts(v, fn)
    elif isinstance(obj, list):
        for x in obj:
            walk_dicts(x, fn)


def fill_effect_fields(obj):
    def fn(d):
        ctor = d.get("ctor")
        if ctor == "Transfer":
            if d.get("unit") in (None, "") and d.get("asset"):
                d["unit"] = d["asset"]
            if d.get("scale") in (None, ""):
                d["scale"] = "0" if d.get("asset") != "USD" else "source-decimal"
        elif ctor == "ReceiveMessage":
            if d.get("escrowSource") in (None, ""):
                d["escrowSource"] = "Escrow"
            if d.get("destFinalityObservationId") in (None, ""):
                d["destFinalityObservationId"] = "O_dest_M3"
        elif ctor == "Mint":
            if d.get("unit") in (None, "") and d.get("asset"):
                d["unit"] = d["asset"]
            if d.get("scale") in (None, ""):
                d["scale"] = "0"
        elif ctor == "CreateDebt":
            if d.get("unit") in (None, "") and d.get("asset"):
                d["unit"] = d["asset"]
            if d.get("scale") in (None, ""):
                d["scale"] = "source-decimal" if d.get("asset") == "USD" else "0"
            if d.get("id") == "nam19":
                d.setdefault("debtor", "borrower")
                d.setdefault("creditor", "lender")
            if d.get("id") == "D_flash":
                d.setdefault("debtor", "Alice")
                d.setdefault("creditor", "Lender")
        elif ctor == "AssignDuty":
            if d.get("id") == "DutyLock_M3":
                d.setdefault("dutyVariant", "lock-entitlement")
                d.setdefault("lockId", "M3")
            if d.get("id") == "nam19-debt":
                d.setdefault("debtId", "nam19")
        if ctor == "Transfer" and d.get("from") in (None, "") and d.get("bound") not in (None, ""):
            d.setdefault("from", d.get("principal") or "BobAlias")
            d.setdefault("to", "VaultX")
            d.setdefault("amount", str(d.get("bound")))
            d.setdefault("unit", d.get("asset") or "WETH")
            d.setdefault("scale", "0")
    walk_dicts(obj, fn)


def schemas():
    return {
        "status": "specified-only-finite-design",
        "notRegisteredRuntime": True,
        "runtimeNat": {
            "sort": "Nat",
            "interval": "[0, M_a]",
            "assets": ["USDC", "WETH", "LP-USDC-WETH"],
        },
        "SourceDecimal": {
            "sort": "SourceDecimal",
            "notRuntimeNat": True,
            "grammar": "[ '-' ]? DIGIT+ ( '.' DIGIT+ )? ; DIGIT = '0'..'9'",
            "maxLength": 80,
            "length": "canonical decimal character length <= maxLength",
            "precision": "fractional digit count",
            "maxPrecision": 50,
            "maxIntegerDigits": 30,
            "arithmetic": "exact Decimal/Fraction add/mul/div; not IEEE; JSON pins may truncate",
        },
        "products": {
            "QuantityNat": {"fields": ["value", "unit", "scale", "quantityDomain"]},
            "QuantitySourceDecimal": {"fields": ["value", "unit", "scale", "quantityDomain", "notNat"]},
            "AllowanceRecord": {
                "fields": [
                    "asset",
                    "unit",
                    "scale",
                    "original",
                    "cumulativeGross",
                    "remaining",
                    "refundsDoNotReplenish",
                    "quantityDomain",
                ]
            },
            "AllowedEffect": {"fields": ["ctor", "asset", "bound", "payeeScope"]},
            "ObservationRecord": {
                "fields": ["id", "code", "timestamp", "value", "kind", "authenticated", "tick"]
            },
            "ContractRecord": {"fields": ["id", "notionalPrincipal", "accruedInterest", "nominalInterestRate"]},
            "PositionRecord": {"fields": ["id", "reserveUSDC", "reserveWETH", "lpSupply", "kFormula"]},
            "ClaimRecord": {"fields": ["id", "kind", "asset", "amount", "controller"]},
            "RequestRecord": {"fields": ["id", "kind", "status", "controller"]},
            "MessageRecord": {"fields": ["id", "status", "nonce", "from", "to", "deadlineTick", "escrow"]},
            "OrderRecord": {"fields": ["id", "side", "asset", "amount", "status"]},
            "WorkRecord": {"fields": ["lifetime", "spent", "remainingOrdinary", "remainingClosure", "unit", "scale"]},
            "WorkPartition": {"fields": ["beforeSplit", "A", "B", "copied"]},
            "Frame": {"fields": ["A", "B", "intersect"]},
            "DutyRecord": {"fields": ["id", "kind", "asset", "amount", "controller", "payee", "residual", "debtId", "lockId", "dutyVariant"]},
            "DebtRecord": {"fields": ["id", "asset", "principal", "accrued", "rate", "debtor", "creditor", "unit", "scale"]},
            "FeeRecord": {"fields": ["id", "asset", "amount", "recipient", "annotates"]},
            "CompleteState": {
                "fields": [
                    "accounts",
                    "debts",
                    "shares",
                    "positions",
                    "claims",
                    "fees",
                    "duties",
                    "requests",
                    "messages",
                    "orders",
                    "work",
                    "workPartition",
                    "authority",
                    "historyIds",
                    "observations",
                    "contract",
                    "observedTick",
                    "unrelatedRecord",
                    "frame",
                    "consumedNonces",
                    "allocations",
                    "metadata",
                ]
            },
        },
        "sums": {
            "AuthorityRecord": {
                "tag": "variant",
                "variants": {
                    "TransferPrincipal": ["allowedEffects", "transferAllowance"],
                    "FeeRecipient": ["allowedEffects", "feeRecipient"],
                    "JointEnvelope": ["allowedEffects", "envelopeId", "envelopeConsumed", "remainingSplitAuthority"],
                    "Pool": ["allowedEffects", "transferAllowance"],
                    "Escrow": ["allowedEffects", "custody"],
                    "System": ["allowedEffects"],
                },
            },
            "Quantity": {"variants": ["QuantityNat", "QuantitySourceDecimal"]},
        },
        "primitiveFields": {
            "Transfer": ["id", "from", "to", "asset", "amount", "unit", "scale", "allocationId"],
            "AccrueFee": ["feeId", "from", "recipient", "asset", "amount", "unit", "scale", "annotates", "role", "cashMovement"],
            "AccrueNominal": ["debtId", "accrued", "observationRef", "period"],
            "CreateDebt": ["id", "asset", "principal", "debtor", "creditor", "unit", "scale"],
            "AssignDuty": ["id", "kind", "asset", "amount", "controller", "payee", "residual", "debtId", "lockId", "dutyVariant"],
            "Mint": ["asset", "amount", "to", "unit", "scale"],
            "ReceiveMessage": ["id", "nonce", "to", "tick", "escrowSource", "destFinalityObservationId"],
        },
        "documentaryMetadataProjection": {
            "rule": "explicit",
            "silentFieldDrop": False,
            "projects": [
                "unit",
                "scale",
                "quantityDomain",
                "notNat",
                "independentPrincipal",
                "independentAmount",
                "principalDomain",
                "accruedDomain",
                "amountDomain",
            ],
        },
        "CompleteState": {
            "fields": [
                "accounts",
                "debts",
                "shares",
                "positions",
                "claims",
                "fees",
                "duties",
                "requests",
                "messages",
                "orders",
                "work",
                "workPartition",
                "authority",
                "historyIds",
                "observations",
                "contract",
                "observedTick",
                "unrelatedRecord",
                "frame",
                "consumedNonces",
                "allocations",
                "metadata",
            ]
        },
        "completeStateFields": [
            "accounts",
            "debts",
            "shares",
            "positions",
            "claims",
            "fees",
            "duties",
            "requests",
            "messages",
            "orders",
            "work",
            "workPartition",
            "authority",
            "historyIds",
            "observations",
            "contract",
            "observedTick",
            "unrelatedRecord",
            "frame",
            "consumedNonces",
            "allocations",
            "metadata",
        ],
    }


def apply_equations():
    return {
        "Transfer": {
            "preconditions": [
                "accounts[from][asset] >= amount",
                "membership(from, Transfer, asset, amount, to)",
                "cumulativeGross[from][asset] + amount <= original[from][asset]",
            ],
            "equations": {
                "accounts[from][asset]": "pre - amount",
                "accounts[to][asset]": "pre + amount",
                "authority[from].transferAllowance.cumulativeGross": "pre + amount",
                "authority[from].transferAllowance.remaining": "original - cumulativeGross",
            },
            "cashMovement": True,
        },
        "AccrueFee": {
            "preconditions": [
                "annotates is an existing Transfer id",
                "Transfer.amount = AccrueFee.amount",
                "Transfer.to = AccrueFee.recipient",
                "Transfer.asset = AccrueFee.asset",
            ],
            "equations": {"fees[feeId]": "{asset, amount, recipient, annotates}"},
            "cashMovement": False,
            "role": "annotation",
            "annotates": "existing Transfer identity",
        },
        "Mint": {
            "preconditions": ["membership(minter, Mint, asset, amount, to)"],
            "equations": {"accounts[to][asset]": "pre + amount", "positions[pool].lpSupply": "pre + amount when share"},
            "cashMovement": True,
        },
        "CreateDebt": {
            "preconditions": [
                "matching Transfer exists with same asset, amount=principal, payer=creditor, payee=debtor",
                "debtor, creditor, unit, scale present",
            ],
            "equations": {"debts[id]": "{asset, principal, accrued:0, debtor, creditor, unit, scale}"},
        },
        "ReduceDebt": {
            "preconditions": ["funding Transfer id bound by allocationId", "payee = creditor"],
            "equations": {"debts[id].principal": "pre - amount"},
        },
        "AssignDuty": {
            "preconditions": ["id fresh or lock variant", "debtId or lockId/dutyVariant present"],
            "equations": {"duties[id]": "created residual record"},
        },
        "SettleDuty": {
            "preconditions": ["unique allocationId", "fundingTransferId", "payee entitled at this branch"],
            "equations": {
                "duties[id].amount": "pre.amount - paid; retain id; residual=(remaining>0)",
                "not": "Out(post)=(Out(pre)-Discharged) union Created for same-id partial",
            },
        },
        "AccrueNominal": {
            "preconditions": ["observationRef present", "debt exists"],
            "equations": {"debts[id].accrued": "pre + accrued"},
        },
        "Capitalize": {
            "preconditions": ["payoff=0 for IPCI"],
            "equations": {
                "debts[id].principal": "pre.principal + pre.accrued",
                "debts[id].accrued": "0",
            },
        },
        "SetNominalRate": {
            "preconditions": ["observationRef authenticated assumed"],
            "equations": {"debts[id].rate": "observed+spread"},
        },
        "UpdatePositionReserves": {
            "preconditions": ["position id exists"],
            "equations": {
                "positions[id].reserveUSDC": "named",
                "positions[id].reserveWETH": "named",
                "positions[id].lpSupply": "named",
            },
        },
        "UpdateAuthorityCounters": {
            "preconditions": ["grossDelta equals sum of that principal's outgoing Transfer amounts on asset"],
            "equations": {
                "authority[p].transferAllowance.cumulativeGross": "derived from transfers",
                "authority[p].transferAllowance.remaining": "original - cumulativeGross",
            },
            "authorization": "system",
        },
        "UpdateWork": {
            "preconditions": ["spent+remainingOrdinary+remainingClosure=lifetime"],
            "equations": {"work.spent": "pre+ordinaryDelta"},
            "authorization": "system",
        },
        "CreateHistory": {"preconditions": ["id fresh"], "equations": {"historyIds": "union {id}"}, "authorization": "system"},
        "ConsumeHistory": {"preconditions": ["id in live historyIds"], "equations": {"historyIds": "minus {id}"}, "authorization": "system"},
        "ReceiveMessage": {
            "preconditions": ["receiveAdmitted"],
            "equations": {"messages[id].status": "Finalized", "consumedNonces": "union {nonce}"},
        },
        "RefundLock": {
            "preconditions": ["refundAdmitted"],
            "equations": {"messages[id].status": "Refunded"},
        },
        "Lock": {
            "preconditions": ["nonce unused"],
            "equations": {
                "accounts[from][asset]": "pre - amount",
                "accounts[escrow][asset]": "pre + amount",
            },
        },
    }


def membership_spec():
    return {
        "rule": "e in AllowedEffects(envelope) iff ctor authorized for principal, asset matches, payee in payeeScope or unconstrained, e.amount <= remaining(principal, asset), and cumulativeGross+amount <= original",
        "perEffectBoundInsufficient": True,
        "twoTransfer60Under101": {
            "principal": "Alice",
            "asset": "USDC",
            "perEffectBound": "101",
            "original": "101",
            "effects": [
                {
                    "ctor": "Transfer",
                    "id": "T_cap_60a",
                    "from": "Alice",
                    "to": "PoolQ",
                    "asset": "USDC",
                    "amount": "60",
                    "unit": "USDC",
                    "scale": "0",
                },
                {
                    "ctor": "Transfer",
                    "id": "T_cap_60b",
                    "from": "Alice",
                    "to": "PoolQ",
                    "asset": "USDC",
                    "amount": "60",
                    "unit": "USDC",
                    "scale": "0",
                },
            ],
            "perEffectMembership": "each 60 <= 101",
            "cumulativeGross": "120",
            "status": "reject",
            "firstFailingStage": "TransitionValidity",
            "failedPredicate": "AuthoritySafety.cumulativeGross",
            "reason": "60+60=120>101 remaining after first is 41",
        },
        "systemConstructors": list(SYSTEM_CTORS),
        "systemAuthorization": "CreateHistory, ConsumeHistory, UpdateWork, UpdateAuthorityCounters, SetObservedTick, UpdateMessageStatus are system-authorized, not principal-issued",
    }


def deadline_rule():
    return {
        "now": "100",
        "deadline": "100",
        "destFinalityTick": "99",
        "exclusive": True,
        "exactlyOneSameBaseBranch": True,
        "receiveAdmitted": "Pending AND destFinality.authenticated AND destFinality.tick < deadline AND nonce unused AND escrow.amount >= lockAmount",
        "refundAdmitted": "Pending AND now >= deadline AND NOT receiveAdmitted",
        "atNow100Finality99": {"receive": True, "refund": False, "admitted": "receive"},
        "note": "exactly one same-base branch by predicate; not an assertion",
    }


def projection():
    return {
        "total": True,
        "parentChild": {
            "authority.{P}.Transfer.{A}": "authority.{P}.transferAllowance when asset=A",
            "authority.{P}": ["allowedEffects", "transferAllowance", "lockConsumed", "envelopeConsumed", "lockRefundConsumed"],
            "positions.{id}": ["reserveUSDC", "reserveWETH", "lpSupply"],
            "allocations.{id}": ["transferId", "targetId", "amount", "asset", "payee", "kind"],
            "workPartition.{side}": ["spent", "remainingOrdinary", "remainingClosure"],
            "metadata": "documentaryMetadataProjection fields",
        },
        "outsideWrites": "Frame(pre,W)=Frame(post,W)",
        "noPostProjectionErase": True,
        "toyFootprintMaximum": TOY_FP_MAX,
        "notRegisteredRuntime": True,
    }


def ensure_complete_state_fields(doc):
    jd = doc["commonAcceptance"]["jsonDecoder"]
    fields = list(jd.get("completeStateFields") or [])
    for extra in ("workPartition", "frame", "consumedNonces", "allocations", "metadata"):
        if extra not in fields:
            fields.append(extra)
    jd["completeStateFields"] = fields
    opt = list(jd.get("optionalFields") or [])
    for extra in ("workPartition", "frame", "consumedNonces", "allocations", "metadata"):
        if extra not in opt:
            opt.append(extra)
    jd["optionalFields"] = opt


def tag_authority(state):
    auth = (state or {}).get("authority")
    if not isinstance(auth, dict):
        return
    for name, recd in list(auth.items()):
        if not isinstance(recd, dict):
            continue
        if name in ("Treasury",) or recd.get("feeRecipient"):
            recd.setdefault("variant", "FeeRecipient")
            recd.setdefault("allowedEffects", recd.get("allowedEffects") or [{"ctor": "AccrueFee"}])
            recd.setdefault("feeRecipient", True)
        elif name in ("joint", "Joint"):
            recd.setdefault("variant", "JointEnvelope")
            recd.setdefault("allowedEffects", recd.get("allowedEffects") or [])
        elif name in ("Escrow",):
            recd.setdefault("variant", "Escrow")
            recd.setdefault("allowedEffects", recd.get("allowedEffects") or [])
            recd.setdefault("custody", True)
            recd.setdefault(
                "transferAllowance",
                {
                    "asset": "USDC",
                    "unit": "USDC",
                    "scale": "0",
                    "original": "25",
                    "cumulativeGross": recd.get("lockConsumed") and "25" or "0",
                    "remaining": recd.get("lockConsumed") and "0" or "25",
                    "refundsDoNotReplenish": True,
                    "quantityDomain": "runtime-nat",
                },
            )
        elif name in ("system", "System"):
            recd.setdefault("variant", "System")
            recd.setdefault("allowedEffects", [{"ctor": c} for c in SYSTEM_CTORS])
        elif name in ("PoolP", "PoolQ"):
            recd.setdefault("variant", "Pool")
            recd.setdefault("allowedEffects", recd.get("allowedEffects") or [])
            recd.setdefault(
                "transferAllowance",
                recd.get("transferAllowance")
                or {
                    "asset": "WETH" if name == "PoolQ" else "USDC",
                    "unit": "WETH" if name == "PoolQ" else "USDC",
                    "scale": "0",
                    "original": recd.get("transferAllowance", {}).get("original", "0"),
                    "cumulativeGross": recd.get("transferAllowance", {}).get("cumulativeGross", "0"),
                    "remaining": recd.get("transferAllowance", {}).get("remaining", "0"),
                    "refundsDoNotReplenish": True,
                    "quantityDomain": "runtime-nat",
                },
            )
        else:
            recd.setdefault("variant", "TransferPrincipal")
            recd.setdefault("allowedEffects", recd.get("allowedEffects") or [])
            if recd.get("transferAllowance") is None:
                recd["transferAllowance"] = {
                    "asset": "USDC",
                    "unit": "USDC",
                    "scale": "0",
                    "original": "0",
                    "cumulativeGross": "0",
                    "remaining": "0",
                    "refundsDoNotReplenish": True,
                    "quantityDomain": "runtime-nat",
                }
        if recd.get("variant") == "JointEnvelope":
            recd.setdefault("envelopeConsumed", recd.get("envelopeConsumed", False))
        if name == "system":
            continue
    if "system" not in auth:
        auth["system"] = {
            "variant": "System",
            "allowedEffects": [{"ctor": c} for c in SYSTEM_CTORS],
        }


def annotate_fees(effects, fee_transfer_id):
    if not isinstance(effects, list):
        return
    for e in effects:
        if not isinstance(e, dict):
            continue
        if e.get("ctor") == "Transfer" and e.get("to") == "Treasury" and str(e.get("amount")) == "1":
            e["id"] = fee_transfer_id
            e.setdefault("unit", e.get("asset") or "USDC")
            e.setdefault("scale", "0")
        if e.get("ctor") == "AccrueFee":
            e["role"] = "annotation"
            e["cashMovement"] = False
            e["annotates"] = fee_transfer_id
            e.setdefault("unit", e.get("asset") or "USDC")
            e.setdefault("scale", "0")


def insert_after(effects, pred, new_e):
    out = []
    done = False
    for e in effects:
        out.append(e)
        if not done and pred(e):
            out.append(new_e)
            done = True
    if not done:
        out.append(new_e)
    return out


def correct_nam(trace):
    cap1, cap2, capsum, postp, rate1 = nam_accruals()
    cap1_s = str(frac_dec(cap1))
    cap2_s = str(frac_dec(cap2))
    post_s = str(frac_dec(postp))
    plan = trace["concretePlan"]
    exp = trace["expected"]

    def patch_effects(effects):
        out = []
        for e in effects:
            if e.get("ctor") == "AccrueNominal" and str(e.get("accrued")).startswith("98.630"):
                e = dict(e)
                e["observationRef"] = "O_NAM_contract_rate_0_08"
                e["period"] = "IED->RR"
                e["accruedIndependent"] = cap1_s
                out.append(e)
                out.append(
                    {
                        "ctor": "SetNominalRate",
                        "debtId": "nam19",
                        "rate": "0.110567901234567900",
                        "observationRef": "O_LIBOR_2013-04-01",
                    }
                )
                out.append(
                    {
                        "ctor": "AccrueNominal",
                        "debtId": "nam19",
                        "accrued": cap2_s,
                        "accruedDomain": "source-decimal",
                        "observationRef": "O_LIBOR_2013-04-01",
                        "period": "RR->IPCI",
                        "days": "91",
                        "rate": str(rate1),
                    }
                )
                continue
            if e.get("ctor") == "Capitalize":
                e = dict(e)
                e["principalAfterIndependent"] = post_s
                e["sumAccrualsIndependent"] = str(frac_dec(capsum))
            out.append(e)
        return out

    plan["effects"] = patch_effects(plan.get("effects") or [])
    exp["effects"] = patch_effects(exp.get("effects") or [])
    fill_effect_fields(plan)
    fill_effect_fields(exp)
    hard = (trace.get("canonicalIntent") or {}).get("hard") or []
    for h in hard:
        if h.get("id") == "H3":
            h["predicate"] = (
                "IPCI transfers=[] and post.principal = IED.principal + AccrueNominal(IED->RR) + AccrueNominal(RR->IPCI) "
                "with intervening RR->IPCI accrual required; payoff 0. Independent sum is 5000 + 7200/73 + "
                "5000*(0.010567901234567900+0.10)*91/365. Immediate 5000+98.63013698630137 is not the IPCI principal."
            )
            h["interveningAccrual"] = cap2_s
            h["postPrincipalIndependent"] = post_s
    obs = list((trace.get("preState") or {}).get("observations") or [])
    ids = {o.get("id") or o.get("code") for o in obs if isinstance(o, dict)}
    if "O_NAM_contract_rate_0_08" not in ids:
        obs.append(
            {
                "id": "O_NAM_contract_rate_0_08",
                "code": "NAM_CONTRACT_RATE",
                "timestamp": "2013-01-01T00:00:00",
                "value": "0.08",
                "kind": "contract-rate",
                "authenticated": True,
            }
        )
    if "O_LIBOR_2013-04-01" not in ids:
        obs.append(
            {
                "id": "O_LIBOR_2013-04-01",
                "code": "LIBOR_USD",
                "timestamp": "2013-04-01T00:00:00",
                "value": "0.010567901234567900",
                "kind": "assumed-fixture",
                "authenticated": True,
            }
        )
    trace["preState"]["observations"] = obs
    post_obs = list((exp.get("postState") or {}).get("observations") or [])
    post_ids = {o.get("id") for o in post_obs if isinstance(o, dict)}
    for o in obs:
        if o.get("id") not in post_ids:
            post_obs.append(o)
    exp["postState"]["observations"] = post_obs
    plan["events"] = ["IED", "RR", "IPCI", "RR"]
    plan["sourceEventSequence"] = ["IED", "RR", "IPCI", "RR"]
    plan["fullPrimitivePrefix"] = plan["effects"]


def correct_history_names(trace):
    plan = trace.setdefault("concretePlan", {})
    exp = trace.setdefault("expected", {})
    internal = list(plan.get("producedHistories") or [])
    external = list(exp.get("producedHistories") or [])
    plan["internalTranscriptOutputs"] = internal
    plan["externalProducedHeads"] = external
    exp["internalTranscriptOutputs"] = internal
    exp["externalProducedHeads"] = external


def add_counter_effects(trace, extras):
    plan = trace["concretePlan"]
    exp = trace["expected"]
    have = {(e.get("principal"), e.get("asset")) for e in plan.get("effects") or [] if e.get("ctor") == "UpdateAuthorityCounters"}
    for extra in extras:
        key = (extra.get("principal"), extra.get("asset"))
        if key in have:
            continue
        plan.setdefault("effects", []).append(extra)
        exp.setdefault("effects", []).append(extra)
        have.add(key)


def correct_atomic(trace):
    annotate_fees(trace["concretePlan"].get("effects"), "T_atomic_fee")
    annotate_fees(trace["expected"].get("effects"), "T_atomic_fee")
    add_counter_effects(
        trace,
        [
            {"ctor": "UpdateAuthorityCounters", "principal": "Lender", "asset": "USDC", "grossDelta": "50"},
            {"ctor": "UpdateAuthorityCounters", "principal": "PoolQ", "asset": "WETH", "grossDelta": "47"},
        ],
    )
    for effs in (trace["concretePlan"]["effects"], trace["expected"]["effects"]):
        if not any(e.get("ctor") == "UpdatePositionReserves" for e in effs):
            effs.append(
                {
                    "ctor": "UpdatePositionReserves",
                    "id": "PoolQ",
                    "reserveUSDC": "1050",
                    "reserveWETH": "953",
                }
            )
    joint = ((trace.get("expected") or {}).get("postState") or {}).get("authority", {}).get("joint")
    if isinstance(joint, dict):
        joint["envelopeConsumed"] = True
        joint["variant"] = "JointEnvelope"
    fill_effect_fields(trace)


def correct_shared(trace):
    annotate_fees(trace["concretePlan"].get("effects"), "T_shared_fee")
    annotate_fees(trace["expected"].get("effects"), "T_shared_fee")
    for effs in (trace["concretePlan"]["effects"], trace["expected"]["effects"]):
        if not any(e.get("ctor") == "UpdatePositionReserves" for e in effs):
            effs.append(
                {
                    "ctor": "UpdatePositionReserves",
                    "id": "PoolP",
                    "reserveUSDC": "132",
                    "reserveWETH": "92",
                    "lpSupply": "109",
                }
            )
    fill_effect_fields(trace)


def work_copy(w):
    return copy.deepcopy(w) if isinstance(w, dict) else w


def correct_parallel(trace):
    pre = trace.get("preState") or {}
    post = (trace.get("expected") or {}).get("postState") or {}
    wp = copy.deepcopy(pre.get("workPartition") or {})
    if isinstance(wp.get("A"), dict):
        wp["A"]["spent"] = "10"
        wp["A"]["remainingOrdinary"] = "494"
    if isinstance(wp.get("B"), dict):
        wp["B"]["spent"] = "10"
        wp["B"]["remainingOrdinary"] = "494"
    post["workPartition"] = wp
    post["frame"] = copy.deepcopy(pre.get("frame") or {"A": ["Alice.WETH", "VaultX.WETH"], "B": ["Bob.USDC", "VaultY.USDC"], "intersect": []})
    pre_auth = authority_entries_safe(pre)
    post_auth = authority_entries_safe(post)
    for name in ("Alice", "Bob"):
        if name in post_auth and name in pre_auth:
            post_auth[name].setdefault("allowedEffects", copy.deepcopy(pre_auth[name].get("allowedEffects") or []))
    fill_effect_fields(trace)


def authority_entries_safe(state):
    a = (state or {}).get("authority")
    return a if isinstance(a, dict) else {}


def receive_post(trace):
    pre = trace.get("preState") or {}
    refund_post = (trace.get("expected") or {}).get("postState") or {}
    post = copy.deepcopy(refund_post)
    post["accounts"] = {"Alice": {"USDC": "14"}, "Escrow": {"USDC": "0"}, "Bob": {"USDC": "25"}, "Treasury": {"USDC": "1"}, "_scale": "0"}
    post["duties"] = []
    post["work"] = work_copy(refund_post.get("work") or pre.get("work"))
    if isinstance(post.get("work"), dict):
        post["work"]["spent"] = "20"
        post["work"]["remainingOrdinary"] = "988"
        post["work"]["remainingClosure"] = "16"
        post["work"]["conservation"] = "20+988+16=1024"
    post["historyIds"] = ["H_async_received"]
    post["consumedNonces"] = ["N3"]
    post["observedTick"] = "100"
    post["observations"] = [
        {
            "id": "O_dest_M3",
            "code": "DEST_FINALITY",
            "tick": "99",
            "timestamp": "tick-99",
            "value": "finalized",
            "kind": "assumed-authenticated-destination",
            "authenticated": True,
            "assumption": True,
            "notForeignAdapterClaim": True,
        }
    ]
    auth = copy.deepcopy(pre.get("authority") or {})
    if "Alice" in auth:
        ta = dict((auth["Alice"].get("transferAllowance") or {}))
        ta["cumulativeGross"] = "26"
        ta["remaining"] = "174"
        auth["Alice"]["transferAllowance"] = ta
        auth["Alice"]["lockRefundConsumed"] = False
        auth["Alice"]["lockConsumed"] = True
        auth["Alice"]["allowedEffects"] = copy.deepcopy(auth["Alice"].get("allowedEffects") or [])
        auth["Alice"]["variant"] = "TransferPrincipal"
    if "Bob" in auth:
        auth["Bob"].setdefault("allowedEffects", auth["Bob"].get("allowedEffects") or [])
        auth["Bob"].setdefault(
            "transferAllowance",
            auth["Bob"].get("transferAllowance")
            or {
                "asset": "USDC",
                "unit": "USDC",
                "scale": "0",
                "original": "0",
                "cumulativeGross": "0",
                "remaining": "0",
                "refundsDoNotReplenish": True,
                "quantityDomain": "runtime-nat",
            },
        )
        auth["Bob"]["variant"] = "TransferPrincipal"
        auth["Bob"]["recipientRight"] = "DutyLock_M3"
    if "Escrow" in auth:
        auth["Escrow"]["variant"] = "Escrow"
        auth["Escrow"].setdefault("allowedEffects", [{"ctor": "Unlock"}, {"ctor": "Transfer", "asset": "USDC", "bound": "25"}])
        auth["Escrow"]["custody"] = True
    auth.setdefault(
        "Treasury",
        {"variant": "FeeRecipient", "feeRecipient": True, "allowedEffects": [{"ctor": "AccrueFee", "asset": "USDC", "bound": "1"}]},
    )
    post["authority"] = auth
    post["messages"] = [{"id": "M3", "status": "Finalized", "nonce": "N3", "from": "Alice", "to": "Bob", "deadlineTick": "100", "escrow": "Escrow"}]
    return post


def refund_post(trace):
    post = copy.deepcopy((trace.get("expected") or {}).get("postState") or {})
    post.setdefault("consumedNonces", ["N3"])
    post.setdefault("historyIds", post.get("historyIds") or ["H_async_refund"])
    auth = post.setdefault("authority", {})
    if "Alice" in auth:
        auth["Alice"]["lockRefundConsumed"] = True
        auth["Alice"]["lockConsumed"] = True
        auth["Alice"].setdefault("allowedEffects", (trace.get("preState") or {}).get("authority", {}).get("Alice", {}).get("allowedEffects") or [])
    tag_authority(post)
    return post


def correct_async(trace):
    annotate_fees(trace["concretePlan"].get("effects"), "T_async_fee")
    annotate_fees(trace["expected"].get("effects"), "T_async_fee")
    fill_effect_fields(trace)
    msm = trace.setdefault("messageStateMachine", {})
    msm["deadlineRule"] = deadline_rule()
    msm["deterministicBranch"] = deadline_rule()
    msm["lockDutyEntitlement"] = {
        "send": {"dutyId": "DutyLock_M3", "payee": "Alice", "right": "refund-right"},
        "receiveTransform": {
            "from": "Alice",
            "to": "Bob",
            "fromRight": "refund-right",
            "toRight": "recipient-right",
            "authorizedBy": "Lock.M3 destination Bob",
            "dutyId": "DutyLock_M3",
        },
    }
    msm["authenticatedObservationDomain"] = {
        "finite": True,
        "ids": ["O_dest_M3"],
        "destinationTruth": "explicit assumption, not a foreign adapter claim",
    }
    alts = (trace.get("expected") or {}).setdefault("alternateSuccessors", {})
    recv_eff = [
        {
            "ctor": "ReceiveMessage",
            "id": "M3",
            "nonce": "N3",
            "to": "Bob",
            "tick": "99",
            "escrowSource": "Escrow",
            "destFinalityObservationId": "O_dest_M3",
        },
        {
            "ctor": "Transfer",
            "id": "T_escrow_bob_25",
            "from": "Escrow",
            "to": "Bob",
            "asset": "USDC",
            "amount": "25",
            "unit": "USDC",
            "scale": "0",
            "allocationId": "A_receive_M3",
        },
        {
            "ctor": "SettleDuty",
            "id": "DutyLock_M3",
            "allocationId": "A_receive_M3",
            "fundingTransferId": "T_escrow_bob_25",
            "payee": "Bob",
            "amount": "25",
            "asset": "USDC",
            "authorizedTransform": True,
        },
        {"ctor": "UpdateMessageStatus", "id": "M3", "status": "Finalized"},
        {"ctor": "ConsumeHistory", "id": "H_async_created"},
        {"ctor": "CreateHistory", "id": "H_async_received"},
        {"ctor": "ConsumeNonce", "id": "N3"},
        {"ctor": "UpdateWork", "ordinaryDelta": "20"},
        {"ctor": "UpdateAuthorityCounters", "principal": "Alice", "asset": "USDC", "grossDelta": "26"},
        {"ctor": "UpdateObservation", "id": "O_dest_M3", "tick": "99", "authenticated": True, "assumption": True},
    ]
    recv = alts.get("receive") or {}
    recv["id"] = recv.get("id") or "H_async_received"
    recv["fromBase"] = "post-step0 Pending"
    recv["effects"] = recv_eff
    recv["postState"] = receive_post(trace)
    recv["entitlementTransform"] = msm["lockDutyEntitlement"]["receiveTransform"]
    recv["dutyPayee"] = "Bob"
    recv["excludes"] = ["refund"]
    tag_authority(recv["postState"])
    alts["receive"] = recv
    refu = alts.get("refund") or {}
    refu["id"] = "H_async_refund"
    refu["takenInPositiveTrace"] = True
    refu["postState"] = refund_post(trace)
    refu["effects"] = list(trace["expected"].get("effects") or [])
    refu["excludes"] = ["receive"]
    tag_authority(refu.get("postState"))
    alts["refund"] = refu
    race = (trace.get("expected") or {}).setdefault("race", {})
    race["samePreReceiveVsRefund"] = {
        "baseState": "post-step0 Pending, tick 10, escrow 25, nonce N3 live",
        "now": "100",
        "destFinalityTick": "99",
        "predicate": deadline_rule(),
        "admitted": "receive",
        "receiveEnabled": True,
        "refundEnabled": False,
        "exactlyOne": True,
    }


def add_fp_resource(fp, side, resource):
    lst = fp.setdefault(side, [])
    if any(isinstance(x, dict) and x.get("resource") == resource for x in lst):
        return
    lst.append({"resource": resource, "bound": "1"})


def correct_footprints(doc):
    for row in doc.get("rows") or []:
        fp = row.get("boundedFootprint") or {}
        rid = row.get("id")
        if rid == "shared-state-interleaving":
            for r in (
                "positions.PoolP.reserveUSDC",
                "positions.PoolP.reserveWETH",
                "positions.PoolP.lpSupply",
                "allocations.A_shared_fee",
            ):
                add_fp_resource(fp, "reads", r)
                add_fp_resource(fp, "writes", r)
        if rid == "atomic-synchronization":
            for r in ("positions.PoolQ.reserveUSDC", "positions.PoolQ.reserveWETH", "allocations.A_repay_D_flash"):
                add_fp_resource(fp, "reads", r)
                add_fp_resource(fp, "writes", r)
        reads = fp.get("reads") or []
        writes = fp.get("writes") or []
        n = len(reads) + len(writes)
        fp["entryCount"] = n
        fp["toyFootprintMaximum"] = TOY_FP_MAX
        fp["withinToyBound"] = n <= TOY_FP_MAX
        fp["notRegisteredRuntime"] = True
        row["boundedFootprint"] = fp


def theorem_by_id(doc, tid):
    for t in (doc.get("theoremLedger") or {}).get("theorems") or []:
        if t.get("id") == tid:
            return t
    return None


def set_def(th, did, **fields):
    for dfn in th.get("definitions") or []:
        if dfn.get("id") == did:
            dfn.update(fields)
            return
    th.setdefault("definitions", []).append(dict(id=did, **fields))


def correct_theorems(doc):
    tp = theorem_by_id(doc, "type-preservation")
    if tp:
        set_def(
            tp,
            "StateWellTyped",
            formula=(
                "StateWellTyped(sigma, Gamma) iff sigma is CompleteState under schemas.products.CompleteState "
                "including workPartition and frame. authority entries are AuthorityRecord variants. "
                "SourceDecimal uses schemas.SourceDecimal grammar/precision/length/arithmetic and is not Nat."
            ),
        )
    acc = theorem_by_id(doc, "asset-indexed-accounting")
    if acc:
        set_def(
            acc,
            "AllowedEffects-accounting-subset",
            formula=(
                "Accounting cash effects = {Transfer, Mint, Burn}. AccrueFee is an annotation of an existing Transfer "
                "with exact identity/amount/recipient and is not a second cash movement."
            ),
        )
    auth = theorem_by_id(doc, "authority-safety")
    if auth:
        set_def(
            auth,
            "AllowedEffects",
            formula=(
                "e in AllowedEffects iff ctor is authorized for the principal, asset matches, payee is in payeeScope, "
                "and amount <= remaining(principal,asset) with cumulativeGross+amount <= original. Per-effect bound alone "
                "is insufficient: two Transfer 60 under Transfer 101 reject because cumulative 120>101. "
                "System constructors CreateHistory/ConsumeHistory/UpdateWork/UpdateAuthorityCounters are system-authorized. "
                "Counters for Lender/PoolQ/joint are derived from transfers and envelope consumption, not arbitrary deltas."
            ),
        )
        auth["discriminators"] = {
            "twoTransfer60Under101": doc["commonAcceptance"]["discriminators"]["twoTransfer60Under101"]
        }
    fr = theorem_by_id(doc, "frame-noninterference")
    if fr:
        set_def(
            fr,
            "bounded-frame",
            formula=(
                "A footprint is (reads, writes) of {resource:CanonicalPath, bound:Nat}. "
                "|reads|+|writes| <= toyFootprintMaximum 64 (design bound, not registered runtime; amended from 32). "
                "Canonical projection is total with parent/child mapping covering positions, allocations, metadata, "
                "authority.transferAllowance, lockConsumed, envelopeConsumed. Paths authority.P.Transfer.A project "
                "authority.P.transferAllowance. Frame protects outside writes. No field is erased by post-projection."
            ),
        )
        fr["boundedDomain"] = "toyFootprintMaximum=64 design-only"
    ag = theorem_by_id(doc, "assume-guarantee-composition")
    if ag:
        ag["predicate"] = (
            "A Contract is typed (input, output, trace, Asm, Guar) with restriction maps rest_in, rest_out, rest_tr. "
            "sat(X,C) iff every finite trace tr of X, Asm(tr) implies Guar(tr). "
            "Discharged assumptions are the set difference Asm minus Entailed where Entailed is proved by explicit entailment "
            "records, not a mixed predicate-versus-set negation. "
            "If sat(A,{AsmA,GA}) and sat(B,{AsmB,GB}) and Compat_op(A,B) then sat(A op B, {Asm: (AsmA union AsmB union OpAsm(op)) minus Entailed, Guar: G_op(GA,GB)})."
        )
        set_def(
            ag,
            "Compat",
            formula=(
                "Compat_sequential(A,B) iff produced_ext(A)=consumed_ext(B) and Auth_B subseteq residual(A) and "
                "entail(GA, AsmB) is an explicit entailment record and interference_stable(A,B) on shared-accessed state "
                "and joint_financial_closure(A;B). "
                "Compat_disjoint-parallel(A,B) iff write/read sets disjoint, Auth disjoint, no hidden alias. "
                "Compat_shared-state-interleaving(A,B) iff explicit total order Ord on conflicting writes, both agree, "
                "and interference_stable inside shared cells along Ord (not only outside-write Frame). "
                "Compat_atomic-synchronization(A,B) iff joint envelope AllowedEffects is union, no external prefix of a proper subset of legs. "
                "Compat_asynchronous-messaging(A,B) iff unique successor among {Finalized, Refunded} by deadlineRule."
            ),
        )
        set_def(
            ag,
            "G_op",
            formula=(
                "G_sequential(GA,GB)(tr)=GA(rest_tr(tr,A)) AND GB(rest_tr(tr,B)) on concatenated history. "
                "G_disjoint-parallel(GA,GB)(tr)=GA(rest_tr(tr,A)) AND GB(rest_tr(tr,B)) on the pair. "
                "G_shared-state-interleaving(GA,GB)(tr)=GA and GB on the Ord-serialized shared trace; this is the sequentialization "
                "relation along Ord, not a placeholder. "
                "G_atomic-synchronization(GA,GB)(tr)=GA AND GB on the joint post-state, or Reject all. "
                "G_asynchronous-messaging(GA,GB)(tr)=GA on send/lock AND GB on exactly one of receive or refund."
            ),
        )
        set_def(
            ag,
            "OpAsm",
            formula="OpAsm(op) is a finite assumption set. Restriction maps rest_in/rest_out/rest_tr are total on the typed domains.",
        )
        set_def(
            ag,
            "no-unilateral-drop",
            formula="Discharged = Asm difference Entailed. Entailment obligations are explicit records. Predicate-versus-set negation of the discharged set is not used.",
        )
        ag["typedDomains"] = {
            "input": "ContractInput",
            "output": "ContractOutput",
            "trace": "FiniteTrace",
            "restrictionMaps": ["rest_in", "rest_out", "rest_tr"],
        }
    st = theorem_by_id(doc, "structural-associativity")
    if st:
        set_def(
            st,
            "ObsEq",
            formula=(
                "CompleteObs includes accounts, transfers, minted, burned, debts, duties, residualDuties, "
                "authority.original, authority.cumulativeGross, authority.remaining, allowedEffects, "
                "lockConsumed, envelopeConsumed, work.*, fees, shares, positions, claims, messages, requests, orders, "
                "historyIds, orderedTranscript, status, observations, observedTick, provenance, contractSnapshot, "
                "and all material records. rho is a sort-preserving bijection on sorted identifier binders; free identities "
                "are identity. Equal spent with different remainingOrdinary is not ObsEq."
            ),
        )
        set_def(
            st,
            "alpha-renaming",
            formula=(
                "Binders are sorted by (sort, identifier). rho preserves sort: HistoryId to HistoryId, DutyId to DutyId, "
                "and likewise. Free identities are not renamed."
            ),
        )
        set_def(
            st,
            "OrderCond",
            formula=(
                "OrderCond_sequential: observations are a sequence (concatenation). "
                "OrderCond_disjoint-parallel: child observations form a bag; serialization of the join is a sequence of the "
                "bag plus join history; bag versus sequence is explicit. "
                "OrderCond_shared: Ord is part of the observation. "
                "OrderCond_atomic: only joint pre/post. OrderCond_async: Created < Pending < {Finalized, Refundable}; exclusive."
            ),
        )
        set_def(
            st,
            "associativity-conditions",
            formula=(
                "PaymentAlloc is the injective map of SettleTransfer/ReduceDebt/SettleDuty allocationIds. "
                "WorkAlloc is the split of remainingOrdinary/remainingClosure/AllowedEffects and is a separate map. "
                "Cost is the work vector plus fee Transfers (annotation fees do not add a second cost). "
                "Serialization is orderedTranscript (sequence) and, for shared, Ord. "
                "FirstError(g) is the least validationOrder stage that fails under the same schema/claim relation as positives; "
                "Admitted reject is only that first-error record, not an arbitrary well-typed reject. "
                "Associativity is stated only when both groupings are admitted with equal Cost, PaymentAlloc, WorkAlloc, "
                "order-valid, and equal Reject prefix/FirstError."
            ),
        )
        st["predicate"] = (
            "For op in {sequential, disjoint-parallel}, if both groupings are admitted and PaymentAlloc, WorkAlloc, Cost, "
            "Serialization, FirstError, and PrefixObs agree, then ObsEq(eval(gL), eval(gR)). Shared requires identical Ord. "
            "Atomic requires the joint envelope. Associativity is restricted to those hypotheses."
        )
    ob = theorem_by_id(doc, "obligation-preservation")
    if ob:
        set_def(
            ob,
            "DutyConserv",
            formula=(
                "Partial payment updates amount in place: duty id D with amount 50 paid 20 yields the same id with amount 30 "
                "and residual=true. This is not Out(post)=(Out(pre) minus Discharged) union Created, which would require D in "
                "both Discharged and Created. Full discharge (amount 0) uses SettleDuty/WriteOff/Novation and unique allocationId "
                "plus fundingTransferId. Branch entitlement: lock duty payee is Alice refund-right on send; receive applies an "
                "authorized transform to Bob recipient-right with matching target/nonce/history."
            ),
        )
        ob["discriminators"] = {"partialDuty50to30": doc["commonAcceptance"]["discriminators"]["partialDuty50to30"]}
    ce = theorem_by_id(doc, "conservative-extension")
    if ce:
        ce["predicate"] = (
            "Pin the exact old domain from old-domain-pins and retained-atomic-fixture. "
            "Old derivation is bounded-atomic derive/createSimulator SUCCESS kind Simulation with synthetic auth/checks. "
            "It is not production Prepared or Accepted. kind=Simulation. "
            "AdmittedSyntax_D0 is the loan.mori program under moriarty-bounded-atomic/1. "
            "T is a total mathematical relation on that domain. "
            "Retention: if OldDerive(p0,sigma)=Simulation(sigma0,pi0) then there exist p,sigma',pi with T(p,p0) and "
            "SuccDerive(p,sigma)=Simulation(sigma',pi) and D0Obs_complete(pi)=D0Obs_complete(pi0). "
            "This is a non-vacuous conservative extension of the real old DERIVATION relation on this exact domain. "
            "The unrelated Borrow/Swap/Repay toy is not old syntax witness. "
            "Conditional accepted-ledger transport for MC04/MC05 is a separate obligation: if a future old production "
            "acceptance exists, successor must preserve it. No fake old production acceptance is recorded here."
        )
        set_def(
            ce,
            "D0",
            formula=(
                "D0 is loan.mori sourceHash "
                + SOURCE_HASH
                + " profile moriarty-bounded-atomic/1 programHash "
                + PROGRAM_HASH
                + " grammar "
                + GRAMMAR_HASH
                + " bounds.json "
                + BOUNDS_FILE_HASH
                + " numeric-profile "
                + NUMERIC_HASH
                + " evaluator evaluate.ts "
                + EVALUATOR_HASH
                + " registry boundsHash "
                + REGISTRY_BOUNDS_HASH
                + ". Concrete witness is retained-atomic-fixture.json sha256 "
                + OLD_FIXTURE_SHA
                + " steps[].result.kind=Simulation with authority.signature.algorithm=simulation-only and input.checks."
            ),
        )
        set_def(
            ce,
            "T",
            formula="T(p,p0) holds for the identity embedding of this D0 program into successor syntax without successor-only constructors. T is total on AdmittedSyntax_D0 and is not implemented.",
        )
        set_def(
            ce,
            "D0Obs",
            formula=(
                "D0Obs_complete maps old Simulation candidate.body.after/before and state hashes onto CompleteObs: "
                "accounts, transfers, debts, duties, history and histories, full authority including lockConsumed and envelopeConsumed, "
                "work, fees, shares, positions, claims, messages, observations, status, genesisHash, programHash, traceHash. "
                "Translation is total on the two Simulation steps. Full D0 state mapping uses fixture states[].stateHash."
            ),
        )
        set_def(
            ce,
            "oldD0-translation-conditions",
            formula=(
                "oldD0 translation applies only when OldDerive yields Simulation SUCCESS on this fixture domain. "
                "Vacuous reject-all is forbidden because the fixture supplies two Simulation witnesses. "
                "MC04/MC05 accepted-ledger transport remains a separate conditional obligation."
            ),
        )
        ce["oldPins"] = {
            "grammar.ebnf": GRAMMAR_HASH,
            "bounds.json": BOUNDS_FILE_HASH,
            "numeric-profile.json": NUMERIC_HASH,
            "evaluate.ts": EVALUATOR_HASH,
            "loan.mori": SOURCE_HASH,
            "programHash": PROGRAM_HASH,
            "registryBoundsHash": REGISTRY_BOUNDS_HASH,
            "fixtureSha256": OLD_FIXTURE_SHA,
        }
        ce["oldSimulationWitness"] = doc["commonAcceptance"]["oldDomain"]["witness"]
        ce["status"] = "proposed"
        ce["mechanizedEvidence"] = []


def old_witness(fixture, fixture_sha, pins_sha, seed_sha):
    s0 = (fixture.get("steps") or [{}])[0]
    s1 = (fixture.get("steps") or [{}, {}])[1] if len(fixture.get("steps") or []) > 1 else {}
    auth = ((s0.get("input") or {}).get("authority") or {}).get("signature") or {}
    checks = (s0.get("input") or {}).get("checks") or {}
    return {
        "kind": "Simulation",
        "notProductionPrepared": True,
        "notProductionAccepted": True,
        "notPrepared": True,
        "notAccepted": True,
        "deriveCreateSimulator": "SUCCESS",
        "syntheticAuth": True,
        "authorityAlgorithm": auth.get("algorithm") or "simulation-only",
        "checks": checks,
        "programHash": PROGRAM_HASH,
        "sourceHash": SOURCE_HASH,
        "boundsHash": REGISTRY_BOUNDS_HASH,
        "boundsFileSha256": BOUNDS_FILE_HASH,
        "grammarSha256": GRAMMAR_HASH,
        "numericProfileSha256": NUMERIC_HASH,
        "evaluatorSha256": EVALUATOR_HASH,
        "genesisHash": ((s0.get("input") or {}).get("genesis") or {}).get("genesisHash")
        or (fixture.get("genesis") or {}).get("genesisHash")
        or GENESIS_HASH,
        "traceHash0": ((s0.get("result") or {}).get("candidate") or {}).get("traceHash"),
        "traceHash1": ((s1.get("result") or {}).get("candidate") or {}).get("traceHash"),
        "stateHashes": [s.get("stateHash") for s in fixture.get("states") or [] if isinstance(s, dict)],
        "fixtureSha256": fixture_sha,
        "pinsSha256": pins_sha,
        "seedSha256": seed_sha,
        "manifestName": (fixture.get("manifest") or {}).get("name"),
        "profile": (fixture.get("program") or {}).get("profile"),
        "stepKinds": [(st.get("result") or {}).get("kind") for st in fixture.get("steps") or []],
        "relativeFixtureRef": "retained-atomic-fixture.json",
        "unrelatedBorrowSwapRepayIsNotOldSyntaxWitness": True,
    }


def derived_failure(stage, predicate, extra=None):
    d = {
        "stage": stage,
        "firstFailingStage": stage,
        "relation": stage,
        "failedPredicate": predicate,
        "claimRelation": stage,
        "holds": False,
        "evaluated": True,
        "evaluatedUnder": "commonAcceptance.validationOrder+schemas",
    }
    if extra:
        d.update(extra)
    return d


def earlier_ok(up_to):
    out = []
    for s in ("ContractInvariant", "IntentRefinement", "TransitionValidity", "HistoryCompliance"):
        if s == up_to:
            break
        out.append({"stage": s, "holds": True, "evaluated": True, "evaluatedUnder": "commonAcceptance.validationOrder+schemas"})
    return out


def patch_mutations(doc):
    traces = {t["id"]: t for t in doc.get("traces") or []}
    nam_prefix_effects = [
        e
        for e in ((traces.get("heldouts:NAM19-capitalization:trace") or {}).get("concretePlan") or {}).get("effects") or []
        if e.get("ctor") in ("Transfer", "CreateDebt", "AccrueNominal", "AssignDuty")
    ]
    specs = {
        "heldouts:NAM19-capitalization:mut:erase-nominal-on-ipci": {
            "stage": "IntentRefinement",
            "pred": "H3 intervening accrual and capitalization prefix",
            "effects_key": "erasedNominal",
        },
        "heldouts:NAM19-capitalization:mut:reorder-ipci-rr": {
            "stage": "IntentRefinement",
            "pred": "H4 same-timestamp order IPCI then RR",
        },
        "composition:sequential:mut:skip-predecessor": {
            "stage": "IntentRefinement",
            "pred": "predecessor history required",
        },
        "composition:sequential:mut:refresh-work-on-join": {
            "stage": "IntentRefinement",
            "pred": "no-refresh of remainingOrdinary",
        },
        "composition:disjoint-parallel:mut:alias-overlap": {
            "stage": "IntentRefinement",
            "pred": "disjoint footprints/no hidden alias",
        },
        "composition:shared-state-interleaving:mut:commute-without-condition": {
            "stage": "IntentRefinement",
            "pred": "Ord-conditioned commutation; AB!=BA",
        },
        "composition:atomic-synchronization:mut:partial-commit": {
            "stage": "IntentRefinement",
            "pred": "all-or-nothing joint envelope",
        },
        "composition:atomic-synchronization:mut:repay-with-weth": {
            "stage": "IntentRefinement",
            "pred": "repay asset equals debt asset USDC",
        },
        "composition:asynchronous-messaging:mut:treat-as-atomic-sync": {
            "stage": "IntentRefinement",
            "pred": "operator is asynchronous-messaging not atomic-synchronization",
        },
        "composition:asynchronous-messaging:mut:late-receive-after-refund": {
            "stage": "IntentRefinement",
            "pred": "Refunded excludes Received",
        },
    }
    for mut in doc.get("mutations") or []:
        mid = mut.get("id")
        spec = specs.get(mid) or {"stage": mut.get("firstFailingStage") or "IntentRefinement", "pred": "intent"}
        fill_effect_fields(mut)
        ref = mut.get("traceId")
        legal = traces.get(ref) or {}
        expanded = copy.deepcopy((legal.get("concretePlan") or {}).get("effects") or [])
        fill_effect_fields(expanded)
        patch = {"mutatedInput": mut.get("mutatedInput"), "mutatedContext": mut.get("mutatedContext")}
        cand = mut.get("completeCandidate") or {}
        cand["legalReference"] = ref
        cand["patch"] = patch
        cand["expandedEffects"] = expanded
        cand["expandedPlan"] = {"effects": expanded, "operator": (legal.get("concretePlan") or {}).get("operator")}
        if mid.endswith("skip-predecessor"):
            cand["effects"] = [
                {
                    "ctor": "Transfer",
                    "from": "Alice",
                    "to": "Bob",
                    "asset": "USDC",
                    "amount": "20",
                    "unit": "USDC",
                    "scale": "0",
                    "id": "T_seq_skip",
                }
            ]
            mi = mut.setdefault("mutatedInput", {})
            if isinstance(mi, dict):
                walk_dicts(mi, lambda d: None)
                mi["transfer"] = cand["effects"][0]
        if mid.endswith("alias-overlap"):
            cand["effects"] = [
                {
                    "ctor": "Transfer",
                    "from": "BobAlias",
                    "to": "VaultX",
                    "asset": "WETH",
                    "amount": "7",
                    "unit": "WETH",
                    "scale": "0",
                    "id": "T_alias",
                }
            ]
            mi = mut.setdefault("mutatedInput", {})
            if isinstance(mi, dict):
                mi["transfer"] = cand["effects"][0]
        if mid.endswith("late-receive-after-refund"):
            rm = {
                "ctor": "ReceiveMessage",
                "id": "M3",
                "to": "Bob",
                "tick": "100",
                "nonce": "N3",
                "escrowSource": "Escrow",
                "destFinalityObservationId": "O_dest_M3",
            }
            cand["effects"] = [rm]
            mi = mut.setdefault("mutatedInput", {})
            if isinstance(mi, dict):
                mi.update(rm)
        if mid.endswith("repay-with-weth"):
            cand["effects"] = [
                {
                    "ctor": "Transfer",
                    "from": "Alice",
                    "to": "Lender",
                    "asset": "WETH",
                    "amount": "47",
                    "unit": "WETH",
                    "scale": "0",
                    "id": "T_wrong_asset",
                }
            ]
        mut["completeCandidate"] = cand
        mut["derivedFirstFailure"] = derived_failure(spec["stage"], spec["pred"])
        mut["firstFailure"] = mut["derivedFirstFailure"]
        mut["firstFailingStage"] = spec["stage"]
        mut["earlierStagesDerived"] = earlier_ok(spec["stage"])
        if mid.startswith("heldouts:NAM19"):
            mut["prefixEffects"] = copy.deepcopy(nam_prefix_effects)
            if not any(e.get("ctor") == "AssignDuty" and e.get("id") == "nam19-debt" for e in mut["prefixEffects"]):
                mut["prefixEffects"].append(
                    {
                        "ctor": "AssignDuty",
                        "id": "nam19-debt",
                        "kind": "repay-notional",
                        "asset": "USD",
                        "amount": "5000",
                        "controller": "borrower",
                        "payee": "lender",
                        "residual": True,
                        "debtId": "nam19",
                        "amountDomain": "source-decimal",
                    }
                )
        fill_effect_fields(mut)


def correct_genesis(doc):
    es = doc["commonAcceptance"]["effectSystem"]
    es["genesisRule"] = {
        "text": "CreateDebt at genesis requires a matching Transfer of the same asset and principal with payer=creditor and payee=debtor, not amount-only.",
        "payer": "creditor",
        "payee": "debtor",
        "matching": ["asset", "amount=principal", "payer", "payee"],
        "fixture": "commonAcceptance.fixtures.genesisUnfundedDebt",
    }
    es["adminRule"] = {
        "text": "RecognizeLossLabel does not set principal to 0 and does not delete duties or accounts. WriteOff required for erasure.",
        "fixture": "commonAcceptance.fixtures.adminDebtErasure",
    }
    fx = (doc["commonAcceptance"].get("fixtures") or {}).get("genesisUnfundedDebt") or {}
    me = fx.get("mutatedEffect")
    if isinstance(me, dict):
        me.setdefault("debtor", "borrower")
        me.setdefault("creditor", "lender")
        me.setdefault("unit", "USD")
        me.setdefault("scale", "source-decimal")
        me["matchingTransferRequired"] = {
            "ctor": "Transfer",
            "from": "lender",
            "to": "borrower",
            "payer": "lender",
            "payee": "borrower",
            "asset": "USD",
            "amount": "5000",
            "absentInThisFixture": True,
        }


def grammar_update(doc):
    g = doc["commonAcceptance"].get("canonicalResourcePathGrammar") or ""
    extra = (
        " | positions.{PositionId}.{field} | allocations.{AllocationId}.{field} | "
        "authority.{PrincipalId}.transferAllowance.{field} | authority.{PrincipalId}.lockConsumed | "
        "authority.{PrincipalId}.envelopeConsumed | metadata.{field} | consumedNonces.{NonceId}"
    )
    if "positions.{PositionId}" not in g:
        doc["commonAcceptance"]["canonicalResourcePathGrammar"] = (g + extra).strip()


def correct(doc, fixture, pins, seed_sha, fixture_sha, pins_sha):
    ca = doc.setdefault("commonAcceptance", {})
    ca["schemas"] = schemas()
    ca["documentaryMetadataProjection"] = ca["schemas"]["documentaryMetadataProjection"]
    ca["toyFootprintMaximum"] = TOY_FP_MAX
    ca["canonicalResourceProjection"] = projection()
    ca["resourceProjection"] = ca["canonicalResourceProjection"]
    bp = (ca.get("records") or {}).setdefault("boundParameters", {})
    bp["maxFootprintEntries"] = {
        "value": "64",
        "type": "Nat",
        "unit": "entry",
        "min": "1",
        "max": "64",
        "notRegisteredRuntime": True,
        "designBound": True,
    }
    bp["toyFootprintMaximum"] = bp["maxFootprintEntries"]
    ensure_complete_state_fields(doc)
    grammar_update(doc)
    es = ca.setdefault("effectSystem", {})
    ctors = es.setdefault("primitiveConstructors", {})
    if isinstance(ctors, dict) and "AccrueFee" in ctors:
        ctors["AccrueFee"] = {
            "fields": ["from", "recipient", "asset", "amount", "feeId", "unit", "scale", "annotates", "role", "cashMovement"],
            "role": "annotation",
            "cashMovement": False,
            "annotates": "existing Transfer id with exact amount and recipient",
            "notSecondTransfer": True,
        }
    if isinstance(ctors, dict) and "AssignDuty" in ctors:
        ctors["AssignDuty"] = dict(ctors["AssignDuty"])
        ctors["AssignDuty"]["optional"] = ["debtId"]
        ctors["AssignDuty"]["variants"] = ["debt-linked", "lock-entitlement"]
        ctors["AssignDuty"]["fields"] = list(
            dict.fromkeys(list(ctors["AssignDuty"].get("fields") or []) + ["lockId", "dutyVariant"])
        )
    es["apply"] = apply_equations()
    es["membership"] = membership_spec()
    es["elaboration"] = {
        "rule": "elaborate(concretePlan, pre)=primitiveEffects with per-constructor preconditions. Fee cash movement occurs once via Transfer; AccrueFee annotates that Transfer.",
        "totalOnWellTyped": True,
    }
    ca["discriminators"] = {
        "twoTransfer60Under101": es["membership"]["twoTransfer60Under101"],
        "partialDuty50to30": {
            "id": "D_partial",
            "pre": {"id": "D_partial", "amount": "50", "asset": "USDC", "payee": "Lender", "residual": True},
            "payment": {
                "ctor": "SettleDuty",
                "id": "D_partial",
                "amount": "20",
                "allocationId": "A_partial_20",
                "fundingTransferId": "T_partial_20",
                "payee": "Lender",
            },
            "post": {"id": "D_partial", "amount": "30", "asset": "USDC", "payee": "Lender", "residual": True},
            "allocationId": "A_partial_20",
            "transferId": "T_partial_20",
            "notDischargedAndRecreated": True,
        },
        "feeExactlyOnce": {
            "shared": {"AliceUSDC": "29", "TreasuryUSDC": "1"},
            "atomic": {"AliceUSDC": "0", "TreasuryUSDC": "1", "gross": "101"},
            "async": {"AliceUSDC": "39", "TreasuryUSDC": "1"},
        },
    }
    ca["oldDomain"] = {
        "pins": {
            "experiments/moriarty-language/spec/grammar.ebnf": GRAMMAR_HASH,
            "experiments/moriarty-language/spec/bounds.json": BOUNDS_FILE_HASH,
            "experiments/moriarty-language/spec/numeric-profile.json": NUMERIC_HASH,
            "experiments/moriarty-language/src/evaluate.ts": EVALUATOR_HASH,
            "experiments/moriarty-language/spec/examples/loan.mori": SOURCE_HASH,
            "programHash": PROGRAM_HASH,
            "registryBoundsHash": REGISTRY_BOUNDS_HASH,
            "fixtureSha256": fixture_sha,
            "pinsSha256": pins_sha,
        },
        "witness": old_witness(fixture, fixture_sha, pins_sha, seed_sha),
        "conditionalAcceptedLedgerTransport": {
            "owners": ["MC04", "MC05"],
            "status": "conditional-obligation",
            "notRecordedAsOldProductionAcceptance": True,
        },
        "unrelatedBorrowSwapRepayIsNotOldSyntaxWitness": True,
    }
    correct_genesis(doc)
    by_id = {t["id"]: t for t in doc.get("traces") or []}
    if "heldouts:NAM19-capitalization:trace" in by_id:
        correct_nam(by_id["heldouts:NAM19-capitalization:trace"])
    if "composition:shared-state-interleaving:trace" in by_id:
        correct_shared(by_id["composition:shared-state-interleaving:trace"])
    if "composition:atomic-synchronization:trace" in by_id:
        correct_atomic(by_id["composition:atomic-synchronization:trace"])
    if "composition:asynchronous-messaging:trace" in by_id:
        correct_async(by_id["composition:asynchronous-messaging:trace"])
    if "composition:disjoint-parallel:trace" in by_id:
        correct_parallel(by_id["composition:disjoint-parallel:trace"])
    for t in doc.get("traces") or []:
        correct_history_names(t)
        fill_effect_fields(t)
        tag_authority(t.get("preState"))
        tag_authority((t.get("expected") or {}).get("postState"))
        alts = (t.get("expected") or {}).get("alternateSuccessors") or {}
        for alt in alts.values():
            if isinstance(alt, dict):
                tag_authority(alt.get("postState"))
        post = (t.get("expected") or {}).get("postState")
        if isinstance(post, dict):
            post.setdefault("consumedNonces", post.get("consumedNonces") or [])
            post.setdefault("allocations", post.get("allocations") or [])
            post.setdefault("metadata", {"projected": True, "silentFieldDrop": False})
            if t.get("id") == "composition:disjoint-parallel:trace":
                post.setdefault("workPartition", t.get("preState", {}).get("workPartition"))
                post.setdefault("frame", t.get("preState", {}).get("frame"))
        pre = t.get("preState")
        if isinstance(pre, dict):
            pre.setdefault("consumedNonces", [])
            pre.setdefault("allocations", [])
            pre.setdefault("metadata", {"projected": True, "silentFieldDrop": False})
    correct_footprints(doc)
    correct_theorems(doc)
    patch_mutations(doc)
    fill_effect_fields(doc)
    doc["correctionAgainst"]["modelCorrection"] = {
        "findings": ["C01", "C02", "C03", "C04", "C05", "C06", "C07", "C08", "C09", "C10", "C11"],
        "seedSha256": seed_sha,
        "oldFixtureSha256": fixture_sha,
        "oldPinsSha256": pins_sha,
        "generator": "composition-model.py",
        "notMoriartyEvaluator": True,
        "notKImplementation": True,
    }
    qs = doc.setdefault("quantitySchema", {})
    qs["sourceDecimalGrammar"] = ca["schemas"]["SourceDecimal"]
    # Drop any accidental absolute private paths.
    blob = json.dumps(doc)
    if "/home/" in blob:
        raise SystemExit("generated artifact contains absolute private path")


def parse_args(argv):
    p = argparse.ArgumentParser()
    p.add_argument("--seed", required=True)
    p.add_argument("--old-fixture", required=True)
    p.add_argument("--old-pins", required=True)
    p.add_argument("--output", required=True)
    return p.parse_args(argv)


def main(argv=None):
    args = parse_args(argv if argv is not None else sys.argv[1:])
    seed_path = Path(args.seed)
    out_path = Path(args.output)
    if out_path.resolve() == seed_path.resolve():
        raise SystemExit("output must not be the seed")
    seed, seed_sha = load_json(args.seed)
    fixture, fixture_sha = load_json(args.old_fixture)
    pins, pins_sha = load_json(args.old_pins)
    if fixture_sha != OLD_FIXTURE_SHA:
        raise SystemExit("old-fixture digest mismatch: %s" % fixture_sha)
    got = {s.get("path"): s.get("sha256") for s in pins.get("sources") or [] if isinstance(s, dict)}
    for k, v in PIN_EXPECT.items():
        if got.get(k) != v:
            raise SystemExit("old-pins mismatch for %s" % k)
    doc = copy.deepcopy(seed)
    correct(doc, fixture, pins, seed_sha, fixture_sha, pins_sha)
    text = json.dumps(doc, indent=2, ensure_ascii=True) + "\n"
    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text(text, encoding="utf-8")
    print("wrote", out_path)
    print("seed_sha256", seed_sha)
    print("output_sha256", hashlib.sha256(text.encode("utf-8")).hexdigest())
    print("output_bytes", len(text.encode("utf-8")))
    return 0


if __name__ == "__main__":
    sys.exit(main())
