"""Independent stop-rule and admission tests for assess()."""
from __future__ import annotations

import unittest

from moriarty_dev.policy import assess


DECISION_KEYS = ("allow", "reasonCode", "nextActionId", "missingEvidence")
ACTION_KEYS = (
    "id",
    "requirement",
    "capability",
    "kind",
    "candidate",
    "admissionRef",
    "commandRef",
    "evidenceProfile",
)
SNAPSHOT_KEYS = (
    "authorityCurrent",
    "entryEligible",
    "candidateCurrent",
    "resourceAdmitted",
    "sameDefectFailures",
    "adminCycles",
    "adminSeconds",
    "primaryActive",
    "reproducerVerified",
    "approachChanged",
    "nextActionId",
    "missingEvidence",
)
KINDS = (
    "implement",
    "reproduce",
    "repair",
    "verify",
    "review",
    "admin",
    "report",
)


def baseline():
    return {
        "authorityCurrent": True,
        "entryEligible": True,
        "candidateCurrent": True,
        "resourceAdmitted": True,
        "sameDefectFailures": 0,
        "adminCycles": 0,
        "adminSeconds": 0,
        "primaryActive": False,
        "reproducerVerified": False,
        "approachChanged": False,
        "nextActionId": "ledger-driver-reproduce",
        "missingEvidence": [],
    }


def action(kind):
    return {
        "id": "ledger-driver-" + kind,
        "requirement": "SP05",
        "capability": "fixed-financial-driver",
        "kind": kind,
        "candidate": "candidate-a",
        "admissionRef": "admission.json",
        "commandRef": "commands.json#driver",
        "evidenceProfile": "local-runtime",
    }


def other_lineage(kind):
    body = action(kind)
    body["id"] = "grammar-" + kind
    body["requirement"] = "SP02"
    body["capability"] = "successor-frontend"
    body["candidate"] = "grammar-candidate"
    return body


def decision_shape(test, result):
    test.assertEqual(tuple(result), DECISION_KEYS)
    test.assertIs(type(result["allow"]), bool)
    test.assertIs(type(result["reasonCode"]), str)
    test.assertTrue(
        result["nextActionId"] is None or type(result["nextActionId"]) is str
    )
    test.assertIs(type(result["missingEvidence"]), list)
    test.assertTrue(all(type(item) is str for item in result["missingEvidence"]))


class StopRules(unittest.TestCase):
    def test_third_broad_attempt_is_denied(self):
        s = baseline()
        s["sameDefectFailures"] = 2
        d = assess(s, action("implement"))
        self.assertFalse(d["allow"])
        self.assertEqual(d["reasonCode"], "REPRODUCE_BEFORE_RETRY")
        self.assertEqual(d["nextActionId"], "ledger-driver-reproduce")

    def test_denial_does_not_block_reproducer(self):
        s = baseline()
        s["sameDefectFailures"] = 2
        self.assertTrue(assess(s, action("reproduce"))["allow"])

    def test_admin_time_boundary(self):
        for seconds, allowed in [(1799, True), (1800, False)]:
            s = baseline()
            s["adminSeconds"] = seconds
            self.assertEqual(assess(s, action("admin"))["allow"], allowed)

    def test_stale_candidate_cannot_run(self):
        s = baseline()
        s["candidateCurrent"] = False
        self.assertEqual(
            assess(s, action("repair"))["reasonCode"], "EVIDENCE_STALE"
        )


class PolicyBoundaries(unittest.TestCase):
    def test_assess_is_callable_and_input_dependent(self):
        self.assertTrue(callable(assess))
        allowed = assess(baseline(), action("implement"))
        blocked = assess(
            dict(baseline(), sameDefectFailures=2), action("implement")
        )
        decision_shape(self, allowed)
        decision_shape(self, blocked)
        self.assertTrue(allowed["allow"])
        self.assertEqual(allowed["reasonCode"], "ALLOWED")
        self.assertFalse(blocked["allow"])
        self.assertNotEqual(allowed["reasonCode"], blocked["reasonCode"])

    def test_one_failure_still_allows_implement(self):
        s = baseline()
        s["sameDefectFailures"] = 1
        self.assertTrue(assess(s, action("implement"))["allow"])

    def test_two_failures_table(self):
        s = baseline()
        s["sameDefectFailures"] = 2
        expected = {
            "implement": False,
            "admin": False,
            "reproduce": True,
            "review": True,
            "report": True,
            "verify": True,
        }
        for kind, allowed in expected.items():
            result = assess(s, action(kind))
            self.assertEqual(result["allow"], allowed, kind)
            if not allowed:
                self.assertEqual(result["reasonCode"], "REPRODUCE_BEFORE_RETRY")

    def test_repair_requires_reproducer_and_changed_approach(self):
        cases = [
            (False, False, False),
            (True, False, False),
            (False, True, False),
            (True, True, True),
        ]
        for reproducer, changed, allowed in cases:
            s = baseline()
            s["sameDefectFailures"] = 2
            s["reproducerVerified"] = reproducer
            s["approachChanged"] = changed
            result = assess(s, action("repair"))
            self.assertEqual(result["allow"], allowed, (reproducer, changed))
            if not allowed:
                self.assertEqual(result["reasonCode"], "REPAIR_UNREADY")
            else:
                self.assertEqual(result["reasonCode"], "ALLOWED")

    def test_repair_ready_does_not_admit_another_implement(self):
        s = baseline()
        s["sameDefectFailures"] = 2
        s["reproducerVerified"] = True
        s["approachChanged"] = True
        self.assertFalse(assess(s, action("implement"))["allow"])
        self.assertTrue(assess(s, action("repair"))["allow"])

    def test_admin_cycle_boundary(self):
        for cycles, allowed in [(1, True), (2, False)]:
            s = baseline()
            s["adminCycles"] = cycles
            result = assess(s, action("admin"))
            self.assertEqual(result["allow"], allowed, cycles)
            if not allowed:
                self.assertEqual(result["reasonCode"], "ADMIN_LIMIT")

    def test_admin_limit_does_not_block_report_or_reproducer(self):
        s = baseline()
        s["adminCycles"] = 2
        s["adminSeconds"] = 1800
        self.assertTrue(assess(s, action("report"))["allow"])
        self.assertTrue(assess(s, action("reproduce"))["allow"])
        self.assertFalse(assess(s, action("admin"))["allow"])

    def test_authority_unavailable(self):
        s = baseline()
        s["authorityCurrent"] = False
        denied = assess(s, action("implement"))
        self.assertFalse(denied["allow"])
        self.assertEqual(denied["reasonCode"], "AUTHORITY_UNAVAILABLE")
        self.assertTrue(assess(s, action("report"))["allow"])

    def test_entry_ineligible(self):
        s = baseline()
        s["entryEligible"] = False
        denied = assess(s, action("implement"))
        self.assertFalse(denied["allow"])
        self.assertEqual(denied["reasonCode"], "ENTRY_INELIGIBLE")
        self.assertTrue(assess(s, action("report"))["allow"])

    def test_resource_exhausted(self):
        s = baseline()
        s["resourceAdmitted"] = False
        denied = assess(s, action("implement"))
        self.assertFalse(denied["allow"])
        self.assertEqual(denied["reasonCode"], "RESOURCE_EXHAUSTED")
        self.assertTrue(assess(s, action("report"))["allow"])

    def test_primary_active_blocks_implement_not_review_or_repair_path(self):
        s = baseline()
        s["primaryActive"] = True
        self.assertEqual(
            assess(s, action("implement"))["reasonCode"], "PRIMARY_ACTIVE"
        )
        self.assertEqual(
            assess(s, action("repair"))["reasonCode"], "PRIMARY_ACTIVE"
        )
        self.assertFalse(assess(s, action("admin"))["allow"])
        self.assertTrue(assess(s, action("review"))["allow"])
        self.assertTrue(assess(s, action("report"))["allow"])
        self.assertTrue(assess(s, action("reproduce"))["allow"])

    def test_repair_cannot_bypass_occupied_primary(self):
        s = baseline()
        s["primaryActive"] = True
        self.assertFalse(assess(s, action("repair"))["allow"])
        s["sameDefectFailures"] = 2
        s["reproducerVerified"] = True
        s["approachChanged"] = True
        result = assess(s, action("repair"))
        self.assertFalse(result["allow"])
        self.assertEqual(result["reasonCode"], "PRIMARY_ACTIVE")
        self.assertTrue(assess(s, action("reproduce"))["allow"])

    def test_check_order_prefers_authority_over_retry_rule(self):
        s = baseline()
        s["authorityCurrent"] = False
        s["sameDefectFailures"] = 2
        self.assertEqual(
            assess(s, action("implement"))["reasonCode"], "AUTHORITY_UNAVAILABLE"
        )

    def test_stale_candidate_beats_retry_rule(self):
        s = baseline()
        s["candidateCurrent"] = False
        s["sameDefectFailures"] = 2
        self.assertEqual(
            assess(s, action("implement"))["reasonCode"], "EVIDENCE_STALE"
        )

    def test_unknown_kind_rejected(self):
        body = action("implement")
        body["kind"] = "orchestrate"
        result = assess(baseline(), body)
        self.assertFalse(result["allow"])
        self.assertEqual(result["reasonCode"], "UNKNOWN_KIND")

    def test_lineage_rename_does_not_clear_failures(self):
        s = baseline()
        s["sameDefectFailures"] = 2
        renamed = action("implement")
        renamed["id"] = "fresh-packet-name"
        renamed["candidate"] = "candidate-b"
        result = assess(s, renamed)
        self.assertFalse(result["allow"])
        self.assertEqual(result["reasonCode"], "REPRODUCE_BEFORE_RETRY")

    def test_independent_capability_not_blocked_by_other_failures(self):
        s = baseline()
        s["sameDefectFailures"] = 0
        self.assertTrue(assess(s, other_lineage("implement"))["allow"])
        blocked = dict(s)
        blocked["sameDefectFailures"] = 2
        self.assertFalse(assess(blocked, action("implement"))["allow"])
        self.assertTrue(assess(s, other_lineage("implement"))["allow"])

    def test_history_unresolved_does_not_become_zero_failures(self):
        s = baseline()
        s["missingEvidence"] = ["operational-history"]
        denied = assess(s, action("implement"))
        self.assertFalse(denied["allow"])
        self.assertEqual(denied["reasonCode"], "HISTORY_UNRESOLVED")
        self.assertTrue(assess(s, action("report"))["allow"])
        self.assertTrue(assess(s, action("reproduce"))["allow"])
        repair = assess(s, action("repair"))
        self.assertFalse(repair["allow"])

    def test_report_survives_every_dispatch_block(self):
        s = baseline()
        s.update(
            authorityCurrent=False,
            entryEligible=False,
            candidateCurrent=False,
            resourceAdmitted=False,
            sameDefectFailures=2,
            adminCycles=2,
            adminSeconds=1800,
            primaryActive=True,
            missingEvidence=["operational-history", "commandRef-unresolved:x"],
        )
        result = assess(s, action("report"))
        self.assertTrue(result["allow"])
        self.assertEqual(result["reasonCode"], "ALLOWED")
        self.assertEqual(
            result["missingEvidence"],
            ["operational-history", "commandRef-unresolved:x"],
        )

    def test_command_gap_blocks_dispatch_not_report(self):
        s = baseline()
        s["missingEvidence"] = ["commandRef-unresolved:commands.json#missing"]
        self.assertEqual(
            assess(s, action("implement"))["reasonCode"], "ADMISSION_UNRESOLVED"
        )
        self.assertEqual(
            assess(s, action("reproduce"))["reasonCode"], "ADMISSION_UNRESOLVED"
        )
        self.assertTrue(assess(s, action("report"))["allow"])

    def test_admission_gap_named(self):
        s = baseline()
        s["missingEvidence"] = ["admissionRef-unresolved:campaign:missing"]
        result = assess(s, action("implement"))
        self.assertFalse(result["allow"])
        self.assertEqual(result["reasonCode"], "ADMISSION_UNRESOLVED")
        self.assertIn("admissionRef-unresolved:campaign:missing", result["missingEvidence"])


class ClosedBoundary(unittest.TestCase):
    def test_bool_is_not_int_for_counts(self):
        s = baseline()
        s["sameDefectFailures"] = True
        result = assess(s, action("implement"))
        self.assertFalse(result["allow"])
        self.assertEqual(result["reasonCode"], "INPUT_INVALID")
        self.assertTrue(
            any("sameDefectFailures" in item for item in result["missingEvidence"])
        )

    def test_int_is_not_bool_for_flags(self):
        s = baseline()
        s["authorityCurrent"] = 1
        result = assess(s, action("implement"))
        self.assertFalse(result["allow"])
        self.assertEqual(result["reasonCode"], "INPUT_INVALID")
        self.assertTrue(
            any("authorityCurrent" in item for item in result["missingEvidence"])
        )

    def test_float_seconds_rejected(self):
        s = baseline()
        s["adminSeconds"] = 1800.0
        result = assess(s, action("admin"))
        self.assertEqual(result["reasonCode"], "INPUT_INVALID")

    def test_unknown_snapshot_field_rejected(self):
        s = baseline()
        s["mutationId"] = "case-7"
        result = assess(s, action("implement"))
        self.assertEqual(result["reasonCode"], "INPUT_INVALID")

    def test_unknown_action_field_rejected(self):
        body = action("implement")
        body["expectedError"] = "REPRODUCE_BEFORE_RETRY"
        result = assess(baseline(), body)
        self.assertEqual(result["reasonCode"], "INPUT_INVALID")
        other = action("implement")
        other["expectedError"] = "ALLOWED"
        self.assertEqual(assess(baseline(), other)["reasonCode"], "INPUT_INVALID")

    def test_missing_action_key_rejected(self):
        body = action("implement")
        del body["capability"]
        self.assertEqual(
            assess(baseline(), body)["reasonCode"], "INPUT_INVALID"
        )

    def test_wrong_action_key_order_still_closed(self):
        body = {key: action("implement")[key] for key in reversed(ACTION_KEYS)}
        result = assess(baseline(), body)
        self.assertTrue(result["allow"])

    def test_snapshot_tuple_rejected(self):
        result = assess(tuple(baseline().values()), action("implement"))
        self.assertEqual(result["reasonCode"], "INPUT_INVALID")

    def test_missing_evidence_must_be_list_of_str(self):
        s = baseline()
        s["missingEvidence"] = ("operational-history",)
        self.assertEqual(
            assess(s, action("implement"))["reasonCode"], "INPUT_INVALID"
        )
        s = baseline()
        s["missingEvidence"] = [2]
        self.assertEqual(
            assess(s, action("implement"))["reasonCode"], "INPUT_INVALID"
        )

    def test_negative_counts_rejected(self):
        s = baseline()
        s["adminCycles"] = -1
        self.assertEqual(
            assess(s, action("admin"))["reasonCode"], "INPUT_INVALID"
        )

    def test_does_not_consult_fixture_identity(self):
        first = action("implement")
        first["id"] = "fixture-alpha"
        second = action("implement")
        second["id"] = "fixture-beta"
        self.assertEqual(
            assess(baseline(), first)["allow"],
            assess(baseline(), second)["allow"],
        )
        blocked = dict(baseline(), sameDefectFailures=2)
        self.assertEqual(
            assess(blocked, first)["reasonCode"],
            assess(blocked, second)["reasonCode"],
        )


if __name__ == "__main__":
    unittest.main()
