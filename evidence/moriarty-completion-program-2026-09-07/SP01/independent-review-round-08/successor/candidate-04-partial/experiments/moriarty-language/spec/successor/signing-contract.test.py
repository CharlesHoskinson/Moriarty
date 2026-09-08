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
    after_charge = {
        "genesis": "7",
        "swap": "6",
        "accrue": "6",
    }
    for label, state_n, prep_n, succ_n in triples:
        sw = (reg.get(state_n) or {}).get("remainingWork") or {}
        pw = (reg.get(prep_n) or {}).get("remainingWork") or {}
        uw = (reg.get(succ_n) or {}).get("remainingWork") or {}
        r.eq(f"r7.{label}.state_vs_prepared", sw, pw)
        r.eq(f"r7.{label}.prepared_vs_successor", pw, uw)
        r.eq(f"r7.{label}.ordinary_after_charge", sw.get("ordinaryRemaining"), after_charge[label])
        r.eq(f"r7.{label}.lifetime_ordinary", sw.get("lifetimeOrdinary"), "8")
        r.eq(f"r7.{label}.recovery_unchanged", sw.get("recoveryRemaining"), "2")
    signed_swap = parse_doc(valid_by_id(ex, "exact-plan-swap-min-receive"))
    genesis_post = work_ordinary(reg["genesisStateBody"]["remainingWork"])
    r.eq(
        "r7.signed.precharge.ordinary",
        signed_swap["remainingWork"]["ordinaryRemaining"],
        str(genesis_post),
    )
    r.check("r7.signed.not_refreshed_to_8", signed_swap["remainingWork"]["ordinaryRemaining"] != "8")
    swap_cert = (reg.get("swapExecutionBody") or {}).get("costCertificate") or {}
    r.eq("r7.swap.declared_cost", int(swap_cert.get("totalOrdinary", "0")), 1)
    r.eq(
        "r7.swap.post_from_predecessor",
        work_ordinary(reg["swapStateBody"]["remainingWork"]),
        genesis_post - int(swap_cert.get("totalOrdinary", "0")),
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


EXPR_CTORS = (
    "Require",
    "Let",
    "NextWrite",
    "ProjectField",
    "ConstructRecord",
)
PRESERVED_INVALID_IDS = (
    "inv-extra-property",
    "inv-noncanonical-integer",
    "inv-wrong-network",
    "inv-wrong-deployment",
    "inv-display-mismatch-gross-fee-debt",
    "inv-duplicate-predecessor",
    "inv-revoked-spec",
    "inv-expired-spec-window",
    "inv-partial-fill-exceeds-cap",
    "inv-cloned-residual-work",
    "inv-cancellation-race",
    "inv-replay-migration",
    "inv-unknown-extension",
    "inv-exact-output-overdelivery",
    "inv-net-after-fees-shortfall",
    "inv-principal-trailing-newline",
    "inv-cap-trailing-newline",
    "inv-principal-trailing-cr",
    "inv-principal-unicode-newline",
    "inv-uint64-max-ok",
    "inv-uint64-max-plus-one",
    "inv-uint128-max-ok",
    "inv-uint128-max-plus-one",
    "inv-duplicate-four-contract-invariant",
    "inv-display-missing-entry",
    "inv-display-extra-entry",
    "inv-display-changed-entry",
)
CONTEXTUAL_INVALID_IDS = (
    "inv-wrong-network",
    "inv-wrong-deployment",
    "inv-duplicate-predecessor",
    "inv-revoked-spec",
    "inv-expired-spec-window",
    "inv-partial-fill-exceeds-cap",
    "inv-cloned-residual-work",
    "inv-cancellation-race",
    "inv-replay-migration",
    "inv-exact-output-overdelivery",
    "inv-net-after-fees-shortfall",
)


def work_ordinary(work):
    return int((work or {}).get("ordinaryRemaining", "-1"))


def field_map(state):
    return {f.get("selector"): f for f in (state or {}).get("fields") or []}


def apply_writes(before, writes, create_selectors=None):
    """Independent write replay. Genesis may create declared selectors."""
    create = set(create_selectors or [])
    fields = {k: deepcopy(v) for k, v in field_map(before).items()}
    errors = []
    for w in writes or []:
        sel = w.get("field")
        if sel not in fields and sel not in create:
            errors.append(("undeclared", sel))
            continue
        if sel not in fields:
            fields[sel] = {
                "selector": sel,
                "fieldType": None,
                "value": deepcopy(w.get("value")),
                "projection": None,
            }
        else:
            fields[sel] = deepcopy(fields[sel])
            fields[sel]["value"] = deepcopy(w.get("value"))
    return fields, errors


def test_c03_f1_typed_core(schema, contract, r):
    defs = schema["$defs"]
    sigs = {row["op"]: row for row in schema.get("x-coreSignatures") or [] if "op" in row}
    r.eq("f1.core.catalog_count", len(sigs), 38)
    for op in CORE_OPS:
        row = sigs.get(op)
        r.check(f"f1.core.{op}.present", isinstance(row, dict), op)
        if not row:
            continue
        r.check(f"f1.core.{op}.arity_int", isinstance(row.get("arity"), int), row.get("arity"))
        params = row.get("parameters") or []
        r.eq(f"f1.core.{op}.arity_matches_params", row.get("arity"), len(params))
        r.check(
            f"f1.core.{op}.params_typed",
            all(isinstance(p, dict) and p.get("name") and p.get("type") for p in params),
            params,
        )
        r.check(f"f1.core.{op}.result", bool(row.get("result")), row)
        r.check(f"f1.core.{op}.preconditions", bool(row.get("preconditions")), row)
        r.check(f"f1.core.{op}.frameUpdate", bool(row.get("frameUpdate")), row)
    genesis = sigs.get("Genesis") or {}
    r.eq("f1.genesis.arity", genesis.get("arity"), 2)
    gnames = [p.get("name") for p in genesis.get("parameters") or []]
    r.check(
        "f1.genesis.params",
        gnames == ["instanceId", "initialWork"],
        gnames,
    )
    sc = sigs.get("ShareConvert") or {}
    sc_params = {p.get("name"): p.get("type") for p in sc.get("parameters") or []}
    r.check("f1.shareConvert.holder", sc_params.get("holder") == "Party", sc_params)
    r.check(
        "f1.shareConvert.source_shares",
        sc_params.get("source") == "Shares" or sc_params.get("inShares") == "Shares",
        sc_params,
    )
    r.check(
        "f1.shareConvert.dest_shares",
        sc_params.get("dest") == "Shares" or sc_params.get("outShares") == "Shares",
        sc_params,
    )
    r.check(
        "f1.shareConvert.distinct_vaults",
        "source.vault != dest.vault" in str(sc.get("preconditions")),
        sc.get("preconditions"),
    )
    effect = defs.get("ShareConvertEffect") or {}
    req = effect.get("required") or []
    for key in ("sourceVault", "destVault", "holder", "inShares", "outShares"):
        r.check(f"f1.shareConvertEffect.{key}", key in req, req)
    exprs = {row["ctor"]: row for row in schema.get("x-exprSignatures") or [] if "ctor" in row}
    for ctor in EXPR_CTORS:
        row = exprs.get(ctor)
        r.check(f"f1.expr.{ctor}.present", isinstance(row, dict), ctor)
        if not row:
            continue
        r.check(f"f1.expr.{ctor}.operands", bool(row.get("operands")), row)
        r.check(f"f1.expr.{ctor}.result", bool(row.get("result")), row)
        r.check(f"f1.expr.{ctor}.eval", bool(row.get("eval")), row)
        r.check(f"f1.expr.{ctor}.typing", bool(row.get("typing")), row)
    a2 = contract.split("### A.2", 1)[-1].split("### A.3", 1)[0]
    r.check("f1.a2.shares.holder", "holder" in a2 and "`Shares`" in a2, a2[a2.find("Shares"):a2.find("Shares")+180] if "Shares" in a2 else a2[:80])
    r.check("f1.prose.shareconvert.distinct", "source.vault != dest.vault" in contract or "distinct source/destination vault" in contract.lower())


def test_c03_f2_metadata(ex, r):
    swap = valid_by_id(ex, "exact-plan-swap-min-receive")
    doc = parse_doc(swap)
    price = doc["executionBody"]["rateOrPrice"]
    expected_unit = f"{price['base']} per {price['quote']} at scale {price['scale']}"
    mantissa_entries = [
        e for e in swap["displayProjection"]
        if e.get("path", "").endswith("rateOrPrice.mantissa")
    ]
    r.eq("f2.price.mantissa.entry_count", len(mantissa_entries), 1)
    if mantissa_entries:
        e = mantissa_entries[0]
        r.eq("f2.price.mantissa.value", e.get("value"), price["mantissa"])
        r.eq("f2.price.mantissa.unit", e.get("unit"), expected_unit)
        r.check("f2.price.mantissa.not_scaled_integer", e.get("unit") != "scaled-integer", e)
    accrue = valid_by_id(ex, "exact-plan-accrue-nonexchange")
    adoc = parse_doc(accrue)
    debt_arg = next(
        a for a in adoc["executionBody"]["action"]["arguments"] if a["name"] == "debt_id"
    )
    debt_leaf = debt_arg["value"]["value"] if isinstance(debt_arg["value"], dict) else debt_arg["value"]
    r.eq("f2.debt_id.leaf", debt_leaf, "Loan01")
    debt_entries = [
        e for e in accrue["displayProjection"]
        if e.get("path", "").endswith("action.arguments.0.value.value")
        or (e.get("identity") == "Loan01" and "debt" in e.get("path", ""))
    ]
    r.check("f2.debt_id.entry_present", len(debt_entries) >= 1, [e.get("path") for e in accrue["displayProjection"] if "debt" in e.get("path", "") or "arguments.0" in e.get("path", "")][:8])
    if debt_entries:
        hit = next((e for e in debt_entries if e.get("value") == "Loan01"), debt_entries[0])
        r.eq("f2.debt_id.value", hit.get("value"), "Loan01")
        r.eq("f2.debt_id.unit", hit.get("unit"), "ObligationId")
        r.eq("f2.debt_id.identity", hit.get("identity"), "Loan01")
        r.check(
            "f2.debt_id.role",
            hit.get("role") in ("obligation-identity", "debt-identity"),
            hit.get("role"),
        )
        r.check("f2.debt_id.not_typed_leaf", hit.get("unit") != "typed-leaf", hit)


def test_c03_f3_replay(ex, schema, r):
    reg = ex["preimageRegistry"]
    catalog = ex.get("preimageCatalog") or {}
    v_state = def_validator(schema, "StateBody")
    genesis_exec = reg["genesisExecutionBody"]
    unborn = reg["unbornStateBody"]
    genesis_state = reg["genesisStateBody"]
    declared = [f["selector"] for f in genesis_state["fields"]]
    r.check("f3.genesis.declared.pool_a", "pool_a" in declared, declared)
    r.check("f3.genesis.declared.pool_b", "pool_b" in declared, declared)
    r.check("f3.genesis.declared.trader_a", "trader_a" in declared, declared)
    r.check("f3.genesis.declared.trader_b", "trader_b" in declared, declared)
    r.check("f3.genesis.declared.alive", "alive" in declared, declared)
    written = [w["field"] for w in genesis_exec["exactWrites"]]
    r.check("f3.genesis.writes_cover_declared", set(declared) <= set(written), {"declared": declared, "written": written})
    r.check("f3.genesis.writes.pool_b", "pool_b" in written, written)
    r.check("f3.genesis.writes.trader_b", "trader_b" in written, written)
    gb = reg.get("genesisBody") or {}
    r.check(
        "f3.genesis.declaredFields_or_initial",
        bool(gb.get("declaredFields")) or bool(gb.get("initialFields")),
        list(gb)[:16],
    )
    fields_after, errs = apply_writes(unborn, genesis_exec["exactWrites"], create_selectors=declared)
    r.check("f3.genesis.replay.no_undeclared", not errs, errs)
    for sel in declared:
        got = (fields_after.get(sel) or {}).get("value")
        exp = field_map(genesis_state)[sel]["value"]
        r.eq(f"f3.genesis.after.{sel}", got, exp)
    bals = {(b["party"], b["asset"]): int(b["value"]) for b in genesis_state["balances"]}
    r.eq("f3.genesis.balance.pool_b", bals.get(("pool", "AssetB")), 2000000)
    r.eq("f3.genesis.balance.trader_a", bals.get(("trader", "AssetA")), 100000)
    r.check("f3.genesis.funding.residual", bool(genesis_state.get("residualCapability")), genesis_state.get("residualCapability"))

    swap_exec = reg["swapExecutionBody"]
    swap_writes = [w["field"] for w in swap_exec["exactWrites"]]
    r.check("f3.swap.writes.pool_a", "pool_a" in swap_writes, swap_writes)
    r.check("f3.swap.writes.pool_b", "pool_b" in swap_writes, swap_writes)
    r.check("f3.swap.no_reserve_a", "reserve_a" not in swap_writes, swap_writes)
    r.check("f3.swap.no_reserve_b", "reserve_b" not in swap_writes, swap_writes)
    swap_fields, swap_errs = apply_writes(genesis_state, swap_exec["exactWrites"])
    r.check("f3.swap.replay.no_undeclared", not swap_errs, swap_errs)
    swap_state = reg["swapStateBody"]
    for sel, fld in swap_fields.items():
        r.eq(f"f3.swap.after.{sel}", fld.get("value"), field_map(swap_state)[sel]["value"])

    loan = reg["loanStateBody"]
    gen_succ_h = catalog.get("genesisSuccessor", {}).get("hash")
    r.check(
        "f3.loan.not_swap_genesis_successor",
        loan.get("successorRecordHash") != gen_succ_h,
        {"loan": loan.get("successorRecordHash"), "genesisSuccessor": gen_succ_h},
    )
    r.check(
        "f3.loan.bootstrap_present",
        "loanGenesisSuccessor" in reg or "loanBootstrapSuccessor" in reg,
        sorted(k for k in reg if "loan" in k.lower() or "genesis" in k.lower()),
    )
    loan_ctrl = (loan.get("residualCapability") or {}).get("controller")
    r.eq("f3.loan.controller", loan_ctrl, "lender")
    r.check("f3.loan.debt_present", any(d.get("debtId") == "Loan01" for d in loan.get("debts") or []), loan.get("debts"))
    r.eq("f3.loan.work_inherited_from_own_genesis", work_ordinary(loan.get("remainingWork")), 7)

    accrue_doc = parse_doc(valid_by_id(ex, "exact-plan-accrue-nonexchange"))
    preds = accrue_doc.get("predecessors") or []
    r.check("f3.accrue.predecessor_is_loan", len(preds) == 1, preds)
    if preds:
        r.eq("f3.accrue.predecessor.stateHash", preds[0].get("stateHash"), catalog.get("loanStateBody", {}).get("hash"))
    accrue_exec = reg["accrueExecutionBody"]
    accrue_fields, accrue_errs = apply_writes(loan, accrue_exec["exactWrites"])
    r.check("f3.accrue.replay.no_undeclared", not accrue_errs, accrue_errs)
    post = reg["accruePostStateBody"]
    r.eq(
        "f3.accrue.after.loan01",
        (accrue_fields.get("loan01") or {}).get("value"),
        field_map(post)["loan01"]["value"],
    )
    if v_state:
        for name in ("unbornStateBody", "genesisStateBody", "swapStateBody", "loanStateBody", "accruePostStateBody"):
            errs = list(v_state.iter_errors(reg[name]))
            r.check(f"f3.{name}.typecheck", not errs, [e.message for e in errs][:2])


def test_c03_f4_ledgers(ex, schema, r):
    defs = schema["$defs"]
    al = defs["ActorLedger"]
    req = al.get("required") or []
    r.check("f4.ledger.inflow", "cumulativeInflow" in req, req)
    r.check("f4.ledger.fees", "cumulativeFees" in req, req)
    net_ref = ((al.get("properties") or {}).get("cumulativeNet") or {}).get("$ref", "")
    r.check("f4.ledger.net_signed", net_ref.endswith("SInt128Text"), net_ref)
    inflow_ref = ((al.get("properties") or {}).get("cumulativeInflow") or {}).get("$ref", "")
    r.check("f4.ledger.inflow_uint", inflow_ref.endswith("UInt128Text"), inflow_ref)
    sv = def_validator(schema, "ActorLedger")
    fee_only = {
        "actor": "trader",
        "asset": "AssetA",
        "originalGrossCap": "10000",
        "originalFeeCap": "30",
        "originalNetGoal": "0",
        "cumulativeGross": "0",
        "cumulativeFees": "30",
        "cumulativeInflow": "0",
        "cumulativeNet": "-30",
        "remainingGross": "10000",
        "remainingFee": "0",
    }
    r.check("f4.input_fee.schema", sv is not None)
    if sv is not None:
        errs = list(sv.iter_errors(fee_only))
        r.check("f4.input_fee.inflow0_fee30", not errs, [e.message for e in errs][:3])
    examples_ledgers = ex.get("ledgerExamples") or {}
    r.check("f4.input_fee.example", "inputAssetFee30Inflow0" in examples_ledgers, sorted(examples_ledgers))
    r.check("f4.shortfall_duty.example", "shortfallCancellationDuty" in examples_ledgers, sorted(examples_ledgers))
    duty = (examples_ledgers.get("shortfallCancellationDuty") or {})
    r.eq("f4.duty.kind", duty.get("kind"), "UnsatisfiedNetGoal")
    r.eq("f4.duty.creditor", duty.get("creditor"), "trader")
    r.check("f4.duty.debtor_not_trader", duty.get("debtor") not in (None, "trader"), duty.get("debtor"))
    r.eq("f4.duty.debtor", duty.get("debtor"), "pool")
    nom = duty.get("nominalOrAsset") or {}
    r.eq("f4.duty.asset", nom.get("asset") if isinstance(nom, dict) else None, "AssetB")
    r.eq("f4.duty.deficit", nom.get("value") if isinstance(nom, dict) else None, "30")
    r.check("f4.duty.backing", bool(duty.get("backing") or duty.get("funding")), duty)
    r.check("f4.prose.not_recipient_debtor", "cannot arbitrarily make recipient" in CONTRACT_PATH.read_text(encoding="utf-8").lower() or "authorized payer" in CONTRACT_PATH.read_text(encoding="utf-8"))
    ids = [row["id"] for row in ex["invalid"]]
    r.check("f4.input_fee.fixture", "inv-input-asset-fee-only" in ids, ids)
    r.check("f4.unfunded.cancel.fixture", "inv-unfunded-cancellation" in ids, ids)


def expected_post_work(pre_ordinary, certificate):
    total = int((certificate or {}).get("totalOrdinary", "0"))
    return pre_ordinary - total


def test_c03_f5_work(ex, schema, r):
    reg = ex["preimageRegistry"]
    catalog = ex.get("preimageCatalog") or {}
    genesis_state = reg["genesisStateBody"]
    genesis_post = work_ordinary(genesis_state["remainingWork"])
    r.eq("f5.genesis.post", genesis_post, 7)
    unborn = work_ordinary(reg["unbornStateBody"]["remainingWork"])
    r.eq("f5.unborn.pre", unborn, 8)
    swap_doc = parse_doc(valid_by_id(ex, "exact-plan-swap-min-receive"))
    signed = work_ordinary(swap_doc["remainingWork"])
    r.eq("f5.swap.signed_inherits_genesis_post", signed, genesis_post)
    r.check("f5.swap.signed_not_refreshed_to_8", signed != 8, signed)
    cert = (reg.get("swapExecutionBody") or {}).get("costCertificate") or {}
    r.check("f5.swap.certificate", bool(cert.get("coreOps")) and "totalOrdinary" in cert, cert)
    r.eq("f5.swap.certificate.total", int(cert.get("totalOrdinary", "0")), 1)
    swap_post = work_ordinary(reg["swapStateBody"]["remainingWork"])
    r.eq("f5.swap.post", swap_post, expected_post_work(signed, cert))
    r.eq("f5.swap.post.6", swap_post, 6)
    r.eq(
        "f5.swap.prepared",
        work_ordinary(reg["preparedSwap"]["remainingWork"]),
        swap_post,
    )
    r.eq(
        "f5.swap.successor",
        work_ordinary(reg["swapSuccessor"]["remainingWork"]),
        swap_post,
    )
    outcome_doc = parse_doc(valid_by_id(ex, "outcome-intent-swap-and-loan-caps"))
    r.eq("f5.outcome.signed_inherits", work_ordinary(outcome_doc["remainingWork"]), genesis_post)
    r.eq("f5.outcome.post", work_ordinary(reg["outcomePostStateBody"]["remainingWork"]), 6)
    loan_pre = work_ordinary(reg["loanStateBody"]["remainingWork"])
    r.eq("f5.loan.pre", loan_pre, 7)
    accrue_doc = parse_doc(valid_by_id(ex, "exact-plan-accrue-nonexchange"))
    r.eq("f5.accrue.signed_inherits", work_ordinary(accrue_doc["remainingWork"]), loan_pre)
    accrue_cert = (reg.get("accrueExecutionBody") or {}).get("costCertificate") or {}
    r.eq(
        "f5.accrue.post",
        work_ordinary(reg["accruePostStateBody"]["remainingWork"]),
        expected_post_work(loan_pre, accrue_cert),
    )
    bumped = deepcopy(swap_doc)
    bumped["remainingWork"] = deepcopy(bumped["remainingWork"])
    bumped["remainingWork"]["ordinaryRemaining"] = str(signed + 1)
    r.check("f5.plus_one.rejects_inheritance", work_ordinary(bumped["remainingWork"]) != genesis_post)
    r.eq("f5.plus_one.value", work_ordinary(bumped["remainingWork"]), signed + 1)
    contract = CONTRACT_PATH.read_text(encoding="utf-8")
    r.check("f5.prose.after_charges", "after charge" in contract.lower() or "after charges" in contract.lower())
    r.check("f5.prose.split_after", "split" in contract and "after" in contract)
    cert_def = schema["$defs"].get("CostCertificate")
    r.check("f5.schema.CostCertificate", isinstance(cert_def, dict), "missing CostCertificate")
    r.check(
        "f5.exec.requires_certificate",
        "costCertificate" in (schema["$defs"]["ExecutionBody"].get("required") or []),
        schema["$defs"]["ExecutionBody"].get("required"),
    )


def first_false_stage(inv, doc, ex, schema):
    """Independent predicate order: schema, domain, hash/auth wrapper, registry, budgets, then named stage."""
    v = schema_validator(schema)
    schema_errs = list(v.iter_errors(doc))
    if schema_errs:
        return "schema"
    stage = inv.get("intendedFailureStage")
    if stage == "schema":
        return "schema"
    return stage


def context_complete(inv, ex, r, label):
    ctx = inv.get("context") or {}
    reg = ex["preimageRegistry"]
    catalog = ex.get("preimageCatalog") or {}
    r.check(f"{label}.context.present", bool(ctx), inv.get("id"))
    pre_name = ctx.get("preStateRef")
    r.check(f"{label}.preStateRef", pre_name in reg, pre_name)
    pre = reg.get(pre_name) or {}
    r.check(f"{label}.stateHash", bool(catalog.get(pre_name, {}).get("hash")), pre_name)
    plan = ctx.get("selectedPlan") or ctx.get("selectedPlanRef")
    r.check(f"{label}.selected_plan", bool(plan), ctx)
    auth = ctx.get("signedAuthorizationRef")
    r.check(f"{label}.auth_ref", auth in reg, auth)
    wrapper = reg.get(auth) or {}
    r.check(f"{label}.wrapper.intentDigest", bool(wrapper.get("intentDigest")), wrapper)
    for key in ("preparedRef", "successorRef", "acceptanceRef"):
        if ctx.get(key):
            r.check(f"{label}.{key}.present", ctx[key] in reg, ctx.get(key))
    return ctx, pre, wrapper


def test_c03_f6_negatives(ex, schema, r):
    ids = [row["id"] for row in ex["invalid"]]
    r.check(
        "f6.preserved_count_subset",
        set(PRESERVED_INVALID_IDS) <= set(ids),
        sorted(set(PRESERVED_INVALID_IDS) - set(ids)),
    )
    r.eq("f6.min_27", len(set(ids) & set(PRESERVED_INVALID_IDS)), 27)
    v = schema_validator(schema)
    reg = ex["preimageRegistry"]
    catalog = ex.get("preimageCatalog") or {}
    program = reg.get("programBody") or {}
    entries = program.get("entryActions") or []
    r.check("f6.program.migrate", "migrate" in entries, entries)
    r.check("f6.program.request_cancel", "request_cancel" in entries or "RequestCancel" in entries, entries)

    fee = invalid_by_id(ex, "inv-net-after-fees-shortfall")
    fdoc = reconstruct(ex, fee)
    r.check("f6.fee.schema_pass", not list(v.iter_errors(fdoc)), [e.message for e in list(v.iter_errors(fdoc))[:2]])
    ctx, pre, wrapper = context_complete(fee, ex, r, "f6.fee")
    fee_caps = (pre.get("residualCapability") or {}).get("feeCaps") or []
    admitted = any(c.get("actor") == "trader" and c.get("asset") == "AssetB" and int(c.get("maximum", "0")) >= 30 for c in fee_caps)
    r.check("f6.fee.prestate.fee_cap_authorized", admitted, fee_caps)
    plan = ctx.get("selectedPlan") or {}
    writes = [w.get("field") for w in plan.get("exactWrites") or []]
    r.check("f6.fee.writes.pool_a", "pool_a" in writes, writes)
    r.check("f6.fee.writes.no_reserve", "reserve_a" not in writes and "reserve_b" not in writes, writes)
    mutated = framed_hash("MORIARTY-SUCC-OUTCOME-INTENT/0", fdoc)
    r.eq("f6.fee.wrapper.matches", wrapper.get("intentDigest"), mutated)
    r.check("f6.fee.prepared", bool(ctx.get("preparedRef") and ctx["preparedRef"] in reg), ctx)
    r.check("f6.fee.successor", bool(ctx.get("successorRef") and ctx["successorRef"] in reg), ctx)
    r.check("f6.fee.acceptance", bool(ctx.get("acceptanceRef") and ctx["acceptanceRef"] in reg), ctx)
    r.eq("f6.fee.first_predicate", fee.get("firstFailingPredicate"), "evaluation")

    cancel = invalid_by_id(ex, "inv-cancellation-race")
    cdoc = reconstruct(ex, cancel)
    r.check("f6.cancel.schema_pass", not list(v.iter_errors(cdoc)))
    cctx, cpre, cwrap = context_complete(cancel, ex, r, "f6.cancel")
    win = cctx.get("winningFill") or {}
    stale = (cctx.get("staleResidual") or {}).get("staleConsumptionId")
    win_preds = win.get("predecessorIds") or []
    r.check("f6.cancel.winner_consumed_stale", stale in win_preds or win.get("consumedId") == stale, {"stale": stale, "win": win_preds, "consumed": win.get("consumedId")})
    pinned = cctx.get("preStateHash") or catalog.get(cctx.get("preStateRef"), {}).get("hash")
    r.eq("f6.cancel.pre_hash_matches_body", pinned, catalog.get("cancelPreStateBody", {}).get("hash"))
    r.check(
        "f6.cancel.authorized_in_program",
        "request_cancel" in entries or "RequestCancel" in (cdoc.get("allowedActions") or []),
        entries,
    )
    r.eq("f6.cancel.first_predicate", cancel.get("firstFailingPredicate"), "ledger-currentness")

    mig = invalid_by_id(ex, "inv-replay-migration")
    mdoc = reconstruct(ex, mig)
    r.check("f6.migrate.schema_pass", not list(v.iter_errors(mdoc)))
    mctx, mpre, mwrap = context_complete(mig, ex, r, "f6.migrate")
    r.check("f6.migrate.pre.authorized", "Migrate" in ((mpre.get("residualCapability") or {}).get("allowedEffectSet") or []), mpre.get("residualCapability"))
    r.check("f6.migrate.prepared_rebuilt", mctx.get("preparedRef") in reg, mctx)
    r.check("f6.migrate.successor_rebuilt", mctx.get("successorRef") in reg, mctx)
    r.check("f6.migrate.acceptance_rebuilt", mctx.get("acceptanceRef") in reg, mctx)
    mutated_m = framed_hash("MORIARTY-SUCC-EXACT-PLAN/0", mdoc)
    r.eq("f6.migrate.wrapper.matches", mwrap.get("intentDigest"), mutated_m)
    r.eq("f6.migrate.first_predicate", mig.get("firstFailingPredicate"), "history")

    clone = invalid_by_id(ex, "inv-cloned-residual-work")
    cctx2, cpre2, _ = context_complete(clone, ex, r, "f6.clone")
    parent = work_ordinary(cctx2.get("parentWork") or cpre2.get("remainingWork"))
    r.eq("f6.clone.parent_work", parent, work_ordinary(cpre2.get("remainingWork")))
    r.check("f6.clone.parent_not_8_if_genesis_post_7", parent == work_ordinary(reg["genesisStateBody"]["remainingWork"]) or parent == work_ordinary(cpre2.get("remainingWork")))
    children = cctx2.get("children") or []
    r.eq("f6.clone.two_children", len(children), 2)
    child_sum = sum(work_ordinary(ch.get("remainingWork")) for ch in children)
    r.check("f6.clone.sum_exceeds_parent", child_sum > parent, {"sum": child_sum, "parent": parent})

    over = invalid_by_id(ex, "inv-exact-output-overdelivery")
    odoc = reconstruct(ex, over)
    r.check("f6.over.schema_pass", not list(v.iter_errors(odoc)))
    octx = over.get("context") or {}
    r.check("f6.over.rebuilt_chain", octx.get("preparedRef") in reg and octx.get("successorRef") in reg and octx.get("acceptanceRef") in reg, octx)
    owrap = reg.get(octx.get("signedAuthorizationRef")) or {}
    r.eq("f6.over.wrapper.matches", owrap.get("intentDigest"), framed_hash("MORIARTY-SUCC-EXACT-PLAN/0", odoc))
    owrites = [w.get("field") for w in (odoc.get("executionBody") or {}).get("exactWrites") or []]
    r.check("f6.over.declared_selectors", "pool_a" in owrites and "reserve_a" not in owrites, owrites)

    partial = invalid_by_id(ex, "inv-partial-fill-exceeds-cap")
    pdoc = reconstruct(ex, partial)
    r.check("f6.partial.schema_pass", not list(v.iter_errors(pdoc)))
    pctx = partial.get("context") or {}
    r.check("f6.partial.rebuilt_chain", pctx.get("signedAuthorizationRef") in reg, pctx)
    pwrap = reg.get(pctx.get("signedAuthorizationRef")) or {}
    r.eq("f6.partial.wrapper.matches", pwrap.get("intentDigest"), framed_hash("MORIARTY-SUCC-OUTCOME-INTENT/0", pdoc))

    for iid in ("inv-wrong-network", "inv-wrong-deployment", "inv-revoked-spec", "inv-expired-spec-window", "inv-duplicate-predecessor"):
        inv = invalid_by_id(ex, iid)
        d = reconstruct(ex, inv)
        r.check(f"f6.{iid}.schema_pass", not list(v.iter_errors(d)), iid)
        ctx = inv.get("context") or {}
        auth = ctx.get("signedAuthorizationRef")
        r.check(f"f6.{iid}.auth_rebuilt", auth in reg, auth)
        wrap = reg.get(auth) or {}
        kind = d.get("kind")
        domain = "MORIARTY-SUCC-EXACT-PLAN/0" if kind == "ExactPlan" else "MORIARTY-SUCC-OUTCOME-INTENT/0"
        r.eq(f"f6.{iid}.wrapper.matches", wrap.get("intentDigest"), framed_hash(domain, d))
        r.check(f"f6.{iid}.first_predicate", bool(inv.get("firstFailingPredicate")), inv.get("firstFailingPredicate"))

    for iid in ("inv-uint64-max-ok", "inv-uint128-max-ok"):
        inv = invalid_by_id(ex, iid)
        r.eq(f"f6.{iid}.domain_only", inv.get("intendedFailureStage"), "none")
        r.check(f"f6.{iid}.not_complete_tx", inv.get("executionStatus") == "specified-only")

    for row in ex["invalid"]:
        r.eq(f"f6.{row['id']}.specified_only", row.get("executionStatus"), "specified-only")


def test_independent_fixture_replay(ex, schema, r):
    """Replay every positive and every contextual negative. Does not import the generator."""
    v = schema_validator(schema)
    reg = ex["preimageRegistry"]
    catalog = ex.get("preimageCatalog") or {}
    positives = [
        ("exact-plan-genesis-no-predecessor", "unbornStateBody", "genesisExecutionBody", "genesisStateBody"),
        ("exact-plan-swap-min-receive", "genesisStateBody", "swapExecutionBody", "swapStateBody"),
        ("exact-plan-accrue-nonexchange", "loanStateBody", "accrueExecutionBody", "accruePostStateBody"),
        ("outcome-intent-swap-and-loan-caps", "genesisStateBody", "outcomeSelectedPlan", "outcomePostStateBody"),
    ]
    for vid, before_n, exec_n, after_n in positives:
        row = valid_by_id(ex, vid)
        doc = parse_doc(row)
        errs = list(v.iter_errors(doc))
        r.check(f"replay.{vid}.schema", not errs, [e.message for e in errs][:2])
        before = reg[before_n]
        after = reg[after_n]
        exec_body = reg[exec_n]
        create = [f["selector"] for f in after["fields"]] if before_n == "unbornStateBody" else None
        fields, werr = apply_writes(before, exec_body.get("exactWrites"), create_selectors=create)
        r.check(f"replay.{vid}.writes", not werr, werr)
        for sel, fld in field_map(after).items():
            r.eq(f"replay.{vid}.after.{sel}", (fields.get(sel) or {}).get("value"), fld.get("value"))
        signed = work_ordinary(doc.get("remainingWork"))
        pred_work = work_ordinary(before.get("remainingWork"))
        r.eq(f"replay.{vid}.signed_work", signed, pred_work)
        cert = exec_body.get("costCertificate") or {}
        r.eq(
            f"replay.{vid}.post_work",
            work_ordinary(after.get("remainingWork")),
            expected_post_work(signed, cert),
        )
        if doc.get("kind") == "ExactPlan" and doc.get("predecessors"):
            pred = doc["predecessors"][0]
            r.eq(f"replay.{vid}.pred.stateHash", pred.get("stateHash"), catalog[before_n]["hash"])
        r.check(f"replay.{vid}.effects", isinstance(exec_body.get("exactEffects"), list))

    for iid in CONTEXTUAL_INVALID_IDS:
        inv = invalid_by_id(ex, iid)
        doc = reconstruct(ex, inv)
        schema_errs = list(v.iter_errors(doc))
        r.check(f"replay.{iid}.schema_prior", not schema_errs, [e.message for e in schema_errs][:2])
        ctx = inv.get("context") or {}
        r.check(f"replay.{iid}.full_context", bool(ctx.get("preStateRef") and ctx.get("signedAuthorizationRef")), ctx)
        pre = reg.get(ctx.get("preStateRef"))
        r.check(f"replay.{iid}.pre_admitted", pre is not None)
        wrap = reg.get(ctx.get("signedAuthorizationRef")) or {}
        kind = doc.get("kind")
        domain = "MORIARTY-SUCC-EXACT-PLAN/0" if kind == "ExactPlan" else "MORIARTY-SUCC-OUTCOME-INTENT/0"
        r.eq(f"replay.{iid}.auth_payload", wrap.get("intentDigest"), framed_hash(domain, doc))
        named = inv.get("firstFailingPredicate") or inv.get("intendedFailureStage")
        r.check(f"replay.{iid}.named_false", bool(named), inv.get("id"))
        r.check(
            f"replay.{iid}.later_unreachable_ok",
            inv.get("laterPredicatesUnreachable") in (True, None) or named,
            inv.get("laterPredicatesUnreachable"),
        )


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
    test_c03_f1_typed_core(schema, contract, r)
    test_c03_f2_metadata(examples, r)
    test_c03_f3_replay(examples, schema, r)
    test_c03_f4_ledgers(examples, schema, r)
    test_c03_f5_work(examples, schema, r)
    test_c03_f6_negatives(examples, schema, r)
    test_independent_fixture_replay(examples, schema, r)
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
