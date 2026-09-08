#!/usr/bin/env python3
"""Independent read-only checks for the SP01.3 successor proposal.

Does not import generate-signing-examples.py. Expectations are derived from
the GPT-6 candidate02 review, the declared formula, and schema $defs.
These tests verify design reconstruction, not a language evaluator.
"""
from __future__ import annotations

import hashlib
import json
import re
import sys
from copy import deepcopy
from pathlib import Path

HERE = Path(__file__).resolve().parent
SCHEMA_PATH = HERE / "signing-display-schema.json"
EXAMPLES_PATH = HERE / "signing-examples.json"
CONTRACT_PATH = HERE / "semantic-contract.md"
DECISIONS_PATH = HERE / "semantic-decisions.json"

UINT64_MAX = (1 << 64) - 1
UINT128_MAX = (1 << 128) - 1
SINT128_MIN = -(1 << 127)
SINT128_MAX = (1 << 127) - 1
SECONDS_PER_YEAR = 31536000
PRINCIPAL = 5_000_000_000
RATE_MANTISSA = 1
RATE_SCALE = 1
PERIOD = 2_592_000  # 30 * 86400
EXPECTED_ACCRUED = (PRINCIPAL * RATE_MANTISSA * PERIOD) // (
    (10 ** RATE_SCALE) * SECONDS_PER_YEAR
)
FORBIDDEN_VALUE = 533_972_602
CORE_OPS = (
    "Transfer",
    "Fee",
    "Exchange",
    "Mint",
    "Burn",
    "Lock",
    "Unlock",
    "DebtCreate",
    "DebtAccrue",
    "DebtRepay",
    "DebtWriteOff",
    "DebtCapitalize",
    "ShareMint",
    "ShareBurn",
    "ShareConvert",
    "PositionOpen",
    "PositionAdjust",
    "PositionClose",
    "RequestOpen",
    "RequestFulfill",
    "RequestClaim",
    "RequestCancel",
    "MessageSend",
    "MessageReceive",
    "MessageRefund",
    "EventClaimOpen",
    "EventClaimResolve",
    "EventClaimSettle",
    "RewardAccrue",
    "RewardClaim",
    "Slash",
    "Rebase",
    "Observe",
    "Genesis",
    "Activate",
    "Pause",
    "Revoke",
    "Migrate",
)
PREIMAGE_TYPES = {
    "profileBody": "ProfileBody",
    "programBody": "ProgramBody",
    "requiredClaims": "ClaimArray",
    "policyBody": "PolicyBody",
    "genesisBody": "GenesisBody",
    "unbornStateBody": "StateBody",
    "genesisStateBody": "StateBody",
    "swapStateBody": "StateBody",
    "loanStateBody": "StateBody",
    "accruePostStateBody": "StateBody",
    "outcomePostStateBody": "StateBody",
    "observationSet": "ObservationSet",
    "consumptionSwap": "ConsumptionIdBody",
    "consumptionIntent": "ConsumptionIdBody",
    "consumptionGenesis": "ConsumptionIdBody",
    "consumptionAccrue": "ConsumptionIdBody",
    "emptySuccessor": "SuccessorRecord",
    "swapSuccessor": "SuccessorRecord",
    "accrueSuccessor": "SuccessorRecord",
    "outcomeSuccessor": "SuccessorRecord",
    "genesisSuccessor": "SuccessorRecord",
    "swapExecutionBody": "ExecutionBody",
    "genesisExecutionBody": "ExecutionBody",
    "accrueExecutionBody": "ExecutionBody",
    "outcomeSelectedPlan": "ExecutionBody",
    "assumptions": "AssumptionSet",
    "effectsSwap": "EffectsBody",
    "effectsGenesis": "EffectsBody",
    "effectsAccrue": "EffectsBody",
    "effectsOutcome": "EffectsBody",
    "residualSwap": "ResidualCapability",
    "residualGenesis": "ResidualCapability",
    "residualAccrue": "ResidualCapability",
    "residualOutcome": "ResidualCapability",
    "authorityWrapperExact": "AuthorityWrapper",
    "authorityWrapperGenesis": "AuthorityWrapper",
    "authorityWrapperAccrue": "AuthorityWrapper",
    "authorityWrapperOutcome": "AuthorityWrapper",
    "preparedSwap": "PreparedBody",
    "preparedGenesis": "PreparedBody",
    "preparedAccrue": "PreparedBody",
    "preparedOutcome": "PreparedBody",
    "proofContextSwap": "ProofContext",
    "acceptanceSwap": "AcceptanceBody",
    "acceptanceGenesis": "AcceptanceBody",
    "acceptanceAccrue": "AcceptanceBody",
    "acceptanceOutcome": "AcceptanceBody",
}
DOMAINS = {
    "ProfileBody": "MORIARTY-SUCC-BOUNDS/0",
    "ProgramBody": "MORIARTY-SUCC-PROGRAM/0",
    "ClaimArray": "MORIARTY-SUCC-CLAIMS/0",
    "PolicyBody": "MORIARTY-SUCC-POLICY/0",
    "GenesisBody": "MORIARTY-SUCC-GENESIS/0",
    "StateBody": "MORIARTY-SUCC-STATE/0",
    "ObservationSet": "MORIARTY-SUCC-OBS/0",
    "ExecutionBody": "MORIARTY-SUCC-EXEC-BODY/0",
    "AuthorityWrapper": "MORIARTY-SUCC-AUTHORITY/0",
    "PreparedBody": "MORIARTY-SUCC-TRACE/0",
    "ProofContext": "MORIARTY-SUCC-PROOF-CTX/0",
    "AcceptanceBody": "MORIARTY-SUCC-ACCEPTANCE/0",
    "SuccessorRecord": "MORIARTY-SUCC-SUCCESSOR/0",
    "ConsumptionIdBody": "MORIARTY-SUCC-CONSUMPTION/0",
    "AssumptionSet": "MORIARTY-SUCC-ASSUMPTIONS/0",
    "EffectsBody": "MORIARTY-SUCC-EFFECTS/0",
    "ResidualCapability": "MORIARTY-SUCC-RESIDUAL/0",
}
ZERO = "0" * 64
FALLBACK = {"signed-field", "last-segment"}


def canonical(obj) -> str:
    return json.dumps(obj, sort_keys=True, separators=(",", ":"), ensure_ascii=False)


def framed_hash(domain: str, obj) -> str:
    payload = canonical(obj).encode("utf-8")
    return hashlib.sha256(domain.encode("utf-8") + b"\x00" + payload).hexdigest()


def raw_hash(text: str) -> str:
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


def load_json(path: Path):
    return json.loads(path.read_text(encoding="utf-8"))


def valid_by_id(examples, vid):
    for row in examples["valid"]:
        if row["id"] == vid:
            return row
    raise KeyError(vid)


def invalid_by_id(examples, iid):
    for row in examples["invalid"]:
        if row["id"] == iid:
            return row
    raise KeyError(iid)


def parse_doc(row):
    return json.loads(row["canonicalUtf8"])


def set_path(obj, dotted, value):
    parts = dotted.split(".")
    cur = obj
    for p in parts[:-1]:
        if p.isdigit():
            cur = cur[int(p)]
        else:
            cur = cur[p]
    last = parts[-1]
    if last.isdigit():
        cur[int(last)] = value
    else:
        cur[last] = value


def reconstruct(examples, inv):
    base = valid_by_id(examples, inv["base"])
    doc = parse_doc(base)
    sets = inv.get("sets") or {}
    if isinstance(sets, dict):
        for path, value in sets.items():
            if path == "executionBody" and isinstance(value, dict):
                doc["executionBody"] = value
            else:
                set_path(doc, path, value)
    return doc


def walk_leaves(value, path=""):
    if isinstance(value, dict):
        if len(value) == 0:
            yield path, value
            return
        for key in sorted(value.keys()):
            child = f"{path}.{key}" if path else key
            yield from walk_leaves(value[key], child)
        return
    if isinstance(value, list):
        if len(value) == 0:
            yield path, value
            return
        for i, item in enumerate(value):
            yield from walk_leaves(item, f"{path}.{i}")
        return
    yield path, value


def domain_ok(name, text):
    if not isinstance(text, str):
        return False
    try:
        n = int(text, 10)
    except ValueError:
        return False
    if name == "UInt64":
        return 0 <= n <= UINT64_MAX
    if name == "UInt128":
        return 0 <= n <= UINT128_MAX
    if name == "SInt128":
        return SINT128_MIN <= n <= SINT128_MAX
    return False


def schema_validator(schema):
    from jsonschema import Draft202012Validator

    return Draft202012Validator(schema)


def def_validator(schema, def_name):
    from jsonschema import Draft202012Validator

    if def_name == "ClaimArray":
        sub = {
            "$schema": schema["$schema"],
            "$id": schema["$id"] + "#claim-array",
            "type": "array",
            "items": {"$ref": "#/$defs/ClaimRequirement"},
            "$defs": schema["$defs"],
        }
        return Draft202012Validator(sub)
    if def_name not in schema["$defs"]:
        return None
    sub = {
        "$schema": schema["$schema"],
        "$id": schema["$id"] + "#" + def_name,
        "$ref": "#/$defs/" + def_name,
        "$defs": schema["$defs"],
    }
    return Draft202012Validator(sub)


class Report:
    def __init__(self):
        self.failures = []
        self.passes = []

    def check(self, name, cond, detail=""):
        if cond:
            self.passes.append(name)
            return
        self.failures.append(f"{name}: {detail}" if detail else name)

    def eq(self, name, got, expected):
        self.check(name, got == expected, f"got {got!r} expected {expected!r}")


def test_r2_preserved(ex, schema, r):
    patterns = {
        "UInt64Text": schema["$defs"]["UInt64Text"]["pattern"],
        "UInt128Text": schema["$defs"]["UInt128Text"]["pattern"],
    }
    for key, pat in patterns.items():
        r.check(f"r2.{key}.true_end", "(?![\\s\\S])" in pat and "$" not in pat, pat)
        cre = re.compile(pat)
        r.check(f"r2.{key}.reject_lf", cre.match("1\n") is None)
        r.check(f"r2.{key}.reject_cr", cre.match("1\r") is None)
        r.check(f"r2.{key}.reject_u2028", cre.match("1\u2028") is None)
    v = schema_validator(schema)
    for iid, expect_fail in (
        ("inv-principal-trailing-newline", True),
        ("inv-principal-trailing-cr", True),
        ("inv-principal-unicode-newline", True),
        ("inv-cap-trailing-newline", True),
        ("inv-duplicate-four-contract-invariant", True),
        ("inv-uint64-max-ok", False),
        ("inv-uint128-max-ok", False),
        ("inv-uint64-max-plus-one", False),
        ("inv-uint128-max-plus-one", False),
    ):
        inv = invalid_by_id(ex, iid)
        doc = reconstruct(ex, inv)
        errs = list(v.iter_errors(doc))
        if expect_fail:
            r.check(f"r2.{iid}.schema_fail", len(errs) > 0, f"errs={len(errs)}")
        else:
            r.check(f"r2.{iid}.schema_pass", len(errs) == 0, [e.message for e in errs][:2])
    r.check("r2.uint64.max_domain", domain_ok("UInt64", str(UINT64_MAX)))
    r.check("r2.uint64.max_plus_one_domain", not domain_ok("UInt64", str(UINT64_MAX + 1)))
    r.check("r2.uint128.max_domain", domain_ok("UInt128", str(UINT128_MAX)))
    r.check(
        "r2.uint128.max_plus_one_domain",
        not domain_ok("UInt128", str(UINT128_MAX + 1)),
    )
    kinds = {"ContractInvariant", "IntentRefinement", "TransitionValidity", "HistoryCompliance"}
    for row in ex["valid"]:
        doc = parse_doc(row)
        seen = [c["kind"] for c in doc["requiredClaims"]]
        r.check(
            f"r2.{row['id']}.four_kinds",
            set(seen) == kinds and len(seen) == 4,
            seen,
        )


def test_r1_typed_core_and_state(schema, contract, r):
    defs = schema["$defs"]
    stored = json.dumps(defs["StoredValue"])
    r.check("r1.storedvalue.rate", "#/$defs/Rate" in stored)
    r.check("r1.storedvalue.price", "#/$defs/Price" in stored)
    rate = {"tag": "Rate", "mantissa": "1", "scale": "1"}
    price = {
        "tag": "Price",
        "base": "AssetB",
        "quote": "AssetA",
        "mantissa": "19743",
        "scale": "4",
        "direction": "basePerQuote",
    }
    sv = def_validator(schema, "StoredValue")
    if sv is None:
        r.check("r1.storedvalue.validator", False, "missing StoredValue")
    else:
        r.check("r1.rate_inhabits_storedvalue", sv.is_valid(rate), rate)
        r.check("r1.price_inhabits_storedvalue", sv.is_valid(price), price)
    shares_req = defs["Shares"].get("required", [])
    r.check("r1.shares.holder", "holder" in shares_req, shares_req)
    msg_req = defs["Message"].get("required", [])
    r.check(
        "r1.message.payload",
        "payloadHash" in msg_req or "payloadCommitment" in msg_req,
        msg_req,
    )
    state_req = defs["StateBody"].get("required", [])
    r.check("r1.state.fields", "fields" in state_req, state_req)
    r.check("r1.def.StateField", "StateField" in defs)
    r.check("r1.def.PolicyBody", "PolicyBody" in defs)
    r.check("r1.def.EffectsBody", "EffectsBody" in defs)
    r.check("r1.def.AssumptionSet", "AssumptionSet" in defs)
    text = contract
    for op in CORE_OPS:
        r.check(f"r1.coreop.{op}.present", op in text)
    r.check(
        "r1.core.signature_table",
        "Parameters" in text and "Result" in text and "State effect" in text,
        "constructor signature table missing",
    )
    r.check("r1.core.arity", "Arity" in text or "arity" in text)
    for name in ("ShareMint", "ShareBurn"):
        req = defs.get(f"{name}Effect", {}).get("required", [])
        r.check(f"r1.{name}.holder", "holder" in req, req)


def test_r3_display(ex, r):
    for row in ex["valid"]:
        doc = parse_doc(row)
        leaves = list(walk_leaves(doc))
        proj = row.get("displayProjection") or []
        r.eq(f"r3.{row['id']}.leaf_count", len(proj), len(leaves))
        by_path = {e.get("path"): e for e in proj}
        type_mismatch = []
        fallback = []
        missing_meta = []
        for path, value in leaves:
            entry = by_path.get(path)
            if entry is None:
                type_mismatch.append(("missing", path))
                continue
            if entry.get("value") != value:
                type_mismatch.append((path, type(entry.get("value")).__name__, type(value).__name__))
            unit = entry.get("unit")
            role = entry.get("role")
            label = entry.get("label")
            if unit in FALLBACK or role in FALLBACK:
                fallback.append(path)
            if not label or not unit or not role:
                missing_meta.append(path)
            if path.split(".")[-1].isdigit() and (not label or label == path.split(".")[-1]):
                missing_meta.append(("index-label", path, label))
        r.check(
            f"r3.{row['id']}.typed_values",
            not type_mismatch,
            type_mismatch[:8],
        )
        r.check(f"r3.{row['id']}.no_fallback", not fallback, fallback[:8])
        r.check(f"r3.{row['id']}.complete_meta", not missing_meta, missing_meta[:8])
        for key in ("label", "role", "unit", "path", "value"):
            r.check(
                f"r3.{row['id']}.entry_has_{key}",
                all(key in e for e in proj),
            )


def test_r4_inner_dag(ex, schema, r):
    reg = ex.get("preimageRegistry") or {}
    catalog = ex.get("preimageCatalog") or {}
    r.check("r4.catalog.present", isinstance(catalog, dict) and len(catalog) > 0, type(catalog))
    domains = ex.get("domainTagsProposed") or {}
    for extra in ("policy", "effects", "residual", "assumptions"):
        r.check(f"r4.domain.{extra}", extra in domains, list(domains)[:20])

    def body_hash(name):
        body = reg.get(name)
        closed = (catalog.get(name) or {}).get("closedType") or PREIMAGE_TYPES.get(name)
        domain = (catalog.get(name) or {}).get("domain") or DOMAINS.get(closed)
        if body is None or domain is None:
            return None
        if name == "sourceUtf8":
            return raw_hash(body) if isinstance(body, str) else None
        return framed_hash(domain, body)

    accrue_succ_h = body_hash("accrueSuccessor")
    post = reg.get("accruePostStateBody") or {}
    r.eq(
        "r4.accruePostStateBody.successorRecordHash",
        post.get("successorRecordHash"),
        accrue_succ_h,
    )
    post_h = body_hash("accruePostStateBody")
    prepared = reg.get("preparedAccrue") or {}
    r.eq("r4.preparedAccrue.postStateHash", prepared.get("postStateHash"), post_h)
    accrue_row = valid_by_id(ex, "exact-plan-accrue-nonexchange")
    accrue_digest = accrue_row.get("hashes", {}).get("exactPlanDigest")
    succ = (prepared.get("successors") or [{}])[0]
    r.eq(
        "r4.preparedAccrue.successor.origin",
        succ.get("originalIntentDigest"),
        accrue_digest,
    )
    r.eq(
        "r4.accrueSuccessor.origin",
        (reg.get("accrueSuccessor") or {}).get("originalIntentDigest"),
        accrue_digest,
    )
    outcome_prep = reg.get("preparedOutcome") or {}
    outcome_post_h = body_hash("outcomePostStateBody")
    r.eq(
        "r4.preparedOutcome.postStateHash",
        outcome_prep.get("postStateHash"),
        outcome_post_h,
    )
    policy = reg.get("policyBody")
    pv = def_validator(schema, "PolicyBody")
    r.check("r4.PolicyBody.def", pv is not None, "PolicyBody missing")
    if pv is not None and policy is not None:
        errs = list(pv.iter_errors(policy))
        r.check("r4.policyBody.typecheck", not errs, [e.message for e in errs][:3])
        r.check("r4.policyBody.no_self_hash", "policyHash" not in policy, list(policy)[:8])
    genesis = reg.get("genesisBody") or {}
    unborn_h = body_hash("unbornStateBody")
    r.eq("r4.genesis.initialStateHash.unborn", genesis.get("initialStateHash"), unborn_h)
    unborn = reg.get("unbornStateBody") or {}
    r.check(
        "r4.unborn.bootstrap_successor",
        unborn.get("successorRecordHash") == ZERO
        or unborn.get("successorRecordHash") == body_hash("emptySuccessor"),
        unborn.get("successorRecordHash"),
    )
    empty = reg.get("emptySuccessor") or {}
    r.eq("r4.genesis_bootstrap.tag", empty.get("tag"), "GenesisInitial")
    r.eq("r4.genesis_bootstrap.origin", empty.get("originalIntentDigest"), ZERO)
    r.check(
        "r4.no_state_genesis_field",
        "genesisHash" not in (reg.get("genesisStateBody") or {}),
    )
    for name, closed in PREIMAGE_TYPES.items():
        if name not in reg:
            r.check(f"r4.preimage.{name}.present", False, "missing body")
            continue
        cat = catalog.get(name) or {}
        r.eq(f"r4.preimage.{name}.closedType", cat.get("closedType"), closed)
        if closed in schema["$defs"] or closed == "ClaimArray":
            dv = def_validator(schema, closed)
            if dv is None:
                r.check(f"r4.preimage.{name}.validator", False, closed)
            else:
                errs = list(dv.iter_errors(reg[name]))
                r.check(
                    f"r4.preimage.{name}.typecheck",
                    not errs,
                    [e.message for e in errs][:2],
                )
    for row in ex["valid"]:
        ref = row.get("selectedPlanRef")
        r.check(
            f"r4.{row['id']}.selectedPlanRef.typed",
            isinstance(ref, dict) and "name" in ref and "closedType" in ref,
            ref,
        )
        prefs = row.get("preimageRefs")
        r.check(
            f"r4.{row['id']}.preimageRefs.typed",
            isinstance(prefs, list)
            and prefs
            and all(isinstance(p, dict) and "name" in p and "closedType" in p for p in prefs),
            type(prefs),
        )


def test_r5_accrual(ex, r):
    r.eq("r5.formula.independent", EXPECTED_ACCRUED, 41_095_890)
    r.check("r5.formula.not_old_wrong", EXPECTED_ACCRUED != FORBIDDEN_VALUE)
    row = valid_by_id(ex, "exact-plan-accrue-nonexchange")
    doc = parse_doc(row)
    effect = doc["executionBody"]["exactEffects"][0]
    r.eq("r5.effect.accruedDelta", int(effect["accruedDelta"]), EXPECTED_ACCRUED)
    r.check("r5.effect.not_old_qty", int(effect["accruedDelta"]) != FORBIDDEN_VALUE)
    write = doc["executionBody"]["exactWrites"][0]["value"]
    principal = int(write["principal"])
    accrued = int(write["accrued"])
    outstanding = int(write["outstanding"])
    cap = int(write["liabilityCap"])
    r.eq("r5.write.principal", principal, PRINCIPAL)
    r.eq("r5.write.accrued", accrued, EXPECTED_ACCRUED)
    r.eq("r5.write.outstanding", outstanding, PRINCIPAL + EXPECTED_ACCRUED)
    r.check("r5.cap.room_or_clip", outstanding <= cap, f"{outstanding} > {cap}")
    if PRINCIPAL + EXPECTED_ACCRUED <= cap:
        r.eq("r5.capApplied.false_when_room", effect.get("capApplied"), False)
    else:
        r.eq("r5.capApplied.true_when_clipped", effect.get("capApplied"), True)
        r.eq("r5.clipped.outstanding", outstanding, cap)
    r.check("r5.lastAccrualEnd.field", "lastAccrualEnd" in write, list(write)[:12])
    if "lastAccrualEnd" in write:
        r.eq("r5.lastAccrualEnd.equals_periodEnd", write["lastAccrualEnd"], effect["periodEnd"])
    r.check(
        "r5.nonoverlap.periodStart_ge_cursor",
        int(effect["periodStart"]) >= int(write.get("startDate", "0")),
    )
    r.check("r5.qty.argument", any(
        a.get("name") == "qty" and int(a["value"]["mantissa"]) == EXPECTED_ACCRUED
        for a in doc["executionBody"]["action"]["arguments"]
    ))
    text = EXAMPLES_PATH.read_text(encoding="utf-8")
    r.check("r5.no_stale_533972602", str(FORBIDDEN_VALUE) not in text, "old wrong delta remains")
    contract = CONTRACT_PATH.read_text(encoding="utf-8")
    r.check("r5.prose.formula_value", "41095890" in contract)
    r.check("r5.prose.nonoverlap", "lastAccrualEnd" in contract)
    r.check("r5.prose.writeoff_components", "principalDelta" in contract and "accruedDelta" in contract)
    r.check("r5.prose.quantity_scale", "scale alignment" in contract or "align scale" in contract)
    r.check("r5.prose.remainder_party", "NamedParty" in contract and "party" in contract)


def test_r6_ledgers(ex, r):
    reg = ex["preimageRegistry"]
    succ = reg.get("swapSuccessor") or {}
    ledgers = {(row["actor"], row["asset"]): row for row in succ.get("actorLedgers") or []}
    r.check("r6.swap.trader_AssetA", ("trader", "AssetA") in ledgers, list(ledgers))
    r.check("r6.swap.trader_AssetB", ("trader", "AssetB") in ledgers, list(ledgers))
    a = ledgers.get(("trader", "AssetA"), {})
    b = ledgers.get(("trader", "AssetB"), {})
    r.eq("r6.AssetA.cumulativeGross", int(a.get("cumulativeGross", "-1")), 10000)
    r.eq("r6.AssetA.remainingGross", int(a.get("remainingGross", "-1")), 0)
    r.check(
        "r6.AssetA.net_not_19743",
        int(a.get("originalNetGoal", "0")) == 0 and int(a.get("cumulativeNet", "0")) == 0,
        a,
    )
    r.eq("r6.AssetB.originalNetGoal", int(b.get("originalNetGoal", "-1")), 19743)
    r.eq("r6.AssetB.cumulativeNet", int(b.get("cumulativeNet", "-1")), 19743)
    r.eq("r6.AssetB.cumulativeGross", int(b.get("cumulativeGross", "-1")), 0)
    caps = {
        (c["actor"], c["asset"]): int(c["maximum"])
        for c in (succ.get("residualCapability") or {}).get("grossDebitCaps") or []
    }
    r.eq("r6.residual.gross.trader_AssetA", caps.get(("trader", "AssetA")), 0)
    debt = (reg.get("accrueSuccessor") or {}).get("debtLedgers") or []
    r.check("r6.accrue.debtLedger", len(debt) == 1, debt)
    if debt:
        d = debt[0]
        created = int(d["cumulativeCreated"])
        repaid = int(d["cumulativeRepaid"])
        written = int(d["cumulativeWrittenOff"])
        accrued = int(d.get("cumulativeAccrued", "0"))
        credit = int(d.get("cumulativeCredit", "0"))
        outstanding = int(d["outstanding"])
        r.eq(
            "r6.debt.equation",
            outstanding,
            created + accrued - repaid - written - credit,
        )
        r.eq("r6.debt.accrued_component", accrued, EXPECTED_ACCRUED)
    r.check("r6.successor.recoveryRights", "recoveryRights" in succ, list(succ))
    r.check("r6.successor.successorId", "successorId" in succ, list(succ))
    contract = CONTRACT_PATH.read_text(encoding="utf-8")
    r.check("r6.prose.prefix_feasibility", "permitted payers" in contract or "same-asset aggregation" in contract)
    r.check("r6.prose.cancel_duty", "UnsatisfiedNetGoal" in contract)
    r.check("r6.prose.attenuation", "attenuation" in contract)


def test_r7_work(ex, r):
    reg = ex["preimageRegistry"]
    triples = [
        ("swap", "swapStateBody", "preparedSwap", "swapSuccessor"),
        ("accrue", "accruePostStateBody", "preparedAccrue", "accrueSuccessor"),
        ("genesis", "genesisStateBody", "preparedGenesis", "genesisSuccessor"),
    ]
    for label, state_n, prep_n, succ_n in triples:
        sw = (reg.get(state_n) or {}).get("remainingWork") or {}
        pw = (reg.get(prep_n) or {}).get("remainingWork") or {}
        uw = (reg.get(succ_n) or {}).get("remainingWork") or {}
        r.eq(f"r7.{label}.state_vs_prepared", sw, pw)
        r.eq(f"r7.{label}.prepared_vs_successor", pw, uw)
        r.eq(f"r7.{label}.ordinary_after_charge", sw.get("ordinaryRemaining"), "7")
        r.eq(f"r7.{label}.lifetime_ordinary", sw.get("lifetimeOrdinary"), "8")
        r.eq(f"r7.{label}.recovery_unchanged", sw.get("recoveryRemaining"), "2")
    signed_swap = parse_doc(valid_by_id(ex, "exact-plan-swap-min-receive"))
    r.eq(
        "r7.signed.precharge.ordinary",
        signed_swap["remainingWork"]["ordinaryRemaining"],
        "8",
    )
    contract = CONTRACT_PATH.read_text(encoding="utf-8")
    r.check("r7.prose.expression_charge", "expression" in contract and "charge" in contract)
    r.check("r7.prose.split_event_charge", "split event" in contract or "charge(split)" in contract)
    r.check("r7.prose.par_reads", "read set" in contract or "cross-branch read" in contract)
    r.check("r7.prose.branch_key", "branch key" in contract)
    r.check("r7.prose.partition", "authority partition" in contract or "ledger partition" in contract)


def _contains_op(container, op):
    if isinstance(container, dict):
        allowed = container.get("allowedEffectSet") or []
        if op in allowed:
            return True
        residual = container.get("residualCapability") or container.get("residualAuthority") or {}
        return op in (residual.get("allowedEffectSet") or [])
    if isinstance(container, list):
        return op in container
    return False


def test_r8_isolated_negatives(ex, schema, r):
    v = schema_validator(schema)
    reg = ex["preimageRegistry"]

    mig = invalid_by_id(ex, "inv-replay-migration")
    doc = reconstruct(ex, mig)
    errs = list(v.iter_errors(doc))
    r.check("r8.migrate.schema_pass", not errs, [e.message for e in errs][:3])
    r.check(
        "r8.migrate.authorized",
        _contains_op(doc.get("residualAuthority") or {}, "Migrate"),
        (doc.get("residualAuthority") or {}).get("allowedEffectSet"),
    )
    pre = reg.get((mig.get("context") or {}).get("preStateRef"), {})
    r.check(
        "r8.migrate.prestate.authorized",
        _contains_op(pre, "Migrate"),
        (pre.get("residualCapability") or {}).get("allowedEffectSet") if pre else "missing prestate",
    )
    listed = set(mig.get("recomputeDependentHashes") or [])
    needed = {
        "executionBodyHash",
        "exactPlanDigest",
        "authorityDigest",
        "preparedHash",
        "acceptanceBind",
        "successorRecordHash",
    }
    r.check("r8.migrate.recompute_all", needed <= listed, sorted(listed))
    wrapper_name = (mig.get("context") or {}).get("signedAuthorizationRef")
    wrapper = reg.get(wrapper_name) or {}
    mutated_digest = framed_hash("MORIARTY-SUCC-EXACT-PLAN/0", doc)
    r.eq(
        "r8.migrate.wrapper.intentDigest",
        wrapper.get("intentDigest"),
        mutated_digest,
    )
    r.eq("r8.migrate.stage", mig.get("intendedFailureStage"), "history")
    r.eq("r8.migrate.specified_only", mig.get("executionStatus"), "specified-only")

    cancel = invalid_by_id(ex, "inv-cancellation-race")
    cdoc = reconstruct(ex, cancel)
    r.check("r8.cancel.schema_pass", not list(v.iter_errors(cdoc)))
    r.check(
        "r8.cancel.authorized",
        "RequestCancel" in (cdoc.get("allowedActions") or [])
        or _contains_op(cdoc.get("residualAuthority") or {}, "RequestCancel"),
        cdoc.get("allowedActions"),
    )
    ctx = cancel.get("context") or {}
    pre_c = reg.get(ctx.get("preStateRef"), {})
    req_id = None
    plan = ctx.get("selectedPlan") or {}
    effects = plan.get("exactEffects") or []
    if effects:
        req_id = effects[0].get("requestId")
    requests = pre_c.get("requests") or []
    r.check(
        "r8.cancel.request_in_prestate",
        any(q.get("requestId") == req_id for q in requests) and req_id,
        {"req": req_id, "requests": requests},
    )
    win = ctx.get("winningFill") or {}
    r.check(
        "r8.cancel.winningFill.typed",
        win.get("tag") in ("AcceptanceBody", "ExactPlan") or "intentDigest" in win or "successorRecord" in win,
        list(win)[:12],
    )
    wrapper_c = reg.get(ctx.get("signedAuthorizationRef"), {})
    r.check(
        "r8.cancel.wrapper.matches_mutated",
        wrapper_c.get("intentDigest")
        in (
            framed_hash("MORIARTY-SUCC-OUTCOME-INTENT/0", cdoc),
            cdoc.get("kind"),
        )
        or wrapper_c.get("intentDigest") == framed_hash("MORIARTY-SUCC-OUTCOME-INTENT/0", cdoc),
        wrapper_c.get("intentDigest"),
    )
    r.eq("r8.cancel.stage", cancel.get("intendedFailureStage"), "ledger-currentness")

    fee = invalid_by_id(ex, "inv-net-after-fees-shortfall")
    fdoc = reconstruct(ex, fee)
    r.check("r8.fee.schema_pass", not list(v.iter_errors(fdoc)))
    plan_f = (fee.get("context") or {}).get("selectedPlan") or {}
    writes = {w["field"]: w["value"] for w in plan_f.get("exactWrites") or []}
    tb = writes.get("trader_b") or writes.get("fields.trader_b")
    if isinstance(tb, dict):
        r.eq("r8.fee.trader_b", tb.get("value"), "19713")
    else:
        r.check("r8.fee.trader_b.present", False, writes)
    residual_fee = None
    for cap in (fdoc.get("residualAuthority") or {}).get("feeCaps") or fdoc.get("feeCaps") or []:
        if cap.get("actor") == "trader" and cap.get("asset") == "AssetB":
            residual_fee = int(cap["maximum"])
    r.eq("r8.fee.residual_fee_cap", residual_fee, 30)
    kinds = [e.get("kind") for e in plan_f.get("exactEffects") or []]
    r.check("r8.fee.effect_present", "Fee" in kinds, kinds)
    r.eq("r8.fee.stage", fee.get("intendedFailureStage"), "evaluation")

    clone = invalid_by_id(ex, "inv-cloned-residual-work")
    children = (clone.get("context") or {}).get("children") or []
    r.eq("r8.clone.two_children", len(children), 2)
    for i, child in enumerate(children):
        r.eq(f"r8.clone.child{i}.tag", child.get("tag"), "ActiveSuccessor")
        r.check(f"r8.clone.child{i}.successorId", "successorId" in child, list(child)[:10])
        r.check(f"r8.clone.child{i}.recoveryRights", "recoveryRights" in child)
        r.check(f"r8.clone.child{i}.actorLedgers", "actorLedgers" in child)
    pre_cl = reg.get((clone.get("context") or {}).get("preStateRef"), {})
    selected = (clone.get("context") or {}).get("selectedPlanRef") or (clone.get("context") or {}).get("selectedPlan")
    r.check(
        "r8.clone.prestate.exchange_if_swap",
        _contains_op(pre_cl, "Exchange") or selected not in ("swapExecutionBody", "swap"),
        (pre_cl.get("residualCapability") or {}).get("allowedEffectSet"),
    )
    r.eq("r8.clone.specified_only", clone.get("executionStatus"), "specified-only")


def test_positives_schema_and_hashes(ex, schema, r):
    v = schema_validator(schema)
    for row in ex["valid"]:
        doc = parse_doc(row)
        errs = list(v.iter_errors(doc))
        r.check(f"pos.{row['id']}.schema", not errs, [e.message for e in errs][:3])
        recanon = canonical(doc)
        r.check(
            f"pos.{row['id']}.canonical_stable",
            recanon == row["canonicalUtf8"] or json.loads(row["canonicalUtf8"]) == doc,
        )
        kind = doc["kind"]
        domain = (
            "MORIARTY-SUCC-EXACT-PLAN/0"
            if kind == "ExactPlan"
            else "MORIARTY-SUCC-OUTCOME-INTENT/0"
        )
        digest = framed_hash(domain, doc)
        advertised = row.get("hashes", {})
        key = "exactPlanDigest" if kind == "ExactPlan" else "outcomeIntentDigest"
        r.eq(f"pos.{row['id']}.digest", advertised.get(key), digest)
        if "executionBody" in doc:
            r.eq(
                f"pos.{row['id']}.execHash",
                advertised.get("executionBodyHash") or doc.get("executionBodyHash"),
                framed_hash("MORIARTY-SUCC-EXEC-BODY/0", doc["executionBody"]),
            )


def test_old_profile_pins(r):
    root = HERE.parents[3]
    pins = {
        "experiments/moriarty-language/spec/bounds.json": "b548641a1a9d74bab68ba699ffb1e2350fa0889d61b8704e98216f9d4a6c3664",
        "experiments/moriarty-language/src/types.ts": "5cc7356e9a38a74f1a202c7c6c8d7ac6d1ef6eac8fe0d8013beb242e3de604d0",
        "experiments/moriarty-language/src/codec.ts": "8575d98e630fed5b6e68b23c574aec450fc2bbce1b83b057ce6154e43610b7e8",
    }
    for rel, expected in pins.items():
        p = root / rel
        got = hashlib.sha256(p.read_bytes()).hexdigest()
        r.eq(f"pin.{rel}", got, expected)


def main():
    r = Report()
    schema = load_json(SCHEMA_PATH)
    examples = load_json(EXAMPLES_PATH)
    contract = CONTRACT_PATH.read_text(encoding="utf-8")
    test_old_profile_pins(r)
    test_r2_preserved(examples, schema, r)
    test_r1_typed_core_and_state(schema, contract, r)
    test_r3_display(examples, r)
    test_r4_inner_dag(examples, schema, r)
    test_r5_accrual(examples, r)
    test_r6_ledgers(examples, r)
    test_r7_work(examples, r)
    test_r8_isolated_negatives(examples, schema, r)
    test_positives_schema_and_hashes(examples, schema, r)
    print(f"PASS {len(r.passes)}")
    print(f"FAIL {len(r.failures)}")
    for line in r.failures:
        print("FAIL", line)
    if r.failures:
        sys.exit(1)
    print("GREEN")
    return 0


if __name__ == "__main__":
    sys.exit(main())
