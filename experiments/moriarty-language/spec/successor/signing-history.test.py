#!/usr/bin/env python3
"""Behavioral tests for funded Genesis -> Exchange signing-history replay.

Independent arithmetic and node counts; does not treat validator output or
fixture labels as expected validity. Does not import signing-contract.test.py.
"""
from __future__ import annotations

import importlib.util
import json
import sys
import unittest
from copy import deepcopy
from pathlib import Path

HERE = Path(__file__).resolve().parent


def _load(name, filename):
    spec = importlib.util.spec_from_file_location(name, HERE / filename)
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod

MANTISSA = 19743
SCALE = 4
DEN = 10 ** SCALE
TRADER_A0 = 100000
POOL_A0 = 1000000
POOL_B0 = 2000000
TRADER_B0 = 0
UINT128_MAX = (1 << 128) - 1
CHILD_KEYS = ("cond", "expr", "left", "right", "record", "value", "operand")


def count_expr_nodes(node) -> int:
    if isinstance(node, list):
        return sum(count_expr_nodes(x) for x in node)
    if not isinstance(node, dict) or "tag" not in node:
        return 0
    total = 1
    for key in CHILD_KEYS:
        if key in node:
            total += count_expr_nodes(node[key])
    return total


def exchange_math(amount_in: int, mantissa: int = MANTISSA, scale: int = SCALE):
    den = 10 ** scale
    prod = amount_in * mantissa
    return prod // den, prod % den, den


def canonical(obj) -> str:
    return json.dumps(obj, sort_keys=True, separators=(",", ":"), ensure_ascii=False)


def preimage(bundle, name):
    for row in bundle["preimages"]:
        if row["name"] == name:
            if row["closedType"] == "SourceUtf8":
                return row["canonicalUtf8"], row
            return json.loads(row["canonicalUtf8"]), row
    raise AssertionError(name)


def set_preimage(bundle, name, body, domain=None, closed=None):
    text = body if isinstance(body, str) else canonical(body)
    for row in bundle["preimages"]:
        if row["name"] == name:
            row["canonicalUtf8"] = text
            if closed is not None:
                row["closedType"] = closed
            if domain is not None:
                row["domain"] = domain
            return row
    raise AssertionError(name)


def program_action(bundle, name):
    program, _ = preimage(bundle, "programBody")
    for action in program["actionBodies"]:
        if action["name"] == name:
            return action
    raise AssertionError(name)


def expected_charges(bundle):
    genesis_n = count_expr_nodes(program_action(bundle, "genesis")["expressions"])
    swap_n = count_expr_nodes(program_action(bundle, "swap")["expressions"])
    return 1 + genesis_n, 1 + swap_n


class HistoryTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.sh = _load("signing_history", "signing-history.py")
        gen = _load("generate_history_example", "generate-history-example.py")
        cls.build_history = staticmethod(gen.build_history)
        cls.baseline_bundle = gen.build_history()
        cls.baseline_source = canonical(cls.baseline_bundle)

    def validate(self, bundle_or_source):
        if isinstance(bundle_or_source, str):
            return self.sh.validate_history(bundle_or_source)
        return self.sh.validate_history(canonical(bundle_or_source))

    def assert_prepared(self, result, msg=None):
        self.assertEqual(result.get("status"), "PreparedHistory", msg or result)

    def assert_rejected(self, result, stage=None, code=None):
        self.assertEqual(result.get("status"), "Rejected", result)
        self.assertIn("stage", result)
        self.assertIn("code", result)
        self.assertIn("path", result)
        self.assertIn("evaluatedStages", result)
        self.assertNotIn("derived", result)
        for banned in ("effects", "postState", "genesisPostState", "exchangePostState"):
            self.assertNotIn(banned, result)
        if stage is not None:
            self.assertEqual(result["stage"], stage, result)
        if code is not None:
            self.assertEqual(result["code"], code, result)
        stages = result["evaluatedStages"]
        self.assertEqual(stages[-1], result["stage"], result)
        order = ["source", "structural", "domain", "hash", "context", "replay"]
        self.assertEqual(stages, order[: order.index(result["stage"]) + 1], result)

    def test_baseline_funded_cash_and_work(self):
        out, rem, den = exchange_math(10000)
        self.assertEqual(out, 19743)
        self.assertEqual(rem, 0)
        self.assertEqual(den, 10000)
        result = self.validate(self.baseline_source)
        self.assert_prepared(result)
        derived = result["derived"]
        bals = {
            (row["party"], row["asset"]): int(row["value"])
            for row in derived["exchangePostState"]["balances"]
        }
        self.assertEqual(bals[("trader", "AssetA")], 90000)
        self.assertEqual(bals[("pool", "AssetA")], 1010000)
        self.assertEqual(bals[("pool", "AssetB")], 1980257)
        self.assertEqual(bals[("trader", "AssetB")], 19743)
        g_chg, s_chg = expected_charges(self.baseline_bundle)
        self.assertGreater(g_chg, 1)
        self.assertGreater(s_chg, 1)
        gen_work = derived["genesisPostState"]["remainingWork"]
        swap_work = derived["exchangePostState"]["remainingWork"]
        life = int(gen_work["lifetimeOrdinary"])
        self.assertEqual(life, g_chg + s_chg)
        self.assertEqual(int(gen_work["ordinaryRemaining"]), life - g_chg)
        self.assertEqual(int(swap_work["ordinaryRemaining"]), life - g_chg - s_chg)
        self.assertEqual(gen_work["lifetimeRecovery"], "2")
        self.assertEqual(swap_work["recoveryRemaining"], "2")
        remaining = {(row["sourceId"], row["asset"]): int(row["balance"]) for row in derived["remainingExternalSources"]}
        self.assertTrue(all(v == 0 for v in remaining.values()), remaining)
        grants = {row["grantId"]: row for row in derived["remainingGrants"]}
        self.assertEqual(int(grants["grantPoolPayout"]["cap"]), 0)
        effects = derived["exchangePostState"] and derived["prepared"][1]["effects"]
        self.assertEqual(len(effects), 2)
        self.assertEqual(effects[0]["kind"], "Transfer")
        self.assertEqual(effects[0]["amount"], "10000")
        self.assertEqual(effects[1]["amount"], "19743")
        self.assertEqual(result["evaluatedStages"], ["source", "structural", "domain", "hash", "context", "replay"])
        self.assertEqual(derived["scope"], "genesis-exchange/0")
        self.assertNotIn("accepted", result)
        self.assertNotIn("proof", result)

    def test_alternate_input_amount(self):
        out, rem, den = exchange_math(5000)
        self.assertEqual(out, 9871)
        self.assertEqual(rem, 5000)
        self.assertEqual(den, 10000)
        bundle = self.build_history(amount_in=5000)
        result = self.validate(bundle)
        self.assert_prepared(result)
        bals = {
            (row["party"], row["asset"]): int(row["value"])
            for row in result["derived"]["exchangePostState"]["balances"]
        }
        self.assertEqual(bals[("trader", "AssetA")], TRADER_A0 - 5000)
        self.assertEqual(bals[("pool", "AssetA")], POOL_A0 + 5000)
        self.assertEqual(bals[("pool", "AssetB")], POOL_B0 - 9871)
        self.assertEqual(bals[("trader", "AssetB")], TRADER_B0 + 9871)
        remainder = json.loads(bundle["documents"][1]["canonicalUtf8"])["executionBody"]["remainder"]
        self.assertEqual(remainder["numerator"], "5000")
        self.assertEqual(remainder["denominator"], "10000")
        g_chg, s_chg = expected_charges(bundle)
        life = int(result["derived"]["genesisPostState"]["remainingWork"]["lifetimeOrdinary"])
        self.assertEqual(life, g_chg + s_chg)

    def test_alternate_party_ids(self):
        bundle = self.build_history(trader="alice", pool="vault")
        result = self.validate(bundle)
        self.assert_prepared(result)
        bals = {
            (row["party"], row["asset"]): int(row["value"])
            for row in result["derived"]["exchangePostState"]["balances"]
        }
        self.assertEqual(bals[("alice", "AssetA")], 90000)
        self.assertEqual(bals[("vault", "AssetB")], 1980257)

    def test_reversed_named_argument_order(self):
        bundle = self.build_history(reverse_swap_args=True)
        doc = json.loads(bundle["documents"][1]["canonicalUtf8"])
        names = [a["name"] for a in doc["executionBody"]["action"]["arguments"]]
        self.assertGreater(names.index("amount_in"), names.index("min_out"))
        result = self.validate(bundle)
        self.assert_prepared(result)
        bals = {
            (row["party"], row["asset"]): int(row["value"])
            for row in result["derived"]["exchangePostState"]["balances"]
        }
        self.assertEqual(bals[("trader", "AssetA")], 90000)

    def test_reordered_source_records(self):
        bundle = self.build_history(reverse_sources=True)
        genesis, _ = preimage(bundle, "genesisBody")
        ids = [row["sourceId"] for row in genesis["initialization"]["externalSources"]]
        self.assertEqual(ids[0], "srcTraderB")
        result = self.validate(bundle)
        self.assert_prepared(result)

    def test_unfunded_initialization(self):
        def mutate(bodies):
            bodies["genesisBody"]["initialization"]["allocations"] = []
            bodies["genesisBody"]["initialization"]["externalSources"] = []

        bundle = self.build_history(mutate_bodies=mutate, expected_sources="empty")
        self.assert_rejected(self.validate(bundle), stage="replay")

    def test_inconsistent_signed_context_funding(self):
        bundle = self.build_history()
        ctx = bundle["callerContext"]
        ctx["expectedSources"] = deepcopy(ctx["expectedSources"])
        ctx["expectedSources"][0]["balance"] = "1"
        self.assert_rejected(self.validate(bundle), stage="context")

    def test_aggregate_overdraw(self):
        def mutate(bodies):
            bodies["genesisBody"]["initialization"]["allocations"][0]["amount"] = "100001"

        self.assert_rejected(self.validate(self.build_history(mutate_bodies=mutate)), stage="replay")

    def test_duplicate_source_identity(self):
        def mutate(bodies):
            sources = bodies["genesisBody"]["initialization"]["externalSources"]
            sources.append(deepcopy(sources[0]))

        self.assert_rejected(self.validate(self.build_history(mutate_bodies=mutate)), stage="replay")

    def test_duplicate_grant_id(self):
        def mutate(bodies):
            grants = bodies["genesisBody"]["initialization"]["debitAuthorities"]
            extra = deepcopy(grants[0])
            extra["principal"] = "pool"
            grants.append(extra)

        self.assert_rejected(self.validate(self.build_history(mutate_bodies=mutate)), stage="replay")

    def test_absent_pool_grant(self):
        bundle = self.build_history()
        bundle["callerContext"]["grants"] = [
            g for g in bundle["callerContext"]["grants"] if g["grantId"] != "grantPoolPayout"
        ]
        self.assert_rejected(self.validate(bundle), stage="replay")

    def test_revoked_pool_grant(self):
        bundle = self.build_history()
        for g in bundle["callerContext"]["grants"]:
            if g["grantId"] == "grantPoolPayout":
                g["revoked"] = True
        self.assert_rejected(self.validate(bundle), stage="replay")

    def test_wrong_context_pool_grant(self):
        bundle = self.build_history()
        for g in bundle["callerContext"]["grants"]:
            if g["grantId"] == "grantPoolPayout":
                g["context"]["deployment"] = "other_deployment_1"
        self.assert_rejected(self.validate(bundle), stage="replay")

    def test_unexplained_debt(self):
        def mutate(bodies):
            bodies["genesisStateBody"]["debts"] = [
                {
                    "tag": "Debt",
                    "debtId": "Loan01",
                    "controller": "trader",
                    "debtor": "trader",
                    "creditor": "pool",
                    "denomination": "AssetA",
                    "principal": "1",
                    "accrued": "0",
                    "outstanding": "1",
                    "startDate": "1690000000",
                    "nextPaymentDate": "1692592000",
                    "lastAccrualEnd": "1690000000",
                    "accrualPeriodSeconds": "2592000",
                    "allocationRule": "AccrualFirst",
                    "negativeRateDisposition": "Reject",
                    "liabilityCap": "1",
                    "settlementAsset": "AssetA",
                    "conversionMantissa": "1",
                    "conversionScale": "0",
                    "conversionRounding": "floor",
                    "status": "Outstanding",
                }
            ]

        self.assert_rejected(self.validate(self.build_history(mutate_bodies=mutate)), stage="replay")

    def test_unexplained_duty(self):
        def mutate(bodies):
            bodies["genesisStateBody"]["duties"] = [
                {
                    "tag": "Duty",
                    "dutyId": "Duty01",
                    "kind": "UnsatisfiedNetGoal",
                    "creditor": "trader",
                    "debtor": "pool",
                    "status": "Open",
                    "nominalOrAsset": {"tag": "Amount", "asset": "AssetB", "value": "1"},
                }
            ]

        result = self.validate(self.build_history(mutate_bodies=mutate))
        self.assert_rejected(result, stage="replay")

    def test_unexplained_field(self):
        def mutate(bodies):
            extra = deepcopy(bodies["genesisStateBody"]["fields"][0])
            extra["selector"] = "ghost_a"
            bodies["genesisStateBody"]["fields"].append(extra)

        self.assert_rejected(self.validate(self.build_history(mutate_bodies=mutate)), stage="replay")

    def test_declared_field_mismatch(self):
        def mutate(bodies):
            bodies["genesisBody"]["declaredFields"][0]["selector"] = "trader_z"

        self.assert_rejected(self.validate(self.build_history(mutate_bodies=mutate)), stage="replay")

    def test_rehashed_balances_plus_999(self):
        def mutate(bodies):
            for row in bodies["exchangeStateBody"]["balances"]:
                if row["party"] == "trader" and row["asset"] == "AssetA":
                    row["value"] = str(int(row["value"]) + 999)
            for fld in bodies["exchangeStateBody"]["fields"]:
                if fld["selector"] == "trader_a":
                    fld["value"]["value"] = str(int(fld["value"]["value"]) + 999)

        result = self.validate(self.build_history(mutate_bodies=mutate))
        self.assert_rejected(result, stage="replay")

    def test_rehashed_transfer_10000_to_1(self):
        def mutate(bodies):
            bodies["exchangeExecutionBody"]["exactEffects"][0]["amount"] = "1"
            bodies["effectsExchange"]["effects"][0]["amount"] = "1"

        self.assert_rejected(self.validate(self.build_history(mutate_bodies=mutate)), stage="replay")

    def test_mismatched_untouched_collection(self):
        def mutate(bodies):
            bodies["exchangeStateBody"]["shares"] = [
                {"tag": "Shares", "vault": "vaultA", "holder": "trader", "value": "1"}
            ]

        self.assert_rejected(self.validate(self.build_history(mutate_bodies=mutate)), stage="replay")

    def test_duplicate_keyed_balances(self):
        def mutate(bodies):
            bals = bodies["exchangeStateBody"]["balances"]
            bals.append(deepcopy(bals[0]))

        self.assert_rejected(self.validate(self.build_history(mutate_bodies=mutate)), stage="replay")

    def test_insufficient_cash(self):
        def mutate(bodies):
            for row in bodies["genesisStateBody"]["balances"]:
                if row["party"] == "trader" and row["asset"] == "AssetA":
                    row["value"] = "5000"
            for fld in bodies["genesisStateBody"]["fields"]:
                if fld["selector"] == "trader_a":
                    fld["value"]["value"] = "5000"
            for w in bodies["genesisExecutionBody"]["exactWrites"]:
                if w["field"] == "trader_a":
                    w["value"]["value"] = "5000"
            for alloc in bodies["genesisBody"]["initialization"]["allocations"]:
                if alloc["selector"] == "trader_a":
                    alloc["amount"] = "5000"
            for src in bodies["genesisBody"]["initialization"]["externalSources"]:
                if src["sourceId"] == "srcTraderA":
                    src["balance"] = "5000"

        bundle = self.build_history(mutate_bodies=mutate, expected_sources="from_genesis")
        self.assert_rejected(self.validate(bundle), stage="replay")

    def test_insufficient_cap(self):
        def mutate(bodies):
            for cap in bodies["residualGenesis"]["grossDebitCaps"]:
                if cap["actor"] == "trader" and cap["asset"] == "AssetA":
                    cap["maximum"] = "1"
            bodies["genesisSuccessor"]["residualCapability"] = deepcopy(bodies["residualGenesis"])
            bodies["genesisStateBody"]["residualCapability"] = deepcopy(bodies["residualGenesis"])
            bodies["preparedGenesis"]["residualCapability"] = deepcopy(bodies["residualGenesis"])
            bodies["acceptanceGenesis"]["successorRecord"]["residualCapability"] = deepcopy(bodies["residualGenesis"])

        self.assert_rejected(self.validate(self.build_history(mutate_bodies=mutate)), stage="replay")

    def test_wrong_unit(self):
        def mutate(bodies):
            bodies["exchangeExecutionBody"]["action"]["arguments"][0]["value"]["asset"] = "AssetB"

        self.assert_rejected(self.validate(self.build_history(mutate_bodies=mutate)), stage="replay")

    def test_wrong_price(self):
        def mutate(bodies):
            bodies["exchangeExecutionBody"]["rateOrPrice"]["mantissa"] = "1"
            for arg in bodies["exchangeExecutionBody"]["action"]["arguments"]:
                if arg["name"] == "price":
                    arg["value"]["mantissa"] = "1"

        self.assert_rejected(self.validate(self.build_history(mutate_bodies=mutate)), stage="replay")

    def test_wrong_remainder(self):
        def mutate(bodies):
            bodies["exchangeExecutionBody"]["remainder"]["location"] = {
                "tag": "NamedParty",
                "party": "trader",
            }

        self.assert_rejected(self.validate(self.build_history(mutate_bodies=mutate)), stage="replay")

    def test_min_predicate_failure(self):
        bundle = self.build_history(min_out=19744, predicate="Min")
        self.assert_rejected(self.validate(bundle), stage="replay")

    def test_exact_predicate_failure(self):
        bundle = self.build_history(min_out=19744, predicate="Exact")
        self.assert_rejected(self.validate(bundle), stage="replay")

    def test_overflow_intermediate(self):
        huge = str(UINT128_MAX)
        def mutate(bodies):
            bodies["exchangeExecutionBody"]["rateOrPrice"]["mantissa"] = huge
            for arg in bodies["exchangeExecutionBody"]["action"]["arguments"]:
                if arg["name"] == "price":
                    arg["value"]["mantissa"] = huge

        self.assert_rejected(self.validate(self.build_history(mutate_bodies=mutate)), stage="replay")

    def test_core_charge_zero_keeps_total(self):
        def mutate(bodies):
            cert = bodies["exchangeExecutionBody"]["costCertificate"]
            total = int(cert["totalOrdinary"])
            cert["coreOps"][0]["ordinary"] = "0"
            cert["totalOrdinary"] = str(total)

        self.assert_rejected(self.validate(self.build_history(mutate_bodies=mutate)), stage="replay")

    def test_omitted_expression_charge(self):
        def mutate(bodies):
            cert = bodies["exchangeExecutionBody"]["costCertificate"]
            dropped = cert["expressions"].pop()
            cert["totalOrdinary"] = str(int(cert["totalOrdinary"]) - int(dropped["ordinary"]))

        self.assert_rejected(self.validate(self.build_history(mutate_bodies=mutate)), stage="replay")

    def test_fake_sidecar_length(self):
        def mutate(bodies):
            cert = bodies["exchangeExecutionBody"]["costCertificate"]
            cert["sidecarBytes"] = "256"
            cert["sidecarCharge"] = "1"
            cert["totalOrdinary"] = str(int(cert["totalOrdinary"]) + 1)

        bundle = self.build_history(mutate_bodies=mutate)
        bundle["callerContext"]["sidecarBytes"] = "256"
        self.assert_rejected(self.validate(bundle), stage="replay")

    def test_restored_spent_cap(self):
        def mutate(bodies):
            for cap in bodies["residualExchange"]["grossDebitCaps"]:
                if cap["actor"] == "trader" and cap["asset"] == "AssetA":
                    cap["maximum"] = "10000"
            bodies["exchangeSuccessor"]["residualCapability"] = deepcopy(bodies["residualExchange"])
            bodies["exchangeStateBody"]["residualCapability"] = deepcopy(bodies["residualExchange"])
            bodies["preparedExchange"]["residualCapability"] = deepcopy(bodies["residualExchange"])
            bodies["acceptanceExchange"]["successorRecord"]["residualCapability"] = deepcopy(
                bodies["residualExchange"]
            )

        self.assert_rejected(self.validate(self.build_history(mutate_bodies=mutate)), stage="replay")

    def test_work_counter_clone(self):
        def mutate(bodies):
            cloned = deepcopy(bodies["exchangeStateBody"]["remainingWork"])
            cloned["ordinaryRemaining"] = bodies["genesisStateBody"]["remainingWork"]["ordinaryRemaining"]
            bodies["exchangeStateBody"]["remainingWork"] = cloned
            bodies["preparedExchange"]["remainingWork"] = deepcopy(cloned)
            bodies["exchangeSuccessor"]["remainingWork"] = deepcopy(cloned)
            bodies["acceptanceExchange"]["remainingWork"] = deepcopy(cloned)

        self.assert_rejected(self.validate(self.build_history(mutate_bodies=mutate)), stage="replay")

    def test_missing_genesis_document(self):
        bundle = self.build_history()
        bundle["documents"][0] = deepcopy(bundle["documents"][1])
        bundle["documents"][0]["name"] = "genesisPlan"
        self.assert_rejected(self.validate(bundle), stage="replay")

    def test_dangling_preimage(self):
        bundle = self.build_history()
        bundle["preimages"] = [row for row in bundle["preimages"] if row["name"] != "genesisBody"]
        self.assert_rejected(self.validate(bundle), stage="hash")

    def test_stale_preimage(self):
        bundle = self.build_history()
        body, row = preimage(bundle, "policyBody")
        body["rules"] = ["tampered_rule"]
        row["canonicalUtf8"] = canonical(body)
        self.assert_rejected(self.validate(bundle), stage="hash")

    def test_swapped_type_domain(self):
        bundle = self.build_history()
        _, row = preimage(bundle, "genesisBody")
        row["closedType"] = "ProgramBody"
        row["domain"] = "MORIARTY-SUCC-PROGRAM/0"
        self.assert_rejected(self.validate(bundle), stage="structural")

    def test_missing_required_claim_class(self):
        def mutate(bodies):
            bodies["requiredClaims"][3]["kind"] = "ContractInvariant"
            bodies["requiredClaims"][3]["claimId"] = "dup_invariant"

        result = self.validate(self.build_history(mutate_bodies=mutate))
        self.assertEqual(result.get("status"), "Rejected", result)
        self.assertIn(result.get("stage"), ("structural", "replay"))

    def test_wrong_source_bytes(self):
        bundle = self.build_history()
        _, row = preimage(bundle, "sourceUtf8")
        row["canonicalUtf8"] = row["canonicalUtf8"] + "\n#tamper"
        self.assert_rejected(self.validate(bundle), stage="hash")

    def test_altered_wrapper_principal(self):
        bundle = self.build_history()
        bundle["wrappers"][1]["principal"] = "pool"
        self.assert_rejected(self.validate(bundle), stage="context")

    def test_wrong_network(self):
        bundle = self.build_history()
        bundle["callerContext"]["network"] = "other_network"
        self.assert_rejected(self.validate(bundle), stage="context")

    def test_wrong_deployment(self):
        bundle = self.build_history()
        bundle["callerContext"]["deployment"] = "other_deployment_1"
        self.assert_rejected(self.validate(bundle), stage="context")

    def test_revoked_principal(self):
        bundle = self.build_history()
        bundle["callerContext"]["revokedPrincipals"] = ["trader"]
        self.assert_rejected(self.validate(bundle), stage="context")

    def test_validity_window(self):
        bundle = self.build_history()
        bundle["callerContext"]["now"] = "1"
        self.assert_rejected(self.validate(bundle), stage="context")

    def test_duplicate_consumed_nonce(self):
        bundle = self.build_history()
        bundle["callerContext"]["consumedNonces"].append(
            {
                "network": "midnight_preview",
                "deployment": "preview_deployment_1",
                "principal": "trader",
                "nonce": "NonceGenesis01",
            }
        )
        self.assert_rejected(self.validate(bundle), stage="context")

    def test_predecessor_mismatch(self):
        self.assert_rejected(
            self.validate(self.build_history(exchange_predecessor="unbornStateBody")),
            stage="replay",
        )

    def test_invented_failure_label_schema(self):
        bundle = self.build_history()
        bundle["intendedFailureStage"] = "hash"
        result = self.validate(bundle)
        self.assert_rejected(result, stage="structural")

    def test_late_failure_no_tentative_outputs(self):
        result = self.validate(self.build_history(min_out=19744, predicate="Min"))
        self.assert_rejected(result, stage="replay")
        dumped = canonical(result)
        self.assertNotIn("90000", dumped)
        self.assertNotIn("PreparedHistory", dumped)

    def test_repeated_source_deterministic(self):
        a = self.validate(self.baseline_source)
        b = self.validate(self.baseline_source)
        self.assertEqual(canonical(a), canonical(b))

    def test_oversized_source(self):
        huge = "{" + "a" * (1024 * 1024) + "}"
        result = self.sh.validate_history(huge)
        self.assert_rejected(result, stage="source")

    def test_duplicate_json_key(self):
        src = '{"tag":"SigningHistory","tag":"SigningHistory"}'
        result = self.sh.validate_history(src)
        self.assert_rejected(result, stage="source")

    def test_deep_nesting(self):
        deep = "[" * 70 + "0" + "]" * 70
        result = self.sh.validate_history(deep)
        self.assert_rejected(result, stage="source")

    def test_malformed_input(self):
        result = self.sh.validate_history("{not json")
        self.assert_rejected(result, stage="source")

    def test_non_str_rejected(self):
        result = self.sh.validate_history(b"{}",)  # type: ignore[arg-type]
        self.assert_rejected(result, stage="source")

    def test_surrogate_rejected(self):
        result = self.sh.validate_history("{\uD800}")
        self.assert_rejected(result, stage="source")


def main():
    unittest.main(verbosity=2)


if __name__ == "__main__":
    main()
