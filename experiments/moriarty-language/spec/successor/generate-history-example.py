#!/usr/bin/env python3
"""Deterministic funded Genesis -> Exchange history builder.

Reuses canonical/hash helpers. Does not call the old full build(). Construction
is not semantic truth and does not consult validate_history.
"""
from __future__ import annotations

import argparse
import importlib.util
import json
import sys
from copy import deepcopy
from pathlib import Path

HERE = Path(__file__).resolve().parent


def _load_signing_history():
    path = HERE / "signing-history.py"
    spec = importlib.util.spec_from_file_location("signing_history", path)
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


_sh = _load_signing_history()
DOMAIN_TAGS = _sh.DOMAIN_TAGS
SCHEMA_PATH = _sh.SCHEMA_PATH
canonical = _sh.canonical
expr_charges = _sh.expr_charges
expr_count = _sh.expr_count
framed_hash = _sh.framed_hash
raw_hash = _sh.raw_hash
successor_id = _sh.successor_id

START = "1690000000"
NOW = "1700000000"
ZERO = "0" * 64
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


def u(n) -> str:
    return str(int(n))


def amount(asset, value):
    return {"tag": "Amount", "asset": asset, "value": u(value)}


def stored_bool(v):
    return {"tag": "Bool", "value": bool(v)}


def stored_u(value):
    return {"tag": "UInt128", "value": u(value)}


def stored_text(value):
    return {"tag": "Text", "value": value}


def field(selector, field_type, value, projection):
    return {"selector": selector, "fieldType": field_type, "value": value, "projection": projection}


def bal(party, asset, value):
    return {"party": party, "asset": asset, "value": u(value)}


def declared(selector, field_type, projection):
    return {"selector": selector, "fieldType": field_type, "projection": projection}


def let(binder, expr):
    return {"tag": "Let", "binder": binder, "expr": expr}


def next_write_local(selector):
    return {"tag": "NextWrite", "selector": selector, "value": {"tag": "ReadLocal", "binder": selector}}


GENESIS_EXPRS = [
    {"tag": "Require", "cond": {"tag": "LitBool", "value": True}},
    next_write_local("trader_a"),
    next_write_local("pool_a"),
    next_write_local("pool_b"),
    next_write_local("trader_b"),
    {"tag": "NextWrite", "selector": "alive", "value": {"tag": "LitBool", "value": True}},
]
EXCHANGE_EXPRS = [
    let("in_amt", {"tag": "ReadArg", "name": "amount_in"}),
    let("min_amt", {"tag": "ReadArg", "name": "min_out"}),
    let("px", {"tag": "ReadArg", "name": "price"}),
    let(
        "in_val",
        {"tag": "AccessField", "record": {"tag": "ReadLocal", "binder": "in_amt"}, "field": "value"},
    ),
    let(
        "mant",
        {"tag": "AccessField", "record": {"tag": "ReadLocal", "binder": "px"}, "field": "mantissa"},
    ),
    let("den", {"tag": "AccessField", "record": {"tag": "ReadArg", "name": "scale_denominator"}, "field": "value"}),
    let(
        "prod",
        {
            "tag": "Mul",
            "left": {"tag": "ReadLocal", "binder": "in_val"},
            "right": {"tag": "ReadLocal", "binder": "mant"},
        },
    ),
    let(
        "out_val",
        {
            "tag": "FloorDiv",
            "left": {"tag": "ReadLocal", "binder": "prod"},
            "right": {"tag": "ReadLocal", "binder": "den"},
        },
    ),
    {
        "tag": "Require",
        "cond": {
            "tag": "Gt",
            "left": {"tag": "ReadLocal", "binder": "in_val"},
            "right": {"tag": "LitUInt", "value": "0"},
        },
    },
]


def work_of(life_o, life_r, rem_o, rem_r):
    return {
        "lifetimeOrdinary": u(life_o),
        "lifetimeRecovery": u(life_r),
        "ordinaryRemaining": u(rem_o),
        "recoveryRemaining": u(rem_r),
    }


def cost_certificate(op, forest):
    charges = expr_charges(forest)
    return {
        "coreOps": [{"op": op, "ordinary": "1", "recovery": "0"}],
        "expressions": charges,
        "sidecarBytes": "0",
        "sidecarCharge": "0",
        "totalOrdinary": u(1 + len(charges)),
        "totalRecovery": "0",
    }


def residual(controller, ops, assets, gross, fees, nets, recipients, partial):
    return {
        "controller": controller,
        "allowedEffectSet": ops,
        "allowedAssets": assets,
        "grossDebitCaps": gross,
        "feeCaps": fees,
        "netGoals": nets,
        "debtCaps": [],
        "permittedRecipients": recipients,
        "permittedCalls": [],
        "validity": {"notBefore": START, "notAfterExclusive": "1800000000"},
        "partialRule": deepcopy(partial),
        "continuation": "principal",
    }


def actor_ledger(actor, asset, gcap, fcap, goal, cg, cf, cin):
    signed_net = int(cin) - int(cf)
    return {
        "actor": actor,
        "asset": asset,
        "originalGrossCap": u(gcap),
        "originalFeeCap": u(fcap),
        "originalNetGoal": u(goal),
        "cumulativeGross": u(cg),
        "cumulativeFees": u(cf),
        "cumulativeInflow": u(cin),
        "cumulativeNet": str(signed_net),
        "remainingGross": u(int(gcap) - int(cg)),
        "remainingFee": u(int(fcap) - int(cf)),
    }


def successor_record(tag, origin, consumption, partial, recipients, assumptions_hash, ledgers, residual_cap, work, mode, recovery, original_work):
    return {
        "tag": tag,
        "successorId": successor_id(origin, consumption, 0),
        "originalIntentDigest": origin,
        "consumptionId": consumption,
        "partialRule": deepcopy(partial),
        "originalValidity": {"notBefore": START, "notAfterExclusive": "1800000000"},
        "originalRecipients": deepcopy(recipients),
        "originalCalls": [],
        "originalAssumptionsHash": assumptions_hash,
        "originalWork": deepcopy(original_work),
        "actorLedgers": ledgers,
        "debtLedgers": [],
        "residualCapability": residual_cap,
        "remainingWork": deepcopy(work),
        "predecessorMode": mode,
        "recoveryRights": deepcopy(recovery),
    }


def envelope_exact(nonce, principal, residual_auth, before, predecessors, exec_body, exec_hash, genesis_hash, profile_hash, program_hash, claim_root, policy_hash, assumptions, remaining_work, instance_id, recovery):
    return {
        "kind": "ExactPlan",
        "signingDomain": "MORIARTY-SUCC-EXACT-PLAN/0",
        "schemaVersion": "moriarty-succ-sign/0",
        "network": "midnight_preview",
        "deployment": "preview_deployment_1",
        "principal": principal,
        "nonce": nonce,
        "validity": {"notBefore": START, "notAfterExclusive": "1800000000"},
        "spec": {
            "semanticVersion": "moriarty-succ-core/0",
            "profileHash": profile_hash,
            "programHash": program_hash,
            "claimRoot": claim_root,
            "activationNotBefore": START,
            "revoked": False,
        },
        "policy": {
            "policyId": "fixed_rational_v0",
            "policyHash": policy_hash,
            "composition": deepcopy(COMP),
        },
        "profileBinding": {"profileName": "moriarty-successor/0", "profileHash": profile_hash},
        "instanceId": instance_id,
        "genesisHash": genesis_hash,
        "requiredClaims": deepcopy(CLAIMS),
        "requiredClaimRoot": claim_root,
        "observationPolicy": {
            "requiredFields": ["now"],
            "maxStalenessSeconds": "3600",
            "requiredRole": "time_oracle",
        },
        "residualAuthority": residual_auth,
        "remainingWork": deepcopy(remaining_work),
        "recoveryRights": deepcopy(recovery),
        "locks": [],
        "assumptions": deepcopy(assumptions),
        "beforeStateHash": before,
        "predecessors": predecessors,
        "executionBody": exec_body,
        "executionBodyHash": exec_hash,
    }


def wrapper(digest, principal, nonce):
    return {
        "tag": "AuthorityWrapper",
        "intentDigest": digest,
        "signingDomain": "MORIARTY-SUCC-EXACT-PLAN/0",
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


def acceptance(digest, plan_hash, obs_hash, effects, work, pred_ids, succ, prepared_hash):
    return {
        "intentDigest": digest,
        "selectedPlanHash": plan_hash,
        "observationsHash": obs_hash,
        "effects": deepcopy(effects),
        "liabilities": [],
        "remainingWork": deepcopy(work),
        "predecessorIds": pred_ids,
        "successorRecord": deepcopy(succ),
        "preparedHash": prepared_hash,
    }


def state_body(instance_id, event_order, fields, balances, residual_cap, work, successor_hash):
    return {
        "instanceId": instance_id,
        "eventOrder": u(event_order),
        "fields": fields,
        "balances": balances,
        "debts": [],
        "shares": [],
        "positions": [],
        "requests": [],
        "messages": [],
        "eventClaims": [],
        "rewardAccounts": [],
        "locks": [],
        "duties": [],
        "status": {"episode": "Live", "agreement": "Active"},
        "residualCapability": residual_cap,
        "remainingWork": deepcopy(work),
        "successorRecordHash": successor_hash,
    }


def grant(grant_id, principal, asset, cap, operation, recipient):
    return {
        "grantId": grant_id,
        "principal": principal,
        "asset": asset,
        "cap": u(cap),
        "operation": operation,
        "recipient": recipient,
        "context": {"network": "midnight_preview", "deployment": "preview_deployment_1"},
    }


def caller_grant(row, revoked=False):
    out = deepcopy(row)
    out["revoked"] = bool(revoked)
    return out


def genesis_action_body():
    return {
        "name": "genesis",
        "coreOp": "Genesis",
        "parameters": [
            {"name": "instanceId", "valueType": "Identifier"},
            {"name": "initialWork", "valueType": "Work"},
        ],
        "operations": [
            {
                "op": "Genesis",
                "operands": [
                    {"name": "instanceId", "binding": "instanceId"},
                    {"name": "initialWork", "binding": "initialWork"},
                ],
            }
        ],
        "expressions": deepcopy(GENESIS_EXPRS),
    }


def swap_action_body():
    return {
        "name": "swap",
        "coreOp": "Exchange",
        "parameters": [
            {"name": "amount_in", "valueType": "Amount"},
            {"name": "min_out", "valueType": "Amount"},
            {"name": "price", "valueType": "Price"},
            {"name": "dust", "valueType": "Dust"},
            {"name": "pool", "valueType": "Text"},
            {"name": "recipient", "valueType": "Text"},
            {"name": "scale_denominator", "valueType": "UInt128"},
        ],
        "operations": [
            {
                "op": "Exchange",
                "operands": [
                    {"name": "in", "binding": "amount_in"},
                    {"name": "minOut", "binding": "min_out"},
                    {"name": "price", "binding": "price"},
                    {"name": "dust", "binding": "dust"},
                ],
            }
        ],
        "expressions": deepcopy(EXCHANGE_EXPRS),
    }


def link_hashes(b):
    program_h = None
    b["programBody"]["sourceHash"] = raw_hash(b["sourceUtf8"])
    ph = framed_hash(DOMAIN_TAGS["ProfileBody"], b["profileBody"])
    prog_h = framed_hash(DOMAIN_TAGS["ProgramBody"], b["programBody"])
    claim_h = framed_hash(DOMAIN_TAGS["ClaimArray"], b["requiredClaims"])
    pol_h = framed_hash(DOMAIN_TAGS["PolicyBody"], b["policyBody"])
    assum_h = framed_hash(DOMAIN_TAGS["AssumptionSet"], b["assumptions"])
    empty_h = framed_hash(DOMAIN_TAGS["SuccessorRecord"], b["emptySuccessor"])
    b["unbornStateBody"]["successorRecordHash"] = empty_h
    unborn_h = framed_hash(DOMAIN_TAGS["StateBody"], b["unbornStateBody"])
    b["genesisBody"]["profileHash"] = ph
    b["genesisBody"]["programHash"] = prog_h
    b["genesisBody"]["claimRoot"] = claim_h
    b["genesisBody"]["initialStateHash"] = unborn_h
    gen_h = framed_hash(DOMAIN_TAGS["GenesisBody"], b["genesisBody"])
    obs_h = framed_hash(DOMAIN_TAGS["ObservationSet"], b["observationSet"])
    cons_g_h = framed_hash(DOMAIN_TAGS["ConsumptionIdBody"], b["consumptionGenesis"])
    cons_e_h = framed_hash(DOMAIN_TAGS["ConsumptionIdBody"], b["consumptionExchange"])
    exec_g_h = framed_hash(DOMAIN_TAGS["ExecutionBody"], b["genesisExecutionBody"])
    b["effectsGenesis"] = {"effects": deepcopy(b["genesisExecutionBody"]["exactEffects"])}
    plan = b["genesisPlan"]
    plan["spec"]["profileHash"] = ph
    plan["spec"]["programHash"] = prog_h
    plan["spec"]["claimRoot"] = claim_h
    plan["policy"]["policyHash"] = pol_h
    plan["profileBinding"]["profileHash"] = ph
    plan["genesisHash"] = gen_h
    plan["requiredClaimRoot"] = claim_h
    plan["beforeStateHash"] = unborn_h
    plan["executionBody"] = b["genesisExecutionBody"]
    plan["executionBodyHash"] = exec_g_h
    plan_g_h = framed_hash(DOMAIN_TAGS["ExactPlanDocument"], plan)
    b["authorityWrapperGenesis"]["intentDigest"] = plan_g_h
    b["genesisSuccessor"]["originalIntentDigest"] = plan_g_h
    b["genesisSuccessor"]["consumptionId"] = cons_g_h
    b["genesisSuccessor"]["originalAssumptionsHash"] = assum_h
    b["genesisSuccessor"]["successorId"] = successor_id(plan_g_h, cons_g_h, 0)
    b["genesisSuccessor"]["residualCapability"] = b["residualGenesis"]
    succ_g_h = framed_hash(DOMAIN_TAGS["SuccessorRecord"], b["genesisSuccessor"])
    b["genesisStateBody"]["successorRecordHash"] = succ_g_h
    b["genesisStateBody"]["residualCapability"] = b["residualGenesis"]
    state_g_h = framed_hash(DOMAIN_TAGS["StateBody"], b["genesisStateBody"])
    b["preparedGenesis"]["postStateHash"] = state_g_h
    b["preparedGenesis"]["effects"] = deepcopy(b["genesisExecutionBody"]["exactEffects"])
    b["preparedGenesis"]["residualCapability"] = b["residualGenesis"]
    b["preparedGenesis"]["successors"] = [deepcopy(b["genesisSuccessor"])]
    prep_g_h = framed_hash(DOMAIN_TAGS["PreparedBody"], b["preparedGenesis"])
    b["acceptanceGenesis"]["intentDigest"] = plan_g_h
    b["acceptanceGenesis"]["selectedPlanHash"] = exec_g_h
    b["acceptanceGenesis"]["observationsHash"] = obs_h
    b["acceptanceGenesis"]["effects"] = deepcopy(b["genesisExecutionBody"]["exactEffects"])
    b["acceptanceGenesis"]["successorRecord"] = deepcopy(b["genesisSuccessor"])
    b["acceptanceGenesis"]["preparedHash"] = prep_g_h
    exec_e_h = framed_hash(DOMAIN_TAGS["ExecutionBody"], b["exchangeExecutionBody"])
    b["effectsExchange"] = {"effects": deepcopy(b["exchangeExecutionBody"]["exactEffects"])}
    eplan = b["exchangePlan"]
    eplan["spec"]["profileHash"] = ph
    eplan["spec"]["programHash"] = prog_h
    eplan["spec"]["claimRoot"] = claim_h
    eplan["policy"]["policyHash"] = pol_h
    eplan["profileBinding"]["profileHash"] = ph
    eplan["genesisHash"] = gen_h
    eplan["requiredClaimRoot"] = claim_h
    eplan["beforeStateHash"] = state_g_h
    eplan["executionBody"] = b["exchangeExecutionBody"]
    eplan["executionBodyHash"] = exec_e_h
    eplan["predecessors"] = [
        {
            "stateHash": state_g_h,
            "instanceId": plan["instanceId"],
            "consumptionId": cons_g_h,
        }
    ]
    plan_e_h = framed_hash(DOMAIN_TAGS["ExactPlanDocument"], eplan)
    b["authorityWrapperExchange"]["intentDigest"] = plan_e_h
    b["exchangeSuccessor"]["originalIntentDigest"] = b["genesisSuccessor"]["originalIntentDigest"]
    b["exchangeSuccessor"]["consumptionId"] = cons_e_h
    b["exchangeSuccessor"]["originalAssumptionsHash"] = assum_h
    b["exchangeSuccessor"]["successorId"] = successor_id(b["genesisSuccessor"]["originalIntentDigest"], cons_e_h, 0)
    b["exchangeSuccessor"]["residualCapability"] = b["residualExchange"]
    succ_e_h = framed_hash(DOMAIN_TAGS["SuccessorRecord"], b["exchangeSuccessor"])
    b["exchangeStateBody"]["successorRecordHash"] = succ_e_h
    b["exchangeStateBody"]["residualCapability"] = b["residualExchange"]
    state_e_h = framed_hash(DOMAIN_TAGS["StateBody"], b["exchangeStateBody"])
    b["preparedExchange"]["postStateHash"] = state_e_h
    b["preparedExchange"]["effects"] = deepcopy(b["exchangeExecutionBody"]["exactEffects"])
    b["preparedExchange"]["residualCapability"] = b["residualExchange"]
    b["preparedExchange"]["successors"] = [deepcopy(b["exchangeSuccessor"])]
    prep_e_h = framed_hash(DOMAIN_TAGS["PreparedBody"], b["preparedExchange"])
    b["acceptanceExchange"]["intentDigest"] = plan_e_h
    b["acceptanceExchange"]["selectedPlanHash"] = exec_e_h
    b["acceptanceExchange"]["observationsHash"] = obs_h
    b["acceptanceExchange"]["effects"] = deepcopy(b["exchangeExecutionBody"]["exactEffects"])
    b["acceptanceExchange"]["predecessorIds"] = [cons_g_h]
    b["acceptanceExchange"]["successorRecord"] = deepcopy(b["exchangeSuccessor"])
    b["acceptanceExchange"]["preparedHash"] = prep_e_h
    residual_e_h = framed_hash(DOMAIN_TAGS["ResidualCapability"], b["residualExchange"])
    effects_e_h = framed_hash(DOMAIN_TAGS["EffectsBody"], b["effectsExchange"])
    wrap_e_h = framed_hash(DOMAIN_TAGS["AuthorityWrapper"], b["authorityWrapperExchange"])
    b["proofContextExchange"] = {
        "programHash": prog_h,
        "specVersion": "moriarty-succ-core/0",
        "policyHash": pol_h,
        "authorizationDigest": wrap_e_h,
        "selectedPlanHash": exec_e_h,
        "predecessorIds": [cons_g_h],
        "observationsHash": obs_h,
        "postStateHash": state_e_h,
        "effectsHash": effects_e_h,
        "residualCapabilityHash": residual_e_h,
        "remainingWork": deepcopy(b["exchangeStateBody"]["remainingWork"]),
        "claims": deepcopy(CLAIMS),
    }
    b["_hashes"] = {
        "profileBody": ph,
        "programBody": prog_h,
        "requiredClaims": claim_h,
        "policyBody": pol_h,
        "assumptions": assum_h,
        "emptySuccessor": empty_h,
        "unbornStateBody": unborn_h,
        "genesisBody": gen_h,
        "observationSet": obs_h,
        "consumptionGenesis": cons_g_h,
        "consumptionExchange": cons_e_h,
        "genesisExecutionBody": exec_g_h,
        "genesisPlan": plan_g_h,
        "genesisSuccessor": succ_g_h,
        "genesisStateBody": state_g_h,
        "preparedGenesis": prep_g_h,
        "exchangeExecutionBody": exec_e_h,
        "exchangePlan": plan_e_h,
        "exchangeSuccessor": succ_e_h,
        "exchangeStateBody": state_e_h,
        "preparedExchange": prep_e_h,
        "authorityWrapperExchange": wrap_e_h,
    }
    return b


def preimage_entry(name, closed, body):
    text = body if closed == "SourceUtf8" else canonical(body)
    return {
        "name": name,
        "closedType": closed,
        "domain": DOMAIN_TAGS[closed],
        "canonicalUtf8": text,
    }


def assemble_bundle(b, ctx):
    docs = [
        {
            "name": "genesisPlan",
            "closedType": "ExactPlanDocument",
            "domain": DOMAIN_TAGS["ExactPlanDocument"],
            "canonicalUtf8": canonical(b["genesisPlan"]),
        },
        {
            "name": "exchangePlan",
            "closedType": "ExactPlanDocument",
            "domain": DOMAIN_TAGS["ExactPlanDocument"],
            "canonicalUtf8": canonical(b["exchangePlan"]),
        },
    ]
    order = [
        ("sourceUtf8", "SourceUtf8"),
        ("profileBody", "ProfileBody"),
        ("programBody", "ProgramBody"),
        ("requiredClaims", "ClaimArray"),
        ("policyBody", "PolicyBody"),
        ("genesisBody", "GenesisBody"),
        ("unbornStateBody", "StateBody"),
        ("genesisStateBody", "StateBody"),
        ("exchangeStateBody", "StateBody"),
        ("emptySuccessor", "SuccessorRecord"),
        ("genesisSuccessor", "SuccessorRecord"),
        ("exchangeSuccessor", "SuccessorRecord"),
        ("genesisExecutionBody", "ExecutionBody"),
        ("exchangeExecutionBody", "ExecutionBody"),
        ("assumptions", "AssumptionSet"),
        ("effectsGenesis", "EffectsBody"),
        ("effectsExchange", "EffectsBody"),
        ("residualGenesis", "ResidualCapability"),
        ("residualExchange", "ResidualCapability"),
        ("observationSet", "ObservationSet"),
        ("consumptionGenesis", "ConsumptionIdBody"),
        ("consumptionExchange", "ConsumptionIdBody"),
        ("authorityWrapperGenesis", "AuthorityWrapper"),
        ("authorityWrapperExchange", "AuthorityWrapper"),
        ("preparedGenesis", "PreparedBody"),
        ("preparedExchange", "PreparedBody"),
        ("acceptanceGenesis", "AcceptanceBody"),
        ("acceptanceExchange", "AcceptanceBody"),
        ("proofContextExchange", "ProofContext"),
    ]
    return {
        "tag": "SigningHistory",
        "schemaVersion": "moriarty-succ-sign/0",
        "documents": docs,
        "preimages": [preimage_entry(name, closed, b[name]) for name, closed in order],
        "wrappers": [deepcopy(b["authorityWrapperGenesis"]), deepcopy(b["authorityWrapperExchange"])],
        "callerContext": ctx,
    }


def build_history(
    amount_in=10000,
    min_out=None,
    predicate="Min",
    trader="trader",
    pool="pool",
    reverse_swap_args=False,
    reverse_sources=False,
    expected_sources="match",
    exchange_predecessor="genesisStateBody",
    mutate_bodies=None,
    mantissa=19743,
    scale=4,
    payout_cap=19743,
):
    instance_id = "funded_swap_demo"
    den = 10 ** scale
    prod = int(amount_in) * int(mantissa)
    out_q = prod // den
    rem_num = prod % den
    if min_out is None:
        min_out = out_q
    g_charge = 1 + expr_count(GENESIS_EXPRS)
    e_charge = 1 + expr_count(EXCHANGE_EXPRS)
    life_o = g_charge + e_charge
    life_r = 2
    work_life = work_of(life_o, life_r, life_o, life_r)
    work_g_post = work_of(life_o, life_r, life_o - g_charge, life_r)
    work_e_post = work_of(life_o, life_r, life_o - g_charge - e_charge, life_r)
    recovery = {
        "controller": trader,
        "allowedEffectSet": ["RequestCancel", "Migrate"],
        "alias": {"tag": "AliasReserve", "which": "recovery"},
    }
    recipients = [{"party": pool, "role": "pool"}, {"party": trader, "role": "principal"}]
    partial = {
        "tag": "Partial",
        "minFill": amount("AssetA", 1),
        "cumulativeCap": amount("AssetA", amount_in),
    }
    source = f"agreement {instance_id}\naction genesis\naction swap\n"
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
    program = {
        "sourceHash": raw_hash(source),
        "entryActions": ["genesis", "swap"],
        "coreVersion": "moriarty-succ-core/0",
        "actionBodies": [genesis_action_body(), swap_action_body()],
    }
    policy = {"policyId": "fixed_rational_v0", "rules": ["seq_genesis_swap"], "composition": deepcopy(COMP)}
    assumption_list = [
        {
            "assumptionId": "time_oracle_unproven",
            "kind": "oracle_truth",
            "statement": "obs.now is authenticated not proven true time",
            "evidenceRole": "required_observation",
        }
    ]
    assumptions = {"assumptions": assumption_list}
    ops = ["Genesis", "Exchange", "Transfer"]
    assets = ["AssetA", "AssetB"]
    residual_unborn = residual(
        trader,
        ops,
        assets,
        [{"actor": trader, "asset": "AssetA", "maximum": "0"}],
        [{"actor": trader, "asset": "AssetB", "maximum": "0"}],
        [{"actor": trader, "asset": "AssetB", "minimumAfterFees": "0"}],
        recipients,
        {
            "tag": "FillOrKill",
            "minFill": amount("AssetA", "0"),
            "cumulativeCap": amount("AssetA", "0"),
        },
    )
    residual_live = residual(
        trader,
        ops,
        assets,
        [
            {"actor": trader, "asset": "AssetA", "maximum": u(amount_in)},
            {"actor": pool, "asset": "AssetB", "maximum": u(payout_cap)},
        ],
        [
            {"actor": trader, "asset": "AssetB", "maximum": "0"},
            {"actor": pool, "asset": "AssetB", "maximum": "0"},
        ],
        [{"actor": trader, "asset": "AssetB", "minimumAfterFees": u(min_out)}],
        recipients,
        partial,
    )
    residual_post = deepcopy(residual_live)
    residual_post["grossDebitCaps"] = [
        {"actor": trader, "asset": "AssetA", "maximum": "0"},
        {"actor": pool, "asset": "AssetB", "maximum": u(int(payout_cap) - out_q if int(payout_cap) >= out_q else 0)},
    ]
    empty = successor_record(
        "GenesisInitial",
        ZERO,
        ZERO,
        residual_unborn["partialRule"],
        [],
        ZERO,
        [],
        residual_unborn,
        work_life,
        "GenesisNone",
        recovery,
        work_life,
    )
    unborn = state_body(instance_id, 0, [], [], residual_unborn, work_life, ZERO)
    swap_declared = [
        declared("trader_a", "Amount", {"tag": "Balance", "party": trader, "asset": "AssetA"}),
        declared("pool_a", "Amount", {"tag": "Balance", "party": pool, "asset": "AssetA"}),
        declared("pool_b", "Amount", {"tag": "Balance", "party": pool, "asset": "AssetB"}),
        declared("trader_b", "Amount", {"tag": "Balance", "party": trader, "asset": "AssetB"}),
        declared("alive", "Bool", {"tag": "Flag"}),
    ]
    sources = [
        {"sourceId": "srcTraderA", "party": trader, "asset": "AssetA", "balance": "100000"},
        {"sourceId": "srcPoolA", "party": pool, "asset": "AssetA", "balance": "1000000"},
        {"sourceId": "srcPoolB", "party": pool, "asset": "AssetB", "balance": "2000000"},
        {"sourceId": "srcTraderB", "party": trader, "asset": "AssetB", "balance": "0"},
    ]
    allocations = [
        {"sourceId": "srcTraderA", "selector": "trader_a", "amount": "100000"},
        {"sourceId": "srcPoolA", "selector": "pool_a", "amount": "1000000"},
        {"sourceId": "srcPoolB", "selector": "pool_b", "amount": "2000000"},
        {"sourceId": "srcTraderB", "selector": "trader_b", "amount": "0"},
    ]
    if reverse_sources:
        sources = list(reversed(sources))
    signed_grants = [
        grant("grantFundTraderA", trader, "AssetA", 100000, "Genesis", trader),
        grant("grantFundTraderB", trader, "AssetB", 0, "Genesis", trader),
        grant("grantFundPoolA", pool, "AssetA", 1000000, "Genesis", pool),
        grant("grantFundPoolB", pool, "AssetB", 2000000, "Genesis", pool),
        grant("grantPoolPayout", pool, "AssetB", payout_cap, "Exchange", trader),
    ]
    genesis_body = {
        "instanceId": instance_id,
        "profileHash": ZERO,
        "programHash": ZERO,
        "claimRoot": ZERO,
        "lifetime": "31536000",
        "horizon": "31536000",
        "ordinaryWork": u(life_o),
        "recoveryWork": u(life_r),
        "initialStateHash": ZERO,
        "network": "midnight_preview",
        "deployment": "preview_deployment_1",
        "principalBindings": [trader, pool],
        "observationBindings": ["now"],
        "declaredFields": swap_declared,
        "initialization": {
            "externalSources": sources,
            "allocations": allocations,
            "debitAuthorities": signed_grants,
        },
    }
    gen_fields = [
        field("trader_a", "Amount", amount("AssetA", 100000), {"tag": "Balance", "party": trader, "asset": "AssetA"}),
        field("pool_a", "Amount", amount("AssetA", 1000000), {"tag": "Balance", "party": pool, "asset": "AssetA"}),
        field("pool_b", "Amount", amount("AssetB", 2000000), {"tag": "Balance", "party": pool, "asset": "AssetB"}),
        field("trader_b", "Amount", amount("AssetB", 0), {"tag": "Balance", "party": trader, "asset": "AssetB"}),
        field("alive", "Bool", stored_bool(True), {"tag": "Flag"}),
    ]
    gen_bals = [
        bal(trader, "AssetA", 100000),
        bal(pool, "AssetA", 1000000),
        bal(pool, "AssetB", 2000000),
        bal(trader, "AssetB", 0),
    ]
    gen_exec = {
        "action": {"name": "genesis", "actor": trader, "arguments": []},
        "exactWrites": [
            {"field": "trader_a", "value": amount("AssetA", 100000)},
            {"field": "pool_a", "value": amount("AssetA", 1000000)},
            {"field": "pool_b", "value": amount("AssetB", 2000000)},
            {"field": "trader_b", "value": amount("AssetB", 0)},
            {"field": "alive", "value": stored_bool(True)},
        ],
        "exactEffects": [
            {
                "kind": "Genesis",
                "ordinal": "0",
                "instanceId": instance_id,
                "initialWork": deepcopy(work_life),
                "claimRoot": ZERO,
            }
        ],
        "costCertificate": cost_certificate("Genesis", GENESIS_EXPRS),
    }
    price = {
        "tag": "Price",
        "base": "AssetB",
        "quote": "AssetA",
        "mantissa": u(mantissa),
        "scale": u(scale),
        "direction": "basePerQuote",
    }
    arguments = [
        {"name": "amount_in", "value": amount("AssetA", amount_in)},
        {"name": "min_out", "value": amount("AssetB", min_out)},
        {"name": "price", "value": deepcopy(price)},
        {"name": "scale_denominator", "value": stored_u(den)},
        {"name": "pool", "value": stored_text(pool)},
        {"name": "recipient", "value": stored_text(trader)},
    ]
    if reverse_swap_args:
        arguments = list(reversed(arguments))
    swap_effects = [
        {"kind": "Transfer", "ordinal": "0", "asset": "AssetA", "from": trader, "to": pool, "amount": u(amount_in)},
        {"kind": "Transfer", "ordinal": "1", "asset": "AssetB", "from": pool, "to": trader, "amount": u(out_q)},
    ]
    swap_exec = {
        "action": {"name": "swap", "actor": trader, "arguments": arguments},
        "exactWrites": [
            {"field": "pool_a", "value": amount("AssetA", 1000000 + amount_in)},
            {"field": "pool_b", "value": amount("AssetB", 2000000 - out_q)},
            {"field": "trader_a", "value": amount("AssetA", 100000 - amount_in)},
            {"field": "trader_b", "value": amount("AssetB", out_q)},
        ],
        "costCertificate": cost_certificate("Exchange", EXCHANGE_EXPRS),
        "exactEffects": swap_effects,
        "dust": {"asset": "AssetB", "value": "1"},
        "rateOrPrice": deepcopy(price),
        "outPredicate": {"tag": predicate, "amount": amount("AssetB", min_out)},
        "remainder": {
            "tag": "Remainder",
            "numerator": u(rem_num),
            "denominator": u(den),
            "unit": "AssetB",
            "location": {"tag": "SourceReserve"},
        },
    }
    swap_fields = [
        field("trader_a", "Amount", amount("AssetA", 100000 - amount_in), {"tag": "Balance", "party": trader, "asset": "AssetA"}),
        field("pool_a", "Amount", amount("AssetA", 1000000 + amount_in), {"tag": "Balance", "party": pool, "asset": "AssetA"}),
        field("pool_b", "Amount", amount("AssetB", 2000000 - out_q), {"tag": "Balance", "party": pool, "asset": "AssetB"}),
        field("trader_b", "Amount", amount("AssetB", out_q), {"tag": "Balance", "party": trader, "asset": "AssetB"}),
        field("alive", "Bool", stored_bool(True), {"tag": "Flag"}),
    ]
    swap_bals = [
        bal(trader, "AssetA", 100000 - amount_in),
        bal(pool, "AssetA", 1000000 + amount_in),
        bal(pool, "AssetB", 2000000 - out_q),
        bal(trader, "AssetB", out_q),
    ]
    ledgers = [
        actor_ledger(trader, "AssetA", amount_in, 0, 0, amount_in, 0, 0),
        actor_ledger(trader, "AssetB", 0, 0, min_out, 0, 0, out_q),
        actor_ledger(pool, "AssetA", 0, 0, 0, 0, 0, amount_in),
        actor_ledger(pool, "AssetB", payout_cap, 0, 0, out_q, 0, 0),
    ]
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
    cons_g = {
        "network": "midnight_preview",
        "deployment": "preview_deployment_1",
        "principal": trader,
        "nonce": "NonceGenesis01",
    }
    cons_e = {
        "network": "midnight_preview",
        "deployment": "preview_deployment_1",
        "principal": trader,
        "nonce": "NonceSwap01",
    }
    genesis_plan = envelope_exact(
        "NonceGenesis01",
        trader,
        residual_live,
        ZERO,
        [],
        gen_exec,
        ZERO,
        ZERO,
        ZERO,
        ZERO,
        ZERO,
        ZERO,
        assumption_list,
        work_life,
        instance_id,
        recovery,
    )
    exchange_plan = envelope_exact(
        "NonceSwap01",
        trader,
        residual_live,
        ZERO,
        [{"stateHash": ZERO, "instanceId": instance_id, "consumptionId": ZERO}],
        swap_exec,
        ZERO,
        ZERO,
        ZERO,
        ZERO,
        ZERO,
        ZERO,
        assumption_list,
        work_g_post,
        instance_id,
        recovery,
    )
    b = {
        "sourceUtf8": source,
        "profileBody": profile,
        "programBody": program,
        "requiredClaims": deepcopy(CLAIMS),
        "policyBody": policy,
        "assumptions": assumptions,
        "emptySuccessor": empty,
        "unbornStateBody": unborn,
        "genesisBody": genesis_body,
        "observationSet": obs,
        "consumptionGenesis": cons_g,
        "consumptionExchange": cons_e,
        "genesisExecutionBody": gen_exec,
        "effectsGenesis": {"effects": gen_exec["exactEffects"]},
        "residualGenesis": residual_live,
        "genesisSuccessor": successor_record(
            "ActiveSuccessor",
            ZERO,
            ZERO,
            partial,
            recipients,
            ZERO,
            [],
            residual_live,
            work_g_post,
            "GenesisNone",
            recovery,
            work_life,
        ),
        "genesisStateBody": state_body(instance_id, 1, gen_fields, gen_bals, residual_live, work_g_post, ZERO),
        "preparedGenesis": prepared(ZERO, gen_exec["exactEffects"], residual_live, empty, work_g_post),
        "authorityWrapperGenesis": wrapper(ZERO, trader, "NonceGenesis01"),
        "acceptanceGenesis": acceptance(ZERO, ZERO, ZERO, gen_exec["exactEffects"], work_g_post, [], empty, ZERO),
        "exchangeExecutionBody": swap_exec,
        "effectsExchange": {"effects": swap_effects},
        "residualExchange": residual_post,
        "exchangeSuccessor": successor_record(
            "ActiveSuccessor",
            ZERO,
            ZERO,
            partial,
            recipients,
            ZERO,
            ledgers,
            residual_post,
            work_e_post,
            "Pinned",
            recovery,
            work_life,
        ),
        "exchangeStateBody": state_body(instance_id, 2, swap_fields, swap_bals, residual_post, work_e_post, ZERO),
        "preparedExchange": prepared(ZERO, swap_effects, residual_post, empty, work_e_post),
        "authorityWrapperExchange": wrapper(ZERO, trader, "NonceSwap01"),
        "acceptanceExchange": acceptance(ZERO, ZERO, ZERO, swap_effects, work_e_post, [ZERO], empty, ZERO),
        "genesisPlan": genesis_plan,
        "exchangePlan": exchange_plan,
        "proofContextExchange": {},
    }
    # fill genesis effect claimRoot after claims exist; link_hashes will rehash
    b["genesisExecutionBody"]["exactEffects"][0]["claimRoot"] = framed_hash(DOMAIN_TAGS["ClaimArray"], b["requiredClaims"])
    link_hashes(b)
    if mutate_bodies:
        mutate_bodies(b)
        b["genesisExecutionBody"]["exactEffects"][0]["claimRoot"] = framed_hash(
            DOMAIN_TAGS["ClaimArray"], b["requiredClaims"]
        )
        link_hashes(b)
    if exchange_predecessor == "unbornStateBody":
        unborn_h = framed_hash(DOMAIN_TAGS["StateBody"], b["unbornStateBody"])
        b["exchangePlan"]["predecessors"][0]["stateHash"] = unborn_h
        b["exchangePlan"]["beforeStateHash"] = unborn_h
        plan_e_h = framed_hash(DOMAIN_TAGS["ExactPlanDocument"], b["exchangePlan"])
        b["authorityWrapperExchange"]["intentDigest"] = plan_e_h
        cons_e_h = framed_hash(DOMAIN_TAGS["ConsumptionIdBody"], b["consumptionExchange"])
        b["exchangeSuccessor"]["originalIntentDigest"] = b["genesisSuccessor"]["originalIntentDigest"]
        b["exchangeSuccessor"]["successorId"] = successor_id(b["genesisSuccessor"]["originalIntentDigest"], cons_e_h, 0)
        succ_e_h = framed_hash(DOMAIN_TAGS["SuccessorRecord"], b["exchangeSuccessor"])
        b["exchangeStateBody"]["successorRecordHash"] = succ_e_h
        b["preparedExchange"]["successors"] = [deepcopy(b["exchangeSuccessor"])]
        b["preparedExchange"]["postStateHash"] = framed_hash(DOMAIN_TAGS["StateBody"], b["exchangeStateBody"])
        b["acceptanceExchange"]["intentDigest"] = plan_e_h
        b["acceptanceExchange"]["successorRecord"] = deepcopy(b["exchangeSuccessor"])
        b["acceptanceExchange"]["preparedHash"] = framed_hash(DOMAIN_TAGS["PreparedBody"], b["preparedExchange"])
        wrap_e_h = framed_hash(DOMAIN_TAGS["AuthorityWrapper"], b["authorityWrapperExchange"])
        b["proofContextExchange"]["authorizationDigest"] = wrap_e_h
        b["proofContextExchange"]["postStateHash"] = b["preparedExchange"]["postStateHash"]
    ctx_sources = deepcopy(b["genesisBody"]["initialization"]["externalSources"])
    if expected_sources == "empty":
        ctx_sources = []
    elif expected_sources == "from_genesis":
        ctx_sources = deepcopy(b["genesisBody"]["initialization"]["externalSources"])
    ctx = {
        "network": "midnight_preview",
        "deployment": "preview_deployment_1",
        "now": NOW,
        "revokedPrincipals": [],
        "consumedNonces": [],
        "expectedSources": ctx_sources,
        "grants": [caller_grant(row) for row in b["genesisBody"]["initialization"]["debitAuthorities"]],
        "sidecarBytes": "0",
        "observationAssumption": deepcopy(assumption_list[0]),
    }
    return assemble_bundle(b, ctx)


def main(argv=None) -> int:
    parser = argparse.ArgumentParser(description="Write funded Genesis->Exchange signing history example")
    parser.add_argument(
        "--output",
        default=str(HERE / "signing-history-example.json"),
        help="Output path",
    )
    args = parser.parse_args(argv)
    bundle = build_history()
    Path(args.output).write_text(canonical(bundle) + "\n", encoding="utf-8")
    print(args.output)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
