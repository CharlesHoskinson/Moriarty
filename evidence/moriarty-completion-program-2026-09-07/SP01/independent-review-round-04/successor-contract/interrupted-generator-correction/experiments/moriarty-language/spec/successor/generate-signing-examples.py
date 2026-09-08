#!/usr/bin/env python3
"""Deterministic SP01.3 signing-example generator.

Builds typed bodies in topological order, hashes canonical framed bytes, and
substitutes every inner reference from computed values. Does not read
signing-examples.json. Writes only the path given to --output.
"""
from __future__ import annotations

import argparse
import hashlib
import json
from copy import deepcopy
from pathlib import Path

HERE = Path(__file__).resolve().parent
SCHEMA_PATH = HERE / "signing-display-schema.json"

ZERO = "0" * 64
SECONDS_PER_YEAR = 31536000
PRINCIPAL = 5_000_000_000
LIABILITY_CAP = 6_000_000_000
RATE_MANTISSA = 1
RATE_SCALE = 1
PERIOD = 2_592_000
ACCRUED = (PRINCIPAL * RATE_MANTISSA * PERIOD) // ((10 ** RATE_SCALE) * SECONDS_PER_YEAR)
START = "1690000000"
PERIOD_END = "1692592000"
NOW = "1700000000"
UINT64_MAX = "18446744073709551615"
UINT64_MAX1 = "18446744073709551616"
UINT128_MAX = "340282366920938463463374607431768211455"
UINT128_MAX1 = "340282366920938463463374607431768211456"

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
    "SuccessorId": "MORIARTY-SUCC-SUCCESSOR-ID/0",
}
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
    "cancelPreStateBody": "StateBody",
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
    "authorityWrapperMigrate": "AuthorityWrapper",
    "authorityWrapperCancel": "AuthorityWrapper",
    "authorityWrapperFee": "AuthorityWrapper",
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

WORK_PRE = {
    "lifetimeOrdinary": "8",
    "lifetimeRecovery": "2",
    "ordinaryRemaining": "8",
    "recoveryRemaining": "2",
}
WORK_POST = {
    "lifetimeOrdinary": "8",
    "lifetimeRecovery": "2",
    "ordinaryRemaining": "7",
    "recoveryRemaining": "2",
}
COMP = {
    "operator": "seq",
    "interleaveConflict": "write_path_or_identity_or_cap_key",
    "parFrame": "disjoint_keys_concat_obs",
    "unknownRejects": True,
    "parReads": "exclude_cross_branch_read_write",
    "branchKey": "lexicographic_branch_id",
}
CLAIMS = [
    {"claimId": "bounded_profile_safety_v0", "kind": "ContractInvariant"},
    {"claimId": "atomic_intent_refinement_v0", "kind": "IntentRefinement"},
    {"claimId": "bounded_atomic_transition_v0", "kind": "TransitionValidity"},
    {"claimId": "bounded_history_compliance_v0", "kind": "HistoryCompliance"},
]
STATUS_LIVE = {"episode": "Live", "agreement": "Active"}
PARTIAL_SWAP = {
    "tag": "Partial",
    "minFill": {"tag": "Amount", "asset": "AssetA", "value": "1"},
    "cumulativeCap": {"tag": "Amount", "asset": "AssetA", "value": "10000"},
}
FOK_LOAN = {
    "tag": "FillOrKill",
    "minFill": {"tag": "Amount", "asset": "USD_TEST_ASSET", "value": "0"},
    "cumulativeCap": {"tag": "Amount", "asset": "USD_TEST_ASSET", "value": "0"},
}
VALIDITY = {"notBefore": START, "notAfterExclusive": "1800000000"}
RECIPIENTS_SWAP = [
    {"party": "pool", "role": "pool"},
    {"party": "trader", "role": "principal"},
]
RECIPIENTS_LOAN = [
    {"party": "borrower", "role": "counterparty"},
    {"party": "lender", "role": "creditor"},
]
RECIPIENTS_ALL = RECIPIENTS_LOAN + RECIPIENTS_SWAP
RECOVERY = {
    "controller": "trader",
    "allowedEffectSet": ["RequestCancel", "Migrate"],
    "alias": {"tag": "AliasReserve", "which": "recovery"},
}
RECOVERY_LENDER = {
    "controller": "lender",
    "allowedEffectSet": ["RequestCancel", "Migrate"],
    "alias": {"tag": "AliasReserve", "which": "recovery"},
}


def canonical(obj) -> str:
    return json.dumps(obj, sort_keys=True, separators=(",", ":"), ensure_ascii=False)


def framed_hash(domain: str, obj) -> str:
    return hashlib.sha256(domain.encode("utf-8") + b"\x00" + canonical(obj).encode("utf-8")).hexdigest()


def raw_hash(text: str) -> str:
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


def u(n) -> str:
    return str(int(n))


def amount(asset, value):
    return {"tag": "Amount", "asset": asset, "value": u(value)}


def stored_u(value):
    return {"tag": "UInt128", "value": u(value)}


def stored_bool(v):
    return {"tag": "Bool", "value": bool(v)}


def field(selector, field_type, value, projection):
    return {
        "selector": selector,
        "fieldType": field_type,
        "value": value,
        "projection": projection,
    }


def bal(party, asset, value):
    return {"party": party, "asset": asset, "value": u(value)}


def resolve(node, root):
    if not isinstance(node, dict):
        return node
    if "$ref" in node:
        name = node["$ref"].split("/")[-1]
        base = deepcopy(root["$defs"][name])
        extra = {k: v for k, v in node.items() if k != "$ref"}
        xd = dict(base.get("x-display") or {})
        xd.update(extra.get("x-display") or {})
        base.update(extra)
        if xd:
            base["x-display"] = xd
        return resolve(base, root)
    return node


def matches(value, node, root):
    node = resolve(node, root)
    if "const" in node:
        return value == node["const"]
    if "enum" in node and not isinstance(value, (dict, list)):
        return value in node["enum"]
    if node.get("type") == "object":
        if not isinstance(value, dict):
            return False
        props = node.get("properties") or {}
        if "tag" in props:
            tag_n = resolve(props["tag"], root)
            if "const" in tag_n:
                return value.get("tag") == tag_n["const"]
        return True
    if node.get("type") == "array":
        return isinstance(value, list)
    if node.get("type") == "string":
        return isinstance(value, str)
    if node.get("type") == "boolean":
        return isinstance(value, bool)
    if node.get("type") == "integer":
        return isinstance(value, int) and not isinstance(value, bool)
    if "oneOf" in node:
        return any(matches(value, alt, root) for alt in node["oneOf"])
    return True


def match_schema(value, node, root):
    node = resolve(node, root)
    if "oneOf" in node:
        for alt in node["oneOf"]:
            if matches(value, alt, root):
                return resolve(alt, root)
    if "allOf" in node:
        merged = dict(node)
        for part in node["allOf"]:
            part = resolve(part, root)
            if "properties" in part:
                props = dict(merged.get("properties") or {})
                props.update(part.get("properties") or {})
                merged["properties"] = props
        return merged
    return node


def project_display(value, schema_root, doc_schema):
    entries = []

    def emit(path, val, meta):
        if not meta.get("label") or not meta.get("unit") or not meta.get("role"):
            raise SystemExit(f"missing display metadata for {path}: {meta}")
        if meta.get("unit") == "signed-field" or meta.get("role") == "signed-field":
            raise SystemExit(f"fallback metadata at {path}")
        entry = {
            "path": path,
            "label": meta["label"],
            "unit": meta["unit"],
            "role": meta["role"],
            "value": val,
        }
        unit = meta["unit"]
        if unit in ("Party", "AssetId", "ObligationId", "InstanceId", "MessageId", "VaultId", "DutyId", "PolicyId"):
            entry["identity"] = val
        if path.endswith(".scale") or unit in ("Scale", "scale"):
            entry["scale"] = val
        last = path.split(".")[-1]
        if last.isdigit() and entry["label"] == last:
            raise SystemExit(f"index label fallback at {path}")
        entries.append(entry)

    def walk(val, node, path, meta):
        node = match_schema(val, node, schema_root)
        xd = dict(meta)
        xd.update(node.get("x-display") or {})
        if isinstance(val, dict):
            if not val:
                emit(path, val, xd)
                return
            props = node.get("properties") or {}
            for k in sorted(val.keys()):
                child = props.get(k, {})
                child_n = resolve(child, schema_root)
                child_meta = dict(xd)
                child_meta.update(child_n.get("x-display") or {})
                if k == "tag" and "label" not in child_n.get("x-display") or {}:
                    child_meta.setdefault("label", "Constructor tag")
                    child_meta.setdefault("unit", "constructor-tag")
                    child_meta.setdefault("role", "type-identity")
                walk(val[k], child, f"{path}.{k}" if path else k, child_meta)
            return
        if isinstance(val, list):
            if not val:
                emit(path, val, xd)
                return
            items = node.get("items") or {}
            item_meta = dict(node.get("x-itemDisplay") or {})
            if not item_meta:
                item_meta = dict(xd)
                item_meta.update(resolve(items, schema_root).get("x-display") or {})
            for i, it in enumerate(val):
                walk(it, items, f"{path}.{i}", item_meta)
            return
        emit(path, val, xd)

    walk(value, doc_schema, "", {})
    return entries


def set_path(obj, dotted, value):
    parts = dotted.split(".")
    cur = obj
    for p in parts[:-1]:
        cur = cur[int(p)] if p.isdigit() else cur[p]
    last = parts[-1]
    if last.isdigit():
        cur[int(last)] = value
    else:
        cur[last] = value


def apply_sets(doc, sets):
    out = deepcopy(doc)
    for path, value in (sets or {}).items():
        if path == "executionBody" and isinstance(value, dict):
            out["executionBody"] = deepcopy(value)
        else:
            set_path(out, path, deepcopy(value) if isinstance(value, (dict, list)) else value)
    return out


def typecheck(schema, def_name, body):
    from jsonschema import Draft202012Validator

    if def_name == "ClaimArray":
        sub = {
            "$schema": schema["$schema"],
            "$defs": schema["$defs"],
            "type": "array",
            "items": {"$ref": "#/$defs/ClaimRequirement"},
        }
    else:
        if def_name not in schema["$defs"]:
            raise SystemExit(f"missing def {def_name}")
        sub = {
            "$schema": schema["$schema"],
            "$defs": schema["$defs"],
            "$ref": "#/$defs/" + def_name,
        }
    errs = list(Draft202012Validator(sub).iter_errors(body))
    if errs:
        raise SystemExit(f"{def_name} typecheck: {errs[0].message} path={list(errs[0].absolute_path)}")


class Registry:
    def __init__(self):
        self.bodies = {}
        self.hashes = {}
        self.catalog = {}

    def put(self, name, closed, body, schema):
        if closed != "raw":
            typecheck(schema, closed, body)
            domain = DOMAINS[closed]
            digest = framed_hash(domain, body)
        else:
            domain = ""
            digest = raw_hash(body) if isinstance(body, str) else framed_hash("", body)
        self.bodies[name] = body
        self.hashes[name] = digest
        self.catalog[name] = {"closedType": closed, "domain": domain, "hash": digest}
        return digest


def residual(controller, ops, assets, gross, fees, nets, debts, recipients, partial, continuation):
    return {
        "controller": controller,
        "allowedEffectSet": ops,
        "allowedAssets": assets,
        "grossDebitCaps": gross,
        "feeCaps": fees,
        "netGoals": nets,
        "debtCaps": debts,
        "permittedRecipients": recipients,
        "permittedCalls": [],
        "validity": deepcopy(VALIDITY),
        "partialRule": deepcopy(partial),
        "continuation": continuation,
    }


def actor_ledger(actor, asset, gcap, fcap, goal, cg, cf, cn):
    return {
        "actor": actor,
        "asset": asset,
        "originalGrossCap": u(gcap),
        "originalFeeCap": u(fcap),
        "originalNetGoal": u(goal),
        "cumulativeGross": u(cg),
        "cumulativeFees": u(cf),
        "cumulativeNet": u(cn),
        "remainingGross": u(int(gcap) - int(cg)),
        "remainingFee": u(int(fcap) - int(cf)),
    }


def debt_ledger(created, accrued, cap):
    return {
        "debtor": "borrower",
        "creditor": "lender",
        "denomination": "USD_micro",
        "originalDebtCap": u(cap),
        "cumulativeCreated": u(created),
        "cumulativeAccrued": u(accrued),
        "cumulativeRepaid": "0",
        "cumulativeWrittenOff": "0",
        "cumulativeCredit": "0",
        "outstanding": u(int(created) + int(accrued)),
    }


def debt_record(accrued, outstanding, cap, cursor):
    return {
        "tag": "Debt",
        "debtId": "Loan01",
        "controller": "lender",
        "debtor": "borrower",
        "creditor": "lender",
        "denomination": "USD_micro",
        "principal": u(PRINCIPAL),
        "accrued": u(accrued),
        "outstanding": u(outstanding),
        "startDate": START,
        "nextPaymentDate": PERIOD_END,
        "lastAccrualEnd": cursor,
        "accrualPeriodSeconds": u(PERIOD),
        "allocationRule": "AccrualFirst",
        "negativeRateDisposition": "Reject",
        "liabilityCap": u(cap),
        "settlementAsset": "USD_TEST_ASSET",
        "conversionMantissa": "1",
        "conversionScale": "0",
        "conversionRounding": "floor",
        "status": "Outstanding",
    }


def state_body(event_order, fields, balances, debts, residual_cap, work, successor_hash, requests=None):
    return {
        "instanceId": "swap_loan_demo",
        "eventOrder": u(event_order),
        "fields": fields,
        "balances": balances,
        "debts": debts,
        "shares": [],
        "positions": [],
        "requests": requests or [],
        "messages": [],
        "eventClaims": [],
        "rewardAccounts": [],
        "locks": [],
        "duties": [],
        "status": deepcopy(STATUS_LIVE),
        "residualCapability": residual_cap,
        "remainingWork": deepcopy(work),
        "successorRecordHash": successor_hash,
    }


def successor_id(origin, consumption, child):
    return framed_hash(
        DOMAINS["SuccessorId"],
        {
            "originalIntentDigest": origin,
            "consumptionId": consumption,
            "childIndex": u(child),
        },
    )


def successor_record(
    tag,
    origin,
    consumption,
    partial,
    recipients,
    assumptions_hash,
    ledgers,
    debts,
    residual_cap,
    work,
    mode,
    recovery,
):
    return {
        "tag": tag,
        "successorId": successor_id(origin, consumption, 0),
        "originalIntentDigest": origin,
        "consumptionId": consumption,
        "partialRule": deepcopy(partial),
        "originalValidity": deepcopy(VALIDITY),
        "originalRecipients": deepcopy(recipients),
        "originalCalls": [],
        "originalAssumptionsHash": assumptions_hash,
        "originalWork": deepcopy(WORK_PRE),
        "actorLedgers": ledgers,
        "debtLedgers": debts,
        "residualCapability": residual_cap,
        "remainingWork": deepcopy(work),
        "predecessorMode": mode,
        "recoveryRights": deepcopy(recovery),
    }


def envelope_exact(nonce, principal, residual_auth, before, predecessors, exec_body, exec_hash, genesis_hash, profile_hash, program_hash, claim_root, policy_hash, assumptions):
    return {
        "kind": "ExactPlan",
        "signingDomain": "MORIARTY-SUCC-EXACT-PLAN/0",
        "schemaVersion": "moriarty-succ-sign/0",
        "network": "midnight_preview",
        "deployment": "preview_deployment_1",
        "principal": principal,
        "nonce": nonce,
        "validity": deepcopy(VALIDITY),
        "spec": {
            "semanticVersion": "moriarty-succ-core/0",
            "profileHash": profile_hash,
            "programHash": program_hash,
            "claimRoot": claim_root,
            "activationNotBefore": START,
            "revoked": False,
        },
        "policy": {
            "policyId": "constant_product_v0",
            "policyHash": policy_hash,
            "composition": deepcopy(COMP),
        },
        "profileBinding": {"profileName": "moriarty-successor/0", "profileHash": profile_hash},
        "instanceId": "swap_loan_demo",
        "genesisHash": genesis_hash,
        "requiredClaims": deepcopy(CLAIMS),
        "requiredClaimRoot": claim_root,
        "observationPolicy": {
            "requiredFields": ["now"],
            "maxStalenessSeconds": "3600",
            "requiredRole": "time_oracle",
        },
        "residualAuthority": residual_auth,
        "remainingWork": deepcopy(WORK_PRE),
        "recoveryRights": deepcopy(RECOVERY if principal == "trader" else RECOVERY_LENDER),
        "locks": [],
        "assumptions": deepcopy(assumptions),
        "beforeStateHash": before,
        "predecessors": predecessors,
        "executionBody": exec_body,
        "executionBodyHash": exec_hash,
    }


def envelope_outcome(profile_hash, program_hash, claim_root, policy_hash, genesis_hash, assumptions, residual_auth, predecessors):
    return {
        "kind": "OutcomeIntent",
        "signingDomain": "MORIARTY-SUCC-OUTCOME-INTENT/0",
        "schemaVersion": "moriarty-succ-sign/0",
        "network": "midnight_preview",
        "deployment": "preview_deployment_1",
        "principal": "trader",
        "nonce": "NonceIntent01",
        "validity": deepcopy(VALIDITY),
        "spec": {
            "semanticVersion": "moriarty-succ-core/0",
            "profileHash": profile_hash,
            "programHash": program_hash,
            "claimRoot": claim_root,
            "activationNotBefore": START,
            "revoked": False,
        },
        "policy": {
            "policyId": "constant_product_v0",
            "policyHash": policy_hash,
            "composition": deepcopy(COMP),
        },
        "profileBinding": {"profileName": "moriarty-successor/0", "profileHash": profile_hash},
        "instanceId": "swap_loan_demo",
        "genesisHash": genesis_hash,
        "requiredClaims": deepcopy(CLAIMS),
        "requiredClaimRoot": claim_root,
        "observationPolicy": {
            "requiredFields": ["now"],
            "maxStalenessSeconds": "3600",
            "requiredRole": "time_oracle",
        },
        "residualAuthority": residual_auth,
        "remainingWork": deepcopy(WORK_PRE),
        "recoveryRights": deepcopy(RECOVERY),
        "locks": [],
        "assumptions": deepcopy(assumptions),
        "allowedActions": ["accrue", "settle", "swap"],
        "grossDebitCaps": [
            {"actor": "trader", "asset": "AssetA", "maximum": "10000"},
            {"actor": "borrower", "asset": "USD_TEST_ASSET", "maximum": "0"},
        ],
        "feeCaps": [
            {"actor": "trader", "asset": "AssetB", "maximum": "0"},
            {"actor": "borrower", "asset": "USD_TEST_ASSET", "maximum": "0"},
        ],
        "netGoals": [
            {"actor": "trader", "asset": "AssetB", "minimumAfterFees": "19743"},
            {"actor": "lender", "asset": "USD_TEST_ASSET", "minimumAfterFees": "0"},
        ],
        "debtCaps": [
            {
                "creditor": "lender",
                "debtor": "borrower",
                "denomination": "USD_micro",
                "maximumOutstanding": u(LIABILITY_CAP),
            }
        ],
        "permittedRecipients": deepcopy(RECIPIENTS_ALL),
        "permittedCalls": [],
        "partialFill": deepcopy(PARTIAL_SWAP),
        "predecessorConstraint": {"tag": "Pinned", "predecessors": predecessors},
        "outPredicate": {"tag": "Min", "amount": amount("AssetB", 19743)},
    }


def wrapper(digest, principal, nonce, domain):
    return {
        "tag": "AuthorityWrapper",
        "intentDigest": digest,
        "signingDomain": domain,
        "principal": principal,
        "nonce": nonce,
        "signatureAbsent": True,
    }


def prepared(post_hash, effects, residual_cap, succ, work):
    return {
        "postStateHash": post_hash,
        "effects": deepcopy(effects),
        "duties": [],
        "residualCapability": residual_cap,
        "successors": [deepcopy(succ)],
        "remainingWork": deepcopy(work),
        "claims": deepcopy(CLAIMS),
    }


def acceptance(digest, plan_hash, obs_hash, effects, debts, work, pred_ids, succ, prepared_hash):
    return {
        "intentDigest": digest,
        "selectedPlanHash": plan_hash,
        "observationsHash": obs_hash,
        "effects": deepcopy(effects),
        "liabilities": deepcopy(debts),
        "remainingWork": deepcopy(work),
        "predecessorIds": pred_ids,
        "successorRecord": deepcopy(succ),
        "preparedHash": prepared_hash,
    }


def proof_context(program_hash, policy_hash, auth, plan_hash, pred_ids, obs_hash, post_hash, effects_hash, residual_hash, work):
    return {
        "programHash": program_hash,
        "specVersion": "moriarty-succ-sign/0",
        "policyHash": policy_hash,
        "authorizationDigest": auth,
        "selectedPlanHash": plan_hash,
        "predecessorIds": pred_ids,
        "observationsHash": obs_hash,
        "postStateHash": post_hash,
        "effectsHash": effects_hash,
        "residualCapabilityHash": residual_hash,
        "remainingWork": deepcopy(work),
        "claims": deepcopy(CLAIMS),
    }


def refs_for(names):
    out = []
    for n in names:
        closed = PREIMAGE_TYPES.get(n)
        if closed:
            out.append({"name": n, "closedType": closed})
    return out


def build(schema):
    r = Registry()
    source = "agreement swap_loan_demo\naction genesis\naction swap\naction accrue\n"
    r.bodies["sourceUtf8"] = source
    r.hashes["sourceUtf8"] = raw_hash(source)
    r.catalog["sourceUtf8"] = {"closedType": "raw", "domain": "", "hash": r.hashes["sourceUtf8"]}

    profile = {
        "profileName": "moriarty-successor/0",
        "sourceUtf8Bytes": 65536,
        "signingUtf8Bytes": 65536,
        "maxNodes": 4096,
        "maxSidecarBytes": 16384,
        "maxTotalVerificationWork": 65536,
        "predecessorFanIn": 8,
        "collectionCapacity": 128,
    }
    profile_hash = r.put("profileBody", "ProfileBody", profile, schema)
    program = {
        "sourceHash": r.hashes["sourceUtf8"],
        "entryActions": ["genesis", "swap", "accrue", "settle"],
        "coreVersion": "moriarty-succ-core/0",
    }
    program_hash = r.put("programBody", "ProgramBody", program, schema)
    claim_root = r.put("requiredClaims", "ClaimArray", CLAIMS, schema)
    policy = {
        "policyId": "constant_product_v0",
        "rules": ["seq_swap", "optional_accrue"],
        "composition": deepcopy(COMP),
    }
    policy_hash = r.put("policyBody", "PolicyBody", policy, schema)
    assumption_list = [
        {
            "assumptionId": "time_oracle_unproven",
            "kind": "oracle_truth",
            "statement": "obs.now is authenticated not proven true time",
            "evidenceRole": "required_observation",
        },
        {
            "assumptionId": "pool_custody",
            "kind": "custody",
            "statement": "pool outgoing AssetB is authorized by provider custody",
            "evidenceRole": "external_unproven",
        },
    ]
    assumptions = {"assumptions": assumption_list}
    assumptions_hash = r.put("assumptions", "AssumptionSet", assumptions, schema)

    ops_admin = [
        "Genesis",
        "Activate",
        "Transfer",
        "Exchange",
        "Fee",
        "RequestCancel",
        "Migrate",
        "DebtAccrue",
        "DebtCreate",
        "DebtRepay",
    ]
    residual_unborn = residual(
        "trader",
        ops_admin,
        ["AssetA", "AssetB", "USD_TEST_ASSET"],
        [{"actor": "trader", "asset": "AssetA", "maximum": "0"}],
        [{"actor": "trader", "asset": "AssetB", "maximum": "0"}],
        [{"actor": "trader", "asset": "AssetB", "minimumAfterFees": "0"}],
        [],
        RECIPIENTS_ALL,
        FOK_LOAN,
        "principal",
    )
    empty = successor_record(
        "GenesisInitial",
        ZERO,
        ZERO,
        FOK_LOAN,
        [],
        ZERO,
        [],
        [],
        residual_unborn,
        WORK_PRE,
        "GenesisNone",
        RECOVERY,
    )
    empty_h = r.put("emptySuccessor", "SuccessorRecord", empty, schema)
    unborn_fields = []
    unborn = state_body(0, unborn_fields, [], [], residual_unborn, WORK_PRE, empty_h)
    unborn_h = r.put("unbornStateBody", "StateBody", unborn, schema)

    genesis_body = {
        "instanceId": "swap_loan_demo",
        "profileHash": profile_hash,
        "programHash": program_hash,
        "claimRoot": claim_root,
        "lifetime": "31536000",
        "horizon": "31536000",
        "ordinaryWork": "8",
        "recoveryWork": "2",
        "initialStateHash": unborn_h,
        "network": "midnight_preview",
        "deployment": "preview_deployment_1",
        "principalBindings": ["trader", "lender", "borrower", "pool"],
        "observationBindings": ["now"],
    }
    genesis_hash = r.put("genesisBody", "GenesisBody", genesis_body, schema)

    obs = {
        "observations": [
            {
                "field": "now",
                "value": stored_u(NOW),
                "anchor": raw_hash("time-oracle-anchor"),
                "freshness": "1700003600",
                "role": "time_oracle",
                "authenticity": "wrapper_ok",
                "truthAssumption": "required_observation",
            }
        ]
    }
    obs_hash = r.put("observationSet", "ObservationSet", obs, schema)

    cons_g = {"network": "midnight_preview", "deployment": "preview_deployment_1", "principal": "trader", "nonce": "NonceGenesis01"}
    cons_s = {"network": "midnight_preview", "deployment": "preview_deployment_1", "principal": "trader", "nonce": "NonceSwap01"}
    cons_i = {"network": "midnight_preview", "deployment": "preview_deployment_1", "principal": "trader", "nonce": "NonceIntent01"}
    cons_a = {"network": "midnight_preview", "deployment": "preview_deployment_1", "principal": "lender", "nonce": "NonceAccrue01"}
    cons_g_h = r.put("consumptionGenesis", "ConsumptionIdBody", cons_g, schema)
    cons_s_h = r.put("consumptionSwap", "ConsumptionIdBody", cons_s, schema)
    cons_i_h = r.put("consumptionIntent", "ConsumptionIdBody", cons_i, schema)
    cons_a_h = r.put("consumptionAccrue", "ConsumptionIdBody", cons_a, schema)

    # --- genesis transition ---
    gen_fields = [
        field("trader_a", "Amount", amount("AssetA", 100000), {"tag": "Balance", "party": "trader", "asset": "AssetA"}),
        field("pool_a", "Amount", amount("AssetA", 1000000), {"tag": "Balance", "party": "pool", "asset": "AssetA"}),
        field("pool_b", "Amount", amount("AssetB", 2000000), {"tag": "Balance", "party": "pool", "asset": "AssetB"}),
        field("trader_b", "Amount", amount("AssetB", 0), {"tag": "Balance", "party": "trader", "asset": "AssetB"}),
        field("alive", "Bool", stored_bool(True), {"tag": "Flag"}),
    ]
    gen_bals = [
        bal("trader", "AssetA", 100000),
        bal("pool", "AssetA", 1000000),
        bal("pool", "AssetB", 2000000),
        bal("trader", "AssetB", 0),
    ]
    residual_genesis_signed = residual(
        "trader",
        ops_admin,
        ["AssetA", "AssetB", "USD_TEST_ASSET"],
        [{"actor": "trader", "asset": "AssetA", "maximum": "10000"}],
        [{"actor": "trader", "asset": "AssetB", "maximum": "0"}],
        [{"actor": "trader", "asset": "AssetB", "minimumAfterFees": "19743"}],
        [{"creditor": "lender", "debtor": "borrower", "denomination": "USD_micro", "maximumOutstanding": u(LIABILITY_CAP)}],
        RECIPIENTS_ALL,
        PARTIAL_SWAP,
        "principal",
    )
    residual_genesis_post = deepcopy(residual_genesis_signed)
    gen_exec = {
        "action": {"name": "genesis", "actor": "trader", "arguments": []},
        "exactWrites": [
            {"field": "trader_a", "value": amount("AssetA", 100000)},
            {"field": "pool_a", "value": amount("AssetA", 1000000)},
            {"field": "alive", "value": stored_bool(True)},
        ],
        "exactEffects": [
            {
                "kind": "Genesis",
                "ordinal": "0",
                "instanceId": "swap_loan_demo",
                "initialWork": deepcopy(WORK_PRE),
                "claimRoot": claim_root,
            }
        ],
    }
    gen_exec_h = r.put("genesisExecutionBody", "ExecutionBody", gen_exec, schema)
    gen_effects = r.put("effectsGenesis", "EffectsBody", {"effects": gen_exec["exactEffects"]}, schema)
    gen_doc = envelope_exact(
        "NonceGenesis01",
        "trader",
        residual_genesis_signed,
        unborn_h,
        [],
        gen_exec,
        gen_exec_h,
        genesis_hash,
        profile_hash,
        program_hash,
        claim_root,
        policy_hash,
        assumption_list,
    )
    typecheck(schema, "ExactPlanDocument", gen_doc)
    gen_digest = framed_hash(DOMAINS["ExecutionBody"] and "MORIARTY-SUCC-EXACT-PLAN/0", gen_doc)
    gen_succ = successor_record(
        "ActiveSuccessor",
        gen_digest,
        cons_g_h,
        PARTIAL_SWAP,
        RECIPIENTS_ALL,
        assumptions_hash,
        [],
        [],
        residual_genesis_post,
        WORK_POST,
        "GenesisNone",
        RECOVERY,
    )
    gen_succ_h = r.put("genesisSuccessor", "SuccessorRecord", gen_succ, schema)
    genesis_state = state_body(1, gen_fields, gen_bals, [], residual_genesis_post, WORK_POST, gen_succ_h)
    genesis_state_h = r.put("genesisStateBody", "StateBody", genesis_state, schema)
    residual_g_h = r.put("residualGenesis", "ResidualCapability", residual_genesis_post, schema)
    prep_g = prepared(genesis_state_h, gen_exec["exactEffects"], residual_genesis_post, gen_succ, WORK_POST)
    prep_g_h = r.put("preparedGenesis", "PreparedBody", prep_g, schema)
    wrap_g = wrapper(gen_digest, "trader", "NonceGenesis01", "MORIARTY-SUCC-EXACT-PLAN/0")
    wrap_g_h = r.put("authorityWrapperGenesis", "AuthorityWrapper", wrap_g, schema)
    acc_g = acceptance(gen_digest, gen_exec_h, obs_hash, gen_exec["exactEffects"], [], WORK_POST, [], gen_succ, prep_g_h)
    acc_g_h = r.put("acceptanceGenesis", "AcceptanceBody", acc_g, schema)

    # --- swap ---
    swap_effects = [
        {"kind": "Transfer", "ordinal": "0", "asset": "AssetA", "from": "trader", "to": "pool", "amount": "10000"},
        {"kind": "Transfer", "ordinal": "1", "asset": "AssetB", "from": "pool", "to": "trader", "amount": "19743"},
    ]
    swap_exec = {
        "action": {
            "name": "swap",
            "actor": "trader",
            "arguments": [
                {"name": "amount_in", "value": amount("AssetA", 10000)},
                {"name": "min_out", "value": amount("AssetB", 19743)},
            ],
        },
        "exactWrites": [
            {"field": "reserve_a", "value": amount("AssetA", 1010000)},
            {"field": "reserve_b", "value": amount("AssetB", 1980257)},
            {"field": "trader_a", "value": amount("AssetA", 90000)},
            {"field": "trader_b", "value": amount("AssetB", 19743)},
        ],
        "exactEffects": swap_effects,
        "dust": {"asset": "AssetB", "value": "1"},
        "rateOrPrice": {
            "tag": "Price",
            "base": "AssetB",
            "quote": "AssetA",
            "mantissa": "19743",
            "scale": "4",
            "direction": "basePerQuote",
        },
        "outPredicate": {"tag": "Min", "amount": amount("AssetB", 19743)},
        "remainder": {
            "tag": "Remainder",
            "numerator": "0",
            "denominator": "10000",
            "unit": "AssetB",
            "location": {"tag": "SourceReserve"},
        },
    }
    swap_exec_h = r.put("swapExecutionBody", "ExecutionBody", swap_exec, schema)
    r.put("outcomeSelectedPlan", "ExecutionBody", deepcopy(swap_exec), schema)
    swap_signed_residual = residual(
        "trader",
        ["Exchange", "Fee", "RequestCancel", "Transfer", "Migrate"],
        ["AssetA", "AssetB"],
        [{"actor": "trader", "asset": "AssetA", "maximum": "10000"}],
        [{"actor": "trader", "asset": "AssetB", "maximum": "0"}],
        [{"actor": "trader", "asset": "AssetB", "minimumAfterFees": "19743"}],
        [],
        RECIPIENTS_SWAP,
        PARTIAL_SWAP,
        "principal",
    )
    swap_post_residual = residual(
        "trader",
        ["Exchange", "Fee", "RequestCancel", "Transfer", "Migrate"],
        ["AssetA", "AssetB"],
        [{"actor": "trader", "asset": "AssetA", "maximum": "0"}],
        [{"actor": "trader", "asset": "AssetB", "maximum": "0"}],
        [{"actor": "trader", "asset": "AssetB", "minimumAfterFees": "19743"}],
        [],
        RECIPIENTS_SWAP,
        PARTIAL_SWAP,
        "principal",
    )
    pred_swap = [{"stateHash": genesis_state_h, "instanceId": "swap_loan_demo", "consumptionId": cons_g_h}]
    swap_doc = envelope_exact(
        "NonceSwap01",
        "trader",
        swap_signed_residual,
        genesis_state_h,
        pred_swap,
        swap_exec,
        swap_exec_h,
        genesis_hash,
        profile_hash,
        program_hash,
        claim_root,
        policy_hash,
        assumption_list,
    )
    typecheck(schema, "ExactPlanDocument", swap_doc)
    swap_digest = framed_hash("MORIARTY-SUCC-EXACT-PLAN/0", swap_doc)
    swap_ledgers = [
        actor_ledger("trader", "AssetA", 10000, 0, 0, 10000, 0, 0),
        actor_ledger("trader", "AssetB", 0, 0, 19743, 0, 0, 19743),
        actor_ledger("pool", "AssetA", 0, 0, 0, 0, 0, 10000),
        actor_ledger("pool", "AssetB", 19743, 0, 0, 19743, 0, 0),
    ]
    swap_succ = successor_record(
        "ActiveSuccessor",
        swap_digest,
        cons_s_h,
        PARTIAL_SWAP,
        RECIPIENTS_SWAP,
        assumptions_hash,
        swap_ledgers,
        [],
        swap_post_residual,
        WORK_POST,
        "Pinned",
        RECOVERY,
    )
    swap_succ_h = r.put("swapSuccessor", "SuccessorRecord", swap_succ, schema)
    swap_fields = [
        field("trader_a", "Amount", amount("AssetA", 90000), {"tag": "Balance", "party": "trader", "asset": "AssetA"}),
        field("pool_a", "Amount", amount("AssetA", 1010000), {"tag": "Balance", "party": "pool", "asset": "AssetA"}),
        field("pool_b", "Amount", amount("AssetB", 1980257), {"tag": "Balance", "party": "pool", "asset": "AssetB"}),
        field("trader_b", "Amount", amount("AssetB", 19743), {"tag": "Balance", "party": "trader", "asset": "AssetB"}),
        field("alive", "Bool", stored_bool(True), {"tag": "Flag"}),
    ]
    swap_bals = [
        bal("trader", "AssetA", 90000),
        bal("pool", "AssetA", 1010000),
        bal("pool", "AssetB", 1980257),
        bal("trader", "AssetB", 19743),
    ]
    swap_state = state_body(2, swap_fields, swap_bals, [], swap_post_residual, WORK_POST, swap_succ_h)
    swap_state_h = r.put("swapStateBody", "StateBody", swap_state, schema)
    effects_swap_h = r.put("effectsSwap", "EffectsBody", {"effects": swap_effects}, schema)
    residual_swap_h = r.put("residualSwap", "ResidualCapability", swap_post_residual, schema)
    prep_s = prepared(swap_state_h, swap_effects, swap_post_residual, swap_succ, WORK_POST)
    prep_s_h = r.put("preparedSwap", "PreparedBody", prep_s, schema)
    wrap_s = wrapper(swap_digest, "trader", "NonceSwap01", "MORIARTY-SUCC-EXACT-PLAN/0")
    wrap_s_h = r.put("authorityWrapperExact", "AuthorityWrapper", wrap_s, schema)
    acc_s = acceptance(swap_digest, swap_exec_h, obs_hash, swap_effects, [], WORK_POST, [cons_g_h], swap_succ, prep_s_h)
    acc_s_h = r.put("acceptanceSwap", "AcceptanceBody", acc_s, schema)
    pctx = proof_context(
        program_hash,
        policy_hash,
        wrap_s_h,
        swap_exec_h,
        [cons_g_h],
        obs_hash,
        swap_state_h,
        effects_swap_h,
        residual_swap_h,
        WORK_POST,
    )
    pctx_h = r.put("proofContextSwap", "ProofContext", pctx, schema)

    # --- accrue ---
    loan_debt = debt_record(0, PRINCIPAL, LIABILITY_CAP, START)
    loan_residual_signed = residual(
        "lender",
        ["DebtAccrue", "DebtCreate", "RequestCancel", "Migrate"],
        ["USD_TEST_ASSET"],
        [{"actor": "borrower", "asset": "USD_TEST_ASSET", "maximum": "0"}],
        [{"actor": "borrower", "asset": "USD_TEST_ASSET", "maximum": "0"}],
        [{"actor": "lender", "asset": "USD_TEST_ASSET", "minimumAfterFees": "0"}],
        [{"creditor": "lender", "debtor": "borrower", "denomination": "USD_micro", "maximumOutstanding": u(LIABILITY_CAP)}],
        RECIPIENTS_LOAN,
        FOK_LOAN,
        "principal",
    )
    loan_fields = [
        field("loan01", "Debt", loan_debt, {"tag": "Debt", "debtId": "Loan01"}),
        field("alive", "Bool", stored_bool(True), {"tag": "Flag"}),
    ]
    # loan prestate successor is genesis successor for bootstrap of this path
    loan_state = state_body(1, loan_fields, [], [loan_debt], loan_residual_signed, WORK_PRE, gen_succ_h)
    loan_h = r.put("loanStateBody", "StateBody", loan_state, schema)
    post_debt = debt_record(ACCRUED, PRINCIPAL + ACCRUED, LIABILITY_CAP, PERIOD_END)
    accrue_effects = [
        {
            "kind": "Accrual",
            "ordinal": "0",
            "debtId": "Loan01",
            "rate": {"tag": "Rate", "mantissa": u(RATE_MANTISSA), "scale": u(RATE_SCALE)},
            "periodStart": START,
            "periodEnd": PERIOD_END,
            "principalBefore": u(PRINCIPAL),
            "accruedDelta": u(ACCRUED),
            "outstandingAfter": u(PRINCIPAL + ACCRUED),
            "capApplied": False,
            "lastAccrualEndBefore": START,
            "periodCursorAfter": PERIOD_END,
        }
    ]
    accrue_exec = {
        "action": {
            "name": "accrue",
            "actor": "lender",
            "arguments": [
                {"name": "debt_id", "value": {"tag": "Text", "value": "Loan01"}},
                {
                    "name": "qty",
                    "value": {
                        "tag": "Quantity",
                        "mantissa": u(ACCRUED),
                        "scale": "0",
                        "units": [{"symbol": "USD_micro", "exponent": 1}],
                    },
                },
            ],
        },
        "exactWrites": [{"field": "loan01", "value": post_debt}],
        "exactEffects": accrue_effects,
    }
    accrue_exec_h = r.put("accrueExecutionBody", "ExecutionBody", accrue_exec, schema)
    pred_acc = [{"stateHash": loan_h, "instanceId": "swap_loan_demo", "consumptionId": cons_g_h}]
    accrue_doc = envelope_exact(
        "NonceAccrue01",
        "lender",
        loan_residual_signed,
        loan_h,
        pred_acc,
        accrue_exec,
        accrue_exec_h,
        genesis_hash,
        profile_hash,
        program_hash,
        claim_root,
        policy_hash,
        assumption_list,
    )
    typecheck(schema, "ExactPlanDocument", accrue_doc)
    accrue_digest = framed_hash("MORIARTY-SUCC-EXACT-PLAN/0", accrue_doc)
    accrue_post_residual = deepcopy(loan_residual_signed)
    dled = debt_ledger(PRINCIPAL, ACCRUED, LIABILITY_CAP)
    accrue_succ = successor_record(
        "ActiveSuccessor",
        accrue_digest,
        cons_a_h,
        FOK_LOAN,
        RECIPIENTS_LOAN,
        assumptions_hash,
        [],
        [dled],
        accrue_post_residual,
        WORK_POST,
        "Pinned",
        RECOVERY_LENDER,
    )
    accrue_succ_h = r.put("accrueSuccessor", "SuccessorRecord", accrue_succ, schema)
    accrue_fields = [
        field("loan01", "Debt", post_debt, {"tag": "Debt", "debtId": "Loan01"}),
        field("alive", "Bool", stored_bool(True), {"tag": "Flag"}),
    ]
    accrue_post = state_body(2, accrue_fields, [], [post_debt], accrue_post_residual, WORK_POST, accrue_succ_h)
    accrue_post_h = r.put("accruePostStateBody", "StateBody", accrue_post, schema)
    r.put("effectsAccrue", "EffectsBody", {"effects": accrue_effects}, schema)
    r.put("residualAccrue", "ResidualCapability", accrue_post_residual, schema)
    prep_a = prepared(accrue_post_h, accrue_effects, accrue_post_residual, accrue_succ, WORK_POST)
    prep_a_h = r.put("preparedAccrue", "PreparedBody", prep_a, schema)
    wrap_a = wrapper(accrue_digest, "lender", "NonceAccrue01", "MORIARTY-SUCC-EXACT-PLAN/0")
    wrap_a_h = r.put("authorityWrapperAccrue", "AuthorityWrapper", wrap_a, schema)
    acc_a = acceptance(accrue_digest, accrue_exec_h, obs_hash, accrue_effects, [post_debt], WORK_POST, [cons_g_h], accrue_succ, prep_a_h)
    acc_a_h = r.put("acceptanceAccrue", "AcceptanceBody", acc_a, schema)

    # --- outcome ---
    outcome_residual_signed = residual(
        "trader",
        ["DebtCreate", "DebtRepay", "Exchange", "Fee", "RequestCancel", "Transfer", "Migrate"],
        ["AssetA", "AssetB", "USD_TEST_ASSET"],
        [
            {"actor": "trader", "asset": "AssetA", "maximum": "10000"},
            {"actor": "borrower", "asset": "USD_TEST_ASSET", "maximum": "0"},
        ],
        [
            {"actor": "trader", "asset": "AssetB", "maximum": "0"},
            {"actor": "borrower", "asset": "USD_TEST_ASSET", "maximum": "0"},
        ],
        [
            {"actor": "trader", "asset": "AssetB", "minimumAfterFees": "19743"},
            {"actor": "lender", "asset": "USD_TEST_ASSET", "minimumAfterFees": "0"},
        ],
        [{"creditor": "lender", "debtor": "borrower", "denomination": "USD_micro", "maximumOutstanding": u(LIABILITY_CAP)}],
        RECIPIENTS_ALL,
        PARTIAL_SWAP,
        "principal",
    )
    outcome_post_residual = residual(
        "trader",
        ["DebtCreate", "DebtRepay", "Exchange", "Fee", "RequestCancel", "Transfer", "Migrate"],
        ["AssetA", "AssetB", "USD_TEST_ASSET"],
        [
            {"actor": "trader", "asset": "AssetA", "maximum": "0"},
            {"actor": "borrower", "asset": "USD_TEST_ASSET", "maximum": "0"},
        ],
        [
            {"actor": "trader", "asset": "AssetB", "maximum": "0"},
            {"actor": "borrower", "asset": "USD_TEST_ASSET", "maximum": "0"},
        ],
        [
            {"actor": "trader", "asset": "AssetB", "minimumAfterFees": "19743"},
            {"actor": "lender", "asset": "USD_TEST_ASSET", "minimumAfterFees": "0"},
        ],
        [{"creditor": "lender", "debtor": "borrower", "denomination": "USD_micro", "maximumOutstanding": u(LIABILITY_CAP)}],
        RECIPIENTS_ALL,
        PARTIAL_SWAP,
        "principal",
    )
    outcome_doc = envelope_outcome(
        profile_hash,
        program_hash,
        claim_root,
        policy_hash,
        genesis_hash,
        assumption_list,
        outcome_residual_signed,
        pred_swap,
    )
    typecheck(schema, "OutcomeIntentDocument", outcome_doc)
    outcome_digest = framed_hash("MORIARTY-SUCC-OUTCOME-INTENT/0", outcome_doc)
    outcome_ledgers = [
        actor_ledger("trader", "AssetA", 10000, 0, 0, 10000, 0, 0),
        actor_ledger("trader", "AssetB", 0, 0, 19743, 0, 0, 19743),
        actor_ledger("pool", "AssetA", 0, 0, 0, 0, 0, 10000),
        actor_ledger("pool", "AssetB", 19743, 0, 0, 19743, 0, 0),
        actor_ledger("borrower", "USD_TEST_ASSET", 0, 0, 0, 0, 0, 0),
        actor_ledger("lender", "USD_TEST_ASSET", 0, 0, 0, 0, 0, 0),
    ]
    outcome_succ = successor_record(
        "ActiveSuccessor",
        outcome_digest,
        cons_i_h,
        PARTIAL_SWAP,
        RECIPIENTS_ALL,
        assumptions_hash,
        outcome_ledgers,
        [debt_ledger(0, 0, LIABILITY_CAP)],
        outcome_post_residual,
        WORK_POST,
        "Pinned",
        RECOVERY,
    )
    outcome_succ_h = r.put("outcomeSuccessor", "SuccessorRecord", outcome_succ, schema)
    outcome_post = state_body(2, swap_fields, swap_bals, [], outcome_post_residual, WORK_POST, outcome_succ_h)
    outcome_post_h = r.put("outcomePostStateBody", "StateBody", outcome_post, schema)
    r.put("effectsOutcome", "EffectsBody", {"effects": swap_effects}, schema)
    r.put("residualOutcome", "ResidualCapability", outcome_post_residual, schema)
    prep_o = prepared(outcome_post_h, swap_effects, outcome_post_residual, outcome_succ, WORK_POST)
    prep_o_h = r.put("preparedOutcome", "PreparedBody", prep_o, schema)
    wrap_o = wrapper(outcome_digest, "trader", "NonceIntent01", "MORIARTY-SUCC-OUTCOME-INTENT/0")
    wrap_o_h = r.put("authorityWrapperOutcome", "AuthorityWrapper", wrap_o, schema)
    acc_o = acceptance(
        outcome_digest,
        swap_exec_h,
        obs_hash,
        swap_effects,
        [],
        WORK_POST,
        [cons_g_h],
        outcome_succ,
        prep_o_h,
    )
    acc_o_h = r.put("acceptanceOutcome", "AcceptanceBody", acc_o, schema)

    request = {
        "tag": "Request",
        "requestId": "SwapFill01",
        "controller": "trader",
        "asset": "AssetA",
        "committed": "10000",
        "filled": "0",
        "claimed": "0",
        "status": "Pending",
    }
    cancel_pre = state_body(1, gen_fields, gen_bals, [], residual_genesis_post, WORK_POST, gen_succ_h, requests=[request])
    r.put("cancelPreStateBody", "StateBody", cancel_pre, schema)

    schema_id = schema["$id"]
    plan_schema = {"$ref": "#/$defs/ExactPlanDocument", "$defs": schema["$defs"]}
    # Use full schema oneOf for display so ExactPlan/OutcomeIntent match.
    def display_for(doc):
        return project_display(doc, schema, schema)

    valid = []

    def add_valid(vid, doc, digest_key, digest, hashes, selected, pref_names):
        disp = display_for(doc)
        valid.append(
            {
                "id": vid,
                "kind": doc["kind"],
                "canonicalUtf8": canonical(doc),
                "displayProjection": disp,
                "hashes": hashes,
                "selectedPlanRef": {"name": selected, "closedType": "ExecutionBody"},
                "preimageRefs": refs_for(pref_names),
            }
        )
        hashes[digest_key] = digest

    add_valid(
        "exact-plan-swap-min-receive",
        swap_doc,
        "exactPlanDigest",
        swap_digest,
        {
            "exactPlanDigest": swap_digest,
            "executionBodyHash": swap_exec_h,
            "acceptanceBind": acc_s_h,
            "observationsHash": obs_hash,
            "profileHash": profile_hash,
            "programHash": program_hash,
            "genesisHash": genesis_hash,
            "stateHash": swap_state_h,
            "authorityDigest": wrap_s_h,
            "preparedHash": prep_s_h,
            "claimRoot": claim_root,
            "consumptionId": cons_s_h,
            "proofContextHash": pctx_h,
            "sourceHash": r.hashes["sourceUtf8"],
        },
        "swapExecutionBody",
        [
            "acceptanceSwap",
            "authorityWrapperExact",
            "requiredClaims",
            "swapExecutionBody",
            "genesisBody",
            "observationSet",
            "preparedSwap",
            "profileBody",
            "programBody",
            "proofContextSwap",
            "swapStateBody",
            "swapSuccessor",
        ],
    )
    add_valid(
        "outcome-intent-swap-and-loan-caps",
        outcome_doc,
        "outcomeIntentDigest",
        outcome_digest,
        {
            "outcomeIntentDigest": outcome_digest,
            "claimRoot": claim_root,
            "acceptanceBind": acc_o_h,
            "selectedPlanHash": swap_exec_h,
            "observationsHash": obs_hash,
            "profileHash": profile_hash,
            "programHash": program_hash,
            "genesisHash": genesis_hash,
            "authorityDigest": wrap_o_h,
            "preparedHash": prep_o_h,
            "sourceHash": r.hashes["sourceUtf8"],
        },
        "outcomeSelectedPlan",
        [
            "acceptanceOutcome",
            "authorityWrapperOutcome",
            "requiredClaims",
            "outcomeSelectedPlan",
            "genesisBody",
            "observationSet",
            "preparedOutcome",
            "profileBody",
            "programBody",
            "outcomePostStateBody",
            "outcomeSuccessor",
        ],
    )
    add_valid(
        "exact-plan-genesis-no-predecessor",
        gen_doc,
        "exactPlanDigest",
        gen_digest,
        {
            "exactPlanDigest": gen_digest,
            "executionBodyHash": gen_exec_h,
            "acceptanceBind": acc_g_h,
            "observationsHash": obs_hash,
            "genesisHash": genesis_hash,
            "stateHash": genesis_state_h,
            "authorityDigest": wrap_g_h,
            "preparedHash": prep_g_h,
            "profileHash": profile_hash,
            "programHash": program_hash,
            "claimRoot": claim_root,
            "sourceHash": r.hashes["sourceUtf8"],
        },
        "genesisExecutionBody",
        [
            "acceptanceGenesis",
            "authorityWrapperGenesis",
            "requiredClaims",
            "genesisExecutionBody",
            "genesisBody",
            "observationSet",
            "preparedGenesis",
            "profileBody",
            "programBody",
            "genesisStateBody",
            "unbornStateBody",
            "genesisSuccessor",
        ],
    )
    add_valid(
        "exact-plan-accrue-nonexchange",
        accrue_doc,
        "exactPlanDigest",
        accrue_digest,
        {
            "exactPlanDigest": accrue_digest,
            "executionBodyHash": accrue_exec_h,
            "acceptanceBind": acc_a_h,
            "observationsHash": obs_hash,
            "stateHash": accrue_post_h,
            "authorityDigest": wrap_a_h,
            "preparedHash": prep_a_h,
            "consumptionId": cons_a_h,
            "programHash": program_hash,
            "sourceHash": r.hashes["sourceUtf8"],
            "genesisHash": genesis_hash,
            "claimRoot": claim_root,
        },
        "accrueExecutionBody",
        [
            "acceptanceAccrue",
            "authorityWrapperAccrue",
            "requiredClaims",
            "accrueExecutionBody",
            "observationSet",
            "preparedAccrue",
            "programBody",
            "loanStateBody",
            "accruePostStateBody",
            "accrueSuccessor",
        ],
    )

    docs = {row["id"]: json.loads(row["canonicalUtf8"]) for row in valid}

    migrate_exec = {
        "action": {"actor": "trader", "arguments": [], "name": "migrate"},
        "exactEffects": [
            {
                "kind": "Migrate",
                "ordinal": "0",
                "fromStateHash": genesis_state_h,
                "toProgramHash": program_hash,
            }
        ],
        "exactWrites": [],
    }
    migrate_exec_h = framed_hash(DOMAINS["ExecutionBody"], migrate_exec)
    migrate_sets = {
        "executionBody": migrate_exec,
        "executionBodyHash": migrate_exec_h,
    }
    migrate_doc = apply_sets(docs["exact-plan-swap-min-receive"], migrate_sets)
    migrate_digest = framed_hash("MORIARTY-SUCC-EXACT-PLAN/0", migrate_doc)
    wrap_m = wrapper(migrate_digest, "trader", "NonceSwap01", "MORIARTY-SUCC-EXACT-PLAN/0")
    wrap_m_h = r.put("authorityWrapperMigrate", "AuthorityWrapper", wrap_m, schema)

    cancel_sets = {"allowedActions": ["RequestCancel"]}
    cancel_doc = apply_sets(docs["outcome-intent-swap-and-loan-caps"], cancel_sets)
    cancel_digest = framed_hash("MORIARTY-SUCC-OUTCOME-INTENT/0", cancel_doc)
    wrap_c = wrapper(cancel_digest, "trader", "NonceIntent01", "MORIARTY-SUCC-OUTCOME-INTENT/0")
    wrap_c_h = r.put("authorityWrapperCancel", "AuthorityWrapper", wrap_c, schema)

    fee_sets = {
        "feeCaps.0.maximum": "30",
        "residualAuthority.feeCaps.0.maximum": "30",
    }
    fee_doc = apply_sets(docs["outcome-intent-swap-and-loan-caps"], fee_sets)
    fee_digest = framed_hash("MORIARTY-SUCC-OUTCOME-INTENT/0", fee_doc)
    wrap_f = wrapper(fee_digest, "trader", "NonceIntent01", "MORIARTY-SUCC-OUTCOME-INTENT/0")
    wrap_f_h = r.put("authorityWrapperFee", "AuthorityWrapper", wrap_f, schema)

    fee_plan = deepcopy(swap_exec)
    fee_plan["exactEffects"] = swap_effects + [
        {"kind": "Fee", "ordinal": "2", "asset": "AssetB", "from": "trader", "to": "pool", "amount": "30"}
    ]
    fee_plan["exactWrites"] = [
        {"field": "reserve_a", "value": amount("AssetA", 1010000)},
        {"field": "reserve_b", "value": amount("AssetB", 1980287)},
        {"field": "trader_a", "value": amount("AssetA", 90000)},
        {"field": "trader_b", "value": amount("AssetB", 19713)},
    ]

    clone_child = deepcopy(outcome_succ)
    clone_child["remainingWork"] = deepcopy(WORK_PRE)
    child_a = deepcopy(clone_child)
    child_a["successorId"] = successor_id(outcome_digest, cons_i_h, 1)
    child_a["residualCapability"] = deepcopy(child_a["residualCapability"])
    child_a["residualCapability"]["controller"] = "delegate_a"
    child_b = deepcopy(clone_child)
    child_b["successorId"] = successor_id(outcome_digest, cons_i_h, 2)
    child_b["residualCapability"] = deepcopy(child_b["residualCapability"])
    child_b["residualCapability"]["controller"] = "delegate_b"

    def inv(iid, base, stage, schema_expected, sets, reason, context=None, **extra):
        row = {
            "id": iid,
            "base": base,
            "intendedFailureStage": stage,
            "schemaExpected": schema_expected,
            "sets": sets,
            "reason": reason,
            "signatureClaim": False,
            "executionStatus": "specified-only",
        }
        if context is not None:
            row["context"] = context
        row.update(extra)
        return row

    pred_dup = [
        {"consumptionId": cons_g_h, "instanceId": "swap_loan_demo", "stateHash": genesis_state_h},
        {"consumptionId": cons_g_h, "instanceId": "swap_loan_demo", "stateHash": genesis_state_h},
    ]
    invalid = [
        inv("inv-extra-property", "exact-plan-swap-min-receive", "schema", "fail", {"unknownExtra": "wildcard"}, "additionalProperties false rejects extra property before hashing."),
        inv("inv-noncanonical-integer", "exact-plan-swap-min-receive", "schema", "fail", {"executionBody.exactEffects.0.amount": "01000"}, "Leading zero is noncanonical UIntText."),
        inv("inv-wrong-network", "outcome-intent-swap-and-loan-caps", "authorization", "pass", {"network": "midnight_mainnet"}, "Schema-valid identifier. Expected midnight_preview is contextual authorization."),
        inv("inv-wrong-deployment", "outcome-intent-swap-and-loan-caps", "authorization", "pass", {"deployment": "other_deployment"}, "Schema-valid deployment. Durable consumption binding rejects mismatch."),
        inv(
            "inv-display-mismatch-gross-fee-debt",
            "outcome-intent-swap-and-loan-caps",
            "display",
            "pass",
            {},
            "Display disagrees with parsed canonical signed bytes for fee, debt, and gross.",
            badDisplay={"path": "feeCaps.0.maximum", "value": "99"},
        ),
        inv("inv-duplicate-predecessor", "exact-plan-swap-min-receive", "history", "pass", {"predecessors": pred_dup}, "Duplicate consumptionId. Schema allows the array. HistoryCompliance rejects."),
        inv("inv-revoked-spec", "outcome-intent-swap-and-loan-caps", "authorization", "pass", {"spec.revoked": True}, "revoked true is schema-valid. SPEC rejects even with a later valid proof."),
        inv("inv-expired-spec-window", "outcome-intent-swap-and-loan-caps", "authorization", "pass", {"validity.notAfterExclusive": "1600000000"}, "Window ends before documented obs.now 1700000000. Relational, not schema."),
        inv("inv-partial-fill-exceeds-cap", "outcome-intent-swap-and-loan-caps", "evaluation", "pass", {"partialFill.minFill.value": "20000"}, "minFill 20000 exceeds cumulativeCap 10000. Relational."),
        inv(
            "inv-cloned-residual-work",
            "outcome-intent-swap-and-loan-caps",
            "evaluation",
            "pass",
            {},
            "Two children each inherit ordinaryRemaining 8 and recoveryRemaining 2. Sum 16+4 exceeds parent 8+2. Recovery alias duplicated.",
            context={
                "preStateRef": "genesisStateBody",
                "signedAuthorizationRef": "authorityWrapperOutcome",
                "selectedPlanRef": "swapExecutionBody",
                "cumulativeHistoryRef": "swapSuccessor",
                "parentWork": deepcopy(WORK_PRE),
                "children": [child_a, child_b],
                "clonePredicate": "sum(child remaining) exceeds parent remainingWork, and both children alias the same recovery reserve",
            },
        ),
        inv(
            "inv-cancellation-race",
            "outcome-intent-swap-and-loan-caps",
            "ledger-currentness",
            "pass",
            cancel_sets,
            "Cancel uses consumed nonce NonceIntent01. Winning fill already consumed the residual.",
            context={
                "preStateRef": "cancelPreStateBody",
                "signedAuthorizationRef": "authorityWrapperCancel",
                "selectedPlan": {
                    "action": {"name": "RequestCancel", "actor": "trader", "arguments": []},
                    "exactEffects": [{"kind": "RequestCancel", "ordinal": "0", "requestId": "SwapFill01"}],
                    "exactWrites": [],
                },
                "cumulativeHistoryRef": "swapSuccessor",
                "winningFill": deepcopy(acc_s),
                "staleResidual": {
                    "action": "RequestCancel",
                    "usesNonce": "NonceIntent01",
                    "staleConsumptionId": cons_i_h,
                    "staleStateHash": genesis_state_h,
                    "staleCurrentnessId": cons_i_h,
                },
            },
        ),
        inv(
            "inv-replay-migration",
            "exact-plan-swap-min-receive",
            "history",
            "pass",
            migrate_sets,
            "Migrate reuses consumed predecessor. Execution body, wrapper, prepared, successor and acceptance hashes recomputed so earlier hash and authorization checks pass. History rejection isolated.",
            context={
                "preStateRef": "genesisStateBody",
                "signedAuthorizationRef": "authorityWrapperMigrate",
                "selectedPlan": migrate_exec,
                "cumulativeHistoryRef": "swapSuccessor",
                "consumedPredecessor": {
                    "consumptionId": cons_g_h,
                    "stateHash": genesis_state_h,
                    "status": "consumed-by-swap",
                },
            },
            recomputeExecutionBodyHash=True,
            recomputeDependentHashes=[
                "executionBodyHash",
                "exactPlanDigest",
                "authorityDigest",
                "preparedHash",
                "acceptanceBind",
                "successorRecordHash",
            ],
        ),
        inv("inv-unknown-extension", "exact-plan-swap-min-receive", "schema", "fail", {"x_extension": {"hook": "unbounded"}}, "Unknown extension. Reject before hash and proof."),
        inv(
            "inv-exact-output-overdelivery",
            "exact-plan-swap-min-receive",
            "evaluation",
            "pass",
            {
                "executionBody.outPredicate.tag": "Exact",
                "executionBody.exactEffects.1.amount": "19744",
                "executionBody.exactWrites.3.value.value": "19744",
                "executionBody.exactWrites.1.value.value": "1980256",
            },
            "Exact 19743 with delivered 19744. Over-delivery rejects.",
            recomputeExecutionBodyHash=True,
        ),
        inv(
            "inv-net-after-fees-shortfall",
            "outcome-intent-swap-and-loan-caps",
            "evaluation",
            "pass",
            fee_sets,
            "Selected plan emits Fee 30 AssetB. Net AssetB to trader is 19713 which is below net goal 19743.",
            context={
                "preStateRef": "genesisStateBody",
                "signedAuthorizationRef": "authorityWrapperFee",
                "selectedPlan": fee_plan,
                "selectedPlanHash": framed_hash(DOMAINS["ExecutionBody"], fee_plan),
                "cumulativeHistoryRef": "swapSuccessor",
                "feeEffect": {"kind": "Fee", "ordinal": "2", "asset": "AssetB", "from": "trader", "to": "pool", "amount": "30"},
                "netShortfall": {"actor": "trader", "asset": "AssetB", "goal": "19743", "grossOut": "19743", "fee": "30", "netAfterFee": "19713"},
            },
        ),
        inv("inv-principal-trailing-newline", "outcome-intent-swap-and-loan-caps", "schema", "fail", {"principal": "trader\n"}, "ECMA-262 $ would allow a final newline. True-end (?![\\s\\S]) rejects."),
        inv("inv-cap-trailing-newline", "outcome-intent-swap-and-loan-caps", "schema", "fail", {"grossDebitCaps.0.maximum": "1\n"}, "Cap value ending newline must fail the canonical scalar pattern."),
        inv("inv-principal-trailing-cr", "outcome-intent-swap-and-loan-caps", "schema", "fail", {"principal": "trader\r"}, "Trailing CR is not a canonical identifier."),
        inv("inv-principal-unicode-newline", "outcome-intent-swap-and-loan-caps", "schema", "fail", {"principal": "trader\u2028"}, "U+2028 line separator must fail the true-end identifier pattern."),
        inv("inv-uint64-max-ok", "outcome-intent-swap-and-loan-caps", "none", "pass", {"validity.notBefore": UINT64_MAX}, "UInt64 max is in range. DomainValidation pass. Not a failure fixture; positive domain bound."),
        inv("inv-uint64-max-plus-one", "outcome-intent-swap-and-loan-caps", "domain", "pass", {"validity.notBefore": UINT64_MAX1}, "2^64 is 20 canonical digits so schema may pass. DomainValidation must reject before hash."),
        inv("inv-uint128-max-ok", "outcome-intent-swap-and-loan-caps", "none", "pass", {"grossDebitCaps.0.maximum": UINT128_MAX}, "UInt128 max is in range. DomainValidation pass."),
        inv("inv-uint128-max-plus-one", "outcome-intent-swap-and-loan-caps", "domain", "pass", {"grossDebitCaps.0.maximum": UINT128_MAX1}, "2^128 is 39 canonical digits so schema may pass. DomainValidation must reject before hash."),
        inv(
            "inv-duplicate-four-contract-invariant",
            "outcome-intent-swap-and-loan-caps",
            "schema",
            "fail",
            {
                "requiredClaims": [
                    {"claimId": "a", "kind": "ContractInvariant"},
                    {"claimId": "b", "kind": "ContractInvariant"},
                    {"claimId": "c", "kind": "ContractInvariant"},
                    {"claimId": "d", "kind": "ContractInvariant"},
                ]
            },
            "Array length 4 with four ContractInvariant copies. Must fail contains-exactly-once.",
        ),
        inv(
            "inv-display-missing-entry",
            "exact-plan-swap-min-receive",
            "display",
            "pass",
            {},
            "Projection omits network. Completeness check rejects.",
            displayMutation={"dropPath": "network"},
        ),
        inv(
            "inv-display-extra-entry",
            "exact-plan-swap-min-receive",
            "display",
            "pass",
            {},
            "Projection adds an unsigned hidden field. Completeness check rejects.",
            displayMutation={"extra": {"path": "hidden", "label": "Hidden", "unit": "none", "role": "unsigned", "value": True}},
        ),
        inv(
            "inv-display-changed-entry",
            "exact-plan-swap-min-receive",
            "display",
            "pass",
            {},
            "Projection changes residual remaining gross. Matching check rejects.",
            displayMutation={"path": "residualAuthority.grossDebitCaps.0.maximum", "value": "1"},
        ),
    ]

    inner_edges = [
        {"from": "accruePostStateBody.successorRecordHash", "to": "accrueSuccessor", "domain": DOMAINS["SuccessorRecord"]},
        {"from": "preparedAccrue.postStateHash", "to": "accruePostStateBody", "domain": DOMAINS["StateBody"]},
        {"from": "preparedAccrue.successors.0.originalIntentDigest", "to": "exact-plan-accrue-nonexchange", "domain": "MORIARTY-SUCC-EXACT-PLAN/0"},
        {"from": "accrueSuccessor.originalIntentDigest", "to": "exact-plan-accrue-nonexchange", "domain": "MORIARTY-SUCC-EXACT-PLAN/0"},
        {"from": "preparedOutcome.postStateHash", "to": "outcomePostStateBody", "domain": DOMAINS["StateBody"]},
        {"from": "genesisBody.initialStateHash", "to": "unbornStateBody", "domain": DOMAINS["StateBody"]},
        {"from": "unbornStateBody.successorRecordHash", "to": "emptySuccessor", "domain": DOMAINS["SuccessorRecord"]},
        {"from": "swapStateBody.successorRecordHash", "to": "swapSuccessor", "domain": DOMAINS["SuccessorRecord"]},
        {"from": "preparedSwap.postStateHash", "to": "swapStateBody", "domain": DOMAINS["StateBody"]},
        {"from": "policy.policyHash", "to": "policyBody", "domain": DOMAINS["PolicyBody"]},
        {"from": "proofContextSwap.effectsHash", "to": "effectsSwap", "domain": DOMAINS["EffectsBody"]},
        {"from": "proofContextSwap.residualCapabilityHash", "to": "residualSwap", "domain": DOMAINS["ResidualCapability"]},
        {"from": "swapSuccessor.originalAssumptionsHash", "to": "assumptions", "domain": DOMAINS["AssumptionSet"]},
        {
            "staleEdgesFixed": [
                "accruePostStateBody.successorRecordHash previously hashed a stale successor copy",
                "preparedAccrue.postStateHash previously hashed a stale post-state copy",
                "preparedAccrue.successors.0.originalIntentDigest previously bound an old accrue digest",
                "preparedOutcome.postStateHash previously reused swapStateBody instead of outcomePostStateBody",
            ]
        },
    ]

    examples = {
        "status": "proposed",
        "schemaId": schema_id,
        "hashFunction": "SHA-256",
        "canonicalEncoding": "json.dumps(sort_keys=True, separators=(',', ':'), ensure_ascii=False)",
        "framedHash": "SHA256(UTF8(domain) || 0x00 || canonical object bytes). sourceHash is SHA256(raw utf-8) with empty domain and no NUL frame.",
        "reconstructInvalid": "Start from valid[id=base] parsed from canonicalUtf8. Apply sets in order. If recomputeExecutionBodyHash, set executionBodyHash to framed hash of executionBody. If recomputeDependentHashes, recompute listed names from retained typed bodies. Context fixtures are specified-only.",
        "displayProjectionRule": "Walk parsed canonical object in parallel with the schema. Object keys in increasing UTF-8 byte order. Arrays in index order. Emit one entry per leaf. Empty arrays emit value [] and empty objects emit {}. Path is dotted with decimal indices. Labels, units and roles come from schema x-display on the property, or x-itemDisplay on primitive array items. Constructor tag fields use label Constructor tag, unit constructor-tag, role type-identity. Missing metadata is an error. No last-segment or signed-field fallback. Value equals the parsed leaf, including JSON booleans, numbers and arrays.",
        "domainValidation": {
            "stage": "pre-hash",
            "uint64": {"min": "0", "max": UINT64_MAX},
            "uint128": {"min": "0", "max": UINT128_MAX},
            "sint128": {"min": str(-(1 << 127)), "max": str((1 << 127) - 1)},
            "requiredClaims": "four kinds exactly once",
            "recoveryAlias": "AliasReserve which=recovery",
            "overflowDeferredToProof": False,
        },
        "domainTagsProposed": {
            "execBody": DOMAINS["ExecutionBody"],
            "exactPlan": "MORIARTY-SUCC-EXACT-PLAN/0",
            "outcomeIntent": "MORIARTY-SUCC-OUTCOME-INTENT/0",
            "acceptance": DOMAINS["AcceptanceBody"],
            "bounds": DOMAINS["ProfileBody"],
            "program": DOMAINS["ProgramBody"],
            "claims": DOMAINS["ClaimArray"],
            "policy": DOMAINS["PolicyBody"],
            "genesis": DOMAINS["GenesisBody"],
            "state": DOMAINS["StateBody"],
            "obs": DOMAINS["ObservationSet"],
            "authority": DOMAINS["AuthorityWrapper"],
            "proof": DOMAINS["ProofContext"],
            "trace": DOMAINS["PreparedBody"],
            "successor": DOMAINS["SuccessorRecord"],
            "consumption": DOMAINS["ConsumptionIdBody"],
            "effects": DOMAINS["EffectsBody"],
            "residual": DOMAINS["ResidualCapability"],
            "assumptions": DOMAINS["AssumptionSet"],
            "notReused": ["MORIARTY-SIGN-bounded-atomic/1", "MORIARTY-OUTCOME-bounded-atomic/1"],
        },
        "preimageCatalog": r.catalog,
        "preimageRegistry": r.bodies,
        "innerEdges": inner_edges,
        "valid": valid,
        "invalid": invalid,
        "schemaInexpressible": [
            "cap sums versus movements",
            "unique (network,deployment,principal,nonce) currentness",
            "relational validity versus obs.now",
            "allowedEffectSet subset across successors",
            "cumulative fill versus cap",
            "conservation equalities",
            "UInt64/UInt128/SInt128 inclusive maxima (DomainValidation, pre-hash)",
            "ledger unique consumption",
            "non-overlapping accrual periods",
        ],
        "structuralChecks": [
            "Draft 2020-12 types, required keys, additionalProperties false",
            "true-end patterns",
            "required-claims contains-exactly-once",
            "genesis predecessors empty; non-genesis ExactPlan predecessors minItems 1",
            "swap/exchange require dust, rateOrPrice, outPredicate, remainder; others forbid them",
        ],
        "domainChecks": [
            "UInt64/UInt128/SInt128 inclusive maxima",
            "Scale 0..18",
            "Quantity unit vector length and unique symbols",
            "canonical re-encode byte identity",
        ],
        "contextualChecks": [
            "authorization network/deployment/revocation/validity vs obs.now",
            "net-after-fees including actual Fee effects",
            "clone residual work",
            "cancellation vs winning fill currentness",
            "migration replay",
            "display completeness",
            "specified-only until an evaluator exists",
        ],
        "accrualToy": {
            "principal": PRINCIPAL,
            "rateMantissa": RATE_MANTISSA,
            "rateScale": RATE_SCALE,
            "periodSeconds": PERIOD,
            "secondsPerYear": SECONDS_PER_YEAR,
            "rawFloor": ACCRUED,
            "liabilityCap": LIABILITY_CAP,
            "rejectedWrongDelta": 533972602,
        },
        "nativeCompatibility": "unresolved",
    }
    return examples


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--output", required=True)
    args = parser.parse_args()
    out = Path(args.output)
    schema = json.loads(SCHEMA_PATH.read_text(encoding="utf-8"))
    examples = build(schema)
    text = json.dumps(examples, sort_keys=True, separators=(",", ":"), ensure_ascii=False) + "\n"
    out.write_text(text, encoding="utf-8")
    print(f"wrote {out} bytes={len(text.encode('utf-8'))}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
