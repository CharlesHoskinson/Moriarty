"""Historical execution observations must coexist with, never grant, admission."""
import copy
import sys
import unittest
from pathlib import Path

sys.path[:0] = [str(Path(__file__).resolve().parent), str(Path(__file__).resolve().parents[1] / "scripts")]
import test_records as tr
from moriarty_dev.records import load_snapshot, read_actions
from moriarty_dev.policy import assess


class CampaignHistory(unittest.TestCase):
    setUp = tr.GenuineRegisters.setUp
    _install_funded_loan_review = tr.GenuineRegisters._install_funded_loan_review

    def install(self):
        self._install_funded_loan_review()
        self.records = tr.load_json(self.campaign_path)
        self.historical = {
            "owner": "MC01", "status": "failed-unsupported-compiler-flag",
            "scope": "Historical local K attempt only",
            "admission": "history/admission.json", "result": "history/result.json",
            "consumed": {"compileAttempts": 1, "krunInvocations": 0},
        }
        tr.write_json(self.root / self.historical["admission"], {"observation": "test admission"})
        tr.write_json(self.root / self.historical["result"], {"exitCode": 1})
        self.records["campaigns"]["historical-k"] = self.historical
        self.save()

    def save(self):
        tr.dump_json(self.campaign_path, self.records)

    def snapshot(self):
        return load_snapshot(str(self.root), "sp01-loan-review", history_reader=tr.verified_history())

    def test_observations_do_not_poison_unrelated_current_admission(self):
        self.install()
        for status, kruns in (("failed-unsupported-compiler-flag", 0),
                              ("failed-decoder-schema-after-successful-k-execution", 1),
                              ("executed-local-projection-awaiting-result-reviews", 16)):
            with self.subTest(status=status):
                self.historical["status"] = status
                self.historical["consumed"]["krunInvocations"] = kruns
                self.save()
                before = self.campaign_path.read_bytes()
                budget = (self.root / tr.CURRENT_ACCOUNTING_REL).read_bytes()
                snap = self.snapshot()
                self.assertTrue(assess(snap, read_actions(str(self.root))[0])["allow"], snap)
                self.assertEqual(self.campaign_path.read_bytes(), before)
                self.assertEqual((self.root / tr.CURRENT_ACCOUNTING_REL).read_bytes(), budget)

    def test_observations_cannot_substitute_for_selected_admission(self):
        self.install()
        actions = read_actions(str(self.root))
        actions[0]["admissionRef"] = "campaign:historical-k"
        tr.write_actions(self.root, actions)
        snap = self.snapshot()
        self.assertFalse(snap["authorityCurrent"])
        self.assertFalse(snap["candidateCurrent"])
        self.assertFalse(snap["resourceAdmitted"])
        self.assertFalse(assess(snap, actions[0])["allow"])

    def test_observations_cannot_promote_failed_record(self):
        self.install()
        for status in ("complete", "admitted-source-implementation", None, []):
            with self.subTest(status=status):
                self.historical["status"] = status
                self.save()
                snap = self.snapshot()
                self.assertFalse(snap["authorityCurrent"])
                self.assertTrue(any("historical-campaign-status" in item for item in snap["missingEvidence"]), snap)

    def test_consumption_is_closed_nonnegative_integer_history(self):
        self.install()
        for consumed in ({}, None, [], {"compileAttempts": 1},
                         {"compileAttempts": 1, "krunInvocations": 0, "refund": 1},
                         *({"compileAttempts": value, "krunInvocations": 0} for value in (-1, True, 1.5, "1")),
                         *({"compileAttempts": 1, "krunInvocations": value} for value in (-1, False, 0.5, "0"))):
            with self.subTest(consumed=consumed):
                self.historical["consumed"] = consumed
                self.save()
                snap = self.snapshot()
                self.assertFalse(snap["authorityCurrent"])
                self.assertTrue(any("historical-campaign-consumed" in item for item in snap["missingEvidence"]), snap)

    def test_provenance_paths_and_complete_observation_required(self):
        self.install()
        original = copy.deepcopy(self.historical)
        for field in ("admission", "result", "consumed"):
            with self.subTest(missing=field):
                self.records["campaigns"]["historical-k"] = copy.deepcopy(original)
                del self.records["campaigns"]["historical-k"][field]
                self.save()
                self.assertFalse(self.snapshot()["authorityCurrent"])
        for field in ("admission", "result"):
            for value in (None, [], "", "missing.json", "../outside.json", "/tmp/outside.json", "credentials.json"):
                with self.subTest(field=field, value=value):
                    self.records["campaigns"]["historical-k"] = copy.deepcopy(original)
                    self.records["campaigns"]["historical-k"][field] = value
                    self.save()
                    self.assertFalse(self.snapshot()["authorityCurrent"])

    def test_unknown_fields_still_fail_closed(self):
        self.install()
        self.historical["approved"] = True
        self.save()
        snap = self.snapshot()
        self.assertFalse(snap["authorityCurrent"])
        self.assertIn("unknown-campaign-field:approved", snap["missingEvidence"])

    def test_live_source_drift_is_not_repaired_by_history_support(self):
        self.install()
        source = self.root / "openspec/sprints/sp01-financial-contract-and-execution-admission.md"
        source.write_text(source.read_text() + "\nUnreviewed change.\n")
        snap = self.snapshot()
        self.assertFalse(snap["authorityCurrent"])
        self.assertFalse(snap["candidateCurrent"])
        self.assertTrue(any(item.startswith("binding-input-stale:") for item in snap["missingEvidence"]), snap)
